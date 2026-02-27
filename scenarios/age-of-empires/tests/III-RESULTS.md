# Puzzle III — Castle Age — Test Results

**Puzzle:** Tech Tree Gap-Fill (4 technology chains, identify missing tech, extract letters)
**Testers:** Huang, Snyder, Gottlieb
**Answer:** LOOM

---

## Tester 1: Wei-Hwa Huang (Deductive Rigor & Solve-Path Quality)

### Solve Attempt

Four chains, each missing one technology. The descriptions are precise enough to uniquely determine each answer.

**Chain 1 — Ranged Attack:** ? -> Bodkin Arrow -> Bracer. Ranged attack upgrades at the Blacksmith. Three tiers: Feudal, Castle, Imperial. The first tier is Fletching. F-L-E-T-C-H-I-N-G (9 letters, matches the blanks). Letter 2 = **L**.

**Chain 2 — Melee Attack:** Forging -> ? -> Blast Furnace. Melee attack upgrades at the Blacksmith. Three tiers: Feudal, Castle, Imperial. The middle tier is Iron Casting. I-R-O-N-C-A-S-T-I-N-G (ignoring the space: 11 letters, matches). Letter 3 = **O**.

**Chain 3 — Lumber Efficiency:** Double-Bit Axe -> ? -> Two-Man Saw. Lumber Camp upgrades. Three tiers: Feudal, Castle, Imperial. The middle tier is Bow Saw. B-O-W-S-A-W (ignoring the space: 6 letters, matches). Letter 2 = **O**.

**Chain 4 — Fortification:** ? -> Architecture. University building upgrades for building HP/armor. Two tiers: Castle, Imperial. The first tier is Masonry. M-A-S-O-N-R-Y (7 letters, matches). Letter 1 = **M**.

Letters in order: L, O, O, M = **LOOM**.

**Answer: LOOM**

Solve time: ~4 minutes. Very fast -- each chain uniquely determines its answer.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Each chain is presented identically: known techs + gap + age labels. The visual arrow diagrams are clear. Blank counts confirm letter length. |
| Solvability | 5 | Each chain has exactly one correct answer. No ambiguity. The AoE2 tech tree is well-documented and consistent. |
| Elegance | 2 | This is four independent lookups. There is zero interlock between chains. Solving Chain 1 gives you no information about Chain 2. The hint about "two chains live at the same building" (the Blacksmith) is flavor, not a solving constraint. |
| Reading Reward | 4 | You must know the AoE2 tech tree. These are real dependency chains -- the solver is demonstrating knowledge of the research order. |
| Fun | 3 | Filling in blanks in a tech tree is satisfying if you know the answers, but it's essentially a recall quiz. There's no aha moment -- just "do I know this?" four times. |
| Confirmation | 5 | LOOM is unambiguous. Four letters, reads perfectly. LOOM is also a technology in AoE2 (the first Dark Age tech at the Town Center), which is a satisfying thematic loop. |
| **Total** | **24** | |

### Issues

| Severity | Issue |
|----------|-------|
| Major | Zero interlock. Four independent gap-fills with no cross-referencing. This is a quiz, not a puzzle (per the Interlock principle). Solving any one chain gives zero information about the others. |
| Minor | The "two chains live at the same building" hint adds no solving value. Both Chain 1 and Chain 2 are at the Blacksmith, but knowing this doesn't help you identify the missing techs. |

### Suggested Fixes

1. Add interlock: perhaps the missing techs share a property that helps disambiguate them, or solving one chain constrains which building another chain must be at.
2. Make the "same building" hint functional -- e.g., if the solver had to determine WHICH building each chain belongs to, and two sharing a building was a constraint that helped solve a harder chain.
3. Consider adding a 5th or 6th chain where the missing tech is less obvious, and the building/age constraints from other chains help narrow it down.

---

## Tester 2: Thomas Snyder (Craftsmanship & Intentional Solve Path)

### Solve Attempt

Four tech chains. Each has a gap. The visual layout is clean -- arrow diagrams with age labels. The blank lengths serve as confirmation.

Chain 1: Fletching (L). Chain 2: Iron Casting (O). Chain 3: Bow Saw (O). Chain 4: Masonry (M).

LOOM.

**Answer: LOOM**

