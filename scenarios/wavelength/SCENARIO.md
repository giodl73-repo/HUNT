# Scenario 4 — WAVELENGTH

**Type:** Real-world domain (music)
**Scale:** 6 puzzles + 1 meta
**Structure:** 1 round
**Pipeline:** Full 11-stage run

---

## Development Goals

1. **Real-world domain with external data** — world/ is fact-checked reference data (actual artists, albums, chart positions, music theory), not invented canon. Tests the real-world branch of Bug #12.
2. **Audio-adjacent puzzle types** — puzzles that encode information in song titles, lyrics, track numbers, album art, music theory. No actual audio files needed — all text-representable.
3. **Smaller scale** — 6 puzzles tests whether the 11-stage pipeline works efficiently at reduced scope. Does the overhead match the scale?
4. **Different delivery aesthetic** — the site and print output should feel like a radio station / music venue, not a game wiki.
5. **Props at small scale** — design 1-2 physical props appropriate to a music theme.

---

## The Hunt

**Name:** WAVELENGTH
**Domain:** Real-world pop/rock music — primarily 1970s–1990s (well-documented, copyright-safe for facts)
**Frame:** The local independent radio station WVLG is going dark after 40 years on the air. The DJ's final broadcast is a puzzle — hidden in the last playlist is a message she's been encoding since 1983. You have until the station signs off to decode it.

**Tone:** Bittersweet. Late-night radio. The warmth of something ending well.

**Meta:** The final broadcast message — a phrase the DJ has been hiding in playlists for 40 years. Emerges from the 6 feeder answers.

---

## Structure

```
WAVELENGTH — 6 puzzles + 1 meta

P1: Track listing puzzle (extraction from song titles)
P2: Music theory / notation (notes spell something)
P3: Album artwork identification (deduction from descriptions)
P4: Chart positions (numbers encode letters)
P5: Lyrics puzzle (first words, last words, or hidden words)
P6: Band/artist names (wordplay, anagram, or pattern)
META: The final broadcast — all 6 answer words converge
```

---

## World Data (real-world, must be fact-checked)

Build world/ from verified sources. Suggested systems:
- `world/systems/artists.md` — band names, active years, members
- `world/systems/albums.md` — titles, track listings, release years, chart positions
- `world/systems/charts.md` — Billboard Hot 100 data for specific years
- `world/systems/theory.md` — music theory facts used in puzzles (note names, intervals)

**Important:** All facts must be verifiable. Flag anything uncertain with [VERIFY]. Do not invent chart positions or track listings — use real data.

---

## Instructions for Opus

Work through all 11 stages. Key differences from IRONFALL:

- **Stage 1/2:** Use `/hunt world` but in real-world mode — research and document actual music facts, don't invent them. Cross-check anything uncertain.
- **Stage 9:** Design the WVLG radio station website (simple, retro, on-air feel). Design one print prop: the DJ's final handwritten playlist (a physical paper prop teams receive).
- **Scale discipline:** 6 puzzles means faster throughput. If stages feel like overkill for the scale, note it as a toolkit bug.
- **Author assignments:** Use only 3 author personalities for 6 puzzles (2 each). Choose which 3 fit best for music puzzles.

Log toolkit gaps in `../../BUGS.md`. Commit at each stage. Claim callsign **Echo** at the end.

---

## Files to Create

```
scenarios/wavelength/
├── SCENARIO.md        ← this file
├── CLAUDE.md          ← scenario status
├── SCOPE.md
├── ROUNDS.md
├── PUZZLES.md
├── world/
│   ├── WORLD.md
│   ├── LOCKED.md
│   └── systems/
├── meta/
├── puzzles/
├── reviews/
├── tests/
└── delivery/
```
