# THE GRAND LARCENY -- Local Bug Log

Bugs found during Stage 10 (Platform Test). For this scenario only.

---

## Open

### BUG-GL-001: P2 print-ready HTML not built
**Severity:** Blocker
**Found:** Stage 10 platform test
**File:** `delivery/print/puzzles/P2-hotel-receipt.html`
**Description:** The layout spec (`layout-P2-hotel-receipt.md`) and source markdown (`puzzles/P2-hotel-receipt.md`) are complete, but the print-ready HTML has not been created. Required before printing dossiers.

### BUG-GL-002: P3 print-ready HTML not built
**Severity:** Blocker
**Found:** Stage 10 platform test
**File:** `delivery/print/puzzles/P3-floor-plan.html`
**Description:** The layout spec exists. The ASCII floor plans need to be rendered as clean HTML (ideally with CSS-drawn diagrams or embedded SVG). ASCII in Courier New works as minimum viable but is not polished.

### BUG-GL-003: P4 print-ready HTML not built
**Severity:** Blocker
**Found:** Stage 10 platform test
**File:** `delivery/print/puzzles/P4-witness-statement.html`
**Description:** The layout spec (`layout-P4-witness-statement.md`) is complete. HTML rendering needs to match the police transcript format with header box, speaker labels, stage directions, and the 32-sentence analysis grid.

### BUG-GL-004: submit.html not built
**Severity:** Blocker
**Found:** Stage 10 platform test
**File:** `delivery/site/submit.html`
**Description:** The site spec (`SITE-SPEC.md`) defines the submit page in detail. Needs form with Case ID + THE METHOD fields, SHA-256 answer validation (client-side), correct/incorrect display, and narrative payoff on success.

### BUG-GL-005: P1 extraction words partially underlined in HTML
**Severity:** Medium
**Found:** Stage 10 platform test
**File:** `delivery/print/puzzles/P1-police-report.html`
**Description:** Extraction words #2 ("annual") and #3 ("persons") have their `extract-word` span starting mid-word. Only the suffix gets underlined: `ann<span>uUal</span>` and `per<span>soSns</span>`. The full word should be inside the underline span. Words #1, #4, #5, #6, #7 appear to be correctly wrapped.

### BUG-GL-006: P4 analysis section ending reference mismatch
**Severity:** Low
**Found:** Stage 10 platform test
**File:** `puzzles/P4-witness-statement.md`
**Description:** The analysis section says the main statement ends with "...paying closer attention" but that phrase appears in Delacroix's final remark AFTER the 32-sentence extractable block. The 32 sentences end with "The thought of anything being wrong did not cross my mind." The enumeration grid (32 rows) is definitive and correct, but the prose instruction could be tightened.

### BUG-GL-007: Kessler timeline discrepancy (9:28 vs 9:30)
**Severity:** Low
**Found:** Stage 10 platform test
**File:** `world/systems/timeline.md`
**Description:** The detailed movements table for Kessler says "9:30 | Room 204 | Key card log" but the key card log table in the same file says 9:28 PM. The police report (P1) says 9:28 PM. The 9:28 figure is canonical. The "9:30" should be corrected to "9:28" for consistency.

---

## Closed

(none yet)
