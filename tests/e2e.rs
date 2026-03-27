//! End-to-end integration tests against real Ethereum mainnet transactions.
//!
//! These tests make live network calls. They are skipped automatically when the
//! required env vars are absent, so they are safe to include in the default
//! `cargo test` run.
//!
//! **Required (at least one):**
//! ```text
//! RPC_URL          — any archive-capable Ethereum mainnet JSON-RPC endpoint
//! ALCHEMY_API_KEY  — uses https://eth-mainnet.g.alchemy.com/v2/<key>
//! ```
//!
//! **Optional:**
//! ```text
//! DUNE_API_KEY     — enables tests that compare the Dune trace provider against RPC
//! ```
//!
//! Suggested invocations:
//! ```sh
//! # Run all e2e tests, skipping those whose env vars are absent
//! ALCHEMY_API_KEY=<key> cargo test --test e2e -- --nocapture
//!
//! # Force sequential execution to avoid provider rate-limiting
//! ALCHEMY_API_KEY=<key> cargo test --test e2e -- --test-threads=1 --nocapture
//!
//! # Include Dune comparison tests
//! ALCHEMY_API_KEY=<key> DUNE_API_KEY=<key> cargo test --test e2e -- --test-threads=1
//! ```

use traxe::{
    chains,
    providers::{ProviderFactory, TraceSource},
    tree::{self, CallNode, EventNode, Node},
    types::CallType,
};

// ─── Hardcoded test transactions (Ethereum mainnet) ───────────────────────────

/// The very first Ethereum transaction (block 46147, ~2015-08-07).
/// A pure ETH transfer from Vitalik Buterin to Hal Finney.
/// Expects: no calldata, no sub-calls, no events, non-zero value.
const TX_FIRST_ETH: &str = "0x5c504ed432cb51138bcf09aa5e8a410dd4a1e204ef84bfed1be16dfba1b22060";

/// ERC-20 token `transfer` call on Ethereum mainnet (block ~6 100 000, August 2018).
/// Input selector: 0xa9059cbb → `transfer(address,uint256)`.
/// Emits a `Transfer(address indexed, address indexed, uint256)` event.
/// Source: Etherscan API reference documentation examples.
const TX_ERC20_TRANSFER: &str = "0xd65b788c610949704a5f9aac2228c7c777434dfe11c863a12306f57fcbd8cdbb";

/// A multi-call transaction from the Dune Analytics reference documentation
/// (`ethereum.traces` example query). Used to verify that traces containing
/// internal sub-calls round-trip correctly, and that the RPC and Dune providers
/// produce structurally consistent trees for the same transaction.
const TX_COMPLEX: &str = "0xb30d6d67cf7d148c2257bf598c5f5cdf5912a3d05c7d3b000398d675d2fa912c";

// ─── Network helpers ─────────────────────────────────────────────────────────

/// Return an Ethereum mainnet RPC URL from the environment, or `None` when
/// neither `RPC_URL` nor `ALCHEMY_API_KEY` is set.
fn rpc_url() -> Option<String> {
    std::env::var("RPC_URL").ok().or_else(|| {
        std::env::var("ALCHEMY_API_KEY")
            .ok()
            .map(|k| format!("https://eth-mainnet.g.alchemy.com/v2/{k}"))
    })
}

/// Build a [`ProviderManager`] that uses the RPC trace provider for Ethereum mainnet.
async fn build_rpc_pm(tx_hash: &str, rpc_url: String) -> traxe::providers::ProviderManager {
    let chain = chains::parse("ethereum").unwrap();
    let factory = ProviderFactory::new()
        .rpc(Some(rpc_url))
        .trace_source(Some(TraceSource::Rpc));
    let rpc = factory.build_rpc(&chain, tx_hash).await.unwrap();
    factory.build_manager(rpc, 1, tx_hash.to_string())
}

/// Build a [`ProviderManager`] that uses the Dune trace provider for Ethereum mainnet.
///
/// Requires `DUNE_API_KEY` to be present in the environment; panics otherwise.
async fn build_dune_pm(tx_hash: &str, rpc_url: String) -> traxe::providers::ProviderManager {
    let chain = chains::parse("ethereum").unwrap();
    let factory = ProviderFactory::new()
        .rpc(Some(rpc_url))
        .trace_source(Some(TraceSource::Dune));
    let rpc = factory.build_rpc(&chain, tx_hash).await.unwrap();
    factory.build_manager(rpc, 1, tx_hash.to_string())
}

