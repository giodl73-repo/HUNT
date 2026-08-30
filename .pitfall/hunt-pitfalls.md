# HUNT Pitfalls

These entries capture recurring puzzle-hunt pipeline failure classes and map
them to HUNT's current controls or open repo-local issues.

## HUNT-PF-01: Review Gates Become Decoration

**Status:** MITIGATED

**Pattern:** Stage reviews, solver tests, editorial checks, or parliament
roles are present as documents but do not block advancement when a puzzle is
unfair, dull, broken, or unshippable.

**Domain:** Stage gates, `/hunt review`, `/puzzle test`, editorial review,
platform tests, print/web delivery, and final polish.

**Detection difficulty:** Authors can be attached to their own mechanisms, and
review prose can look substantial even when no pass/fail decision is enforced.

**Structural solution:** Use explicit role panels, blind solver personas,
scenario review files, and simulator validation as separate acceptance signals.

**Evidence:** `.roles/ROLE.md`, `toolkit/skills/hunt/review.md`,
`toolkit/skills/puzzle/test.md`, and `scenarios/*/reviews/`.

## HUNT-PF-02: Scenario Coupling Ships As Generic Toolkit

**Status:** OPEN

**Pattern:** A toolkit skill hardcodes one scenario's path, answer protocol,
secret encoding, project-specific file, or author workflow and then fails for
new users.

**Domain:** `/hunt plan`, `/puzzle author`, `/puzzle test`, package install,
scenario initialization, user-facing toolkit docs, and reference backports.

**Actor:** Toolkit maintainer, scenario author, external puzzle-hunt producer,
or portfolio dependency adopter.

**Task:** Install or reuse HUNT commands for a new hunt that did not create the
original scenario-specific workflow.

**Surface:** `toolkit/skills/`, `toolkit/GETTING-STARTED.md`, README quickstart,
scenario initialization, answer protocol prompts, and reference backports.

**Likely mistake:** Treat a command that worked for one scenario as a generic
toolkit contract even though paths, answer custody, vocabulary, or author
workflow are still scenario-specific.

**Consequence:** New users hit missing files, leaked secrets, broken resume
state, or a failed clean install after the portfolio has advertised HUNT as a
reusable production pipeline.

**Owner:** `gate-integrity-steward`, `ship-readiness-auditor`, and toolkit
maintainers.

**Detection difficulty:** The pipeline works in the scenario that created the
skill, so the coupling is invisible until a clean repo or external user tries
the command.

**Structural solution:** Replace hardcoded paths and secrets with scenario
contracts, prompts, relative paths, and a clean-install acceptance test.

**Evidence:** `README.md`, `CLAUDE.md`, and `BUGS.md` issues 1, 3, 4, 5, 6,
10, and 11.

**Test:** `pwsh -NoProfile -File tests\check-pitfall-policy.ps1`

## HUNT-PF-03: Plaintext Or Prohibited Content Leaks Into Git

**Status:** OPEN

**Pattern:** Answers, spoilers, hidden-layer material, or copyrighted lyric text
enter tracked files or solver-facing delivery artifacts.

**Domain:** Puzzle pools, answer sheets, hints, solution drafts, music hunts,
delivery sites, print packs, and publish archives.

**Actor:** Puzzle author, editor, publisher, scenario maintainer, or future
packaging agent.

**Task:** Move a puzzle, hint set, solution draft, music-hunt source, or
publish archive from working state toward solver-facing delivery.

**Surface:** `PUZZLES.md`, `PUZZLE-POOL.md`, `ANSWERS.md`, hints, solution
drafts, `delivery/`, `/hunt publish`, music-domain world files, and git-tracked
scenario artifacts.

**Likely mistake:** Keep plaintext answers, hidden-layer material, or lyric
text in tracked files because authors need it during construction and testing.

**Consequence:** The hunt is spoiled, replayability is lost, prohibited content
can ship, and downstream packaging cannot prove that solver-facing artifacts
were scrubbed.

**Owner:** `ship-readiness-auditor`, `gate-integrity-steward`, and scenario
publish owners.

**Detection difficulty:** Authors need answer material while building and
testing, and local drafts can be accidentally promoted into public artifacts.

**Structural solution:** Enforce encoded or gitignored answer custody, prohibit
lyrics in puzzle content, and make publish/package checks reject exposed answer
or prohibited text.

**Evidence:** `README.md`, `BUGS.md` issues 7 and 17,
`toolkit/skills/hunt/publish.md`, and `scenarios/wavelength/`.

**Test:** `pwsh -NoProfile -File tests\check-pitfall-policy.ps1`

## HUNT-PF-04: Mechanism Works But Extraction Fails

**Status:** OPEN

**Pattern:** A puzzle has a strong concept but the final answer cannot be
derived because indices, letters, source data, answer words, or meta
constraints were not verified before authoring.

**Domain:** Puzzle briefs, answer assignment, Stage 4 assignment, Stage 5 meta
design, multi-author handoff, and final puzzle testing.

**Actor:** Puzzle author, assignment editor, meta designer, blind-test reviewer,
or scenario producer.

**Task:** Issue a puzzle brief or assign a target answer before the source data
and extraction path have been checked.

**Surface:** `PUZZLES.md`, per-puzzle briefs, Stage 4 assignment, Stage 5 meta
design, `/puzzle author`, `/puzzle test`, `toolkit/PRINCIPLES.md`, and
scenario test reports.

**Likely mistake:** Approve a clever mechanism or theme while the actual answer
word cannot be extracted letter by letter from the puzzle's domain material.

**Consequence:** Authors spend substantial time on an impossible puzzle, meta
design loses flexibility, and blind testers encounter dead ends that should
have been blocked at brief issue.

**Owner:** `puzzle-fairness-editor`, `solver-experience-advocate`, and
`gate-integrity-steward`.

**Detection difficulty:** Mechanism quality can distract reviewers from the
last mile, and answer-word impossibility may only appear after substantial
authoring work.

**Structural solution:** Add mechanical feasibility checks to briefs, validate
answer words against world/source data before issue, and verify extraction
letter by letter.

**Evidence:** `toolkit/PRINCIPLES.md`, `BUGS.md` issues 9 and 18,
`scenarios/boardgames/tests/`, and `tools/hunt-sim/README.md`.

**Test:** `pwsh -NoProfile -File tests\check-pitfall-policy.ps1`

## HUNT-PF-05: Simulation Is Mistaken For Solver Truth

**Status:** MITIGATED

**Pattern:** Seeded solver simulations or variant comparisons are treated as
proof that a puzzle is fair, delightful, or shippable.

**Domain:** `tools/hunt-sim`, RALLY comparison packets, hint timing, bottleneck
analysis, meta readiness, admin cadence, and production scheduling.

**Detection difficulty:** Simulation output is quantitative and repeatable,
while human delight, ambiguity, frustration, and aha quality are messier.

**Structural solution:** Keep simulator evidence bounded to timing, pressure,
variants, and handoff risk; require blind tests and role reviews for craft
quality and ship readiness.

**Evidence:** `tools/hunt-sim/README.md`, `toolkit/solvers/`,
`scenarios/*/tests/`, and `.roles/ROLE.md`.
