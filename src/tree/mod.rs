mod decode;

use std::collections::{HashMap, HashSet};

use anyhow::Result;
use serde::{Deserialize, Serialize};

use crate::providers::{well_known, DataProvider};
use crate::types::{CallFrame, CallType, DecodedArg, Log, ResolvedAbi};

/// The IR node tree. Every observable event during a transaction is one of these.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Node {
    Call(CallNode),
    Event(EventNode),
}

/// A call/create frame in the execution trace.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallNode {
    pub call_type: CallType,
    pub from: String,
    pub to: Option<String>,
    pub value: Option<String>,
    pub gas: String,
    pub gas_used: String,
    pub input: String,
    pub output: Option<String>,
    pub error: Option<String>,
    pub revert_reason: Option<String>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub function_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoded_input: Option<Vec<DecodedArg>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoded_output: Option<Vec<DecodedArg>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub contract_label: Option<String>,

    /// Ordered child nodes: sub-calls interleaved with emitted events.
    pub children: Vec<Node>,
}

/// An event log emitted during a call.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventNode {
    pub address: String,
    pub topics: Vec<String>,
    pub data: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub event_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub decoded_args: Option<Vec<DecodedArg>>,
}

/// Build the annotated IR tree from scratch.
///
/// Drives the full pipeline:
///   1. Fetch raw trace via providers (or use `cached` if supplied)
///   2. Collect all contract addresses referenced in the trace
///   3. Resolve ABIs and labels concurrently via providers
///   4. Build the IR tree, annotating each node inline
///   5. Propagate function names through delegate-call chains
///   6. Resolve any remaining unknown selectors via 4-byte lookup
pub async fn build(pm: &impl DataProvider) -> Result<Node> {
    let chain_id = pm.chain_id();
    let tx_hash = pm.tx_hash();

    // 1. Fetch trace (RpcProvider handles cache-hit internally)
    let raw = pm.fetch_trace(tx_hash, chain_id).await?;

    // 2. Collect addresses
    let addresses = collect_addresses(&raw);
    log::debug!("tree: {} unique addresses", addresses.len());

    // 3. Resolve ABIs + labels concurrently
    let (abis, labels) = tokio::join!(
        pm.resolve_abis(&addresses),
        pm.resolve_labels(&addresses),
    );
    log::debug!("tree: {} ABIs, {} labels resolved", abis.len(), labels.len());

    // 4. Build annotated tree
    let mut root = Node::Call(build_call_node(&raw, &abis, &labels));

    // 5. Propagate delegatecall names
    propagate_delegatecall_names(&mut root);

    // 6. 4-byte fallback for unresolved selectors
    let unresolved = collect_unresolved_selectors(&root);
    log::debug!("tree: {} unresolved selectors", unresolved.len());
    if !unresolved.is_empty() {
        let fourbyte = pm.resolve_selectors(&unresolved).await;
        log::debug!("tree: 4byte resolved {} selectors", fourbyte.len());
        apply_fourbyte_names(&mut root, &fourbyte);
    }

    Ok(root)
}

// ── Address collection ────────────────────────────────────────────────────────

fn collect_addresses(frame: &CallFrame) -> Vec<String> {
    let mut addrs = Vec::new();
    collect_addresses_inner(frame, &mut addrs);
    let mut seen = HashSet::new();
    addrs.retain(|a| seen.insert(a.to_lowercase()));
    addrs
}

fn collect_addresses_inner(frame: &CallFrame, out: &mut Vec<String>) {
    if let Some(to) = &frame.to {
        out.push(to.clone());
    }
    for log in &frame.logs {
        if !log.address.is_empty() {
            out.push(log.address.clone());
        }
    }
    for child in &frame.calls {
        collect_addresses_inner(child, out);
    }
}

// ── Tree construction + annotation ───────────────────────────────────────────

