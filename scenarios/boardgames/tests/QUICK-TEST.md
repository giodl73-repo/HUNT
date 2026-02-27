# Quick Test — Game Night

**Stage 9: Abbreviated 1-tester check per puzzle**

Tester: The Captain (admin), acting as a cold solver for each puzzle.

---

## Scoring Dimensions

Each puzzle scored 1-5 on six dimensions:

| Dimension | 1 (Fail) | 3 (Acceptable) | 5 (Excellent) |
|-----------|----------|-----------------|---------------|
| **Clarity** | Cannot determine what to do | Instructions parseable with effort | Immediately clear what the puzzle asks |
| **Solvability** | Broken — no valid path to answer | Solvable but with friction | Clean, satisfying solve path |
| **Correctness** | Factual errors or impossible math | Minor issues that don't block solving | Every fact verified, math airtight |
| **Engagement** | Tedious busywork | Reasonably interesting | Genuinely fun — would tell someone about it |
| **Difficulty** | Trivially easy or impossibly hard | Appropriate for target audience | Sweet spot — satisfying challenge |
| **Extraction** | Extraction broken or arbitrary | Works but feels bolted on | Natural, elegant, earns the answer |

**PASS** = average >= 3.0 and no dimension at 1.
**REVISE** = any dimension at 1, or average < 3.0.

---

## M1 — Reconstructed (Chess)

**Tester approach**: Read the 8 observations cold. Attempt to reconstruct the position without the author's notes. Track time and friction.

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | "Place every piece" is clear. "The strongest move" is vague — is it the only legal move? The one that checkmates? The prettiest? Docked 1 point for ambiguity. |
| Solvability | 4 | The observations genuinely narrow to a unique position. The interlock between Obs 3, 4, 5, and 7 is well-designed — you MUST cross-reference. However, Obs 8's "exactly 6 squares" requires tedious counting. Docked 1 for the counting step. |
| Correctness | 4 | All chess facts are correct per world data. Castling conditions verified. Position is legal. Docked 1 because "strongest move" admits alternatives (other legal White moves exist). |
| Engagement | 5 | This is a genuine Dinner Party puzzle. "I reconstructed a chess position from 8 clues and the answer was castling" — that's a story worth telling. |
| Difficulty | 4 | Solidly difficulty 3. A chess player will recognize the castling setup quickly. A non-chess player will struggle with Obs 8's mobility count. Appropriate for target. |
| Extraction | 4 | "The name of that move" → CASTLE. Clean, surprising (not CHECKMATE). But extraction could be cleaner if the puzzle explicitly said "the chess term for this move is your answer" rather than "the name of that move." |

**Average: 4.17 — PASS**

**Flags**: Ambiguity in "strongest move" framing. Must be fixed in editing (see editorial review).

---

## M2 — Market Day (Settlers of Catan)

**Tester approach**: Start with the resource table. Track resources through all 5 actions. Determine what remains. Identify the answer.

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 3 | "Track the resources" is clear. "What is the one structure they could build?" is clear. But the port trade clue is ambiguous — the solver must deduce the port type, which is the aha, but it's not flagged as a puzzle-within-the-puzzle. |
| Solvability | 1 | **BROKEN.** Both 3:1 and 2:1 port interpretations lead to impossible action sequences (Wool or Lumber runs out before later actions). The puzzle cannot be solved as written because the action sequence is impossible with any port type. |
| Correctness | 1 | **Resource math is wrong.** Starting hand + action sequence + building costs produce an impossible state. Additionally, TRADE is not a Settlers structure name, so the answer doesn't connect to "the one structure they could build." |
| Engagement | 3 | The CONCEPT is engaging — deducing port type from resource ratios is clever. The execution kills it. If the math worked, this would score 4. |
| Difficulty | N/A | Cannot assess difficulty of an unsolvable puzzle. |
| Extraction | 1 | **No extraction path.** The puzzle asks for a structure name; Settlers structures are Road, Settlement, City, Development Card. None = TRADE. |

