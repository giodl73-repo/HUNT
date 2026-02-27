# /puzzle-plan — Design a Puzzle Hunt

Walk through the complete design process for a puzzle hunt, from blank page to construction-ready plan. Each stage has a review gate — the expert panel evaluates before you proceed.

## Usage

```
/puzzle-plan                    — start from scratch (guided walkthrough)
/puzzle-plan library <path>     — set the content library path
/puzzle-plan stage              — show current stage and what's next
/puzzle-plan stage <N>          — jump to a specific stage
```

---

## The 8 Stages

Each stage produces a deliverable. Each gate requires a panel review before proceeding. You can loop back to any earlier stage if the review surfaces issues.

```
Stage 1: SCOPE          → what's the hunt about?
    ↓ review gate
Stage 2: STRUCTURE      → how many rounds, puzzles, metas?
    ↓ review gate
Stage 3: PUZZLE POOL    → brainstorm all candidate puzzles
    ↓ review gate (panel ranks the pool)
Stage 4: ASSIGNMENT     → assign puzzles to slots
    ↓ review gate
Stage 5: META DESIGN    → design meta puzzles for each round
    ↓ review gate
Stage 6: AUTHORING      → write every puzzle
    ↓ testing gate (blind test each puzzle)
Stage 7: INTEGRATION    → assemble into books/rounds, verify crossings
    ↓ review gate
Stage 8: POLISH         → final pass, answer verification, difficulty curve
    ↓ ship it
```

---

## Stage 1: SCOPE

**Goal:** Define the hunt's identity.

**Questions to answer:**
- What content library does this hunt use? (path, structure, size)
- Who is the audience? (solo puzzler? team of 12? general public?)
- What's the theme/framing? (deck of cards? periodic table? mythology? custom?)
- What's the tone? (friendly guide? cryptic challenge? competitive?)
- What's the scale? (10 puzzles? 50? 100?)
- Is there a narrative? (narrator character? story arc?)
- Physical format? (book? website? hybrid?)

**Deliverable:** `SCOPE.md` — one page capturing all decisions.

**Review gate:** `/puzzle-review full SCOPE.md` — panel evaluates scope for feasibility, audience fit, and narrative coherence.

---

## Stage 2: STRUCTURE

**Goal:** Define rounds, puzzle counts, and meta architecture.

**Flexible structures the toolkit supports:**

### Single-round (simplest)
```
N puzzles → 1 meta → final answer
```

### Multi-round (MIT Mystery Hunt style)
```
Round 1: N puzzles → meta 1
Round 2: N puzzles → meta 2
Round 3: N puzzles → meta 3
...
All round metas → super-meta → final answer
```

### Two-book (Red/Black Joker style)
```
Book 1: N individual puzzles → meta 1
Book 2: N synthesis puzzles → meta 2
Both metas → combined final answer
```

### Progressive unlock
```
Solve puzzles to unlock new rounds
Meta answers from early rounds are inputs to later rounds
```

**Questions to answer:**
- How many rounds?
- How many puzzles per round?
- Does each round have a meta?
- Is there a super-meta / meta-meta?
- How do rounds relate to each other? (independent? sequential? interlocking?)
- What's the difficulty curve across rounds?
- What's the numbering system? (sequential? thematic? encoded?)

**Deliverable:** `STRUCTURE.md` — round map, puzzle counts, meta architecture, numbering.

**Review gate:** `/puzzle-review full STRUCTURE.md` — panel evaluates structure for pacing (Katz), meta robustness (Huang), narrative arc (Selinker), buildability (Kenny).

---

## Stage 3: PUZZLE POOL

**Goal:** Generate more puzzle ideas than you need. The panel will rank them.

**Process:**
1. For each round/section, brainstorm 3-5 candidate puzzles
2. For each candidate: name, one-line pitch, mechanism, section reference, estimated difficulty
3. Include physical/visual puzzles, cross-section puzzles, identification puzzles, construction puzzles — variety is key
4. Don't self-censor — bad ideas spark good ones

**Deliverable:** `PUZZLE-POOL.md` — all candidates with one-line briefs.

**Review gate:** `/puzzle-review rank PUZZLE-POOL.md` — each reviewer picks their ideal lineup per round. Consensus emerges. See which puzzles land on 8+ of 12 lists (locks) vs. 4-7 (contenders) vs. 0-3 (cut).

---

## Stage 4: ASSIGNMENT

**Goal:** Assign specific puzzles to specific slots based on pool rankings.

**Process:**
1. Take the consensus locks from Stage 3
2. Fill remaining slots from contenders
3. Verify: mechanism variety (no two puzzles use the same type), difficulty curve (ramps per round), visual variety (grids, diagrams, text, physical)
4. Assign each puzzle: answer word, section reference, puzzle type, round placement

