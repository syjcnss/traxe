pub mod html;
pub mod json;
pub mod tree;

use anyhow::Result;
use std::io;

use crate::ir::Node;

pub trait Printer {
    fn print(&self, root: &Node, out: &mut dyn io::Write) -> Result<()>;
}