**Average: 1.80 — REVISE**

**Flags**: Three critical bugs (resource math, extraction, answer connection). This puzzle needs a complete v3 with verified math. The port-deduction concept should be preserved.

---

## M3 — Unnamed Continents (Risk)

**Tester approach**: Read each landmass description. Identify the continent using the world data. Perform the extraction. Check if letters spell BORDER.

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | Instructions are clear: identify continents, extract indexed letters, assemble. Each description has an explicit extraction instruction. |
| Solvability | 3 | Continent identification works — every description maps to exactly one continent. But the extraction step produces wrong letters (see editorial review). The PUZZLE is solvable; the EXTRACTION is broken. Solver would correctly identify all 6 continents but extract non-BORDER letters. |
| Correctness | 2 | Continent descriptions are factually correct per world data (territory counts, bonuses, entry points all verified). But extracted letters D-B-G-K-H-R do not spell BORDER. This is an error in the extraction design, not in the game facts. |
| Engagement | 4 | The descriptions are well-crafted — they interlock (references to "another continent with more than twice my territories") and embed numerical signatures. The Skeptic's compromise approach works. |
| Difficulty | 4 | Appropriate difficulty 2. A casual Risk player recognizes most continents from the descriptions. The interlock between descriptions adds satisfying cross-referencing. |
| Extraction | 1 | **Broken.** The territory names and letter indices do not produce B-O-R-D-E-R in order I-VI. The Skeptic's claim of "verified three times" is contradicted by the actual letters extracted. |

**Average: 3.00 — REVISE (due to extraction score of 1)**

**Flags**: The puzzle mechanism is excellent — the best-designed continent identification in the set. Only the extraction step (territory selection + index) needs repair. This is a surgical fix, not a redesign.

---

## M4 — Patient Zero (Pandemic)

**Tester approach**: Read the FINAL CLEAN VERSION only (lines 315+). Analyze the discard pile. Find layer boundaries. Reconstruct infection order. Extract first letters.

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 3 | The concept is clear after reading all instructions. However, the Step 1/2/3 scaffolding is too prescriptive — it tells the solver HOW to solve rather than WHAT to solve. This violates No Over-Scaffolding. The solver should discover the layer analysis approach themselves. |
| Solvability | 3 | The layer analysis logic works. Re-infections (Essen 3x, Delhi 2x, etc.) correctly reveal the boundaries. The cube-count = appearance-count rule simplifies the puzzle cleanly. But the first-infection-order extraction gives SPERAD, not SPREAD. Solvable in mechanism; broken in extraction. |
| Correctness | 2 | Cube counts match appearance counts — verified. Layer boundaries are recoverable — verified. But (a) the letter order gives SPERAD not SPREAD, and (b) the Epidemic Infect mechanic is hand-waved away (the puzzle says "each city's cube count equals appearances" which is only true if Epidemic Infect targets are outside this set). A Pandemic expert would flag this inconsistency. |
| Engagement | 5 | The "discard pile as stratigraphic record" concept is a Dinner Party puzzle. "I read the archaeology of a Pandemic discard pile and traced the infection back to Patient Zero" — outstanding concept. Best aha in the set. |
| Difficulty | 4 | Difficulty 3, well-calibrated. Requires real understanding of the Intensify mechanic. Non-Pandemic players would struggle; Pandemic players would feel rewarded. |
| Extraction | 2 | The extraction concept (first letters of cities in infection order) is thematic and natural. But the letter order is wrong — Essen at position 10 and Riyadh at position 9 produce E-before-R, giving SPERAD instead of SPREAD. A 2-position swap in the discard pile fixes this cleanly. |

**Average: 3.17 — REVISE (due to correctness issues)**

**Flags**: Swap positions 9 and 10 in the discard pile (Riyadh before Essen). Remove Step 1/2/3 scaffolding. Address Epidemic Infect hand-wave with a one-sentence clarification.

