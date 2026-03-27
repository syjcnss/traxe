use anyhow::{anyhow, Context as AnyhowContext, Result};
use serde_json::{json, Value};

use revm::{
    Context, DatabaseRef, InspectEvm, Inspector, MainBuilder,
    bytecode::Bytecode,
    context::{ContextTr, TxEnv},
    context_interface::CreateScheme,
    database::CacheDB,
    database_interface::DBErrorMarker,
    handler::MainnetContext,
    interpreter::{
        CallInputs, CallOutcome, CallScheme, CallValue, CreateInputs, CreateOutcome,
        interpreter::EthInterpreter,
    },
    primitives::{Address, B256, Bytes, TxKind, U256},
    primitives::hardfork::SpecId,
    state::AccountInfo,
};

use crate::types::{CallFrame, CallType, Log as CrateLog};

// ── helpers ──────────────────────────────────────────────────────────────────

fn parse_u256_hex(s: &str) -> U256 {
    let s = s.strip_prefix("0x").unwrap_or(s);
    U256::from_str_radix(s, 16).unwrap_or(U256::ZERO)
}

fn parse_u64_hex(s: &str) -> u64 {
    let s = s.strip_prefix("0x").unwrap_or(s);
    u64::from_str_radix(s, 16).unwrap_or(0)
}

fn fmt_addr(a: Address) -> String {
    format!("0x{}", hex::encode(a.as_slice()))
}

fn fmt_bytes(b: &[u8]) -> String {
    format!("0x{}", hex::encode(b))
}

fn decode_slot(s: &str) -> U256 {
    let s = s.strip_prefix("0x").unwrap_or(s);
    let bytes = hex::decode(s).unwrap_or_default();
    let mut arr = [0u8; 32];
    let len = bytes.len().min(32);
    arr[32 - len..].copy_from_slice(&bytes[bytes.len() - len..]);
    U256::from_be_bytes(arr)
}

// ── Database error type ───────────────────────────────────────────────────────

#[derive(Debug)]
struct DbError(String);

impl std::fmt::Display for DbError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for DbError {}
impl DBErrorMarker for DbError {}

// ── Lazy RPC database backend ─────────────────────────────────────────────────

/// A revm `DatabaseRef` that fetches state on demand via standard `eth_*` RPC
/// calls, forked at the parent block (block before the transaction).
///
/// This mirrors the approach used by `cast run`: no debug methods required.
struct HttpBackend {
    http: reqwest::Client,
    rpc_url: String,
    /// Hex block tag for state queries, e.g. `"0x12abc"` (block_number - 1).
    parent_block: String,
}

impl HttpBackend {
    fn new(http: reqwest::Client, rpc_url: String, tx_block_num: u64) -> Self {
        let parent = if tx_block_num > 0 { tx_block_num - 1 } else { 0 };
        Self { http, rpc_url, parent_block: format!("0x{:x}", parent) }
    }

    /// Fetch balance, nonce, and bytecode for `address` at the parent block —
    /// three calls in parallel.
    async fn fetch_account_info(&self, address: Address) -> Result<AccountInfo, DbError> {
        let addr = fmt_addr(address);
        let tag = &self.parent_block;
        let rpc = &self.rpc_url;

        log::debug!("[simulate] RPC fetch account info: addr={addr} block={tag}");
        let (bal, nonce, code): (Value, Value, Value) = tokio::try_join!(
            async {
                self.http
                    .post(rpc)
                    .json(&json!({
                        "jsonrpc": "2.0", "id": 1,
                        "method": "eth_getBalance",
                        "params": [&addr, tag]
                    }))
                    .send()
                    .await
                    .map_err(|e| DbError(e.to_string()))?
                    .json()
                    .await
                    .map_err(|e| DbError(e.to_string()))
            },
            async {
                self.http
                    .post(rpc)
                    .json(&json!({
                        "jsonrpc": "2.0", "id": 2,
                        "method": "eth_getTransactionCount",
                        "params": [&addr, tag]
                    }))
                    .send()
                    .await
                    .map_err(|e| DbError(e.to_string()))?
                    .json()
                    .await
                    .map_err(|e| DbError(e.to_string()))
            },
            async {
                self.http
                    .post(rpc)
                    .json(&json!({
                        "jsonrpc": "2.0", "id": 3,
                        "method": "eth_getCode",
                        "params": [&addr, tag]
                    }))
                    .send()
                    .await
                    .map_err(|e| DbError(e.to_string()))?
                    .json()
                    .await
                    .map_err(|e| DbError(e.to_string()))
            }
        )
        .map_err(|e| DbError(format!("account fetch failed for {addr}: {e}")))?;

        let balance = bal
            .get("result")
            .and_then(|v| v.as_str())
            .map(parse_u256_hex)
            .unwrap_or(U256::ZERO);

        let nonce_val = nonce
            .get("result")
            .and_then(|v| v.as_str())
            .map(parse_u64_hex)
            .unwrap_or(0);

        let code_hex = code.get("result").and_then(|v| v.as_str()).unwrap_or("0x");
        let code_bytes =
            hex::decode(code_hex.strip_prefix("0x").unwrap_or(code_hex)).unwrap_or_default();
        let has_code = !code_bytes.is_empty();
        let bytecode = if code_bytes.is_empty() {
            Bytecode::new()
        } else {
            Bytecode::new_legacy(Bytes::from(code_bytes))
        };
        log::debug!(
            "[simulate] account fetched: addr={addr} balance={balance} nonce={nonce_val} has_code={has_code}"
        );
        Ok(AccountInfo {
            balance,
            nonce: nonce_val,
            code_hash: bytecode.hash_slow(),
            code: Some(bytecode),
            account_id: None,
        })
    }

