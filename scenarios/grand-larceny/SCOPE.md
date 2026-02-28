# THE GRAND LARCENY — Scope

## Hunt Identity

| Field | Value |
|-------|-------|
| **Name** | THE GRAND LARCENY |
| **Genre** | Noir heist mystery |
| **Scale** | 4 puzzles + 1 meta |
| **Audience** | Teams of 2-4, mixed experience |
| **Format** | Physical-first (evidence dossier), website for answer submission only |
| **Duration** | 90-120 minutes |
| **Structure** | 1 round, linear mystery |
| **Tone** | Noir. Deadpan wit. Every clue is a real document. |

## The Frame

A priceless painting -- Renaud's "Lady in Vermillion" -- was stolen from the Grand Hotel during last night's charity gala. Four suspects. Four pieces of evidence. Your team is the investigative unit assigned to the case. Solve it before the trail goes cold.

## The Universe

The Grand Hotel is a fictional Art Deco landmark. Everything about it -- the staff, the guests, the rooms, the gala timeline -- is invented. The universe is designed so that:
- The police report contains a logic grid of alibis and contradictions
- The hotel receipt contains arithmetic patterns that reveal an anomaly
- The floor plan encodes a spatial path through rooms
- The witness statement hides a message in the text itself

## The Puzzles

| ID | Title | Type | Document form | Answer |
|----|-------|------|---------------|--------|
| P1 | The Police Report | Logic/grid | Police incident report | (encoded) |
| P2 | The Hotel Receipt | Arithmetic/pattern | Itemized hotel bill | (encoded) |
| P3 | The Floor Plan | Spatial/path | Architectural floor plan | (encoded) |
| P4 | The Witness Statement | Text/hidden message | Typed witness transcript | (encoded) |
| META | The Method | Combination | Evidence log / case summary | (encoded) |

## The Meta

The four feeder answers combine to reveal THE METHOD -- how the painting was actually stolen. The answer words are coordinated: SUSPECT, LOCATION, TIME, OBJECT. These four words, when combined with the hotel's fiction, point to the method.

## Physical Delivery (Primary)

Each team receives an evidence dossier -- a manila folder containing:
1. **Cover sheet:** Case number, detective assignment, briefing paragraph
2. **P1:** Police incident report (printed document)
3. **P2:** Itemized hotel receipt (printed document)
4. **P3:** Architectural floor plan (printed document)
5. **P4:** Typed witness statement (printed document)
6. **Evidence log:** Tracks what evidence has been examined, with space for notes

Props:
- `dossier` (kit_item): the physical folder with all documents
- `hotel-key` (component): a printed hotel room key card for P3
- `calling-card` (flavor): a small printed card with an embossed symbol, relevant to the meta

## Website (Secondary)

Answer submission only. No puzzle content on the website. Teams solve entirely from the physical dossier.

## Voice

The documents speak for themselves. Each puzzle IS its document -- police report language, hotel accounting format, architectural notation, witness transcript style. No puzzle instructions, no worksheets. The solver reads the document as an investigator would.

The cover sheet is written in official police briefing voice: terse, factual, slightly weary.

## Answer Security

ROT13 encoded. Stored in `.claude/` project memory only. No plaintext answers in git-tracked files.
