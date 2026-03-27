use clap::{Args, Parser, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "trace-tx",
    about = "Fetch and visualize EVM transaction call traces",
    version
)]
pub struct Cli {
    /// Transaction hash to trace
    pub tx_hash: String,

    /// RPC endpoint URL (e.g. https://eth-mainnet.g.alchemy.com/v2/<key>)
    #[arg(long)]
    pub rpc: Option<String>,

    /// Blockscout explorer endpoint URL (e.g. https://eth.blockscout.com)
    #[arg(long)]
    pub blockscout: Option<String>,

    /// Chain name or chain ID (e.g. ethereum, polygon, 1, 137).
    /// Required when --rpc is not provided.
    #[arg(short = 'c', long)]
    pub chain: Option<String>,

    /// Force a specific trace provider
    #[arg(long, value_enum)]
    pub trace_provider: Option<TraceProvider>,

    /// Printer to use for output
    #[arg(long, value_enum, default_value = "tree")]
    pub printer: PrinterKind,

    /// Disable colored output
    #[arg(long)]
    pub no_color: bool,

    #[command(flatten)]
    pub tree: TreeArgs,

    /// Write output to a file instead of stdout
    #[arg(short = 'o', long, value_name = "FILE")]
    pub output: Option<PathBuf>,

    /// Enable debug logging
    #[arg(long, short = 'd')]
    pub debug: bool,
}

#[derive(ValueEnum, Debug, Clone, PartialEq)]
pub enum TraceProvider {
    Rpc,
    Dune,
    Blockscout,
    Simulator,
}

#[derive(ValueEnum, Debug, Clone, PartialEq)]
pub enum PrinterKind {
    Json,
    Tree,
    Html,
}

#[derive(Args, Debug)]
pub struct TreeArgs {
    /// Show raw call input and return data (hex) [tree printer only]
    #[arg(long = "tree-raw-data")]
    pub raw_data: bool,

    /// Hide emitted events (logs) [tree printer only]
    #[arg(long = "tree-no-events")]
    pub no_events: bool,
}
