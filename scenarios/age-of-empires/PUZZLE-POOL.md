# Puzzle Pool — Wololo

**Stage 3: PUZZLE POOL** — 3 candidates per slot. The panel ranks them.

---

## Slot I — Dark Age (Civilizations)

### I-A: Bonus Matcher ★
**Pitch:** Match 8 unique civilization bonuses to their civilizations.
**Mechanism:** 8 bonuses listed (e.g., "Shepherds work 25% faster," "Start with +200 food"). 8 civilization names. The solver matches them. But 3 pairs share similar bonuses (Mongols/Huns cavalry bonuses) — requires precise knowledge, not guessing. Extraction: matched civ names, specific letters indexed by bonus number.
**Difficulty:** 2
**Interlock:** Two bonuses reference each other ("this civilization's team bonus helps the answer to Clue 6").

### I-B: Wonder Identifier
**Pitch:** Name the civilization from a description of its Wonder.
**Mechanism:** 6 Wonders described architecturally (not by name). "A stepped pyramid with a temple at the summit" → Aztecs (Great Pyramid of Tenochtitlan). Solver identifies the civ from the Wonder. First letters of civ names spell the answer.
**Difficulty:** 2
**Dinner Party potential:** "I identified civilizations from their Wonder architecture."

### I-C: Unique Unit Lineup
**Pitch:** A police lineup of unique units — identify who belongs to whom.
**Mechanism:** 7 unique units described by stats and abilities only (no names). "A cavalry archer that fires while moving, costs no gold." → Mangudai → Mongols. Match unit to civ. Extract from civ names.
**Difficulty:** 3
**Riven Standard:** This IS what AoE players do — recognize units and know their civs.

---

## Slot II — Feudal Age (Units)

### II-A: Counter Circle
**Pitch:** Trace a counter chain — what beats what beats what.
**Mechanism:** 6 units in a circular counter relationship (archers → pikemen → cavalry → archers, but with specific named units). The solver traces the cycle, but one link is BROKEN — identifying the break reveals the answer. The break is the puzzle.
**Difficulty:** 2
**Interlock:** The broken link connects to a unit from Puzzle I.

### II-B: Rock-Paper-Scissors Tournament ★
**Pitch:** A tournament bracket where each matchup is a unit-vs-unit fight. Predict the winner using counter logic.
**Mechanism:** 8 units in a single-elimination bracket. The solver must know which unit type counters which (infantry < cavalry < archers < infantry, plus exceptions for unique units). Predict all 7 matchup winners. The winners' unit-type initials spell the answer. One matchup has a trick — a unique unit that breaks the normal counter rules.
**Difficulty:** 2
**Dinner Party:** "I predicted an 8-unit tournament bracket using Age of Empires counter logic."

### II-C: Upgrade Path
**Pitch:** Trace a unit's upgrade path from Dark to Imperial, filling in the missing upgrades.
**Mechanism:** A chain like Militia → Man-at-Arms → ??? → ??? → Champion. 5 upgrade chains with missing links. The solver fills them in. Missing unit names, with specific letters extracted, spell the answer.
**Difficulty:** 2

---

## Slot III — Castle Age (Technologies)

### III-A: Tech Tree Gap-Fill ★
**Pitch:** Six technologies are missing from a research tree. Fill the gaps.
**Mechanism:** An ASCII tech tree showing dependencies (Loom → ??? → Sappers, or Fletching → Bodkin Arrow → ???). 6 blanks. Each requires knowing the AoE tech tree. The missing tech names, first letters extracted, spell the answer.
**Difficulty:** 3
**Interlock:** Two techs have dependencies on each other — filling one constrains the other.
**Riven Standard:** This IS the AoE tech tree — the solver navigates it like they would in-game.

### III-B: Research Cost Detective
**Pitch:** Identify technologies from their research costs alone.
**Mechanism:** 6 tech costs given as Food/Wood/Gold/Stone values (e.g., "200F 100G" → Wheelbarrow). The solver identifies each tech. A harder variant: the costs are from a specific civ that gets a discount — the solver must figure out WHICH civ and the base costs.
**Difficulty:** 3

### III-C: Age Advancement Requirements
**Pitch:** What do you need to reach each Age? The requirements ARE the puzzle.
**Mechanism:** For each Age transition (Dark→Feudal, Feudal→Castle, Castle→Imperial), list the REAL requirements (building count, resource cost). But present them scrambled — the solver must match the right requirements to the right transition. The transitions, correctly ordered with their building requirements, encode the answer through building name initials.
**Difficulty:** 2

---

## Slot IV — Imperial Age (Maps)

