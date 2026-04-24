# Ablation Study: Reviewer Context Depth
## Puzzle: P3 — EXHIBIT C (Architectural Floor Plan), Grand Larceny
## Answer: TIME (verified via route markers 5→E, 9→I, 13→M, 20→T)
## Date: 2026-02-28

---

## Puzzle Summary (for reference)

**P3 — EXHIBIT C — Architectural Floor Plan** is a physical spatial puzzle set in a 1947 noir hotel. Solvers receive a 4-floor floor plan (Basement, 1F, 2F, 3F) marked with circled reference numbers at specific locations. They must trace the thief's logical route through the building using spatial reasoning and architectural clues (connecting door, locked vs. open access), collecting four marker numbers (5, 9, 13, 20) at each stop, then convert those to letters (A=1) to spell TIME.

The puzzle IS a real floor plan — drafted in ASCII with architectural notation, room numbers, occupant names, access notes, and a vertical access summary table. The worksheet scaffolds route-logging but does not instruct what logic to use.

---

## Rubric (all conditions)

| Dimension | Description | Max |
|-----------|-------------|-----|
| Clarity | Can the solver understand what to do? | 5 |
| Solvability | Is the logic sufficient to force a unique solution? | 5 |
| Elegance | Does the mechanic respect solver intelligence? | 5 |
| Reading Reward | Does close reading pay off? | 5 |
| Fun | Is the experience enjoyable vs. frustrating? | 5 |
| Confirmation | Does the answer feel earned at the end? | 5 |
| **Total** | | **30** |

Pass threshold: **≥ 22 / 30**

---

## C0 — Baseline: "You are an expert reviewer."

**Context injected:** "You are an expert reviewer."

**Reasoning:**

Without principles, personas, or a rubric, a generic "expert" reviewer applies intuitive heuristics. The floor plan format is legible; the route-tracing task is clear; the conversion mechanism (A=1) is explicitly printed on the page. The reviewer likely approves the clarity of instruction. However, without a framework for what "solvability" means in a hunt context, the reviewer may not probe whether any step in the route is genuinely forced vs. ambiguous.

The puzzle's key vulnerability — the solving logic requires inferring that the alley exit is the entry point, that the service elevator is the vertical access (staff key = thief had key), and that the linen closet's connecting door is the gallery escape route — depends on reading the access notes carefully. A generic expert may not test whether a solver could validly choose a different route.

The answer mechanism (anagram of E, I, M, T = TIME) is not discoverable except by assembling the letters, and the worksheet prompts this explicitly. A baseline reviewer is likely to score this reasonably well across clarity and fun, but may not flag the ambiguity in route logic.

**Scores:**

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | Task is clearly stated; conversion mechanism explicitly printed |
| Solvability | 3 | A generic reviewer may not test whether the route is uniquely forced |
| Elegance | 3 | No framework for evaluating whether scaffolding crosses into over-instruction |
| Reading Reward | 3 | Likely noted that detail pays off; not rigorously tested |
| Fun | 4 | Format is distinctive and atmospheric; reviewer appreciates noir context |
| Confirmation | 3 | TIME as answer feels satisfying; reviewer may not probe whether it feels inevitable |
| **Total** | **20 / 30** | **FAIL (−2 from pass threshold)** |

**Failure mode:** Under-scores solvability because the reviewer doesn't probe the branching route logic. Misses the over-scaffolding concern because no principle flags it. Confirmation scored generously without testing whether the anagram arrival feels earned.

---

## C1 — Rubric Added

**Context injected:** "You are an expert reviewer." + scoring rubric (6 dimensions, 1-5 each, /30, pass ≥ 22).

**Reasoning:**

The rubric forces the reviewer to assign scores across six distinct dimensions rather than producing a holistic impression. This immediately raises the quality of evaluation: "Solvability" now requires a dedicated assessment rather than being bundled into general approval. The reviewer is prompted to ask explicitly whether the puzzle can be solved to a unique answer.

For this puzzle, the rubric changes behavior on Solvability: the reviewer is more likely to probe whether each route stop is forced. On Elegance: the worksheet format may now trigger a question about over-scaffolding. On Reading Reward: the reviewer must assess whether close reading of the access tables, occupant names, and "locked from gallery side" note actually pays off — which it does (the connecting door is the key escape mechanism, only discoverable by reading the note carefully).

The rubric also anchors Confirmation more precisely. TIME is satisfying as an answer — it names something the thief literally stole from their movements through the hotel (duration, timing, the heist schedule). A rubric-focused reviewer is more likely to give this credit.

The rubric does not solve the over-scaffolding concern — the reviewer still lacks a principle that names it — but the six-dimension breakdown catches issues the baseline reviewer bundled.

