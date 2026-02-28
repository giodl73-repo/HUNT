# P07 — The Perfect Build

*Author: The Social*

---

> CircuitBreaker asked and the whole forum answered. Three crafted items. Five enemies. One perfect loadout. And when Rook drew the farming chart, the names lined up. Morimoto had arranged the enemies.

---

## ironfall-archive.net/forum/threads/optimal-endgame-build

**CircuitBreaker (2001):** "The Iron Citadel is destroying me. Every enemy is Dark element with ATK in the 40s-60s. I need a loadout that gives at least ATK +30, at least DEF +25, and resists Dark. What three crafted items should I build?"

**Rook (2001):** "Only one crafted item resists Dark. Start there."

**SpeedQueen (2001):** "Find the highest-ATK weapon and the highest-DEF armor or shield. Those plus the Dark-resist piece are your three."

**DataMiner_X (2001):** "I made the chart. When you trace the farming route, it is five unique enemies. Look at the chart carefully. Read the marked letters."

---

## The Build Planner

Determine the optimal 3-item loadout, then trace each crafted item back to its recipe ingredients and the enemies that drop them.

### Step 1: Choose 3 crafted items

**Requirement 1 — Dark resistance:** Search the crafted items list. Find the item that resists Dark.

**Requirement 2 — ATK ≥ 30:** Find the crafted item with the highest ATK bonus.

**Requirement 3 — DEF ≥ 25:** Find the crafted item with the highest DEF bonus.

### Step 2: Trace the crafting chain

For each crafted item, identify:
- Recipe: identify the two ingredients.
- Source: identify the enemy that drops each ingredient.

List all unique enemies that must be farmed.

### Step 3: Read the chart

DataMiner_X drew the farming chart below. Five enemy names are listed. One letter in each name is circled (shown in **[brackets]**). The circled letters encode a message.

Read the bracketed letters from top to bottom.

---

## DataMiner_X's Farming Chart

```
╔═══════════════════════════════════════════════════════════╗
║  ENDGAME BUILD — FARMING CHART                            ║
║  by DataMiner_X, June 2001                                ║
╠═══════════════════════════════════════════════════════════╣
║                                                           ║
║  CRAFTED ITEMS NEEDED:                                    ║
║  ─────────────────────                                    ║
║  1. _____________ (Dark resist)                           ║
║  2. _____________ (Highest ATK)                           ║
║  3. _____________ (Highest DEF)                           ║
║                                                           ║
║  ENEMIES TO FARM:                                         ║
║  ────────────────                                         ║
║  M [O] S S  C R A W L E R          ← Damp Stone          ║
║  D [R] E A D  K N I G H T          ← Obsidian Blade      ║
║  [B] L I Z Z A R D  W R A I T H   ← Crystal Tear        ║
║  S H A D E  W [I] S P              ← Shadow Veil (×2)    ║
║  P E R M A F R O S [T] K N I G H T ← Knight's Crest     ║
║                                                           ║
║  "The names are the message." — DataMiner_X               ║
╚═══════════════════════════════════════════════════════════╝
```

---

## Solution

### Step 1: Identify the loadout

**Dark resistance:** Dawn Mantle (C03) — DEF +15, resist Dark. The only crafted item with Dark resistance.

**Highest ATK:** Void Edge (C08) — ATK +30, drains HP. No other crafted item has ATK +30 or higher.

**Highest DEF:** Earthen Wall (C07) — DEF +25, resist Thunder. Highest DEF bonus of any crafted item.

**Optimal loadout:** Dawn Mantle + Void Edge + Earthen Wall.
- ATK: +30 (meets ≥30)
- DEF: +25 + 15 = +40 (meets ≥25)
- Dark resistance: Yes

### Step 2: Trace the chain

| Item | Recipe | Ingredient 1 | Enemy | Ingredient 2 | Enemy |
|------|--------|-------------|-------|-------------|-------|
| Dawn Mantle (C03) | Shadow Veil + Crystal Tear | Shadow Veil | Shade Wisp (#03) | Crystal Tear | Blizzard Wraith (#08) |
| Void Edge (C08) | Shadow Veil + Obsidian Blade | Shadow Veil | Shade Wisp (#03)* | Obsidian Blade | Dread Knight (#19) |
| Earthen Wall (C07) | Damp Stone + Knight's Crest | Damp Stone | Moss Crawler (#05) | Knight's Crest | Permafrost Knight (#12) |

*Shadow Veil needed twice. Farm Shade Wisp for 2 drops.

**Unique enemies to farm:** Moss Crawler (#05), Shade Wisp (#03), Blizzard Wraith (#08), Permafrost Knight (#12), Dread Knight (#19).

### Step 3: Read the chart

The farming chart shows 5 enemy names with one letter each in brackets:

| Row | Enemy Name | Bracketed Letter |
|-----|-----------|-----------------|
| 1 | M **[O]** S S  C R A W L E R | O |
| 2 | D **[R]** E A D  K N I G H T | R |
| 3 | **[B]** L I Z Z A R D  W R A I T H | B |
| 4 | S H A D E  W **[I]** S P | I |
| 5 | P E R M A F R O S **[T]** K N I G H T | T |

**Answer: ORBIT** (ROT13: BEOVG)

### Extraction Verification

Answer: ORBIT. Puzzle slot: Act II, position 2. Super-meta extracts position 2: ORBIT[2] = R. R in ROT13 = E. Matches the extraction table.

---

### Social's Notes

Four people in the thread. CircuitBreaker asks, Rook narrows, SpeedQueen confirms, DataMiner_X draws the chart. That is how forums work. Nobody solves alone.

The marked letters in the chart are the extraction. The solver does not need to figure out why those particular letters are marked — they just need to determine the correct enemies (through the crafting chain deduction) and confirm that the chart matches their work. If it matches, read the marked letters. Done.

---

*The crafting reference is in the archive. The items table does not lie.*
