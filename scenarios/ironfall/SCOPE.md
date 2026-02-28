# IRONFALL — Scope

## Identity

**Name:** IRONFALL
**Genre:** Fictional 16-bit JRPG, released 1993 on the Super Famicom / SNES
**Developer:** Ashfield Interactive (fictional, Osaka-based)
**Tone:** Warm nostalgia for something that never existed. The game feels real. The community feels real. There is genuine loss in the countdown.
**Scale:** 10 puzzles + 2 round metas + 1 super-meta (true ending cheat code)
**Audience:** Teams of 4-8, mixed puzzle experience, some gaming nostalgia appreciated but not required
**Format:** Hybrid (website + optional print materials)
**Duration:** 4-6 hours

---

## The Frame

You are a speedrunner. IRONFALL's original fan forum archive — ironfall-archive.net — is shutting down in 24 hours. The site has been running since 1997. All the community's guides, theories, datamined secrets, and discovered exploits will be lost forever.

Somewhere in the archive is the key to the game's TRUE ENDING — a secret ending cutscene that Ashfield Interactive hid in the code but no player has ever triggered. Forum rumors say the final developer, Kenji Morimoto, embedded the cheat code across the game's systems before the studio closed in 1996.

Find the True Ending before the servers go dark.

---

## Core Hook

The hunt IS the archive. Puzzles are presented as archived forum posts, wiki pages, save file analyses, bestiary entries, achievement guides, and datamined code dumps. The solver is not "doing puzzles about a game" — the solver is doing what speedrunners and dataminers actually do: cross-referencing game data, spotting anomalies in stats, decoding developer-hidden messages, and piecing together the game's secrets from incomplete community knowledge.

---

## Core Systems

IRONFALL has five interlocking game systems that puzzles will reference:

1. **Bestiary** — 24 enemies across 4 regions. Each has HP, ATK, DEF, SPD, Element, and a rare drop. Stat combinations are unique enough for deduction puzzles. Some enemies are palette swaps with identical stats except one field.
2. **Items & Crafting** — 20 key items and a crafting system where combining two items produces a third. Crafting rules follow elemental logic (Fire + Ice = Steam, etc.). Item names are extractable.
3. **Achievements** — 16 in-game achievements (called "Marks of Mastery"). Each has a name, description, and unlock condition. Achievement names contain hidden letter patterns.
4. **Locations** — A 6-region world map with connections. Each region has a dungeon, town, and secret area. Map topology supports spatial/navigation puzzles.
5. **Battle System** — Turn-based with 8 special moves. Each move has a name, element, power, MP cost, and a hidden damage formula. The formulas are inferrable from observed battle logs.
6. **Lore & Characters** — 4 playable characters, each with a backstory quest. 6 NPCs with dialogue that contains coded hints. A secret 5th character (the developer's self-insert) who appears only if specific conditions are met.

---

## Meta Architecture

```
Act I — "The Archive" (5 puzzles + round meta)
  Puzzles sourced from the fan wiki side of the archive:
  bestiary pages, item databases, old guides, save data analysis

Act II — "The Game" (5 puzzles + round meta)
  Puzzles sourced from the game itself:
  battle system, map exploration, achievements, lore, secrets

Act I meta → yields a 5-letter directional word (compass/D-pad direction)
Act II meta → yields a 5-letter directional word (compass/D-pad direction)

True Ending (super-meta):
  The 10 feeder answer words + 2 round meta words combine to produce
  a 12-input cheat code (a sequence of SNES controller button presses:
  UP, DOWN, LEFT, RIGHT, A, B, X, Y, L, R, START, SELECT)
```

---

## Answer Encoding

Answers stored in `.claude/` project memory only. Encoding: ROT13. No plaintext answers in any git-tracked file.

---

## Narrator Voice

The archive speaks. Not a character — the archive itself. Past tense for forum memories. Present tense for the countdown. No exclamation marks. No questions. Short sentences. The solver is "you."

Example:
> The archive remembers. This page was last updated on March 14, 1998, by user DarkKnight_97. The bestiary data is hand-counted from 200 hours of gameplay. Some entries have been disputed. You will need to decide which numbers to trust.

---

## Visual Theme

Late-1990s web aesthetic. Tiled backgrounds. Pixel fonts. Visitor counters. "Under construction" GIFs. A countdown timer to server shutdown. The warm glow of a CRT monitor in a dark room. The colors of the SNES era: deep purples, golds, steel greys, forest greens.

---

## Review Gate

This scope defines a hunt that:
- Uses a fictional world designed for puzzle affordances (Riven Standard)
- Has interlocking data systems that reward cross-referencing (Interlock, Not Independence)
- Frames the meta target in-universe (the cheat code is a diegetic goal)
- Can be solved without prior IRONFALL knowledge (all data provided in the archive)
- Has room for at least one interactive/UX component (achievement screen, battle sim, save file viewer)
