# Economy — Reference Data

**Source:** Age of Empires II: Definitive Edition (AoE2 DE)
**Scope:** Gather rates, costs, and production numbers for puzzle design.

---

## Villager Gather Rates

Base rates (no upgrades, no civ bonuses). All rates in **resource per second**.

| Source | Rate | Notes |
|--------|------|-------|
| **Sheep / Turkey** | 0.33 | Must be gathered under TC to prevent decay |
| **Boar / Elephant** | 0.41 | Lured to TC; decays if not garrisoned nearby |
| **Deer** | 0.41 | Same as boar, but must be pushed (no lure) |
| **Berries** | 0.31 | Forage Bushes near TC; limited supply (typically 125 food each, 6 bushes) |
| **Farms** | 0.40 | Infinite (renewable); requires Farm building (60W) |
| **Shore Fish** | 0.43 | Villagers gather directly; highest villager food rate |
| **Deep-Sea Fish** | 0.43 [VERIFY] | Fishing Ships only; faster than villager rates |
| **Wood** | 0.39 | From trees; walk time to lumber camp matters |
| **Gold Mining** | 0.38 | From gold piles; walk time to mining camp matters |
| **Stone Mining** | 0.36 | From stone piles; slower than gold |

### Effective Rates After Walk Time

Real gather rates are lower due to walk time (to drop-off building and back). Effective rates:

| Source | Effective Rate (approx) |
|--------|------------------------|
| Sheep (under TC) | 0.33 (minimal walk) |
| Boar (under TC) | 0.41 (minimal walk) |
| Farms (next to TC/Mill) | 0.32-0.37 (with reseeding) [VERIFY exact] |
| Wood (fresh treeline) | 0.32-0.36 |
| Gold (fresh pile) | 0.32-0.35 |
| Stone (fresh pile) | 0.30-0.33 |

### Gather Rate Hierarchy (fastest to slowest)

```
Shore Fish > Boar/Deer > Farms > Sheep > Wood > Gold > Berries > Stone
  0.43        0.41       0.40    0.33    0.39   0.38    0.31     0.36
```

Note: Raw rates vs. effective rates differ. Wood raw rate is higher than sheep, but walk time makes sheep more efficient early game.

---

## Age-Up Costs

| Age | Food | Gold | Total Resources | Buildings Required |
|-----|------|------|-----------------|--------------------|
| Feudal | 500 | 0 | 500 | 2 Dark Age buildings |
| Castle | 800 | 200 | 1,000 | 2 Feudal Age buildings |
| Imperial | 1,000 | 800 | 1,800 | 2 Castle Age buildings or 1 Castle |
| **Total** | **2,300** | **1,000** | **3,300** | — |

### Age-Up Research Times

| Age | Time (seconds) |
|-----|---------------|
| Feudal | 130 |
| Castle | 160 |
| Imperial | 190 |

---

## Key Building Costs

| Building | Cost | Age Available | Build Time (sec) |
|----------|------|---------------|-----------------|
| House | 25W | Dark | 25 |
| Mill | 100W | Dark | 35 |
| Lumber Camp | 100W | Dark | 35 |
| Mining Camp | 100W | Dark | 35 |
| Barracks | 175W | Dark | 50 |
| Dock | 150W | Dark | 35 |
| Farm | 60W | Dark | 15 |
| Blacksmith | 150W | Feudal | 40 |
| Archery Range | 175W | Feudal | 50 |
| Stable | 175W | Feudal | 50 |
| Market | 175W | Feudal | 60 |
| Town Center | 275W 100S | Castle | 150 |
| Monastery | 175W | Castle | 40 |
| Siege Workshop | 200W | Castle | 40 |
| University | 200W | Castle | 60 |
| Castle | 650S | Castle | 200 |
| Wonder | 1000W 1000F 1000G 1000S | Imperial | 3500 [VERIFY] |

---

## Unit Training Costs and Times

