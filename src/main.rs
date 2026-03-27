use anyhow::{Context, Result};
use clap::Parser;
use traxe::chains;
use traxe::cli::{Cli, PrinterKind};
use traxe::printers::{self, Printer};
use traxe::providers::ProviderFactory;
use traxe::tree;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    if cli.debug {
        std::env::set_var("RUST_LOG", "traxe=debug");
    }
    env_logger::init();

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

    let factory = ProviderFactory::new()
        .rpc(cli.rpc.clone())
        .blockscout(cli.blockscout.clone())
        .eightbyte(cli.eightbyte.clone())
        .trace_source(cli.trace_provider.clone());

    // Build the RPC provider: --rpc > ALCHEMY_API_KEY > chainlist.
    // When chainlist is used the provider has a pre-seeded trace cache.
    let rpc = factory.build_rpc(&chain, &cli.tx_hash).await?;

    // If --chain was not provided, query the RPC for the actual chain ID.
    let chain_id = if cli.chain.is_none() {
        let id = rpc
            .chain_id()
            .await
            .context("failed to fetch chain ID from RPC; use --chain/-c to specify it")?;
        log::debug!("chain ID from RPC: {}", id);
        id
    } else {
        chain_id
    };

    let pm = factory.build_manager(rpc, chain_id, cli.tx_hash.clone());
    pm.print_enabled();

    // Build the fully annotated tree (fetch → enrich → annotate).
    let tree_root = tree::build(&pm).await?;

    let ctx = printers::PrintContext {
        tx_hash: cli.tx_hash.clone(),
        native_symbol: chains::native_symbol(chain_id).to_string(),
        config: printers::PrinterConfig { text: cli.text },
    };

    let p: Box<dyn Printer> = match cli.printer {
        PrinterKind::Json => Box::new(printers::json::JsonPrinter::new(&ctx)),
        PrinterKind::Text => Box::new(printers::text::TextPrinter::new(&ctx)),
        PrinterKind::Html => Box::new(printers::html::HtmlPrinter::new(&ctx)),
    };

    let output_path = cli.output.clone().or_else(|| {
        if p.print_to_file() { p.default_path() } else { None }
    });
    if let Some(ref path) = output_path {
        colored::control::set_override(false);
        let mut file = std::fs::File::create(path)
            .with_context(|| format!("failed to create output file: {}", path.display()))?;
        p.print(&tree_root, &mut file)?;
        log::debug!("written to {}", path.display());
    } else {
        p.print(&tree_root, &mut std::io::stdout())?;
    }

    Ok(())
}
