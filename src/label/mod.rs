use std::collections::HashMap;

/// Resolve human-readable labels for addresses from Etherscan, Blockscout, or ERC20 `name()`.
/// Returns a map of lowercase address -> label string.
pub async fn resolve_labels(
    http: &reqwest::Client,
    addresses: &[String],
    chain_id: u64,
    rpc_url: Option<&str>,
) -> HashMap<String, String> {
    let mut labels = HashMap::new();

    for addr in addresses {
        let lower = addr.to_lowercase();

        // Try on-chain ERC20 name() first
        if let Some(rpc) = rpc_url {
            if let Ok(name) = fetch_erc20_name(http, rpc, &lower).await {
                labels.insert(lower, name);
                continue;
            }
        }

        // Try Etherscan contract name
        if let Ok(name) = fetch_etherscan_label(http, &lower, chain_id).await {
            labels.insert(lower.clone(), name);
            continue;
        }

        // Try Blockscout
        if let Some(bs_url) = std::env::var("BLOCKSCOUT_URL").ok() {
            if let Ok(name) = fetch_blockscout_label(http, &bs_url, &lower).await {
                labels.insert(lower, name);
            }
        }
    }

    labels
}

async fn fetch_etherscan_label(
    http: &reqwest::Client,
    address: &str,
    chain_id: u64,
) -> anyhow::Result<String> {
    let api_key = std::env::var("ETHERSCAN_API_KEY")
        .unwrap_or_else(|_| "YourApiKeyToken".to_string());

    let url = format!(
        "https://api.etherscan.io/v2/api\
         ?module=contract&action=getsourcecode\
         &address={address}&chainid={chain_id}&apikey={api_key}"
    );

    let resp: serde_json::Value = http
        .get(&url)
        .send()
        .await?
        .json()
        .await?;

    let status = resp.get("status").and_then(|s| s.as_str()).unwrap_or("0");
    if status != "1" {
        return Err(anyhow::anyhow!("etherscan failed"));
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

async fn fetch_erc20_name(
    http: &reqwest::Client,
    rpc_url: &str,
    address: &str,
) -> anyhow::Result<String> {
    // ERC20 symbol() selector
    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "method": "eth_call",
        "params": [{"to": address, "data": "0x95d89b41"}, "latest"],
        "id": 1
    });

    let resp: serde_json::Value = http
        .post(rpc_url)
        .json(&body)
        .send()
        .await?
        .json()
        .await?;

    let hex = resp
        .get("result")
        .and_then(|v| v.as_str())
        .filter(|s| *s != "0x" && !s.is_empty())
        .ok_or_else(|| anyhow::anyhow!("no result"))?;

    let data = hex::decode(hex.trim_start_matches("0x"))?;
    let name = decode_abi_string(&data).ok_or_else(|| anyhow::anyhow!("decode failed"))?;
    if name.is_empty() {
        return Err(anyhow::anyhow!("empty name"));
    }
    Ok(name)
}

/// Decode an ABI-encoded `string` return value (offset + length + data).
fn decode_abi_string(data: &[u8]) -> Option<String> {
    if data.len() < 64 {
        return None;
    }
    // bytes 32..64 hold the length as a big-endian u256; we only need the last 8 bytes
    let len_bytes: [u8; 8] = data[56..64].try_into().ok()?;
    let len = u64::from_be_bytes(len_bytes) as usize;
    if data.len() < 64 + len {
        return None;
    }
    String::from_utf8(data[64..64 + len].to_vec()).ok()
}

async fn fetch_blockscout_label(
    http: &reqwest::Client,
    base_url: &str,
    address: &str,
) -> anyhow::Result<String> {
    let base = base_url.trim_end_matches('/');
    let url = format!("{base}/api/v2/addresses/{address}");

    let resp: serde_json::Value = http.get(&url).send().await?.json().await?;

    // Try name tag first
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

    // Fall back to name field
    let name = resp
        .get("name")
        .and_then(|v| v.as_str())
        .filter(|s| !s.is_empty())
        .ok_or_else(|| anyhow::anyhow!("no label"))?
        .to_string();

    Ok(name)
}
