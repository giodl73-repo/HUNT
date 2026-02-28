# THE GRAND LARCENY — Meta Design

## The Question

How was the painting stolen?

The four feeder puzzles answer WHO, WHERE, WHEN, and WHAT. The meta combines these to answer HOW.

## Feeder Answers

| Puzzle | Dimension | Answer (ROT13) | Length |
|--------|-----------|----------------|--------|
| P1 | Who (the suspect) | FHFCRPG | 7 |
| P2 | Where (the location) | YBPNGVBA | 8 |
| P3 | When (the time) | GVZR | 4 |
| P4 | What (the object) | BOWRPG | 6 |

## Meta Answer

ROT13: ZRGUBQ (6 letters)

## Mechanism

The meta is presented as the "Evidence Log" — the final page of the dossier. It contains a structured case summary with four blank fields:

```
CASE SUMMARY — CONFIDENTIAL

The painting "Lady in Vermillion" was removed from the Sargent Gallery
by means yet to be determined.

INVESTIGATING OFFICER'S FINDINGS:

    The primary _______ is [P1 answer, 7 letters]
    The critical ________ was [P2 answer, 8 letters]
    The decisive ____ was [P3 answer, 4 letters]
    The key ______ was [P4 answer, 6 letters]

When these findings are combined, the METHOD becomes clear.

    THE METHOD: _ _ _ _ _ _
```

The solver fills in the four answer words. The meta extraction works as follows:

### Extraction

Each answer word contributes one letter to the meta answer. The evidence log has a grid:

```
EVIDENCE CORRELATION MATRIX

         Letter position:  1  2  3  4  5  6  7  8
P1 (7):  S  U  S  P  E  C  T
P2 (8):  L  O  C  A  T  I  O  N
P3 (4):  T  I  M  E
P4 (6):  O  B  J  E  C  T
```

The calling card prop (the small card left at the crime scene) has a symbol on it: a diagonal arrow going from top-left to bottom-right. This is the extraction instruction.

Reading the diagonal of the matrix (position 1 from P1, position 2 from P2, position 3 from P3, position 4 from P4):
- P1 position 1: S
- P2 position 2: O
- P3 position 3: M...

Wait. That gives S-O-M-E, not the target.

### Revised extraction

The calling card has four numbers: 3, 5, 2, 4. These are the extraction positions.

- P1 position 3: S
- P2 position 5: T
- P3 position 2: I
- P4 position 4: E

That gives S-T-I-E. Not right either.

### Final extraction design

The evidence log contains a "CASE CORRELATION" section with four clues. Each clue points to a specific letter in the corresponding answer word:

```
CASE CORRELATION

The third finding in the suspect file:         _  (letter 3 of P1)
The sixth entry in the location record:         _  (letter 6 of P2)
The fourth mark on the timeline:                _  (letter 4 of P3)
The second item in the object catalog:          _  (letter 2 of P4)

These four letters, with the calling card, reveal THE METHOD.
```

P1 letter 3: S → ... Hmm, letter 3 of SUSPECT is S.
P2 letter 6: I → letter 6 of LOCATION is I.
P3 letter 4: E → letter 4 of TIME is E.
P4 letter 2: B → letter 2 of OBJECT is B.

That gives S-I-E-B. Not useful.

### Working backwards from METHOD

The answer is METHOD (ROT13: ZRGUBQ). I need extraction positions from the four feeder words that produce M-E-T-H-O-D.

```
SUSPECT:   S U S P E C T     → need position that gives a letter in METHOD
LOCATION:  L O C A T I O N   → need position that gives a letter in METHOD
TIME:      T I M E           → need position that gives a letter in METHOD
OBJECT:    O B J E C T       → need position that gives a letter in METHOD
```

Six letters needed (METHOD), four words available. So each word must contribute 1-2 letters. Let me find positions:

M: SUSPECT has no M. LOCATION has no M. TIME position 3 = M. OBJECT has no M.
E: SUSPECT position 5 = E. LOCATION has no E. TIME position 4 = E. OBJECT position 4 = E.
T: SUSPECT position 7 = T. LOCATION position 5 = T. TIME position 1 = T. OBJECT position 6 = T.
H: None of the words contain H.

Problem: METHOD contains H, and none of the four feeder words contain H.

### Revised meta answer

The meta answer should be extractable from the four feeder words. Let me pick a 6-letter answer where every letter appears in at least one feeder word.

