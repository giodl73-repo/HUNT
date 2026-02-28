# Hidden in Plain Sight — GAMES Magazine Spread

**Scenario 6** — testing publication-format pipeline (print magazine, no event).

## Summary

| Field | Value |
|-------|-------|
| **Publication** | GAMES Magazine, Vol. 47, No. 3 |
| **Section** | "Hidden in Plain Sight" — 12-page puzzle spread |
| **Scale** | 8 puzzles, no meta, 12 pages exactly |
| **Audience** | Magazine readers (puzzle enthusiasts, casual to expert) |
| **Format** | Print only — HTML with print CSS |
| **Theme** | Things concealed within things |
| **Difficulty** | ★★ to ★★★★★ |

## Voice Rules

1. Second person ("you") for instructions
2. Present tense in intros
3. No exclamation marks
4. Intros: 40-80 words exactly
5. Stars for difficulty: ★★ to ★★★★★
6. "Solution on page 12" on every puzzle page
7. Fictional bylines from the author roster

## Fictional Authors

| Name | Specialty |
|------|-----------|
| Margaret Huang | Crosswords, word puzzles |
| R.J. Calder | Cryptics, wordplay |
| Nadia Koresh | Logic, mathematics |
| Felix Ortega | Visual, creative formats |
| Sam Whitfield | Mixed, accessible |

## Page Plan

```
Page 1:     Title + editorial intro
Pages 2-3:  P1 Crossword (★★★) — Margaret Huang
Page 4:     P2 Cryptic (★★★★) — R.J. Calder
Page 5:     P3 Logic grid (★★★) — Nadia Koresh
Pages 6-7:  P4 Visual (★★) — Felix Ortega
Page 8:     P5 Word puzzle (★★★★) — Margaret Huang
Page 9:     P6 Mathematical (★★★★★) — Nadia Koresh
Page 10:    P7 Short (★★) + P8 intro (★★★★★) — Sam Whitfield / R.J. Calder
Page 11:    P8 Mini puzzle continued (★★★★★) — R.J. Calder
Page 12:    Solutions + editor's note
```

## Files

| File | Stage | Status |
|------|-------|--------|
| `SCENARIO.md` | — | ✅ Brief |
| `CLAUDE.md` | — | ✅ This file |
| `SCOPE.md` | Stage 1 | ✅ Complete |
| `PIPELINE-NOTES.md` | Ongoing | ✅ Active |
| `ROUNDS.md` | Stage 2 | ✅ Page plan (repurposed from rounds) |
| `PUZZLE-POOL.md` | Stage 3 | ✅ 14 candidates, 8 selected |
| `PUZZLES.md` | Stage 4 | ✅ 8 puzzles assigned to pages |
| `puzzles/` | Stage 6 | ✅ All 8 puzzles authored + title page + solutions page |
| `reviews/editorial-review.md` | Stage 7 | ✅ Complete (6 approved, 2 minor revision) |
| `reviews/integration-check.md` | Stage 8 | ✅ Complete (all pages fit, one minor flag) |
| `delivery/THEME.md` | Stage 9 | ✅ Magazine aesthetic, fonts, colors, CSS |
| `delivery/print/PRINT-SPEC.md` | Stage 9 | ✅ Magazine print specifications |
| `delivery/print/layout-guide.md` | Stage 9 | ✅ Per-page layout guide |
| `tests/platform-test.md` | Stage 10 | ✅ Print proof checklist + reader simulation |

## Key Divergences from Standard Pipeline

- No world/ directory (puzzles are self-contained)
- No meta (editorial coherence replaces meta design)
- No answer security (solutions on page 12)
- Page budget is the hard constraint (not puzzle count)
- Print specs are magazine-grade (not letter/A4)

## Stage Progress

| Stage | Name | Status | Notes |
|-------|------|--------|-------|
| 1 | SCOPE | ✅ Done | Theme, page budget, voice, print specs |
| 2 | STRUCTURE | ✅ Done | Page plan (replaces ROUNDS) |
| 3 | PUZZLE POOL | ✅ Done | 14 candidates, 8 selected |
| 4 | ASSIGNMENT | ✅ Done | 8 puzzles assigned to pages + bylines |
| 5 | EDITORIAL COHERENCE | ✅ Done | Replaces META DESIGN — 6/6 checks pass |
| 6 | AUTHORING | ✅ Done | All 8 puzzles + title page + solutions page |
| 7 | EDITORIAL REVIEW | ✅ Done | 6 approved, 2 minor revision (grid construction) |
| 8 | INTEGRATION | ✅ Done | All pages fit, solutions verified |
| 9 | DELIVERY BUILD | ✅ Done | THEME.md + PRINT-SPEC.md + layout-guide.md |
| 10 | PLATFORM TEST | ✅ Done | Print proof complete — 8/8 checks pass, 3 minor issues for polish |

**Known construction gaps:** P1 crossword grid, P6 number grid, and P8 word search grid all need professional construction software to verify validity. The mechanisms, theme, and editorial format are sound. See `reviews/editorial-review.md` for details.

**Bugs:** See `BUGS-local.md` for scenario-specific issues (7 logged, 2 fixed).

**Stage 10 findings:** Platform test passed with 3 minor issues for Stage 11 polish: P4 intro under 40-word minimum (34 words), P3 .md intro says "remaining eight" (should be "nine"), P5 essay grammar mismatch ("a single MICRODOTS"). See `tests/platform-test.md`.

## Toolkit References

- Principles: `../../toolkit/PRINCIPLES.md`
- Skills: `../../toolkit/skills/`
- Templates: `../../toolkit/templates/`