| Unit | Cost | Train Time (sec) | Trained At |
|------|------|-------------------|------------|
| Villager | 50F | 25 | Town Center |
| Militia | 60F 20G | 21 | Barracks |
| Man-at-Arms | 60F 20G | 21 | Barracks |
| Spearman | 35F 25W | 22 | Barracks |
| Pikeman | 35F 25W | 22 | Barracks |
| Halberdier | 35F 25W | 22 | Barracks |
| Archer | 25W 45G | 35 | Archery Range |
| Crossbowman | 25W 45G | 27 | Archery Range |
| Skirmisher | 25F 35W | 22 | Archery Range |
| Hand Cannoneer | 45F 50G | 34 | Archery Range |
| Scout Cavalry | 80F | 30 | Stable |
| Knight | 60F 75G | 30 | Stable |
| Cavalier | 60F 75G | 30 | Stable |
| Paladin | 60F 75G | 30 | Stable |
| Camel Rider | 55F 60G | 22 | Stable |
| Monk | 100G | 51 | Monastery |
| Battering Ram | 160W 75G | 36 | Siege Workshop |
| Mangonel | 160W 135G | 46 | Siege Workshop |
| Scorpion | 75W 75G | 30 | Siege Workshop |
| Trebuchet | 200W 200G | 50 | Castle |
| Bombard Cannon | 225W 225G | 56 | Siege Workshop |
| Fishing Ship | 75W | 40 | Dock |
| Trade Cart | 100W 50G | 51 | Market |

---

## Economic Upgrades

### Farm Upgrades (Mill)

| Technology | Age | Cost | Effect |
|-----------|-----|------|--------|
| Horse Collar | Feudal | 75F 75W | Farms +75 food (total 250 → 325) [VERIFY exact base] |
| Heavy Plow | Castle | 125F 125W | Farms +125 food; Farmers +1 carry |
| Crop Rotation | Imperial | 250F 250W | Farms +175 food |

### Lumber Upgrades (Lumber Camp)

| Technology | Age | Cost | Effect |
|-----------|-----|------|--------|
| Double-Bit Axe | Feudal | 100F 50W | Lumberjacks work 20% faster |
| Bow Saw | Castle | 150F 100W | Lumberjacks work 20% faster |
| Two-Man Saw | Imperial | 300F 200W | Lumberjacks work 10% faster |

### Mining Upgrades (Mining Camp)

| Technology | Age | Cost | Effect |
|-----------|-----|------|--------|
| Gold Mining | Feudal | 100F 75W | Gold miners work 15% faster |
| Gold Shaft Mining | Castle | 200F 150W | Gold miners work 15% faster |
| Stone Mining | Feudal | 100F 75W | Stone miners work 15% faster |
| Stone Shaft Mining | Castle | 200F 150W | Stone miners work 15% faster |

### Wheelbarrow and Hand Cart (Town Center)

| Technology | Age | Cost | Effect |
|-----------|-----|------|--------|
| Wheelbarrow | Feudal | 175F 50W | Villagers +10% speed, +25% carry |
| Hand Cart | Castle | 300F 200W | Villagers +10% speed, +50% carry |

---

## Key Economic Numbers

### Starting Resources (Standard Game)

| Resource | Amount |
|----------|--------|
| Food | 200 |
| Wood | 200 |
| Gold | 100 |
| Stone | 200 |
| Villagers | 3 |
| Scout | 1 |

### Relic Income

- Each Relic garrisoned in a Monastery generates **0.5 gold per second** (30 gold per minute).
- Standard maps have 5 Relics.
- Maximum passive Relic income: **150 gold per minute** (with all 5 Relics).
- Aztec team bonus: Relics generate +33% gold → **40 gold per Relic per minute**.

### Market Trade Rates

- Trade Cart gold generation depends on distance between Markets.
- Longer distance = more gold per trip.
- Trade is the only infinite gold source besides Relics.

### Population and Villager Guidelines

| Benchmark | Typical Value |
|-----------|--------------|
| Max population (standard) | 200 |
| Villagers at Feudal click | 20-23 |
| Villagers at Castle click | 28-35 |
| Ideal villager ratio (late game) | ~100 villagers, ~100 military |
| Villagers on food (Feudal) | 10-14 |
| Villagers on wood (Feudal) | 7-10 |
| Villagers on gold (Castle) | 5-8 |

---

## Notes for Puzzle Authors

- Gather rates create clean numerical puzzles: "How many villagers to sustain constant Knight production?" (Knights cost 60F 75G; Knight takes 30 sec; need 60F/30s = 2 F/sec from food vils, 75G/30s = 2.5 G/sec from gold vils).
- The 200/200/100/200 starting resources are a memorable pattern.
- Age-up costs follow a clear escalation: 500 → 1000 → 1800 total.
- Relic math (30 gold/min per Relic, 5 Relics) is clean puzzle material.
- Wonder cost (1000 of each resource) is the most symmetric cost in the game.
- Farm cost (60W) and Villager cost (50F) are the two most fundamental micro-costs.
- Monk is the only unit that costs purely gold. All trash units cost no gold.
