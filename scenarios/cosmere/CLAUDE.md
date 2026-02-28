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
| 4. ASSIGNMENT | Done | `PUZZLES.md` | 36 briefs, all 36 answer words coordinated with metas |
| 5. META DESIGN | Done | `meta/` | 3 round metas + 1 super-meta, all verified |
| 6. AUTHORING | Done | `puzzles/` | All 36 puzzles authored (P01-P36) |
| 7. EDITORIAL | Done | `reviews/editorial-review.md` | Voice: PASS w/ advisories. Extractions: 36/36 PASS. Metas: 4/4 PASS. 3 bugs fixed, 5 new logged. |
| 8. INTEGRATION | Done (conditional) | `reviews/integration-check.md` | 1 BLOCKER (UNITY in P10 word bank), 2 MAJOR (SUPER-META truncation, difficulty triplets). Fix BLOCKER before Stage 9. |
| 9. DELIVERY BUILD | Done | `delivery/` | Site spec, homepage, theme system, print spec |
| 10. PLATFORM TEST | Done | `tests/platform-test.md` | Site: PASS. Solve sim (5 solvers, 11-12h): PASS. All 4 metas verified. UNITY moment lands. 0 blockers, 3 MAJOR, 8 MINOR for Stage 11. |
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
| A | Scadrial | Allomancy chart, metals, powers | P01-P06 | Team Alpha | Assigned |
| B | Scadrial | History, characters, Era 1+2 | P07-P12 | Team Beta | Assigned |
| C | Roshar | Knights Radiant, orders, surges | P13-P18 | Team Gamma | Assigned |
| D | Roshar | Storms, fabrials, Singers | P19-P24 | Team Delta | Assigned |
| E | Cosmere | The 16 Shards | P25-P30 | Team Epsilon | Assigned |
| F | Cosmere | Hoid, Worldhoppers, crossovers | P31-P36 | Team Zeta | Assigned |

## Puzzle Registry

### Module A — Allomancy (Team Alpha)

