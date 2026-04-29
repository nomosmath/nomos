pub mod slashing;

use crate::verifier::ProofId;
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct ValidatorSet {
    validators: HashMap<String, ValidatorState>,
    quorum_threshold: f64,
}

#[derive(Debug, Clone)]
pub struct ValidatorState {
    pub address: String,
    pub stake: u64,
    pub is_active: bool,
    pub last_vote_epoch: u64,
}

#[derive(Debug, Clone)]
pub struct Vote {
    pub validator: String,
    pub proof_id: ProofId,
    pub accept: bool,
    pub epoch: u64,
}

#[derive(Debug)]
pub struct ConsensusOutcome {
    pub proof_id: ProofId,
    pub accepted: bool,
    pub votes_for: u32,
    pub votes_against: u32,
    pub epoch: u64,
}

impl ValidatorSet {
    pub fn new(quorum_threshold: f64) -> Self {
        Self {
            validators: HashMap::new(),
            quorum_threshold,
        }
    }

    pub fn register(&mut self, address: String, stake: u64) {
        self.validators.insert(address.clone(), ValidatorState {
            address,
            stake,
            is_active: true,
            last_vote_epoch: 0,
        });
    }

    pub fn active_count(&self) -> usize {
        self.validators.values().filter(|v| v.is_active).count()
    }

    pub fn total_stake(&self) -> u64 {
        self.validators.values()
            .filter(|v| v.is_active)
            .map(|v| v.stake)
            .sum()
    }

    pub fn evaluate_votes(&self, votes: &[Vote]) -> Option<ConsensusOutcome> {
        if votes.is_empty() {
            return None;
        }

        let proof_id = votes[0].proof_id.clone();
        let epoch = votes[0].epoch;

        let total_voting_stake: u64 = votes.iter()
            .filter_map(|v| self.validators.get(&v.validator))
            .filter(|vs| vs.is_active)
            .map(|vs| vs.stake)
            .sum();

        let accept_stake: u64 = votes.iter()
            .filter(|v| v.accept)
            .filter_map(|v| self.validators.get(&v.validator))
            .filter(|vs| vs.is_active)
            .map(|vs| vs.stake)
            .sum();

        let total = self.total_stake();
        if total == 0 || (total_voting_stake as f64 / total as f64) < self.quorum_threshold {
            return None;
        }

        let accepted = (accept_stake as f64 / total_voting_stake as f64) >= self.quorum_threshold;
        let votes_for = votes.iter().filter(|v| v.accept).count() as u32;
        let votes_against = votes.iter().filter(|v| !v.accept).count() as u32;

        Some(ConsensusOutcome {
            proof_id,
            accepted,
            votes_for,
            votes_against,
            epoch,
        })
    }
}

// add quorum evaluation tests

// implement BFT view-change per RFC-0004

// off-by-one in commit-phase quorum count

// extract vote evaluation into method

// gossip backpressure with adaptive fanout

// connection-pool double-free on peer churn

// implement downtime tracking with rolling window

// add validator jailing and unjail logic

// implement tombstoning for repeat offenders
