# Calling Card -- Design Spec

**Prop ID:** PR09
**Type:** flavor + META component (physical prop required for meta extraction)
**Material:** Heavy cardstock (110lb or thicker), cream or light grey paper
**Dimensions:** 3 x 2 inches (slightly smaller than business card)
**Print:** Double-sided, black ink
**Position in dossier:** Loose in folder (not stapled or clipped)
**Print-ready file:** `delivery/print/props/calling-cards.html`

---

## Purpose

The calling card is the meta's key. It contains the four extraction positions (3, 1, 2, 6) that the solver uses to pull one letter from each of the four exhibit answers, producing the final answer.

Narratively, the card was left at the crime scene by the thief -- a gentleman's calling card, a signature. It is presented as evidence (the back has an evidence tag). The solver must figure out that the four numbers on the front tell them which letter to take from each answer word.

---

## Front Design

```
+----------------------------------+
|                                  |
|              [diamond]           |
|                                  |
|    A gentleman never leaves      |
|    without a signature.          |
|                                  |
|       3  .  1  .  2  .  6       |
|                                  |
|              [diamond]           |
|                                  |
+----------------------------------+
```

**Typography:**
- Diamond symbols: Unicode U+25C6 (black diamond), 16pt, centered
- Motto: Georgia serif italic, 8pt, centered, 1.3 line height
- Numbers: Courier New monospace, 14pt, bold, letter-spacing 3px
- Dots between numbers: middot (U+00B7), colored #999 (light grey)

**Design intent:** The numbers are presented as a mysterious sequence -- part of the thief's signature. They are prominent (14pt bold) but not explained. The solver must connect these four numbers to the four exhibits.

---

## Back Design

```
+----------------------------------+
|                                  |
|    EVIDENCE TAG                  |
|    Item: Calling card            |
|    Found: Gallery floor,         |
|           near the frame         |
|    Logged: 9:47 PM               |
|    Case No.: 47-1011-GL          |
|                                  |
+----------------------------------+
```

**Typography:**
- "EVIDENCE TAG": Courier New, 8pt, bold, uppercase
- Body fields: Courier New, 7pt, 1.2 line height, left-aligned

**Design intent:** The back reinforces that this is a piece of evidence. "Found: Gallery floor, near the frame" connects it to the crime scene. "Logged: 9:47 PM" is consistent with the timeline (police arrived at 9:45, evidence collection began shortly after).

---

## How the Numbers Work (design reference, NOT printed)

The four numbers correspond to the four exhibits in order (A, B, C, D):

| Position on card | Exhibit | Instruction | Extraction |
|------------------|---------|-------------|------------|
| 3 | A (P1) | Take letter 3 from answer | ROT13: F |
| 1 | B (P2) | Take letter 1 from answer | ROT13: Y |
| 2 | C (P3) | Take letter 2 from answer | ROT13: V |
| 6 | D (P4) | Take letter 6 from answer | ROT13: G |

Result (ROT13): F-Y-V-G (4 letters)

The solver must realize:
1. The four numbers on the calling card are extraction positions
2. They map to the four exhibits in order (A=first number, B=second, etc.)
3. "Take the indicated letter" on the evidence log connects to these numbers

---

## Production Notes

- Print on cream or light grey cardstock to visually distinguish from the hotel key card (which is white)
- Cut to 3 x 2 inches -- slightly smaller than the hotel key card, which helps teams identify them as different props
- Print fronts on one side, flip on short edge, print backs on reverse
- The print-ready HTML (`calling-cards.html`) produces 12 card fronts per page in a 3x4 grid

**Print yield:** 12 fronts per letter-size sheet. For N teams plus 4 spares, print ceil((N+4)/12) sheets of fronts and the same of backs.

---

## Differentiation from Hotel Key Card

| Feature | Hotel Key Card (PR08) | Calling Card (PR09) |
|---------|----------------------|---------------------|
| Paper color | White | Cream or light grey |
| Size | 3.5 x 2 in | 3 x 2 in (slightly smaller) |
| Font style | Hotel typography (Georgia + Courier) | Elegant (Georgia italic) |
| Front content | Room number, hotel name | Motto, four numbers |
| Back content | Hotel instructions | Evidence tag |
| Purpose | P3 starting point | Meta extraction key |

---

## Canon Verification

| Fact | Source | Verified |
|------|--------|----------|
| Numbers 3, 1, 2, 6 | meta/META-DESIGN.md (final extraction) | Yes |
| Found on gallery floor | Consistent with theft location (Room 200) | Yes |
| Logged 9:47 PM | timeline.md (police arrived 9:45, collection would start ~9:47) | Yes |
| Case No. 47-1011-GL | Consistent across all documents | Yes |
| Exhibit order A-B-C-D | ROUNDS.md, cover sheet | Yes |
