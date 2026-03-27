use anyhow::{anyhow, Context, Result};
use alloy_json_abi::JsonAbi;
use async_trait::async_trait;
use serde_json::Value;

use crate::types::ResolvedAbi;

use super::Provider;

/// Fetches contract ABIs and labels from Etherscan (requires ETHERSCAN_API_KEY).
pub struct EtherscanProvider {
    http: reqwest::Client,
    api_key: String,
}

impl EtherscanProvider {
    pub fn new(http: reqwest::Client, api_key: String) -> Self {
        Self { http, api_key }
    }
}

#[async_trait]
impl Provider for EtherscanProvider {
    fn name(&self) -> &'static str {
        "etherscan"
    }

    async fn fetch_abi(&self, address: &str, chain_id: u64) -> Result<ResolvedAbi> {
        let (abi, contract_name) =
            fetch_abi_and_name(&self.http, address, chain_id, &self.api_key).await?;
        Ok(ResolvedAbi { abi, contract_name, selector_free: false })
    }

    async fn fetch_label(&self, address: &str, chain_id: u64) -> Result<String> {
        fetch_contract_name(&self.http, address, chain_id, &self.api_key).await
    }
}

async fn fetch_abi_and_name(
    http: &reqwest::Client,
    address: &str,
    chain_id: u64,
    api_key: &str,
) -> Result<(JsonAbi, Option<String>)> {
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

async fn fetch_contract_name(
    http: &reqwest::Client,
    address: &str,
    chain_id: u64,
    api_key: &str,
) -> Result<String> {
    let url = format!(
        "https://api.etherscan.io/v2/api\
         ?module=contract&action=getsourcecode\
         &address={address}&chainid={chain_id}&apikey={api_key}"
    );

    let resp: serde_json::Value = http.get(&url).send().await?.json().await?;

    let status = resp.get("status").and_then(|s| s.as_str()).unwrap_or("0");
    if status != "1" {
        anyhow::bail!("etherscan label request failed");
    }

    let name = resp
        .get("result")
        .and_then(|r| r.as_array())
        .and_then(|a| a.first())
        .and_then(|item| item.get("ContractName"))
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| anyhow::anyhow!("no contract name"))?
        .to_string();

    Ok(name)
}
