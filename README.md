<div align="center">

# Nomos

**Formal verification framework for distributed mathematical proof validation**

[![CI](https://img.shields.io/badge/CI-passing-brightgreen?style=flat-square)]()
[![License](https://img.shields.io/badge/license-MIT%20%2F%20Apache--2.0-blue?style=flat-square)]()
[![Spec](https://img.shields.io/badge/spec-v0.5-orange?style=flat-square)]()
[![Rust](https://img.shields.io/badge/rust-nightly--2026--04--10-dea584?style=flat-square)]()
[![Stars](https://img.shields.io/badge/stars-84-yellow?style=flat-square&logo=github)]()

> **Status: experimental — DO NOT USE IN PRODUCTION.**
> No external security audit has been performed.

[Website](https://nomosmath.xyz) · [Spec](spec/) · [Architecture](docs/architecture.md)

**Contract:** `0x877d2f72E4Dc4770bDa62b65245649Dc679CEa02`

</div>

---

## Overview

Nomos is a distributed protocol for collaborative verification of mathematical
proofs, with initial focus on computational approaches to the **Strong Goldbach
Conjecture**. Validators run proof-checking nodes that verify submitted proofs
against formal specifications and reach consensus on proof validity through a
BFT protocol.

The protocol incentivizes honest verification through staking and slashing
([RFC-0007](spec/RFC-0007-slashing.md)) and funds ongoing research via
Protocol Tax.

## Architecture

```
Submitter → P2P Gossip → Validators → BFT Consensus → State Trie
```

See [docs/architecture.md](docs/architecture.md) for the full diagram.

## Spec Index

| RFC | Title | Status |
|-----|-------|--------|
| [0001](spec/RFC-0001-overview.md) | Protocol Overview | Accepted |
| [0004](spec/RFC-0004-consensus.md) | Consensus Mechanism | Accepted |
| [0007](spec/RFC-0007-slashing.md) | Slashing Protocol | Accepted |
| [0009](spec/RFC-0009-proof-sharding.md) | Proof Sharding | Draft |
| [0011](spec/RFC-0011-state-layout.md) | State Layout | Accepted |

## Build

```bash
# requires Rust nightly (pinned in rust-toolchain.toml)
cargo build
cargo test
cargo bench --no-run
```

## Run a Node

```bash
cargo run -p nomos-cli -- run --config config.toml
```

> Node implementation is WIP. See the CLI for available commands.

## SDKs

| Language | Package | Status |
|----------|---------|--------|
| Rust | [`sdk/rust`](sdk/rust/) | Types + client stub |
| TypeScript | [`sdk/ts`](sdk/ts/) | Types + client stub |
| Python | [`sdk/py`](sdk/py/) | Types + client stub |

## Zero-Knowledge Proofs

> **Status: Experimental** — opt-in via `ENABLE_ZK=true`

| Component | Status |
|-----------|--------|
| Groth16 prover | WIP |
| Proof verification | WIP |
| PLONK backend | Planned |
| Recursive proofs | Planned |

```bash
ENABLE_ZK=true cargo run -p nomos-cli -- run
```

## Security Model

Nomos assumes ≤1/3 Byzantine validators by stake. See
[docs/security-model.md](docs/security-model.md) for the full threat model.

### Audit Status

| Component | Audited |
|-----------|---------|
| Consensus (BFT) | Internal review |
| Slashing engine | Internal review |
| State trie | Not yet |
| ZK module | N/A (WIP) |

External audit planned for Q4 2026.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md). Protocol changes require an RFC in `spec/`.

## License

Dual-licensed under [MIT](LICENSE-MIT) and [Apache 2.0](LICENSE-APACHE).
