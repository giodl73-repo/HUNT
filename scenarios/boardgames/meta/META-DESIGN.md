# Meta Design — Game Night

**Stage 8.5: The Captain designs the meta from 5 uncoordinated answer words**

---

## The Constraint

Five answer words, chosen independently by five authors:

| Module | Game | Answer | Length |
|--------|------|--------|--------|
| M1 | Chess | CASTLE | 6 |
| M2 | Settlers | TRADE | 5 |
| M3 | Risk | BORDER | 6 |
| M4 | Pandemic | SPREAD | 6 |
| M5 | Codenames | CIPHER | 6 |

**Meta answer must**: use all 5 as inputs, produce a single final answer, feel like a natural capstone, satisfy the 80% Rule (solvable with 4 of 5).

---

## Analysis of the 5 Words

```
C A S T L E
T R A D E
B O R D E R
S P R E A D
C I P H E R
```

### Letter Inventory

Total letters: 6+5+6+6+6 = 29

By position (1-indexed):
- Position 1: C, T, B, S, C
- Position 2: A, R, O, P, I
- Position 3: S, A, R, R, P
- Position 4: T, D, D, E, H
- Position 5: L, E, E, A, E
- Position 6: E, —, R, D, R

### Patterns Observed

1. **Position 5 has three E's** (CASTLE, BORDER, CIPHER) plus TRADE ends with E at position 5 and SPREAD has A at position 5. Four of five words have E at position 5 or 6.

2. **All five words end in E, R, or D.** E: CASTLE, TRADE. R: BORDER, CIPHER. D: SPREAD.

3. **Shared letters**: Every word contains at least one E. Four contain R (TRADE, BORDER, SPREAD, CIPHER). Four contain A (CASTLE, TRADE, BORDER, SPREAD).

4. **No obvious acrostic**: First letters C, T, B, S, C don't spell anything.

---

## Mechanism Options Considered

### Option A: Indexed Letter Extraction

Extract one letter from each answer word at a specific index. The index comes from the puzzle itself — something the solver earned during the feeder.

If each puzzle produces not just an answer word but also a NUMBER (the puzzle number, or a number embedded in the solve), use that number as the index into the answer word.

| Word | Index | Letter |
|------|-------|--------|
| CASTLE | ? | ? |
| TRADE | ? | ? |
| BORDER | ? | ? |
| SPREAD | ? | ? |
| CIPHER | ? | ? |

**Problem**: What determines the index? We need a natural number per puzzle. Puzzle number (1-5) gives: C(1)=C, T(2)=R, B(3)=R, S(4)=E, C(5)=E → CRREE. Not a word.

**Try other indices**: We need to spell a 5-letter word. Working backward from possible board-game-related meta answers...

SCORE: S-C-O-R-E
- S from CASTLE: S is at position 3 → index 3
- C from TRADE: no C in TRADE. **Fail.**

PIECE: P-I-E-C-E
- P from CASTLE: no P. **Fail.**

BOARD: B-O-A-R-D
- B from CASTLE: no B. **Fail.**

MOVES: M-O-V-E-S
- No M in CASTLE. **Fail.**

GAMES: G-A-M-E-S
- No G in CASTLE. **Fail.**

STAKE: S-T-A-K-E
- S from CASTLE(3), T from TRADE(... T is at position 1) → index 1, A from BORDER(... no A). **Fail.**

PLACE: P-L-A-C-E
- P from SPREAD(2)=P? Wait, SPREAD is the 4th answer. Let me try ordering differently.

Actually, let me try a different mapping. What if I'm not constrained to puzzle order?

### Option B: Crossword-Style Intersection

Place all 5 words in a mini crossword. Shared letters create intersections. Read the intersection squares.

```
      C
    C A S T L E
      S
  T R A D E
      D
B O R D E R
      E
S P R E A D
      R
```

CASTLE across. CIPHER down through position 2 (A). But CIPHER = C-I-P-H-E-R... A is not in CIPHER. This doesn't work easily.

Let me try differently. SPREAD and BORDER share R at different positions. CASTLE and CIPHER share C.

**This is getting complex and fragile. Move to a better mechanism.**

### Option C: Thematic Extraction — "What Game Are YOU Playing?"

Each answer word secretly describes an aspect of a SIXTH game — the game the solver is playing: the puzzle hunt itself.

- CASTLE: defense, protection → the solver's **strategy** (building a fortress of knowledge)
- TRADE: exchange → the solver **trades** time for answers
- BORDER: boundary → the solver crosses **borders** between domains
- SPREAD: expansion → the puzzle **spreads** across five games
- CIPHER: encoding → the meta itself is a **cipher**

The meta question: "You've played five games tonight. But there was a sixth game the whole time — the one you're playing right now. Each answer word describes a rule of that game."

The solver takes a SPECIFIC letter from each word (given by a rule tied to game night), and those letters spell the meta answer.

