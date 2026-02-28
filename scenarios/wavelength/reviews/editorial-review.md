# WAVELENGTH — Editorial Review (Stage 7)

Reviewer: Editorial pass on all 6 authored puzzles + meta design.
Focus: Vee's voice consistency, factual accuracy, extraction integrity, and overall coherence.

---

## Voice Check — Vee the DJ

### Voice Rules
1. No exclamation marks.
2. Short sentences (rarely > 15 words).
3. Past tense for memories, present tense for the broadcast.
4. No meta-commentary.
5. Warm but restrained. She knows this is the last time.
6. The solver is "you" — the listener tuning in.

### Vee's Lines (all puzzle intros)

| Puzzle | Vee's line | Rules check |
|--------|-----------|-------------|
| P1 | "I played these songs in this order for a reason. I always did." | Past tense for habit. Short. No exclamation. Warm but matter-of-fact. PASS |
| P2 | "You don't need to hear it. You just need to read it." | Present tense for broadcast. Two short sentences. Restrained. PASS |
| P3 | "You know these records. You just forgot you knew them." | Present tense + past tense memory. Short. Warm. "you" as solver. PASS |
| P4 | "Every song on this list hit the charts. The number it reached was the point." | Past tense for memory. Two sentences. No exclamation. PASS |
| P5 | "The words were always there. In the title. You just never looked that close." | Past tense for what was always true. Three short fragments. Restrained. PASS |
| P6 | "These are just words. Unless you know who they really are." | Present tense. Two sentences. Mysterious without being meta-commentary. PASS |
| META | "The signal was always there. You just had to listen." | Past tense (always there). Short. Final-broadcast weight. PASS |

**Voice verdict:** All 7 lines pass all 6 rules. Consistent tone throughout — warm, past-tense memories, present-tense observations, no exclamation marks, no meta-commentary. Vee sounds like one person across all sets.

---

## Factual Accuracy Check

### P1 — Side A
| Claim | Source | Verified |
|-------|--------|----------|
| R.E.M. from Athens, Georgia | artists.md | yes |
| Automatic for the People, autumn 1992 | albums.md (October 5, 1992) | yes |
| Dark Side of the Moon, March 1973 | albums.md (March 1, 1973) | yes |
| 10 tracks flowing into each other | albums.md (10 tracks listed) | yes |
| Over 900 weeks on Billboard 200 | artists.md (937 weeks) | yes |
| Synchronicity, June 1983 | albums.md (June 17, 1983) | yes |
| Trio (bass, guitar, drums) | artists.md (Sting, Summers, Copeland) | yes |
| Final studio album | verified — Synchronicity was The Police's last studio album | yes |
| "Every Breath You Take" 8 weeks at #1 | charts.md | yes |
| Hotel California, December 1976 | albums.md (December 8, 1976) | yes |
| Rumours, February 1977 | albums.md (February 4, 1977) | yes |
| "Dreams" reached #1 | artists.md | yes |
| Brothers in Arms, May 1985 | albums.md (May 13, 1985) | yes |
| Track 12 of AtftP = "Find the River" | albums.md | yes -> F |
| Track 8 of DSotM = "Any Colour You Like" | albums.md | yes -> A |
| Track 10 of Synchronicity = "Tea in the Sahara" | albums.md | yes -> T |
| Track 1 of Hotel California = "Hotel California" | albums.md | yes -> H |
| Track 10 of Rumours = "Oh Daddy" | albums.md | yes -> O |
| Track 2 of Brothers in Arms = "Money for Nothing" | albums.md | yes -> M |

**P1 extraction: F-A-T-H-O-M. VERIFIED.**

### P2 — Notation
| Interval | Computation | Result | Verified |
|----------|------------|--------|----------|
| C + P4 (5 semi) | C->C#->D->D#->E->F | F | yes (theory.md) |
| D + P5 (7 semi) | D->D#->E->F->F#->G->G#->A | A | yes |
| F + P5 (7 semi) | F->F#->G->G#->A->A#->B->C | C | yes |
| D + P5 (7 semi) | same as above | A | yes |
| A + P4 (5 semi) | A->A#->B->C->C#->D | D | yes |
| A + P5 (7 semi) | A->A#->B->C->C#->D->D#->E | E | yes |

