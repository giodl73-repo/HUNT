# WAVELENGTH — Integration Check (Stage 8)

Full end-to-end verification: every puzzle extraction, every meta feed, every answer encoding.

---

## 1. Feeder Answer Verification

### P1 — Side A
```
Album 1: Automatic for the People -> Track 12 -> "Find the River" -> F
Album 2: The Dark Side of the Moon -> Track 8 -> "Any Colour You Like" -> A
Album 3: Synchronicity -> Track 10 -> "Tea in the Sahara" -> T
Album 4: Hotel California -> Track 1 -> "Hotel California" -> H
Album 5: Rumours -> Track 10 -> "Oh Daddy" -> O
Album 6: Brothers in Arms -> Track 2 -> "Money for Nothing" -> M

ANSWER: F-A-T-H-O-M = FATHOM (6 letters)
ROT13: SNGUBZ
ANSWERS.md P1: SNGUBZ -- MATCH
```

### P2 — Notation
```
1. C + Perfect 4th (5 semitones) = F
2. D + Perfect 5th (7 semitones) = A
3. F + Perfect 5th (7 semitones) = C
4. D + Perfect 5th (7 semitones) = A
5. A + Perfect 4th (5 semitones) = D
6. A + Perfect 5th (7 semitones) = E

ANSWER: F-A-C-A-D-E = FACADE (6 letters)
ROT13: SNPNQR
ANSWERS.md P2: SNPNQR -- MATCH
```

### P3 — Deep Cuts
```
I.   Closer (Joy Division) -> C
II.  Automatic for the People (R.E.M.) -> A
III. Disintegration (The Cure) -> D
IV.  Electric Ladyland (Jimi Hendrix) -> E
V.   Nevermind (Nirvana) -> N
VI.  Crime of the Century (Supertramp) -> C
VII. Eliminator (ZZ Top) -> E

ANSWER: C-A-D-E-N-C-E = CADENCE (7 letters)
ROT13: PNQRAPR
ANSWERS.md P3: PNQRAPR -- MATCH
```

### P4 — Chart Toppers
```
Sledgehammer = 19 = S
Enjoy the Silence = 8 = H
Burning Down the House = 18 = R
Bohemian Rhapsody = 9 = I
Everybody Wants to Rule the World = 14 = N
Go Your Own Way = 5 = E

ANSWER: S-H-R-I-N-E = SHRINE (6 letters)
ROT13: FUEVAR
ANSWERS.md P4: FUEVAR -- MATCH
```

### P5 — Between the Lines
```
1. "Another One Bites the Dust" -> HERON -> H
2. "Total Eclipse of the Heart" -> ALE -> A
3. "Livin' on a Prayer" -> VINO -> V
4. "The Same Deep Water as You" -> ERAS -> E
5. "Born in the U.S.A." -> NINTH -> N

ANSWER: H-A-V-E-N = HAVEN (5 letters)
ROT13: UNIRA
ANSWERS.md P5: UNIRA -- MATCH
```

### P6 — Name That Band
```
1. ASIA -> letter 1 = A
2. ENIGMA -> letter 2 = N
3. HEART -> letter 3 = A
4. EAGLES -> letter 4 = L
5. BOSTON -> letter 5 = O
6. CHICAGO -> letter 6 = G

ANSWER: A-N-A-L-O-G = ANALOG (6 letters)
ROT13: NANYBT
ANSWERS.md P6: NANYBT -- MATCH
```

---

## 2. Meta Extraction Verification

```
Position:  1 2 3 4 5 6
           - - - - - -
P1: F A T H O M        -> position 1 = F
P2: F A C A D E        -> position 2 = A
P3: C A D E N C E      -> position 3 = D
P4: S H R I N E        -> position 4 = I
P5: H A V E N          -> position 5 = N
P6: A N A L O G        -> position 6 = G

META ANSWER: F-A-D-I-N-G = FADING (6 letters)
ROT13: SNQVAT
ANSWERS.md META: SNQVAT -- MATCH
```

Visual staircase:
```
[F] A  T  H  O  M
 F [A] C  A  D  E
 C  A [D] E  N  C  E
 S  H  R [I] N  E
 H  A  V  E [N]
 A  N  A  L  O [G]

Reading diagonal: F - A - D - I - N - G
```

---

## 3. Answer Length Consistency

| P# | Answer | Length | ANSWERS.md length | Match |
|----|--------|--------|-------------------|-------|
| P1 | FATHOM | 6 | 6 | yes |
| P2 | FACADE | 6 | 6 | yes |
| P3 | CADENCE | 7 | 7 | yes |
| P4 | SHRINE | 6 | 6 | yes |
| P5 | HAVEN | 5 | 5 | yes |
| P6 | ANALOG | 6 | 6 | yes |
| META | FADING | 6 | 6 | yes |

---

## 4. Cross-Puzzle Conflict Check

