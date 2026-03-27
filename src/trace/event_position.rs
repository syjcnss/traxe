use std::collections::HashMap;

use crate::types::{CallFrame, CallType, Log};

// ── Public types ──────────────────────────────────────────────────────────────

/// A raw event log fetched from Dune's `<chain>.logs` table.
/// Logs arrive pre-sorted by `index` (log_index), so no sorting is needed.
#[derive(Debug)]
pub struct RawLog {
    pub contract_address: String,
    /// Topics in order (topic0 first). Only non-null topics are included.
    pub topics: Vec<String>,
    pub data: String,
}

// ── Strategy 1: simulation ────────────────────────────────────────────────────

/// Merge logs from a simulation call tree into a Dune call tree (which lacks logs).
///
/// Walks both trees in parallel and copies logs from each simulation frame into
/// the corresponding Dune frame. The trees must have the same call structure;
/// if they diverge (simulation inaccuracy), the best-effort overlap is used.
pub fn position_from_simulation(dune_root: &mut CallFrame, sim_root: &CallFrame) {
    merge_logs_recursive(dune_root, sim_root);
}

fn merge_logs_recursive(dune: &mut CallFrame, sim: &CallFrame) {
    dune.logs = sim.logs.clone();
    let min_len = dune.calls.len().min(sim.calls.len());
    for i in 0..min_len {
        merge_logs_recursive(&mut dune.calls[i], &sim.calls[i]);
    }
}

// ── Strategy 2: bytecode analysis ────────────────────────────────────────────

/// Assign raw logs to call frames using EVM bytecode analysis.
///
/// For each contract, disassembles its deployed bytecode to determine how many
/// LOG instructions each function emits before vs after external CALL/CREATE
/// instructions. Then walks the call tree in DFS execution order (matching EVM
/// execution order) and assigns each log to its emitting frame.
///
/// Limitations:
/// - Uses a simplified linear scan; complex control flow (loops, many branches)
///   may yield imprecise counts.
/// - Falls back to greedy post-order assignment when no bytecode profile exists.
pub fn position_from_bytecode(
    root: &mut CallFrame,
    raw_logs: Vec<RawLog>,
    bytecodes: &HashMap<String, Vec<u8>>,
) {
    let profiles = build_all_profiles(bytecodes);
    let mut cursor = 0usize;
    assign_logs_recursive(root, &raw_logs, &mut cursor, &profiles);
}

// ── Per-function log profile ──────────────────────────────────────────────────

/// How many LOG instructions a function emits before vs after external calls.
#[derive(Debug, Default, Clone)]
struct LogProfile {
    /// LOG instructions encountered before any CALL/CREATE in the function body.
    before_calls: usize,
    /// LOG instructions encountered after the first CALL/CREATE.
    after_calls: usize,
}

/// Build profiles for all contracts.
/// Returns: lowercase address → selector (None = no selector / fallback) → profile.
fn build_all_profiles(
    bytecodes: &HashMap<String, Vec<u8>>,
) -> HashMap<String, HashMap<Option<[u8; 4]>, LogProfile>> {
    bytecodes
        .iter()
        .map(|(addr, code)| (addr.to_lowercase(), analyze_contract(code)))
        .collect()
}

fn analyze_contract(code: &[u8]) -> HashMap<Option<[u8; 4]>, LogProfile> {
    let instrs = disassemble(code);
    let dispatch = find_dispatch_entries(&instrs);

    // Build PC → instruction index map for fast lookup
    let pc_to_idx: HashMap<usize, usize> =
        instrs.iter().enumerate().map(|(i, ins)| (ins.pc, i)).collect();

    let mut profiles: HashMap<Option<[u8; 4]>, LogProfile> = HashMap::new();

    for (selector, entry_pc) in &dispatch {
        if let Some(&start_idx) = pc_to_idx.get(entry_pc) {
            let profile = scan_function_body(&instrs, start_idx);
            profiles.insert(Some(*selector), profile);
        }
    }

    profiles
}

// ── EVM Disassembler ──────────────────────────────────────────────────────────

#[derive(Debug)]
struct Instr {
    pc: usize,
    op: u8,
    /// For PUSH1..PUSH32: the pushed bytes. Empty for all other opcodes.
    operand: Vec<u8>,
}

/// Parse EVM bytecode into a flat instruction list.
/// PUSH1..PUSH32 (0x60..0x7f) consume `op - 0x5f` additional bytes as operand.
fn disassemble(code: &[u8]) -> Vec<Instr> {
    let mut instrs = Vec::new();
    let mut pc = 0;
    while pc < code.len() {
        let op = code[pc];
        let (operand, skip) = if (0x60..=0x7f).contains(&op) {
            let size = (op - 0x5f) as usize;
            let end = (pc + 1 + size).min(code.len());
            (code[pc + 1..end].to_vec(), size)
        } else {
            (vec![], 0)
        };
        instrs.push(Instr { pc, op, operand });
        pc += 1 + skip;
    }
    instrs
}

// ── Function dispatch detection ───────────────────────────────────────────────

