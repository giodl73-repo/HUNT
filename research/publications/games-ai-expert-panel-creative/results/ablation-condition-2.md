# Ablation Study — Condition 2: Structured Rubric + Design Principle Names and Quotes Only

**Reviewer persona:** "You are an expert reviewer of puzzle and game design."
**Date:** 2026-02-28
**Puzzles evaluated:** Age of Empires hunt, Puzzles I–III
**Correct answers (withheld from output):** CASTLE / ONAGER / LOOM
**Condition:** C2 — Rubric provided; principles provided as names and one-line quotes only (no explanations, no test cases, no personas)

---

## Puzzle I — Dark Age — C2 (Principles Compact)

### Solving Notes

The puzzle presents eight civilization-specific bonuses and asks the solver to identify the civilization behind each one, then extract one letter from each civilization name at a stated position. Eight letters are collected, and the puzzle concludes: "Eight letters. Not all of them matter. Find the word."

Working through the identifications:
- Bonus A (infantry moves 15% faster, Feudal): Vikings (5 letters) → letter 1 = V
- Bonus B (infantry attacks 33% faster, Feudal): Japanese (8 letters) → letter 2 = A
- Bonus C (Town Center + Dock work rate per Age, staggered): the clue describes an 8-letter civilization; Chinese (7 letters) is the most familiar TC-work-rate civ, but the blank line shows 8 underscores — a possible factual or formatting error. Accepting Chinese with a counting discrepancy → letter 4 = N (if 7-letter) or uncertain
- Bonus D (foot archers +1 range Castle, +1 Imperial): Britons (7 letters) → letter 4 = T
- Bonus E (cavalry archers fire 25% faster): Mongols (7 letters) → letter 6 = L
- Bonus F (villagers carry +5, military 11% faster): a 6-letter civilization; the combined bonus is ambiguous across versions — no single identification is certain
- Bonus G (cavalry +20% HP): Franks (6 letters) → letter 1 = F
- Bonus H (trash units cost 25% less): a 10-letter civilization → letter 1 uncertain without confident identification

Attempting to form CASTLE from the set {V, A, N/?, T, L, ?, F, ?}: the letters C, A, S, T, L, E must come from somewhere in the eight. T and L appear clearly (Bonus D and E). A appears (Bonus B). C, S, and E are not obviously produced by the confident identifications. The puzzle provides no rule for which letters to keep.

The last-mile problem: the solver collects eight letters (some uncertain) and must somehow arrive at a 6-letter word with no stated selection rule. There is no filter, no secondary clue, no positional logic explained. "Find the word" is the entire instruction.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 3 | The per-bonus identification mechanic is clearly explained, and a worked example is provided for letter extraction. The failure point is the final assembly step: "Eight letters. Not all of them matter. Find the word." gives no rule for which letters matter or how to order them. The solver is left with a bag of letters and an unexplained filter. |
| Solvability | 2 | Identification of civilizations requires accurate recall of exact bonus statistics — this is genuinely demanding and appropriate. However, two bonuses are partially ambiguous (F is unclear, C has a letter-count discrepancy), and the final extraction step has no stated mechanism. A solver who correctly identifies all eight civilizations still has no procedure for arriving at the answer. |
| Elegance | 2 | The identification task is clean and the letter-extraction idea is sound. Both are undermined by the "not all of them matter" device, which is unexplained. An unexplained filter is not a puzzle mechanism — it is an instruction gap. The puzzle does the hard work of setting up a clean identification task and then fails to land it. |
| Reading Reward | 4 | A non-player cannot complete this puzzle. Identifying civilizations by precise bonus statistics is genuine deep-knowledge work. Every correct identification is earned. The content is load-bearing throughout the identification step. |
| Fun | 3 | The identification phase is engaging for a knowledgeable player — there is real pleasure in matching a bonus to a civilization. The fun ends at the extraction step, where the absence of a rule replaces satisfaction with frustration. The puzzle sets up a good aha and then doesn't deliver it. |
| Confirmation | 2 | CASTLE is a meaningful in-game term, providing some indirect confirmation if the solver reaches it. However, with no stated selection rule, the solver cannot confirm their process — only their output. If they arrive at a different word, they cannot distinguish an identification error from an extraction error. |
| **Total** | **16/30** | |

### Which principles triggered and how

**Verify the Last Mile** — "The mechanism and the extraction to the answer word are two separate skills." This is the central failure. The identification mechanism works. The extraction to CASTLE does not. These are two separate skills, and the puzzle only teaches one of them. A solver who masters the identification step has no guidance for the extraction step.