Solve time: ~3 minutes.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Excellent visual design. The arrow chains are immediately legible. Age labels anchor each tech in the progression. |
| Solvability | 5 | Each gap has one answer. Blank lengths confirm. No ambiguity. |
| Elegance | 2 | Could a computer generate this puzzle? Yes -- pick any four tech chains, remove one node, index into the answer. There's no hand-crafted quality. No intentional solving path. The solver fills in four independent blanks and reads four letters. Snyder's Computer Test: fails. |
| Reading Reward | 4 | You need real tech tree knowledge. The dependency order (Feudal -> Castle -> Imperial) and the correct names (Fletching, not "Archery upgrade 1") require domain knowledge. |
| Fun | 2 | This feels like homework. Four recall questions. No surprise, no twist, no aha. The answer LOOM is thematic (it's a tech in the game), but the path to it is flat. |
| Confirmation | 5 | LOOM is clean and thematic. |
| **Total** | **23** | |

### Issues

| Severity | Issue |
|----------|-------|
| Major | Fails Snyder's Computer Test. A script that maps tech chains to their members and indexes letters would solve this instantly. There is no deduction, no judgment, no insight. |
| Major | No interlock (same as Huang's critique). Four independent recall questions. |
| Minor | The puzzle title says "Four technologies are missing" but the intro says "four technology chains." The intro also says "six missing techs" in the PUZZLES.md brief but only four are actually missing. (This is a discrepancy between the brief and the authored puzzle -- not visible to solvers, but noted.) |

### Suggested Fixes

1. Add a deduction layer: perhaps one chain is deliberately ambiguous (two possible techs fit) and the solver must use a cross-chain constraint to disambiguate.
2. Increase the number of chains and make some of them share letters, creating interlock through the answer word itself.
3. Consider a variant where the solver must also determine the ORDER of chains (which age each belongs to), adding a sorting puzzle on top of the identification.

---

## Tester 3: Mark Gottlieb (Systems Engineering & Academic Rigor)

### Solve Attempt

The tech tree is a dependency graph. Each chain is a linear sequence. One node is missing per chain. This is a graph-completion problem.

Chain 1: Fletching (the Feudal-age ranged attack upgrade). Letter 2 = L.
Chain 2: Iron Casting (the Castle-age melee attack upgrade). Letter 3 = O.
Chain 3: Bow Saw (the Castle-age lumber upgrade). Letter 2 = O.
Chain 4: Masonry (the Castle-age building HP upgrade). Letter 1 = M.

LOOM. A Dark Age technology. The answer is a tech that doesn't appear in any of the chains -- it's a standalone tech, not part of a dependency chain. That's a subtle thematic point: LOOM stands alone, just as the answer stands apart from the chains.

**Answer: LOOM**

Solve time: ~4 minutes.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | The chain format is clean and consistent. The "Two of these chains live at the same building" hint is a nice flavor note. |
| Solvability | 5 | Uniquely determined at every step. The blank counts serve as confirmation. |
| Elegance | 3 | The system is clean but shallow. The tech tree has a rich dependency structure that this puzzle barely touches -- it only uses linear chains. A more ambitious version would use branching dependencies or shared prerequisites. The "two chains at the same building" hint gestures at this richer structure but doesn't use it. |
| Reading Reward | 4 | Requires tech tree knowledge. The solver must know the specific upgrade names and their order. |
| Fun | 3 | Functional but not exciting. The answer LOOM is a nice thematic payoff, but the journey to it is flat. |
| Confirmation | 5 | LOOM is clean. The thematic resonance (Loom is the first tech most players research) adds to the satisfaction. |
| **Total** | **25** | |

### Issues

| Severity | Issue |
|----------|-------|
| Major | The puzzle uses the simplest possible structure of the tech tree (linear chains) when the actual tech tree has branching, shared prerequisites, and building-level organization. This is a missed opportunity to create genuine deduction. |
| Minor | The "two chains live at the same building" hint is decorative, not functional. The solver doesn't need this information to solve. |

### Suggested Fixes

1. Use branching dependencies: "Technology X requires both Y and Z" -- where the solver must deduce X from knowing Y and Z (or vice versa).
2. Make building identification part of the puzzle: "These four techs are researched at three different buildings. Which building is each from?"
3. Consider having some chains where the missing tech could be one of two options, and the building constraint or a shared letter helps disambiguate.

---

## Synthesis

| Dimension | Huang | Snyder | Gottlieb | Average |
|-----------|-------|--------|----------|---------|
| Clarity | 5 | 5 | 5 | 5.0 |
| Solvability | 5 | 5 | 5 | 5.0 |
| Elegance | 2 | 2 | 3 | 2.3 |
| Reading Reward | 4 | 4 | 4 | 4.0 |
| Fun | 3 | 2 | 3 | 2.7 |
| Confirmation | 5 | 5 | 5 | 5.0 |
| **Total** | **24** | **23** | **25** | **24.0** |

### Verdict: PASS (24.0/30) -- Barely

This puzzle passes the threshold but is the weakest in the set. It has excellent clarity, solvability, and confirmation, but critically low elegance and fun. All three testers flagged the same core problem: four independent lookups with no interlock, no deduction, and no aha moment. It fails Snyder's Computer Test. The answer LOOM is thematically satisfying, and the tech tree is the right content domain for a Castle Age puzzle, but the mechanism needs a deduction layer.

### Consensus Issues

1. **No interlock** (Major): All three testers flagged this. Four independent gap-fills. This is the puzzle's most serious structural weakness.
2. **Fails Computer Test** (Major): Snyder explicitly flagged this. A script could solve this puzzle trivially.
3. **Flat fun curve** (Major): No aha moment. The puzzle is a recall quiz. Acceptable for difficulty 3, but the fun scores (2-3) suggest the mechanism needs work.
4. **Unused building hint** (Minor): "Two chains live at the same building" adds no solving value. Make it functional or remove it.
5. **Strong confirmation** (Positive): LOOM as an answer is thematically perfect -- it's the first tech most players research. All three testers praised this.
