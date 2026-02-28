# IRONFALL Achievements — Marks of Mastery

Source of truth for the 16 in-game achievements. Called "Marks of Mastery" in-game — displayed as pixel-art badges on the save file screen.

---

## Marks of Mastery

| # | Mark Name | Description | Unlock Condition | Category |
|---|-----------|-------------|-----------------|----------|
| 01 | Seedling | A journey begins with a single step. | Complete the Ashveil tutorial. | Progress |
| 02 | Trailblazer | Every path has a first footprint. | Visit all 6 regions on the world map. | Exploration |
| 03 | Ashen Victor | The forest bows to the strong. | Defeat all 6 Ashveil Forest enemies. | Combat |
| 04 | Relentless | Cold cannot stop the determined. | Defeat all 6 Glacial Reach enemies. | Combat |
| 05 | Tempest Born | You are the storm now. | Defeat all 6 Stormspire Peaks enemies. | Combat |
| 06 | Ironclad | The citadel falls. | Defeat all 6 Iron Citadel enemies. | Combat |
| 07 | Artisan | Creation is its own reward. | Craft 6 different items at the Forge. | Crafting |
| 08 | Collector | Every piece matters. | Obtain all 20 key items. | Collection |
| 09 | Night Walker | Some doors open only in darkness. | Find all 6 secret areas. | Exploration |
| 10 | Ghost Runner | Speed is truth. | Complete the game in under 4 hours. | Speed |
| 11 | Scholar | The old words still have power. | Read all 12 lore tablets scattered across the world. | Lore |
| 12 | Loyal Heart | The party stands together. | Complete all 4 character backstory quests. | Story |
| 13 | Oathbreaker | Some promises were meant to be broken. | Betray an NPC during the Stormspire quest. | Story |
| 14 | Unseen | They never knew you were there. | Complete Glacial Reach without entering combat. | Stealth |
| 15 | Legacy | The developer left something behind. | Defeat Morimoto's Shadow. | Secret |
| 16 | Transcendent | You found the way. | Trigger the True Ending. | Secret |

---

## Mark Badge Display

On the save file screen, Marks are displayed as a 4x4 grid of pixel badges:

```
┌────┬────┬────┬────┐
│ 01 │ 02 │ 03 │ 04 │
├────┼────┼────┼────┤
│ 05 │ 06 │ 07 │ 08 │
├────┼────┼────┼────┤
│ 09 │ 10 │ 11 │ 12 │
├────┼────┼────┼────┤
│ 13 │ 14 │ 15 │ 16 │
└────┴────┴────┴────┘
```

Earned marks glow gold. Unearned marks are greyed out with "???" for name and description.

---

## Extraction Affordances

**First letters of Mark names:** S, T, A, R, T, I, A, C, N, G, S, L, O, U, L, T

Reading positions 1, 2, 5, 6, 10, 14 gives: S, T, T, I, G, U — not directly useful, but the full sequence of 16 first letters has embedded patterns.

**Nth-letter extraction from Mark names:**
| Mark | Name | Position (= Mark #) | Letter |
|------|------|---------------------|--------|
| 01 | Seedling | 1st letter | S |
| 02 | Trailblazer | 2nd letter | R |
| 03 | Ashen Victor | 3rd letter | H |
| 04 | Relentless | 4th letter | E |
| 05 | Tempest Born | 5th letter | E |
| 06 | Ironclad | 6th letter | L |
| 07 | Artisan | 7th letter | — (only 7 letters, so last = N) |
| 08 | Collector | 8th letter | R |
| 09 | Night Walker | 9th letter | E |
| 10 | Ghost Runner | 10th letter | R |
| 11 | Scholar | 11th letter — wraps: 11 mod 7 = 4 | L |
| 12 | Loyal Heart | 12th letter — wraps: 12 mod 10 = 2 | O |

**Design note:** The primary extraction mechanism for achievement puzzles uses the Mark number to index into the Mark name (with wrapping for short names). This produces hidden letters that spell parts of the answer.

---

## Category Summary

| Category | Count | Marks |
|----------|-------|-------|
| Progress | 1 | 01 |
| Exploration | 2 | 02, 09 |
| Combat | 4 | 03, 04, 05, 06 |
| Crafting | 1 | 07 |
| Collection | 1 | 08 |
| Speed | 1 | 10 |
| Lore | 1 | 11 |
| Story | 2 | 12, 13 |
| Stealth | 1 | 14 |
| Secret | 2 | 15, 16 |
