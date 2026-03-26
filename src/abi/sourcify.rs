use anyhow::{anyhow, Context, Result};
use alloy_json_abi::JsonAbi;
use serde_json::Value;

/// Fetch ABI from Sourcify.
/// Endpoint: GET https://sourcify.dev/server/v2/contract/{chainId}/{address}?fields=abi
pub async fn fetch_abi(
    http: &reqwest::Client,
    address: &str,
    chain_id: u64,
) -> Result<JsonAbi> {
    let url = format!(
        "https://sourcify.dev/server/v2/contract/{chain_id}/{address}?fields=abi"
    );

    let resp: Value = http
        .get(&url)
        .send()
        .await
        .context("Sourcify request failed")?
        .json()
        .await
        .context("Sourcify response parse failed")?;

    // Check for error
    if let Some(msg) = resp.get("message").and_then(|m| m.as_str()) {
        return Err(anyhow!("Sourcify: {}", msg));
    }

    let abi_val = resp
        .get("abi")
        .ok_or_else(|| anyhow!("Sourcify: no 'abi' field"))?;

    let abi: JsonAbi = serde_json::from_value(abi_val.clone())
        .context("Sourcify: failed to parse ABI")?;

    Ok(abi)
}
