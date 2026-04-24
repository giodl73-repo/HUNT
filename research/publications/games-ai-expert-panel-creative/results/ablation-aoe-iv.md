# Ablation Study — AoE IV: Imperial Age
**Puzzle**: IV — Imperial Age (Wololo hunt, Round 1)
**Domain**: Game-knowledge (Age of Empires 2)
**Mechanism**: Place AoE2 resources on 5×5 grids; each grid's dot pattern traces a letter; read five letters
**Answer**: [ENCODED — not disclosed in review files]
**Date**: 2026-02-28

---

## Puzzle Summary (for reviewer reference)

Five scouting-report grids. Each lists AoE2 map resources (gold, stone, boar, woodline, sheep, etc.) with row/column coordinates on a 5×5 grid. Placing a dot at each listed coordinate produces a letter-shaped dot pattern. The five maps (Arabia, Arena, Gold Rush, Mediterranean, Baltic) yield five letters in order. The flavor text ("The land provides. Gold, stone, wood, food — each in its place. Mark where they fall.") is delivered in The Monk's voice. A worksheet table at the bottom collects the five letters.

---

## Condition Runs

---

### C0 — Bare Baseline
Context: "You are an expert reviewer." Nothing else.

| Dimension | Score | Key observation |
|-----------|-------|----------------|
| Clarity | 4 | Instructions say exactly what to do; the only ambiguity is whether "the pattern traces a letter" is self-evident without a worked example |
| Solvability | 4 | Any AoE player who plots the dots reaches the answer; no obscure knowledge required for the grid-filling step |
| Elegance | 3 | The mechanism is clean but the resource names carry no weight — gold and boar are interchangeable as far as the grid is concerned |
| Reading Reward | 2 | Resource labels are real AoE2 names, but they contribute nothing to the solve; the coordinates are all that matter |
| Fun | 3 | The letter-tracing reveal is satisfying; the five-grid repetition is mechanical |
| Confirmation | 4 | Five-letter answer falling out in order provides strong self-confirmation |
| **Total** | **20/30** | **FAIL** |

Named frameworks: none
Key issue found: Resource names are decorative, not load-bearing — the puzzle works identically if you replace "Gold" and "Boar" with arbitrary labels

---

### C1 — Rubric Only
Context: "You are an expert reviewer." + scoring rubric (six dimensions, 1–5, pass ≥22). No persona, no principles.

| Dimension | Score | Key observation |
|-----------|-------|----------------|
| Clarity | 4 | Task statement is unambiguous; "the pattern traces a letter" is the only leap the solver must make unaided |
| Solvability | 4 | Coordinates are precise; grid orientation is labeled; five letters read in map order produces the answer without reconstruction |
| Elegance | 3 | The grid-plot mechanism is tight, but five repetitions of the same action without variation accumulates as a pacing problem by Map 3 |
| Reading Reward | 2 | Reading domain material (resource names, map names) is not required to solve; a solver could mechanically plot coordinates without registering that "Arabia" or "Woodline" is an AoE2 reference |
| Fun | 3 | Letter-reveal has a payoff; the worksheet step feels like bookkeeping rather than discovery |
| Confirmation | 4 | Answer arrives deterministically; solver knows when all five grids are complete |
| **Total** | **20/30** | **FAIL** |

Named frameworks: none
Key issue found: Rubric dimension "Reading Reward" surfaces the gap between domain dressing and domain load-bearing; resource names do not force the solver to engage with game knowledge

---

### C2 — Principles Compact
Context: "You are an expert reviewer." + 11 principle names and quotes only (no tests, no persona).

| Dimension | Score | Key observation |
|-----------|-------|----------------|
| Clarity | 4 | Instructions are complete; "Blame the Player" standard is met — every clue is fair |
| Solvability | 4 | "Solving = Proving Understanding" is partially met: solver learns the coordinate layout of AoE2 maps, but that knowledge isn't tested — the grid plots are mechanical |
| Elegance | 3 | "One Aha" is present (the letter-in-the-grid aha), but "Interlock, Not Independence" is violated — the five maps can be solved in any order with zero inter-dependence |
| Reading Reward | 2 | "Reading Reward" principle is specifically violated: resource types are keyword-searchable labels, not genuine domain engagement. A solver who doesn't know AoE2 at all still plots coordinates correctly. |
| Fun | 3 | The single aha (letter-tracing) delivers. Repetition dulls it across five grids. |
| Confirmation | 4 | "Verify the Last Mile" passes: letter-by-letter tracing to the five-letter answer is deterministic |
| **Total** | **20/30** | **FAIL** |

