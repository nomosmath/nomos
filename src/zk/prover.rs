use super::{CircuitArtifact, Proof, ProverBackend, ZkBackend};
use crate::config::FeatureFlags;

pub struct Groth16Prover;

impl ProverBackend for Groth16Prover {
    fn name(&self) -> &str {
        "groth16-nomos"
    }

    fn prove(&self, _witness: &[u8], _circuit: &CircuitArtifact) -> Result<Proof, String> {
        Err("ZK proving is WIP — track progress at https://github.com/elgonrpc/nomos/issues".into())
    }

    fn verify(&self, _proof: &Proof, _public_signals: &[String]) -> Result<bool, String> {
        Err("ZK verification is WIP".into())
    }
}

pub fn create_prover(backend: ZkBackend, flags: &FeatureFlags) -> Result<Box<dyn ProverBackend>, String> {
    if !flags.zk_proofs {
        return Err("ZK proofs not enabled — set ENABLE_ZK=true".into());
    }
    match backend {
        ZkBackend::Groth16 => Ok(Box::new(Groth16Prover)),
        ZkBackend::Plonk => Err("PLONK backend not yet implemented".into()),
    }
}

// improved error messages

