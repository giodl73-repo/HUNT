# IRONFALL Battle System

Source of truth for combat mechanics, special moves, and damage formulas.

---

## Combat Overview

IRONFALL uses a turn-based battle system. Turn order is determined by SPD stat (highest goes first, ties broken by enemy/player priority). Each turn, a character can: Attack, Defend, use a Special Move, or use an Item.

---

## Basic Damage Formula

```
Physical Damage = (ATK × 2 - DEF) × (1 + Level/50)
```

- Minimum damage is always 1.
- DEF reduces damage but cannot reduce below 1.
- Level multiplier ranges from 1.02 (Level 1) to 2.0 (Level 50).

---

## Elemental Multipliers

| Attacker Element vs Defender Element | Multiplier |
|-------------------------------------|-----------|
| Strong against (e.g., Fire vs Ice) | ×2.0 |
| Neutral | ×1.0 |
| Weak against (e.g., Fire vs Thunder) | ×0.5 |
| Same element | ×0.75 |

**Elemental weakness chart:**

| Element | Strong vs | Weak vs |
|---------|----------|---------|
| Fire | Ice | Thunder |
| Ice | Fire | Earth (special: Ice resists Thunder) |
| Thunder | Earth | Fire |
| Earth | Thunder | Ice |
| Dark | Holy | (no weakness except Holy) |
| Holy | Dark | (no weakness except Dark) |

Note: The elemental chart is NOT fully symmetric. Ice is strong vs Fire AND resists Thunder (takes 0.5x from Thunder), making Ice defensively powerful. This is a deliberate design choice by Ashfield Interactive and a common source of forum debate.

---

## Special Moves (8)

Each playable character learns 2 special moves. These are the only source of elemental damage for the player party.

| # | Move Name | Element | Power | MP Cost | User | Effect |
|---|-----------|---------|-------|---------|------|--------|
| 01 | Blazing Arc | Fire | 45 | 8 | Rynn | Single target, may Burn (30%) |
| 02 | Pyre Storm | Fire | 65 | 15 | Rynn | All enemies, may Burn (20%) |
| 03 | Frost Needle | Ice | 40 | 7 | Elara | Single target, may Freeze (25%) |
| 04 | Absolute Zero | Ice | 70 | 18 | Elara | All enemies, may Freeze (15%) |
| 05 | Thunder Lance | Thunder | 50 | 10 | Kael | Single target, may Paralyze (35%) |
| 06 | Storm Judgment | Thunder | 60 | 14 | Kael | All enemies, may Paralyze (20%) |
| 07 | Shadow Rend | Dark | 55 | 12 | Maren | Single target, drains HP (25% of damage) |
| 08 | Void Collapse | Dark | 80 | 22 | Maren | Single target, ignores DEF, may Doom (10%) |

---

## Special Move Damage Formula

```
Special Damage = Power × (1 + MAG/100) × Elemental Multiplier × (1 + Level/50)
```

Where:
- Power = move's base power value
- MAG = caster's Magic stat (not in bestiary — only player characters have MAG)
- Elemental Multiplier = from the chart above
- Level multiplier = same as physical

**Player character MAG stats (at Level 30, typical mid-game):**

| Character | MAG |
|-----------|-----|
| Rynn | 35 |
| Elara | 50 |
| Kael | 30 |
| Maren | 45 |

---

## Status Effects

| Effect | Duration | Result |
|--------|----------|--------|
| Burn | 3 turns | Lose 5% max HP per turn |
| Freeze | 1 turn | Cannot act, takes ×1.5 physical damage |
| Paralyze | 2 turns | 50% chance to lose turn each turn |
| Doom | 5 turns | Instant death when timer expires (bosses immune) |

---

## Battle Log Format

In-game battle logs display as:

```
[TURN 1]
Rynn uses Blazing Arc on Frostclaw!
  → Fire vs Ice (×2.0)
  → 45 × 1.35 × 2.0 × 1.60 = 194 damage!
  → Frostclaw is Burned!

Frostclaw attacks Elara!
  → 32 × 2 - 18 = 46 × 1.60 = 73 damage

[TURN 2]
Elara uses Frost Needle on Frostclaw!
  → Ice vs Ice (×0.75)
  → 40 × 1.50 × 0.75 × 1.60 = 72 damage
...
```

The battle log format is: `Power × (1 + MAG/100) × Elemental × (1 + Level/50) = Total`

---

## Battle System Puzzle Affordances

1. **Formula inference**: Give partial battle logs with some values hidden. Solver must infer the damage formula by comparing observed outcomes.
2. **Elemental deduction**: Given a set of damage results, deduce which element an unknown enemy is.
3. **Optimization**: Given a party and enemy lineup, determine the optimal move order to win in fewest turns.
4. **Anomaly detection**: Insert one battle log with "wrong" numbers — the solver finds which value was changed (indicating data corruption / dataminer evidence).

---

## Boss Fights

| Boss | Region | HP | ATK | DEF | SPD | Element | Immune to | Special |
|------|--------|----|-----|-----|-----|---------|----------|---------|
| Thornmother | 1 | 800 | 25 | 20 | 10 | Earth | Paralyze | Summons Thornbacks |
| Frost Wyrm | 2 | 1500 | 40 | 30 | 20 | Ice | Freeze | Counters Fire with Blizzard |
| Storm Tyrant | 3 | 2000 | 55 | 25 | 30 | Thunder | Paralyze | Changes weakness each phase |
| The Iron King | 4 | 5000 | 70 | 45 | 15 | Dark | Doom, Freeze | Final boss, 3 phases |
| Morimoto's Shadow | 4* | 999 | 80 | 60 | 30 | Dark | All status | Secret boss, DEV room only |
