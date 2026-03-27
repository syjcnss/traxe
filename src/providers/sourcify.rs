use std::collections::HashMap;

use anyhow::{anyhow, Context, Result};
use alloy_json_abi::JsonAbi;
use async_trait::async_trait;
use serde_json::Value;

use crate::types::ResolvedAbi;

use super::Provider;

/// Fetches verified contract ABIs from Sourcify (always available, no credentials required).
pub struct SourcifyProvider {
    http: reqwest::Client,
}

impl SourcifyProvider {
    pub fn new(http: reqwest::Client) -> Self {
        Self { http }
    }
}

#[async_trait]
impl Provider for SourcifyProvider {
    fn name(&self) -> &'static str {
        "sourcify"
    }

    async fn fetch_abi(&self, address: &str, chain_id: u64) -> Result<ResolvedAbi> {
        let url = format!(
            "https://sourcify.dev/server/v2/contract/{chain_id}/{address}?fields=abi,compilation"
        );
        let resp: Value = self.http
            .get(&url)
            .send()
            .await
            .context("Sourcify request failed")?
            .json()
            .await
            .context("Sourcify response parse failed")?;

        if let Some(msg) = resp.get("message").and_then(|m| m.as_str()) {
            return Err(anyhow!("Sourcify: {}", msg));
        }

        let abi_val = resp
            .get("abi")
            .ok_or_else(|| anyhow!("Sourcify: no 'abi' field"))?;
        let abi: JsonAbi = serde_json::from_value(abi_val.clone())
            .context("Sourcify: failed to parse ABI")?;

        let contract_name = resp
            .get("compilation")
            .and_then(|c| c.get("name"))
            .and_then(|n| n.as_str())
            .map(|s| s.to_string());

        Ok(ResolvedAbi { abi, contract_name, selector_free: false })
    }

    async fn fetch_label(&self, address: &str, chain_id: u64) -> Result<String> {
        let url = format!(
            "https://sourcify.dev/server/v2/contract/{chain_id}/{address}?fields=compilation"
        );
        let resp: Value = self.http
            .get(&url)
            .send()
            .await
            .context("Sourcify request failed")?
            .json()
            .await
            .context("Sourcify response parse failed")?;

        if let Some(msg) = resp.get("message").and_then(|m| m.as_str()) {
            return Err(anyhow!("Sourcify: {}", msg));
        }

        resp.get("compilation")
            .and_then(|c| c.get("name"))
            .and_then(|n| n.as_str())
            .map(|s| s.to_string())
            .ok_or_else(|| anyhow!("Sourcify: no contract name"))
    }
}

/// Look up function names for a set of 4-byte selectors via openchain.xyz.
/// `selectors` should be lowercase hex strings like "0x18cbafe5".
/// Returns a map of selector -> function name (without argument types).
pub async fn lookup_selectors(
    http: &reqwest::Client,
    selectors: &[String],
) -> HashMap<String, String> {
    if selectors.is_empty() {
        return HashMap::new();
    }

    let query = selectors.join(",");
    let url = format!(
        "https://api.4byte.sourcify.dev/signature-database/v1/lookup?function={query}&filter=true"
    );

    log::debug!("4byte: looking up {} selectors", selectors.len());
    let resp: serde_json::Value = match http.get(&url).send().await {
        Ok(r) => match r.json().await {
            Ok(v) => v,
            Err(e) => {
                log::debug!("4byte: response parse failed: {}", e);
                return HashMap::new();
            }
        },
        Err(e) => {
            log::debug!("4byte: request failed: {}", e);
            return HashMap::new();
        }
    };

    // Response: { "ok": true, "result": { "function": { "0x18cbafe5": [{ "name": "swapExactTokensForETH(...)" }] } } }
    let mut result = HashMap::new();

    if let Some(fns) = resp
        .get("result")
        .and_then(|r| r.get("function"))
        .and_then(|f| f.as_object())
    {
        for (selector, entries) in fns {
            if let Some(sig) = entries
                .as_array()
                .and_then(|a| a.first())
                .and_then(|e| e.get("name"))
                .and_then(|n| n.as_str())
            {
                result.insert(selector.to_lowercase(), sig.to_string());
            }
        }
    }

    result
}


