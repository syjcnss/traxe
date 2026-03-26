use anyhow::{anyhow, Context, Result};
use serde_json::{json, Value};

use crate::types::{CallFrame, CallType};

/// Last-resort: re-fetch the transaction and simulate using eth_call with the historical block.
/// This uses debug_traceTransaction with a basic call tracer if the node supports it,
/// otherwise tries eth_getTransactionByHash + eth_call to reconstruct a minimal trace.
pub async fn fetch(
    http: &reqwest::Client,
    rpc_url: Option<&str>,
    tx_hash: &str,
    _chain_id: u64,
) -> Result<CallFrame> {
    let rpc = rpc_url.ok_or_else(|| {
        anyhow!("No RPC URL available for simulation. Provide --rpc or ALCHEMY_API_KEY.")
    })?;

    // First try: debug_traceTransaction with a simpler prestateTracer to at least get call info
    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "debug_traceTransaction",
        "params": [tx_hash, {"tracer": "callTracer"}]
    });

    let resp: Value = http
        .post(rpc)
        .json(&body)
        .send()
        .await
        .context("Simulate RPC request failed")?
        .json()
        .await
        .context("Simulate RPC response parse failed")?;

    if resp.get("error").is_none() {
        if let Some(result) = resp.get("result") {
            return super::rpc::parse_minimal_frame(result);
        }
    }

    // Fallback: get the transaction and build a single-frame trace
    let tx_body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "eth_getTransactionByHash",
        "params": [tx_hash]
    });

    let tx_resp: Value = http
        .post(rpc)
        .json(&tx_body)
        .send()
        .await
        .context("eth_getTransactionByHash failed")?
        .json()
        .await?;

    let tx = tx_resp
        .get("result")
        .ok_or_else(|| anyhow!("Transaction not found: {}", tx_hash))?;

    let receipt_body = json!({
        "jsonrpc": "2.0",
        "id": 2,
        "method": "eth_getTransactionReceipt",
        "params": [tx_hash]
    });

    let receipt_resp: Value = http
        .post(rpc)
        .json(&receipt_body)
        .send()
        .await
        .context("eth_getTransactionReceipt failed")?
        .json()
        .await?;

    let receipt = receipt_resp.get("result").and_then(|r| {
        if r.is_null() { None } else { Some(r) }
    });

    let success = receipt
        .and_then(|r| r.get("status"))
        .and_then(|s| s.as_str())
        .map(|s| s == "0x1")
        .unwrap_or(true);

    Ok(CallFrame {
        call_type: CallType::Call,
        from: tx
            .get("from")
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string(),
        to: tx
            .get("to")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        value: tx
            .get("value")
            .and_then(|v| v.as_str())
            .filter(|s| *s != "0x0")
            .map(|s| s.to_string()),
        gas: tx
            .get("gas")
            .and_then(|v| v.as_str())
            .unwrap_or("0x0")
            .to_string(),
        gas_used: receipt
            .and_then(|r| r.get("gasUsed"))
            .and_then(|v| v.as_str())
            .unwrap_or("0x0")
            .to_string(),
        input: tx
            .get("input")
            .and_then(|v| v.as_str())
            .unwrap_or("0x")
            .to_string(),
        output: None,
        error: if !success {
            Some("reverted".to_string())
        } else {
            None
        },
        revert_reason: None,
        calls: vec![],
        logs: vec![],
        function_name: None,
        decoded_input: None,
        decoded_output: None,
        contract_label: None,
    })
}
