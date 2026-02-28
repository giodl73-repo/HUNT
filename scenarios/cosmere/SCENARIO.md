# Scenario 7 — THE COSMERE HUNT

**Type:** Real-world domain (Brandon Sanderson's Cosmere shared universe)
**Scale:** 36 puzzles + 3 round metas + 1 super-meta
**Structure:** 3 rounds × 12 puzzles, 6 modules of 6 puzzles each
**Pipeline:** Full 11-stage run

---

## Development Goals

1. **Mega scale** — 36 puzzles tests whether the pipeline holds at MIT Mystery Hunt size. Does overhead collapse? Where does it break?
2. **Multi-world content library** — the Cosmere spans 10+ books and 8+ planets. Tests the real-world branch: fact-checked canon from published books, no invented facts.
3. **Interlocking data tables** — Allomancy, Feruchemy, Hemalurgy, the Knight Radiant orders, the 10 Surges, the 16 Shards. Rich structured data that makes great puzzles.
4. **6 parallel author modules** — largest multi-author test yet. 6 teams working in parallel, 6 puzzles each.
5. **Cross-world meta architecture** — round metas draw from single-world knowledge; super-meta requires cross-world synthesis. Tests whether the meta system scales.

---

## The Hunt

**Name:** THE SIXTEENTH SHARD
**Frame:** Hoid has hidden something in the Cosmere's history — a message left across three worlds for anyone with the pattern recognition to find it. The solvers are Worldhoppers. They move through Scadrial, Roshar, and the wider Cosmere, piecing together the secret Hoid encoded in the fabric of each world's Investiture.

**Tone:** Reverent but playful. The Cosmere is dense — the hunt respects that knowledge. But Hoid is watching, and he's amused. There's wit under the lore.

**Narrator:** Hoid/Wit. Short, dry observations. Present tense. No exclamation marks. He knows more than he's letting on.

**Meta target:** The super-meta answer is the 17th Shard — the shard of Adonalsium that Hoid has been seeking. It doesn't exist in the books. But the puzzle logic creates it.

---

## Structure

```
ROUND 1: SCADRIAL — "The Metal Arts" (12 puzzles)
  Module A: Allomancy (metals, powers, the full chart — Mistborn era)
  Module B: The Final Empire & Beyond (history, characters, Wax & Wayne era)
  Round Meta I: THE WELL — feeds into the super-meta

ROUND 2: ROSHAR — "Words of Radiance" (12 puzzles)
  Module C: The Knights Radiant (10 orders, 10 surges, 10 heralds, Spren)
  Module D: The Stormlight World (highstorms, fabrials, the Everstorm, the Singers)
  Round Meta II: THE OATHS — feeds into the super-meta

ROUND 3: THE COSMERE — "The Shattering" (12 puzzles)
  Module E: The 16 Shards (names, vessels, planets, Investiture types)
  Module F: Hoid & the Worldhoppers (crossover moments, the 17th Shard Society, connections)
  Round Meta III: ADONALSIUM — feeds into the super-meta

SUPER-META: THE SEVENTEENTH SHARD
  Three round meta answers combine to reveal what Hoid has been looking for.
```

---

## Content Library

Real-world domain. All puzzle facts from published Sanderson canon:
- *Mistborn* trilogy (Era 1) + *Wax & Wayne* (Era 2)
- *The Way of Kings*, *Words of Radiance*, *Oathbringer*, *Rhythm of War*, *Wind and Truth*
- *Warbreaker*, *Elantris*, *The Emperor's Soul*
- Arcanum Unbounded (novellas + essays)
- The Coppermind wiki (community-verified canon)

**Important:** No WoBs (Words of Brandon) unless confirmed in published text. The Coppermind is the source of truth. Flag anything uncertain with [VERIFY].

---

## Module Assignments (6 author teams)

| Module | World | Focus | Puzzles | Author Team |
|--------|-------|-------|---------|-------------|
| A | Scadrial | Allomancy chart, metals, powers | P01-P06 | Team Alpha |
| B | Scadrial | History, characters, Era 1+2 | P07-P12 | Team Beta |
| C | Roshar | Knights Radiant, orders, surges, heralds | P13-P18 | Team Gamma |
| D | Roshar | Stormlight world: storms, fabrials, Singers | P19-P24 | Team Delta |
| E | Cosmere | The 16 Shards: names, vessels, Investiture | P25-P30 | Team Epsilon |
| F | Cosmere | Hoid, Worldhoppers, crossover connections | P31-P36 | Team Zeta |

Each meta is admin-authored after its round's 12 puzzles are locked.

---

## Key Puzzle Affordances

**Allomancy table** (Module A):
- 16 metals in a 4×4 grid (Internal/External × Push/Pull × Physical/Mental/Temporal/Enhancement)
- Each metal has a name, allomantic power, feruchemical power, hemalurgic use
- Massive encoding potential: position in table, first letters, property matching

**Knights Radiant** (Module C):
- 10 orders, each with 2 Surges, 1 Herald, 1 Gemstone, 1 associated Spren type
- 10 Surges with specific properties
- The Oaths (each order has a progression of 5 Oaths)
- Perfect for matching, sequencing, and extraction puzzles

**The 16 Shards** (Module E):
- 16 Shards each with: Name (Intent), original Vessel, current status (Splintered/held/combined), planet/location
- Shard Intents are single words (Preservation, Ruin, Honor, Cultivation, Odium, etc.)
- 16 = 4×4 grid encoding potential

**Hoid appearances** (Module F):
- Hoid appears in every Cosmere book under different names/disguises
- His appearances encode information across the timeline
- Perfect for identification/deduction puzzles

---

## Meta Architecture Notes

**Round Meta I (THE WELL):**
The 12 Scadrial answers combine to point at something hidden in the Well of Ascension. The mechanism should use the Allomancy table in some way — the table IS the meta.

**Round Meta II (THE OATHS):**
The 12 Roshar answers encode as the 5 Oaths of a specific Radiant order. The Oaths themselves are the extraction mechanism.

**Round Meta III (ADONALSIUM):**
The 12 Cosmere answers reference 12 of the 16 Shards. The 4 missing Shards spell the round meta answer.

**Super-meta (THE SEVENTEENTH SHARD):**
The 3 round meta answers, combined with the Investiture logic of all three rounds, reveal the nature of the 17th Shard. The aha: there is no 17th Shard — the answer is a word that means "the seeker" or "the questioner," which is Hoid's true nature. Something like CURIOSITY or INQUIRY.

---

## Instructions for Opus

Work through all 11 stages. This is the largest scenario — pace yourself.

**Stage 1/2:** Build `world/` with at minimum 6 data systems:
- `allomancy.md` — the full 16-metal table with all properties
- `feruchemy.md` — feruchemical attributes per metal
- `hemalurgy.md` — hemalurgic uses per metal
- `knights-radiant.md` — 10 orders × surges × heralds × spren × oaths
- `shards.md` — 16 Shards × vessels × planets × investiture
- `hoid.md` — Hoid's appearances across all books with book/chapter/alias

**Stage 3/4:** Design 36 puzzle briefs. Every puzzle must reference a specific world/ data table. Coordinate all 36 answer words before any authoring begins (Principle #20).

**Stage 5:** Design all 4 metas before authoring begins. Verify the super-meta works with the 3 round meta answers.

**Commit frequently.** One commit per stage minimum.
Log toolkit gaps to `BUGS-local.md`. Do NOT write to `../../BUGS.md`.
Claim callsign **Sierra** at the end.

---

## Files to Create

```
scenarios/cosmere/
├── SCENARIO.md         ← this file
├── CLAUDE.md           ← status tracker (create at Stage 1)
├── SCOPE.md            ← Stage 1
├── ROUNDS.md           ← Stage 2
├── PUZZLES.md          ← Stage 4 (all 36 briefs)
├── world/
│   ├── WORLD.md
│   ├── LOCKED.md
│   └── systems/
│       ├── allomancy.md
│       ├── feruchemy.md
│       ├── hemalurgy.md
│       ├── knights-radiant.md
│       ├── shards.md
│       └── hoid.md
├── meta/
│   ├── META-I-WELL.md
│   ├── META-II-OATHS.md
│   ├── META-III-ADONALSIUM.md
│   └── SUPER-META.md
├── puzzles/            ← Stages 6-7 (36 puzzle files)
├── reviews/
├── tests/
└── delivery/
```