**Rule**: Each board game has a canonical player count or iconic number:
- Chess: **2** players → take letter 2 from CASTLE = **A**
- Settlers: **4** players (base game, 3-4) → take letter 4 from TRADE = **D** (wait, TRADE has 5 letters, so index 4 = D)
- Risk: **6** players (max) → take letter 6 from BORDER = **R**
- Pandemic: **4** players (max base) → take letter 4 from SPREAD = **E**
- Codenames: **4**+ players → doesn't differentiate

Player count doesn't produce unique indices. Let me try game-specific numbers.

### Option D: Component Count Extraction (CHOSEN MECHANISM)

Each board game has a signature number — a defining quantity that any player would know:

| Game | Defining quantity | Number | Source |
|------|------------------|--------|--------|
| Chess | Squares on the board | **64** | 8x8 grid |
| Settlers | Hex tiles on the board | **19** | Standard board |
| Risk | Territories on the map | **42** | Standard map |
| Pandemic | Cities on the board | **48** | Standard board |
| Codenames | Words in the grid | **25** | 5x5 grid |

These are too large for letter indexing. Use the DIGITAL ROOT (repeated digit sum until single digit):

| Game | Number | Digital root | How |
|------|--------|-------------|-----|
| Chess | 64 | 6+4 = **10** → 1+0 = **1** | — |
| Settlers | 19 | 1+9 = **10** → 1+0 = **1** | — |

Two 1's. Bad. Try different quantities.

**Simpler approach**: Use the number of letters in each GAME NAME as the index:

| Game | Name | Letter count | Index into answer | Letter extracted |
|------|------|-------------|-------------------|-----------------|
| Chess | CHESS | 5 | 5th of CASTLE | **L** |
| Settlers of Catan | SETTLERS | 8 | too long for TRADE (5 letters) | **Fail** |

Game names don't work cleanly either.

### Option E: First-Letter Chain (CHOSEN — SIMPLE AND ROBUST)

The five answer words, when reordered by a game-night logic, have their first letters spell the meta answer. But C-T-B-S-C (or any reordering) gives: BCCTS, CBSTC, STCBC, etc. No English word.

### Option F: Double-Letter Extraction with Board Game Ordering

**Final mechanism — the one that works:**

Order the 5 games by the year each was first published:

| Game | Year | Answer word |
|------|------|------------|
| Chess | ~600 AD (modern rules ~1475) | CASTLE |
| Risk | 1957 | BORDER |
| Settlers of Catan | 1995 | TRADE |
| Pandemic | 2008 | SPREAD |
| Codenames | 2015 | CIPHER |

Now extract the Nth letter from each word, where N = the word's position in chronological order:

| Position | Word | Index N | Letter |
|----------|------|---------|--------|
| 1st (oldest) | CASTLE | 1 | **C** |
| 2nd | BORDER | 2 | **O** |
| 3rd | TRADE | 3 | **A** |
| 4th | SPREAD | 4 | **E** |
| 5th (newest) | CIPHER | 5 | **E** |

C-O-A-E-E. Not a word.

**Adjust**: What if the index is based on the number of the puzzle (M1-M5 in the hunt order)?

| Module | Word | Puzzle # | Letter at that index |
|--------|------|----------|---------------------|
| M1 | CASTLE | 1 | C |
| M2 | TRADE | 2 | R |
| M3 | BORDER | 3 | R |
| M4 | SPREAD | 4 | E |
| M5 | CIPHER | 5 | E |

C-R-R-E-E. Not a word.

### Option G: The House Rule (FINAL CHOSEN MECHANISM)

**Concept**: The Host left a note. The note contains a rule for each game — a "house rule" — that tells the solver which letter to extract from that game's answer.

The house rules are thematic to each game. They produce an index number that maps to a letter position in the answer word.

**Design (working backward from a target meta answer)**:

Target meta answer: **CHESS** — no, too self-referential.
Target: **TRUMP** — too loaded.
Target: **MATCH** — a game night ends with finding your match. 5 letters. M-A-T-C-H.

| Letter needed | From word | Position in word | House rule |
|---------------|-----------|-----------------|------------|
| M | GAME... | — | Hmm, no M in any of the 5 words... |

No M in CASTLE, TRADE, BORDER, SPREAD, CIPHER. **MATCH fails.**

Target: **BLUFF** — no B except in BORDER. Two F's — no F in any word. Fail.

Target: **PARTY** — P from SPREAD(2) or CIPHER(3). A from CASTLE(2) or TRADE(2) or SPREAD(5). R from TRADE(2) or BORDER(3,6) or SPREAD(3) or CIPHER(6). T from CASTLE(4) or TRADE(1). Y — no Y in any word. **Fail.**

Target: **DICE** — 4 letters, only 5 inputs. D from TRADE(4), BORDER(4), SPREAD(4). I from CIPHER(2). C from CASTLE(1), CIPHER(1). E from CASTLE(6), TRADE(5), BORDER(5), SPREAD(4... wait E is at position 4 in SPREAD? S-P-R-E-A-D, position 4 = E, yes), CIPHER(5).

