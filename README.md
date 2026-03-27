# trace-tx

A Rust CLI tool that fetches and visualizes EVM transaction call traces given a transaction hash.

## Overview

Given a transaction hash, the tool:
1. Fetches the call trace from a trace provider (RPC, Dune, Blockscout, or simulator)
2. Resolves contract source code and ABIs to decode function calls
3. Resolves contract labels/tags from Etherscan/Blockscout
4. Outputs either raw JSON or a human-readable tree with decoded args and return values

At startup the tool prints which providers are enabled and what enables each one.

## Network Support

Any EVM-compatible chain — provide the RPC URL and/or Blockscout endpoint, or use `--chain` with an `ALCHEMY_API_KEY`.

See [CHAINS.md](CHAINS.md) for the full list of supported chain names and IDs. A raw numeric chain ID is also accepted for any EVM chain.

## Installation

```
cargo install --path .
```

## Usage

```
trace-tx [OPTIONS] <TX_HASH>

Options:
  -c, --chain <name|id>                                   Chain name or ID (e.g. ethereum, polygon, 1, 137).
                                                          Required when --rpc is not provided.
      --rpc <URL>                                         RPC endpoint URL
      --blockscout <URL>                                  Blockscout explorer endpoint URL
      --trace-provider <rpc|dune|blockscout|simulator>    Force a specific trace provider
      --printer <json|tree|html>                          Printer to use for output (default: tree)
  -o, --output <FILE>                                     Write output to a file instead of stdout
      --tree-raw-data                                     Show raw hex call input and return data [tree printer only]
      --tree-no-events                                    Hide emitted events (logs) [tree printer only]
      --no-color                                          Disable colored output
  -d, --debug                                             Enable debug logging
  -h, --help                                              Print help
  -V, --version                                           Print version
```

If `--rpc` is not provided, `--chain`/`-c` is required. The tool will construct an Alchemy RPC URL using `ALCHEMY_API_KEY` and the given chain.

## Configuration

Providers are enabled by CLI flags and environment variables:

| Provider | Enabled by | Used for |
|---|---|---|
| `sourcify` | always | ABI resolution |
| `rpc` | `--rpc` or `ALCHEMY_API_KEY` | Trace fetching, ERC-20 label resolution |
| `simulator` | `--rpc` or `ALCHEMY_API_KEY` | Fallback trace (may be inaccurate) |
| `dune` | `DUNE_API_KEY` | Trace fetching |
| `blockscout` | `--blockscout` or `BLOCKSCOUT_URL` | Trace fetching, ABI and label resolution |
| `etherscan` | `ETHERSCAN_API_KEY` | ABI and label resolution |

## Providers

### Trace (auto-fallback order, or force with `--trace-provider`)

1. **rpc** — `debug_traceTransaction` via `--rpc` or Alchemy
2. **dune** — queries `ethereum.traces` (or chain-specific table) via Dune Analytics API
3. **blockscout** — internal transactions via `--blockscout` endpoint
4. **simulator** — local simulation (like `cast run`); prints a warning that the trace may be inaccurate

Use `--trace-provider <name>` to skip the fallback chain and force a specific provider.

### ABI / Source code (auto-fallback order)

1. **sourcify** — free, no API key required
2. **etherscan** — requires `ETHERSCAN_API_KEY`
3. **blockscout** — requires `--blockscout` or `BLOCKSCOUT_URL`

### Contract labels

Resolved in order from: ERC-20 `symbol()` via RPC, Etherscan contract name, Blockscout tags/name.

## Output Formats

**Tree** (default) — human-readable hierarchical call tree:
```
TransferHelper::safeTransferFrom(token=0xA0b..., from=0x123..., to=0x456..., value=1000000000)
├── ERC20::transferFrom(sender=0x123..., recipient=0x456..., amount=1000000000) → true
└── ...
```

**JSON** — raw call trace as returned by the trace provider.

**HTML** — self-contained interactive HTML file. Defaults to writing `<tx_hash>.html` in the current directory; override with `-o <file>`.
