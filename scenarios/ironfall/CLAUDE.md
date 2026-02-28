# IRONFALL — Scenario Status Tracker

## Hunt Summary

| Field | Value |
|-------|-------|
| **Name** | IRONFALL |
| **Genre** | Fictional 16-bit JRPG (1993, SNES) |
| **Scale** | 10 puzzles + 2 round metas + 1 super-meta |
| **Audience** | Teams of 4-8, mixed experience |
| **Format** | Hybrid (website + print) |
| **Duration** | 4-6 hours |
| **Theme** | Fan archive countdown — 90s web aesthetic |
| **Meta target** | True Ending cheat code (SNES button sequence) |

## Content Source

Fictional world. All data in `world/` directory. No external source — all puzzle facts verified against world/ tables only.

## Narrator Voice

The archive speaks. Past tense for forum memories. Present tense for the countdown. No exclamation marks. No questions. Short sentences. The solver is "you."

## Pipeline Status

| Stage | Status | Deliverable | Notes |
|-------|--------|-------------|-------|
| 1. SCOPE | Done | `SCOPE.md`, `world/WORLD.md`, `CLAUDE.md` | Universe designed, 6 core systems |
| 2. STRUCTURE | -- | `ROUNDS.md`, world/ data tables | |
| 3. PUZZLE POOL | -- | Pool brainstorm + panel rank | |
| 4. ASSIGNMENT | -- | `PUZZLES.md` with full briefs | |
| 5. META DESIGN | -- | `meta/` files | |
| 6. AUTHORING | -- | `puzzles/` files | |
| 7. EDITORIAL | -- | `reviews/editorial-review.md` | |
| 8. INTEGRATION | -- | Integration check | |
| 9. DELIVERY BUILD | -- | `delivery/` files | |
| 10. PLATFORM TEST | -- | Test checklists | |
| 11. POLISH | -- | Final pass, hints, encoding | |

## World Systems

| System | File | Status |
|--------|------|--------|
| Bestiary | `world/systems/bestiary.md` | -- |
| Items & Crafting | `world/systems/items.md` | -- |
| Achievements | `world/systems/achievements.md` | -- |
| Locations | `world/systems/locations.md` | -- |
| Battle System | `world/systems/battle.md` | -- |
| Lore & Characters | `world/systems/lore.md` | -- |
| Save File Format | `world/systems/savefile.md` | -- |
| Cheat Codes | `world/systems/cheats.md` | -- |

## Answer Registry

Answers stored in `.claude/` project memory only. ROT13 encoded. No plaintext in git-tracked files.

## Authors (Stage 6)

| Author | Personality | Puzzles |
|--------|-------------|---------|
| The Methodical | Slow, thorough, verbose | TBD |
| The Speedster | Fast, error-prone, energetic | TBD |
| The Skeptic | Challenges briefs, deviates | TBD |
| The Social | Collaborative, messy notes | TBD |
| The Lurker | Silent, no notes, precise | TBD |

## File Index

```
scenarios/ironfall/
├── SCENARIO.md          ← scenario brief (read-only)
├── CLAUDE.md            ← THIS FILE (status tracker)
├── SCOPE.md             ← Stage 1 deliverable
├── ROUNDS.md            ← Stage 2
├── PUZZLES.md           ← Stage 4
├── world/
│   ├── WORLD.md         ← universe overview
│   ├── LOCKED.md        ← canon lock log
│   └── systems/         ← data tables
├── meta/                ← Stage 5
├── puzzles/             ← Stage 6
├── reviews/             ← editorial + panel output
├── tests/               ← blind test results
└── delivery/            ← Stage 9
```