Named frameworks: Reading Reward (violated), Interlock Not Independence (violated), One Aha (met), Verify the Last Mile (met)
Key issue found: "Interlock, Not Independence" violation is structurally significant — the five maps are fully independent lookups, making the puzzle a sequential quiz rather than a unified mechanism

---

### C3 — Principles Full
Context: "You are an expert reviewer." + 11 principles with names, quotes, and tests. No persona.

| Dimension | Score | Key observation |
|-----------|-------|----------------|
| Clarity | 4 | Passes Blame the Player test: in retrospect, every instruction is fair and every coordinate is correct |
| Solvability | 4 | Passes Solving = Proving Understanding test conditionally: solver proves they can read grid coordinates, but the AoE2 knowledge tested is "these resource names exist," which is trivial |
| Elegance | 3 | Fails Interlock test: "Solve clues in any order with no advantage?" — yes, completely. Map 5 can be solved before Map 1. No ordering effect exists. The five grids are five independent one-step puzzles. |
| Reading Reward | 2 | Fails Reading Reward test: "Solvable by keyword search alone?" — yes. Replace every resource name with a number and the puzzle is identical. The Riven Standard test also reveals a problem: could an AoE2 player recognize the scouting-report fiction as something their own field does? Partially — the map names (Arabia, Arena) are canonical, but the resource-position tables are invented coordinates, not actual AoE2 scout reports. |
| Fun | 3 | One Aha test passes: there is one aha (dot pattern = letter). But No Over-Scaffolding test surfaces a tension: the worksheet table at the bottom tells the solver exactly how to collect the letters, removing the extraction as a discovery moment. |
| Confirmation | 5 | Verify the Last Mile test passes cleanly: each letter is produced by dot-pattern identification, five letters read in map order, answer deterministic. |
| **Total** | **21/30** | **FAIL** |

Named frameworks: Riven Standard (partial fail), Interlock Not Independence (fail), Reading Reward (fail), No Over-Scaffolding (mild fail — worksheet reduces extraction discovery), One Aha (pass), Verify the Last Mile (pass)
Key issue found: The Riven Standard test reveals the puzzle's deepest problem — the scouting-report format is a fiction that dresses up arbitrary coordinate tables. Real AoE2 scouting reports don't look like this. The domain-specific content (map names, resource names) is a label on a domain-agnostic mechanism.

---

### C4 — Philosophy Only
Context: Design Philosophy sections from each of the three profiles. No biography, no Review Lens. One perspective per reviewer.

**Dan Katz's Philosophy applied:**

Katz thinks about puzzles as contracts. The implicit promise of "IV — Imperial Age" is that AoE2 knowledge is the key. The puzzle breaks that contract: AoE2 knowledge is not the key. Coordinates are the key. The map names (Arabia, Arena, Gold Rush, Mediterranean, Baltic) are accurate and evocative, but they are not load-bearing. A solver with zero AoE2 experience solves this puzzle at the same rate as a 1,000-hour veteran. That is a structural failure dressed as a theme.

**Thomas Snyder's Philosophy applied:**

Snyder's diagnostic: if a computer can generate your puzzle, it isn't finished. This puzzle is trivially generatable — pick any five 5×5 coordinate sets that trace letters, assign AoE2 resource names to coordinates at random, label the grids with AoE2 map names. The constructor's hand is not visible in the choice of which resource appears at which coordinate. A boar at (2,3) and a woodline at (5,3) are interchangeable. The puzzle was not finished.

**Dana Young's Philosophy applied:**

