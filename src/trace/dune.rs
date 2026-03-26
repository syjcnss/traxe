use anyhow::{anyhow, Context, Result};
use serde_json::{json, Value};
use std::collections::HashMap;
use std::time::Duration;

use crate::types::{CallFrame, CallType};

/// Dune Analytics schema name for a given EVM chain ID.
/// Returns None for chains without a Dune traces table.
fn dune_chain_table(chain_id: u64) -> Option<&'static str> {
    match chain_id {
        // Mainnets
        1 => Some("ethereum"),
        5 => Some("goerli"),
        10 => Some("optimism"),
        14 => Some("flare"),
        56 => Some("bnb"),
        88 => Some("viction"),
        100 => Some("gnosis"),
        130 => Some("unichain"),
        137 => Some("polygon"),
        146 => Some("sonic"),
        196 => Some("xlayer"),
        204 => Some("opbnb"),
        250 => Some("fantom"),
        288 => Some("boba"),
        324 => Some("zksync"),
        360 => Some("shape"),
        480 => Some("worldchain"),
        999 => Some("hyperevm"),
        1101 => Some("zkevm"),
        1329 => Some("sei"),
        1516 => Some("story"),
        2020 => Some("ronin"),
        2741 => Some("abstract"),
        3338 => Some("peaq"),
        5000 => Some("mantle"),
        5330 => Some("superseed"),
        8217 => Some("kaia"),
        8333 => Some("b3"),
        8453 => Some("base"),
        10143 => Some("monad_testnet"),
        11155111 => Some("sepolia"),
        21000000 => Some("corn"),
        33139 => Some("apechain"),
        34443 => Some("mode"),
        42161 => Some("arbitrum"),
        42170 => Some("nova"),
        42220 => Some("celo"),
        43111 => Some("hemi"),
        43114 => Some("avalanche_c"),
        50104 => Some("sophon"),
        57073 => Some("ink"),
        59144 => Some("linea"),
        60808 => Some("bob"),
        80094 => Some("berachain"),
        98865 => Some("plume"),
        167000 => Some("taiko"),
        534352 => Some("scroll"),
        666666666 => Some("degen"),
        _ => None,
    }
}

pub async fn fetch(
    http: &reqwest::Client,
    tx_hash: &str,
    chain_id: u64,
) -> Result<CallFrame> {
    let api_key = std::env::var("DUNE_API_KEY").context("DUNE_API_KEY not set")?;
    let chain = dune_chain_table(chain_id)
        .ok_or_else(|| anyhow!("Dune: no traces table for chain_id {}", chain_id))?;
    // Normalize tx_hash: Dune stores tx_hash as varbinary
    let hash = tx_hash.to_lowercase();

    let sql = format!(
        "SELECT \
            from_hex(substring(cast(tx_hash as varchar), 3)) as tx_hash, \
            cast(\"from\" as varchar) as from_addr, \
            cast(\"to\" as varchar) as to_addr, \
            cast(value as varchar) as value, \
            gas, gas_used, \
            cast(input as varchar) as input, \
            cast(output as varchar) as output, \
            success, error, \
            trace_address, type, call_type \
        FROM {chain}.traces \
        WHERE tx_hash = {hash} \
        ORDER BY trace_address"
    );

    // Execute SQL
    let exec_resp: Value = http
        .post("https://api.dune.com/api/v1/sql/execute")
        .header("X-Dune-Api-Key", &api_key)
        .json(&json!({"sql": sql, "performance": "medium"}))
        .send()
        .await
        .context("Dune execute request failed")?
        .json()
        .await
        .context("Dune execute response parse failed")?;

    let execution_id = exec_resp
        .get("execution_id")
        .and_then(|v| v.as_str())
        .ok_or_else(|| anyhow!("Dune: no execution_id in response: {}", exec_resp))?
        .to_string();

    // Poll for completion
    let results = poll_and_fetch(http, &api_key, &execution_id).await?;

    let rows = results
        .get("result")
        .and_then(|r| r.get("rows"))
        .and_then(|r| r.as_array())
        .ok_or_else(|| anyhow!("Dune: no rows in result"))?;

    if rows.is_empty() {
        return Err(anyhow!("Dune: no traces found for tx {}", tx_hash));
    }

    build_tree(rows)
}

async fn poll_and_fetch(
    http: &reqwest::Client,
    api_key: &str,
    execution_id: &str,
) -> Result<Value> {
    loop {
        let status: Value = http
            .get(format!(
                "https://api.dune.com/api/v1/execution/{execution_id}/status"
            ))
            .header("X-Dune-Api-Key", api_key)
            .send()
            .await
            .context("Dune status request failed")?
            .json()
            .await
            .context("Dune status parse failed")?;

        let state = status
            .get("state")
            .and_then(|s| s.as_str())
            .unwrap_or("");

        match state {
            "QUERY_STATE_COMPLETED" => break,
            "QUERY_STATE_FAILED" | "QUERY_STATE_CANCELLED" | "QUERY_STATE_EXPIRED" => {
                return Err(anyhow!("Dune execution {}: state={}", execution_id, state));
            }
            _ => {
                tokio::time::sleep(Duration::from_secs(2)).await;
            }
        }
    }

    let results: Value = http
        .get(format!(
            "https://api.dune.com/api/v1/execution/{execution_id}/results"
        ))
        .header("X-Dune-Api-Key", api_key)
        .send()
        .await
        .context("Dune results request failed")?
        .json()
        .await
        .context("Dune results parse failed")?;

    Ok(results)
}

