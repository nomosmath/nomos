use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SlashableOffense {
    DoubleSign,
    Downtime,
    Equivocation,
    InvalidStateTransition,
}

#[derive(Debug, Clone)]
pub struct SlashingParams {
    pub double_sign_pct: u32,
    pub downtime_pct: u32,
    pub equivocation_pct: u32,
    pub invalid_transition_pct: u32,
    pub downtime_window: u32,
    pub downtime_threshold: u32,
    pub jail_duration: u64,
    pub max_infractions: u32,
    pub cooldown_epochs: u64,
}

impl Default for SlashingParams {
    fn default() -> Self {
        Self {
            double_sign_pct: 500,       // 5%
            downtime_pct: 10,           // 0.1%
            equivocation_pct: 1000,     // 10%
            invalid_transition_pct: 2000, // 20%
            downtime_window: 100,
            downtime_threshold: 50,
            jail_duration: 3,
            max_infractions: 5,
            cooldown_epochs: 2,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Infraction {
    pub offense: SlashableOffense,
    pub epoch: u64,
    pub evidence: String,
    pub slash_pct: u32,
    pub amount: u64,
}

#[derive(Debug, Clone)]
pub struct ValidatorSlashState {
    pub address: String,
    pub stake: u64,
    pub slashed_total: u64,
    pub is_jailed: bool,
    pub jailed_until: u64,
    pub infractions: Vec<Infraction>,
    pub missed_blocks: u32,
}

pub struct SlashingEngine {
    validators: HashMap<String, ValidatorSlashState>,
    params: SlashingParams,
    current_epoch: u64,
    tombstoned: std::collections::HashSet<String>,
}

impl SlashingEngine {
    pub fn new(params: SlashingParams) -> Self {
        Self {
            validators: HashMap::new(),
            params,
            current_epoch: 0,
            tombstoned: std::collections::HashSet::new(),
        }
    }

    pub fn register_validator(&mut self, address: String, stake: u64) -> Result<(), String> {
        if self.tombstoned.contains(&address) {
            return Err(format!("validator {} is tombstoned", address));
        }
        self.validators.insert(address.clone(), ValidatorSlashState {
            address,
            stake,
            slashed_total: 0,
            is_jailed: false,
            jailed_until: 0,
            infractions: Vec::new(),
            missed_blocks: 0,
        });
        Ok(())
    }

    pub fn set_epoch(&mut self, epoch: u64) {
        self.current_epoch = epoch;
        for v in self.validators.values_mut() {
            if v.is_jailed && v.jailed_until <= epoch {
                v.is_jailed = false;
            }
        }
    }

    pub fn record_missed_block(&mut self, address: &str) -> Result<Option<Infraction>, String> {
        let v = self.validators.get_mut(address)
            .ok_or_else(|| format!("unknown validator: {}", address))?;
        v.missed_blocks += 1;
        if v.missed_blocks >= self.params.downtime_threshold {
            v.missed_blocks = 0;
            let inf = self.slash_inner(address, SlashableOffense::Downtime, "auto:downtime")?;
            return Ok(Some(inf));
        }
        Ok(None)
    }

    pub fn slash(
        &mut self,
        address: &str,
        offense: SlashableOffense,
        evidence: &str,
    ) -> Result<Infraction, String> {
        self.slash_inner(address, offense, evidence)
    }

    fn slash_inner(
        &mut self,
        address: &str,
        offense: SlashableOffense,
        evidence: &str,
    ) -> Result<Infraction, String> {
        if self.tombstoned.contains(address) {
            return Err(format!("{} already tombstoned", address));
        }

        let epoch = self.current_epoch;
        let cooldown = self.params.cooldown_epochs;

        let v = self.validators.get(address)
            .ok_or_else(|| format!("unknown validator: {}", address))?;

        let last_same = v.infractions.iter()
            .filter(|i| i.offense == offense)
            .map(|i| i.epoch)
            .max();

        if let Some(last_epoch) = last_same {
            if epoch - last_epoch < cooldown {
                return Err(format!("cooldown active for {:?} on {}", offense, address));
            }
        }

        let slash_pct = self.get_slash_pct(offense);
        let v = self.validators.get_mut(address).unwrap();
        let amount = (v.stake as u128 * slash_pct as u128 / 10000) as u64;
        let amount = amount.max(1);

        v.stake = v.stake.saturating_sub(amount);
        v.slashed_total += amount;
        v.is_jailed = true;
        v.jailed_until = epoch + self.params.jail_duration;

        let infraction = Infraction {
            offense,
            epoch,
            evidence: evidence.to_string(),
            slash_pct,
            amount,
        };
        v.infractions.push(infraction.clone());

        if v.infractions.len() as u32 >= self.params.max_infractions {
            let remaining = v.stake;
            v.stake = 0;
            v.slashed_total += remaining;
            self.tombstoned.insert(address.to_string());
        }

        Ok(infraction)
    }

    pub fn is_tombstoned(&self, address: &str) -> bool {
        self.tombstoned.contains(address)
    }

    pub fn get_validator(&self, address: &str) -> Option<&ValidatorSlashState> {
        self.validators.get(address)
    }

    pub fn active_validators(&self) -> Vec<&ValidatorSlashState> {
        self.validators.values()
            .filter(|v| !v.is_jailed && !self.tombstoned.contains(&v.address) && v.stake > 0)
            .collect()
    }

    fn get_slash_pct(&self, offense: SlashableOffense) -> u32 {
        match offense {
            SlashableOffense::DoubleSign => self.params.double_sign_pct,
            SlashableOffense::Downtime => self.params.downtime_pct,
            SlashableOffense::Equivocation => self.params.equivocation_pct,
            SlashableOffense::InvalidStateTransition => self.params.invalid_transition_pct,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn setup() -> SlashingEngine {
        let mut engine = SlashingEngine::new(SlashingParams::default());
        engine.register_validator("alice".into(), 1_000_000).unwrap();
        engine.register_validator("bob".into(), 500_000).unwrap();
        engine.set_epoch(1);
        engine
    }

    #[test]
    fn double_sign_slashes_5pct() {
        let mut engine = setup();
        let inf = engine.slash("alice", SlashableOffense::DoubleSign, "ev1").unwrap();
        assert_eq!(inf.amount, 50_000);
        assert_eq!(engine.get_validator("alice").unwrap().stake, 950_000);
    }

    #[test]
    fn jailed_after_slash() {
        let mut engine = setup();
        engine.slash("alice", SlashableOffense::DoubleSign, "ev1").unwrap();
        let v = engine.get_validator("alice").unwrap();
        assert!(v.is_jailed);
        assert_eq!(v.jailed_until, 1 + SlashingParams::default().jail_duration);
    }

    #[test]
    fn unjail_after_duration() {
        let mut engine = setup();
        engine.slash("alice", SlashableOffense::DoubleSign, "ev1").unwrap();
        engine.set_epoch(1 + SlashingParams::default().jail_duration);
        assert!(!engine.get_validator("alice").unwrap().is_jailed);
    }

    #[test]
    fn cooldown_prevents_repeat() {
        let mut engine = setup();
        engine.slash("alice", SlashableOffense::DoubleSign, "ev1").unwrap();
        let result = engine.slash("alice", SlashableOffense::DoubleSign, "ev2");
        assert!(result.is_err());
    }

    #[test]
    fn tombstone_after_max_infractions() {
        let mut engine = setup();
        let params = SlashingParams::default();
        for i in 0..params.max_infractions {
            engine.set_epoch(1 + i as u64 * (params.cooldown_epochs + 1));
            engine.slash("alice", SlashableOffense::DoubleSign, &format!("ev{}", i)).unwrap();
        }
        assert!(engine.is_tombstoned("alice"));
        assert_eq!(engine.get_validator("alice").unwrap().stake, 0);
    }
}
