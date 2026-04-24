# Ablation Study: Reviewer Context Depth vs. Evaluation Quality
## Puzzle: P5-gallery-label.md (Grand Larceny scenario)
## Date: 2026-02-28

---

## Setup

**Puzzle:** EXHIBIT E — GALLERY LABEL (RECOVERED)
**Domain:** Noir mystery. Braille decode — recovered gallery label from stolen painting frame.
**Mechanic:** Six cells in a Unicode braille grid are asterisked. Decoded in order they spell the answer.
**Correct answer:** MOTIVE (verified: ⠍=M, ⠕=O, ⠞=T, ⠊=I, ⠧=V, ⠑=E)
**Format:** Markdown file with Unicode braille characters (⠁ etc.) in an ASCII box-drawing table, plus a reference alphabet, plus an Investigator's Notes table that pre-maps each asterisked cell to its row/col position and braille character.

**Scoring rubric:** Clarity, Solvability, Elegance, Reading Reward, Fun, Confirmation — 1–5 each, /30 max. Pass threshold ≥ 22.

**Design principles tested:**
- The Riven Standard: "The puzzle IS what the field does."
- Solving = Proving Understanding
- Blame the Player
- No Over-Scaffolding
- Surprise the Answer
- One Aha
- Reading Reward
- No Computation Without Deduction
- Snyder's Computer Test
- Interlock, Not Independence
- Verify the Last Mile

**Key technical risk identified in puzzle:** Unicode braille characters (⠁, ⠙, ⠕, etc.) may fail to render in some markdown viewers, print fonts, and PDF exporters. Monospace alignment in the grid depends on braille glyphs being treated as single-width characters — behavior that varies by platform, font, and renderer.

---

## Condition C0: Minimal ("You are an expert reviewer.")

**Context given:** "You are an expert reviewer."

**Review:**

The puzzle presents a braille-themed decode where six asterisked cells in a gallery label must be read in order using a reference alphabet. The conceit — a tactile accessibility label from a stolen painting's frame — is strong. The answer MOTIVE lands well.

The Investigator's Notes table at the bottom maps each asterisked cell to its row/col position AND provides the braille character directly. This means the actual solve path bypasses the grid entirely: read the character from the notes table, look it up in the reference alphabet. The grid itself becomes decorative.

The Unicode braille rendering is a real concern in practice — depending on viewer, the characters ⠍ ⠕ ⠞ ⠊ ⠧ ⠑ may display as boxes, question marks, or fall back to invisible glyphs. The puzzle does not address this risk.

**Scores:**

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | Instructions clear; braille render risk unaddressed |
| Solvability | 4 | Notes table makes it too easy; never breaks |
| Elegance | 3 | Notes table is a crutch that undermines the grid |
| Reading Reward | 3 | Noir flavor is charming but load-bearing? No |
| Fun | 4 | "Motive in the accessibility label" is delightful |
| Confirmation | 5 | MOTIVE is perfect — inevitable, reframes the artifact |
| **Total** | **23/30** | **PASS (narrow)** |

**What this condition catches:**
- Over-scaffolding concern (notes table) — YES, visible from basic analysis
- Braille render risk — YES, mentioned but not analyzed deeply
- Answer quality — YES

**What it misses:**
- No systematic principle-by-principle analysis
- No named vocabulary for the scaffolding failure (no "over-scaffolding" principle cited)
- No test for Snyder's Computer Test (10-line script)
- No framework for "Reading Reward" as a named criterion

**Braille-specific flag:** Flagged as "real concern" but no actionable recommendation.

---

## Condition C1: Rubric Added

**Context given:** "You are an expert reviewer." + scoring rubric with dimension definitions.

**Review:**

Working through the rubric dimensions:

**Clarity (1–5):** The puzzle is well-structured. The grid, the reference alphabet, and the notes table are clearly presented. One risk: Unicode braille characters (⠁, ⠙, etc.) may render incorrectly in some environments — as blank boxes, missing glyphs, or misaligned characters — because braille Unicode occupies a non-standard range and font support varies. Depending on delivery medium (web page with system font vs. printed PDF), this is anywhere from a minor to a critical failure. Score: **3** (rubric forces me to weight the rendering risk more seriously than the baseline does; "clarity" includes whether solvers can see the puzzle).

**Solvability (1–5):** The notes table provides row/col position AND the braille character for each of the six asterisked cells. The solver's actual task is: look up 6 characters in the reference alphabet. The grid is never required. On one hand: never breaks. On the other: so scaffolded that the difficulty is effectively zero. For a hunt puzzle, that's a calibration problem. Score: **4** (solvable, but understuffed).

**Elegance (1–5):** The conceit is genuinely elegant — a stolen painting's accessibility label contains the motive encoded for the visually impaired. That's thematically rich. But the notes table destroys the elegance by pre-extracting everything. The solver should locate the asterisked cells themselves, which would be a satisfying scan of the label. Pre-numbering them in a worksheet collapses the search into a lookup. Score: **2** (strong concept, damaged execution).

**Reading Reward (1–5):** The noir framing — 1947, Grand Hotel, painting stolen, label recovered from gallery floor — adds real texture. A solver who engages with it feels like an investigator. But the solve itself (look up 6 braille characters) requires zero knowledge of noir, art history, or the case. A solver who ignores all flavor text entirely arrives at MOTIVE equally fast. Score: **3**.

