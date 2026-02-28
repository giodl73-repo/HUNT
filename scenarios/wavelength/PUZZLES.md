# WAVELENGTH — Puzzle Registry

Master registry for all puzzles. Answer words are ROT13-encoded (see ANSWERS.md).

---

## Answer Coordination (Principle #20)

All 6 answer words were coordinated before assignment. The meta extraction takes the Nth letter from the Nth answer word (diagonal read). This spells the meta answer.

| P# | Answer (ROT13) | Length | Meta position | Meta letter |
|----|-----------------|--------|---------------|-------------|
| P1 | SNGUBZ | 6 | 1st | (ROT13: S) |
| P2 | SNPNQR | 6 | 2nd | (ROT13: N) |
| P3 | PNQRAPR | 7 | 3rd | (ROT13: Q) |
| P4 | FUEVAR | 6 | 4th | (ROT13: V) |
| P5 | UNIRA | 5 | 5th | (ROT13: A) |
| P6 | NANYBT | 6 | 6th | (ROT13: T) |
| META | SNQVAT | 6 | — | — |

---

## Puzzle Briefs

### P1 — Side A
**Type:** Track listing extraction
**Mechanism:** 6 famous albums are presented (clued, not named). For each album, the solver takes the Nth letter of track N's title (letter 1 from track 1, letter 2 from track 2, etc., where N = the album's order in the puzzle). This yields one letter per album. The 6 letters spell the answer.
**Aha:** The track number tells you which letter position to extract.
**Answer:** (ROT13) SNGUBZ — 6 letters
**Difficulty:** 2/5
**Domain:** world/systems/albums.md
**Vee's note:** *"I played these songs in this order for a reason. I always did."*
**Author:** Patrick Berry
**Interlock:** Albums are all from the WVLG era (1975-1992). Identifying one helps narrow the era for others.

---

### P2 — Notation
**Type:** Music theory encoding
**Mechanism:** 6 musical intervals are described in words (e.g., "a perfect fifth above D"). The solver must compute each target note. The 6 note names, read in order, spell a word using only the letters A-G.
**Aha:** The computed notes ARE the answer — musical notes are letters.
**Answer:** (ROT13) SNPNQR — 6 letters
**Special:** The answer word ITSELF can be spelled entirely with musical note names (A-G). This is a double payoff — the puzzle mechanism uses music theory, and the answer IS a musical word.
**Difficulty:** 3/5
**Domain:** world/systems/theory.md
**Vee's note:** *"You don't need to hear it. You just need to read it."*
**Author:** Mike Selinker
**Deduction layer:** The solver must apply interval arithmetic (not just read notes off a staff). Some intervals are ambiguous without key context, requiring cross-reference.

---

### P3 — Deep Cuts
**Type:** Album identification (deduction)
**Mechanism:** 7 oblique descriptions of famous albums. Each description covers the album's themes, sound, era, and cultural context — but never names the album or artist. The solver identifies each album. The first letters of the 7 album titles, in order, spell the answer.
**Aha:** Recognizing the albums from cryptic descriptions, then reading the first letters.
**Answer:** (ROT13) PNQRAPR — 7 letters
**Difficulty:** 3/5
**Domain:** world/systems/albums.md, world/systems/artists.md
**Vee's note:** *"You know these records. You just forgot you knew them."*
**Author:** Lucas Pope
**Interlock:** Descriptions reference each other obliquely (e.g., "released the same year as Description 4's album"). Solving one constrains others.

---

### P4 — Chart Toppers
**Type:** Chart position to letter encoding
**Mechanism:** 6 songs are listed (by title and artist). The solver looks up each song's Billboard Hot 100 peak position. Each position corresponds to a letter (A=1, B=2, ... Z=26). The 6 letters spell the answer.
**Aha:** The chart positions are not trivia — they encode letters.
**Answer:** (ROT13) FUEVAR — 6 letters
**Difficulty:** 3/5
**Domain:** world/systems/charts.md
**Vee's note:** *"Every song on this list hit the charts. The number it reached was the point."*
**Author:** Patrick Berry
**Verification note:** All peak positions must be confirmed against Billboard archives. Some entries in charts.md are flagged [VERIFY].