**Scores:**

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | Task explicit; rubric prompts more careful read of instruction density |
| Solvability | 3 | Rubric forces the question; reviewer still uncertain whether route is unique |
| Elegance | 3 | Worksheet may feel over-instructed; rubric raises the question but provides no framework |
| Reading Reward | 4 | Rubric directly prompts this; connecting door note and access table are genuine payoff |
| Fun | 4 | Noir format scores well; atmospheric distinctiveness rewarded |
| Confirmation | 4 | TIME feels resonant; rubric prompts reviewer to evaluate, and it passes |
| **Total** | **22 / 30** | **PASS (threshold exactly)** |

**Marginal pass.** The rubric elevates Reading Reward and Confirmation but doesn't resolve the core solvability concern. The reviewer passes the puzzle on the strength of its atmosphere and extraction, while leaving the route-logic ambiguity unaddressed.

---

## C2 — Principle Names + Quotes (No Persona)

**Context injected:** C1 rubric + 11 design principle names and quotes. No reviewer persona.

**Reasoning:**

Now the reviewer has named vocabulary. Each principle becomes a test the reviewer can apply:

- **The Riven Standard** ("The puzzle IS what the field does"): The floor plan IS an actual floor plan. An architect or detective would use this document to trace a route. Pass — strongly.
- **Solving = Proving Understanding**: After solving, the solver understands the hotel's access architecture — which routes are staff-only, which doors connect rooms not adjacent by hallway. The spatial knowledge is real. Pass.
- **Blame the Player**: Every clue needed to solve is present in the document. The connecting door note, the access table, the marker positions — all visible. The entry point (alley exit, basement, near marker 5) is the only exterior access without a visible observer. Pass — narrowly.
- **No Over-Scaffolding**: The Route Log worksheet is explicit about the format (4 stops, 4 markers, convert to letters). This is scaffolding. The principle asks: remove the worksheet — is it still a puzzle? YES — the floor plan and marker key stand alone. The worksheet provides recording space, not solving logic. Marginal pass.
- **Surprise the Answer**: Can a solver guess TIME from knowing this is a heist floor plan puzzle? Unlikely. TIME is not a "heist" word; it arrives as an anagram of the four marker letters. Surprise maintained. Pass.
- **One Aha**: The single aha is the connecting door — the linen closet (206) connects to the Sargent Gallery (200) via a door locked from the gallery side. The thief unlocked it from the gallery, slipped into the linen closet, and exited without using the hallway. Everything else in the route is derived from access logic. The aha is singular and clear. Pass.
- **Reading Reward**: The connecting door note is subtle: "NOTE: The thin line (┃) between rooms 200 and 206 indicates a connecting door. Per hotel records, this door is typically locked from the gallery side." A solver who keyword-searches "connecting" finds nothing useful — they need to read the full NOTE. Reading reward: high. Pass.
- **No Computation Without Deduction**: The only arithmetic is A=1 number-to-letter conversion, which is deductively determined (the route forces specific markers; the conversion table is explicit). No arbitrary computation. Pass.
- **Snyder's Computer Test**: Can a 10-line script solve this? NO — the script would need to infer the thief's entry point, know which access points are available to an outsider at 2 AM, understand that the alley exit is the only unobserved exterior access. This is spatial and inferential reasoning. Pass.
- **Interlock, Not Independence**: The four route stops build on each other — you cannot determine Stop 3 (linen closet) without having first established Stop 2 (gallery), and you cannot establish the exit without having established the escape. Linear dependency. Pass.
- **Verify the Last Mile**: TIME — T (20=T), I (9=I), M (13=M), E (5=E). 20→T: ✓. 9→I: ✓. 13→M: ✓. 5→E: ✓. Anagram of E,I,M,T = TIME: ✓. Pass.

Most principles pass. The principle names give the reviewer specific vocabulary to articulate what works. The Elegance dimension is now more precisely scored because No Over-Scaffolding has a name. The one lingering concern: the solvability uniqueness of the route is still not rigorously tested by these principles — Interlock addresses ordering but not whether alternative routes could produce different markers.

**Scores:**

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | Riven Standard confirms document form is appropriate; task reads naturally |
| Solvability | 4 | Interlock and Computer Test both pass; logical route is forced by access constraints |
| Elegance | 4 | No Over-Scaffolding analysis clears the worksheet; One Aha is clean |
| Reading Reward | 5 | Principle highlights it directly; connecting door note is genuine reward |
| Fun | 4 | Blame the Player + Surprise the Answer both pass; no resentment triggers |
| Confirmation | 4 | Verify the Last Mile passes cleanly; Surprise the Answer satisfied |
| **Total** | **25 / 30** | **PASS** |