**Fun (1–5):** The "motive is encoded in the accessibility label" conceit is memorable and clever. The physical prop form (evidence tag, gallery label) is immersive. Score: **4**.

**Confirmation (1–5):** MOTIVE is an excellent answer for a noir mystery. It names exactly what the detective seeks. The answer feels earned. Score: **5**.

**Scores:**

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 3 | Braille render risk is a real clarity failure |
| Solvability | 4 | Solves reliably; too easy |
| Elegance | 2 | Notes table pre-extracts everything |
| Reading Reward | 3 | Flavor rich; not load-bearing |
| Fun | 4 | Conceit is memorable |
| Confirmation | 5 | MOTIVE is perfect |
| **Total** | **21/30** | **FAIL (narrow miss)** |

**What this condition catches over C0:**
- Rubric structure forces granular scoring — Elegance drops from 3 to 2 because the rubric asks specifically about elegance
- Clarity drops from 4 to 3 because the rubric asks about solver experience, making the render risk more consequential
- The puzzle now FAILS (21/30) rather than narrowly passing — the rubric reveals a weaker puzzle
- Braille render risk is elevated: "anywhere from minor to critical depending on delivery medium"

**What it still misses:**
- No named principles — the over-scaffolding failure has no vocabulary yet
- No Snyder's Computer Test applied
- No persona-specific lenses
- The render risk observation lacks a concrete recommendation (e.g., "provide dot-grid fallback")

**Braille-specific flag:** Stronger — now affecting the Clarity score. Delivery medium specified as a variable.

---

## Condition C2: Principle Names + Quotes, No Persona

**Context given:** C1 + design principle names with quotes, no persona.

**Review:**

Applying the design principles:

**The Riven Standard — "The puzzle IS what the field does."**
The puzzle claims to be a tactile braille label for visually impaired gallery visitors. The mechanic asks solvers to decode Unicode braille characters visually. A practitioner who reads braille does so by touch — they identify dot patterns by feel. The puzzle asks solvers to do something adjacent but different: identify visual Unicode representations of those patterns and match them to a reference chart. This is closer to "decoding a code based on braille" than "reading braille." The Riven Standard is partially met — there is a real connection to the field — but the physicality claim is ironic given the delivery medium. **Partial pass.**

**Solving = Proving Understanding — "Test: More knowledge after?"**
A solver who completes this puzzle knows that ⠍=M, ⠕=O, etc. They have done character lookups. Do they understand braille? Not meaningfully — the reference alphabet is provided, so the solve requires no prior knowledge and builds none. **Fails.**

**No Over-Scaffolding — "Test: Remove worksheet — still a puzzle?"**
The Investigator's Notes table provides row number, column number, and the braille character for each of the six asterisked cells. Remove the notes table: the solver must locate the asterisked cells themselves (a scanning task) and then read the braille characters from the grid (a visual parsing task). The puzzle is harder but more interesting without the notes table. The current version has the worksheet doing too much work. **Fails.**

**Surprise the Answer — "Test: Guessable from topic?"**
MOTIVE is not trivially guessable from "stolen painting" or "braille label" — a solver cannot jump to it without decoding. **Passes.**

**One Aha — "Test: Name the single aha."**
The single aha: "The asterisked cells in the braille label, decoded in order, spell the answer." That is one clean aha. **Passes.**

**Reading Reward — "Test: Keyword search sufficient?"**
A solver could, in principle, look up ⠍ ⠕ ⠞ ⠊ ⠧ ⠑ in any braille lookup table and decode the answer without engaging with the noir context at all. The noir frame is flavor, not load-bearing structure. **Partial pass** — the flavor enriches but doesn't constrain.

**No Computation Without Deduction — "Test: Remove — still a puzzle?"**
The solve is entirely lookup — there is no deduction. Remove any individual step: the character lookup is the entire mechanism. Snyder's Computer Test (10-line script): trivial. Define a dictionary {⠍:M, ⠕:O, ⠞:T, ⠊:I, ⠧:V, ⠑:E}, read the six asterisked characters from the notes table, concatenate. Zero logic required beyond the dictionary lookup. **The puzzle is a pure lookup puzzle.** This is a significant design concern. **Fails.**

**Verify the Last Mile — "Test: Trace letter by letter."**
M-O-T-I-V-E: each cell, each character, each lookup verified. The notes table makes this trivially checkable. **Passes.**

**Scores:**

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 3 | Braille render risk; Unicode may fail in print |
| Solvability | 4 | Reliable but understuffed |
| Elegance | 2 | Over-scaffolded; notes table is a worksheet, not a hint |
| Reading Reward | 3 | Noir flavor is present; not integrated into solve |
| Fun | 4 | Conceit excellent; execution too easy |
| Confirmation | 5 | MOTIVE is inevitable and perfect |
| **Total** | **21/30** | **FAIL** |

**Principle summary:**
- Passes: Surprise the Answer, One Aha, Verify the Last Mile
- Fails: Solving = Proving Understanding, No Over-Scaffolding, No Computation Without Deduction
- Partial: The Riven Standard, Reading Reward

**What this condition catches over C1:**
- Named vocabulary: "over-scaffolding," "Riven Standard," "No Computation Without Deduction" — allows precise diagnosis rather than general unease
- The Snyder Computer Test is now explicitly applied — and the puzzle fails it (trivially scriptable)
- Partial/pass/fail breakdown makes the design's weaknesses enumerable
- The distinction between "lookup puzzle" and "deduction puzzle" is now articulated

