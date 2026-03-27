use clap::{Args, Parser};
use std::path::PathBuf;

pub use crate::providers::TraceSource;

#[derive(Parser, Debug)]
#[command(
    name = "traxe",
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
    pub trace_provider: Option<TraceSource>,

    /// Enable debug logging
    #[arg(long, short = 'd')]
    pub debug: bool,

    /// Printer to use for output
    #[arg(long, value_enum, default_value = "text", help_heading = "Printer Options")]
    pub printer: PrinterKind,

    /// Write output to a file instead of stdout
    #[arg(short = 'o', long, value_name = "FILE", help_heading = "Printer Options")]
    pub output: Option<PathBuf>,

    #[command(flatten)]
    pub text: TextConfig,

    #[command(flatten)]
    pub html: HtmlConfig,

    #[command(flatten)]
    pub json: JsonConfig,
}

#[derive(clap::ValueEnum, Debug, Clone, PartialEq)]
pub enum PrinterKind {
    Json,
    Text,
    Html,
}

#[derive(Args, Debug, Clone)]
#[command(next_help_heading = "Text Printer Options")]
pub struct TextConfig {
    /// Show raw call input and return data (hex)
    #[arg(long = "text-raw-data")]
    pub raw_data: bool,

    /// Hide emitted events (logs)
    #[arg(long = "text-no-events")]
    pub no_events: bool,

    /// Show gas usage
    #[arg(long = "text-show-gas")]
    pub show_gas: bool,

    /// Disable colored output
    #[arg(long = "text-no-color")]
    pub no_color: bool,
}

#[derive(Args, Debug, Clone)]
#[command(next_help_heading = "HTML Printer Options")]
pub struct HtmlConfig {}

#[derive(Args, Debug, Clone)]
#[command(next_help_heading = "JSON Printer Options")]
pub struct JsonConfig {}
