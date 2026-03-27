mod blockscout;
mod dune;
mod etherscan;
pub mod factory;
mod manager;
mod rpc;
mod simulator;
mod sourcify;
pub mod well_known;

pub use blockscout::BlockscoutProvider;
pub use dune::DuneProvider;
pub use etherscan::EtherscanProvider;
pub use factory::{ProviderFactory, TraceSource};
pub use manager::ProviderManager;
pub use rpc::RpcProvider;
pub use simulator::SimulatorProvider;
pub use sourcify::SourcifyProvider;
pub use well_known::WellKnownProvider;

use std::collections::HashMap;

use anyhow::Result;
use async_trait::async_trait;

use crate::types::{CallFrame, ResolvedAbi};

/// Specifies which roles a provider should be registered for and at what priority.
///
/// Passed to [`ProviderManager::register`] by the caller. Lower values are tried first
/// (0 = highest priority). `None` means do not register the provider for that role.
#[derive(Debug, Clone, Default)]
pub struct Priorities {
    pub trace: Option<u8>,
    pub abi: Option<u8>,
    pub label: Option<u8>,
}

impl Priorities {
    pub fn trace(mut self, priority: u8) -> Self {
        self.trace = Some(priority);
        self
    }

    pub fn abi(mut self, priority: u8) -> Self {
        self.abi = Some(priority);
        self
    }

    pub fn label(mut self, priority: u8) -> Self {
        self.label = Some(priority);
        self
    }
}

/// A data source that can supply one or more of: transaction traces, contract ABIs, address labels.
///
/// Methods for unsupported roles return an error by default and should not be called directly —
/// use [`ProviderManager`] which routes each request to the right providers.
#[async_trait]
pub trait Provider: Send + Sync {
    fn name(&self) -> &'static str;

    async fn fetch_trace(&self, _tx_hash: &str, _chain_id: u64) -> Result<CallFrame> {
        anyhow::bail!("{} does not support trace", self.name())
    }

    async fn fetch_abi(&self, _address: &str, _chain_id: u64) -> Result<ResolvedAbi> {
        anyhow::bail!("{} does not support abi lookup", self.name())
    }

    async fn fetch_label(&self, _address: &str, _chain_id: u64) -> Result<String> {
        anyhow::bail!("{} does not support label lookup", self.name())
    }
}

/// Composite data interface consumed by the tree pipeline.
///
/// Extends [`Provider`] with batch ABI/label resolution and 4-byte selector lookup.
/// Implementations cache the target transaction hash and chain ID so callers don't
/// need to thread them through every call.
/// [`ProviderManager`] implements this trait; any other type that wraps providers can too.
#[async_trait]
pub trait DataProvider: Provider {
    /// The chain ID this manager is operating on.
    fn chain_id(&self) -> u64;

    /// The target transaction hash this manager was built for.
    fn tx_hash(&self) -> &str;

    async fn resolve_abis(&self, addresses: &[String]) -> HashMap<String, ResolvedAbi>;

    async fn resolve_labels(&self, addresses: &[String]) -> HashMap<String, String>;

    async fn resolve_selectors(&self, selectors: &[String]) -> HashMap<String, String>;
}