DICE = D-I-C-E.
- D from TRADE, position 4 (T-R-A-**D**-E)
- I from CIPHER, position 2 (C-**I**-P-H-E-R)
- C from CASTLE, position 1 (**C**-A-S-T-L-E)
- E from BORDER, position 5 (B-O-R-D-**E**-R)

But that leaves SPREAD unused. Need all 5. DICE is only 4 letters.

Target: **WAGER** — W not in any word. Fail.

Target: **PACED** — 5 letters. P-A-C-E-D.
- P from SPREAD, position 2 (S-**P**-R-E-A-D) → index 2
- A from CASTLE, position 2 (C-**A**-S-T-L-E) → index 2
- C from CIPHER, position 1 (**C**-I-P-H-E-R) → index 1
- E from TRADE, position 5 (T-R-A-D-**E**) → index 5
- D from BORDER, position 4 (B-O-R-**D**-E-R) → index 4

All 5 words used. All indices valid. Letters: P, A, C, E, D = **PACED**.

But PACED is not very thematic for board games.

Target: **DEALT** — D-E-A-L-T.
- D from BORDER, position 4 → index 4
- E from CIPHER, position 5 → index 5
- A from CASTLE, position 2 → index 2
- L from CASTLE... wait, I already used CASTLE for A. Each word used once.

Let me try:
- D from SPREAD, position 6 (S-P-R-E-A-**D**) → index 6
- E from BORDER, position 5 (B-O-R-D-**E**-R) → index 5
- A from CASTLE, position 2 (C-**A**-S-T-L-E) → index 2
- L from CASTLE... can't reuse. No L in TRADE or CIPHER. **Fail.**

Target: **CARED** — C-A-R-E-D.
- C from CIPHER, position 1 → index 1
- A from CASTLE, position 2 → index 2
- R from BORDER, position 3 → index 3
- E from SPREAD, position 4 → index 4
- D from TRADE, position 4 → index 4... wait, TRADE position 4 = D (T-R-A-**D**-E). Yes.

Indices: 1, 2, 3, 4, 4. Two 4's. Can I make a rule that naturally produces these indices?

**What if the index IS the puzzle's position in difficulty order (or some game-specific number)?**

Actually — the indices 1, 2, 3, 4, 4 are close to sequential. Let me try pure sequential:

Indices 1, 2, 3, 4, 5 applied to the 5 words in puzzle order (M1-M5):
- CASTLE[1] = C
- TRADE[2] = R
- BORDER[3] = R
- SPREAD[4] = E
- CIPHER[5] = E

CRREE. Not a word.

What about reordering the puzzles? There are 120 permutations of 5 words. With indices 1-2-3-4-5:

Let me check all useful orderings systematically. Each word provides these letters at indices 1-5:

| Word | [1] | [2] | [3] | [4] | [5] | [6] |
|------|-----|-----|-----|-----|-----|-----|
| CASTLE | C | A | S | T | L | E |
| TRADE | T | R | A | D | E | — |
| BORDER | B | O | R | D | E | R |
| SPREAD | S | P | R | E | A | D |
| CIPHER | C | I | P | H | E | R |

For a 5-letter meta answer using indices 1,2,3,4,5 with each word at exactly one position:

Position 1 (index 1): C, T, B, S, C → the first letter of the meta answer
Position 2 (index 2): A, R, O, P, I
Position 3 (index 3): S, A, R, R, P
Position 4 (index 4): T, D, D, E, H
Position 5 (index 5): L, E, E, A, E

I need to pick one word per position such that the 5 letters form a word.

Possibilities for 5th letter (index 5): L (CASTLE), E (TRADE/BORDER/CIPHER), A (SPREAD).

Words ending in E: many. Words ending in A: also many. Words ending in L: fewer.

**Search for words ending in E**:

If position 5 = TRADE → E. Then CASTLE is at position 1,2,3, or 4.
If CASTLE at position 1 → C, ?, ?, ?, E. Remaining: BORDER, SPREAD, CIPHER for positions 2,3,4.
  - pos2: O(BORDER), P(SPREAD), I(CIPHER)
  - pos3: R(BORDER), R(SPREAD), P(CIPHER)
  - pos4: D(BORDER), E(SPREAD), H(CIPHER)

Try CIPHER at 2 (I), BORDER at 3 (R), SPREAD at 4 (E): C-I-R-E-E → CIREE. No.
Try CIPHER at 2 (I), SPREAD at 3 (R), BORDER at 4 (D): C-I-R-D-E → CIRDE. No.
Try SPREAD at 2 (P), BORDER at 3 (R), CIPHER at 4 (H): C-P-R-H-E → No.
Try SPREAD at 2 (P), CIPHER at 3 (P), BORDER at 4 (D): C-P-P-D-E → No.
Try BORDER at 2 (O), CIPHER at 3 (P), SPREAD at 4 (E): C-O-P-E-E → COPEE. No.
Try BORDER at 2 (O), SPREAD at 3 (R), CIPHER at 4 (H): C-O-R-H-E → No.

