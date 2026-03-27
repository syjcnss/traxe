use clap::{Parser, ValueEnum};

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

    /// Force a specific trace source
    #[arg(long, value_enum)]
    pub trace_source: Option<TraceSource>,

    /// Output format
    #[arg(long, value_enum, default_value = "tree")]
    pub output: OutputFormat,

    /// Disable colored output
    #[arg(long)]
    pub no_color: bool,

    /// Show raw call input and return data (hex) in tree output
    #[arg(long)]
    pub raw_data: bool,

    /// Hide emitted events (logs) in tree output
    #[arg(long)]
    pub no_events: bool,

    /// Enable debug logging
    #[arg(long, short = 'd')]
    pub debug: bool,
}

#[derive(ValueEnum, Debug, Clone, PartialEq)]
pub enum TraceSource {
    Rpc,
    Dune,
    Blockscout,
}

#[derive(ValueEnum, Debug, Clone, PartialEq)]
pub enum OutputFormat {
    Json,
    Tree,
}
