# Scenario 5 — THE GRAND LARCENY

**Type:** Narrative/mystery, props-heavy
**Scale:** 4 puzzles + 1 meta
**Structure:** 1 round
**Pipeline:** Full 11-stage run

---

## Development Goals

1. **Small scale** — 4 puzzles is the minimum viable hunt. Does the pipeline work at this size, or does it collapse under its own overhead? Note every place the process feels over-engineered for 4 puzzles.
2. **Physical-first delivery** — the primary delivery medium is a printed dossier, not a website. Teams receive a physical evidence pack. The website is secondary (answer submission only). Tests `/hunt print` and `/hunt props` as the *primary* delivery system.
3. **Narrative integration** — puzzles are embedded in the mystery fiction (the evidence IS the puzzle). Tests whether puzzle mechanism and narrative can be inseparable.
4. **Props variety** — multiple prop types: printed documents, a physical object, a consumable. Tests `/hunt props` flavor-design and distribution for a small event.
5. **Print layout discipline** — each puzzle is a "document" in an evidence dossier. Tests page layout, document design, print theming at high fidelity.

---

## The Hunt

**Name:** THE GRAND LARCENY
**Frame:** A priceless painting was stolen from the Grand Hotel during last night's gala. Four suspects. Four pieces of evidence. Your team is the investigative unit — solve the case before the trail goes cold.

**Tone:** Noir. Deadpan wit. Every clue is a real document — a police report, a hotel receipt, a witness photo, a floor plan.

**Meta:** The answer is the METHOD — how the painting was actually stolen. The 4 feeder answers (SUSPECT, LOCATION, TIME, OBJECT) combine to reveal the method.

**Fictional universe:** The Grand Hotel, its staff, guests, and layout are all invented. Design them to support the puzzle mechanisms.

---

## Structure

```
THE GRAND LARCENY — 4 puzzles + 1 meta

P1: The Police Report (grid/logic — extracting the suspect's alibi contradiction)
P2: The Hotel Receipt (arithmetic/pattern — something doesn't add up)
P3: The Floor Plan (spatial — the route the thief took encodes something)
P4: The Witness Statement (text puzzle — the witness said more than they meant to)
META: The Method — how the 4 answers combine to reveal what actually happened
```

---

## Physical Delivery (PRIMARY medium)

Each team receives an **evidence dossier** — a manila folder containing:
- 4 puzzle documents (each is a "real" document in the fiction)
- A cover sheet: case number, detective assignment, briefing
- An evidence log (tracks what's been examined)

This is the hunt. No website needed to solve — website is for answer submission only.

**Props to design:**
1. `dossier` (kit_item) — the physical folder with all printed documents, one per team
2. `hotel-key` (component) — a physical hotel room key (prop or printed card) that's part of P3
3. `calling-card` (flavor) — a small printed card left at the crime scene, embossed with a symbol that's part of the meta

---

## Instructions for Opus

Work through all 11 stages. Key differences from IRONFALL and WAVELENGTH:

- **Stage 1/2:** Use `/hunt world` to design The Grand Hotel — rooms, staff, guests, timeline of the gala. Invent everything. Design it so the floor plan can encode a path.
- **Stage 9:** `/hunt props` is the primary deliverable. Design the dossier contents, document layout for each puzzle, label/format for the hotel key prop. The website is minimal — answer submission only, no puzzle pages.
- **Scale test:** 4 puzzles. Actively note every stage where the pipeline feels like overkill. This is a legitimate bug to surface.
- **Document design:** Each puzzle page should look like a real document (police report header, hotel receipt format, etc.). Push `/hunt print` on document-style layouts, not just worksheet-style.
- **Author assignments:** Use 2 author personalities, 2 puzzles each. The narrative voice must be consistent across all 4 — the Methodical and the Social are good fits.

Log toolkit gaps in `../../BUGS.md`. Commit at each stage. Claim callsign **Foxtrot** at the end.

---

## Files to Create

```
scenarios/grand-larceny/
├── SCENARIO.md        ← this file
├── CLAUDE.md
├── SCOPE.md
├── ROUNDS.md
├── PUZZLES.md
├── world/
│   ├── WORLD.md       ← The Grand Hotel: rooms, staff, gala timeline
│   ├── LOCKED.md
│   └── systems/
├── meta/
├── puzzles/
├── reviews/
├── tests/
└── delivery/
    ├── props/         ← dossier contents, hotel key, calling card
    └── print/         ← document-style puzzle layouts
```