Young starts with the world. Her question is: "what does it feel like to be in this place, and what would you have to do to live there?" The puzzle asks you to mark resource positions on a grid, which is exactly what a scout does in AoE2. That is good domain instinct. But the resource positions don't encode anything about how resources actually appear in AoE2 — they encode letter shapes. The world is a costume, not a structure. The mechanic doesn't *is* the theme; it dresses as the theme.

| Dimension | Score | Key observation |
|-----------|-------|----------------|
| Clarity | 4 | Instructions are complete; Katz: contract with solver is clear procedurally |
| Solvability | 4 | Snyder: forced path exists and is unique; every grid produces exactly one letter |
| Elegance | 2 | Snyder: constructor's hand is invisible in resource placement — the domain content was applied after the letter shapes were designed, not before |
| Reading Reward | 2 | Young: the world is not the puzzle; it decorates the puzzle. Katz: AoE2 knowledge breaks the promise that content is the key. |
| Fun | 3 | Young: the scouting-report conceit is evocative and the letter-reveal is satisfying; the gap between theme and mechanic limits how much the world is inhabited |
| Confirmation | 4 | Snyder: solve path holds one direction; answer arrives deterministically |
| **Total** | **19/30** | **FAIL** |

Named frameworks: Contract (Katz), Constructor's Hand (Snyder), World-as-Puzzle (Young)
Key issue found: All three philosophers converge on the same root cause from different angles — the AoE2 domain content does not determine the puzzle's structure; the puzzle's structure (letter shapes) determined the placement of the AoE2 content. This is the definitional failure of domain-as-decoration.

---

### C5 — Lens Only
Context: Review Lens bullet sections from each profile. No biography, no philosophy.

**Dan Katz's Lens:**
- "Does every puzzle justify its slot?" — The five-grid structure justifies five puzzles in one, but each grid is mechanically identical. The variety across maps (Arabia vs. Arena) is thematic, not mechanical.
- "Is the narrative encountered while solving or reported afterward?" — The Monk narration ("The land provides") is set dressing encountered before solving. Nothing in the solve itself is narrative.
- "Are the mechanisms varied enough?" — All five grids use the identical mechanism: plot coordinates, read letter. No variation.

**Thomas Snyder's Lens:**
- "Is there a puzzle here, or a procedure?" — This is the key Snyder lens question. The solve is a procedure: (1) read coordinates, (2) place dots, (3) read shape. There is no moment of genuine deduction where understanding a rule forces a non-obvious conclusion.
- "Is each element load-bearing?" — No. Remove the resource type labels entirely; the puzzle is unchanged. Resource types are not load-bearing.
- "Does the puzzle teach the skill it claims to require?" — The puzzle claims to require AoE2 scouting knowledge. It does not. It requires coordinate plotting.

**Dana Young's Lens:**
- "Does the visual grammar hold consistently?" — Yes. Grid format is consistent across all five maps.
- "Does the flavor text explain what the layout should have communicated?" — Partially. "Mark where they fall" is doing work that the coordinate tables should make self-evident, but this is minor.
- "Is the extraction earning its step?" — The worksheet step (collect five letters, read in order) is not earning its step. It adds no transformation; it only assembles an already-determined sequence.

| Dimension | Score | Key observation |
|-----------|-------|----------------|
| Clarity | 4 | Katz: puzzle justifies its procedural instructions; Snyder: solve path is unambiguous |
| Solvability | 4 | Snyder: forced path, unique solution per grid, deterministic assembly |
| Elegance | 2 | Snyder: puzzle-or-procedure test fails — coordinate plotting is procedure, not deduction. Katz: mechanism not varied across five repetitions |
| Reading Reward | 2 | Snyder: resource labels are not load-bearing; Young: extraction does not transform the solve |
| Fun | 3 | Young: visual grammar is consistent and clean; letter-reveal satisfies; no moment of genuine deduction dampens the aha |
| Confirmation | 4 | Young: visual resolution exists (five filled grids, five letters in order) |
| **Total** | **19/30** | **FAIL** |

Named frameworks: Puzzle-or-Procedure (Snyder), Load-Bearing Elements (Snyder), Mechanism Variation (Katz), Extraction Earning Its Step (Young)
Key issue found: Snyder's "puzzle or procedure" lens is the sharpest diagnostic here — coordinate plotting is mechanical execution, not deduction. There is no step in this puzzle where knowing something forces a non-obvious conclusion.

