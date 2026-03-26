use colored::Colorize;

use crate::types::{CallFrame, CallType, DecodedArg, Log};

pub fn print(frame: &CallFrame, native_symbol: &str, raw_data: bool, show_events: bool) {
    // Print the transaction sender as the tree root
    println!("{}", frame.from.bright_white().bold());
    print_frame(frame, "", true, native_symbol, raw_data, show_events);
}

fn print_frame(frame: &CallFrame, prefix: &str, is_last: bool, native_symbol: &str, raw_data: bool, show_events: bool) {
    let (branch, pipe) = if is_last {
        ("└─ ", "   ")
    } else {
        ("├─ ", "│  ")
    };

    let connector = format!("{}{}", prefix.bright_black(), branch.bright_black());
    let child_prefix = format!("{}{}", prefix, pipe);

    // Build header line
    let value_str = format_value(frame, native_symbol);
    let gas_str = format_gas(frame);
    let error_str = format_error(frame);

    let input = frame.input.trim_start_matches("0x");
    let selector = if input.len() >= 8 {
        Some(format!("0x{}", &input[..8]))
    } else {
        None
    };

    print!("{connector}");
    print!("{}", color_call_type(&frame.call_type));
    print!(" {}", format_target_colored(frame));
    if let Some(name) = &frame.function_name {
        print!(" {}", "·".bright_black());
        print!(" {}", name.bright_green().bold());
        if let Some(sel) = &selector {
            print!(" {}", format!("[{sel}]").bright_black());
        }
    } else if let Some(sel) = &selector {
        print!(" {} {}", "·".bright_black(), sel.bright_green().bold());
    }
    if !value_str.is_empty() {
        print!(" {}", value_str.bright_magenta().bold());
    }
    if !gas_str.is_empty() {
        print!(" {}", gas_str.bright_black());
    }
    if !error_str.is_empty() {
        print!(" {}", error_str.bright_red().bold());
    }
    println!();

    // Print decoded input args (or raw input)
    let has_decoded_input = frame.decoded_input.as_ref().map_or(false, |a| !a.is_empty());
    if raw_data && !frame.input.is_empty() && frame.input != "0x" {
        let after = !has_decoded_input && frame.calls.is_empty() && frame.decoded_output.is_none() && frame.output.is_none();
        print_raw_data(&frame.input, &child_prefix, "input", after);
    }
    if let Some(args) = &frame.decoded_input {
        if !args.is_empty() {
            print_args(args, &child_prefix, "inputs", frame.calls.is_empty() && frame.decoded_output.is_none() && (!raw_data || frame.output.is_none()));
        }
    }

    // Print decoded output (or raw output)
    let raw_output = raw_data
        && frame.output.as_deref().map_or(false, |o| !o.is_empty() && o != "0x");
    if let Some(args) = &frame.decoded_output {
        if !args.is_empty() {
            let after_output = frame.calls.is_empty() && !raw_output;
            print_args(args, &child_prefix, "outputs", after_output);
        }
    }
    if raw_output {
        if let Some(output) = &frame.output {
            print_raw_data(output, &child_prefix, "output", frame.calls.is_empty());
        }
    }

    // Print children
    let log_count = if show_events { frame.logs.len() } else { 0 };
    let total = frame.calls.len() + log_count;
    for (i, child) in frame.calls.iter().enumerate() {
        let last = i + 1 == total;
        print_frame(child, &child_prefix, last, native_symbol, raw_data, show_events);
    }

    // Print logs after sub-calls
    if show_events {
        let call_count = frame.calls.len();
        for (i, log) in frame.logs.iter().enumerate() {
            let last = call_count + i + 1 == total;
            print_log(log, &child_prefix, last);
        }
    }
}

