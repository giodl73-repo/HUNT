# Puzzle Hunt Toolkit

Build puzzle hunts with Claude Code. Plug in any content library — an encyclopedia, a textbook, a codebase, a museum collection — and create a puzzle hunt around it.

---

## Quick Start

```bash
# 1. Copy this toolkit directory somewhere
cp -r toolkit/ ~/my-puzzle-hunt/

# 2. Install the skills
cp -r skills/* ~/.claude/skills/

# 3. Create your scenario directory
mkdir -p scenarios/my-hunt/{puzzles,reviews,tests,meta}

# 4. Start Claude Code
cd ~/my-puzzle-hunt
claude

# 5. Begin designing
/puzzle-plan
```

---

## What's in the Toolkit

| File/Directory | Purpose |
|---------------|---------|
| `CLAUDE.md` | This file — Claude reads it automatically |
| `PRINCIPLES.md` | 18 design principles with tests (the quality bar) |
| `GETTING-STARTED.md` | Step-by-step tutorial (10 steps from blank to ship) |
| `HINTS.md` | Hint system design template |
| `profiles/` | 12 expert reviewer personas for blind testing |
| `skills/` | 5 Claude Code skills (plan, review, author, test, status) |
| `templates/` | Puzzle page template + 20 puzzle type guides |

---

## The 5 Skills

After installing to `~/.claude/skills/`:

| Command | What it does |
|---------|-------------|
| `/puzzle-plan` | 8-stage design workflow with expert review gates |
| `/puzzle-review full <file>` | 12-expert panel reviews any design document |
| `/puzzle-review rank <file>` | Panel ranks a puzzle pool and picks the best |
| `/puzzle-author <id>` | Write a complete puzzle from its brief |
| `/puzzle-test <id>` | 3 experts solve the puzzle blind, score 6 dimensions |
| `/puzzle-test iterate <id>` | Fix issues and retest until passing (≥22/30) |
| `/puzzle-status` | Pipeline dashboard — what's authored, tested, passing |

---

## The Pipeline

```
Stage 1: SCOPE        → what's the hunt about?
  ↓ review gate
Stage 2: STRUCTURE    → rounds, puzzle counts, metas
  ↓ review gate
Stage 3: PUZZLE POOL  → brainstorm candidates
  ↓ review gate (panel ranks)
Stage 4: ASSIGNMENT   → assign puzzles to slots
  ↓ review gate
Stage 5: META DESIGN  → how feeder answers combine
  ↓ review gate
Stage 6: AUTHORING    → write every puzzle
  ↓ testing gate (blind test each)
Stage 7: INTEGRATION  → assemble, verify crossings
  ↓ review gate
Stage 8: POLISH       → final pass, hints, answer encoding
  ↓ ship it
```

Each gate is a checkpoint. The expert panel evaluates before you proceed. You can loop back to any earlier stage.

---

## The Expert Panel

12 reviewer personas, each with a documented design philosophy. Claude reads each profile and evaluates from that perspective.

| # | Name | Lens | Origin |
|---|------|------|--------|
| 1 | Dan Katz | Structure & pacing | MIT Mystery Hunt (8 wins) |
| 2 | Thomas Snyder | Puzzle craftsmanship | 3x World Sudoku Champion |
| 3 | Mike Selinker | Narrative & experience | Lone Shark Games, Puzzlecraft |
| 4 | Wei-Hwa Huang | Deductive rigor | 4x World Puzzle Champion |
| 5 | Kenny Young | Buildability | 24-year MS Puzzlehunt veteran |
| 6 | Dana Young | Craft & accessibility | 25-year MS Puzzlehunt veteran |
| 7 | Peter Sarrett | Experience design | "Chicago Fire" puzzle, Puzzlehop |
| 8 | Mark Gottlieb | Systems engineering | MIT thesis on puzzle hunt theory |
| 9 | Alex Rosenthal | Accessibility & wonder | TED-Ed, TED Games |
| 10 | Rand Miller | World-as-puzzle | Myst, Cyan Worlds |
| 11 | Jonathan Blow | Epiphany design | Braid, The Witness |
| 12 | Lucas Pope | Deductive identification | Papers Please, Obra Dinn |

---

## 18 Design Principles (Summary)

The full list with sources and tests is in `PRINCIPLES.md`. The top 5:

1. **The Riven Standard** — the puzzle IS what the field does, not overlaid on it
2. **Solving = Proving Understanding** — the solver knows more after solving
3. **Blame the Player** — every clue is fair in retrospect
4. **No Over-Scaffolding** — worksheets provide space, not instructions
5. **No Computation Without Deduction** — pure-computation puzzles always fail testing

---

## Your Scenario

Create a directory for your hunt:

```
scenarios/
└── my-hunt/
    ├── SCOPE.md       ← Stage 1 (copy from GETTING-STARTED.md prompts)
    ├── ROUNDS.md      ← Stage 2
    ├── PUZZLES.md     ← Stage 4 (master registry — puzzles, testers, status)
    ├── HINTS.md       ← Stage 8
    ├── meta/          ← Stage 5
    ├── puzzles/       ← Stage 6 (authored puzzle pages)
    ├── reviews/       ← panel output
    └── tests/         ← blind test results
```

Tell Claude: "Let's work on the my-hunt scenario." All operations target that directory.

---

## Answer Security

Encode answers using A1Z26 → periodic table element symbols:

```
A=H  B=He  C=Li  D=Be  E=B   F=C   G=N   H=O   I=F   J=Ne
K=Na L=Mg  M=Al  N=Si  O=P   P=S   Q=Cl  R=Ar  S=K   T=Ca
U=Sc V=Ti  W=V   X=Cr  Y=Mn  Z=Fe
```

PUZZLE → S·Sc·Fe·Fe·Mg·B

Store encoded answers in `.claude/` project memory, not in the repo.

---

## Extending the Toolkit

- **Add reviewer personas**: create a new profile in `profiles/` following the existing format
- **Add puzzle types**: add to `templates/TYPES.md`
- **Add principles**: add to `PRINCIPLES.md` with source, test, and priority position
- **Customize the pipeline**: edit the stages in `skills/puzzle-plan/SKILL.md`