**Significant jump from C1.** Principle vocabulary elevates Solvability (+1), Elegance (+1), and Reading Reward (+1). The reviewer is now naming what works rather than feeling it.

---

## C3 — Principle Names + Quotes + Tests

**Context injected:** C2 content + specific test questions for each principle.

**Reasoning:**

The tests operationalize each principle into a binary pass/fail check. This is where the reviewer's behavior changes most sharply within the no-persona conditions: instead of asking "does this feel like a Reading Reward puzzle," the reviewer asks the specific test question: "Does keyword search find the answer, or does close reading pay off?"

Running each test:

- Riven Standard test ("Practitioner recognizes their own work?"): A real architect or crime scene investigator reading this document would immediately understand it as a survey floor plan. The notation (ELV for elevator, room numbers, access notes) is correct. The document form is authentic. YES. Pass.
- Solving = Proving Understanding test ("More knowledge after?"): The solver now knows this hotel's access architecture in detail — which floors have fire escapes, which rooms connect via service doors, which access points require staff keys. Real spatial knowledge gained. Pass.
- Blame the Player test ("Respect or resentment?"): Every inference the solver is asked to make is fair. The alley exit being the entry point is supported by: it's the only exterior access without an observable lobby or front desk, it's adjacent to the basement (unmonitored), and marker 5 is placed there by police request (suggesting investigative relevance). No ambush. Pass.
- No Over-Scaffolding test ("Remove worksheet — still a puzzle?"): Remove the Route Log entirely. The floor plan, reference marker key, and vertical access summary remain. The puzzle is still solvable — the markers are on the map, the conversion is implicit (A=1 is standard), the task is traceable from context. The worksheet is workspace, not instruction. PASS.
- Surprise the Answer test ("Guessable from topic?"): The puzzle is about a heist floor plan. Plausible guesses for the answer might include ESCAPE, ROUTE, HEIST, ALLEY — none of which land. TIME is not guessable from theme. Pass.
- One Aha test ("Name the single aha"): The connecting door between the Sargent Gallery (200) and the Linen Closet (206), locked from the gallery side. The thief unlocked it while in the gallery, moved to the linen closet, and exited to the hallway without being seen re-entering the main gallery corridor. This is the puzzle's single insight. Clean. Pass.
- Reading Reward test ("Keyword search sufficient?"): A solver scanning for "route" finds the Route Log (scaffolding, not insight). A solver scanning for "exit" finds the fire escape reference. The connecting door is discovered only by reading the full NOTE block, which appears below the 2F diagram as plain prose. Keyword search fails to surface it. Reading rewarded. Pass.
- No Computation Without Deduction test ("Remove instructions — still a puzzle?"): Remove the "Convert each marker number to a letter (A=1, B=2...)" instruction. The solver still has 4 numbers (5, 9, 13, 20) derived from the route. A=1 is a well-known puzzle convention. The deduction produces letters that form an anagram. The instruction is redundant for a puzzle-literate solver. Pass.
- Snyder's Computer Test ("10-line script?"): Requires: spatial inference about thief capabilities, understanding of social context (who is observed where at 2 AM), knowledge of hotel norms (connecting doors, staff access). Not scriptable without AI-level reasoning. Pass.
- Interlock test ("Any order, no advantage?"): Stop 1 must be entry point before Stop 2 can be gallery access, which must precede Stop 3 (escape route) and Stop 4 (building exit). Linear dependency. No advantage to any other order. Pass.
- Verify the Last Mile test ("Trace letter by letter"): Stop 1: Alley exit (B02), Marker 5, → E. Stop 2: 2F Hall/Gallery (200), Marker 9, → I. Stop 3: Linen Closet (206), Marker 13, → M. Stop 4: Roof Access (303), Marker 20, → T. E-I-M-T anagram = TIME. Verified.

**One concern surfaced by tests:** The test "Practitioner recognizes their own work?" surfaces a small defect: the architect notation is not quite consistent. Marker 9 appears TWICE in the reference key (once in 200, once in 2F-HALL), which is flagged in the key table. This is intentional — both locations get marker 9 in the puzzle — but it creates a potential solve ambiguity: should Stop 2 be the gallery (200, marker 9) or the 2F Hall junction (also marker 9)? The route log asks for 4 stops; if the solver routes through BOTH marker-9 locations, they may try to use 9 twice, which breaks the 4-letter answer. The tests reveal this more clearly than principles alone.

**Scores:**

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | Tests confirm document form authentic; marker key ambiguity (dual 9) is minor but real |
| Solvability | 3 | Dual marker-9 ambiguity surfaces in testing; solver may route through both, producing 5 stops |
| Elegance | 4 | No Over-Scaffolding and One Aha both pass their tests cleanly |
| Reading Reward | 5 | Test confirms keyword search fails; reading the NOTE block is necessary and rewarded |
| Fun | 4 | All fairness tests pass; one structural concern but doesn't break experience |
| Confirmation | 5 | Last Mile verification passes letter by letter; TIME is unambiguous |
| **Total** | **25 / 30** | **PASS** |

