# traxe

EVM transaction call trace viewer. Fetches and annotates transaction traces with decoded function calls, event logs, and contract labels across 50+ chains.

## Install

```bash
cargo install --path .
```

## Usage

```
traxe <TX_HASH> [OPTIONS]
```

**Examples:**

```bash
# Ethereum via Alchemy
ALCHEMY_API_KEY=<key> traxe 0x5c504ed432cb51138bcf09aa5e8a410dd4a1e204ef84bfed1be16dfba1b22060 -c ethereum

# Explicit RPC endpoint
traxe 0x... --rpc https://rpc.ankr.com/eth

# HTML visualization
traxe 0x... -c ethereum --printer html -o trace.html

# Polygon via Dune
traxe 0x... -c polygon --trace-provider dune
```

## Options

### RPC / Chain

| Flag | Description |
|------|-------------|
| `--rpc <URL>` | RPC endpoint |
| `-c, --chain <CHAIN>` | Chain name or numeric ID (see [Supported Chains](#supported-chains)) |

At least one of `--rpc` or `--chain` is required. If only `--chain` is given, traxe auto-constructs an Alchemy URL (requires `ALCHEMY_API_KEY`) or probes Chainlist.org for a public RPC.

### Trace Provider

| Flag | Description |
|------|-------------|
| `--trace-provider <SOURCE>` | Force a specific trace source: `rpc`, `dune`, `blockscout`, `simulator` |

Default auto-fallback order: RPC → Dune → Blockscout → local EVM simulation.

### ABI / Label Providers

| Flag | Environment Variable | Description |
|------|---------------------|-------------|
| — | `ETHERSCAN_API_KEY` | Enable Etherscan ABI lookup |
| `--blockscout <URL>` | `BLOCKSCOUT_URL` | Blockscout explorer endpoint |
| `--eightbyte <URL>` | `EIGHTBYTE_URL` | 4-byte signature lookup service |

ABIs are resolved from (in order): built-in precompiles → Sourcify → Etherscan → Blockscout.

### Output

| Flag | Description |
|------|-------------|
| `--printer <FORMAT>` | Output format: `text` (default), `json`, `html` |
| `-o, --output <FILE>` | Write to file (HTML defaults to `<TX_HASH>.html`) |

### Text Printer Options

| Flag | Description |
|------|-------------|
| `--text-raw-data` | Show hex calldata even when decoded |
| `--text-no-events` | Hide emitted event logs |
| `--text-show-gas` | Show gas usage |
| `--text-no-color` | Disable colors |

### Other

| Flag | Description |
|------|-------------|
| `-d, --debug` | Enable debug logging |

## Environment Variables

| Variable | Description |
|----------|-------------|
| `ALCHEMY_API_KEY` | Alchemy API key for RPC auto-construction |
| `DUNE_API_KEY` | Dune Analytics API key for trace fetching |
| `ETHERSCAN_API_KEY` | Etherscan API key for ABI lookup |
| `BLOCKSCOUT_URL` | Blockscout endpoint (alternative to `--blockscout`) |
| `EIGHTBYTE_URL` | 8-byte service endpoint (signature with function parameters) |
| `RUST_LOG` | Log level (e.g. `traxe=debug`) |

## Output Formats

**text** — colored ASCII tree for terminal display.

```
0x5c504ed...
├─ CALL UniswapV2Router(0x7a250d...) · swapExactTokensForETH [0x18cbafe5]
│  ├─ input
│  │  ├─ amountIn: 1000000000000000000
│  │  ├─ amountOutMin: 0
│  │  ├─ path: [0x6b175474..., 0xc02aaa39...]
│  │  ├─ to: 0xabcdef...
│  │  └─ deadline: 1620000000
│  ├─ EVENT Transfer(from: 0x..., to: 0x..., value: 1000000000000000000)
│  └─ CALL WETH(0xc02aaa39...) · withdraw [0x2e1a7d4d]
└─ output: true
```

**json** — serialized annotated call tree, suitable for programmatic processing.

**html** — self-contained interactive page with a React-based collapsible tree view.

## Supported Chains

See [CHAINS.md](CHAINS.md) for the full list. Summary:

| Category | Chains |
|----------|--------|
| Alchemy-supported | Ethereum, Polygon, Arbitrum, Optimism, Base, Avalanche, zkSync Era, Linea, and testnets |
| Dune-only | BSC, Scroll, Mantle, Celo, Gnosis, Fantom, Berachain, Taiko, and 25+ others |
| Any EVM | Use a numeric chain ID (e.g. `-c 1234`) with `--rpc` |

## Building

Requires Rust 1.94.1+.

```bash
cargo build --release
```

## Testing

```bash
# Unit tests
cargo test

# End-to-end tests (requires Alchemy key, run sequentially to avoid rate limits)
ALCHEMY_API_KEY=<key> cargo test --test e2e -- --test-threads=1 --nocapture
```
