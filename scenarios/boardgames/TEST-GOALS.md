# Scenario 2: Game Night — Test Goals

What Scenario 2 is designed to test that Scenario 1 (AoE) didn't cover.

---

## What Scenario 1 Tested

AoE validated the **pipeline itself**: 10 stages, review gates, world building, authoring, testing, meta design. Single author. Single round. Simple structure.

## What Scenario 2 Tests

Game Night validates the **multi-author workflow** — the messy human reality of 5 different people (or AI sessions) building puzzles independently that must integrate into one hunt.

---

## Test Goals

### G1: Module Isolation
**Question:** Can 5 authors work in parallel without stepping on each other?
**Success criteria:** Each module directory is self-contained. No author needs to read another's files to do their work. Briefs are sufficient for handoff.
**Measured by:** Whether any author asks "what did the other person do?"

### G2: Brief Quality
**Question:** Does the puzzle brief contain enough detail for a NEW author to write the puzzle without re-reading the pool ranking or attending the design meetings?
**Success criteria:** An author who sees only their `brief.md` and the `world/` data can produce a puzzle that passes testing.
**Measured by:** Whether authors ask clarifying questions about the brief. (Bug 9)

### G3: Personality Conflicts
**Question:** What happens when an author disagrees with the principles or the admin's direction?
**Success criteria:** The admin has a protocol for resolving disagreements. The Rebel's objections are evaluated fairly — sometimes the Rebel is right.
**Measured by:** Whether The Rebel's puzzle is better or worse than the compliant authors'.

### G4: Speed vs Quality
**Question:** Does the Speedrunner's fast-but-sloppy approach produce passing puzzles after revision? Is it faster overall than The Methodical's slow-but-correct approach?
**Success criteria:** Both approaches eventually produce passing puzzles. Track total time (authoring + testing + revision) for each.
**Measured by:** Total cycles to PASS for each author personality.

### G5: Meta Dependency
**Question:** Can the meta be designed AFTER all 5 puzzles are authored, when answer words are locked?
**Success criteria:** The admin can build a working meta from 5 independently-chosen answer words.
**Measured by:** Whether the crossword/meta is constructable. (This is the realistic case — metas are usually designed last.)

### G6: Integration Coherence
**Question:** Do 5 independently authored puzzles feel like ONE hunt?
**Success criteria:** The live test team (Stage 9) doesn't notice the puzzles were written by different authors. Tone, difficulty, and style feel consistent.
**Measured by:** Cheerleader's vibe check + Skeptic's consistency audit during live test.

### G7: Admin Tooling
**Question:** Does `/hunt admin` have everything the admin needs to manage 5 modules?
**Success criteria:** The admin can assign, track, review, and integrate without leaving the toolkit. No manual file editing required for routine operations.
**Measured by:** Whether the admin hits friction points that need new features. (Bug 8)

### G8: Onboarding
**Question:** Can `/hunt onboard` generate a brief that a new author understands immediately?
**Success criteria:** The Newbie personality (if simulated as an author, not solver) can start working from the onboarding brief alone.
**Measured by:** First-attempt quality of the Newbie author's puzzle.

---

## Findings Log

Fill in during and after the scenario.

| Goal | Finding | Status |
|------|---------|--------|
| G1 | | |
| G2 | | |
| G3 | | |
| G4 | | |
| G5 | | |
| G6 | | |
| G7 | Simulator follow-up: serial admin review is the schedule bottleneck; `parallel-review` and `ship-room` variants materially improve target-window success. | PROMOTE |
| G8 | | |

---

## Comparison: Scenario 1 vs Scenario 2

| Dimension | Scenario 1 (AoE) | Scenario 2 (Game Night) |
|-----------|-------------------|------------------------|
| Authors | 1 (single AI) | 5 (different personalities) |
| Rounds | 1 | 1 |
| Puzzles | 5 + 1 meta | 5 + 1 meta |
| Pipeline tested | Stages 1-10 | Stages 5-10 (assume 1-4 done by admin) |
| Key risk | Pipeline itself | Human coordination |
| Module structure | None (single author) | 6 modules (M1-M6) |
| Admin tooling | Minimal | Full `/hunt admin` workflow |
| World building | Web knowledge (AoE) | Board game rules (5 games) |
| Expected bugs | Pipeline bugs | Admin/handoff/integration bugs |
