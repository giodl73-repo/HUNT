# Ablation Study: Context Depth vs. Evaluation Quality
## Puzzle: P01 — Bestiary v3.2 — Complete Stats (IRONFALL)
## Answer: UMBRA | Domain: Fictional retro-RPG (Ironfall)
## Date: 2026-02-28

---

## Puzzle Summary (for reference)

P01 presents six bestiary entries from an invented 16-bit RPG, each with a blank stat value. Community notes (attributed to fictional forum users from 1997–2002) supply each missing value. The extraction step orders the six enemies by the year their community note was posted, then indexes into each enemy name at a specified position. The five resulting letters spell UMBRA. The sixth entry (Sanctum Guardian / Element: Holy) is a fill-in that does not contribute a letter — its note provides context but no extracted character.

The puzzle's fictional framing (a degraded archive from a 1998 fan wiki) is entirely invented. No external game exists. All data is self-contained.

---

## Scoring Rubric

| Dimension | What it tests | Scale |
|-----------|--------------|-------|
| Clarity | Are instructions unambiguous? Does the solver know what to do? | 1–5 |
| Solvability | Are all blanks fillable from the notes? Is the extraction deterministic? | 1–5 |
| Elegance | Is the mechanism clean? Is the work proportionate to the insight? | 1–5 |
| Reading Reward | Does Ironfall world knowledge matter — or can anyone solve it? | 1–5 |
| Fun | Would a solver enjoy this? Does the flavor earn its presence? | 1–5 |
| Confirmation | Can the solver verify the answer step by step? | 1–5 |

Pass threshold: ≥ 22 / 30

---

## C0 — Bare Authority ("You are an expert reviewer.")

**Prompt context given:** "You are an expert reviewer."

### Evaluation

A reviewer operating with only an authority claim and no other scaffolding defaults to intuitive pattern-matching. Without a rubric, the reviewer imports whatever review habits feel most salient. Without principles, the reviewer cannot name what they are testing. Without profiles, they apply a generic "good puzzle" heuristic.

**Clarity — 3/5**
The table structure is readable. The blanks are labeled (A through F). However, the extraction instruction ("DarkKnight_97 marked them in chronological order of discovery by the community") is embedded in prose and not visually separated from flavor text. A solver who skims may not distinguish the archive flavor from the operative instruction. The disambiguation between "complete the grid" and "follow the extraction" is present but not emphasized.

**Solvability — 4/5**
Each blank maps to exactly one community note. The arithmetic for E (¾ × 32 = 24) is transparent. The element for F (Holy, per IronFan_Kenji) is direct lookup. The extraction table at the bottom resolves the letter positions explicitly — perhaps over-explicitly (the verification line restates each step). No ambiguity in the final extraction. Minor deduction: one note calls Shade Wisp "highest in Ashveil" (implying contextual knowledge that is not otherwise provided) — a solver without Ironfall context cannot verify "highest in Ashveil" means ATK 22 is correct, only accept it on authority.

**Elegance — 3/5**
The mechanism is: fill blanks → reorder by year → index into names → read letters. This is a standard hunt extraction chain. It works. But the year-based reordering is not motivated by anything in the puzzle structure beyond the premise ("the archive indexed entries in chronological order of discovery"). The connection between the fill step and the extraction step feels administrative rather than integrated — the blanks you fill do not determine what you extract. You could, in principle, leave blanks A–F blank and still complete the extraction (since the extraction table uses enemy names, not stat values). This is a structural decoupling that a plain "expert" reviewer would notice as a faint wrongness without being able to name it precisely.

**Reading Reward — 2/5**
The flavor — 1990s fan wiki voice, forum usernames with years, "last updated March 14, 1998" — is genuinely evocative. A solver who enjoys retro-gaming nostalgia will get something from the framing. However: Ironfall is not a real game. The reward for knowing it is zero, because no one knows it. The flavor text rewards familiarity with the *genre* (1990s JRPG wikis) rather than the specific domain. This is a design-level finding, not a quality failure.

**Fun — 3/5**
The archive conceit is charming. The invented usernames (BattleMath_99, SpeedQueen, DataMiner_X) give personality to what is otherwise a stats-fill exercise. The puzzle is competently enjoyable without being remarkable.

**Confirmation — 4/5**
The solution space is fully printed in the puzzle file itself, which is unusual — this is an authored puzzle document, not a solver-facing version. Assuming solver-facing delivery removes the solution block, confirmation requires: verify each note matches one blank, verify E arithmetic, verify F via IronFan_Kenji, verify year ordering, verify name indexing. All steps are checkable. The ROT13 at the end (HZOEN) is confirmable by anyone with a decoder.

**Total: 19/30 — FAIL**

**Bare-authority reviewer finding:** The puzzle is functional but does not reach the 22/30 threshold. The elegance gap — stat-fill and extraction are structurally decoupled — is the primary deficit. A reviewer with only authority credentialing names the gap as "something feels disconnected" without being able to say why.

---

## C1 — Authority + Rubric

**Prompt context given:** "You are an expert reviewer." + full 6-dimension rubric with descriptions.

### Evaluation

The rubric gives the reviewer a vocabulary and a scoring structure. Dimensions are named. The 22/30 threshold is explicit. This forces the reviewer to address each dimension rather than holistically gesturing.

