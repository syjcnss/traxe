use std::collections::HashMap;

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
                // Keep only the function name (strip argument types)
                let name = sig.split('(').next().unwrap_or(sig).to_string();
                result.insert(selector.to_lowercase(), name);
            }
        }
    }

    result
}
