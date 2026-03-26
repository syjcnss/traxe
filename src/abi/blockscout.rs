use anyhow::{anyhow, Context, Result};
use alloy_json_abi::JsonAbi;
use serde_json::Value;

/// Fetch ABI from Blockscout REST API v2.
/// Endpoint: GET {base}/api/v2/smart-contracts/{address}
pub async fn fetch_abi(
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