**Same total as C2, but different distribution.** Tests surface the dual-marker-9 concern that drops Solvability by 1, but drive Confirmation up to 5 via the letter-by-letter check. The tests make the reviewer more precise in their concerns, not just more generous.

---

## C4 — Design Philosophy from Profiles Only

**Context injected:** C1 rubric + Design Philosophy sections from Dan Katz, Thomas Snyder, and Dana Young. No Review Lens sections. No principles.

**Reasoning:**

Design Philosophy describes how these designers think about puzzles — their convictions, their priorities, their standards. This is different from reviewing: it's identity-level belief, not applied evaluation. Using it for review means the evaluator is applying the spirit of these designers' worldviews without their specific diagnostic tools.

**Katz Design Philosophy applied:** He thinks about puzzles as contracts. The floor plan puzzle makes an implicit promise: use the document to trace the route, and the answer will be fair. He would check whether that contract is honored. He would also note that this puzzle's size matches its audience — 4 stops, clear conversion, one aha — and that the structural architecture is proportionate. He would note whether the puzzle "justifies its slot" (does the hunt need a floor plan puzzle? Yes — it's the only spatial puzzle in a 4-puzzle set). The Katz philosophy prompts architectural thinking, which this puzzle rewards: the design is deliberate.

**Snyder Design Philosophy applied:** "If a computer can generate your puzzle, your puzzle isn't finished." The floor plan cannot be computer-generated — the access logic, the connecting door placement, the marker selection, and the route are hand-designed choices. The constructor's hand is visible: the NOTE block about the connecting door is placed where a solver will read it after the 2F diagram, requiring them to have processed the map before the reveal. Theme and structure are the same thing — the floor plan IS the mechanic, not a wrapper around it. Snyder's philosophy elevates this puzzle significantly.

**Young Design Philosophy applied:** "She starts with a world." The Grand Hotel is a world — the document has occupant names, case numbers, a surveyor's signature, a date. She would approve of the mechanic being chosen because it IS the theme (a detective reading a floor plan is not playing a game; they are doing their job). She would ask whether the answer names the experience — does TIME name what the solver was doing? The solver was tracing a route through time (the thief's chronological path), and the answer names the temporal dimension of that trace. She would approve.

Without the Review Lenses, the evaluator is thinking about this puzzle from first principles rather than using specific diagnostic questions. This produces warmer, more holistic evaluations — picking up the gestalt quality of the design — but missing specific failure modes.

**Scores:**

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | Katz's contract framing: the document makes a legible promise. Honored. |
| Solvability | 3 | Philosophy doesn't supply diagnostic questions for route uniqueness; left at rubric default |
| Elegance | 5 | Snyder's hand-design test and Young's world-first philosophy both strongly endorse this |
| Reading Reward | 4 | Young's visual precision philosophy notices the NOTE block; not rigorously tested |
| Fun | 4 | Katz's audience-match check passes; proportionate design |
| Confirmation | 4 | Young's "answer names the experience" test passes for TIME |
| **Total** | **24 / 30** | **PASS** |

**Philosophy raises Elegance but leaves Solvability soft.** The three designers' worldviews collectively affirm the puzzle's fundamental design integrity — the form-equals-mechanic insight, the hand-designed route, the world-embedded experience — but their philosophies don't generate the specific probe questions needed to test whether the route is uniquely forced or whether the dual marker-9 is a problem.

---

## C5 — Review Lens from Profiles Only

**Context injected:** C1 rubric + Review Lens sections from Dan Katz, Thomas Snyder, and Dana Young. No Design Philosophy sections. No principles.

**Reasoning:**

Review Lenses are diagnostic — specific questions these reviewers apply to identify failure modes. This is the most directly applicable context for evaluation. The shift from Design Philosophy is: we now have expert probes rather than expert worldviews.

**Katz Review Lens applied:**
- "Does every puzzle justify its slot?" The floor plan puzzle is the only spatial puzzle in the set — it's load-bearing for variety. Justified.
- "Is the narrative encountered while solving or reported afterward?" The 1947 date, the occupant names, the case number — all encountered while reading the map, not reported separately. Pass.
- "Would he want to solve this?" The format is novel within the set; the mechanic is distinct from P1 (word extraction) and P2 (numerical extraction). He would solve this. Pass.
- He would also probe: "Are the mechanisms varied enough?" — P3's spatial trace is mechanically distinct from all other puzzles in the set. Pass.

