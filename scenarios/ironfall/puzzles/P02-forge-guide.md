# P02 — Forge Guide — by Old_Forge_Fan

*Author: The Methodical*

---

> The forge does not lie. Old_Forge_Fan tested every combination. Three years of notes. The rules survive even when the data does not.

---

## ironfall-archive.net/guides/crafting

Study the complete recipes to learn the elemental crafting rules. Then fill in the incomplete ones.

### Recipes

| # | Ingredient 1 | (Ele) | Ingredient 2 | (Ele) | Product | Type |
|---|-------------|-------|-------------|-------|---------|------|
| 1 | Damp Stone | Earth | Knight's Crest | Ice | Earthen Wall | ??? |
| 2 | Runed Plate | Earth | Warden's Key | Earth | ??? | Helm |
| 3 | Ember Dust | Fire | Ice Shard | Ice | Steam Lance | Weapon |
| 4 | Storm Feather | Thunder | Drake Fang | Thunder | Tempest Bow | ??? |
| 5 | Spark Thread | Thunder | ??? | ??? | Thunder Root | Weapon |

### Community Notes

**Old_Forge_Fan:** "Same element + same element = typed equipment. I've confirmed: Fire+Fire→Ring, Ice+Ice→Armor, Thunder+Thunder→Bow, Dark+Dark→Sword, Earth+Earth→Helm."

**DarkKnight_97:** "Opposing elements (Fire vs Ice, Thunder vs Earth, Dark vs Holy) always make a Weapon with a status effect."

**BattleMath_99:** "Cross-element (not same, not opposing) combinations produce utility items or special equipment. Earth+Ice made the Earthen Wall — it's a shield, not a weapon."

**SpeedQueen:** "Recipe 5 needs an Earth-element ingredient. Thunder+Earth = opposing elements = Weapon. The missing ingredient is Vine Whip."

### Known Crafted Items

Steam Lance, Thunder Root, Dawn Mantle, Inferno Ring, Glacier Mail, Tempest Bow, Earthen Wall, Void Edge, Ironcrown Helm, Phoenix Plume, Moonlight Shard, Astral Compass

### Working Space

```
Recipe 1: Earth + Ice (cross-element) → Earthen Wall → Type: ___________
Recipe 2: Earth + Earth (same) → Product: ___________ → Type: Helm (given)
Recipe 3: Fire + Ice (opposing) → Steam Lance → Type: Weapon (given)
Recipe 4: Thunder + Thunder (same) → Tempest Bow → Type: ___________
Recipe 5: Thunder + ___________ (opposing) → Thunder Root → Type: Weapon (given)
  Missing ingredient: ___________ (Element: _____)
```

### Solutions

```
Recipe 1 type: Shield/Wall (cross-element utility — not Weapon, not typed equipment)
Recipe 2 product: Ironcrown Helm (Earth+Earth → Helm per the rule)
Recipe 4 type: Bow (Thunder+Thunder → Bow per the rule)
Recipe 5 missing ingredient: Vine Whip (Earth element — Thunder+Earth = opposing = Weapon)
```

---

### Hex Archive — Crafted Item Database

Old_Forge_Fan appended a full hex dump of the game's internal item database to every version of his guide. He was meticulous that way.

| Crafted Item | Internal Hex ID |
|-------------|----------------|
| Steam Lance | 0x05 |
| Thunder Root | 0x0C |
| Dawn Mantle | 0x1B |
| Inferno Ring | 0x0E |
| Glacier Mail | 0x13 |
| Tempest Bow | 0x0C |
| Earthen Wall | 0x11 |
| Void Edge | 0x08 |
| Ironcrown Helm | 0x15 |
| Phoenix Plume | 0x19 |
| Moonlight Shard | 0x1F |
| Astral Compass | 0x09 |

*"Thunder Root and Tempest Bow share internal ID 0x0C. A code collision. If you own both, the game gets confused about which you're equipping. Classic Morimoto." — Old_Forge_Fan*

### Extraction

Find the five products from the recipes above. Look up each product's internal hex ID. Convert hex to decimal. A=1, B=2, ..., Z=26.

| # | Product | Hex ID | Decimal | Letter |
|---|---------|--------|---------|--------|
| 1 | Earthen Wall | 0x11 | 17 | Q |
| 2 | Ironcrown Helm | 0x15 | 21 | U |
| 3 | Steam Lance | 0x05 | 5 | E |
| 4 | Tempest Bow | 0x0C | 12 | L |
| 5 | Thunder Root | 0x0C | 12 | L |

**Answer: QUELL** (ROT13: DHRYY)

---

*You may find the items and crafting section of the archive helpful.*
