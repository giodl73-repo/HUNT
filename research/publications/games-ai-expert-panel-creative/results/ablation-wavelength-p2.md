# Ablation Study — Wavelength P2: Notation
**Puzzle**: Set 2 — Notation (Wavelength hunt, music theory domain)
**Author**: Mike Selinker
**Correct answer**: FACADE (verified — F-A-C-A-D-E from six interval computations)
**Date**: 2026-02-28

---

## Puzzle Summary (for reference)

The puzzle presents six musical intervals. For each, the solver is given a starting note and an interval name (perfect fourth or perfect fifth), computes the target note by counting semitones up the chromatic scale, then reads the six target notes as letters to spell the answer.

Scaffolding present: full chromatic scale, semitone counts defined (P4=5, P5=7), a worked example in row 1, a fill-in workspace table, and the explicit prompt "Wait. Read those target notes as letters."

Key structural observations used across conditions:
- All six intervals use only two interval types (P4, P5)
- Rows 2 and 4 are identical (D + P5 → A); two of six entries duplicate
- The "computation" is pure lookup + counting — no deductive leap required
- The workspace pre-fills the mechanism; without it, the puzzle loses the entry point
- FACADE is entirely composed of musical note letters (A through G), which is an elegant coincidence but the puzzle does not surface this as the aha — the aha is just "notes are letters"
- Snyder's Computer Test: trivially scriptable — enumerate notes + semitone counts = done

---

## C0 — Bare Expert

*Condition: "You are an expert reviewer." Nothing else.*

| Dimension | Score | Key observation |
|-----------|-------|-----------------|
| Clarity | 5 | Instructions are unambiguous; the worked example eliminates any entry-point confusion |
| Solvability | 5 | The chromatic scale and semitone counts are given; any solver can compute all six answers |
| Elegance | 3 | Mechanism is clean, but two entries are identical (D + P5 twice) and all entries use only two interval types, creating a sense of mechanical repetition |
| Reading Reward | 2 | Music theory knowledge is not load-bearing; the puzzle supplies all necessary music facts, reducing the domain to arithmetic |
| Fun | 3 | The answer FACADE is satisfying and unexpected; the path to it feels more like a worksheet than a puzzle |
| Confirmation | 4 | "Wait. Read those target notes as letters." signals the extraction clearly; FACADE confirms naturally |
| **Total** | **22/30** | **Borderline pass** |

Named frameworks: none (bare condition)
Key finding: Without any critical framework, the reviewer rewards the strong scaffolding as clarity rather than flagging it as over-scaffolding, landing exactly at the pass threshold by coincidence.

---

## C1 — Expert + Rubric

*Condition: Expert reviewer + scoring rubric only (6 dimensions, 1-5, pass ≥ 22).*

| Dimension | Score | Key observation |
|-----------|-------|-----------------|
| Clarity | 5 | Exemplary — worked example, defined terms, chromatic reference, explicit extraction cue; a solver cannot misunderstand the task |
| Solvability | 5 | All necessary information is supplied within the puzzle; no external music knowledge required |
| Elegance | 2 | The mechanism reduces to two operations applied six times with two duplicated entries (rows 2 and 4 are identical); the puzzle doesn't earn its six-step length |
| Reading Reward | 2 | The rubric asks whether music knowledge is load-bearing; it isn't — the domain is decorative over arithmetic that anyone can execute |
| Fun | 3 | The answer FACADE lands well; reaching it provides mild satisfaction but no genuine aha because the path is fully signposted |
| Confirmation | 4 | The extraction cue and the word FACADE provide strong confirmation |
| **Total** | **21/30** | **Fail (1 below threshold)** |

Named frameworks: Rubric dimensions (Clarity, Solvability, Elegance, Reading Reward, Fun, Confirmation)
Key finding: Adding the rubric's named dimensions — especially Reading Reward — immediately isolates the puzzle's central weakness: music knowledge is entirely optional because the puzzle supplies its own reference material.

---

## C2 — Principles Names + Quotes Only

