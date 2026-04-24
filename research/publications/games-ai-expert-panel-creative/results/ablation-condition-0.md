# Ablation Study — Condition 0: No Profile (Bare Prompt)

**Reviewer persona:** "An expert reviewer. You have broad knowledge of game and puzzle design."
**Date:** 2026-02-28
**Puzzles evaluated:** Age of Empires hunt, Puzzles I–III
**Correct answers (withheld from output):** CASTLE / ONAGER / LOOM

---

## Puzzle I — Dark Age

### Solving Notes

The puzzle presents eight civilization-specific bonuses and asks the solver to identify each civilization, then extract one letter from each civilization name according to a stated position. Eight letters are collected; the puzzle states "not all of them matter" and asks the solver to find the word.

Working through the identifications:
- Bonus A (infantry moves 15% faster, Feudal): Vikings (5 letters) → letter 1 = V
- Bonus B (infantry attacks 33% faster, Feudal): Japanese (8 letters) → letter 2 = A
- Bonus C (Town Center + Dock work rate per Age): Chinese (8 letters) → letter 4 = N
- Bonus D (foot archers +1 range Castle, +1 Imperial): Britons (7 letters) → letter 4 = T
- Bonus E (cavalry archers fire 25% faster): Mongols (7 letters) → letter 6 = L
- Bonus F (villagers carry +5, military 11% faster): a 6-letter civilization; ambiguous without confident identification
- Bonus G (cavalry +20% HP): Franks (6 letters) → letter 1 = F
- Bonus H (trash units cost 25% less): a 10-letter civilization → letter 1 uncertain

The target answer CASTLE (6 letters) should emerge from the eight collected letters. Letters A through E yield V, A, N, T, L — five of the six letters in CASTLE are plausible (C, A, S, T, L, E). The mechanism of "not all letters matter" acts as a filter, but the puzzle provides no explicit instruction for which letters to keep or discard, and no secondary instruction for ordering. A solver who collects eight correct letters and cannot determine which six (or fewer) to use, or in what order, may be stuck.

The puzzle says "Find the word" without explaining the selection or ordering step. This is the primary structural weakness: the final extraction step is underspecified.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 3 | The per-bonus mechanic (identify civ, circle stated letter) is explained clearly. The hint box shows a worked example. However, the final step — "Eight letters. Not all of them matter. Find the word." — provides no guidance on how to identify which letters matter or how to order them. A solver left with 8 letters and no filter rule is likely to stall. |
| Solvability | 2 | Identification of the civilizations requires accurate recall of precise bonus statistics, which is demanding. Two bonuses (A and B) both concern infantry, and the puzzle flags this, but distinguishing them requires knowing the exact speed vs. attack-rate distinction. Bonus F is particularly ambiguous — the combined bonus (carry capacity + production rate) could fit more than one civilization depending on version knowledge. The final assembly step has no stated rule, which risks a solver correctly identifying all eight civilizations and still failing to reach CASTLE. |
| Elegance | 3 | The core idea — use precise game knowledge to identify civs, extract letters — is clean. The "not all letters matter" device is a standard puzzle technique, but here it is left implicit and unexplained, which undercuts the elegance. If the word emerges naturally from reading the letters in order, the puzzle would be more satisfying; as written, the solver does not know whether to anagram, select, or reorder. |
| Reading Reward | 4 | This puzzle strongly favors players who have memorized civ bonuses. A non-player cannot reasonably complete it; there is no path to the answer without genuine game knowledge. The payoff for knowing the bonuses is real. |
| Fun | 3 | The identification step is engaging for a knowledgeable player. The fun diminishes at the extraction stage, where uncertainty about the final rule replaces the satisfaction of correct identification. If the final step resolves cleanly, the puzzle would be more enjoyable; as written, the ending is frustrating rather than rewarding. |
| Confirmation | 2 | The answer CASTLE is meaningful within the game world (it is a major Age), which provides some indirect confirmation. However, because the letter-selection rule is unstated, a solver cannot verify their process — only their output. If they arrive at a different word, they cannot determine whether their civ identifications or their letter selection is at fault. |
| **Total** | **17/30** | |

### Issues Identified

1. **Underspecified final extraction step.** "Not all of them matter. Find the word." does not tell the solver how to identify which letters matter. Standard puzzle practice would either make all letters contribute, or provide a visible filter rule (e.g., highlight certain bonuses, mark letters that participate).
2. **Potential ambiguity in Bonus F.** The combined bonus (villagers carry +5 resources; military units created 11% faster) may not point unambiguously to a single civilization across all players' knowledge, particularly if version differences exist.
3. **No ordering instruction for final assembly.** The letters are collected A→H in order; the puzzle does not confirm whether the answer reads in that order or requires rearrangement.
4. **Two infantry bonuses increase identification difficulty without providing a crosscheck.** The puzzle flags them ("read them closely") but this is a warning, not a disambiguation aid.

### Verdict: FAIL

---

## Puzzle II — Feudal Age

### Solving Notes

