# Hunt-Level Synthesis — Wololo (Age of Empires)

**5 puzzles tested · 15 tester sessions · 90 dimension scores**

---

## Scorecard

| Puzzle | Age | Testers | Clarity | Solvability | Elegance | Reading Reward | Fun | Confirmation | Total | Verdict |
|--------|-----|---------|---------|-------------|----------|----------------|-----|--------------|-------|---------|
| I | Dark | Rosenthal, Miller, Dana | 5.0 | 4.0 | 3.0 | 4.0 | 4.0 | 4.3 | **24.3** | PASS |
| II | Feudal | Katz, Selinker, Blow | 4.3 | 5.0 | 4.3 | 3.7 | 4.7 | 4.0 | **26.0** | PASS |
| III | Castle | Huang, Snyder, Gottlieb | 5.0 | 5.0 | 2.3 | 4.0 | 2.7 | 5.0 | **24.0** | PASS |
| IV | Imperial | Sarrett, Miller, Dana | 4.3 | 4.0 | 4.3 | 3.0 | 5.0 | 4.0 | **24.7** | PASS |
| V | Post-Imp | Huang, Kenny, Katz | 5.0 | 5.0 | 1.3 | 2.0 | 2.0 | 4.0 | **19.3** | REDESIGN |

**Hunt average: 23.7/30 (excluding V: 24.8/30)**

---

## Dimension Averages Across All 5 Puzzles

| Dimension | Average | Assessment |
|-----------|---------|------------|
| Clarity | 4.7 | Excellent -- instructions are clear throughout |
| Solvability | 4.6 | Excellent -- no puzzle is unfairly hard or ambiguous |
| Elegance | 3.1 | Weak -- the biggest gap across the set. III and V drag the average down hard |
| Reading Reward | 3.3 | Below target -- the hunt doesn't consistently require AoE2 knowledge to solve |
| Fun | 3.7 | Mixed -- II and IV shine, III and V are flat |
| Confirmation | 4.3 | Strong -- all answer words are clean and thematic |

---

## Verdict Summary

| Puzzle | Status | Action Required |
|--------|--------|-----------------|
| I — Dark Age | **PASS** | Minor polish (verify Japanese bonus %, consider improving chaff letters) |
| II — Feudal Age | **PASS** | Minor polish (use or remove FINAL bracket slot, consider trimming explicit counter rules) |
| III — Castle Age | **PASS** (barely) | Major revision recommended: add interlock between chains, add a deduction layer, make the building hint functional |
| IV — Imperial Age | **PASS** | Major revision recommended for Reading Reward: make resource types structural, not decorative |
| V — Post-Imperial | **REDESIGN** | Fundamental mechanism change required. Keep PATROL as answer, keep economy theme, redesign from scratch |

---

## Does the 5-Puzzle Set Work as a Unit?

### Difficulty Curve

**Intended:** 2, 2, 3, 3, 3 (ascending with the Ages)

**Actual (by solve time and tester experience):**

| Puzzle | Intended Difficulty | Actual Difficulty | Mismatch |
|--------|-------------------|-------------------|----------|
| I | 2 | 2 | Correct |
| II | 2 | 2 | Correct |
| III | 3 | 2 | Too easy -- pure recall, no deduction |
| IV | 3 | 2.5 | Plotting is mechanical; letter recognition adds some challenge |
| V | 3 | 1 | Catastrophically miscalibrated -- six division problems |

The difficulty curve is **flat, then descends**. The hunt gets EASIER as you progress, which is the opposite of the intended Age progression. The Dark Age and Feudal Age puzzles are appropriately calibrated. The Castle Age, Imperial Age, and Post-Imperial puzzles should be harder but are not.

**Fix:** III and V need deduction layers. IV needs Reading Reward. The Post-Imperial puzzle especially must be the hardest in the set to match the Age theme.

### Variety

| Puzzle | Mechanism | Physical Activity | Cognitive Activity |
|--------|-----------|-------------------|-------------------|
| I | Identify-and-index | Write names | Recall civ bonuses |
| II | Bracket deduction | Fill in bracket | Apply counter logic + one exception |
| III | Gap-fill | Write tech names | Recall tech chains |
| IV | Coordinate plotting | Draw dots on grids | Recognize letters |
| V | Arithmetic | Compute | Divide numbers |

**Assessment:** Good surface variety -- five different puzzle types. But the cognitive activities cluster around recall and computation. Only Puzzle II requires genuine deduction (the Cataphract exception). The set needs more puzzles where the solver must REASON, not just REMEMBER or CALCULATE.

### Mechanism Diversity

The extraction mechanisms are similar across all five puzzles:
- I: Circle indexed letter from answer word
- II: Circle indexed letter from winner name
- III: Circle indexed letter from tech name
- IV: Plot dots, recognize letter
- V: Number-to-letter conversion (A=1)

Four of five puzzles use the same "index into a word" extraction. Only IV (dot plotting) and V (number conversion) differ. This is somewhat repetitive. The meta crossword will use these answer words, so the extraction itself is constrained, but the intermediate mechanisms could vary more.

### Tone Consistency

The Monk voice ("Eight civilizations stand before you," "Eight warriors enter. One remains," "The research queue is broken") is consistent across all five puzzles. Each puzzle opens with a thematic line in the Monk's voice, followed by clear instructions, followed by the puzzle content. This is well-executed.

