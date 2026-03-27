use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use serde_json::{json, Value};

use crate::types::{CallFrame, CallType, Log};

use super::Provider;

/// Fetches traces via `debug_traceTransaction` and labels via ERC-20 `symbol()`.
///
/// Optionally holds a pre-fetched trace cache (keyed by tx hash) seeded during
/// RPC discovery so the first `fetch_trace` call avoids a redundant round-trip.
pub struct RpcProvider {
    http: reqwest::Client,
    url: String,
    cache: HashMap<String, Value>,
}

impl RpcProvider {
    pub fn new(http: reqwest::Client, url: String) -> Self {
        Self { http, url, cache: HashMap::new() }
    }

    pub fn url(&self) -> &str {
        &self.url
    }

    /// Seed the cache with a raw trace `Value` for `tx_hash` obtained during probing.
    pub fn with_cached_trace(mut self, tx_hash: String, trace: Value) -> Self {
        self.cache.insert(tx_hash, trace);
        self
    }

    pub async fn chain_id(&self) -> Result<u64> {
        fetch_chain_id(&self.http, &self.url).await
    }
}

#[async_trait]
impl Provider for RpcProvider {
    fn name(&self) -> &'static str {
        "rpc"
    }

    async fn fetch_trace(&self, tx_hash: &str, _chain_id: u64) -> Result<CallFrame> {
        if let Some(cached) = self.cache.get(tx_hash) {
            log::debug!("rpc: cache hit for {}", tx_hash);
            return parse_call_tracer_frame(cached);
        }
        fetch(&self.http, &self.url, tx_hash).await
    }

    async fn fetch_label(&self, address: &str, _chain_id: u64) -> Result<String> {
        fetch_erc20_symbol(&self.http, &self.url, address).await
    }
}

pub async fn fetch_chain_id(http: &reqwest::Client, rpc_url: &str) -> Result<u64> {
    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "eth_chainId",
        "params": []
    });

    log::debug!("rpc: POST eth_chainId to {}", rpc_url);
    let resp: Value = http
        .post(rpc_url)
        .json(&body)
        .send()
        .await
        .context("RPC request failed")?
        .json()
        .await
        .context("RPC response parse failed")?;

    let hex = resp
        .get("result")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("eth_chainId response missing 'result'"))?;

    let id = u64::from_str_radix(hex.trim_start_matches("0x"), 16)
        .context("eth_chainId: invalid hex")?;
    Ok(id)
}

async fn fetch(http: &reqwest::Client, rpc_url: &str, tx_hash: &str) -> Result<CallFrame> {
    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "debug_traceTransaction",
        "params": [tx_hash, {"tracer": "callTracer", "tracerConfig": {"withLog": true}}]
    });

    log::debug!("rpc: POST debug_traceTransaction {} to {}", tx_hash, rpc_url);
    let resp: Value = http
        .post(rpc_url)
        .json(&body)
        .send()
        .await
        .context("RPC request failed")?
        .json()
        .await
        .context("RPC response parse failed")?;

    if let Some(err) = resp.get("error") {
        return Err(anyhow!("RPC error: {}", err));
    }

    let result = resp
        .get("result")
        .ok_or_else(|| anyhow!("RPC response missing 'result'"))?;

    log::debug!("rpc: parsing call tracer frame");
    let frame = parse_call_tracer_frame(result)?;
    log::debug!("rpc: parsed {} top-level subcalls", frame.calls.len());
    Ok(frame)
}

pub fn parse_call_tracer_frame(v: &Value) -> Result<CallFrame> {
    let call_type = parse_call_type(v.get("type").and_then(|t| t.as_str()).unwrap_or("CALL"));

    let calls = if let Some(arr) = v.get("calls").and_then(|c| c.as_array()) {
        arr.iter()
            .filter_map(|c| parse_call_tracer_frame(c).ok())
            .collect()
    } else {
        vec![]
    };

    let logs = if let Some(arr) = v.get("logs").and_then(|l| l.as_array()) {
        arr.iter().map(parse_log).collect()
    } else {
        vec![]
    };

    Ok(CallFrame {
        call_type,
        from: v
            .get("from")
            .and_then(|s| s.as_str())
            .unwrap_or("")
            .to_string(),
        to: v
            .get("to")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string()),
        value: v
            .get("value")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string()),
        gas: v
            .get("gas")
            .and_then(|s| s.as_str())
            .unwrap_or("0x0")
            .to_string(),
        gas_used: v
            .get("gasUsed")
            .and_then(|s| s.as_str())
            .unwrap_or("0x0")
            .to_string(),
        input: v
            .get("input")
            .and_then(|s| s.as_str())
            .unwrap_or("0x")
            .to_string(),
        output: v
            .get("output")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string()),
        error: v
            .get("error")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string()),
        revert_reason: v
            .get("revertReason")
            .and_then(|s| s.as_str())
            .map(|s| s.to_string()),
        calls,
        logs,
    })
}

fn parse_log(v: &Value) -> Log {
    let address = v
        .get("address")
        .and_then(|s| s.as_str())
        .unwrap_or("")
        .to_string();

    let topics = v
        .get("topics")
        .and_then(|t| t.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|t| t.as_str().map(|s| s.to_string()))
                .collect()
        })
        .unwrap_or_default();

    let data = v
        .get("data")
        .and_then(|s| s.as_str())
        .unwrap_or("0x")
        .to_string();

    Log { address, topics, data }
}

fn parse_call_type(s: &str) -> CallType {
    match s.to_uppercase().as_str() {
        "CALL" => CallType::Call,
        "DELEGATECALL" => CallType::DelegateCall,
        "STATICCALL" => CallType::StaticCall,
        "CALLCODE" => CallType::CallCode,
        "CREATE" => CallType::Create,
        "CREATE2" => CallType::Create2,
        _ => CallType::Call,
    }
}

/// Call ERC-20 `symbol()` on `address` and return the decoded string.
pub(crate) async fn fetch_erc20_symbol(
    http: &reqwest::Client,
    rpc_url: &str,
    address: &str,
) -> Result<String> {
    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "eth_call",
        "params": [{"to": address, "data": "0x95d89b41"}, "latest"],
        "id": 1
    });

    let resp: serde_json::Value =
        http.post(rpc_url).json(&body).send().await?.json().await?;

    let hex = resp
        .get("result")
        .and_then(|v| v.as_str())
        .filter(|s| *s != "0x" && !s.is_empty())
        .ok_or_else(|| anyhow::anyhow!("no result"))?;

    let data = hex::decode(hex.trim_start_matches("0x"))?;
    let symbol = decode_abi_string(&data).ok_or_else(|| anyhow::anyhow!("decode failed"))?;
    if symbol.is_empty() {
        anyhow::bail!("empty symbol");
    }
    Ok(symbol)
}

/// Decode an ABI-encoded `string` return value (offset + length + data).
fn decode_abi_string(data: &[u8]) -> Option<String> {
    if data.len() < 64 {
        return None;
    }
    let len_bytes: [u8; 8] = data[56..64].try_into().ok()?;
    let len = u64::from_be_bytes(len_bytes) as usize;
    if data.len() < 64 + len {
        return None;
    }
    String::from_utf8(data[64..64 + len].to_vec()).ok()
}
