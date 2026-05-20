# RFC-0011: State Layout

**Status:** Accepted
**Authors:** k.tanaka
**Created:** 2026-06-01

## Abstract

Defines the Merkle trie layout for storing verified proof records.

## Schema

Each leaf in the state trie is a `ProofRecord`:

```
ProofRecord {
    proof_id:  bytes32    // unique identifier
    submitter: address    // who submitted the proof
    verified:  bool       // consensus result
    epoch:     u64        // finalization epoch
    hash:      bytes32    // sha256 of proof payload
}
```

## Key Derivation

Keys are derived as `sha256("nomos:proof:" || proof_id)`, giving a uniform
distribution across the trie.

## Root Computation

The state root is computed incrementally: each new record triggers a partial
recomputation of the Merkle path from the leaf to the root.

## Implementation

See `src/state/mod.rs`.
