use std::collections::HashMap;
use std::sync::Arc;

use anyhow::Result;
use async_trait::async_trait;

use crate::types::{CallFrame, ResolvedAbi};

use super::{sourcify, DataProvider, Priorities, Provider};

/// Composite provider that dispatches to registered underlying providers ordered by priority,
/// falling back to the next one on error. Itself implements [`Provider`] and [`DataProvider`]
/// so it can be used anywhere those traits are expected.
pub struct ProviderManager {
    http: reqwest::Client,
    chain_id: u64,
    tx_hash: String,
    /// (priority, provider) pairs sorted ascending by priority (lowest = tried first).
    trace_providers: Vec<(u8, Arc<dyn Provider>)>,
    abi_providers: Vec<(u8, Arc<dyn Provider>)>,
    label_providers: Vec<(u8, Arc<dyn Provider>)>,
}

impl ProviderManager {
    pub fn new(http: reqwest::Client, chain_id: u64, tx_hash: String) -> Self {
        Self {
            http,
            chain_id,
            tx_hash,
            trace_providers: Vec::new(),
            abi_providers: Vec::new(),
            label_providers: Vec::new(),
        }
    }

    /// Register a provider with explicit priorities for each role it should fill.
    /// `None` for a role means the provider is not registered for that role.
    /// Lower priority values are tried first (0 = highest priority).
    pub fn register<P: Provider + 'static>(&mut self, provider: P, priorities: Priorities) {
        let p: Arc<dyn Provider> = Arc::new(provider);
        if let Some(priority) = priorities.trace {
            self.trace_providers.push((priority, Arc::clone(&p)));
            self.trace_providers.sort_by_key(|(pri, _)| *pri);
        }
        if let Some(priority) = priorities.abi {
            self.abi_providers.push((priority, Arc::clone(&p)));
            self.abi_providers.sort_by_key(|(pri, _)| *pri);
        }
        if let Some(priority) = priorities.label {
            self.label_providers.push((priority, Arc::clone(&p)));
            self.label_providers.sort_by_key(|(pri, _)| *pri);
        }
    }

    /// Log a summary of registered providers per role with their priorities.
    pub fn print_enabled(&self) {
        let fmt = |v: &[(u8, Arc<dyn Provider>)]| -> String {
            v.iter().map(|(pri, p)| format!("{}({})", p.name(), pri)).collect::<Vec<_>>().join(", ")
        };
        log::info!("Providers:");
        log::info!("  trace  [{}]", fmt(&self.trace_providers));
        log::info!("  abi    [{}]", fmt(&self.abi_providers));
        log::info!("  label  [{}]", fmt(&self.label_providers));
    }
}

/// ProviderManager itself is a Provider — it tries registered providers in priority order.
#[async_trait]
impl Provider for ProviderManager {
    fn name(&self) -> &'static str {
        "manager"
    }

    async fn fetch_trace(&self, tx_hash: &str, chain_id: u64) -> Result<CallFrame> {
        let mut last_err = anyhow::anyhow!("no trace providers registered");
        for (_, provider) in &self.trace_providers {
            log::debug!("trace: trying {}", provider.name());
            match provider.fetch_trace(tx_hash, chain_id).await {
                Ok(frame) => {
                    log::debug!("trace: {} succeeded", provider.name());
                    return Ok(frame);
                }
                Err(e) => {
                    log::debug!("trace: {} failed: {}", provider.name(), e);
                    log::debug!("{} trace failed, trying next provider...", provider.name());
                    last_err = e;
                }
            }
        }
        Err(last_err)
    }

    async fn fetch_abi(&self, address: &str, chain_id: u64) -> Result<ResolvedAbi> {
        for (_, provider) in &self.abi_providers {
            log::debug!("abi: trying {} for {}", provider.name(), address);
            match provider.fetch_abi(address, chain_id).await {
                Ok(resolved) => {
                    log::debug!("abi: {} hit for {}", provider.name(), address);
                    return Ok(resolved);
                }
                Err(e) => log::debug!("abi: {} miss for {}: {}", provider.name(), address, e),
            }
        }
        anyhow::bail!("no ABI found for {}", address)
    }

    async fn fetch_label(&self, address: &str, chain_id: u64) -> Result<String> {
        for (_, provider) in &self.label_providers {
            log::debug!("label: trying {} for {}", provider.name(), address);
            match provider.fetch_label(address, chain_id).await {
                Ok(label) => {
                    log::debug!("label: {} hit for {}: {}", provider.name(), address, label);
                    return Ok(label);
                }
                Err(e) => log::debug!("label: {} miss for {}: {}", provider.name(), address, e),
            }
        }
        anyhow::bail!("no label found for {}", address)
    }
}

/// ProviderManager implements DataProvider — the interface consumed by the tree pipeline.
#[async_trait]
impl DataProvider for ProviderManager {
    fn chain_id(&self) -> u64 {
        self.chain_id
    }

    fn tx_hash(&self) -> &str {
        &self.tx_hash
    }

    async fn resolve_abis(&self, addresses: &[String]) -> HashMap<String, ResolvedAbi> {
        let mut result = HashMap::new();
        for addr in addresses {
            let lower = addr.to_lowercase();
            if let Ok(resolved) = self.fetch_abi(&lower, self.chain_id).await {
                result.insert(lower, resolved);
            }
        }
        result
    }

    async fn resolve_labels(&self, addresses: &[String]) -> HashMap<String, String> {
        let mut result = HashMap::new();
        for addr in addresses {
            let lower = addr.to_lowercase();
            if let Ok(label) = self.fetch_label(&lower, self.chain_id).await {
                result.insert(lower, label);
            }
        }
        result
    }

    async fn resolve_selectors(&self, selectors: &[String]) -> HashMap<String, String> {
        if selectors.is_empty() {
            return HashMap::new();
        }
        sourcify::lookup_selectors(&self.http, selectors).await
    }
}
