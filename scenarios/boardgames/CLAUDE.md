# Game Night — A Board Game Puzzle Hunt

**Scenario 2** — testing the multi-author module workflow.

## Hunt Summary

| Field | Value |
|-------|-------|
| **Name** | Game Night |
| **Theme** | 5 classic board games, each a puzzle |
| **Scale** | 5 puzzles + 1 meta |
| **Audience** | Board game players (casual to serious) |
| **Authors** | 5 different AI personalities (1 per module) + 1 admin (meta) |
| **Purpose** | Test multi-author workflow, module handoff, personality conflicts |

## Module Structure

Each puzzle is a module with its own author and working directory:

```
modules/
├── M1-chess/          ← Author: The Methodical (precise, slow, over-documents)
│   ├── brief.md       ← puzzle brief (from admin)
│   ├── puzzle.md      ← authored puzzle page
│   ├── notes.md       ← author's working notes
│   └── tests/         ← test results for this puzzle
│
├── M2-settlers/       ← Author: The Speedster (fast, sloppy, needs revision)
│   └── ...
│
├── M3-risk/           ← Author: The Skeptic (argues with principles, finds edge cases)
│   └── ...
│
├── M4-pandemic/       ← Author: The Social (asks everyone for help, shares ideas)
│   └── ...
│
├── M5-codenames/      ← Author: The Lurker (delivers quietly, minimal notes)
│   └── ...
│
└── M6-meta/           ← Author: Admin (designs the meta after all 5 are done)
    ├── brief.md
    ├── meta.md
    └── tests/
```

## The 5 Author Personalities (from toolkit/solvers/)

Reusing solver profiles as author personalities — tests whether the profiles are dual-purpose.

| Module | Game | Author | Profile | Challenge they create |
|--------|------|--------|---------|---------------------|
| M1 | Chess | **The Methodical** | `solvers/methodical.md` | Slow, thorough, over-documents. Chess demands this. Quality high but verbose — needs editing down. |
| M2 | Settlers | **The Speedster** | `solvers/speedster.md` | Ships v1 in minutes. Sloppy on details. Fails first test. Resists revision feedback. |
| M3 | Risk | **The Skeptic** | `solvers/skeptic.md` | "Why can't I use probability tables?" Questions the principles. Argues with the panel. Sometimes right. |
| M4 | Pandemic | **The Social** | `solvers/social.md` | Asks The Methodical for help. Shares partial work with everyone. Interlock improves but creates dependencies. |
| M5 | Codenames | **The Lurker** | `solvers/lurker.md` | Delivers on time. Zero communication. No notes. Admin has no visibility until delivery. Either great or needs rework. |
| M6 | Meta | **The Captain** | `solvers/captain.md` | The admin. Designs meta after all 5 lock. Manages the whole show. Delegates, tracks, resolves conflicts. |

## Files

| File | Stage | Status |
|------|-------|--------|
| `CLAUDE.md` | — | ✅ This file |
| `SCOPE.md` | Stage 1 | ✅ Complete |
| `world/WORLD.md` | Stage 2 | ✅ Overview + index |
| `world/chess.md` | Stage 2 | ✅ Pieces, notation, openings, famous games, checkmates |
| `world/settlers.md` | Stage 2 | ✅ Hexes, resources, costs, dev cards, ports, robber |
| `world/risk.md` | Stage 2 | ✅ 42 territories, 6 continents, combat, cards |
| `world/pandemic.md` | Stage 2 | ✅ 7 roles, infection, outbreaks, cures, epidemics |
| `world/codenames.md` | Stage 2 | ✅ Grid, spymaster/operative, clue rules, key card |
| `ROUNDS.md` | Stage 3 | ✅ 1 round, 5 puzzles, 1 meta, Host narrator |
| `PUZZLE-POOL.md` | Stage 4 | ✅ 15 candidates (3 per game), starred picks |
| `PUZZLES.md` | Stage 5 | ✅ Full registry, 5 puzzles assigned, answer words locked |
| `ANSWERS.md` | Stage 5 | ✅ ROT13-encoded answers + solution paths |
| `MODULE-LOG.md` | Stage 7 | ✅ Multi-author dynamics log |
| `modules/M1-chess/brief.md` | Stage 5 | ✅ Brief issued |
| `modules/M1-chess/puzzle.md` | Stage 7 | ✅ Authored by The Methodical (needs editing for length) |
| `modules/M2-settlers/brief.md` | Stage 5 | ✅ Brief issued |
| `modules/M2-settlers/puzzle.md` | Stage 7 | ✅ Authored by The Speedster (has known bugs, needs v3) |
| `modules/M3-risk/brief.md` | Stage 5 | ✅ Brief issued |
| `modules/M3-risk/puzzle.md` | Stage 7 | ✅ Authored by The Skeptic (includes editorial commentary) |
| `modules/M4-pandemic/brief.md` | Stage 5 | ✅ Brief issued |
| `modules/M4-pandemic/puzzle.md` | Stage 7 | ✅ Authored by The Social (working notes need cleanup) |
| `modules/M5-codenames/brief.md` | Stage 5 | ✅ Brief issued |
| `modules/M5-codenames/puzzle.md` | Stage 7 | ✅ Authored by The Lurker (spoiler section needs removal) |
| `modules/M6-meta/` | Stage 6 | Pending — all 5 answers locked, meta can now be designed |
| `meta/` | Stage 6 | — |

## What This Scenario Tests

1. **Module isolation** — can 5 authors work independently?
2. **Admin control** — can the admin manage 5 personalities?
3. **Handoff quality** — do briefs contain enough detail?
4. **Conflict resolution** — what happens when The Rebel disagrees?
5. **Meta dependency** — meta can't start until all 5 answers are locked
6. **Integration** — do 5 independently authored puzzles feel like one hunt?

## Simulator-backed operating guidance

`tools/hunt-sim` now includes a Game Night mode that pressure-tests this
scenario's multi-author risks:

```powershell
cd tools\hunt-sim
cargo run --quiet -- --scenario boardgames --seed game-night-smoke --compare-variants --runs 12
```

The useful product finding is that serial review is the dominant schedule risk.
The baseline simulation regularly overruns the 2-3 hour window because module
review, rework, and meta integration queue behind one admin. `parallel-review`
is the first major unlock, and `ship-room` is the recommended operating mode for
future multi-author hunts: parallel review lanes, explicit editorial relief,
light meta briefing, and admin standups for low-visibility authors.

For this scenario, treat `ship-room` as the postmortem recommendation rather
than a retcon of the recorded run. The original log remains the evidence of what
happened; the simulator finding is the workflow to use next time.
