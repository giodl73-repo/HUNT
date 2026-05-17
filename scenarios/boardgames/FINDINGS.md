# Findings — Game Night (Scenario 2)

**Stage 11: What did Scenario 2 teach us?**

Findings logged against the TEST-GOALS.md goals (G1-G8).

---

## G1: Module Isolation

**Question**: Can 5 authors work in parallel without stepping on each other?

**Finding**: YES, with caveats.

The five modules were fully self-contained. No author needed to read another author's puzzle file. The directory structure (M1-chess/, M2-settlers/, etc.) enforced clean separation. The world/ data files provided shared reference without shared state.

**The caveat**: The Social (M4) deliberately broke isolation by reaching out to The Methodical (M1). This cross-pollination produced a better Pandemic puzzle — the "constraint elimination" insight from chess improved M4's structure. But it also created a soft dependency: if M1 had been late, M4 would have stalled.

**Verdict**: Module isolation WORKS as a structural guarantee. Cross-module collaboration can improve quality but should be optional, not required. The toolkit should support both modes: isolated authoring (the default) and opt-in collaboration (for Socials who benefit from it).

**Status**: PASS

---

## G2: Brief Quality

**Question**: Does the puzzle brief contain enough detail for a NEW author to write the puzzle?

**Finding**: PARTIALLY. The briefs were sufficient for puzzle DESIGN but not for EXTRACTION VERIFICATION.

Every author successfully designed a puzzle mechanism that matched the brief's intent. The Methodical built a chess reconstruction. The Speedster built resource tracking. The Skeptic built continent identification. The Social built infection forensics. The Lurker built spymaster word-finding.

But 3 of 5 authors failed at the extraction step — the part where the mechanism produces the specific answer word. The briefs said "the answer word is X, extract it from the mechanism" but provided no guidance on HOW to verify the extraction. Authors assumed their extractions worked without end-to-end verification.

**Root cause**: The briefs specified WHAT (answer word, mechanism concept) but not HOW TO VERIFY (run the extraction, check every letter, confirm the word spells correctly). The verification step is a separate skill from the design step.