**No Over-Scaffolding** — "A worksheet that does all the work is not a puzzle. It's a coloring book." The puzzle sits at the other end of this failure: there is too little scaffolding at exactly the step where scaffolding is needed. The worksheet provides a worked example for letter extraction but nothing for the final assembly. This is not a case of over-scaffolding — it is under-scaffolding at the critical junction.

**Blame the Player** — "Once you understand the solution and look back, you should blame yourself for not finding it." Applied here as a diagnostic: if a solver fails to reach CASTLE, will they blame themselves? No — they will not know what they did wrong because no rule was given. A puzzle that leaves the solver unable to self-diagnose fails this test.

**Interlock, Not Independence** — "Independent lookups with no cross-referencing is a quiz, not a puzzle." The eight bonuses are independent identifications. They do not cross-reference each other. The puzzle provides no mechanism that makes solving bonus A help with bonus B. The only interlock is the final extraction, which is broken.

### Issues identified

1. **Underspecified final extraction step.** "Not all of them matter. Find the word." provides no filter rule. Standard practice would either use all letters or state a visible selection criterion.
2. **Letter-count discrepancy on Bonus C.** The blank line shows 8 underscores, but the most natural identification (Chinese) has 7 letters. Either the civilization is different from the expected answer, or there is a formatting error. This blocks confident extraction of the letter from that position.
3. **Bonus F ambiguity.** The combined bonus (carry +5, military 11% faster) does not map unambiguously to one civilization across all players' version knowledge.
4. **No ordering rule for final assembly.** The puzzle does not confirm whether letters are read A→H in order or require rearrangement.

### Verdict: FAIL

---

## Puzzle II — Feudal Age — C2 (Principles Compact)

### Solving Notes

A single-elimination bracket of eight AoE2 units decided by counter logic. The counter hierarchy is provided explicitly. One exception is flagged: a cavalry unit that resists the damage bonus from its standard counter.

Working through the bracket:
- QF-1: Onager vs Crossbowman → siege destroys archers → Onager wins → letter 1 of ONAGER = O
- QF-2: Pikeman vs Knight → anti-cavalry infantry destroys cavalry → Pikeman wins → letter 7 of PIKEMAN = N
- QF-3: Cataphract vs Halberdier → the flagged exception: Cataphract armor resists anti-cavalry bonus damage → Cataphract wins → letter 2 of CATAPHRACT = A
- QF-4: Mangonel vs Champion → siege destroys infantry → Mangonel wins → letter 4 of MANGONEL = G
- SF-1: Onager vs Pikeman → siege destroys infantry → Onager wins → letter 5 of ONAGER = E
- SF-2: Cataphract vs Mangonel → cavalry destroys siege → Cataphract wins → letter 7 of CATAPHRACT = R

Letters in order (QF-1 through SF-2): O, N, A, G, E, R = ONAGER.

The answer is the name of one of the tournament's own participants. The mechanism is complete, unambiguous, and self-confirming.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Rules are fully stated. Counter hierarchy is explicit. The exception is flagged without being solved. Worksheet mirrors the bracket. Extraction positions are unambiguous. Nothing is missing. |
| Solvability | 5 | Every match resolves deterministically from the provided rules plus basic unit-type knowledge. The only judgment call is QF-3, and the puzzle tells the solver an exception exists — they must supply the specific unit knowledge. That knowledge is accessible to any experienced AoE2 player. |
| Elegance | 5 | The answer is one of the tournament's own competitors. The tournament produces its own answer. The bracket structure, the counter logic, and the exception are all integrated into a single coherent mechanism. Nothing is arbitrary. |
| Reading Reward | 5 | The counter logic table is provided, but knowing which unit type each entry belongs to — and knowing that the Cataphract specifically resists anti-cavalry bonus damage — requires genuine game knowledge. A non-player cannot solve this from the rules alone. |
| Fun | 5 | The tournament format is engaging. The Cataphract exception is the pivot: a solver who knows the unit gets a clean aha. The payoff — the answer is sitting in the bracket — is immediately satisfying. |
| Confirmation | 5 | ONAGER is one of the tournament's own participants. A solver who reaches ONAGER recognizes it without any external check. The answer is self-confirming within the puzzle's own content. |
| **Total** | **30/30** | |

### Which principles triggered and how

**The Riven Standard** — "The puzzle IS what the field does, not an obstacle overlaid on it." The bracket simulates how AoE2 players actually think about unit matchups. Running counter logic on a tournament is not an obstacle placed on top of game knowledge — it is game knowledge. The mechanism and the domain are the same thing.

**Solving = Proving Understanding** — "Solving is showing you understood what the puzzle was about." To solve this puzzle, the solver must demonstrate understanding of: unit types, counter relationships, and the Cataphract's specific resistance property. Reaching ONAGER is proof of that understanding.