**Snyder Review Lens applied:**
- "Is the entry point constructed or discovered?" The alley exit (Marker 5, B02) is the only exterior access without a staffed observation point (no front desk, no lobby, no listed occupant). The entry point is constructed — the designer chose which access point gets marker 5. Pass.
- "Does the solve path hold one direction?" The route is linear: entry → access point → gallery → escape → exit. Each step is derived from the previous. Pass.
- "Is each element load-bearing?" The NOTE about the connecting door is essential — remove it and the escape route becomes invisible. The access table is essential — without it, the solver cannot determine which routes require staff keys. Each element carries weight.
- "Is there a puzzle here, or a procedure?" The moment of genuine deduction is the connecting door: inferring that the thief used it requires synthesizing (1) the thief didn't use the main hallway (too exposed), (2) the connecting door exists, (3) it was locked from the gallery side (thief was in the gallery and could unlock it). Three facts combined. Deduction, not procedure. Pass.
- **KEY CONCERN:** "Does the solve path hold one direction?" — Snyder's lens surfaces the dual marker-9 concern more sharply than the principles test. He would ask: if the solver routes through 2F Hall (marker 9) on the way to the gallery AND through the gallery (also marker 9), do they get two 9s? The route log provides 4 blank stops. If the solver fills: B02(5) → 2F-Hall(9) → Gallery(9) → Linen(13), they skip Stop 4 and can't reach marker 20 (Roof Access). Snyder would flag this as a uniqueness problem in the solve path.

**Young Review Lens applied:**
- "Does the visual grammar hold consistently?" The ASCII floor plans use consistent notation (ELV for elevator, ┃ for connecting door, ⑤⑨⑬⑳ for markers). One inconsistency: marker 9 appears twice in the reference key with different locations listed, which creates ambiguity in the visual grammar. Young would flag this immediately.
- "Does the layout direct the solver, or leave them searching?" The floor plans proceed top-to-bottom (3F → 2F → 1F → B), but the route runs B → 1F → 2F → 3F. The solver must read the maps in reverse order from their presentation. Young would ask if this creates unnecessary friction.
- "Does it work without the context?" Yes — the document is self-contained. No prior knowledge of noir, hotels, or 1947 is required to trace the route.
- "Is the answer arbitrary or inevitable?" TIME — Young would ask immediately whether it names the experience. The thief's route is temporal; the solve traces a chronological path. TIME is more than arbitrary but slightly less than inevitable — a solver who gets E, I, M, T might find the anagram less natural if they're not alert to "TIME" as a word.

**Scores:**

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | Katz lens confirms narrative-in-solving; Young lens flags map order vs. route direction friction |
| Solvability | 3 | Snyder lens sharpens dual-9 concern; Young lens confirms visual grammar inconsistency at marker 9 |
| Elegance | 4 | Snyder's load-bearing test passes for all major elements; connecting door note is essential |
| Reading Reward | 4 | Katz narrative-in-solving lens confirms; Young's "does flavor text substitute for design?" test passes |
| Fun | 4 | Katz "would he want to solve this?" passes; Young flags the reverse-map-order friction as minor |
| Confirmation | 3 | Young: "is the answer inevitable?" — TIME is satisfying but not self-evidently named by the experience; the anagram step muffles inevitability |
| **Total** | **22 / 30** | **PASS (threshold exactly)** |

**Review Lenses are the most diagnostic context type for single-puzzle review.** They surface two specific failure modes (dual-9 ambiguity, reverse map order) that no other condition has fully articulated. However, the persona-less application means the concerns are identified without the weight of "Dan Katz thinks this breaks the puzzle" — they register as issues but not necessarily blocking ones. Confirmation drops because Young's inevitability test is applied without the warmth of her full design philosophy.

---

## C6 — Full Profiles

**Context injected:** C1 rubric + full Dan Katz, Thomas Snyder, and Dana Young profiles (Design Philosophy + Review Lens + credentials + identity sections).

**Reasoning:**

Full profiles combine the worldview (Design Philosophy) with the diagnostic tools (Review Lens) and add biographical context that calibrates the reviewer's standards. Knowing that Thomas Snyder has won three World Sudoku Championships and co-invented Just One Cell format means his "does the solve path hold one direction?" question carries the weight of someone who has designed puzzles specifically to force unique deduction paths. Knowing Dana Young has authored 19 puzzles with 19 distinct mechanics means her "does the visual grammar hold consistently?" question comes from someone who has never gotten away with sloppy notation.

The full profiles also add something neither section alone provides: the reviewer's relationship to the puzzle type. Dan Katz authored Hotel@MIT (a puzzle literally set in a hotel at MIT Mystery Hunt). Thomas Snyder's Just One Cell format is built on the principle "isolate one logical technique, make guessing useless." Dana Young's Placement Test mechanic for Time Frame (PH6) was a draggable interlocking tile puzzle — another spatial assembly task.

