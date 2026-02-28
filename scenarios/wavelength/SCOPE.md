# WAVELENGTH — Scope

## Identity

| Field | Value |
|-------|-------|
| **Name** | WAVELENGTH |
| **Domain** | Real-world pop/rock music (1970s-1990s) |
| **Scale** | 6 puzzles + 1 meta, 1 round |
| **Audience** | Music lovers, casual puzzlers, teams of 2-4 |
| **Format** | Website + 1 physical prop |
| **Tone** | Bittersweet. Late-night radio. The warmth of something ending well. |

## Frame

The local independent radio station WVLG is going dark after 40 years on the air. The DJ's final broadcast is a puzzle. Hidden in the last playlist is a message she has been encoding since 1983. You have until the station signs off to decode it.

## Narrative Voice

The DJ — known only as "Vee" — narrates through handwritten playlist notes, on-air dedications, and sticky notes left in the booth. Her voice:
- No exclamation marks.
- Short sentences (rarely > 15 words).
- Past tense for memories, present tense for the broadcast.
- No meta-commentary.
- Warm but restrained. She knows this is the last time.
- The solver is "you" — the listener tuning in one final time.

## Core Systems

1. **Artists** — band names, members, active years, genres
2. **Albums** — titles, track listings, release years, labels
3. **Charts** — Billboard Hot 100 positions for specific songs in specific weeks
4. **Music Theory** — note names (A-G), intervals, key signatures, time signatures

## Content Sources

All facts must be verifiable via:
- Billboard chart archives
- Discogs / AllMusic for track listings and release years
- Standard music theory references

Flag anything uncertain with [VERIFY]. Do not invent chart positions or track listings.

## Answer Security

ROT13 encoding for all answer words. Encoded answers stored in scenarios/wavelength/ANSWERS.md. No plaintext answers in any other committed file.

## Meta Target

The DJ's final broadcast message. A phrase Vee has been hiding in playlists for 40 years. Emerges from the 6 feeder answer words.

## Physical Props

1. **The DJ's Final Playlist** — a handwritten playlist card on aged paper. Contains the full set list for the final broadcast. Teams receive this as a physical handout. It is the meta puzzle.

## Puzzle Types (Target)

| Slot | Type | Domain |
|------|------|--------|
| P1 | Track listing extraction | Song titles from real albums |
| P2 | Music theory encoding | Notes spell something |
| P3 | Album identification | Deduce albums from descriptions |
| P4 | Chart position patterns | Numbers encode letters |
| P5 | Lyrics / hidden words | Words hidden in real lyrics |
| P6 | Artist name wordplay | Anagrams or patterns in band names |
| META | The final broadcast | All 6 answers converge |
