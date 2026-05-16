# Puzzle Hunt Toolkit — Development

This is the **development** CLAUDE.md. For the **user-facing** toolkit instructions, see `toolkit/CLAUDE.md`.

## What This Repo Is

A reusable framework for building puzzle hunts with Claude Code. Two audiences:
- **Users** (Kenny, Dana, anyone we send this to): they use `toolkit/` to build their own hunts
- **Developers** (us): we develop the toolkit, test it with scenarios, fix bugs, backport to the reference library

## Development Workflow

1. **Develop** toolkit features in `toolkit/`
2. **Test** by running scenarios in `scenarios/`
3. **Track bugs** in `BUGS.md`
4. **Backport** improvements to `reference\puzzle-hunt\` and `.claude/skills/`

## Active Test Scenario

**Age of Empires** (`scenarios/age-of-empires/`) — a 5-puzzle + 1-meta hunt to validate the full 8-stage pipeline. Every stage tests the toolkit skills, templates, and principles.

## Structure

```
puzzlehunt/
├── CLAUDE.md              ← THIS FILE (development)
├── BUGS.md                ← issue tracker
│
├── toolkit/               ← THE DISTRIBUTABLE (what users get)
│   ├── CLAUDE.md          ← user-facing entry point
│   ├── PRINCIPLES.md
│   ├── GETTING-STARTED.md
│   ├── HINTS.md
│   ├── profiles/          ← 12 expert personas
│   ├── skills/            ← 5 Claude Code skills
│   └── templates/         ← puzzle page + 20 type templates
│
├── scenarios/             ← test scenarios (dev use)
│   └── age-of-empires/    ← active test
│
└── examples/              ← completed scenarios (shipped with toolkit)
```

## Backport Checklist

When a toolkit improvement is ready:
- [ ] Update `toolkit/` files
- [ ] Update corresponding files in `reference\.claude\skills\`
- [ ] Update `reference\puzzle-hunt\PRINCIPLES.md` if principles changed
- [ ] Commit in both repos


---

## proof — documentation linting and guide compilation

proof is the markdown QA and compilation tool for this repo. Binary lives at
`C:/src/target/debug/proof` (workspace build — run `cd C:/src && cargo build` once).

```bash
# Lint all markdown
C:/src/target/debug/proof check .

# Compile guides: src/guides/ → docs/guides/
bash scripts/build-guides.sh

# Watch mode — recompiles on every save
C:/src/target/debug/proof compile --watch

# Check without writing
bash scripts/build-guides.sh --check
```

Source guides go in `src/guides/*.source.md`. Compiled output lands in `docs/guides/`.
Directives: `proof:tree`, `proof:element`, `proof:math`, `proof:bullets`, `proof:callout`.
See `C:/src/proof/docs/guides/07-compile.md` for the full directive reference.