// ─── Tree introspection helpers ───────────────────────────────────────────────

fn root_call(node: &Node) -> &CallNode {
    match node {
        Node::Call(c) => c,
        Node::Event(_) => panic!("root node is an event, expected a call"),
    }
}

/// Count all `Call` nodes in the subtree (including the root if it is a call).
fn count_calls(node: &Node) -> usize {
    match node {
        Node::Call(c) => 1 + c.children.iter().map(count_calls).sum::<usize>(),
        Node::Event(_) => 0,
    }
}

/// Count all `Event` nodes in the subtree.
fn count_events(node: &Node) -> usize {
    match node {
        Node::Call(c) => c.children.iter().map(count_events).sum::<usize>(),
        Node::Event(_) => 1,
    }
}

/// Depth-first search for the first `Event` node whose `event_name` equals `name`.
fn find_event<'a>(node: &'a Node, name: &str) -> Option<&'a EventNode> {
    match node {
        Node::Event(e) if e.event_name.as_deref() == Some(name) => Some(e),
        Node::Call(c) => c.children.iter().find_map(|child| find_event(child, name)),
        Node::Event(_) => None,
    }
}

/// Depth-first search for the first `Call` node whose `function_name` equals `name`.
#[allow(dead_code)]
fn find_call<'a>(node: &'a Node, fn_name: &str) -> Option<&'a CallNode> {
    match node {
        Node::Call(c) => {
            if c.function_name.as_deref() == Some(fn_name) {
                return Some(c);
            }
            c.children.iter().find_map(|child| find_call(child, fn_name))
        }
        Node::Event(_) => None,
    }
}

// ─── Tests ───────────────────────────────────────────────────────────────────

/// Verifies the tree structure for a pure ETH transfer:
/// - Root is a CALL with a non-empty `from` and `to`
/// - `value` is present (non-zero ETH)
/// - No calldata → no `function_name` / decoded input
/// - No sub-calls and no emitted events
#[tokio::test]
async fn test_first_eth_transfer_rpc() {
    let Some(rpc) = rpc_url() else {
        eprintln!("SKIP test_first_eth_transfer_rpc — set RPC_URL or ALCHEMY_API_KEY to enable");
        return;
    };

    let pm = build_rpc_pm(TX_FIRST_ETH, rpc).await;
    let root = tree::build(&pm).await.expect("tree::build failed");

    let call = root_call(&root);

    assert_eq!(call.call_type, CallType::Call, "expected top-level CALL");
    assert!(!call.from.is_empty(), "`from` must not be empty");
    assert!(call.to.is_some(), "`to` must be present for an ETH transfer");

    // Pure ETH transfer carries value
    let value = call.value.as_deref().unwrap_or("0x0");
    assert!(
        value != "0x0" && value != "0x" && value != "0",
        "ETH transfer should have non-zero value, got {:?}",
        call.value
    );

    // No calldata → no decoded function
    assert!(
        call.function_name.is_none(),
        "plain ETH transfer should have no function_name, got {:?}",
        call.function_name
    );
    assert!(call.decoded_input.is_none(), "plain ETH transfer should have no decoded_input");

    // No error / revert
    assert!(call.error.is_none(), "unexpected error: {:?}", call.error);

    // No internal sub-calls and no emitted events
    assert!(
        call.children.is_empty(),
        "plain ETH transfer should produce no child nodes, got {}",
        call.children.len()
    );
    assert_eq!(count_calls(&root), 1, "only the root call should exist");
    assert_eq!(count_events(&root), 0, "no events expected for a plain transfer");
}

