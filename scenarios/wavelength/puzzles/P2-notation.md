# Set 2 — Notation

*"You don't need to hear it. You just need to read it."*

---

Between songs, Vee sometimes hummed intervals — two notes, one after another. She said it was a habit from her conservatory days. Tonight, she hummed six intervals, one between each pair of songs. She wrote the starting note and the interval name on the playlist card.

Find the target note for each interval. Write the six notes in order. Read them as letters.

---

## The Six Intervals

**1.**
Starting note: **C**
Interval: a perfect fourth up

**2.**
Starting note: **D**
Interval: a perfect fifth up

**3.**
Starting note: **F**
Interval: a perfect fifth up

**4.**
Starting note: **D**
Interval: a perfect fifth up

**5.**
Starting note: **A**
Interval: a perfect fourth up

**6.**
Starting note: **A**
Interval: a perfect fifth up

---

## How Intervals Work

A perfect fourth up means counting 5 semitones (half steps) above the starting note.
A perfect fifth up means counting 7 semitones (half steps) above the starting note.

The chromatic scale for reference:
```
C - C#/Db - D - D#/Eb - E - F - F#/Gb - G - G#/Ab - A - A#/Bb - B - C
```

Each step between adjacent notes = 1 semitone.

---

## Workspace

| # | Start | Interval | Semitones | Count up | Target note |
|---|-------|----------|-----------|----------|-------------|
| 1 | C | Perfect 4th up | 5 | C→C#→D→D#→E→**F** | ___ |
| 2 | D | Perfect 5th up | ___ | | ___ |
| 3 | F | Perfect 5th up | ___ | | ___ |
| 4 | D | Perfect 5th up | ___ | | ___ |
| 5 | A | Perfect 4th up | ___ | | ___ |
| 6 | A | Perfect 5th up | ___ | | ___ |

Wait. Read those target notes as letters.

**Your answer** (6 letters): _ _ _ _ _ _

---

*The music theory reference at WVLG may help you with intervals.*

---

## Author's Working Notes (not solver-facing)

**Author: Mike Selinker**

Extraction verification:
1. C + perfect 4th (5 semitones): C→C#→D→D#→E→F = **F**
2. D + perfect 5th (7 semitones): D→D#→E→F→F#→G→G#→A = **A**
3. F + perfect 5th (7 semitones): F→F#→G→G#→A→A#→B→C = **C**
4. D + perfect 5th (7 semitones): D→D#→E→F→F#→G→G#→A = **A**
5. A + perfect 4th (5 semitones): A→A#→B→C→C#→D = **D**
6. A + perfect 5th (7 semitones): A→A#→B→C→C#→D→D#→E = **E**

Answer: F-A-C-A-D-E = FACADE

Wait — item 5 workspace says "7 semitones" but a perfect 4th is 5 semitones. This is an error in the workspace template. The instructions say "A perfect fourth up means counting 5 semitones." The workspace for item 5 should say 5, not 7. But the workspace is intentionally left for the SOLVER to fill in — the "7" in item 5 is a deliberate part of the workspace showing the wrong default. Actually no, let me not pre-fill the semitone count. The workspace shows "5" only for entry 1 as a worked example. The rest are blank.

CORRECTION TO PUZZLE: The workspace should NOT pre-fill the semitone count for entries 2-6. Entry 1 is the worked example.

Principles check:
- Riven Standard: This IS music theory — computing intervals.
- Solving = Proving Understanding: The solver learns how intervals work.
- Blame the Player: The chromatic scale and interval definitions are given.
- No Over-Scaffolding: The workspace has rows to fill but doesn't solve itself.
- Surprise the Answer: FACADE is unexpected from a music theory puzzle. But FACADE is spelled entirely with musical note letters (A-G) — a beautiful coincidence that rewards the solver.
- One Aha: Notes are letters. After that, it's interval arithmetic.
- No Deliberate Errors: All intervals are correct.
- Deduction: The solver must count semitones, not just look up a table.

Note: Perfect 4th = 5 semitones. Perfect 5th = 7 semitones. These are the only two intervals used, keeping it accessible.
