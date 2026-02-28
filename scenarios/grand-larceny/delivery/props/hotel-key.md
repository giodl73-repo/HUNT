# Hotel Key Card -- Design Spec

**Prop ID:** PR08
**Type:** component (physical prop required for P3)
**Material:** Heavy cardstock (110lb or thicker), white
**Dimensions:** 3.5 x 2 inches (standard business card size)
**Print:** Double-sided, black ink
**Position in dossier:** Loose in folder (not stapled or clipped to anything)
**Print-ready file:** `delivery/print/props/hotel-key-cards.html`

---

## Purpose

The hotel key card is a physical prop that provides the starting point for P3 (The Floor Plan). Without it, the thief's entry point is ambiguous. The card tells the solver: the thief entered through Room 303, Roof Access.

It also reinforces the fiction -- this is a real hotel key card, recovered as evidence from the fire escape landing.

---

## Front Design

```
+----------------------------------+
|                                  |
|      THE GRAND HOTEL             |
|      ---------------             |
|                                  |
|      ROOM 303                    |
|      ROOF ACCESS                 |
|                                  |
|      Guest: ___________          |
|                                  |
|          [pointing hand]         |
|                                  |
+----------------------------------+
```

**Typography:**
- "THE GRAND HOTEL": Georgia serif, 11pt, bold, letter-spacing 2px
- Divider: thin horizontal rule, 40% width, centered
- "ROOM 303": Courier New monospace, 16pt, bold
- "ROOF ACCESS": 8pt, uppercase, letter-spacing 1px
- "Guest:": 8pt, with underscored blank line
- Pointing hand symbol: Unicode U+261E (white right pointing index), 14pt, centered

**Layout:** Centered vertically and horizontally. Clean, institutional. No decorative borders beyond the card edge.

---

## Back Design

```
+----------------------------------+
|                                  |
|  This key provides access to     |
|  the roof level via the north    |
|  stairwell on the third floor.   |
|                                  |
|  Please return to front desk     |
|  upon departure.                 |
|                                  |
|  THE GRAND HOTEL                 |
|  1 Grand Avenue                  |
|                                  |
+----------------------------------+
```

**Typography:**
- Body text: Georgia serif, 7.5pt, 1.3 line height, left-aligned
- Hotel footer: centered, italic, 7pt

**Key information embedded in back text:**
- "north stairwell on the third floor" -- subtly points to the fire escape on the north face of the building, confirming the entry route
- "roof level" -- confirms this is the roof access, not a guest room

---

## Production Notes

- Print on heavy cardstock (110lb minimum) so the card has substance and feels like a real hotel key
- Cut to 3.5 x 2 inches along borders
- Optional: round corners with a corner punch for added authenticity
- Print fronts on one side of the sheet, flip on the short edge, print backs on the reverse
- The print-ready HTML file (`hotel-key-cards.html`) produces 12 card fronts per page in a 3x4 grid

**Print yield:** 12 fronts per letter-size sheet. For N teams plus 4 spares, print ceil((N+4)/12) sheets of fronts and the same of backs.

---

## Puzzle Function

| Element | Function |
|---------|----------|
| ROOM 303 | Starting point for P3 path trace. Solver locates Room 303 on the 3F floor plan. |
| ROOF ACCESS | Confirms this is the locked door to the roof, adjacent to the fire escape. |
| "north stairwell" (back) | Subtle hint that the north face has an external route (the fire escape). |
| Blank "Guest:" field | Flavor. The thief did not check in -- this key was obtained by other means. |

---

## Canon Verification

| Fact | Source | Verified |
|------|--------|----------|
| Room 303 = Roof Access | world/systems/layout.md (3F table) | Yes |
| "north stairwell" / north face fire escape | layout.md (Fire Escape: External, North face) | Yes |
| Fire escape: 3F roof (303) to ground alley | layout.md (Vertical connections) | Yes |
| Locked from inside | layout.md (3F table: "Locked door to flat roof") | Yes |
| 1 Grand Avenue | WORLD.md canon rules | Yes |
