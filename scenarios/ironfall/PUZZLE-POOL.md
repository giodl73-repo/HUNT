# IRONFALL — Puzzle Pool

Brainstorm of 18 candidate puzzles. The panel will rank and select 10 for the final hunt.

---

## Act I Candidates — "The Archive" (fan wiki puzzles)

### A1: "Bestiary v3.2 — Complete Stats" (BESTIARY)
**Mechanism:** Grid deduction. A forum post presents 6 enemy entries with partially redacted stats. Clues describe relationships ("the fastest enemy in Ashveil has the lowest HP," "the enemy whose ATK is exactly double its DEF drops the Shadow Veil"). Solver fills in the grid using deduction, not lookup.
**Aha:** Realizing that palette swaps share most stats but differ in exactly the right fields to disambiguate.
**Extraction:** Solved enemy names indexed by a position number yield the answer word.
**System:** bestiary.md
**Difficulty:** Easy

### A2: "Forge Guide — by Old_Forge_Fan" (ITEMS)
**Mechanism:** Logic puzzle. An incomplete crafting guide shows 6 recipes with missing ingredients or products. Solver uses elemental crafting rules (inferred from the 2-3 complete recipes) to deduce the missing entries. Interlock: one recipe's product is another's ingredient.
**Aha:** Discovering the elemental crafting grammar (same element = typed equipment, opposing = weapon, cross = consumable).
**Extraction:** First letters of the 6 crafted item names in the correct order.
**System:** items.md
**Difficulty:** Easy-Medium

### A3: "Save File Deep Dive" (SAVE FILE + ACHIEVEMENTS)
**Mechanism:** Pattern recognition + hex decoding. A forum post shows 4 hex dumps of save files. Solver converts hex fields to game stats, identifies which save file meets specific criteria (has the right Marks, right items, right location). Achievement bitmask reading is the key skill.
**Aha:** Realizing the achievement bitmask encodes which Marks are earned and that the "missing" marks point to specific letters.
**Extraction:** The Marks NOT earned in the target save file → Mark names → specific letter positions → answer word.
**System:** savefile.md, achievements.md
**Difficulty:** Medium

### A4: "World Map — All Connections" (LOCATIONS)
**Mechanism:** Spatial navigation. The forum's world map page shows the connection graph. A series of directional clues ("From the town where Elara lives, go south, then east, then to the region with 5 dungeon floors") traces a path across the map. Each visited region contributes a letter.
**Aha:** The path traces a specific shape on the map, or the regions visited in order spell the answer via their first letters.
**Extraction:** First letter of each region visited in order.
**System:** locations.md
**Difficulty:** Medium

### A5: "Battle Damage Calculator v2" (BATTLE)
**Mechanism:** Formula inference. A forum thread shows 5 battle logs with all values visible except one per log. The solver must infer the damage formula from the complete examples, then use it to calculate the hidden values. The hidden values, converted to letters (A1Z26), spell the answer.
**Aha:** Discovering the damage formula by comparing multiple battles.
**Extraction:** Calculated hidden values → A1Z26 → letters → answer word.
**System:** battle.md
**Difficulty:** Medium-Hard

### A6: "The Complete Rare Drop Table" (BESTIARY + ITEMS)
**Mechanism:** Cross-reference matching. A partial drop table shows 8 enemies and their drops, but the enemy-drop pairings are scrambled. Solver uses bestiary stats + item descriptions to match correctly.
**Aha:** Elemental affinity of the enemy matches the element of its drop.
**Extraction:** Correct pairings → letter from each item name at the position matching the enemy's region number.
**System:** bestiary.md, items.md
**Difficulty:** Medium

### A7: "Forum Census — Who Said What" (LORE)
**Mechanism:** Attribution puzzle. 6 quotes from forum users, each discussing a different aspect of IRONFALL. Match each quote to its author using clues in the writing style and content knowledge.
**Aha:** Each forum user specializes in a specific game system — their quotes reference data only an expert in that system would know.
**Extraction:** Forum usernames in correct order → specific letter positions → answer.
**System:** lore.md
**Difficulty:** Easy-Medium

