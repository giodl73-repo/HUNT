# Game Night — A Board Game Puzzle Hunt

**Scenario 2** — testing the multi-author module workflow.

## Hunt Summary

| Field | Value |
|-------|-------|
| **Name** | Game Night |
| **Theme** | 5 classic board games, each a puzzle |
| **Scale** | 5 puzzles + 1 meta |
| **Audience** | Board game players (casual to serious) |
| **Authors** | 5 different AI personalities (1 per module) + 1 admin (meta) |
| **Purpose** | Test multi-author workflow, module handoff, personality conflicts |

## Module Structure

Each puzzle is a module with its own author and working directory:

```
modules/
├── M1-chess/          ← Author: The Professor (meticulous, slow, over-documents)
│   ├── brief.md       ← puzzle brief (from admin)
│   ├── puzzle.md      ← authored puzzle page
│   ├── notes.md       ← author's working notes
│   └── tests/         ← test results for this puzzle
│
├── M2-settlers/       ← Author: The Speedrunner (fast, sloppy, needs revision)
│   └── ...
│
├── M3-risk/           ← Author: The Rebel (questions the principles, argues)
│   └── ...
│
├── M4-pandemic/       ← Author: The Collaborator (asks for help, shares ideas)
│   └── ...
│
├── M5-codenames/      ← Author: The Silent One (delivers quietly, minimal notes)
│   └── ...
│
└── M6-meta/           ← Author: Admin (designs the meta after all 5 are done)
    ├── brief.md
    ├── meta.md
    └── tests/
```

## The 5 Author Personalities

| Module | Author | Personality | Challenge they create |
|--------|--------|-------------|---------------------|
| M1 | The Professor | Meticulous. Over-documents. Slow. Writes 3x more than needed. | Needs editing down. Quality high but verbose. |
| M2 | The Speedrunner | Fast. Ships v1 quickly. Sloppy on details. | Fails first test. Needs revision. Resists feedback. |
| M3 | The Rebel | Questions the principles. "Why can't I do it my way?" | Argues with the panel. Sometimes right. |
| M4 | The Collaborator | Asks The Professor for help. Shares ideas with everyone. | Interlock between puzzles improves but creates dependencies. |
| M5 | The Silent One | Delivers on time. Minimal communication. No notes. | Admin has no visibility until delivery. Puzzle is either great or needs rework. |

## Files

| File | Stage | Status |
|------|-------|--------|
| `CLAUDE.md` | — | ✅ This file |
| `SCOPE.md` | Stage 1 | — |
| `world/` | Stage 2 | — |
| `ROUNDS.md` | Stage 3 | — |
| `PUZZLES.md` | Stage 5 | — |
| `ANSWERS.md` | Stage 5 | — |
| `modules/M1-M6` | Stage 7 | Created |
| `meta/` | Stage 6 | — |

## What This Scenario Tests

1. **Module isolation** — can 5 authors work independently?
2. **Admin control** — can the admin manage 5 personalities?
3. **Handoff quality** — do briefs contain enough detail?
4. **Conflict resolution** — what happens when The Rebel disagrees?
5. **Meta dependency** — meta can't start until all 5 answers are locked
6. **Integration** — do 5 independently authored puzzles feel like one hunt?
