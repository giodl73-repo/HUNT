# Pipeline Notes — Publication Format Divergences

This file documents every place the GAMES Magazine workflow diverges from the standard live-event puzzle hunt pipeline. Updated at each stage.

---

## Stage 1: SCOPE

### Divergences

1. **No world/ directory** — Standard pipeline requires world/ with verified reference data for every puzzle domain. Magazine puzzles are self-contained: the puzzle IS the content. There is no encyclopedia or game wiki to reference. The puzzle author invents the content and the mechanism together. **Recommendation:** Make world/ optional in the toolkit. Add a `world: none` flag to SCOPE.md that suppresses world-building stages and warnings.

2. **No answer security** — Standard pipeline encrypts answers (ROT13, Base64, custom cipher) because solvers might find the repo. Magazine solutions are printed on page 12. Plaintext answers are fine. **Recommendation:** Add `answer_security: none` option to SCOPE.md.

3. **No narrator character** — Standard pipeline expects a narrator (The Joker, The Host). Magazine voice is the implicit editorial voice — no character name, no persona. **Recommendation:** Allow `narrator: editorial` as a voice mode.

4. **Page budget replaces puzzle count** — Standard pipeline asks "how many puzzles?" and derives page count. Magazine workflow starts with page count (12) and derives how many puzzles fit. This is backwards from the standard flow. **Recommendation:** Add `constraint: pages` vs `constraint: puzzles` to SCOPE.

5. **Editorial coherence replaces meta design** — No meta. But the 8 puzzles still need to feel like one issue. This is editorial design, not puzzle design. The standard pipeline has no stage for this. **Recommendation:** Add an optional "editorial coherence" stage that replaces meta design for publication workflows.

---

## Stage 2: STRUCTURE (Page Allocation)

### Divergences

6. **Page allocation replaces round structure** — Standard ROUNDS.md defines rounds, puzzle counts per round, meta architecture. For a magazine, the structure IS the page plan: which puzzle gets which pages, how many pages each, what type goes where. ROUNDS.md is repurposed as a page plan. **Recommendation:** Rename to STRUCTURE.md in the toolkit and support both round-based and page-based layouts.

7. **No round progression** — Standard hunts have difficulty ramps per round and across rounds. Magazine has a single difficulty curve across 12 pages. The curve is more like a magazine TOC: accessible early, fiendish late, with a visual breather in the middle. **Recommendation:** Document "editorial pacing" as an alternative to "round progression."

---

## Stage 3: PUZZLE POOL

### Divergences

8. **Page fit is a ranking criterion** — Standard pool ranking uses puzzle quality, mechanism variety, and domain coverage. Magazine adds "page fit": does this puzzle actually fit in its allocated page count? A brilliant logic grid that needs 3 pages fails if it is allocated 1 page. **Recommendation:** Add `page_fit` as a ranking dimension when `constraint: pages` is set.

9. **Type diversity is mandatory, not preferred** — Standard hunts can have multiple puzzles of the same type. A magazine issue MUST have variety: you cannot run three crosswords. **Recommendation:** Add a diversity constraint that prevents duplicate types.

---

## Stage 4: ASSIGNMENT

### Divergences

10. **No answer word coordination** — Standard pipeline coordinates answer words for meta compatibility. No meta means no coordination needed. Each puzzle's answer is independent. **Recommendation:** Make answer coordination optional, triggered by meta presence.

11. **Fictional bylines replace author IDs** — Standard assigns puzzles to real authors (human or AI). Magazine assigns fictional bylines. The same fictional author can write multiple puzzles across issues. **Recommendation:** Support `byline: fictional` mode in PUZZLES.md.

---

## Stage 5: EDITORIAL COHERENCE (Replaces META DESIGN)

### Divergences

12. **New stage type: editorial coherence** — This stage does not exist in the standard pipeline. It checks: do all 8 puzzles feel like the same issue? Is the difficulty curve right? Is type variety sufficient? Does the visual rhythm work (dense, sparse, dense)? **Recommendation:** Create `/hunt coherence` skill for publication workflows.

---

## Stage 6: AUTHORING

### Divergences

13. **Magazine format replaces puzzle page template** — Standard template has: header, narrator intro, puzzle content, worksheet, answer blank, reference hint. Magazine format has: title, difficulty stars, byline, intro paragraph (40-80 words), puzzle content, "Solution on page 12." Different structure. **Recommendation:** Create a `magazine` page template alongside the standard `puzzle-page` template.

