# HUNT Invariants

These entries summarize properties that must remain true for HUNT stage gates,
craft doctrine, solver tests, simulator evidence, and distributable packaging.

## HUNT-I-01: Scenario Status Has A Single Durable Source

**Status:** PARTIAL

**Claim:** Each scenario can be resumed from durable stage status, deliverable
paths, review outcomes, and next-action state.

**Why it matters:** Crash-safe resume is a central product claim; manual or
stale status creates lost work and false progress.

**Enforcement:** `toolkit/skills/hunt/resume.md`, scenario `CLAUDE.md` files,
and the open `BUGS.md` status-table items define the required source of truth.

**Evidence:** `README.md`, `CLAUDE.md`, and `BUGS.md` issues 1 and 2.

## HUNT-I-02: Puzzle Outputs Preserve Craft Tests

**Status:** VERIFIED

**Claim:** Authored puzzles remain subject to fairness, domain, reading reward,
deduction, aha, interlock, extraction, and voice checks.

**Why it matters:** HUNT should not ship puzzle-shaped worksheets, quizzes,
search tasks, or mechanisms that cannot be verified after solving.

**Enforcement:** `toolkit/PRINCIPLES.md`, `/puzzle check`, reviewer profiles,
blind solver tests, and scenario review files preserve the craft contract.

**Evidence:** `toolkit/PRINCIPLES.md`, `toolkit/skills/puzzle/check.md`,
`toolkit/profiles/`, and `scenarios/*/reviews/`.

## HUNT-I-03: Answer Material Is Not Plain Public State

**Status:** MITIGATED

**Claim:** Plaintext answers and spoiler material do not appear in public,
git-tracked solver-facing artifacts.

**Why it matters:** Answer leakage destroys replayability and can expose hidden
layers before the hunt opens.

**Enforcement:** `README.md` answer-security rules, `BUGS.md` blocking issue 7,
scenario delivery separation, and future packaging checks define the boundary.

**Evidence:** `README.md`, `BUGS.md`, `toolkit/skills/hunt/publish.md`,
`docs/pitfall-boundaries.v1.json`, and `tests/check-pitfall-policy.ps1`.

## HUNT-I-04: Simulator Evidence Does Not Replace Playtest

**Status:** VERIFIED

**Claim:** RALLY-backed simulation may forecast timing, bottlenecks, variants,
and handoff pressure, but it does not certify puzzle fairness or solver delight
without blind tests and review.

**Why it matters:** Synthetic solver traces are useful for tuning, but puzzle
hunts fail on ambiguity, surprise, delight, and last-mile extraction.

**Enforcement:** HUNT keeps simulator commands, solver archetypes, review
profiles, and platform tests as separate evidence types.

**Evidence:** `tools/hunt-sim/README.md`, `tools/hunt-sim/src/main.rs`,
`toolkit/solvers/`, and `scenarios/*/tests/`.

## HUNT-I-05: Distribution Requires A Clean-Install Contract

**Status:** MITIGATED

**Claim:** The toolkit is not treated as an externally supported product until
discoverable skill entry points, coherent command hierarchy, dependency
reconstruction, and clean-install acceptance tests pass.

**Why it matters:** HUNT currently contains source command documents and rich
scenario evidence, but external users need a reproducible install path.

**Enforcement:** README packaging status, maintenance ownership, dependency
reconstruction rules, and bug backlog keep distribution claims bounded.

**Evidence:** `README.md`, `CLAUDE.md`, `BUGS.md`,
`docs/pitfall-boundaries.v1.json`, and `tests/check-pitfall-policy.ps1`.

## HUNT-I-06: Briefs Prove Extraction Before Authoring

**Status:** MITIGATED

**Claim:** Puzzle briefs and answer assignments cannot advance on concept
quality alone; answer words and extraction paths must be feasible against
world/source data and checked character by character.

**Why it matters:** A strong theme can still create an impossible puzzle if the
answer cannot be extracted from the available material.

**Enforcement:** `toolkit/PRINCIPLES.md`, `/puzzle author`, BUGS feasibility
rules, and the retained PITFALL policy test.

**Evidence:** `toolkit/PRINCIPLES.md`, `toolkit/skills/puzzle/author.md`,
`BUGS.md`, `docs/pitfall-boundaries.v1.json`, and
`tests/check-pitfall-policy.ps1`.
