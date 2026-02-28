# THE GRAND LARCENY — Scenario Status Tracker

## Hunt Summary

| Field | Value |
|-------|-------|
| **Name** | THE GRAND LARCENY |
| **Genre** | Noir heist mystery |
| **Scale** | 4 puzzles + 1 meta |
| **Audience** | Teams of 2-4, mixed experience |
| **Format** | Physical-first (evidence dossier); website for answer submission only |
| **Duration** | 90-120 minutes |
| **Theme** | Art Deco noir, 1947. The Grand Hotel. Stolen painting. |
| **Meta target** | THE METHOD — how the painting was stolen |

## Content Source

Fictional world. All data in `world/` directory. No external source — all puzzle facts verified against world/ tables only.

## Narrator Voice

Documents speak for themselves. Each puzzle IS its document type:
- P1: Police incident report — bureaucratic, formal, terse
- P2: Itemized hotel receipt — accounting format, columns, totals
- P3: Architectural floor plan — drafting notation, clean lines, labels
- P4: Witness statement — transcribed speech, verbal tics, halting

Cover sheet: official police briefing voice. No exclamation marks. No questions. Short sentences. The solver is "you" (the investigator).

## Pipeline Status

| Stage | Status | Deliverable | Notes |
|-------|--------|-------------|-------|
| 1. SCOPE | Done | `SCOPE.md`, `world/WORLD.md`, `CLAUDE.md` | Universe designed, 4 data systems |
| 2. STRUCTURE | Done | `ROUNDS.md`, `world/LOCKED.md` | 1 round, 4+meta, world locked |
| 3. PUZZLE POOL | Done | `PUZZLE-POOL.md` | 4 slots, 3 variants each, panel ranked |
| 4. ASSIGNMENT | Done | `PUZZLES.md` with briefs | 4 answers coordinated for meta |
| 5. META DESIGN | Done | `meta/META-DESIGN.md` | Answer=SLIT, calling card extraction |
| 6. AUTHORING | Done | `puzzles/P1-P4` authored | 4 document-style puzzles complete |
| 7. EDITORIAL | Done | `reviews/editorial-review.md` | All 4+meta approved |
| 8. INTEGRATION | Done | `reviews/integration-check.md` | All cross-refs verified, meta extraction confirmed |
| 9. DELIVERY BUILD | Done | `delivery/` files | Props, print layouts, site spec complete. See delivery notes below. |
| 10. PLATFORM TEST | -- | Test checklists | |
| 11. POLISH | -- | Final pass | |

## World Systems

| System | File | Status |
|--------|------|--------|
| Hotel Layout | `world/systems/layout.md` | LOCKED |
| Staff & Guests | `world/systems/characters.md` | LOCKED |
| Gala Timeline | `world/systems/timeline.md` | LOCKED |
| Hotel Services | `world/systems/services.md` | LOCKED |

## Answer Registry

Answers stored in `.claude/` project memory only. ROT13 encoded. No plaintext in git-tracked files.

## Authors (Stage 6)

| Author | Personality | Puzzles |
|--------|-------------|---------|
| The Methodical | Slow, thorough, precise documentation | P1, P3 |
| The Social | Collaborative, narrative-forward | P2, P4 |

## Stage 9 Delivery Notes

### Props (`delivery/props/`)
- `PROPS-REGISTRY.md` — master inventory of all 10 physical items per team kit
- `dossier-cover.md` — case briefing cover sheet design (PR02), voice calibration, canon verification
- `evidence-log.md` — meta worksheet design (PR07), extraction mechanism reference
- `hotel-key.md` — hotel key card prop (PR08), 3.5x2in, ROOM 303/Roof Access, puzzle function for P3
- `calling-card.md` — calling card prop (PR09), 3x2in cream stock, numbers 3-1-2-6, meta extraction key
- `DISTRIBUTION.md` — per-team kit contents, step-by-step assembly, quantities, distribution protocol, day-of timeline, packing checklist
- `PROPS.md` — earlier comprehensive props document (predates registry split; kept for reference)

### Print (`delivery/print/`)
- `PRINT-SPEC.md` — full print specifications: page size, margins, typography (Courier New / Georgia / Arial Narrow), color palette, header boxes, answer blanks, extraction markers, CSS, file index
- `layout-P1-police-report.md` — layout spec for P1: police header, 4 suspects, physical evidence, extraction markers (7 underlined words with boxed letters), 2 pages
- `layout-P2-hotel-receipt.md` — layout spec for P2: hotel letterhead, 4 guest folios, columnar accounting, cents-to-letter extraction, 2 pages
- `layout-P3-floor-plan.md` — layout spec for P3: architectural survey, 4 floor diagrams, reference markers, connecting door discovery, route log worksheet, 1-2 pages
- `layout-P4-witness-statement.md` — layout spec for P4: transcript format, speaker labels, stage directions, acrostic text, 32-sentence enumeration, 2 pages
- `PRINT-CHECKLIST.md` — assembly order, print counts, supply list (predates PRINT-SPEC; kept for reference)
- `THEME.md` — theme/CSS overview (predates PRINT-SPEC; kept for reference)

### Print-ready HTML (already built)
- `print/puzzles/cover-sheet.html` — cover sheet
- `print/puzzles/P1-police-report.html` — Exhibit A with extraction markers
- `print/puzzles/evidence-log.html` — meta worksheet
- `print/props/labels-dossier.html` — 6 folder labels per sheet
- `print/props/hotel-key-cards.html` — 12 key card fronts per sheet
- `print/props/calling-cards.html` — 12 calling card fronts per sheet

### Still to build (HTML)
- `print/puzzles/P2-hotel-receipt.html` — from P2 source + layout spec
- `print/puzzles/P3-floor-plan.html` — from P3 source + layout spec (needs graphic treatment for diagrams)
- `print/puzzles/P4-witness-statement.html` — from P4 source + layout spec

### Site (`delivery/site/`)
- `SITE-SPEC.md` — minimal site spec: landing page + answer submission form, dark noir aesthetic, no puzzle content
- `index.html` — landing page (built), links to submit form
- `submit.html` — answer submission form (to build, spec in SITE-SPEC.md)

### Outstanding for Stage 10 (Platform Test)
- Build remaining HTML files (P2, P3, P4)
- Build submit.html with answer validation
- Test full print workflow: print all HTML files, cut cards, assemble one test dossier
- Blind test with a volunteer team

## File Index

```
scenarios/grand-larceny/
├── SCENARIO.md          <- scenario brief (read-only)
├── CLAUDE.md            <- THIS FILE (status tracker)
├── SCOPE.md             <- Stage 1 deliverable
├── ROUNDS.md            <- Stage 2
├── PUZZLES.md           <- Stage 4
├── world/
│   ├── WORLD.md         <- universe overview
│   ├── LOCKED.md        <- canon lock log
│   └── systems/         <- data tables
├── meta/                <- Stage 5
├── puzzles/             <- Stage 6
├── reviews/             <- editorial + panel output
├── tests/               <- blind test results
└── delivery/            <- Stage 9
    ├── props/           <- dossier, hotel key, calling card
    ├── print/           <- document-style puzzle layouts
    └── site/            <- answer submission only
```
