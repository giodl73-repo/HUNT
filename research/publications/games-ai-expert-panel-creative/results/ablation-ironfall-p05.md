# Ablation Study — P05 Battle Damage Calculator (Ironfall)

**Puzzle:** P05 — Battle Damage Calculator v2
**Author:** The Skeptic
**Hunt:** IRONFALL — fictional 16-bit JRPG hunt
**Answer:** ANVIL (verified)
**Date:** 2026-02-28
**Special focus:** "No Computation Without Deduction" — whether formula-inference structure rescues or fails this principle
**Reviewers (C4–C7):** Dan Katz · Thomas Snyder · Dana Young (one per condition, rotating)

---

## Puzzle Summary

BattleMath_99, a fictional 1990s forum researcher, logged controlled battle data for a game whose damage formula was never documented. The puzzle provides:

- **4 training battles** (all values known: ATK, Level, DEF, Damage) — solver derives the formula
- **5 test battles** (ATK, Level, Damage known; DEF unknown) — solver back-solves for DEF values
- DEF values map via A=1…Z=26 to ANVIL

**Derived formula:** `Physical Damage = (ATK × 2 - DEF) × (1 + Level/50)`

All test battles use Level 50, so multiplier = 2.0 throughout the test set. Rearranged: `DEF = ATK × 2 - Damage / 2`.

**Extraction:**

| Log | ATK | Damage | DEF | Letter |
|-----|-----|--------|-----|--------|
| 5 | 11 | 42 | 1 | A |
| 6 | 22 | 60 | 14 | N |
| 7 | 20 | 36 | 22 | V |
| 8 | 15 | 42 | 9 | I |
| 9 | 16 | 40 | 12 | L |

**Answer:** ANVIL

---

## Condition C0 — Bare Baseline (No Profile, No Principles, No Rubric)

**Prompt context:** "You are an expert reviewer. You have broad knowledge of game and puzzle design."

### Solving Notes

The puzzle asks the solver to infer a damage formula from four battle logs and then back-calculate five unknown DEF values. Training logs 1–3 share Level 50; log 4 uses Level 25.

Tracing the solve path:

- Comparing logs 1 and 2 (same ATK=65, different DEF): DEF changes by 43, damage changes by 86. Rate of 2 damage per 1 DEF is visible.
- Testing `(ATK × 2 - DEF) × 2` against logs 1–3 confirms all three at Level 50.
- Log 4 (Level 25) produces 168 with that formula, but actual damage is 126. Mismatch forces a multiplier revision.
- Testing multiplier = (1 + Level/50): Level 50 → 2.0 (unchanged), Level 25 → 1.5. Log 4: 84 × 1.5 = 126. Confirmed.
- Formula established. All five test DEF values compute cleanly. DEF=1 is slightly unexpected (below the game's stated 5–60 range per the author's note) but the math is unambiguous.
- A1Z26 step is explicit in the puzzle text.

The mechanical chain is: observe → hypothesize → test → revise → confirm → back-calculate × 5 → read letters. All steps are prompted and scaffolded by the puzzle text itself.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Training data and test data are clearly separated. The A1Z26 instruction is explicit ("convert each DEF value to a letter using the standard numbering (A=1, B=2, ... Z=26)"). Format — battle logs, clean table structure — is immediately readable. No ambiguity about what to do at any step. |
| Solvability | 4 | Formula derivation is achievable but requires methodical work: two comparison pairs to isolate variables, then a cross-level check to catch the level multiplier. A solver who only uses logs 1–3 will miss the multiplier and get wrong DEF values. Log 4 acts as the necessary curveball. One point off because the level-multiplier insight is genuinely demanding — a solver might confidently fit logs 1–3 and not think to check log 4 until stuck. |
| Elegance | 4 | The formula has real internal structure: two steps of insight (the ×2 on ATK/DEF, then the level multiplier) producing a clean back-calculation. The fact that all test battles use Level 50 eliminates a confounding variable at the extraction stage, which is considerate design. Minor tension: the solve involves significant arithmetic (14 separate calculations across training and test phases), which is a lot of number-crunching even for a mathematically inclined solver. |
| Reading Reward | 5 | A non-player cannot solve this. The flavor — BattleMath_99's forum research, "Morimoto planted it" — is coherent with the fictional archive world. The reward for having played a game with this kind of battle system (and knowing how to read it) is genuine. The puzzle assumes the player treats battle data as parseable evidence, which is a real skill for JRPG players. |
| Fun | 4 | The formula-discovery step is genuinely satisfying — finding the level multiplier from log 4 is a real aha moment. The back-calculation is methodical rather than exciting, but it's purposeful and short (5 battles). The BattleMath_99 voice is charming. The payoff — ANVIL, a solid medieval weapon — is a decent answer word for a 16-bit RPG world. |
| Confirmation | 4 | ANVIL is a real word with clear RPG connotations (crafting, forging). Each letter traces to a computed DEF value. A solver can verify every step independently. Minor deduction: DEF=1 for the first letter (A) may cause a solver to double-check their arithmetic because 1 is below the game's stated stat range — a moment of doubt that resolves but costs time. |
| **Total** | **26/30** | **PASS** |

### Issues Identified

1. **Log 4 is load-bearing but not signaled.** The level multiplier only appears in log 4 (Level 25). A solver who treats logs 1–3 as sufficient will derive a broken formula. The puzzle provides no indication that log 4 carries special importance beyond being part of the training set.
2. **DEF=1 is below the game's stated range.** The author notes this is due to a modded encounter. A solver unfamiliar with this note (or who hasn't read the author's notes section, which is post-solution) may doubt correct arithmetic.
3. **Arithmetic volume.** 14 calculations across the full solve. No partial answers are confirmable until the full extraction step. A computational error early propagates silently.

### Verdict: PASS (26/30)

---

