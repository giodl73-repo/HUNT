# Layout Spec: P1 -- The Police Report

**Exhibit:** A
**Document type:** Police incident report
**Source:** `puzzles/P1-police-report.md`
**Print-ready:** `puzzles/P1-police-report.html`
**Pages:** 2 (stapled)
**Stock:** White 20lb letter paper

---

## Document Identity

This is an official police incident report: bureaucratic, institutional, typed on a police department typewriter. It has the visual weight of government paperwork -- header box, case number, reporting officer, classification. Every line says "this is official."

---

## Page 1 Layout

### Header Box (top of page)
- 3px double-ruled border
- Centered content:
  - "CITY POLICE DEPARTMENT" -- Georgia 14pt bold, letter-spacing 2px
  - "INCIDENT REPORT" -- Georgia 12pt regular
  - Case No., Date, Reporting Officer, Classification, Location, Item -- Courier New 9pt, one per line
- The painting details (title, artist, medium, dimensions, value) are in the header box
- Margin below header: 20px

### Section: SUMMARY OF EVENTS
- Header: Courier New 10pt bold, UPPERCASE, underlined
- Body: Courier New 10pt, standard paragraph
- Two paragraphs of summary text
- **Extraction words embedded:** The seven underlined words with boxed letters appear throughout the body text (see extraction treatment below)

### Section: PERSONS OF INTEREST
- Header: Courier New 10pt bold, UPPERCASE, underlined
- Four subsections, each for one suspect:

#### Per-suspect block:
```
### 1. Mrs. Vivian Ashworth -- Room 202

**Statement (10:30 PM):**  [paragraph of alibi]

**Corroborating:**  [bullet list]

**Contradicting:**  [bullet list]

**Unaccounted window:**  [time range, duration]
```

- Suspect name: Courier New 10pt bold
- Field labels ("Statement:", "Corroborating:", etc.): bold
- Body text: Courier New 10pt regular
- Horizontal rule between suspects (1px solid #ccc)

### Page Break
After suspect #4 (Delacroix), insert page break.

---

## Page 2 Layout

### Section: PHYSICAL EVIDENCE
- Numbered list (1-5) of physical evidence items
- Item 4 includes a table (key card log):

| Room | Last entry before 9:15 | Next entry after 9:15 |
|------|------------------------|-----------------------|
| 202  | 6:30 PM                | 10:15 PM              |
| etc. | ...                    | ...                   |

Table: 9pt, 1px borders, #f0f0f0 header background

### Section: CRITICAL DEDUCTION
- Body text with five numbered clues in a shaded box:
  - Clue box: 3px left border (#999), light background (#f8f8f8), 8px padding, 9pt font
  - Each clue is a `<p>` with `<strong>Clue N:</strong>` prefix

### Section: ANSWER EXTRACTION (worksheet box)
- Bordered box: 1px solid #1a1a1a, light background (#fafafa), 10-14px padding
- Instruction: "Seven words in this report are underlined, each with one letter in a box. Find all seven..."
- Extraction table (7 rows):

| # | Word | Boxed letter |
|---|------|-------------|
| 1 | ____________ | ___ |
| ... | ... | ... |

- Answer grid: 7 cells, 28px square, centered
- Label: "(7 letters)"

---

## Extraction Marker Treatment

Seven words throughout the report text are marked with the extraction formatting:

1. **Saturday** (in Summary) -- "S" boxed
2. **annual** (in Summary) -- "U" boxed
3. **persons** (in Persons section) -- "S" boxed
4. **approximately** (in Ashworth statement) -- "P" boxed
5. **Evening** (in Fontaine statement) -- "E" boxed
6. **mechanical** (in Physical Evidence) -- "C" boxed
7. **question** (in Critical Deduction) -- "T" boxed

### HTML rendering:
```html
<span class="extract-word">
  <span class="extract-letter">S</span>aturday
</span>
```

- `.extract-word`: text-decoration: underline
- `.extract-letter`: display: inline-block; border: 2px solid #1a1a1a; padding: 1px 4px; font-weight: bold; background: #f9f9f0

The underline makes the word noticeable. The box around one letter within the word makes the extraction letter unmistakable. On the printed page, these look like a typewriter with someone going back and drawing a box around specific letters in ink -- consistent with the police evidence marking metaphor.

---

## Voice Notes

- All text in typewriter voice: terse, factual, third-person
- No contractions
- Timestamps always in "HH:MM PM" format
- Room references always include room number: "The Brass Bar (Room 102)"
- Evidence labels always bold: "**Corroborating:**", "**Contradicting:**"
- Horizontal rules between major sections

---

## Print Notes

- Page 1 is text-dense. Font size drops to 10pt to fit all four suspects on the page. If it runs long, the page break goes after suspect #2 or #3 instead of #4.
- The existing HTML file (`P1-police-report.html`) is print-ready and verified. It renders all extraction markers correctly.
- Staple the two pages together at the top-left corner.
