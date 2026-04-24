# Ablation Study — Condition 1: Structured Rubric Only

**Condition**: C1 — Rubric-only reviewer. No persona, no design philosophy, no domain frameworks beyond the six scoring dimensions.
**Reviewer identity**: "Expert reviewer of puzzle and game design."
**Date**: 2026-02-28

---

## Puzzle I — Dark Age · C1 (Rubric Only)

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 3 | The mechanic is explained and the worked example helps, but two issues create friction. First, the instruction "not all of them matter" is opaque — the solver does not know which letters to keep and which to discard, leaving the final extraction step underspecified. Second, letter-position instructions are given per-bonus but the worksheet asks solvers to "find the word," without guidance on ordering or filtering. A solver who correctly identifies all eight civilizations may still be stuck at extraction. |
| Solvability | 3 | Some bonus-to-civilization mappings appear accurate (Britons for foot-archer range, Mongols for cavalry-archer fire rate). Others are suspect: Bonus B specifies "attacks 33% faster" with an 8-letter blank — no standard AoE2 civilization cleanly matches both the mechanic and the letter count. Bonus F pairs villager carry capacity (+5) with 11% faster military production into a 6-letter blank, which does not obviously resolve to a single civilization. If one or two bonuses have factual errors, the extraction chain breaks and the answer cannot be reached. |
| Elegance | 3 | The identification-then-extraction format is a clean puzzle type. The "not all letters matter" mechanism, however, introduces an arbitrary-feeling filter: the solver does not understand why some extracted letters are discarded. Without a visible logic for which letters count, the end step feels like guessing rather than reasoning. The two-sentence clue about infantry bonuses ("Two of these concern infantry. Read them closely.") is a nice pointer but does not resolve the ambiguity at extraction. |
| Reading Reward | 4 | AoE2 knowledge is genuinely load-bearing. The bonuses are specific enough that a non-player cannot look them up without effectively playing the game or consulting a detailed reference. A player with a few hundred hours recognizes the Mongol cavalry-archer speed, the Briton archer range, and the Persian TC/Dock rates from memory. The puzzle rewards that investment. |
| Fun | 3 | The identification phase is satisfying — matching bonuses to civilizations feels like a quiz a player would enjoy. The aha moment is undermined by the opaque final step. If the solver cannot see why certain letters are selected over others, the solve ends with uncertainty rather than a clean click. |
| Confirmation | 2 | The answer CASTLE is a real AoE2 building and a plausible puzzle answer, but the puzzle provides no thematic callback confirming the word is correct. "Find the word" with no frame means the solver cannot distinguish a correct solve from a partial-extraction error. A non-player who guessed an English word from the letters available would not know whether they had the right answer. |
| **Total** | **18/30** | |

### Issues Identified

1. **Underspecified extraction endpoint**: "Eight letters. Not all of them matter. Find the word." does not tell the solver how many letters are relevant or by what rule the irrelevant letters are excluded. This is the most critical clarity failure.
2. **Potential factual errors in bonus specifications**: Bonus B (infantry attacks 33% faster / 8-letter blank) and Bonus F (villagers carry +5, military 11% faster / 6-letter blank) do not resolve cleanly to a single AoE2 civilization with the stated letter count. If either is wrong, the extraction chain is broken.
3. **No confirmation signal**: The puzzle does not give the solver a way to verify the answer is correct. CASTLE is plausible but nothing in the puzzle points back to it thematically once the word is formed.
4. **Duplicate letter-position instruction**: Bonuses C, D, and F all say "circle letter 4," which produces three letters for position 4 in the worksheet. With 8 bonuses all contributing one letter but "not all mattering," the solver has no decision rule.

### Verdict: FAIL

---

## Puzzle II — Feudal Age · C1 (Rubric Only)

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | The tournament bracket format is immediately legible. Counter logic is stated explicitly in a dedicated section. The Cataphract exception is signaled twice (once in the intro, once in the counter-logic section) so the key deviation from expectation is not hidden. The worksheet mirrors the bracket cleanly. One small issue: the bracket diagram uses "Winner [7th]" twice (once in the top half, once in the bottom half) which is a notation conflict — the SF-2 slot showing [7th] is the same label as QF-2, potentially causing a solver to circle the wrong position. |
| Solvability | 5 | The extraction resolves cleanly. Onager beats Crossbowman (siege over archers) → O. Pikeman beats Knight (anti-cavalry over cavalry) → N. Cataphract beats Halberdier (exception, explicitly flagged) → A. Mangonel beats Champion (siege over infantry) → G. Onager beats Pikeman (siege over infantry) → E. Cataphract beats Mangonel (cavalry over siege) → R. Spelling ONAGER exactly. Every matchup follows stated rules or the stated exception. No factual uncertainty. |
| Elegance | 5 | The mechanism is unusually tight. The puzzle teaches the counter system as it goes, the exception is foregrounded rather than hidden, and the extraction letters emerge naturally from the game logic. The tournament format organizes the information without wasted steps. The answer ONAGER is itself a siege unit — a unit in the puzzle — creating a satisfying recursive quality. |
| Reading Reward | 5 | A non-player cannot solve this. The Cataphract's specific resistance to anti-cavalry bonus damage is obscure AoE2 knowledge. Getting QF-3 wrong produces a different letter at positions 2 and 7 that does not spell a word, so the AoE2 knowledge is mechanically necessary, not decorative. |
| Fun | 5 | The aha moment is strong: the solver works through six matchups using genuine game knowledge, the exception creates a satisfying subversion of expectation, and spelling out a unit name that was present in the bracket itself is a clean payoff. This is the best-designed puzzle of the three. |
| Confirmation | 4 | ONAGER is present in the bracket, which gives the solver a strong confirmation signal: the answer is a unit they just used in the puzzle. This is elegant self-confirmation. One point deducted because the puzzle does not explicitly state the answer will be a unit name, so a solver who misspells could believe they have a different answer. |
| **Total** | **28/30** | |

