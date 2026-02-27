# Bugs — Known Issues and Fixes

Track bugs found during scenario testing. Fix and document.

---

## Open

| # | Severity | Location | Description | Found during |
|---|----------|----------|-------------|-------------|
| 1 | Major | puzzle-plan skill | Stage 1 should auto-generate the scenario CLAUDE.md from scope answers. Currently the scenario CLAUDE.md must be written manually — the skill doesn't produce it. | AoE Stage 1 |
| 2 | Major | All skills | Skills should update the scenario CLAUDE.md status table as they produce deliverables (e.g., mark Stage 1 ✅ when SCOPE.md is written, Stage 2 ✅ when ROUNDS.md is filled). Currently no skill touches the scenario CLAUDE.md. | AoE Stage 1 |
| 3 | Minor | puzzle-plan skill | Skill references paths like `puzzle-hunt/PRINCIPLES.md` but in the toolkit structure the path is `toolkit/PRINCIPLES.md`. Path references need to be relative to the scenario, not hardcoded. | AoE Stage 1 |
| 4 | Minor | puzzle-author skill | Skill references `puzzle-hunt/FINAL-52.md` which is project-specific. Generic toolkit should reference the scenario's `PUZZLES.md` instead. | AoE Stage 1 |
| 5 | Minor | puzzle-test skill | Skill references `puzzle-hunt/TEST-CREWS.md` which is project-specific. Generic toolkit should reference the scenario's `PUZZLES.md` tester assignments. | AoE Stage 1 |
| 6 | Major | toolkit CLAUDE.md + answer protocol | Periodic table encoding is OUR project-specific secret — don't ship it in the generic toolkit. During /puzzle-plan Stage 1, ASK the user to choose their own encoding. Never hardcode one system. | AoE Stage 3 |
| 7 | Blocking | All git-tracked files | Plaintext answers must NEVER appear in git-tracked files — searchable in history forever. Answers only in .claude/ project memory (gitignored) or encoded. Puzzle pool descriptions must not contain answer words. | AoE Stage 3 |
| 8 | Major | toolkit structure | Need `admin/` directory with skills for post-plan administration: module assignment, author onboarding, progress tracking, review scheduling, integration checklist. Currently only creative skills, no admin tooling. | AoE Stage 3 |
| 9 | Major | PUZZLES.md + puzzle briefs | Registry needs more detail per puzzle for handoff to other authors. Need full briefs: mechanism, interlock design, extraction, references, voice draft. Either expand PUZZLES.md or link to per-puzzle brief files. An interactive script or `/puzzle-brief` skill that walks through brief creation would help. | AoE Stage 4 |
| 10 | Minor | /puzzle-plan | Should offer an interactive walkthrough mode that asks questions step-by-step rather than requiring the user to know what to fill in. Like a wizard/script. | AoE Stage 4 |
| 11 | Major | /puzzle-author, /puzzle-test | These skills don't leverage /puzzle — they should use /puzzle <id> brief to read the full brief, /puzzle <id> status to update status, /puzzle <id> comment to log actions. Currently each skill independently reads PUZZLES.md. The /puzzle skill should be the single data layer all other skills go through. | AoE Stage 4 |
| 12 | Major | Scenario structure | Each scenario needs a `world/` directory with verified reference data (facts, rules, data tables). Puzzles reference this — not the author's memory or web searches. For AoE: civs, units, techs, maps, counters. For any hunt: the knowledge base the puzzles are built on. Also need a `World.md` overview and a _vocab entity for facts. Without this, puzzle authors make inconsistent claims. | AoE Stage 5 |

## Fixed

| # | Severity | Location | Description | Fix | Fixed in |
|---|----------|----------|-------------|-----|---------|
| | | | | | |

---

## Bug Categories

| Severity | Definition |
|----------|-----------|
| **Blocking** | Prevents a skill from running or a puzzle from being solvable |
| **Major** | Skill runs but produces wrong output, or puzzle has an unintended solution |
| **Minor** | Cosmetic, unclear instructions, or edge case that rarely triggers |

## How to Report

Add a row to the Open table with:
- Sequential number
- Severity
- Which file/skill the bug is in
- What happened vs. what should have happened
- Which scenario/stage exposed it

When fixed: move row to Fixed table, note the fix and commit hash.
