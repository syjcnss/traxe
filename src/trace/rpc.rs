use anyhow::{anyhow, Context, Result};
use serde_json::{json, Value};

use crate::types::{CallFrame, CallType, Log};

pub async fn fetch(
    http: &reqwest::Client,
    rpc_url: &str,
    tx_hash: &str,
) -> Result<CallFrame> {
    let body = json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "debug_traceTransaction",
        "params": [tx_hash, {"tracer": "callTracer", "tracerConfig": {"withLog": true}}]
    });

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

    parse_call_tracer_frame(result)
}

pub fn parse_minimal_frame(v: &Value) -> Result<CallFrame> {
    parse_call_tracer_frame(v)
}

fn parse_call_tracer_frame(v: &Value) -> Result<CallFrame> {
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
        function_name: None,
        decoded_input: None,
        decoded_output: None,
        contract_label: None,
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

    Log { address, topics, data, event_name: None, decoded_args: None }
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