fn build_call_node(
    frame: &CallFrame,
    abis: &HashMap<String, ResolvedAbi>,
    labels: &HashMap<String, String>,
) -> CallNode {
    let addr_lower = frame.to.as_deref().map(|a| a.to_lowercase());

    let mut function_name: Option<String> = None;
    let mut decoded_input: Option<Vec<DecodedArg>> = None;
    let mut decoded_output: Option<Vec<DecodedArg>> = None;
    let mut contract_label: Option<String> = None;

    if let Some(addr) = &addr_lower {
        // Label (WellKnownProvider is top-priority, covers precompile names)
        if let Some(lbl) = labels.get(addr) {
            contract_label = Some(lbl.clone());
        }

        // Resolved ABI
        if let Some(resolved) = abis.get(addr) {
            if contract_label.is_none() {
                contract_label = resolved.contract_name.clone();
            }
            if !frame.input.is_empty() && frame.input != "0x" {
                let input_bytes =
                    hex::decode(frame.input.trim_start_matches("0x")).unwrap_or_default();
                if resolved.selector_free {
                    // Precompile: no selector prefix — decode raw bytes by function name
                    if let Some(fn_name) = resolved.contract_name.as_deref() {
                        if let Some(args) =
                            decode::decode_raw_input(&resolved.abi, fn_name, &input_bytes)
                        {
                            decoded_input = Some(args);
                            if let Some(output_hex) = &frame.output {
                                let output_bytes = hex::decode(
                                    output_hex.trim_start_matches("0x"),
                                )
                                .unwrap_or_default();
                                decoded_output =
                                    decode::decode_output(&resolved.abi, fn_name, &output_bytes);
                            }
                        }
                        function_name = Some(fn_name.to_string());
                    }
                } else if let Some((name, args)) =
                    decode::decode_input(&resolved.abi, &input_bytes)
                {
                    function_name = Some(name.clone());
                    decoded_input = Some(args);
                    if let Some(output_hex) = &frame.output {
                        let output_bytes =
                            hex::decode(output_hex.trim_start_matches("0x")).unwrap_or_default();
                        decoded_output = decode::decode_output(&resolved.abi, &name, &output_bytes);
                    }
                }
            } else if resolved.selector_free {
                // Precompile with empty input: function name is known from contract_name
                function_name = resolved.contract_name.clone();
            }
        }

        // Well-known ABI fallback (ERC-20, ERC-721)
        if function_name.is_none() && !frame.input.is_empty() && frame.input != "0x" {
            let input_bytes =
                hex::decode(frame.input.trim_start_matches("0x")).unwrap_or_default();
            'wk: for make_abi in
                [well_known::erc20_abi as fn() -> _, well_known::erc721_abi as fn() -> _]
            {
                let wk_abi = make_abi();
                if let Some((name, args)) = decode::decode_input(&wk_abi, &input_bytes) {
                    if let Some(output_hex) = &frame.output {
                        let output_bytes =
                            hex::decode(output_hex.trim_start_matches("0x")).unwrap_or_default();
                        decoded_output = decode::decode_output(&wk_abi, &name, &output_bytes);
                    }
                    function_name = Some(name);
                    decoded_input = Some(args);
                    break 'wk;
                }
            }
        }
    }

    // Recursive children: sub-calls first, then events
    let mut children: Vec<Node> = frame
        .calls
        .iter()
        .map(|c| Node::Call(build_call_node(c, abis, labels)))
        .collect();

    for log in &frame.logs {
        children.push(build_event_node(log, abis, addr_lower.as_deref(), frame.call_type == CallType::DelegateCall));
    }

    CallNode {
        call_type: frame.call_type.clone(),
        from: frame.from.clone(),
        to: frame.to.clone(),
        value: frame.value.clone(),
        gas: frame.gas.clone(),
        gas_used: frame.gas_used.clone(),
        input: frame.input.clone(),
        output: frame.output.clone(),
        error: frame.error.clone(),
        revert_reason: frame.revert_reason.clone(),
        function_name,
        decoded_input,
        decoded_output,
        contract_label,
        children,
    }
}

