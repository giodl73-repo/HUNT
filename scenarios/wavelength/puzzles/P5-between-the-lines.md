# Set 5 — Between the Lines

*"The words were always there. In the title. You just never looked that close."*

---

Vee had a game she played during the midnight shift. She would stare at the song titles on her playlist cards until she saw other words hiding inside them. Words that had always been there, stretched across the spaces between the printed words, invisible until you knew to look.

Tonight, she played five songs. She circled something in each title on her card. Not the song. Not the artist. Something else. Something hidden.

Find the hidden word in each song title. Take the first letter of each hidden word. Read them in order.

---

## The Five Songs

**1.**
"Another One Bites the Dust"
*Queen (1980)*

**2.**
"Total Eclipse of the Heart"
*Bonnie Tyler (1983)*

**3.**
"Livin' on a Prayer"
*Bon Jovi (1987)*

**4.**
"The Same Deep Water as You"
*The Cure (1989)*

**5.**
"Born in the U.S.A."
*Bruce Springsteen (1984)*

---

## How to Find Hidden Words

A hidden word spans across word boundaries in the title. Ignore the spaces between words and look for an English word hiding in the run of letters.

Example: In the title "Every Breath You Take," ignore the spaces and read the letters in sequence: E-V-E-R-Y-B-R-E-A-T-H-Y-O-U-T-A-K-E. The word **VERY** is hiding inside — the last four letters of EVERY.

The hidden words in the five songs above are each at least three letters long and cross at least one word boundary.

---

## Workspace

| # | Song title | Hidden word | First letter |
|---|-----------|-------------|-------------|
| 1 | Another One Bites the Dust | _____________ | ___ |
| 2 | Total Eclipse of the Heart | _____________ | ___ |
| 3 | Livin' on a Prayer | _____________ | ___ |
| 4 | The Same Deep Water as You | _____________ | ___ |
| 5 | Born in the U.S.A. | _____________ | ___ |

**Your answer** (5 letters): _ _ _ _ _

---

*Vee always said the best things were hidden in plain sight.*

---

## Author's Working Notes (not solver-facing)

**Author: Mike Selinker**

Hidden word extraction (letter by letter):

1. "Another One Bites the Dust"
   A-N-O-T-H-E-R | O-N-E | B-I-T-E-S | T-H-E | D-U-S-T
   Hidden: HERON — anotHER ONe — H(5),E(6),R(7) from ANOTHER + O(1),N(2) from ONE
   First letter: **H**

2. "Total Eclipse of the Heart"
   T-O-T-A-L | E-C-L-I-P-S-E | O-F | T-H-E | H-E-A-R-T
   Hidden: ALE — totAL Eclipse — A(4),L(5) from TOTAL + E(1) from ECLIPSE
   First letter: **A**

3. "Livin' on a Prayer"
   L-I-V-I-N | O-N | A | P-R-A-Y-E-R
   Hidden: VINO — liVIN' On — V(3),I(4),N(5) from LIVIN' + O(1) from ON
   First letter: **V**

4. "The Same Deep Water as You"
   T-H-E | S-A-M-E | D-E-E-P | W-A-T-E-R | A-S | Y-O-U
   Hidden: ERAS — watER AS — E(4),R(5) from WATER + A(1),S(2) from AS
   First letter: **E**

5. "Born in the U.S.A."
   B-O-R-N | I-N | T-H-E | U-S-A
   Hidden: NINTH — borN IN THe — N(4) from BORN + I(1),N(2) from IN + T(1),H(2) from THE
   First letter: **N**

Answer: H-A-V-E-N = HAVEN

Meta extraction check: HAVEN, letter 5 = N. ROT13(N) = A. Matches ANSWERS.md position 5.

Principles check:
- Riven Standard: The puzzle IS about music — song titles are the medium, and the solver engages with iconic songs.
- Solving = Proving Understanding: The solver must know or recognize these song titles to parse them correctly.
- Blame the Player: The hidden words are clearly present in the letter sequences. Fair in retrospect.
- No Over-Scaffolding: The workspace has blanks, but the solver must find the words themselves.
- Surprise the Answer: HAVEN is not guessable from "hidden words in song titles."
- One Aha: Seeing letters across word boundaries as new words. After the first one clicks, the rest follow the same logic.
- No Deliberate Errors: All song titles and artists are verified against world data (charts.md, albums.md, artists.md).
- Interlock: Finding one hidden word teaches the solver the technique. HERON (5 letters, obvious crossing) is the gateway. NINTH (spanning 3 words) is the hardest.
- Bug #17 compliance: NO LYRICS. Only song titles and artist names appear. Song titles are not copyrightable expression.

Difficulty notes:
- HERON is moderate — the "HERONE" sequence is noticeable once you ignore spaces.
- ALE is easy — short word, obvious crossing.
- VINO is moderate — solver must know the word VINO (wine, informal).
- ERAS is moderate — the ER/AS crossing is clean.
- NINTH is hard — spans three words (BORN/IN/THE), requires patience.
- Overall difficulty: 4/5 as specified in the brief.