fn print_log(log: &Log, prefix: &str, is_last: bool) {
    let (branch, pipe) = if is_last { ("└─ ", "   ") } else { ("├─ ", "│  ") };
    let connector = format!("{}{}", prefix.bright_black(), branch.bright_black());
    let child_prefix = format!("{prefix}{pipe}");

    print!("{connector}");
    print!("{}", "EVENT".bright_yellow().bold());

    // Show event name or topic0
    let event_label = if let Some(name) = &log.event_name {
        name.bright_green().bold().to_string()
    } else if let Some(topic0) = log.topics.first() {
        let t = topic0.trim_start_matches("0x");
        format!("0x{t}").bright_black().to_string()
    } else {
        String::new()
    };

    if !event_label.is_empty() {
        print!(" {} {}", "·".bright_black(), event_label);
    }

    println!();

    // Print decoded args (or raw topics/data)
    let has_decoded = log.decoded_args.as_ref().map_or(false, |a| !a.is_empty());
    if has_decoded {
        if let Some(args) = &log.decoded_args {
            print_args(args, &child_prefix, "args", true);
        }
    } else {
        // Not resolved — show all topics and data
        let has_data = !log.data.is_empty() && log.data != "0x";
        let remaining_topics = log.topics.get(1..).unwrap_or_default();
        let topic_count = remaining_topics.len();
        for (i, topic) in remaining_topics.iter().enumerate() {
            let is_last = i + 1 == topic_count && !has_data;
            print_raw_data(topic, &child_prefix, &format!("topic{}", i + 1), is_last);
        }
        if has_data {
            print_raw_data(&log.data, &child_prefix, "data", true);
        }
    }
}

fn print_raw_data(hex: &str, prefix: &str, label: &str, is_last: bool) {
    let (branch, pipe) = if is_last { ("└─ ", "   ") } else { ("├─ ", "│  ") };
    let connector = format!("{}{}", prefix.bright_black(), branch.bright_black());
    let sub_prefix = format!("{prefix}{pipe}");

    println!("{}{}", connector, label.bright_black().italic());

    let con = format!("{}{}", sub_prefix.bright_black(), "└─ ".bright_black());
    println!("{}{}", con, hex.bright_black());
}

fn color_call_type(ct: &CallType) -> colored::ColoredString {
    match ct {
        CallType::Call => "CALL".cyan().bold(),
        CallType::DelegateCall => "DELEGATECALL".blue().bold(),
        CallType::StaticCall => "STATICCALL".bright_black().bold(),
        CallType::CallCode => "CALLCODE".purple().bold(),
        CallType::Create => "CREATE".yellow().bold(),
        CallType::Create2 => "CREATE2".bright_yellow().bold(),
    }
}

fn format_target_colored(frame: &CallFrame) -> String {
    let addr = frame.to.as_deref().unwrap_or(&frame.from);
    if let Some(label) = &frame.contract_label {
        format!(
            "{} {}",
            label.bright_white().bold(),
            format!("({addr})").bright_black()
        )
    } else {
        addr.bright_yellow().to_string()
    }
}


fn format_value(frame: &CallFrame, native_symbol: &str) -> String {
    if let Some(val) = &frame.value {
        if val != "0x0" && val != "0" {
            let wei_opt = if val.starts_with("0x") || val.starts_with("0X") {
                u128::from_str_radix(val.trim_start_matches("0x").trim_start_matches("0X"), 16).ok()
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

fn format_gas(frame: &CallFrame) -> String {
    let used = parse_hex_u64(&frame.gas_used);
    format!("[gas {used}]")
}

fn format_error(frame: &CallFrame) -> String {
    if let Some(err) = &frame.error {
        return format!("REVERT({err})");
    }
    String::new()
}

fn print_args(args: &[DecodedArg], prefix: &str, label: &str, is_last: bool) {
    let (branch, pipe) = if is_last { ("└─ ", "   ") } else { ("├─ ", "│  ") };
    let connector = format!("{}{}", prefix.bright_black(), branch.bright_black());
    let sub_prefix_plain = format!("{prefix}{pipe}");

    println!("{}{}", connector, label.bright_black().italic());

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
        println!("{con}{name_part}: {val}");
    }
}

fn color_value(s: String) -> colored::ColoredString {
    // Ethereum address: 0x followed by 40 hex chars
    if s.starts_with("0x") && s.len() == 42 && s[2..].chars().all(|c| c.is_ascii_hexdigit()) {
        return s.yellow();
    }
    // true / false
    if s == "true" {
        return s.bright_green();
    }
    if s == "false" {
        return s.bright_red();
    }
    // Hex data / bytes
    if s.starts_with("0x") {
        return s.bright_black();
    }
    // Pure number (decimal integer or float)
    if s.chars().next().map_or(false, |c| c.is_ascii_digit() || c == '-') &&
        s.chars().skip(1).all(|c| c.is_ascii_digit() || c == '.' || c == '_')
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