/// Reconstruct the nested call tree from Dune's flat trace rows.
/// Each row has `trace_address` = array of indices, e.g. [] (root), [0], [0,0], [1], etc.
fn build_tree(rows: &[Value]) -> Result<CallFrame> {
    // Build a map: trace_address -> row index
    let mut frames: Vec<CallFrame> = rows
        .iter()
        .map(|row| row_to_frame(row))
        .collect::<Result<Vec<_>>>()?;

    // We build the tree bottom-up using trace_address paths.
    // trace_address of a parent is the child's trace_address[..len-1]
    // We'll use index-based approach: sort by trace_address length desc, then insert into parent.

    // Collect trace_addresses
    let addresses: Vec<Vec<usize>> = rows
        .iter()
        .map(|row| parse_trace_address(row))
        .collect();

    // Build index map: trace_address -> index in frames
    let mut addr_to_idx: HashMap<Vec<usize>, usize> = HashMap::new();
    for (i, addr) in addresses.iter().enumerate() {
        addr_to_idx.insert(addr.clone(), i);
    }

    // Process in reverse order (deepest first) to attach children to parents
    // We need a different approach since we can't mutably borrow twice.
    // Build a child map: parent_idx -> Vec<child_idx>
    let mut children: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut root_idx = 0;

    for (i, addr) in addresses.iter().enumerate() {
        if addr.is_empty() {
            root_idx = i;
        } else {
            let parent_addr = &addr[..addr.len() - 1];
            if let Some(&parent_idx) = addr_to_idx.get(parent_addr) {
                children.entry(parent_idx).or_default().push(i);
            }
        }
    }

    // Recursively build the tree
    fn attach_children(
        idx: usize,
        frames: &mut Vec<CallFrame>,
        children: &HashMap<usize, Vec<usize>>,
    ) {
        if let Some(child_indices) = children.get(&idx) {
            let mut sorted = child_indices.clone();
            sorted.sort(); // preserve order
            for &child_idx in &sorted {
                attach_children(child_idx, frames, children);
            }
            // Now move children frames into parent
            let child_frames: Vec<CallFrame> = sorted
                .iter()
                .map(|&ci| frames[ci].clone())
                .collect();
            frames[idx].calls = child_frames;
        }
    }

    attach_children(root_idx, &mut frames, &children);

    Ok(frames.remove(root_idx))
}

fn parse_trace_address(row: &Value) -> Vec<usize> {
    row.get("trace_address")
        .and_then(|v| v.as_array())
        .map(|arr| {
            arr.iter()
                .filter_map(|x| x.as_u64().map(|n| n as usize))
                .collect()
        })
        .unwrap_or_default()
}

fn row_to_frame(row: &Value) -> Result<CallFrame> {
    let str_val = |key: &str| -> String {
        row.get(key)
            .and_then(|v| v.as_str())
            .unwrap_or("")
            .to_string()
    };

    let call_type_str = if str_val("type") == "call" {
        str_val("call_type")
    } else {
        str_val("type")
    };

    let call_type = match call_type_str.to_lowercase().as_str() {
        "call" => CallType::Call,
        "delegatecall" => CallType::DelegateCall,
        "staticcall" => CallType::StaticCall,
        "callcode" => CallType::CallCode,
        "create" | "create2" => CallType::Create,
        _ => CallType::Call,
    };

    let gas = row
        .get("gas")
        .and_then(|v| v.as_i64())
        .map(|n| format!("{:#x}", n))
        .unwrap_or_else(|| "0x0".to_string());

    let gas_used = row
        .get("gas_used")
        .and_then(|v| v.as_i64())
        .map(|n| format!("{:#x}", n))
        .unwrap_or_else(|| "0x0".to_string());

    let success = row
        .get("success")
        .and_then(|v| v.as_bool())
        .unwrap_or(true);

    Ok(CallFrame {
        call_type,
        from: str_val("from_addr"),
        to: {
            let s = str_val("to_addr");
            if s.is_empty() { None } else { Some(s) }
        },
        value: {
            let s = str_val("value");
            if s.is_empty() || s == "0" { None } else { Some(s) }
        },
        gas,
        gas_used,
        input: {
            let s = str_val("input");
            if s.is_empty() { "0x".to_string() } else { s }
        },
        output: {
            let s = str_val("output");
            if s.is_empty() { None } else { Some(s) }
        },
        error: if !success {
            let e = str_val("error");
            if e.is_empty() { Some("reverted".to_string()) } else { Some(e) }
        } else {
            None
        },
        revert_reason: None,
        calls: vec![],
        logs: vec![],
        function_name: None,
        decoded_input: None,
        decoded_output: None,
        contract_label: None,
    })
}