---

### P5 — Between the Lines
**Type:** Lyrics hidden words
**Mechanism:** 5 famous song titles are presented (title and artist only — no lyrics quoted). Each title contains a hidden English word spanning across word boundaries in the title text (e.g., "EVERY BREATH YOU TAKE" contains hidden word "VERY"). The solver finds the target hidden word in each title. The first letters of the 5 hidden words spell the answer.
**Aha:** Familiar song titles contain words you never noticed hiding inside them.
**Answer:** (ROT13) UNIRA — 5 letters
**Difficulty:** 4/5
**Domain:** world/systems/albums.md, world/systems/artists.md (titles only — no lyrics)
**Copyright rule:** Song titles are not copyrightable expression. No lyrics appear in this puzzle.
**Vee's note:** *"The words were always there. In the title. You just never looked that close."*
**Author:** Mike Selinker
**Interlock:** The hidden words are all thematically related (they describe aspects of radio/communication), which helps the solver confirm they found the right hidden word.

---

### P6 — Name That Band
**Type:** Artist name wordplay
**Mechanism:** 6 clues each define a common English word. Each answer is also the name of a real band/artist. The solver writes the 6 words, then extracts the Nth letter from the Nth word (clue 1 → letter 1, clue 2 → letter 2, etc.). The extracted letters spell the answer.
**Aha:** Every clue answer is a band name. The extraction is diagonal.
**Answer:** (ROT13) NANYBT — 6 letters
**Difficulty:** 4/5
**Domain:** world/systems/artists.md
**Vee's note:** *"These are just words. Unless you know who they really are."*
**Author:** Lucas Pope
**Deduction layer:** Some clues have multiple possible answers (e.g., "a continent" could be ASIA or EUROPE). The solver must try both and see which produces a valid letter.

---

### META — Sign Off
**Type:** Diagonal extraction from feeder answers
**Mechanism:** The solver writes the 6 feeder answer words in order (P1 through P6). Taking the Nth letter from the Nth word (letter 1 from P1, letter 2 from P2, etc.) spells the meta answer.
**Presentation:** The meta is the DJ's final playlist card — the physical prop. The card lists all songs played that night (the 6 puzzle sets). Below the song list, there are 6 numbered blanks. Above each blank is a small number indicating "which letter." The solver fills in the answer words and reads the diagonal.
**Answer:** (ROT13) SNQVAT — 6 letters
**80% Rule:** With 5 of 6 answers, the solver has 5 of 6 letters. A single missing letter in a 6-letter common English word is often guessable (especially when the frame suggests radio/broadcasting themes).
**Vee's final words:** "The signal was always there. You just had to listen."

---

## Author Assignments

| Author Personality | Puzzles | Fit |
|-------------------|---------|-----|
| **Patrick Berry** | P1 (Side A), P4 (Chart Toppers) | Berry's precision with language and clean extraction mechanisms. Track listing and chart data puzzles need meticulous construction — every letter must be verified. |
| **Mike Selinker** | P2 (Notation), P5 (Between the Lines) | Selinker's narrative instinct and experience design. The music theory puzzle needs to FEEL like discovering a secret, not doing homework. The lyrics puzzle is inherently narrative. |
| **Lucas Pope** | P3 (Deep Cuts), P6 (Name That Band) | Pope's deduction and identification expertise. Album ID from oblique descriptions is pure Obra Dinn — lateral information, multiple paths to each answer. Band name wordplay requires recognizing hidden identities. |

---

## Status Tracker

| ID | Brief | Authored | Editorial | Tested | Final |
|----|-------|----------|-----------|--------|-------|
| P1 | done | -- | -- | -- | -- |
| P2 | done | -- | -- | -- | -- |
| P3 | done | -- | -- | -- | -- |
| P4 | done | -- | -- | -- | -- |
| P5 | done | -- | -- | -- | -- |
| P6 | done | -- | -- | -- | -- |
| META | done | -- | -- | -- | -- |
