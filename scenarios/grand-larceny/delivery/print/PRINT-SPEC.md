# Print Specifications -- THE GRAND LARCENY

## Design Language

**Era:** 1947 institutional documents. Typewriter-era bureaucracy meets Art Deco hotel elegance.

**Guiding principle:** Every printed page looks like a real document from 1947. Police reports are police reports. Hotel receipts are hotel receipts. The solver never sees a "puzzle page" -- they see evidence.

---

## Paper and Page

| Spec | Value |
|------|-------|
| Page size | US Letter (8.5 x 11 inches) |
| Margins | 0.75 inches all sides (documents live in a comfortable text block) |
| Print mode | Single-sided, black ink only |
| Paper stock (documents) | White, 20lb standard |
| Paper stock (cover sheet) | White, 65lb cardstock or heavier |
| Paper stock (key cards) | White, 110lb cardstock or heavier |
| Paper stock (calling cards) | Cream or light grey, 110lb cardstock or heavier |

---

## Typography

### Primary: Courier New (monospace)

The typewriter font. Used for all body text, field data, tables, and worksheet sections. Evokes 1947 police reports typed on a Smith Corona.

| Context | Size | Weight | Notes |
|---------|------|--------|-------|
| Body text | 10-11pt | Regular | All document content |
| Section headers | 10-12pt | Bold | ALL CAPS, underlined |
| Tables | 9-10pt | Regular | Tight spacing for data density |
| Worksheets | 10pt | Regular | Answer blanks and extraction grids |
| Field labels | 10pt | Bold | "Case No.:", "Statement:", etc. |

### Secondary: Georgia (serif)

The institutional header font. Used for department names, hotel names, and formal headings. Authoritative, period-appropriate.

| Context | Size | Weight | Notes |
|---------|------|--------|-------|
| "CITY POLICE DEPARTMENT" | 14-16pt | Bold | Centered in header box, letter-spacing 2-3px |
| "THE GRAND HOTEL" | 11-14pt | Bold | Hotel letterhead, letter-spacing 2px |
| Subtitles | 12-13pt | Regular/Italic | "Investigative Division", "Guest Folio" |
| Calling card motto | 8pt | Italic | Elegant, centered |

### Tertiary: Arial Narrow (sans-serif)

Used only on the floor plan (P3) for room labels and reference markers. Architectural notation convention.

| Context | Size | Notes |
|---------|------|-------|
| Room labels | 8-9pt | Inside or adjacent to room outlines |
| Reference markers | 8pt | Inside circled numbers |
| Scale bar text | 7pt | "1 inch = approx. 20 feet" |

---

## Color Palette

| Swatch | Hex | Usage |
|--------|-----|-------|
| Near-black | #1a1a1a | All text, borders, rules |
| Dark red | #8B0000 | "CONFIDENTIAL" stamps, "STATUS: OPEN" on labels. Accent only. |
| Medium grey | #999999 | Table borders, subtle separators, middot between numbers |
| Light grey | #f0f0f0 | Table header backgrounds |
| Off-white | #fafaf5 | Worksheet/correlation box backgrounds |
| White | #FFFFFF | Page background |

No color is required for solving. All extraction mechanisms work in grayscale. The dark red accent is purely atmospheric.

---

## Document Header Box

Used on the cover sheet, P1, evidence log, and P4. Consistent across all police department documents.

```
+==================================================================+
|  (3px double rule border)                                         |
|                                                                    |
|            CITY POLICE DEPARTMENT                                  |
|            (Georgia, 14-16pt, bold, letter-spacing 2-3px)          |
|                                                                    |
|            INCIDENT REPORT / WITNESS STATEMENT / etc.              |
|            (Georgia, 12pt, regular)                                |
|                                                                    |
|  Case No.: 47-1011-GL          Date: October 12, 1947             |
|  (Courier New, 9pt)                                                |
|                                                                    |
+==================================================================+
```

Padding: 14-20px. Margin below: 20-30px.

---

## Hotel Header Box