**One Aha** — "No ahas is dull, one aha is fun, two ahas is a stretch, three ahas is a slog." There is exactly one aha: the Cataphract defies its counter. Every other match follows the stated rules. The single exception lands cleanly.

**Surprise the Answer** — "The answer word should make the solver pause, not nod." ONAGER is a siege weapon in the tournament. Finding its name spelled out by the winners causes a pause — the answer was competing the whole time.

**Blame the Player** — "Once you understand the solution and look back, you should blame yourself for not finding it." A solver who missed the Cataphract exception and got the wrong answer will immediately understand what they missed. No unfairness, no ambiguity.

**Verify the Last Mile** — "The mechanism and the extraction to the answer word are two separate skills." Both skills are present and both are taught. The mechanism (counter logic, exception) leads to unit names; the extraction (letter positions) leads to ONAGER. No gap between them.

### Issues identified

None. The puzzle is complete, solvable, unambiguous, and self-confirming.

### Verdict: PASS

---

## Puzzle III — Castle Age — C2 (Principles Compact)

### Solving Notes

Four technology upgrade chains from AoE2 are shown, each with one technology removed. The solver names the missing technology and extracts one letter from it.

Working through the chains:
- Chain 1: [?] → Bodkin Arrow → Bracer (ranged attack chain). Missing Feudal-Age first tier: Fletching (9 letters). Letter 2: F-L-E-T-C-H-I-N-G → L
- Chain 2: Forging → [?] → Blast Furnace (melee attack chain). Missing Castle-Age middle tier: Iron Casting (ignoring space: I-R-O-N-C-A-S-T-I-N-G, 11 letters). Letter 3: O
- Chain 3: Double-Bit Axe → [?] → Two-Man Saw (lumber efficiency chain). Missing Castle-Age middle tier: Bow Saw (ignoring space: B-O-W-S-A-W, 6 letters). Letter 2: O
- Chain 4: [?] → Architecture (fortification chain). Missing Castle-Age first tier: Masonry (7 letters). Letter 1: M

Letters in order: L, O, O, M = LOOM.

LOOM is an AoE2 technology (villager HP and armor upgrade, available in the Dark Age). The answer is a meaningful in-game term. The solve path is complete and unambiguous.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | Chain format (visual sequence with blanks) is easy to read. The "ignore spaces when counting" rule is given and necessary. Slight friction: the rule does not address hyphens, though in practice hyphens appear only in chain labels (Double-Bit Axe, Two-Man Saw), not in the missing technologies, so this does not block solving. |
| Solvability | 4 | All four missing technologies are identifiable from chain context plus game knowledge. The ranged attack and melee attack chains are canonical and widely known. The lumber efficiency chain (Bow Saw) is less prominent but still accessible. Chain 4 (Masonry) is the least frequently discussed — it is a correct identification but requires familiarity with the fortification tree. No external lookup strictly required; factual lookup would resolve it. |
| Elegance | 4 | Four fill-in-the-blank chains producing a four-letter answer is structurally clean. The answer LOOM is itself an AoE2 technology, which gives the solution resonance — the solver has been working through the tech tree and the answer is also a tech. Slight imbalance: Chain 4 has only two tiers vs. three for the others, making it shorter and slightly easier. |
| Reading Reward | 4 | Knowing which technologies exist in each chain requires genuine tech-tree familiarity. A non-player cannot complete this without a reference. Slight deduction: the chains are all resolvable by a direct internet search of the AoE2 tech tree, which lowers the knowledge barrier compared to Puzzle II where the Cataphract exception requires judgment rather than lookup. |
| Fun | 4 | Chain-completion is a consistent, methodical experience. Each chain is its own small solve. The final assembly is clean — four letters in order produce a real word that is part of the game. The aha is moderate: LOOM is satisfying but not as striking as ONAGER, where the answer was a tournament participant. |
| Confirmation | 4 | LOOM is an AoE2 technology, recognizable to any player who has used it. It confirms readily for an experienced player. It does not self-confirm from within the puzzle's own content the way ONAGER does — the solver must carry in the knowledge that LOOM is a tech. |
| **Total** | **24/30** | |

### Which principles triggered and how

**The Riven Standard** — "The puzzle IS what the field does, not an obstacle overlaid on it." Completing upgrade chains is a core activity in AoE2 — players make decisions about research order constantly. Identifying missing tiers in a chain is not an arbitrary task; it mirrors in-game reasoning. The puzzle holds up.

**Solving = Proving Understanding** — "Solving is showing you understood what the puzzle was about." To solve this, the solver must know the full ranged attack chain, the full melee attack chain, the lumber efficiency chain, and the fortification chain. Reaching LOOM is proof of that knowledge.

