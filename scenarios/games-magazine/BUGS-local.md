# BUGS — Games Magazine (Local)

Bugs and issues specific to the Games Magazine scenario. Not backported to `../../BUGS.md` unless they reveal toolkit-level problems.

---

## Open

### BUG-GM-001: P1 grid row 7 has 16 cells (should be 15)
**Severity:** Medium
**File:** `puzzles/P1-lurking-words.md`, line 26
**Description:** Row 7 shows WHISTLE (7) + block + OVERHEAD (8) = 16 cells. A 15x15 grid should have exactly 15 cells per row. Either OVERHEAD needs to be 7 letters or the grid needs reconstruction. This is a symptom of the known grid construction limitation documented in the editorial review.
**Resolution:** Requires professional crossword construction software. Cannot be fixed by manual character editing.

### BUG-GM-002: P3 PUZZLES.md says "8 clues" but puzzle has 10
**Severity:** Low (cosmetic)
**File:** `PUZZLES.md` line 60 vs `puzzles/P3-camouflage-club.md`
**Description:** The brief in PUZZLES.md said the logic grid would have 8 clues with one false. The authored puzzle has 10 clues with one false. The puzzle file is authoritative — the brief was an early estimate.
**Status:** Puzzle is correct. Brief is outdated. No action needed beyond noting the divergence.

### BUG-GM-003: P3 said "nine clues" when there are ten
**Severity:** Low
**File:** `puzzles/P3-camouflage-club.md`, line 47
**Description:** The puzzle text said "One of the nine clues above is false" but there are 10 clues.
**Status:** FIXED — changed to "ten."

### BUG-GM-004: P4 clue #5 said "Unlock a door with this" (KEY) but answer is OWL
**Severity:** High (puzzle-breaking)
**File:** `puzzles/P4-plain-sight.md`, clue 5
**Description:** The solution page lists the 5th hidden object as OWL (to spell C-A-U-M-O-F-L-A-G-E). The clue said "Unlock a door with this (3)" which gives KEY, not OWL. The first letters would become CAUMKFLAGE which is not a word. The clue and solution were inconsistent.
**Status:** FIXED — changed clue to "A nocturnal bird with wide eyes (3)" which gives OWL.

### BUG-GM-005: P7 brief says answer is SECRET, actual puzzle yields UNSEEN
**Severity:** Low (cosmetic, brief vs. puzzle mismatch)
**File:** `PUZZLES.md` line 116 vs `puzzles/P7-spot-the-difference.md` + `puzzles/page-12-solutions.md`
**Description:** The brief in PUZZLES.md says "The 6 changed letters, in order of appearance, spell SECRET." The authored puzzle uses born/burn, lime/line, light/sight, long/lone, tint/tine, bale/bane which spells UNSEEN. The puzzle file and solution page are consistent with each other. The brief was an early design that was revised during authoring.
**Status:** Puzzle is correct. Brief is outdated. No action needed.

---

## Pipeline Divergences Noted

### DIV-001: No commit-per-stage in this scenario
**Description:** The standard pipeline expects a commit after each stage. This scenario had Stages 1-8 completed across multiple sessions without stage-by-stage commits. The CLAUDE.md was never updated to reflect progress beyond Stage 4. This is a common failure mode when the pipeline is run across multiple long sessions.
**Recommendation:** Add a "status audit" skill or prompt that checks CLAUDE.md freshness at the start of each session.

### DIV-002: Print delivery has no website component
**Description:** The standard `/hunt print` skill generates HTML for web display. This scenario needs HTML for print CSS rendering only — no web hosting, no interactivity, no answer submission. The existing HTML files (page-01, page-04, page-05) are print-CSS pages, not web pages.
**Recommendation:** `/hunt print` should support a `print_only: true` flag that skips web-related elements (navigation, answer forms, QR codes).