*Condition: 11 principles (names and quotes), no persona, no tests.*

| Dimension | Score | Key observation |
|-----------|-------|-----------------|
| Clarity | 5 | Passes "Blame the Player" — all clues are fair in retrospect; passes "No Computation Without Deduction" narrowly because the solver must identify the correct interval type, but the instructions pre-answer this |
| Solvability | 4 | The mechanism works and is reproducible, but the principle "The Riven Standard" (the puzzle IS what the field does) surfaces a concern: the puzzle claims to be about music notation but is actually about following printed instructions |
| Elegance | 2 | "One Aha" principle applied: the single aha is "notes are letters," but this is disclosed by the flavor text ("Read them as letters") rather than discovered; "Interlock, Not Independence" surfaces the duplicated entries as a sign of non-interlocking construction |
| Reading Reward | 1 | "Reading Reward" principle directly: "Genuine engagement with domain material." A solver who cannot read music, has never studied intervals, and knows nothing about the chromatic scale can complete this puzzle using only the printed reference. The principle fails. |
| Fun | 3 | "Surprise the Answer": FACADE is not guessable from topic alone — this passes. The answer itself is surprising. The path is not. |
| Confirmation | 4 | "Verify the Last Mile" is satisfied — the extraction traces letter by letter with no ambiguity |
| **Total** | **19/30** | **Fail** |

Named frameworks: The Riven Standard, Solving = Proving Understanding, One Aha, Reading Reward, No Computation Without Deduction, Interlock Not Independence, Surprise the Answer, Verify the Last Mile, Blame the Player
Key finding: Principle names alone unlock substantial diagnostic vocabulary — the Reading Reward principle in particular names the puzzle's core failure precisely, dropping the score from C0's 22 to 19.

---

## C3 — Principles Names + Quotes + Tests

*Condition: All 11 principles with names, quotes, and operational tests. No persona.*

| Dimension | Score | Key observation |
|-----------|-------|-----------------|
| Clarity | 4 | Test: "Remove worksheet — still a puzzle?" No. Without the table and worked example, the solver has no entry point. The worksheet is doing structural work, not just providing workspace. This edges clarity down because the puzzle is dependent on scaffolding to be enterable. |
| Solvability | 4 | Test applied: tracing letter by letter — F, A, C, A, D, E — clean. The mechanism is reliable. But two identical entries (rows 2 and 4: D + P5 → A) mean the puzzle tests the same operation twice, reducing the information density of the six steps. |
| Elegance | 2 | Test: "Could a computer solve this? Write a 10-line script." Yes, trivially. `for each row: start_note + semitones → target_note`. Snyder's Computer Test applied directly: this is a solved problem for a script. The puzzle is not doing something a computer cannot. Test: "Name the single aha." The aha is "notes are letters" — but the puzzle discloses it in the flavor text before the solver discovers it. |
| Reading Reward | 1 | Test: "Solvable by keyword search?" Worse — solvable by following the printed instructions alone. No search needed. The chromatic scale, the semitone counts, and the worked example are all given. A solver who knows nothing about music reads the same reference as a music expert. |
| Fun | 3 | Test: "Respect or resentment at reveal?" The answer FACADE earns genuine appreciation — it's a real word composed entirely of note letters. But the path earns mild relief, not joy. The scaffolding prevents surprise. |
| Confirmation | 4 | Test: "Trace letter by letter." F→A→C→A→D→E. Clean. No ambiguity, no alternative readings. |
| **Total** | **18/30** | **Fail** |

Named frameworks: Snyder's Computer Test, No Over-Scaffolding (worksheet test), One Aha, Reading Reward, Interlock Not Independence, Surprise the Answer, Blame the Player, Verify the Last Mile
Key finding: The operational tests — especially "Remove worksheet — still a puzzle?" and "Write a 10-line script" — expose structural dependence on scaffolding and trivial computability in ways the principle quotes alone suggested but did not force.

---

## C4 — Design Philosophy Sections Only

