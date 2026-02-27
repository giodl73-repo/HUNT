# Puzzle V — Post-Imperial — Test Results

**Puzzle:** Economy Puzzle (6 arithmetic questions, numbers become letters)
**Testers:** Huang, Kenny, Katz
**Answer:** PATROL

---

## Tester 1: Wei-Hwa Huang (Deductive Rigor & Solve-Path Quality)

### Solve Attempt

Six arithmetic questions. Each yields a number. A=1, B=2, ... Z=26.

**Q1:** 800 food / 50 food per Villager = **16** -> P

**Q2:** Standard game start: 1 Town Center, 1 Scout, 3 Villagers. "How many Scout Cavalry do you start with?" = **1** -> A

**Q3:** 1000 food / 50 food per Villager = **20** -> T

**Q4:** Total farm upgrade cost: 75 + 125 + 250 = 450 wood. 450 / 25 wood per House = **18** -> R

**Q5:** Cavalier upgrade costs 300 gold. Militia costs 20 gold. 300 / 20 = **15** -> O

**Q6:** 7 minutes = 420 seconds. 420 / 35 seconds per Archer = **12** -> L

Letters: P, A, T, R, O, L = **PATROL**

**Answer: PATROL**

Solve time: ~3 minutes. Trivially fast.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Each question is unambiguous. The numbers are explicitly provided. The A=1, B=2 conversion is standard. |
| Solvability | 5 | Pure arithmetic. No ambiguity, no tricks ("no tricks, no traps" is literally stated). Every number is given. |
| Elegance | 1 | This is six division problems with a substitution cipher. There is zero deduction. Zero interlock. Zero aha. Every input is given, every operation is division, every output is a small integer. A calculator could solve this. This fails every craft test in the PRINCIPLES.md. |
| Reading Reward | 2 | Q2 requires knowing the starting units (trivial AoE2 knowledge). All other questions provide every number in the question itself. You don't need to know ANYTHING about AoE2 to solve Q1, Q3, Q4, Q5, or Q6. The game costs are given inline. |
| Fun | 2 | Six identical division problems. No variety, no surprise, no discovery. The flavor text is AoE-themed but the solving experience is "divide and look up letter." |
| Confirmation | 4 | PATROL is clean and thematic (Patrol is a unit command in AoE2). Six letters, six questions. |
| **Total** | **19** | |

### Issues

