# THE SIXTEENTH SHARD — Print Companion Spec

## Overview

A physical artifact for solvers who want to work on paper. One page per puzzle: the puzzle content cleanly formatted, with relevant world/ data tables included as reference. The booklet is self-contained — a solver with only the printed pages and Cosmere knowledge can solve every puzzle without a screen.

Format: Letter size (8.5 x 11 in). Single-sided for workspace. Stapled or spiral-bound. 56 pages total.

---

## Booklet Structure

```
Page 1:       Cover
Pages 2-3:    Welcome (Hoid's opening + hunt rules)
Pages 4-9:    Reference Sheets (Allomancy table, Radiant Orders, Shard list)
Pages 10-15:  Round 1 divider + P01-P06 (Module A)
Pages 16-21:  P07-P12 (Module B)
Page 22:      Round 1 Meta worksheet (THE WELL)
Pages 23-28:  Round 2 divider + P13-P18 (Module C)
Pages 29-34:  P19-P24 (Module D)
Page 35:      Round 2 Meta worksheet (THE OATHS)
Pages 36-41:  Round 3 divider + P25-P30 (Module E)
Pages 42-47:  P31-P36 (Module F)
Page 48:      Round 3 Meta worksheet (ADONALSIUM)
Page 49:      Super-Meta worksheet (THE SEVENTEENTH SHARD)
Pages 50-55:  Appendix — extended reference data
Page 56:      Back cover (Hoid's closing)
```

**Total: 56 pages.** Print double-sided if desired (28 sheets), but single-sided is recommended so solvers can spread pages out and cross-reference.

---

## Cover (Page 1)

**Layout:**

```
┌─────────────────────────────────────────────┐
│                                             │
│                                             │
│                                             │
│         THE SIXTEENTH SHARD                 │  ← Cinzel, centered, large
│                                             │
│         A Cosmere Puzzle Archive            │  ← Alegreya italic, smaller
│                                             │
│                                             │
│                                             │
│         36 puzzles · 3 metas · 1 answer     │  ← monospace, small
│                                             │
│                                             │
│                                             │
│                                             │
│                                             │
│  "One has maintained this archive for       │  ← Alegreya italic, bottom third
│   longer than most civilizations endure."   │
│                                             │
└─────────────────────────────────────────────┘
```

- Black background, silver text (or white background, black text for B&W printing)
- No imagery. Typography only. The cover is spare.
- Print on heavier stock if available (cardstock).

---

## Welcome Pages (Pages 2-3)

**Page 2: Hoid's Opening**

The same monologue as the homepage `index.html`, formatted for print:

> One has maintained this archive for longer than most civilizations endure. The puzzles were placed across three worlds by someone with too much time and a very particular sense of humor. They encode a message. The message has a single word at its center.
>
> Scadrial's metals burn with names that have letters in useful positions. Roshar's knights swear oaths with measurable depth. The Cosmere's Shards number sixteen, but the count is never quite right.
>
> A careful Worldhopper might start with the metals. They have always been the most honest.

**Page 3: Hunt Rules**

```
STRUCTURE
  3 rounds: Scadrial (12 puzzles), Roshar (12 puzzles), The Cosmere (12 puzzles)
  Each round has a meta puzzle that unlocks after all 12 feeders are solved
  A super-meta unlocks after all 3 round metas are solved
  Puzzles within a round can be solved in any order

ANSWERS
  Every puzzle has a single-word answer (4-8 letters, English)
  Submit answers at [site URL] or write them on the answer blanks
  Answers are not case-sensitive
  Answers contain only letters (no numbers, no spaces)

REFERENCE DATA
  Pages 4-9 contain the reference tables needed for solving
  Each puzzle page also includes the specific data tables relevant to that puzzle
  All puzzle facts come from published Sanderson novels and the Coppermind wiki

HINTS
  [Hint system URL or instructions — configure before print]

THE NARRATOR
  The archive is maintained by someone who has been everywhere and remembers everything.
  He does not use exclamation marks. He does not ask questions. He implies.
```

---

## Reference Sheets (Pages 4-9)

### Page 4-5: The Allomancy Table

Full 4x4 grid from `world/systems/allomancy.md`:

```
┌──────────────┬───────────────┬───────────────┬──────────────┬───────────────┐
│              │  PHYSICAL     │  MENTAL       │  TEMPORAL    │  ENHANCEMENT  │
├──────────────┼───────────────┼───────────────┼──────────────┼───────────────┤
│ PULLING      │ #1 Iron       │ #5 Zinc       │ #9 Gold      │ #13 Aluminum  │
│ (Internal)   │ Pulls metals  │ Riots emotions│ Past selves  │ Wipes own     │
│              │               │               │              │   reserves    │
├──────────────┼───────────────┼───────────────┼──────────────┼───────────────┤
│ PUSHING      │ #2 Steel      │ #6 Brass      │ #10 Electrum │ #14 Duralumin │
│ (External)   │ Pushes metals │ Soothes emot. │ Future self  │ Flares all    │
│              │               │               │              │   metals      │
├──────────────┼───────────────┼───────────────┼──────────────┼───────────────┤
│ PULLING      │ #3 Tin        │ #7 Copper     │ #11 Cadmium  │ #15 Chromium  │
│ (Internal)   │ Senses        │ Coppercloud   │ Slows time   │ Wipes other's │
│              │               │               │              │   reserves    │
├──────────────┼───────────────┼───────────────┼──────────────┼───────────────┤
│ PUSHING      │ #4 Pewter     │ #8 Bronze     │ #12 Bendalloy│ #16 Nicrosil  │
│ (External)   │ Strength      │ Seeking       │ Speeds time  │ Flares other's│
│              │               │               │              │   metals      │
└──────────────┴───────────────┴───────────────┴──────────────┴───────────────┘
```

Include: metal number, name, Allomantic power (1-line summary), Pull/Push designation, Internal/External designation, quadrant label.

**Page 5 extension:** Feruchemical powers column and Hemalurgic steal column (where confirmed). Mark unconfirmed entries with [?].

### Page 6-7: The Knights Radiant

Full 10-Order table from `world/systems/knights-radiant.md`:

| # | Order | Herald | Surges | Gemstone | Spren | Documented Ideals |
|---|-------|--------|--------|----------|-------|-------------------|
| 1 | Windrunners | Jezrien | Adhesion, Gravitation | Sapphire | Honorspren | 4 |
| 2 | Skybreakers | Nale | Gravitation, Division | Smokestone | Highspren | 5 |
| 3 | Dustbringers | Chanarach | Division, Abrasion | Ruby | Ashspren | 1 |
| 4 | Edgedancers | Vedel | Abrasion, Progression | Diamond | Cultivationspren | 3 |
| 5 | Truthwatchers | Pailiah | Progression, Illumination | Emerald | Mistspren | 1 |
| 6 | Lightweavers | Shalash | Illumination, Transformation | Garnet | Cryptics | 3 |
| 7 | Elsecallers | Battar | Transformation, Transportation | Zircon | Inkspren | 1 |
| 8 | Willshapers | Kalak | Transportation, Cohesion | Amethyst | Reachers | 1 |
| 9 | Stonewards | Talenel | Cohesion, Tension | Topaz | Peakspren | 1 |
| 10 | Bondsmiths | Ishar | Tension, Adhesion | Heliodor | Unique (3 max) | 4 |

Include: The Double Eye ring diagram (visual layout of Orders with shared Surges on edges). The 10 Surges listed with their connections.

