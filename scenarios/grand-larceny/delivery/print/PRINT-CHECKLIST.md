# Print Checklist — THE GRAND LARCENY

## Puzzles (N teams x pages each)

| Item | Pages | Copies per team | File |
|------|-------|----------------|------|
| [ ] Cover Sheet | 1 | 1 | `puzzles/cover-sheet.html` |
| [ ] P1 — The Police Report | 2 | 1 | `puzzles/P1-police-report.html` |
| [ ] P2 — The Hotel Receipt | 2 | 1 | (to be created from puzzle source) |
| [ ] P3 — The Floor Plan | 1-2 | 1 | (to be created — needs vector art) |
| [ ] P4 — The Witness Statement | 2 | 1 | (to be created from puzzle source) |
| [ ] Evidence Log (meta page) | 1 | 1 | `puzzles/evidence-log.html` |

**Total puzzle pages per team:** 9-10

## Props

| Item | Format | Copies per team | File |
|------|--------|----------------|------|
| [ ] Dossier folder labels | Adhesive, 1 per team | 1 | `props/labels-dossier.html` |
| [ ] Hotel key cards | Card stock, 3.5x2in | 1 | `props/hotel-key-cards.html` |
| [ ] Calling cards | Card stock, 3x2in | 1 | `props/calling-cards.html` |

## Other Supplies

| Item | Qty | Notes |
|------|-----|-------|
| [ ] Manila letter-size folders | N + 2 spare | Standard manila, tab closure |
| [ ] Pens | 2 per team | For solving |
| [ ] Scrap paper | Optional | Teams may want scratch paper |

## Print Summary

For N teams:
- Puzzle pages: 9-10 pages x N = **9N-10N pages** (white letter paper)
- Cover sheets: N pages (heavier stock recommended)
- Key cards: N cards (card stock, cut)
- Calling cards: N cards (card stock, cut)
- Folder labels: N labels (adhesive paper, cut)

**Total print jobs:** ~12N pages + cutting

## Assembly Order (per team)

1. Print and staple P1 (2 pages)
2. Print and staple P2 (2 pages)
3. Print P3 (1-2 pages, do not fold)
4. Print and staple P4 (2 pages)
5. Print evidence log (1 page)
6. Print cover sheet (heavy stock)
7. Stack in order: cover on top, then P1, P2, P3, P4, evidence log on bottom
8. Place stack in labeled manila folder
9. Drop hotel key card and calling card loose into folder
10. Close folder

**Assembly time per team:** ~15 minutes (once everything is printed and cut)

## Notes on Document-Style Printing

- P1 should look like a real police incident report — typewriter font, institutional header
- P2 should look like a real hotel billing statement — monospaced columns, right-aligned amounts
- P3 is the most print-sensitive puzzle — the floor plan must be clean and readable at letter size. Consider printing at 11x17 if available.
- P4 should look like a real typed transcript — typewriter font, speaker labels in bold
- Evidence log should match P1 style — same institutional header, same font
- All documents should feel like they came from the same case file

## Gap Identified

The `/hunt print` skill assumes worksheet-style puzzle pages (title + content + answer blank). For document-style puzzles like THE GRAND LARCENY, each page needs its own unique layout matching its document type. The standard puzzle page template does not accommodate this variety — each document must be designed as a custom HTML file.

**BUG to log:** `/hunt print` needs a "document mode" or "custom layout" option for hunts where puzzles ARE documents rather than worksheets with puzzle content. The current template (header + flavor + content + answer blank) is too rigid for narrative-embedded puzzles.
