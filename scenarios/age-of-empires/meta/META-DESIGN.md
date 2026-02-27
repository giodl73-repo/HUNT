# Meta Puzzle Design

How feeder puzzle answers combine into the final answer. Fill in during Stage 5.

---

## Meta Architecture

| Field | Value |
|-------|-------|
| **Number of metas** | (one per round? plus super-meta?) |
| **Feeder count per meta** | |
| **Meta mechanism** | (crossword, acrostic, pattern, elimination, physical, custom) |
| **Super-meta mechanism** | (if applicable) |
| **Final answer** | (encoded) |

---

## Meta Mechanisms — Reference

### Crossword
Feeder answers fill a crossword grid. Where words cross, letters must match (confirmation). Highlighted squares spell the meta answer.
- **Pros:** Built-in confirmation via crossings. Backsolving possible. 80% rule naturally satisfied.
- **Cons:** Grid must be constructable with the actual answer words. Requires careful letter-overlap planning.

### Acrostic
First letters of feeder answers, in some order, spell the meta answer.
- **Pros:** Simple, elegant.
- **Cons:** Brute-forceable with partial answers. Every feeder is load-bearing (fails 80% rule).

### Pattern Recognition
All feeder answers share a hidden property. The property IS the meta answer.
- **Pros:** Beautiful when it works.
- **Cons:** Hard to construct. Multiple valid patterns may exist.

### Elimination (Clue-style)
Multiple candidate answers. Evidence from feeders eliminates all but one.
- **Pros:** Deductive. Hard to brute-force. Multiple "aha" moments.
- **Cons:** Complex to design. Solver needs to know the candidates exist.

### Physical
Build something from puzzle pages. The physical object reveals the answer.
- **Pros:** Legendary when it works. The "Chicago Fire" moment.
- **Cons:** High construction risk. Requires physical format.

### Combined
Layer two mechanisms (e.g., acrostic yields a word that leads to a hidden file).
- **Pros:** Multiple reward moments.
- **Cons:** More complex to design and test.

---

## Round Meta Design

### Round 1 Meta

| Field | Value |
|-------|-------|
| **Feeds from** | (puzzle IDs) |
| **Mechanism** | |
| **Answer (encoded)** | |
| **80% rule** | Can solve with N-1 feeders? |
| **Backsolving** | Can a solver deduce a feeder from the meta? |
| **Brute-force resistance** | |

### Round N Meta

*(Copy template as needed)*

---

## Super-Meta (if applicable)

| Field | Value |
|-------|-------|
| **Feeds from** | (round meta answers) |
| **Mechanism** | |
| **Answer (encoded)** | |
| **The relationship** | (how do the round answers connect?) |
| **The aha** | (what does the solver realize?) |
| **Hard to backsolve?** | (knowing round 1 answer, can you guess round 2?) |

---

## Verification Checklist

Before shipping the meta:
- [ ] All feeder answer words confirmed correct
- [ ] Crossword grid (if applicable) is constructable — no impossible crossings
- [ ] The meta is solvable with ~80% of feeders
- [ ] The meta cannot be brute-forced from partial information
- [ ] The meta answer is surprising yet inevitable in retrospect
- [ ] The meta has been tested (use `/puzzle-test` with Katz, Huang, Gottlieb)
