# THE GRAND LARCENY -- Props Registry

## Overview

All physical items that go into each team's evidence dossier. The dossier IS the hunt -- no website required for solving. Website is answer submission only.

---

## Master Inventory

| ID | Item | Type | Linked To | Material | Qty/Team | Notes |
|----|------|------|-----------|----------|----------|-------|
| PR01 | Evidence Dossier | kit_item | ALL | Manila folder, letter-size | 1 | Contains all documents and loose props |
| PR02 | Cover Sheet (Case Briefing) | document | ALL | Cardstock (65lb+), letter | 1 | Top page in dossier. Official police assignment. |
| PR03 | Exhibit A: Police Report (P1) | document | P1 | White letter paper, 2 pages stapled | 1 | Incident report with alibi analysis |
| PR04 | Exhibit B: Hotel Receipt (P2) | document | P2 | White letter paper, 2 pages stapled | 1 | Consolidated guest billing statement |
| PR05 | Exhibit C: Floor Plan (P3) | document | P3 | White letter paper, 1-2 pages | 1 | Architectural survey, all 4 levels |
| PR06 | Exhibit D: Witness Statement (P4) | document | P4 | White letter paper, 2 pages stapled | 1 | Transcribed interview, Miss Delacroix |
| PR07 | Evidence Correlation Log | document | META | White letter paper, 1 page | 1 | Case summary + meta worksheet. Last page. |
| PR08 | Hotel Key Card | component | P3 | Heavy cardstock (110lb+), 3.5x2in | 1 | ROOM 303 / Roof Access. Loose in dossier. |
| PR09 | Calling Card | flavor/META | META | Heavy cardstock, 3x2in, cream paper | 1 | Thief's signature. Numbers 3-1-2-6. Loose in dossier. |
| PR10 | Dossier Label | kit_item | ALL | Adhesive paper, ~4x2.5in | 1 | Applied to front of manila folder. |

---

## Dossier Assembly Order

Contents of each folder, top to bottom:

1. Cover Sheet (PR02) -- on top
2. Exhibit A -- P1: The Police Report (PR03) -- 2 pages, stapled
3. Exhibit B -- P2: The Hotel Receipt (PR04) -- 2 pages, stapled
4. Exhibit C -- P3: The Floor Plan (PR05) -- 1-2 pages, not folded
5. Exhibit D -- P4: The Witness Statement (PR06) -- 2 pages, stapled
6. Evidence Correlation Log (PR07) -- last printed page
7. Hotel Key Card (PR08) -- dropped loose into folder
8. Calling Card (PR09) -- dropped loose into folder

**Total per team:** 9-10 printed pages + 2 small cards, in 1 labeled manila folder.

---

## Prop Relationships

```
DOSSIER (PR01)
  ├── Label (PR10) ............. applied to folder exterior
  ├── Cover Sheet (PR02) ....... establishes assignment, lists exhibits
  ├── P1 document (PR03) ....... EXHIBIT A
  ├── P2 document (PR04) ....... EXHIBIT B
  ├── P3 document (PR05) ....... EXHIBIT C
  ├── P4 document (PR06) ....... EXHIBIT D
  ├── Evidence Log (PR07) ...... META worksheet, references calling card
  ├── Hotel Key Card (PR08) .... needed for P3 (starting point of path)
  └── Calling Card (PR09) ...... needed for META (extraction positions)
```

---

## Material Summary (per N teams)

| Material | Quantity | Source |
|----------|----------|--------|
| Manila folders, letter-size | N + 2 spare | Office supply |
| White letter paper (20lb) | 9N-10N sheets + spares | Standard printer paper |
| Cardstock, white (65lb+) | N + 2 sheets | For cover sheets |
| Cardstock, white (110lb+) | Enough for N+4 cards at 3.5x2in | For hotel key cards |
| Cardstock, cream (110lb+) | Enough for N+4 cards at 3x2in | For calling cards |
| Adhesive paper (Avery 5163 or similar) | Enough for N+2 labels | For dossier folder labels |
| Staples | ~3 per team (P1, P2, P4 each stapled) | Standard stapler |
| Pens | 2 per team | For solving |

---

## Design Specs

Detailed designs for each prop are in their own files:
- `dossier-cover.md` -- Cover sheet (PR02)
- `evidence-log.md` -- Evidence correlation log (PR07)
- `hotel-key.md` -- Hotel key card (PR08)
- `calling-card.md` -- Calling card (PR09)

Print-ready HTML for production is in `delivery/print/`:
- `puzzles/cover-sheet.html`
- `puzzles/evidence-log.html`
- `props/labels-dossier.html`
- `props/hotel-key-cards.html`
- `props/calling-cards.html`
