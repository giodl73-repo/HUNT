# Ablation Study — Condition 3: Structured Rubric + Full Design Principles

**Reviewer identity:** Expert reviewer of puzzle and game design.
**Condition:** C3 = structured rubric + full design principles (names, quotes, and operational test for each).
**Puzzles reviewed:** I (Dark Age), II (Feudal Age), III (Castle Age)
**Known answers:** I = CASTLE, II = ONAGER, III = LOOM

---

## Puzzle I — Dark Age · C3 (Principles Full)

**Theme:** Identify eight AoE2 civilizations from their unique bonuses; take a circled letter from each name; eight letters produce CASTLE.

### Full Solve Attempt

Working through each bonus against AoE2 game knowledge:

- **Bonus A** (infantry 15% faster, Feudal+, 5 letters): CELTS. C-E-L-T-S. Circle letter 1 = **C**.
- **Bonus B** (infantry attacks 33% faster, 8 letters): JAPANESE. J-A-P-A-N-E-S-E. Circle letter 2 = **A**.
- **Bonus C** (TC + Dock escalating +10/+15/+20%, 8 letters): SARACENS is 8 letters but has a market bonus, not this. ITALIANS (8) do not have this bonus. The best AoE2 match for this escalating TC/Dock work rate bonus is PORTUGUESE (10 letters), but the blank shows 8 underscores. This is an unresolved discrepancy — the letter count does not match the most likely answer. Circle letter 4 = **unknown** (flagged below).
- **Bonus D** (foot archers +1 range Castle, +1 range Imperial, 7 letters): BRITONS. B-R-I-T-O-N-S. Circle letter 4 = **T**.
- **Bonus E** (cavalry archers fire 25% faster, 7 letters): MONGOLS. M-O-N-G-O-L-S. Circle letter 6 = **L**.
- **Bonus F** (villagers carry +5, military 11% faster, 6 letters): MALIANS have both these bonuses (villagers carry +5 as team bonus, military 11% faster as civ bonus) but MALIANS is 7 letters. The blank shows 6 underscores. Discrepancy flagged. Circle letter 4 = **unknown**.
- **Bonus G** (cavalry +20% HP, 6 letters): FRANKS. F-R-A-N-K-S. Circle letter 1 = **F**.
- **Bonus H** (trash units cost 25% less, 10 letters): VIETNAMESE. V-I-E-T-N-A-M-E-S-E. Circle letter 1 = **V**.

Confirmed letters: C (A), A (B), T (D), L (E), F (G), V (H). Unresolved: C and F positions.

To yield CASTLE = C-A-S-T-L-E with "not all letters matter," the extracted sequence must contain these six letters with two discarded. C and A are confirmed. T and L are confirmed. S and E remain — they must come from Bonus C or Bonus F positions, both of which have letter-count discrepancies with the most plausible civ identifications.

**Extraction path: partially verified.** C, A, T, L confirmed. S and E require resolving Bonus C and Bonus F, where blank counts do not match the best-fit civilizations (PORTUGUESE = 10 letters vs 8 blanks; MALIANS = 7 letters vs 6 blanks).

---

### Principle Checks

**The Riven Standard** — FIRES POSITIVELY.
"The puzzle IS what the field does." Identifying civilizations by their actual in-game bonuses is exactly what AoE2 players do when choosing a civ or countering an opponent. A practitioner (experienced AoE2 player) would recognize this immediately as their own domain knowledge. Full credit.

**Solving = Proving Understanding** — FIRES POSITIVELY, with reservation.
Solving requires knowing specific civ bonuses, not just civ names. The solver who finishes this does understand more about civ differentiation than before. However, the "not all letters matter" instruction means some effort (correctly identifying a civ) produces a discarded letter — the learning is real, but the payoff structure is uneven.

**Blame the Player** — FIRES NEGATIVELY on Bonus C and Bonus F.
If the solver puts PORTUGUESE for Bonus C (the correct civ by game knowledge) but the blank has only 8 underscores, the solver cannot write the answer. They will feel confused, not blamed for missing something. The discrepancy creates designer-blame, not player-blame. Same issue with Bonus F: MALIANS is the correct answer but doesn't fit 6 blanks. A solver who knows the game will be stopped by the physical impossibility of writing the correct answer in the provided space.

