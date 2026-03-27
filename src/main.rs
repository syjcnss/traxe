mod abi;
mod chainlist;
mod chains;
mod cli;
mod ir;
mod label;
mod printer;
mod provider;
mod trace;
mod types;

use anyhow::{Context, Result};
use clap::Parser;
use cli::{Cli, PrinterKind, TraceProvider};
use std::path::PathBuf;
use printer::Printer;
use std::collections::HashMap;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.debug {
        std::env::set_var("RUST_LOG", "trace_tx=debug");
    }
    env_logger::init();

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

    let http = reqwest::Client::new();

    // Resolve RPC URL (always required): --rpc > ALCHEMY_API_KEY > chainlist
    // When chainlist is used, it probes via debug_traceTransaction and returns the cached trace.
    let (rpc_url, cached_trace) =
        resolve_rpc_url(&http, cli.rpc.as_deref(), &chain, &cli.tx_hash).await?;

    // If --chain was not provided, query the RPC for the actual chain ID.
    let chain_id = if cli.chain.is_none() {
        let id = trace::rpc::fetch_chain_id(&http, &rpc_url).await
            .context("failed to fetch chain ID from RPC; use --chain/-c to specify it")?;
        log::debug!("chain ID from RPC: {}", id);
        id
    } else {
        chain_id
    };

    // Build Providers and report what is available.
    let providers = provider::Providers::new(Some(rpc_url), cli.blockscout.clone());
    providers.print_enabled();

    // --- 1. Fetch trace ---
    log::debug!("fetching trace for {} on chain {}", cli.tx_hash, chain_id);
    let root = if let (Some(frame), None | Some(TraceProvider::Rpc)) =
        (&cached_trace, &cli.trace_provider)
    {
        log::debug!("using cached debug_traceTransaction result from chainlist probe");
        frame.clone()
    } else {
        fetch_trace(&cli, &http, &providers, chain_id).await?
    };

    // --- 2. Collect all unique addresses ---
    let addresses = collect_addresses(&root);
    log::debug!("collected {} unique addresses: {}", addresses.len(), addresses.join(", "));

    // --- 3. Resolve ABIs ---
    log::debug!("resolving ABIs for {} addresses", addresses.len());
    let abis = abi::resolve_abis(&http, &addresses, chain_id, &providers).await;
    log::debug!("resolved {} ABIs", abis.len());

    // --- 4. Resolve labels ---
    log::debug!("resolving labels");
    let labels = label::resolve_labels(&http, &addresses, chain_id, &providers).await;
    log::debug!("resolved {} labels", labels.len());

    // --- 5. Decode + annotate trace ---
    let mut root = root;
    annotate(&mut root, &abis, &labels);
    propagate_delegatecall_names(&mut root);

    // --- 5b. 4-byte selector lookup for any still-unresolved calls ---
    let unresolved = collect_unresolved_selectors(&root);
    log::debug!("{} unresolved selectors, querying 4byte", unresolved.len());
    if !unresolved.is_empty() {
        let fourbyte = abi::fourbyte::lookup_selectors(&http, &unresolved).await;
        log::debug!("4byte resolved {} selectors", fourbyte.len());
        apply_fourbyte_names(&mut root, &fourbyte);
    }

    let native_symbol = chains::native_symbol(chain_id);

    // --- 6. Build IR and print ---
    let ir_root = ir::Node::from(root);
    let p: Box<dyn Printer> = match cli.printer {
        PrinterKind::Json => Box::new(printer::json::JsonPrinter),
        PrinterKind::Tree => Box::new(printer::tree::TreePrinter::new(
            native_symbol.to_string(),
            &cli.tree,
        )),
        PrinterKind::Html => Box::new(printer::html::HtmlPrinter {
            tx_hash: cli.tx_hash.clone(),
            native_symbol: native_symbol.to_string(),
        }),
    };

    // For the HTML printer, default to <tx_hash>.html when no -o is given.
    let output_path: Option<PathBuf> = cli.output.clone().or_else(|| {
        if cli.printer == PrinterKind::Html {
            Some(PathBuf::from(format!("{}.html", cli.tx_hash)))
        } else {
            None
        }
    });

    if let Some(path) = &output_path {
        // Disable colors when writing to a file (unless already forced off)
        if !cli.no_color {
            colored::control::set_override(false);
        }
        let mut file = std::fs::File::create(path)
            .with_context(|| format!("failed to create output file: {}", path.display()))?;
        p.print(&ir_root, &mut file)?;
        if cli.printer == PrinterKind::Html {
            eprintln!("HTML trace written to {}", path.display());
        }
    } else {
        p.print(&ir_root, &mut std::io::stdout())?;
    }

    Ok(())
}