**What it still misses:**
- No persona-specific biases or priorities
- The Riven Standard analysis is correct but shallow — no person who actually reads braille is named
- Still no concrete fix recommendations

**Braille-specific flag:** Upgraded — the render risk is now linked to print delivery context. Unicode in print fonts is flagged as a specific failure mode for the first time.

---

## Condition C3: Principles + Tests

**Context given:** C2 + tests for each principle.

**Review:**

Running each principle's specific test:

**The Riven Standard:** Test: "Practitioner recognizes their own work?"
Would a visually impaired person who reads braille by touch recognize the puzzle as "their" activity? The puzzle renders braille as Unicode dot patterns to be identified visually. A braille reader's skill — haptic pattern recognition — is not what's being tested. A sighted person with no knowledge of braille can solve this using only the provided reference alphabet. The puzzle uses braille as a code, not as a communication system. A braille reader would recognize the patterns but would not find the puzzle's framing to be their practice. **Riven Standard: FAIL.** The puzzle is about braille as cipher, not braille as language.

**Solving = Proving Understanding:** Test: "More knowledge after?"
After solving, the solver knows: (1) ⠍=M, (2) that six asterisked cells in the label spell the answer. They do not know: how braille works, what the dot pattern system represents, why visually impaired gallery visitors need tactile labels. The provided reference alphabet means zero learning is required or incidentally happens. **FAIL.**

**Blame the Player:** Test: "Respect or resentment?"
Every clue is present and legible. The reference alphabet is complete and correct. The notes table maps every asterisked cell. A solver who fails can only blame themselves for not doing the lookup. **PASS.** Though note: if Unicode braille fails to render (see below), a solver has legitimate grievance.

**No Over-Scaffolding:** Test: "Remove worksheet — still a puzzle?"
Remove the Investigator's Notes table. The solver must: (1) scan the 12-column grid for asterisked cells, (2) read the braille character in each asterisked cell, (3) look it up in the reference alphabet. This is a more satisfying puzzle. The notes table provides not just space but extraction sequence, position coordinates, and the braille character itself — three layers of scaffolding. Remove it: puzzle improves. **FAIL.** Recommendation: Remove the notes table entirely, or provide only the asterisk-count ("six cells are marked — find and decode them in order").

**Surprise the Answer:** Test: "Guessable from topic?"
From "gallery label on a stolen painting," would a solver guess MOTIVE? The word is too specific and too apt to be guessed without decoding. The braille grid does not broadcast it. **PASS.**

**One Aha:** Test: "Name the single aha."
"The six asterisked braille cells, decoded in order, spell what the detective is looking for." One aha, cleanly named. **PASS.**

**Reading Reward:** Test: "Keyword search sufficient?"
A solver who types ⠍ into a braille lookup table gets M. The noir context — 1947, Grand Hotel, stolen painting — is entirely bypassed. The flavor text enriches but does not deepen the required operation. **PARTIAL PASS** — the context makes it more satisfying but is not structurally required.

**No Computation Without Deduction:** Test: "Remove — still a puzzle?"
The entire solve is lookup. There is no step where understanding produces a conclusion that wasn't visible before. Remove the computation (the lookups): there is no puzzle remaining. This test reveals that the puzzle is a pure cipher-decode with no deductive layer. **FAIL.**

**Snyder's Computer Test:** Test: "10-line script."
```python
ref = {'⠁':'A','⠃':'B','⠉':'C','⠙':'D','⠑':'E','⠋':'F','⠛':'G',
       '⠓':'H','⠊':'I','⠚':'J','⠅':'K','⠇':'L','⠍':'M','⠝':'N',
       '⠕':'O','⠏':'P','⠟':'Q','⠗':'R','⠎':'S','⠞':'T','⠥':'U',
       '⠧':'V','⠺':'W','⠭':'X','⠽':'Y','⠵':'Z'}
cells = ['⠍','⠕','⠞','⠊','⠧','⠑']  # from notes table
print(''.join(ref[c] for c in cells))  # MOTIVE
```
Six lines. The notes table provides the input directly. The computer solves this in zero deductive steps. **FAILS Snyder's Computer Test comprehensively.**

**Interlock, Not Independence:** Test: "Any order, no advantage?"
The notes table specifies order (row by row, left to right). If a solver decoded out of order, they'd get a wrong answer. The ordering constraint is present but trivially enforced by position. The cells themselves are independent — decoding cell 3 gives no information about cell 4. The cells do not interlock. **PARTIAL PASS** — ordering enforced; cells independent.

**Verify the Last Mile:** Test: "Trace letter by letter."
M(⠍) O(⠕) T(⠞) I(⠊) V(⠧) E(⠑) → MOTIVE. Each step fully traceable. **PASS.**

**Scores:**

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 3 | Render risk now a test failure: if Unicode fails, Blame the Player flips |
| Solvability | 4 | Reliable; Snyder's Computer Test confirms mechanical triviality |
| Elegance | 2 | Notes table fails Over-Scaffolding test; Riven Standard partial |
| Reading Reward | 3 | Flavor present; test confirms it's bypassed entirely |
| Fun | 4 | Conceit strong; execution thin |
| Confirmation | 5 | MOTIVE is perfect |
| **Total** | **21/30** | **FAIL** |

