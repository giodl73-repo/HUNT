# IRONFALL — Puzzle Registry

Master registry of all 10 puzzles, 2 round metas, and 1 super-meta. Answer words are ROT13-encoded per answer security protocol.

---

## Answer Word Coordination

All answer words were coordinated before assignment (Principle #20). Each word has a specific letter at a specific position that feeds the super-meta. The extraction position = the puzzle's slot number within its act. Meta answers use position 1.

**Encoding:** ROT13. To decode: A↔N, B↔O, C↔P, D↔Q, etc.

---

## Act I — "The Archive"

### P01 — "Bestiary v3.2 — Complete Stats"

| Field | Value |
|-------|-------|
| **Round** | Act I, Slot 1 |
| **System** | Bestiary |
| **Mechanism** | Grid deduction — 6 enemies with partial stats, solve using relational clues |
| **Aha** | Palette swaps differ in exactly the right stats to disambiguate |
| **Extraction** | Solved enemy names, indexed by clue number, yield answer letters |
| **Answer (ROT13)** | HFURE |
| **Extraction position** | 1st letter of answer |
| **Difficulty** | Easy |
| **Author** | The Methodical |
| **Status** | Assigned |

**Brief:** Present a forum-style bestiary page with 6 enemy stat blocks where some values are replaced with "???". Include 6 relational clues that cross-reference stats across enemies. The solver fills in the grid through deduction (not lookup), then extracts letters from enemy names at specific positions. The puzzle should feel like completing a community wiki page.

---

### P02 — "Forge Guide — by Old_Forge_Fan"

| Field | Value |
|-------|-------|
| **Round** | Act I, Slot 2 |
| **System** | Items & Crafting |
| **Mechanism** | Logic puzzle — incomplete crafting recipes, discover elemental grammar |
| **Aha** | Discovering the crafting rules: same element = typed equipment, opposing = weapon |
| **Extraction** | First letters of 6 crafted item names in correct recipe order |
| **Answer (ROT13)** | DHRYY |
| **Extraction position** | 2nd letter of answer |
| **Difficulty** | Easy-Medium |
| **Author** | The Methodical |
| **Status** | Assigned |

**Brief:** Present a forum crafting guide with 6 recipes. Three recipes are complete (showing both ingredients and the product). Three have missing elements (one missing an ingredient, one missing the product, one missing both). Solver infers the crafting rules from complete examples and fills in the gaps. The crafted item names' first letters in recipe order spell the answer. Include a worksheet for tracking elemental categories.

---

### P03 — "Save File Deep Dive"

| Field | Value |
|-------|-------|
| **Round** | Act I, Slot 3 |
| **System** | Save File + Achievements |
| **Mechanism** | Pattern recognition — decode hex save data, read achievement bitmask |
| **Aha** | The achievement bitmask encodes which Marks are earned; unearned Marks point to letters |
| **Extraction** | Unearned Marks in target save → Mark names → Nth letter (N=Mark#) → answer |
| **Answer (ROT13)** | BEQRE |
| **Extraction position** | 3rd letter of answer |
| **Difficulty** | Medium |
| **Author** | The Speedster |
| **Status** | Assigned |

**Brief:** Present 4 hex dump save files as "archived save data from the forum's save trading thread." Each save file is a formatted hex table with labeled offsets. Provide the save file format reference (from savefile.md). The solver must decode each save's key fields (level, location, achievements, items). One save file meets specific criteria (has beaten the game, has the Developer's Key, is in the Iron Citadel). The unearned Marks in that save → extract Nth letter from each Mark name → answer word.

---

### P04 — "World Map — All Connections"

| Field | Value |
|-------|-------|
| **Round** | Act I, Slot 4 |
| **System** | Locations |
| **Mechanism** | Spatial navigation — follow directional clues across the map graph |
| **Aha** | NPC clues reference locations indirectly; the path spells letters via region names |
| **Extraction** | Regions visited in order → specific letter from each region name → answer |
| **Answer (ROT13)** | FUNQR |
| **Extraction position** | 4th letter of answer |
| **Difficulty** | Medium |
| **Author** | The Speedster |
| **Status** | Assigned |

**Brief:** Present the world map as an ASCII connection graph (from the archive's map page). Include 5 navigation clues, each pointing to a region using in-game references (NPC locations, dungeon features, enemy habitats). Starting from a specified region, follow each clue to determine the next destination. Each visited region contributes a specific letter (Nth letter of the region name, where N = step number). The 5 letters spell the answer.

---

### P05 — "Battle Damage Calculator v2"

| Field | Value |
|-------|-------|
| **Round** | Act I, Slot 5 |
| **System** | Battle System |
| **Mechanism** | Formula inference — deduce the damage formula from battle logs |
| **Aha** | Discovering the formula structure (ATK × 2 - DEF) × level multiplier × elemental multiplier |
| **Extraction** | Calculated missing values → A1Z26 → answer letters |
| **Answer (ROT13)** | NAIVY |
| **Extraction position** | 5th letter of answer |
| **Difficulty** | Medium-Hard |
| **Author** | The Skeptic |
| **Status** | Assigned |

**Brief:** Present a forum thread titled "I'm trying to figure out the damage formula" by user BattleMath_99. Show 5 battle logs. Three logs have all values visible (training data). Two logs each have one value hidden ("?" — one missing the damage result, one missing the ATK stat). The solver must infer the formula from complete logs, apply it to calculate missing values. Each missing value → A1Z26 → letter. Concatenated letters (plus additional extractions from the formula pattern) spell the answer.

---

## Act II — "The Game"

### P06 — "Enemy Sightings — Unconfirmed"

| Field | Value |
|-------|-------|
| **Round** | Act II, Slot 1 |
| **System** | Bestiary + Lore |
| **Mechanism** | Identification — match narrative descriptions to bestiary entries |
| **Aha** | Behavioral clues + one visible stat narrow to exactly one enemy |
| **Extraction** | Identified enemy names → Nth letter of each name (N=sighting number) → answer |
| **Answer (ROT13)** | YBGHF |
| **Extraction position** | 1st letter of answer |
| **Difficulty** | Medium |
| **Author** | The Skeptic |
| **Status** | Assigned |

**Brief:** Present 5 "unconfirmed sighting reports" from forum users who encountered enemies but couldn't identify them. Each report gives: (1) a narrative description of behavior, (2) the region where it was seen, (3) one visible stat (e.g., "it took 3 hits from my Lv.20 Rynn to kill" → inferrable HP). The solver cross-references bestiary data to identify each enemy uniquely. Extract the Nth letter of each enemy's name (N = sighting number 1-5).

---

### P07 — "The Perfect Build"

| Field | Value |
|-------|-------|
| **Round** | Act II, Slot 2 |
| **System** | Items & Crafting + Bestiary |
| **Mechanism** | Chain logic — work backwards from loadout requirements to enemy farming plan |
| **Aha** | The crafting chain creates dependencies that must be resolved in a specific order |
| **Extraction** | Base-material enemies in chain order → first letters → answer |
| **Answer (ROT13)** | BEOVG |
| **Extraction position** | 2nd letter of answer |
| **Difficulty** | Medium-Hard |
| **Author** | The Social |
| **Status** | Assigned |

**Brief:** A forum post titled "Optimal Endgame Loadout?" asks which 4 crafted items maximize stats for the Iron Citadel. Given the stat requirements (need ATK > X, DEF > Y, resist Dark), solver determines the best loadout. Then works backward: crafted items → ingredients → enemy drops → which enemies to farm. The 5 base enemies needed, ordered by bestiary number, contribute letters at specific positions to spell the answer.

---

### P08 — "100% Completion Guide — Mark Tracker"

| Field | Value |
|-------|-------|
| **Round** | Act II, Slot 3 |
| **System** | Achievements |
| **Mechanism** | Interactive — fake achievement screen with real + fake Marks |
| **Aha** | Comparing the displayed Marks against the known list reveals imposters |
| **Extraction** | Fake Mark names → specific letters → answer |
| **Answer (ROT13)** | RZORE |
| **Extraction position** | 3rd letter of answer |
| **Difficulty** | Medium |
| **Author** | The Social |
| **Status** | Assigned |

**Brief:** Present a "100% Completion Screenshot" showing a 4x4 achievement grid. It looks like the standard Mark of Mastery screen, but 5 of the 16 entries have been subtly altered — either the name is slightly different, the description doesn't match, or the unlock condition is wrong. The solver compares against the archive's achievement list (from achievements.md) to find the 5 fakes. Extract the 3rd letter from each fake Mark's name. This is the UX/interactive component: design the achievement screen as a visual element (ASCII art or styled grid).

---

### P09 — "The Speedrunner's Route"

| Field | Value |
|-------|-------|
| **Round** | Act II, Slot 4 |
| **System** | Locations + Items |
| **Mechanism** | Pathfinding — shortest route collecting required items with dependencies |
| **Aha** | The obvious shortest path fails because of item dependencies (keys needed for access) |
| **Extraction** | Regions in optimal visit order → specific letter from each → answer |
| **Answer (ROT13)** | TYRNZ |
| **Extraction position** | 4th letter of answer |
| **Difficulty** | Hard |
| **Author** | The Lurker |
| **Status** | Assigned |

**Brief:** A forum post from SpeedQueen: "What's the fastest route to get from the start to the Iron Citadel with the Developer's Key?" The solver must plan a route that collects all required items (keys for region access, Developer's Key from secret boss) while minimizing the number of region transitions. The dependency chain: some regions require items found in other regions. The optimal route visits 5 specific regions in a specific order. Each region contributes its Nth letter (N = visit order position).

---

### P10 — "Anomaly in the Code"

| Field | Value |
|-------|-------|
| **Round** | Act II, Slot 5 |
| **System** | Battle System + Save File |
| **Mechanism** | Anomaly detection — find the corrupted battle log entry |
| **Aha** | One battle record has been deliberately altered; finding the wrong value reveals a coded message |
| **Extraction** | Correct-vs-wrong value difference → specific conversion → answer letters |
| **Answer (ROT13)** | URYVK |
| **Extraction position** | 5th letter of answer |
| **Difficulty** | Hard |
| **Author** | The Lurker |
| **Status** | Assigned |

**Brief:** A "datamined" collection of 8 battle records, presented as raw data tables. Seven follow the damage formula exactly. One has a single altered value. The solver applies the formula to all 8 records, identifies the anomaly, calculates what the value should be, and determines the difference. The anomalous record + the nature of the corruption encodes the answer. Design for maximum detective satisfaction — the solver should feel like a dataminer finding proof that the developer changed something on purpose.

---

## Round Metas

### META-I — "The Archive Index"

| Field | Value |
|-------|-------|
| **Round** | Act I meta |
| **System** | Cross-system |
| **Mechanism** | The 5 Act I answer words fill a grid; highlighted positions spell the meta answer |
| **Answer (ROT13)** | ERNYZ |
| **Extraction position** | 1st letter of answer |

**Design:** The 5 Act I answers (ROT13: HZOEN, DHRYY, BEQRE, FUNQR, NAIVY) are placed in a 5-row grid. Column positions marked with archive-themed clues (page edit counts → mod 5 + 1) identify which letter from each word to read. The extracted letters spell the meta answer. Solvable with 4 of 5 answers (80% rule) because one letter can be inferred from crossing constraints.

---

### META-II — "The Game Secrets"

| Field | Value |
|-------|-------|
| **Round** | Act II meta |
| **System** | Cross-system |
| **Mechanism** | The 5 Act II answer words interact with game data to produce the meta answer |
| **Answer (ROT13)** | YRTVG |
| **Extraction position** | 1st letter of answer |

**Design:** The 5 Act II answers (ROT13: YBGHF, BEOVG, RZORE, TYRNZ, URYVK) are displayed on a mock "Game Secrets" screen. Each secret name has one letter highlighted by a discovery marker. Reading the highlighted letters in display order spells the meta answer. Solvable with 4 of 5.

---

### SUPER-META — "The True Ending"

| Field | Value |
|-------|-------|
| **Round** | Super-meta |
| **System** | All |
| **Mechanism** | 12 answer words → extract one letter each → 12 button cheat code |

**Design:** Each answer word contributes one letter at the position matching its slot number. The 12 letters spell the cheat code in button abbreviations: H-H-Q-Q-Y-E-Y-E-O-N-K-L (ROT13 encoded).

The code sequence, when entered on the IRONFALL title screen, triggers the True Ending.

---

## Author Assignments

| Author | Personality | Puzzles | Notes |
|--------|-------------|---------|-------|
| The Methodical | Slow, thorough, verbose | P01, P02 | Grid deduction + logic — plays to thoroughness |
| The Speedster | Fast, error-prone, energetic | P03, P04 | Pattern recognition + spatial — plays to speed |
| The Skeptic | Challenges briefs, deviates | P05, P06 | Formula inference + identification — will challenge mechanisms |
| The Social | Collaborative, messy notes | P07, P08 | Chain logic + interactive — benefits from collaboration style |
| The Lurker | Silent, no notes, precise | P09, P10 | Pathfinding + anomaly detection — precision required |