async fn fetch_trace(
    cli: &Cli,
    http: &reqwest::Client,
    providers: &provider::Providers,
    chain_id: u64,
) -> Result<types::CallFrame> {
    match &cli.trace_provider {
        Some(TraceProvider::Rpc) => {
            let url = providers.rpc_url.as_deref()
                .context("--rpc or ALCHEMY_API_KEY required for rpc trace provider")?;
            log::debug!("trace provider: rpc ({})", url);
            trace::rpc::fetch(http, url, &cli.tx_hash).await
        }
        Some(TraceProvider::Dune) => {
            log::debug!("trace provider: dune");
            let mut frame = trace::dune::fetch(http, &cli.tx_hash, chain_id).await?;
            populate_dune_logs(http, providers, &mut frame, chain_id, &cli.tx_hash).await;
            Ok(frame)
        }
        Some(TraceProvider::Blockscout) => {
            let url = providers.blockscout_url.as_deref()
                .context("--blockscout or BLOCKSCOUT_URL required for blockscout trace provider")?;
            log::debug!("trace provider: blockscout ({})", url);
            trace::blockscout::fetch(http, url, &cli.tx_hash).await
        }
        Some(TraceProvider::Simulator) => {
            log::debug!("trace provider: simulator");
            trace::simulate::fetch(http, providers.rpc_url.as_deref(), &cli.tx_hash, chain_id).await
        }
        None => {
            // Auto-select: RPC → Dune → Blockscout → simulate
            if let Some(url) = &providers.rpc_url {
                log::debug!("trying rpc trace ({})", url);
                match trace::rpc::fetch(http, url, &cli.tx_hash).await {
                    Ok(frame) => {
                        log::debug!("rpc trace succeeded");
                        return Ok(frame);
                    }
                    Err(e) => {
                        log::debug!("rpc trace failed: {}", e);
                        eprintln!("RPC trace failed, trying Dune...");
                    }
                }
            }

            if providers.dune_key.is_some() {
                log::debug!("trying dune trace");
                match trace::dune::fetch(http, &cli.tx_hash, chain_id).await {
                    Ok(mut frame) => {
                        log::debug!("dune trace succeeded");
                        populate_dune_logs(http, providers, &mut frame, chain_id, &cli.tx_hash)
                            .await;
                        return Ok(frame);
                    }
                    Err(e) => {
                        log::debug!("dune trace failed: {}", e);
                        eprintln!("Dune trace failed, trying Blockscout...");
                    }
                }
            }

            if let Some(url) = &providers.blockscout_url {
                log::debug!("trying blockscout trace ({})", url);
                match trace::blockscout::fetch(http, url, &cli.tx_hash).await {
                    Ok(frame) => {
                        log::debug!("blockscout trace succeeded");
                        return Ok(frame);
                    }
                    Err(e) => {
                        log::debug!("blockscout trace failed: {}", e);
                        eprintln!("Blockscout trace failed, falling back to simulation...");
                    }
                }
            }

            eprintln!("Warning: falling back to local simulation — trace may be inaccurate");
            log::debug!("falling back to simulate");
            trace::simulate::fetch(http, providers.rpc_url.as_deref(), &cli.tx_hash, chain_id).await
        }
    }
}

