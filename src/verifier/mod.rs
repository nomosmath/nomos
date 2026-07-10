use sha2::{Sha256, Digest};

pub type ProofId = String;

#[derive(Debug, Clone)]
pub enum ProofKind {
    GoldbachPartition,
    FormalDerivation,
    ArithmeticSequence,
    HarmonicBound,
}

#[derive(Debug, Clone)]
pub struct ProofSubmission {
    pub id: ProofId,
    pub kind: ProofKind,
    pub submitter: String,
    pub payload: Vec<u8>,
    pub claimed_result: String,
}

#[derive(Debug, Clone)]
pub enum VerificationResult {
    Valid { proof_id: ProofId, hash: [u8; 32] },
    Invalid { proof_id: ProofId, reason: String },
    Inconclusive { proof_id: ProofId },
}

pub trait ProofVerifier {
    fn verify(&self, submission: &ProofSubmission) -> VerificationResult;
    fn supported_kinds(&self) -> &[ProofKind];
}

pub struct BasicVerifier;

impl ProofVerifier for BasicVerifier {
    fn verify(&self, submission: &ProofSubmission) -> VerificationResult {
        if submission.payload.is_empty() {
            return VerificationResult::Invalid {
                proof_id: submission.id.clone(),
                reason: "empty payload".into(),
            };
        }

        let mut hasher = Sha256::new();
        hasher.update(&submission.payload);
        let hash: [u8; 32] = hasher.finalize().into();

        VerificationResult::Valid {
            proof_id: submission.id.clone(),
            hash,
        }
    }

    fn supported_kinds(&self) -> &[ProofKind] {
        &[
            ProofKind::GoldbachPartition,
            ProofKind::FormalDerivation,
            ProofKind::ArithmeticSequence,
            ProofKind::HarmonicBound,
        ]
    }
}

// batch verification interface

