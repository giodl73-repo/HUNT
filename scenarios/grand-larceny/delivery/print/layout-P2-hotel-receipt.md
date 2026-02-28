# Layout Spec: P2 -- The Hotel Receipt

**Exhibit:** B
**Document type:** Hotel billing statement (consolidated guest folios)
**Source:** `puzzles/P2-hotel-receipt.md`
**Print-ready:** `puzzles/P2-hotel-receipt.html` (to be created from source)
**Pages:** 2 (stapled)
**Stock:** White 20lb letter paper

---

## Document Identity

This is a hotel billing statement from 1947: columnar accounting, right-aligned dollar amounts, dotted leaders between item descriptions and prices. The hotel's letterhead is at the top -- more elegant than the police header, reflecting The Grand Hotel's Art Deco character. The accounting department prepared this at police request. Every line item has a timestamp, a service category, an item description, and a charge.

---

## Page 1 Layout

### Hotel Letterhead Header Box (top of page)
- 3px double-ruled border
- Centered content:
  - "THE GRAND HOTEL" -- Georgia 14pt bold, letter-spacing 2px
  - "1 Grand Avenue" -- Georgia 11pt regular
  - blank line
  - "GUEST FOLIO -- CONSOLIDATED BILLING" -- Georgia 12pt regular
  - "Gala Evening: Saturday, October 11, 1947" -- Georgia 10pt italic
  - blank line
  - "Prepared by: Accounts Department" -- Courier New 9pt
  - "Authorized by: Mr. A. Hargrove, General Manager" -- Courier New 9pt
  - "For: City Police Department, Case No. 47-1011-GL" -- Courier New 9pt
- Margin below header: 20px

### Introductory note
One paragraph, Courier New 10pt: "The following is a complete accounting of all charges incurred by the four persons of interest..."

### Folio 202 -- Mrs. Vivian Ashworth
- Folio header: Courier New 10pt bold, "FOLIO 202 -- Mrs. Vivian Ashworth"
- Charges table:

```
| Time     | Service    | Item                         | Charge  |
|----------|------------|------------------------------|---------|
| 7:00 PM  | Restaurant | Gala dinner (fixed menu)     |   $8.50 |
| 7:15 PM  | Restaurant | Champagne, glass             |   $1.50 |
| ...      | ...        | ...                          |    ...  |
```

- Table format: 4 columns, Courier New 9pt
- Column alignment: Time (left), Service (left), Item (left), Charge (right-aligned)
- Dollar amounts right-aligned, always with 2 decimal places
- Subtotal row: bold, item column says "Dining & Drinks subtotal"
- Additional charges (room rate, newspaper) below subtotal
- **FOLIO TOTAL** row: bold, double underline on amount
- Horizontal rule below each folio

### Folio 203 -- Mr. Lucien Fontaine
Same table format as Folio 202.

### Page Break
After Folio 203, insert page break.

---

## Page 2 Layout

### Folio 204 -- Mr. Heinrich Kessler
Same table format. Includes the telephone charge ($3.15 for 7 min at $0.45/min).

### Folio 205 -- Miss Colette Delacroix
Same table format. Notably the most modest bill.

### Section: ACCOUNTS DEPARTMENT NOTES
- Short paragraph noting all charges verified
- The key hint: "charges timestamped between 9:00 PM and 9:30 PM may be of particular interest given the reported theft window"
- Courier New 10pt, slightly indented or in a lightly bordered box

### Section: INVESTIGATOR'S ANALYSIS (worksheet)
- Bordered box: 1px solid #1a1a1a, light background (#fafafa)
- Instruction text: "Something does not add up..." (sets tone)
- Guidance: examine cents values of gap-period charges
- Four extraction blocks, one per folio:

```
FOLIO 202 (Ashworth) -- gap: 9:15 to 9:45 PM
   Charges during gap:
   9:18 PM  -- Cigarettes, house brand        $0.12
   9:25 PM  -- Coat retrieval fee             $0.15
   Cents: ___  ___
```

- Conversion instruction: "Convert cents to letters (A=01, B=02, C=03, ... Z=26)"
- Final conversion grid:

```
Ashworth:    12 = ___    15 = ___
Fontaine:    03 = ___
Kessler:     01 = ___    20 = ___    09 = ___
Delacroix:   15 = ___    14 = ___
```

- "Read the letters in folio order (202, 203, 204, 205)"
- Answer grid: 8 cells, 28px square, centered
- Label: "(8 letters)"

---

## Receipt Formatting Details

### Column widths (approximate)
| Column | Width | Alignment |
|--------|-------|-----------|
| Time | 70px | Left |
| Service | 80px | Left |
| Item | Remaining | Left |
| Charge | 65px | Right |

### Dollar amount formatting
- Always show dollar sign: `$8.50`, `$0.12`
- Always two decimal places
- Right-aligned within the Charge column
- Totals in bold, optionally double-underlined

### Period-correct receipt conventions
- Line items listed chronologically by timestamp
- Subtotals for dining/drinks category
- Non-dining charges (room rate, laundry, newspaper) listed below subtotal
- Final total labeled "FOLIO [room] TOTAL"
- No tax line (1947, hotel taxes varied)

### The suspicious charges
The gap-period charges are intentionally small and unusual:
- $0.12 (cigarettes) -- plausible but small. Cents = 12 = L.
- $0.15 (coat retrieval) -- plausible. Cents = 15 = O.
- $0.03 (garment pressing) -- suspiciously cheap. Cents = 03 = C.
- $0.01 (beer) -- absurdly cheap. Cents = 01 = A.
- $0.20 (shoe shine, evening) -- plausible. Cents = 20 = T.
- $0.09 (aspirin) -- plausible. Cents = 09 = I.
- $0.15 (postcard) -- plausible. Cents = 15 = O.
- $0.14 (package wrapping) -- plausible. Cents = 14 = N.

The beer at $0.01 is the biggest tell -- no beer costs a penny. This should be visually identical to the other line items; the solver notices the absurdity.

---

## Voice Notes

- Accounting voice: impersonal, precise, columnar
- No narrative -- just data
- The introductory paragraph and accounts department notes are the only prose
- The investigator's analysis section shifts to detective voice (still terse, still factual)

---

## Print Notes

- Page 1 holds the header, intro, and 2 folios (Ashworth and Fontaine)
- Page 2 holds 2 folios (Kessler and Delacroix), accounts notes, and the analysis worksheet
- If tight on space, the analysis section can start at the bottom of page 2 and run onto a third page -- but aim for 2 pages
- The HTML file needs to match the monospaced columnar style of `P1-police-report.html` but with hotel letterhead instead of police header
- Staple the two pages together at the top-left corner