This biographical resonance makes the full-profile reviewer more confident in their positive assessments (Katz recognizes the hotel context; Snyder recognizes the single-technique isolation; Young recognizes the spatial assembly logic) and more pointed in their concerns (Snyder's uniqueness obsession is fully activated by the dual-9 issue; Young's visual precision standard is fully activated by the inconsistent marker notation).

**Katz (full profile):** The hotel setting resonates — he authored Hotel@MIT. He would find the puzzle's proportionality excellent (4 stops, clean conversion, one aha). His structural-engineer reading would approve the load-bearing architecture. He would note that the hint economics are honest: if a team gets stuck on the connecting door, a well-placed hint ("look at the notes below each floor plan") is surgically precise. He would pass this puzzle confidently.

**Snyder (full profile):** His Computer Test (10-line script) fails for this puzzle — spatial inference about human behavior is required. His uniqueness obsession drives him to test the dual-9 concern rigorously. His conclusion: the puzzle has two locations with marker 9, but only one of them (the Sargent Gallery, room 200) is a logical route stop — the 2F-Hall junction marker 9 is incidental (it's where the elevator is, not where the thief logically stopped). The puzzle's route logic forces the gallery visit, not the hall junction. However, this requires the solver to understand that "route stops" means meaningful locations in the theft narrative, not every point traversed. Snyder would flag this as requiring better disambiguation in the task description. Minor concern, not a blocker.

**Young (full profile):** Her visual precision is fully activated. She would note: (1) the dual-9 in the marker key is a visual grammar error — same symbol, two meanings in the same table; (2) the map presentation order (3F → B) is opposite the route direction (B → 3F), which she would fix by reordering the diagrams or adding a visual cue; (3) the NOTE block about the connecting door is well-placed (after the 2F diagram, before the 1F diagram) but could be visually distinguished more clearly to signal its puzzle-critical status; (4) TIME as an answer is confirmed by her "answer names the experience" test — the solve IS temporal, tracing a path through the hotel at a specific time.

**Scores:**

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | Katz hotel resonance confirms legibility; Young flags map order vs. route direction |
| Solvability | 4 | Snyder resolves dual-9 concern: gallery (200) is forced by narrative logic; hall junction is traversed not stopped at |
| Elegance | 5 | Full profiles strongly affirm: Snyder's hand-design test, Young's world-first check, Katz's contract framing |
| Reading Reward | 5 | All three confirm via different lenses: Katz (narrative-in-solving), Snyder (load-bearing NOTE), Young (visual grammar reward) |
| Fun | 4 | Katz would want to solve this (hotel context, clean aha); Young notes minor map-order friction |
| Confirmation | 4 | Young fully satisfied by TIME naming the experience; Snyder satisfied by letter-by-letter verification |
| **Total** | **26 / 30** | **PASS** |

**Highest single-expert-context score.** The full profiles produce a reviewer who resolves ambiguities (Snyder's dual-9 analysis) rather than just flagging them, and who brings biographical resonance that deepens both positive assessments and specific concerns. The remaining deduction (−4 from 30) comes from two consistent concerns across all conditions: map presentation order vs. route direction, and the dual-9 notation ambiguity.

---

## C7 — Full Profiles + Principles

**Context injected:** C1 rubric + full profiles (C6) + principle names, quotes, and tests (C3).

**Reasoning:**

This is the maximum context condition. The reviewer has: the rubric (structural scoring framework), three expert profiles in full (worldview + diagnostics + biography), and 11 named principles with tests. This condition should surface the most complete analysis.

The combination produces a qualitatively different review behavior: the reviewer can triangulate. When the Snyder Review Lens asks "does the solve path hold one direction?" AND the Interlock principle asks "any order, no advantage?" AND the Computer Test asks "10-line script?" — these three independent probes all converge on the same route-logic question. The dual-9 issue is now tested three times from different angles, producing a more confident resolution: the gallery (200) marker is the correct Stop 2 because (a) narrative logic forces it (the painting was stolen from there), (b) the interlock is linear (can't reach the linen closet without being in the gallery first), and (c) the 2F Hall marker 9 is not a "stop" in any story-meaningful sense.

The full principle suite also catches something the profiles miss individually: the **One Aha principle** and **Verify the Last Mile** together confirm that the connecting door is the singular insight AND that the extraction is letter-by-letter verifiable. The Snyder profile contributes "is there a puzzle here or a procedure?" and the Dana Young profile contributes "is the answer arbitrary or inevitable?" — both now strengthened by having specific principle names to anchor them.

The **No Over-Scaffolding** principle and the **Dana Young Review Lens** ("does flavor text explain what the layout should have communicated?") now overlap and reinforce each other: both ask whether the Route Log worksheet is doing work the visual design should have done. The answer: no. The Route Log is workspace (blank boxes to fill in), not instruction. The puzzle design is complete without it.

One new concern in C7 that doesn't appear in C6: the **Blame the Player** test ("respect or resentment?") combined with the **Young Review Lens** ("is the difficulty curve honest?") now raises a specific question about the NOTE block. The note about the connecting door is the single hardest element to discover — it's prose text below a large diagram, easy to skip. A solver who doesn't read it will stall on Step 3 (gallery escape). Is this an ambush or a reading reward? The C7 reviewer distinguishes: it's a reading reward because the NOTE is visually placed at the exact location where a reader would pause (end of 2F diagram), and because the thin-line symbol (┃) appears in the diagram itself before the note explains it. A solver who notices the ┃ will seek explanation; the NOTE is the payoff. This is designed, not accidental. Blame the Player passes.

The C7 reviewer also applies **Surprise the Answer** more rigorously: TIME is not guessable from topic (confirmed), and the anagram step is small enough to feel natural rather than arbitrary. However, the C7 reviewer notes that because the puzzle produces E-I-M-T rather than T-I-M-E in route order (5→E, 9→I, 13→M, 20→T = EIMT), the anagram is explicit and not self-ordering. The extraction produces the letters out of order, requiring a final unscrambling step. This is a minor Elegance point — the letters could have been arranged in TIME order (T=20, I=9, M=13, E=5 = Stop order 20, 9, 13, 5) if the route had been designed for that stop sequence, which it wasn't. The current route is logically sequenced (entry, access, escape, exit) and the anagram is conventional. Not a defect, but a craftsmanship note.

**Scores:**

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | Triangulation confirms document form is authentic and task is legible; map order friction remains |
| Solvability | 4 | Dual-9 resolved by three independent probes; route is forced by narrative + interlock + access logic |
| Elegance | 5 | No Over-Scaffolding + Young lens converge: worksheet is workspace not instruction; One Aha is clean |
| Reading Reward | 5 | Three independent confirmations (Katz, Snyder NOTE test, Young visual precision) |
| Fun | 4 | All fairness tests pass; Blame the Player + difficulty curve both honest; minor EIMT anagram note |
| Confirmation | 4 | Verify the Last Mile confirmed; Young's inevitability test passes; EIMT→TIME requires one step of unscrambling |
| **Total** | **26 / 30** | **PASS** |

**Ties C6 at the top.** The additional principles do not change the final score — the resolution of the dual-9 concern and the confirmation of the connecting-door aha were already achieved in C6. What C7 adds is triangulated confidence: the reviewer can name why specific concerns are or aren't blockers, cite multiple independent tests that agree, and articulate the craftsmanship note (EIMT route order vs. TIME letter order) with precision. The depth of reasoning increases; the score does not.

---

## Summary Table

| Condition | Context | Total / 30 | Pass? | Key Behavior |
|-----------|---------|------------|-------|--------------|
| C0 | "Expert reviewer" only | 20 | FAIL | Holistic impression; doesn't probe route uniqueness or over-scaffolding |
| C1 | + Rubric | 22 | PASS (marginal) | Rubric forces dimension separation; passes on atmosphere, leaves solvability soft |
| C2 | + Principle names + quotes | 25 | PASS | Vocabulary enables precise articulation; surfaces what's working more than what's failing |
| C3 | + Principle tests | 25 | PASS | Tests surface dual-9 ambiguity; redistribute scores without changing total |
| C4 | Design Philosophy only | 24 | PASS | Philosophical alignment produces warm holistic endorsement; misses specific failure modes |
| C5 | Review Lens only | 22 | PASS (marginal) | Most diagnostic for single-puzzle review; surfaces dual-9 and map-order concerns precisely; confidence low without philosophy |
| C6 | Full profiles | 26 | PASS | Biographical resonance resolves ambiguities; strongest positive and negative assessments |
| C7 | Full profiles + principles | 26 | PASS | Triangulated confidence; same score as C6 but richer reasoning and craftsmanship-level notes |

---

## Dimension-by-Dimension Heatmap

| Dimension | C0 | C1 | C2 | C3 | C4 | C5 | C6 | C7 |
|-----------|----|----|----|----|----|----|----|----|
| Clarity | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 |
| Solvability | 3 | 3 | 4 | 3 | 3 | 3 | 4 | 4 |
| Elegance | 3 | 3 | 4 | 4 | 5 | 4 | 5 | 5 |
| Reading Reward | 3 | 4 | 5 | 5 | 4 | 4 | 5 | 5 |
| Fun | 4 | 4 | 4 | 4 | 4 | 4 | 4 | 4 |
| Confirmation | 3 | 4 | 4 | 5 | 4 | 3 | 4 | 4 |
| **Total** | **20** | **22** | **25** | **25** | **24** | **22** | **26** | **26** |

---

## Findings

### 1. The Rubric Is Necessary But Not Sufficient

C0 → C1 jump: +2 points, from fail to marginal pass. The rubric alone gets the reviewer over threshold, but barely. Clarity and Fun were already being assessed intuitively; the rubric forces explicit attention to Solvability, Elegance, and Confirmation. However, without vocabulary or personas, the reviewer still can't identify *why* those dimensions score as they do — they score defensively (mid-range) rather than confidently.

### 2. Principle Names Unlock Specific Articulation

C1 → C2 jump: +3 points. This is the largest single gain in the study. The principle names give the reviewer a vocabulary to name what's working, not just what's failing. "No Over-Scaffolding" immediately answers the worksheet question. "One Aha" forces identification of the connecting door. "Reading Reward" makes the NOTE block analysis systematic. The vocabulary effect is generative, not just diagnostic.

### 3. Principle Tests Redistribute Rather Than Elevate

C2 → C3: same total, different distribution. Tests surface the dual-9 concern (Solvability −1) while elevating Confirmation (+1 via letter-by-letter verification). The tests make the reviewer more precise but not more generous. They identify a specific concern the names-only condition missed, but they also confirm what the names-only condition credited.

### 4. Design Philosophy Warms, Review Lens Diagnoses

C4 (Philosophy only) vs. C5 (Lens only): same total (24 vs. 22), different behavior. C4 is warmer and higher on Elegance (5) because the designers' worldviews directly endorse this puzzle's form-equals-mechanic structure. C5 surfaces more specific concerns (dual-9, map order friction) but scores Confirmation lower (3) because without the full philosophy, the reviewer lacks the "answer names the experience" framework that confirms TIME's inevitability. Neither half-profile is sufficient alone.

### 5. Full Profiles Resolve, Not Just Flag

C5 → C6: +4 points on total, with Solvability recovering from 3 to 4. The critical difference: in C5, the dual-9 concern is flagged but unresolved (the reviewer lacks confidence to decide if it's a blocker). In C6, Snyder's full profile — his obsession with unique solve paths, his biography as a competition puzzle designer, his specific "is there a puzzle here or a procedure?" question — gives the reviewer the framework to conclude that the gallery (200) marker is forced by narrative logic, not optional. Full profiles resolve ambiguities; partial profiles surface them.

### 6. Full Profiles + Principles Add Depth, Not Score

C6 → C7: no score change. This is the most important finding of this study. Once the full profiles are present, adding principles does not move the needle on any dimension. What it adds is triangulated confidence and craftsmanship-level precision (the EIMT route-order note). The reviewer in C7 can name three independent reasons why the dual-9 concern is not a blocker; the reviewer in C6 can name one. The quality of the reasoning improves substantially; the evaluation outcome does not.

### 7. Consistent Residuals Across All Conditions

Clarity sits at 4 in every condition. Fun sits at 4 in every condition. These dimensions are either robustly designed (Clarity — the task is genuinely legible) or robustly assessed (Fun — the noir format's atmospheric distinctiveness registers consistently regardless of framework). The map-order concern (diagrams run top-to-bottom 3F→B while the route runs B→3F) surfaces in C5, C6, and C7 but does not score Clarity below 4 in any condition. This suggests reviewers regard it as friction, not a failure.

### 8. The Puzzle Passes in 7 of 8 Conditions

P3 is a strong puzzle. Only C0 fails, and only because the reviewer has no framework for evaluating it. Every other condition produces a pass, with scores ranging from 22 (marginal) to 26 (strong). The puzzle's design integrity — Riven Standard compliance, single clear aha, reading reward in the NOTE block, answer that names the experience — is robust enough to survive evaluation at every context depth.

---

## Recommendation

For panel review of this puzzle type (spatial deduction with embedded extraction), the minimum effective context is **C2 (rubric + principle names)**. This reliably produces a pass/fail decision with specific articulation of what works and what needs attention. The full-profile conditions (C6, C7) produce richer analysis and resolve ambiguities rather than leaving them open — use them when the puzzle is borderline or when the reviewer's feedback will be given to the author.

**For production review of P3:** Score = 26/30. Pass. Two minor notes for author:
1. Disambiguate the dual marker-9 in the reference key (e.g., give the 2F-Hall junction a different marker number, or explicitly note that the gallery marker 9 is the route stop).
2. Consider reordering floor diagrams B→3F to match the route direction, or add a visual cue indicating the route runs bottom-up.