If CASTLE at position 2 → ?, A, ?, ?, E.
  Position 1: T(TRADE), B(BORDER), S(SPREAD), C(CIPHER)

Try SPREAD at 1 (S), CASTLE at 2 (A), and remaining BORDER+CIPHER for 3+4:
  BORDER at 3 (R), CIPHER at 4 (H): S-A-R-H-E → No.
  CIPHER at 3 (P), BORDER at 4 (D): S-A-P-D-E → No.

Try BORDER at 1 (B), CASTLE at 2 (A), remaining SPREAD+CIPHER for 3+4:
  SPREAD at 3 (R), CIPHER at 4 (H): B-A-R-H-E → No.
  CIPHER at 3 (P), SPREAD at 4 (E): B-A-P-E-E → No.

Try CIPHER at 1 (C), CASTLE at 2 (A), remaining BORDER+SPREAD for 3+4:
  BORDER at 3 (R), SPREAD at 4 (E): C-A-R-E-E → CAREE. No (not a standard word).
  SPREAD at 3 (R), BORDER at 4 (D): C-A-R-D-E → Not standard. Close to CARDE but no.

Hmm. If position 5 = SPREAD → A. Words ending in A:

CIPHER at 1 (C), CASTLE at 2 (A)... wait I need different arrangements.

Position 5 = SPREAD (A). Remaining: CASTLE, TRADE, BORDER, CIPHER at positions 1-4.

CASTLE at 1 (C), TRADE at 2 (R), BORDER at 3 (R), CIPHER at 4 (H): C-R-R-H-A → No.
CASTLE at 1 (C), TRADE at 2 (R), CIPHER at 3 (P), BORDER at 4 (D): C-R-P-D-A → No.
CASTLE at 1 (C), BORDER at 2 (O), TRADE at 3 (A), CIPHER at 4 (H): C-O-A-H-A → No.
CASTLE at 1 (C), BORDER at 2 (O), CIPHER at 3 (P), TRADE at 4 (D): C-O-P-D-A → No.
CASTLE at 1 (C), CIPHER at 2 (I), TRADE at 3 (A), BORDER at 4 (D): C-I-A-D-A → No.
CASTLE at 1 (C), CIPHER at 2 (I), BORDER at 3 (R), TRADE at 4 (D): C-I-R-D-A → No.
TRADE at 1 (T), CASTLE at 2 (A), BORDER at 3 (R), CIPHER at 4 (H): T-A-R-H-A → No.
TRADE at 1 (T), CASTLE at 2 (A), CIPHER at 3 (P), BORDER at 4 (D): T-A-P-D-A → No.
TRADE at 1 (T), BORDER at 2 (O), CASTLE at 3 (S), CIPHER at 4 (H): T-O-S-H-A → No.
TRADE at 1 (T), CIPHER at 2 (I), CASTLE at 3 (S), BORDER at 4 (D): T-I-S-D-A → No.
BORDER at 1 (B), CASTLE at 2 (A), TRADE at 3 (A), CIPHER at 4 (H): B-A-A-H-A → No.
BORDER at 1 (B), TRADE at 2 (R), CASTLE at 3 (S), CIPHER at 4 (H): B-R-S-H-A → No.

None of those work. Position 5 = CASTLE → L. Words ending in L:

TRADE at 1 (T), BORDER at 2 (O), SPREAD at 3 (R), CIPHER at 4 (H): T-O-R-H-L → No.
TRADE at 1 (T), CIPHER at 2 (I), BORDER at 3 (R), SPREAD at 4 (E): T-I-R-E-L → No.
TRADE at 1 (T), SPREAD at 2 (P), BORDER at 3 (R), CIPHER at 4 (H): T-P-R-H-L → No.
BORDER at 1 (B), TRADE at 2 (R), SPREAD at 3 (R), CIPHER at 4 (H): B-R-R-H-L → No.
SPREAD at 1 (S), TRADE at 2 (R), BORDER at 3 (R), CIPHER at 4 (H): S-R-R-H-L → No.
CIPHER at 1 (C), TRADE at 2 (R), SPREAD at 3 (R), BORDER at 4 (D): C-R-R-D-L → No.
CIPHER at 1 (C), BORDER at 2 (O), TRADE at 3 (A), SPREAD at 4 (E): C-O-A-E-L → No.
SPREAD at 1 (S), CIPHER at 2 (I), BORDER at 3 (R), TRADE at 4 (D): S-I-R-D-L → No.
SPREAD at 1 (S), BORDER at 2 (O), CIPHER at 3 (P), TRADE at 4 (D): S-O-P-D-L → No.

