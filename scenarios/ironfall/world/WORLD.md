# IRONFALL — World Overview

**Genre/Aesthetic:** 16-bit JRPG, Super Famicom, 1993. Pixel art, Mode 7 world map, Nobuo Uematsu-style score (fictional composer: Yuki Tanabe). Published by Ashfield Interactive, Osaka.
**Hook:** You are a speedrunner racing to find the game's True Ending before the fan archive shuts down.
**Core systems:** Bestiary, Items & Crafting, Achievements (Marks of Mastery), Locations (World Map), Battle System, Lore & Characters
**Meta target:** The True Ending cheat code — a sequence of SNES controller button presses hidden across the game's data systems.

---

## Tone

IRONFALL is a cult classic that never broke through to mainstream success. It sold 140,000 copies in Japan and 60,000 in North America. The fan community — small, devoted, meticulous — ran the archive from 1997 to present day. The community knew the game better than its creators.

The world feels like a real game. The data tables are consistent, the systems interlock, the lore has depth. Forum posts have usernames, timestamps, and personality. Wiki edits show revision histories. Everything is internally coherent.

The emotional register is warm loss. The archive is shutting down. These pages will not exist tomorrow. But the community's work — counting enemy HP values by hand, mapping every screen transition, cataloguing every crafting combination — that work mattered. And somewhere in it is the answer to the one question nobody solved.

---

## Canon Rules

- IRONFALL is a complete, self-consistent game. Every stat, formula, name, and location is defined and fixed.
- The world/ data tables are the source of truth. Authors verify against world/ files, not their own invention.
- No fact in any puzzle should contradict the world/ tables.
- Forum posts, wiki entries, and community guides are in-universe framing — they present world/ data in a narrative wrapper.
- The True Ending is real. Kenji Morimoto (lead developer) embedded the cheat code before the studio closed. The code is not random — it maps to game data in a discoverable way.
- The game has a "standard ending" that plays when you defeat the final boss. The True Ending is a different cutscene triggered by the cheat code entered on the title screen after completing the game once.

---

## World Systems Index

| System | File | Puzzles that reference it | Design affordance |
|--------|------|--------------------------|-------------------|
| Bestiary | `systems/bestiary.md` | P01, P06 | Grid deduction (unique stat combos), identification |
| Items & Crafting | `systems/items.md` | P02, P07 | Logic puzzle (crafting rules), extraction from item names |
| Achievements | `systems/achievements.md` | P03, P08 | Hidden letter extraction, pattern recognition |
| Locations | `systems/locations.md` | P04, P09 | Spatial navigation, map topology, path-finding |
| Battle System | `systems/battle.md` | P05, P10 | Formula inference, damage calculation, mini-game |
| Lore & Characters | `systems/lore.md` | P06, P07, P08 | Identification, cross-referencing, NPC dialogue codes |
| Save File Format | `systems/savefile.md` | P03 | Pattern recognition, hex/binary reading |
| Cheat Codes | `systems/cheats.md` | Meta | Button sequence mapping, final extraction |

---

## Data Design Philosophy

Every data table is designed for puzzle affordances first:
- **Bestiary**: exactly the right number of unique stat combinations for a 6x4 deduction grid
- **Items**: crafting rules follow consistent elemental logic that can be inferred from partial data
- **Achievements**: names chosen so hidden letters spell extractable words
- **Locations**: map has exactly the topology needed for a pathfinding puzzle with a unique solution
- **Battle System**: damage formulas are inferrable from 4-5 observed battles, not given
- **Lore**: character dialogue contains coded messages using a discoverable cipher
