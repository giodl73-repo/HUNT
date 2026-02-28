# IRONFALL — Meta Design

## Super-Meta Target

The True Ending cheat code is a 12-button sequence on the SNES controller.

**The Code (ROT13):** HH QQ YE YE ON KY

This is entered on the title screen after completing the game. The first 8 inputs mirror the most famous cheat code in gaming history — but the final 4 inputs diverge. That recognition IS the aha.

---

## Architecture

```
P01─┐
P02─┤
P03─┼─ Act I feeder answers ──→ META-I answer ──┐
P04─┤                                            │
P05─┘                                            │
                                                  ├──→ SUPER-META
P06─┐                                            │     (12 letters
P07─┤                                            │      = 12 buttons
P08─┼─ Act II feeder answers ──→ META-II answer ─┘      = cheat code)
P09─┤
P10─┘
```

Each of the 12 answer words contributes ONE letter. The extraction position = the puzzle's slot number within its act. Meta answers use position 1.

---

## Extraction Table (all answers ROT13-encoded)

| # | Source | Answer (ROT13) | Slot | Extract Pos | Letter (ROT13) | Button |
|---|--------|----------------|------|-------------|----------------|--------|
| 1 | P01 | HZOEN | I-1 | 1 | H | (ROT13) |
| 2 | P02 | DHRYY | I-2 | 2 | H | (ROT13) |
| 3 | P03 | BEQRE | I-3 | 3 | Q | (ROT13) |
| 4 | P04 | FUNQR | I-4 | 4 | Q | (ROT13) |
| 5 | P05 | NAIVY | I-5 | 5 | Y | (ROT13) |
| 6 | META-I | ERNYZ | -- | 1 | E | (ROT13) |
| 7 | P06 | YBGHF | II-1 | 1 | Y | (ROT13) |
| 8 | P07 | BEOVG | II-2 | 2 | E | (ROT13) |
| 9 | P08 | RZORE | II-3 | 3 | O | (ROT13) |
| 10 | P09 | TYRNZ | II-4 | 4 | N | (ROT13) |
| 11 | P10 | URYVK | II-5 | 5 | K | (ROT13) |
| 12 | META-II | YRTVG | -- | 1 | Y | (ROT13) |

**Verification:** Each letter in the extraction column, when ROT13-decoded, produces a button abbreviation (U/D/L/R/A/B/X). The 12 buttons in order form the cheat code.

---

## META-I — "The Archive Index"

### Concept

After solving Act I's 5 puzzles, the solver receives (or discovers) the archive's index page — a table of the 5 most-edited wiki pages. The answer words appear as page titles.

### Mechanism

The archive index shows:

```
╔══════════════════════════════════════════════════╗
║  IRONFALL-ARCHIVE.NET — TOP EDITED PAGES         ║
╠══════════════════════════════════════════════════╣
║  Page Title          Edits    Last Edited        ║
║  ─────────────────   ─────   ─────────────       ║
║  [Answer Word 1]      24      Mar 14, 1998       ║
║  [Answer Word 2]      17      Jun 8, 2001        ║
║  [Answer Word 3]      12      Aug 22, 1999       ║
║  [Answer Word 4]       9      Nov 3, 1997        ║
║  [Answer Word 5]       6      Feb 11, 2003       ║
╚══════════════════════════════════════════════════╝
```

Each answer word is a 5-letter page title. The edit count determines which letter to extract:

**Extraction rule:** (edit count mod 5) + 1 = column position

| Answer (ROT13) | Edits | (Edits mod 5)+1 | Extracted letter |
|-----------------|-------|-----------------|-----------------|
| BEQRE | 24 | (24 mod 5)+1 = 5 | E(ROT13) |
| DHRYY | 17 | (17 mod 5)+1 = 3 | R(ROT13) |
| FUNQR | 12 | (12 mod 5)+1 = 3 | N(ROT13) |
| NAIVY | 9 | (9 mod 5)+1 = 5 | Y(ROT13) |
| HZOEN | 6 | (6 mod 5)+1 = 2 | Z(ROT13) |

**Reading order:** Sort pages by edit count descending (most edited first):

| Rank | Answer (ROT13) | Edits | Position | Letter (ROT13) |
|------|----------------|-------|----------|----------------|
| 1 | BEQRE | 24 | 5 | E |
| 2 | DHRYY | 17 | 3 | R |
| 3 | FUNQR | 12 | 3 | N |
| 4 | NAIVY | 9 | 5 | Y |
| 5 | HZOEN | 6 | 2 | Z |

Reading extracted letters top to bottom: E, R, N, Y, Z → ROT13 decode → meta answer.

### 80% Rule Check

