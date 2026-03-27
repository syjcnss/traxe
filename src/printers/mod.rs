pub mod html;
pub mod json;
pub mod text;

use anyhow::Result;
use std::io;
use std::path::PathBuf;

use crate::cli::TextConfig;
use crate::tree::Node;

pub struct PrinterConfig {
    pub text: TextConfig,
}

/// Common context available to all printers.
pub struct PrintContext {
    pub tx_hash: String,
    pub native_symbol: String,
    pub config: PrinterConfig,
}

pub trait Printer {
    fn print(&self, root: &Node, out: &mut dyn io::Write) -> Result<()>;

    /// Whether this printer writes to a file by default (when -o is not specified).
    fn print_to_file(&self) -> bool {
        false
    }

    /// Default output path when `print_to_file` is true and no explicit path is given.
    fn default_path(&self) -> Option<PathBuf> {
        None
    }

}