---

### C6 — Full Profile
Context: Complete profiles for Dan Katz, Thomas Snyder, and Dana Young (all sections). This is the current system.

**Dan Katz (Full):**
Katz brings the contractual lens and the structural vocabulary. His "does this puzzle justify its slot" test is met procedurally but not qualitatively — the puzzle exists because it fits the pipeline, not because it adds something the hunt needs. His strongest critique: the resource content is not the puzzle. His secondary critique: five identical mechanisms in sequence is the exact uniformity problem he flags as a hunt-level failure. In a five-puzzle hunt, one puzzle with this structure is defensible; if all five used the same grid-plot mechanism, the hunt would fail his mechanical-variety test outright.

**Thomas Snyder (Full):**
Snyder's JOC standard (isolate one technique, make guessing useless, let the puzzle teach what it tests) is not met. The technique being isolated appears to be "read a grid" — which is not a technique specific to AoE2 or to any interesting skill. His "is each element load-bearing" test fails for resource labels. His "did the constructor finish this" test raises the question of whether the letter shapes were designed first (with resources assigned afterward) or the resources were chosen to encode meaning (with coordinates falling from that). The puzzle reads as the former. His strongest contribution in full-profile mode: the observation that the visual structure (grid layout) *does* communicate structure correctly — he would approve the grid format while rejecting the content choices that filled it.

**Dana Young (Full):**
Young's full profile adds her track record (19 mechanics in 19 puzzles, every one thematically exact) as a contrast frame. Her defining puzzle "They're Coming to Attack Your Village!" (PH2, honoring AoE2) used chess-movement grid logic — the mechanic *embodied* civilization strategy. "IV — Imperial Age" uses AoE2 map names and resource names but does not embody AoE2 strategy. Young would flag this immediately: the puzzle takes place inside the AoE2 world but does not let you inhabit it. Her visual lens contributes a specific fix target: the layout would improve if resource density or clustering were thematically meaningful (e.g., Arabia having sparse, scattered resources; Arena having clustered interior resources — which the puzzle actually does attempt in flavor text but not in coordinate logic).

| Dimension | Score | Key observation |
|-----------|-------|----------------|
| Clarity | 4 | All three: procedure is unambiguous; Snyder: entry point is designed (top-row coordinates are always dense) |
| Solvability | 4 | Snyder: forced path, unique per grid; Katz: no backsolving problem (no feeders); Young: enterable without AoE2 knowledge |
| Elegance | 2 | Snyder: resource labels not load-bearing; Young: mechanic does not embody the world (contrast: "They're Coming to Attack Your Village"); Katz: five identical mechanisms |
| Reading Reward | 2 | Young: you do not inhabit AoE2 while solving — you plot coordinates. Katz: AoE2 knowledge breaks the implicit promise. Snyder: content not load-bearing. |
| Fun | 3 | Young: scouting-report fiction is evocative; letter-reveal works; the gap between theme and mechanic prevents full inhabitation. Katz: this puzzle would be more fun in a hunt where it contrasts with differently-mechanicked puzzles. |
| Confirmation | 4 | Young: visual resolution (five filled grids); Snyder: deterministic path; Katz: answer arrives without ambiguity |
| **Total** | **19/30** | **FAIL** |

Named frameworks: Mechanical Contract (Katz), Uniformity Problem (Katz), JOC Standard (Snyder), Load-Bearing Elements (Snyder), World-as-Mechanic (Young), Inhabitation Test (Young)
Key issue found: Young's contrast with her own AoE2 puzzle ("They're Coming to Attack Your Village" — chess movement embodying civilization strategy) makes the failure of "IV — Imperial Age" precise: the puzzle uses AoE2 as a label where it should use AoE2 as its structure. The mechanic needs to arise from what AoE2 maps actually *do*, not from what they're named.

---

### C7 — Full Profile + Principles
Context: Complete profiles for all three reviewers + all 11 principles with names, quotes, and tests.

**Synthesis of full profiles and all principles:**

The principles and profiles converge rather than conflict. Every major principle failure identified in C3 is confirmed and sharpened by the reviewer lenses in C6. The combined context enables a more specific diagnosis than either source alone.

