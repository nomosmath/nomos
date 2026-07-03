use std::collections::HashSet;

#[derive(Debug, Clone)]
pub struct PeerInfo {
    pub id: String,
    pub address: String,
    pub last_seen: u64,
}

pub struct GossipConfig {
    pub fanout: usize,
    pub max_peers: usize,
    pub heartbeat_interval_ms: u64,
}

impl Default for GossipConfig {
    fn default() -> Self {
        Self {
            fanout: 6,
            max_peers: 50,
            heartbeat_interval_ms: 1000,
        }
    }
}

pub struct PeerSet {
    peers: HashSet<String>,
    config: GossipConfig,
}

impl PeerSet {
    pub fn new(config: GossipConfig) -> Self {
        Self {
            peers: HashSet::new(),
            config,
        }
    }

    pub fn add_peer(&mut self, id: String) -> bool {
        if self.peers.len() >= self.config.max_peers {
            return false;
        }
        self.peers.insert(id)
    }

    pub fn remove_peer(&mut self, id: &str) -> bool {
        self.peers.remove(id)
    }

    pub fn peer_count(&self) -> usize {
        self.peers.len()
    }

    pub fn select_fanout(&self) -> Vec<&String> {
        self.peers.iter().take(self.config.fanout).collect()
    }
}

// heartbeat extracted

