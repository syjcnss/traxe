use anyhow::Result;
use std::io;

use crate::tree::Node;
use super::{Printer, PrintContext};

pub struct JsonPrinter;

impl JsonPrinter {
    pub fn new(_ctx: &PrintContext) -> Self {
        Self
    }
}

impl Printer for JsonPrinter {
    fn print(&self, root: &Node, out: &mut dyn io::Write) -> Result<()> {
        let json = serde_json::to_string_pretty(root)?;
        writeln!(out, "{json}")?;
        Ok(())
    }
}