### IV-A: Map Identifier
**Pitch:** Identify 5 standard maps from terrain descriptions.
**Mechanism:** 5 poetic descriptions of map types ("Two landmasses separated by a shallow strait, with fish on both sides" → Mediterranean). Solver names each map. First letters or indexed letters spell the answer.
**Difficulty:** 2

### IV-B: Resource Map ★
**Pitch:** Plot resources on a blank map — the pattern spells a letter.
**Mechanism:** The Monk describes where resources spawn on a specific map type. "Gold appears at 2 o'clock and 8 o'clock. Stone at 11 and 5. Berries due north." The solver plots these on a blank circular map. The plotted resources trace letter shapes. 5 maps → 5 letters → answer.
**Difficulty:** 3
**Physical:** The solver DRAWS on the page — same as the Star Chart in the reference project.
**Dinner Party:** "I plotted resource spawns on Age of Empires maps and the dots spelled a word."
**Selinker upgrade candidate:** THIS is the Dinner Party puzzle for the hunt.

### IV-C: Terrain Puzzle
**Pitch:** A grid of terrain types — navigate from Town Center to enemy, collecting letters.
**Mechanism:** An ASCII grid showing forest, water, hills, open ground. The solver must find the valid path (units can't cross deep water, cavalry is slow in forest, etc.). Letters along the valid path spell the answer.
**Difficulty:** 2
**Interlock:** The optimal path depends on which unit type you're moving — connects to Puzzle II.

---

## Slot V — Post-Imperial (Strategy)

### V-A: Build Order Sequence
**Pitch:** Put 8 build-order steps in the correct sequence to reach Castle Age.
**Mechanism:** 8 steps given out of order (e.g., "Send 3 villagers to gold," "Build a barracks," "Research Loom," "Click up to Feudal"). Solver orders them. First letters in correct order spell the answer.
**Difficulty:** 3
**Rosenthal flag:** Competitive knowledge — alienates casual players.

### V-B: Economy Puzzle ★
**Pitch:** Balance your economy — how many villagers on each resource to afford a specific army.
**Mechanism:** The Monk says: "You need 10 Knights (60F 75G each) and a Castle (650S) in 10 minutes. Your villagers gather 25 food/min on farms, 20 gold/min on mining, 18 stone/min on quarry." The solver does resource math to figure out the villager allocation. But there are CONSTRAINTS — you only have 40 villagers total. The optimal allocation, expressed as numbers on each resource, maps via A1Z26 to letters.
**Difficulty:** 3
**Riven Standard:** This IS AoE strategy — resource management under constraints.
**Rosenthal fix:** Economy math is more accessible than build-order memorization. Anyone who's played 10 games understands villager allocation.

### V-C: Historical Battle Reconstruction
**Pitch:** Reconstruct a famous AoE campaign scenario from clues.
**Mechanism:** Descriptions of 6 historical events (matching AoE campaign missions). "A Spanish conquest of an island civilization" → El Dorado / Montezuma. Identify each, extract from scenario names.
**Difficulty:** 2
**Low interlock potential.**

---

## Cross-Slot Candidates (bonus — for potential inclusion)

### X-1: Taunt Decoder
**Pitch:** Decode the in-game taunt numbers — "11" means something specific.
**Mechanism:** 6 taunt numbers given. The solver must know (or look up) what each taunt says ("11" = "Don't resign," "2" = "Yes"). The taunt texts, first letters extracted, spell a word.
**Fun factor:** Very high for AoE players. Pure nostalgia.

### X-2: Sound Effect Identification
**Pitch:** Descriptions of iconic AoE sounds — identify what makes each sound.
**Mechanism:** "A rising choral chord" → Age advancement. "A deep horn blast" → Attack alarm. "A single spoken word that changes everything" → Wololo (but don't use this as it's the meta answer).

---

## Pool Summary

| Slot | Candidates | My pick | Why |
|------|-----------|---------|-----|
| I | Bonus Matcher, Wonder Identifier, Unique Unit Lineup | I-A (Bonus Matcher) | Interlock, accessible, Riven Standard |
| II | Counter Circle, Tournament Bracket, Upgrade Path | II-B (Tournament) | Dinner Party, the trick matchup adds deduction |
| III | Tech Tree Gap-Fill, Research Cost, Age Requirements | III-A (Tech Tree) | Interlock, Riven Standard, IS the tech tree |
| IV | Map Identifier, Resource Map, Terrain Puzzle | IV-B (Resource Map) | Physical drawing, Dinner Party, Selinker's upgrade |
| V | Build Order, Economy Puzzle, Historical Battle | V-B (Economy) | Rosenthal-safe, Riven Standard, real strategy |

The panel will rank. Stars (★) are my picks.

---

## Stage 3 Status: COMPLETE

Ready for review gate → panel ranks the pool.