With 4 of 5 answers, the solver has 4 letters and 1 blank. The word is common enough that the blank can be inferred. Even with 3 of 5, the pattern should be guessable for experienced solvers.

### Aha

The archive index — which looks like decoration — IS the meta. The edit counts aren't flavor. They're the extraction key.

---

## META-II — "The Game Secrets"

### Concept

After solving Act II's 5 puzzles, the solver encounters a "Game Secrets" page from the archive — a compilation of easter eggs and hidden features. Each answer word appears as the name of a secret.

### Mechanism

The "Game Secrets" page lists 5 discovered secrets, each labeled with a difficulty rating (number of stars). The answer words are the secret names.

**Extraction rule:** Each secret's star rating (1-5 stars) tells the solver which letter of that secret's name to read.

| Secret (ROT13) | Stars | Position | Letter |
|----------------|-------|----------|--------|
| YBGHF | 3 | 3 | G(ROT13) |
| BEOVG | 1 | 1 | B(ROT13) |
| RZORE | 5 | 5 | E(ROT13) |
| TYRNZ | 2 | 2 | Y(ROT13) |
| URYVK | 4 | 4 | V(ROT13) |

Wait, I need to verify this produces LEGIT (ROT13: YRTVG).

YBGHF = LOTUS. LOTUS[3] = T. T in ROT13 = G. Check.
BEOVG = ORBIT. ORBIT[1] = O. O in ROT13 = B. Check.
RZORE = EMBER. EMBER[5] = R. R in ROT13 = E. Check.
TYRNZ = GLEAM. GLEAM[2] = L. L in ROT13 = Y. Check.
URYVK = HELIX. HELIX[4] = I. I in ROT13 = V. Check.

So extracted letters (ROT13): G, B, E, Y, V

Reading order: I need these in order to spell YRTVG (= LEGIT).

**Reading order:** Sort by star rating ascending (1-star first):

| Stars | Secret (ROT13) | Letter (ROT13) |
|-------|----------------|----------------|
| 1 | BEOVG | B |
| 2 | TYRNZ | Y |
| 3 | YBGHF | G |
| 4 | URYVK | V |
| 5 | RZORE | E |

Reading: B, Y, G, V, E → doesn't spell YRTVG.

Hmm. Let me re-order. I need L, E, G, I, T → ROT13 → Y, R, T, V, G.

So reading order must produce Y, R, T, V, G.

Y comes from GLEAM[2]=L → ROT13=Y. Star rating = ?
R comes from... I need R (ROT13) which is E (plaintext). EMBER[1]=E → ROT13=R. Or GLEAM[3]=E → ROT13=R. But GLEAM already provides Y.

Wait, let me reconsider. I need to assign star ratings (1-5) to extract specific positions AND reading order to produce LEGIT.

I'll assign reading order by star rating ascending, and I need to produce LEGIT in that order:

Position 1 (1 star) → L: need a word where the star-indicated position gives L
Position 2 (2 stars) → E: need a word where position 2 gives E
Position 3 (3 stars) → G: need a word where position 3 gives G
Position 4 (4 stars) → I: need a word where position 4 gives I
Position 5 (5 stars) → T: need a word where position 5 gives T

1-star → extract position 1 → letter L: LOTUS[1]=L. Assign LOTUS 1 star.
2-star → extract position 2 → letter E: HELIX[2]=E. Assign HELIX 2 stars.
3-star → extract position 3 → letter G: no word has G at position 3. ORBIT[3]=B, EMBER[3]=B, GLEAM[3]=E. No G at position 3.

Doesn't work with star = position. Let me decouple: star rating determines READING ORDER, and a SEPARATE mechanism determines which position to extract.

**Revised mechanism:** The "Game Secrets" page shows a table with each secret having a "discovery date" (year). The year's last digit determines the extraction position. Star rating determines reading order.

| Secret (ROT13) | Stars | Year | Last digit+1 | Position | Letter |
|----------------|-------|------|---------------|----------|--------|
| YBGHF | 3 | 2002 | 3 | 3 | T→G(ROT13) |
| BEOVG | 5 | 1998 | 9→mod5+1=5 | 5 | T→G(ROT13)... |

This is getting too complex. Let me use a simpler mechanism.

**Final META-II mechanism:**

The 5 Act II answer words are placed in a simple 5-row display. Clues on the page (themed as "hint level" or "secret difficulty") tell the solver which letter from each word to extract, and the display order on the page IS the reading order.

I just need to find an ordering and position assignment that produces LEGIT.

LEGIT = L, E, G, I, T

Available:
L: LOTUS[1], GLEAM[2], HELIX[3]
E: EMBER[1], HELIX[2], GLEAM[3]
G: GLEAM[1]
I: ORBIT[4], HELIX[4]
T: LOTUS[3], ORBIT[5]