The Age progression is thematic:
- Dark Age: basic civ identification (fundamental knowledge)
- Feudal Age: unit combat (first military encounters)
- Castle Age: technology research (mid-game development)
- Imperial Age: map control (late-game territorial awareness)
- Post-Imperial: economy (endgame resource mastery)

This arc works narratively even though the difficulty curve doesn't match.

### Answer Words

| Puzzle | Answer | Thematic Fit | Surprise |
|--------|--------|-------------|----------|
| I | CASTLE | Building / Age name | Moderate -- expected from a civ puzzle but not guessable from bonuses |
| II | ONAGER | Siege unit (appears in the bracket) | High -- self-referential twist |
| III | LOOM | Dark Age technology | High -- a basic tech as the answer for a Castle Age puzzle is unexpected |
| IV | TOWER | Building | Moderate -- thematic but straightforward |
| V | PATROL | Unit command | High -- a command, not a unit or building |

Good variety. The answer words span buildings (CASTLE, TOWER), units (ONAGER), technologies (LOOM), and commands (PATROL). The "Surprise the Answer" principle is mostly satisfied -- ONAGER and PATROL are genuinely surprising.

### Meta Compatibility

The five answers feed a crossword meta whose highlighted squares spell WOLOLO:

- CASTLE (6)
- ONAGER (6)
- LOOM (4)
- TOWER (5)
- PATROL (6)

These words must cross in a grid. LOOM is the shortest at 4 letters, which limits crossing options. The O appears in ONAGER, LOOM, TOWER, and PATROL, providing abundant crossing material. The L appears in CASTLE, LOOM, and PATROL. WOLOLO requires W-O-L-O-L-O from highlighted squares -- the repeated O and L are well-supplied by these answers.

**The answer set is well-designed for the meta.**

---

## Top Issues (Hunt-Level)

### Blocking

1. **Puzzle V requires redesign.** 19.3/30 is well below threshold. All three testers identified the same core failure: pure computation with no deduction, minimal AoE2 knowledge required. The Post-Imperial puzzle must be the hardest, not the easiest.

### Major

2. **Reading Reward is below target across the set.** Average 3.3/5. Three puzzles (II, IV, V) scored below 4.0. The hunt should not be solvable without AoE2 knowledge, but currently large portions of II (explicit counter rules given), IV (only coordinates needed), and V (all numbers inline) could be solved by a non-player.

3. **Elegance gap.** Average 3.1/5. Puzzles III and V are the primary offenders. III is four independent lookups. V is six independent computations. Both lack interlock and deduction.

4. **Difficulty curve is inverted.** The hunt gets easier as it progresses. Fix: add deduction layers to III, IV, and the redesigned V, with V being the hardest.

### Minor

5. **Extraction repetition.** 4/5 puzzles use index-into-word extraction. Consider varying the extraction mechanism for at least one puzzle.

6. **No interlock within puzzles.** Only Puzzle II has any cross-referencing (the Cataphract exception informs the semifinal outcomes). Puzzles I, III, IV, and V are all sets of independent sub-tasks.

---

## Recommendations (Priority Order)

1. **Redesign Puzzle V** from scratch. Keep the answer PATROL and the economy theme. Replace six-independent-divisions with a constrained optimization problem. The solver should allocate resources under constraints, with the allocation values mapping to letters. This should be difficulty 3-4 and require real AoE2 economy knowledge.

2. **Revise Puzzle III** to add interlock and a deduction layer. Increase from 4 chains to 5-6. Make at least one chain ambiguous (two possible techs fit). Use cross-chain constraints (shared building, shared age, letter constraints from the answer word) to disambiguate. Target: Elegance 3+, Fun 3+.

3. **Boost Reading Reward for Puzzle IV.** Make resource types matter for solving. Options: different symbols for different resource types (only one type forms the letter), relative positioning (some coordinates derived from map knowledge rather than given), or one map where the solver must deduce a resource position from the map description.

4. **Consider trimming explicit counter rules in Puzzle II** to increase Reading Reward from 3.7 to 4.0+. The rules could be partially given (e.g., "Siege beats ___" with blanks) so the solver must fill in the counter chart from AoE2 knowledge.

5. **Clean up Puzzle II's bracket** -- either extract from the FINAL or remove it.

6. **Verify Puzzle I's Japanese bonus percentage** (33% vs 25%) against AoE2 Definitive Edition.

---

## Final Assessment

The Wololo hunt has a strong foundation. The Age-based progression is thematically compelling. Four of five answer words are well-chosen. The Monk voice is consistent. Puzzle II is genuinely excellent (26/30). The meta design (crossword with WOLOLO from highlighted squares) is sound.

The critical gap is depth. Too many puzzles are recall-and-compute rather than deduce-and-discover. The set needs more moments where the solver must THINK, not just KNOW. Puzzle II shows this is possible -- the Cataphract exception is a genuine aha that requires both game knowledge and reasoning. The other four puzzles need similar deductive beats.

After V is redesigned and III/IV are revised, this hunt should clear 24+ across all five puzzles and deliver a satisfying 2-3 hour solo experience for an AoE2 player.

| Metric | Current | Target |
|--------|---------|--------|
| Puzzles passing (>=22) | 4/5 | 5/5 |
| Puzzles >=24 | 4/5 | 5/5 |
| Hunt average | 23.7 | 25+ |
| Min elegance | 1.3 | 3.0+ |
| Min Reading Reward | 2.0 | 3.5+ |
| Min fun | 2.0 | 3.5+ |