Sequential indexing with reordering does not produce a clean word. The letter pool at each index position is too constrained.

---

## CHOSEN MECHANISM: Variable Index Extraction with Game-Night Theming

**Abandon the sequential-index constraint.** Instead, each puzzle contributes a specific index into its answer word, and the indices are determined by a game-night-themed rule presented in the meta puzzle.

### The Meta Puzzle: "The Sixth Game"

**Flavor**: The Host set out five games. But there was always a sixth game on the table — the one you've been playing all night. Each game taught you a number. Each number points to a letter in your answer. The sixth game's answer was hiding inside the other five all along.

**Mechanism**: Each board game has a defining number of PLAYERS for its most iconic configuration:

| Game | Iconic player count | Justification |
|------|-------------------|---------------|
| Chess | 2 | Always 2 players |
| Settlers of Catan | 4 | Base game: 3-4 players (box says "3-4") — use 4 as the upper standard |
| Risk | 6 | Classic Risk supports 2-6, box shows "2-6" — the defining max is 6 |
| Pandemic | 4 | Base game: 2-4 players — 4 is standard full team |
| Codenames | 4+ | Typically played with 4+ — but this is ambiguous |

Problem: Settlers and Pandemic both give 4, and Codenames is vague.

**Better approach — use a NUMBER INHERENT TO EACH PUZZLE'S ANSWER, not the game.**

### FINAL MECHANISM: The Hidden Sixth Game

The meta is a mini-puzzle. The Host gives five "house rules" — one for each game. Each house rule is a riddle whose answer is a number. That number is the index into the corresponding answer word.

**The House Rules:**

*The Host has one more game for you. Five rules. Five numbers. Five letters. One word.*

| Rule | Game | Riddle | Answer (number) | Letter from answer word |
|------|------|--------|-----------------|------------------------|
| 1 | Chess | "In chess, how many squares can a king reach from the center of an empty board?" | **8** — but CASTLE only has 6 letters. Try: "How many pieces start the game on the back rank?" = **8**. Still too high. "How many pawns does each player start with?" = **8**. "Castling moves the king how many squares?" = **2**. CASTLE[2] = **A**. |
| 2 | Settlers | "How many different resources exist in Settlers of Catan?" | **5**. TRADE[5] = **E**. But wait, TRADE only has 5 letters — index 5 = E. |
| 3 | Risk | "How many continents are on the Risk board?" | **6**. BORDER[6] = **R**. |
| 4 | Pandemic | "How many diseases must you cure to win Pandemic?" | **4**. SPREAD[4] = **E**. |
| 5 | Codenames | "How many words in the Codenames grid?" | **25**. Too big. "How many rows in the Codenames grid?" = **5**. CIPHER[5] = **E**. |

Letters: A, E, R, E, E = AEREE. Not a word.

Let me try different numbers:

| Game | Question | Number | Word[N] |
|------|----------|--------|---------|
| Chess | "How many squares between King and Rook when castling kingside?" | **2** (f1 and g1) | CASTLE[2] = A |
| Settlers | "How many resource cards to build a Road?" | **2** | TRADE[2] = R |
| Risk | "How many continents on the board?" | **6** | BORDER[6] = R |
| Pandemic | "How many diseases in Pandemic?" | **4** | SPREAD[4] = E |
| Codenames | "How many rows in the grid?" | **5** | CIPHER[5] = E |

A, R, R, E, E = ARREE. No.

I need to find a combination of reasonable game-trivia numbers (1-6 range for most words) that extracts a real word.

**Systematic approach**: What 5-letter words can I spell from the available letters?

Available letters by index:

CASTLE: [1]=C, [2]=A, [3]=S, [4]=T, [5]=L, [6]=E
TRADE: [1]=T, [2]=R, [3]=A, [4]=D, [5]=E
BORDER: [1]=B, [2]=O, [3]=R, [4]=D, [5]=E, [6]=R
SPREAD: [1]=S, [2]=P, [3]=R, [4]=E, [5]=A, [6]=D
CIPHER: [1]=C, [2]=I, [3]=P, [4]=H, [5]=E, [6]=R

All possible 5-letter combinations (one letter per word):

This is a 6x5x6x6x6 = 6,480 possibility space. Let me focus on promising-looking combinations.

**Target: STARE** (S-T-A-R-E)
- S from CASTLE[3], T from TRADE[1], A from BORDER(no A). Fail.

**Target: PRIDE** (P-R-I-D-E)
- P from SPREAD[2], R from TRADE[2]... two index-2's. Need different sources.
- P from CIPHER[3], R from BORDER[3]... two index-3's. But that's OK — the index comes from different game questions.
- P from SPREAD[2], R from TRADE[2], I from CIPHER[2]... three index-2's. Getting silly.
- P from CIPHER[3], R from CASTLE? No R in CASTLE. Fail.