/// Detect Solidity-style function dispatch entries.
///
/// Looks for the pattern:
///   PUSH4 <selector>  (maybe DUP / other bookkeeping)  EQ  PUSH1/2/3 <target>  JUMPI
///
/// Returns selector → JUMPDEST PC where the function body starts.
fn find_dispatch_entries(instrs: &[Instr]) -> HashMap<[u8; 4], usize> {
    let mut entries = HashMap::new();
    let n = instrs.len();

    for i in 0..n {
        // Must be PUSH4 (opcode 0x63 = 0x60 + 4 - 1)
        if instrs[i].op != 0x63 { continue }
        if instrs[i].operand.len() != 4 { continue }
        let selector: [u8; 4] = instrs[i].operand[..4].try_into().unwrap();

        // Look for EQ (0x14) within the next 6 instructions (DUP1, NOT, etc. may appear)
        let Some(eq_j) = (i + 1..n.min(i + 7)).find(|&j| instrs[j].op == 0x14) else {
            continue
        };

        // After EQ, look for PUSH1/PUSH2/PUSH3 (0x60..0x62) within 4 instructions
        let Some(push_j) =
            (eq_j + 1..n.min(eq_j + 5)).find(|&j| instrs[j].op <= 0x62 && instrs[j].op >= 0x60)
        else {
            continue
        };

        // Immediately after the push must be JUMPI (0x57)
        if push_j + 1 >= n || instrs[push_j + 1].op != 0x57 { continue }

        // Decode jump target from PUSH operand
        let target_pc = instrs[push_j]
            .operand
            .iter()
            .fold(0usize, |acc, &b| (acc << 8) | b as usize);

        entries.insert(selector, target_pc);
    }

    entries
}

// ── Function body scan ────────────────────────────────────────────────────────

/// Linearly scan a function body starting at `start_idx`, counting LOG
/// instructions before and after the first external CALL/CREATE instruction.
///
/// Stops at STOP / RETURN / REVERT / INVALID. Does not follow jumps.
/// Scans at most 500 instructions to bound runtime on pathological inputs.
fn scan_function_body(instrs: &[Instr], start_idx: usize) -> LogProfile {
    let mut profile = LogProfile::default();
    let mut seen_external_call = false;
    let limit = instrs.len().min(start_idx + 500);

    for instr in &instrs[start_idx..limit] {
        match instr.op {
            // LOG0..LOG4
            0xa0..=0xa4 => {
                if seen_external_call {
                    profile.after_calls += 1;
                } else {
                    profile.before_calls += 1;
                }
            }
            // CREATE, CALL, CALLCODE, DELEGATECALL, CREATE2, STATICCALL
            0xf0 | 0xf1 | 0xf2 | 0xf4 | 0xf5 | 0xfa => {
                seen_external_call = true;
            }
            // STOP, RETURN, REVERT, INVALID
            0x00 | 0xf3 | 0xfd | 0xfe => break,
            _ => {}
        }
    }

    profile
}

// ── Log assignment ────────────────────────────────────────────────────────────

/// Recursively assign raw logs to call frames in DFS execution order.
///
/// For each frame we ask the bytecode profile: "how many logs does this function
/// emit before vs after its subcalls?" Then:
///   1. Assign `before_calls` logs to this frame (taken from the global cursor).
///   2. Recurse into all child frames (they consume logs during subcalls).
///   3. Assign `after_calls` logs to this frame.
///
/// If no bytecode profile is available, all logs are assigned after children
/// (greedy post-order), which is correct for the common "emit at end" pattern.
fn assign_logs_recursive(
    frame: &mut CallFrame,
    raw_logs: &[RawLog],
    cursor: &mut usize,
    profiles: &HashMap<String, HashMap<Option<[u8; 4]>, LogProfile>>,
) {
    // The bytecode executing is always the callee (frame.to).
    let callee_addr = frame.to.as_deref().unwrap_or("").to_lowercase();

    // In DELEGATECALL the LOG opcode runs in the CALLER's storage context, so
    // the log's `address` field is frame.from, not frame.to.
    let log_addr = if frame.call_type == CallType::DelegateCall {
        frame.from.to_lowercase()
    } else {
        callee_addr.clone()
    };

    let selector = extract_selector(&frame.input);

    // Look up the profile for this callee + selector
    let profile = profiles
        .get(&callee_addr)
        .and_then(|m| m.get(&selector).or_else(|| m.get(&None)))
        .cloned();

    let before_count = profile.as_ref().map(|p| p.before_calls).unwrap_or(0);
    let after_count = profile.as_ref().map(|p| p.after_calls); // None → greedy

    // 1. Assign logs emitted before subcalls
    for _ in 0..before_count {
        if let Some(log) = consume_log_for(raw_logs, cursor, &log_addr) {
            frame.logs.push(log);
        }
    }

    // 2. Recurse into child frames (execution of external calls)
    for child in &mut frame.calls {
        assign_logs_recursive(child, raw_logs, cursor, profiles);
    }

    // 3. Assign logs emitted after subcalls
    match after_count {
        Some(n) => {
            for _ in 0..n {
                if let Some(log) = consume_log_for(raw_logs, cursor, &log_addr) {
                    frame.logs.push(log);
                }
            }
        }
        None => {
            // No profile: greedily consume all consecutive matching logs
            while *cursor < raw_logs.len()
                && raw_logs[*cursor].contract_address.to_lowercase() == log_addr
            {
                frame.logs.push(raw_log_to_crate_log(&raw_logs[*cursor]));
                *cursor += 1;
            }
        }
    }
}

fn consume_log_for(logs: &[RawLog], cursor: &mut usize, addr: &str) -> Option<Log> {
    if *cursor < logs.len() && logs[*cursor].contract_address.to_lowercase() == addr {
        let log = raw_log_to_crate_log(&logs[*cursor]);
        *cursor += 1;
        Some(log)
    } else {
        None
    }
}

fn raw_log_to_crate_log(raw: &RawLog) -> Log {
    Log {
        address: raw.contract_address.clone(),
        topics: raw.topics.clone(),
        data: raw.data.clone(),
        event_name: None,
        decoded_args: None,
    }
}

fn extract_selector(input: &str) -> Option<[u8; 4]> {
    let hex = input.trim_start_matches("0x");
    if hex.len() < 8 { return None; }
    let bytes = hex::decode(&hex[..8]).ok()?;
    bytes.try_into().ok()
}