/// Verifies ABI decoding for an ERC-20 `transfer` call:
/// - `function_name` is decoded as "transfer" (via the built-in ERC-20 ABI)
/// - `decoded_input` has exactly 2 args with correct types (address, uint256)
/// - A `Transfer` event is present in the tree and carries 3 decoded args
#[tokio::test]
async fn test_erc20_transfer_decoded_rpc() {
    let Some(rpc) = rpc_url() else {
        eprintln!("SKIP test_erc20_transfer_decoded_rpc — set RPC_URL or ALCHEMY_API_KEY to enable");
        return;
    };

    let pm = build_rpc_pm(TX_ERC20_TRANSFER, rpc).await;
    let root = tree::build(&pm).await.expect("tree::build failed");

    let call = root_call(&root);

    // Decoded via the built-in well-known ERC-20 ABI (selector 0xa9059cbb)
    assert_eq!(
        call.function_name.as_deref(),
        Some("transfer"),
        "expected function_name='transfer', got {:?}",
        call.function_name
    );

    // transfer(address recipient, uint256 amount) — 2 args
    let args = call
        .decoded_input
        .as_ref()
        .expect("decoded_input should be present for a decoded transfer()");

    assert_eq!(args.len(), 2, "transfer() takes 2 args, got {:?}", args);

    let types: Vec<&str> = args.iter().map(|a| a.ty.as_str()).collect();
    assert!(
        types.contains(&"address"),
        "expected an `address` arg in decoded_input, got {:?}",
        types
    );
    assert!(
        types.contains(&"uint256"),
        "expected a `uint256` arg in decoded_input, got {:?}",
        types
    );

    // ERC-20 Transfer(address indexed from, address indexed to, uint256 value)
    let transfer_event = find_event(&root, "Transfer")
        .expect("expected a Transfer event; check that the tx emits one");

    let event_args = transfer_event
        .decoded_args
        .as_ref()
        .expect("Transfer event should have decoded args");

    assert_eq!(
        event_args.len(),
        3,
        "ERC-20 Transfer has 3 decoded args (from, to, value), got {:?}",
        event_args
    );
}

/// Verifies that a multi-call transaction builds a tree with internal sub-calls
/// and that the root call completes without error.
#[tokio::test]
async fn test_complex_tx_rpc() {
    let Some(rpc) = rpc_url() else {
        eprintln!("SKIP test_complex_tx_rpc — set RPC_URL or ALCHEMY_API_KEY to enable");
        return;
    };

    let pm = build_rpc_pm(TX_COMPLEX, rpc).await;
    let root = tree::build(&pm).await.expect("tree::build failed");

    let call = root_call(&root);
    assert!(call.error.is_none(), "unexpected root call error: {:?}", call.error);

    let total_calls = count_calls(&root);
    assert!(
        total_calls > 1,
        "TX_COMPLEX should produce multiple call nodes, got {}",
        total_calls
    );
}

/// Verifies that the Dune trace provider builds a structurally equivalent tree
/// to the RPC provider for the same multi-call transaction:
/// - Same call count
/// - Same root `from` and `to` addresses
///
/// Skipped when `DUNE_API_KEY` is absent.
#[tokio::test]
async fn test_complex_tx_rpc_vs_dune() {
    let Some(rpc) = rpc_url() else {
        eprintln!("SKIP test_complex_tx_rpc_vs_dune — set RPC_URL or ALCHEMY_API_KEY to enable");
        return;
    };
    if std::env::var("DUNE_API_KEY").is_err() {
        eprintln!("SKIP test_complex_tx_rpc_vs_dune — set DUNE_API_KEY to enable");
        return;
    }

    let pm_rpc = build_rpc_pm(TX_COMPLEX, rpc.clone()).await;
    let pm_dune = build_dune_pm(TX_COMPLEX, rpc).await;

    let rpc_tree = tree::build(&pm_rpc).await.expect("RPC tree::build failed");
    let dune_tree = tree::build(&pm_dune).await.expect("Dune tree::build failed");

    let rpc_calls = count_calls(&rpc_tree);
    let dune_calls = count_calls(&dune_tree);
    assert_eq!(
        rpc_calls, dune_calls,
        "RPC and Dune should produce the same number of call nodes ({rpc_calls} vs {dune_calls})"
    );

    let rpc_root = root_call(&rpc_tree);
    let dune_root = root_call(&dune_tree);

    assert_eq!(
        rpc_root.from.to_lowercase(),
        dune_root.from.to_lowercase(),
        "root `from` should agree between RPC and Dune"
    );
    assert_eq!(
        rpc_root.to.as_deref().map(str::to_lowercase),
        dune_root.to.as_deref().map(str::to_lowercase),
        "root `to` should agree between RPC and Dune"
    );
    assert_eq!(
        rpc_root.function_name,
        dune_root.function_name,
        "root `function_name` should agree between RPC and Dune"
    );
}