### Issues Identified

1. **Duplicate bracket label**: The QF-2 slot and the SF-2 slot both show "[7th]" in the bracket diagram. This notation ambiguity could cause a solver to re-circle a letter they already extracted, although the worksheet separates these cleanly.
2. **No explicit statement that the answer is a unit name**: The confirmation would be stronger if the puzzle noted that the champion's identity is meaningful. Minor issue.

### Verdict: PASS

---

## Puzzle III — Castle Age · C1 (Rubric Only)

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | The chain format is visually clear. Each chain shows what is present and marks the gap explicitly with [? ? ?]. The instruction to ignore spaces when counting letters is provided where it is needed. The hint "two of these chains live at the same building" is a useful nudge without giving anything away. The worksheet is a direct copy of the needed work. No ambiguity in what the solver is asked to do. |
| Solvability | 5 | All four chains verify correctly. Chain 1: Fletcher (missing) → Bodkin Arrow → Bracer; 2nd letter of FLETCHER = L. Chain 2: Forging → Iron Casting (missing) → Blast Furnace; 3rd letter of IRONCASTING (spaces ignored) = O. Chain 3: Double-Bit Axe → Bow Saw (missing) → Two-Man Saw; 2nd letter of BOWSAW = O. Chain 4: Masonry (missing) → Architecture; 1st letter of MASONRY = M. Spells LOOM. Every answer is verifiable AoE2 technology knowledge with no ambiguity in letter position. |
| Elegance | 5 | Each chain has a consistent internal logic (each tier grants a stated bonus). The missing-step format is a well-established puzzle type that feels natural here because technology trees are a core AoE2 interface. The "ignore spaces" instruction is applied consistently. Four chains yielding four letters yielding a four-letter word is a tight, clean structure with no slack. |
| Reading Reward | 4 | AoE2 knowledge is necessary for Chains 1–3, which require recall of specific technology names in sequence. Chain 4 (Masonry → Architecture) is somewhat easier — a player who knows only the Castle Age building improvements can get Chain 4 from context. A diligent non-player who searched technology trees could solve this, though it would require a detailed source. The knowledge required is real but more accessible than Puzzle II. |
| Fun | 4 | Filling in a broken technology tree is a satisfying exercise for someone who has navigated the AoE2 research queue. The aha moment is moderate rather than strong — the chains do not subvert expectations, they simply require recall. LOOM (a real AoE2 technology, available in the Dark Age) makes a clean answer, but the thematic connection to "Castle Age" puzzle is loose. |
| Confirmation | 4 | LOOM is a specific, real AoE2 technology — a solver who knows AoE2 will recognize it immediately and feel confident. A solver who is less certain of one or two chains can cross-check: if they get LOOM, they can verify all four technologies are real and the chain order is correct. The puzzle title "Castle Age" does not directly confirm LOOM, but the answer's existence in the game is sufficient confirmation. |
| **Total** | **27/30** | |

### Issues Identified

1. **Chain 4 is easier than the others**: Masonry → Architecture is a two-step chain with one missing entry that a solver could get from the chain description alone ("buildings +10% HP and armor"), without knowing the specific technology name. This does not break the puzzle but the difficulty is uneven.
2. **Loose thematic connection between puzzle title and answer**: The puzzle is called "Castle Age" but LOOM is a Dark Age technology. This is not an error, but a player who expects the answer to be a Castle Age element may doubt a correct solve.

### Verdict: PASS

---

## C1 Summary

| Puzzle | Clarity | Solvability | Elegance | Reading Reward | Fun | Confirmation | Total |
|--------|---------|-------------|----------|----------------|-----|--------------|-------|
| I | 3 | 3 | 3 | 4 | 3 | 2 | 18/30 |
| II | 4 | 5 | 5 | 5 | 5 | 4 | 28/30 |
| III | 5 | 5 | 5 | 4 | 4 | 4 | 27/30 |
| Avg | 4.0 | 4.3 | 4.3 | 4.3 | 4.0 | 3.3 | 24.3/30 |

**Named frameworks introduced**: 0
No named frameworks, design philosophies, or domain terminology beyond what the rubric supplied were introduced during scoring.

**Actionable fixes**: 4

1. **Puzzle I — Specify the extraction rule**: Replace "Eight letters. Not all of them matter. Find the word." with an explicit instruction identifying which letters form the answer (e.g., by adding a letter-ordering index or marking which bonuses contribute to the final word).
2. **Puzzle I — Audit bonus-to-civilization accuracy**: Verify Bonus B (infantry attacks 33% faster, 8-letter blank) and Bonus F (villagers carry +5 and military 11% faster, 6-letter blank) against the AoE2 database. Correct any mismatch between stated mechanic and blank length.
3. **Puzzle I — Add a confirmation signal**: Give the solver a way to verify the answer is correct, such as a thematic callback, a note that the answer is a specific game element, or a length constraint stated explicitly.
4. **Puzzle II — Resolve duplicate bracket label**: The "[7th]" label appears on both QF-2 and SF-2 slots in the bracket diagram. Differentiate them visually or rename one to avoid notation conflict.

**Language register**: Generic. Evaluation relied on standard puzzle-design vocabulary (extraction, confirmation, clarity, mechanic) without introducing specialist terminology, named design frameworks, or field-specific concepts. Observations were grounded in the rubric dimensions only.
