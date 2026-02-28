# Hidden in Plain Sight — Print Theme

**Font (body):** Georgia, serif — fallback: Times New Roman, serif
**Font (headers):** Helvetica Neue, sans-serif — fallback: Arial, sans-serif
**Font (puzzle content):** Courier New, monospace (for grids and fixed-width content)
**Primary color:** #1a1a2e (dark navy — headings, rules)
**Secondary color:** #c4a35a (warm gold — star ratings, accents, drop caps)
**Background:** #ffffff (white — clean magazine page)
**Text color:** #2d2d2d (soft black — body text)
**Logo:** GAMES Magazine masthead (text-only, Helvetica Neue Bold)
**Page border:** Thin 0.5pt rule at trim edge (bleeds off page)
**Flavor:** Classic puzzle magazine. Clean, confident typography. No decorative elements — the puzzles ARE the decoration. Think GAMES Magazine circa 1990-2000: dense, intelligent, elegant.

## Print Specs

- **Page size:** 8.375 x 10.875 in (standard US magazine trim)
- **Bleed:** 0.125 in all sides
- **Safety margin:** 0.25 in from trim on all sides
- **Column grid:** 3 columns, each 2.25 in wide, 0.1875 in gutter
- **Body font size:** 10pt / 13pt leading
- **Header font size:** 18pt (puzzle titles), 12pt (section headers)
- **Star rating:** Unicode filled stars (★) in gold (#c4a35a), 14pt
- **Byline:** Italic, 9pt
- **Footer:** "Solution on page 12" — 8pt, right-aligned
- **Page numbers:** 8pt, bottom center

## CSS Overrides

```css
@page {
  size: 8.375in 10.875in;
  margin: 0;
}

body {
  font-family: Georgia, 'Times New Roman', serif;
  font-size: 10pt;
  line-height: 13pt;
  color: #2d2d2d;
}

h1 {
  font-family: 'Helvetica Neue', Arial, sans-serif;
  font-size: 18pt;
  font-weight: 700;
  color: #1a1a2e;
  letter-spacing: 0.05em;
}

.difficulty {
  color: #c4a35a;
  font-size: 14pt;
}

.byline {
  font-style: italic;
  font-size: 9pt;
  color: #666;
}

.solution-ref {
  font-size: 8pt;
  text-align: right;
  color: #999;
}

.grid {
  font-family: 'Courier New', monospace;
  font-size: 9pt;
  line-height: 11pt;
}

.page-number {
  font-size: 8pt;
  text-align: center;
  color: #999;
}
```