No two puzzles use the same source data in a way that creates spoilers:
- P1 uses 6 albums from albums.md (Automatic for the People, DSotM, Synchronicity, Hotel California, Rumours, Brothers in Arms)
- P3 uses 7 albums, only 1 overlap with P1 (Automatic for the People appears in both). P3's description of AtftP does not reveal track 12's title (used in P1). No spoiler.
- P2 uses music theory (intervals). No overlap with any other puzzle.
- P4 uses 6 songs from charts.md. One song ("Go Your Own Way") also appears in P1's album (Rumours, track 5). No spoiler — P4 doesn't reveal which album it's on.
- P5 uses 5 song titles from charts.md/albums.md. "Born in the U.S.A." is also an album in P1's era but not used in P1. No conflict.
- P6 uses band names. EAGLES and HEART appear in P6 as definitions. Eagles' album is in P1. No spoiler.

**Cross-puzzle conflicts: NONE.**

---

## 5. Meta Physical Prop Consistency

The META-DESIGN.md specifies a physical playlist card showing:
- 6 sets with songs
- 6 answer blanks with position markers (carets)
- "Now read down" instruction

Checking that each puzzle's content maps to a "set" on the card:
- Set 1 (P1): 6 albums -> list album names (or songs from each)
- Set 2 (P2): 6 intervals -> list musical notation
- Set 3 (P3): 7 albums -> list album descriptions
- Set 4 (P4): 6 songs -> list songs with numbers
- Set 5 (P5): 5 songs -> list song titles
- Set 6 (P6): 6 definitions -> list... hmm, the definitions aren't songs.

**Issue:** P6's definitions are about continents and cities, not songs. The playlist card should list SONGS for each set. P6 needs a framing adjustment for the physical prop — perhaps the songs played during Set 6 are songs BY the bands (Asia, Enigma, Heart, Eagles, Boston, Chicago), and the definitions are on a separate card.

**Resolution:** This is a delivery/presentation issue, not an extraction issue. The puzzle mechanism is sound. The prop can list representative songs from each band alongside the definitions. Flag for Stage 9 (DELIVERY BUILD).

---

## 6. Difficulty Curve

```
Intended:  P1(2) -> P2(3) -> P3(3) -> P4(2) -> P5(4) -> P6(4)
Tested:    P1(27) P2(29) P3(28) P4(25) P5(26) P6(29)

The tested scores show P4 as the weakest (25.3) and P6 as the strongest (29.0).
P4's lower score is due to Reading Reward (too easy), not difficulty.
P5 and P6 are both 4/5 difficulty but P6 tested higher due to elegance.
```

The curve works: two easy entry points (P1, P4), two medium challenges (P2, P3), two hard closers (P5, P6). Teams can start anywhere and build momentum.

---

## 7. No Plaintext Answers in Files

Checking all committed files for plaintext answer words:

- FATHOM appears in: P1 author's notes (solver-facing section says "_ _ _ _ _ _")
- FACADE appears in: P2 author's notes
- CADENCE appears in: P3 author's notes
- SHRINE appears in: P4 author's notes
- HAVEN appears in: P5 author's notes
- ANALOG appears in: P6 author's notes
- FADING appears in: META-DESIGN.md (encoded section only, ROT13)

**Issue:** Author's working notes in each puzzle file contain plaintext answers. These sections are marked "not solver-facing" but they ARE in the committed files.

**Resolution:** Before delivery (Stage 9), strip all "Author's Working Notes" sections from puzzle files. Create a separate `reviews/answer-key.md` (gitignored) for author reference. For now, the notes are needed for editorial/integration. Flag for Stage 9.

---

## 8. Bug #17 Compliance

Scanning all puzzle files for lyric content:
- P1: No lyrics. Album descriptions only.
- P2: No lyrics. Musical intervals only.
- P3: No lyrics. Album descriptions only.
- P4: No lyrics. Song titles and numbers only.
- P5: No lyrics. Song titles and artist names only. Song titles are not copyrightable.
- P6: No lyrics. Definitions of common English words only.

**Bug #17: FULLY COMPLIANT. Zero lyric content in any puzzle.**

---

## Integration Verdict

| Check | Status |
|-------|--------|
| All 6 feeder extractions verified | PASS |
| Meta extraction verified | PASS |
| ROT13 encodings match ANSWERS.md | PASS |
| Answer lengths consistent | PASS |
| Cross-puzzle conflicts | NONE |
| 80% Rule | PASS |
| Voice consistency | PASS (Stage 7) |
| Bug #17 (no lyrics) | PASS |
| Difficulty curve | PASS |
| Plaintext answer security | Flag for Stage 9 (strip author notes) |
| Meta prop consistency | Flag for Stage 9 (P6 framing on card) |

**Stage 8 INTEGRATION: COMPLETE.** All extractions verified. All answers match. Two items flagged for Stage 9 delivery (author note stripping, P6 prop framing).
