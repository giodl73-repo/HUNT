# WAVELENGTH — Puzzle Pool

Brainstorm of 12 puzzle candidates for 6 slots. Panel ranks to select the final 6.

---

## Candidates

### Track Listing Puzzles (for P1 slot)

**T1: "Side A" — Nth Letter Extraction** (recommended)
- Mechanism: Given 6 well-known albums, take the Nth letter of the Nth track title (letter 1 of track 1, letter 2 of track 2, etc.)
- Domain: Album track listings from world/systems/albums.md
- Aha: Realizing the track number tells you which letter to extract
- Difficulty: 2/5
- Dinner Party: "I spelled a word using track listings from classic albums"

**T2: "Vinyl" — First Letter Acrostic**
- Mechanism: The first letters of consecutive tracks on a single album spell a word
- Domain: One album's track listing
- Weakness: Limited to albums where first letters happen to spell something. Constrains album choice severely.
- Difficulty: 1/5 (too easy once identified)

### Music Theory Puzzles (for P2 slot)

**T3: "Notation" — Notes Spell a Word** (recommended)
- Mechanism: A sequence of musical notes on a staff, when read as letter names (A-G), spells a word
- Domain: Music theory from world/systems/theory.md
- Aha: Realizing the notes ARE letters (A through G)
- Constraint: Answer must use only letters A-G. See theory.md for list of valid words.
- Difficulty: 3/5
- Dinner Party: "The answer was literally written in music"

**T4: "Key Change" — Key Signature Encoding**
- Mechanism: Each key signature has a specific number of sharps or flats. Convert count to letter.
- Domain: Key signatures
- Weakness: Pure computation (Principle #16). No deduction layer.
- Difficulty: 2/5

### Album Identification Puzzles (for P3 slot)

**T5: "Deep Cuts" — Identify Albums from Oblique Descriptions** (recommended)
- Mechanism: 6 descriptions of famous albums (describing artwork, track themes, cultural impact) without naming the album or artist. Solver identifies each album, then extracts a letter from each album title at a given position.
- Domain: Album knowledge from world/systems/albums.md
- Aha: Recognizing the album from its description
- Interlock: Knowing one album may help narrow others (all from the same era, or same station format)
- Difficulty: 3/5
- Dinner Party: "I identified 6 classic albums from cryptic descriptions"

**T6: "Album Art" — Visual Identification**
- Mechanism: Describe album cover artwork in words; solver identifies the album
- Weakness: Relies on visual memory. Harder to fact-check from text alone.
- Difficulty: 3/5

### Chart Position Puzzles (for P4 slot)

**T7: "Chart Toppers" — Peak Position to Letter** (recommended)
- Mechanism: Given 6 songs, look up each song's Billboard Hot 100 peak position. Convert position to letter (A=1, B=2, ... Z=26).
- Domain: Chart data from world/systems/charts.md
- Aha: Realizing the chart positions spell something
- Deduction layer: Some songs are well-known enough that their peak position is common knowledge; others require the solver to consult the world data or their own music knowledge.
- Difficulty: 3/5
- Dinner Party: "Billboard chart positions literally spelled out a word"

**T8: "Year End" — Year of Release Encoding**
- Mechanism: Songs from specific years; extract digits from years
- Weakness: Very limited letter range (years 1965-1999 give digits 1-9, missing most of the alphabet). Clunky.
- Difficulty: 2/5

### Lyrics Puzzles (for P5 slot)

**T9: "Between the Lines" — Hidden Words in Lyrics** (recommended)
- Mechanism: Famous lyric lines each contain a hidden word spanning across word boundaries. Solvers find the hidden word in each lyric, then take the first letter of each hidden word to spell the answer.
- Domain: Lyrics from world/systems/lyrics.md
- Aha: Spotting the hidden words in familiar lyrics
- Interlock: The answer's first letter constrains what hidden word to look for in each line
- Difficulty: 4/5
- Dinner Party: "There were secret words hiding in song lyrics I've heard a thousand times"

**T10: "First Words" — Extract First/Last Words of Lyrics**
- Mechanism: Take the first word of famous opening lyrics
- Weakness: Too easy. No deduction. Just lookup.
- Difficulty: 1/5

### Artist Name Puzzles (for P6 slot)

**T11: "Name That Band" — Artist Names Contain Common Words** (recommended)
- Mechanism: Clues describe common English words (not the bands). Each answer word is also the name of a band/artist. The solver must realize the clue answers are band names. Extract a specific letter from each band name.
- Example: "A remedy or treatment" = CURE (The Cure). "A female monarch" = QUEEN (Queen).
- Domain: Artist wordplay affordances from world/systems/artists.md
- Aha: Realizing every answer is also a band name
- Difficulty: 4/5
- Dinner Party: "Every answer to the clues was secretly a band name"

**T12: "Anagram" — Anagrammed Artist Names**
- Mechanism: Anagram band names into other words/phrases
- Weakness: Many band names are too short or have difficult letter combos. Feels arbitrary.
- Difficulty: 3/5

---

## Panel Ranking

### Locks (selected for final 6)

| Slot | Candidate | Votes | Notes |
|------|-----------|-------|-------|
| P1 | T1: "Side A" | 10/12 | Clean mechanism. Real album data. Good entry point. |
| P2 | T3: "Notation" | 11/12 | Riven Standard — the puzzle IS music theory. Beautiful constraint (A-G only). |
| P3 | T5: "Deep Cuts" | 9/12 | Deduction + identification. Album descriptions are fun to write and solve. |
| P4 | T7: "Chart Toppers" | 9/12 | Requires verified chart data. Solver learns real chart history. |
| P5 | T9: "Between the Lines" | 10/12 | Hidden words in lyrics is a classic puzzle type applied to a perfect domain. |
| P6 | T11: "Name That Band" | 11/12 | Selinker: "This is the dinner party puzzle. Every clue has a double meaning." |

### Cut

| Candidate | Votes | Why cut |
|-----------|-------|---------|
| T2 | 2/12 | Too constrained and too easy |
| T4 | 3/12 | Pure computation, no deduction |
| T6 | 5/12 | Hard to verify from text; better as a visual puzzle |
| T8 | 1/12 | Clunky encoding, limited range |
| T10 | 2/12 | No deduction, just lookup |
| T12 | 4/12 | Feels arbitrary. T11 is strictly better. |

---

## Scale Observation

**Bug noted:** For 6 puzzles, brainstorming 12 candidates (2 per slot) feels proportional. At 5+ puzzles, the pool step adds real value — bad candidates were cut. But the formal "panel ranks" step feels like ceremony for this scale. A simple "brainstorm 2 per slot, pick the better one" would suffice for hunts under 8 puzzles. Logged in BUGS.md.
