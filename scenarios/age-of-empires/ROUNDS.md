# Rounds — Hunt Structure

Define rounds, puzzle flow, meta architecture, and how rounds connect. Filled out during Stage 2 (Structure).

---

## Hunt Architecture

### Round Map

<!-- Draw your round structure. Examples: -->

#### Example: Linear (3 rounds + super-meta)
```
Round 1 (8 puzzles) → Meta 1
Round 2 (8 puzzles) → Meta 2
Round 3 (8 puzzles) → Meta 3
                          ↓
              Meta 1 + Meta 2 + Meta 3
                          ↓
                     Super-Meta
                          ↓
                     Final Answer
```

#### Example: Branching (unlock-based)
```
Round 1 (6 puzzles) → Meta 1 → unlocks Round 2 + Round 3
Round 2 (8 puzzles) → Meta 2 ─┐
Round 3 (8 puzzles) → Meta 3 ─┤→ Super-Meta → Final Answer
Round 4 (6 puzzles) → Meta 4 ─┘
                               (Round 4 unlocks after Meta 2 OR Meta 3)
```

#### Your architecture:
```
(draw yours here)
```

---

## Round Definitions

### Round 1

| Field | Value |
|-------|-------|
| **Name** | |
| **Theme** | |
| **Puzzles** | (count) |
| **Difficulty range** | (1-5 warm-up to expert) |
| **Section(s) covered** | |
| **Meta mechanism** | |
| **Unlocks** | (what does solving this round open?) |
| **Narrator voice** | (if applicable) |

**Puzzle slots:**

| Slot | Section | Puzzle type | Difficulty | Status |
|------|---------|-------------|------------|--------|
| 1.1 | | | | — |
| 1.2 | | | | — |
| 1.3 | | | | — |
| ... | | | | — |

**Meta 1:**

| Field | Value |
|-------|-------|
| **Mechanism** | |
| **Feeds from** | (which puzzle slots) |
| **Answer (encoded)** | |
| **80% rule** | Can it be solved with N-1 of N feeder answers? |

---

### Round 2

*(Copy the Round 1 template and fill in)*

---

### Round N

*(Copy as needed)*

---

## Super-Meta (if applicable)

| Field | Value |
|-------|-------|
| **Feeds from** | (which round metas) |
| **Mechanism** | |
| **Answer (encoded)** | |
| **Relationship to round metas** | (how do they combine?) |
| **The aha** | (what does the solver realize when it clicks?) |

---

## Difficulty Curve

Map difficulty across the full hunt:

```
Difficulty
5 |                                          ██
4 |                          ██    ██    ██  ██
3 |              ██    ██    ██    ██    ██  ██
2 |    ██    ██  ██    ██    ██
1 |    ██    ██
  └──────────────────────────────────────────────
    R1.1  R1.4  R2.1  R2.4  R3.1  R3.4  Meta  Super
```

---

## Unlock Logic

How do rounds gate each other?

| Condition | Unlocks |
|-----------|---------|
| Complete Round 1 meta | Round 2, Round 3 |
| Complete any 2 round metas | Super-Meta |
| Solve N puzzles in Round X | Round X meta becomes available |
| (define yours) | |

---

## Numbering System

| Round | Puzzle numbering | Rationale |
|-------|-----------------|-----------|
| Round 1 | 1-8 / or custom | |
| Round 2 | 9-16 / or custom | |
| Meta | M1, M2... | |
| Super | S1 | |

If using thematic numbering (atomic numbers, fibonacci, etc.), document the system here.