Available letters across all four words:
SUSPECT: S, U, P, E, C, T
LOCATION: L, O, C, A, T, I, N
TIME: T, I, M, E
OBJECT: O, B, J, E, C, T

Combined alphabet: A, B, C, E, I, J, L, M, N, O, P, S, T, U

Missing: D, F, G, H, K, Q, R, V, W, X, Y, Z

So METHOD (with H and D) cannot be extracted. Let me choose a better meta answer.

Candidates using only available letters:
- STOLEN (S, T, O, L, E, N) -- all available. S from SUSPECT, T from TIME, O from LOCATION, L from LOCATION, E from OBJECT, N from LOCATION
- COBALT -- no
- MOTIVE (M, O, T, I, V, E) -- V not available
- ESCAPE (E, S, C, A, P, E) -- all available

STOLEN is perfect. The method is that the painting was STOLEN (cut from its frame and taken). Let me verify the extraction:

```
SUSPECT:   S(1) U(2) S(3) P(4) E(5) C(6) T(7)
LOCATION:  L(1) O(2) C(3) A(4) T(5) I(6) O(7) N(8)
TIME:      T(1) I(2) M(3) E(4)
OBJECT:    O(1) B(2) J(3) E(4) C(5) T(6)
```

STOLEN = S, T, O, L, E, N

S → SUSPECT position 1 (or 3)
T → SUSPECT position 7 (or TIME position 1, or LOCATION position 5, or OBJECT position 6)
O → LOCATION position 2 (or 7) (or OBJECT position 1)
L → LOCATION position 1
E → SUSPECT position 5 (or TIME position 4, or OBJECT position 4)
N → LOCATION position 8

One clean extraction: take one letter from each word, plus two from LOCATION:

P1 (SUSPECT), position 1 → S
P2 (LOCATION), position 5 → T
P3 (TIME), position ... none of TIME gives O, L, E, or N directly in a clean way.

Let me try a different approach. The calling card has a pattern — it tells you which position to extract from each word:

Calling card shows: 1, 5, 3, 4, 1, 8

Reading positions: SUSPECT[1]=S, LOCATION[5]=T, TIME[3]=O... wait, TIME[3]=M, not O.

Let me reconsider. The issue is that 4 words of different lengths make a clean 6-letter extraction awkward.

### Simpler meta: 4-letter answer

If each feeder contributes exactly one letter:

Available 4-letter words:
- PLOT: P from SUSPECT, L from LOCATION, O from LOCATION... two from same word.
- COIL: C from SUSPECT, O from LOCATION, I from TIME, L from LOCATION... two from LOCATION.
- LOOT: L from LOCATION, O from LOCATION/OBJECT, O again, T from SUSPECT/TIME/OBJECT... repeats.
- BOLT: B from OBJECT, O from LOCATION, L from LOCATION, T... two from LOCATION.
- TILE: T from SUSPECT/TIME, I from TIME/LOCATION, L from LOCATION, E from multiple. T from SUSPECT, I from TIME, L from LOCATION, E from OBJECT. That works -- one per word.
- SLIP: S from SUSPECT, L from LOCATION, I from TIME, P... P is in SUSPECT only. Two from SUSPECT.
- TOIL: T from SUSPECT, O from LOCATION, I from TIME, L... L is only in LOCATION. Two from same.
- SUIT: S from SUSPECT, U from SUSPECT... no.
- EMIT: E from SUSPECT, M from TIME, I from TIME... two from TIME.
- MICE: M from TIME, I from LOCATION, C from SUSPECT/OBJECT, E from multiple. One each possible: M(TIME), I(LOCATION), C(SUSPECT), E(OBJECT). Yes.
- MELT: M from TIME, E from OBJECT, L from LOCATION, T from SUSPECT. One each. Yes.
- MIST: M from TIME, I from LOCATION, S from SUSPECT, T from OBJECT. One each. Yes.
- SNIP: only 2 words have S...
- SPICE: 5 letters, only 4 words.
- ECLIPSE: 7 letters, mismatch.
- CUT: too short.

MELT is not great thematically. MIST is atmospheric but doesn't answer "how."

Let me try 5 letters with some words contributing 2:
- OPTIC: O(LOCATION), P(SUSPECT), T(TIME), I(LOCATION), C(OBJECT) -- nice but doesn't answer "how"
- SPLIT: S(SUSPECT), P(SUSPECT), L(LOCATION), I(TIME), T(OBJECT) -- two from SUSPECT
- UNLIT: U(SUSPECT), N(LOCATION), L(LOCATION), I(TIME), T(OBJECT) -- two from LOCATION

