# RFC-0007: Slashing Protocol

**Status:** Accepted
**Authors:** k.tanaka, e.morales
**Created:** 2026-04-12

## Abstract

Validators who submit incorrect verification results or fail to participate must
be penalized to maintain protocol integrity.

## Slashable Offenses

| Offense | Slash % | Description |
|---------|---------|-------------|
| Double sign | 5% | Signing conflicting verification results for the same proof |
| Downtime | 0.1% | Missing more than 50 blocks in a 100-block window |
| Equivocation | 10% | Broadcasting contradictory messages in the same consensus round |
| Invalid state transition | 20% | Committing a state root that doesn't match the verified proofs |

## Jailing

After any slash event, the validator is jailed for `jail_duration` epochs
(default: 3). Jailed validators cannot participate in consensus or earn fees.

## Tombstoning

After `max_infractions` (default: 5) cumulative infractions, the validator is
permanently banned. Remaining stake is forfeit to the slash pool.

## Cooldown

Same-type slashes are subject to a `cooldown_epochs` (default: 2) gap to prevent
double-slashing from duplicate evidence.

## Implementation

See `src/consensus/slashing.rs` for the working engine with full test coverage.