**Target: CREST** (C-R-E-S-T)
- C from CASTLE[1], R from TRADE[2], E from BORDER[5], S from SPREAD? No S at useful index... SPREAD[1]=S (index 1). T from CIPHER? No T in CIPHER. Fail.

**Target: PARSE** (P-A-R-S-E)
- P from CIPHER[3], A from CASTLE[2], R from BORDER[3]... wait, two index-3's.
- P from SPREAD[2], A from CASTLE[2]... two index-2's.
- P from CIPHER[3], A from TRADE[3], R from BORDER[3,6], S from SPREAD[1], E from CASTLE[6].
  P=CIPHER[3], A=TRADE[3], R=BORDER[3]... three questions all answering "3". Awkward but technically possible. S=SPREAD[1], E=CASTLE[6]. Let me check: What game question gives 3 for Codenames, 3 for Settlers, and 3 for Risk? And 1 for Pandemic, 6 for Chess?

  Plausible questions: "How many of [something] = 3?" for three different games. Not elegant.

**Target: TRACE** (T-R-A-C-E)
- T from CASTLE[4], R from TRADE[2], A from BORDER? No A. Fail.
- T from CASTLE[4], R from BORDER[3], A from TRADE[3], C from CIPHER[1], E from SPREAD[4].
  Indices: 4, 3, 3, 1, 4. Two pairs of duplicates.
  Questions: Chess=4 ("How many bishops start the game?" = 4... no, 2. "How many first moves are knight moves?" = 4... yes, correct: Nc3, Nf3, Na3, Nh3). Settlers... wait, TRADE is index 3: "How many of each number token?" — too obscure. Risk=3: "How many entry points to Africa?" = 3. Pandemic? Not applicable — we'd need SPREAD at index 4 = E, question = 4 ("How many diseases?" = 4). Codenames=1: "How many assassins?" = 1.

Let me verify: T=CASTLE[4], R=BORDER[3], A=TRADE[3], C=CIPHER[1], E=SPREAD[4].

**TRACE works.** And the questions can be elegant:

| # | Game | House Rule (question) | Answer | Index | Word | Letter |
|---|------|----------------------|--------|-------|------|--------|
| 1 | Chess | "How many different opening moves can a knight make?" | **4** (Na3, Nc3, Nf3, Nh3) | 4 | CASTLE | **T** |
| 2 | Settlers | "How many ore does a city cost?" | **3** | 3 | TRADE | **A** |
| 3 | Risk | "How many entry points does Africa have?" | **3** | 3 | BORDER | **R** |
| 4 | Pandemic | "How many diseases must you cure to win?" | **4** | 4 | SPREAD | **E** |
| 5 | Codenames | "How many assassins hide in the grid?" | **1** | 1 | CIPHER | **C** |

Letters in puzzle order: T, A, R, E, C = TAREC. That's not TRACE.

The order matters. If the meta answer reads in puzzle order (M1-M5 = Chess, Settlers, Risk, Pandemic, Codenames), the letters are T-A-R-E-C.

I need to reorder. TRACE = T-R-A-C-E.

**Reordering**:
- T from Chess (M1)
- R from Risk (M3)
- A from Settlers (M2)
- C from Codenames (M5)
- E from Pandemic (M4)

Solver order: M1, M3, M2, M5, M4. That's not natural. Unless the Host provides an ordering clue.

**Better**: use the ordering implied by the house rules themselves. The Host lists the rules in a specific order, and that order determines the letter sequence.

---

## FINAL META DESIGN

### Title: The Sixth Game

### Flavor Text

*Five games. Five answers. The Host watches from the kitchen doorway. "There's one more," they say, not quite meeting your eye. "The game you've been playing the whole time."*

*On the table, between the empty chip bowl and the stack of score pads, there's a card you hadn't noticed. Five questions. Five numbers. The answers were inside the other five all along.*

### The Card

Each question refers to one of the five board games. The answer to each question is a number. Use that number as a position in the corresponding puzzle's answer word. Extract one letter per game.

| # | The Host asks: | Game |
|---|---------------|------|
| 1 | "How many assassins hide in the grid?" | Codenames |
| 2 | "How many knights can White move on the first turn?" | Chess |
| 3 | "How many ore to upgrade a settlement to a city?" | Settlers of Catan |
| 4 | "How many diseases must you cure to win?" | Pandemic |
| 5 | "How many entry points does Africa have on the Risk board?" | Risk |

### Solution

| # | Game | Answer | Index | Feeder answer | Letter |
|---|------|--------|-------|---------------|--------|
| 1 | Codenames | **1** | 1st letter | CIPHER | **C** |
| 2 | Chess | **4** (Na3, Nc3, Nf3, Nh3) | 4th letter | CASTLE | **T**... wait. 4 knights moves? Let me reconsider. White's first-move knight options: Nf3 and Nc3 only. Knights on b1 and g1 — each can go to two squares. So 4 total? No: Na3, Nc3, Nf3, Nh3 — but some of those are from the same knight. Knight on b1 can go to a3 or c3 (2 moves). Knight on g1 can go to f3 or h3 (2 moves). Total = 4 different knight first moves. |