The puzzle runs a single-elimination tournament of eight game units decided by counter logic. The puzzle provides the full counter hierarchy explicitly. One match is flagged as an exception — a cavalry unit that resists the damage bonus from its standard counter.

Working through the bracket:
- QF-1: Onager vs Crossbowman → Siege beats archers → Onager wins → letter 1 of ONAGER = O
- QF-2: Pikeman vs Knight → Anti-cavalry infantry beats cavalry → Pikeman wins → letter 7 of PIKEMAN = N
- QF-3: Cataphract vs Halberdier → The flagged exception: Cataphract's armor resists the Halberdier's anti-cavalry bonus → Cataphract wins → letter 2 of CATAPHRACT = A
- QF-4: Mangonel vs Champion → Siege beats infantry → Mangonel wins → letter 4 of MANGONEL = G
- SF-1: Onager vs Pikeman → Siege beats infantry → Onager wins → letter 5 of ONAGER = E
- SF-2: Cataphract vs Mangonel → Cavalry beats siege → Cataphract wins → letter 7 of CATAPHRACT = R

Letters in order (QF-1 through SF-2): O, N, A, G, E, R = ONAGER.

The answer is one of the tournament's own competitors. This resolves completely and correctly. The exception rule (Cataphract defies the spear) is the key insight, and the puzzle signals it clearly without giving it away. The counter logic table is sufficient to resolve every other match without external knowledge beyond knowing what unit type each entry belongs to.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | The rules are stated completely. The counter hierarchy is explicit. The exception is flagged without being solved. The worksheet mirrors the bracket. Letter extraction instructions are unambiguous (position in winner's name). The worked-example note is not needed here because the mechanism is simple enough, but nothing is missing. |
| Solvability | 5 | Every match resolves deterministically from the provided rules. The only judgment call is the Cataphract match, and the puzzle explicitly tells the solver an exception exists — the solver simply needs to know which unit it applies to. That knowledge is accessible to any player familiar with AoE2 unit mechanics. No external lookup is required beyond what the game teaches. |
| Elegance | 5 | The answer is the name of one of the units in the tournament. The extraction produces the answer through the tournament logic itself. The exception match is the pivot: a solver who knows the Cataphract's unusual armor resists the Halberdier's bonus damage gets the aha moment. Everything is self-contained, mechanically consistent, and resolves without ambiguity. |
| Reading Reward | 5 | A non-player cannot solve this. The counter logic is provided, but identifying which unit type each entry belongs to (is Cataphract cavalry? is Champion infantry?) requires game familiarity. The Cataphract exception is only known to experienced players. Game knowledge is genuinely load-bearing. |
| Fun | 5 | The tournament format is inherently engaging. The counterintuitive result in QF-3 creates a genuine aha moment for players who know the Cataphract. The payoff — the answer is one of the competitors — is satisfying. The progression from identifying all winners to reading the final word is clean. |
| Confirmation | 5 | ONAGER is one of the tournament's own participants, which is immediately recognizable as correct. A solver who reaches ONAGER knows they are right without any external check. The answer is self-confirming within the puzzle's own content. |
| **Total** | **30/30** | |

### Issues Identified

None. The puzzle is complete, solvable, unambiguous, and self-confirming.

### Verdict: PASS

---

## Puzzle III — Castle Age

### Solving Notes

The puzzle presents four technology upgrade chains from Age of Empires 2, each with one technology removed. The solver names the missing technology and extracts a letter from it.

Working through the chains:
- Chain 1: [?] → Bodkin Arrow → Bracer. This is the ranged attack upgrade chain. The Feudal-Age first tier is Fletching (9 letters). Letter 2: F-L-E-T-C-H-I-N-G → L
- Chain 2: Forging → [?] → Blast Furnace. This is the melee attack chain. The Castle-Age middle tier is Iron Casting (ignoring space: IRONCASTING, 11 letters). Letter 3: I-R-O-N-C-A-S-T-I-N-G → O
- Chain 3: Double-Bit Axe → [?] → Two-Man Saw. This is the lumber efficiency chain. The Castle-Age middle tier is Bow Saw (ignoring space: BOWSAW, 6 letters). Letter 2: B-O-W-S-A-W → O
- Chain 4: [?] → Architecture. Fortification chain, first tier is Masonry (7 letters). Letter 1: M-A-S-O-N-R-Y → M

Letters in order: L, O, O, M = LOOM.

LOOM is an AoE2 technology (villager HP upgrade available in the Dark Age). The answer is a meaningful in-game term. The solve path is clean and complete.

