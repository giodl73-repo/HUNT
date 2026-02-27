# Settlers of Catan — World Data

Source material for Module M2 (The Speedrunner).

---

## The Board

### Hex Terrain Types

| Terrain | Resource produced | Count on standard board | Number tokens placed |
|---------|-------------------|------------------------|---------------------|
| Hills | Brick | 3 | Yes |
| Forest | Lumber | 4 | Yes |
| Mountains | Ore | 3 | Yes |
| Fields | Grain | 4 | Yes |
| Pasture | Wool | 4 | Yes |
| Desert | None | 1 | No (Robber starts here) |
| **Total** | — | **19 hexes** | **18 tokens** |

### Number Tokens

Each non-desert hex gets a number token (2-12, excluding 7). The number indicates which dice roll produces that hex's resource.

| Number | Dots on token | Probability (out of 36) | Frequency rank |
|--------|---------------|------------------------|----------------|
| 2 | 1 dot | 1/36 (2.8%) | Rarest |
| 3 | 2 dots | 2/36 (5.6%) | |
| 4 | 3 dots | 3/36 (8.3%) | |
| 5 | 4 dots | 4/36 (11.1%) | |
| 6 | 5 dots (red) | 5/36 (13.9%) | 2nd most common |
| 7 | — | 6/36 (16.7%) | Most common (Robber) |
| 8 | 5 dots (red) | 5/36 (13.9%) | 2nd most common |
| 9 | 4 dots | 4/36 (11.1%) | |
| 10 | 3 dots | 3/36 (8.3%) | |
| 11 | 2 dots | 2/36 (5.6%) | |
| 12 | 1 dot | 1/36 (2.8%) | Rarest |

Note: 6 and 8 are printed in red on standard tokens to highlight their high probability. There is one token each of 2 and 12; two tokens each of all others (3-11 except 7).

---

## Resource Cards

Five resource types, each tied to one terrain:

| Resource | Terrain | Card color |
|----------|---------|-----------|
| Brick | Hills | Red/brown |
| Lumber | Forest | Dark green |
| Ore | Mountains | Grey/dark |
| Grain | Fields | Yellow |
| Wool | Pasture | Light green |

---

## Building Costs

| Structure | Brick | Lumber | Ore | Grain | Wool | VP | Limit per player |
|-----------|-------|--------|-----|-------|------|----|-----------------|
| **Road** | 1 | 1 | — | — | — | 0 | 15 |
| **Settlement** | 1 | 1 | — | 1 | 1 | 1 | 5 |
| **City** (upgrade) | — | — | 3 | 2 | — | 2 | 4 |
| **Development Card** | — | — | 1 | 1 | 1 | — | 25 in deck |

### Reading the Cost Table

- A **Road** costs 1 Brick + 1 Lumber
- A **Settlement** costs 1 Brick + 1 Lumber + 1 Grain + 1 Wool (4 cards total)
- A **City** upgrades a Settlement: costs 3 Ore + 2 Grain (5 cards total); replaces the settlement
- A **Development Card** costs 1 Ore + 1 Grain + 1 Wool (3 cards total)

---

## Development Cards

25 total in the deck:

| Card | Count | Effect |
|------|-------|--------|
| **Knight** | 14 | Move the robber. Steal 1 resource from adjacent player. Counts toward Largest Army. |
| **Victory Point** | 5 | Worth 1 VP. Kept hidden until you win. Cards named: Library, Market, Chapel, University of Catan, Great Hall [VERIFY: exact VP card names] |
| **Road Building** | 2 | Place 2 roads for free |
| **Year of Plenty** | 2 | Take any 2 resources from the bank |
| **Monopoly** | 2 | Name 1 resource type — all other players must give you all cards of that type |

---

## The Robber

| Rule | Detail |
|------|--------|
| **Activation** | When a 7 is rolled |
| **Discard rule** | Any player with > 7 resource cards must discard half (rounded down) |
| **Placement** | Active player moves robber to any hex (except current location) |
| **Blocking** | Hex with robber produces no resources |
| **Stealing** | Active player steals 1 random resource from any player with a settlement/city on that hex |
| **Knight card** | Also moves the robber (same steal rules apply) |

---

## Trading

### Domestic Trade (between players)

- On your turn, you may trade any number of resources with other players
- Any ratio agreed by both parties
- Other players cannot trade with each other (only with the active player)

### Maritime Trade (with the bank)

| Port type | Ratio | Condition |
|-----------|-------|-----------|
| No port | 4:1 | Default — any 4 identical resources for 1 of any type |
| **Generic port (3:1)** | 3:1 | Any 3 identical resources for 1 of any type |
| **Specific port (2:1)** | 2:1 | 2 of the specified resource for 1 of any type |

There are 9 harbor/port locations on a standard board: four 3:1 generic ports and five 2:1 specific ports (one for each resource type).

---

## Special Awards

| Award | Requirement | VP | Rules |
|-------|------------|----|----|
| **Longest Road** | First to build a continuous road of 5+ segments | 2 VP | Taken by first player to reach 5; stolen if another player builds longer. Settlements/cities of opponents break continuity. |
| **Largest Army** | First to play 3+ Knight development cards | 2 VP | Taken by first player to play 3; stolen if another player plays more. |

---

## Victory Conditions

- First player to reach **10 Victory Points** on their turn wins
- VP sources: Settlements (1 each), Cities (2 each), Longest Road (2), Largest Army (2), VP development cards (1 each)

### Example VP Calculation

| Source | Count | VP |
|--------|-------|----|
| Settlements | 2 | 2 |
| Cities | 2 | 4 |
| Longest Road | 1 | 2 |
| VP card (hidden) | 2 | 2 |
| **Total** | — | **10 — wins** |

---

## Standard Board Layout

19 hexes arranged in a roughly circular pattern:
- Inner ring: 1 hex (often desert)
- Middle ring: 6 hexes
- Outer ring: 12 hexes

The board is surrounded by ocean frame pieces with harbors/ports at fixed locations.

Player count: 3-4 (base game). 5-6 with expansion.