**Deliverable:** `ASSIGNMENT.md` — the complete puzzle-to-slot mapping.

**Review gate:** `/puzzle-review full ASSIGNMENT.md` — panel evaluates the full assignment for systemic coherence (Gottlieb), variety (Snyder), buildability (Kenny), experience arc (Selinker).

---

## Stage 5: META DESIGN

**Goal:** Design the meta puzzle(s) that combine feeder answers.

**Meta types supported:**
- **Crossword:** feeder answers fill a grid, highlighted squares spell the meta answer
- **Acrostic:** first letters of feeder answers spell the meta answer
- **Pattern recognition:** feeder answers share a hidden property
- **Jigsaw:** feeder answers are pieces that assemble into something
- **Elimination:** multiple candidate answers, evidence narrows to one (Clue-style)
- **Physical:** assemble a 3D object, overlay pages, punch card
- **Custom:** design your own

**For multi-round hunts:** design each round's meta AND the super-meta. The super-meta should use round-meta answers as inputs.

**Deliverable:** `meta/META-DESIGN.md` — mechanism, answer words, extraction method, verification that the meta is solvable with 80% of feeders (Katz's rule).

**Review gate:** `/puzzle-review full meta/META-DESIGN.md` — panel evaluates meta solvability (Katz 80% rule), deductive rigor (Huang), narrative payoff (Selinker), backsolving potential (Katz).

---

## Stage 6: AUTHORING

**Goal:** Write every puzzle.

**Process:**
For each puzzle in the assignment:
1. `/puzzle-author <puzzle-id>` — writes the full puzzle page
2. Verify against PRINCIPLES.md (18 design principles)
3. `/puzzle-test <puzzle-id>` — 3 blind testers score it
4. If PASS (≥22/30, all dimensions ≥4) → done
5. If REVISE → fix issues, retest (max 3 iterations)
6. If REDESIGN → go back to pool, pick alternate

**Deliverable:** Complete puzzle files in `puzzles/` with test results in `tests/`.

**Testing gate:** Every puzzle must pass blind testing before proceeding. No exceptions. The principles are the bar.

---

## Stage 7: INTEGRATION

**Goal:** Assemble puzzles into the final hunt format.

**Checklist:**
- [ ] All puzzles pass testing
- [ ] Meta crosswords/grids are constructable with the actual answer words
- [ ] Difficulty curve verified across each round
- [ ] Answer words verified (no duplicates, no unintended overlaps)
- [ ] Physical puzzles verified (templates print correctly)
- [ ] Narrative elements written (intro, per-puzzle flavor, closing)
- [ ] Numbering verified (no collisions, gaps are intentional)
- [ ] Cross-references verified (if puzzles interlock across rounds)

**Deliverable:** Assembled hunt in `puzzles/` with navigation/ordering.

**Review gate:** `/puzzle-review full` on the complete assembled hunt — full panel pass, Gottlieb's system integrity check, Dana's visual/physical assessment.

---

## Stage 8: POLISH

**Goal:** Final quality pass.

- [ ] Answer verification: every puzzle's extraction produces the correct answer
- [ ] Difficulty curve: play-test the full hunt in order
- [ ] Hint system: do you provide hints? If so, write them
- [ ] Answer confirmation: how does a solver know they got the right answer?
- [ ] Answer encoding: encode the master answer key (see CLAUDE.md § Answer Security)
- [ ] Print/publish prep: formatting, page breaks, templates

**Deliverable:** Ship-ready hunt.

---

## Looping Back

Any review gate can send you back to an earlier stage:

- Pool review reveals structural problems → back to Stage 2
- Assignment review reveals meta incompatibility → back to Stage 5
- Puzzle testing reveals systemic issues → back to Stage 4
- Integration reveals missing pieces → back to Stage 6

This is normal. The pipeline is designed for iteration.

---

## Hunt Structures — Examples

### Small hunt (game night)
- 1 round, 8 puzzles, 1 meta
- Solo or team of 4
- 2-3 hours

### Medium hunt (weekend project)
- 3 rounds × 8 puzzles = 24 puzzles + 3 metas + 1 super-meta
- Team of 4-8
- Full weekend

### Large hunt (MIT/Microsoft scale)
- 6 rounds × 12 puzzles = 72 puzzles + 6 metas + 1 super-meta
- Team of 12-200
- 24-72 hours

### Two-book hunt (Red/Black Joker style)
- Book 1: 26 guided puzzles + meta
- Book 2: 26 synthesis puzzles + meta + physical builds
- Solo or team
- Multi-week

The toolkit scales to any of these. The principles and pipeline are the same regardless of size.