One potential issue: "ignore spaces when counting" is a necessary clarification, and the puzzle provides it. Without this note, "Iron Casting" could be counted as 4+7=11 letters with a space at position 5, or as a 10-letter string. The note resolves this.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | The chain format (visual sequence with blanks) is easy to read. The instruction to ignore spaces is given. The letter-extraction rule is consistent with previous puzzles. Minor friction: the puzzle says "ignore spaces when counting" but does not confirm whether hyphens in hyphenated names (Double-Bit Axe, Two-Man Saw) are also ignored. Solvers naming these technologies do not extract from them, so this is not a blocking issue — but the rule boundary is slightly fuzzy. |
| Solvability | 4 | All four missing technologies are identifiable from the chain context plus game knowledge. The upgrade chains are canonical AoE2 content known to experienced players. The main risk is solvers who know the technologies but miscount letter positions, particularly in Iron Casting if they second-guess the "ignore spaces" rule. Chain 4 (Masonry) requires knowing the fortification upgrade chain, which is less frequently discussed than combat upgrades — slightly harder but still accessible. |
| Elegance | 4 | Four fill-in-the-blank chains producing a four-letter answer is structurally clean. The answer (LOOM) is itself an AoE2 technology, which gives the solution a satisfying resonance — the solver has been researching technologies and the answer is also a technology. The chains vary by building and by tech tree branch, providing variety. Minor deduction: Chain 4 has only two tiers, which makes it shorter and slightly easier to parse, creating a slight imbalance across the four chains. |
| Reading Reward | 4 | The puzzle requires knowing which technologies exist in each chain — a player who has spent time in the tech tree will recognize Fletching, Iron Casting, Bow Saw, and Masonry. A non-player cannot complete this without looking up the game. The game knowledge is genuine and specific. Slight deduction: an internet search would resolve all four chains quickly, making the barrier lower than Puzzle II where the Cataphract exception requires judgment rather than lookup. |
| Fun | 4 | The chain-completion format produces a consistent, methodical experience. Each chain is its own small solve. The final assembly is clean — four letters, read in order, produce a real word that is itself part of the game. The aha moment is moderate: LOOM is recognizable but not as immediately striking as ONAGER (where the answer is a tournament participant). |
| Confirmation | 4 | LOOM is an AoE2 technology (villager HP and armor upgrade), so a player with game knowledge will recognize it as correct. The answer does not self-confirm from within the puzzle's own content the way ONAGER does, but it confirms readily for any player who has seen the tech tree. |
| **Total** | **24/30** | |

### Issues Identified

1. **Minor ambiguity on hyphen counting.** The "ignore spaces" rule does not address hyphens. In practice this does not affect the answer since hyphens only appear in chain labels, not in missing technologies — but the rule statement is slightly incomplete.
2. **Chain 4 is shorter (2 tiers vs 3).** This creates a mild imbalance in challenge across the four chains. Masonry is easier to identify from a 2-tech chain than Iron Casting is from a 3-tech chain.
3. **Searchability concern.** All four chain gaps are resolvable by a simple internet search of AoE2 tech tree references, which lowers the game-knowledge barrier compared to Puzzle II.

### Verdict: PASS

---

## Condition 0 Summary (No Profile — Bare Prompt)

| Puzzle | Clarity | Solvability | Elegance | Reading Reward | Fun | Confirmation | Total |
|--------|---------|-------------|----------|----------------|-----|--------------|-------|
| I — Dark Age | 3 | 2 | 3 | 4 | 3 | 2 | 17/30 |
| II — Feudal Age | 5 | 5 | 5 | 5 | 5 | 5 | 30/30 |
| III — Castle Age | 4 | 4 | 4 | 4 | 4 | 4 | 24/30 |
| **Avg** | **4.0** | **3.7** | **4.0** | **4.3** | **4.0** | **3.7** | **23.7/30** |

**Pass threshold:** 22/30
**Results:** Puzzle I FAIL (17/30) · Puzzle II PASS (30/30) · Puzzle III PASS (24/30)

---

## Meta-Review Fields

### Named frameworks introduced
None. No named design philosophies, schools of puzzle design, or evaluative frameworks were introduced. All observations use generic functional language (clarity, solvability, ambiguity, confirmation).

### Actionable fixes identified
5 total:

1. **Puzzle I — Specify the final extraction rule.** State explicitly which letters contribute to the answer and in what order, or redesign so all 8 letters are used and no selection step is required.
2. **Puzzle I — Audit Bonus F for uniqueness.** Confirm that the combined bonus (carry +5, production 11% faster) maps unambiguously to exactly one civilization in the target version of the game.
3. **Puzzle I — Confirm letter ordering.** Make explicit whether the answer is read in bonus order (A→H) or requires rearrangement.
4. **Puzzle III — Clarify hyphen rule.** Extend "ignore spaces" to "ignore spaces and hyphens" to close the rule gap, even though it has no practical effect on this puzzle.
5. **Puzzle III — Consider lengthening Chain 4.** Adding a third tier or additional context to the Masonry chain would bring it in line with the other chains' difficulty level.

### Language register
Generic and functional throughout. Observations are expressed in terms of what a solver can or cannot do, what information is present or absent, and whether the answer confirms. No named design principles are cited. No specialist vocabulary from puzzle-hunt culture, game design theory, or educational frameworks appears. The register is that of a careful but non-specialized reader working through the puzzle step by step and noting where the process succeeds or stalls.