---

## M5 — Spymaster (Codenames)

**Tester approach**: Read the grid and key card. Identify the 9 Blue words. Look for a thematic connection among subsets. Check against the assassin. Determine the clue word.

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Crystal clear. The grid, key card, team roster, and rules are all presented cleanly. "Find the one-word clue that connects exactly 3 of your Blue words" is unambiguous. |
| Solvability | 4 | The solver must find a subset of 3 Blue words with a common theme and name that theme. ENCODE + SECRET + LOCK → CIPHER is a strong connection. The challenge is distinguishing this from weaker connections (e.g., could SHADOW + SECRET + LOCK connect via "hidden"? Yes, but HIDDEN doesn't fit ENCODE as well, and it's too close to BOMB territory). Docked 1 because the solver might consider alternate 3-word groupings before converging. |
| Correctness | 5 | Key card math is correct (9B/8R/7bystander/1X = 25). All words are real English words. CIPHER genuinely connects ENCODE + SECRET + LOCK and genuinely does NOT connect to BOMB. No factual errors. |
| Engagement | 4 | Playing spymaster-in-reverse is fun. "I had to find the perfect clue" is a good story. The BOLT/LOCK near-miss adds tension. Not quite Dinner Party level — the puzzle is more intellectual than dramatic. |
| Difficulty | 3 | Difficulty 2, well-calibrated. A Codenames player will pattern-match quickly. The cryptography theme (ENCODE, SECRET, LOCK) is not obscure. The main difficulty is avoiding BOMB — which a careful solver handles naturally. |
| Extraction | 5 | The answer IS the clue word. No indexing, no letter extraction, no additional transformation. The purest possible extraction: solve the puzzle, and the solution is the answer. Elegant. |

**Average: 4.33 — PASS**

**Flags**: Remove "The Connection" spoiler section. Otherwise, no changes needed. Cleanest puzzle in the set.

---

## Summary

| Module | Puzzle | Avg Score | Verdict | Critical Issues |
|--------|--------|-----------|---------|-----------------|
| M1 | Reconstructed | 4.17 | **PASS** | Reframe "strongest move" |
| M2 | Market Day | 1.80 | **REVISE** | Resource math impossible; no extraction |
| M3 | Unnamed Continents | 3.00 | **REVISE** | Extraction letters wrong |
| M4 | Patient Zero | 3.17 | **REVISE** | Letter order wrong (SPERAD); over-scaffolded |
| M5 | Spymaster | 4.33 | **PASS** | Remove spoiler section |

### Dimension Averages Across All 5 Puzzles

| Dimension | M1 | M2 | M3 | M4 | M5 | Avg |
|-----------|----|----|----|----|----|----|
| Clarity | 4 | 3 | 4 | 3 | 5 | 3.8 |
| Solvability | 4 | 1 | 3 | 3 | 4 | 3.0 |
| Correctness | 4 | 1 | 2 | 2 | 5 | 2.8 |
| Engagement | 5 | 3 | 4 | 5 | 4 | 4.2 |
| Difficulty | 4 | N/A | 4 | 4 | 3 | 3.75 |
| Extraction | 4 | 1 | 1 | 2 | 5 | 2.6 |

**Weakest dimension: Extraction (2.6).** Three of five puzzles have broken extractions. The puzzle mechanisms are strong (Engagement: 4.2) but the "last mile" — getting from the aha to the answer word — is where the failures cluster.

**Strongest dimension: Engagement (4.2).** Every puzzle has a genuine concept worth discussing. The board-game theme is rich territory.

### Pattern

The extraction failure pattern confirms a toolkit design principle worth documenting: **Extraction design is a separate skill from puzzle design.** Authors who build excellent mechanisms (the Skeptic's continent fingerprinting, the Social's discard pile stratigraphy) still fail at the combinatorial task of making specific letters from specific words spell a target answer. This suggests the toolkit should include an extraction verification tool or checklist.
