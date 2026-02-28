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

**BUG-L03: R3-05 operation count encoding is fragile** — RESOLVED in authoring
- The 12-operation count for R3-05 depends on a specific shutdown/cleanup sequence (D re-toggled, guards A/B closed) that the solver must discover from the procedure log.
- The log must be designed carefully during authoring so the 12-count is unambiguous.
- **Impact:** Medium. Authoring must make the full sequence discoverable.
- **Resolution:** R3-05-emergency-sequence.md reference card now lists all 12 operations
  step by step. The procedure log clues ("thermal before magnetic", "sensor re-secured",
  "thermal and magnetic guards stowed") are explicitly stated. The full 12-step sequence
  is printed on the reference card as the "COMPLETE SEQUENCE" section. Unambiguous.

**BUG-L04: PUZZLES.md R3-05 answer encoding ambiguity (binary vs register count)**
- PUZZLES.md describes R3-05 answer as "12 (final switch state configuration)" and
  discusses binary encoding: "A=ON, B=ON, C=ON, D=OFF → 1100 = 12" but also says
  "procedure register readout = 12."
- The binary reading 1100 only equals 12 if C is OFF (1100₂ = 12), but in the authored
  puzzle C is ON — final state is A=ON, B=ON, C=ON, D=OFF = 1110₂ = 14, not 12.
- In the authored puzzle (R3-05-emergency-sequence.md), the answer is the PROCEDURE
  REGISTER count (12 physical operations), NOT a binary encoding of switch states.
  This is unambiguous and consistent with META-DESIGN.md which uses value 12.
- **Impact:** Low. PUZZLES.md brief text is confusing but the answer value (12) is
  correct. The authored puzzle uses operation count, not binary encoding.
- **Fix:** Optionally clean up PUZZLES.md R3-05 brief to remove the binary encoding
  discussion and clarify that the answer is the register count.

## Closed

**BUG-L03** — closed 2026-02-28. Resolved in R3-05 authoring.
