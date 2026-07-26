#[derive(Debug, Clone)]
pub struct FeatureFlags {
    pub zk_proofs: bool,
    pub zk_recursive: bool,
    pub zk_batch: bool,
    pub mainnet: bool,
    pub rollup_mode: bool,
}

impl Default for FeatureFlags {
    fn default() -> Self {
        Self {
            zk_proofs: false,
            zk_recursive: false,
            zk_batch: false,
            mainnet: false,
            rollup_mode: false,
        }
    }
}

impl FeatureFlags {
    pub fn from_env() -> Self {
        Self {
            zk_proofs: std::env::var("ENABLE_ZK").map(|v| v == "true").unwrap_or(false),
            zk_recursive: false,
            zk_batch: false,
            mainnet: std::env::var("ENABLE_MAINNET").map(|v| v == "true").unwrap_or(false),
            rollup_mode: std::env::var("ENABLE_ROLLUP_MODE").map(|v| v == "true").unwrap_or(false),
        }
    }
}

// per-validator downtime override


// feature flag combination tests