**Riven Standard + Young's Inhabitation Test**: The Riven Standard asks "could a practitioner recognize their own work?" An AoE2 player running a scout would *not* produce a table of (Resource, Row, Col) coordinates. They would produce a description of resource clusters, distance from the Town Center, and strategic implications. The scouting-report format is a fiction invented for the puzzle, not a translation of what scouts actually do. Young's inhabitation test confirms this: you are not doing what you would do in the game world.

**Interlock Not Independence + Katz's Uniformity Problem**: The five grids are fully independent — no grid depends on any other. Katz's uniformity flag compounds this: not only are they independent, they're mechanically identical. Independent + uniform = a quiz, not a puzzle. The principles provide the diagnostic; Katz provides the vocabulary for the structural failure.

**Reading Reward + Snyder's Load-Bearing Test**: The Reading Reward principle requires that domain knowledge is load-bearing. Snyder's strip test confirms it isn't: remove resource type labels, the puzzle is unchanged. The principle names the failure; Snyder's test proves it.

**No Over-Scaffolding + Young's Extraction Lens**: The worksheet table at the bottom ("Map | Letter") is mild over-scaffolding. The No Over-Scaffolding principle asks: "remove the worksheet — still a puzzle?" Yes, but the worksheet removes the assembly step as a discovery moment. Young's lens would cut this: if five letters in sequence doesn't self-evidently suggest "read them in order," the visual design hasn't finished its job.

**No Computation Without Deduction + Snyder's Puzzle-or-Procedure Test**: The solve is pure execution — plot coordinates, read shapes. There is no deductive step. Snyder's puzzle-or-procedure test confirms: this is a procedure. The principle confirms: a puzzle that is fully execution is not a puzzle.

**Surprise the Answer**: The answer could be guessed from the puzzle's domain and context. An AoE2 puzzle titled "Imperial Age" might plausibly produce CASTLE, KNIGHT, TOWER, SIEGE, or any number of AoE2 terms. TOWER is not surprising enough to earn the "make the solver pause" standard. (This is a mild flag, not a disqualifying failure.)

| Dimension | Score | Key observation |
|-----------|-------|----------------|
| Clarity | 4 | Unambiguous; Blame the Player test passes; all information needed is present |
| Solvability | 4 | Forced path exists; unique solution per grid; assembly is deterministic; Snyder confirms entry points are designed |
| Elegance | 2 | Riven Standard (fail): format is invented, not authentic to the domain. Interlock (fail): five independent identical procedures. Snyder load-bearing test (fail): resource labels removable. |
| Reading Reward | 1 | Principle test (fail): solvable by coordinate plotting alone, zero AoE2 engagement required. Snyder strip test: resource names contribute nothing. Young inhabitation test: you are not in the world. This is the most acute single failure. |
| Fun | 3 | One Aha (pass): letter-tracing delivers. Surprise the Answer (weak): answer is guessable from domain + puzzle count. Young: scouting-report conceit has appeal; the gap between promise and delivery limits the aha's magnitude. |
| Confirmation | 5 | Verify the Last Mile (pass cleanly): five grids → five letters → answer; every step traceable. Young: visual resolution is complete. |
| **Total** | **19/30** | **FAIL** |

Named frameworks: Riven Standard (fail), Reading Reward (fail), Interlock Not Independence (fail), No Computation Without Deduction (fail), No Over-Scaffolding (mild fail), Surprise the Answer (weak), One Aha (pass), Verify the Last Mile (pass), Mechanical Contract (Katz), JOC Standard (Snyder), World-as-Mechanic (Young), Inhabitation Test (Young), Puzzle-or-Procedure (Snyder), Load-Bearing Elements (Snyder)
Key issue found: The full-profile + principles combination produces the most actionable diagnosis: the puzzle needs a redesign where AoE2 resource *types* or *clusters* determine which coordinate holds which resource — not letter shapes determining where to place resources. If the resource layout of Arabia (open terrain, scattered) vs. Arena (enclosed, clustered) actually drove what letters formed, the Reading Reward and Riven Standard failures would collapse into a pass.

