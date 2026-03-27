pub mod html;
pub mod json;
pub mod tree;

use anyhow::Result;
use std::io;
use std::path::PathBuf;

use crate::ir::Node;

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
