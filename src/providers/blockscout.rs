use anyhow::{anyhow, Context, Result};
use alloy_json_abi::JsonAbi;
use async_trait::async_trait;
use serde_json::Value;

use crate::types::{CallFrame, CallType, ResolvedAbi};

use super::Provider;

/// Fetches traces, ABIs, and labels from a Blockscout explorer instance.
pub struct BlockscoutProvider {
    http: reqwest::Client,
    base_url: String,
}

impl BlockscoutProvider {
    pub fn new(http: reqwest::Client, base_url: String) -> Self {
        Self { http, base_url }
    }
}

#[async_trait]
impl Provider for BlockscoutProvider {
    fn name(&self) -> &'static str {
        "blockscout"
    }

    async fn fetch_trace(&self, tx_hash: &str, _chain_id: u64) -> Result<CallFrame> {
        let base = self.base_url.trim_end_matches('/');
        let url = format!("{base}/api/v2/transactions/{tx_hash}/internal-transactions");

        let resp: Value = self
            .http
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

        let tx_url = format!("{base}/api/v2/transactions/{tx_hash}");
        let tx_resp: Value = self
            .http
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

    async fn fetch_abi(&self, address: &str, _chain_id: u64) -> Result<ResolvedAbi> {
        let abi = fetch_contract_abi(&self.http, &self.base_url, address).await?;
        Ok(ResolvedAbi { abi, contract_name: None, selector_free: false })
    }

    async fn fetch_label(&self, address: &str, _chain_id: u64) -> Result<String> {
        fetch_address_label(&self.http, &self.base_url, address).await
    }
}

fn build_frame_from_tx_and_internals(tx: &Value, internals: &[Value]) -> Result<CallFrame> {
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

    let child_frames: Vec<CallFrame> = internals
        .iter()
        .filter_map(|item| parse_internal(item).ok())
        .collect();

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
    })
}

async fn fetch_contract_abi(
    http: &reqwest::Client,
    base_url: &str,
    address: &str,
) -> Result<JsonAbi> {
    let base = base_url.trim_end_matches('/');
    let url = format!("{base}/api/v2/smart-contracts/{address}");

    let resp: Value = http
        .get(&url)
        .send()
        .await
        .context("Blockscout smart-contract request failed")?
        .json()
        .await
        .context("Blockscout smart-contract parse failed")?;

    if let Some(msg) = resp.get("message").and_then(|m| m.as_str()) {
        if resp.get("status").and_then(|s| s.as_str()) == Some("0") {
            return Err(anyhow!("Blockscout: {}", msg));
        }
    }

    let abi_val = resp
        .get("abi")
        .ok_or_else(|| anyhow!("Blockscout: no 'abi' field"))?;

    let abi: JsonAbi = serde_json::from_value(abi_val.clone())
        .context("Blockscout: failed to parse ABI")?;

    Ok(abi)
}

async fn fetch_address_label(
    http: &reqwest::Client,
    base_url: &str,
    address: &str,
) -> Result<String> {
    let base = base_url.trim_end_matches('/');
    let url = format!("{base}/api/v2/addresses/{address}");

    let resp: serde_json::Value = http.get(&url).send().await?.json().await?;

    if let Some(name) = resp
        .get("private_tags")
        .and_then(|t| t.as_array())
        .and_then(|a| a.first())
        .and_then(|t| t.get("label"))
        .and_then(|v| v.as_str())
    {
        return Ok(name.to_string());
    }

    if let Some(name) = resp
        .get("public_tags")
        .and_then(|t| t.as_array())
        .and_then(|a| a.first())
        .and_then(|t| t.get("label"))
        .and_then(|v| v.as_str())
    {
        return Ok(name.to_string());
    }

    let name = resp
        .get("name")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| anyhow::anyhow!("no label"))?
        .to_string();

    Ok(name)
}
