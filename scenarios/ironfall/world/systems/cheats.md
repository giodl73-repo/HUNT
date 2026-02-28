# IRONFALL Cheat Codes & Button Mapping

Source of truth for the True Ending cheat code and how it maps to game data. This file is the key to the super-meta.

---

## SNES Controller Buttons

The SNES controller has 12 input buttons:

```
        L                           R
  ┌─────────────────────────────────────┐
  │                                     │
  │    ↑                     X          │
  │  ←   →               Y     A       │
  │    ↓                     B          │
  │                                     │
  │         SELECT  START               │
  └─────────────────────────────────────┘
```

12 possible inputs: UP, DOWN, LEFT, RIGHT, A, B, X, Y, L, R, START, SELECT

---

## The True Ending Cheat Code

The code is 12 button presses entered on the title screen after completing the game and defeating the secret boss.

### Button Mapping (from feeder puzzle answers)

Each feeder puzzle answer word maps to one button press. The mapping is:

**The answer words themselves ARE the button names or map to buttons through extraction.**

The super-meta design encodes the cheat code as follows:
- 10 feeder answer words each contribute one letter to a grid
- 2 round meta answers contribute additional letters
- The 12 extracted letters (in round order) spell out abbreviated button names

### Button Abbreviation Key

| Button | Abbreviation | Notes |
|--------|-------------|-------|
| UP | U | D-pad |
| DOWN | D | D-pad |
| LEFT | L | D-pad (NOT shoulder L) |
| RIGHT | R | D-pad (NOT shoulder R) |
| A | A | Face button |
| B | B | Face button |
| X | X | Face button |
| Y | Y | Face button |
| L (shoulder) | SL | Shoulder left |
| R (shoulder) | SR | Shoulder right |
| START | ST | Center button |
| SELECT | SE | Center button |

For the cheat code, the shoulder buttons use two-letter abbreviations. D-pad and face buttons use single letters.

---

## Code Verification

When the correct 12-button sequence is entered on the title screen:
1. The screen flashes white
2. Morimoto's ghost appears
3. The Iron Citadel transforms into a garden
4. Text appears: "Thank you for playing. — K.M."
5. The save file's True Ending flag (offset 0x66) flips to 0x01
6. Mark 16 (Transcendent) is awarded

Wrong sequences produce no response — no error, no feedback. The player simply re-enters from the title screen.

---

## Design Notes for Meta

The cheat code is the super-meta answer. It emerges from the 10 feeder answers + 2 round meta answers as follows:

- Act I feeders (5 words) + Act I meta (1 word) = 6 inputs (first half of the code)
- Act II feeders (5 words) + Act II meta (1 word) = 6 inputs (second half of the code)

The ordering is: Act I puzzles 1-5 in order, then Act I meta, then Act II puzzles 1-5 in order, then Act II meta.

Each answer word contributes one letter. That letter maps to a button via the abbreviation key above.

---

## In-Universe Context

The cheat code was Kenji Morimoto's final act as lead developer before Ashfield Interactive closed. He embedded it in the game's data — not in any dialogue, not in any obvious place. The code is distributed across the game's systems: one piece in the bestiary, one in the items, one in the achievements, and so on. The fan community has spent 30 years trying to find it. The puzzle hunt is the moment it comes together.
