# Puzzle Hunt Toolkit

Design, author, test, and iterate puzzle hunts using Claude Code. Plug in any content library and build a puzzle hunt around it.

## Structure

```
puzzlehunt/
├── CLAUDE.md              ← you are here
├── toolkit/               ← the reusable framework (don't edit these)
│   ├── PRINCIPLES.md      ← 18 design principles
│   ├── GETTING-STARTED.md ← step-by-step tutorial
│   ├── HINTS.md           ← hint system template
│   ├── profiles/          ← 12 expert reviewer personas
│   ├── skills/            ← 5 Claude Code skills
│   └── templates/         ← puzzle page templates + 20 puzzle types
│
├── scenarios/             ← hunt instances (one directory per hunt)
│   └── [your-hunt]/       ← your active scenario
│       ├── SCOPE.md       ← what the hunt is about
│       ├── ROUNDS.md      ← round structure + metas
│       ├── PUZZLES.md     ← master puzzle registry
│       ├── HINTS.md       ← your hint design
│       ├── meta/          ← meta puzzle design
│       ├── puzzles/       ← authored puzzle pages
│       ├── reviews/       ← panel review output
│       └── tests/         ← blind test results
│
└── examples/              ← completed example scenarios
```

## Skills

Install to `.claude/skills/` from `toolkit/skills/`:

| Skill | Purpose |
|-------|---------|
| `/puzzle-plan` | 8-stage design workflow with review gates |
| `/puzzle-review` | 12-expert panel reviews |
| `/puzzle-author` | Write puzzles from briefs |
| `/puzzle-test` | Blind-test with sanitization + iteration |
| `/puzzle-status` | Pipeline dashboard |

## Workflow

See `toolkit/GETTING-STARTED.md` for the full tutorial:
```
/puzzle-plan → design → /puzzle-review → author → /puzzle-test → iterate → ship
```

## Active Scenario

Set the active scenario by telling Claude which directory to work in:
> "Let's work on the age-of-empires scenario"

All puzzle operations target the active scenario's files.

## Expert Panel

12 reviewer personas in `toolkit/profiles/`. Based on real puzzle hunt and game designers. Each brings a distinct lens: structure, craftsmanship, narrative, rigor, buildability, accessibility, experience, systems, wonder, world-design, epiphany, deduction.

## Design Principles

18 evidence-based principles in `toolkit/PRINCIPLES.md`. Every principle was earned by a test failure or panel finding. The top 5:

1. **The Riven Standard** — the puzzle IS what the field does
2. **Solving = Proving Understanding** — the answer is proof of engagement
3. **Blame the Player** — every clue is fair in retrospect
4. **No Over-Scaffolding** — worksheets provide space, not instructions
5. **No Computation Without Deduction** — pure-computation always fails testing

## Answer Security

Answers encoded via A1Z26 → periodic table element symbols. Never store plaintext answer keys in the repo. See toolkit GETTING-STARTED.md for the encoding reference.