Used on P2 (Hotel Receipt). Different style from police header -- this is hotel stationery.

```
+==================================================================+
|  (3px double rule border)                                         |
|                                                                    |
|                      THE GRAND HOTEL                               |
|                   1 Grand Avenue                                   |
|                                                                    |
|             GUEST FOLIO -- CONSOLIDATED BILLING                    |
|          Gala Evening: Saturday, October 11, 1947                  |
|                                                                    |
+==================================================================+
```

Same border treatment, but hotel typography (Georgia, centered, more generous spacing).

---

## Rules and Borders

| Element | Style |
|---------|-------|
| Header boxes | 3px double rule (#1a1a1a) |
| Section dividers | Single horizontal rule, 1px solid #ccc |
| Table borders | 1px solid #999 |
| Worksheet boxes | 1px solid #1a1a1a, light background |
| Answer cells | 1.5-2px solid #1a1a1a, square (24-36px per side) |
| Page border | None. The margin provides the frame. |

---

## Answer Blanks

Two styles used across the documents:

### Underscored blanks (for writing in answers)
```
___ ___ ___ ___ ___ ___ ___
```
22-24px wide per blank, 1.5px bottom border, 8px letter-spacing between blanks. Used in the evidence log for exhibit answers.

### Boxed cells (for final answers)
```
+---+---+---+---+
|   |   |   |   |
+---+---+---+---+
```
24-36px square cells, 1.5-2px border. Used at the bottom of each puzzle page and the evidence log for THE METHOD.

---

## Extraction Markers (P1 specific)

P1 uses underlined words with one boxed letter per word. The HTML implementation:

- **Underlined word:** `<span>` with `text-decoration: underline`
- **Boxed letter within the word:** `<span>` with 2px solid border, 1-4px padding, bold, light yellow background (#f9f9f0)

This creates a visual pattern: normal text... underlined word with one letter in a box... normal text. The solver collects the seven boxed letters.

---

## Special Elements

### CONFIDENTIAL Watermark (optional)
- Text: "CONFIDENTIAL"
- Font: Georgia, 60pt
- Color: rgba(139, 0, 0, 0.06) -- barely visible
- Rotation: -35 degrees
- Position: Fixed center of page
- Usage: Cover sheet and P1 only

### Floor Plan (P3)
- Unique layout among the documents
- Clean line drawings in architectural drafting style
- Light grey fill (#f0f0f0) for corridors and hallways
- Room labels in Arial Narrow, small caps
- Reference markers as circled numbers
- North arrow and scale bar
- Four separate level diagrams: B, 1F, 2F, 3F
- NOTE: The ASCII art in the puzzle source file is the content spec; the print version should ideally be rendered as clean vector art or carefully formatted monospaced text

---

## Print CSS

All print-ready HTML files include `@media print` rules:

```css
@media print {
  body { margin: 0; padding: 0.75in; }
  .page-break { page-break-before: always; }
  @page { size: letter; margin: 0; }
}
```

Documents that span 2 pages use `<div class="page-break">` at the break point.

---

## File Index

### Puzzle documents (letter paper)
| File | Content | Pages |
|------|---------|-------|
| `puzzles/cover-sheet.html` | Case briefing cover sheet | 1 |
| `puzzles/P1-police-report.html` | Exhibit A: Police Report | 2 |
| `puzzles/P2-hotel-receipt.html` | Exhibit B: Hotel Receipt | 2 (to be created) |
| `puzzles/P3-floor-plan.html` | Exhibit C: Floor Plan | 1-2 (to be created) |
| `puzzles/P4-witness-statement.html` | Exhibit D: Witness Statement | 2 (to be created) |
| `puzzles/evidence-log.html` | Evidence Correlation Log | 1 |

### Props (cardstock)
| File | Content | Yield per sheet |
|------|---------|-----------------|
| `props/labels-dossier.html` | Dossier folder labels | 6 per sheet |
| `props/hotel-key-cards.html` | Hotel key card fronts | 12 per sheet |
| `props/calling-cards.html` | Calling card fronts | 12 per sheet |
