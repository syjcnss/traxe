mod abi;
mod chains;
mod cli;
mod label;
mod output;
mod trace;
mod types;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Cli, OutputFormat, TraceSource};
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.no_color {
        colored::control::set_override(false);
    }

    // Resolve chain: parse --chain if provided, otherwise require it when --rpc is absent
    let chain = match &cli.chain {
        Some(s) => chains::parse(s)?,
        None => {
            if cli.rpc.is_none() {
                anyhow::bail!(
                    "--chain/-c is required when --rpc is not provided \
                     (e.g. -c ethereum, -c polygon, -c 1)"
                );
            }
            // --rpc provided without --chain: default to Ethereum mainnet for ABI/label lookups
            chains::Chain { chain_id: 1, alchemy_network: Some("eth-mainnet") }
        }
    };
    let chain_id = chain.chain_id;

    // Resolve RPC URL: explicit flag > construct from ALCHEMY_API_KEY + chain
    let rpc_url = cli.rpc.clone().or_else(|| {
        let key = std::env::var("ALCHEMY_API_KEY").ok()?;
        let network = chain.alchemy_network?;
        Some(format!("https://{network}.g.alchemy.com/v2/{key}"))
    });

    let http = reqwest::Client::new();

    // --- 1. Fetch trace ---
    let root = fetch_trace(&cli, &http, rpc_url.as_deref(), chain_id).await?;

    // --- 2. Collect all unique addresses ---
    let addresses = collect_addresses(&root);

    // --- 3. Resolve ABIs ---
    let abis = abi::resolve_abis(&http, &addresses, chain_id).await;

    // --- 4. Resolve labels ---
    let labels = label::resolve_labels(&http, &addresses, chain_id, rpc_url.as_deref()).await;

    // --- 5. Decode + annotate trace ---
    let mut root = root;
    annotate(&mut root, &abis, &labels);
    propagate_delegatecall_names(&mut root);

    // --- 5b. 4-byte selector lookup for any still-unresolved calls ---
    let unresolved = collect_unresolved_selectors(&root);
    if !unresolved.is_empty() {
        let fourbyte = abi::fourbyte::lookup_selectors(&http, &unresolved).await;
        apply_fourbyte_names(&mut root, &fourbyte);
    }

    let native_symbol = chains::native_symbol(chain_id);

    // --- 6. Output ---
    match cli.output {
        OutputFormat::Json => output::json::print(&root)?,
        OutputFormat::Tree => output::tree::print(&root, native_symbol, cli.raw_data, !cli.no_events),
    }

    Ok(())
}

async fn fetch_trace(
    cli: &Cli,
    http: &reqwest::Client,
    rpc_url: Option<&str>,
    chain_id: u64,
) -> Result<types::CallFrame> {
    match &cli.trace_source {
        Some(TraceSource::Rpc) => {
            let url = rpc_url.context("--rpc or ALCHEMY_API_KEY required for RPC trace source")?;
            trace::rpc::fetch(http, url, &cli.tx_hash).await
        }
        Some(TraceSource::Dune) => {
            trace::dune::fetch(http, &cli.tx_hash, chain_id).await
        }
        Some(TraceSource::Blockscout) => {
            let url = cli.blockscout.as_deref()
                .context("--blockscout required for Blockscout trace source")?;
            trace::blockscout::fetch(http, url, &cli.tx_hash).await
        }
        None => {
            // Priority: RPC → Dune → Blockscout → simulate
            if let Some(url) = rpc_url {
                if let Ok(frame) = trace::rpc::fetch(http, url, &cli.tx_hash).await {
                    return Ok(frame);
                }
                eprintln!("RPC trace failed, trying Dune...");
            }

            let dune_key = std::env::var("DUNE_API_KEY").ok();
            if dune_key.is_some() {
                if let Ok(frame) = trace::dune::fetch(http, &cli.tx_hash, chain_id).await {
                    return Ok(frame);
                }
                eprintln!("Dune trace failed, trying Blockscout...");
            }

            if let Some(url) = cli.blockscout.as_deref() {
                if let Ok(frame) = trace::blockscout::fetch(http, url, &cli.tx_hash).await {
                    return Ok(frame);
                }
                eprintln!("Blockscout trace failed, falling back to simulation...");
            }

            eprintln!("Warning: falling back to local simulation — trace may be inaccurate");
            trace::simulate::fetch(http, rpc_url, &cli.tx_hash, chain_id).await
        }
    }
}

