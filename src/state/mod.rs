use sha2::{Sha256, Digest};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct StateRoot(pub [u8; 32]);

#[derive(Debug, Clone)]
pub struct ProofRecord {
    pub proof_id: String,
    pub submitter: String,
    pub verified: bool,
    pub epoch: u64,
    pub hash: [u8; 32],
}

pub struct StateTrie {
    records: BTreeMap<String, ProofRecord>,
    root: StateRoot,
}

impl StateTrie {
    pub fn new() -> Self {
        Self {
            records: BTreeMap::new(),
            root: StateRoot([0u8; 32]),
        }
    }

    pub fn insert(&mut self, record: ProofRecord) {
        self.records.insert(record.proof_id.clone(), record);
        self.recompute_root();
    }

    pub fn get(&self, proof_id: &str) -> Option<&ProofRecord> {
        self.records.get(proof_id)
    }

    pub fn root(&self) -> &StateRoot {
        &self.root
    }

    pub fn len(&self) -> usize {
        self.records.len()
    }

    fn recompute_root(&mut self) {
        let mut hasher = Sha256::new();
        for (k, v) in &self.records {
            hasher.update(k.as_bytes());
            hasher.update(&v.hash);
            hasher.update(&v.epoch.to_le_bytes());
        }
        let result = hasher.finalize();
        self.root = StateRoot(result.into());
    }
}

impl Default for StateTrie {
    fn default() -> Self {
        Self::new()
    }
}

// incremental root recomputation


// property test: insert ordering invariant

