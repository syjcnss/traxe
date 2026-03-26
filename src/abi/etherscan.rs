use anyhow::{anyhow, Context, Result};
use alloy_json_abi::JsonAbi;
use serde_json::Value;

/// Fetch ABI (and contract name) from Etherscan.
/// Endpoint: GET https://api.etherscan.io/v2/api?module=contract&action=getsourcecode&address={address}&chainid={chainId}&apikey={key}
pub async fn fetch_abi(
    http: &reqwest::Client,
    address: &str,
    chain_id: u64,
) -> Result<(JsonAbi, Option<String>)> {
    let api_key = std::env::var("ETHERSCAN_API_KEY")
        .unwrap_or_else(|_| "YourApiKeyToken".to_string());

    let url = format!(
        "https://api.etherscan.io/v2/api\
         ?module=contract&action=getsourcecode\
         &address={address}&chainid={chain_id}&apikey={api_key}"
    );

    let resp: Value = http
        .get(&url)
        .send()
        .await
        .context("Etherscan request failed")?
        .json()
        .await
        .context("Etherscan response parse failed")?;

    let status = resp.get("status").and_then(|s| s.as_str()).unwrap_or("0");
    if status != "1" {
        let msg = resp
            .get("result")
            .and_then(|r| r.as_str())
            .unwrap_or("unknown error");
        return Err(anyhow!("Etherscan: {}", msg));
    }

    let item = resp
        .get("result")
        .and_then(|r| r.as_array())
        .and_then(|a| a.first())
        .ok_or_else(|| anyhow!("Etherscan: empty result"))?;

    let abi_str = item
        .get("ABI")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("Etherscan: no ABI field"))?;

    if abi_str == "Contract source code not verified" {
        return Err(anyhow!("Etherscan: contract not verified"));
    }

    let abi: JsonAbi =
        serde_json::from_str(abi_str).context("Etherscan: failed to parse ABI")?;

    let contract_name = item
        .get("ContractName")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .map(|s| s.to_string());

    Ok((abi, contract_name))
}