14. **Solutions page is a separate deliverable** — Standard puzzles have answers encoded in .claude/ memory. Magazine has a physical page 12 with compact solutions. This is a layout task, not a security task. **Recommendation:** Add `/hunt print solutions` command.

---

## Stage 7: EDITORIAL REVIEW

### Divergences

15. **Word count is a hard constraint** — Standard editorial trims for quality. Magazine editorial trims for SPACE. Every word on the page costs physical room. An intro that runs 100 words when 80 is the max pushes content off the page. **Recommendation:** Add word-count enforcement to `/hunt edit` when page constraints are active.

---

## Stage 8: INTEGRATION (Page Layout)

### Divergences

16. **Page-count verification** — Standard integration checks: all puzzles pass testing, meta works, difficulty curve verified. Magazine integration checks: does everything FIT? Are any pages over or under? Does page 12 have room for all 8 solutions? **Recommendation:** Add layout verification to `/hunt checklist` for publication mode.

---

## Stage 9: DELIVERY BUILD (Print Production)

### Divergences

17. **Magazine print specs differ from standard** — Standard `/hunt print` assumes letter (8.5x11) or A4 with 0.75in margins. Magazine needs 8.375x10.875 trim, 0.125 bleed, 3-column grid, serif body at 10pt. The print skill has no support for custom page sizes, bleed, or column grids. **Recommendation:** Add configurable page specs to `/hunt print`. Support at minimum: page size, bleed, margin, column count, font specs.

18. **Spread layout (facing pages)** — Pages 2-3, 6-7 are spreads (two facing pages designed as one unit). Standard `/hunt print` generates single pages. **Recommendation:** Add spread support to `/hunt print` — two-page layouts with shared gutter.

19. **No QR codes, no answer submission links** — Standard puzzle pages include QR codes linking to website answer submission. Magazine has no website. **Recommendation:** Make QR codes optional in `/hunt print`.

---

## Stage 10: PLATFORM TEST (Print Proof)

### Divergences

20. **Print proof replaces platform test** — Standard tests puzzles in the delivery medium (website, app). Magazine tests by reviewing the printed/PDF output: column alignment, grid sizing, font readability, solution page completeness. **Recommendation:** Add a print-proof checklist to `/hunt print`.

---

## Stage 11: POLISH

### Divergences

21. **Editor's note is a deliverable** — Standard polish checks answer verification, hint system, encoding. Magazine polish includes writing the editor's note (100 words on page 12) and a final copy-edit pass. **Recommendation:** Recognize editor's note as a standard deliverable for publication workflows.

---

## Summary: What a Publication-Mode Pipeline Looks Like

```
Stage 1:  SCOPE (editorial)      → theme, page budget, voice, print specs
Stage 2:  PAGE PLAN              → page allocation (replaces ROUNDS)
Stage 3:  PUZZLE POOL            → candidates ranked by quality + page fit + diversity
Stage 4:  ASSIGNMENT             → select puzzles, assign pages + bylines (no answer coordination)
Stage 5:  EDITORIAL COHERENCE    → replaces META DESIGN — theme unity + difficulty curve
Stage 6:  AUTHORING              → write in magazine format (title, stars, byline, intro, content)
Stage 7:  EDITORIAL REVIEW       → copy-edit with word-count enforcement
Stage 8:  LAYOUT CHECK           → page-count verification (replaces INTEGRATION)
Stage 9:  PRINT BUILD            → generate magazine-spec print output
Stage 10: PRINT PROOF            → review PDF output (replaces PLATFORM TEST)
Stage 11: POLISH                 → final copy-edit + editor's note
```

### New Toolkit Features Needed

1. `world: none` option in SCOPE.md
2. `answer_security: none` option
3. `narrator: editorial` voice mode
4. `constraint: pages` vs `constraint: puzzles`
5. Editorial coherence stage (new skill: `/hunt coherence`)
6. Magazine page template
7. `/hunt print solutions` command
8. Word-count enforcement in `/hunt edit`
9. Configurable print specs (page size, bleed, columns)
10. Spread (facing page) support in `/hunt print`
11. QR codes as optional
12. Print-proof checklist
13. Editor's note as standard deliverable
