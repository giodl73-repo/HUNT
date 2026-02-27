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