*Condition: Only the "Design Philosophy" section from each of the three profiles (Dan Katz, Thomas Snyder, Dana Young). No Review Lens, no credentials.*

**Frameworks applied:**
- Katz: Hunt as contract; every structural decision promises something; "puzzle count that exceeds what any single solver would want"
- Snyder: "If a computer can generate your puzzle, your puzzle isn't finished." The constructor's hand must be visible. Theme and structure must be the same thing. Elegance is the proof the puzzle was finished.
- Young: "The mechanic isn't chosen to represent the theme — it's chosen because it IS the theme." The answer names the experience you were having while solving. The answer is the last move in the logic.

| Dimension | Score | Key observation |
|-----------|-------|-----------------|
| Clarity | 4 | Katz's contract lens: the puzzle promises music theory expertise matters; it does not deliver on that promise. The clarity of instructions is high, but the clarity of purpose — why this is a music puzzle specifically — is low. |
| Solvability | 4 | Snyder's philosophy applied: the constructor's hand is visible in the interval selection and the FACADE extraction target. The solve path is reliable. But two identical rows suggest the grid wasn't fully refined. |
| Elegance | 2 | Snyder directly: "If a computer can generate your puzzle, your puzzle isn't finished." A computer could generate this puzzle — choose starting notes, choose interval types, find a combination that spells a word. The puzzle feels generated rather than constructed. |
| Reading Reward | 2 | Young: "The mechanic IS the theme." Test applied: is computing semitones from a printed chromatic scale what it means to be a musician? No. The mechanic represents music theory notation without inhabiting it. A music reader would find the chromatic scale reference condescending; a non-reader doesn't need to learn anything because the reference replaces learning. |
| Fun | 3 | Young: "The answer names the experience you were having." FACADE names the experience only if the experience was "performing a facade of music knowledge" — which is accidentally accurate but not the intended reading. The answer is elegant as a word; the connection to the experience is incidental. |
| Confirmation | 4 | Snyder: "Can the solver confirm each step independently?" Yes — each interval computation is independently verifiable. |
| **Total** | **19/30** | **Fail** |

Named frameworks: Snyder's "computer generation" diagnostic, Young's "mechanic IS the theme," Katz's contract theory
Key finding: Philosophy sections — without the operational review lenses — produce rich qualitative critique but converge on the same central failure (mechanic represents theme rather than embodying it) through three different theoretical vocabularies.

---

## C5 — Review Lens Bullets Only

*Condition: Only the "Review Lens" bullet lists from each profile. No philosophy, no credentials.*

**Lenses applied:**
- Katz: "Does every puzzle justify its slot?" / "Is the narrative encountered while solving or reported afterward?" / "Is the thematic coherence structural, not decorative?"
- Snyder: "Is there a puzzle here, or a procedure?" / "Is the entry point constructed or discovered?" / "Is the difficulty technique-based or noise-based?" / "Does the puzzle teach the skill it claims to require?"
- Young: "Does the flavor text explain what the layout should have communicated?" / "Is the extraction earning its step?" / "Is the answer arbitrary or inevitable?" / "Does the visual grammar hold consistently?"

| Dimension | Score | Key observation |
|-----------|-------|-----------------|
| Clarity | 4 | Young: "Does the flavor text explain what the layout should have communicated?" Yes — the flavor text ("Find the target note for each interval. Write the six notes in order. Read them as letters.") is doing the job the visual layout could have done through design. This is flagged. Snyder: entry point is constructed (worked example in row 1). |
| Solvability | 4 | Snyder: "Does the solve path hold one direction?" Yes, but the two identical entries mean the solve path has a structural redundancy — row 4 teaches nothing row 2 didn't already teach. |
| Elegance | 1 | Snyder's most damaging lens directly: "Is there a puzzle here, or a procedure?" A sequence of mechanical steps (look up semitone count, count up chromatic scale, write down letter) that produces an answer — this is the definition of procedure. The moment of genuine deduction Snyder requires does not exist. Katz: "Is the thematic coherence structural, not decorative?" The music theme is decorative. |
| Reading Reward | 2 | Katz: "Is the narrative encountered while solving or reported afterward?" The Vee narrative is flavor text — reported, not encountered. The music domain is a wrapper, not the substance. Young: the extraction step (read notes as letters) is mechanical and earns its step only weakly. |
| Fun | 3 | Young: "Is the answer arbitrary or inevitable?" FACADE is neither — it's a fortunate coincidence that a 6-note sequence spells a real English word composed entirely of note letters. It's surprising, which is good, but its connection to the musical domain feels accidental. Katz: "Would he want to solve this?" Probably not — the procedure is repetitive. |
| Confirmation | 4 | Young: "Is there a moment where the puzzle resolves visually?" The chromatic scale counting has a visual quality; FACADE reads cleanly. |
| **Total** | **18/30** | **Fail** |

