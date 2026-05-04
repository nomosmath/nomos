# Contributing to Nomos

## Getting Started

1. Fork the repo
2. Create a feature branch from `main`
3. Make changes, add tests
4. Run `cargo test` and `cargo clippy`
5. Open a PR against `main`

## Code Style

- `cargo fmt` before committing
- `cargo clippy` must pass with no warnings
- Conventional commits: `feat:`, `fix:`, `refactor:`, `test:`, `docs:`, `bench:`

## Spec Changes

Protocol changes require an RFC in `spec/`. See existing RFCs for format.
Open a draft PR with the RFC for discussion before implementing.

## Branch Strategy

- `main` — stable, all CI must pass
- `feat/*` — feature branches
- `fix/*` — bugfix branches
