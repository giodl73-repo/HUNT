# THE SIXTEENTH SHARD — Scenario Status Tracker

## Hunt Summary

| Field | Value |
|-------|-------|
| **Name** | THE SIXTEENTH SHARD |
| **Domain** | Real-world — Brandon Sanderson's Cosmere (published novels + Coppermind) |
| **Scale** | 36 puzzles + 3 round metas + 1 super-meta |
| **Structure** | 3 rounds x 12 puzzles, 6 modules of 6 puzzles each |
| **Audience** | Cosmere readers (Mistborn + Stormlight minimum) |
| **Format** | Website + print companion |
| **Duration** | 10-16 hours (full team) |
| **Narrator** | Hoid/Wit — dry, present tense, no exclamation marks |
| **Meta target** | The 17th Shard — the nature of Hoid's quest |
| **Answer encoding** | ROT13 |
| **Toolkit path** | `../../toolkit/` |

## Content Source

Real-world domain. All facts from published Sanderson works and the Coppermind wiki. No WoBs unless confirmed in published text. Flag uncertain facts with [VERIFY].

## Voice Rules (Hoid/Wit)

1. No exclamation marks.
2. Present tense default. Past tense only for Shattering-era events.
3. Short sentences. Rarely more than 15 words.
4. No direct questions to the solver.
5. Dry humor. The kind that requires a second read.
6. The solver is addressed obliquely — never "you."
7. No meta-commentary.

## Pipeline Status

| Stage | Status | Deliverable | Notes |
|-------|--------|-------------|-------|
| 1. SCOPE | Done | `SCOPE.md`, `CLAUDE.md` | Identity, structure, narrator, meta target |
| 2. WORLD BUILD | Done | `world/` — 6 systems | Allomancy, Knights Radiant, Shards, Hoid, Scadrial, Roshar |
| 3. PUZZLE POOL | Done | `PUZZLE-POOL.md` | 72 candidates (12/module), panel ranked, 36 recommended |
| 4. ASSIGNMENT | -- | `PUZZLES.md` | 36 briefs, all answers coordinated |
| 5. META DESIGN | -- | `meta/` | 3 round metas + 1 super-meta |
| 6. AUTHORING | -- | `puzzles/` | 36 puzzle files |
| 7. EDITORIAL | -- | `reviews/editorial-review.md` | |
| 8. INTEGRATION | -- | `reviews/integration-check.md` | |
| 9. DELIVERY BUILD | -- | `delivery/` | Website + print |
| 10. PLATFORM TEST | -- | `tests/platform-test.md` | |
| 11. POLISH | -- | Final pass, hints, encoding | |

## World Systems

| System | File | Status |
|--------|------|--------|
| Allomancy (16 metals) | `world/systems/allomancy.md` | LOCKED |
| Knights Radiant (10 orders) | `world/systems/knights-radiant.md` | LOCKED |
| Shards of Adonalsium (16) | `world/systems/shards.md` | LOCKED |
| Hoid's Appearances | `world/systems/hoid.md` | LOCKED |
| Scadrial | `world/systems/scadrial.md` | LOCKED |
| Roshar | `world/systems/roshar.md` | LOCKED |

## Module Assignments

| Module | World | Focus | Puzzles | Author Team | Status |
|--------|-------|-------|---------|-------------|--------|
| A | Scadrial | Allomancy chart, metals, powers | P01-P06 | Team Alpha | -- |
| B | Scadrial | History, characters, Era 1+2 | P07-P12 | Team Beta | -- |
| C | Roshar | Knights Radiant, orders, surges | P13-P18 | Team Gamma | -- |
| D | Roshar | Storms, fabrials, Singers | P19-P24 | Team Delta | -- |
| E | Cosmere | The 16 Shards | P25-P30 | Team Epsilon | -- |
| F | Cosmere | Hoid, Worldhoppers, crossovers | P31-P36 | Team Zeta | -- |

## Puzzle Registry

| ID | Title | Type | Author | Score | Status |
|----|-------|------|--------|-------|--------|
| P01-P36 | (not yet assigned) | -- | -- | -- | -- |
| META-I | THE WELL | Round meta | Admin | -- | -- |
| META-II | THE OATHS | Round meta | Admin | -- | -- |
| META-III | ADONALSIUM | Round meta | Admin | -- | -- |
| SUPER | THE SEVENTEENTH SHARD | Super-meta | Admin | -- | -- |

## Answer Registry

Answers stored in `.claude/` project memory only. ROT13 encoded. No plaintext in git-tracked files.

## File Index

```
scenarios/cosmere/
├── SCENARIO.md          <- scenario brief (read-only)
├── CLAUDE.md            <- THIS FILE (status tracker)
├── SCOPE.md             <- Stage 1 deliverable
├── ROUNDS.md            <- Stage 2
├── PUZZLES.md           <- Stage 4
├── BUGS-local.md        <- local bug tracker
├── world/
│   ├── WORLD.md         <- universe overview
│   ├── LOCKED.md        <- canon lock log
│   └── systems/
│       ├── allomancy.md
│       ├── knights-radiant.md
│       ├── shards.md
│       ├── hoid.md
│       ├── scadrial.md
│       └── roshar.md
├── meta/                <- Stage 5
├── puzzles/             <- Stage 6
├── reviews/             <- editorial + panel output
├── tests/               <- blind test results
└── delivery/            <- Stage 9
```