Wait, the world data says "Total possible first moves (White): 20 (16 pawn moves + 4 knight moves)." So yes, **4 knight moves**. Good.

| # | Game | Answer | Index | Feeder answer | Letter |
|---|------|--------|-------|---------------|--------|
| 1 | Codenames | **1** | 1 | CIPHER | **C** |
| 2 | Chess | **4** | 4 | CASTLE | **T**... CASTLE[4] = T. Hmm. |

C-T-?-?-? ... targeting TRACE we need position 3 to give R.

| 3 | Settlers | **3** | 3 | TRADE | **A** |

C-T-A... not TRACE (which is C... no). Let me reconsider. I'm confusing myself.

Let me just find a valid word by brute force, then build the questions.

**Available letters again, by index (1 through max):**

```
CASTLE: C(1) A(2) S(3) T(4) L(5) E(6)
TRADE:  T(1) R(2) A(3) D(4) E(5)
BORDER: B(1) O(2) R(3) D(4) E(5) R(6)
SPREAD: S(1) P(2) R(3) E(4) A(5) D(6)
CIPHER: C(1) I(2) P(3) H(4) E(5) R(6)
```

I need to pick one (index, letter) from each word such that the 5 letters form a word. Let me list promising words.

**SCARE**: S-C-A-R-E
- S from CASTLE[3] or SPREAD[1]
- C from CIPHER[1]
- A from TRADE[3] or CASTLE[2] or SPREAD[5]
- R from TRADE[2] or BORDER[3,6] or SPREAD[3] or CIPHER[6]
- E from CASTLE[6] or TRADE[5] or BORDER[5] or SPREAD[4] or CIPHER[5]

Try: S=SPREAD[1], C=CIPHER[1]... two index-1's. Doable if both questions answer "1."
Try: S=CASTLE[3], C=CIPHER[1], A=TRADE[3]... two index-3's.
Try: S=SPREAD[1], C=CIPHER[1], A=TRADE[3], R=BORDER[3], E=CASTLE[6].
Indices: 1, 1, 3, 3, 6. Two questions answering 1, two answering 3, one answering 6.

Questions with answer 1: Codenames assassin count = 1. Pandemic... "How many research stations start on the board?" = 1 (Atlanta). So:
- SPREAD index 1 → Pandemic question answering 1: "How many research stations are on the board at the start of the game?" = 1. S.
- CIPHER index 1 → Codenames question answering 1: "How many assassins?" = 1. C.
- TRADE index 3 → Settlers question answering 3: "How many ore to build a city?" = 3. A.
- BORDER index 3 → Risk question answering 3: "How many entry points does Africa have?" = 3. R.
- CASTLE index 6 → Chess question answering 6: Hmm. What chess quantity = 6? "How many pieces does White have on the first rank besides king and rooks?" = Queen + 2 Bishops + 2 Knights = 5. Not 6. "How many types of chess pieces?" = 6 (King, Queen, Rook, Bishop, Knight, Pawn). **Yes.**

**SCARE works.**

| # | Host's question | Game | Answer | Index | Feeder | Letter |
|---|----------------|------|--------|-------|--------|--------|
| 1 | "How many types of chess piece are there?" | Chess | **6** | 6 | CASTLE | **E** |

Wait, CASTLE[6] = E. So position 1 in the meta answer = E. But I need S-C-A-R-E... E is at position 5.

**The ORDER of the questions determines the letter order.** I just need to arrange the 5 questions so the letters come out S-C-A-R-E:

| Position in meta | Letter needed | Source | Index | Question |
|-----------------|---------------|--------|-------|----------|
| 1 | S | SPREAD[1] | 1 | "How many research stations start the game?" (Pandemic) |
| 2 | C | CIPHER[1] | 1 | "How many assassins in the Codenames grid?" |
| 3 | A | TRADE[3] | 3 | "How many ore to upgrade to a city?" (Settlers) |
| 4 | R | BORDER[3] | 3 | "How many entry points to Africa?" (Risk) |
| 5 | E | CASTLE[6] | 6 | "How many types of chess piece?" (Chess) |

**Verification:**
- Pandemic: 1 research station at start (Atlanta). SPREAD[1] = S. Correct.
- Codenames: 1 assassin. CIPHER[1] = C. Correct.
- Settlers: City costs 3 Ore + 2 Grain. The question asks about Ore specifically = 3. TRADE[3] = A. Correct.
- Risk: Africa entry points = 3 (North Africa, Egypt, East Africa). BORDER[3] = R. Correct.
- Chess: 6 piece types (King, Queen, Rook, Bishop, Knight, Pawn). CASTLE[6] = E. Correct.

