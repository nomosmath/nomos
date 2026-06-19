pub mod prover;

#[derive(Debug, Clone)]
pub enum ZkBackend {
    Groth16,
    Plonk,
}

#[derive(Debug, Clone)]
pub struct Proof {
    pub protocol: ZkBackend,
    pub pi_a: [String; 2],
    pub pi_b: [[String; 2]; 2],
    pub pi_c: [String; 2],
    pub public_signals: Vec<String>,
}

pub struct CircuitArtifact {
    pub wasm_path: String,
    pub zkey_path: String,
    pub vkey_path: String,
}

pub trait ProverBackend {
    fn name(&self) -> &str;
    fn prove(&self, witness: &[u8], circuit: &CircuitArtifact) -> Result<Proof, String>;
    fn verify(&self, proof: &Proof, public_signals: &[String]) -> Result<bool, String>;
}
