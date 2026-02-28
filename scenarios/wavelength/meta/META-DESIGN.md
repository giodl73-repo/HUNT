# WAVELENGTH — Meta Design

## The Final Broadcast

### Concept

The meta IS the DJ's final playlist card — the physical prop teams receive at the start. It lists all songs played during Vee's final broadcast (the 6 puzzle sets). At the bottom, there are 6 numbered blanks with small position indicators. Once solvers have the 6 feeder answer words, they write them in order and read the diagonal to discover Vee's hidden message.

### Mechanism: Diagonal Extraction

Write the 6 answer words vertically aligned (left-justified). Take the Nth letter from the Nth word:

```
Position:  1 2 3 4 5 6
           ─ ─ ─ ─ ─ ─
P1 word:  [1]_ _ _ _ _
P2 word:   _[2]_ _ _ _
P3 word:   _ _[3]_ _ _ _ _
P4 word:   _ _ _[4]_ _
P5 word:   _ _ _ _[5]
P6 word:   _ _ _ _ _[6]
```

Reading the bracketed positions: the 6 extracted letters spell the meta answer.

### Verification (ROT13)

P1 = SNGUBZ → position 1 = (ROT13: S → decoded first letter)
P2 = SNPNQR → position 2 = (ROT13: N → decoded second letter)
P3 = PNQRAPR → position 3 = (ROT13: Q → decoded third letter)
P4 = FUEVAR → position 4 = (ROT13: V → decoded fourth letter)
P5 = UNIRA → position 5 = (ROT13: A → decoded fifth letter)
P6 = NANYBT → position 6 = (ROT13: T → decoded sixth letter)

Meta answer (ROT13): SNQVAT

### The 80% Rule (Principle #11)

With 5 of 6 letters of the meta answer, the solver has a common English word with one blank. Given the radio station context (a station going dark, a final broadcast, a DJ signing off), the missing letter is highly inferrable. The word is common enough that any 5-letter subset strongly suggests the complete word.

**Test:** Remove any one letter from the decoded meta word. Is the remaining pattern + context sufficient?
- _ADING → only one common English word fits
- F_DING → only one common English word fits
- FA_ING → several possibilities (fading, faking, faring, fating) — context of a radio station going dark disambiguates to one
- FAD_NG → only one common English word fits
- FADI_G → only one common English word fits
- FADIN_ → only one common English word fits

Result: 5/6 unambiguous, 1/6 requires context. Passes the 80% rule.

### Presentation on the Physical Prop

The DJ's final playlist card shows:

```
┌─────────────────────────────────────┐
│  WVLG 92.7 FM — THE LONG WAVE      │
│  Final Broadcast — Vee              │
│                                     │
│  SET 1: [songs from P1]            │
│  SET 2: [songs from P2]            │
│  SET 3: [songs from P3]            │
│  SET 4: [songs from P4]            │
│  SET 5: [songs from P5]            │
│  SET 6: [songs from P6]            │
│                                     │
│  ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─ ─   │
│                                     │
│  The answer was always in the       │
│  music. Write what you found:       │
│                                     │
│  Set 1: _ _ _ _ _ _                │
│         ^                           │
│  Set 2: _ _ _ _ _ _                │
│           ^                         │
│  Set 3: _ _ _ _ _ _ _              │
│             ^                       │
│  Set 4: _ _ _ _ _ _                │
│               ^                     │
│  Set 5: _ _ _ _ _                  │
│                 ^                   │
│  Set 6: _ _ _ _ _ _                │
│                   ^                 │
│                                     │
│  Now read down: _ _ _ _ _ _        │
│                                     │
│  "The signal was always there.      │
│   You just had to listen."          │
│                                     │
│                           — Vee     │
└─────────────────────────────────────┘
```

The carets (^) under each answer word mark which letter to extract. Reading those letters down gives the meta answer. The closing quote is Vee's final transmission.

### Meta Elegance

- The mechanism is visual and intuitive — a diagonal staircase through the answer words
- The physical prop IS the meta — you solve it by writing on it
- The meta answer is thematically resonant (a radio station's signal ending)
- The closing quote gives emotional closure
- No computation, just assembly + pattern recognition

### Scale Observation

For 6 feeders, a diagonal extraction is simple but effective. More complex meta mechanisms (crossword grids, overlays) would be overkill at this scale. The simplicity IS the point — the emotional payoff carries the meta, not mechanical complexity.
