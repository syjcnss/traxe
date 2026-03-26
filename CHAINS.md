# Supported Chains

Pass a chain name or numeric ID via `--chain` / `-c`.

## Mainnets

| Chain | `--chain` name(s) | Chain ID | Alchemy | Dune |
|---|---|---|---|---|
| Ethereum | `ethereum`, `eth`, `mainnet` | 1 | yes | yes |
| Polygon | `polygon`, `matic`, `polygon-mainnet` | 137 | yes | yes |
| Arbitrum One | `arbitrum`, `arb`, `arbitrum-one`, `arb-mainnet` | 42161 | yes | yes |
| Optimism | `optimism`, `op`, `op-mainnet` | 10 | yes | yes |
| Base | `base`, `base-mainnet` | 8453 | yes | yes |
| Avalanche | `avalanche`, `avax`, `avax-mainnet`, `avalanche-c` | 43114 | yes | yes |
| zkSync Era | `zksync`, `zksync-era` | 324 | yes | yes |
| Linea | `linea` | 59144 | yes | yes |
| BSC | `bsc`, `bnb`, `binance` | 56 | — | yes |
| Scroll | `scroll` | 534352 | — | yes |
| Mantle | `mantle` | 5000 | — | yes |
| Celo | `celo` | 42220 | — | yes |
| Gnosis | `gnosis`, `xdai` | 100 | — | yes |
| Fantom | `fantom`, `ftm` | 250 | — | yes |
| Moonbeam | `moonbeam` | 1284 | — | — |
| Moonriver | `moonriver` | 1285 | — | — |

## Testnets

| Chain | `--chain` name(s) | Chain ID | Alchemy | Dune |
|---|---|---|---|---|
| Ethereum Sepolia | `sepolia`, `eth-sepolia` | 11155111 | yes | yes |
| Ethereum Goerli | `goerli`, `eth-goerli` | 5 | yes | yes |
| Polygon Mumbai | `polygon-mumbai`, `mumbai` | 80001 | yes | — |
| Arbitrum Sepolia | `arbitrum-sepolia`, `arb-sepolia` | 421614 | yes | — |
| Optimism Sepolia | `optimism-sepolia`, `op-sepolia` | 11155420 | yes | — |
| Base Sepolia | `base-sepolia` | 84532 | yes | — |
| Monad Testnet | numeric only | 10143 | — | yes |

## Dune-indexed chains

These chains are only accessible via Dune (no Alchemy support). Use the `--chain` name or numeric chain ID along with `--rpc` if needed.

| Chain | `--chain` name | Chain ID | Dune table |
|---|---|---|---|
| Flare | `flare` | 14 | `flare` |
| Viction | `viction` | 88 | `viction` |
| Unichain | `unichain` | 130 | `unichain` |
| Sonic | `sonic` | 146 | `sonic` |
| X Layer | `xlayer` | 196 | `xlayer` |
| opBNB | `opbnb` | 204 | `opbnb` |
| Boba | `boba` | 288 | `boba` |
| Shape | `shape` | 360 | `shape` |
| World Chain | `worldchain` | 480 | `worldchain` |
| HyperEVM | `hyperevm` | 999 | `hyperevm` |
| Polygon zkEVM | `zkevm` | 1101 | `zkevm` |
| Sei | `sei` | 1329 | `sei` |
| Story | `story` | 1516 | `story` |
| Ronin | `ronin` | 2020 | `ronin` |
| Abstract | `abstract` | 2741 | `abstract` |
| Peaq | `peaq` | 3338 | `peaq` |
| Superseed | `superseed` | 5330 | `superseed` |
| Kaia | `kaia` | 8217 | `kaia` |
| B3 | `b3` | 8333 | `b3` |
| Monad Testnet | `monad-testnet` | 10143 | `monad_testnet` |
| Corn | `corn` | 21000000 | `corn` |
| ApeChain | `apechain` | 33139 | `apechain` |
| Mode | `mode` | 34443 | `mode` |
| Arbitrum Nova | `nova` | 42170 | `nova` |
| Hemi | `hemi` | 43111 | `hemi` |
| Sophon | `sophon` | 50104 | `sophon` |
| Ink | `ink` | 57073 | `ink` |
| BOB | `bob` | 60808 | `bob` |
| Berachain | `berachain` | 80094 | `berachain` |
| Plume | `plume` | 98865 | `plume` |
| Taiko | `taiko` | 167000 | `taiko` |
| Degen | `degen` | 666666666 | `degen` |

## Other chains

Any EVM chain can be used by providing a raw numeric chain ID (e.g. `-c 1234`) along with `--rpc`. Alchemy auto-construction is only available for chains listed above with "yes" in the Alchemy column.
