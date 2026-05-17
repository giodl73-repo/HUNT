# HUNT — A Puzzle Hunt Production Pipeline for Claude Code

Build a real puzzle hunt — from blank page to ship-ready print-and-web package — with an
agent that knows what good hunts look like, whose review gates block you until the work
clears the bar, and whose authors, editors, testers, and prop managers are all the same
LLM playing different roles from different files.

**Review roles:** This repo uses
[ROLES](https://github.com/giodl73-repo/ROLES), the `.roles` convention for
repository-local review panels.

This is one of the first large, multi-stage **skills pipelines** built on Claude Code:
17 slash-command skills, 30 reviewer personas, 12 solver archetypes, 20 design
principles with failing/passing tests, and a crash-safe 11-stage workflow that has
already produced nine end-to-end hunts in the `scenarios/` directory.

---

## Goals

1. **Make puzzle hunt construction legible.** Hunts are notoriously hard to design —
   most of the craft lives in veteran heads. This repo pulls it onto disk: principles
   with sources, reviewer profiles with philosophies, stage gates with pass/fail tests.
2. **Put a review panel in the loop.** No puzzle ships without blind testing from at
   least three solver personas and editorial review from the relevant expert lenses.
   Gates aren't decoration — they reject work that can't be solved, isn't fair, or
   isn't interesting.
3. **Prove the methodology with real scenarios.** Every skill in `toolkit/` earned its
   place by being needed during an actual run. If a stage in `scenarios/` hit a problem
   the toolkit couldn't solve, the fix became a skill, and `BUGS.md` tracked it.
4. **Ship something a non-expert can use.** A first-time hunt runner copies `toolkit/`,
   types `/hunt plan`, and the pipeline walks them through every decision with the
   panel grading as they go.

---

## Why "first major skills pipeline"

Claude Code skills are usually single-purpose — format a file, review a PR, generate a
commit message. This project chains 17 of them into a dependency-ordered workflow with:

- **Typed hand-offs.** Stage N writes deliverables Stage N+1 reads. `_vocab/*.json`
  freezes the shape of every artifact (stage, puzzle, round, module, review, test,
  fact, honor).
- **Review gates between every stage.** `/hunt review` invokes a configurable subset of
  the 30 reviewer profiles. Each profile is a documented design philosophy with source
  citations, not a generic "be critical."
- **Crash-safe resume.** `/hunt resume` reads the scenario's `CLAUDE.md` status table
  and re-enters the pipeline at the next incomplete stage — a skill can be killed
  mid-run and the next session picks up cleanly.
- **Two namespaces, one data layer.** `/hunt *` operates at the hunt level (plan,
  world, meta, site, print, props). `/puzzle *` operates at the puzzle level (brief,
  author, test, check). Both read and write the same registry.
- **A principled backport loop.** Bugs surfaced during a scenario update the toolkit,
  the reference library, and the scenario skills in lockstep. `BUGS.md` is the
  receipt.

If you're building your own skills pipeline this size, this repo is a reference
implementation.

---

## Quickstart

```bash
# Clone
git clone https://github.com/giodl73-repo/HUNT.git
cd HUNT

# Install skills into your global Claude Code config
cp -r toolkit/skills/* ~/.claude/skills/        # macOS / Linux
xcopy /E toolkit\skills\* %USERPROFILE%\.claude\skills\   # Windows

# Open Claude Code in the repo and start the pipeline
claude
> /hunt plan
```

A 10-step walkthrough — from blank directory to print-ready hunt — is in
`toolkit/GETTING-STARTED.md`.

---

## The 11-stage pipeline

```
Stage  1  SCOPE              what's the hunt about?
          ↓ review gate
Stage  2  STRUCTURE          rounds, puzzle counts, metas
          ↓ review gate
Stage  3  PUZZLE POOL        brainstorm candidates
          ↓ review gate  (panel ranks the pool)
Stage  4  ASSIGNMENT         assign puzzles to slots
          ↓ review gate
Stage  5  META DESIGN        how feeder answers combine
          ↓ review gate
Stage  6  AUTHORING          write every puzzle
          ↓ testing gate  (3 solver personas blind test each)
Stage  7  EDITORIAL          admin reviews submissions
          ↓ edit gate
Stage  8  INTEGRATION        assemble, verify crossings
          ↓ review gate
Stage  9  DELIVERY BUILD     website, print, props, UX
          ↓ build gate
Stage 10  PLATFORM TEST      it works in the actual delivery medium
          ↓ test gate
Stage 11  POLISH             final pass, hints, answer encoding
          ↓ ship it
```

You can loop back to any earlier stage. The panel's job at each gate is to catch what
you can't see.

## Simulator

`tools/hunt-sim/` is the RALLY-backed puzzle-hunt simulation pilot. It starts
with `scenarios/wavelength/` and reports seeded feeder solve times, hint
pressure, meta readiness, and target-window findings.

```powershell
cd tools\hunt-sim
cargo run --quiet -- --seed wavelength-smoke
cargo run --quiet -- --seed wavelength-smoke --runs 12
cargo run --quiet -- --seed wavelength-smoke --compare-variants --runs 12
```

---

## The skills

### `/hunt` — hunt-level (the admin runs these)

| Command | What it does |
|---|---|
| `/hunt plan` | 11-stage design workflow with review gates |
| `/hunt world` | Fictional universe design — canon, data tables, lock protocol |
| `/hunt review full <file>` | Multi-expert panel review of any document |
| `/hunt review rank <file>` | Panel ranks a puzzle pool |
| `/hunt resume` | Re-enter the pipeline at the next incomplete stage |
| `/hunt publish` | Build solver-facing distribution zip (strips answers + working files) |
| `/hunt script` | Generate game architecture — scenes, navigation, state, widgets |
| `/hunt status` | Pipeline dashboard |
| `/hunt meta` | Meta design, verification, grid building |
| `/hunt edit` | Editorial review of author submissions before blind testing |
| `/hunt print` | Print-ready PDFs for puzzles, props, labels, game manual |
| `/hunt props` | Physical asset logistics — inventory, team kits, day-of checklist |
| `/hunt site` | Hunt website — scaffold, theme, puzzle pages, standings, submission |
| `/hunt admin` | Module assignment, author registry, schedule, integration checklist |
| `/hunt profile` | Claim a NATO phonetic callsign for an author |

### `/puzzle` — puzzle-level (authors use these)

| Command | What it does |
|---|---|
| `/puzzle <id>` | Overview — brief, status, scores, history |
| `/puzzle <id> brief` | View or edit the full brief |
| `/puzzle <id> author` | Write the puzzle from its brief |
| `/puzzle <id> test` | 3 solver personas solve it blind, score it |
| `/puzzle <id> test iterate` | Fix and retest until it passes (≥ 22/30) |
| `/puzzle <id> check` | Run against the 20 design principles |
| `/puzzle list` / `/puzzle board` | Registry views |

---

## The review panel

**30 expert reviewers** across puzzle construction, game design, narrative, deductive
rigor, accessibility, and buildability. Each profile in `toolkit/profiles/` is a
documented philosophy with sources — not a generic critic. A partial sample:

| Lens | Reviewer |
|---|---|
| Structure & pacing | Dan Katz (MIT Mystery Hunt) |
| Puzzle craftsmanship | Thomas Snyder (3× World Sudoku Champion) |
| Narrative & experience | Mike Selinker (*Puzzlecraft*) |
| Deductive rigor | Wei-Hwa Huang (4× World Puzzle Champion) |
| Buildability | Kenny Young, Dana Young (MS Puzzlehunt veterans) |
| World-as-puzzle | Rand Miller (Myst, Riven) |
| Epiphany design | Jonathan Blow (Braid, The Witness) |
| Deductive identification | Lucas Pope (Papers Please, Obra Dinn) |
| Accessibility & wonder | Alex Rosenthal (TED-Ed) |
| Theory | Mark Gottlieb (MIT thesis on hunt structure) |

The full 30 are in `toolkit/profiles/`.

**12 solver archetypes** in `toolkit/solvers/` simulate blind testing: the speedster,
the methodical, the lateral, the newbie, the skeptic, the specialist, and six more.
Every puzzle is tested by three of them before it can clear Stage 6.

---

## Design principles

Twenty principles, each with a source, a one-line statement, and a failing test. The
top five:

1. **The Riven Standard** — the puzzle IS what the field does, not overlaid on it.
2. **Solving = Proving Understanding** — the solver knows more after solving.
3. **Blame the Player** — every clue is fair in retrospect.
4. **No Over-Scaffolding** — worksheets provide space, not instructions.
5. **No Computation Without Deduction** — pure-computation puzzles always fail testing.

The rest are in `toolkit/PRINCIPLES.md` with their sources and tests.

---

## Repo layout

```
HUNT/
├── README.md              ← this file
├── CLAUDE.md              ← development instructions (for contributors)
├── BUGS.md                ← issue tracker — every scenario feeds back here
├── MODULES.md             ← module taxonomy (scope types the pipeline supports)
│
├── toolkit/               ← THE DISTRIBUTABLE
│   ├── CLAUDE.md          ← user-facing entry point
│   ├── PRINCIPLES.md      ← 20 design principles with sources and tests
│   ├── GETTING-STARTED.md ← 10-step walkthrough
│   ├── HINTS.md           ← hint system design template
│   ├── profiles/          ← 30 reviewer personas
│   ├── solvers/           ← 12 solver archetypes for blind testing
│   ├── skills/hunt/       ← 14 hunt-level skills
│   ├── skills/puzzle/     ← 3 puzzle-level skills
│   └── templates/         ← puzzle page + 20 puzzle type guides
│
├── scenarios/             ← real runs of the pipeline (dev + validation)
│   ├── age-of-empires/    ← active test scenario
│   ├── boardgames/        cosmere/       games-magazine/
│   ├── grand-larceny/     ironfall/      space-game/  space-game2/
│   └── wavelength/
│
├── examples/              ← completed scenarios shipped with the toolkit
├── evidence/              ← research that fed the principles (MIT MH, MS Hunt)
├── reference/             ← external puzzle reference library
└── _vocab/                ← JSON schemas for pipeline data types
```

---

## Answer security

**Plaintext answers must never appear in git-tracked files** — they're searchable in
history forever. During `/hunt plan` Stage 1 the pipeline asks you to pick an encoding
(ROT13, Base64, a custom cipher, or no-storage). Encoded answers live in `.claude/`
project memory (gitignored), never in the repo.

This was bug #7 in `BUGS.md`. It stays blocking for every scenario.

---

## Status

- **Pipeline:** 11 stages, all covered end-to-end by working skills.
- **Scenarios shipped:** 9 in `scenarios/`, one (DEAD-RECKONING / Age of Empires)
  completed Stage 11 and is in `examples/`.
- **Outstanding bugs:** tracked in `BUGS.md` — fixable issues become toolkit
  improvements, which are then backported.
- **Backport loop:** every toolkit change is mirrored to
  `reference\puzzle-hunt\` and `.claude/skills/` (see `CLAUDE.md` for the
  checklist).

---

## Contributing

If you run the pipeline on your own hunt, two things help:

1. File anything the pipeline couldn't handle in `BUGS.md` with the scenario name and
   stage where it surfaced.
2. If you build a new puzzle type, add it to `toolkit/templates/TYPES.md` with a brief
   and an authoring note.

---

## Research

Seven papers document the pipeline's creative evaluation methodology. LaTeX sources in [`research/publications/`](research/publications/); build all PDFs with `make -C research`.

- [An AI-Assisted Pipeline for Puzzle Hunt Creation](research/publications/games-generative-hunt-pipeline/main.pdf)
- [Expert AI Panels for Creative Work Evaluation](research/publications/games-ai-expert-panel-creative/main.pdf)
- [Calibrating AI Expert Panel Scores Against Human Expert Judgment](research/publications/games-human-ai-calibration/main.pdf)
- [Orientation Before Expertise: A Unified Tier Model of Creative Quality](research/publications/games-creative-quality-tiers/main.pdf)
- [What Does a Persona Do? Cognitive Trait Decomposition](research/publications/games-persona-authoring-traits/main.pdf)
- [Profile Depth and the Reviewing-to-Authoring Transfer Problem](research/publications/games-profile-taxonomy-creative/main.pdf)
- [Structured Divergence in AI Creative Generation](research/publications/games-same-input-divergence/main.pdf)

---

## License

MIT for the toolkit, skills, and pipeline code. Reviewer profiles synthesize publicly
documented design philosophies of named designers — cited in each profile — and do
not represent those designers' own words or endorsement. Respect the source material.
