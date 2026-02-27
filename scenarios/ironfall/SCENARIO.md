# Scenario 3 — IRONFALL

**Type:** Fictional world (invented RPG universe)
**Scale:** 10 puzzles + 2 round metas + 1 super-meta
**Pipeline:** Full 11-stage run

---

## Development Goals

This scenario tests capabilities not exercised in Scenarios 1 or 2:

1. **Fictional world-building** — use `/hunt world` to design IRONFALL's universe, data tables, and canon before any authoring begins. World designed for puzzle affordances, not the other way around.
2. **Delivery infrastructure** — Stage 9 DELIVERY BUILD. Design a website concept and print asset (game manual page). First time the pipeline reaches Stage 9+.
3. **UX/interactive component** — at least one puzzle should be an interactive mini-game or fake UI (achievement screen, battle simulator, inventory screen, etc.).
4. **Hunt theming** — establish a consistent visual/narrative design language that applies across all puzzle pages.

---

## The Hunt

**Name:** IRONFALL
**Universe:** A fictional 16-bit RPG from 1993. Cult classic. Never completed by anyone.
**Frame:** You are a speedrunner. The game's original fan forum archive is shutting down in 24 hours. All the community's guides, theories, and discovered secrets will be lost. Somewhere in the archive is the key to the game's true ending — a secret no one has ever triggered. Find it before the servers go dark.

**Tone:** Warm nostalgia for something that never existed. The game feels real. The community feels real. There is genuine loss in the countdown.

**Meta target:** The super-meta answer is the TRUE ENDING CHEAT CODE — a sequence of button inputs that triggers a secret cutscene. The cheat code emerges from the 10 feeder answers.

---

## Structure

```
Act I — "The Archive" (5 puzzles + round meta)
  Puzzles drawn from the fan wiki: bestiary pages, item lists, old guides, save data

Act II — "The Game" (5 puzzles + round meta)
  Puzzles drawn from the game itself: battle system, map, achievements, lore, secrets

True Ending — super-meta
  All 12 answer words (10 feeders + 2 round metas) feed into the cheat code
```

---

## Instructions for Opus

You are running this scenario from scratch through all 11 stages. This is a development test — your job is to BUILD THE HUNT and SURFACE BUGS in the toolkit as you go.

**Start with Stage 1: SCOPE**
- Use `/hunt world init` to establish IRONFALL's universe
- Define the genre, tone, core systems, and meta target
- Let the world design drive the puzzle pool — invent data structures that make great puzzles

**Key things to exercise:**
- `/hunt world` — use it fully. Design at least 5 world systems (bestiary, items, achievements, locations, one more of your choice). Lock the canon before Stage 4.
- At least 1 puzzle should have an interactive/UX component — design it in the brief, even if the actual code is a stub
- At Stage 9, design the delivery website concept and at least one print asset (a page from the IRONFALL game manual)
- Track all toolkit gaps in `../../BUGS.md` as you find them

**Author personalities to use (Stage 6):**
Same 5 as Scenario 2. They know IRONFALL now. Their personalities carry over.
- The Methodical
- The Speedster
- The Skeptic
- The Social
- The Lurker

Assign 2 puzzles per author (10 puzzles total). The round metas and super-meta are admin-authored.

**Commit frequently.** One commit per stage minimum. Use `/admin honor` at the end to claim your callsign.

---

## What Good Looks Like

- A world/ directory with locked canon before any puzzle is written
- 10 puzzle briefs that each reference a specific world/ data table
- At least 1 puzzle with an interactive component described in its brief
- A delivery website concept (even a sketch) in a `delivery/` directory
- A print asset concept (manual page layout)
- All answer words coordinated before authoring (Principle #20)
- Extractions verified character-by-character before marking any puzzle PASS (Principle #19)
- A super-meta that works cleanly with the 10 feeder answers

---

## Files to Create

As you work through the pipeline, create these in `scenarios/ironfall/`:

```
scenarios/ironfall/
├── SCENARIO.md        ← this file
├── CLAUDE.md          ← scenario status (create at Stage 1, update throughout)
├── SCOPE.md           ← Stage 1
├── ROUNDS.md          ← Stage 2
├── PUZZLES.md         ← Stage 4 (master registry)
├── world/             ← Stage 1/2 (/hunt world)
│   ├── WORLD.md
│   ├── LOCKED.md
│   └── systems/
├── meta/              ← Stage 5
├── puzzles/           ← Stage 6
├── reviews/           ← panel output
├── tests/             ← blind test results
└── delivery/          ← Stage 9 (website concept, print assets)
```