**What this condition catches over C2:**
- The Riven Standard FAILS outright (not partial) when the test is run: braille-as-cipher ≠ braille-as-language; a practitioner would not recognize their own work
- Snyder's Computer Test is now actually run (script provided) — definitive proof of the mechanical triviality
- Over-Scaffolding test produces a concrete fix: "remove the notes table, or reduce to asterisk-count only"
- Blame the Player test surfaces the render risk as a fairness issue, not just an aesthetics issue
- Interlock analysis is now formalized: cells are independent, which is a mild weakness

**What it still misses:**
- Still no persona — the analysis is principle-driven but impersonal
- No structural critique of hunt-fit (where does this puzzle sit in a 5-puzzle hunt?)
- The note about the physical prop form (this is a physical-first hunt) is not considered

**Braille-specific flag:** Now explicitly flagged as a Blame the Player failure mode — if Unicode renders as boxes, the solver has a legitimate grievance that cannot be blamed on the player. First condition to make this a fairness issue.

---

## Condition C4: Design Philosophy Only (Dan Katz, Thomas Snyder, Dana Young)

**Context given:** Design philosophy sections only from the three reviewer profiles. No review lens, no rubric, no principles.

**Review applying the three philosophies:**

**Through Katz's philosophy (puzzles as contracts; architectural lens):**

Katz thinks about puzzle hunts as contracts. What does this puzzle promise? It promises a noir artifact — a recovered document with embedded information. The contract is: "engage with this found object to discover the answer." Does it deliver? The grid is visually convincing as a period artifact. The notes table, however, breaks the contract by doing the investigator's work for them. The puzzle promises investigation and delivers administration. In a 5-puzzle hunt (4 feeders + 1 meta), this puzzle justifies its slot only on conceit — the "accessibility label contains the motive" idea is genuinely strong. But it would be stronger at half the scaffolding.

The answer MOTIVE is structurally sound for a noir hunt — it names what the investigator is hunting, which is a structural pay-off, not just a clever word. Katz would approve.

**Through Snyder's philosophy (constructor's hand visible; design is intention):**

Snyder asks: if a computer can generate your puzzle, your puzzle isn't finished. The braille lookup is precisely the kind of puzzle a computer generates. The reference alphabet is provided; the cells are pre-identified; the characters are pre-listed. The constructor's hand is visible only in the conceit (which is strong) and the grid design (which is detailed and convincing). The deduction sequence is lookup-lookup-lookup — there is no "forcing" of conclusions in Snyder's sense. The puzzle did not isolate a technique and then build a path that requires that technique; it selected a code and listed the code values.

Theme and structure diverge: the theme (braille as a communication system for the visually impaired, embedded in a recovered artifact) is evocative. The structure (dictionary lookup) is generic. Snyder would find this divergence disqualifying.

**Through Dana's philosophy (the puzzle IS the world; mechanic chosen because it IS the theme):**

Dana would ask: what does it feel like to be in this world, and does the mechanic put you there? The world here is the 1947 Grand Hotel, a stolen painting, a recovered frame label. The mechanic asks you to decode braille characters from a grid. This is close — a detective examining physical evidence really might do this — but the notes table collapses the investigative act into a worksheet. The artifact is beautifully constructed (the box-drawing table is visually convincing as a period label). The extraction mechanism destroys the immersion by telling you exactly where to look.

Dana's strongest criterion: "Is the answer arbitrary or inevitable?" MOTIVE is inevitable — it is exactly what the detective was looking for, and finding it in the accessibility label is the kind of reframe that makes an answer feel earned. She would keep the answer and cut the notes table.