The thematic answer should describe HOW the painting was stolen.

Looking at the narrative: the painting was CUT from its frame, rolled up, transported via service elevator to the basement, and taken out the alley. Key verbs: CUT, ROLL, STEAL, LIFT.

- BUILT: no
- CLIMB: C from SUSPECT, L from LOCATION, I from TIME, M from TIME, B from OBJECT -- two from TIME
- UNCUT: repeats
- CABLE: C from SUSPECT, A from LOCATION, B from OBJECT, L from LOCATION, E from TIME -- nice, two from LOCATION though

Let me just pick a clean one. HEIST would be perfect but H isn't available.

Actually, let me reconsider the meta mechanism entirely. Instead of extracting from answer words, the meta can work differently.

## Revised Meta Mechanism

The four answer words don't get letters extracted from them. Instead, the four words ARE the inputs to a final deduction. The evidence log presents a scenario:

"Given the SUSPECT, the LOCATION, the TIME, and the OBJECT -- what is THE METHOD?"

The solver fills in the four blanks. Then the evidence log has a final clue:

"The method is hidden in the evidence. Count the letters."

SUSPECT = 7, LOCATION = 8, TIME = 4, OBJECT = 6

7-8-4-6 → convert to letters: G-H-D-F. No.

Or: The lengths (7, 8, 4, 6) index into a sentence on the evidence log page. Word 7, word 8, word 4, word 6 of a specific sentence spell or indicate the method.

### Final meta design: acrostic of the answers

The first letters of the four answer words: S, L, T, O. Rearranged: SLOT, LOST, LOTS, SLOT.

SLOT is interesting — "the painting was slotted out through [something]." Not great.

### Final meta design: the calling card cipher

The calling card (flavor prop) has a 6x4 grid on the back. Each row represents one answer word. The solver writes the four answer words into the grid rows. The grid columns, when read, spell something.

```
S U S P E C T _
L O C A T I O N
T I M E _ _ _ _
O B J E C T _ _
```

Reading columns: S-L-T-O, U-O-I-B, S-C-M-J, P-A-E-E, E-T-_-C, C-I-_-T, T-O-_-_, _-N-_-_

Column 1: S, L, T, O → anagram of SLOT/LOST/LOTS
Column 2: U, O, I, B → nothing obvious
Column 3: S, C, M, J → no
Column 4: P, A, E, E → no

Not clean.

### Final meta: the method IS the narrative

The meta does not extract new letters. Instead, the meta asks: "Given these four findings, reconstruct what happened."

The evidence log presents a fill-in-the-blank narrative:

```
At approximately [TIME], the [SUSPECT] used the [OBJECT] to access the
[LOCATION] and remove the painting.

Now decode the method:

Take letter [TIME length] from SUSPECT  → S-U-S-P → P (4th letter)
Take letter [OBJECT length] from LOCATION → L-O-C-A-T-I → I (6th letter)
Take letter [SUSPECT length] from TIME → T-I-M-E → no, TIME only has 4 letters, SUSPECT=7
```

This is getting circular. Let me commit to a simple, clean design.

## FINAL META DESIGN

The answer is STOLEN (6 letters). Each feeder word contributes one letter via a specific indexed position. The calling card tells the solver which position from each word.

The calling card front has four suit symbols (spade, heart, diamond, club) with numbers:

```
♠ 1    ♥ 5    ♦ 3    ♣ 4
```

These correspond to:
- ♠ (P1 - Police Report): take letter 1 of answer = S
- ♥ (P2 - Hotel Receipt): take letter 5 of answer = T
- ♦ (P3 - Floor Plan): take letter ... wait, TIME position 3 = M, not O.

OK. Let me accept that STOLEN requires creative routing.

The calling card numbers index into the answer words to pull specific letters. I need S-T-O-L-E-N from 4 words.

Each word contributes at least one letter, and LOCATION (8 letters) contributes three since it's the longest:

SUSPECT[1] = S
LOCATION[5] = T
LOCATION[2] = O
LOCATION[1] = L
TIME[4] = E
LOCATION[8] = N

That's 4 from LOCATION, 1 from SUSPECT, 1 from TIME, 0 from OBJECT. Unbalanced and OBJECT contributes nothing.

