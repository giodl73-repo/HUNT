# Units — Reference Data

**Source:** Age of Empires II: Definitive Edition (AoE2 DE)
**Scope:** Core military units, counter relationships, and notable unique units.

---

## The Counter Triangle

The fundamental rock-paper-scissors of AoE2:

```
    Infantry
   /        \
  /  weak vs  \
 v              \
Cavalry -------> Archers
        strong vs
```

- **Infantry** are strong vs. buildings and siege, weak vs. archers and cavalry
- **Archers** are strong vs. infantry (kiting), weak vs. cavalry and siege
- **Cavalry** are strong vs. archers and siege, weak vs. pikemen (anti-cav infantry)

**Key exceptions:**
- Pikemen/Halberdiers (infantry) hard-counter cavalry
- Skirmishers (archers) hard-counter other archers
- Monks convert any single unit (especially effective vs. expensive cavalry)
- Siege (Mangonels, Scorpions) shreds grouped archers

---

## Core Unit Lines

### Infantry

| Unit | Upgrade Line | Age | Cost | Strong vs. | Weak vs. |
|------|-------------|-----|------|------------|----------|
| Militia | base | Dark | 60F 20G | nothing much | everything |
| Man-at-Arms | Militia → | Feudal | 60F 20G | Spearmen, Eagles | Archers, Knights |
| Long Swordsman | M-a-A → | Castle | 60F 20G | Eagles, buildings | Archers, Knights |
| Two-Handed Swordsman | LS → | Imperial | 60F 20G | Trash, buildings | Archers, Cavalry |
| Champion | THS → | Imperial | 60F 20G | Trash, buildings, Huskarls | Archers, Cavalry |
| Spearman | base | Feudal | 35F 25W | Scout Cavalry | Archers, Militia |
| Pikeman | Spearman → | Castle | 35F 25W | Knights, Cavalry | Archers, Infantry |
| Halberdier | Pikeman → | Imperial | 35F 25W | Paladins, War Elephants | Archers, Infantry |

### Archers

| Unit | Upgrade Line | Age | Cost | Strong vs. | Weak vs. |
|------|-------------|-----|------|------------|----------|
| Archer | base | Feudal | 25W 45G | Infantry, Spearmen | Skirmishers, Knights |
| Crossbowman | Archer → | Castle | 25W 45G | Infantry, Spearmen | Skirmishers, Knights, Mangonels |
| Arbalester | Xbow → | Imperial | 25W 45G | Infantry, Cavalry | Skirmishers, Siege Onager, Huskarls |
| Skirmisher | base | Feudal | 25F 35W | Archers, Spearmen | Knights, Infantry |
| Elite Skirmisher | Skirm → | Castle | 25F 35W | Archers | Knights, Infantry, Siege |
| Cavalry Archer | base | Castle | 40W 60G | Infantry (kiting) | Skirmishers, Camels |
| Hand Cannoneer | — | Imperial | 45F 50G | Infantry, high pierce armor | Cavalry, Skirmishers |

### Cavalry

| Unit | Upgrade Line | Age | Cost | Strong vs. | Weak vs. |
|------|-------------|-----|------|------------|----------|
| Scout Cavalry | base | Feudal | 80F | Monks, Archers (raiding) | Spearmen, TC fire |
| Light Cavalry | Scout → | Castle | 80F | Monks, Siege, Archers | Pikemen, Knights |
| Hussar | LC → | Imperial | 80F | Monks, Siege | Pikemen, Halberdiers |
| Knight | base | Castle | 60F 75G | Archers, Infantry, Siege | Pikemen, Monks, Camels |
| Cavalier | Knight → | Imperial | 60F 75G | Archers, Infantry | Halberdiers, Monks, Camels |
| Paladin | Cavalier → | Imperial | 60F 75G | Archers, Infantry, Siege | Halberdiers, Monks, Camels |
| Camel Rider | base | Castle | 55F 60G | Cavalry | Pikemen, Archers, Monks |

### Siege

| Unit | Age | Cost | Strong vs. | Weak vs. |
|------|-----|------|------------|----------|
| Battering Ram | Castle | 160W 75G | Buildings, Archer fire | Melee infantry, Mangonels |
| Capped Ram | Imperial | 160W 75G | Buildings | Melee infantry |
| Siege Ram | Imperial | 160W 75G | Buildings (devastating) | Melee infantry |
| Mangonel | Castle | 160W 135G | Grouped archers, infantry | Cavalry, Bombard Cannons |
| Onager | Imperial | 160W 135G | Grouped units, trees | Cavalry, Bombard Cannons |
| Scorpion | Castle | 75W 75G | Grouped infantry | Cavalry, Mangonels |
| Trebuchet | Imperial | 200W 200G | Buildings, static defenses | Cavalry, Bombard Cannons |
| Bombard Cannon | Imperial | 225W 225G | Buildings, Siege, Trebuchets | Cavalry |

### Monks

| Unit | Age | Cost | Special |
|------|-----|------|---------|
| Monk | Castle | 100G | Converts enemy units; heals friendly units; collects Relics |

---

## Notable Unique Units (for puzzles)

| Unique Unit | Civ | Type | Cost | Key Trait |
|------------|-----|------|------|-----------|
| Longbowman | Britons | Foot Archer | 35W 40G | Range 6 (elite: 7) — outranges everything except Trebuchets |
| Mangudai | Mongols | Cavalry Archer | 55W 65G | +3 bonus vs. siege; fast fire rate |
| Cataphract | Byzantines | Cavalry | 70F 75G | Trample damage; resists anti-cavalry bonus damage |
| War Elephant | Persians | Cavalry | 200F 75G | 450 HP (elite: 600); trample; extremely slow |
| Samurai | Japanese | Infantry | 60F 30G | +12 bonus damage vs. unique units (elite) |
| Woad Raider | Celts | Infantry | 65F 25G | Speed 1.03 — fastest infantry in the game |
| Throwing Axeman | Franks | Infantry | 55F 25G | Ranged infantry — throws axes past walls |
| Jaguar Warrior | Aztecs | Infantry | 60F 30G | +11 bonus vs. infantry (elite); anti-infantry infantry |

---

## Counter Cheat Sheet

Quick "what beats what" for puzzle design:

| If you face... | Counter with... |
|----------------|-----------------|
| Knights / Cavalry | Pikemen, Halberdiers, Camels, Monks |
| Archers / Crossbows | Skirmishers, Cavalry, Mangonels, Huskarls |
| Pikemen / Halberdiers | Archers, Hand Cannoneers, Scorpions |
| Siege (Rams, Trebs) | Cavalry, Mangudai, Bombard Cannons |
| Monks | Light Cavalry, Hussars (fast + cheap) |
| Massed infantry | Archers, Scorpions, Mangonels |
| Buildings / Castles | Trebuchets, Bombard Cannons, Rams |
| Eagles | Militia line (Champion), Knights |

---

## Notes for Puzzle Authors

- The counter triangle is the conceptual backbone for Puzzle II (Tournament Bracket).
- Unit costs are consistent across all upgrade levels within a line (e.g., Knight/Cavalier/Paladin all cost 60F 75G to train — the upgrade research cost is separate).
- Trash units (Spearman line, Skirmisher line, Scout line) cost no gold — critical for late game.
- Monks cost only gold. This makes them unique economically.
- Siege units cost wood and gold (no food) — also an economic quirk usable in puzzles.
