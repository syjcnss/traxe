/// Runtime configuration for all external data providers.
///
/// Constructed once from CLI args and environment variables, then passed through
/// the call stack so individual modules never need to re-read env vars.
pub struct Providers {
    /// RPC endpoint URL — set by `--rpc` flag or constructed from `ALCHEMY_API_KEY` + chain.
    pub rpc_url: Option<String>,
    /// Dune Analytics API key — from `DUNE_API_KEY` env var.
    pub dune_key: Option<String>,
    /// Blockscout base URL — from `--blockscout` flag or `BLOCKSCOUT_URL` env var.
    pub blockscout_url: Option<String>,
    /// Etherscan API key — from `ETHERSCAN_API_KEY` env var.
    pub etherscan_key: Option<String>,
}

impl Providers {
    /// Build from the resolved RPC URL, the CLI `--blockscout` value, and environment variables.
    pub fn new(rpc_url: Option<String>, blockscout_cli: Option<String>) -> Self {
        // CLI --blockscout takes precedence over BLOCKSCOUT_URL env var.
        let blockscout_url = blockscout_cli.or_else(|| std::env::var("BLOCKSCOUT_URL").ok());
        Self {
            rpc_url,
            dune_key: std::env::var("DUNE_API_KEY").ok(),
            blockscout_url,
            etherscan_key: std::env::var("ETHERSCAN_API_KEY").ok(),
        }
    }

    /// Print enabled providers to stderr before the trace begins.
    pub fn print_enabled(&self) {
        struct Row {
            name: &'static str,
            via: String,
        }

        let mut rows: Vec<Row> = Vec::new();

        // Sourcify is always available — no credentials required.
        rows.push(Row { name: "sourcify", via: "always".to_string() });

        if let Some(url) = &self.rpc_url {
            rows.push(Row { name: "rpc", via: format!("--rpc  ({})", url) });
            rows.push(Row { name: "simulator", via: "--rpc  (--trace-provider simulator)".to_string() });
        }

        if self.dune_key.is_some() {
            rows.push(Row { name: "dune", via: "DUNE_API_KEY".to_string() });
        }

        if let Some(url) = &self.blockscout_url {
            rows.push(Row { name: "blockscout", via: format!("--blockscout / BLOCKSCOUT_URL  ({})", url) });
        }

        if self.etherscan_key.is_some() {
            rows.push(Row { name: "etherscan", via: "ETHERSCAN_API_KEY".to_string() });
        }

        let name_width = rows.iter().map(|r| r.name.len()).max().unwrap_or(0);
        log::info!("Providers:");
        for r in &rows {
            log::info!("  {:<width$}  {}", r.name, r.via, width = name_width);
        }
    }
}

