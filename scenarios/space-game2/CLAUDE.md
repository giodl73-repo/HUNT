# DEAD RECKONING v2 — Scenario Status Tracker

## Hunt Summary

| Field | Value |
|-------|-------|
| **Name** | DEAD RECKONING (v2) |
| **Genre** | Hard science fiction — starship incident reconstruction via instrument operation |
| **Scale** | 17 feeders (6 + 6 + 5) + 3 round metas + 1 final meta |
| **Audience** | Puzzle enthusiasts; physics literacy rewarded but not required |
| **Format** | Interactive HTML instrument panels + text fallback |
| **Duration** | 8-12 hours (team), 15-20 hours (solo) |
| **Theme** | Trek/Firefly starship. Six missing hours. Operate instruments to reconstruct the gap. |
| **Answer format** | Numeric values (frequencies, bearings, time offsets, data rates, etc.) |

## Relationship to v1

The **fiction** is identical to `../space-game/` (same ship, crew, incident, departments, six missing hours). The **puzzle mechanism** is completely different:
- v1: classify interference types, name them, submit word answers
- v2: operate real instruments, achieve physical states, submit numeric answers

The v2 world adds a **physics layer** on top of the v1 fictional world. Technical instrument documentation lives in `world/systems/instruments/`.

## Content Source

SPACEGAMIVERSE fictional world (copied from v1) + physics instrument reference (new for v2). All data in `world/` directory. No puzzle facts from memory — verify against world files only.

## Narrator Voice

The instruments speak. Terse. No flavor text. Evidence only.

- No exclamation marks. No questions.
- Short sentences. Rarely more than 10 words.
- Readout labels. Status lines. Log entries.
- The solver is "you" only in commission briefings. In instrument panels, there is no "you" — only the display.

## Pipeline Status

| Stage | Status | Deliverable | Notes |
|-------|--------|-------------|-------|
| 1. SCOPE | Done | `SCOPE.md`, `CLAUDE.md` | Instrument-based redesign scoped |
| 2. WORLD BUILD | Done | `world/` | v1 fiction + 6 physics instrument docs |
| 3. PUZZLE POOL | Done | `PUZZLE-POOL.md`, `HINTS.md` | 30+ candidates, panel ranked, 17 selected |
| 4. ASSIGNMENT | Done | `PUZZLES.md` | 17 widget-first briefs, all answers numeric |
| 5. META DESIGN | Done | `meta/META-DESIGN.md` | CyclicGroupDisplay final meta, 3 round metas |
| 6. AUTHORING | Done | `puzzles/` | 17 puzzle files authored with widget configs, reference cards, solve paths |
| 7. EDITORIAL | Done | `reviews/editorial-review.md` | 0 blockers, 3 major (1 open — R2-META mechanism), 7 minor. In-place fixes applied. |
| 8. INTEGRATION | Done | `reviews/integration-check.md` | 8/8 checks PASS. 3 round metas authored. BUG-L02/L06 resolved. BUG-L09 new (low). |
| 9. DELIVERY BUILD | -- | `delivery/` | |
| 10. PLATFORM TEST | Done | `tests/platform-test.md` | 0 blockers, 2 major, 4 minor. Dual-solver PASS on all 20 puzzles. Narrative arc PASS. Meta chain verified. |
| 11. POLISH | -- | Final pass | |

## Round Overview

| Round | Name | Evidence Type | Instruments | Puzzles |
|-------|------|--------------|-------------|---------|
| 1 | TELEMETRY | External signals | 6 physics instruments | 6 |
| 2 | SYSTEMS LOG | Internal ship instruments | Ship-internal readouts | 6 |
| 3 | CREW RECORD | Personnel/biological data | Personnel instruments | 5 |
| META | THE COMMISSION | Full reconstruction | All | 1 |

## Meta Architecture

| Element | Answers to | Answer type |
|---------|-----------|-------------|
| Round 1 meta (CONTACT) | WHAT — the nature of the contact | Combined from 6 numeric measurements |
| Round 2 meta (RESPONSE) | HOW — the ship responded | Combined from 6 system state values |
| Round 3 meta (COVER-UP) | WHO — ordered the cover-up | Combined from 5 personnel data points |
| Final meta (THE COMMISSION) | THE INCIDENT — what was hidden and why | All three round-meta conclusions |

## World Systems

| System | File | Status |
|--------|------|--------|
| Ship overview, 16 departments, power, console model | `world/WORLD.md` | LOCKED |
| Per-department detail, cross-links, puzzle hooks | `world/systems/departments.md` | LOCKED |
| ODN topology, room/console layout, replicator flags, EPS power | `world/systems/data-tables.md` | LOCKED |
| Spectrum Analyzer reference | `world/systems/instruments/spectrum-analyzer.md` | LOCKED |
| Oscilloscope (Lissajous) reference | `world/systems/instruments/oscilloscope.md` | LOCKED |
| Correlator reference | `world/systems/instruments/correlator.md` | LOCKED |
| Phased Array (Null Steering) reference | `world/systems/instruments/phased-array.md` | LOCKED |
| FFT Analyzer reference | `world/systems/instruments/fft-analyzer.md` | LOCKED |
| Vector Signal Analyzer reference | `world/systems/instruments/vector-signal-analyzer.md` | LOCKED |

## Answer Security

Numeric answers stored in `.claude/` project memory only. No plaintext answers in git-tracked files.

## File Index

```
scenarios/space-game2/
├── CLAUDE.md            <- THIS FILE (status tracker)
├── SCENARIO.md          <- v2 design philosophy
├── SCOPE.md             <- Stage 1
├── PUZZLE-POOL.md       <- Stage 3
├── HINTS.md             <- Hint skeleton (Stage 3)
├── BUGS-local.md        <- Local bug log
├── world/
│   ├── WORLD.md         <- SPACEGAMIVERSE overview (from v1)
│   ├── LOCKED.md        <- canon lock log
│   └── systems/
│       ├── departments.md  <- per-department detail (from v1)
│       ├── data-tables.md  <- ODN, rooms, replicator, EPS (from v1)
│       └── instruments/    <- NEW for v2
│           ├── spectrum-analyzer.md
│           ├── oscilloscope.md
│           ├── correlator.md
│           ├── phased-array.md
│           ├── fft-analyzer.md
│           └── vector-signal-analyzer.md
├── meta/                <- Stage 5
├── puzzles/             <- Stage 6
├── reviews/             <- editorial + panel output
├── delivery/            <- Stage 9
└── tests/               <- Stage 10
```
