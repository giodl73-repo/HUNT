# Puzzles — Wololo Master Registry

**Stage 4: ASSIGNMENT**

---

## Hunt Identity

| Field | Value |
|-------|-------|
| **Hunt name** | Wololo |
| **Content library** | Age of Empires game knowledge (AoE2 primary) |
| **Rounds** | 1 |
| **Puzzles per round** | 5 |
| **Total puzzles** | 5 + 1 meta |
| **Metas** | 1 (crossword → WOLOLO) |
| **Format** | Printable Markdown / PDF |
| **Audience** | Solo AoE player |
| **Estimated solve time** | 2-3 hours |

---

## Puzzle Registry

### Status codes

| Code | Meaning |
|------|---------|
| `—` | Not started |
| `BRIEF` | Brief written |
| `AUTHORED` | Full puzzle written |
| `TESTING` | Sent to testers |
| `PASS` | Passed testing (≥22/30) |
| `REVISE` | Failed testing |
| `SHIP` | Ready for integration |

### The 5 Puzzles

| ID | Age | Domain | Puzzle type | Answer (encoded) | Difficulty | Status | Score | Tester 1 | Tester 2 | Tester 3 |
|----|-----|--------|-------------|-----------------|------------|--------|-------|----------|----------|----------|
| I | Dark | Civilizations | Bonus Matcher — match 8 civ bonuses, interlocking pairs | (see ANSWERS.md) | 2 | BRIEF | — | Rosenthal | Miller | Dana |
| II | Feudal | Units | Tournament Bracket — 8-unit elimination, predict winners by counter logic | (see ANSWERS.md) | 2 | BRIEF | — | Katz | Selinker | Blow |
| III | Castle | Technologies | Tech Tree Gap-Fill — 6 missing techs in dependency chain | (see ANSWERS.md) | 3 | BRIEF | — | Huang | Snyder | Gottlieb |
| IV | Imperial | Maps | Resource Map — plot spawn locations, dots trace letters | (see ANSWERS.md) | 3 | BRIEF | — | Sarrett | Miller | Dana |
| V | Post-Imp | Strategy | Economy Puzzle — allocate villagers under constraints | (see ANSWERS.md) | 3 | BRIEF | — | Huang | Kenny | Katz |

### Meta

| ID | Mechanism | Feeds from | Answer (encoded) | Status |
|----|-----------|-----------|-----------------|--------|
| ∞ | Crossword — 5 answers fill grid, highlighted squares spell answer | I, II, III, IV, V | (see ANSWERS.md) | BRIEF |

---

## Tester Assignments

Rationale for crew selection:

| ID | Type | Why these testers |
|----|------|------------------|
| I | Identification | Rosenthal (accessibility), Miller (world-as-puzzle), Dana (book test) |
| II | Bracket deduction | Katz (structure), Selinker (fun/narrative), Blow (epiphany) |
| III | Dependency reasoning | Huang (rigor), Snyder (craftsmanship), Gottlieb (systems) |
| IV | Spatial drawing | Sarrett (physical/Chicago Fire), Miller (diegetic), Dana (visual) |
| V | Constrained optimization | Huang (rigor), Kenny (buildability), Katz (calibration) |

---

## Answer Words — Design Constraints

Each answer must:
1. Be an AoE-related word (thematic)
2. Work in the crossword grid (crossing letters must match)
3. Not be guessable from the puzzle domain alone (Surprise the Answer principle)
4. Be encodable in ROT13 for ANSWERS.md

The meta answer WOLOLO constrains the crossword: highlighted squares must spell W-O-L-O-L-O across the 5-word grid. This means at least 2 answer words must contain O and at least 2 must contain L.

Answer words will be chosen during authoring (Stage 6) and added to ANSWERS.md encoded.

---

## Progress Summary

| Metric | Count |
|--------|-------|
| Total puzzles | 5 + 1 meta |
| Briefed | 5 |
| Authored | 0 |
| Tested | 0 |
| Passed | 0 |
| Ship-ready | 0 |

---

## Stage 4 Status: COMPLETE

Ready for Stage 5 (Meta Design) then Stage 6 (Authoring).