Include: Full text of all documented Ideals (the First Ideal universal text, then each Order's known Oaths). Critical for META-II.

### Page 8-9: The 16 Shards of Adonalsium

Full table from `world/systems/shards.md`:

| # | Intent | Vessel (Original) | Current Status | Planet | Known Magic |
|---|--------|-------------------|----------------|--------|-------------|
| 1 | Honor | Tanavast | Splintered | Roshar | Surgebinding |
| 2 | Cultivation | Koravellium Avast | Intact | Roshar | Old Magic |
| 3 | Odium | Rayse (now Taravangian) | Intact (contained) | Roshar | Voidbinding |
| 4 | Preservation | Leras | Transformed (into Harmony) | Scadrial | Allomancy |
| 5 | Ruin | Ati | Transformed (into Harmony) | Scadrial | Hemalurgy |
| 6 | Harmony | Sazed | Intact | Scadrial | All three Metallic Arts |
| 7 | Devotion | Aona | Splintered (the Dor) | Sel | AonDor, Forgery |
| 8 | Dominion | Skai | Splintered (the Dor) | Sel | AonDor, Forgery |
| 9 | Endowment | Edgli | Intact | Nalthis | Awakening, Breath |
| 10 | Autonomy | Bavadin | Intact | Taldain | Sand Mastery |
| 11 | Ambition | Uli Da | Splintered | Threnody | Shadows |
| 12 | Mercy | Unknown | Unknown | Unknown | Unknown |
| 13 | Valor | Unknown | Unknown | Unknown | Unknown |
| 14 | Whimsy | Unknown | Unknown | Unknown | Unknown |
| 15 | Invention | Unknown | Unknown | Unknown | Unknown |
| 16 | Virtuosity | Unknown | Splintered (self) | Unknown | Unknown |

Include: note on Harmony = Preservation + Ruin combined. Note on which Shards are Splintered vs. Intact vs. Unknown.

---

## Puzzle Pages (Pages 10-49)

### Layout Per Puzzle

Each puzzle occupies one page (Letter size, 8.5 x 11 in, 0.75 in margins):

```
┌──────────────────────────────────────────────────────────┐
│  THE SIXTEENTH SHARD                                     │
│  ROUND 1: SCADRIAL                    P01 · Iron · ★★   │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  BURNING THROUGH                                         │  ← Cinzel, 14pt
│                                                          │
│  Sixteen metals burn in a Mistborn's stomach. Each       │  ← Alegreya italic, 11pt
│  grants a different power. Each power is described       │
│  below in the words of those who have burned them —      │
│  though the metal's name has been excised.               │
│                                                          │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  [Puzzle content: tables, grids, clues, text]            │  ← Alegreya, 11pt
│                                                          │
│  [Full puzzle content from puzzles/P01-burning-through.md│
│   converted to clean print layout]                       │
│                                                          │
├──────────────────────────────────────────────────────────┤
│                                                          │
│  [Extraction instructions]                               │
│                                                          │
├──────────────────────────────────────────────────────────┤
│  ANSWER: ___ ___ ___ ___ ___ ___                         │  ← monospace, 12pt
│                                                          │
│  ☐ Submit at [site-url]/submit                           │  ← small, bottom
└──────────────────────────────────────────────────────────┘
```

### Print Specs

| Property | Value |
|----------|-------|
| Page size | Letter (8.5 x 11 in) |
| Margins | 0.75 in all sides |
| Body font | Alegreya, 11pt, black on white |
| Header font | Cinzel, 14pt (title), 10pt (metadata) |
| Table font | Alegreya, 10pt |
| Answer blanks | JetBrains Mono, 12pt, underlined |
| Line height | 1.5 (body), 1.3 (tables) |
| Color | Black text, white background (B&W safe) |
| Borders | 0.5pt black for tables, 1pt for section dividers |

### Reference Data Per Puzzle

Each puzzle page includes a condensed version of the world/ data relevant to that puzzle. This eliminates the need to flip back to the reference sheets for most puzzles.

| Puzzle | Embedded Reference Data |
|--------|------------------------|
| P01 | 16 Allomantic metals: name + number + power (1-line each) |
| P02 | Alloy pairs: which metals are alloys of which |
| P03 | 16 metals: name + Feruchemical power (1-line each) |
| P04 | Full 4x4 Allomancy grid (compact) |
| P05 | Triple chart: Allomancy, Feruchemy, Hemalurgy for Physical quadrant |
| P06 | No external reference needed (self-contained logic puzzle) |
| P07 | Final Empire timeline (major events, 1000-year span) |
| P08 | Kelsier's crew: member names + roles |
| P09 | Scadrial social structure (skaa, nobility, Terris) |
| P10 | Era 1 + Era 2 key characters and events |
| P11 | Siege of Luthadel: key participants + roles |
| P12 | Twinborn abilities: Allomantic + Feruchemical combinations |
| P13 | 10 Heralds: name + Order + epithet + gemstone |
| P14 | All documented Radiant Ideals (full text) |
| P15 | 10 Spren types: name + Order + behavior notes |
| P16 | Known Radiants: character + Order + status |
| P17 | Double Eye diagram (visual) + 10 Surges |
| P18 | 10 Orders: 2 Surges each (the Surge-sharing pattern) |
| P19 | Singer forms: name + description + abilities |
| P20 | Bridge Four members: name + notable trait |
| P21 | Singer Rhythms: name + emotional association |
| P22 | No external reference needed (cryptic crossword — clues are self-contained) |
| P23 | Fabrials: type + effect + gemstone |
| P24 | Roshar geography: key locations + relative positions |
| P25 | 16 Shard Intents: name + status (1-line each) |
| P26 | 16 Vessels: name + Shard |
| P27 | Odium's campaign: chronological events |
| P28 | Cosmere magic systems: name + world + Shard |
| P29 | 16 Shards: Intent + key properties (compact) |
| P30 | Shattering participants: Vessel names (those known) |
| P31 | Hoid's aliases: name + world + book appearance |
| P32 | Cognitive Shadows: who + world + original identity |
| P33 | Investiture types: power + world + Shard |
| P34 | Epigraph letters from Stormlight Archive (attributions) |
| P35 | No external reference needed (self-contained) |
| P36 | Cross-Cosmere character connections |

### Round Divider Pages

Before each round's first puzzle, a half-page divider:

```
┌──────────────────────────────────────────────────────────┐
│                                                          │
│                                                          │
│         ROUND 1: SCADRIAL                                │  ← Cinzel, 18pt
│         The Metal Arts                                   │  ← Alegreya italic, 14pt
│                                                          │
│         12 puzzles · Modules A + B                       │  ← monospace, 10pt
│                                                          │
│         "The metals have names. The names have letters.  │  ← Alegreya italic
│          The letters have positions. One suspects the    │
│          Lord Ruler noticed this too, though he had      │
│          other concerns at the time."                    │
│                                                          │
└──────────────────────────────────────────────────────────┘
```

---

## Meta Worksheets

### THE WELL (Page 22)

Printed 4x4 grid with pre-labeled metal names. 12 blank answer slots beside the corresponding metals. Quadrant-depth extraction guide. Space for the 5 extracted letters. Answer blank.

### THE OATHS (Page 35)

Printed Double Eye ring diagram with 10 Order positions. Table of Ideal counts. Answer mapping grid. Space for 5 extracted letters. Answer blank.

### ADONALSIUM (Page 48)

Printed 16-Shard checklist. 12 identification slots. Space to mark the 4 missing Shards. Anagram workspace. Answer blank.

### THE SEVENTEENTH SHARD (Page 49)

Three input rows (one per round meta answer). The Realms cycle extraction diagram. Five extraction steps laid out. Final answer blank.

---

## Appendix (Pages 50-55)

Extended reference data for solvers who want everything in one place:

- **Page 50:** Complete Allomancy table (expanded — all three Metallic Arts per metal)
- **Page 51:** All documented Radiant Ideals (full text, organized by Order)
- **Page 52:** The Double Eye diagram (large, labeled, with Surge connections)
- **Page 53:** Complete Shard table (all 16, all known properties)
- **Page 54:** Hoid's appearances (chronological, with aliases and books)
- **Page 55:** Cosmere geography (worlds, Shards, magic systems, known Worldhoppers)

---

## Back Cover (Page 56)

```
┌──────────────────────────────────────────────────────────┐
│                                                          │
│                                                          │
│                                                          │
│                                                          │
│                                                          │
│  "One supposes the journey was the point.                │
│   But the destination was never another fragment.        │
│   It was the memory of wholeness.                        │
│   Before the breaking, there was a word for              │
│   what Adonalsium was.                                   │
│   A Worldhopper suspects that word                       │
│   has been here all along."                              │
│                                                          │
│                                                          │
│                                                          │
│                                                          │
│                                                          │
│  THE SIXTEENTH SHARD · A Cosmere Puzzle Archive          │  ← small, bottom center
│                                                          │
└──────────────────────────────────────────────────────────┘
```

---

## Production Notes

### Paper

- Interior: standard 20lb letter paper (or 24lb for durability)
- Cover: cardstock if available (65lb+)
- Single-sided recommended (puzzle workspace on backs)

### Binding

- **Stapled:** for 56 pages, saddle-stitch works but is tight. Use 2 staples.
- **Spiral-bound:** recommended for lay-flat solving. Most print shops offer this.
- **Three-hole punch + binder:** allows solvers to rearrange and remove pages.

### Copies

- Print one per team (for group solving with the booklet as shared reference)
- Print one per solver (for individual work)
- Keep 2 spare copies at the admin table

### Print Workflow

1. Generate all puzzle page HTML files in `delivery/print/puzzles/`
2. Open each in a browser
3. Print > Save as PDF (or print directly)
4. Collate in booklet order
5. Add cover and back cover
6. Bind

No special software required. Browser print CSS handles all formatting.

---

## HTML Generation

Each print page is a standalone HTML file with embedded print CSS:

```html
<!DOCTYPE html>
<html>
<head>
  <meta charset="UTF-8">
  <title>P01 — Burning Through</title>
  <style>
    @page { size: letter; margin: 0.75in; }
    @media print {
      body { font-family: 'Alegreya', Palatino, serif; font-size: 11pt; color: #1a1a1a; }
      h1 { font-family: 'Cinzel', Georgia, serif; font-size: 14pt; }
      table { border-collapse: collapse; width: 100%; }
      td, th { border: 0.5pt solid #1a1a1a; padding: 4pt 6pt; font-size: 10pt; }
      .answer-blank { font-family: 'JetBrains Mono', Consolas, monospace; font-size: 12pt; }
      .no-print { display: none; }
    }
  </style>
</head>
<body>
  <header>
    <span class="hunt-name">THE SIXTEENTH SHARD</span>
    <span class="round-name">ROUND 1: SCADRIAL</span>
    <span class="puzzle-meta">P01 · Iron · ★★</span>
  </header>
  <h1>Burning Through</h1>
  <div class="flavor"><!-- Hoid's intro --></div>
  <div class="puzzle-content"><!-- puzzle body --></div>
  <div class="extraction"><!-- extraction instructions --></div>
  <div class="answer-area">
    <span class="answer-blank">ANSWER: __ __ __ __ __ __</span>
  </div>
  <footer>
    <span>Submit: [site-url]/submit</span>
  </footer>
</body>
</html>
```
