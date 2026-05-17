# hunt-sim

Seeded puzzle-hunt simulation pilot for HUNT.

The first adapter targets `scenarios/wavelength/` and models a casual
music-lover team solving six parallel feeder puzzles plus the diagonal meta.
HUNT owns puzzle graph, solver, hint, and meta policy; RALLY supplies
deterministic run, actor trace, metric, and validation primitives.

## Commands

```powershell
cargo test --quiet
cargo run --quiet -- --seed wavelength-smoke
cargo run --quiet -- --seed wavelength-smoke --runs 12
cargo run --quiet -- --seed wavelength-smoke --compare-variants --runs 12
```

## Current validation signal

- Per-puzzle solve time and hint pressure.
- Meta solve time after the 80% feeder threshold.
- Total wall-clock time against the 2-3 hour target.
- Batch pass rate across speedster, casual, and methodical solver profiles.
- Bottleneck counts showing which feeder most often dominates wall-clock time.
- Variant comparison for hint, clue-relief, meta-prop, and team-parallelism
  tuning.
- RALLY validation status and findings.

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
