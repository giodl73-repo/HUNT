# IRONFALL Items & Crafting System

Source of truth for all key items and crafting combinations in IRONFALL.

---

## Elements

Six elements govern crafting and combat:

| Element | Symbol | Color | Opposed by |
|---------|--------|-------|-----------|
| Fire | F | Red | Ice |
| Ice | I | Blue | Fire |
| Thunder | T | Yellow | Earth |
| Earth | E | Green | Thunder |
| Dark | D | Purple | Holy |
| Holy | H | White | Dark |

---

## Key Items (20)

Items are found, dropped, or crafted. Each has a name, element, source, and use.

| # | Name | Element | Source | Use | Notes |
|---|------|---------|--------|-----|-------|
| 01 | Vine Whip | Earth | Thornback drop | Crafting ingredient | |
| 02 | Ember Dust | Fire | Cindermoth drop | Crafting ingredient | |
| 03 | Shadow Veil | Dark | Shade Wisp drop | Crafting ingredient | |
| 04 | Oak Shield | Earth | Ironbark drop | Equip: DEF +12 | |
| 05 | Damp Stone | Earth | Moss Crawler drop | Crafting ingredient | |
| 06 | Flame Fang | Fire | Emberfang drop | Crafting ingredient | |
| 07 | Ice Shard | Ice | Frostclaw drop | Crafting ingredient | |
| 08 | Crystal Tear | Ice | Blizzard Wraith drop | Crafting ingredient | Glows in dark areas |
| 09 | Frozen Core | Ice | Tundra Golem drop | Crafting ingredient | |
| 10 | Frost Scale | Ice | Glacier Serpent drop | Crafting ingredient | |
| 11 | Lucky Paw | Ice | Snow Hare drop | Equip: Luck +5 | Rarest standard drop |
| 12 | Knight's Crest | Ice | Permafrost Knight drop | Crafting ingredient | |
| 13 | Storm Feather | Thunder | Thunder Hawk drop | Crafting ingredient | |
| 14 | Spark Thread | Thunder | Volt Spider drop | Crafting ingredient | |
| 15 | Runed Plate | Earth | Stone Sentinel drop | Equip: DEF +20 | |
| 16 | Drake Fang | Thunder | Storm Drake drop | Crafting ingredient | |
| 17 | Gale Charm | Thunder | Wind Caller drop | Equip: SPD +8 | |
| 18 | Warden's Key | Earth | Peak Warden drop | Opens secret passage | Quest item |
| 19 | Obsidian Blade | Dark | Dread Knight drop | Equip: ATK +25 | |
| 20 | Developer's Key | Dark | Morimoto's Shadow drop | ??? | Unlocks secret room in Iron Citadel |

---

## Crafted Items (12)

Crafting uses the Forge in Ashveil Town. Combine two items to produce a third. Rules follow elemental logic.

| # | Name | Recipe | Element | Effect | Notes |
|---|------|--------|---------|--------|-------|
| C01 | Steam Lance | Ember Dust + Ice Shard | Fire+Ice | ATK +18, chance to Stun | Opposing elements = weapon |
| C02 | Thunder Root | Vine Whip + Spark Thread | Earth+Thunder | ATK +15, chance to Paralyze | Opposing elements = weapon |
| C03 | Dawn Mantle | Shadow Veil + Crystal Tear | Dark+Ice | DEF +15, resist Dark | |
| C04 | Inferno Ring | Flame Fang + Ember Dust | Fire+Fire | ATK +10, Fire damage aura | Same element = ring |
| C05 | Glacier Mail | Frozen Core + Frost Scale | Ice+Ice | DEF +18, resist Fire | Same element = armor |
| C06 | Tempest Bow | Storm Feather + Drake Fang | Thunder+Thunder | ATK +22, hits all enemies | Same element = bow |
| C07 | Earthen Wall | Damp Stone + Knight's Crest | Earth+Ice | DEF +25, resist Thunder | |
| C08 | Void Edge | Shadow Veil + Obsidian Blade | Dark+Dark | ATK +30, drains HP | Same element = sword |
| C09 | Ironcrown Helm | Runed Plate + Warden's Key | Earth+Earth | DEF +22, immune to Stun | Same element = helm |
| C10 | Phoenix Plume | Flame Fang + Storm Feather | Fire+Thunder | Revive with full HP | Cross-element = consumable |
| C11 | Moonlight Shard | Crystal Tear + Lucky Paw | Ice+Ice | Reveals hidden paths | Same element = utility |
| C12 | Astral Compass | Developer's Key + Void Essence | Dark+Dark | Points toward True Ending | Requires secret boss drop |

---

## Crafting Rules

1. **Opposing elements** (Fire+Ice, Thunder+Earth, Dark+Holy) → produce a **weapon** with Stun/Paralyze chance
2. **Same element + Same element** → produces typed equipment:
   - Fire+Fire → Ring
   - Ice+Ice → Armor/Mail (or Utility for special)
   - Thunder+Thunder → Bow
   - Earth+Earth → Helm
   - Dark+Dark → Sword (or special)
3. **Cross-element** (non-opposing, non-same) → produces a **consumable** or **utility item**
4. Not all combinations work. Only the 12 recipes above are valid. Invalid combinations return the ingredients.
5. The Forge requires the Forge Key (given by the Ashveil blacksmith after completing the first dungeon).

---

## Extraction Affordance

Item names by first letter: V, E, S, O, D, F, I, C, F, F, L, K, S, S, R, D, G, W, O, D
Crafted item names by first letter: S, T, D, I, G, T, E, V, I, P, M, A

Crafted items in order spell out a discoverable pattern when the right letters are extracted.
