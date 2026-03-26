use anyhow::Result;
use crate::types::CallFrame;

pub fn print(frame: &CallFrame) -> Result<()> {
    let json = serde_json::to_string_pretty(frame)?;
    println!("{json}");
    Ok(())
}
