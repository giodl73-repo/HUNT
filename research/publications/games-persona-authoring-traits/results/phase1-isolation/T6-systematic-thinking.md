# Condition T6 — Systematic Thinking

## Puzzle

### CHAIN OF UPGRADES

Five units and technologies from Age of Empires II are clued below.
For each entry: identify the term, then extract the single letter at the position shown.
Spaces and hyphens are not counted when indexing into the term.
Read the five extracted letters in order from 1 to 5.

---

```
┌───┬─────────────────────────────────────────────────────────────────────────────┬─────┬──────────┐
│ # │ Clue                                                                        │ Pos │ Letter   │
├───┼─────────────────────────────────────────────────────────────────────────────┼─────┼──────────┤
│ 1 │ The entry-level ram at the Siege Workshop. It excels against buildings and  │     │          │
│   │ shrugs off archer fire, but melee infantry and Mangonels cut it down fast.  │  3  │  ___     │
│   │ Two further upgrades follow it before the Imperial Age is done.             │     │          │
├───┼─────────────────────────────────────────────────────────────────────────────┼─────┼──────────┤
│ 2 │ The first farm-output technology in the Mill upgrade chain. Researched in   │     │          │
│   │ the Feudal Age, it is the opening step of a three-stage sequence that ends  │  2  │  ___     │
│   │ with Crop Rotation in the Imperial Age.                                     │     │          │
├───┼─────────────────────────────────────────────────────────────────────────────┼─────┼──────────┤
│ 3 │ The Persian unique unit. Cavalry class; trained at the Castle from the      │     │          │
│   │ Castle Age onward. The most expensive unique unit in the game by food cost, │  1  │  ___     │
│   │ at 200 food and 75 gold. Deals trample damage and moves slowly.             │     │          │
├───┼─────────────────────────────────────────────────────────────────────────────┼─────┼──────────┤
│ 4 │ The Imperial Age upgrade to the Mangonel, researched at the Siege Workshop. │     │          │
│   │ Retains the Mangonel's role against grouped units, and adds the ability to  │  5  │  ___     │
│   │ fell trees — a decisive advantage on forested maps like Black Forest.       │     │          │
├───┼─────────────────────────────────────────────────────────────────────────────┼─────┼──────────┤
│ 5 │ The Castle Age tier of the Blacksmith's melee attack chain.               │     │          │
│   │ (Forging → ___ → Blast Furnace). Costs 200 food and 100 gold.              │  2  │  ___     │
│   │ Grants +1 attack to all infantry and cavalry.                               │     │          │
└───┴─────────────────────────────────────────────────────────────────────────────┴─────┴──────────┘
```

---

**Read the five letters in rows 1–5:**

    ___ ___ ___ ___ ___

**What is the final answer?**

    ___________________________

---

## Solution Key

### Step 1 — Identify each term

| # | Clue answer    | Source in world data                                                                                           |
|---|----------------|----------------------------------------------------------------------------------------------------------------|
| 1 | BATTERING RAM  | units.md — Siege table: "Battering Ram, Castle, 160W 75G, Strong vs. Buildings and Archer fire, Weak vs. Melee infantry and Mangonels." Two upgrades follow: Capped Ram, Siege Ram. |
| 2 | HORSE COLLAR   | techs.md — Chain 5 (Economy): "Feudal → Horse Collar (farm +75 food) → Heavy Plow → Crop Rotation." First of the three Mill upgrades. |
| 3 | WAR ELEPHANT   | units.md — Unique Units table: "War Elephant, Persians, Cavalry, 200F 75G, 450 HP (elite: 600); trample; extremely slow." civs.md: Persian unique unit confirmed. |
| 4 | ONAGER         | units.md — Siege table: "Onager, Imperial, 160W 135G, Strong vs. Grouped units, trees. Weak vs. Cavalry, Bombard Cannons." maps.md: Black Forest — "Onagers and Siege Onagers can cut through trees." |
| 5 | IRON CASTING   | techs.md — Melee Attack chain: "Forging (Feudal) → Iron Casting (Castle, 200F 100G, +1 attack) → Blast Furnace (Imperial)." |

### Step 2 — Extract the indicated letter (spaces and hyphens not counted)

| # | Term          | Indexed sequence                                    | Pos | Extracted |
|---|---------------|-----------------------------------------------------|-----|-----------|
| 1 | BATTERING RAM | B(1) A(2) **T(3)** T(4) E(5) R(6) I(7) N(8) G(9) R(10) A(11) M(12) |  3  | **T**     |
| 2 | HORSE COLLAR  | H(1) **O(2)** R(3) S(4) E(5) C(6) O(7) L(8) L(9) A(10) R(11)       |  2  | **O**     |
| 3 | WAR ELEPHANT  | **W(1)** A(2) R(3) E(4) L(5) E(6) P(7) H(8) A(9) N(10) T(11)       |  1  | **W**     |
| 4 | ONAGER        | O(1) N(2) A(3) G(4) **E(5)** R(6)                                   |  5  | **E**     |
| 5 | IRON CASTING  | I(1) **R(2)** O(3) N(4) C(5) A(6) S(7) T(8) I(9) N(10) G(11)       |  2  | **R**     |

### Step 3 — Read the answer

T — O — W — E — R = **TOWER**

The answer is self-confirming: TOWER is the defensive structure that dominates the University upgrade chain (Watch Tower → Guard Tower → Keep), which each of the five clued terms relates to in a causal or strategic way. The Battering Ram destroys towers; the War Elephant's trample demolishes walled positions; the Onager clears forests hiding tower networks; IRON CASTING and HORSE COLLAR are the economic and military underpinnings of any army capable of reaching and sustaining a tower rush or tower defense.

---

## Trait Application

Systematic Thinking shaped this design by treating the puzzle as a procedure rather than a collection of clues: each row specifies an input (clue), an operation (identify the term), a parameter (position index), and an output (one letter), making the full solution path a five-step chain that can be executed — and verified — one row at a time without any ambiguity about what the solver is supposed to do. The extraction indices were chosen so that the correct path through each term's letter sequence is unambiguous and checkable against the world data, reflecting the same deterministic logic that governs the AoE2 tech tree itself: every prerequisite is explicit, every output follows from a known input.