/// After a Dune trace fetch (which contains no event logs), populate logs by
/// running the local simulator and copying logs from its output into the
/// corresponding Dune call frames.
async fn populate_dune_logs(
    http: &reqwest::Client,
    providers: &provider::Providers,
    root: &mut types::CallFrame,
    chain_id: u64,
    tx_hash: &str,
) {
    if let Some(rpc_url) = &providers.rpc_url {
        log::debug!("dune: populating event logs via simulation");
        match trace::simulate::fetch(http, Some(rpc_url), tx_hash, chain_id).await {
            Ok(sim_root) => {
                trace::event_position::position_from_simulation(root, &sim_root);
                log::debug!("dune: event logs populated via simulation");
            }
            Err(e) => {
                log::debug!("dune: simulation failed ({}), skipping event log population", e);
            }
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
        // Apply precompile name and decode its input/output
        if let Some(pc_name) = precompile_name(addr) {
            frame.contract_label = Some(pc_name.to_string());
            if frame.function_name.is_none() {
                frame.function_name = Some(pc_name.to_string());
            }
            if frame.decoded_input.is_none() && !frame.input.is_empty() && frame.input != "0x" {
                if let Some(pc_abi) = abi::well_known::precompile_abi(addr) {
                    let input_bytes =
                        hex::decode(frame.input.trim_start_matches("0x")).unwrap_or_default();
                    if let Some(decoded) = abi::decode_raw_input(&pc_abi, pc_name, &input_bytes) {
                        frame.decoded_input = Some(decoded);
                        if let Some(output_hex) = &frame.output {
                            let output_bytes =
                                hex::decode(output_hex.trim_start_matches("0x")).unwrap_or_default();
                            if let Some(out) =
                                abi::decode_output(&pc_abi, pc_name, &output_bytes)
                            {
                                frame.decoded_output = Some(out);
                            }
                        }
                    }
                }
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

/// Resolve a working RPC URL in priority order:
/// 1. Explicit `--rpc` flag
/// 2. `ALCHEMY_API_KEY` env var (requires a known Alchemy network slug for the chain)
/// 3. Chainlist.org — probes each public RPC via `debug_traceTransaction`; returns the URL
///    plus the cached trace result so the caller can skip a second identical request.
///
/// Returns `(rpc_url, cached_trace)`. `cached_trace` is `Some` only when chainlist was used.
async fn resolve_rpc_url(
    http: &reqwest::Client,
    cli_rpc: Option<&str>,
    chain: &chains::Chain,
    tx_hash: &str,
) -> Result<(String, Option<types::CallFrame>)> {
    // 1. Explicit --rpc flag
    if let Some(url) = cli_rpc {
        return Ok((url.to_string(), None));
    }

    // 2. ALCHEMY_API_KEY env var
    if let Ok(key) = std::env::var("ALCHEMY_API_KEY") {
        if let Some(network) = chain.alchemy_network {
            return Ok((format!("https://{network}.g.alchemy.com/v2/{key}"), None));
        }
    }

    // 3. Chainlist.org fallback — probe via debug_traceTransaction and cache the result.
    eprintln!("No RPC configured — searching chainlist.org for a public RPC endpoint...");
    if let Some((url, trace_json)) =
        chainlist::find_working_rpc(http, chain.chain_id, tx_hash).await
    {
        eprintln!("Using RPC from chainlist: {url}");
        let cached = trace::rpc::parse_call_tracer_frame(&trace_json)
            .map_err(|e| log::debug!("chainlist: failed to parse cached trace: {}", e))
            .ok();
        return Ok((url, cached));
    }

    anyhow::bail!(
        "No working RPC found for chain {}. \
         Provide --rpc <url>, set ALCHEMY_API_KEY, or ensure chainlist.org lists an \
         accessible RPC for this chain.",
        chain.chain_id
    )
}

fn delegatecall_selector(input: &str) -> Option<[u8; 4]> {
    let hex = input.trim_start_matches("0x");
    if hex.len() < 8 {
        return None;
    }
    let bytes = hex::decode(&hex[..8]).ok()?;
    bytes.try_into().ok()
}