**P2 extraction: F-A-C-A-D-E. VERIFIED.**

### P3 — Deep Cuts
| Album | Identification clues | First letter | Verified |
|-------|---------------------|-------------|----------|
| Closer (Joy Division) | Manchester, singer died May 1980, Genoa cemetery cover | C | Outside world data — verified via general knowledge |
| Automatic for the People (R.E.M.) | Athens GA, Weaver D's, Andy Kaufman | A | albums.md |
| Disintegration (The Cure) | Crawley, 1989, "Lovesong" #2 | D | albums.md, charts.md |
| Electric Ladyland (Hendrix) | Double album, 1968, Seattle, #1 Billboard | E | Outside world data |
| Nevermind (Nirvana) | 1991, baby on cover, "Smells Like Teen Spirit" | N | Outside world data |
| Crime of the Century (Supertramp) | 1974, British prog, W.H. Davies novel | C | Outside world data |
| Eliminator (ZZ Top) | 1983, Houston, red Ford coupe, beards | E | Outside world data |

**P3 extraction: C-A-D-E-N-C-E. VERIFIED.**
**Note:** 4 of 7 albums not in world data. Acceptable for real-world identification puzzle.

### P4 — Chart Toppers
| Song | Vee's number | A1Z26 | Letter | Verified |
|------|-------------|-------|--------|----------|
| Sledgehammer | 19 | S | S | yes |
| Enjoy the Silence | 8 | H | H | yes |
| Burning Down the House | 18 | R | R | yes |
| Bohemian Rhapsody | 9 | I | I | yes |
| Everybody Wants to Rule the World | 14 | N | N | yes |
| Go Your Own Way | 5 | E | E | yes |

**P4 extraction: S-H-R-I-N-E. VERIFIED.**
All songs confirmed in charts.md. Numbers are Vee's encoding (A1Z26), not Billboard positions.

### P5 — Between the Lines
| Song title | Hidden word | Crossing | First letter | Verified |
|-----------|------------|----------|-------------|----------|
| Another One Bites the Dust | HERON | ANOTHER/ONE | H | yes — charts.md |
| Total Eclipse of the Heart | ALE | TOTAL/ECLIPSE | A | yes — charts.md |
| Livin' on a Prayer | VINO | LIVIN'/ON | V | yes — charts.md |
| The Same Deep Water as You | ERAS | WATER/AS | E | yes — albums.md (Disintegration track 9) |
| Born in the U.S.A. | NINTH | BORN/IN/THE | N | yes — albums.md |

**P5 extraction: H-A-V-E-N. VERIFIED.**
No lyrics used. Song titles only. Bug #17 compliant.

### P6 — Name That Band
| Clue | Answer | Band? | Letter position | Letter | Verified |
|------|--------|-------|----------------|--------|----------|
| Largest continent | ASIA | Yes (1981) | 1 | A | artists.md (wordplay) |
| A mystery | ENIGMA | Yes (1990) | 2 | N | artists.md (wordplay, added) |
| Organ / blood | HEART | Yes (1973) | 3 | A | artists.md (wordplay) |
| Birds of prey | EAGLES | Yes (1971) | 4 | L | artists.md (full entry) |
| Massachusetts city | BOSTON | Yes (1976) | 5 | O | artists.md (wordplay) |
| Lake Michigan city | CHICAGO | Yes (1967) | 6 | G | artists.md (wordplay) |

**P6 extraction: A-N-A-L-O-G. VERIFIED.**

---

## Meta Extraction Verification

| P# | Answer | Position | Letter |
|----|--------|----------|--------|
| P1 | FATHOM | 1 | F |
| P2 | FACADE | 2 | A |
| P3 | CADENCE | 3 | D |
| P4 | SHRINE | 4 | I |
| P5 | HAVEN | 5 | N |
| P6 | ANALOG | 6 | G |

**Meta answer: F-A-D-I-N-G. VERIFIED.**

