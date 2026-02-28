# Layout Spec: P3 -- The Floor Plan

**Exhibit:** C
**Document type:** Architectural survey / floor plans
**Source:** `puzzles/P3-floor-plan.md`
**Print-ready:** `puzzles/P3-floor-plan.html` (to be created; needs graphic treatment)
**Pages:** 1-2 (not stapled, not folded)
**Stock:** White 20lb letter paper (consider 11x17 tabloid if available)

---

## Document Identity

This is an architectural survey prepared at police request: clean drafted lines, room labels, reference markers, north arrow, scale bar. It looks like something a surveyor would produce -- technical, precise, minimal. Four floor levels (Basement, 1F, 2F, 3F) are presented as separate diagrams on the page.

This is the most visually distinct document in the dossier. Where the other exhibits are text-heavy typewriter documents, this one is primarily a diagram. It should feel like unfolding a blueprint from the evidence file.

---

## Header

### Architectural survey header box
- Same 3px double-ruled border as other documents
- "THE GRAND HOTEL" -- Georgia 14pt bold, centered
- "1 Grand Avenue" -- Georgia 11pt, centered
- "ARCHITECTURAL SURVEY -- ALL LEVELS" -- Georgia 12pt, centered
- "Prepared for: City Police Department" -- Courier New 9pt
- "Case No.: 47-1011-GL" -- Courier New 9pt
- "Surveyor: J. Wellman & Associates, Architects" -- Courier New 9pt
- "Date: October 12, 1947" -- Courier New 9pt
- Below header: one line of instruction text re: reference markers and north orientation

---

## Floor Plan Layout

### Arrangement
Four floor diagrams arranged vertically on the page(s):
1. Third Floor (3F) -- top
2. Second Floor (2F) -- second
3. First Floor (1F) -- third
4. Basement (B) -- bottom

This top-to-bottom arrangement mirrors the building's vertical structure and matches the thief's route (entered from roof, descended).

### Single-page option (letter size)
- Each floor diagram: approximately 2.25 x 3.5 inches
- Tight but readable with 8pt labels
- Header compresses to minimal height
- Diagrams in a 2x2 grid (3F and 2F on top, 1F and B on bottom) -- alternative layout

### Two-page option (letter size)
- Page 1: Header + 3F + 2F (the upper floors, where the main action is)
- Page 2: 1F + B + reference key + route log worksheet
- Each diagram: approximately 3 x 4.5 inches, more readable

### Tabloid option (11x17)
- All four diagrams on one large sheet
- Each diagram: approximately 4 x 5 inches
- Most architectural feel -- unfolds from the folder
- Header at top, reference key and worksheet at bottom

**Recommended:** Two-page letter size. Most teams will have standard printers.

---

## Diagram Style

### Walls and boundaries
- Outer walls: 2px solid black lines
- Interior walls: 1.5px solid black
- Doors: gaps in wall lines (3-4mm gap)
- The connecting door between Gallery (200) and Linen Closet (206): a thin line (1px) that is visually present but subtle. Shown as a door (gap) in the wall between the two rooms. Not labeled as a passage. The solver must notice this.

### Room labels
- Font: Arial Narrow, 8-9pt, regular
- Format: Room name + room code
  - "SARGENT GALLERY" / "200"
  - "LINEN CLOSET" / "206"
  - "PENTHOUSE SUITE" / "300"
- Position: centered within room outline
- Guest names shown in parentheses where applicable: "(Ashworth)", "(Fontaine)", "(Kessler)", "(Delacroix)"

