# Rounds — Wololo

**Stage 2: STRUCTURE**

---

## Architecture

Single round. 5 puzzles. 1 meta. The simplest possible hunt — validating the toolkit pipeline.

```
I   (Civilizations) ─┐
II  (Units)          ├─→ Meta (∞) → Final Answer: WOLOLO
III (Technologies)   │
IV  (Maps)           │
V   (Strategy)      ─┘
```

---

## Round 1: The Ages

| Field | Value |
|-------|-------|
| **Name** | The Ages |
| **Theme** | Each puzzle represents an Age advancement |
| **Puzzles** | 5 |
| **Difficulty range** | 2-3 (accessible to any AoE player) |
| **Sections covered** | Civilizations, Units, Technologies, Maps, Strategy |
| **Meta mechanism** | Crossword — 5 answer words fill a small grid, highlighted squares spell WOLOLO |
| **Narrator** | The Monk — short, present tense, no exclamation marks |

### Puzzle slots

| Slot | Age | Domain | Puzzle type | Difficulty | Answer (encoded) |
|------|-----|--------|-------------|------------|-----------------|
| I | Dark | Civilizations | Identification — match 8 unique bonuses to civs | 2 | TBD |
| II | Feudal | Units | Counter chain — trace what beats what in a cycle | 2 | TBD |
| III | Castle | Technologies | Tech tree path — fill 6 missing techs in a research chain | 3 | TBD |
| IV | Imperial | Maps | Map features — identify 5 standard maps from terrain descriptions | 2 | TBD |
| V | Post-Imperial | Strategy | Build order — sequence 8 steps to reach Castle Age optimally | 3 | TBD |

### Meta (∞ — Wonder Victory)

| Field | Value |
|-------|-------|
| **Mechanism** | Small crossword grid. 5 answer words cross. Highlighted squares spell the meta answer. |
| **Feeds from** | All 5 puzzles |
| **Answer** | WOLOLO (the Monk's conversion sound) |
| **80% rule** | 5-word crossword — solvable with 4 of 5 answers via crossing constraints |
| **Backsolving** | Possible — crossing letters + WOLOLO constraint narrow missing answers |

---

## Difficulty Curve

```
Difficulty
3 |              ██              ██
2 |    ██    ██         ██
1 |
  └──────────────────────────────
    I     II    III    IV    V
```

Gentle start (matching), ramp for tech tree and build order.

---

## Numbering

Roman numerals I through V = the five Ages. Meta = ∞ (Wonder Victory — the game's "infinite" win condition).

---

## Narrator Voice — The Monk

Adapted from toolkit principles:
- Short sentences. Present tense. No exclamation marks.
- Speaks as if the game world is real. "The villagers gather. The barracks trains."
- References game mechanics naturally. "Research costs food and gold."
- Catchphrase "Wololo" appears exactly once — when the solver completes the meta.

### Sample intros:

**Puzzle I (Civilizations):**
> Eight civilizations stand before you. Each one carries a gift no other has. Name them.

**Puzzle III (Technologies):**
> The research queue is broken. Six technologies are missing from the tree. The Castle Age waits.

**Meta (∞):**
> You have advanced through all five ages. One sound remains. You know what it is.

---

## Stage 2 Status: COMPLETE

Ready for review gate → send SCOPE.md + ROUNDS.md to the panel.
