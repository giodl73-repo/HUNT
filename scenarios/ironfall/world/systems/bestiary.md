# IRONFALL Bestiary

Source of truth for all enemies in IRONFALL. 24 enemies across 4 regions. Each enemy has unique stat combinations for deduction.

---

## Stat Definitions

| Stat | Abbrev | Range | Description |
|------|--------|-------|-------------|
| Hit Points | HP | 50-999 | Damage before death |
| Attack | ATK | 10-80 | Physical damage dealt |
| Defense | DEF | 5-60 | Physical damage reduced |
| Speed | SPD | 5-40 | Turn order priority |
| Element | ELE | Fire/Ice/Thunder/Earth/Dark/Holy | Elemental affinity |
| Rare Drop | DROP | item name | 1/32 chance drop |

---

## Region 1: Ashveil Forest (Levels 1-12)

| # | Name | HP | ATK | DEF | SPD | ELE | Rare Drop | Notes |
|---|------|----|-----|-----|-----|-----|-----------|-------|
| 01 | Thornback | 85 | 14 | 12 | 8 | Earth | Vine Whip | Common, first enemy encountered |
| 02 | Cindermoth | 60 | 18 | 6 | 22 | Fire | Ember Dust | Fast, low HP |
| 03 | Shade Wisp | 45 | 22 | 5 | 35 | Dark | Shadow Veil | Fastest in region, fragile |
| 04 | Ironbark | 220 | 12 | 45 | 5 | Earth | Oak Shield | Highest DEF in region |
| 05 | Moss Crawler | 130 | 16 | 18 | 12 | Earth | Damp Stone | Palette swap of Thornback (different HP, DEF) |
| 06 | Emberfang | 95 | 28 | 10 | 18 | Fire | Flame Fang | Palette swap of Cindermoth (higher ATK) |

## Region 2: Glacial Reach (Levels 13-24)

| # | Name | HP | ATK | DEF | SPD | ELE | Rare Drop | Notes |
|---|------|----|-----|-----|-----|-----|-----------|-------|
| 07 | Frostclaw | 180 | 32 | 22 | 15 | Ice | Ice Shard | Standard mid-game enemy |
| 08 | Blizzard Wraith | 120 | 38 | 8 | 30 | Ice | Crystal Tear | High ATK, low DEF |
| 09 | Tundra Golem | 350 | 24 | 50 | 6 | Ice | Frozen Core | Tank enemy, very slow |
| 10 | Glacier Serpent | 200 | 35 | 15 | 28 | Ice | Frost Scale | Fast and strong |
| 11 | Snow Hare | 50 | 10 | 5 | 40 | Ice | Lucky Paw | Fastest enemy in game, very weak |
| 12 | Permafrost Knight | 280 | 30 | 38 | 10 | Ice | Knight's Crest | High HP and DEF |

## Region 3: Stormspire Peaks (Levels 25-36)

| # | Name | HP | ATK | DEF | SPD | ELE | Rare Drop | Notes |
|---|------|----|-----|-----|-----|-----|-----------|-------|
| 13 | Thunder Hawk | 160 | 40 | 12 | 34 | Thunder | Storm Feather | Fast aerial enemy |
| 14 | Volt Spider | 140 | 45 | 14 | 20 | Thunder | Spark Thread | High ATK for level range |
| 15 | Stone Sentinel | 400 | 28 | 55 | 5 | Earth | Runed Plate | Highest DEF in game |
| 16 | Storm Drake | 320 | 50 | 20 | 25 | Thunder | Drake Fang | Mini-boss, high ATK |
| 17 | Wind Caller | 110 | 35 | 10 | 32 | Thunder | Gale Charm | Glass cannon |
| 18 | Peak Warden | 260 | 38 | 30 | 14 | Earth | Warden's Key | Guards secret passage |

## Region 4: The Iron Citadel (Levels 37-50)

| # | Name | HP | ATK | DEF | SPD | ELE | Rare Drop | Notes |
|---|------|----|-----|-----|-----|-----|-----------|-------|
| 19 | Dread Knight | 380 | 55 | 35 | 18 | Dark | Obsidian Blade | Standard endgame enemy |
| 20 | Void Shade | 200 | 60 | 10 | 36 | Dark | Void Essence | Highest ATK, very fragile |
| 21 | Iron Revenant | 500 | 42 | 48 | 8 | Dark | Iron Heart | Highest HP in game |
| 22 | Sanctum Guardian | 450 | 48 | 42 | 12 | Holy | Holy Crest | Only Holy enemy |
| 23 | Ash Phantom | 150 | 52 | 8 | 38 | Dark | Phantom Ring | Glass cannon, second fastest |
| 24 | Morimoto's Shadow | 999 | 80 | 60 | 30 | Dark | Developer's Key | Secret boss, absurd stats |

---

## Design Notes

- No two enemies share the same (ATK, DEF, SPD) triple. This enables unique identification from partial stats.
- Palette swaps (Thornback/Moss Crawler, Cindermoth/Emberfang) share Element but differ in at least 2 stat values.
- Enemy #24 (Morimoto's Shadow) is the developer's secret boss — only accessible via a specific sequence in the Iron Citadel. Its rare drop is needed for the True Ending.
- Rare drops are all unique items that appear in `items.md`.
- The bestiary numbering (01-24) is the in-game bestiary order, which matches encounter order.

## Extraction Affordance

Enemy names can be indexed by position for letter extraction. First letters of all 24 names: T, C, S, I, M, E, F, B, T, G, S, P, T, V, S, S, W, P, D, V, I, S, A, M.