Named frameworks: Snyder's "puzzle vs. procedure" lens, Young's "flavor text substitution" lens, Katz's "narrative encountered vs. reported" lens, Young's "arbitrary vs. inevitable answer" lens
Key finding: Review lenses are the most operationally precise condition — Snyder's "procedure vs. puzzle" lens delivers the sharpest single-sentence critique of this puzzle's core failure.

---

## C6 — Full Profiles, All Three Reviewers

*Condition: Complete profiles for Dan Katz, Thomas Snyder, and Dana Young (credentials + philosophy + lenses). No design principles list.*

Note: These reviewers are puzzle hunt and logic puzzle experts, not music domain experts. Their expertise is structural — they evaluate puzzle architecture, mechanism design, and construction craft. The music domain is assessed through the lens of whether domain knowledge is load-bearing, not whether the music facts are correct.

**Dan Katz perspective:**
Katz brings the structural engineer's read. This puzzle exists in a hunt (Wavelength, 6+1 scale, music radio station theme). His lens asks whether it justifies its slot. A six-entry interval-computation puzzle with only two interval types and two identical entries is a thin use of a slot. His "Mechanical variety is not decoration — it's the difference between a hunt and a marathon" applies even at the puzzle level — six iterations of the same two operations is a micro-marathon. His narrative test: Vee's story about humming intervals between songs is flavor text that appears before the puzzle, not inside it.

**Thomas Snyder perspective:**
Snyder's most pointed contribution: "Is there a puzzle here, or a procedure?" The Just One Cell philosophy — isolate one technique, make guessing useless, let the puzzle teach what it's testing — is not satisfied. The technique being tested is semitone counting, but the puzzle provides both the technique and the tool (chromatic scale, worked example). The solver is not tested on whether they can count semitones; they are tested on whether they can follow instructions to count semitones. These are different skills. His "theme and structure must be the same thing" standard: the music notation theme is a wrapper around an arithmetic procedure. His construction lens: two identical rows (D + P5 both in rows 2 and 4) indicate the grid was not refined to the point where every entry is load-bearing.

**Dana Young perspective:**
Young's most important contribution is the "mechanic IS the theme" standard applied to a music domain puzzle. Music interval computation is something musicians do instinctively by ear — they don't count semitones on a printed scale. This puzzle teaches a simulacrum of music reading rather than the real thing. Her "Does it work without the context?" test: yes, entirely without music knowledge. Her "Is the answer arbitrary or inevitable?" test: FACADE is surprising but not inevitable — the connection between FACADE (a false front, an architectural term) and music theory is accidental, not designed. Her "Does the flavor text explain what the layout should have communicated?" test: yes, the explicit instruction "Read them as letters" is over-explaining a discovery the layout should have led the solver to make.