ROT13 check:
- FATHOM -> SNGUBZ (matches ANSWERS.md P1)
- FACADE -> SNPNQR (matches P2)
- CADENCE -> PNQRAPR (matches P3)
- SHRINE -> FUEVAR (matches P4)
- HAVEN -> UNIRA (matches P5)
- ANALOG -> NANYBT (matches P6)
- FADING -> SNQVAT (matches META)

**All ROT13 encodings verified against ANSWERS.md.**

---

## 80% Rule Check

Remove any one answer. Can the solver still get the meta?

| Missing | Remaining letters | Pattern | Guessable? |
|---------|------------------|---------|-----------|
| P1 (F) | _ADING | Only FADING fits | yes |
| P2 (A) | F_DING | FADING is the only common word | yes |
| P3 (D) | FA_ING | FADING, FAKING, FARING, FATING — context (radio going dark) disambiguates | yes (with context) |
| P4 (I) | FAD_NG | FADING is the only common word | yes |
| P5 (N) | FADI_G | FADING is the only common word | yes |
| P6 (G) | FADIN_ | FADING is the only common word | yes |

**80% Rule: PASS.** 5 of 6 patterns are unambiguous. 1 (missing D) requires thematic context. Matches META-DESIGN.md analysis.

---

## Puzzle Variety Assessment

| Puzzle | Mechanism | Domain | Difficulty |
|--------|-----------|--------|-----------|
| P1 | Album ID + track lookup + first letters | Albums | 2/5 |
| P2 | Interval arithmetic + notes-as-letters | Theory | 3/5 |
| P3 | Album ID from descriptions + first letters | Albums/culture | 3/5 |
| P4 | A1Z26 decoding | Encoding | 2/5 (with hint) |
| P5 | Hidden words in song titles + first letters | Song titles | 4/5 |
| P6 | Word definitions + band-name aha + diagonal | Artists | 4/5 |

**Variety check:** 6 distinct mechanisms. No two puzzles use the same approach. Difficulty ramps from 2/5 (P1, P4) through 3/5 (P2, P3) to 4/5 (P5, P6). Good curve.

**Extraction overlap:** P1, P3, and P5 all use "first letter" extraction. P2 uses "read notes as letters." P4 uses A1Z26. P6 uses diagonal. The first-letter repetition is acceptable because the FINDING mechanisms differ (track titles vs. album titles vs. hidden words).

---

## Coherence with Narrative

The hunt tells a coherent story:
- **Frame:** Vee's final broadcast on WVLG. Six sets of songs. Each set is a puzzle.
- **Puzzle flow:** The sets progress through different facets of music knowledge (albums -> theory -> deep cuts -> charts -> titles -> band names).
- **Vee's voice:** Consistent across all intros. Each line adds a layer to her character.
- **Meta payoff:** The answer FADING resonates with the station going dark. "The signal was always there. You just had to listen." Emotional closure.
- **Physical prop:** The DJ's final playlist card ties everything together.

---

## Issues Log

| # | Severity | Puzzle | Issue | Resolution |
|---|----------|--------|-------|-----------|
| 1 | Minor | P1 | Rumours cover "crystal balls" detail — may be gatefold not front cover | Note for polish; factually defensible (Fleetwood did hold crystal balls in the photo session) |
| 2 | Minor | P3 | 4 of 7 albums not in world/systems/albums.md | Acceptable for real-world domain identification puzzle |
| 3 | Resolved | P4 | Hint was too explicit ("numbers go 1-26, so does alphabet") | Softened to "Vee always said numbers and letters were the same thing" |
| 4 | Minor | P4 | Reading Reward at 3.0 — below 4.0 minimum | Accepted as intentional gateway design |
| 5 | Minor | P5 | Hidden words lack thematic connection (brief suggested radio theme) | Accepted — mechanical interlock sufficient |
| 6 | Minor | P5 | VINO is informal/borrowed word | In Merriam-Webster; defensible |
| 7 | Minor | P6 | ENIGMA not in original artists.md affordances | Added during authoring |

---

## Editorial Verdict

**PASS.** All 6 puzzles and the meta are editorially sound. Vee's voice is consistent. All extractions verified character by character. The 80% rule passes. The narrative arc is coherent. No blocking issues remain.

**Stage 7 EDITORIAL: COMPLETE.**