| ID | Title | Pool ID | Mechanism | Difficulty | Metal | Status |
|----|-------|---------|-----------|------------|-------|--------|
| P01 | Burning Through | A-02 | Identification + bold-word acrostic | ★★ | Iron (#1) | Authored (27/30) |
| P02 | Alloy Partners | A-04 | Pairing + sentence-initial acrostic | ★★ | Steel (#2) | Authored (27/30) |
| P03 | Feruchemical Reserves | A-07 | Identification + sentence-initial acrostic | ★★★ | Zinc (#5) | Authored (26/30) |
| P04 | The Allomantic Table | A-01 | Grid logic + scrambled acrostic | ★★★ | Brass (#6) | Authored (28/30) |
| P05 | Three Arts, One Spike | A-05 | Triple-matching + intro acrostic | ★★★★ | Pewter (#4) | Authored (27/30) |
| P06 | The Coppercloud | A-09 | Deductive logic (seating + metals) | ★★★★ | Bronze (#8) | Authored (29/30) |

### Module B — Scadrial History (Team Beta)

| ID | Title | Pool ID | Mechanism | Difficulty | Metal | Status |
|----|-------|---------|-----------|------------|-------|--------|
| P07 | The Lord Ruler's 1000 Years | B-02 | Chronological ordering + acrostic | ★★ | Electrum (#10) | Authored |
| P08 | Kelsier's Crew | B-01 | Identification + positional extraction | ★★ | Gold (#9) | Authored |
| P09 | The Social Ladder | B-05 | Categorization + ordered extraction | ★★★ | Cadmium (#11) | Authored |
| P10 | The Survivor's Legacy | B-11 | Cross-era matching (cause/effect) | ★★★ | Chromium (#15) | Authored |
| P11 | The Siege of Luthadel | B-03 | Logic grid (multi-variable deduction) | ★★★★ | Bendalloy (#12) | Authored |
| P12 | The Roughs Justice | B-06 | Deduction (Twinborn ability ID) | ★★★★ | Duralumin (#14) | Authored |

### Module C — Knights Radiant (Team Gamma)

| ID | Title | Pool ID | Mechanism | Difficulty | Order | Status |
|----|-------|---------|-----------|------------|-------|--------|
| P13 | Herald's Madness | C-06 | Identification (behavioral) + mapping | ★★ | Dustbringers | Authored |
| P14 | Ideals and Oaths | C-02 | Fill-in-blank + acrostic | ★★★ | Edgedancers | Authored |
| P15 | Spren Bonds | C-04 | Identification (narrative) + mapping | ★★★ | Truthwatchers | Authored |
| P16 | Radiant Roster | C-07 | Character ID + conditional indexing | ★★★ | Skybreakers | Authored |
| P17 | The Double Eye | C-01 | Ring topology logic + multi-attribute | ★★★★ | Windrunners | Authored |
| P18 | The Surge Wheel | C-05 | Circular constraint placement | ★★★★ | Lightweavers (dup) | Authored |

### Module D — Roshar World (Team Delta)

| ID | Title | Pool ID | Mechanism | Difficulty | Order | Status |
|----|-------|---------|-----------|------------|-------|--------|
| P19 | Singer Forms | D-04 | Identification + indexing | ★★ | Stonewards | Authored |
| P20 | Bridge Four | D-12 | Anagram + indexing | ★★ | Windrunners (dup) | Authored |
| P21 | Rhythm of Answers | D-01 | Identification + name extraction | ★★★ | Willshapers | Authored |
| P22 | The Chasms Below | D-07 | Cryptic crossword (Cosmere-themed) | ★★★ | Lightweavers (crit) | Authored |
| P23 | Fabrial Workshop | D-03 | Triple-matching (effect/type/gem) | ★★★ | Bondsmiths | Authored |
| P24 | Stormwall | D-02 | Spatial/directional logic | ★★★★ | Elsecallers | Authored |

### Module E — The 16 Shards (Team Epsilon)

| ID | Title | Pool ID | Mechanism | Difficulty | Shard | Status |
|----|-------|---------|-----------|------------|-------|--------|
| P25 | Shattered and Whole | E-03 | Categorization + count indexing | ★★ | Honor | Authored |
| P26 | Vessel Roll Call | E-02 | Identification + extraction | ★★★ | Preservation | Authored |
| P27 | Odium's Kill List | E-04 | ID + chronological ordering | ★★★ | Odium | Authored |
| P28 | Investiture Types | E-08 | Magic-system ID + chained mapping | ★★★ | Endowment | Authored |
| P29 | The 4x4 Shard Grid | E-07 | Grid logic (axis deduction) | ★★★★ | Autonomy | Authored |
| P30 | The Shattering | E-11 | Circular logic (seating arrangement) | ★★★★★ | Virtuosity | Authored |

### Module F — Hoid & Worldhoppers (Team Zeta)

| ID | Title | Pool ID | Mechanism | Difficulty | Shard | Status |
|----|-------|---------|-----------|------------|-------|--------|
| P31 | The Wanderer's Aliases | F-01 | ID (presence/absence) + alias extraction | ★★★ | Ruin | Authored |
| P32 | Cognitive Shadows | F-04 | Identification + system matching | ★★★ | Harmony | Authored |
| P33 | Collected Powers | F-02 | Triple ID (ability/world/Shard) | ★★★★ | Cultivation | Authored |
| P34 | Letters to a Dragon | F-06 | Attribution (author ID from style) | ★★★★ | Devotion | Authored |
| P35 | The Memory Thief | F-11 | Error detection (true/false) | ★★★★ | Whimsy | Authored |
| P36 | All Roads Lead to Hoid | F-12 | Connection mapping (cross-reference) | ★★★★★ | Valor | Authored |

### Metas

| ID | Name | Mechanism | Answer Length | Status |
|----|------|-----------|-------------- |--------|
| META-I | THE WELL | Allomancy table: Pulling metals, quadrant-depth extraction | 5 | Designed |
| META-II | THE OATHS | Radiant Oaths: Orders with 3+ Ideals, Ideal-count indexing | 5 | Designed |
| META-III | ADONALSIUM | 4 missing Shards: first-letter anagram | 4 | Designed |
| SUPER | THE SEVENTEENTH SHARD | Realms cycle (1-2-3-1-2), positions (1-2-3-4-5) from 3 meta answers | 5 | Designed |

## Answer Registry

Answers stored in `meta/cosmere-answers.md` (ROT13 encoded). No plaintext answers in git-tracked files.

## Meta Architecture Summary

- Round 1 (Scadrial) meta answer feeds super-meta positions 1 and 4
- Round 2 (Roshar) meta answer feeds super-meta positions 2 and 5
- Round 3 (Cosmere) meta answer feeds super-meta position 3
- Super-meta extraction cycles through Realms: Physical-Cognitive-Spiritual-Physical-Cognitive
- All verification chains confirmed (see `meta/cosmere-answers.md`)

## File Index

```
scenarios/cosmere/
├── SCENARIO.md          <- scenario brief (read-only)
├── CLAUDE.md            <- THIS FILE (status tracker)
├── SCOPE.md             <- Stage 1 deliverable
├── PUZZLES.md           <- Stage 4 (36 puzzle briefs)
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
├── meta/
│   ├── SUPER-META.md       <- Stage 5: THE SEVENTEENTH SHARD
│   ├── META-I-WELL.md      <- Stage 5: Scadrial round meta
│   ├── META-II-OATHS.md    <- Stage 5: Roshar round meta
│   ├── META-III-ADONALSIUM.md <- Stage 5: Cosmere round meta
│   └── cosmere-answers.md  <- ROT13 answer registry (all 40 answers)
├── puzzles/             <- Stage 6 (authoring)
├── reviews/             <- editorial + panel output
├── tests/               <- blind test results
└── delivery/            <- Stage 9
```
