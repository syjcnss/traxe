use anyhow::{bail, Result};

pub struct Chain {
    pub chain_id: u64,
    /// Alchemy network slug, e.g. "eth-mainnet"
    pub alchemy_network: Option<&'static str>,
}

/// Parse a chain name or numeric chain ID string into a [`Chain`].
pub fn parse(s: &str) -> Result<Chain> {
    // Try numeric first
    if let Ok(id) = s.parse::<u64>() {
        return Ok(Chain {
            chain_id: id,
            alchemy_network: alchemy_network_for_id(id),
        });
    }

    let lower = s.to_lowercase();
    let lower = lower.trim_matches('-').replace("_", "-");

    let (chain_id, alchemy_network) = match lower.as_str() {
        "ethereum" | "eth" | "mainnet" => (1, Some("eth-mainnet")),
        "goerli" | "eth-goerli" => (5, Some("eth-goerli")),
        "sepolia" | "eth-sepolia" => (11155111, Some("eth-sepolia")),
        "polygon" | "matic" | "polygon-mainnet" => (137, Some("polygon-mainnet")),
        "polygon-mumbai" | "mumbai" => (80001, Some("polygon-mumbai")),
        "arbitrum" | "arb" | "arbitrum-one" | "arb-mainnet" => (42161, Some("arb-mainnet")),
        "arbitrum-sepolia" | "arb-sepolia" => (421614, Some("arb-sepolia")),
        "optimism" | "op" | "op-mainnet" => (10, Some("opt-mainnet")),
        "optimism-sepolia" | "op-sepolia" => (11155420, Some("opt-sepolia")),
        "base" | "base-mainnet" => (8453, Some("base-mainnet")),
        "base-sepolia" => (84532, Some("base-sepolia")),
        "avalanche" | "avax" | "avax-mainnet" | "avalanche-c" => (43114, Some("avax-mainnet")),
        "bsc" | "bnb" | "binance" => (56, None),
        "zksync" | "zksync-era" => (324, Some("zksync-mainnet")),
        "linea" => (59144, Some("linea-mainnet")),
        "scroll" => (534352, None),
        "mantle" => (5000, None),
        "celo" => (42220, None),
        "gnosis" | "xdai" => (100, None),
        "fantom" | "ftm" => (250, None),
        "moonbeam" => (1284, None),
        "moonriver" => (1285, None),
        // Dune-indexed chains (no Alchemy support)
        "flare" => (14, None),
        "viction" => (88, None),
        "unichain" => (130, None),
        "sonic" => (146, None),
        "xlayer" => (196, None),
        "opbnb" => (204, None),
        "boba" => (288, None),
        "shape" => (360, None),
        "worldchain" => (480, None),
        "hyperevm" => (999, None),
        "zkevm" => (1101, None),
        "sei" => (1329, None),
        "story" => (1516, None),
        "ronin" => (2020, None),
        "abstract" => (2741, None),
        "peaq" => (3338, None),
        "superseed" => (5330, None),
        "kaia" => (8217, None),
        "b3" => (8333, None),
        "monad-testnet" => (10143, None),
        "corn" => (21000000, None),
        "apechain" => (33139, None),
        "mode" => (34443, None),
        "nova" => (42170, None),
        "hemi" => (43111, None),
        "sophon" => (50104, None),
        "ink" => (57073, None),
        "bob" => (60808, None),
        "berachain" => (80094, None),
        "plume" => (98865, None),
        "taiko" => (167000, None),
        "degen" => (666666666, None),
        _ => bail!(
            "Unknown chain {:?}. Provide a numeric chain ID (e.g. -c 1) or a known name \
             (ethereum, polygon, arbitrum, optimism, base, avalanche, bsc, zksync, …)",
            s
        ),
    };

    Ok(Chain {
        chain_id,
        alchemy_network,
    })
}

/// Returns the native gas token symbol for a chain ID.
pub fn native_symbol(chain_id: u64) -> &'static str {
    match chain_id {
        56 => "BNB",
        137 | 80001 => "MATIC",
        250 => "FTM",
        43114 => "AVAX",
        100 => "xDAI",
        42220 => "CELO",
        1284 => "GLMR",
        1285 => "MOVR",
        _ => "ETH",
    }
}

fn alchemy_network_for_id(id: u64) -> Option<&'static str> {
    match id {
        1 => Some("eth-mainnet"),
        5 => Some("eth-goerli"),
        11155111 => Some("eth-sepolia"),
        137 => Some("polygon-mainnet"),
        80001 => Some("polygon-mumbai"),
        42161 => Some("arb-mainnet"),
        421614 => Some("arb-sepolia"),
        10 => Some("opt-mainnet"),
        11155420 => Some("opt-sepolia"),
        8453 => Some("base-mainnet"),
        84532 => Some("base-sepolia"),
        43114 => Some("avax-mainnet"),
        324 => Some("zksync-mainnet"),
        59144 => Some("linea-mainnet"),
        _ => None,
    }
}
