use anyhow::Result;
use colored::Colorize;
use std::io;

use crate::cli::TreeArgs;
use crate::ir::{CallNode, EventNode, Node};
use crate::types::{CallType, DecodedArg};
use super::Printer;

pub struct TreePrinter {
    native_symbol: String,
    raw_data: bool,
    show_events: bool,
}

impl TreePrinter {
    pub fn new(native_symbol: String, args: &TreeArgs) -> Self {
        Self {
            native_symbol,
            raw_data: args.raw_data,
            show_events: !args.no_events,
        }
    }
}

impl Printer for TreePrinter {
    fn print(&self, root: &Node, out: &mut dyn io::Write) -> Result<()> {
        if let Node::Call(call) = root {
            writeln!(out, "{}", call.from.bright_white().bold())?;
        }
        print_node(out, root, "", true, &self.native_symbol, self.raw_data, self.show_events)?;
        Ok(())
    }
}

fn print_node(out: &mut dyn io::Write, node: &Node, prefix: &str, is_last: bool, native_symbol: &str, raw_data: bool, show_events: bool) -> Result<()> {
    match node {
        Node::Call(call) => print_call(out, call, prefix, is_last, native_symbol, raw_data, show_events),
        Node::Event(event) => print_event(out, event, prefix, is_last),
    }
}

fn print_call(out: &mut dyn io::Write, call: &CallNode, prefix: &str, is_last: bool, native_symbol: &str, raw_data: bool, show_events: bool) -> Result<()> {
    let (branch, pipe) = if is_last { ("└─ ", "   ") } else { ("├─ ", "│  ") };
    let connector = format!("{}{}", prefix.bright_black(), branch.bright_black());
    let child_prefix = format!("{}{}", prefix, pipe);

    let value_str = format_value(call, native_symbol);
    let gas_str = format_gas(call);
    let error_str = format_error(call);

    let input = call.input.trim_start_matches("0x");
    let selector = if input.len() >= 8 {
        Some(format!("0x{}", &input[..8]))
    } else {
        None
    };

    write!(out, "{connector}")?;
    write!(out, "{}", color_call_type(&call.call_type))?;
    write!(out, " {}", format_target_colored(call))?;
    if let Some(name) = &call.function_name {
        write!(out, " {}", "·".bright_black())?;
        write!(out, " {}", name.bright_green().bold())?;
        if let Some(sel) = &selector {
            write!(out, " {}", format!("[{sel}]").bright_black())?;
        }
    } else if let Some(sel) = &selector {
        write!(out, " {} {}", "·".bright_black(), sel.bright_green().bold())?;
    }
    if !value_str.is_empty() {
        write!(out, " {}", value_str.bright_magenta().bold())?;
    }
    if !gas_str.is_empty() {
        write!(out, " {}", gas_str.bright_black())?;
    }
    if !error_str.is_empty() {
        write!(out, " {}", error_str.bright_red().bold())?;
    }
    writeln!(out)?;

    // Whether there are any sub-call children (not counting events, which come after).
    // Used for tree connector direction on input/output sections, matching original behavior.
    let has_call_children = call.children.iter().any(|n| matches!(n, Node::Call(_)));

    // Print decoded input args (or raw input)
    let has_decoded_input = call.decoded_input.as_ref().map_or(false, |a| !a.is_empty());
    if raw_data && !call.input.is_empty() && call.input != "0x" {
        let after = !has_decoded_input
            && !has_call_children
            && call.decoded_output.is_none()
            && call.output.is_none();
        print_raw_data(out, &call.input, &child_prefix, "input", after)?;
    }
    if let Some(args) = &call.decoded_input {
        if !args.is_empty() {
            let is_last_item = !has_call_children
                && call.decoded_output.is_none()
                && (!raw_data || call.output.is_none());
            print_args(out, args, &child_prefix, "inputs", is_last_item)?;
        }
    }

    // Print decoded output (or raw output)
    let raw_output =
        raw_data && call.output.as_deref().map_or(false, |o| !o.is_empty() && o != "0x");
    if let Some(args) = &call.decoded_output {
        if !args.is_empty() {
            print_args(out, args, &child_prefix, "outputs", !has_call_children && !raw_output)?;
        }
    }
    if raw_output {
        if let Some(output) = &call.output {
            print_raw_data(out, output, &child_prefix, "output", !has_call_children)?;
        }
    }

    // Print children: all sub-calls, then events (if enabled).
    let visible: Vec<&Node> = call
        .children
        .iter()
        .filter(|n| matches!(n, Node::Call(_)) || show_events)
        .collect();
    let total = visible.len();
    for (i, child) in visible.iter().enumerate() {
        print_node(out, child, &child_prefix, i + 1 == total, native_symbol, raw_data, show_events)?;
    }
    Ok(())
}

