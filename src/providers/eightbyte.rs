use std::collections::HashMap;

use alloy_json_abi::{Event, Function};
use serde::Deserialize;

/// Fetches function and event signatures from an eightbyte-compatible service.
///
/// Endpoint conventions (pass the service base URL via `--eightbyte`):
///   Function : GET <url>/signatures/<4-byte hex selector>
///   Event    : GET <url>/signatures/<32-byte hex topic0>
///   Error    : GET <url>/signatures/<4-byte hex selector>01
///
/// Response format:
///   `{"selector": "0x...", "signatures": ["name(inputs)(outputs)", ...]}`
///
/// Signatures are compact strings in the form `name(inputs)(outputs)`.
/// Multiple entries for the same selector indicate a 4-byte collision.
pub struct EightbyteProvider {
    http: reqwest::Client,
    url: String,
}

#[derive(Deserialize)]
struct EbResponse {
    signatures: Vec<String>,
}

impl EightbyteProvider {
    pub fn new(http: reqwest::Client, url: String) -> Self {
        let url = url.trim_end_matches('/').to_string();
        Self { http, url }
    }

    async fn fetch(&self, selector: &str) -> Vec<String> {
        let url = format!("{}/signatures/{}", self.url, selector);
        log::debug!("eightbyte: GET {}", url);
        match self.http.get(&url).send().await {
            Ok(r) => match r.json::<EbResponse>().await {
                Ok(resp) => resp.signatures,
                Err(e) => {
                    log::debug!("eightbyte: parse failed for {}: {}", selector, e);
                    vec![]
                }
            },
            Err(e) => {
                log::debug!("eightbyte: request failed for {}: {}", selector, e);
                vec![]
            }
        }
    }

    /// Look up function signatures for 4-byte selectors.
    ///
    /// Returns all matching `Function` definitions per selector. Multiple entries for
    /// the same selector represent a 4-byte collision; callers resolve it by trying to
    /// ABI-decode the calldata against each candidate.
    pub async fn lookup_function_selectors(
        &self,
        selectors: &[String],
    ) -> HashMap<String, Vec<Function>> {
        let mut result = HashMap::new();
        for selector in selectors {
            let sigs = self.fetch(selector).await;
            let funcs: Vec<Function> = sigs.iter().filter_map(|s| parse_function_sig(s)).collect();
            if !funcs.is_empty() {
                result.insert(selector.to_lowercase(), funcs);
            }
        }
        result
    }

    /// Look up event definitions for 32-byte topic0 hashes.
    pub async fn lookup_event_topics(&self, topics: &[String]) -> HashMap<String, Event> {
        let mut result = HashMap::new();
        for topic in topics {
            let sigs = self.fetch(topic).await;
            if let Some(event) = sigs.iter().find_map(|s| s.parse::<Event>().ok()) {
                result.insert(topic.to_lowercase(), event);
            }
        }
        result
    }
}

/// Parse an eightbyte-format function signature string into a `Function`.
///
/// The service uses the compact form `"name(inputs)(outputs)"` where the outputs
/// parentheses follow immediately after the inputs. Alloy's parser expects the
/// `returns` keyword, so this converts the format before parsing.
///
/// Examples:
///   `"getReserves()(uint112 _reserve0, uint112 _reserve1, uint32 _blockTimestampLast)"`
///   `"transfer(address _to, uint256 _value)(bool)"`
///   `"decimals()(uint8)"`
fn parse_function_sig(sig: &str) -> Option<Function> {
    let inputs_end = find_paren_close(sig, sig.find('(')?)?;
    let suffix = sig[inputs_end + 1..].trim();
    if suffix.starts_with('(') {
        // Has outputs: convert "name(inputs)(outputs)" → "name(inputs) returns (outputs)"
        format!("{} returns {}", &sig[..=inputs_end], suffix).parse().ok()
    } else {
        sig.parse().ok()
    }
}

/// Find the index of the closing parenthesis that matches the opening one at `open`.
fn find_paren_close(s: &str, open: usize) -> Option<usize> {
    let mut depth = 0usize;
    for (i, c) in s[open..].char_indices() {
        match c {
            '(' => depth += 1,
            ')' => {
                depth -= 1;
                if depth == 0 {
                    return Some(open + i);
                }
            }
            _ => {}
        }
    }
    None
}
