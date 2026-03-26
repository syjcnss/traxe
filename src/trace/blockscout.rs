use anyhow::{anyhow, Context, Result};
use serde_json::Value;

use crate::types::{CallFrame, CallType};

/// Fetch trace from Blockscout REST API v2.
/// Endpoint: GET {base_url}/api/v2/transactions/{tx_hash}/internal-transactions
pub async fn fetch(
    http: &reqwest::Client,
    base_url: &str,
    tx_hash: &str,
) -> Result<CallFrame> {
    let base = base_url.trim_end_matches('/');
    let url = format!("{base}/api/v2/transactions/{tx_hash}/internal-transactions");

    let resp: Value = http
        .get(&url)
        .send()
        .await
        .context("Blockscout request failed")?
        .json()
        .await
        .context("Blockscout response parse failed")?;

    if let Some(msg) = resp.get("message").and_then(|m| m.as_str()) {
        if resp.get("status").and_then(|s| s.as_str()) == Some("0") {
            return Err(anyhow!("Blockscout error: {}", msg));
        }
    }

    // Also try to get the top-level transaction for the root call
    let tx_url = format!("{base}/api/v2/transactions/{tx_hash}");
    let tx_resp: Value = http
        .get(&tx_url)
        .send()
        .await
        .context("Blockscout tx request failed")?
        .json()
        .await
        .context("Blockscout tx response parse failed")?;

    let items = resp
        .get("items")
        .and_then(|i| i.as_array())
        .ok_or_else(|| anyhow!("Blockscout: no items in response"))?;

    build_frame_from_tx_and_internals(&tx_resp, items)
}

fn build_frame_from_tx_and_internals(tx: &Value, internals: &[Value]) -> Result<CallFrame> {
    // Build root frame from the transaction itself
    let from = tx
        .get("from")
        .and_then(|v| v.get("hash"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let to = tx
        .get("to")
        .and_then(|v| v.get("hash"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let value = tx
        .get("value")
        .and_then(|v| v.as_str())
        .filter(|s| *s != "0")
        .map(|s| s.to_string());

    let input = tx
        .get("raw_input")
        .and_then(|v| v.as_str())
        .unwrap_or("0x")
        .to_string();

    let gas = tx
        .get("gas_limit")
        .and_then(|v| v.as_str())
        .unwrap_or("0x0")
        .to_string();

    let gas_used = tx
        .get("gas_used")
        .and_then(|v| v.as_str())
        .unwrap_or("0x0")
        .to_string();

    let status = tx.get("status").and_then(|v| v.as_str()).unwrap_or("ok");
    let error = if status == "error" {
        tx.get("result")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
    } else {
        None
    };

    // Build internal call frames
    let mut child_frames: Vec<CallFrame> = internals
        .iter()
        .filter_map(|item| parse_internal(item).ok())
        .collect();

    // Blockscout returns internals in flat list; they may have index to reconstruct ordering.
    // For simplicity, treat them as direct children of the root.
    // A more sophisticated approach would use `index` and `call_index` to reconstruct nesting.
    // Sort by index if available
    child_frames.sort_by_key(|_| 0usize); // already in order

    Ok(CallFrame {
        call_type: CallType::Call,
        from,
        to,
        value,
        gas,
        gas_used,
        input,
        output: None,
        error,
        revert_reason: None,
        calls: child_frames,
        logs: vec![],
        function_name: None,
        decoded_input: None,
        decoded_output: None,
        contract_label: None,
    })
}

fn parse_internal(item: &Value) -> Result<CallFrame> {
    let call_type_str = item
        .get("call_type")
        .and_then(|v| v.as_str())
        .unwrap_or("call");

    let call_type = match call_type_str.to_lowercase().as_str() {
        "call" => CallType::Call,
        "delegatecall" => CallType::DelegateCall,
        "staticcall" => CallType::StaticCall,
        "callcode" => CallType::CallCode,
        "create" | "create2" => CallType::Create,
        _ => CallType::Call,
    };

    let from = item
        .get("from")
        .and_then(|v| v.get("hash"))
        .and_then(|v| v.as_str())
        .unwrap_or("")
        .to_string();

    let to = item
        .get("to")
        .and_then(|v| v.get("hash"))
        .and_then(|v| v.as_str())
        .map(|s| s.to_string());

    let value = item
        .get("value")
        .and_then(|v| v.as_str())
        .filter(|s| *s != "0")
        .map(|s| s.to_string());

    let gas_str = item
        .get("gas_limit")
        .and_then(|v| v.as_str())
        .unwrap_or("0x0")
        .to_string();

    let success = item
        .get("success")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    let error = if !success {
        item.get("error")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string())
            .or(Some("reverted".to_string()))
    } else {
        None
    };

    Ok(CallFrame {
        call_type,
        from,
        to,
        value,
        gas: gas_str.clone(),
        gas_used: gas_str,
        input: item
            .get("input")
            .and_then(|v| v.as_str())
            .unwrap_or("0x")
            .to_string(),
        output: item
            .get("output")
            .and_then(|v| v.as_str())
            .map(|s| s.to_string()),
        error,
        revert_reason: None,
        calls: vec![],
        logs: vec![],
        function_name: None,
        decoded_input: None,
        decoded_output: None,
        contract_label: None,
    })
}