**Clarity — 3/5**
Same finding as C0, but now articulable: the instruction "follow the extraction" is a prose directive inside a flavor-dense header. The separation between flavor and operative instruction is insufficient. A reviewer with the rubric can now say: "Clarity is about whether the solver knows what to do — the extraction instruction fails this because it is buried." This is an improvement over C0's vague "might be confusing."

**Solvability — 4/5**
All six blanks have unique solutions from the notes. The notes are dated and attributed, which prevents ambiguity about which note resolves which blank. However, the note for Shade Wisp ATK ("Highest in Ashveil. Glass cannon.") introduces regional context ("Ashveil") that appears nowhere else in the puzzle. A solver cannot verify "highest ATK in Ashveil" without access to the full bestiary — which the puzzle does not provide. The claim must be taken on faith. This is a minor solvability crack.

**Elegance — 2/5**
With the rubric in hand, the reviewer can now apply "Reading Reward: does knowing the Ironfall fictional world matter, or can anyone solve it?" to elegance as well as to the named dimension. The result: the mechanic is two separate puzzles glued together. Phase 1 (fill the blanks) uses the stat values. Phase 2 (extract) uses the enemy names and note years. The filled values never feed into the extraction. This means a solver could bypass Phase 1 entirely and still get UMBRA. A rubric-equipped reviewer scores this 2 because the decoupling violates the implicit elegance promise: "the work is proportionate to the insight."

**Reading Reward — 2/5**
With the rubric's specific framing ("does knowing the Ironfall fictional world matter, or can anyone solve it?"), this dimension now has a sharp answer: no Ironfall knowledge is required or rewarded. The fictional framing is backdrop, not puzzle content. Someone who has never heard of Ironfall solves this identically to someone who has "played" it. Score 2 rather than 1 because the flavor is well-executed even if it is purely cosmetic.

**Fun — 3/5**
Rubric does not change this finding. The puzzle is agreeable. The archive voice is pleasant. Nothing about it is surprising or particularly memorable.

**Confirmation — 4/5**
Same as C0 but now explicitly tested: can the solver trace the answer letter by letter? Yes. Each name-position pair is unambiguous. TUNDRAGOLEM[2]=U, etc., are all correct (1-indexed). The verification is airtight.

**Total: 18/30 — FAIL**

**Rubric-equipped finding:** Scores drop slightly from C0 because the rubric forces the elegance problem to be scored rather than gestured at. The structural decoupling between the fill step and the extraction step is now explicitly rated. The rubric reveals the puzzle's most significant weakness: filling in blanks A–F is unnecessary work if the extraction only uses enemy names. This finding was present in C0 but underweighted.

---

## C2 — Principle Names + Quotes (no tests)

**Prompt context given:** Authority + rubric + 11 principle names with their defining quotes (no diagnostic tests).

### Principles applied:

- **The Riven Standard** — "The puzzle IS what the field does."
- **Solving = Proving Understanding** — "Test: More knowledge after?"
- **Blame the Player** — "Test: Respect or resentment?"
- **No Over-Scaffolding** — "Test: Remove — still a puzzle?"
- **Surprise the Answer** — "Test: Guessable from topic?"
- **One Aha** — "Test: Name the single aha."
- **Reading Reward** — "Test: Keyword search sufficient?"
- **No Computation Without Deduction** — "Test: Remove — still a puzzle?"
- **Snyder's Computer Test** — "Test: 10-line script."
- **Interlock, Not Independence** — "Test: Any order, no advantage?"
- **Verify the Last Mile** — "Test: Trace letter by letter."

### Evaluation

**Clarity — 3/5**
Unchanged. The operative extraction instruction is parseable but under-emphasized.

**Solvability — 4/5**
Unchanged. The "Ashveil" context gap remains a minor flaw but does not block solving.

**Elegance — 2/5**
The Riven Standard now applies with force: does filling in bestiary stats constitute "what a bestiary fan does"? The answer is: partially. Fans do verify stats, and the community-note format is authentic to the archival hobby. But the extraction step — indexing into enemy names by year of note — is not "what a bestiary fan does." The Riven Standard exposes the seam: the puzzle is half-Riven (the fill) and half-generic-hunt-extraction (the letter index). A puzzle that earns a full Riven score would have the extraction emerge from the stats themselves — for example, converting stat values to letters, or using the stat totals to determine which enemy is extracted.

The **Interlock, Not Independence** principle is the second blow: the blanks and the extraction do not interlock. The fill-in step can be completed in any order. The extraction step can be completed without completing the fill-in step. These are two independent puzzles. A well-designed puzzle has steps that are mutually constraining — each deduction narrows future deductions.

**Reading Reward — 2/5**
No Ironfall knowledge provides any solving advantage. The fictional world is flavor only. A solver who knows nothing about JRPGs solves this as easily as one who does. Reading Reward is technically 2 — the flavor text is well-crafted and rewards attention, but it is not puzzle-material.

**Fun — 3/5**
Unchanged.

**Confirmation — 5/5**
Verify the Last Mile now named explicitly: TUNDRAGOLEM[2]=U (T=1, U=2 — correct), EMBERFANG[2]=M (E=1, M=2 — correct), THORNBACK[6]=B (T=1, H=2, O=3, R=4, N=5, B=6 — correct), FROSTCLAW[2]=R (F=1, R=2 — correct), SHADEWISP[3]=A (S=1, H=2, A=3 — correct). All five verified. Score 5.

**Total: 19/30 — FAIL**