### Corridors and hallways
- Filled with light grey (#f0f0f0)
- Labeled: "2F HALL", "B-HALL", etc.

### Service elevator shaft
- Drawn as a double-bordered rectangle (representing the shaft)
- Labeled "ELV" with "2F-ELEV", "B-ELEV", etc. below
- Consistent position across all four floors (center of building), so the solver can trace the vertical path

### Fire escape
- Shown on 3F only, on the north face
- Drawn as an external staircase symbol (zigzag lines descending)
- Labeled "FIRE ESCAPE (north face)"
- Arrow indicating "to ground level alley"

### Stairs
- Main staircase: shown on 1F as a rectangle with up/down arrows, labeled "STAIR" with "to 2F / 3F"
- Kitchen stairs: shown on 1F (kitchen) and B (staff corridor) with down/up arrows

### North arrow and scale bar
- North arrow: top-right corner of the page (or top-right of each diagram)
- Scale bar: bottom of page, "1 inch = approx. 20 feet" or similar approximate scale

---

## Reference Markers

Circled numbers placed at key locations by the surveyor. These are the extraction mechanism.

| Marker | Location | Room | A1Z26 Value | Letter |
|--------|----------|------|-------------|--------|
| (20) | Roof Access entrance | 303 | 20 | T |
| (9) | Gallery center | 200 | 9 | I |
| (9) | 2F Hall junction near elevator | 2F-HALL | 9 | -- |
| (13) | Linen Closet interior | 206 | 13 | M |
| (5) | Laundry, near alley exit | B02 | 5 | E |

### Marker rendering
- Circle: 14-16px diameter, 1.5px solid border
- Number inside: Arial Narrow 8pt, centered
- Position: at the entrance or center of the room, as noted in the reference key

### Important: the 2F-HALL marker (9)
There is a second marker (9) at the 2F hallway junction near the service elevator. This is NOT on the thief's path (the thief went Gallery --> Linen Closet --> elevator, not through the hallway). Its presence is a potential red herring that the solver must evaluate. The route log asks for 4 stops, which constrains the path.

---

## Reference Marker Key (printed on plan)

A table below the diagrams:

```
REFERENCE MARKER KEY

Marker | Location                          | Description
-------|-----------------------------------|----------------------------------
  (5)  | B02 (Laundry)                    | Near alley exit door
  (9)  | 200 (Sargent Gallery)            | Center of gallery
  (9)  | 2F-HALL                          | Junction near service elevator
  (13) | 206 (Linen Closet)              | Interior, near connecting door
  (20) | 303 (Roof Access)               | At locked door to roof
```

---

## Investigator's Task (worksheet)

At the bottom of the final page:

```
INVESTIGATOR'S TASK

A hotel key card was found on the fire escape landing. The card
reads ROOM 303 (see physical evidence in your dossier).

Trace the thief's route through the hotel:
1. Entry point: Where did the thief enter?
2. Path to gallery: How did they reach Room 200?
3. Escape from gallery: How did they leave without using the hall?
4. Exit route: How did they leave the building?

ROUTE LOG
Stop 1: _____________ (room)    Marker: ____
Stop 2: _____________ (room)    Marker: ____
Stop 3: _____________ (room)    Marker: ____
Stop 4: _____________ (room)    Marker: ____

Convert markers to letters (A=1, B=2, ... Z=26):

+---+---+---+---+
|   |   |   |   |
+---+---+---+---+
(4 letters)
```

---

## The Key Discovery

The solver's aha moment: discovering the unlabeled connecting door between the Sargent Gallery (200) and the Linen Closet (206). This door is shown on the floor plan as a thin line / gap in the shared wall between the two rooms. It is noted in the text below the 2F diagram: "The thin line between rooms 200 and 206 indicates a connecting door. Per hotel records, this door is typically locked from the gallery side."

This note is present in the source markdown. In the print version, it should appear as a small footnote or annotation on the 2F diagram, in a smaller font (8pt). The presence of this note makes the door discoverable without being overly hidden.

---

## Vertical Access Summary (printed on plan)

A small table showing all vertical connections:

```
VERTICAL ACCESS SUMMARY

Connection       | From              | To           | Access
-----------------|-------------------|--------------|------------------
Main Staircase   | 1F Lobby (100)    | 2F / 3F Hall | Open to all
Service Elevator | B-ELEV            | All floors   | Staff key required
Kitchen Stairs   | 1F Kitchen (103)  | B-HALL       | Staff only
Fire Escape      | 3F Roof (303)     | Ground alley | External, locked
```

---

## Print Notes

- This is the most print-sensitive document. The floor plan diagrams must be clean and readable.
- The ASCII art in the source markdown is the content specification. For production:
  - **Minimum viable:** Render the ASCII art in Courier New at a size that fits the page. This works but is not beautiful.
  - **Better:** Convert to a clean graphic (SVG or high-resolution PNG) using a vector drawing tool, matching the ASCII layout exactly. Room labels in Arial Narrow, walls as clean lines.
  - **Best:** Hand-draw or CAD-draft the floor plans in architectural style, then scan or export as high-resolution images.
- Do NOT fold the floor plan pages. They should lie flat in the folder.
- If using 2 pages, do NOT staple them -- the solver needs to lay them side by side.
