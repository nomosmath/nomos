# Architecture

```
                    ┌─────────────┐
                    │  Submitter  │
                    └──────┬──────┘
                           │ proof submission
                           ▼
                    ┌──────────────┐
                    │   P2P Layer  │  gossip protocol (RFC-0009)
                    └──────┬───────┘
                           │
              ┌────────────┼────────────┐
              ▼            ▼            ▼
        ┌──────────┐ ┌──────────┐ ┌──────────┐
        │Validator │ │Validator │ │Validator │
        │  Node A  │ │  Node B  │ │  Node C  │
        └────┬─────┘ └────┬─────┘ └────┬─────┘
             │             │             │
             └──────┬──────┘──────┬──────┘
                    │ BFT votes   │
                    ▼             ▼
             ┌──────────────────────┐
             │   Consensus Layer    │  RFC-0004
             │   (2/3+1 quorum)    │
             └──────────┬──────────┘
                        │
                        ▼
             ┌──────────────────────┐
             │    State Trie        │  RFC-0011
             │  (verified proofs)   │
             └──────────────────────┘
```

## Components

- **Verifier** (`src/verifier/`): pluggable proof checking engine
- **Consensus** (`src/consensus/`): BFT agreement + slashing
- **State** (`src/state/`): Merkle trie for verified proofs
- **P2P** (`src/p2p/`): gossip-based proof distribution
- **ZK** (`src/zk/`): experimental zero-knowledge proof backend (WIP)