**Recommendation**: Add an "Extraction Verification Checklist" to every brief:
1. Run the extraction from scratch (pretend you're a solver)
2. Write down every extracted letter
3. Confirm they spell the answer word in order
4. If any letter is wrong, fix the extraction instruction (not the puzzle mechanism)

**Status**: PARTIAL PASS — design quality was good; extraction verification was poor

---

## G3: Personality Conflicts

**Question**: What happens when an author disagrees with the principles or the admin's direction?

**Finding**: THE SKEPTIC WAS RIGHT — and the compromise was better than either pure approach.

The Skeptic argued that numerical continent properties (territory count, bonus, entry points, adjacency degree) were more rigorous than "cryptic descriptions." The Captain mandated a compromise: embed the numbers in prose descriptions.

The live test showed that BOTH layers contributed. The Newbie identified continents from the prose ("the fortress," "the crossroads"). The Skeptic verified using the numbers. Casual players engaged with the descriptions; analytical players appreciated the embedded data. The compromise served both audiences.

However, the Skeptic's original objection was partially vindicated: the extraction failure was in the "prose wrapper" layer (territory names + letter indices), not in the structural identification layer (which the Skeptic would have handled differently).

**Verdict**: The admin's protocol for resolving disagreements (compromise, let the author vent in notes, evaluate the final product) worked. The Skeptic's puzzle was among the best-designed. The lesson: listen to Skeptics seriously. Their objections often identify real tensions. Even when you overrule them on the approach, their concerns can inform the review criteria.

**Status**: PASS — conflict resolved productively

---

## G4: Speed vs Quality

**Question**: Does the Speedster's fast-but-sloppy approach produce passing puzzles after revision? Is it faster overall?

**Finding**: NO — the Speedster's approach was the SLOWEST when accounting for revision cycles.

| Author | Authoring time | Review cycles | Total time | Final quality |
|--------|---------------|---------------|------------|---------------|
| Methodical (M1) | 2h 14m | 0 (pass first try) | 2h 14m | Excellent |
| Speedster (M2) | 12m + fixes | 2+ (still broken) | 2h+ and counting | FAIL |
| Skeptic (M3) | 48m + 25m arguing | 1 (extraction fix needed) | ~1h 30m | Good (mechanism) |
| Social (M4) | 1h 38m | 1 (extraction fix needed) | ~2h | Good (mechanism) |
| Lurker (M5) | Unknown | 0 (pass with minor edits) | Unknown | Good |

The Speedster shipped in 12 minutes but the puzzle has 3 critical bugs. Two revision rounds later, it still doesn't work. The total investment (authoring + review + feedback + re-review) exceeds the Methodical's 2h 14m of careful first-pass work — and the Methodical's puzzle PASSED.

**The Speedster's contribution**: The port-deduction concept IS clever. The idea of deducing which port was used from resource ratios is worth preserving. But the execution — resource math, extraction, answer connection — was never verified. Speed without verification produces more work for everyone downstream.

**Verdict**: For puzzle authoring, "slow and correct" beats "fast and broken" on total throughput. The Speedster profile is more valuable as a SOLVER (finding shortcuts, generating early momentum) than as an AUTHOR (shipping unverified work).

**Status**: FAIL — Speedster approach does not produce passing puzzles efficiently

---

## G5: Meta Dependency

**Question**: Can the meta be designed AFTER all 5 puzzles are authored, when answer words are locked?

**Finding**: YES — but it required extensive combinatorial work and the meta answer is constrained.

The 5 uncoordinated answer words (CASTLE, TRADE, BORDER, SPREAD, CIPHER) were workable but not trivially elegant. The Captain tested 15+ target words and dozens of index combinations before finding SCARE. The mechanism (game-trivia questions producing indices into feeder answers) is sound but the designer had very little control over WHAT the meta answer would be.

The available letter pool at each index (1-6) per word was:
```
CASTLE: C A S T L E
TRADE:  T R A D E
BORDER: B O R D E R
SPREAD: S P R E A D
CIPHER: C I P H E R
```

Many desirable meta answers (SCORE, BOARD, GAMES, MATCH) were impossible because needed letters didn't appear in any word at any index. SCARE was the best available — thematically connected to the Host's personality but not an obvious "board game" word.

**Lesson**: Uncoordinated answer words make meta design roughly 10x harder. Even minimal coordination ("your answer must contain an A" or "your answer should be 6 letters") would dramatically expand options. For real hunts, the toolkit should recommend light constraints on answer words that preserve author freedom while enabling meta design.

**Status**: PASS — meta was constructable, but barely

---

## G6: Integration Coherence

**Question**: Do 5 independently authored puzzles feel like ONE hunt?

**Finding**: MOSTLY YES — the board-game theme creates natural cohesion.

The five puzzles use different mechanisms (reconstruction, resource tracking, continent matching, infection forensics, word association) but they all feel like "game night" because each one IS its respective board game. The Riven Standard — "the puzzle IS what the section does" — serves as a unifying principle even when five different authors interpret it differently.

**What DOES feel inconsistent**:

1. **Author's notes in puzzle files.** M1, M3, and M4 have extensive authoring detritus. M2 has none. M5 has a spoiler section. In a solver-facing version, these must all be stripped, but the current files read like five different internal documents, not one polished hunt.

2. **Difficulty variation.** M5 (Codenames) is significantly easier than M1 (Chess). A 30-second solve and a 25-minute solve feel like different events. This is partially intentional (the hunt has a difficulty curve) but the gap is larger than ideal.

3. **Tone.** The Skeptic's editorial commentary, the Social's iteration history, and the Lurker's dead silence create different authorial vibes. The Host's voice (defined in ROUNDS.md) was supposed to unify these, but the Host's flavor text only appears in the intro and doesn't wrap individual puzzles.

**Recommendation**: Add Host flavor text to each puzzle's opening and closing. The Host's voice should bracket every puzzle, creating a consistent narrator layer even when the puzzle mechanisms differ.

**Status**: PARTIAL PASS — theme creates coherence; polish and narrator consistency need work

---

## G7: Admin Tooling

**Question**: Does the admin have everything needed to manage 5 modules?

**Finding**: The admin managed through direct communication and file-checking, not through automated tooling.

The Captain's workflow was:
1. Issue briefs → manual (write brief.md, send to author)
2. Track status → manual (ask each author, check file timestamps)
3. Review submissions → manual (read puzzle.md, verify against world data)
4. Resolve conflicts → manual (conversation with Skeptic)
5. Design meta → manual (combinatorial search)

**Friction points**:

1. **No automated extraction verification.** The Captain had to manually trace every extraction to find the 3 broken ones. A tool that takes (puzzle mechanism → extracted letters → target word) and verifies the chain would have caught all 3 bugs instantly.

2. **No status dashboard.** The Captain had to message each author individually and check file timestamps for the Lurker. A simple status board (module → submitted? → reviewed? → passed?) would save time.

3. **No diff between author versions.** When the Speedster submitted v2, the Captain had to re-review the entire file to find what changed. A diff tool would help.

4. **No separation of author-facing and solver-facing content.** Every puzzle file mixes the two. A build step that strips author notes and produces a clean solver-facing version would prevent the spoiler problem.

**Status**: PARTIAL PASS — the admin managed, but several tools would significantly improve the workflow

---

## G8: Onboarding

**Question**: Can a new author understand the brief immediately?

**Finding**: The briefs were understood by all authors. The Newbie (as solver) was onboarded effectively by the Codenames puzzle.

For authoring: All five personalities understood their briefs (the Methodical asked clarifying questions, but these were about design choices, not comprehension). The brief format (puzzle concept + answer word + world data reference + principles) was sufficient.

For solving: The Newbie's first puzzle (Codenames) served as a natural on-ramp. The familiar board-game format (Codenames grid, spymaster role) translated directly into puzzle-hunt solving. The Newbie said: "The Codenames format really helped — I already knew what a spymaster does." This validates the Riven Standard as an onboarding mechanism: when the puzzle IS the game, board-game knowledge IS puzzle-hunt knowledge.

**The Host's voice**: The Newbie found the Host's opening text ("The board is out. The pieces are sorted. Mostly.") natural and approachable. The short sentences, present tense, and slight anxiety ("Nobody asked for a timer but it felt right") created a welcoming tone without being condescending. The Newbie did NOT ask "what's a puzzle hunt?" — the Host's framing ("Tonight you solve them") was sufficient.

**Status**: PASS

---

## Summary Table

| Goal | Finding | Status |
|------|---------|--------|
| G1: Module Isolation | Modules self-contained. Cross-pollination optional and beneficial. | PASS |
| G2: Brief Quality | Briefs good for design, weak for extraction verification. Need checklist. | PARTIAL PASS |
| G3: Personality Conflicts | Skeptic's objection was valid. Compromise was productive. Admin protocol works. | PASS |
| G4: Speed vs Quality | Speedster approach is slower overall due to revision cycles. Slow+correct wins. | FAIL |
| G5: Meta Dependency | Meta constructable from uncoordinated words, but barely. Need light constraints. | PASS |
| G6: Integration Coherence | Theme creates coherence. Narrator voice and polish need work. | PARTIAL PASS |
| G7: Admin Tooling | Admin managed manually. Extraction verification, status board, and build tools needed. | PARTIAL PASS |
| G8: Onboarding | Briefs understood by authors. Codenames on-ramp works for solvers. Host voice is effective. | PASS |

---

## Key Lessons for the Toolkit

### Lesson 1: Extraction is a separate skill from puzzle design.

Three of five authors designed excellent puzzle mechanisms but failed at the extraction step. The toolkit needs:
- An extraction verification checklist in every brief
- A tool or protocol for end-to-end extraction testing
- The editorial review must specifically check extraction correctness, not just puzzle mechanism quality

### Lesson 2: The Speedster profile is better suited for solving than authoring.

Fast authoring without verification creates more downstream work than slow, careful authoring. The Speedster's ideas are valuable (the port-deduction concept is clever) but the execution needs a second pass — either by the Speedster or by a reviewer. Consider pairing Speedsters with Methodicals: Speedster generates the concept, Methodical verifies the execution.

### Lesson 3: The Skeptic is the most valuable reviewer.

The Skeptic found every extraction bug, flagged the Epidemic Infect inconsistency, caught the chess phrasing ambiguity, and identified the meta's piece/pawn edge case. The Skeptic profile should be used primarily as a REVIEWER, not just as an author. Every puzzle should pass a Skeptic review before ship.

### Lesson 4: Uncoordinated answer words work but constrain the meta.

Five independently chosen words produced a workable meta (SCARE via indexed extraction). But the designer had almost no control over the meta answer. For real hunts, provide authors with minimal answer-word constraints (e.g., "must contain at least one of {A, E, R}") to ensure meta flexibility.

### Lesson 5: The Riven Standard is the strongest unifying principle.

When every puzzle IS its game, five different authors with five different styles still produce a cohesive hunt. The board-game theme does the integration work that would otherwise require careful style coordination. The Riven Standard should be the first principle applied and the last one evaluated.

### Lesson 6: The Host narrator needs to wrap individual puzzles, not just the intro.

The Host's voice was effective in the opening but absent from individual puzzles. Adding Host flavor text to each puzzle's opening and closing would create the narrator consistency that G6 requires. Each author wrote their own flavor text (the Methodical's napkin framing, the Social's "how did it get this bad?" opening) — these should be supplemented, not replaced, by the Host's wrapping voice.

### Lesson 7: Multi-author hunts need a ship-room, not a serial admin queue.

The RALLY-backed Game Night simulator replayed the scenario across author
handoff, rework, admin visibility, and meta integration risk. The baseline model
confirmed the postmortem intuition: once five modules exist independently, the
bottleneck is no longer authoring, it is serial review plus late meta assembly.

`parallel-review` was the first strong variant because it removes the admin
queue without changing puzzle content. `ship-room` was the best full operating
mode because it combines parallel review, editorial relief, light meta briefing,
and standups for low-visibility authors. This should become the default
multi-author workflow for future Scenario 2 descendants.

---

## Scenario 2 vs Scenario 1 Comparison

| Dimension | Scenario 1 (AoE) | Scenario 2 (Game Night) |
|-----------|-------------------|------------------------|
| Pipeline tested | Full 10-stage pipeline | Multi-author workflow (Stages 5-10) |
| Key finding | Pipeline works end-to-end | Extraction is the weak point across authors |
| Biggest risk realized | — | Speedster ships broken puzzles faster than they can be fixed |
| Most valuable role | Single author doing everything | The Skeptic (as reviewer) |
| Meta construction | Designed with coordinated answers | Designed from uncoordinated answers (hard but doable) |
| Integration quality | Naturally coherent (single author) | Thematically coherent, editorially inconsistent |
| Onboarding | N/A (no Newbie test) | Effective — Codenames on-ramp works |

---

## What Scenario 3 Should Test

Based on Scenario 2 findings, the next scenario should explore:

1. **Revision workflow**: How many cycles does it take to fix a REVISE puzzle? What does the feedback-revision-retest loop look like?
2. **Extraction tooling**: Can we build a tool that verifies extractions automatically?
3. **Larger scale**: 10+ puzzles with 3+ rounds — do the coordination problems scale?
4. **Author pairing**: Speedster + Methodical as a pair — does the combination produce faster AND correct work?
5. **Coordinated answer words**: Give authors minimal constraints and measure the improvement in meta design flexibility.
