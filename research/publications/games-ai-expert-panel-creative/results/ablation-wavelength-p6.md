# Ablation Study — Wavelength P6: Name That Band

**Puzzle:** P6 — Name That Band (Wavelength hunt, Music domain)
**Correct answer (verification only):** ANALOG
**Date:** 2026-02-28
**Design:** 8 conditions × 1 puzzle — testing how reviewer context depth affects evaluation quality

---

## Puzzle Summary (for reference)

Six dictionary definitions, each describing a word that is simultaneously a band name. Extract letter N from definition N's answer. The six answers are: ASIA (1), ENIGMA (2), HEART (3), EAGLES (4), BOSTON (5), CHICAGO (6). Letters: A-N-A-L-O-G = ANALOG.

**Structural features under evaluation:**
- The single aha: "these dictionary words are all band names"
- ASIA/EUROPE ambiguity in clue 1 (lateral interlock — wrong choice breaks extraction)
- Extraction: positional acrostic, each definition contributes exactly one letter
- Answer (ANALOG) has thematic resonance: analog recording, analog radio (the hunt's WVLG station frame)
- Flavor text ("She wrote definitions. As if the names were just words.") signals the band-name layer without giving it away

---

## Condition 0 — Bare Baseline

**Prompt:** "You are an expert reviewer."
**No rubric, no principles, no persona.**

---

### Solving Notes

The puzzle presents six numbered definitions, each identifying a word. The extraction rule is stated explicitly: take the letter at position equal to the definition number.

Working through the definitions:
- 1: Largest continent = ASIA (4 letters). Letter 1 of ASIA = A
- 2: Something mysterious, Churchill quote = ENIGMA (6 letters). Letter 2 = N
- 3: Organ that pumps blood = HEART (5 letters). Letter 3 = A
- 4: Large birds of prey, bald variety = EAGLES (6 letters). Letter 4 = L
- 5: Port city, Massachusetts, marathon = BOSTON (6 letters). Letter 5 = O
- 6: Third-largest US city, Lake Michigan = CHICAGO (7 letters). Letter 6 = G

Answer: ANALOG.

The definitions are unambiguous in isolation. The positional extraction is stated and confirmed by the provided table. The puzzle confirms on its own.

One structural note: ASIA and EUROPE are both bands and both continents. The puzzle does not flag this ambiguity. A solver who chooses EUROPE gets letter 1 = E, producing a different six-letter string that is not a word. The extraction itself is the disambiguator — the puzzle relies on the solver recognizing the failure and backtracking.

A deeper observation: all six answers (ASIA, ENIGMA, HEART, EAGLES, BOSTON, CHICAGO) are band names. This is not stated anywhere in the puzzle. The definitions are dictionary definitions of the words themselves. Whether a solver notices the band-name pattern or not, they can complete the extraction purely from definitional answers. The band-name layer is either a hidden reward or a red herring depending on what the puzzle intends.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | The extraction rule is fully stated. The table format mirrors the mechanic cleanly. The flavor text sets atmosphere without encoding required information. Instructions are complete. |
| Solvability | 4 | All six definitions are solvable without specialized knowledge. The only risk is the ASIA/EUROPE ambiguity in clue 1, which is not flagged. A solver who picks EUROPE will reach a non-word and must backtrack — the puzzle provides no warning, but the extraction failure is itself a signal. |
| Elegance | 4 | The mechanic is clean: define a word, extract its Nth letter. The positional extraction is satisfying and requires no secondary encoding. Slight tension: the band-name connection appears to be the conceptual heart of the puzzle, but it is not required for solving. If it is decorative, the puzzle is merely a clean definitional acrostic. If it is structural, the puzzle has not earned that layer. |
| Reading Reward | 4 | The definitions are well-written and accurate. A music fan may recognize band names and gain an extra layer of pleasure. The Churchill quote (clue 2) is a genuine pleasure. However, the domain engagement is primarily vocabulary, not music knowledge — a solver with no music knowledge can complete the puzzle identically to an expert. |
| Fun | 4 | The definitions are enjoyable to read and solve. The moment of recognition (if a solver sees the band-name pattern) is pleasant. The answer ANALOG has a satisfying music-adjacent resonance. |
| Confirmation | 3 | ANALOG is a real word with music relevance, and it fits the extraction cleanly. However, a solver who did not notice the band-name pattern cannot verify they "got it" beyond extracting a pronounceable word. The confirmation mechanism does not close the loop on what the puzzle was actually about. |
| **Total** | **24/30** | |

### Issues Identified

1. **ASIA/EUROPE ambiguity unaddressed.** Clue 1 fits both continents, and both ASIA and EUROPE are band names. The puzzle does not flag this. A solver may stall if they choose EUROPE and do not understand why the extraction fails.
2. **Band-name layer is structurally invisible.** The defining insight of the puzzle — that all six answers are band names — is never confirmed for the solver. A successful solve can happen with zero awareness of the band-name connection. This raises a question about what the puzzle is actually testing.
3. **No closure on the conceptual layer.** The flavor text gestures at "names that are also words," but the puzzle ends at letter extraction. The thematic closure is absent.

### Verdict: PASS

---

## Condition 1 — Rubric Only

**Prompt:** "You are an expert reviewer." + 6-dimension rubric (Clarity / Solvability / Elegance / Reading Reward / Fun / Confirmation, 1-5 each, pass ≥ 22).
**No principles, no persona.**

---

### Solving Notes

Working through definitions: ASIA (A), ENIGMA (N), HEART (A), EAGLES (L), BOSTON (O), CHICAGO (G) = ANALOG. Extraction is positional and fully stated. The band-name pattern (all six words are band names) is discoverable but not required for solving.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | The mechanic is stated completely. Table layout mirrors the solve. The extraction rule — position N, take letter N — is explained and illustrated. No ambiguity in the instructions. |
| Solvability | 4 | Five of six definitions are unambiguous. Clue 1 (ASIA vs EUROPE) creates an unannounced branch: both are continents, both are bands. A solver who chooses EUROPE will not reach ANALOG. The puzzle relies on extraction failure as the disambiguation signal, which is an implicit mechanism rather than a designed one. The remaining five definitions are clean. |
| Elegance | 4 | The positional acrostic structure is elegant. One definition → one letter → positional extraction → six-letter answer. No secondary encoding. The puzzle is efficient. Deduction: the band-name layer, if intentional, is an orphaned constraint — it is the organizing principle that generated the puzzle but does not appear in the solve path or the confirmation. A mechanic that organized construction but not solving is structural weight with no payoff. |
| Reading Reward | 3 | The definitions are accurate and well-phrased, but they are lexicographic rather than musical. A music domain puzzle that rewards vocabulary rather than music knowledge has not committed to its genre. The Churchill quote in clue 2 is the one moment of genuine writing pleasure. The remaining definitions are functional. A crossword-quality solver can complete this puzzle as well as a music expert. |
| Fun | 4 | The definitional format is enjoyable. The progression through six answers produces mounting pleasure for a solver who notices the band-name pattern. The answer ANALOG is satisfying. The post-solve recognition — "every definition described a band name" — is the puzzle's best moment, but it arrives after the puzzle has ended. |
| Confirmation | 3 | ANALOG confirms as a plausible six-letter word with music relevance. But the puzzle has no mechanism that tells the solver they found the band-name layer. A solver who extracted ANALOG without noticing band names cannot know whether they solved the puzzle correctly or missed its point. Confirmation is ambiguous when the puzzle's actual thesis is invisible. |
| **Total** | **23/30** | |

### Issues Identified

1. **ASIA/EUROPE ambiguity is a silent failure.** The branch is real, and the disambiguation relies on the solver noticing extraction failure rather than on any designed mechanism. If the puzzle requires ASIA not EUROPE, the definition should make that unambiguous — or the puzzle should acknowledge the ambiguity exists.
2. **The band-name layer is the puzzle's actual hook but is not in the solve path.** The definitions were chosen because those words are band names. The solver can extract ANALOG without ever knowing that. This is the core design question: is the band-name pattern a confirmation mechanism that should be present, or a construction method that the puzzle outgrew?
3. **Reading Reward does not match the music domain.** Six vocabulary definitions in a music hunt is a domain mismatch. A puzzle about music should require music knowledge.

### Verdict: PASS

---

## Condition 2 — Principles Compact (Names + Quotes)

**Prompt:** "You are an expert reviewer." + rubric + 11 principle names and quotes only (no tests, no persona).

**Principles applied:**
- The Riven Standard: "The puzzle IS what the field does."
- Solving = Proving Understanding: "Solving shows understanding."
- Blame the Player: "Blame yourself."
- No Over-Scaffolding: "Worksheet that does all the work is not a puzzle."
- Surprise the Answer: "Pause, not nod."
- One Aha: "One aha is fun."
- Reading Reward: "Genuine domain engagement."
- No Computation Without Deduction: "Formula ≠ solving."
- Snyder's Computer Test: "Computer can solve?"
- Interlock, Not Independence: "Independent lookups = quiz."
- Verify the Last Mile: "Mechanism and extraction separate."

---

### Solving Notes

ASIA(A), ENIGMA(N), HEART(A), EAGLES(L), BOSTON(O), CHICAGO(G) = ANALOG. Confirmed letter by letter.

### Principle Analysis

**The Riven Standard ("The puzzle IS what the field does."):** The puzzle's field is music — specifically, band names. The puzzle's mechanism is dictionary definitions. The mechanism is not what band-name experts do. A music expert does not look up the geographical description of ASIA to identify the band. This is a lexicographic puzzle wearing a music costume. The Riven Standard is not met.

**Solving = Proving Understanding:** Does a solver know more about music after solving? They know ASIA, ENIGMA, HEART, EAGLES, BOSTON, CHICAGO are bands — but only if they noticed the band-name pattern. A solver who extracted ANALOG without noticing the pattern has proven nothing about music. Partial credit at best.

**Blame the Player:** Every definition is accurate. No misdirection. A solver who picks EUROPE (clue 1) can self-diagnose from extraction failure. Fair in retrospect. This principle is met.

**No Over-Scaffolding:** The table provides space for words and letters, and states the rule once. It does not solve the puzzle. The flavor text does not give away the mechanic. This principle is met.

**Surprise the Answer:** Is ANALOG guessable from the topic (music definitions)? No — there is no path from "music definitions" to ANALOG without solving. This principle is met.

**One Aha:** What is the single aha? The puzzle has a candidate aha — "these words are all band names." But this aha is not required for solving. If the aha is not in the solve path, it cannot be the One Aha. A puzzle without a required aha is a procedure, not a puzzle with an insight.

**Reading Reward ("Genuine domain engagement."):** The definitions are vocabulary questions. Music knowledge is not required. This principle is not met: the reading is rewarding as prose, but not as domain engagement.

**No Computation Without Deduction:** The extraction is pure position lookup — count to letter N. No computation, no deduction. This principle is not applicable (no computation issue).

**Snyder's Computer Test ("Computer can solve?"):** A ten-line script can: (1) identify the answer to each definition from a dictionary, (2) extract letter N from each answer, (3) concatenate. No band-name reasoning is required because the band-name layer is not in the solve path. A computer solves this puzzle without the aha. The puzzle fails the computer test.

**Interlock, Not Independence:** Each definition is solved independently. The results are concatenated. There is one interlock moment — ASIA/EUROPE ambiguity in clue 1 — where the correct answer is determined by whether the extraction produces a valid result. This is one moment of genuine interlock. The other five answers are independent. Partial credit.

**Verify the Last Mile:** Extracting letter N from word N is mechanically sound. A-N-A-L-O-G = ANALOG. The extraction is traceable letter by letter. This principle is met.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Instructions are complete. Table is clear. Flavor text does not over-scaffold. |
| Solvability | 4 | Five definitions clean; one (ASIA/EUROPE) has an unannounced branch resolved by extraction failure. |
| Elegance | 3 | The positional structure is clean, but the organizing principle (band names) does not appear in the solve path. A mechanic that explains why the puzzle was constructed but not how to solve it is an incomplete translation from concept to form. The One Aha is structurally orphaned. |
| Reading Reward | 2 | Dictionary definitions in a music domain puzzle. The domain is present in the flavor text, absent in the mechanics. Music expertise provides no advantage. This is the puzzle's most significant flaw relative to its context. |
| Fun | 4 | The definitions are pleasant to solve. The post-solve recognition (if it occurs) is the puzzle's best moment. |
| Confirmation | 3 | ANALOG confirms as a real word with music resonance. But confirmation requires that the solver know they found what the puzzle was looking for — and the band-name layer provides no confirmatory mechanism. |
| **Total** | **21/30** | |

### Issues Identified

1. **Band-name layer fails Snyder's Computer Test.** A script solves the puzzle without the aha. The defining insight is not in the solve path.
2. **Reading Reward does not match the domain.** Music expertise is not load-bearing.
3. **One Aha is structurally orphaned.** The insight ("they're all band names") should either be required for solving or acknowledged as a construction artifact that needs redesign.
4. **ASIA/EUROPE silent ambiguity.** One implicit interlock is not sufficient; the puzzle relies on accidental disambiguation.
5. **Confirmation incomplete.** Solver cannot close the loop without the band-name insight.

### Verdict: FAIL

---

## Condition 3 — Principles Full (Names + Quotes + Tests)

**Prompt:** C2 + explicit operational tests for each principle.

**Tests added:**
- Riven Standard test: "Practitioner recognizes their own work?"
- Solving = Proving Understanding test: "More knowledge after?"
- Blame the Player test: "Respect or resentment?"
- No Over-Scaffolding test: "Remove — still a puzzle?"
- Surprise the Answer test: "Guessable from topic?"
- One Aha test: "Name the single aha."
- Reading Reward test: "Keyword search sufficient?"
- No Computation Without Deduction test: "Remove instructions — still a puzzle?"
- Snyder's Computer Test: "10-line script."
- Interlock test: "Any order, no advantage?"
- Verify the Last Mile test: "Trace letter by letter."

---

### Solving Notes

ASIA(A), ENIGMA(N), HEART(A), EAGLES(L), BOSTON(O), CHICAGO(G) = ANALOG. Verified letter by letter.

### Operational Test Results

**Riven Standard — Practitioner recognizes their own work?**
Test: Would a working musician or music fan see this and say "yes, this is a music puzzle"? The definitions describe continents, medical organs, cities, and raptors. The musical frame is entirely absent until the solver either recognizes the band-name pattern or reads the flavor text. A musician would not recognize their domain in the puzzle body. FAIL.

**Solving = Proving Understanding — More knowledge after?**
Test: Does the solver know more about music after solving? Only if they noticed the band-name pattern. A solver who extracts ANALOG without the aha knows: (1) the largest continent is Asia, (2) Churchill described Russia as an enigma, (3) the heart is a muscular organ. None of this is music knowledge. FAIL for domain; PASS only if the band-name recognition occurs.

**Blame the Player — Respect or resentment?**
Test: If a solver takes 20 minutes on clue 1 trying EUROPE instead of ASIA, do they blame themselves or the puzzle? The puzzle does not flag the ambiguity; it relies on extraction failure. A solver who gets stuck on ASIA/EUROPE has legitimate grounds to resent the puzzle for not acknowledging the ambiguity. MARGINAL FAIL.

**No Over-Scaffolding — Remove instructions, still a puzzle?**
Test: Remove the table and "take the letter at position N" instruction. What remains? Six numbered definitions. Without the extraction rule, a solver has no mechanic. The mechanic is entirely in the instructions. This is different from over-scaffolding — the instruction is necessary, not excessive. PASS.

**Surprise the Answer — Guessable from topic?**
Test: Can a solver guess ANALOG from "music definitions"? No. PASS.

**One Aha — Name the single aha.**
Test: Name it. The candidate is "these words are all band names." But running the operational test: is this aha required to solve the puzzle? No. Can a solver complete the puzzle without it? Yes. A required aha is one whose recognition changes what you do next. The band-name recognition changes nothing about the extraction procedure. The aha is present but structurally inert. FAIL on the "required" criterion.

**Reading Reward — Keyword search sufficient?**
Test: Can a solver keyword-search their way through the puzzle? Search "largest continent" → ASIA. Search "Churchill mystery quote" → ENIGMA. Search "organ pumps blood" → HEART. All six definitions resolve by basic search. Music knowledge adds nothing. FAIL.

**No Computation Without Deduction:**
Test: Remove instructions. The definitions remain, but there is nothing to do with the answers. No deduction is implied by the definitions alone. The puzzle's deductive content (band names!) is structural at the construction level, not the solving level. PASS on computation (no formula), but this exposes a deeper issue: the deductive layer was not translated into the mechanic.

**Snyder's Computer Test — 10-line script:**
```python
import re
definitions = {"ASIA": 1, "ENIGMA": 2, "HEART": 3, "EAGLES": 4, "BOSTON": 5, "CHICAGO": 6}
answer = ""
for word, pos in definitions.items():
    answer += word[pos - 1]
print(answer)  # ANALOG
```
The script solves the puzzle. No music knowledge, no band-name recognition, no aha required. FAIL.

**Interlock — Any order, no advantage?**
Test: Can a solver tackle definitions in any order? Yes. Does any order reveal information about another definition? No — except: ASIA/EUROPE in clue 1 can only be resolved by running the extraction and checking if the full result is a word. This is one moment of cross-definition interlock, but it is implicit. Solving the other five first does not help with clue 1 except retrospectively. MARGINAL PASS.

**Verify the Last Mile — Trace letter by letter:**
Test: ASIA→letter 1→A. ENIGMA→letter 2→N. HEART→letter 3→A. EAGLES→letter 4→L. BOSTON→letter 5→O. CHICAGO→letter 6→G. A-N-A-L-O-G = ANALOG. Clean. PASS.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Instructions complete. The only missing element is acknowledgment of ASIA/EUROPE ambiguity — but extraction failure covers this. |
| Solvability | 4 | Five clean definitions plus one silent ambiguity. All definitions resolvable by basic search or general knowledge. No music expertise required. |
| Elegance | 2 | The conceptual architecture (band names as hidden layer) and the solving architecture (dictionary lookup + positional extraction) are parallel but not connected. The elegant puzzle was the one where band-name recognition was required for solving. This is not that puzzle — it is a clean positional extraction wearing that puzzle's clothes. |
| Reading Reward | 2 | Keyword-search-sufficient. Music knowledge provides no advantage. The Riven Standard and Reading Reward tests both FAIL for domain engagement. A music puzzle that does not require music knowledge has not been finished. |
| Fun | 3 | The definitions are pleasant. The band-name recognition (if it occurs) is the puzzle's most fun moment. But the One Aha is not required, so it arrives accidentally. Accidental fun is not designed fun. |
| Confirmation | 3 | ANALOG extracts cleanly. But the solver cannot confirm they found the intended insight. The band-name pattern is the confirmation mechanism the puzzle needs but does not have. |
| **Total** | **19/30** | |

### Issues Identified

1. **The defining insight is not in the solve path (Snyder's Computer Test: FAIL).** A script solves the puzzle without band-name recognition. This is the core structural failure.
2. **Reading Reward FAIL: music knowledge is not load-bearing.** Any domain could supply definitions of ASIA, ENIGMA, HEART, EAGLES, BOSTON, CHICAGO. The puzzle is not a music puzzle mechanically.
3. **One Aha is structurally orphaned.** Redesign should make band-name recognition required — e.g., the definitions could omit geographical and biological context, forcing the solver to identify each answer as a band name specifically.
4. **ASIA/EUROPE ambiguity should be acknowledged or removed.** Either make clue 1 unambiguous (ASIA not EUROPE), or introduce the band-name constraint into the mechanic (which would automatically resolve the ambiguity: both ASIA and EUROPE are bands, but the puzzle would then need to distinguish them at the band-knowledge level, not the geographical level).
5. **Blame the Player marginal:** The ASIA/EUROPE trap is unfair without acknowledgment.

### Verdict: FAIL

---

## Condition 4 — Design Philosophy Only (No Lens)

**Prompt:** C1 (rubric) + Design Philosophy sections only from Dan Katz, Thomas Snyder, and Dana Young profiles. No Review Lens bullet-points. No biographical context.

**Philosophies present:**
- Katz: puzzle hunts as contracts; naming failure modes; the whole must be proportionate; no peace with hunts that are too long; structural coherence
- Snyder: constructor's hand must be visible; theme and structure must be the same thing; noise is not difficulty; elegance is proof the puzzle was finished; a procedure is not a puzzle
- Young: starts with a world; mechanic IS the theme; designed toward an inevitable answer; range is fidelity; the answer names the experience

---

### Solving Notes

ASIA(A), ENIGMA(N), HEART(A), EAGLES(L), BOSTON(O), CHICAGO(G) = ANALOG.

### Philosophy-Informed Analysis

**Katz's contract lens:** The puzzle makes an implicit contract with the solver: "these definitions describe something in the music domain." The solver who signs that contract and expects to use music knowledge will find that no music knowledge was required — only geography, biology, and US history. The contract is broken. A puzzle that advertises a domain and then operates entirely in other domains has made a promise it doesn't keep.

**Katz's architectural lens:** Does every puzzle justify its slot? In a music hunt about radio, this puzzle appears to be about the band-name wordplay affordance. The execution uses band names as a construction tool (they organized which words to pick) rather than a solving tool. The slot is not justified by the mechanic — it is justified by the concept, which is not the same thing.

**Snyder's "theme and structure must be the same thing":** The theme is band names. The structure is positional letter extraction from dictionary definitions. These are not the same thing — theme is in the concept, structure is in the mechanics. Snyder calls this "a label, not a puzzle." The constructor had a theme and then placed an independent mechanic on top of it.

**Snyder's "a procedure is not a puzzle":** The solve path is: look up definition → identify word → count to position N → record letter → repeat × 6. This is a procedure. Where is the moment of genuine deduction? Not in the definitions (all resolvable by search). Not in the extraction (purely mechanical). The band-name recognition would provide the deductive moment — but it is not required. There is no puzzle here; there is a procedure that happens to have a thematic concept attached.

**Snyder's "elegance is proof the puzzle was finished":** The puzzle is clean but not elegant. Clean means no errors. Elegant means the form proves the concept. This puzzle has the form of a definitional acrostic and the concept of a band-name puzzle. They do not prove each other.

**Young's "mechanic IS the theme":** The mechanic is dictionary definitions + positional extraction. The theme is music / band names. These are not the same thing. Young would identify this immediately: "You didn't build a music puzzle; you built a vocabulary puzzle in a music setting."

**Young's "answer names the experience":** ANALOG. Does ANALOG name the experience of solving this puzzle? It is music-adjacent (analog recording). But the experience of solving was: reading geography definitions, identifying continents and cities, extracting letters. ANALOG does not name that experience. It names a music concept adjacent to the hunt's setting. It does not name what you were doing while solving.

**Young's "inevitable vs. arbitrary answer":** ANALOG feels inevitable once seen in a music-radio context. But it is not inevitable from the solve path — it is a word that emerged from the positional extraction of six specifically chosen definitional answers. Its music resonance is coincidental to the mechanism, not generated by it. Young would call this arbitrary: "a word the designer happened to land on."

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Instructions are complete. |
| Solvability | 4 | Definitions resolvable; ASIA/EUROPE branch is unannounced. |
| Elegance | 2 | Theme and structure are parallel but not integrated. The puzzle is a procedure, not a deduction. Snyder: not finished. Young: mechanic does not equal theme. |
| Reading Reward | 2 | Vocabulary puzzle in a music domain. Domain knowledge is not required. Katz: the contract with the solver is broken. |
| Fun | 3 | Band-name recognition (if it occurs) is the puzzle's most enjoyable moment. But accidental aha ≠ designed aha. |
| Confirmation | 3 | ANALOG is musically resonant but not generated by the musical insight. |
| **Total** | **19/30** | |

### Issues Identified

1. **The puzzle is a procedure (Snyder diagnosis): no required deductive moment.** The band-name recognition is the missing deductive step; it must be integrated into the mechanic.
2. **Theme and structure are not the same thing (Young/Snyder consensus).** Dictionary definitions are not what music practitioners do.
3. **Contract violation (Katz diagnosis).** Music hunt puzzle that requires no music knowledge breaks the solver's expectation contract.
4. **Answer does not name the experience (Young).** ANALOG is music-adjacent but does not reframe the solve.
5. **ASIA/EUROPE ambiguity is a structural gap the philosophy lenses notice as a "not finished" signal.**

### Verdict: FAIL

---

## Condition 5 — Review Lens Only (No Biography)

**Prompt:** C1 (rubric) + Review Lens bullet-points only from Katz, Snyder, Young. No Design Philosophy sections. No biographical context.

**Active lenses:**

*Katz lens:* Does every puzzle justify its slot? Does the narrative live in the puzzles or get reported afterward? Would he want to solve this?

*Snyder lens:* Is there a puzzle here, or a procedure? Is each element load-bearing? Does the puzzle teach the skill it claims to require? Would he want to have constructed this?

*Young lens:* Does the visual grammar hold? Is the extraction earning its step? Is the answer arbitrary or inevitable? Does flavor text explain what layout should have communicated?

---

### Solving Notes

ASIA(A), ENIGMA(N), HEART(A), EAGLES(L), BOSTON(O), CHICAGO(G) = ANALOG.

### Lens Checks

**Katz — "Does every puzzle justify its slot?"**
A music hunt puzzle slot expects music knowledge to be load-bearing. Music knowledge here is not load-bearing. The puzzle exists in this slot because its concept (band names) fits the theme — but the mechanic doesn't require it. The slot is claimed but not earned.

**Katz — "Is the narrative encountered while solving or reported afterward?"**
The narrative (Vee writing band names as definitions, "as if the names were just words") is fully contained in the flavor text, which is solved before any puzzle mechanics begin. The narrative is frontloaded, not distributed through the solve. It is reported, not encountered.

**Katz — "Would he want to solve this?"**
A positional extraction from dictionary definitions does not present the structural challenge Katz finds engaging. A solver who enjoys music knowledge has nowhere to apply it. The puzzle is pleasant but does not engage the type of solver this hunt is designed for.

**Snyder — "Is there a puzzle here, or a procedure?"**
The solve path is: identify word → extract letter → repeat × 6. This is a procedure. The moment of genuine deduction would be recognizing the band-name constraint and using it to resolve ASIA vs EUROPE. But this recognition is not required. Procedure verdict: yes.

**Snyder — "Is each element load-bearing?"**
Remove any one definition: the extraction produces a 5-letter partial. Each definition is load-bearing for extraction. But is the band-name constraint load-bearing? No: remove it, and the puzzle still works. The thematic constraint is construction scaffolding, not solving architecture.

**Snyder — "Does the puzzle teach the skill it claims to require?"**
The puzzle appears to claim: "know your bands." But a solver who finishes the puzzle has learned nothing about bands unless they noticed the pattern. The skill claimed (music domain expertise) is not what the puzzle tests.

**Snyder — "Would he want to have constructed this?"**
The constructor chose six words that are simultaneously bands and dictionary-clear concepts. That is a genuine construction achievement. But the solving path does not reflect the construction insight. The construct is interesting; the puzzle is not.

**Young — "Does the visual grammar hold?"**
The table format is consistent. The extraction columns are parallel. Each row is the same type of task. The visual grammar is clean and consistent. PASS.

**Young — "Is the extraction earning its step?"**
Extracting letter N from word N is purely mechanical: count to position N. There is no transformation that adds meaning. The extraction is a tax. Young would ask: could the extraction be the moment where the band-name insight activates? Currently it is not.

**Young — "Is the answer arbitrary or inevitable?"**
ANALOG. Is it inevitable? Only if the solver understands that all six words are band names and that ANALOG connects to music. But since the band-name insight is not required, ANALOG's music resonance is coincidental to the solve path. Young verdict: arbitrary in practice, would-be-inevitable by design intent.

**Young — "Does flavor text explain what layout should have communicated?"**
"She wrote definitions. As if the names were just words." This flavor tells the solver that the definitions are obscuring a hidden identity — which is exactly the band-name pattern. The flavor text is doing the work the mechanic should do. Young's diagnosis: the flavor text substitutes for a design that should surface the band-name constraint into the solving mechanics. Two problems: over-scaffolding the concept, and exposing that the mechanic didn't finish.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Table and instructions are clear. Flavor text is well-written. |
| Solvability | 4 | All definitions solvable. ASIA/EUROPE unannounced. |
| Elegance | 2 | Snyder verdict: procedure, not puzzle. Band-name constraint is construction-only. |
| Reading Reward | 2 | Katz: slot not earned. Music knowledge not required. |
| Fun | 3 | The band-name recognition is pleasant if it occurs. The extraction is mechanical. |
| Confirmation | 2 | Young: flavor text is doing the work the mechanic should do. Solver cannot close the confirmation loop because the defining insight never enters the mechanic. |
| **Total** | **18/30** | |

### Issues Identified

1. **Procedure, not puzzle (Snyder lens).** No required deductive moment. Band-name recognition is optional.
2. **Flavor text substitutes for mechanic (Young lens).** "As if the names were just words" tells the solver what the puzzle is about, which is the work the mechanic should do.
3. **Slot not earned (Katz lens).** Music expertise provides no advantage.
4. **Extraction is a tax (Young lens).** Positional letter extraction adds no meaning; it merely delays the answer.
5. **ASIA/EUROPE ambiguity: Katz's "does every element justify its presence" fires — the ambiguity is unintentional noise, not designed interlock.**

### Verdict: FAIL

---

## Condition 6 — Full Profiles

**Prompt:** C1 (rubric) + complete profiles for Dan Katz, Thomas Snyder, Dana Young (all sections: Identity, Hunt Credentials, Design Philosophy, Review Lens).

---

### Solving Notes

ASIA(A), ENIGMA(N), HEART(A), EAGLES(L), BOSTON(O), CHICAGO(G) = ANALOG.

Reviewing as: Dan Katz (structural), Thomas Snyder (craft), Dana Young (thematic coherence).

### Dan Katz Review

*8× MIT Mystery Hunt winner; Puzzlvaria critic; author of the "Duck Konundrum"; Brown mathematics faculty. Sees puzzles as contracts; invented the vocabulary for hunt failure modes (mettleneck, binding choice, backsolving, wheel-of-fortuning).*

Katz reads puzzle hunts as contracts, and he reads individual puzzles as local contracts within that larger one. This puzzle makes a specific local contract: it lives in a music hunt, in a round about a DJ's last playlist, and its title is "Name That Band." The solver signs that contract and expects to use music knowledge to name bands.

The puzzle does not require the solver to name any bands. All six answers are achievable through geography, biology, and US history. The contract is broken.

That said: the puzzle is not fraudulent — it has a genuine concept and a clean mechanic. The concept (band names hidden in definitions) is excellent. The mechanic (positional extraction) is sound. The failure is architectural: the concept was not translated into the mechanic. The solver who should name bands gets a vocabulary exercise instead.

Katz's structural test: Does this puzzle justify its slot in the hunt? The hunt is about music; the puzzle is about words that happen to be bands. Those are not the same thing. A puzzle that fills a music slot without requiring music knowledge is occupying a slot under false premises.

Katz's "would he want to solve this" test: No. A solver who comes to a music hunt for music knowledge engagement will feel cheated when music knowledge turns out to be decorative.

### Thomas Snyder Review

*3× World Sudoku Champion, 6× US Puzzle Champion; first dual WSC/WPC winner; Grandmaster Puzzles founder. "If a computer can generate your puzzle, your puzzle isn't finished." "Theme and structure must be the same thing."*

Snyder tests: Is there a puzzle here, or a procedure?

The solve path: look up definition → write word → count to letter N → record. Repeat six times. This is a procedure. The deductive moment — "these words are all band names" — is not required to execute the procedure. A solver who finishes without noticing the band-name pattern has not been robbed of an insight; they simply never needed it.

Snyder's second test: Does the theme shape the structure, or sit on top of it? The theme is band names. The structure is positional extraction from definitions. The theme organized the choice of words. The structure does not require the theme to operate. Theme sits on top.

Snyder's construction test: Would he want to have constructed this? The constructor faced a genuine challenge: find six band names whose Nth letter (taken in order) spells a thematically resonant word. ANALOG is an excellent target — musically meaningful and not guessable from the topic. The construction is skilled. The architecture failed: the construction insight (every word is a band name) needed to become a solving constraint, not a construction convenience.

Snyder's elegance test: "Elegance is not a luxury. It is the proof that the puzzle was finished." The puzzle is not finished. A finished version of this puzzle would require the solver to recognize each answer as a band name — either by providing clues that only work with band-name knowledge, or by making the band-name connection the disambiguation mechanism for the ASIA/EUROPE branch.

The ASIA/EUROPE branch is the puzzle's most interesting structural element: two valid answers to clue 1, one of which produces the correct extraction. This is precisely the kind of "lateral information" Snyder prizes in puzzle design — a constraint from one part of the puzzle resolves an ambiguity in another. But currently this interlock is implicit and unannounced. A designed version would surface this as the puzzle's feature, not its flaw.

### Dana Young Review

*25-year Microsoft Puzzlehunt veteran; Placement Test round designer (19 puzzles, 19 distinct mechanics, every one thematically exact). "She starts with a world." "The mechanic IS the theme." "The answer names the experience you just had."*

Young reads the flavor text first, because flavor text reveals what the designer thought the puzzle was. "She wrote definitions. As if the names were just words." This tells the solver: the definitions are disguising something. The something is band names. The flavor text has done the puzzle's conceptual work before the solver opens the table.

Young's diagnosis: when flavor text has to tell you what the puzzle is about, the visual design failed to communicate it. In this case, the table of definitions looks like a vocabulary quiz. Nothing in the table's structure signals "these words are secretly band names." The flavor text compensates for a mechanic that did not finish.

Young's "inevitable answer" test: ANALOG. Does it name the experience? The solver spent time identifying continents, organs, birds, and cities. ANALOG does not name that experience. It names a music concept that resonates with the hunt's radio setting. Young's verdict: the answer is architecturally arbitrary even though it is conceptually deliberate. An inevitable answer would emerge from what the solver was actually doing — and the solver was doing geography, not music.

Young's extraction test: Is the extraction earning its step? Positional extraction — take letter N from word N — adds no meaning. The mechanic could instead require: (1) identify the band, (2) confirm it is real, (3) extract the Nth letter of the band's name as a band name. That would make the extraction earn its step by requiring music knowledge at the point of extraction.

Young's thematic coherence test: the theme dissolves at the mechanic level. It holds in the flavor text (Vee writing definitions, "as if the names were just words"), in the title ("Name That Band"), in the six chosen words. It disappears the moment the solver opens the extraction table and writes down "continent," "mystery," "organ," "birds," "city," "city."

### Composite Scores

| Dimension | Katz | Snyder | Young | Avg | Notes |
|-----------|------|--------|-------|-----|-------|
| Clarity | 5 | 5 | 5 | 5.0 | Instructions complete; flavor text well-written |
| Solvability | 4 | 4 | 4 | 4.0 | Five clean; ASIA/EUROPE unannounced |
| Elegance | 2 | 2 | 2 | 2.0 | Concept and mechanic not integrated; procedure not puzzle |
| Reading Reward | 2 | 2 | 2 | 2.0 | Music knowledge not load-bearing |
| Fun | 3 | 3 | 4 | 3.3 | Band-name recognition pleasant if it occurs; Young finds more fun in the construction intent |
| Confirmation | 3 | 2 | 2 | 2.3 | ANALOG music-resonant but not mechanically inevitable |
| **Total** | **19** | **18** | **19** | **18.7/30** | |

### Issues Identified

1. **Band-name recognition is not required (Katz + Snyder + Young consensus).** The puzzle's defining insight — the aha — is structurally optional. This is the primary failure.
2. **Procedure, not puzzle (Snyder).** The solve path is six lookup-and-count operations. No deduction required.
3. **Flavor text compensates for mechanic failure (Young).** The table should communicate "band names" structurally; the flavor text does it instead.
4. **Domain engagement absent (Katz).** Music expertise provides no advantage over vocabulary knowledge.
5. **Extraction does not earn its step (Young).** Positional extraction adds mechanical complexity without semantic payoff.
6. **ASIA/EUROPE ambiguity is unfinished architecture (Snyder).** What could be a designed lateral-information interlock is currently an unannounced failure mode.
7. **Answer does not name the experience (Young).** ANALOG is conceptually resonant but experientially arbitrary.

### Verdict: FAIL

---

## Condition 7 — Full Profiles + Full Principles

**Prompt:** C6 (full profiles) + all 11 principles with names, quotes, and operational tests.

---

### Solving Notes

ASIA(A), ENIGMA(N), HEART(A), EAGLES(L), BOSTON(O), CHICAGO(G) = ANALOG.

**Verify the Last Mile (explicit trace):**
- Word 1: ASIA. Length 4. Position 1. Letter: A[1] = A. ✓
- Word 2: ENIGMA. Length 6. Position 2. Letter: E[N]IGMA = N. ✓
- Word 3: HEART. Length 5. Position 3. Letter: HE[A]RT = A. ✓
- Word 4: EAGLES. Length 6. Position 4. Letter: EAG[L]ES = L. ✓
- Word 5: BOSTON. Length 6. Position 5. Letter: BOST[O]N = O. ✓
- Word 6: CHICAGO. Length 7. Position 6. Letter: CHICAG[O] — wait. CHICAGO: C(1) H(2) I(3) C(4) A(5) G(6) O(7). Letter 6 = G. ✓
Answer: A-N-A-L-O-G = ANALOG. Confirmed.

Note on CHICAGO: length 7, position 6 = G. Correct. (Not the final letter — important for solvers who miscounting assume the last letter of a 7-letter word at position 6 must be O.)

### Full Analysis

**The Riven Standard — Practitioner recognizes their own work?**
Test: Would a music practitioner (session musician, radio DJ, band historian) look at six dictionary definitions of continent/mystery/organ/birds/city/city and recognize this as music-domain work? No. The puzzle's domain is lexicography. The music domain appears only in the concept (band names) and the hunt's frame (radio station), not in the puzzle mechanics.

**Solving = Proving Understanding — More knowledge after?**
Test: After solving, does the solver know more about music? Specifically: can they name these six bands' biggest hits, their formation dates, their genres? No — the puzzle never required this knowledge. A solver who noticed all six words are band names knows six band names they may already have known. Net music knowledge gain: zero or trivially small.

**Blame the Player — Respect or resentment?**
Test: Is every clue fair? Five of six clues are unambiguous. Clue 1 is fair in the sense that ASIA is the correct answer to "the largest continent" — but it is unfair in the sense that EUROPE is also a valid continent and a valid band, and the puzzle does not signal this branch exists. A solver who chooses EUROPE and stalls at extraction may feel the puzzle trapped them. Blame the Player is marginal: fair in isolation, unfair as a silent trap.

**No Over-Scaffolding — Remove, still a puzzle?**
Remove the extraction table. What remains: six numbered definitions and the title "Name That Band." Without the table, a solver knows to name six bands but has no extraction rule. The puzzle collapses without the table — the table is required. This is not over-scaffolding; the table provides necessary space and instruction without solving the puzzle. PASS on this principle.

**Surprise the Answer — Guessable from topic?**
Is ANALOG guessable before solving? From "music definitions," the solver cannot guess the answer without working through the definitions. PASS.

**One Aha — Name the single aha.**
The candidate aha: "these dictionary definitions all describe band names." Operational test: is this aha required to solve the puzzle? No. Can a solver complete the extraction procedure without it? Yes. A required aha changes what the solver does next. This aha changes nothing about the extraction procedure. FAIL: the aha is present but inert.

**Reading Reward — Keyword search sufficient?**
Test: Can a solver complete the puzzle using only keyword searches ("largest continent," "Churchill quote mystery," "organ pumps blood," "birds of prey bald," "Massachusetts marathon city," "third largest US city Lake Michigan")? Yes, completely. No music knowledge engaged at any step. FAIL.

**No Computation Without Deduction:**
Test: Remove the "take letter at position N" instruction. Does the puzzle still have a deductive structure? Without the extraction rule, the solver has six band names and no mechanic. There is no deductive content in the definitions themselves — they are facts about words. The extraction rule adds a mechanical step but not a deductive one. The puzzle's only potential deductive moment (band-name recognition) is not in the mechanic. MARGINAL: not a computation problem, but the deduction is absent for different reasons.

**Snyder's Computer Test — 10-line script:**
```python
# Given: the definitions resolve to these words (a dictionary lookup step)
answers = [("ASIA", 1), ("ENIGMA", 2), ("HEART", 3),
           ("EAGLES", 4), ("BOSTON", 5), ("CHICAGO", 6)]
result = "".join(word[pos-1] for word, pos in answers)
print(result)  # ANALOG
```
The puzzle is fully solved by this script. No music knowledge, no band-name recognition, no aha. FAIL.

**Interlock, Not Independence — Any order, no advantage?**
Test: Can all six definitions be solved independently in any order with no advantage from solving others first? Yes — except for the ASIA/EUROPE ambiguity in clue 1. That ambiguity can only be resolved by running the extraction for all six and checking if the result is a valid word. This is one implicit interlock. All other definitions are fully independent. Partial: one designed interlock would be the threshold; this one is implicit and unannounced. MARGINAL.

**Verify the Last Mile — Trace letter by letter:**
Traced above. ASIA→A, ENIGMA→N, HEART→A, EAGLES→L, BOSTON→O, CHICAGO→G = ANALOG. PASS.

### Profile Additions

**Katz (full profile, adding to principles analysis):** The puzzle hunt contract is the overarching frame. In a music hunt, every puzzle implicitly promises music engagement. This puzzle breaks that promise. Katz's "are the mechanisms varied enough" fires: this is a sixth puzzle in a six-puzzle round, and if its mechanism (definitional lookup + extraction) is interchangeable with any other domain, the puzzle has no unique contribution to the round's variety. It occupies a music slot while being a vocabulary puzzle.

**Snyder (full profile, adding to principles analysis):** "The constructor's hand must be visible in every decision." The constructor made a genuinely skilled decision: chose six words whose Nth letters spell ANALOG, all of which are band names. That decision is not visible in the solve path. The constructor's insight is invisible to the solver. This means the puzzle was not finished — the insight was contained in the construction process and never translated into solving architecture.

**Young (full profile, adding to principles analysis):** The "world" Young starts with would be: a DJ's final broadcast, six songs, six bands, six names that are just words. That is a rich world. The puzzle should let the solver inhabit that world. The current puzzle lets the solver visit a vocabulary quiz adjacent to it. Young would rebuild the mechanic from the world up: what does it feel like to be Vee, writing band names as definitions? The solver should be doing what Vee did — recognizing that these words have dual identities. The current mechanic skips that experience entirely.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Instructions complete. Extraction table clean. CHICAGO letter-6=G confirmed (not O). |
| Solvability | 3 | Computer-solvable (Snyder's test FAIL). ASIA/EUROPE unannounced. Music knowledge irrelevant. The puzzle is too easy for its domain position — a music hunt puzzle that requires no music knowledge. |
| Elegance | 2 | Theme and mechanic not integrated. One Aha inert. Procedure not puzzle. Constructor's hand invisible. |
| Reading Reward | 1 | Keyword-search-sufficient (principle FAIL). Music knowledge not load-bearing. Riven Standard FAIL. Domain engagement: zero. |
| Fun | 3 | The concept is fun; the band-name recognition (if it occurs) is fun; the extraction is mechanical. Post-solve recognition does not compensate for during-solve absence of engagement. |
| Confirmation | 2 | Verify the Last Mile PASS (clean extraction). But confirmation requires closing the thematic loop: solver cannot confirm they found what the puzzle was about, because the band-name insight was never required. |
| **Total** | **16/30** | |

### Issues Identified (C7 — most rigorous pass)

1. **Snyder's Computer Test FAIL (critical).** A 3-line script solves the puzzle. No music knowledge, no aha. This is the definitive test that the puzzle's insight is not in the solve path.
2. **One Aha inert (critical).** "These words are band names" is the puzzle's concept but not its mechanic. A required aha that is not required is not an aha — it is a decorative concept.
3. **Reading Reward FAIL: keyword-search-sufficient.** Music domain engagement is zero.
4. **Riven Standard FAIL.** Music practitioners would not recognize this as music domain work.
5. **The ASIA/EUROPE silent branch violates Blame the Player.** The puzzle provides no mechanism for the solver to anticipate or handle this branch. An implicit disambiguation is not a designed one.
6. **Extraction does not earn its step (Young + principle cross-confirmation).** Positional extraction adds no semantic value and could be replaced without affecting the concept.
7. **Constructor's hand invisible to solver (Snyder).** The skilled construction choice (ANALOG from band-name Nth letters) is invisible because it was never translated into a solving constraint.
8. **CHICAGO extraction clarification note:** Length 7, position 6 = G (not the final letter O). Solvers who reach the last clue and assume "second-to-last letter" need explicit confirmation. Consider making this explicit in the table or renaming position numbering to clarify.

### Verdict: FAIL

---

## Per-Condition Summary Table

| Condition | Context | Clarity | Solvability | Elegance | Reading Reward | Fun | Confirmation | Total | Verdict |
|-----------|---------|---------|-------------|----------|----------------|-----|--------------|-------|---------|
| C0 — Bare baseline | "Expert reviewer" | 5 | 4 | 4 | 4 | 4 | 3 | **24** | PASS |
| C1 — Rubric only | + 6-dim rubric | 5 | 4 | 4 | 3 | 4 | 3 | **23** | PASS |
| C2 — Principles compact | + names + quotes | 5 | 4 | 3 | 2 | 4 | 3 | **21** | FAIL |
| C3 — Principles full | + operational tests | 5 | 4 | 2 | 2 | 3 | 3 | **19** | FAIL |
| C4 — Philosophy only | worldview, no lens | 5 | 4 | 2 | 2 | 3 | 3 | **19** | FAIL |
| C5 — Lens only | checklist, no biography | 5 | 4 | 2 | 2 | 3 | 2 | **18** | FAIL |
| C6 — Full profiles | current system | 5 | 4 | 2 | 2 | 3.3 | 2.3 | **18.7** | FAIL |
| C7 — Profile + Principles | everything | 5 | 3 | 2 | 1 | 3 | 2 | **16** | FAIL |

**Pass threshold: 22/30**

---

## Cross-Condition Analysis

### Score gradient

```
C0  24  PASS  ████████████████████████
C1  23  PASS  ███████████████████████
C2  21  FAIL  █████████████████████
C3  19  FAIL  ███████████████████
C4  19  FAIL  ███████████████████
C5  18  FAIL  ██████████████████
C6  18.7 FAIL ██████████████████▌
C7  16  FAIL  ████████████████
```

The gradient is strictly descending: more context → lower score. This is the inverse of the Age of Empires Puzzle II pattern (where richer context improved already-passing puzzles) and mirrors the Age of Empires Puzzle I pattern (where richer context correctly identified a failing puzzle).

### Inversion point

C0 and C1 both PASS. The inversion occurs at C2, the first condition that introduces named design principles. The principle introduction is what drives the puzzle below threshold — specifically, the principles surface that music knowledge is not required and that the band-name aha is not in the solve path.

This is a test of the system's sensitivity. The puzzle has a real structural flaw (the defining insight is not in the solve path). C0 and C1 do not see it because they lack the evaluative vocabulary to name it. C2 introduces the vocabulary and the score drops. C3 adds operational tests and the score drops further. C4 and C5 use different routes (design philosophy, lens checklists) to arrive at the same diagnosis.

### C0/C1 false positive analysis

C0 scores this puzzle 24/30 (PASS). The structural flaw — band-name recognition is not required — is mentioned in C0's Issue 2 ("The band-name layer is structurally invisible") but scores are not penalized proportionately. C0 awards Elegance 4/5 and Reading Reward 4/5 despite identifying the gap. This is the hallmark of a bare-prompt limitation: the reviewer notices the issue but lacks the framework to weight it heavily.

C1 (rubric only) scores 23/30, with Reading Reward dropping to 3 because the rubric's "genuine domain engagement" criterion puts direct pressure on the music-knowledge gap. The rubric alone moves the score 1 point and sharpens the Reading Reward diagnosis, but not enough to flip the verdict.

### The decisive principle: Snyder's Computer Test

The score drops most sharply when the Computer Test is applied (C3, C7). When the reviewer explicitly writes a 3-line script that solves the puzzle, the structural absence of the aha becomes undeniable. This principle does more work on this puzzle than any other single principle. No profile lens, no philosophy section, no rubric criterion directly operationalizes "can a script solve this?" — which is why the principle adds unique signal beyond full profiles.

### C6 vs. C7 delta

Full profiles (C6) score 18.7/30. Full profiles + principles (C7) score 16/30. The delta is driven by:
- Reading Reward: C6 scores 2, C7 scores 1 (Computer Test + Riven Standard together push it to the floor)
- Solvability: C6 scores 4, C7 scores 3 (Computer Test directly impacts solvability — if a script solves it, the puzzle is too easy for its domain)
- Confirmation: C6 scores 2.3, C7 scores 2 (minor)

The delta confirms the prior study's finding: Snyder's Computer Test adds unique signal not subsumed by profiles.

### Named frameworks introduced by condition

| Condition | Frameworks named |
|-----------|-----------------|
| C0 | 0 (generic: "structural weakness," "unambiguous") |
| C1 | 0 (rubric dimension names only) |
| C2 | 8 (Riven Standard, One Aha, Snyder's Computer Test, Blame the Player, Interlock, Surprise the Answer, Reading Reward, Solving = Understanding) |
| C3 | 11 (all principles by name, with operational test results) |
| C4 | 6 (Katz's "contract," Snyder's "procedure," Young's "mechanic IS the theme," "inevitable answer," "constructor's hand visible") |
| C5 | 5 (Snyder's "puzzle or procedure," Young's "extraction earning its step," "arbitrary vs. inevitable," "flavor substituting for mechanic") |
| C6 | 7 (Katz's "contract," "slot justification," Snyder's "theme vs. structure," "not finished," Young's "mechanic IS the theme," "naming the experience," "inevitable answer") |
| C7 | 13 (all 11 principles + Katz/Snyder/Young frameworks) |

### Actionable fixes identified

| Condition | Critical fixes | Specific, actionable |
|-----------|---------------|---------------------|
| C0 | 2 | Moderate specificity |
| C1 | 3 | Structural specificity |
| C2 | 5 | Named, moderate |
| C3 | 5 | Named + operationally tested |
| C4 | 5 | Philosophically grounded |
| C5 | 5 | Lens-specific, procedural |
| C6 | 7 | Domain-specific, actionable |
| C7 | 8 | Most comprehensive, includes CHICAGO extraction clarification |

---

## Key Findings — P6 Specific

### Finding 1: P6 is a false positive under bare conditions

C0 and C1 both PASS (24/30 and 23/30) a puzzle with a genuine structural flaw. The flaw — the defining insight is not in the solve path — is visible to both reviewers but is not weighted proportionately. Bare and rubric-only reviewers see the issue but lack the framework to treat it as disqualifying.

This is the puzzle-hunt equivalent of a logic puzzle that looks elegant but has an unintended second solution path. The surface is clean; the architecture is broken.

### Finding 2: The Computer Test is the decisive principle for this puzzle type

Across all principle-enabled conditions (C2, C3, C7), the score drop is most concentrated in the dimensions that the Computer Test directly implicates (Reading Reward, Elegance, Solvability). The Computer Test uniquely operationalizes "is the aha required?" — which is the central question for any puzzle whose defining insight could be bypassed.

For puzzle types where the key insight is not mechanically required (definitional puzzles, wordplay puzzles with unforced aha moments), the Computer Test is the highest-value principle.

### Finding 3: The ASIA/EUROPE ambiguity is a structural diagnostic, not a defect

Every condition flags the ASIA/EUROPE branch. C0 and C1 treat it as a solvability defect. C3 treats it as a Blame the Player marginal failure. C6 (Snyder's profile) identifies it as potentially the puzzle's most interesting structural element — two valid answers, only one producing a valid extraction, creating lateral-information interlock.

The ambiguity is not a flaw to fix; it is an unfinished feature. A redesign that makes band-name recognition required would either: (a) use the ASIA/EUROPE ambiguity as the designed interlock (both are bands; only ASIA's extraction character works in context), or (b) resolve the ambiguity by making the definition uniquely ASIA in the band-knowledge domain rather than the geographical domain.

This is the most sophisticated diagnosis produced by the study. It required C6 (Snyder's full profile, with his attention to the "designed entry point") to surface it.

### Finding 4: C6 over-penalizes relative to C3

C6 (full profiles) produces 18.7/30 vs. C3's 19/30. The scores are nearly identical, but C6 generates more domain-specific and actionable diagnoses (7 fixes vs. 5, with higher specificity). This mirrors the master study finding: full profiles produce calibrated assessments with better actionability, even when the final score is similar to a principles-only review.

### Finding 5: The puzzle's score history (29/30 in PUZZLES.md) represents a different evaluation

The WAVELENGTH CLAUDE.md records P6 tested at 29.0/30. That evaluation used a different standard — likely a warm testing pass by domain-knowledgeable reviewers who noticed the band-name pattern and enjoyed the aha. The ablation study, particularly C3-C7, reveals that the 29/30 score reflects solver satisfaction when the aha is noticed, not structural quality of the aha as a required element.

A puzzle can be delightful when the aha occurs and structurally incomplete when the aha is optional. C0-C1 reflect the former. C3-C7 reflect the latter.

---

## Recommended Fix (from C7 analysis)

The puzzle needs the band-name recognition to be mechanically required. The most direct fix:

**Option A — Make the definitions work only with band knowledge.** Rewrite the definitions so they can only be resolved by knowing band names. For example: clue 1 could be "Rock quartet featuring John Wetton and Geoff Downes. Named for a continent." This forces the solver to identify the band before writing the word. The dictionary layer becomes the confirmation layer, not the identification layer.

**Option B — Use the ASIA/EUROPE ambiguity as the designed interlock.** Keep the current definitions. Add to the instructions: "Some definitions fit more than one word. Only band names matter here." This makes the band-name constraint explicit and required for disambiguation. The solver must now think: "ASIA or EUROPE? Both are bands. The extraction will tell me which — I need to try both and see which one produces a valid final answer." Now the band-name recognition is required (you must verify each answer is a band name to know which branch to take) and the interlock is designed, not accidental.

**Option B preferred**: It preserves the puzzle's current elegant definitions, activates the ASIA/EUROPE interlock as a feature, and requires exactly the music-domain knowledge the puzzle's slot promises.

---

## Comparison with Prior Study (Age of Empires Puzzles I–III)

| Pattern | AoE I | AoE II | AoE III | P6 Wavelength |
|---------|-------|--------|---------|---------------|
| True condition (ground truth) | FAIL | PASS | PASS | FAIL (structural) |
| C0 verdict | FAIL ✓ | PASS ✓ | PASS ✓ | PASS ✗ |
| C6 verdict | PASS ✗ | PASS ✓ | PASS ✓ | FAIL ✓ |
| C7 verdict | FAIL ✓ | PASS ✓ | PASS ✓ | FAIL ✓ |
| Key finding | C7 harshest | C5 catches ASCII error | Consistent across C0–C7 | C2+ correctly identify structural flaw; C0/C1 false positive |

**New insight:** P6 introduces a puzzle type not seen in the prior study — a puzzle that is correct (verifiable answer, clean extraction, no errors) but structurally incomplete (the organizing insight is not in the solve path). This type produces **false positives at low context levels** and **correct FAIL verdicts at principle-enabled levels**. The Computer Test is the decisive discriminator.

This complements the prior study's finding that C5 (lens only) uniquely caught a visual/structural error in AoE II. The new finding: **C2+ correctly catches the "insight not in solve path" failure mode** — and this finding is robust across C2 through C7.
