# HUNT Principles

These entries summarize durable HUNT decision rules for puzzle craft, staged
production, review gates, solver evidence, and distributable toolkit safety.

## HUNT-P-01: Gates Must Block Weak Work

**Status:** ACTIVE

**Statement:** A stage gate is only useful when it can reject unsolvable,
unfair, uninteresting, unshippable, or unsafe hunt work.

**Rationale:** HUNT's value comes from making puzzle quality visible before
teams encounter a broken stage, puzzle, meta, website, print pack, or prop.

**Decision rule:** Stage advancement requires the relevant craft, playtest, or
parliament review evidence rather than author confidence alone.

**Evidence:** `README.md`, `.roles/ROLE.md`, `toolkit/skills/hunt/review.md`,
and scenario review files under `scenarios/*/reviews/`.

## HUNT-P-02: The Puzzle Is The Domain

**Status:** ACTIVE

**Statement:** A puzzle should make the solver practice the domain rather than
apply an unrelated mechanism pasted over a theme.

**Rationale:** The strongest HUNT principle is the Riven Standard: a solver
should learn and prove understanding through the activity the puzzle claims to
represent.

**Decision rule:** If the puzzle still works after removing the domain, world,
or source material, revise the mechanism before shipping.

**Evidence:** `toolkit/PRINCIPLES.md`,
`research/publications/games-profile-taxonomy-creative/PRINCIPLES.md`, and
`examples/age-of-empires/reviews/world-verification.md`.

## HUNT-P-03: Blind Solvers Are Production Evidence

**Status:** ACTIVE

**Statement:** Puzzle confidence comes from blind solver personas, explicit
scores, review notes, and integration tests, not from author intent.

**Rationale:** Fairness, aha count, extraction clarity, hint timing, and final
meta readiness fail in ways authors usually cannot see.

**Decision rule:** Puzzles and stages that lack blind-test, editorial, or
platform-test evidence stay in development status.

**Evidence:** `README.md`, `toolkit/solvers/`,
`scenarios/*/tests/*`, and `tools/hunt-sim/README.md`.

## HUNT-P-04: The Toolkit Is Not A Scenario

**Status:** ACTIVE

**Statement:** Generic toolkit commands, templates, and packaging must not
depend on one scenario's secrets, paths, answer encoding, vocabulary, or author
workflow.

**Rationale:** HUNT is meant to become a reusable production pipeline; hidden
scenario coupling makes clean installs and external users fail.

**Decision rule:** Scenario-specific lessons may become toolkit rules only
after the generic contract, relative paths, user choices, and acceptance test
are explicit.

**Evidence:** `README.md`, `CLAUDE.md`, `toolkit/GETTING-STARTED.md`, and
`BUGS.md`.

## HUNT-P-05: Answer Custody Is A Release Boundary

**Status:** ACTIVE

**Statement:** Answers, hidden layers, spoilers, copyrighted text, and solver
state require explicit custody and release rules.

**Rationale:** A puzzle hunt can be spoiled or made unsafe for distribution by
tracked plaintext answers, exposed solution material, lyrics, or hidden state
that leaks before play.

**Decision rule:** Store only the permitted encoded or hidden answer material,
exclude prohibited copyrighted text, and treat answer exposure as a blocking
release defect.

**Evidence:** `README.md`, `BUGS.md`, `toolkit/HINTS.md`, and
`scenarios/*/delivery/`.
