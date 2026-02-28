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
| 2. STRUCTURE | Done | `ROUNDS.md`, world/ data tables, `world/LOCKED.md` | 8 data systems populated and locked |
| 3. PUZZLE POOL | Done | `PUZZLE-POOL.md` — 18 candidates, 10 selected | 8 locks, 2 contenders promoted |
| 4. ASSIGNMENT | Done | `PUZZLES.md` with full briefs, answer words coordinated | 5 authors x 2 puzzles each |
| 5. META DESIGN | Done | `meta/META-DESIGN.md` | Both round metas + super-meta verified |
| 6. AUTHORING | Done | `puzzles/` files | All 10 puzzles authored, blind-tested >= 22/30 |
| 7. EDITORIAL | Done | `reviews/editorial-review.md` | Voice consistent; P02 answer mismatch flagged |
| 8. INTEGRATION | Done | `reviews/integration-check.md` | P03-P10 pass; P01+P02 need answer revision |
| 9. DELIVERY BUILD | -- | `delivery/` files | |
| 10. PLATFORM TEST | -- | Test checklists | |
| 11. POLISH | -- | Final pass, hints, encoding | |

## World Systems

| System | File | Status |
|--------|------|--------|
| Bestiary | `world/systems/bestiary.md` | LOCKED |
| Items & Crafting | `world/systems/items.md` | LOCKED |
| Achievements | `world/systems/achievements.md` | LOCKED |
| Locations | `world/systems/locations.md` | LOCKED |
| Battle System | `world/systems/battle.md` | LOCKED |
| Lore & Characters | `world/systems/lore.md` | LOCKED |
| Save File Format | `world/systems/savefile.md` | LOCKED |
| Cheat Codes | `world/systems/cheats.md` | LOCKED |

## Puzzle Registry (Stage 6)

| ID | Title | Author | File | Score | Status |
|----|-------|--------|------|-------|--------|
| P01 | Bestiary v3.2 — Complete Stats | The Methodical | `puzzles/P01-bestiary-stats.md` | -- | FIXED — enemies swapped, answer UMBRA verified |
| P02 | Forge Guide — by Old_Forge_Fan | The Methodical | `puzzles/P02-forge-guide.md` | -- | FIXED — hex ID extraction, answer QUELL verified |
| P03 | Save File Deep Dive | The Speedster | `puzzles/P03-save-file-deep-dive.md` | 26/30 | PASS |
| P04 | World Map — All Connections | The Speedster | `puzzles/P04-world-map-connections.md` | 24/30 | PASS |
| P05 | Battle Damage Calculator v2 | The Skeptic | `puzzles/P05-battle-damage-calc.md` | 28/30 | PASS |
| P06 | Enemy Sightings — Unconfirmed | The Skeptic | `puzzles/P06-enemy-sightings.md` | 26/30 | PASS |
| P07 | The Perfect Build | The Social | `puzzles/P07-perfect-build.md` | 24/30 | PASS |
| P08 | 100% Completion Guide | The Social | `puzzles/P08-completion-guide.md` | 27/30 | PASS |
| P09 | The Speedrunner's Route | The Lurker | `puzzles/P09-speedrunners-route.md` | 26/30 | PASS |
| P10 | Anomaly in the Code | The Lurker | `puzzles/P10-anomaly-in-the-code.md` | 29/30 | PASS |

### Known Issues

- **P01 answer mismatch:** PUZZLES.md says HFURE (=USHER), META-DESIGN.md says HZOEN (=UMBRA). Authored P01 produces USHER. Super-meta unaffected (both give U at pos 1). META-I requires UMBRA[2]=M to spell REALM. P01 needs revision to produce UMBRA.
- **P02 answer mismatch:** PUZZLES.md specifies answer DHRYY (ROT13), but the authored P02 file produces DRESS (QERFF). The coordinated answer should be QUELL per the meta extraction table. P02 needs revision to produce QUELL.
- **P03 extraction redesign:** Brief specified achievement bitmask extraction; authored puzzle uses ASCII-in-padding instead, because no Mark name produces 'D' via the Nth-letter rule. Extraction is cleaner and answer is verified.
- **P06 extraction redesign:** Brief specified Nth-letter-of-enemy-name; authored puzzle uses bestiary-number-to-A1Z26 instead, because no enemy name starts with 'L'. Extraction is cleaner and answer is verified.
- **P07 extraction is visual:** Marked letters in a chart rather than a formulaic rule. Justified by The Social's collaborative style.

## Answer Registry

Answers stored in `.claude/` project memory only. ROT13 encoded. No plaintext in git-tracked files.

## Authors (Stage 6)

| Author | Personality | Puzzles |
|--------|-------------|---------|
| The Methodical | Slow, thorough, verbose | P01, P02 |
| The Speedster | Fast, error-prone, energetic | P03, P04 |
| The Skeptic | Challenges briefs, deviates | P05, P06 |
| The Social | Collaborative, messy notes | P07, P08 |
| The Lurker | Silent, no notes, precise | P09, P10 |

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