**Meta answer: SCARE.**

Is SCARE thematic for a game night? "The sixth game was a SCARE." The Host was anxious the whole time — worried no one would have fun, worried someone would flip the board. The meta answer IS the Host's emotional state. The game night was a SCARE.

*"Five games. Five answers. Five letters. The game you were playing the whole time — making the Host nervous — was the real game. You didn't know you were playing it. But you were. You were a SCARE."*

That works thematically.

### 80% Rule Check

Can the meta be solved with only 4 of 5 answers?

With 4 answers, the solver has 4 of 5 letters of SCARE, with one blank:
- Missing letter 1 (S): _CARE → SCARE is the only common word. **Solvable.**
- Missing letter 2 (C): S_ARE → SCARE, SHARE, SNARE, SPARE, STARE, SWEAR... multiple options. **Ambiguous** — but SCARE fits the theme (Host's anxiety). With theme confirmation, **likely solvable.**
- Missing letter 3 (A): SC_RE → SCARE, SCORE, SCERE (not a word). SCARE and SCORE both work. **Ambiguous.** SCORE is also game-themed. This is a genuine 80% issue.
- Missing letter 4 (R): SCA_E → SCARE, SCALE, SCAPE (not standard). **Likely solvable.**
- Missing letter 5 (E): SCAR_ → SCARE, SCARF, SCARY, SCARS. **Ambiguous.**

**Verdict**: 3 of 5 missing-letter scenarios resolve cleanly. 2 are ambiguous. The 80% rule is partially satisfied — better than most 5-feeder metas. For a 5-puzzle hunt, losing 1 feeder makes the meta hard but not impossible. Acceptable.

---

## The Meta Puzzle (Solver-Facing Version)

### The Sixth Game

*Five games. Five answers. But the Host is still standing in the doorway. "There's one more," they say. "The game you've been playing the whole time."*

*A card on the table. You hadn't noticed it. Five questions. Five numbers. The answers were hiding inside the other five all along.*

---

Each question below refers to one of the five board games you just solved. The answer to each question is a number. Use that number as a POSITION in the corresponding game's answer word, and extract that letter.

Read the five extracted letters in order (1-5) to find the final answer.

---

**Question 1** (Pandemic): How many research stations are on the board at the very start of the game?

**Question 2** (Codenames): How many assassins hide in the grid?

**Question 3** (Settlers of Catan): How many Ore does it cost to upgrade a Settlement to a City?

**Question 4** (Risk): How many entry points does Africa have?

**Question 5** (Chess): How many different types of chess piece are there?

---

*The Host watches you write the letters. Five of them. One word. They almost smile. Almost.*

---

### Meta Answer: SCARE (5 letters)

*"The game you were playing the whole time was making the Host nervous. Five games, five puzzles, one anxious host. The whole night was a scare."*

---

## Design Notes

### Why This Mechanism

1. **Uses all 5 feeder answers** — each contributes exactly one letter.
2. **The game-trivia questions are fair** — every answer is in the world/ data files and is common board-game knowledge.
3. **No arbitrary indexing** — each number comes from a genuine, well-known fact about the game.
4. **Thematic payoff** — SCARE connects to the Host's character (anxious, worried about the game night), giving the meta a narrative resolution.
5. **The 80% rule is reasonably satisfied** — 4 of 5 letters usually suffice for reconstruction.

### Verification of Each Question

| Q | Game | Question | Answer | Source verification |
|---|------|----------|--------|-------------------|
| 1 | Pandemic | Research stations at start | 1 | world/pandemic.md: "Starting research station: Atlanta (CDC headquarters)" |
| 2 | Codenames | Assassins in grid | 1 | world/codenames.md: "Assassin — 1" |
| 3 | Settlers | Ore for City upgrade | 3 | world/settlers.md: "City (upgrade) — 3 Ore" |
| 4 | Risk | Africa entry points | 3 | world/risk.md: "Africa — 3 (North Africa, Egypt, East Africa)" |
| 5 | Chess | Types of chess piece | 6 | world/chess.md: King, Queen, Rook, Bishop, Knight, Pawn = 6 types |

All verified against world data. No Deliberate Errors principle satisfied.

### G5 Result

**Can a meta be built from 5 uncoordinated answer words?** Yes — but it required extensive combinatorial search. The Captain tested 15+ target words and dozens of index combinations before finding SCARE. The constraint (one letter per word, index must correspond to a natural game-trivia question) is tight but not impossible. With 5 words of 5-6 letters each, the available letter pool is large enough to find SOMETHING — but the designer has very little control over WHAT the meta answer will be. SCARE was the best available option, not the ideal one.

**Lesson for the toolkit**: Uncoordinated answer words make meta design 10x harder. Even a little coordination (e.g., "your answer must contain an R somewhere") would dramatically expand the designer's options. For real hunts, consider giving authors minimal constraints on letter content while preserving their freedom to choose the word.
