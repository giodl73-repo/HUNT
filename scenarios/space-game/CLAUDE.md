# DEAD RECKONING — Scenario Status Tracker

## Hunt Summary

| Field | Value |
|-------|-------|
| **Name** | DEAD RECKONING |
| **Genre** | Sci-fi investigation / certification hunt |
| **Scale** | 15 puzzles + 3 round-metas + 1 final meta |
| **Audience** | Puzzle enthusiasts |
| **Format** | Digital — website or downloadable files |
| **Duration** | 4–6 hours |
| **Theme** | Trek/Firefly starship. Six missing hours. Who deleted them and why? |
| **Final meta answer** | TBD — what happened during the gap |

## Content Source

Fictional world (SPACEGAMIVERSE). All data in `world/` directory. No puzzle facts from memory — verify against world files only.

## Narrator Voice

Evidence speaks for itself. Each puzzle IS a document type — sensor log, EPS routing report, medical intake form, transporter usage log, comm intercept, security badge record. No puzzle instructions. The solver reads as an officer would.

Player register: new officer, competent, under real pressure. The CO believes in you. Prove it.

## Pipeline Status

| Stage | Status | Deliverable | Notes |
|-------|--------|-------------|-------|
| 1. SCOPE | Done | `SCOPE.md`, `world/WORLD.md`, `CLAUDE.md` | Universe in, concept locked |
| 2. STRUCTURE | Done | `ROUNDS.md` | 3 rounds × 5 feeders, 3 round-metas, 1 final; 19 total |
| 3. PUZZLE POOL | Done | `PUZZLE-POOL.md` | 45 candidates, 15 selected, panel ranked |
| 4. ASSIGNMENT | Done | `PUZZLES.md` | 15 briefs + 3 round-metas; answers coordinated |
| 5. META DESIGN | Done | `meta/META-DESIGN.md` | All 4 metas designed; answers in ANSWERS.md |
| 6. AUTHORING | Done | `puzzles/` | 18 files (15 feeders + 3 round-metas), 3003 lines |
| 7. EDITORIAL | Done | `reviews/editorial-review.md` | 2 blocking fixes applied; 6 non-blocking notes |
| 8. INTEGRATION | Done | `reviews/integration-check.md` | All metas verified; holodeck MW canonized; PASSED |
| 9. DELIVERY BUILD | Done | `delivery/` | Site + 19 puzzle pages built; SHA-256 hashes loaded |
| 10. PLATFORM TEST | Done | `tests/platform-test.md` | Checklist written; deploy options documented |
| 11. POLISH | -- | Final pass | |

## Round Overview

| Round | Name | Evidence Type | Departments | Certification |
|-------|------|--------------|-------------|---------------|
| 1 | TELEMETRY | External — what was out there | COMMS, NAV, TAC, SEC, SCIENCE | OPERATIONS |
| 2 | SYSTEMS LOG | Internal — what the ship recorded | POWER, COMPUTER, ENVIRO, PROPULSION, AUXILIARY | ENGINEERING |
| 3 | CREW RECORD | Personnel — what happened to people | MEDICAL, TRANSIT, SEC (personnel), DAMAGE CONTROL, REPLICATOR | COMMAND |
| META | THE COMMISSION | Reconstruct the incident | All | COMMISSIONED OFFICER |

## Meta Answer Architecture

| Element | Answers to |
|---------|-----------|
| Round 1 meta | WHAT — the nature of the contact |
| Round 2 meta | HOW — the ship responded |
| Round 3 meta | WHO — ordered the cover-up |
| Final meta | THE INCIDENT — what was hidden and why |

## World Systems

| System | File | Status |
|--------|------|--------|
| Ship overview, 16 departments, power, console model | `world/WORLD.md` | LOCKED |
| Per-department detail, cross-links, puzzle hooks | `world/systems/departments.md` | LOCKED |
| ODN topology, 15 computer nodes, data flow | `world/systems/data-tables.md` — Table 1 | LOCKED |
| Room/console layout, access depth, shuttlepod | `world/systems/data-tables.md` — Table 2 | LOCKED |
| Replicator security flags, auth tiers, hard limits | `world/systems/data-tables.md` — Table 3 | LOCKED |

## Answer Registry

Answers stored in `.claude/` project memory only. ROT13 encoded. No plaintext in git-tracked files.

## File Index

```
scenarios/space-game/
├── CLAUDE.md            <- THIS FILE (status tracker)
├── SCOPE.md             <- Stage 1 ✓
├── ROUNDS.md            <- Stage 2
├── PUZZLE-POOL.md       <- Stage 3
├── PUZZLES.md           <- Stage 4
├── world/
│   ├── WORLD.md         <- SPACEGAMIVERSE overview (source)
│   ├── LOCKED.md        <- canon lock log (Stage 2)
│   └── systems/
│       └── departments.md  <- per-department detail (source)
├── meta/                <- Stage 5
├── puzzles/             <- Stage 6 (15 + 3 round-metas)
├── reviews/             <- editorial + panel output
└── delivery/            <- Stage 9
    ├── site/            <- website / downloadable files
    └── print/           <- optional print-friendly layouts
```