    async fn fetch_storage(&self, address: Address, slot: U256) -> Result<U256, DbError> {
        let addr = fmt_addr(address);
        let slot_hex = format!("0x{:0>64x}", slot);

        let resp: Value = self
            .http
            .post(&self.rpc_url)
            .json(&json!({
                "jsonrpc": "2.0", "id": 1,
                "method": "eth_getStorageAt",
                "params": [&addr, &slot_hex, &self.parent_block]
            }))
            .send()
            .await
            .map_err(|e| DbError(e.to_string()))?
            .json()
            .await
            .map_err(|e| DbError(format!("storage fetch failed for {addr}[{slot_hex}]: {e}")))?;

        log::debug!("[simulate] RPC fetch storage: addr={addr} slot={slot_hex} block={}", self.parent_block);
        let value = resp
            .get("result")
            .and_then(|v| v.as_str())
            .map(decode_slot)
            .unwrap_or(U256::ZERO);
        log::debug!("[simulate] storage fetched: addr={addr} slot={slot_hex} value={value}");
        Ok(value)
    }
}

impl DatabaseRef for HttpBackend {
    type Error = DbError;

    /// Called by CacheDB on a cache miss. Fetches balance, nonce, and code.
    /// Returning `code: Some(...)` means CacheDB will never call `code_by_hash_ref`.
    fn basic_ref(&self, address: Address) -> Result<Option<AccountInfo>, Self::Error> {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(self.fetch_account_info(address))
                .map(Some)
        })
    }

    /// Not called when `basic_ref` returns `code: Some(...)`.
    fn code_by_hash_ref(&self, _hash: B256) -> Result<Bytecode, Self::Error> {
        Ok(Bytecode::new())
    }

    fn storage_ref(&self, address: Address, index: U256) -> Result<U256, Self::Error> {
        tokio::task::block_in_place(|| {
            tokio::runtime::Handle::current()
                .block_on(self.fetch_storage(address, index))
        })
    }

    /// Returns zero — block hashes aren't needed for typical transaction simulation.
    fn block_hash_ref(&self, _number: u64) -> Result<B256, Self::Error> {
        Ok(B256::ZERO)
    }
}

// ── Call tracer inspector ─────────────────────────────────────────────────────

struct PartialFrame {
    call_type: CallType,
    from: String,
    to: Option<String>,
    value: Option<String>,
    gas: String,
    input: String,
    calls: Vec<CallFrame>,
    logs: Vec<CrateLog>,
}

pub struct CallTracer {
    stack: Vec<PartialFrame>,
    pub result: Option<CallFrame>,
}

impl CallTracer {
    pub fn new() -> Self {
        Self { stack: vec![], result: None }
    }

    fn finish_frame(&mut self, gas_used: u64, output: Option<String>, error: Option<String>) {
        let Some(frame) = self.stack.pop() else { return };
        let cf = CallFrame {
            call_type: frame.call_type,
            from: frame.from,
            to: frame.to,
            value: frame.value,
            gas: frame.gas,
            gas_used: format!("0x{:x}", gas_used),
            input: frame.input,
            output,
            error,
            revert_reason: None,
            calls: frame.calls,
            logs: frame.logs,
            function_name: None,
            decoded_input: None,
            decoded_output: None,
            contract_label: None,
        };
        if self.stack.is_empty() {
            self.result = Some(cf);
        } else {
            self.stack.last_mut().unwrap().calls.push(cf);
        }
    }
}

