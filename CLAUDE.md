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