fn print_event(out: &mut dyn io::Write, event: &EventNode, prefix: &str, is_last: bool) -> Result<()> {
    let (branch, pipe) = if is_last { ("└─ ", "   ") } else { ("├─ ", "│  ") };
    let connector = format!("{}{}", prefix.bright_black(), branch.bright_black());
    let child_prefix = format!("{prefix}{pipe}");

    write!(out, "{connector}")?;
    write!(out, "{}", "EVENT".bright_yellow().bold())?;

    let event_label = if let Some(name) = &event.event_name {
        name.bright_green().bold().to_string()
    } else if let Some(topic0) = event.topics.first() {
        format!("0x{}", topic0.trim_start_matches("0x"))
            .bright_black()
            .to_string()
    } else {
        String::new()
    };

    if !event_label.is_empty() {
        write!(out, " {} {}", "·".bright_black(), event_label)?;
    }
    writeln!(out)?;

    let has_decoded = event.decoded_args.as_ref().map_or(false, |a| !a.is_empty());
    if has_decoded {
        if let Some(args) = &event.decoded_args {
            print_args(out, args, &child_prefix, "args", true)?;
        }
    } else {
        let has_data = !event.data.is_empty() && event.data != "0x";
        let remaining_topics = event.topics.get(1..).unwrap_or_default();
        let topic_count = remaining_topics.len();
        for (i, topic) in remaining_topics.iter().enumerate() {
            print_raw_data(out, topic, &child_prefix, &format!("topic{}", i + 1), i + 1 == topic_count && !has_data)?;
        }
        if has_data {
            print_raw_data(out, &event.data, &child_prefix, "data", true)?;
        }
    }
    Ok(())
}

fn print_raw_data(out: &mut dyn io::Write, hex: &str, prefix: &str, label: &str, is_last: bool) -> Result<()> {
    let (branch, pipe) = if is_last { ("└─ ", "   ") } else { ("├─ ", "│  ") };
    let connector = format!("{}{}", prefix.bright_black(), branch.bright_black());
    let sub_prefix = format!("{prefix}{pipe}");

    writeln!(out, "{}{}", connector, label.bright_black().italic())?;
    let con = format!("{}{}", sub_prefix.bright_black(), "└─ ".bright_black());
    writeln!(out, "{}{}", con, hex.bright_black())?;
    Ok(())
}

fn color_call_type(ct: &CallType) -> colored::ColoredString {
    match ct {
        CallType::Call => "CALL".cyan().bold(),
        CallType::DelegateCall => "DELEGATECALL".blue().bold(),
        CallType::StaticCall => "STATICCALL".green().bold(),
        CallType::CallCode => "CALLCODE".purple().bold(),
        CallType::Create => "CREATE".yellow().bold(),
        CallType::Create2 => "CREATE2".bright_yellow().bold(),
    }
}

fn format_target_colored(call: &CallNode) -> String {
    let addr = call.to.as_deref().unwrap_or(&call.from);
    if let Some(label) = &call.contract_label {
        format!(
            "{} {}",
            label.bright_white().bold(),
            format!("({addr})").bright_black()
        )
    } else {
        addr.bright_yellow().to_string()
    }
}

fn format_value(call: &CallNode, native_symbol: &str) -> String {
    if let Some(val) = &call.value {
        if val != "0x0" && val != "0" {
            let wei_opt = if val.starts_with("0x") || val.starts_with("0X") {
                u128::from_str_radix(
                    val.trim_start_matches("0x").trim_start_matches("0X"),
                    16,
                )
                .ok()
            } else {
                val.parse::<u128>().ok()
            };
            if let Some(wei) = wei_opt {
                if wei > 0 {
                    let native = wei as f64 / 1e18;
                    return format!("[{native:.6} {native_symbol}]");
                }
            }
        }
    }
    String::new()
}

fn format_gas(call: &CallNode) -> String {
    let used = parse_hex_u64(&call.gas_used);
    format!("[gas {used}]")
}

fn format_error(call: &CallNode) -> String {
    if let Some(err) = &call.error {
        return format!("REVERT({err})");
    }
    String::new()
}

fn print_args(out: &mut dyn io::Write, args: &[DecodedArg], prefix: &str, label: &str, is_last: bool) -> Result<()> {
    let (branch, pipe) = if is_last { ("└─ ", "   ") } else { ("├─ ", "│  ") };
    let connector = format!("{}{}", prefix.bright_black(), branch.bright_black());
    let sub_prefix_plain = format!("{prefix}{pipe}");

    writeln!(out, "{}{}", connector, label.bright_black().italic())?;

    let n = args.len();
    for (i, arg) in args.iter().enumerate() {
        let last = i + 1 == n;
        let (b2, _) = if last { ("└─ ", "   ") } else { ("├─ ", "│  ") };
        let con = format!("{}{}", sub_prefix_plain.bright_black(), b2.bright_black());
        let name_part = if arg.name.is_empty() || arg.name == "_" {
            format!("({})", arg.ty.bright_black())
        } else {
            format!("{} {}", arg.name.white().bold(), format!("({})", arg.ty).bright_black())
        };
        let val = color_value(truncate_value(&arg.value, 120));
        writeln!(out, "{con}{name_part}: {val}")?;
    }
    Ok(())
}

fn color_value(s: String) -> colored::ColoredString {
    if s.starts_with("0x") && s.len() == 42 && s[2..].chars().all(|c| c.is_ascii_hexdigit()) {
        return s.yellow();
    }
    if s == "true" {
        return s.bright_green();
    }
    if s == "false" {
        return s.bright_red();
    }
    if s.starts_with("0x") {
        return s.bright_black();
    }
    if s.chars().next().map_or(false, |c| c.is_ascii_digit() || c == '-')
        && s.chars().skip(1).all(|c| c.is_ascii_digit() || c == '.' || c == '_')
    {
        return s.bright_cyan();
    }
    s.normal()
}

fn truncate_value(s: &str, max: usize) -> String {
    if s.len() > max {
        format!("{}…", &s[..max])
    } else {
        s.to_string()
    }
}

fn parse_hex_u64(s: &str) -> u64 {
    let hex = s.trim_start_matches("0x");
    u64::from_str_radix(hex, 16).unwrap_or_else(|_| s.parse().unwrap_or(0))
}