fn build_event_node(
    log: &Log,
    abis: &HashMap<String, ResolvedAbi>,
    frame_addr: Option<&str>,
    frame_is_delegatecall: bool,
) -> Node {
    let log_addr = log.address.to_lowercase();
    let topics: Vec<Vec<u8>> = log
        .topics
        .iter()
        .map(|t| hex::decode(t.trim_start_matches("0x")).unwrap_or_default())
        .collect();
    let data = hex::decode(log.data.trim_start_matches("0x")).unwrap_or_default();

    let decoded = abis.get(&log_addr).and_then(|r| decode::decode_event(&r.abi, &topics, &data));

    let decoded = decoded.or_else(|| {
        if frame_is_delegatecall {
            frame_addr
                .and_then(|a| abis.get(a))
                .and_then(|r| decode::decode_event(&r.abi, &topics, &data))
        } else {
            None
        }
    });

    let decoded = decoded
        .or_else(|| decode::decode_event(&well_known::erc20_abi(), &topics, &data))
        .or_else(|| decode::decode_event(&well_known::erc721_abi(), &topics, &data));

    let (event_name, decoded_args) = match decoded {
        Some((name, args)) => (Some(name), Some(args)),
        None => (None, None),
    };

    Node::Event(EventNode {
        address: log.address.clone(),
        topics: log.topics.clone(),
        data: log.data.clone(),
        event_name,
        decoded_args,
    })
}

// ── Post-build passes ─────────────────────────────────────────────────────────

fn propagate_delegatecall_names(node: &mut Node) {
    let Node::Call(call) = node else { return };

    for child in &mut call.children {
        propagate_delegatecall_names(child);
    }

    if call.function_name.is_some() {
        return;
    }

    let parent_selector = delegatecall_selector(&call.input);
    if parent_selector.is_none() {
        return;
    }

    let name = call.children.iter().find_map(|child| {
        if let Node::Call(c) = child {
            if c.call_type == CallType::DelegateCall
                && c.function_name.is_some()
                && delegatecall_selector(&c.input) == parent_selector
            {
                return c.function_name.clone();
            }
        }
        None
    });

    if let Some(n) = name {
        call.function_name = Some(n);
    }
}

fn collect_unresolved_selectors(node: &Node) -> Vec<String> {
    let mut selectors = HashSet::new();
    collect_unresolved_inner(node, &mut selectors);
    selectors.into_iter().collect()
}

fn collect_unresolved_inner(node: &Node, out: &mut HashSet<String>) {
    let Node::Call(call) = node else { return };
    if call.function_name.is_none() {
        let hex = call.input.trim_start_matches("0x");
        if hex.len() >= 8 {
            out.insert(format!("0x{}", &hex[..8].to_lowercase()));
        }
    }
    for child in &call.children {
        collect_unresolved_inner(child, out);
    }
}

fn apply_fourbyte_names(node: &mut Node, names: &HashMap<String, String>) {
    let Node::Call(call) = node else { return };
    if call.function_name.is_none() {
        let hex = call.input.trim_start_matches("0x");
        if hex.len() >= 8 {
            let key = format!("0x{}", &hex[..8].to_lowercase());
            if let Some(sig) = names.get(&key) {
                let input_bytes =
                    hex::decode(call.input.trim_start_matches("0x")).unwrap_or_default();
                if let Some((name, args)) = decode::decode_input_from_sig(sig, &input_bytes) {
                    call.function_name = Some(name);
                    call.decoded_input = Some(args);
                } else {
                    call.function_name = Some(sig.clone());
                }
            }
        }
    }
    for child in &mut call.children {
        apply_fourbyte_names(child, names);
    }
}

// ── Helpers ───────────────────────────────────────────────────────────────────

fn delegatecall_selector(input: &str) -> Option<[u8; 4]> {
    let hex = input.trim_start_matches("0x");
    if hex.len() < 8 {
        return None;
    }
    hex::decode(&hex[..8]).ok()?.try_into().ok()
}
