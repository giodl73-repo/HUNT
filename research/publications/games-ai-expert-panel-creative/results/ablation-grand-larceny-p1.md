# Ablation Study: Grand Larceny P1 — "Police Report"
## Reviewer Context Depth vs. Evaluation Quality

**Puzzle:** P1-police-report.md — "EXHIBIT A: INCIDENT REPORT"
**Domain:** Noir mystery, physical document, 1947 hotel art theft
**Correct answer:** SUSPECT (7 boxed letters extracted from underlined words)
**Format:** Logic-deduction worksheet embedded in a period-authentic police incident report

**Study date:** 2026-02-28
**Conditions:** 8 (C0–C7)

---

## Puzzle Summary (for reference)

The puzzle presents a 1947 police incident report documenting the theft of "Lady in Vermillion" from The Grand Hotel. Four suspects have alibi gaps overlapping the 9:15–9:25 PM theft window. Solvers fill a worksheet determining which suspects could plausibly have been at the gallery. A Critical Deduction section argues that Kessler's 9:28 PM room entry is inconsistent with the thief's escape route (down via service elevator to basement/alley), narrowing the field. The answer SUSPECT is extracted via seven underlined words, each contributing one boxed letter: S-U-S-P-E-C-T.

**Two-layer structure:** (1) logic puzzle — alibi elimination; (2) extraction — letter-indexing from underlined words.

---

## Condition Results

---

### C0: Baseline — "You are an expert reviewer."

**Prompt context given:** "You are an expert reviewer."

**Scores:**

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | The report format is legible and the worksheet is well-structured. The alibi gaps are clearly labeled. Minor issue: the word "unaccounted window" appears in both the narrative and the worksheet, creating slight redundancy. |
| Solvability | 3 | The logic is sound but the puzzle gives contradictory signals. The Critical Deduction section does significant interpretive work that arguably belongs to the solver. If that section is removed, the puzzle becomes genuinely hard; as written, it nearly solves itself. |
| Elegance | 3 | The worksheet format is functional. The extraction (7 boxed letters from underlined words) is standard and unintegrated with the deduction layer. |
| Reading Reward | 3 | The noir document voice is present but thin. Anyone who can read can work through this — domain knowledge of 1947 hotel culture, detective procedure, or noir conventions adds color but nothing structural. |
| Fun | 3 | Satisfying if you like logic grids. The period flavor is pleasant. The Critical Deduction section deflates what should be the solver's own "aha." |
| Confirmation | 4 | SUSPECT spells clearly and thematically. The answer fits. |
| **Total** | **20/30** | **FAIL** |

**Summary:** A competent but over-scaffolded logic puzzle. The extraction is clean; the deduction is undermined by its own Critical Deduction section. Would benefit from editorial restraint.

**Observed reviewer behavior:** Generic expert framing produces accurate scoring but lacks specificity. Notes cluster around vague structural observations. No principle vocabulary. No named failure modes. No transfer observations about format mismatch between puzzle-hunt extraction and physical-document authenticity.

---

### C1: Rubric Added — Baseline + scoring rubric

**Prompt context given:** "You are an expert reviewer." + full 6-dimension rubric (Clarity, Solvability, Elegance, Reading Reward, Fun, Confirmation — 1-5 each, /30, pass ≥ 22).

**Scores:**

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | The report is legible. All four alibi sections follow a consistent structure: statement, corroborating evidence, contradicting evidence, unaccounted window. The worksheet mirrors this cleanly. |
| Solvability | 3 | The logic chain is solvable but the Critical Deduction section does explicit elimination work (Kessler ruled out, logic summarized) that removes a key deduction step from the solver. As a reviewer I can solve it, but I'm solving through text the puzzle wrote rather than writing my own deduction. |
| Elegance | 3 | Two separate mechanisms — alibi logic and letter extraction — that don't interact. The extraction (underlined words → boxed letters) is appended rather than integrated. A more elegant design would encode the answer within the logical structure itself. |
| Reading Reward | 2 | The rubric asks: does noir/mystery domain knowledge matter? Answer: no. The puzzle can be solved entirely by reading the contradiction lists. No knowledge of 1940s hotel culture, detective procedure, or the noir genre is structurally required. The domain is costume, not mechanic. |
| Fun | 3 | The alibi gap table is genuinely enjoyable to reason through. The period voice adds atmosphere. The overexplained ending is anticlimactic. |
| Confirmation | 4 | SUSPECT is an excellent answer — thematic, fits the context, creates a final-word satisfaction. |
| **Total** | **19/30** | **FAIL** |

**Summary:** The rubric forces explicit articulation of "Reading Reward" as a dimension, which surfaces the puzzle's key weakness: the domain is decorative. The rubric-guided review catches this where C0 only noted it vaguely.

**Observed reviewer behavior:** The rubric creates structure and forces per-dimension accountability. "Reading Reward" as a named category directly exposes the puzzle's domain-as-costume problem. Scores are slightly lower than C0 because the rubric makes the reviewer articulate what "Reading Reward" actually tests. Specificity improves meaningfully over C0.

---

### C2: Principle Names + Quotes — No persona

**Prompt context given:** The 11 design principles by name and quote only (no tests, no persona).

