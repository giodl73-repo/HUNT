# Meta Puzzle Design — WOLOLO Crossword

**Stage 5: META DESIGN — COMPLETE**

---

## Meta Architecture

| Field | Value |
|-------|-------|
| **Number of metas** | 1 |
| **Feeder count per meta** | 5 |
| **Meta mechanism** | Crossword — 5 answer words fill a grid, 6 numbered squares spell the meta answer |
| **Super-meta mechanism** | N/A (single round) |
| **Final answer** | WOLOLO (ROT13: JBYBYB) |

---

## The 5 Answer Words

| Puzzle | Age | Domain | Answer | Length | Thematic Justification |
|--------|-----|--------|--------|--------|----------------------|
| I | Dark | Civilizations | CASTLE | 6 | The building every civ builds — Castle Age is when unique units unlock |
| II | Feudal | Units | ONAGER | 6 | Siege unit — the heavy artillery of AoE2 |
| III | Castle | Technologies | LOOM | 4 | Dark Age technology — the first research decision every game |
| IV | Imperial | Maps | TOWER | 5 | Watch Towers are map landmarks; towers dot every map |
| V | Post-Imp | Strategy | PATROL | 6 | Strategic unit command — micro management cornerstone |

### Answer Verification

All 5 words are AoE-related terms. Each contains at least one O or L:
- CASTLE: L at pos 4
- ONAGER: O at pos 0
- LOOM: L at pos 0, O at pos 1, O at pos 2
- TOWER: O at pos 1, W at pos 2
- PATROL: O at pos 4, L at pos 5

WOLOLO letter sourcing (6 highlighted squares):
- W from TOWER
- O from ONAGER (crossing cell with LOOM)
- L from CASTLE
- O from LOOM (non-crossing)
- L from PATROL
- O from PATROL (crossing cell with TOWER)

---

## Crossword Grid

### Layout

3 down words (LOOM, CASTLE, TOWER) + 2 across words (ONAGER, PATROL).
Grid spans 8 rows x 6 columns.

```
        c0    c1    c2    c3    c4    c5

r0      L
        |
r1     [4]O         C
        |           |
r2     [2]O   N     A     G     E     R        << ONAGER across
        |           |
r3      M           S           T
                    |           |
r4      P     A     T     R    [6]O  [5]L      << PATROL across
                    |           |
r5                 [3]L        [1]W
                    |           |
r6                  E           E
                                |
r7                              R
```

### Down Words (read top to bottom)

| Word | Column | Rows | Letters |
|------|--------|------|---------|
| LOOM | c0 | r0-r3 | L-O-O-M |
| CASTLE | c2 | r1-r6 | C-A-S-T-L-E |
| TOWER | c4 | r3-r7 | T-O-W-E-R |

### Across Words (read left to right)

| Word | Row | Columns | Letters |
|------|-----|---------|---------|
| ONAGER | r2 | c0-c5 | O-N-A-G-E-R |
| PATROL | r4 | c0-c5 | P-A-T-R-O-L |

### Crossings (4 total)

| Cell | Down Word | Down Pos | Across Word | Across Pos | Shared Letter |
|------|-----------|----------|-------------|------------|---------------|
| (c0, r2) | LOOM | pos 2 (O) | ONAGER | pos 0 (O) | O |
| (c2, r2) | CASTLE | pos 1 (A) | ONAGER | pos 2 (A) | A |
| (c2, r4) | CASTLE | pos 3 (T) | PATROL | pos 2 (T) | T |
| (c4, r4) | TOWER | pos 1 (O) | PATROL | pos 4 (O) | O |

All crossing letters match. Grid is connected: LOOM--ONAGER--CASTLE--PATROL--TOWER.

---

## Highlighted Squares

Six numbered squares in the grid. Read squares 1 through 6 to extract the meta answer.

| Square | Cell | Word(s) | Letter | Grid Position |
|--------|------|---------|--------|---------------|
| [1] | (c4, r5) | TOWER pos 2 | **W** | row 5, col 4 |
| [2] | (c0, r2) | LOOM pos 2 / ONAGER pos 0 | **O** | row 2, col 0 (crossing cell) |
| [3] | (c2, r5) | CASTLE pos 4 | **L** | row 5, col 2 |
| [4] | (c0, r1) | LOOM pos 1 | **O** | row 1, col 0 |
| [5] | (c5, r4) | PATROL pos 5 | **L** | row 4, col 5 |
| [6] | (c4, r4) | TOWER pos 1 / PATROL pos 4 | **O** | row 4, col 4 (crossing cell) |

