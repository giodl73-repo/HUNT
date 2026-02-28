# Scenario 6 — GAMES MAGAZINE SPREAD

**Type:** Print publication (magazine)
**Scale:** 8 puzzles across 12 pages
**Structure:** No rounds — single editorial package
**Pipeline:** Modified (publication workflow, not live-event workflow)

---

## Development Goals

1. **Publication constraints** — everything must fit in exactly 12 pages. Page count is money. Tests whether the toolkit handles hard layout constraints (word count, grid size, page allocation).
2. **Editorial format** — each puzzle needs: title, difficulty rating (★–★★★★★), byline, intro paragraph, puzzle content, and solution (in back section). Tests a completely different content format from live-event puzzles.
3. **Print-first, print-only** — no website, no props, no event. The magazine page IS the complete experience. Tests `/hunt print` at publication quality: bleed, trim, column grids, typography.
4. **Self-contained puzzles** — no props, no QR codes, no online components. Every puzzle must be 100% solvable from the printed page alone.
5. **Puzzle type diversity** — a real magazine issue has variety: crossword, cryptic, logic grid, wordplay, visual, mathematical. Tests whether the toolkit can support varied puzzle types in one package.
6. **New: publication workflow** — this scenario uses a MODIFIED pipeline suited to editorial production, not live-event production. Document what changes are needed.

---

## The Spread

**Publication:** GAMES Magazine (fictional issue — Vol. 47, No. 3)
**Theme:** "HIDDEN IN PLAIN SIGHT" — all 8 puzzles share a thematic thread: things concealed within things. Not a meta-puzzle hunt — a themed editorial package. The puzzles are independent but tonally unified.

**Tone:** The authoritative warmth of a classic puzzle magazine. Witty intros. Elegant grids. The sense that the editor loves this.

**Page allocation (12 pages):**
```
Page 1:    Section title spread — "Hidden in Plain Sight" intro (full page)
Pages 2–3: P1 — Crossword (full spread, 15×15 grid)
Page 4:    P2 — Cryptic clues (single page, 10 clues)
Page 5:    P3 — Logic grid (single page)
Pages 6–7: P4 — Visual puzzle (full spread — images, diagrams)
Page 8:    P5 — Word puzzle (single page)
Page 9:    P6 — Mathematical puzzle (single page)
Page 10:   P7 — Short puzzle (half page) + P8 teaser/intro (half page)
Page 11:   P8 — Mini puzzle (full page, more complex than it looks)
Page 12:   Solutions for all 8 puzzles + editor's note
```

---

## Editorial Requirements

Each puzzle page must include:
- **Title** (evocative, 2–4 words)
- **Difficulty** (★ to ★★★★★)
- **Byline** ("By [fictional author name]")
- **Intro paragraph** (40–80 words — sets up the puzzle, adds flavor)
- **Puzzle content** (grid, clues, diagrams — formatted for column grid)
- **Solution reference** ("Solution on page 12")

Page 12 (solutions) includes:
- Compact solutions for all 8 puzzles
- **Editor's note** (100 words — reflects on the theme, teases next issue)

---

## Modified Pipeline for Publication

This scenario uses a publication workflow, not a live-event workflow. Document differences as you find them.

**Suggested modifications:**
- Stage 1 (SCOPE): Define the editorial theme and page budget, not hunt narrative
- Stage 2 (STRUCTURE): Page allocation plan, not rounds/metas
- Stage 3 (PUZZLE POOL): 12+ candidates ranked by type diversity + page fit
- Stage 4 (ASSIGNMENT): Select 8, assign pages, assign fictional bylines
- Stage 5 (META DESIGN): Skip — no meta. Replace with: editorial coherence check (do all 8 puzzles feel like the same issue?)
- Stage 6 (AUTHORING): Write all 8 puzzles
- Stage 7 (EDITORIAL): Tighter than live-event editorial — every word on the page counts
- Stage 8 (INTEGRATION): Page layout review — does everything fit? Page 12 solutions check
- Stage 9 (DELIVERY BUILD): `/hunt print` at publication spec — bleed, trim, column grid
- Stage 10 (PLATFORM TEST): Print proof — does it look like a real magazine spread?
- Stage 11 (POLISH): Final copy-edit pass

**Key question:** What pipeline changes should become permanent toolkit features for publication-format hunts? Log as bugs/suggestions.

---

## Print Specifications

Magazine print specs (apply to `/hunt print` output):
- **Page size:** 8.375 × 10.875 in (standard magazine trim)
- **Bleed:** 0.125 in all sides
- **Safety margin:** 0.25 in inside bleed
- **Column grid:** 3-column (each ~2.25 in wide, 0.1875 in gutter)
- **Body font:** Serif, 9.5–10.5pt, 12–13pt leading
- **Header font:** Sans-serif or display
- **Color:** Full color OR carefully designed for B&W reproduction
- **Resolution:** All images 300dpi minimum (use SVG where possible)

---

## Instructions for Opus

Work through the modified 11-stage pipeline. Key differences:

- **No meta** — this is not a hunt, it's an editorial package. Replace meta design with editorial coherence review.
- **No props, no website** — print only. All energy goes into page layout and puzzle quality.
- **Page budget is law** — every puzzle must fit its allocated pages. If a puzzle runs over, cut it, don't expand the page count.
- **Fictional bylines** — invent 4–5 puzzle author names used across the 8 puzzles. Brief bios on page 12.
- **Difficulty curve** — the 8 puzzles should range from ★★ (accessible) to ★★★★★ (fiendish). Intentional variety.
- **Push `/hunt print`** — this is the primary test of the print skill at publication quality. Note every gap between what the skill describes and what a real magazine needs.
- **Document the pipeline diffs** — every place the publication workflow diverges from the live-event workflow, note it in a file `scenarios/games-magazine/PIPELINE-NOTES.md`. This is valuable toolkit research.

Log toolkit gaps in `../../BUGS.md`. Commit at each stage. Claim callsign **Golf** at the end.

---

## Files to Create

```
scenarios/games-magazine/
├── SCENARIO.md           ← this file
├── CLAUDE.md
├── SCOPE.md              ← editorial theme + page budget
├── ROUNDS.md             ← page allocation plan (not rounds)
├── PUZZLES.md            ← puzzle registry with page assignments
├── PIPELINE-NOTES.md     ← document publication workflow diffs
├── world/                ← not applicable — skip or minimal
├── puzzles/              ← 8 puzzle files
├── reviews/
├── tests/
└── delivery/
    └── print/
        ├── spread.html   ← full 12-page layout
        ├── page-01.html
        ├── page-02-03.html (spread)
        └── ...
```