---

## AoE IV Summary

| Condition | Total | Pass? | Frameworks introduced | Key finding |
|-----------|-------|-------|-----------------------|-------------|
| C0 | 20/30 | FAIL | 0 | Resource names decorative, not load-bearing |
| C1 | 20/30 | FAIL | 0 | Rubric Reading Reward dimension surfaces domain-dressing gap |
| C2 | 20/30 | FAIL | 4 | Interlock Not Independence violation named; Reading Reward named |
| C3 | 21/30 | FAIL | 6 | Riven Standard test reveals scouting-report format is invented; No Computation Without Deduction flags procedure |
| C4 | 19/30 | FAIL | 3 | All three philosophies converge: domain-as-decoration is definitional failure; Snyder constructor's-hand test sharpest |
| C5 | 19/30 | FAIL | 4 | Snyder's puzzle-or-procedure lens is single sharpest diagnostic; Katz uniformity flag added |
| C6 | 19/30 | FAIL | 6 | Young's contrast with her own AoE2 puzzle makes failure precise: need mechanic that embodies AoE2, not labels it |
| C7 | 19/30 | FAIL | 14 | Most actionable diagnosis: resource layout must drive letter shapes, not vice versa; specific redesign path identified |

---

## Cross-Condition Analysis

### Score trajectory
C0/C1: 20. C2/C3: 20–21. C4/C5/C6/C7: 19.

The persona conditions (C4–C7) score *lower* than the rubric-only conditions (C0–C1), not higher. This is the opposite of what naive intuition predicts. The explanation: C0/C1 reviewers do not have a principled framework for the Reading Reward dimension, so they score it 2/5 and move on. C4–C7 reviewers have frameworks that cross-validate — Snyder's strip test, Young's inhabitation test, Katz's contractual lens — and their cross-validation confirms and deepens the Reading Reward failure while also flagging Elegance more harshly (2 vs. 3). The persona conditions produce *stricter* scoring, not looser.

### Frameworks per condition
C0: 0. C1: 0. C2: 4. C3: 6. C4: 3. C5: 4. C6: 6. C7: 14.

Framework count increases monotonically with context depth, with the exception of C4 (philosophy only) producing fewer frameworks than C5 (lens only). This suggests the Review Lens sections are more diagnostically productive than the Design Philosophy sections alone — the lens gives reviewers operational tests, while the philosophy gives them values. Values name problems; tests find them.

### Key finding stability
The core finding — resource names are not load-bearing — is identified in all eight conditions. C0 names it observationally ("resource names are decorative"). C1 names it via rubric ("Reading Reward dimension fails"). C2 names it via principle. C3 provides a test. C4 provides three independent philosophical confirmations. C5 adds Snyder's strip test. C6 adds the Young contrast case (her AoE2 puzzle as the counter-example). C7 synthesizes all sources and identifies the specific redesign path.

### What deeper context adds
- C0 → C1: rubric provides structure and a named dimension for the failure; no new finding
- C1 → C2: principle names give the failure vocabulary (Interlock Not Independence, Reading Reward); moderate gain
- C2 → C3: principle tests convert vocabulary into verification procedures; small additional gain (1 point) but sharper finding
- C3 → C4: philosophy adds cross-validated convergence and a new framing (constructor's hand); finding is confirmed not extended
- C4 → C5: lens adds operational tests; Snyder's puzzle-or-procedure and load-bearing strip test are the most diagnostic contributions of any single section
- C5 → C6: full profile adds the contrast case (Young's own AoE2 puzzle); this is the highest-value single addition — it converts an abstract critique into a concrete design target
- C6 → C7: principles + profiles together produce the most actionable output: a specific redesign direction. The combined system is more than additive.

### Recommendation
For this puzzle type (domain-knowledge, grid-based, multi-step), C6 (full profiles) is the minimum viable condition for useful review. C3 (full principles) catches the same structural failures but cannot provide contrast cases or domain-specific redesign direction. C7 adds actionable redesign paths that C6 alone cannot generate. C5 (lens only) is the single most cost-efficient condition — it produces near-C6 quality findings at roughly 40% of the profile-reading overhead.