G must come from GLEAM[1].
Since GLEAM→G(pos 1), GLEAM is used.

I from ORBIT[4] or HELIX[4].
T from LOTUS[3] or ORBIT[5].

If I from ORBIT[4] and T from LOTUS[3]:
L from HELIX[3]... but T is LOTUS[3] and L is HELIX[3] — different words, same position number. That's fine.
E from EMBER[1].

Assignment:
L → HELIX, pos 3
E → EMBER, pos 1
G → GLEAM, pos 1
I → ORBIT, pos 4
T → LOTUS, pos 3

Reading order (to spell LEGIT): HELIX, EMBER, GLEAM, ORBIT, LOTUS

Now I need a thematic mechanism that tells the solver:
- HELIX → extract position 3
- EMBER → extract position 1
- GLEAM → extract position 1
- ORBIT → extract position 4
- LOTUS → extract position 3

And read in this order: HELIX, EMBER, GLEAM, ORBIT, LOTUS

**Mechanism: "Secret Difficulty Ratings"**

Each secret is rated by the community on a 1-5 difficulty scale. The difficulty rating IS the extraction position.

But HELIX needs pos 3 and GLEAM needs pos 1 — the difficulty ratings would be 3 and 1 respectively. And reading order... could be by difficulty ascending: 1(EMBER or GLEAM), 1, 3, 3, 4. Duplicates break unique ordering.

Let me adjust. What if I use:
L → GLEAM, pos 2 (instead of HELIX pos 3)
Then HELIX is free for something else.

L → GLEAM[2], E → EMBER[1], G → ???. GLEAM already used for L.

G only comes from GLEAM[1]. If GLEAM→L at pos 2, then G has no source. Stuck.

Let me try:
L → LOTUS[1]
E → HELIX[2]
G → GLEAM[1]
I → ORBIT[4]
T → EMBER? EMBER has no T. Only LOTUS[3] and ORBIT[5] have T.

If LOTUS→L(pos 1), T must come from ORBIT[5]. But ORBIT→I(pos 4). Can't use ORBIT for both.

LOTUS→L(1), T from ORBIT[5], I from HELIX[4], E from EMBER[1], G from GLEAM[1]:
But then ORBIT provides both T(pos 5) and I is from HELIX(4). No collision.
LOTUS→L(1), EMBER→E(1), GLEAM→G(1), HELIX→I(4), ORBIT→T(5)

Positions: 1, 1, 1, 4, 5. Three extractions from position 1.

**Mechanism: "Morimoto's clue system"**

Morimoto left a note in the developer's room: "Count the letters in each secret's name. The answer is in the number." Each secret name has a different number of letters:

LOTUS = 5 letters → extract position 5 → S. No, I need L from LOTUS.

This doesn't match. Let me just use a visual mechanism: the secrets are displayed on a game screen where each word has one letter highlighted (drawn in a different color or with a cursor pointing to it). The highlighted letter is the extraction. The visual display order IS the reading order.

This is the cleanest — no arithmetic, just "read the highlighted letters from top to bottom."

**Final META-II: "The Game Secrets Screen"**

The solver sees a mock game screen showing 5 "discovered secrets" in a vertical list. Each secret name has one letter circled/highlighted (shown in brackets or asterisks). Reading the highlighted letters from top to bottom spells the meta answer.

```
IRONFALL — SECRETS DISCOVERED

  [L]O T U S          ← L
  E M B[E]R           ← E  (wait, EMBER[4]=E, but I need E at pos...)
```

Hmm, I need to carefully assign which letter is highlighted:

L-E-G-I-T in order:
Row 1: _OTUS → highlight L at pos 1 → **L**OTUS
Row 2: _MBER → highlight E at pos 1 → **E**MBER
Row 3: _LEAM → highlight G at pos 1 → **G**LEAM
Row 4: ORB_T → highlight I at pos 4 → ORB**I**T
Row 5: LO_US → highlight T at pos 3 → LO**T**US — but LOTUS already used in row 1.

Each answer word appears only once. Let me re-derive:

LEGIT: need 5 letters from 5 different words.
L: LOTUS(1), GLEAM(2), HELIX(3)
E: EMBER(1), HELIX(2), GLEAM(3)
G: GLEAM(1)
I: ORBIT(4), HELIX(4)
T: LOTUS(3), ORBIT(5)

G must be GLEAM. So GLEAM is used for G.
Remaining letters: L, E, I, T from {LOTUS, ORBIT, EMBER, HELIX}

L: LOTUS(1), HELIX(3)
E: EMBER(1), HELIX(2)
I: ORBIT(4), HELIX(4)
T: LOTUS(3), ORBIT(5)