**No Computation Without Deduction** — "Computation is not deduction. Applying a formula is not solving." The "ignore spaces" rule is purely mechanical, but it applies only to the counting step after the identification is complete. The identification itself is deductive. The computation is minor and incidental.

**Snyder's Computer Test** — "Could a computer generate and solve this puzzle? If yes, it's not hand-crafted enough." A computer with access to the AoE2 tech tree database could generate and solve all four chains trivially. The chains are canonical game data. The puzzle relies entirely on the solver not having that database at hand. This is a mild concern — the puzzle is hand-curated, but the content it tests is directly searchable, which weakens it relative to Puzzle II's judgment-based Cataphract exception.

**Verify the Last Mile** — "The mechanism and the extraction to the answer word are two separate skills." Both skills are present. Identifying the missing technology (mechanism) and extracting a letter at a stated position (last mile) are both taught and unambiguous. No gap.

**One Aha** — "No ahas is dull, one aha is fun, two ahas is a stretch, three ahas is a slog." The puzzle has one implicit aha: LOOM is a tech in the same game that the chains came from. This is a quiet, understated confirmation rather than a sharp surprise. It works but sits closer to the lower end of satisfying.

### Issues identified

1. **Snyder's Computer Test weakness.** All four missing technologies are retrievable from any AoE2 tech tree reference. The puzzle requires knowledge, not judgment. This makes it pass the reading-reward test but partially fail the hand-craftedness test.
2. **Chain 4 imbalance.** Two tiers vs. three in the other chains. Masonry is easier to identify from a 2-tech chain than Iron Casting is from a 3-tech chain.
3. **Hyphen rule gap.** "Ignore spaces" does not address hyphens. Harmless in practice for this puzzle, but the rule boundary is incompletely stated.

### Verdict: PASS

---

## C2 Summary

| Puzzle | Clarity | Solvability | Elegance | Reading Reward | Fun | Confirmation | Total |
|--------|---------|-------------|----------|----------------|-----|--------------|-------|
| I — Dark Age | 3 | 2 | 2 | 4 | 3 | 2 | 16/30 |
| II — Feudal Age | 5 | 5 | 5 | 5 | 5 | 5 | 30/30 |
| III — Castle Age | 4 | 4 | 4 | 4 | 4 | 4 | 24/30 |
| **Avg** | **4.0** | **3.7** | **3.7** | **4.3** | **4.0** | **3.7** | **23.3/30** |

**Pass threshold:** 22/30
**Results:** Puzzle I FAIL (16/30) · Puzzle II PASS (30/30) · Puzzle III PASS (24/30)

---

**Named frameworks introduced:** 10 — Verify the Last Mile (Puzzles I and III), No Over-Scaffolding (Puzzle I), Blame the Player (Puzzles I and II), Interlock Not Independence (Puzzle I), The Riven Standard (Puzzles II and III), Solving = Proving Understanding (Puzzles II and III), One Aha (Puzzles II and III), Surprise the Answer (Puzzle II), No Computation Without Deduction (Puzzle III), Snyder's Computer Test (Puzzle III)

**Actionable fixes:** 6

1. **Puzzle I — Specify the final extraction rule.** State which of the eight letters contribute to the answer and in what order, or redesign so all eight letters are used with no selection step. Leaving the solver with an unexplained filter violates Verify the Last Mile — the extraction to CASTLE is a separate skill from identification, and it is currently unteachable.
2. **Puzzle I — Resolve the Bonus C letter count.** The blank line shows 8 underscores but the expected civilization (Chinese) has 7 letters. Confirm the correct identification, fix the blank count, and recheck the extracted letter.
3. **Puzzle I — Audit Bonus F for uniqueness.** Confirm that the combined bonus (carry +5, military 11% faster) maps unambiguously to exactly one civilization in the target game version.
4. **Puzzle I — Add an ordering rule.** Make explicit whether the answer is read in bonus order (A→H) or requires rearrangement.
5. **Puzzle III — Consider strengthening Chain 4.** Adding a third tier or requiring identification from a more demanding context would bring Chain 4 in line with the challenge level of Chains 1–3 and partially address the Snyder's Computer Test concern.
6. **Puzzle III — Clarify hyphen rule.** Extend "ignore spaces when counting" to "ignore spaces and hyphens" for completeness, even though it has no practical effect on the current answer.

**Language register:** Named-principles throughout. Every substantive observation is anchored to a named principle and its one-line quote. Generic functional language is used only to describe the solving sequence; evaluative judgments are expressed entirely through the principle vocabulary. Specialist terminology from puzzle-hunt design culture appears where directly supplied by the principle names (aha, last mile, confirmation, extraction). No general game-design theory or educational frameworks introduced beyond the supplied set.
