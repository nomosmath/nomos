# RFC-0009: Proof Sharding

**Status:** Draft
**Authors:** r.singh
**Created:** 2026-05-20

## Abstract

As the proof submission rate grows, a single validator set cannot verify all
proofs in real time. This RFC proposes partitioning the proof space into shards,
each handled by a subset of validators.

## Design

Proofs are assigned to shards based on a deterministic hash of the proof ID:

```
shard_id = sha256(proof_id) % num_shards
```

Each shard has its own consensus instance with a minimum validator set size of
`min_shard_validators` (default: 7).

## Cross-shard Communication

When a proof references results from another shard (e.g., a formal derivation
that depends on a previously verified Goldbach partition), the dependent shard
must wait for finalization on the source shard before proceeding.

## Status

Draft — not yet implemented. Targeted for v0.7.0.
