use crate::types::CallFrame;

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
