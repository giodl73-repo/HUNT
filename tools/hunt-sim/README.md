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
```

## Current validation signal

- Per-puzzle solve time and hint pressure.
- Meta solve time after the 80% feeder threshold.
- Total wall-clock time against the 2-3 hour target.
- RALLY validation status and findings.
