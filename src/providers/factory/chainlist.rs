use std::time::{Duration, SystemTime};

use serde::Deserialize;
use serde_json::Value;

use crate::providers::RpcProvider;

const CACHE_TTL: Duration = Duration::from_secs(7 * 24 * 60 * 60);
const CHAINLIST_URL: &str = "https://chainlist.org/rpcs.json";

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

pub struct Chainlist {
    http: reqwest::Client,
}

/// Returns `~/.cache/traxe/chainlist.json` if HOME is set.
fn cache_path() -> Option<std::path::PathBuf> {
    let home = std::env::var_os("HOME")?;
    let mut path = std::path::PathBuf::from(home);
    path.push(".cache/traxe/chainlist.json");
    Some(path)
}

/// Returns cached bytes if the file exists and is younger than CACHE_TTL.
fn load_fresh_cache(path: &std::path::Path) -> Option<Vec<u8>> {
    let meta = std::fs::metadata(path).ok()?;
    let age = SystemTime::now().duration_since(meta.modified().ok()?).ok()?;
    if age > CACHE_TTL {
        log::debug!("chainlist: cache is {:.0}h old, will refresh", age.as_secs_f64() / 3600.0);
        return None;
    }
    log::debug!("chainlist: using cached chainlist ({:.0}h old)", age.as_secs_f64() / 3600.0);
    std::fs::read(path).ok()
}

/// Saves bytes to the cache path, creating parent dirs as needed. Errors are non-fatal.
fn save_cache(path: &std::path::Path, data: &[u8]) {
    if let Some(parent) = path.parent() {
        let _ = std::fs::create_dir_all(parent);
    }
    if let Err(e) = std::fs::write(path, data) {
        log::debug!("chainlist: failed to write cache: {}", e);
    } else {
        log::debug!("chainlist: saved to cache at {}", path.display());
    }
}

impl Chainlist {
    pub fn new(http: reqwest::Client) -> Self {
        Self { http }
    }

    /// Search chainlist.org for an RPC endpoint for `chain_id` that supports
    /// `debug_traceTransaction` for `tx_hash`. Returns a ready-to-use `RpcProvider`
    /// with the probe response cached so the first `fetch_trace` call is free.
    pub async fn find_working_rpc(&self, chain_id: u64, tx_hash: &str) -> Option<RpcProvider> {
        let cache = cache_path();

        // Try fresh cache first.
        let raw: Vec<u8> = if let Some(bytes) = cache.as_deref().and_then(load_fresh_cache) {
            bytes
        } else {
            log::debug!("chainlist: fetching RPC list from chainlist.org");
            let bytes = self.http
                .get(CHAINLIST_URL)
                .send()
                .await
                .map_err(|e| log::debug!("chainlist: fetch failed: {}", e))
                .ok()?
                .bytes()
                .await
                .map_err(|e| log::debug!("chainlist: read failed: {}", e))
                .ok()?
                .to_vec();

            // Persist to cache (non-fatal on failure).
            if let Some(path) = &cache {
                save_cache(path, &bytes);
            }
            bytes
        };

        let entries: Vec<ChainEntry> = serde_json::from_slice(&raw)
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
            if let Some(trace) = self.probe_debug_trace(url, tx_hash).await {
                log::debug!("chainlist: working RPC found: {}", url);
                let provider = RpcProvider::new(self.http.clone(), url.to_string())
                    .with_cached_trace(tx_hash.to_string(), trace);
                return Some(provider);
            }
        }

        None
    }

    /// Calls `debug_traceTransaction` on `rpc_url` for `tx_hash`.
    /// Returns the trace result JSON on success, or `None` if the call fails or returns an error.
    async fn probe_debug_trace(&self, rpc_url: &str, tx_hash: &str) -> Option<Value> {
        let body = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "debug_traceTransaction",
            "params": [tx_hash, {"tracer": "callTracer", "tracerConfig": {"withLog": true}}]
        });

        let result = self.http
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
}