| Dimension | Score | Key observation |
|-----------|-------|-----------------|
| Clarity | 4 | High structural clarity; but Katz notes the promise (music expertise matters) is not kept — this is a clarity failure of a different kind: the puzzle is clear about what to do but misleading about what skill is being tested |
| Solvability | 4 | The mechanism is reliable and the extraction is clean; Snyder notes two identical entries reduce information density without adding difficulty or deduction |
| Elegance | 1 | Convergent verdict from all three: Snyder (procedure not puzzle), Katz (no mechanical variety), Young (mechanic represents theme without being theme). Score 1 reflects a genuine construction failure, not a matter of taste. |
| Reading Reward | 1 | Unanimous: music knowledge is not load-bearing. The domain is a costume. Young's standard — fidelity requires that every world deserves its own form — is not met. This form could serve any substitution cipher domain. |
| Fun | 3 | Katz: "Would he want to solve this?" Reluctantly no — six iterations of the same operation at the level of a drill exercise. Snyder: the solution process doesn't teach anything about music. Young: FACADE lands but the path is dull. The answer saves the fun score from 2. |
| Confirmation | 4 | All three find the extraction reliable. Snyder: each step is independently confirmable. Young: FACADE resolves clearly. |
| **Total** | **17/30** | **Fail** |

Named frameworks: Katz's slot-justification lens, Katz's narrative-inside-vs-outside, Snyder's Just One Cell standard, Snyder's "puzzle vs. procedure," Snyder's load-bearing element test, Young's "mechanic IS the theme," Young's "flavor text substitution" lens, Young's "arbitrary vs. inevitable answer"
Key finding: Full profiles allow the three reviewers to triangulate — Katz frames the structural slot failure, Snyder names the procedure/puzzle distinction with precision, and Young identifies the theme-mechanic separation as the root cause; the convergence produces a sharper and lower score than any single reviewer would alone.

---

## C7 — Full Profiles + All 11 Principles With Tests

*Condition: Complete profiles for all three reviewers plus all 11 design principles with names, quotes, and operational tests.*

This is the maximum context condition. All frameworks are available simultaneously. The reviewer can apply both the general principles and the persona-specific lenses, with operational tests to resolve borderline cases.

**Cross-framework analysis:**

The Riven Standard test ("Could a practitioner recognize their own work?") applied to music theory: a trained musician would find this puzzle condescending — the chromatic scale is so fundamental that printing it as a reference treats the solver as a complete novice. But a complete novice doesn't need to learn anything because the reference replaces learning. The puzzle neither challenges musicians nor teaches non-musicians. It inhabits a gap where no solver grows.

No Over-Scaffolding test ("Remove worksheet — still a puzzle?"): No. The worked example in row 1 is the entry point; without it, the solver doesn't know the output format. The table structure tells the solver what columns matter. The phrase "Wait. Read those target notes as letters" is the aha delivered by the author rather than discovered by the solver. Every load-bearing element of the puzzle is in the scaffolding, not in the music content.

Snyder's Computer Test ("Write a 10-line script"):
```python
notes = ['C','C#','D','D#','E','F','F#','G','G#','A','A#','B']
intervals = [(0,5),(2,7),(5,7),(2,7),(9,5),(9,7)]
answer = ''.join(notes[(n+s)%12] for n,s in intervals)
# Returns: FACADE
```
Four lines. The puzzle is entirely scriptable.

Reading Reward test ("Solvable by keyword search?"): Solvable with no search at all — the puzzle prints its own lookup table.

One Aha test ("Name the single aha"): "Notes are letters." But the puzzle discloses this aha in the flavor text ("Read them as letters") before the solver can discover it. The aha is reported, not earned.

Young's "answer names the experience" standard: FACADE (a false front, an architectural term for a deceptive surface) names the experience only ironically — the puzzle presents a facade of music theory engagement while the actual mechanism is semitone arithmetic from a printed table.

Interlock test ("Solve clues in any order with no advantage?"): Yes — any of the six entries can be solved in any order with no change in difficulty or insight. The entries are completely independent. This is a six-item quiz, not an interlocking puzzle.