### A8: "Achievement Checklist — Optimal Order" (ACHIEVEMENTS)
**Mechanism:** Ordering puzzle. Given the 16 Marks with unlock conditions, determine the earliest possible order to earn them (dependency analysis: some Marks require visiting regions, which requires other Marks first).
**Aha:** The dependency chain creates a unique optimal ordering.
**Extraction:** Achievement numbers in optimal order, indexed into achievement names.
**System:** achievements.md, locations.md
**Difficulty:** Medium-Hard

---

## Act II Candidates — "The Game" (in-game puzzles)

### B1: "Enemy Sightings — Unconfirmed" (BESTIARY + LORE)
**Mechanism:** Identification. 6 narrative descriptions of encountering enemies (written as forum "sighting reports"). Each describes behavior, environment, and one visible stat. Solver matches to bestiary entries using behavioral clues + the one visible stat to narrow to a unique enemy.
**Aha:** The behavioral clue eliminates all but 2-3 candidates; the visible stat narrows to exactly one.
**Extraction:** Identified enemy names → letter at position matching the sighting number.
**System:** bestiary.md, lore.md
**Difficulty:** Medium

### B2: "The Perfect Build" (ITEMS + LORE)
**Mechanism:** Chain logic. A forum post asks: "What's the optimal endgame loadout for the Iron Citadel?" Solver must work backwards from desired equipment stats to determine which crafted items are needed, then which base items, then which enemies must be farmed. The chain: enemies → drops → crafting → loadout.
**Aha:** The crafting chain creates dependencies — you need specific drops, which come from specific enemies in specific regions.
**Extraction:** The base enemies in the chain, ordered by region number → first letters → answer.
**System:** items.md, bestiary.md, locations.md
**Difficulty:** Medium-Hard

### B3: "100% Completion Guide — Mark Tracker" (ACHIEVEMENTS)
**Mechanism:** Interactive/UX component. A fake "achievement screen" (styled like the in-game 4x4 grid). Solver clicks/interacts with earned Marks to reveal descriptions. By reading descriptions and cross-referencing with unlock conditions, solver identifies which Marks are fake (not in the real game) vs. real. The fake Marks' names contain the answer.
**Aha:** Comparing the displayed Marks against the archive's achievement list reveals subtle changes.
**Extraction:** Letters from the names of the "fake" marks spell the answer.
**System:** achievements.md
**Difficulty:** Medium

### B4: "The Speedrunner's Route" (LOCATIONS + LORE)
**Mechanism:** Pathfinding optimization. Given a list of items that must be collected from specific regions, and the world map connection graph, find the shortest route that collects all items. The constraint: some items require other items to access (locked doors, key requirements).
**Aha:** The dependency chain means the "obvious" shortest path doesn't work — you need a specific traversal order.
**Extraction:** Regions visited in optimal order → specific letter from each region name → answer.
**System:** locations.md, items.md
**Difficulty:** Hard

### B5: "Anomaly in the Code" (BATTLE + SAVE FILE)
**Mechanism:** Anomaly detection. A "datamined" set of 8 battle records. Seven are correct per the damage formula. One has a single wrong value — a "corruption" in the data. Solver applies the formula to all 8, finds the one that doesn't work, identifies which value is wrong and what it should be. The difference between wrong and right value is the key.
**Aha:** The anomaly reveals a hidden message — the "corrupted" value was changed deliberately by the developer.
**Extraction:** The wrong value and the right value, processed through a conversion, yield the answer.
**System:** battle.md, savefile.md
**Difficulty:** Hard

### B6: "NPC Dialogue Compilation" (LORE)
**Mechanism:** Cipher extraction. All NPC dialogue from lore.md presented as a "compiled dialogue dump." Solver notices that certain words are oddly capitalized or certain sentences have unusual patterns. Extracting the first letter of each Morimoto dialogue sentence spells a message.
**Aha:** The developer hid a message in his own character's dialogue across all appearances.
**Extraction:** First letters of Morimoto's sentences → answer word.
**System:** lore.md
**Difficulty:** Medium

### B7: "Lore Tablet Transcription" (LORE + LOCATIONS)
**Mechanism:** Ordering + extraction. The 12 lore tablets are presented in scrambled order. Solver must determine the correct order (by matching each tablet to its location using content clues, then ordering by region progression).
**Aha:** The tablets form a narrative when read in order — and the first letters of each tablet's first word spell a message.
**Extraction:** First word of each tablet in correct order → first letters → answer.
**System:** lore.md, locations.md
**Difficulty:** Medium