**Scores:**

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | Artifact presentation is polished |
| Solvability | 4 | Reliable; scaffolding ensures it |
| Elegance | 2 | Snyder: theme/structure diverge; conceit ≠ mechanic |
| Reading Reward | 4 | Dana: noir world is present throughout |
| Fun | 5 | Three reviewers all find the conceit strong |
| Confirmation | 5 | MOTIVE is inevitable (Dana's test) |
| **Total** | **24/30** | **PASS** |

**What this condition catches over C3:**
- Elegance framing shifts: the divergence between "conceit" and "mechanism" is now articulated philosophically (Snyder: "theme and structure diverge")
- Reading Reward RISES (to 4) — Dana's world-inhabiting philosophy captures what the noir context contributes, which principle analysis (C2/C3) undervalued
- Fun rises to 5 — all three philosophies find the conceit strong, which is convergent evidence
- The "contract" framing (Katz) is a new lens: the puzzle promises investigation, delivers administration
- MOTIVE gets a stronger endorsement (Dana: "inevitable answer reframes everything before it")

**What it misses:**
- No specific technical flags — the braille render issue is NOT raised in this condition
- No principle vocabulary (over-scaffolding is described but not named)
- The Snyder's Computer Test is not explicitly run
- The structural position of this puzzle in the hunt is not considered

**Braille-specific flag:** NOT RAISED. Design philosophy is too abstract to catch a rendering artifact. This is a significant miss for the question "does any condition catch braille rendering issues?"

---

## Condition C5: Review Lens Only (Dan Katz, Thomas Snyder, Dana Young)

**Context given:** Review lens sections only from the three profiles. No design philosophy, no rubric, no principles.

**Review applying the three lenses:**

**Dan Katz's lens — structural and architectural:**

- "Does every puzzle justify its slot?" In a 4+meta hunt, P5 is the last feeder. It needs to carry weight. The braille decode is mechanically thin for a slot that should feel like a capstone discovery. The conceit (motive in the accessibility label) earns the slot; the execution does not match.
- "Is the narrative encountered while solving or reported afterward?" The noir frame here is genuine — the artifact IS the puzzle. Katz would note that the investigator notes table is narrative reported afterward (here's what to look at), not narrative encountered while solving (you discover the asterisked cells yourself).
- "Would he want to solve this?" Katz's honest answer: probably yes, for the conceit and the answer, but he'd finish in ninety seconds and want more. That's a concern.

- "Is the thematic coherence structural, not decorative?" The 1947 noir dressing is structural — the label format is convincing as a period artifact. But the actual decode is identical to any braille cipher regardless of noir context. Structural-ish.

**Thomas Snyder's lens — construction and deduction:**

- "Is the entry point constructed or discovered?" The entry point is given: the asterisked cells. There is no designed moment of discovery. The solver is told exactly where to look before they start. This is a constructed-for-accessibility entry point, not a puzzle entry point.
- "Is there a puzzle here, or a procedure?" Six lookups in a provided dictionary. This is a procedure. There is no moment of genuine deduction — no point where understanding the rules forces a conclusion that wasn't visible before. The puzzle fails Snyder's most basic test.
- "Did someone add information to increase difficulty?" The opposite problem here — information was added to reduce difficulty (the notes table). The puzzle as designed cannot increase in difficulty without removing scaffolding.
- "Would he want to have constructed this?" No. The braille grid is well-crafted visually, but it does not represent genuine constructor choices at the logic level. The character selection (⠍⠕⠞⠊⠧⠑) is not a design decision — it is a consequence of the answer. There is no solving path in Snyder's sense: no forced first deduction, no interlocking constraints.

**Dana Young's lens — visual and experiential:**

- "Does the visual grammar hold consistently?" YES. The Unicode braille characters appear consistently in the middle row of each grid cell, with the text representation above and the asterisk marker below. The grammar is clear and consistent. HOWEVER: if Unicode braille renders as blank boxes (font failure), the visual grammar collapses entirely. This is the clearest identification of the braille render risk across all conditions so far — Dana's visual-first lens surfaces it directly.
- "Does the flavor text explain what the layout should have communicated?" The investigator notes table is exactly this failure. The layout (asterisked grid cells with Unicode braille) should communicate "find these cells and decode them." The notes table explains what the layout should have done, which means the layout didn't finish. Dana cuts it.
- "Is the extraction earning its step?" Lookup in a provided alphabet. The extraction adds process (turning braille character into letter) but no meaning. It does not transform the solve.
- "Is there a moment where the puzzle resolves visually?" There is a structural answer: when the six letters spell MOTIVE. But because the notes table does all the visual work, the solver never experiences the grid resolving — they fill in a worksheet. The visual resolution is lost.
- "Does it work without the context?" YES — and that is the problem. The solve works without engaging the noir context at all.

**Scores:**

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 3 | Dana's visual lens: if Unicode fails, grammar collapses |
| Solvability | 4 | Snyder: reliable procedure; Katz: too fast for the slot |
| Elegance | 1 | Snyder: procedure, not puzzle; no deductive moment |
| Reading Reward | 3 | Katz: narrative reported, not encountered |
| Fun | 4 | Conceit saves it; Katz would still enjoy the answer |
| Confirmation | 5 | Dana: MOTIVE is inevitable |
| **Total** | **20/30** | **FAIL** |

**What this condition catches over C4:**
- Elegance drops to 1 (lowest yet) — Snyder's lens is brutal about "procedure vs. puzzle"; this is the first condition to score Elegance at 1
- The braille render risk is now EXPLICITLY AND PRECISELY identified for the first time — Dana's visual grammar lens surfaces it as "if Unicode braille renders as blank boxes, the visual grammar collapses entirely." This is the strongest catch of the rendering issue in the entire study.
- Katz's "narrative reported vs. encountered" lens names the notes table failure more precisely than any previous condition
- Dana's "does flavor text explain what layout should have communicated?" is exactly the right diagnosis of the notes table

**What it misses:**
- No named principles — the analysis is insightful but the vocabulary is personalistic rather than systematic
- No explicit Snyder's Computer Test run
- No hunt-level structural questions (hunt fit, feeder weight)

**Braille-specific flag:** BEST CATCH in the study. Dana's review lens directly surfaces the render risk as a visual grammar failure. Snyder's lens catches the "procedure" problem which is structurally related: if the puzzle is a procedure, it cannot survive a rendering failure because there is no puzzle underneath.

---

## Condition C6: Full Profiles (Dan Katz, Thomas Snyder, Dana Young)

**Context given:** Full profiles (design philosophy + review lens) for all three reviewers. No rubric, no principles.

**Review by each reviewer:**

---

**Dan Katz**

I've been thinking about puzzle hunts as contracts for a long time. This puzzle makes an excellent promise — the motive is hidden in the painting's accessibility label — and then breaks it by handing the solver a worksheet that does the search for them.

In a 4+meta hunt with mixed-experience teams, I understand the design choice. The notes table prevents frustration. But it trades frustration prevention for puzzle integrity. The investigator notes are the structural equivalent of a free answer — they remove the solving experience while preserving the answer.

For a feeder puzzle at this position (P5, last before the meta), I'd want more. The braille decode should be the final satisfying step, not the only step. Where's the choice? Where's the binding decision that makes the solve feel earned?

The answer MOTIVE is architecturally sound — it serves the hunt's meta, it names what the detective wants, it arrives as a revelation. I'll give credit for that. The conceit is strong enough that I'd keep this puzzle, but I'd remove the investigator notes table entirely. Let the solver find the asterisked cells. That's the hunt's payoff for engaging with the artifact.

One structural flag: the notes table provides Unicode braille characters directly. If the Unicode rendering fails in the delivery medium (plausible for a physical-first hunt with PDF printing), the notes table becomes the solver's only tool — which reveals that the grid itself is not necessary to the solve. A puzzle that can be solved without its primary visual element has a design problem.

Scores: Clarity 3, Solvability 4, Elegance 2, Reading Reward 3, Fun 4, Confirmation 5. **Total: 21/30. FAIL.**

---

**Thomas Snyder**

The braille grid is beautifully constructed as a visual artifact. I mean that. Whoever built the 12-column Unicode grid with box-drawing characters, with the text above and the braille below and the asterisk markers in the third row of each cell — that's detail work, and I respect it.

But the puzzle isn't there. The deduction sequence is: find the asterisked cell in the notes table, read the character, look it up in the reference alphabet. That is a procedure, not a solve. There is no moment where understanding the rules forces a conclusion. Every step is lookup. A ten-line Python script solves it from the notes table in one pass.

The entry point was not designed — it was listed. The constructor told the solver where to start (here are the six cells), what to read (here are the characters), and where to look (here is the alphabet). Nothing was left for the solver to discover. I have no way to evaluate whether the puzzle was "designed" at the logical level, because there is no logical level to evaluate.

The theme is rich. The grid is detailed. The answer is excellent. The puzzle is absent.

My concrete recommendation: strip the notes table. Add a constraint that makes the ordering nontrivial (perhaps the asterisked cells must be identified by some property of their position in the catalog text, not just their visual asterisk marker). Give the solver something to find before they decode.

**Critical flag:** Unicode braille rendering is an untested assumption in this puzzle. In any print-to-PDF workflow, the braille characters may render as blank cells, question marks, or font-substitution artifacts. If that happens, the notes table becomes the complete puzzle (it has the characters directly). This means the visual braille grid is decorative, not structural — which is exactly the kind of construction failure I identify when theme and structure diverge.

Scores: Clarity 3, Solvability 3, Elegance 1, Reading Reward 3, Fun 3, Confirmation 5. **Total: 18/30. FAIL.**

---

**Dana Young**

The visual grammar of this puzzle is nearly right, and nearly right is what keeps me up at night.

The grid is beautiful. The evidence tag framing, the box-drawing cells, the Unicode braille characters in their proper positions, the period-appropriate catalog text — this is a lovingly constructed artifact. I can feel the work in it.

But there are two problems. First: the investigator notes table is doing the job the layout was supposed to do. A solver who encounters the asterisked cells in the grid should experience the discovery themselves — scan the label, notice the asterisks, locate each one, read its character, decode it. The notes table collapses that discovery into a form-filling exercise. The layout finished; the notes table undoes the finish.

Second, and more importantly for production: this puzzle assumes that Unicode braille characters render correctly in the delivery environment. For a physical-first hunt, the delivery environment is a printed PDF. Braille Unicode (U+2800–U+28FF) is absent from most print fonts, including the Courier New and Georgia that this hunt's print spec uses. What the solver will likely see on paper is a row of empty boxes or unmapped glyphs, while the reference alphabet row shows the same kind of failure. The notes table becomes the complete solve — meaning the grid was never needed.

The visual grammar test: does it hold when the Unicode fails? No. The grammar requires the braille characters. Remove them and the grid communicates nothing.

MOTIVE is an inevitable answer. It names the thing the detective is looking for, and finding it in the painting's accessibility label is a resonant discovery. The answer deserves a better puzzle around it.

My recommendation: build a dot-grid fallback. For each braille cell, include a 2×3 grid of dots (filled or empty) as ASCII or HTML, in addition to the Unicode character. This makes the puzzle medium-independent and genuinely tactile-feeling without requiring braille font support. It also makes the grid the puzzle, not the notes table.

Scores: Clarity 2, Solvability 4, Elegance 2, Reading Reward 4, Fun 4, Confirmation 5. **Total: 21/30. FAIL.**

---

**Aggregate C6 scores (average):**

| Dimension | Katz | Snyder | Dana | Avg |
|-----------|------|--------|------|-----|
| Clarity | 3 | 3 | 2 | 2.7 |
| Solvability | 4 | 3 | 4 | 3.7 |
| Elegance | 2 | 1 | 2 | 1.7 |
| Reading Reward | 3 | 3 | 4 | 3.3 |
| Fun | 4 | 3 | 4 | 3.7 |
| Confirmation | 5 | 5 | 5 | 5.0 |
| **Total** | **21** | **18** | **21** | **20/30** |

**FAIL (consensus)**

**What this condition catches over C5:**
- Three distinct reviewer voices give divergent total scores (18, 21, 21) — the range reveals reviewer-specific severity
- Snyder scores the puzzle at 18/30, significantly lower than others — his "procedure vs. puzzle" lens is the harshest
- Dana provides a concrete, actionable fix for the render issue (dot-grid fallback in ASCII/HTML)
- Katz frames the notes table as "the structural equivalent of a free answer" — the sharpest formulation of this problem in the study
- The print font issue (Courier New / Georgia) is explicitly named for the first time — Dana cross-references the hunt's print spec

**What it misses:**
- No named design principles — the vocabulary is reviewer-personal
- No systematic pass/fail on the principle checklist

**Braille-specific flag:** MOST COMPLETE ANALYSIS. Dana provides a concrete production fix (dot-grid ASCII fallback), names the specific print fonts at risk (Courier New / Georgia), and diagnoses the consequence (notes table becomes the complete puzzle). Katz flags the same structural implication. Snyder frames it as theme/structure divergence. Three independent paths to the same finding.

---

## Condition C7: Full Profiles + All Principles

**Context given:** Full profiles (C6) + all 11 design principles with tests (C3).

**Review:**

This is the maximal-context condition. Running all 11 principle tests, then applying each reviewer's lens.

**Principle Tests (abbreviated — full analysis in C3; noting only changes/additions):**

| Principle | Test Result | Notes vs. C3 |
|-----------|-------------|--------------|
| The Riven Standard | FAIL | Braille-as-cipher ≠ braille-as-language; confirmed by Snyder (theme/structure diverge) and Dana (visual grammar) |
| Solving = Proving Understanding | FAIL | Same as C3 |
| Blame the Player | CONDITIONAL | If Unicode renders; Dana adds: if print font lacks braille glyphs, solver cannot be blamed |
| No Over-Scaffolding | FAIL | Katz: "structural equivalent of a free answer"; Dana: "layout did the job; notes table undoes it" |
| Surprise the Answer | PASS | MOTIVE not guessable |
| One Aha | PASS | Single clean aha |
| Reading Reward | PARTIAL | Dana's lens raises this: the world is present; the mechanic doesn't require it |
| No Computation Without Deduction | FAIL | Confirmed by all three: pure procedure |
| Snyder's Computer Test | FAIL | Script provided in C3; confirmed by Snyder in C6 |
| Interlock, Not Independence | PARTIAL | Ordering enforced; cells independent |
| Verify the Last Mile | PASS | Full trace clean |

**Failures:** Riven Standard (test), Solving = Proving Understanding, Blame the Player (conditional), No Over-Scaffolding, No Computation Without Deduction, Snyder's Computer Test. 6 fails, 3 passes, 2 partials.

**Reviewer synthesis:**

The principle failures and the reviewer lenses converge on three problems:

1. **The notes table (over-scaffolding / free answer / layout undone):** All three reviewers flag this. It is the single most damaging element. Fix: remove it, or reduce to asterisk-count only.

2. **The braille render risk (Blame the Player conditional / visual grammar collapse / theme-structure divergence):** Dana's lens is clearest. The print spec uses fonts that likely lack braille Unicode glyphs. A dot-grid ASCII fallback is the concrete fix. This is medium-critical for a physical-first hunt.

3. **Procedure, not puzzle (No Computation Without Deduction / Snyder's Computer Test / no deductive moment):** Snyder's lens is most explicit. The puzzle needs a genuine deductive layer, not just a lookup. Possible fix: the asterisked cells should require some reasoning to identify (not just visual scan), making the identification step a deductive act.

**Scores (maximal context, informed by all lenses + principles):**

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 2 | Render risk is a production blocker for physical-first hunt |
| Solvability | 4 | Reliable; but trivial |
| Elegance | 1 | Principle failures stack: Riven, Over-Scaffolding, No Computation |
| Reading Reward | 3 | Dana's lens partially rehabilitates; still not load-bearing |
| Fun | 4 | Conceit survives everything |
| Confirmation | 5 | MOTIVE: all reviewers agree, inevitable |
| **Total** | **19/30** | **FAIL** |

**Priority recommendations from C7:**
1. Remove the Investigator's Notes table entirely. Let solvers find and decode the asterisked cells from the grid.
2. Build a dot-grid ASCII fallback for every braille cell (2×3 filled/empty dots). This makes the puzzle medium-independent and resolves the print rendering risk.
3. Consider adding a minimal deductive layer: e.g., the asterisked cells are identified by some property of their position in the catalog text (not just a visual marker), requiring the solver to reason about placement before decoding.
4. The answer MOTIVE is excellent. Do not change it.

**What this condition catches over C6:**
- Principle failures are now enumerated and counted (6 fails) — the severity is quantified
- The "Blame the Player conditional" is the most nuanced treatment of the render risk: it is now a fairness problem, not just a technical one
- The three-problem synthesis (notes table, render risk, procedure) is more compact and actionable than the per-reviewer analysis
- Elegance hits 1 (matching Snyder's C6 score) with full principle + lens context

**What it adds over all previous conditions:**
- The dot-grid ASCII fallback recommendation (from C6 Dana) is now elevated to a numbered priority
- The "minimal deductive layer" fix is a new recommendation not present in any earlier condition — it comes from combining Snyder's lens (what makes a puzzle?) with the No Computation principle

---

## Summary Table

| Condition | Context | Total Score | Pass/Fail | Braille Render Issue Caught? | Scaffolding Issue Caught? | Key Addition |
|-----------|---------|-------------|-----------|------------------------------|---------------------------|--------------|
| C0 | "Expert reviewer" | 23/30 | PASS | Mentioned, weak | Yes, unnamed | Baseline |
| C1 | + Rubric | 21/30 | FAIL | Elevated to "anywhere from minor to critical" | Yes, unnamed | Rubric forces lower Clarity + Elegance; flips pass/fail |
| C2 | + Principle names+quotes | 21/30 | FAIL | Linked to print delivery | Named: "No Over-Scaffolding" | Vocabulary; Snyder Computer Test mentioned |
| C3 | + Tests | 21/30 | FAIL | Named as Blame the Player failure mode | Notes table removable; fix proposed | Snyder Computer Test actually run; Riven Standard fails |
| C4 | Design Philosophy only | 24/30 | PASS | NOT RAISED | Named philosophically (Snyder: theme/structure diverge) | Reading Reward rises; Fun rises; render risk blind spot |
| C5 | Review Lens only | 20/30 | FAIL | BEST IDENTIFICATION (Dana: visual grammar collapses) | Dana: "flavor text does layout's job" | Most precise render diagnosis; Elegance drops to 1 |
| C6 | Full profiles | 20/30 avg | FAIL (consensus) | Most complete: dot-grid fix named, print fonts named | Katz: "structural equivalent of free answer" | Actionable fix; divergent scorer severity (18–21 range) |
| C7 | Full profiles + principles | 19/30 | FAIL | Production blocker; Blame the Player conditional | 6 principle failures enumerated | Synthesis; priority fix list; deductive layer fix proposed |

---

## Key Findings

### 1. The Rubric Is the Most Important Single Addition

C0 (no rubric) passes at 23/30. C1 (adds rubric) fails at 21/30. The rubric is what catches the puzzle. Without named dimensions, the reviewer lacks a framework to weight the Elegance failure (over-scaffolding) against the Fun success (conceit). The rubric forces precision that intuition elides.

### 2. Design Philosophy Alone (C4) Is the Worst Condition for Problem Detection

C4 gives the highest score (24/30 — PASS) while every other rubric-applying condition fails the puzzle. Philosophy provides orientation but not diagnostic power. The reviewers' philosophies find the conceit compelling (which it is) and underweight the execution failures. C4 is the condition most at risk of approving a flawed puzzle.

### 3. Review Lens Alone (C5) Is the Best Single-Context Condition

C5 gives 20/30 with the clearest identification of the braille render issue and the sharpest diagnosis of the notes table. The lens operationalizes the philosophy into specific questions, and Dana's visual-first lens is perfectly suited to catch a rendering failure.

### 4. On Braille Rendering in Markdown/Print

The question of whether Unicode braille characters (⠁–⠿) render correctly is the most production-critical issue in this puzzle. The findings across conditions:

- **C0/C1:** Mentioned as "a concern" with no specificity
- **C2/C3:** Linked to print delivery; named as a Blame the Player failure mode
- **C4:** NOT RAISED — design philosophy is too abstract to catch rendering artifacts
- **C5:** First precise identification — Dana's lens: "if Unicode braille renders as blank boxes, the visual grammar collapses entirely"
- **C6:** Most complete — Dana cross-references the hunt's print spec (Courier New / Georgia), confirms those fonts lack braille glyphs, proposes dot-grid ASCII fallback as concrete fix
- **C7:** Elevates to production blocker; frames as fairness issue (Blame the Player conditional)

**Conclusion on the render question:** Full persona profiles (C6/C7) are required to catch this issue fully. The review lens alone (C5) catches the visual grammar failure; the full profile (C6) catches the production-specific font issue. Principle tests alone (C3) catch it as a fairness problem. Philosophy alone (C4) misses it entirely.

### 5. The Puzzle's Fundamental Issue

P5-gallery-label.md has an excellent conceit (motive in the accessibility label), a perfect answer (MOTIVE), and a beautiful artifact (the Unicode braille grid). Its execution is damaged by a single decision: the Investigator's Notes table pre-extracts everything, converting a discovery puzzle into a form-filling exercise. Remove the notes table: the puzzle improves from FAIL to probable PASS. The braille render risk is the second issue; the dot-grid ASCII fallback resolves it.

The puzzle passes C0 (no rubric) and C4 (philosophy only), and fails every condition that includes a rubric or a review lens. This tells us the puzzle is good enough to feel right but not good enough to hold up under analysis — exactly the kind of failure mode that the ablation study is designed to surface.

---

## Appendix: Braille Decode Verification

| Cell | Position | Braille | Unicode | Expected Letter | Verified |
|------|----------|---------|---------|-----------------|----------|
| 1 | Row 1, Col 12 | dots 1,3,4 | ⠍ | M | YES |
| 2 | Row 2, Col 5 | dots 1,3,5 | ⠕ | O | YES |
| 3 | Row 5, Col 6 | dots 2,3,4,5 | ⠞ | T | YES |
| 4 | Row 7, Col 5 | dots 2,4 | ⠊ | I | YES |
| 5 | Row 9, Col 2 | dots 1,2,3,6 | ⠧ | V | YES |
| 6 | Row 10, Col 10 | dots 1,5 | ⠑ | E | YES |

**Answer: MOTIVE. Correct.**

*Note: Dot numbering convention verified against standard Grade 1 Braille alphabet (Braille Authority of North America). Left column top-to-bottom = dots 1,2,3; right column top-to-bottom = dots 4,5,6.*
