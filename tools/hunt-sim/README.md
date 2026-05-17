# hunt-sim

Seeded puzzle-hunt simulation pilot for HUNT.

The first adapter targets `scenarios/wavelength/` and models a casual
music-lover team solving six parallel feeder puzzles plus the diagonal meta.
The second adapter targets `scenarios/boardgames/` and models the Game Night
multi-author handoff: five independent module authors, answer-word readiness,
admin visibility, rework pressure, and meta integration. HUNT owns puzzle graph,
solver, hint, author, and meta policy; RALLY supplies deterministic run, actor
trace, metric, validation, and comparison primitives.

## Commands

```powershell
cargo test --quiet
cargo run --quiet -- --seed wavelength-smoke
cargo run --quiet -- --seed wavelength-smoke --runs 12
cargo run --quiet -- --seed wavelength-smoke --compare-variants --runs 12
cargo run --quiet -- --scenario boardgames --seed game-night-smoke
cargo run --quiet -- --scenario boardgames --seed game-night-smoke --runs 12
cargo run --quiet -- --scenario boardgames --seed game-night-smoke --compare-variants --runs 12
```

## Current validation signal

- Per-puzzle solve time and hint pressure.
- Meta solve time after the 80% feeder threshold.
- Total wall-clock time against the 2-3 hour target.
- Batch pass rate across speedster, casual, and methodical solver profiles.
- Bottleneck counts showing which feeder most often dominates wall-clock time.
- Variant comparison for hint, clue-relief, meta-prop, and team-parallelism
  tuning.
- RALLY comparison-report status for each variant (`improved`, `mixed`, or
  `regressed`) against baseline pass rate, average time, and p95 time.
- RALLY validation status and findings.
- Game Night author handoff quality, rework count, admin visibility gaps, meta
  readiness, and target-window risk.

## Variants

| Variant | Purpose |
|---|---|
| `baseline` | Current WAVELENGTH pressure model. |
| `stronger-hints` | Tests earlier hint delivery without changing puzzle content. |
| `p5-p6-clue-relief` | Lowers P5/P6 difficulty pressure to test final-feeder bottlenecks. |
| `meta-prop-clarity` | Tests a clearer physical meta prop. |
| `team-parallelism` | Tests whether team-size/work-splitting assumptions dominate. |
| `guided-final-set` | Combines P5/P6 relief, earlier hints, and modest meta-prop clarity. |

Early signal from `--compare-variants --runs 12`: `guided-final-set` improves
pass rate without relying on unrealistic extra parallelism. `p5-p6-clue-relief`
also helps and moves bottleneck pressure off the hard final feeders. Pure
`stronger-hints` increases hint usage but does not materially improve pass rate.

## Game Night variants

| Variant | Purpose |
|---|---|
| `baseline` | Current multi-author handoff pressure model. |
| `editorial-pass` | Tests whether explicit editing reduces module rework. |
| `admin-standups` | Tests whether communication cadence closes visibility gaps. |
| `meta-briefing` | Tests earlier meta constraints without changing author output. |
| `parallel-review` | Tests whether review concurrency, not content, dominates schedule risk. |
| `ship-room` | Combines communication, editorial relief, meta clarity, and parallel review. |

Early signal from `--scenario boardgames --compare-variants --runs 12`:
`parallel-review` is the first large schedule unlock, while `ship-room` turns the
scenario into a clean integration pass by combining schedule, quality, and meta
readiness support.
