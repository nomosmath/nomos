# Security Model

## Threat Model

Nomos assumes at most 1/3 of validators (by stake) are Byzantine. Under this
assumption, the protocol guarantees:

- **Safety:** A proof marked as valid was accepted by >2/3 of validators.
- **Liveness:** As long as >2/3 of validators are honest and online, new proofs
  can be verified and finalized.

## Slashing as Deterrent

Economic penalties (RFC-0007) make attacks costly. A double-sign attack requires
controlling >1/3 of total stake and results in at least 5% slash per incident.

## Audit Status

| Component | Audited | Auditor | Date |
|-----------|---------|---------|------|
| Consensus (BFT) | No | — | — |
| Slashing engine | Internal review | — | 2026-05 |
| State trie | No | — | — |
| P2P gossip | No | — | — |
| ZK module | N/A (WIP) | — | — |

External audit planned for Q4 2026.