### B8: "Boss Rush — Damage Preview" (BATTLE)
**Mechanism:** Calculation + optimization. For each of 5 bosses, determine which party member should use which special move to deal maximum damage (considering elemental matchups, MAG stats, and the damage formula). The optimal choices produce a sequence of character initials.
**Aha:** The elemental weakness chart is asymmetric (Ice resists Thunder), which changes the optimal strategy for one boss.
**Extraction:** Optimal attacker for each boss → character initial → answer word.
**System:** battle.md, lore.md
**Difficulty:** Medium-Hard

### B9: "The Developer's Room" (ALL SYSTEMS)
**Mechanism:** Meta-puzzle within a puzzle. A description of Morimoto's Lab with objects on shelves — each object references a different game system. Solver identifies each object, looks up its data in the corresponding system, and extracts a letter from each.
**Aha:** The room is a miniature version of the entire game — each shelf is one system.
**Extraction:** One letter per shelf/system → answer word.
**System:** All
**Difficulty:** Hard

### B10: "Glitch Gallery" (SAVE FILE + BATTLE)
**Mechanism:** Error correction. 5 "screenshots of glitches" — save file displays or battle screens with incorrect values. Solver identifies what's wrong in each, determines the correct value, and extracts letters.
**Aha:** Each "glitch" is a specific, identifiable error that points to a specific correct value.
**Extraction:** Correct values replace glitched ones → letters → answer.
**System:** savefile.md, battle.md
**Difficulty:** Medium-Hard

---

## Panel Ranking

The 12-expert panel evaluates each candidate on:
1. **Riven Standard** — is the puzzle what the game system does?
2. **Mechanism clarity** — one aha, clean extraction
3. **Deduction vs. computation** — does it require insight, not just execution?
4. **Interlock** — do clues cross-reference within the puzzle?
5. **Dinner Party factor** — would you tell someone about this at dinner?

### Consensus Rankings

**LOCKS (8+ of 12 panelists):**
- A1 (Bestiary grid) — clean mechanism, great entry puzzle
- A2 (Forge guide) — elegant elemental logic discovery
- A3 (Save file) — unique, the hex reading is a genuine aha
- A5 (Battle calculator) — strong formula inference, avoids pure computation
- B1 (Enemy sightings) — identification puzzle that IS what naturalists do
- B3 (100% guide) — the interactive component, very exciting
- B4 (Speedrunner's route) — satisfying optimization with a twist
- B5 (Anomaly in the code) — great capstone, detective work

**CONTENDERS (4-7 panelists):**
- A4 (World map) — good but might be too simple for the mechanism
- B2 (Perfect build) — chain logic is strong but might feel like homework
- B6 (NPC dialogue) — cipher extraction is clean but might be too easy once seen
- B8 (Boss rush) — risk of pure computation without enough deduction

**CUT (0-3 panelists):**
- A6 (Rare drop table) — too similar to A1
- A7 (Forum census) — attribution puzzles often feel arbitrary
- A8 (Achievement order) — dependency analysis is more CS than puzzling
- B7 (Lore tablets) — ordering puzzle is a bit mechanical
- B9 (Developer's room) — too meta, might confuse scope
- B10 (Glitch gallery) — too similar to B5

---

## Selected 10

Based on panel rankings and need for variety:

| Slot | ID | Candidate | Reason for selection |
|------|----|-----------|---------------------|
| Act I-1 | P01 | A1 Bestiary grid | Lock. Clean entry puzzle. |
| Act I-2 | P02 | A2 Forge guide | Lock. Elegant mechanism discovery. |
| Act I-3 | P03 | A3 Save file | Lock. Unique hex-reading aha. |
| Act I-4 | P04 | A4 World map | Contender promoted. Spatial variety needed. Mechanism strengthened with NPC clue interlock. |
| Act I-5 | P05 | A5 Battle calculator | Lock. Strong formula inference. |
| Act II-1 | P06 | B1 Enemy sightings | Lock. Identification IS the game system. |
| Act II-2 | P07 | B2 Perfect build | Contender promoted. Chain logic adds depth. Streamlined to avoid homework feel. |
| Act II-3 | P08 | B3 100% guide | Lock. The interactive component. |
| Act II-4 | P09 | B4 Speedrunner's route | Lock. Satisfying optimization. |
| Act II-5 | P10 | B5 Anomaly in the code | Lock. Capstone detective work. |