## Condition C1 — Rubric Only

**Prompt context:** Bare + 6-dimension rubric (Clarity, Solvability, Elegance, Reading Reward, Fun, Confirmation — 1-5 each, /30, pass ≥ 22).

### Solving Notes

The rubric's six dimensions immediately surface structural questions:

- **Clarity:** The rubric asks whether the solver knows what to do at each step. The puzzle has three distinct phases (observe training data, derive formula, back-calculate), and each is cued by a text prompt. "Find the word" — the extraction announcement — is embedded in BattleMath_99's voice ("convert each DEF value to a letter"). No ambiguity in the instruction chain.
- **Solvability:** The rubric asks whether a qualified solver can complete this. The gate is the level multiplier — log 4 is necessary, not optional. Solvability is conditional on the solver exhausting all training data.
- **Elegance:** The rubric prompts checking whether the mechanic is necessary and clean. The level multiplier adds a variable that isn't exercised in the test phase (all Level 50) — it's required to find the formula but then disappears. This creates a slight feeling of setup-without-payoff on the level variable.
- **Reading Reward:** Clean — game knowledge drives the solve.
- **Fun:** The discovery arc is good; the arithmetic tail is tolerable.
- **Confirmation:** Each answer letter is independently verifiable via the formula.

The rubric's Elegance dimension surfaces the key tension: the level multiplier is both the puzzle's best insight and its most under-utilized element (it's only exercised once in training data, then frozen at 2.0 for all extraction).

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Three-phase structure is well-labeled. A1Z26 instruction is explicit. No missing steps. |
| Solvability | 4 | Dependent on using all four training logs. Log 4 is necessary but not flagged as special. The formula derivation is methodical and achievable; the back-calculation is mechanical once the formula is known. |
| Elegance | 3 | The level multiplier is the puzzle's most interesting structural feature, but the test set eliminates it by fixing all battles at Level 50. The level variable is introduced, demonstrated once (log 4), and then benched. A cleaner design would either exercise the level multiplier in the test set or remove it as a training variable. |
| Reading Reward | 5 | Genuine game-knowledge requirement. JRPG battle-system literacy is the key. |
| Fun | 4 | Formula discovery is satisfying. Back-calculation is arithmetic work with a clear endpoint. The fictional researcher framing adds warmth. |
| Confirmation | 4 | Letter-by-letter verification available. DEF=1 edge case may cause momentary doubt. |
| **Total** | **25/30** | **PASS** |

### Issues Identified

1. **Elegance gap on level multiplier.** The level variable is introduced and then not exercised in the extraction phase. The solver learns it, proves it, and then does all five extractions without it. Either use it (include a non-Level-50 test battle) or simplify to a level-independent formula.
2. **Log 4 is unmarked as the key to the multiplier.** No signal that it is structurally distinct from logs 1–3.
3. **Arithmetic volume relative to insight density.** Five back-calculations with the same formula is repetitive. The aha is one insight; the extraction is five applications of the same rearrangement.

### Verdict: PASS (25/30)

---

## Condition C2 — Principles Compact (Names + Quotes, No Tests)

**Prompt context:** Bare + rubric + 11 principle names and one-line descriptions.

Principles listed: The Riven Standard / Solving = Proving Understanding / Blame the Player / No Over-Scaffolding / Surprise the Answer / One Aha / Reading Reward / No Computation Without Deduction / Snyder's Computer Test / Interlock, Not Independence / Verify the Last Mile.

### Solving Notes

Principles immediately bring new pressure points into focus:

**No Computation Without Deduction** fires directly. The puzzle involves formula inference (deduction) followed by back-calculation (computation). The question is whether the deduction is rich enough to justify the computational tail.

**One Aha** creates tension: the puzzle has two sequential insights (the ×2 rate from comparing logs 1-2, then the level multiplier from log 4). Is that one aha or two? The author calls it "one aha (finding the formula)" — but finding the formula requires two distinct realizations.

**Surprise the Answer** passes: ANVIL is not guessable from "Battle Damage Calculator."

**Verify the Last Mile** fires against the DEF=1 edge case: tracing A (DEF=1) requires confirming that the formula indeed produces DEF=1 for Log 5.

**Snyder's Computer Test** applies: a 10-line script can be written to check all training data against the formula AND compute all test DEF values. This means the puzzle is fully automatable — the "insight" is expressible as arithmetic, not as a deductive judgment that resists scripting.

**No Over-Scaffolding** is mostly clean — the puzzle provides data tables and log format, not answer-extraction instructions. The author's notes (post-solution) explain the design choices, which is appropriate scaffolding.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Principle: Blame the Player. Every step is fair in retrospect. The A1Z26 instruction is unambiguous. Passes. |
| Solvability | 4 | No Computation Without Deduction: deduction exists but the ratio of deduction to computation is low. Two insights produce five identical back-calculations. |
| Elegance | 3 | One Aha: the puzzle has a two-step formula discovery (rate from pair comparison, then level multiplier from cross-level comparison). Whether this counts as "one aha" is arguable. Snyder's Computer Test: a 10-line script handles this completely, which suggests the puzzle may be more procedure than puzzle. |
| Reading Reward | 5 | Riven Standard: the puzzle IS the thing a JRPG battle-system researcher would do. The mechanic is the domain activity. Passes cleanly. |
| Fun | 3 | Interlock, Not Independence: the five back-calculations are independent of each other — each is the same formula applied to different numbers. There is no interlocking between test battles. Order doesn't matter; no answer checks another. The extraction is a sequence of parallel operations, not a web of dependencies. |
| Confirmation | 4 | Verify the Last Mile: DEF=1 (letter A) is the weakest point. A solver confident in the formula may briefly doubt it because DEF=1 is sub-range. The formula traces correctly. |
| **Total** | **24/30** | **PASS** |

### Issues Identified

1. **No Computation Without Deduction tension.** The deduction is front-loaded (formula inference); the computation is back-loaded (five identical back-calculations). The principle is technically satisfied but the ratio is poor — one insight does all the intellectual work, then the solver becomes a calculator for five steps.
2. **Two-step aha vs. One Aha.** The formula requires two separate observations to derive. Whether this is "the formula insight" (one aha) or two sequential insights depends on how the solver experiences the process.
3. **Snyder's Computer Test fails.** A script can derive the formula from training data and compute all test answers in under 10 lines. This is the principle's warning sign.
4. **Interlock, Not Independence gap.** The five test calculations are independent parallel operations. No test battle informs another. A solver who makes an error in one DEF value cannot catch it from adjacent answers.

### Verdict: PASS (24/30)

---

## Condition C3 — Principles Full (Names + Quotes + Tests)

**Prompt context:** Bare + rubric + 11 principles with full test language.

**Reviewer:** Thomas Snyder (rotating assignment)
*3x World Sudoku Champion; Grandmaster Puzzles; co-invented Just One Cell Sudoku; Puzzlecraft co-author*

Principles with tests applied:

- **No Computation Without Deduction** — Test: Remove the computation. Is there still a puzzle? Here: remove the back-calculation step. What remains? The formula. But the formula alone doesn't give you an answer. The back-calculation IS the extraction. So the computation cannot be removed. This is a puzzle where computation is the vehicle for extraction, not the reward.
- **One Aha** — Test: Name the single aha. "The formula has a level multiplier" is the aha. The pair comparison (logs 1–2) is setup, not aha. But "the formula is (ATK×2 - DEF) × (1 + Level/50)" is a formula with two structural features — is discovering each feature one aha or two? Answer: the level multiplier is the aha (the pair comparison is mechanical pattern matching; the cross-level correction requires genuine conceptual work).
- **Snyder's Computer Test** — Test: Write the 10-line script. Python: read four training logs, fit a two-parameter formula, validate, then invert for five test logs. This runs. The puzzle is automatable.
- **Verify the Last Mile** — Test: Trace letter by letter. Log 5: ATK=11, Damage=42, Level=50. DEF = 11×2 - 42/2 = 22 - 21 = 1. A=1. Verified. Log 6: ATK=22, Damage=60. DEF = 44 - 30 = 14. N. Log 7: DEF = 40 - 18 = 22. V. Log 8: DEF = 30 - 21 = 9. I. Log 9: DEF = 32 - 20 = 12. L. ANVIL. Confirmed clean.
- **Interlock, Not Independence** — Test: Any order, no advantage? The test battles are completely independent. Solving log 6 before log 5 changes nothing. No interlocking structure. This is a parallel computation, not an interlocked deduction.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Blame the Player test: every instruction is fair in retrospect. A1Z26 is explicit. Phase separation is clear. Passes. |
| Solvability | 4 | Solving = Proving Understanding test: does the solver know more about JRPG damage systems after solving? Yes — they have derived a real formula from data. The solver gains genuine understanding of the domain mechanic. Good. Deduction of the level multiplier requires cross-log comparison that is non-trivial. |
| Elegance | 2 | Snyder's Computer Test fires. The puzzle is fully automatable. The Just One Cell standard (can any approach other than the intended one reach the answer?) reveals that arithmetic is the only path — no logical deduction resists scripting. No Computation Without Deduction: the computation is load-bearing at extraction. Interlock gap compresses the elegance further: five independent back-calculations is a sequence, not a puzzle architecture. |
| Reading Reward | 5 | Riven Standard test: does the mechanic match the domain? Yes — running controlled battle tests, logging data, reverse-engineering a game's hidden formula is exactly what this type of player would do. The puzzle IS the domain activity. Passes cleanly. |
| Fun | 3 | One Aha test: the level multiplier is the aha. The formula as a whole is the payoff. But the five back-calculations are not fun — they are work. The ratio of insight (1 aha) to work (5 applications of the same rearrangement) is unfavorable. |
| Confirmation | 4 | Verify the Last Mile test: traces cleanly letter by letter. DEF=1 is the only wobble — sub-range stat that the author explains only in post-solution notes. A solver mid-puzzle has no access to this explanation. |
| **Total** | **23/30** | **PASS** |

### Issues Identified (Snyder lens applied)

1. **Computer Test failure.** The full solve is scriptable in 10 lines. No step requires a deduction that resists automation. This is the diagnostic Snyder uses to identify "procedure masquerading as puzzle."
2. **No Computation Without Deduction ratio is poor.** One real deduction (level multiplier discovery) precedes five identical arithmetic operations. The computational mass overwhelms the deductive nucleus.
3. **Interlock, Not Independence fails.** The five test battles are independent parallel calculations. A solver who happens to solve them out of order (or makes an error in one) gets no cross-verification from adjacent answers. The answer letters do not police each other.
4. **DEF=1 explanation is in post-solution notes only.** Mid-solve, a solver encountering DEF=1 with no in-puzzle explanation has to trust the math against their game-knowledge intuition. This is an underdisclosed edge case.
5. **Level variable introduced and benched.** The level multiplier is demonstrated once (log 4) and then eliminated from the test set. The solver masters a concept that the extraction phase doesn't use.

### Verdict: PASS (23/30) — weak pass; Snyder notes it is close to a design-level rejection

---

## Condition C4 — Philosophy Only (Worldview, No Lens Checklist)

**Reviewer:** Dan Katz — philosophy and worldview only
*From profile: "Dan Katz thinks about puzzle hunts as contracts. Every structural decision a designer makes implicitly promises something to solvers about the experience they're signing up for." "His core conviction is architectural: a puzzle hunt is one thing, not a collection of puzzles." "A puzzle that could be removed without affecting the experience should be removed."*

### Solving Notes

Katz reads from a contract lens. What does Battle Damage Calculator promise?

The puzzle promises: you will be a reverse-engineer. You will work from data to formula. This is a specific epistemic contract — empirical reasoning about a hidden system. The flavor text (BattleMath_99's three years of research, the forum thread framing, the "Morimoto planted it" line) reinforces this contract compellingly.

Does the puzzle deliver? Partially. The formula derivation step delivers on the empirical-reasoning promise. But the extraction phase (five arithmetic back-calculations) converts the empirical-reasoning contract into an accounting task. A solver who signed up for "figure out how this game works" ends up doing five instances of "plug into formula and divide."

From a hunt architecture perspective: does this puzzle justify its slot in Act I? The formula-inference hook distinguishes it from neighboring puzzles. The JRPG battle-system domain is coherent with the hunt's Ironfall setting. A puzzle that could be removed without affecting the experience? No — the battle-system domain is load-bearing for the hunt's identity, and the formula inference is a genuine capability test.

The hint-economy question: what do solvers do when stuck? A stuck solver at the formula-inference stage needs a nudge toward "try log 4" — that's a reasonable hint. A stuck solver at the back-calculation stage has no excuse: the formula is known, the arithmetic is direct. This is a good hint-economy structure: hard step (formula) is hintable, easy step (arithmetic) is unhintable-by-design.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Contract is delivered clearly. Phases are labeled. Instructions are unambiguous. The solver knows what they've been asked to do. |
| Solvability | 4 | The formula-inference step is the genuine difficulty. The level multiplier requires cross-log comparison that a solver who stops at logs 1–3 will miss. This is a contractual warning: the puzzle implies all four logs are needed, but doesn't say so explicitly. A careful solver uses all four; a hasty one might not. |
| Elegance | 3 | The contract is rich at the formula-inference stage and thin at the extraction stage. Katz asks: does this puzzle justify its slot? Yes — but it's a stronger puzzle for its first half than its second. The five back-calculations are not puzzle content; they are the consequence of puzzle content. A design that collapsed the extraction to 3 battles would be tighter. |
| Reading Reward | 5 | The puzzle IS what an advanced JRPG player would do. The empirical-reasoning contract is authentic. Genuine domain knowledge required. |
| Fun | 4 | The formula discovery produces real satisfaction. BattleMath_99's voice is the hunt's best flavor character so far — the dedicated amateur researcher with three years of frame-counted data is an instantly legible archetype. The arithmetic back-end is tolerable because the formula is the prize. |
| Confirmation | 4 | ANVIL lands as a satisfying answer in an RPG world (blacksmithing, crafting). The letter-trace is verifiable. DEF=1 is the one contractual wobble: it implies an edge case the puzzle doesn't explain until after the solution. |
| **Total** | **25/30** | **PASS** |

### Issues Identified (Katz philosophy applied)

1. **Contract delivery gap at extraction phase.** The empirical-reasoning contract is compelling at the formula-inference stage. The extraction phase (five arithmetic back-calculations) is an accounting task that weakens the contract's payoff. The puzzle over-promises on the intellectual engagement and under-delivers at the arithmetic stage.
2. **Log 4 is structurally critical but unmarked.** From a contract standpoint: the puzzle implicitly promises that all training data is meaningful. Log 4 is, in fact, the most meaningful piece — but the solver doesn't know this. This is technically fair (use all the data) but would be stronger if log 4 had a subtle visual or textual marker that invited closer inspection.
3. **Five back-calculations is one too many for the insight-to-work ratio.** The formula discovery is one aha. Five identical calculations feel like the designer needed five letters in ANVIL and filled the word. (They did, and it's fine, but the arithmetic mass is noticeable.)

### Verdict: PASS (25/30)

---

## Condition C5 — Lens Only (Checklist, No Biography or Philosophy)

**Reviewer:** Thomas Snyder — lens checklist only, no biography or worldview context
*Checklist applied cold: Is the entry point constructed or discovered? Does the solve path hold one direction? Is each element load-bearing? Is there a puzzle here, or a procedure?*

### Lens Checks Applied

- **Is the entry point constructed or discovered?** The entry point is the pair comparison (logs 1 and 2, same ATK, different DEF and damage). This is discoverable but not explicitly constructed — there's no visual or structural cue that tells the solver "start here." A solver who tries to derive the formula from log 1 alone cannot. The entry point requires comparing two logs, and the logs are presented in sequence without a "compare these" marker.
- **Does the solve path hold one direction?** No branching — the formula inference has a linear path. But the five back-calculations are parallel operations, not a directed chain.
- **Is each element load-bearing?** Log 4 is essential (the only Level-25 log). Logs 1 and 2 establish the rate. Log 3 provides cross-validation. Each training log is load-bearing. The five test logs are load-bearing only as letter-extractors — removing any one of them would change the answer, but not the puzzle's structure.
- **Is the difficulty technique-based or noise-based?** The difficulty is technique-based: formula inference is a genuine skill. No irrelevant data is present. The training data is minimal and sufficient.
- **Can the solver confirm each step independently?** The formula can be confirmed against all four training logs (and the solution section provides this). Each back-calculation is independently verifiable. Yes — checkpoint structure is good.
- **Is there a puzzle here, or a procedure?** This is the critical lens question. The formula inference is a puzzle (non-obvious structure, requires cross-log comparison, genuine deductive work). The back-calculation is a procedure (apply formula, read result). The puzzle has a puzzle followed by a procedure. The procedure is not trivial, but it is not where the puzzle lives.
- **Would he want to have constructed this?** The formula design is elegant (two-variable formula with a multiplicative level term). The extraction design (five parallel calculations → A1Z26) is functional but architecturally flat. A constructor Snyder would admire built the training set carefully; the test set is adequate but not crafted at the same level.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | The entry point exists but is not explicitly flagged. Phase structure is clear. One point off because "start by comparing logs 1 and 2" is not cued — a solver could attempt log-by-log analysis and stall. |
| Solvability | 3 | The entry point is discoverable but requires the solver to recognize that pair comparison is the right move. A solver who tries to work from a single log first will fail and need to reframe. The level multiplier requires a second cross-log comparison (logs 1-3 vs. log 4). Two undirected entry points is two possible stall moments. |
| Elegance | 2 | The puzzle here, or a procedure? lens fires hard. Formula inference = puzzle. Back-calculation × 5 = procedure. The elegant design would integrate the deduction into the extraction — perhaps some test battles require non-Level-50 work to force continued engagement with the level multiplier. As designed, the level variable is discovered and immediately shelved. |
| Reading Reward | 5 | Domain knowledge requirement is genuine. Passes without qualification. |
| Fun | 3 | The formula discovery is genuinely satisfying. The back-calculation is not fun — it is work the solver already knows how to do from the moment they have the formula. No interlocking structure provides variety. |
| Confirmation | 4 | Verify-each-step works. DEF=1 is a wobble. |
| **Total** | **21/30** | **FAIL** |

### Issues Identified (Lens checklist applied cold)

1. **Entry point is undirected.** No structural cue tells the solver to compare logs 1 and 2 first. A solver attempting to derive the formula from a single log will fail immediately but with no diagnostic signal about why.
2. **Two-stage entry problem.** Both the ATK/DEF rate discovery AND the level multiplier require cross-log comparison that the solver must independently recognize as the correct move.
3. **Puzzle → Procedure transition kills the solve arc.** The most interesting part of the puzzle ends at formula confirmation. Everything after is arithmetic. The ratio of puzzle-work to procedure-work is approximately 1:5 (one formula derivation, five identical applications).
4. **Level variable is introduced and eliminated.** All test battles are Level 50, making the level multiplier a training artifact that never transfers to the extraction. A puzzle with better interlock would require the solver to apply the full formula (including level multiplier) at extraction.
5. **No cross-verification between test battles.** Independent parallel calculations — an error in one produces a wrong letter with no signal.

### Verdict: FAIL (21/30)

**Note:** C5 is the most critical verdict of any condition. The lens checklist, applied without biographical framing, issues a FAIL where all other conditions (so far) have passed. The puzzle-vs-procedure distinction fires hard against the extraction phase.

---

## Condition C6 — Full Profile (Current System v2)

**Reviewer:** Dana Young — full profile (identity + philosophy + lens)
*25-year Microsoft Puzzlehunt veteran; Placement Test round; "the puzzle IS the world, not a game placed inside one"; visual-first design; "the answer names what you were doing the whole time"*

### Solving Notes

Dana reads this puzzle as a world-first designer. What world does Battle Damage Calculator construct?

The world is BattleMath_99's archive — a forum thread from 1998, data logged frame-by-frame, the puzzle of an undocumented formula. The solver inhabits a specific role: the person who finds BattleMath_99's thread, reads the data, and finishes the work he couldn't explain.

**Does the world hold throughout?**

- The training logs section: yes, firmly in the world. The battle log format is authentic JRPG interface language. The researcher's voice ("hex-verified") is specific and charming.
- The formula derivation: yes. The solver is doing exactly what BattleMath_99 did.
- The test logs section: yes — "I ran five more battles against unknown enemies." The premise is coherent.
- The A1Z26 instruction: partial wobble. "Convert each DEF value to a letter using the standard numbering" is puzzle-extraction language, not forum-thread language. BattleMath_99 would not have written this in a 1998 forum post.
- "The letters spell a word. That word was not in my notes. I think Morimoto planted it." — this is world-building at the extraction step, which is the right instinct. The "Morimoto planted it" line tries to make the A1Z26 step feel like a discovery rather than a puzzle instruction.

**Is the answer the last move in the logic?**

ANVIL: does it name what the solver was doing? Marginally — ANVIL is a forging/crafting tool, adjacent to the JRPG crafting system world, but the solve experience was "reverse-engineering a damage formula," not "forging." The answer names a noun in the world's mythology rather than the solver's activity. Compare: a puzzle where the answer is FORMULA, or RESEARCH, or ARCHIVE would have stronger thematic resonance. ANVIL is chosen because it has five letters in the right A1Z26 positions, not because it names the experience.

**Visual grammar:** The puzzle is text-only with battle log code blocks. The code-block format communicates "data" rather than "narrative," which is appropriate for BattleMath_99's research thread framing. No visual inconsistency.

**Does the flavor text explain what the structure should have communicated?** The Skeptic's design notes (post-solution) explain the DEF=1 edge case. These notes are solver-invisible during the solve. The note about DEF=1 being from a modded encounter should be surfaced during the solve — perhaps as a parenthetical in the test log, in BattleMath_99's voice ("against heavily modded mobs — stats outside normal range").

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Text grammar is consistent. Battle log format holds throughout. The A1Z26 instruction is clear even if slightly out of world-register. |
| Solvability | 4 | The world is the solve — the solver's epistemic task (derive the formula from battle data) is the world's task. Nothing requires knowledge outside the provided data. One point off for the level multiplier requiring log 4, which is not signaled as the key log. |
| Elegance | 3 | The world dissolves slightly at extraction. Five parallel arithmetic calculations break the "inhabiting the world" quality that the formula-inference step established. The solver stops being BattleMath_99 and starts being a calculator. Dana would ask: is there a way to keep the solver in the world for the entire extraction? |
| Reading Reward | 5 | Inhabiting BattleMath_99's research is genuine domain engagement. Passes. |
| Fun | 4 | The formula discovery + "Morimoto planted it" reveal is the puzzle's best moment. The solve arc that ends with a word inside the data is the right shape. The back-calculation work is acceptable but breaks the arc slightly before the payoff. |
| Confirmation | 4 | ANVIL lands in the world (forging, blacksmithing). The word is not arbitrary — it has RPG-world coherence. But it doesn't name the solving experience. The confirmation is thematic rather than epiphanic. |
| **Total** | **25/30** | **PASS** |

### Issues Identified (Dana Young full profile applied)

1. **A1Z26 instruction breaks world register.** "Convert each DEF value to a letter using the standard numbering" is puzzle-instruction language. A 1998 forum researcher would not have written this. The "Morimoto planted it" line attempts a world-register recovery but the instruction preceding it is mechanical puzzle-language. Fix: embed the A1Z26 instruction more naturally in BattleMath_99's voice, perhaps as confusion rather than instruction.
2. **ANVIL doesn't name the solver's experience.** The answer lands in the world but doesn't retroactively name what the solver was doing. "You were being BattleMath_99. The word is ANVIL." ANVIL is a forging tool — adjacent but not resonant with the solve experience (formula inference, data research). A stronger word would name the activity: something from the crafting or archives domain that reflects what reverse-engineering a damage formula means in this world.
3. **Extraction phase breaks the world immersion.** The five back-calculations are procedure without world-texture. Compare: if each test battle featured a named enemy with a brief description, the solver stays in BattleMath_99's archive throughout. As designed, the test logs are stripped-down arithmetic inputs.
4. **DEF=1 edge case is explained in post-solution notes only.** BattleMath_99 should mention the modded encounter in the thread itself — this is a 1998 forum researcher; they would note unusual stats.

### Verdict: PASS (25/30)

---

## Condition C7 — Full Profile + Full Principles

**Reviewer:** Dan Katz — full profile + all 11 principles
*All profile elements + all principle tests applied*

### Profile lens checks that fired

**"Does every puzzle justify its slot?"** Yes — the battle-system domain is irreplaceable in IRONFALL's identity, and formula inference distinguishes this puzzle from neighboring puzzles. The slot is justified.

**"Are the mechanisms varied enough?"** This puzzle's mechanism (empirical formula inference) is unique in the hunt. No uniformity concern at the hunt level.

**"Is the thematic coherence structural, not decorative?"** Partial. The battle system world is structural at the formula-inference stage and decorative at the back-calculation stage. The five test battles are data inputs without narrative content. Decorative elements (BattleMath_99 voice, "Morimoto planted it") are present, but the extraction phase's world-texture is thin.

**"Would he want to solve this?"** Yes, for the formula-inference step. Ambivalent for the arithmetic tail.

### Principle tests that fired

**No Computation Without Deduction — Test: Remove the computation. Is there still a puzzle?**
Remove the five back-calculations. What remains? The formula. But the formula is not the answer — it's the method. Without back-calculation, there is no extraction. This test reveals a fundamental structural tension: the computation IS the extraction. The deduction (formula inference) and the computation (back-calculation) are not separable puzzle/non-puzzle layers — they are the formula-phase and the answer-phase of a two-phase puzzle. The principle's test flags this as a warning but does not, on its own, mandate rejection. A puzzle can have a computational extraction if the deduction is rich enough to justify it.

Assessment: the deduction (formula inference with level multiplier) is substantive. The computation (five identical back-calculations with a known formula) is repetitive. The deduction-to-computation ratio is 1:5. Marginal pass on this principle.

**One Aha — Test: Name the single aha.**
The aha: "The formula has a level-dependent multiplier, and log 4 reveals it." This is the single aha. The pair comparison (logs 1–2) is setup work, not the aha — it produces the ×2 rate mechanically from the numbers. The level multiplier requires conceptual work: noticing log 4 produces a mismatch, hypothesizing that level is a variable, deriving the (1 + Level/50) function. That is the puzzle's single genuine deduction. One aha. Passes.

**Snyder's Computer Test — Test: Write a 10-line script.**
```
# Python pseudocode, 10 lines
logs = [(65,12,50,236),(65,55,50,150),(50,22,50,156),(45,6,25,126)]
# fit: dmg = (atk*2 - def) * (1 + lvl/50)
# verify all 4 logs: confirmed
test = [(11,42),(22,60),(20,36),(15,42),(16,40)]
# all level 50: def = atk*2 - dmg/2
defs = [a*2 - d//2 for a,d in test]   # [1,14,22,9,12]
word = ''.join(chr(64+v) for v in defs) # ANVIL
print(word)
```
**Fires.** The puzzle is fully automatable. The formula inference can be scripted (or guessed from the ×2 rate in the first comparison), and the back-calculation is pure arithmetic. Snyder's Computer Test is the principle that most directly challenges this puzzle type.

**Verify the Last Mile — Test: Trace letter by letter.**
Log 5: ATK=11, Damage=42, Level=50. DEF = 11×2 - 42/2 = 22 - 21 = 1. A=1. ✓
Log 6: ATK=22, Damage=60. DEF = 44 - 30 = 14. N=14. ✓
Log 7: ATK=20, Damage=36. DEF = 40 - 18 = 22. V=22. ✓
Log 8: ATK=15, Damage=42. DEF = 30 - 21 = 9. I=9. ✓
Log 9: ATK=16, Damage=40. DEF = 32 - 20 = 12. L=12. ✓
Answer: ANVIL. Clean. No errors in extraction. DEF=1 is arithmetically correct. Last mile is verified.

**Interlock, Not Independence — Test: Any order, no advantage?**
Fires. The five back-calculations can be done in any order. No test battle's result informs another. The extraction is five independent parallel operations. Order confers no advantage, and order confers no verification: an error in log 7 (V) is invisible to logs 6 or 8. There is no interlocking.

**Blame the Player — Test: Retrospective fairness.**
Passes. Every step is fair in retrospect. The A1Z26 instruction is explicit. The level multiplier is discoverable from the training data. DEF=1 is arithmetic-correct. A solver who fails this puzzle failed to use log 4 or made an arithmetic error — both are their own responsibility.

**Surprise the Answer — Test: Guessable from topic?**
ANVIL from "Battle Damage Calculator" — not guessable. Passes cleanly.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Blame the Player fires: every step is fair. A1Z26 is explicit. No ambiguity at any phase. |
| Solvability | 4 | One Aha: the level multiplier is the real insight. The entry point (pair comparison) is discoverable. The level multiplier is the aha. One point off for log 4 being structurally critical and unmarked. |
| Elegance | 2 | Snyder's Computer Test fires. Interlock, Not Independence fires. No Computation Without Deduction ratio is poor (1 deduction : 5 computations). Three principles fire simultaneously against the same structural weakness: the extraction phase is automated procedure, not puzzle architecture. The elegant puzzle would either (a) require the level multiplier to appear in at least one test battle, forcing the solver to use the full formula at extraction, or (b) reduce the test battles to 3 (fewer computations for the same A1Z26 concept) with interlock via answer verification. |
| Reading Reward | 5 | Riven Standard: the mechanic IS the domain activity. Passes. |
| Fun | 3 | One Aha is present and good. The arithmetic tail is not fun. Contract delivery (Katz): the empirical-reasoning promise is partially breached by the accounting extraction. |
| Confirmation | 4 | Verify the Last Mile: clean trace. DEF=1 is the only wobble, and it verifies correctly. ANVIL is thematically coherent. |
| **Total** | **23/30** | **PASS** |

### Issues Identified (Katz + All Principles)

1. **Snyder's Computer Test fails.** Fully automatable. The puzzle is procedure-executable, which is the principle's warning sign for "computation masquerading as deduction."
2. **Interlock, Not Independence fails.** Five parallel independent calculations. No cross-verification structure. An error in any one test battle is invisible.
3. **No Computation Without Deduction ratio is poor (1:5).** The one genuine deduction (level multiplier) is disproportionately outweighed by five identical arithmetic operations.
4. **Level variable introduced and benched.** The level multiplier is the aha, but all five test battles use Level 50, making the full formula unnecessary at extraction. The solver demonstrates mastery of a formula and then only uses half of it.
5. **Log 4 is the critical structural element but carries no marker.** From the contract lens: the puzzle promises all four logs are equivalent training data. Log 4 is, in fact, the key to the entire puzzle. This is technically fair but architecturally slightly dishonest — log 4 is not equivalent; it is the pivot.
6. **DEF=1 edge case is solver-invisible during the solve.** Post-solution notes explain it. Mid-solve, the explanation is unavailable.

### Verdict: PASS (23/30) — weakest condition alongside C3; both catch the Snyder Computer Test failure

---

## Summary Table

### Scores by Condition

| Condition | Context | Reviewer | Clarity | Solvability | Elegance | Reading Reward | Fun | Confirmation | Total | Verdict |
|-----------|---------|----------|---------|-------------|----------|----------------|-----|--------------|-------|---------|
| C0 — Bare | Nothing | Generic | 5 | 4 | 4 | 5 | 4 | 4 | **26** | PASS |
| C1 — Rubric only | 6-dim rubric | Generic | 5 | 4 | 3 | 5 | 4 | 4 | **25** | PASS |
| C2 — Principles compact | Names + quotes | Generic | 5 | 4 | 3 | 5 | 3 | 4 | **24** | PASS |
| C3 — Principles full | + Tests | Snyder | 5 | 4 | 2 | 5 | 3 | 4 | **23** | PASS |
| C4 — Philosophy only | Worldview | Katz | 5 | 4 | 3 | 5 | 4 | 4 | **25** | PASS |
| C5 — Lens only | Checklist | Snyder | 4 | 3 | 2 | 5 | 3 | 4 | **21** | **FAIL** |
| C6 — Full profile | v2 system | Dana | 5 | 4 | 3 | 5 | 4 | 4 | **25** | PASS |
| C7 — Profile + Principles | Everything | Katz | 5 | 4 | 2 | 5 | 3 | 4 | **23** | PASS |

**Mean:** 24.0/30
**Pass rate:** 7/8 conditions

---

### Dimension Pattern Analysis

| Dimension | C0 | C1 | C2 | C3 | C4 | C5 | C6 | C7 | Mean | Variance |
|-----------|----|----|----|----|----|----|----|----|------|---------|
| Clarity | 5 | 5 | 5 | 5 | 5 | 4 | 5 | 5 | 4.9 | Low |
| Solvability | 4 | 4 | 4 | 4 | 4 | 3 | 4 | 4 | 3.9 | Low |
| Elegance | 4 | 3 | 3 | 2 | 3 | 2 | 3 | 2 | 2.8 | **High** |
| Reading Reward | 5 | 5 | 5 | 5 | 5 | 5 | 5 | 5 | 5.0 | **Zero** |
| Fun | 4 | 4 | 3 | 3 | 4 | 3 | 4 | 3 | 3.5 | Medium |
| Confirmation | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4.0 | Low |

**Observations:**

- **Reading Reward is the puzzle's unanimous strength.** 5/5 across all 8 conditions. Zero variance. The Riven Standard fires cleanly: the mechanic IS the domain activity. Formula inference from battle data is exactly what a serious JRPG player would do. No condition found any weakness here.
- **Elegance is the puzzle's contested dimension.** Scores range from 2 (C3, C5, C7 — conditions with operational principle tests) to 4 (C0 bare). The elegance split follows the presence of operational tests: conditions that apply principle tests (particularly Snyder's Computer Test and Interlock, Not Independence) score Elegance at 2; conditions without operational tests score Elegance at 3–4. The dimension is doing the most analytical work.
- **Clarity is near-perfect but not invariant.** C5 (lens only) scores Clarity at 4, not 5, because the lens checklist asks "Is the entry point constructed?" and identifies that the pair-comparison entry is undirected. Every other condition accepts the puzzle's clarity at face value.

---

### The "No Computation Without Deduction" Finding

**This is the puzzle type where this principle is most relevant, and the ablation directly tests whether different context depths catch it.**

| Condition | Does it catch the principle tension? | Severity |
|-----------|--------------------------------------|---------|
| C0 | No — notes "significant arithmetic" but not as a principle violation | None |
| C1 | Partially — Elegance score drops (level variable benched) | Minor |
| C2 | Yes — principle named, ratio problem identified | Moderate |
| C3 | Yes, with test applied — "1:5 ratio, marginal pass" | Strong |
| C4 | Partially — "accounting task" framing, contract gap identified | Moderate |
| C5 | Yes, drives to FAIL — "puzzle followed by procedure" | Strong (FAIL trigger) |
| C6 | Partially — "solver stops being BattleMath_99 and starts being a calculator" | Moderate |
| C7 | Yes, with test applied — Snyder's Computer Test fires explicitly | Strong |

**Key finding:** The principle fires at full strength in C3, C5, and C7 (the conditions that include operational tests). In C0 and C1, the same structural weakness is visible but not named as a principle violation. C2 names the principle but without the test, identifies the tension rather than diagnosing the severity. C4 and C6 translate the principle into worldview language ("the solver becomes a calculator") rather than a structural test.

**The C5 FAIL** is the most significant verdict divergence. The lens checklist applied cold — without biographical context about who is applying it — produces a FAIL because "Is there a puzzle here, or a procedure?" has a direct answer: "Both, and the procedure outweighs the puzzle." With the full biography (C6 — Dana Young applying the same structural concern through world-immersion language), the same weakness is identified but weighted more charitably: the answer still "lands in the world" and the formula discovery provides sufficient foundation for a pass.

**What biography adds:** The biographical frame explains *why* a checklist item fires at the severity it does. C5 fires "procedure vs. puzzle" as a binary. C6 fires "the solver stops inhabiting the world" as a matter of degree. Same defect, different severity because the biographical frame carries calibration information about how much arithmetic is tolerable before the world-illusion breaks.

---

### Verdict Divergence Analysis

All conditions agree: this puzzle PASSES if you score it with any context that includes either (a) the rubric or (b) a philosophical lens. C5 is the single outlier — the lens-only condition issues a FAIL because it applies operational tests cold, without the calibrating context that biographical framing provides.

The puzzle's score is compressed against the pass threshold:

- Highest: C0 at 26/30 (no principles to fire)
- Lowest: C5 at 21/30 (principles fire cold, no calibration)
- Most conditions: 23–25/30 (principles fired but calibrated by biography or rubric)

**This is a different pattern from the prior ablation study's Puzzle I** (Age of Empires Dark Age), which FAILed in every condition above 14/30. P05 is a genuinely passing puzzle that only a cold-lens condition pushes below threshold. The prior study's Puzzle I had a structural defect (underspecified extraction) that was severe enough for all conditions to catch it.

**P05 has no structural defect.** It has a structural tension (computation-heavy extraction, level variable benched) that is a design quality issue, not a solvability issue. Most conditions correctly identify this tension and score it as an elegance deduction rather than a failure. C5 converts the same tension into a FAIL because, without biographical framing, the "puzzle vs. procedure" test becomes a stricter binary.

---

### Comparison with Prior Ablation Results

| Metric | Puzzle I (AoE Dark Age) | Puzzle III (AoE Castle Age) | P05 (Ironfall Battle Calc) |
|--------|------------------------|----------------------------|---------------------------|
| C0 score | 17 ❌ | 24 ✓ | 26 ✓ |
| C5 score | 16 ❌ | 28 ✓ | 21 ❌ |
| C7 score | 13 ❌ | 24 ✓ | 23 ✓ |
| Pass rate | 0/8 | 8/8 | 7/8 |
| Contested dimension | Clarity, Solvability | Elegance | Elegance |
| Principle that fires hardest | Verify Last Mile, Surprise the Answer | None critical | No Computation Without Deduction, Snyder's Computer Test |

P05 behaves like a genuinely strong puzzle (high floor, low variance) rather than a borderline puzzle (low floor, high variance). The single FAIL (C5) is methodological rather than substantive: it reflects the limitation of cold-lens application, not a real design defect.

---

### Implications for Ablation Study Methodology

**New finding from P05:** The ablation effect on P05 differs from its effect on the prior puzzles in a structurally interesting way. For a *strong* puzzle (no genuine defects, only design tensions), context depth determines *how much* the tensions are weighted, not *whether* they're detected. All conditions detect the elegance tension; the conditions differ in whether they call it a failure or a deduction.

This suggests a typology of puzzle quality relative to ablation sensitivity:

| Puzzle type | C0 behavior | C5 behavior | Context depth effect |
|-------------|-------------|-------------|---------------------|
| Broken puzzle (structural defect) | FAILs in most conditions | FAILs harder | Context depth deepens diagnosis, doesn't change verdict |
| Strong puzzle (no defects, tensions) | PASSes with slight deductions | Potentially FAILs (cold lens strips calibration) | Context depth calibrates severity; may change verdict |
| Borderline puzzle | Ambiguous in all conditions | Depends on which lens fires | Context depth most decisive for verdict |

P05 is a strong puzzle in the "potential cold-lens FAIL" category. The prior study's Puzzle I was a broken puzzle in the "consistent FAIL" category. A future ablation study should include all three types to map the full space.

---

*End of ablation report — P05 Battle Damage Calculator*
*Output: C:\src\workspace\research\games\games-ai-expert-panel-creative\results\ablation-ironfall-p05.md*