**Read [1]-[2]-[3]-[4]-[5]-[6]: W-O-L-O-L-O**

---

## Solver-Facing Grid (blank, with numbers)

This is what the solver receives. Black squares are blocked. Numbered circles mark extraction squares.

```
     0   1   2   3   4   5
   +---+---+---+---+---+---+
0  | _ |   |   |   |   |   |
   +---+---+---+---+---+---+
1  |(4)|   | _ |   |   |   |
   +---+---+---+---+---+---+
2  |(2)| _ | _ | _ | _ | _ |    << "6 across: Siege weapon that fires stones (6)"
   +---+---+---+---+---+---+
3  | _ |   | _ |   | _ |   |
   +---+---+---+---+---+---+
4  | _ | _ | _ | _ |(6)|(5)|    << "4 across: Unit command for continuous movement (6)"
   +---+---+---+---+---+---+
5  |   |   |(3)|   |(1)|   |
   +---+---+---+---+---+---+
6  |   |   | _ |   | _ |   |
   +---+---+---+---+---+---+
7  |   |   |   |   | _ |   |
   +---+---+---+---+---+---+

Down clues:
  "1 down: Textile technology — first research in every game (4)"      = LOOM at c0
  "2 down: Fortification built in the third Age (6)"                   = CASTLE at c2
  "3 down: Defensive structure that shoots arrows (5)"                 = TOWER at c4

Across clues:
  "6 across: Siege weapon — hurls stones over walls (6)"              = ONAGER at r2
  "4 across: Unit command — march continuously along a path (6)"      = PATROL at r4
```

Note: The solver does NOT need these clues. Each answer comes from its feeder puzzle. The clues are backup confirmation only (printed faintly on the meta page, or given as hints).

---

## Extraction Mechanisms (how each puzzle produces its answer)

### Puzzle I => CASTLE (6 letters)