**Principle-names finding:** Two principles (Riven Standard and Interlock, Not Independence) directly target the puzzle's primary structural weakness: the stat-fill and name-extraction do not interlock, and the mechanic is not fully grounded in "what a bestiary fan does." The principle names give the reviewer language to articulate what was only a vague sense of wrongness in C0. Confirmation rises to 5 because "Verify the Last Mile" prompts an explicit letter-by-letter trace.

---

## C3 — Principle Names + Quotes + Diagnostic Tests

**Prompt context given:** Everything in C2 + all 11 diagnostic tests spelled out.

### Diagnostic Tests Applied:

- **The Riven Standard** — "Practitioner recognizes their own work?" → A bestiary archivist would recognize filling in stat gaps from community notes. They would NOT recognize "index into enemy names by year of note post" — this is puzzle-hunt extraction imposed on a real-world activity.
- **Solving = Proving Understanding** — "More knowledge after?" → After solving, the solver knows: the stat values for six enemies, and a five-letter word (UMBRA). They do not learn anything about how the Ironfall bestiary system works, what regions mean, or how element types affect combat. Knowledge gain is minimal.
- **Blame the Player** — "Respect or resentment?" → The "Ashveil" claim in the Shade Wisp note cannot be independently verified. A solver who asks "where does 22 come from?" has no answer inside the puzzle. This is a minor resentment vector — the note is credible but not self-contained.
- **No Over-Scaffolding** — "Remove — still a puzzle?" → Remove the solution space block (which is present in the authored file): yes, still a puzzle. Remove the extraction table: the puzzle collapses — the extraction instruction alone ("chronological order of discovery") does not specify which position to index in each name. The extraction table is therefore NOT over-scaffolding; it is the extraction mechanic. But: the extraction table makes the puzzle trivially solvable without completing the fill step, because the table provides enemy names directly.
- **Surprise the Answer** — "Guessable from topic?" → UMBRA could plausibly be guessed as a dark-themed JRPG enemy name before solving. It is a common fantasy word. The answer is not surprising enough — it fits the genre too naturally. A solver might get UMBRA and not feel confirmed because the word seems "right" for the wrong reason.
- **One Aha** — "Name the single aha." → The aha is: enemies should be sorted by the year their community note was written, not by their number in the bestiary. This is a clean single insight. Identifiable, discrete, satisfying when found.
- **Reading Reward** — "Keyword search sufficient?" → Yes. A solver who searches for "TUNDRA" in the extraction table finds the letter and position immediately. The puzzle is fully keyword-searchable. This is both a strength (clear confirmation) and a weakness (no reading required to extract).
- **No Computation Without Deduction** — "Remove — still a puzzle?" → The stat-fill step involves deduction (reading notes, matching to blanks) and one arithmetic step (¾ × 32). The arithmetic is trivial. Remove the arithmetic: Tundra Golem ATK would be stated directly, and the puzzle loses nothing. The arithmetic is decoration, not deduction.
- **Snyder's Computer Test** — "10-line script." → A 10-line script could: parse the extraction table, read enemy names, index into each name at the given position, concatenate. The stat-fill step could also be automated (the notes directly state most values). This puzzle is almost entirely automatable, which indicates insufficient deductive depth.
- **Interlock, Not Independence** — "Any order, no advantage?" → Blanks can be filled in any order. Extraction can be done before filling blanks. No solving advantage comes from any particular sequence. The two steps are independent modules. This is a structural failure.
- **Verify the Last Mile** — "Trace letter by letter." → Already traced in C2. All five positions correct. UMBRA confirmed.

### Scores

**Clarity — 3/5**
Extraction instruction identified as parseable but under-separated from flavor.

**Solvability — 3/5**
(Downgrade from C1.) The Blame the Player test reveals the "Ashveil" verification gap more sharply. The Snyder Computer Test identifies that the stat-fill can be automated, which means it is not deductive in the puzzle-design sense. Two solvability-adjacent failures: one credibility gap, one mechanical shallowness.

**Elegance — 1/5**
(Downgrade from C2.) The full battery of tests makes the structural failures compounding:
1. Stat-fill and extraction do not interlock (Interlock test).
2. Mechanic is not fully grounded in "what a bestiary fan does" (Riven Standard test).
3. Both steps are automatable (Computer test).
4. Arithmetic is decorative, not deductive (No Computation Without Deduction test).
Elegance is nearly absent at the structural level.

**Reading Reward — 2/5**
Keyword search test confirms: no reading required, any solver can extract mechanically. Score holds at 2 (flavor well-executed, but not puzzle-material).

**Fun — 3/5**
The archive voice remains charming. The One Aha (sort by year, not by ID) is clean and satisfying when found. Fun holds.

**Confirmation — 5/5**
Verify the Last Mile passes completely. Score holds at 5.

**Total: 17/30 — FAIL**

**Tests-equipped finding:** The diagnostic tests are the most powerful discriminator in this study. They expose three separate structural failures invisible to C0 and C1:
1. The fill step and extraction step are independent modules — solving one does not help the other.
2. Both steps are essentially automatable — insufficient deductive depth.
3. The answer (UMBRA) is guessable from genre knowledge before solving.

The tests also identify the single genuine strength: One Aha (the year-reordering insight) is clean and well-executed.

---

## C4 — Reviewer Profiles: Design Philosophy Only

**Reviewer context:** Dan Katz Design Philosophy + Thomas Snyder Design Philosophy + Dana Young Design Philosophy (no Review Lenses).