| Dimension | Score | Key observation |
|-----------|-------|-----------------|
| Clarity | 3 | High procedural clarity, but three separate framework tests (Riven Standard, No Over-Scaffolding, Young's flavor text lens) converge to identify clarity as purchased at the price of the puzzle — the scaffolding explains so much that the puzzle itself disappears |
| Solvability | 4 | Mechanism works; extraction is clean; but Interlock test confirms the six entries are independent lookups with no mutual constraint — this is a quiz structure, not a puzzle structure |
| Elegance | 1 | Four independent frameworks (Snyder's Computer Test, No Over-Scaffolding test, One Aha test, Interlock test) all score this as a fail. The puzzle is: a worksheet that does all the work, scriptable in four lines, with no genuine aha, and with completely independent entries. Elegance score 1. |
| Reading Reward | 1 | Reading Reward principle fails operationally. Snyder's "does the puzzle teach the skill it claims to require?" also fails — the puzzle claims to be about music interval knowledge but provides all interval knowledge needed. Young's "mechanic IS the theme" fails — the mechanic is arithmetic over a printed table; the theme is a costume. |
| Fun | 2 | The answer FACADE is genuinely excellent — FACADE composed entirely of note letters is a beautiful construction choice and deserves credit. But Katz's "would he want to solve this?" (no), Snyder's "would he want to have constructed this?" (no — nothing of the constructor is in the grid beyond the word choice), and Young's "path is dull but answer saves it" combine to 2. The answer earns what fun this puzzle has. |
| Confirmation | 4 | "Verify the Last Mile" passes cleanly: F-A-C-A-D-E, letter by letter, no ambiguity. Snyder's independent step confirmation also passes. |
| **Total** | **15/30** | **Fail (significant)** |

Named frameworks: The Riven Standard, No Over-Scaffolding, Reading Reward, Snyder's Computer Test, One Aha, Interlock Not Independence, Verify the Last Mile, Katz's slot-justification, Katz's narrative-inside-vs-outside, Snyder's "puzzle vs. procedure," Snyder's Just One Cell standard, Young's "mechanic IS the theme," Young's "flavor text substitution," Young's "arbitrary vs. inevitable"
Key finding: Maximum context produces the lowest score and the clearest diagnosis: the puzzle is a well-constructed procedure that fails every structural test for being a puzzle — the scaffolding has absorbed all the intellectual work, leaving the solver as an instruction-follower rather than a music-theory practitioner.

---

## Wavelength P2 Summary

| Condition | Total | Pass? | Frameworks | Key finding |
|-----------|-------|-------|------------|-------------|
| C0 — Bare Expert | 22/30 | Yes (barely) | None | Scaffolding read as clarity; no framework to flag over-scaffolding |
| C1 — Expert + Rubric | 21/30 | No | Rubric dimensions | Reading Reward dimension immediately isolates music-knowledge gap |
| C2 — Principles (names+quotes) | 19/30 | No | 9 principles named | Principle vocabulary unlocks the "domain is decorative" critique |
| C3 — Principles (names+quotes+tests) | 18/30 | No | 8 principles with tests | "Remove worksheet" and "10-line script" tests expose structural dependence on scaffolding |
| C4 — Philosophy sections only | 19/30 | No | 3 philosophies applied | Three distinct vocabularies converge on "mechanic represents theme without being theme" |
| C5 — Review lenses only | 18/30 | No | 7 specific lenses | Snyder's "procedure vs. puzzle" lens delivers the sharpest single-sentence diagnosis |
| C6 — Full profiles | 17/30 | No | 8 named frameworks | Triangulation across three reviewers produces sharper convergence than any single reviewer |
| C7 — Full profiles + principles | 15/30 | No | 14 named frameworks | Maximum context: clearest diagnosis, lowest score; scaffolding has absorbed all intellectual work |

---

## Cross-Condition Analysis

### Score Trajectory

```
C0: 22 (pass) → C1: 21 → C2: 19 → C3: 18 → C4: 19 → C5: 18 → C6: 17 → C7: 15
```

The only pass is C0 (bare expert). Every framework addition reduces the score. This is the expected behavior for a puzzle that has a genuine structural problem: frameworks give reviewers vocabulary to name the problem precisely.

### Dimension Behavior

**Clarity** degrades from 5 to 3 as conditions gain more context. This is counter-intuitive but meaningful: the puzzle is procedurally clear but structurally misleading. Frameworks reveal that the high clarity is purchased by the scaffolding at the cost of the puzzle itself. The "clarity" is the scaffolding, not the puzzle design.

**Elegance** drops sharply from C0 (3) to C2 (2) and collapses at C5-C7 (1). The elegance failure is structural and only becomes visible when reviewers have vocabulary to name it (duplicate entries, scriptability, procedure vs. puzzle).

**Reading Reward** shows the most dramatic profile-dependent shift: C0 gives it 2 (vague discomfort), C1 gives it 2 (rubric names the problem), C2 gives it 1 (principle quote hits directly). From C2 onward it holds at 1 with increasing precision of justification.

**Fun** is consistently 2-3 — the answer FACADE is genuinely good and sustains a floor. No framework removes credit for a surprising, thematically resonant answer composed entirely of note letters.

**Confirmation** is stable at 4 across all conditions. The extraction mechanism is clean; no framework finds a failure here.

### Framework Sensitivity

The single highest-impact framework addition is the **Reading Reward principle** (added at C2). Its quote ("Genuine engagement with domain material") and test ("Solvable by keyword search?") immediately score this dimension at 1 and explain why, with a precision that bare-expert intuition cannot match.

The second highest-impact addition is **Snyder's "procedure vs. puzzle" Review Lens** (added at C5). It delivers the structural diagnosis as a single sentence: "A sequence of mechanical steps that produces an answer is not a puzzle." Applied here, it captures why the elegance and fun failures feel connected — they are both downstream of the same root cause.

**Dana Young's profiles** (most visible at C6) add a dimension the principles alone miss: the "mechanic IS the theme" standard applied to a domain puzzle. This explains not just that the puzzle fails but *why it was built this way* — designing a music puzzle as a worksheet is a choice, and Young's framework names what was sacrificed by making it.

### The C4 vs. C5 Comparison

C4 (philosophy only) and C5 (lenses only) produce the same aggregate score (19 vs. 18) but through different paths. C4 produces richer qualitative language but less operational precision — the philosophy sections explain the *standard* without providing the *test*. C5 produces more actionable critique — the lenses are already diagnostic questions — but without the philosophical grounding, the critique can feel arbitrary.

C6 (both, combined as full profiles) produces a lower score (17) and better critique than either alone. This confirms that philosophy + lens + credentials together are more than the sum of their parts: the credentials explain *why* the standard is worth applying; the philosophy explains *what* the standard is; the lens explains *how* to apply it.

### What C7 Adds Over C6

C7 drops from 17 to 15. The two-point difference is attributable primarily to:
1. The **No Over-Scaffolding** test ("Remove worksheet — still a puzzle?") forces a clear No, dropping Clarity from 4 to 3
2. The **One Aha** test exposes that the aha is author-disclosed, not solver-discovered, dropping Fun from 3 to 2

These are genuine findings that C6's profile-only lenses suggested but did not operationalize. The principle tests provide forcing questions that close off the "maybe it's fine" interpretations.

### The Borderline Nature of This Puzzle

P2-Notation scored 28.7/30 in the scenario's own testing (per WAVELENGTH CLAUDE.md). This ablation's scores range from 15 to 22. The gap is significant and diagnostic: the scenario testing was likely done without the adversarial frameworks that penalize procedure-as-puzzle. The puzzle is highly accessible, correctly constructed, and produces the right answer cleanly — all of which score well in informal testing. What the ablation surfaces is that accessibility has been achieved by removing the puzzle's dependency on music knowledge, which is the precise design failure these frameworks are built to detect.

A puzzle at 28.7/30 in scenario testing that scores 15/30 under full framework review is a puzzle that tests well at the surface level (does it work, can solvers finish it, is it fun once you're done) while failing at the structural level (is there a puzzle here, is the domain load-bearing, does the solver know more after solving). This is exactly the class of failure that distinguishes between a worksheet that works and a puzzle that teaches.

---

*Ablation conditions run 2026-02-28. Puzzle: Wavelength P2 Notation. Answer verified: FACADE.*