impl<CTX> Inspector<CTX, EthInterpreter> for CallTracer
where
    CTX: ContextTr,
{
    fn call(&mut self, context: &mut CTX, inputs: &mut CallInputs) -> Option<CallOutcome> {
        let input_bytes = inputs.input.bytes(context);
        log::debug!(
            "[simulate] call: depth={} {:?} from={} to={} gas={:#x} input_len={}",
            self.stack.len(),
            inputs.scheme,
            fmt_addr(inputs.caller),
            fmt_addr(inputs.target_address),
            inputs.gas_limit,
            input_bytes.len(),
        );
        let call_type = match inputs.scheme {
            CallScheme::Call => CallType::Call,
            CallScheme::DelegateCall => CallType::DelegateCall,
            CallScheme::StaticCall => CallType::StaticCall,
            CallScheme::CallCode => CallType::CallCode,
        };
        let value = match &inputs.value {
            CallValue::Transfer(v) if *v != U256::ZERO => Some(format!("0x{:x}", v)),
            _ => None,
        };
        self.stack.push(PartialFrame {
            call_type,
            from: fmt_addr(inputs.caller),
            to: Some(fmt_addr(inputs.target_address)),
            value,
            gas: format!("0x{:x}", inputs.gas_limit),
            input: fmt_bytes(&input_bytes),
            calls: vec![],
            logs: vec![],
        });
        None
    }

    fn call_end(&mut self, _ctx: &mut CTX, _inputs: &CallInputs, outcome: &mut CallOutcome) {
        let gas_used = outcome.result.gas.spent();
        let ok = outcome.result.result.is_ok();
        let output = ok.then(|| fmt_bytes(&outcome.result.output));
        let error = (!ok).then(|| {
            if outcome.result.result.is_revert() { "reverted" } else { "halted" }.to_string()
        });
        log::debug!(
            "[simulate] call_end: depth={} ok={ok} gas_used={gas_used}",
            self.stack.len().saturating_sub(1),
        );
        self.finish_frame(gas_used, output, error);
    }

    fn create(&mut self, _ctx: &mut CTX, inputs: &mut CreateInputs) -> Option<CreateOutcome> {
        log::debug!(
            "[simulate] create: depth={} from={} gas={:#x} initcode_len={}",
            self.stack.len(),
            fmt_addr(inputs.caller()),
            inputs.gas_limit(),
            inputs.init_code().len(),
        );
        let call_type = match inputs.scheme() {
            CreateScheme::Create2 { .. } => CallType::Create2,
            _ => CallType::Create,
        };
        let value = inputs.value();
        self.stack.push(PartialFrame {
            call_type,
            from: fmt_addr(inputs.caller()),
            to: None,
            value: (value != U256::ZERO).then(|| format!("0x{:x}", value)),
            gas: format!("0x{:x}", inputs.gas_limit()),
            input: fmt_bytes(inputs.init_code()),
            calls: vec![],
            logs: vec![],
        });
        None
    }

    fn create_end(
        &mut self,
        _ctx: &mut CTX,
        _inputs: &CreateInputs,
        outcome: &mut CreateOutcome,
    ) {
        let gas_used = outcome.result.gas.spent();
        let ok = outcome.result.result.is_ok();
        log::debug!(
            "[simulate] create_end: depth={} ok={ok} gas_used={gas_used} deployed={:?}",
            self.stack.len().saturating_sub(1),
            outcome.address.map(fmt_addr),
        );
        let output = ok.then(|| outcome.address.map(fmt_addr)).flatten();
        let error = (!ok).then(|| {
            if outcome.result.result.is_revert() { "reverted" } else { "halted" }.to_string()
        });
        self.finish_frame(gas_used, output, error);
    }

    fn log(&mut self, _ctx: &mut CTX, log: revm::primitives::Log) {
        let Some(frame) = self.stack.last_mut() else { return };
        let topics = log
            .topics()
            .iter()
            .map(|t| format!("0x{}", hex::encode(t.as_slice())))
            .collect();
        frame.logs.push(CrateLog {
            address: fmt_addr(log.address),
            topics,
            data: fmt_bytes(&log.data.data),
            event_name: None,
            decoded_args: None,
        });
    }
}