### Dan Katz design philosophy applied:
A puzzle hunt is a contract. The stat-fill step implicitly promises that the work done in Phase 1 feeds Phase 2. That contract is broken — the extraction uses enemy names (not stats). Hunts that make implicit promises and break them create resentment. The hunt-level contract concern also applies at puzzle level: the solver signs up to "complete the bestiary" and ends up doing letter-indexing, which is a different activity.

**Structural load**: Does the puzzle justify its slot? A puzzle that is two independent activities compressed into one page is doing less structural work than its footprint suggests. A single well-integrated puzzle would be stronger.

### Thomas Snyder design philosophy applied:
"If a computer can generate your puzzle, your puzzle isn't finished." The stat-fill is automated by the notes (each note states a value directly). The extraction is automated by the table (each enemy name, position, and year are listed). A 10-line script produces UMBRA. The constructor's hand is not visible in the deduction sequence — there is no moment where understanding the Ironfall world forces a conclusion. The puzzle is a procedure, not a deduction.

**Theme vs. structure**: The bestiary theme is genuine (the form matches a real archival activity). But the extraction layer imposes hunt-convention mechanics on top of the theme without integrating them. Theme and structure diverge at the extraction step.

### Dana Young design philosophy applied:
Dana's central question: "What does it feel like to be in this place?" Being in the Ironfall archive feels like reading old forum posts and cross-checking disputed data — this is authentic. But the extraction step breaks immersion: no real archivist would index the second letter of an enemy name in the year 1997. The world stops being the puzzle the moment the extraction begins. The answer (UMBRA) does not name the experience — "you were a shadow" is not what you were doing. A fully Dana-coherent version would have the answer name the archival activity: VERIFY, RECOVER, ARCHIVE, something that completes "what you were doing the whole time."

### Scores

**Clarity — 3/5**
Katz: parseable but the contract is ambiguous about what Phase 1 contributes to Phase 2.

**Solvability — 4/5**
All three designers would acknowledge the puzzle is technically solvable. No blocking ambiguity. Minor: "Ashveil" context gap remains.