**No Over-Scaffolding** — FIRES NEUTRALLY.
The worksheet is present but not trivializing. The solver still needs to know the civ bonuses — the worksheet just organizes the extraction. Removing the worksheet would leave a reasonably inferrable task. The scaffolding is appropriate in weight.

**Surprise the Answer** — FIRES POSITIVELY.
CASTLE is not predictable from "identify eight civilizations." The word does not announce itself from the puzzle's topic. The solver who arrives at C-A-S-T-L-E will notice the satisfying Age-of-Empires resonance (the Castle Age is central to AoE2 strategy). Good thematic payoff.

**One Aha, Not Zero, Not Three** — FIRES POSITIVELY.
One aha: "the letter extraction spells a meaningful AoE2 word." The identification step is knowledge recall, not a cascade of insights. Clean single-aha structure.

**Reading Reward** — FIRES POSITIVELY.
A solver who has not played AoE2 cannot solve this. There is no shortcut — each bonus requires genuine knowledge of civ mechanics. The bonuses are precise (percentages, age restrictions, unit categories) and cannot be answered by keyword searches without deep domain expertise. Reading Reward is high.

**No Computation Without Deduction** — FIRES NEUTRALLY.
No computation is required. Pure recall and identification. The principle does not penalize.

**Snyder's Computer Test** — FIRES POSITIVELY (passes the test).
A 10-line script cannot solve this without a complete AoE2 civ database. The bonuses are precise enough that lookup is non-trivial. However, a script with a proper AoE2 data source could solve it mechanically — the puzzle relies entirely on knowledge rather than adding a deduction layer atop the knowledge. This is a design choice worth noting but not a failure.

**Interlock, Not Independence** — FIRES NEGATIVELY (mild).
Each bonus is solved independently. There is no cross-referencing — knowing that Bonus A is CELTS does not help solve Bonus B. The puzzle is a quiz of eight independent lookups. The "not all letters matter" instruction requires the solver to find the word, which adds one layer of interlock at the end, but the identification stage has zero interlock. This is a structural weakness of the civ-bonus-identification format.

**Verify the Last Mile** — FIRES NEGATIVELY (significant).
Cannot trace from solved puzzle to CASTLE without ambiguity. As documented above, Bonus C (8 blanks, best civ answer is 10 letters) and Bonus F (6 blanks, best civ answer is 7 letters) have letter-count mismatches. The reviewer cannot independently verify the extraction produces CASTLE because two of the eight letter slots have unresolved civ identities. This is a material verification failure that must be resolved before the puzzle ships.

