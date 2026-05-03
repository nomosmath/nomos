# RFC-0001: Protocol Overview

**Status:** Accepted
**Authors:** k.tanaka, e.morales
**Created:** 2026-02-18

## Abstract

Nomos is a distributed protocol for collaborative verification of mathematical
proofs. Validators run proof-checking nodes that verify submitted proofs against
formal specifications and reach consensus on proof validity through a BFT
protocol.

## Motivation

Formal verification of complex mathematical proofs (such as computational
approaches to the Strong Goldbach Conjecture) requires significant computational
resources. A distributed network of validators can parallelize verification,
provide economic guarantees of correctness through staking and slashing, and
create a permanent ledger of verified results.

## Design Goals

1. **Correctness first.** The protocol must never mark an invalid proof as valid.
   False negatives (rejecting a valid proof) are tolerable; false positives are not.

2. **Deterministic verification.** Given the same proof payload, every honest
   validator must arrive at the same verification result.

3. **Economic incentives.** Validators stake tokens and earn fees from Protocol Tax.
   Dishonest behavior is punished via slashing (see RFC-0007).

4. **Composability.** The verification engine supports multiple proof kinds
   (Goldbach partitions, formal derivations, arithmetic sequences, harmonic bounds)
   through a pluggable verifier interface.

## Architecture

```
Submitter -> [Proof Submission] -> P2P Gossip -> Validators
                                                    |
                                                    v
                                              [Verification]
                                                    |
                                                    v
                                            [BFT Consensus]
                                                    |
                                                    v
                                             [State Commit]
```

See `docs/architecture.md` for the full diagram.

## References

- RFC-0004: Consensus mechanism
- RFC-0007: Slashing protocol
- RFC-0009: Proof sharding
- RFC-0011: State layout