| Severity | Issue |
|----------|-------|
| Blocking | Fails "No Computation Without Deduction" principle. ALL six questions are pure computation: given complete inputs, apply division, get output. 0/6 questions require deduction. The PRINCIPLES.md notes that "4/4 pure-computation puzzles failed testing." This is a 5th. |
| Blocking | Fails Snyder's Computer Test. A 3-line script solves this puzzle: `answers = [800//50, 1, 1000//50, 450//25, 300//20, 420//35]; print(''.join(chr(64+a) for a in answers))` |
| Major | Reading Reward is critically low (2/5). Five of six questions provide ALL necessary numbers inline. Only Q2 requires any game knowledge (and it's the most trivial knowledge in the game: "you start with 1 scout"). |
| Major | Zero interlock. Six independent questions. The answer to Q1 gives no information about Q2. |
| Minor | Every question uses the same operation (division). No variety in mathematical structure. |

### Suggested Fixes

1. Remove inline numbers. Force the solver to find costs in the game knowledge (or an in-puzzle "research table" that doesn't directly state the answer). Example: "How much gold would it take to..." where the solver must recall or look up the gold cost.
2. Add at least one question that requires deduction, not just computation. Example: "Your enemy has 6 Knights. What is the MINIMUM number of Pikemen to counter them, given that each Pikeman has 22 attack vs cavalry and a Knight has 100 HP?" -- this requires understanding counter damage, not just dividing.
3. Create interlock: the answer to one question constrains another. Example: "The number from Q1 is also the number of Villagers assigned to food in Q4's scenario."
4. Vary the operations: include a question that requires multiplication, comparison, or optimization -- not just division.

---

## Tester 2: Kenny Young (Infrastructure & Buildability)

### Solve Attempt

Six questions, straightforward arithmetic. Let me work through them.

Q1: 800/50 = 16 -> P
Q2: 1 Scout -> A
Q3: 1000/50 = 20 -> T
Q4: (75+125+250)/25 = 18 -> R
Q5: 300/20 = 15 -> O
Q6: 420/35 = 12 -> L

PATROL.

**Answer: PATROL**

Solve time: ~2 minutes.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Crystal clear. Every number is stated. The conversion rule is standard. |
| Solvability | 5 | Cannot be clearer. A middle-schooler with AoE2 knowledge could solve this. |
| Elegance | 2 | The seed idea ("economy questions become letters") has potential, but the execution is pure computation. No puzzle structure beyond "compute and convert." Q4 at least requires summing three values before dividing, which is marginally more interesting. |
| Reading Reward | 2 | Almost all numbers are provided inline. Q2 is the only question requiring game knowledge, and it's the most basic fact in the game. |
| Fun | 2 | Filling in a worksheet of division problems is not fun. It's homework. The AoE2 flavor is nice but doesn't rescue the experience. |
| Confirmation | 4 | PATROL is clean. The word itself is a nice surprise -- "Patrol" is a unit command, which fits the Post-Imperial theme (endgame micro-management). |
| **Total** | **20** | |

### Issues

| Severity | Issue |
|----------|-------|
| Blocking | The seed is there -- "economy questions whose answers become letters" -- but the execution doesn't develop it into a puzzle. Each question is self-contained, self-sufficient computation. There's no constraint, no discovery, no deduction. The seed needs a redesign pass, not just polish. |
| Major | Q1 and Q3 are structurally identical (food cost / villager cost). This feels repetitive. |
| Minor | Q6's "35 seconds per Archer" is a fact most players would need to look up. Providing it inline is consistent with the other questions, but it means the puzzle has zero lookup requirements. |

### Suggested Fixes

1. Redesign around a single economy scenario: "You have 40 villagers and need to reach Imperial Age. Allocate villagers to food, wood, gold, and stone." The constraints (age-up costs, build costs, research costs) force multi-variable optimization. The answer letters come from the optimal allocation values.
2. At minimum, remove inline numbers for 3-4 questions and require the solver to recall or derive them from AoE2 knowledge.
3. Add variety: include a question about trade (gold income from trade carts), military production (how many barracks to produce X units in Y time), or eco upgrades (how much does Wheelbarrow save over Z trips).

---

## Tester 3: Dan Katz (Structure & Calibration)

### Solve Attempt

Q1: 16 (P). Q2: 1 (A). Q3: 20 (T). Q4: 18 (R). Q5: 15 (O). Q6: 12 (L).

PATROL.

**Answer: PATROL**

Solve time: ~2 minutes.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Impeccable clarity. Nothing ambiguous. |
| Solvability | 5 | Trivially solvable. This is the easiest puzzle in the set despite being labeled difficulty 3 and positioned as the Post-Imperial (hardest age) entry. |
| Elegance | 1 | Six division problems. No structure, no interlock, no deduction, no aha. The puzzle is its own worksheet. Removing the worksheet and instructions leaves... nothing. This is the definition of "No Computation Without Deduction" failure mode. |
| Reading Reward | 2 | Five of six questions give all numbers inline. Q2 requires knowing you start with 1 Scout. That's it. I could hand this puzzle to someone who's never played AoE2 and they'd solve 5 of 6 questions. |
| Fun | 2 | "I can't imagine how this would be someone's favorite puzzle in the set." It's positioned as the difficulty 3 capstone and it's easier than Puzzle I. The difficulty rating is miscalibrated. |
| Confirmation | 4 | PATROL is clean and thematic. |
| **Total** | **19** | |

### Issues

| Severity | Issue |
|----------|-------|
| Blocking | Complete failure of the "No Computation Without Deduction" principle. This is the archetype of what that principle was written to prevent. Given inputs + given operation + given formula = worksheet, not puzzle. |
| Blocking | Difficulty miscalibration. Labeled difficulty 3 (hardest), positioned at Post-Imperial (the endgame), but is the easiest puzzle in the set. A difficulty 3 puzzle should be the most challenging, not the most trivial. |
| Major | "Every number you need is in the game. No tricks, no traps -- just clean arithmetic." This is the puzzle TELLING you it has no deduction layer. The author seems to think "clean arithmetic" is a feature. It's a bug. |
| Major | Q1 and Q3 are the same question with different numbers. This is repetition, not variety. |

### Suggested Fixes

1. Fundamental redesign required. The answer word PATROL is good. The economy theme is right. But the mechanism must change from "six independent computations" to something with deduction, interlock, or constraint satisfaction.
2. Possible redesign: an economy optimization problem where the solver must allocate limited resources across competing goals. The optimal allocation values map to letters. This would create genuine constraint satisfaction with AoE2 flavor.
3. At minimum: remove all inline numbers, require the solver to know or derive AoE2 economy values, and add at least two questions that require reasoning beyond simple division.

---

## Synthesis

| Dimension | Huang | Kenny | Katz | Average |
|-----------|-------|-------|------|---------|
| Clarity | 5 | 5 | 5 | 5.0 |
| Solvability | 5 | 5 | 5 | 5.0 |
| Elegance | 1 | 2 | 1 | 1.3 |
| Reading Reward | 2 | 2 | 2 | 2.0 |
| Fun | 2 | 2 | 2 | 2.0 |
| Confirmation | 4 | 4 | 4 | 4.0 |
| **Total** | **19** | **20** | **19** | **19.3** |

### Verdict: REDESIGN (19.3/30)

This puzzle fails testing decisively. All three testers independently identified the same blocking issues: pure computation with no deduction, critically low Reading Reward, and zero interlock. The average score of 19.3 is well below the 22-point PASS threshold. The elegance average of 1.3 is catastrophic.

The answer word PATROL is good and should be preserved. The economy theme is appropriate for a Post-Imperial puzzle. But the mechanism requires a fundamental redesign, not fixes. The six-independent-divisions structure cannot be salvaged through polish.

### Consensus Issues

1. **Pure computation** (Blocking): All three testers flagged this as the primary failure. 6/6 questions are given-input division. Zero deduction.
2. **Reading Reward** (Blocking): All three scored 2/5. Five of six questions provide all numbers inline. No game knowledge required.
3. **No interlock** (Major): Six independent questions. Zero cross-referencing.
4. **Difficulty miscalibration** (Major): Labeled difficulty 3 but is the easiest puzzle in the set. Positioned as Post-Imperial capstone but could be solved by a non-AoE2-player.
5. **Repetitive structure** (Minor): Q1 and Q3 are the same question with different numbers. All six use the same operation.
6. **Good answer word** (Positive): PATROL is thematic and surprising. Preserve it in the redesign.