---

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | Instructions are clear; the "not all letters matter" instruction is honest about the format. The worked example is helpful. The blank-count discrepancies (Bonus C, F) will cause confusion for solvers who know the game well. |
| Solvability | 3 | High AoE2 knowledge reaches most of the answer. Bonuses A, B, D, E, G, H are cleanly solvable. Bonuses C and F have letter-count mismatches that block a knowledgeable solver — these represent genuine solvability failures, not just reviewer uncertainty. |
| Elegance | 3 | "Not all letters matter" is an inelegant extraction device — it requires the solver to do extra work (anagramming or pattern-matching 8 letters to find a 6-letter word) without a clear rule for which letters are discarded. The mechanism would be cleaner if all 6 identified civs were used in order. |
| Reading Reward | 5 | Maximum. Domain knowledge is fully load-bearing. No keyword shortcut exists. |
| Fun | 3 | The identification tasks are satisfying for civ enthusiasts. The blank-count mismatches deflate the fun for players who know the game well (they'll second-guess correct answers). The "not all letters matter" ending is mildly unsatisfying. |
| Confirmation | 3 | The answer word CASTLE is thematically resonant and self-confirming once found. But the solver cannot confirm individual civ answers because blank counts contradict known civ names. Partial confirmation only. |
| **Total** | **21/30** | |

### Issues Identified

1. **CRITICAL — Bonus C blank count mismatch.** The puzzle shows 8 underscores. The civ with the escalating +10/+15/+20% TC and Dock work rate bonus in AoE2 is most likely PORTUGUESE (10 letters) or SARACENS (8 letters, but Saracens do not have this bonus). If the intended answer is PORTUGUESE, the blank must be 10 underscores. If the intended answer is a different 8-letter civ, that civ and its bonus should be verified against current AoE2 patch data.

2. **CRITICAL — Bonus F blank count mismatch.** The puzzle shows 6 underscores. The civ with "villagers carry +5 resources" and "military units created 11% faster" in AoE2 is MALIANS (7 letters). If MALIANS is correct, the blank must be 7 underscores. If a 6-letter civ has been assigned, the bonus description needs to be verified — these two bonuses should not both belong to a 6-letter civ.

3. **DESIGN — "Not all letters matter" is weakly clued.** The solver is told 8 letters produce a word but not how many letters the word has, what letters are discarded, or by what rule. This creates a guessing step at the end that dilutes the satisfaction of a clean identification mechanism. Consider either using exactly 6 civs (so all letters matter) or adding a rule for which letters to keep (e.g., "take letters in the order they spell a word related to the hunt theme").

4. **DESIGN — No interlock.** All eight bonuses are solved independently. A solver can work in any order with no advantage from cross-referencing. The puzzle is a knowledge quiz with letter extraction, not a puzzle with interlocking deduction.

### Verdict: FAIL (21/30, below threshold of 22)

**Blocking issues:** Bonus C and Bonus F have letter-count mismatches that prevent independent verification of the extraction path. These must be resolved before the puzzle passes.

---

## Puzzle II — Feudal Age · C3 (Principles Full)

**Theme:** Single-elimination tournament of eight AoE2 units decided by counter logic. One exception (Cataphract resists anti-cavalry bonus damage). Six winner names yield ONAGER.

### Full Solve

- **QF-1:** Onager vs Crossbowman. Siege destroys archers. **Onager wins.** ONAGER: O(1)N(2)A(3)G(4)E(5)R(6). Circle 1st = **O**.
- **QF-2:** Pikeman vs Knight. Standard counter: anti-cavalry infantry destroys cavalry. **Pikeman wins.** PIKEMAN: P(1)I(2)K(3)E(4)M(5)A(6)N(7). Circle 7th = **N**.
- **QF-3:** Cataphract vs Halberdier. The puzzle explicitly states one cavalry unit defies its counter. The Cataphract (Byzantine unique unit) has bonus armor vs anti-cavalry that reduces the Halberdier's bonus damage. **Cataphract wins.** CATAPHRACT: C(1)A(2)T(3)A(4)P(5)H(6)R(7)A(8)C(9)T(10). Circle 2nd = **A**.
- **QF-4:** Mangonel vs Champion. Siege destroys infantry. **Mangonel wins.** MANGONEL: M(1)A(2)N(3)G(4)O(5)N(6)E(7)L(8). Circle 4th = **G**.
- **SF-1:** Onager (QF-1) vs Pikeman (QF-2). Siege destroys infantry. **Onager wins.** ONAGER circle 5th = **E**.
- **SF-2:** Cataphract (QF-3) vs Mangonel (QF-4). Cavalry destroys siege. **Cataphract wins.** CATAPHRACT circle 7th = **R**.

Extracted sequence: O, N, A, G, E, R = **ONAGER**. Fully verified, letter by letter.

---

### Principle Checks

**The Riven Standard** — FIRES STRONGLY POSITIVE.
This puzzle IS the AoE2 unit counter system. Every decision in the bracket (which unit wins each match) requires exactly the knowledge that AoE2 players apply when building armies and choosing counters. A veteran player would recognize this as their own strategic thinking made explicit. The Cataphract exception is the deepest piece of knowledge tested — it requires knowing not just the general rule but the specific unit that breaks it. This is the Riven Standard at full strength.

**Solving = Proving Understanding** — FIRES STRONGLY POSITIVE.
After solving, the solver understands the counter system more clearly than before — specifically that (a) siege beats archers and infantry, (b) cavalry beats siege, (c) anti-cavalry infantry beats cavalry, and (d) the Cataphract is the exception to rule (c). The Cataphract exception is the most important learning moment: it tests whether the solver knows AoE2 deeply enough to recall a specific unit's armor mechanic. Solving genuinely proves understanding.

**Blame the Player** — FIRES POSITIVELY.
Every rule is stated clearly in the Counter Logic section. The Cataphract exception is telegraphed explicitly ("One unit in this bracket defies its counter. Its armor was designed to resist the very bonus that should destroy it"). A solver who gets QF-3 wrong and then sees the answer will say "I should have known about the Cataphract's armor" — classic self-blame, not designer-blame.

**No Over-Scaffolding** — FIRES POSITIVELY.
The Counter Logic section states rules, not answers. Removing the Counter Logic section would leave a solvable puzzle for experienced players (they know the rules). The rules are present as a courtesy to solvers who might be rusty on specifics, not as a crutch that does the work. The scaffolding earns its keep.

**Surprise the Answer** — FIRES POSITIVELY.
ONAGER is not predictable from "unit counter tournament." The solver working through the bracket would not anticipate that the Onager — which enters in QF-1 and keeps winning — would also name the final answer. There is a satisfying self-referential quality: the unit that dominates the bracket spells the answer. The solver who notices this will feel genuine delight.

**One Aha, Not Zero, Not Three** — FIRES POSITIVELY.
One aha: "the bracket winners' letters spell a unit name, and it's the unit that won the bracket." The counter logic is rule-following, not insight-generation. The single insight arrives at the end. Clean.

**Reading Reward** — FIRES STRONGLY POSITIVE.
A solver who does not know AoE2 unit counters cannot solve this. The Cataphract exception is the most demanding test — casual players may not know the Cataphract exists, let alone its specific armor property. The puzzle cannot be solved by searching "AoE2 unit counters" because the exception requires knowing specific unit stats. Reading Reward is at maximum.

**No Computation Without Deduction** — FIRES POSITIVELY.
No computation. Pure deductive reasoning from stated rules and game knowledge.

**Snyder's Computer Test** — FIRES POSITIVELY (the puzzle resists).
A 10-line script with a counter-logic ruleset would mis-solve QF-3 (Cataphract vs Halberdier) unless the script also encodes the Cataphract armor exception. The puzzle is explicitly designed around the exception to the general rule — it is resistant to mechanical solution. Strong result.

**Interlock, Not Independence** — FIRES POSITIVELY.
The bracket is inherently interlocked. QF results feed SF results — you cannot solve SF-1 without knowing the QF-1 and QF-2 winners. The tournament structure creates genuine dependency. Knowing that the Onager wins QF-1 is required to evaluate SF-1. Full interlock.

**Verify the Last Mile** — FIRES POSITIVELY.
Verified above: O(ONAGER[1]) + N(PIKEMAN[7]) + A(CATAPHRACT[2]) + G(MANGONEL[4]) + E(ONAGER[5]) + R(CATAPHRACT[7]) = ONAGER. Every letter traces cleanly from winner name to final answer. Perfect last-mile verification.

---

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Instructions are unambiguous. The bracket diagram is clear. The note about using only QF and SF rounds (not the Final) prevents a common extraction mistake. The Counter Logic section is well-organized. |
| Solvability | 5 | Every match has a determinate winner using the stated rules plus AoE2 knowledge. The Cataphract exception is explicitly telegraphed. A knowledgeable solver will reach ONAGER cleanly. |
| Elegance | 5 | The mechanism is deeply elegant: a counter-logic tournament where the winning unit's name also spells the answer. The extraction feels inevitable in retrospect. The Cataphract exception is the correct level of difficulty — it separates casual from experienced players without being obscure. |
| Reading Reward | 5 | Maximum. Every decision requires AoE2 unit knowledge. The Cataphract exception is domain knowledge at its most specific. |
| Fun | 5 | This is a genuinely exciting puzzle. The tournament format creates narrative tension. The Cataphract upset is a satisfying surprise. The self-referential answer (ONAGER wins the Onager) is delightful. |
| Confirmation | 5 | ONAGER is an AoE2 unit the solver just watched win the bracket. When they spell it from the letters, they will feel certain they are right. Maximum self-confirmation. |
| **Total** | **30/30** | |

### Issues Identified

None. This puzzle has no significant issues.

Minor observation: The bracket diagram uses "Winner [7th]" twice — once for QF-2 (PIKEMAN[7]) and once for SF-2 (CATAPHRACT[7]). This is not an error but could momentarily confuse a solver scanning the bracket. The worksheet table makes the letter positions unambiguous, so this is cosmetic only.

### Verdict: PASS (30/30)

---

## Puzzle III — Castle Age · C3 (Principles Full)

**Theme:** Four AoE2 technology chains, each with one missing entry. Identify the missing technology; take a circled letter from each. Four letters spell LOOM.

### Full Solve

- **Chain 1** (ranged attack: [?] → Bodkin Arrow → Bracer): The Feudal-age ranged attack technology in AoE2 is FLETCHING. F-L-E-T-C-H-I-N-G (9 letters, ignoring spaces as instructed — no space in "Fletching"). Circle letter 2 = **L**.
- **Chain 2** (melee attack: Forging → [?] → Blast Furnace): The Castle-age melee attack technology is IRON CASTING. Ignoring the space: I-R-O-N-C-A-S-T-I-N-G (11 characters). Circle letter 3 = **O**.
- **Chain 3** (lumber: Double-Bit Axe → [?] → Two-Man Saw): The Castle-age lumber technology is BOW SAW. Ignoring the space: B-O-W-S-A-W (6 characters). Circle letter 2 = **O**.
- **Chain 4** (fortification: [?] → Architecture): The Castle-age fortification technology that grants buildings +10% HP and armor is MASONRY. M-A-S-O-N-R-Y (7 letters). Circle letter 1 = **M**.

Extracted sequence: L, O, O, M = **LOOM**. Fully verified, letter by letter.

Thematic note: LOOM is itself an AoE2 technology — a Dark Age technology that grants villagers +15 HP and +1/+2 armor, historically one of the most debated researches in the early game. Placing LOOM as the answer to a Castle Age puzzle creates a satisfying cross-age callback.

---

### Principle Checks

**The Riven Standard** — FIRES POSITIVELY.
Technology chains are a core element of AoE2 gameplay. Every player who has opened the technology tree knows these chains. Identifying a missing technology from its neighbors in the chain is precisely how players reason about research order ("I need Bodkin Arrow — did I research Fletching first?"). This is practitioner knowledge made into a puzzle.

**Solving = Proving Understanding** — FIRES POSITIVELY.
After solving, the solver understands four technology chains more precisely than before — specifically the three-tier structure (Feudal/Castle/Imperial) for ranged attack and melee attack, the lumber efficiency chain, and the fortification chain. The solver who completes this knows FLETCHING → BODKIN ARROW → BRACER as a named sequence, not just a vague memory. Solving = learning.

**Blame the Player** — FIRES POSITIVELY.
The chain context (neighbors) provides everything needed to identify the missing technology. A solver who knows AoE2 will recognize the chain structure. A solver who gets a chain wrong will see, upon correction, that the answer was deducible from the neighbors. Self-blame is the natural response.

**No Over-Scaffolding** — FIRES POSITIVELY.
The worksheet is minimal. The clue is the chain structure itself — the solver needs to know the tech, not fill out a complex form. Removing the worksheet leaves the chains themselves, which are the puzzle. Clean.

**Surprise the Answer** — FIRES POSITIVELY.
LOOM is not guessable from "Castle Age technology puzzle." The four-letter answer, drawn from four unrelated chains (ranged attack, melee attack, lumber, fortification), produces a word that has nothing to do with any of those chains — but is a deeply resonant AoE2 technology. The solver will pause, smile, and recognize the elegance of the choice.

**One Aha, Not Zero, Not Three** — FIRES POSITIVELY.
One aha: "the letters from these four different chains spell a completely different AoE2 technology." The identification of each missing tech is recall, not insight. The single insight arrives when LOOM appears. Clean single-aha structure.

**Reading Reward** — FIRES POSITIVELY.
A solver without AoE2 knowledge cannot identify FLETCHING, IRON CASTING, BOW SAW, or MASONRY from their positions in named chains. The puzzle cannot be answered by pattern-matching alone — the solver must know what goes between Forging and Blast Furnace, between Double-Bit Axe and Two-Man Saw. Full domain engagement required.

**No Computation Without Deduction** — FIRES POSITIVELY.
No computation. Pure recall and chain completion.

**Snyder's Computer Test** — FIRES POSITIVELY (resists).
A script would need a complete AoE2 technology tree database to solve this. The chains themselves are the deduction layer — knowing that IRON CASTING goes between Forging and Blast Furnace requires encoded tech-tree knowledge. The puzzle resists algorithmic solution without specialized data.

**Interlock, Not Independence** — FIRES NEGATIVELY (mild, inherent to format).
The four chains are solved independently. Knowing the answer to Chain 1 does not help with Chain 2. This is structurally identical to Puzzle I's weakness: a set of independent identification tasks with letter extraction at the end. The interlock is zero during the identification phase.

However, the hint "Two of these chains live at the same building. That building forges both swords and arrows" adds a small amount of interlock: a solver who identifies the Blacksmith as the shared building knows that chains 1 (ranged) and 2 (melee) are both Blacksmith chains. This does not help identify the missing techs directly, but it provides a verification check — if your two Blacksmith answers are not in those two chains, something is wrong. This is weak interlock, not none.

**Verify the Last Mile** — FIRES POSITIVELY.
Verified above: L(FLETCHING[2]) + O(IRONCASTNG[3]) + O(BOWSAW[2]) + M(MASONRY[1]) = LOOM. Every letter traces cleanly from identified technology to final answer. Clean last-mile verification.

---

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | The chain format is visually clear. The "(ignore spaces when counting)" instruction for letter extraction is precise and necessary. The building hint is a helpful orientation without being a spoiler. |
| Solvability | 5 | All four missing technologies are determinately identifiable from their chain neighbors plus AoE2 knowledge. No ambiguity. The blank-letter counts match the correct technologies (9 for FLETCHING, 11 for IRON CASTING [ignoring spaces], 6 for BOW SAW [ignoring spaces], 7 for MASONRY). |
| Elegance | 5 | Four chains from different domains of the tech tree (combat ranged, combat melee, economy, fortification) each contribute one letter to spell a fifth, thematically unrelated AoE2 technology. The cross-domain assembly is elegant. LOOM as the answer — a humble Dark Age technology hiding in a Castle Age puzzle — is a perfect payoff. |
| Reading Reward | 5 | Maximum. Every chain requires knowing the specific technology name, not just the category. |
| Fun | 5 | The "research queue is broken" framing is evocative. The puzzle creates a satisfying sense of repair and discovery. The LOOM payoff is a genuine delight for AoE2 players — it is both surprising and perfectly right. |
| Confirmation | 4 | LOOM is self-confirming for AoE2 players — they know it immediately. The only minor uncertainty: the "ignore spaces" instruction for IRON CASTING and BOW SAW requires active attention. A solver who miscounts (counting the space as a character) would get wrong letters for Chains 2 and 3 and would not reach LOOM. The instruction is present and clear, but the consequence of ignoring it is a broken extraction. A single-character inconsistency (spaces in two of four tech names) slightly weakens confirmation confidence. |
| **Total** | **29/30** | |

### Issues Identified

1. **MINOR — Space-counting inconsistency.** Two of the four missing technologies contain spaces (IRON CASTING, BOW SAW) while two do not (FLETCHING, MASONRY). The puzzle addresses this with "(ignore spaces when counting)" but the asymmetry creates a small trap. A solver who forgets the instruction mid-solve will miscalculate letters for exactly the chains where spaces matter. Consider: (a) reformatting the technology names as IRON-CASTING and BOW-SAW with hyphens instead of spaces, or (b) using technology choices where none of the four names contain spaces.

2. **OBSERVATION — Independent chains.** The four chains are solved with zero cross-referencing. This is noted under Interlock, but is not a blocking issue — it is a structural feature of the tech-chain format. The "same building" hint provides minimal interlock. If a revision pass is undertaken, consider whether one chain's answer could constrain another (e.g., if solving Chain 2 reveals the building, which changes the candidate space for Chain 1). This would be difficult to engineer without changing the format significantly.

### Verdict: PASS (29/30)

---

## Summary Table

| Puzzle | Clarity | Solvability | Elegance | Reading Reward | Fun | Confirmation | Total | Verdict |
|--------|---------|-------------|----------|----------------|-----|--------------|-------|---------|
| I — Dark Age | 4 | 3 | 3 | 5 | 3 | 3 | 21/30 | **FAIL** |
| II — Feudal Age | 5 | 5 | 5 | 5 | 5 | 5 | 30/30 | **PASS** |
| III — Castle Age | 5 | 5 | 5 | 5 | 5 | 4 | 29/30 | **PASS** |

**Overall: 2 of 3 puzzles pass (threshold ≥ 22).**

---

## Named Frameworks Count

The following named principles were invoked in this review:

| Principle | Fired in Puzzle I | Fired in Puzzle II | Fired in Puzzle III |
|-----------|:-----------------:|:------------------:|:-------------------:|
| The Riven Standard | + | ++ | + |
| Solving = Proving Understanding | +/res | ++ | + |
| Blame the Player | — (C, F) | + | + |
| No Over-Scaffolding | neutral | + | + |
| Surprise the Answer | + | + | + |
| One Aha, Not Zero, Not Three | + | + | + |
| Reading Reward | ++ | ++ | ++ |
| No Computation Without Deduction | neutral | + | + |
| Snyder's Computer Test | neutral | + | + |
| Interlock, Not Independence | — | ++ | mild+ |
| Verify the Last Mile | — (critical) | ++ | + |

**Total principle invocations across all three puzzles: 33**
**Positive firings: 27 | Negative firings: 5 | Neutral: 3**
**Distinct named frameworks invoked: 11 of 11 (all frameworks fired at least once)**

---

## Actionable Fixes Count

**Puzzle I — 4 actionable fixes (2 critical, 2 design):**

1. CRITICAL: Verify and correct the letter count for Bonus C. Confirm the intended civ, count its letters, and update the blank underscores to match exactly.
2. CRITICAL: Verify and correct the letter count for Bonus F. If MALIANS is intended, change 6 blanks to 7. If a different 6-letter civ is intended, document why that civ has both stated bonuses.
3. DESIGN: Replace "not all letters matter" with an extraction that uses exactly the letters needed. Either reduce to 6 civ/bonus pairs (all letters matter) or add a clear rule for which letters to drop.
4. DESIGN: Add at least one interlock between clues — e.g., one bonus description that can only be resolved after another is solved, or a pair of civs where knowing one narrows the other.

**Puzzle II — 0 actionable fixes.**
(1 cosmetic observation: "Winner [7th]" appears twice in the bracket diagram for different positions; the worksheet makes this unambiguous, so no change required.)

**Puzzle III — 1 actionable fix (minor):**

1. MINOR: Resolve the space-counting inconsistency for IRON CASTING and BOW SAW. Either hyphenate (IRON-CASTING, BOW-SAW) so there are no spaces to miscount, or replace one or both technologies with equivalent-chain entries whose names contain no spaces.

**Total actionable fixes: 5 (2 critical, 2 design, 1 minor)**

---

*Condition 3 — Structured rubric + full design principles (names, quotes, and operational tests). All 11 named frameworks applied as concrete tests to each puzzle.*
