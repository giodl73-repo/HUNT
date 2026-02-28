# THE GRAND LARCENY — Print Theme

**Font (body):** Courier New — monospaced typewriter font (evokes 1947 police reports)
**Font (headers):** Georgia — serif, authoritative, period-appropriate
**Primary color:** #1a1a1a (near-black — documents are text-heavy, minimal decoration)
**Secondary color:** #8B0000 (dark red — accent only, used sparingly for "CONFIDENTIAL" stamps)
**Background:** #FFFFFF (white — these are official documents)
**Logo:** None. The "CITY POLICE DEPARTMENT" header serves as the visual identity.
**Page border:** Thin single rule (0.5pt) — 0.75in margins on all sides
**Flavor:** "1947 police evidence file. Typewritten documents, institutional headers, carbon-copy look. Nothing flashy. The documents are austere, official, and slightly yellowed in spirit if not in print."

## Document Styles

Each puzzle page has its own document type, but all share the institutional header:

### Police Report (P1)
- Typewriter font (Courier New)
- Double-ruled header box
- Section headers in CAPS, underlined
- Tables with simple borders
- "CONFIDENTIAL" watermark (light grey, diagonal, optional for print)

### Hotel Receipt (P2)
- Monospaced font for columns (Courier New)
- Right-aligned dollar amounts
- Dotted leaders between item and price
- Hotel letterhead (Georgia, centered)
- Horizontal rules between folios

### Floor Plan (P3)
- Clean line drawing (architectural style)
- Room labels in small sans-serif (Arial Narrow or similar)
- North arrow and scale bar
- Reference markers as circled numbers
- Light grey fill for corridors/hallways
- Separate diagram per floor level

### Witness Statement (P4)
- Typewriter font for transcript
- Italic for stage directions ([pause])
- Bold for speaker names
- Indented paragraph style
- Case header box matching P1

### Evidence Log (Meta)
- Same institutional header as P1
- Fill-in blanks as underscored lines
- Grid boxes for final answer
- Minimal decoration

## CSS Overrides

```css
/* Base document style */
body {
    font-family: 'Courier New', monospace;
    font-size: 11pt;
    line-height: 1.4;
    color: #1a1a1a;
    margin: 0.75in;
}

/* Institutional header */
.case-header {
    border: 2px double #1a1a1a;
    padding: 12px 16px;
    text-align: center;
    font-family: Georgia, serif;
    margin-bottom: 24px;
}

/* Section headers */
h2 {
    font-family: 'Courier New', monospace;
    text-transform: uppercase;
    text-decoration: underline;
    font-size: 12pt;
    margin-top: 20px;
}

/* Tables */
table {
    border-collapse: collapse;
    width: 100%;
    font-size: 10pt;
}
td, th {
    border: 1px solid #999;
    padding: 4px 8px;
    text-align: left;
}

/* Answer blanks */
.answer-blank {
    display: inline-block;
    width: 24px;
    height: 24px;
    border: 1px solid #1a1a1a;
    margin: 2px;
    text-align: center;
    vertical-align: middle;
}

/* Print settings */
@media print {
    body { margin: 0; padding: 0.75in; }
    .page-break { page-break-before: always; }
    @page { size: letter; margin: 0; }
}
```