Let me try:
SUSPECT[1] = S
OBJECT[6] = T
LOCATION[7] = O
LOCATION[1] = L
OBJECT[4] = E
LOCATION[8] = N

Still unbalanced (3 from LOCATION, 2 from OBJECT, 1 from SUSPECT, 0 from TIME).

### Truly final approach: change to a 4-letter meta answer

Each word contributes exactly one letter. The evidence log says:

"The answer lies at the start."

First letter of each answer: S, L, T, O → anagram → SLOT, LOTS, LOST, SLOT

SLOT works narratively: the painting was passed through a SLOT (the connecting door between gallery and linen closet was described as a narrow passage). Or: the thief used the service elevator SLOT.

But the answer should be unambiguous: SLOT.

The calling card has the numbers 3, 1, 2, 4 on it — the order to read the first letters:

First letters in puzzle order: S(P1), L(P2), T(P3), O(P4)
Reordered by calling card (3,1,2,4): T, S, L, O → TSLO... no.

Actually the calling card should give the ORDER:
Position 1 = P3 (T), Position 2 = P1 (S)... this is getting confusing.

Simpler: The evidence log has four labeled blanks that map to the exhibits:

```
EXHIBIT C first letter: T
EXHIBIT A first letter: S
EXHIBIT B first letter: L
EXHIBIT D first letter: O
```

Read in exhibit order A-B-C-D: S-L-T-O → rearrange... still needs an instruction.

### Actually final design

The calling card has four numbers: **3, 1, 4, 2**

These tell the solver in what ORDER to read the first letters of their answers:

| Position on card | Exhibit | First letter |
|-----------------|---------|-------------|
| 1 | Exhibit C (P3 = TIME) | T |
| 2 | Exhibit A (P1 = SUSPECT) | S |
| 3 | Exhibit D (P4 = OBJECT) | O |
| 4 | Exhibit B (P2 = LOCATION) | L |

No wait -- that gives TSOL. Not a word.

Let me just choose which LETTER (not first) to extract:

Card says: take letter ♠3, ♥1, ♦2, ♣6

- P1[3] = S (third letter of SUSPECT)
- P2[1] = L (first letter of LOCATION)
- P3[2] = I (second letter of TIME)
- P4[6] = T (sixth letter of OBJECT)

= SLIT. That's a real word and it's thematically perfect -- the thief SLIT the canvas from the frame.

**META ANSWER: SLIT** (4 letters)

This is the method: the painting was SLIT from its frame with a blade, rolled up, and smuggled out.

Verification:
- SUSPECT letter 3 = S ✓ (S-U-**S**-P-E-C-T)
- LOCATION letter 1 = L ✓ (**L**-O-C-A-T-I-O-N)
- TIME letter 2 = I ✓ (T-**I**-M-E)
- OBJECT letter 6 = T ✓ (O-B-J-E-C-**T**)

S-L-I-T = SLIT ✓

The calling card shows: **3 - 1 - 2 - 6**

These numbers, combined with the exhibit order (A=P1, B=P2, C=P3, D=P4), tell the solver which letter to extract from each answer word.

## The 80% Rule

With 4 feeders, 80% = 3 answers. If a team has 3 of 4 answers:
- Missing P1: have _-L-I-T → ?LIT → SLIT is strongly implied
- Missing P2: have S-_-I-T → S?IT → SLIT, SPIT, SKIT, SMIT → SLIT fits the crime narrative
- Missing P3: have S-L-_-T → SL?T → SLIT, SLOT, SLUT → SLIT fits
- Missing P4: have S-L-I-_ → SLI? → SLIM, SLID, SLIT, SLIP → SLIT fits the crime

In all cases, SLIT is the most thematically justified completion. The 80% rule is satisfied.

## Physical Implementation

1. The evidence log (last page of dossier) has the CASE CORRELATION section with four extraction instructions
2. The calling card (separate prop) has the four numbers (3, 1, 2, 6) and a decorative design
3. The solver fills in the four answer words, applies the calling card numbers, and reads SLIT
4. They submit SLIT as the final answer on the website

## Narrative Payoff

"The canvas was slit from its frame." The solver now knows exactly how the theft happened in that 10-minute window: the thief entered the gallery, slit the canvas from the frame with a blade, rolled it, and escaped through the connecting door to the linen closet and down the service elevator. The physical evidence all clicks into place.
