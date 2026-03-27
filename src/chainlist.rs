use serde::Deserialize;
use serde_json::Value;

/// An RPC entry in the chainlist JSON — either a plain URL string or an object with a `url` field.
#[derive(Deserialize)]
#[serde(untagged)]
enum RpcEntry {
    Url(String),
    Object { url: String },
}

impl RpcEntry {
    fn url(&self) -> &str {
        match self {
            RpcEntry::Url(s) => s,
            RpcEntry::Object { url } => url,
        }
    }
}

#[derive(Deserialize)]
struct ChainEntry {
    #[serde(rename = "chainId")]
    chain_id: u64,
    #[serde(default)]
    rpc: Vec<RpcEntry>,
}

/// Search chainlist.org for an RPC endpoint for `chain_id` that supports
/// `debug_traceTransaction` for `tx_hash`. Returns the first working URL and the
/// raw trace result JSON (so the caller can avoid a second identical request).
pub async fn find_working_rpc(
    http: &reqwest::Client,
    chain_id: u64,
    tx_hash: &str,
) -> Option<(String, Value)> {
    log::debug!("chainlist: fetching RPC list from chainlist.org");

    let resp = http
        .get("https://chainlist.org/rpcs.json")
        .send()
        .await
        .map_err(|e| log::debug!("chainlist: fetch failed: {}", e))
        .ok()?;

    let entries: Vec<ChainEntry> = resp
        .json()
        .await
        .map_err(|e| log::debug!("chainlist: JSON parse failed: {}", e))
        .ok()?;

    let chain = entries.into_iter().find(|e| e.chain_id == chain_id)?;
    log::debug!("chainlist: found {} RPCs for chain {}", chain.rpc.len(), chain_id);

    for entry in &chain.rpc {
        let url = entry.url();

        // Skip template URLs (require an API key to fill in)
        if url.contains("${") {
            continue;
        }
        // Skip non-HTTP transports (websockets, etc.)
        if !url.starts_with("http://") && !url.starts_with("https://") {
            continue;
        }

        log::debug!("chainlist: probing {} for debug_traceTransaction", url);
        if let Some(trace) = probe_debug_trace(http, url, tx_hash).await {
            log::debug!("chainlist: working RPC found: {}", url);
            return Some((url.to_string(), trace));
        }
    }

    None
}

/// Calls `debug_traceTransaction` on `rpc_url` for `tx_hash`.
/// Returns the trace result JSON on success, or `None` if the call fails or returns an error.
async fn probe_debug_trace(
    http: &reqwest::Client,
    rpc_url: &str,
    tx_hash: &str,
) -> Option<Value> {
    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "method": "debug_traceTransaction",
        "params": [tx_hash, {"tracer": "callTracer", "tracerConfig": {"withLog": true}}]
    });

    let result = http
        .post(rpc_url)
        .json(&body)
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await;

    match result {
        Ok(resp) => {
            if let Ok(json) = resp.json::<Value>().await {
                // Successful only if "result" is present and non-null, with no "error"
                if json.get("error").is_some() {
                    return None;
                }
                let trace = json.get("result")?;
                if trace.is_null() {
                    return None;
                }
                Some(trace.clone())
            } else {
                None
            }
        }
        Err(_) => None,
    }
}
