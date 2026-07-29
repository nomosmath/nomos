# RFC-0004: Consensus Mechanism

**Status:** Accepted
**Authors:** k.tanaka
**Created:** 2026-03-05

## Abstract

This RFC defines the BFT consensus protocol used by Nomos validators to agree on
proof verification results.

## Protocol

Nomos uses a stake-weighted BFT protocol with a 2/3+1 quorum threshold. Each
consensus round proceeds in three phases:

### Phase 1: Proposal

The designated proposer (round-robin by stake) broadcasts a batch of proof
verification results to the validator set.

### Phase 2: Pre-vote

Each validator independently verifies the proofs in the batch and broadcasts a
signed pre-vote (accept or reject) for each proof.

### Phase 3: Pre-commit

Once a validator observes 2/3+1 pre-votes for a proof, it broadcasts a
pre-commit. After 2/3+1 pre-commits, the proof is finalized.

## View Change

If the proposer fails to produce a valid batch within the timeout (configurable,
default 30s), validators trigger a view change. The next proposer is selected by
stake-weighted round-robin.

## Trade-offs

- **Finality latency:** 2-round BFT gives fast finality (~3s) but requires
  all validators to be online.
- **Throughput:** Batching proofs amortizes consensus overhead. Target: 50 proofs
  per batch.
- **Liveness:** Protocol halts if >1/3 validators go offline. This is acceptable
  for a research network; production deployments should consider fallback to
  optimistic mode (see ENABLE_ROLLUP_MODE flag).

## Glossary

- **Finality gadget**: mechanism that marks a proof as irreversibly accepted.