If HELIX→L(3): then E from EMBER(1), I from ORBIT(4), T from LOTUS(3).
Reading order: HELIX→L, EMBER→E, GLEAM→G, ORBIT→I, LOTUS→T

If HELIX→E(2): then L from LOTUS(1), I from ORBIT(4), T from... LOTUS(3) — LOTUS already used for L. T from ORBIT(5) — ORBIT already used for I. Stuck.

If HELIX→I(4): then L from LOTUS(1), E from EMBER(1), T from ORBIT(5). Or T from LOTUS(3) — LOTUS used for L. So T from ORBIT(5).
Reading order: LOTUS→L, EMBER→E, GLEAM→G, HELIX→I, ORBIT→T

Both work. Let me use the second (more natural ordering where the positions gradually increase):

Row 1: **L**OTUS (pos 1)
Row 2: **E**MBER (pos 1)
Row 3: **G**LEAM (pos 1)
Row 4: HEL**I**X (pos 4)
Row 5: ORBI**T** (pos 5)

This is very clean. The first three secrets have their first letter highlighted, then the pattern shifts — the solver must notice the highlighted letters don't always start at the beginning.

**Mechanism justification:** The game's "Secrets" screen shows each secret with a "discovery marker" — a glowing pixel under one letter of the secret name. Morimoto placed these markers deliberately. Reading the marked letters spells his hidden message.

The aha: "The markers aren't random decorations — they're spelling something."

### 80% Rule Check

With 4 of 5 answers, the solver has 4 of 5 letters of LEGIT and can infer the missing one. Common English word, high confidence even with 3 of 5.

---

## SUPER-META — "The True Ending"

### Concept

Once both round metas are solved, the solver has all 12 answer words. The archive's final page reveals the extraction mechanism: each answer word corresponds to one button press, and the slot number determines which letter to extract.

### Presentation

The final archive page is a post from DataMiner_X:

```
╔══════════════════════════════════════════════════════╗
║  [SOLVED] THE TRUE ENDING — I FOUND THE CODE        ║
║  Posted by DataMiner_X — June 15, 2026 — 11:59 PM   ║
╠══════════════════════════════════════════════════════╣
║                                                      ║
║  After 30 years, I finally understand what           ║
║  Morimoto was hiding. It was never in one place.     ║
║  It was spread across everything he built.           ║
║                                                      ║
║  The answer words you found. Each one has a          ║
║  number — its position in the archive.               ║
║  That number points to one letter in the word.       ║
║  That letter IS a button.                            ║
║                                                      ║
║  Read them in order. Act I first, then Act II.       ║
║  The code is the last thing he wrote.                ║
║                                                      ║
║  Enter it on the title screen.                       ║
║                                                      ║
║  Time remaining: 00:00:01                            ║
╚══════════════════════════════════════════════════════╝
```

### Extraction (ROT13-encoded)

| # | Word (ROT13) | Position | Letter (ROT13) | Button |
|---|-------------|----------|----------------|--------|
| 1 | HZOEN | 1 | H | (decode) |
| 2 | DHRYY | 2 | H | (decode) |
| 3 | BEQRE | 3 | Q | (decode) |
| 4 | FUNQR | 4 | Q | (decode) |
| 5 | NAIVY | 5 | Y | (decode) |
| 6 | ERNYZ | 1 | E | (decode) |
| 7 | YBGHF | 1 | Y | (decode) |
| 8 | BEOVG | 2 | E | (decode) |
| 9 | RZORE | 3 | O | (decode) |
| 10 | TYRNZ | 4 | N | (decode) |
| 11 | URYVK | 5 | K | (decode) |
| 12 | YRTVG | 1 | Y | (decode) |

The 12 decoded letters form the cheat code.

### The Aha

The first 8 buttons of the code will be immediately recognizable to anyone who has ever played a video game. The solver who sees the pattern will feel a rush of recognition — then notice the last 4 buttons are different. That difference IS Morimoto's signature. The code is a love letter to gaming, with the developer's own twist.

### Verification

The code can be verified against the save file system: entering the correct code on the title screen (with game_complete=0x01 and secret_boss_beaten=0x01) flips the True Ending flag from 0x00 to 0x01.

---

## Summary

| Meta | Answer (ROT13) | Mechanism | Aha |
|------|----------------|-----------|-----|
| META-I | ERNYZ | Archive edit counts → extraction positions, read by rank | The index IS the puzzle |
| META-II | YRTVG | Secret discovery markers highlight one letter per word | The markers spell a word |
| SUPER | (12-button code) | Slot number → letter position → button abbreviation | The Konami code... but different |