**Scores:**

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | Per "Blame the Player" — every clue is fair in retrospect? Yes: every contradiction is explicitly labeled. Per "No Over-Scaffolding" — the worksheet provides space and labels, which is appropriate. The worksheet does NOT solve the puzzle for you... except the Critical Deduction section does. |
| Solvability | 2 | "No Computation Without Deduction" is not violated (there's no arithmetic). However, the Riven Standard is instructive: "The puzzle IS what the field does." What does a detective do? They eliminate. The puzzle does the elimination for the solver in the Critical Deduction section. This is the puzzle doing the detective work, not the solver. Solvability in principle is fine; solvability as designed is undermined by the author. |
| Elegance | 3 | "One Aha: One aha is fun." The intended single aha — Kessler's room entry time proves he went back rather than out — is a strong logical insight. But it's spelled out in the puzzle text. The aha is narrated, not discovered. "Surprise the Answer" — SUSPECT is not guessable from topic alone, good. "Interlock, Not Independence" — the four suspects are interrelated by the shared timeline and service elevator clue. This is structurally sound. |
| Reading Reward | 2 | "Reading Reward: Genuine domain engagement." Test: Is keyword search sufficient? Yes. You could solve this by searching for "no witness" in each section and reading the time values. No noir fluency required. No 1947 hotel knowledge needed. The domain is not doing structural work. |
| Fun | 3 | The puzzle has a single excellent aha buried under prose that explains it. Remove the explanation and the aha would be genuinely pleasurable. |
| Confirmation | 4 | "Verify the Last Mile" — tracing S-U-S-P-E-C-T letter by letter: Saturday(1)=S, annual(4)=U, persons(4)=S, approximately(3)=P, evening(1)=E, mechanical(4)=C, question(5)=T. All verified. Clean extraction. |
| **Total** | **18/30** | **FAIL** |

**Summary:** Principle names without tests produce targeted diagnosis. "The Riven Standard" applied to detective fiction immediately surfaces the over-scaffolding. "Verify the Last Mile" enables explicit extraction tracing. Principle framing anchors critique to design intent rather than impressionism.

**Observed reviewer behavior:** The principle names generate specific vocabulary for naming failures, even without tests. The reviewer uses "Riven Standard" and "One Aha" as diagnostic labels. Quality of critique improves substantially over C0 and C1. However, without the tests, application is somewhat intuitive — the reviewer maps principle quotes to the puzzle by inference rather than explicit test procedure. Score lower than C0/C1 because the principles highlight multiple failures simultaneously.

---

### C3: Principles + Tests — No persona

**Prompt context given:** Full 11 principles with names, quotes, AND explicit tests.

**Scores:**

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | Test: "Remove — still a puzzle?" Applied to the worksheet headers: remove the column labels and the puzzle still functions. The structure is genuine, not load-bearing scaffolding. Test: "Blame yourself" — every contradiction is documented with source evidence (named witness, specific time). Fair throughout. |
| Solvability | 2 | Test (Riven Standard): "Practitioner recognizes their own work?" A real detective investigating this case would read the contradictions, cross-reference the timeline, and deduce the escape route — independently. As written, a "detective" solving this puzzle has the deduction presented to them in the Critical Deduction section. The practitioner (detective) is denied the practice. FAIL on Riven Standard. Test (One Aha): "Name the single aha." The aha is: Kessler's 9:28 room entry is inconsistent with the basement exit route. That's a strong, specific, single aha. But it appears as narrated text, not as a puzzle element the solver must discover. |
| Elegance | 3 | Test (Snyder's Computer Test): "10-line script." A Python script could: parse each suspect's gap start/end, check for overlap with 9:15-9:25, and eliminate Kessler based on the room-entry time + direction-of-travel logic. The core deduction is automatable. This means the puzzle fails Snyder's test — the key insight is logical but mechanical enough to script. However, the extraction (7 indexed letters from underlined words) requires human recognition of underlining. The two layers have different scriptability profiles. Test (Interlock): suspects interlock via the shared service elevator and single theft window. The timeline is a common constraint. This is structurally sound. |
| Reading Reward | 2 | Test: "Keyword search sufficient?" Yes — "no witness" and "unaccounted" do most of the work. Test: "Genuine domain engagement?" A solver who knows nothing about 1947 hotel hierarchy, the significance of key card mechanical systems, or period police report conventions can solve this exactly as well as one who does. The domain is not rewarding domain knowledge. |
| Fun | 3 | Test (Surprise the Answer): "Guessable from topic?" SUSPECT is not guessable from "police report" alone — you might guess MURDER, GUILTY, GUILTY, THIEF. The answer surprises appropriately. Test (Blame the Player): "Respect or resentment?" Nothing unfair here. The contradiction evidence is clear and consistently formatted. A solver who fails has misread, not been deceived. |
| Confirmation | 5 | Test (Verify the Last Mile): "Trace letter by letter." S (Saturday, pos 1) ✓, U (annual, pos 4) ✓, S (persons, pos 4) ✓, P (approximately, pos 3) ✓, E (evening, pos 1) ✓, C (mechanical, pos 4) ✓, T (question, pos 5) ✓. Full verification passes. "Mechanism and extraction separate?" Yes — the deduction and the extraction are completely independent layers. |
| **Total** | **19/30** | **FAIL** |

**Summary:** Principles with tests produce the most rigorous diagnostic pass of any non-persona condition. Snyder's Computer Test catches that the core alibi deduction is automatable. The Riven Standard applied with its test catches the core structural failure: the detective is denied detective work. The Verify the Last Mile test produces explicit letter-by-letter extraction confirmation.

**Observed reviewer behavior:** Adding tests to principle names measurably increases evaluation precision. The reviewer can now answer specific procedural questions (Can I write a 10-line script? Can I trace the extraction letter by letter?) rather than relying on intuition. The evaluation becomes more falsifiable. Scores are calibrated similarly to C2 but individual dimension notes are richer and more actionable. The principal advance over C2: the reviewer engages the puzzle procedurally, not just conceptually.

---

### C4: Design Philosophy Only (from profiles)

**Prompt context given:** Only the "Design Philosophy" sections from Dan Katz, Thomas Snyder, and Dana Young profiles — no names, no credentials, no review lenses.

**Scores:**

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | The document has the structure of a genuine police report — bureaucratic, hierarchical, consistent. It fulfills its format contract. The worksheet mirrors the report structure cleanly. Nothing misleads. |
| Solvability | 2 | One philosophy fragment is directly applicable: "If a computer can generate your puzzle, your puzzle isn't finished." The alibi elimination in this puzzle is deterministic given the data. The Critical Deduction section makes it more deterministic by narrating the conclusion. The constructor's hand should be visible in restraint — in what is NOT said. Here, the author over-articulated. A second fragment: "A puzzle hunt is one thing, not a collection of puzzles." This is a single puzzle, not a hunt, but the principle applies structurally: the deduction layer and the extraction layer are two things, not one. They don't cohere. |
| Elegance | 3 | "The answer is the last move in this logic." (Dana's philosophy, paraphrased.) SUSPECT works as a final move if the solver has genuinely eliminated the other suspects themselves. If the puzzle has done the elimination, the answer is just a label, not a conclusion. "Theme is not decoration... when theme and structure are the same thing, the puzzle has coherence." The noir setting is decorative. The structure is logic-grid alibi analysis. These are not the same thing. |
| Reading Reward | 2 | "The mechanic isn't chosen to represent the theme. It's chosen because it IS the theme." This puzzle's mechanic (alibi logic worksheet) represents detective work but does not instantiate it — the detective's key deductive move is performed by the puzzle text. The mechanic and the theme are still separate. |
| Fun | 3 | The period document voice creates genuine atmosphere. The alibi contradictions are well-constructed. The puzzle would be more fun without the Critical Deduction section. |
| Confirmation | 4 | The answer is good. It has the "reframes everything before it" quality from Dana's philosophy — after finding SUSPECT, you realize the whole puzzle was about identifying the suspect. Works. |
| **Total** | **18/30** | **FAIL** |

**Summary:** Design philosophy without credentials or review lenses generates philosophically-oriented critique. The reviewer reaches for framework fragments ("the puzzle IS the theme," "the constructor's hand must be visible") to anchor specific observations. This produces richer conceptual critique than C0/C1 but less procedural precision than C3.

**Observed reviewer behavior:** Philosophy-only framing produces more interpretive, essayistic critique. The reviewer synthesizes across the three philosophies rather than applying them as distinct lenses. This creates cross-contamination that is intellectually interesting but potentially less reliable for calibration. The notable absence: no structural vocabulary for hunt-level concerns (Dan Katz's vocabulary about architecture, mettlenecks, etc.) emerges without the Review Lens to anchor it.

---

### C5: Review Lens Only (from profiles)

**Prompt context given:** Only the "Review Lens" sections from Dan Katz, Thomas Snyder, and Dana Young profiles — no names, no credentials, no design philosophies.

**Scores:**

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | Lens check (Snyder): "Is the entry point constructed or discovered?" The entry point is clearly the alibi gap table — it's designed to be encountered first, and the formatting confirms this. Constructed. (Dana): "Does the layout direct the solver, or leave them searching?" The hierarchical report structure directs confidently. Each suspect section follows the same template. No searching required. |
| Solvability | 2 | Lens check (Snyder): "Is there a puzzle here, or a procedure?" The alibi analysis, as written, is a procedure: read contradictions, note unaccounted windows, check overlap with theft window. The Critical Deduction section converts what should be a puzzle into explicit procedure. (Snyder): "Does the solve path hold one direction?" Yes — but only because the answer is given. (Katz): "Does every puzzle justify its slot?" The Critical Deduction section does not justify its slot — it occupies the slot where the solver's own deduction should live. |
| Elegance | 3 | Lens check (Snyder): "Is each element load-bearing?" Test individual suspect entries: all four are necessary. The service elevator evidence (item 3) is critical for the escape route logic. The key card log (item 4) is essential for Kessler's elimination. (Snyder): "Are there multiple valid solving paths that obscure the intended one?" Yes — solvers can reach Fontaine as the answer via the 3F placement alone, bypassing the Kessler elimination. Multiple routes to the answer exist. (Dana): "Is the extraction earning its step?" The underlined-word extraction is additive process, not additive meaning. It does not transform the solve. |
| Reading Reward | 2 | Lens check (Dana): "Does it work without the context?" Yes — fully solvable without noir knowledge. "Does the flavor text explain what the layout should have communicated?" The Critical Deduction section is flavor text explaining what the deduction layer should have communicated. This is Dana's exact failure mode: "If the intro text is doing the job the visual design was supposed to do, the puzzle has two problems: it over-scaffolds, and it exposes that the design didn't finish." EXACT match. |
| Fun | 3 | Lens check (Katz): "Would he want to solve this?" The alibi logic is genuinely interesting. The period setting is appealing. The Critical Deduction section makes the puzzle less engaging by solving it for you. The answer is good. Net: yes, with reservations. (Dana): "Is the answer arbitrary or inevitable?" SUSPECT is inevitable given the structure. Good. |
| Confirmation | 4 | Lens check (Snyder): "Can the solver confirm each step independently?" Each alibi gap has documentary evidence (receipts, staff testimony, key card logs). Every elimination is independently confirmable. Full marks for verification quality. (Katz): "Verify the Last Mile" implicit — the extraction is explicitly documented in the puzzle itself with position indices. Cannot fail. |
| **Total** | **18/30** | **FAIL** |

**Summary:** The Review Lens condition produces the most targeted and procedurally specific critique of the single-source conditions. Dana Young's lens catches the Critical Deduction section as a precise over-scaffolding failure mode ("flavor text doing the job the layout should have"). Snyder's lens on "procedure vs. puzzle" provides a clean diagnostic for the deduction layer. Katz's lens is less applicable here (hunt-level structural concerns don't transfer well to a single physical document) but still produces useful individual-puzzle diagnostics.

**Observed reviewer behavior:** Review lenses are immediately applicable without philosophical mediation. The reviewer can ask specific questions and get specific answers. The Dana Young lens is most productive for this puzzle type — her visual/layout concerns translate directly to document-puzzle critique. Snyder's lens on deduction quality is highly applicable. Katz's lens transfers partially (individual puzzle justification, Last Mile verification) but his strongest concerns (meta architecture, hunt scale, unlock systems) are irrelevant here. Domain mismatch is visible but doesn't prevent useful critique.

---

### C6: Full Profiles — Dan Katz, Thomas Snyder, Dana Young

**Prompt context given:** Complete profiles for all three reviewers (credentials, design philosophy, review lens).

**Three-reviewer panel evaluation:**

#### Dan Katz

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | The structural contract here is clear: a police report with an alibi analysis worksheet. The format promises that four suspects will be evaluated and one will be identified. It keeps that promise. Every section header signals its purpose. No false promises in the design. |
| Solvability | 2 | I am asking: does every element justify its slot? The Critical Deduction section does not. It occupies the slot where the solver's own meta-deduction should live. For a hunt puzzle, this would be like giving solvers a completed meta worksheet. The puzzle has a mettleneck: if you don't know how to reason about escape routes, you stall — so the author wrote the escape-route reasoning into the puzzle. That's the wrong fix. The fix is to make the escape route reasoning more accessible without narrating it. |
| Elegance | 3 | The extraction mechanism (7 underlined words → boxed letters) is structurally independent of the deduction. In hunt terms, this is a puzzle with two disconnected halves — a feeder (the deduction) and an extraction — that don't use each other's logic. An elegant design would make the extraction grow from the deduction. Who is the suspect? Extract from the evidence against them. Here, the extraction could have been from the words that constitute the key Kessler-elimination clue, not random distribution across the text. |
| Reading Reward | 2 | This puzzle doesn't transfer domain knowledge. A solver who finishes this puzzle knows nothing more about 1947 hotel crime investigation than when they started. They've applied labeled steps. The domain is setting, not substance. |
| Fun | 3 | The alibi grid is satisfying. The period voice is genuine. But I'm solving a procedure. The joy of a really good hunt puzzle is that you discover something — about the domain, about the mechanism, about yourself. This puzzle tells you what you're about to discover. |
| Confirmation | 4 | SUSPECT works. Good final-word design. |
| **Katz Total** | **18/30** | **FAIL** |

#### Thomas Snyder

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | The document has a precisely designed entry point: the worksheet at the bottom. The eye reaches the worksheet via the narrative accumulation above it — each suspect section is a deduction unit feeding the worksheet. This is constructed, not emergent. The visual structure communicates correctly. |
| Solvability | 2 | Snyder's Computer Test: can I write a 10-line script? Yes. Parse alibi gaps, check for overlap with 9:15-9:25, apply room-entry constraint to Kessler, output remaining suspects. This is a solvable search problem. There is a genuine deduction buried in it — the direction-of-travel logic for Kessler is non-trivial — but the Critical Deduction section writes the script for you. The puzzle fails the computer test not because a computer could solve it, but because the puzzle is written for a computer. "There is no puzzle here, only a procedure with period formatting." |
| Elegance | 3 | The solve path holds one direction (good). But there are multiple routes: you can eliminate Kessler via escape-route logic OR via the simple observation that he has the longest alibi gap (66 minutes) which should make him MORE suspicious, not less, creating a productive confusion that the puzzle then resolves — or should. The multiple-route issue is present. Is each element load-bearing? The service elevator evidence and key card log are load-bearing. The weather note ("Clear, 52°F, fire escape accessible") is not — it creates a potential escape route that is never referenced again. Either use it or cut it. |
| Reading Reward | 2 | The deduction sequence does not teach the solver anything about its own logic — about detective work, about alibi analysis, about the specific techniques used in 1940s police investigation. A solver who finishes this has applied labeled steps, not learned a technique. The puzzle did not teach what it claims to require. |
| Fun | 3 | The period document format is genuinely charming. The alibi contradictions are well-constructed — real documentary weight behind each contradiction (named witnesses, receipts, key card logs). This is careful work. The Critical Deduction section is the single most damaging editorial choice: it preempts the puzzle's best moment. |
| Confirmation | 5 | Extraction verification passes completely. Seven words, seven positions, seven letters, one thematic word. The extraction is clean, independently verifiable, and mechanically exact. The answer itself — SUSPECT — is appropriate and non-guessable from the topic alone. Strong finish. |
| **Snyder Total** | **20/30** | **FAIL** |

#### Dana Young

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | The visual grammar is consistent — every suspect entry uses the same four-part structure (statement, corroboration, contradiction, window). This is Dana's test: "Same element type, same meaning — everywhere." It holds. The layout directs the solver confidently through the document. |
| Solvability | 3 | The puzzle works without thematic context — yes, you can solve it without knowing noir conventions. It would be richer with that context only if the domain were doing structural work, which it isn't. "Does it work without the context?" passes in the wrong direction: the puzzle works without context, but it should be richer with context. It isn't. |
| Elegance | 2 | "Is the extraction earning its step?" No. The extraction (underlined words, boxed letters) adds process without adding meaning. It doesn't transform the solve — it appends a step after the solve. An earnest extraction would encode something about the detective work: perhaps the letters of the detective's deduction, or letters appearing only in the contradicting evidence sections. "Is the answer arbitrary or inevitable?" SUSPECT is inevitable thematically, but extracted arbitrarily (letters from seven randomly distributed words). The inevitability is in the word, not the mechanism. |
| Reading Reward | 2 | "Does the flavor text explain what the layout should have communicated?" Yes — the Critical Deduction section is flavor text that explains what the deduction structure should have communicated. This is the puzzle's central failure in Dana's terms: she would cut that section and ask the designer to rebuild the document so the deduction is discoverable without narration. The theme dissolves in the Critical Deduction section: the document stops being a police report and becomes a puzzle hint. |
| Fun | 3 | The period document form is committed and appealing. The physical-document format (designed for print) is appropriate for the puzzle hunt context. The reading experience of the alibi section is genuinely enjoyable. |
| Confirmation | 4 | The answer is inevitable in naming, if not in extraction mechanism. SUSPECT is the right word for the experience of solving this puzzle. |
| **Dana Total** | **18/30** | **FAIL** |

**Panel composite score:** (18 + 20 + 18) / 3 = **18.7/30 FAIL**

**Summary:** Full profiles generate the richest single-puzzle critique. Each reviewer contributes a distinct lens that catches distinct failures: Katz notes the structural incoherence of a two-layer puzzle whose layers don't communicate; Snyder identifies the Critical Deduction section as a procedure-writing failure (and catches the unmoored weather note as a non-load-bearing element); Dana identifies the extraction as extracting without earning and the Critical Deduction section as flavor text doing design work. The three evaluations converge on the same two failures — over-scaffolding and extraction-deduction disconnect — but from different angles.

**Observed reviewer behavior:** Full profiles produce differentiated, attributable critique. Each reviewer notices different things; the overlap is revealing (independent confirmation of key failures) and the divergence is informative (Katz: architectural incoherence; Snyder: automatable deduction; Dana: visual language breakdown). The domain mismatch (hunt designers reviewing a physical-document puzzle) is visible in Katz's review — his strongest concerns (meta architecture, unlock systems, hint economics) don't apply, and his useful contributions come from his individual-puzzle diagnostic tools rather than his hunt-level framework. Snyder and Dana transfer more cleanly because their lens is fundamentally about craft quality, not hunt scale.

---

### C7: Full Profiles + Design Principles

**Prompt context given:** Complete profiles (Dan Katz, Thomas Snyder, Dana Young) + all 11 design principles with names, quotes, and tests.

**Three-reviewer panel evaluation with principle cross-references:**

#### Dan Katz (+ principles)

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | Principle: Blame the Player ("Every clue is fair in retrospect"). Test: "Respect or resentment?" Full marks — every contradiction has named witnesses, documentary evidence, times. No solver can feel deceived. The puzzle structure is transparent. Principle: No Over-Scaffolding ("Worksheet that does all the work is not a puzzle"). Test: "Remove — still a puzzle?" The worksheet itself (column headers, suspect names, blank cells) can be removed and the puzzle still functions — solvers can do the analysis in their heads. The worksheet is appropriate space, not over-scaffolding. But the Critical Deduction section fails this test: remove it and the puzzle is harder but better. Keep it and it becomes a scaffold that does the work. |
| Solvability | 2 | Principle: The Riven Standard ("The puzzle IS what the field does"). Test: "Practitioner recognizes their own work?" A detective would: read witness statements, identify contradictions, map timeline, trace escape route, eliminate. The puzzle gives them the reading but performs the trace-and-eliminate step for them. The practitioner's core skill (deductive synthesis) is denied. Principle: One Aha ("One aha is fun"). Test: "Name the single aha." The intended aha is: Kessler went to his room at 9:28, not out of the building — therefore he's not the thief. This is the right aha. It's named and explained in the puzzle text. The aha is described, not discovered. |
| Elegance | 3 | Principle: Interlock, Not Independence ("Independent lookups = quiz"). Test: "Any order, no advantage?" The four suspects are not interchangeable. The service elevator clue (item 3) is required to understand the escape route; without it, the Kessler elimination has no basis. The suspects interlock through the shared 9:15-9:25 window and the single elevator. This is structurally sound — the puzzle passes Interlock. The extraction does NOT interlock with the deduction. SUSPECT could be extracted from any seven words in the document; it has no relationship to which suspect is identified. |
| Reading Reward | 2 | Principle: Reading Reward ("Genuine domain engagement"). Test: "Keyword search sufficient?" Yes. "no witness" + time gaps does all the necessary work. A solver using keyword search misses nothing. The domain is not rewarding attention or knowledge. No Computation Without Deduction: no computation required (no arithmetic). Passes this test. |
| Fun | 3 | Principle: Surprise the Answer ("Pause, not nod"). Test: "Guessable from topic?" Not immediately — SUSPECT is not the obvious answer to "police report puzzle." You'd more likely guess a name (FONTAINE, KESSLER) or a verdict (GUILTY). SUSPECT surprises appropriately. |
| Confirmation | 5 | Principle: Verify the Last Mile ("Mechanism and extraction separate"). Test: "Trace letter by letter." S (Saturday, L1), U (annual, L4), S (persons, L4), P (approximately, L3), E (evening, L1), C (mechanical, L4), T (question, L5). Seven verifications, all pass. The extraction mechanism is fully documented within the puzzle and independently verifiable. |
| **Katz Total** | **19/30** | **FAIL** |

#### Thomas Snyder (+ principles)

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Principle: Blame the Player. All evidence sourced. Principle: No Over-Scaffolding. The worksheet appropriately provides structure without performing deduction. Snyder confirms: the entry point (worksheet) is constructed and clear. Visual structure communicates correctly. |
| Solvability | 2 | Principle: Snyder's Computer Test ("Computer can solve?"). Test: "10-line script." A script can: (1) read alibi gaps, (2) check overlap with 9:15-9:25, (3) identify suspects with proximity to gallery, (4) apply key card + escape route logic to Kessler. The Critical Deduction section is a written version of this script. If the puzzle's core mechanism can be scripted, the puzzle hasn't finished being designed. Principle: Solving = Proving Understanding ("More knowledge after?"). Test: "More knowledge after?" No. A solver who completes this knows they identified the suspect, but they don't know more about detective work, alibi analysis technique, or the logical structure of timeline reasoning. The puzzle didn't teach what it used. |
| Elegance | 3 | Principle: No Computation Without Deduction. No computation present — passes. Principle: Interlock, Not Independence. The suspects interlock; the extraction does not interlock with the deduction. Two separate layers that don't use each other's logic. Snyder would ask: are there multiple valid solving paths? Yes — you can arrive at Fontaine (seen on 3F, could descend in time) without processing the Kessler escape-route logic at all. The designer intended two routes, but only one is elegant. |
| Reading Reward | 2 | Principle: Reading Reward. Test: "Keyword search sufficient?" Yes. But a deeper failure: the alibi contradiction logic does not illuminate the noir/detective domain. Reading this doesn't teach period police procedure, hotel investigation protocols, or the conventions of 1947 witness testimony. The domain is a wrapper. |
| Fun | 3 | The physical document format is well-executed. The alibi construction is meticulous — each contradiction has real evidential weight. The pleasure of reading these is genuine. The Critical Deduction section is the wrong kind of gift: it gives you the answer before you earn it. |
| Confirmation | 5 | Extraction verified per Last Mile principle. Every letter independently traceable. Document includes its own verification instructions, which is appropriate for a physical-document format (the solver can check their work). |
| **Snyder Total** | **20/30** | **FAIL** |

#### Dana Young (+ principles)

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | Principle: Blame the Player. The visual grammar holds: consistent four-part structure per suspect. Principle: No Over-Scaffolding. Dana's test: "Remove — still a puzzle?" Remove the worksheet column labels: still a puzzle. Remove the Critical Deduction section: still a puzzle, and a better one. Remove the "Unaccounted window" summary line per suspect: still a puzzle (the solver can compute it from the times). The summaries are borderline scaffolding but defensible as format-appropriate (real police reports summarize). |
| Solvability | 3 | Principle: The Riven Standard. "The puzzle IS what the field does." For a physical dossier puzzle, the field is document-reading and evidence synthesis. Does the puzzle instantiate those practices? Partially — the evidence is genuine and requires synthesis. The failure is that the synthesis is performed by the puzzle, not the solver. Dana would ask: where does the theme dissolve? It dissolves precisely at the Critical Deduction section, where the document voice shifts from reporting officer to puzzle author explaining the answer. |
| Elegance | 2 | Principle: One Aha. The single aha (Kessler's return-to-room proves he's not the thief who exited via basement) is genuinely elegant. But it is described. Principle: Reading Reward. Dana's lens: "Is the extraction earning its step?" The extraction — seven underlined words, random positions, boxed letters → SUSPECT — does not earn its step. It is a tax with no transformation. A better extraction would encode something discovered during the deduction: perhaps the first letters of the four suspect names in order of elimination, or letters from the specific words that constitute the key eliminating evidence against each suspect. SUSPECT as an extraction destination is right. SUSPECT as extracted from these seven words is arbitrary. |
| Reading Reward | 2 | Principle: Reading Reward ("Genuine domain engagement"). The noir document flavor is present but the domain adds no structural value. Dana would ask: "Does it work without the context?" Yes, fully. "Is it richer with the context?" Marginally — the period voice is charming. But the mechanics are domain-agnostic. |
| Fun | 3 | The physical document form is committed. The period voice is sustained without anachronism. The puzzle's printed format (document + worksheet) is appropriate for the dossier hunt context. The extraction answer SUSPECT is the right word for the experience. |
| Confirmation | 4 | Principle: Verify the Last Mile. "Mechanism and extraction separate?" Yes — completely. Dana would note: the separation is a feature of the format (physical document + answer box) but also a weakness of the design (the extraction learns nothing from the deduction). |
| **Dana Total** | **17/30** | **FAIL** |

**Panel composite score:** (19 + 20 + 17) / 3 = **18.7/30 FAIL**

**Key principle firings across the panel:**

| Principle | Fired by | Finding |
|-----------|----------|---------|
| The Riven Standard | Katz, Snyder, Dana | FAIL — Critical Deduction section denies the practitioner the detective's core move |
| Solving = Proving Understanding | Snyder | FAIL — solver learns nothing new about detective work or the domain |
| No Over-Scaffolding | Katz, Dana | PARTIAL FAIL — worksheet OK, Critical Deduction section is over-scaffolding |
| One Aha | Katz | PARTIAL FAIL — the aha is correct but narrated, not discovered |
| Reading Reward | All three | FAIL — keyword search sufficient; domain is not structural |
| Interlock, Not Independence | Katz, Snyder | PARTIAL FAIL — suspects interlock, extraction does not interlock with deduction |
| Verify the Last Mile | All three | PASS — extraction fully verified |
| Snyder's Computer Test | Snyder | FAIL — core deduction is scriptable; Critical Deduction section is the script |
| Blame the Player | All three | PASS — all evidence fairly sourced |
| Surprise the Answer | Katz | PASS — SUSPECT is not guessable from topic |
| No Computation Without Deduction | Snyder | PASS — no arithmetic |

**Summary:** Full profiles + principles produces the highest-fidelity evaluation. The principle tests operationalize the reviewers' philosophical commitments — without tests, C4 and C5 apply philosophy and lens intuitively; with tests, C7 applies them procedurally. The "fired principles" table is the key diagnostic artifact: it shows at a glance which principles are violated, by whom, and with what finding. The combination also surfaces the most important editorial recommendation: cut the Critical Deduction section and trust the deduction structure to carry the puzzle.

**Observed reviewer behavior:** The richest, most actionable output of any condition. Reviewers use principle vocabulary to anchor specific claims ("the puzzle passes Interlock but fails Riven Standard"). The editorial recommendation (remove Critical Deduction section) is derived from three independent sources (Katz via Over-Scaffolding, Snyder via Computer Test, Dana via flavor-text substitution). Domain mismatch is visible but managed: each reviewer applies their lens to what is applicable and notes when hunt-specific concerns don't transfer.

---

## Summary Table

| Condition | Context | Total Score | Composite | Pass? | Key Finding | Failure Modes Named |
|-----------|---------|-------------|-----------|-------|-------------|-------------------|
| C0 | "Expert reviewer" | 20/30 | — | FAIL | Over-scaffolded; extraction thin | None (impressionistic) |
| C1 | + rubric | 19/30 | — | FAIL | Domain is decorative (rubric forces Reading Reward) | "Reading Reward" category |
| C2 | Principle names + quotes | 18/30 | — | FAIL | Riven Standard broken; extraction unverified | Riven Standard, One Aha, Reading Reward |
| C3 | + principle tests | 19/30 | — | FAIL | Computer-solvable; extraction verified; domain confirmed decorative | Riven Standard + test, Snyder's Computer Test, Verify the Last Mile |
| C4 | Design philosophy only | 18/30 | — | FAIL | Theme ≠ mechanic; two incoherent layers | Mechanic-as-theme framework |
| C5 | Review lens only | 18/30 | — | FAIL | Flavor text substituting for design; procedure not puzzle | Dana's flavor-text diagnostic, Snyder's procedure test |
| C6 | Full profiles | Panel: 18.7/30 | 18.7/30 | FAIL | Three differentiated diagnoses; convergent on over-scaffolding + extraction disconnect | Named reviewer frameworks, domain transfer notes |
| C7 | Full profiles + principles | Panel: 18.7/30 | 18.7/30 | FAIL | Full principle firing table; strongest editorial recommendation | All applicable principles, fired/pass table |

---

## Cross-Condition Analysis

### Score Stability

Scores are consistent across conditions: 18–20/30 for single-reviewer conditions, 18.7/30 for panel conditions. The puzzle has genuine moderate-quality characteristics that all conditions agree on. No condition significantly inflated or deflated the score. Score stability suggests the puzzle's quality profile is robust to reviewer framing — the puzzle passes what it passes (Clarity, Confirmation) and fails what it fails (Reading Reward, Solvability) regardless of context depth.

### The Critical Deduction Section: Unanimous Diagnostic

Every condition from C2 onward independently identifies the Critical Deduction section as the primary failure. C0 notes it vaguely ("deflates the solver's aha"); C1 attributes it to the over-scaffolding rubric category; C2 identifies it as a Riven Standard violation; C3 confirms it via the Snyder Computer Test; C4 names it via the philosophy of constructor restraint; C5 identifies it via Dana's flavor-text diagnostic; C6 reaches it from three independent reviewer angles; C7 fires three principles against it simultaneously. This is a robust finding: the editorial fix is clear and supported by all conditions.

### Reading Reward: Consistent Failure

All conditions score Reading Reward at 2/5. This is remarkable because the metric is named explicitly only in C1 (rubric) and the principle vocabulary, yet every condition without those labels still identifies the domain-as-costume problem. The failure is observable enough that it doesn't require principle framing to detect — but principle framing (C2+) allows it to be stated precisely and tested.

### Domain Transfer: Puzzle Hunt Designers on a Physical Document

The experiment's key transfer question: do hunt-design specialists (Dan Katz, Thomas Snyder, Dana Young) evaluate a physical-document logic puzzle validly? Finding: yes, with nuance.

- **Dan Katz's** strongest lens (meta architecture, hunt scale, unlock systems) doesn't apply. His useful contributions come from individual-puzzle diagnostics: structural coherence, "does every element justify its slot," Verify the Last Mile. He performs about 70% as well as in his native format.
- **Thomas Snyder's** lens transfers most cleanly. His core concerns — constructed entry points, load-bearing elements, procedure vs. puzzle, the Computer Test — apply directly to any logic-deduction puzzle regardless of format. The police report is, structurally, a constrained deduction problem. Snyder is fully at home.
- **Dana Young's** lens transfers well, particularly her visual/layout diagnostics and the flavor-text-as-design-failure test. The physical document format is visually designed, and her concern with whether the layout directs the solver applies directly. Her weakest transfer: her meta-feeder concern ("does the meta feel earned by the feeders?") is irrelevant for a standalone puzzle.

**Transfer quality rank:** Snyder > Dana > Katz

### Context Depth vs. Evaluation Quality: Monotonic Relationship?

No. The relationship is not strictly monotonic:

- C0 → C1: Adding a rubric improves named-category articulation significantly. +1 quality step.
- C1 → C2: Adding principle names improves vocabulary and specificity. +1 quality step.
- C2 → C3: Adding tests improves procedural precision. +1 quality step.
- C3 → C4: Switching to philosophy-only (removing tests) produces comparable quality through a different mechanism (interpretive rather than procedural). Lateral.
- C4 → C5: Switching to lens-only produces comparable quality, slightly higher procedural precision than C4 but lower than C3. Roughly lateral.
- C5 → C6: Adding full profiles to lens adds credentials (which affects authority more than substance) and design philosophy (which adds interpretive richness). +0.5 quality steps — differentiated voices appear.
- C6 → C7: Adding principles to full profiles produces the highest-quality output. The principle firing table is the most useful editorial artifact generated. +1 quality step.

**Pattern:** C3 and C7 produce the highest-quality evaluations via different mechanisms. C3 (principles + tests, no persona) produces rigorous procedural critique. C7 (full profiles + principles) produces procedurally rigorous critique with differentiated reviewer perspectives. For a single reviewer, C3 is the highest value-per-word condition. For a panel, C7 is the ceiling.

### The Minimum Viable Context Threshold

C1 (rubric) is the minimum viable condition. C0 (baseline) fails to name the domain-as-costume problem explicitly; C1 names it ("Reading Reward" category) even without principle vocabulary. However, C1 alone cannot produce actionable editorial recommendations — it can score but not prescribe. The minimum condition for actionable prescriptions is C2 (principle names + quotes), which generates vocabulary for naming failures in ways that imply fixes.

### What Full Profiles Add Beyond Lens or Philosophy Alone

The value of the full profile (C6 vs. C4 or C5) is:
1. **Attribution**: critique can be attributed to a specific reviewer with known credibility and specific expertise history, which affects how it is weighted
2. **Differentiation**: three distinct reviewers naturally produce non-redundant critique even for the same puzzle
3. **Convergence as signal**: when three reviewers with different philosophies and lenses independently identify the same failure, that convergence is stronger evidence than any single framing's diagnosis

The full profiles do not add substantially to the *accuracy* of the diagnosis (C3 already finds the same failures) but they add to the *confidence* of the diagnosis and the *richness* of the editorial prescription.

---

## Editorial Recommendations Derived from Study

All conditions converge on the same primary recommendation, stated with increasing precision from C0 to C7:

**Primary:** Remove or substantially revise the Critical Deduction section. The analysis of Kessler's escape-route inconsistency is the puzzle's central aha. It must be discoverable, not narrated. Revision options:
- Remove the section entirely and trust the alibi grid + evidence to carry the deduction
- Replace it with a blank "Cross-reference your findings" prompt that names the parameters (service elevator direction, key card timing, room location) without performing the logic
- Encode the escape-route clue only in the Physical Evidence section and let the worksheet drive the synthesis

**Secondary:** Integrate the extraction with the deduction. SUSPECT as an answer is right. Seven underlined words distributed across the document is arbitrary. Consider: letters from the evidence items that specifically eliminate Kessler, or letters from the exact timestamps in the key card log, or letters indexed from the four suspects' names by their elimination order. The extraction should be earned by the deduction.

**Tertiary (Snyder-specific):** The weather note ("Clear, 52°F. The north-face fire escape was accessible") is not load-bearing. Either use the fire escape as a route in the escape-route logic, or cut the note. Non-load-bearing elements at Snyder's standard are failures of editing.

---

*Study complete. All conditions documented.*