fn collect_addresses(frame: &types::CallFrame) -> Vec<String> {
    let mut addrs = Vec::new();
    collect_addresses_inner(frame, &mut addrs);
    // Deduplicate while preserving order
    let mut seen = std::collections::HashSet::new();
    addrs.retain(|a| seen.insert(a.to_lowercase()));
    addrs
}

fn collect_addresses_inner(frame: &types::CallFrame, out: &mut Vec<String>) {
    if let Some(to) = &frame.to {
        out.push(to.clone());
    }
    for log in &frame.logs {
        if !log.address.is_empty() {
            out.push(log.address.clone());
        }
    }
    for child in &frame.calls {
        collect_addresses_inner(child, out);
    }
}

fn precompile_name(addr: &str) -> Option<&'static str> {
    match addr {
        "0x0000000000000000000000000000000000000001" => Some("ecrecover"),
        "0x0000000000000000000000000000000000000002" => Some("sha256"),
        "0x0000000000000000000000000000000000000003" => Some("ripemd160"),
        "0x0000000000000000000000000000000000000004" => Some("identity"),
        "0x0000000000000000000000000000000000000005" => Some("modexp"),
        "0x0000000000000000000000000000000000000006" => Some("ecadd"),
        "0x0000000000000000000000000000000000000007" => Some("ecmul"),
        "0x0000000000000000000000000000000000000008" => Some("ecpairing"),
        "0x0000000000000000000000000000000000000009" => Some("blake2f"),
        "0x000000000000000000000000000000000000000a" => Some("pointEvaluation"),
        _ => None,
    }
}

fn annotate(
    frame: &mut types::CallFrame,
    abis: &HashMap<String, types::ResolvedAbi>,
    labels: &HashMap<String, String>,
) {
    let addr_lower = frame.to.as_deref().map(|a| a.to_lowercase());

    if let Some(addr) = &addr_lower {
        // Apply precompile name first
        if let Some(name) = precompile_name(addr) {
            frame.contract_label = Some(name.to_string());
            // Use the precompile name as the function label too when no selector decoding is done
            if frame.function_name.is_none() {
                frame.function_name = Some(name.to_string());
            }
        }

        // Apply label
        if frame.contract_label.is_none() {
            if let Some(lbl) = labels.get(addr) {
                frame.contract_label = Some(lbl.clone());
            }
        }

        // Apply ABI decoding
        if let Some(resolved) = abis.get(addr) {
            if frame.contract_label.is_none() {
                frame.contract_label = resolved.contract_name.clone();
            }
            if !frame.input.is_empty() && frame.input != "0x" {
                let input_bytes = hex::decode(frame.input.trim_start_matches("0x"))
                    .unwrap_or_default();
                if let Some((name, decoded)) = abi::decode_input(&resolved.abi, &input_bytes) {
                    frame.function_name = Some(name);
                    frame.decoded_input = Some(decoded);
                }
                if let Some(fn_name) = &frame.function_name {
                    if let Some(output_hex) = &frame.output {
                        let output_bytes = hex::decode(output_hex.trim_start_matches("0x"))
                            .unwrap_or_default();
                        if let Some(decoded) =
                            abi::decode_output(&resolved.abi, fn_name, &output_bytes)
                        {
                            frame.decoded_output = Some(decoded);
                        }
                    }
                }
            }
        }

        // Fall back to well-known ERC-20 / ERC-721 call decoding when the address ABI didn't
        // cover this selector. Only applies when the calldata exactly matches the known signature.
        if frame.function_name.is_none() && !frame.input.is_empty() && frame.input != "0x" {
            let input_bytes = hex::decode(frame.input.trim_start_matches("0x")).unwrap_or_default();
            let well_known: &[&dyn Fn() -> alloy_json_abi::JsonAbi] = &[
                &abi::well_known::erc20_abi,
                &abi::well_known::erc721_abi,
            ];
            'wk: for make_abi in well_known {
                let wk_abi = make_abi();
                if let Some((name, decoded)) = abi::decode_input(&wk_abi, &input_bytes) {
                    frame.function_name = Some(name.clone());
                    frame.decoded_input = Some(decoded);
                    if let Some(output_hex) = &frame.output {
                        let output_bytes =
                            hex::decode(output_hex.trim_start_matches("0x")).unwrap_or_default();
                        if let Some(out) = abi::decode_output(&wk_abi, &name, &output_bytes) {
                            frame.decoded_output = Some(out);
                        }
                    }
                    break 'wk;
                }
            }
        }
    }

    // Decode events in logs
    for log in &mut frame.logs {
        let log_addr = log.address.to_lowercase();
        let topics: Vec<Vec<u8>> = log
            .topics
            .iter()
            .map(|t| hex::decode(t.trim_start_matches("0x")).unwrap_or_default())
            .collect();
        let data = hex::decode(log.data.trim_start_matches("0x")).unwrap_or_default();

        // Try the log emitter's own ABI first.
        let decoded = abis.get(&log_addr).and_then(|r| abi::decode_event(&r.abi, &topics, &data));

        // For delegatecall frames the log is emitted by the proxy (log.address) but the event
        // definition lives in the implementation ABI (this frame's `to` address). Fall back to it.
        let decoded = decoded.or_else(|| {
            if frame.call_type == types::CallType::DelegateCall {
                addr_lower.as_ref()
                    .and_then(|a| abis.get(a))
                    .and_then(|r| abi::decode_event(&r.abi, &topics, &data))
            } else {
                None
            }
        });

        // Last resort: try well-known ERC-20 / ERC-721 event ABIs.
        let decoded = decoded
            .or_else(|| abi::decode_event(&abi::well_known::erc20_abi(), &topics, &data))
            .or_else(|| abi::decode_event(&abi::well_known::erc721_abi(), &topics, &data));

        if let Some((name, args)) = decoded {
            log.event_name = Some(name);
            log.decoded_args = Some(args);
        }
    }

    for child in &mut frame.calls {
        annotate(child, abis, labels);
    }
}

