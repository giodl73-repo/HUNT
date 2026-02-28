# WAVELENGTH — Scenario Status

**Scenario 4** — real-world music domain, testing pipeline at reduced scale (6+1).

## Hunt Summary

| Field | Value |
|-------|-------|
| **Name** | WAVELENGTH |
| **Domain** | Real-world pop/rock music (1970s-1990s) |
| **Scale** | 6 puzzles + 1 meta |
| **Structure** | 1 round |
| **Audience** | Music lovers, casual puzzlers, teams of 2-4 |
| **Format** | Website (WVLG radio station) + 1 physical prop (DJ's final playlist) |
| **Narrator** | Vee — the DJ signing off after 40 years |
| **Answer encoding** | ROT13 |
| **Toolkit path** | `../../toolkit/` |

## Voice Rules (Vee the DJ)

1. No exclamation marks.
2. Short sentences (rarely > 15 words).
3. Past tense for memories, present tense for the broadcast.
4. No meta-commentary.
5. Warm but restrained. She knows this is the last time.
6. The solver is "you" — the listener tuning in.

## Pipeline Status

| Stage | Deliverable | Status |
|-------|-------------|--------|
| 1. SCOPE | `SCOPE.md` | done |
| 2. STRUCTURE | `ROUNDS.md` + `world/` data | done |
| 3. PUZZLE POOL | `PUZZLE-POOL.md` | done |
| 4. ASSIGNMENT | `PUZZLES.md` | done |
| 5. META DESIGN | `meta/META-DESIGN.md` | done |
| 6. AUTHORING | `puzzles/*.md` | done |
| 7. EDITORIAL | `reviews/editorial-review.md` | done |
| 8. INTEGRATION | Integration checklist | done |
| 9. DELIVERY BUILD | `delivery/` | done |
| 10. PLATFORM TEST | `tests/live-solve-simulation.md` | -- |
| 11. POLISH | Ship-ready | -- |

## World Data Files

| File | Status |
|------|--------|
| `world/WORLD.md` | done |
| `world/systems/artists.md` | done |
| `world/systems/albums.md` | done |
| `world/systems/charts.md` | done |
| `world/systems/theory.md` | done |
| `world/systems/lyrics.md` | done |
| `world/systems/station.md` | done |

## Puzzle Registry

| ID | Title | Type | Author | Status |
|----|-------|------|--------|--------|
| P1 | Side A | Track listing extraction | Patrick Berry | authored, tested (27.0/30) |
| P2 | Notation | Music theory encoding | Mike Selinker | authored, tested (28.7/30) |
| P3 | Deep Cuts | Album identification | Lucas Pope | authored, tested (28.0/30) |
| P4 | Chart Toppers | Chart position encoding | Patrick Berry | authored, tested (25.3/30) |
| P5 | Between the Lines | Song title hidden words | Mike Selinker | authored, tested (26.0/30) |
| P6 | Name That Band | Artist name wordplay | Lucas Pope | authored, tested (29.0/30) |
| META | Sign Off | Diagonal extraction | — | designed |

## Scale Observations

(Log any stages where the 11-stage pipeline feels like overkill for 6 puzzles.)

## Author Assignments

| Author | Personality | Puzzles |
|--------|-------------|---------|
| Patrick Berry | Craftsmanship, precision, clean extraction | P1 (Side A), P4 (Chart Toppers) |
| Mike Selinker | Narrative, experience, memorable moments | P2 (Notation), P5 (Between the Lines) |
| Lucas Pope | Deduction, identification, lateral info | P3 (Deep Cuts), P6 (Name That Band) |

## Notes

- All music facts must be verifiable. No invented data.
- Use Billboard Hot 100 as canonical chart source.
- Original album track listings only (not reissues).
