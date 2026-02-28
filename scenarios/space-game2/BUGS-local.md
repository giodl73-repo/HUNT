# DEAD RECKONING v2 — Local Bug Log

Route bugs here during concurrent agent work. Do NOT write to the shared `../../BUGS.md`.

---

## Open

**BUG-L01: v2 answer values supersede v1 meta values**
- Old META-DESIGN.md had answers (2.618 GHz, 147 degrees, etc.) from the v1-derived design.
- New PUZZLES.md + META-DESIGN.md use different values (2.340 GHz, 213 degrees, etc.) for the widget-first redesign.
- The old values in the console-widget-catalog Part 5 readiness map still reference v1 puzzle names (R1-01 Carrier Wave, etc.).
- Part 5 of console-widget-catalog.md is now historical and does not reflect v2 puzzle designs.
- **Impact:** Low. Part 5 was a readiness assessment for v1 puzzles, not a data dependency.
- **Fix:** Optionally update Part 5 or add a note that it reflects v1 only.

**BUG-L02: R2-META mechanism needs tightening during authoring**
- The R2-META "first alert station → starting position = 3" derivation is described at high level in META-DESIGN.md but the exact extraction mechanism (how 6 numeric values produce the number 3) needs a cleaner single-path derivation during Stage 6 authoring.
- Current design offers two candidate extraction methods; only one should survive to authoring.
- **Impact:** Medium. Does not block Stage 4/5 but must be resolved before Stage 6.

**BUG-L03: R3-05 operation count encoding is fragile**
- The 12-operation count for R3-05 depends on a specific shutdown/cleanup sequence (D re-toggled, guards A/B closed) that the solver must discover from the procedure log.
- The log must be designed carefully during authoring so the 12-count is unambiguous.
- **Impact:** Medium. Authoring must make the full sequence discoverable.

## Closed

(none)