/// After annotation, propagate a resolved function name from a DELEGATECALL child up to its
/// parent CALL when the parent's selector is unresolved and the child's selector matches.
/// This handles the proxy pattern where the CALL hits a proxy and the DELEGATECALL hits the
/// implementation that actually has the ABI.
fn propagate_delegatecall_names(frame: &mut types::CallFrame) {
    // Post-order: resolve children first so nested proxies work recursively.
    for child in &mut frame.calls {
        propagate_delegatecall_names(child);
    }

    if frame.function_name.is_some() {
        return;
    }

    let parent_selector = delegatecall_selector(&frame.input);
    if parent_selector.is_none() {
        return;
    }

    let name = frame.calls.iter().find_map(|child| {
        if child.call_type == types::CallType::DelegateCall
            && child.function_name.is_some()
            && delegatecall_selector(&child.input) == parent_selector
        {
            child.function_name.clone()
        } else {
            None
        }
    });

    if let Some(n) = name {
        frame.function_name = Some(n);
    }
}

/// Collect unique selectors (lowercase "0xABCD1234") from frames that have no function name.
fn collect_unresolved_selectors(frame: &types::CallFrame) -> Vec<String> {
    let mut selectors = std::collections::HashSet::new();
    collect_unresolved_selectors_inner(frame, &mut selectors);
    selectors.into_iter().collect()
}

fn collect_unresolved_selectors_inner(
    frame: &types::CallFrame,
    out: &mut std::collections::HashSet<String>,
) {
    if frame.function_name.is_none() {
        let hex = frame.input.trim_start_matches("0x");
        if hex.len() >= 8 {
            out.insert(format!("0x{}", &hex[..8].to_lowercase()));
        }
    }
    for child in &frame.calls {
        collect_unresolved_selectors_inner(child, out);
    }
}

/// Apply 4-byte lookup results to frames that still have no function name.
fn apply_fourbyte_names(frame: &mut types::CallFrame, names: &HashMap<String, String>) {
    if frame.function_name.is_none() {
        let hex = frame.input.trim_start_matches("0x");
        if hex.len() >= 8 {
            let key = format!("0x{}", &hex[..8].to_lowercase());
            if let Some(name) = names.get(&key) {
                frame.function_name = Some(name.clone());
            }
        }
    }
    for child in &mut frame.calls {
        apply_fourbyte_names(child, names);
    }
}

fn delegatecall_selector(input: &str) -> Option<[u8; 4]> {
    let hex = input.trim_start_matches("0x");
    if hex.len() < 8 {
        return None;
    }
    let bytes = hex::decode(&hex[..8]).ok()?;
    bytes.try_into().ok()
}