**Elegance — 2/5**
Katz: contract broken (fill doesn't feed extraction). Snyder: theme and structure diverge at extraction. Dana: world stops being the puzzle. Three independent critiques converge on the same structural seam.

**Reading Reward — 2/5**
Dana in particular: the world should be the puzzle, not the frame. When it is only the frame, Reading Reward is cosmetic.

**Fun — 3/5**
Katz: would he want to solve this? Probably yes — the archive voice is enjoyable, the year-sort insight is clean. Snyder: would he want to have constructed this? Less clear — the extraction is conventional, not inventive. Dana: the experience is enjoyable until the extraction breaks immersion. Averaged: 3.

**Confirmation — 4/5**
No profile-specific concerns beyond the already-noted "Ashveil" gap.

**Total: 18/30 — FAIL**

**Design-philosophy-only finding:** The three design philosophies converge on the same core problem from three different directions (contract, craftsmanship, world-as-puzzle). This convergence is stronger than C0-C3's findings, because each philosopher articulates the failure through their own lens. The mutual reinforcement increases confidence. Dana's "does the answer name the experience?" is a new critique not raised in any previous condition — it targets the answer word itself as potentially mismatched to the puzzle experience.

---

## C5 — Reviewer Profiles: Review Lens Only

**Reviewer context:** Dan Katz Review Lens + Thomas Snyder Review Lens + Dana Young Review Lens (no Design Philosophy).

### Dan Katz Review Lens applied:
- **Does every puzzle justify its slot?** The stat-fill contributes nothing to the extraction. This puzzle could be tightened to extraction-only, or stat values could be woven into the extraction. As authored, the fill step takes solver time without structural payoff.
- **Are the mechanisms varied enough?** Within this puzzle: two mechanisms (fill-in and letter-indexing) that do not interact. At the hunt level, this is the simplest extraction type in the toolkit — Katz would flag it as under-varied.
- **Is the narrative encountered while solving or reported afterward?** The archive narrative (forum posts, timestamps, disputed data) is genuinely encountered while solving — it is the medium of the fill step. This is a Katz point in the puzzle's favor.
- **Would he want to solve this?** Probably. The archive conceit is genuine. The year-sort insight is clean. The puzzle is small, which Katz respects.

### Thomas Snyder Review Lens applied:
- **Is the entry point constructed or discovered?** The entry point — "fill in the blanks from the notes" — is obvious and clear. This is a strength.
- **Does the solve path hold one direction?** The fill step can be done in any order. The extraction step is sequential (year order). The two steps are not connected. The path has a structural gap at the seam.
- **Is each element load-bearing?** The stat values (A–F) are not load-bearing for the extraction — they could be removed and the extraction would still work. This is Snyder's sharpest test: anything that can be removed without consequence was not design.
- **Is there a puzzle here, or a procedure?** The fill step: procedure (note → value). The extraction step: procedure (table → letter). Neither step involves a moment of genuine deduction. The year-sort is the only deductive step, and it is stated in the extraction table, removing the need for the solver to discover it.
- **Would he want to have constructed this?** No. The architecture does not represent genuine choices — the extraction table could have been generated by formula.

### Dana Young Review Lens applied:
- **Does the visual grammar hold consistently?** The table is clear. The extraction table uses a different visual register (Markdown table vs. ASCII box table). Minor consistency gap.
- **Is the extraction earning its step?** The extraction adds a step (year-sort then index) that transforms the fill work into a word. But since the fill work is not actually used in the extraction, the transformation does not "earn" it — it replaces it.
- **Is the difficulty curve honest?** The puzzle is easy throughout. No ambush. But the difficulty is uniformly low — no escalation from entry through extraction. A solver does not feel earned difficulty anywhere.
- **Is the answer arbitrary or inevitable?** UMBRA is not inevitable. It is a valid dark-fantasy word, but the puzzle experience does not build toward it. When you read UMBRA, you confirm it via the letter trace, but you do not feel it was the only possible right answer.
- **Does the flavor text explain what the layout should have communicated?** The archive flavor (header quote, attribution to DarkKnight_97) does the work of establishing context — this is good flavor design. Dana would not cut it.

### Scores

**Clarity — 3/5**
Katz: narrative encountered during solve is a genuine strength. Snyder: entry point is clear. Dana: visual grammar minor inconsistency. Hold at 3.

**Solvability — 4/5**
Lenses do not change solvability finding. All blanks resolvable. Snyder notes the solve path has a gap at the seam.

**Elegance — 1/5**
Snyder: stat values not load-bearing for extraction = not design. This is the harshest elegance score yet. Dana: extraction does not earn its step (replaces, not transforms). Katz: fill step has no structural payoff. Three lenses, three independent failures. Elegance collapses to 1.

**Reading Reward — 2/5**
All three reviewers would note: no domain-specific knowledge required or rewarded. The world is backdrop.

**Fun — 3/5**
Katz: would want to solve it, small, clean. Dana: difficulty curve flat, answer not inevitable — minor fun deflation. Hold at 3.

**Confirmation — 4/5**
Snyder: "can the solver confirm each step independently?" Yes — but the extraction table makes confirmation trivial, which also makes the extraction trivial. The confirmability is complete but unearned.

**Total: 17/30 — FAIL**

**Review-lens-only finding:** The lenses are operationally precise. Snyder's "is each element load-bearing?" is the single most devastating test in this condition — the stat values are not load-bearing for extraction, which means a core mechanism of the puzzle is structurally unnecessary. This finding is more specific and more actionable than any C0–C3 finding. Dana's "is the answer inevitable?" raises a new quality concern: UMBRA lands as confirmed but not earned.

---

## C6 — Full Profiles: Dan Katz + Thomas Snyder + Dana Young

**Reviewer context:** Complete profiles for all three (Identity + Puzzle Hunt Credentials + Design Philosophy + Review Lens). No principles.

### Dan Katz — Full Profile

Dan Katz has won MIT Mystery Hunt eight times. He reads hunts as structural engineers read blueprints. He invented the vocabulary for hunting failure modes — backsolving, mettleneck, mystery crate, buyer's remorse. His most relevant lens here: "Does every puzzle justify its slot?"

**Full evaluation:**
Katz approaches P01 knowing that a feeder puzzle's job is to fit the hunt's structure, be solvable by teams at the appropriate difficulty level, and contribute something architecturally distinct to the round. For a 10-puzzle hunt with a super-meta, each puzzle must earn its position.

P01 is a Round 1 feeder. As such, its job is to be accessible, set the thematic register, and deliver an answer that contributes to the round meta. Katz checks: (a) does this puzzle serve the median team? Yes — it is straightforward. (b) can it be short-circuited? Technically yes — someone who guesses UMBRA skips the extraction entirely, since UMBRA is a plausible dark-fantasy word. This is a backsolving concern: if the meta is REALM and the solver guesses the first letter of UMBRA without solving P01, the meta is vulnerable to partial backsolvability. (c) is the narrative encountered during solving? Yes — the archive format is not decoration but the medium of the puzzle. This is a genuine structural strength. (d) would he want to solve this? Probably yes — it is small, self-contained, and has a clean single aha.

**Katz verdict:** The puzzle passes the hunt-level test (accessible feeder, clean answer delivery) but fails the individual puzzle test (fill step and extraction are two separate activities with no mechanical interlock). He would flag this for revision: the stat values should contribute to the extraction, not just fill the table.

### Thomas Snyder — Full Profile

Snyder is the most decorated competitive puzzler in American history. His diagnostic standard: "If a computer can generate your puzzle, your puzzle isn't finished." He would evaluate P01 in under 30 seconds and identify the construction failure immediately.

**Full evaluation:**
The fill step: each community note states a value. The extraction table: states the enemy name, year, position, and letter directly. A solver reads the table and writes down UMBRA. There is no deductive moment — the extraction table removes the need to discover anything. Snyder's harshest test: "Is there a puzzle here, or a procedure?" Answer: it is two procedures stapled together.

His co-invention of Just One Cell Sudoku is relevant here. JOC isolates one logical technique and makes every other approach produce nothing. P01 is the inverse: it has two approaches (fill and extract) that can both be completed independently, making the integrated solution irrelevant. A solver who completes Phase 1 gains no advantage in Phase 2.

His "would he want to have constructed this?" test: no. The extraction table is formulaic. The community notes are charming but do not represent a genuine construction choice — they are a delivery mechanism for values, not a designed deduction sequence.

**Snyder verdict:** The puzzle is not finished. The extraction table makes the puzzle automatable. The stat-fill and extraction are independent, violating the principle that every element must be load-bearing. He would send it back for a redesign of the extraction mechanism: the stat values should be input to the extraction, not parallel to it.

### Dana Young — Full Profile

Dana has 25 years of building puzzles where the mechanic is the theme, not a container for it. Her masterwork — the Placement Test round — has 19 puzzles with 19 distinct mechanics, each thematically exact. She reads visually, she starts with a world, and she sends back anything that hasn't been finished.

**Full evaluation:**
Dana starts with the world: what does it feel like to be in this archive? It feels like cross-checking disputed fan data, reconciling conflicting claims, establishing which numbers to trust. The fill step captures this feeling exactly. The archive framing ("some entries were disputed. You will need to decide which numbers to trust.") is a perfect setup for the fill mechanic.

Then the extraction breaks the world. No fan-archivist has ever been asked to "take the second letter of each enemy name in chronological order of community note year." This is a hunt convention imposed on the archival world. The world stops being the puzzle at the extraction step.

Her test: "Is the answer arbitrary or inevitable?" UMBRA names something dark and shadowy. But the puzzle's experience is archival — cross-referencing, verifying, trusting sources. UMBRA does not name that experience. A Dana-coherent version would have the answer emerge from the archival activity itself: perhaps identifying which number to trust among conflicting notes, and the reliable numbers spell something.

Her test: "Does cutting this puzzle make the hunt better?" At this quality level, maybe — it is the weakest puzzle in the hunt by her standards.

**Dana verdict:** The puzzle has a genuine world-entry (the fill step is archivally authentic) and then abandons it (the extraction is hunt-generic). She would redesign the extraction to stay inside the archive world, and reconsider the answer word to name the archival experience.

### Combined Full-Profile Scores

**Clarity — 3/5**
All three reviewers: parseable but the boundary between flavor and instruction is soft.

**Solvability — 4/5**
Katz: accessible for median team. Snyder: automatable, but all blanks resolvable. Dana: no blocking ambiguity.

**Elegance — 1/5**
All three converge: the fill-extract seam is the definitive flaw. Snyder makes it most precise: stat values not load-bearing for extraction = not finished.

**Reading Reward — 2/5**
Dana most sharply: the world entry is authentic (fill step) but cosmetic (extraction step). Katz: narrative encountered during solving (positive) but world knowledge not required. Net 2.

**Fun — 3/5**
Katz: would solve it, values small clean puzzles. Snyder: would not construct it. Dana: archival entry is genuinely enjoyable, extraction breaks it. Averaged 3.

**Confirmation — 4/5**
All three: the letter trace is complete and verifiable. Katz notes the backsolving risk (UMBRA guessable). Net 4.

**Total: 17/30 — FAIL**

**Full-profiles finding:** The three full profiles provide the richest qualitative analysis of any condition. Each reviewer brings a distinct lens that generates a distinct actionable critique: Katz identifies the backsolving risk and the structural slot question; Snyder identifies the automation failure and the "two procedures" problem; Dana identifies the world-exit at extraction and the answer-experience mismatch. These three critiques are non-overlapping — each adds unique diagnostic value. The scoring, however, is similar to C5 (17/30). The profiles add qualitative depth, not scoring precision.

---

## C7 — Full Profiles + Design Principles

**Reviewer context:** All three full profiles + all 11 principles with names, quotes, and diagnostic tests.

This is the maximum-context condition. The reviewer has: three fully-characterized expert identities, their documented review practices, and 11 named principles with explicit tests.

### Synthesis approach:

In C7, the principles and profiles reinforce each other at every failure point, but they also occasionally generate tension — a productive tension that no lower condition can produce.

**Example reinforcing convergence:**
- Snyder (profile): "If a computer can generate your puzzle, your puzzle isn't finished."
- Snyder Computer Test (principle): "Can a 10-line script solve this?"
- Result: Convergent confirmation that the puzzle is automatable. This is the strongest possible failure signal — profile and principle agree.

**Example productive tension:**
- Katz (profile): "Would he want to solve this? Probably yes — small, self-contained, clean aha."
- No Over-Scaffolding principle: "Remove — still a puzzle?" → The extraction table, while not over-scaffolding in the traditional sense, pre-resolves the year-sort discovery, which is the puzzle's only genuine aha.
- Tension: the extraction table is simultaneously the clearest possible confirmation mechanism AND the thing that removes the need to discover the aha. The puzzle scores high on Confirmation and low on Surprise for the same reason.

**Principle application with profile context:**

| Principle | Katz lens | Snyder lens | Dana lens | Finding |
|-----------|-----------|-------------|-----------|---------|
| Riven Standard | Archive narrative is encountered during solve (positive) | Theme diverges at extraction (negative) | World exits at extraction (negative) | 2:1 negative |
| Solving = Proving Understanding | Solver learns: stat values for 6 enemies + UMBRA | Nothing about game mechanics or element interactions | Nothing about the archive's organization system | Low knowledge gain |
| Blame the Player | "Ashveil" context unverifiable within puzzle | Notes must be trusted without internal verification | Solver should be able to verify all inputs | Minor fairness gap |
| No Over-Scaffolding | Extraction table is necessary (not scaffolding) | Extraction table pre-solves the aha (over-scaffolding) | Table is clear and honest | Snyder tension identified |
| Surprise the Answer | UMBRA guessable pre-solve (backsolving risk) | UMBRA is a common dark-fantasy word | Answer doesn't name the experience | All negative |
| One Aha | Year-sort is the aha; clean and discrete | Aha pre-resolved by extraction table | Aha is hunt-conventional, not archival | Mixed |
| Reading Reward | Keyword search sufficient; no world knowledge required | Every letter position is stated in table | Archive knowledge cosmetic | All negative |
| No Computation Without Deduction | ¾ × 32 arithmetic is decorative | Arithmetic adds no deductive insight | Arithmetic is accurate but pointless | All negative |
| Snyder Computer Test | Automatable | Automatable | Automatable | Strong failure |
| Interlock, Not Independence | Fill and extract are independent modules | No element is load-bearing for another | Extraction does not depend on fill | All negative |
| Verify the Last Mile | Letter-by-letter trace passes completely | All positions verified | Confirmation is airtight | All positive |

### Scores

**Clarity — 3/5**
The extraction instruction is present but under-emphasized relative to the flavor density. All three reviewers agree. Principle (No Over-Scaffolding): the instruction is not over-scaffolded but it is under-separated.

**Solvability — 3/5**
(Downgrade from C6.) Blame the Player (principle) + Katz (note that some claims must be taken on faith) + Snyder (the extraction table makes the puzzle technically trivial but removes the deductive challenge): three concurrent signals that solvability is qualified. The puzzle is technically solvable but the "Ashveil" gap is a fairness crack and the trivial extraction is a deductive gap.

**Elegance — 1/5**
Convergence of: Interlock, Not Independence (principle), Snyder Computer Test (principle), Riven Standard (principle, 2:1 negative), Snyder's "is there a puzzle here or a procedure?" (profile), Katz's "fill step has no structural payoff" (profile), Dana's "extraction exits the world" (profile). Six independent failure signals at the same structural seam. This is the most extensively diagnosed elegance failure in the study.

**Reading Reward — 1/5**
(Downgrade from 2.) The convergence of Reading Reward principle ("keyword search sufficient?"), Dana's world-exit critique, Katz's "world knowledge not required," and the Computer Test (puzzle is automatable) — four signals that Reading Reward is absent. Score drops to 1: the flavor is well-executed but it is doing zero puzzle work.

**Fun — 3/5**
Katz holds at 3 (would solve it). Snyder would not construct it, but that is a constructor perspective. Dana's archival entry is enjoyable. The One Aha is clean. Fun remains 3 because the experience up to the extraction is genuinely pleasant.

**Confirmation — 5/5**
Verify the Last Mile (principle) + all three profiles: the letter trace is airtight. The only concern is that the extraction table makes confirmation trivial (Snyder), but trivial confirmation is still confirmation. Score 5.

**Total: 16/30 — FAIL**

**Full-profiles + principles finding:**

C7 produces the richest diagnostic output of any condition. It generates findings that no other condition produces:
1. A table mapping all 11 principles across all 3 reviewer lenses — showing where they converge (Interlock test, Computer test = all three agree) and where they diverge (Over-Scaffolding = Snyder sees extraction table as problem, others do not).
2. The productive Confirmation/Surprise tension: the extraction table is simultaneously the puzzle's strongest and weakest element — strongest for confirmation, weakest for discovery.
3. Reading Reward drops to 1 because the combined weight of four independent signals is stronger than any single signal.
4. The answer word (UMBRA) receives attention from multiple angles: guessable from genre (Surprise the Answer, Katz), not naming the experience (Dana), common dark-fantasy word (Snyder). This is a new finding not foregrounded in C0–C5.

---

## Results Summary Table

| Condition | Context Provided | Clarity | Solv. | Eleg. | RR | Fun | Conf. | Total | Pass? |
|-----------|-----------------|---------|-------|-------|----|-----|-------|-------|-------|
| C0 | Expert authority only | 3 | 4 | 3 | 2 | 3 | 4 | **19** | FAIL |
| C1 | + Rubric | 3 | 4 | 2 | 2 | 3 | 4 | **18** | FAIL |
| C2 | + Principle names+quotes | 3 | 4 | 2 | 2 | 3 | 5 | **19** | FAIL |
| C3 | + Diagnostic tests | 3 | 3 | 1 | 2 | 3 | 5 | **17** | FAIL |
| C4 | Design Philosophy only | 3 | 4 | 2 | 2 | 3 | 4 | **18** | FAIL |
| C5 | Review Lens only | 3 | 4 | 1 | 2 | 3 | 4 | **17** | FAIL |
| C6 | Full profiles | 3 | 4 | 1 | 2 | 3 | 4 | **17** | FAIL |
| C7 | Full profiles + principles | 3 | 3 | 1 | 1 | 3 | 5 | **16** | FAIL |

---

## Per-Dimension Analysis

### Clarity (range: 3–3)
Flat across all conditions. Every condition from C0 upward identifies the same issue: the extraction instruction is buried in flavor text. This is the most context-independent finding — basic readability analysis produces it without any scaffolding. Context depth does not improve clarity scoring.

### Solvability (range: 3–4)
Drops from 4 to 3 only in C3 (full tests) and C7 (full everything). The "Ashveil" verification gap and the deductive shallowness of the fill step become visible only when Blame the Player is tested explicitly. C0–C2 miss this gap. Context depth does improve solvability precision, but the gap is small (1 point).

### Elegance (range: 1–3)
The biggest variance in the study. C0 scores 3 — the "something feels disconnected" intuition without a name. C1 drops to 2 (rubric forces the decoupling to be scored). C2–C3 drop to 2→1 (principles name the failures). C4 drops to 2 (philosophies converge on the same seam). C5–C7 score 1 consistently (lenses and tests make the decoupling unambiguous). Elegance is the most context-sensitive dimension.

### Reading Reward (range: 1–2)
Mostly flat at 2. Drops to 1 only in C7 (four independent signals converge). The dimension is easy to identify at all context levels — no Ironfall knowledge is required to solve — but C7's four-signal convergence justifies a lower score that no other condition reaches.

### Fun (range: 3–3)
Perfectly flat. Fun is the most context-independent dimension. No amount of scaffolding changes the basic quality of the solver experience. This is consistent with the hypothesis that fun is perceptual/affective and resistant to analytical frameworks.

### Confirmation (range: 4–5)
Rises from 4 (C0–C1) to 5 (C2+) because Verify the Last Mile prompts an explicit letter-by-letter trace. Without that principle, reviewers confirm at the gestural level ("seems correct") rather than operationally (verifying each index position). Context depth does improve confirmation rigor.

---

## Key Findings

### Finding 1: Diagnostic Tests (C3) are the most powerful single upgrade
The jump from C2 (principle names+quotes) to C3 (+tests) drives the largest single-condition score change: elegance from 2 to 1, solvability from 4 to 3. The tests are executable — they transform named principles into verdicts. Names alone trigger recognition; tests force application.

### Finding 2: Review Lenses (C5) outperform Design Philosophy (C4) at equivalent cost
C4 (philosophy only) scores 18. C5 (lens only) scores 17 and identifies the "stat values not load-bearing" failure precisely. The lenses are operationally sharper than the philosophies for individual puzzle evaluation. Philosophies explain *why* something is wrong; lenses identify *what* is wrong and *where*.

### Finding 3: Full Profiles (C6) add qualitative depth, not scoring precision
C6 scores identically to C5 (17) but generates three non-overlapping critiques: Katz's backsolving risk, Snyder's "two procedures" architecture failure, Dana's world-exit and answer-experience mismatch. These are distinct actionable findings that C5 does not produce. The profiles add editorial richness — they tell the author *how to fix it*, not just *that it is broken*.

### Finding 4: Maximum context (C7) uniquely produces the Confirmation/Surprise tension
Only C7 simultaneously scores Confirmation at 5 and Reading Reward at 1 for reasons that are causally linked: the extraction table is the puzzle's strongest element (makes confirmation trivial and airtight) and its weakest (removes the need to discover the year-sort aha). This productive tension is visible only when profiles and principles are combined.

### Finding 5: The puzzle's genuine strength is invisible to low-context conditions
The archive voice, the year-sort aha, and the fidelity to the fan-wiki genre are genuine strengths. Low-context conditions (C0–C1) can name these as "charming" or "pleasant" but cannot articulate *why* they work. High-context conditions (C6–C7) give Dana's framework ("the fill step is archivally authentic — the solver inhabits the archive world during Phase 1") which makes the strength specific and preservable in revision.

### Finding 6: The critical diagnosis is consistent across all conditions
Every condition from C0 to C7 identifies the same core failure: the stat-fill and the extraction are decoupled. The failure is visible even to a bare authority reviewer. What changes with context is: (a) the precision of the diagnosis, (b) the actionability of the revision guidance, and (c) the ability to name what works alongside what doesn't.

---

## Recommendation: What Context Level to Use

| Use case | Recommended condition | Rationale |
|----------|-----------------------|-----------|
| Fast pass/fail triage | C1 (rubric only) | Reliable PASS/FAIL, fast, no profiles needed |
| Identifying specific repair targets | C3 (principles + tests) | Tests are the highest-value single addition |
| Editorial guidance for the author | C6 (full profiles) | Profiles generate non-overlapping actionable critiques |
| Full diagnostic for difficult or borderline puzzles | C7 (full everything) | Maximum signal, productive tensions, complete coverage |
| Hunt-level structural review | C4/C6 (Katz philosophy/lens) | His framework is purpose-built for hunt architecture |

---

## Ironfall-Specific Note: Fictional Domain Transfer

The reviewer profiles (Katz, Snyder, Dana) know nothing about Ironfall — it is an invented game world. This ablation tests whether domain expertise in puzzle design generally transfers to an invented domain.

**Finding:** Transfer is complete. All three profiles apply successfully to a fictional domain because their frameworks operate on puzzle structure, not domain content. Snyder does not need to know what a Tundra Golem is — he only needs to ask whether the stat value for Tundra Golem ATK is load-bearing in the extraction. Dana does not need to have played Ironfall — she only needs to ask whether the extraction step stays inside the world established by the fill step. Katz does not need to know what Ashveil is — he only needs to ask whether the puzzle justifies its slot.

The fictional domain slightly strengthens the Reading Reward analysis: because no one has domain knowledge of Ironfall, the question "does Ironfall knowledge matter?" has a forced answer (no — it cannot, because it doesn't exist). This makes the Reading Reward finding sharper than it would be for a real-world domain where some solvers might have relevant knowledge.

---

*Study complete. Puzzle P01 fails at all context levels (range: 16–19/30). The primary failure — stat-fill and extraction are structurally decoupled — is visible at C0 but precisely diagnosed only at C3+. Maximum context (C7) generates the richest editorial guidance and the most actionable revision path.*