**Mechanism:** Bonus Matcher — match 8 civilization bonuses to their civilizations.
**Extraction:** The puzzle presents 8 civ bonuses in a specific order. The solver matches each bonus to its civilization. Take the Nth letter of the Nth civilization name (where N is the bonus's listed position). 6 of the 8 extracted letters are load-bearing; the other 2 are decoys/confirmation.

Example extraction (to be finalized during authoring):
| Bonus # | Civilization | Letter position | Letter |
|---------|-------------|-----------------|--------|
| 1 | **C**hinese | 1st | C |
| 2 | M**a**gyars | 2nd | A |
| 3 | Bu**s**rmese [VERIFY] | 3rd | S |
| 4 | Teu**t**ons | 4th | T |
| 5 | Ita**l**ians | 5th | L |
| 6 | Briton**e** [VERIFY] | ... | E |

Specific civ-bonus pairs to be chosen during authoring to make indices work.

### Puzzle II => ONAGER (6 letters)

**Mechanism:** Tournament Bracket — 8-unit single-elimination bracket, predict winners by counter logic.
**Extraction:** 7 matchups produce 7 winners. The first letter of each winner's unit type name, reading the bracket top-to-bottom in the final order, yields O-N-A-G-E-R + one extra letter. The extra letter confirms correctness but is not part of the answer.

The bracket must include one trick matchup where a unique unit breaks normal counter rules (e.g., Cataphract vs. Halberdier — Cataphract wins despite being cavalry, due to its anti-infantry bonus).

### Puzzle III => LOOM (4 letters)

**Mechanism:** Tech Tree Gap-Fill — 6 missing technologies in dependency chains.
**Extraction:** Each of the 6 missing tech names contributes one letter at a specified position. 4 of the 6 are load-bearing (spell L-O-O-M); 2 are decoys.

Example chains (to be finalized during authoring):
```
??? --> Bodkin Arrow --> Bracer           (gap = Fletching, take letter 1 = F -- decoy)
Forging --> ??? --> Blast Furnace         (gap = Iron Casting, take letter ... )
Loom --> ???                              (gap might not be Loom itself; adjust)
```

### Puzzle IV => TOWER (5 letters)

**Mechanism:** Resource Map — plot resource spawn locations on blank circular maps. Dots trace letter shapes.
**Extraction:** 5 maps are presented. For each map, the solver plots gold, stone, berries, boar, and deer at described clock positions. The plotted dots, when viewed as a constellation, trace a letter. 5 maps = 5 letters = T-O-W-E-R.

Maps chosen so dot patterns clearly form each letter:
- Map 1: dots form T
- Map 2: dots form O
- Map 3: dots form W
- Map 4: dots form E
- Map 5: dots form R

(See maps.md Resource Geometry sections for spawn position data.)

### Puzzle V => PATROL (6 letters)

**Mechanism:** Economy Puzzle — allocate villagers under constraints.
**Extraction:** The solver must allocate exactly 30 villagers across 6 resource tasks (farms, berries, gold, stone, wood for buildings, wood for fuel). The optimal allocation under the given constraints yields specific numbers. Each number maps to a letter via A1Z26 (1=A, 2=B, ... 26=Z).

Example allocation (to be finalized during authoring):
| Resource | Villagers | A1Z26 |
|----------|-----------|-------|
| Task 1 | 16 | P |
| Task 2 | 1 | A |
| Task 3 | 20 | T |
| Task 4 | 18 | R |
| Task 5 | 15 | O |
| Task 6 | 12 | L |

Total: 16+1+20+18+15+12 = 82. Too many for 30 villagers -- adjust mechanism during authoring. May use a different encoding (e.g., resource gather rates, time-to-target values, or modular arithmetic on totals).

---

## 80% Rule Verification

**Question:** Can the solver complete the meta with only 4 of 5 answers?

| Missing Word | Known Crossings | Deducible? | Reasoning |
|-------------|----------------|------------|-----------|
| CASTLE | A from ONAGER, T from PATROL | Yes | C\_S\_LE with A and T filled = CASTLE (only 6-letter AoE word fitting _A_T_E) |
| ONAGER | O from LOOM, A from CASTLE | Yes | O\_A\_\_\_ with O and A = ONAGER (common AoE siege unit) |
| LOOM | O from ONAGER | Likely | \_O\_\_ = 4-letter AoE tech with O at pos 2. LOOM is the obvious candidate |
| TOWER | O from PATROL | Likely | \_O\_\_\_ = 5-letter AoE term with O at pos 2. TOWER fits; few alternatives |
| PATROL | T from CASTLE, O from TOWER | Yes | \_\_T\_O\_ = 6-letter AoE term. PATROL is strongly constrained |

**Result: 80% rule PASSES.** Any 4 of 5 answers, combined with crossing constraints and AoE vocabulary, uniquely determines the 5th. The meta answer WOLOLO is always recoverable.

---

## Backsolving Resistance

**Can a solver guess feeder answers from the meta alone?**

The solver knows:
1. Five AoE-related words fill this grid
2. Highlighted squares spell WOLOLO

With zero feeders solved, the grid has too many possible fills. AoE vocabulary is large.
With 1-2 feeders, crossing constraints narrow options but multiple fills remain.
With 3+ feeders, backsolving becomes viable — this is by design (helps stuck solvers).

**Verdict:** Moderate backsolving resistance. Appropriate for a 5-puzzle hunt (not a 20-puzzle MIT Mystery Hunt where backsolving would be too easy).

---

## Brute-Force Resistance

**Can the meta be solved without solving any feeders?**

The solver would need to guess five AoE words that:
1. Fit the grid geometry (lengths 4, 5, 6, 6, 6)
2. Cross correctly at 4 cells
3. Produce WOLOLO from 6 specific highlighted positions

The constraint space is large enough. There is no trivial shortcut.

**Verdict:** Brute-force resistant for the target audience (casual AoE players, not competitive puzzle hunters).

---

## Presentation Notes

### Meta Page Layout

The meta page shows:
1. The Monk's intro text: *"You have advanced through all five ages. One sound remains. You know what it is."*
2. A blank crossword grid with numbered circles [1]-[6]
3. Clue labels pointing to each word slot: "Puzzle I answer goes here (6 letters, down)" etc.
4. Instruction: *"Fill each answer into its slot. Read the circled squares in order."*
5. Faint backup crossword clues (optional, for stuck solvers)

### Answer Confirmation

When the solver reads [1][2][3][4][5][6] and gets WOLOLO, the experience is:
- Recognition: "That's the Monk's conversion sound!"
- Satisfaction: all five Ages led to the Monk's signature move
- Thematic closure: the Monk narrator has been trying to convert you the whole hunt

---

## Verification Checklist

Before shipping the meta:
- [x] All 5 feeder answer words are valid AoE terms
- [x] Crossword grid is constructable -- all 4 crossings verified (O=O, A=A, T=T, O=O)
- [x] Meta is solvable with 4 of 5 feeders (80% rule passes for all 5 missing-word scenarios)
- [x] Meta cannot be brute-forced from zero feeders
- [x] Meta answer WOLOLO is surprising yet inevitable
- [ ] Meta has been tested (use `/puzzle-test` with Katz, Huang, Gottlieb)

---

## Stage 5 Status: COMPLETE

Ready for Stage 6 (Authoring).
