# Technologies — Reference Data

**Source:** Age of Empires II: Definitive Edition (AoE2 DE)
**Scope:** Key technology chains, prerequisites, and costs for puzzle design.

---

## Age Advancement

### Requirements and Costs

| Advance to | Cost | Buildings Required | Research Time |
|-----------|------|--------------------|--------------|
| **Feudal Age** | 500F | 2 Dark Age buildings (Barracks, etc. — Houses/TC don't count) | 130 sec |
| **Castle Age** | 800F 200G | 2 Feudal Age buildings (Archery Range, Stable, Blacksmith, Market) | 160 sec |
| **Imperial Age** | 1000F 800G | 2 Castle Age buildings (Siege Workshop, Monastery, University) OR 1 Castle | 190 sec |

### Age Advancement Chain

```
Dark Age ──500F──> Feudal Age ──800F 200G──> Castle Age ──1000F 800G──> Imperial Age
           need 2       need 2 Feudal           need 2 Castle Age
           Dark bldgs    buildings               buildings or 1 Castle
```

**Total cost to reach Imperial:** 2,300F + 1,000G

---

## Blacksmith Upgrades

The Blacksmith provides three parallel upgrade chains. Each chain has three tiers (Feudal / Castle / Imperial).

### Ranged Attack (archer damage + range)

```
Fletching ──────> Bodkin Arrow ──────> Bracer
(Feudal)          (Castle)             (Imperial)
100F 50G          200F 100G            300F 200G
+1 atk, +1 range  +1 atk, +1 range    +1 atk, +1 range
```

### Ranged Armor (archer defense)

```
Padded Archer Armor ──> Leather Archer Armor ──> Ring Archer Armor
(Feudal)                (Castle)                  (Imperial)
100F                    150F 150G                  250F 250G
+1/+1 armor             +1/+1 armor               +1/+1 armor
```

### Melee Attack (infantry + cavalry damage)

```
Forging ────────> Iron Casting ──────> Blast Furnace
(Feudal)          (Castle)             (Imperial)
150F              200F 100G            275F 225G [VERIFY]
+1 attack          +1 attack           +1 attack
```

### Melee Armor (infantry + cavalry defense)

```
Scale Mail Armor ──> Chain Mail Armor ──> Plate Mail Armor
(Feudal)             (Castle)             (Imperial)
100F                 200F 100G            250F 150G [VERIFY]
+1/+1 armor          +1/+1 armor          +1/+2 armor
```

### Cavalry Armor

```
Scale Barding Armor ──> Chain Barding Armor ──> Plate Barding Armor
(Feudal)                (Castle)                 (Imperial)
150F                    250F 150G                350F 200G [VERIFY]
+1/+1 armor             +1/+1 armor              +1/+2 armor
```

---

## University Technologies

Available from Castle Age. University requires Castle Age.

| Technology | Age | Cost | Effect |
|-----------|-----|------|--------|
| **Masonry** | Castle | 150W 175S | Buildings +10% HP, +1/+1 armor, +3 building armor |
| **Architecture** | Imperial | 200W 300S | Buildings +10% HP, +1/+1 armor, +3 building armor (stacks with Masonry) |
| **Ballistics** | Castle | 300W 175G | Ranged units lead moving targets (critical accuracy upgrade) |
| **Murder Holes** | Castle | 200F 200S | Removes minimum range from Castles, towers |
| **Chemistry** | Imperial | 300F 200G | +1 attack for all ranged units; enables gunpowder units |
| **Heated Shot** | Castle | 350F 100G | Towers +125% damage vs. ships |
| **Treadmill Crane** | Castle | 200F 300W | Buildings built 20% faster |
| **Guard Tower** | Castle | 100F 250S | Upgrades Watch Tower → Guard Tower |
| **Keep** | Imperial | 500F 350S [VERIFY] | Upgrades Guard Tower → Keep |
| **Bombard Tower** | Imperial | 800F 400W | Enables Bombard Towers (requires Chemistry) |

### University Dependency Chain

```
Masonry ──────> Architecture
(Castle)        (Imperial)

Chemistry ──────> Bombard Tower
(Imperial)        (Imperial)

Watch Tower ──> Guard Tower ──> Keep
(Feudal)        (Castle)        (Imperial)
```

---

## Monastery Technologies

Available from Castle Age. Key to the Monk/conversion theme of this hunt.

| Technology | Cost | Effect |
|-----------|------|--------|
| **Redemption** | 475G | Monks can convert buildings and siege weapons |
| **Atonement** | 325G | Monks can convert enemy Monks |
| **Fervor** | 140G | Monks move 15% faster |
| **Sanctity** | 175G [VERIFY] | Monks +15 HP |
| **Heresy** | 1000G | Your converted units die instead of switching sides |
| **Faith** | 750F 1000G | Units resist conversion (takes longer to convert) |
| **Illumination** | 120G | Monks regain faith 50% faster after conversion |
| **Block Printing** | 200G | Monks +3 range |
| **Theocracy** | 200F 100G | Only one Monk in a group needs to rest after conversion |

### Monastery Tech Notes

- All Monastery techs cost only gold (except Faith and Theocracy).
- Redemption, Sanctity, and Block Printing are the three most impactful Monk techs.
- Not all civs get all Monastery techs. Aztecs get all of them (strong Monk civ).
- Monks have a base range of 9 for conversion. Block Printing raises it to 12.
- Relics generate 30 gold per minute (per Relic) when garrisoned in a Monastery.

---

## Unit Upgrade Costs (line upgrades)

These are the research costs to upgrade a unit to its next tier (not the per-unit training cost).

### Infantry Line

| Upgrade | Age | Cost |
|---------|-----|------|
| Man-at-Arms | Feudal | 100F 40G |
| Long Swordsman | Castle | 200F 65G |
| Two-Handed Swordsman | Imperial | 300F 100G |
| Champion | Imperial | 750F 350G |

### Spearman Line

| Upgrade | Age | Cost |
|---------|-----|------|
| Pikeman | Castle | 215F 90G |
| Halberdier | Imperial | 300F 600G |

### Archer Line

| Upgrade | Age | Cost |
|---------|-----|------|
| Crossbowman | Castle | 125F 75G |
| Arbalester | Imperial | 350F 300G |

### Cavalry Line

| Upgrade | Age | Cost |
|---------|-----|------|
| Light Cavalry | Castle | 150F 50G |
| Hussar | Imperial | 500F 600G |
| Cavalier | Imperial | 300F 300G |
| Paladin | Imperial | 1300F 750G |

---

## Key Dependency Chains (for Puzzle III)

These chains show "you must research X before Y":

```
Chain 1 — Archer Power:
  Fletching → Bodkin Arrow → Bracer
  Archer → Crossbowman → Arbalester
  (Fletching available Feudal; Bracer requires Imperial)

Chain 2 — Siege:
  Castle Age → Siege Workshop → Mangonel / Battering Ram / Scorpion
  Imperial Age → Trebuchet (from Castle building), Siege Ram, Onager
  Chemistry → Bombard Cannon

Chain 3 — Castle Tech:
  Castle Age + 650 stone → Castle building → Unique Unit + Unique Tech
  Imperial Age → Elite Unique Unit upgrade

Chain 4 — Monk Power:
  Castle Age → Monastery → Monk
  Sanctity + Fervor + Redemption + Block Printing = full Monk rush
  (No linear dependency between Monastery techs — all independent)

Chain 5 — Economy:
  Feudal → Horse Collar (farm +75 food)
  Castle → Heavy Plow (farm +125 food)
  Imperial → Crop Rotation (farm +175 food)
  Mill: each tier requires the previous
```

---

## Notes for Puzzle Authors

- Blacksmith chains are the most puzzle-friendly: three tiers, clean dependencies, rising costs.
- Monastery techs are all independent (no chain) but thematically perfect for the Monk narrator.
- The Feudal→Castle→Imperial cost escalation (500F → 800F+200G → 1000F+800G) is a clean arithmetic pattern.
- Chemistry is a gatekeeper tech: it unlocks gunpowder units AND adds +1 ranged attack globally.
- Ballistics is one of the most important techs in competitive play — makes archers dramatically better.
