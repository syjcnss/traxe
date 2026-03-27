mod chainlist;

use anyhow::Result;
use clap::ValueEnum;

use crate::chains;

use super::{
    BlockscoutProvider, DuneProvider, EightbyteProvider, EtherscanProvider, Priorities,
    ProviderManager, RpcProvider, SimulatorProvider, SourcifyProvider, WellKnownProvider,
};

#[derive(ValueEnum, Debug, Clone, PartialEq)]
pub enum TraceSource {
    Rpc,
    Dune,
    Blockscout,
    Simulator,
}

pub struct ProviderFactory {
    http: reqwest::Client,
    rpc_url: Option<String>,
    blockscout_url: Option<String>,
    eightbyte_url: Option<String>,
    trace_source: Option<TraceSource>,
}

impl ProviderFactory {
    pub fn new() -> Self {
        Self {
            http: reqwest::Client::new(),
            rpc_url: None,
            blockscout_url: None,
            eightbyte_url: None,
            trace_source: None,
        }
    }

    pub fn rpc(mut self, url: Option<String>) -> Self {
        self.rpc_url = url;
        self
    }

    pub fn blockscout(mut self, url: Option<String>) -> Self {
        self.blockscout_url = url.or_else(|| std::env::var("BLOCKSCOUT_URL").ok());
        self
    }

    pub fn eightbyte(mut self, url: Option<String>) -> Self {
        self.eightbyte_url = url.or_else(|| std::env::var("EIGHTBYTE_URL").ok());
        self
    }

    pub fn trace_source(mut self, source: Option<TraceSource>) -> Self {
        self.trace_source = source;
        self
    }

    /// Build an RPC provider in priority order:
    /// 1. Explicit `--rpc` flag
    /// 2. `ALCHEMY_API_KEY` env var
    /// 3. Chainlist.org — probes each public RPC and returns a provider with cached trace.
    pub async fn build_rpc(
        &self,
        chain: &chains::Chain,
        tx_hash: &str,
    ) -> Result<RpcProvider> {
        if let Some(url) = &self.rpc_url {
            return Ok(RpcProvider::new(self.http.clone(), url.clone()));
        }

        if let Ok(key) = std::env::var("ALCHEMY_API_KEY") {
            if let Some(network) = chain.alchemy_network {
                let url = format!("https://{network}.g.alchemy.com/v2/{key}");
                return Ok(RpcProvider::new(self.http.clone(), url));
            }
        }

        log::debug!("No RPC configured — searching chainlist.org for a public RPC endpoint...");
        if let Some(provider) = chainlist::Chainlist::new(self.http.clone()).find_working_rpc(chain.chain_id, tx_hash).await {
            let url = provider.url().to_string();
            log::debug!("Using RPC from chainlist: {url}");
            return Ok(provider);
        }

        anyhow::bail!(
            "No working RPC found for chain {}. \
             Provide --rpc <url>, set ALCHEMY_API_KEY, or ensure chainlist.org lists an \
             accessible RPC for this chain.",
            chain.chain_id
        )
    }

    /// Build a ProviderManager and register all providers whose credentials are available.
    pub fn build_manager(&self, rpc: RpcProvider, chain_id: u64, tx_hash: String) -> ProviderManager {
        let dune_key = std::env::var("DUNE_API_KEY").ok();
        let etherscan_key = std::env::var("ETHERSCAN_API_KEY").ok();
        let rpc_url = rpc.url().to_string();

        let mut pm = ProviderManager::new(self.http.clone(), chain_id, tx_hash);

        // --- Trace providers ---
        // Auto mode: register all available providers in fallback order.
        // Forced mode: register only the requested source for the trace role.
        match &self.trace_source {
            Some(TraceSource::Rpc) => {
                pm.register(rpc, Priorities::default().trace(0).label(6));
            }
            Some(TraceSource::Dune) => {
                let key = dune_key.expect("DUNE_API_KEY required for --trace-provider dune");
                pm.register(DuneProvider::new(self.http.clone(), key, Some(rpc_url.clone())), Priorities::default().trace(0));
                pm.register(RpcProvider::new(self.http.clone(), rpc_url.clone()), Priorities::default().label(6));
            }
            Some(TraceSource::Blockscout) => {
                let url = self.blockscout_url
                    .as_deref()
                    .expect("--blockscout or BLOCKSCOUT_URL required for --trace-provider blockscout")
                    .to_string();
                pm.register(BlockscoutProvider::new(self.http.clone(), url), Priorities::default().trace(0).abi(30).label(30));
                pm.register(RpcProvider::new(self.http.clone(), rpc_url.clone()), Priorities::default().label(6));
            }
            Some(TraceSource::Simulator) => {
                pm.register(SimulatorProvider::new(self.http.clone(), rpc_url.clone()), Priorities::default().trace(0));
                pm.register(RpcProvider::new(self.http.clone(), rpc_url.clone()), Priorities::default().label(6));
            }
            None => {
                pm.register(rpc, Priorities::default().trace(0).label(6));
                if let Some(key) = &dune_key {
                    pm.register(DuneProvider::new(self.http.clone(), key.clone(), Some(rpc_url.clone())), Priorities::default().trace(10));
                }
                if let Some(url) = &self.blockscout_url {
                    pm.register(BlockscoutProvider::new(self.http.clone(), url.clone()), Priorities::default().trace(20).abi(30).label(30));
                }
                pm.register(SimulatorProvider::new(self.http.clone(), rpc_url.clone()), Priorities::default().trace(30));
            }
        }

        // --- ABI providers (always registered) ---
        // In None mode, blockscout is already registered above (covers abi+label).
        // In forced modes, add abi providers explicitly; skip blockscout if it already owns the trace role.
        pm.register(WellKnownProvider, Priorities::default().abi(0).label(5));
        pm.register(SourcifyProvider::new(self.http.clone()), Priorities::default().abi(10).label(10));
        if let Some(key) = &etherscan_key {
            pm.register(EtherscanProvider::new(self.http.clone(), key.clone()), Priorities::default().abi(20).label(20));
        }
        // Add blockscout for abi+label in forced modes where it wasn't the forced trace source,
        // and in auto mode it is already registered above with all three capabilities.
        if let Some(url) = &self.blockscout_url {
            if !matches!(self.trace_source, None | Some(TraceSource::Blockscout)) {
                pm.register(BlockscoutProvider::new(self.http.clone(), url.clone()), Priorities::default().abi(30).label(30));
            }
        }

        if let Some(url) = &self.eightbyte_url {
            pm.set_eightbyte(EightbyteProvider::new(self.http.clone(), url.clone()));
        }

        pm
    }
}
