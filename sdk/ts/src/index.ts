export interface ProofSubmission {
  id: string;
  kind: "goldbach_partition" | "formal_derivation" | "arithmetic_sequence" | "harmonic_bound";
  payload: Uint8Array;
  claimedResult: string;
}

export interface VerificationResult {
  proofId: string;
  status: "valid" | "invalid" | "inconclusive";
  hash?: string;
  reason?: string;
}

export class NomosClient {
  private endpoint: string;

  constructor(endpoint: string) {
    this.endpoint = endpoint;
  }

  async submitProof(submission: ProofSubmission): Promise<string> {
    throw new Error("not yet implemented");
  }

  async getStatus(proofId: string): Promise<VerificationResult> {
    throw new Error("not yet implemented");
  }
}