// ── Main entry point ──────────────────────────────────────────────────────────

/// Local EVM simulation using only standard `eth_*` RPC methods.
///
/// Steps:
///   1. `eth_getTransactionByHash` — tx fields + block number
///   2. `eth_getBlockByNumber`     — block environment (number, timestamp, basefee, …)
///   3. Build `CacheDB<HttpBackend>` — state is fetched lazily during EVM execution
///      via `eth_getBalance` / `eth_getCode` / `eth_getTransactionCount` /
///      `eth_getStorageAt` at `blockNumber - 1` (parent block).
///   4. Execute with revm + `CallTracer` inspector → full call tree.
///
/// No `debug_traceTransaction` or `prestateTracer` required.
pub async fn fetch(
    http: &reqwest::Client,
    rpc_url: Option<&str>,
    tx_hash: &str,
    chain_id: u64,
) -> Result<CallFrame> {
    let rpc = rpc_url.ok_or_else(|| {
        anyhow!("No RPC URL available for simulation. Provide --rpc or ALCHEMY_API_KEY.")
    })?;

    log::debug!("[simulate] starting simulation for tx={tx_hash} chain_id={chain_id}");

    // ── Step 1: fetch transaction ─────────────────────────────────────────────
    log::debug!("[simulate] step 1: eth_getTransactionByHash");
    let tx_resp: Value = http
        .post(rpc)
        .json(&json!({
            "jsonrpc": "2.0", "id": 1,
            "method": "eth_getTransactionByHash",
            "params": [tx_hash]
        }))
        .send()
        .await
        .context("eth_getTransactionByHash request failed")?
        .json()
        .await
        .context("eth_getTransactionByHash parse failed")?;

    let tx = tx_resp
        .get("result")
        .and_then(|r| if r.is_null() { None } else { Some(r) })
        .ok_or_else(|| anyhow!("Transaction not found: {}", tx_hash))?;

    let block_number_str = tx
        .get("blockNumber")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("Transaction is pending (no blockNumber)"))?;
    let block_num = parse_u64_hex(block_number_str);
    log::debug!("[simulate] tx found in block={block_num} ({block_number_str})");

    // ── Step 2: fetch block for environment ───────────────────────────────────
    log::debug!("[simulate] step 2: eth_getBlockByNumber({block_number_str})");
    let block_resp: Value = http
        .post(rpc)
        .json(&json!({
            "jsonrpc": "2.0", "id": 2,
            "method": "eth_getBlockByNumber",
            "params": [block_number_str, false]
        }))
        .send()
        .await
        .context("eth_getBlockByNumber request failed")?
        .json()
        .await
        .context("eth_getBlockByNumber parse failed")?;

    let block = block_resp
        .get("result")
        .and_then(|r| if r.is_null() { None } else { Some(r) })
        .ok_or_else(|| anyhow!("Block {} not found", block_number_str))?;

    // ── Step 3: build lazy DB ─────────────────────────────────────────────────
    log::debug!("[simulate] step 3: building lazy RPC database (parent block={})", block_num - 1);
    let backend = HttpBackend::new(http.clone(), rpc.to_string(), block_num);
    let db: CacheDB<HttpBackend> = CacheDB::new(backend);

    // ── Step 4: block environment ─────────────────────────────────────────────
    let timestamp =
        parse_u64_hex(block.get("timestamp").and_then(|v| v.as_str()).unwrap_or("0x0"));
    let coinbase: Address = block
        .get("miner")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse().ok())
        .unwrap_or_default();
    let basefee =
        parse_u64_hex(block.get("baseFeePerGas").and_then(|v| v.as_str()).unwrap_or("0x0"));
    let block_gas_limit =
        parse_u64_hex(block.get("gasLimit").and_then(|v| v.as_str()).unwrap_or("0x1c9c380"));
    let difficulty =
        parse_u256_hex(block.get("difficulty").and_then(|v| v.as_str()).unwrap_or("0x0"));
    let prevrandao: Option<B256> = block
        .get("mixHash")
        .and_then(|v| v.as_str())
        .and_then(|s| {
            let b = hex::decode(s.strip_prefix("0x").unwrap_or(s)).ok()?;
            (b.len() == 32).then(|| B256::from_slice(&b))
        });

    log::debug!(
        "[simulate] block env: timestamp={timestamp} basefee={basefee} gas_limit={block_gas_limit} coinbase={}",
        fmt_addr(coinbase)
    );

    // ── Step 5: tx environment ────────────────────────────────────────────────
    let from: Address = tx
        .get("from")
        .and_then(|v| v.as_str())
        .and_then(|s| s.parse().ok())
        .unwrap_or_default();
    let to_opt: Option<Address> = tx
        .get("to")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .and_then(|s| s.parse().ok());
    let value = parse_u256_hex(tx.get("value").and_then(|v| v.as_str()).unwrap_or("0x0"));
    let gas_limit = parse_u64_hex(tx.get("gas").and_then(|v| v.as_str()).unwrap_or("0x0"));
    let data: Bytes = tx
        .get("input")
        .and_then(|v| v.as_str())
        .and_then(|s| hex::decode(s.strip_prefix("0x").unwrap_or(s)).ok())
        .map(Bytes::from)
        .unwrap_or_default();
    let nonce = parse_u64_hex(tx.get("nonce").and_then(|v| v.as_str()).unwrap_or("0x0"));
    let tx_chain_id: Option<u64> =
        tx.get("chainId").and_then(|v| v.as_str()).map(parse_u64_hex);
    let tx_type: u8 =
        parse_u64_hex(tx.get("type").and_then(|v| v.as_str()).unwrap_or("0x0")) as u8;
    let (gas_price, gas_priority_fee): (u128, Option<u128>) = if tx_type == 2 {
        let max_fee = parse_u64_hex(
            tx.get("maxFeePerGas").and_then(|v| v.as_str()).unwrap_or("0x0"),
        ) as u128;
        let priority = parse_u64_hex(
            tx.get("maxPriorityFeePerGas").and_then(|v| v.as_str()).unwrap_or("0x0"),
        ) as u128;
        (max_fee, Some(priority))
    } else {
        let price =
            parse_u64_hex(tx.get("gasPrice").and_then(|v| v.as_str()).unwrap_or("0x0"))
                as u128;
        (price, None)
    };

    log::debug!(
        "[simulate] tx env: from={} to={:?} value={value} gas_limit={gas_limit} tx_type={tx_type}",
        fmt_addr(from),
        to_opt.map(fmt_addr),
    );

    // ── Step 6: create revm context and execute ───────────────────────────────
    let spec = spec_id_for(chain_id, block_num);
    log::debug!("[simulate] step 6: executing with revm spec={spec:?}");
    let mut context: MainnetContext<CacheDB<HttpBackend>> = Context::new(db, spec);

    context.block.number = U256::from(block_num);
    context.block.timestamp = U256::from(timestamp);
    context.block.beneficiary = coinbase;
    context.block.basefee = basefee;
    context.block.gas_limit = block_gas_limit;
    context.block.difficulty = difficulty;
    context.block.prevrandao = prevrandao;

    context.cfg.chain_id = chain_id;
    context.cfg.disable_nonce_check = true;
    context.cfg.tx_chain_id_check = false;
    context.cfg.disable_base_fee = true;
    context.cfg.disable_block_gas_limit = true;
    context.cfg.disable_eip3607 = true;

    let tx_env = TxEnv::builder()
        .tx_type(Some(tx_type))
        .caller(from)
        .kind(to_opt.map(TxKind::Call).unwrap_or(TxKind::Create))
        .value(value)
        .data(data)
        .gas_limit(gas_limit)
        .gas_price(gas_price)
        .gas_priority_fee(gas_priority_fee)
        .nonce(nonce)
        .chain_id(tx_chain_id)
        .build_fill();

    let mut evm = context.build_mainnet_with_inspector(CallTracer::new());

    evm.inspect_tx(tx_env)
        .map_err(|e| anyhow!("EVM execution failed: {:?}", e))?;

    log::debug!("[simulate] EVM execution complete");
    evm.inspector
        .result
        .ok_or_else(|| anyhow!("Simulation produced no call trace"))
}

fn spec_id_for(chain_id: u64, block_num: u64) -> SpecId {
    match chain_id {
        1 => {
            if block_num >= 22_431_084 { SpecId::OSAKA }
            else if block_num >= 19_426_587 { SpecId::CANCUN }
            else if block_num >= 17_034_870 { SpecId::SHANGHAI }
            else if block_num >= 15_537_394 { SpecId::MERGE }
            else { SpecId::LONDON }
        }
        _ => SpecId::CANCUN,
    }
}
