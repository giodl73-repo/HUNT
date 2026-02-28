# IRONFALL — Round Structure

## Architecture

```
Act I — "The Archive" (5 puzzles + round meta)
  ├── P01: Bestiary deduction grid
  ├── P02: Crafting logic puzzle
  ├── P03: Save file analysis + achievement decoding
  ├── P04: World map navigation
  ├── P05: Battle log analysis
  └── META-I: Archive Index meta
         → Act I answer: a 5-letter word representing a D-pad direction

Act II — "The Game" (5 puzzles + round meta)
  ├── P06: Enemy identification from partial data
  ├── P07: Item combination chain
  ├── P08: Achievement screen interactive puzzle
  ├── P09: Secret area pathfinding
  ├── P10: Boss battle optimization / damage formula reverse-engineering
  └── META-II: Game Secrets meta
         → Act II answer: a 5-letter word representing a D-pad direction

True Ending — Super-Meta
  └── 10 feeder answers + 2 round meta answers → 12-button cheat code
```

---

## Act I — "The Archive"

**Narrative frame:** You are browsing the fan wiki section of ironfall-archive.net. These are the community's carefully maintained reference pages — bestiary databases, item catalogs, save file guides, world maps, and battle analysis.

**Tone:** Community knowledge. Hand-counted data. Forum disagreements. Revision histories. The warmth of thousands of hours of volunteer work.

**Difficulty curve:** Easy → Medium. P01 is the entry point (clear mechanism, guided deduction). P05 is the hardest in Act I (formula inference requires mathematical reasoning).

| Slot | ID | Title | System | Mechanism | Difficulty |
|------|----|-------|--------|-----------|-----------|
| 1 | P01 | "Bestiary v3.2 — Complete Stats" | Bestiary | Grid deduction: 6 enemies, match to stats using partial info | Easy |
| 2 | P02 | "Forge Guide — by Old_Forge_Fan" | Items & Crafting | Logic puzzle: determine crafting outcomes from incomplete rules | Easy-Medium |
| 3 | P03 | "Save File Deep Dive" | Save File + Achievements | Pattern recognition: decode hex save data, read achievement bitmask | Medium |
| 4 | P04 | "World Map — All Connections" | Locations | Spatial puzzle: navigate the map following clues to trace a path that spells letters | Medium |
| 5 | P05 | "Battle Damage Calculator v2" | Battle System | Formula inference: given battle logs with missing values, deduce the damage formula | Medium-Hard |

---

## Act II — "The Game"

**Narrative frame:** You have dived deeper. You are now inside the game itself — or rather, inside the archive's collection of screenshots, recorded gameplay, datamined code, and unsolved mysteries.

**Tone:** Deeper. The community's open questions. The unsolved mysteries. The posts that end with "...I don't know what this means." The feeling of being close to something no one has found.

**Difficulty curve:** Medium → Hard. P06 mirrors P01's mechanism but with harder deduction. P10 is the capstone puzzle (complex formula work + anomaly detection).

| Slot | ID | Title | System | Mechanism | Difficulty |
|------|----|-------|--------|-----------|-----------|
| 1 | P06 | "Enemy Sightings — Unconfirmed" | Bestiary + Lore | Identification: match enemy descriptions to bestiary entries using behavioral clues | Medium |
| 2 | P07 | "The Perfect Build" | Items & Crafting + Lore | Chain logic: craft a specific endgame loadout by working backwards from requirements | Medium-Hard |
| 3 | P08 | "100% Completion Guide" | Achievements | Interactive: a fake achievement screen that the solver clicks/interacts with | Medium |
| 4 | P09 | "The Speedrunner's Route" | Locations + Lore | Pathfinding: find the optimal route that collects specific items in the right order | Hard |
| 5 | P10 | "Anomaly in the Code" | Battle System + Save File | Anomaly detection: one battle log has wrong numbers — find the corrupted value | Hard |

---

## Round Metas

### Act I Meta — "The Archive Index"

The 5 Act I answer words are indexed in some way (first letters, specific positions, or a grid) to produce a 5-letter word. This word represents one of the D-pad directions (UP, DOWN, LEFT, RIGHT — or a word that maps to one).

**Mechanism:** The 5 answer words fill into a grid. Highlighted cells spell the meta answer.
**Design constraint:** Must be solvable with 4 of 5 answers (80% rule).

### Act II Meta — "The Game Secrets"

Same structure as Act I meta but with Act II answer words. Produces a second 5-letter word.

**Mechanism:** Answer words interact with a game-themed extraction (enemy names, item properties, etc.).
**Design constraint:** Same 80% rule.

### Super-Meta — "The True Ending"

The 10 feeder answers + 2 round meta answers produce 12 letters. Each letter maps to a SNES controller button. The 12 buttons in order form the cheat code.

**Mechanism:** Each answer word contributes one specific letter (determined by puzzle slot position). That letter IS a button abbreviation (U, D, L, R, A, B, X, Y for face/D-pad buttons).
**Design constraint:** The code must be a memorable sequence — not random. It should feel like something a developer would embed as a hidden reward.

---

## Answer Word Coordination

All 12 answer words (10 feeders + 2 metas) are coordinated before authoring begins.

**Constraints on answer words:**
1. Each answer word must be 5-7 letters
2. A specific letter position in each word (determined by puzzle slot) must produce the cheat code letter
3. No duplicate answer words
4. Answer words should "surprise the answer" — not be predictable from the puzzle topic
5. Answer words should be real English words

---

## Puzzle-to-Button Mapping

| Puzzle | Slot | Answer Word | Extracted Letter (position) | Button |
|--------|------|-------------|---------------------------|--------|
| P01 | Act I, slot 1 | TBD | TBD | TBD |
| P02 | Act I, slot 2 | TBD | TBD | TBD |
| P03 | Act I, slot 3 | TBD | TBD | TBD |
| P04 | Act I, slot 4 | TBD | TBD | TBD |
| P05 | Act I, slot 5 | TBD | TBD | TBD |
| META-I | Act I meta | TBD | TBD | TBD |
| P06 | Act II, slot 1 | TBD | TBD | TBD |
| P07 | Act II, slot 2 | TBD | TBD | TBD |
| P08 | Act II, slot 3 | TBD | TBD | TBD |
| P09 | Act II, slot 4 | TBD | TBD | TBD |
| P10 | Act II, slot 5 | TBD | TBD | TBD |
| META-II | Act II meta | TBD | TBD | TBD |

This table will be filled during Stage 4 (ASSIGNMENT) and Stage 5 (META DESIGN).
