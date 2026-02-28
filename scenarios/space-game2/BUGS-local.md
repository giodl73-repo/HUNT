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

**BUG-L02: R2-META mechanism needs tightening during authoring** — RESOLVED in R2-META authoring
- The R2-META "first alert station → starting position = 3" derivation is described at high level in META-DESIGN.md but the exact extraction mechanism (how 6 numeric values produce the number 3) needs a cleaner single-path derivation during Stage 6 authoring.
- Current design offers two candidate extraction methods; only one should survive to authoring.
- **Impact:** Medium. Does not block Stage 4/5 but must be resolved before Stage 6.
- **Resolution:** R2-META-response.md selects parity-based mechanism: odd = ANOMALY,
  lowest odd value = first alert, controlling station roster position = starting position.
  Single clean derivation path. Both BUG-L02 and BUG-L06 resolved.
- **Status:** CLOSED.

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

**BUG-L05: PUZZLES.md R2-02 says node 7 is "Comms Computer" — world data says "Security Computer"**
- PUZZLES.md R2-02 narrative: "Someone injected unauthorized data into the ODN through node 7 —
  the Comms Computer node."
- data-tables.md ODN topology: NODE-07 = Security Computer (SEC). NODE-05 = Comms Computer (OPS).
- The authored R2-02-data-breach.md uses the correct world data label: node 7 = SECURITY.
- The narrative revelation in R2-02 says the breach came through "NODE 07 — SECURITY COMPUTER"
  with payload class "SENSOR DATA OVERRIDE" routed to NODE 15 (SENSOR COMPUTER).
- The R2-02 narrative that the breach was used "to forge sensor readings" still holds —
  the Security Computer has access to internal sensor feeds and access control databases.
- **Impact:** Low. Answer value (7) is unchanged. Only the PUZZLES.md label is wrong.
- **Fix:** Update PUZZLES.md R2-02 narrative to say "Security Computer" instead of "Comms Computer."

**BUG-L06: R2-META derivation still has two candidate mechanisms** — RESOLVED in R2-META authoring
- BUG-L02 noted that META-DESIGN.md describes two candidate extraction methods for
  producing starting position = 3 from the six R2 feeder values.
- Now that all 6 R2 puzzles are authored, the R2-META authoring (Stage 6) needs to pick one.
- The narrative revelations establish a clear timeline:
  GAP +00:03 (R2-04 shields), GAP +00:14 (R2-02 sensor suppression),
  GAP +00:18 (R2-05 relay failure), GAP +00:22 (R2-01 EPS reroute),
  GAP +00:25 (R2-06 reactor state), GAP +01:08 (R2-03 heat source).
- **Resolution:** R2-META-response.md authored with parity-based mechanism. See BUG-L02.
- **Status:** CLOSED.

**BUG-L07: R3-03 and R3-05 timeline summaries were not chronological**
- R3-03 section 7 listed GAP +01:22 before GAP +00:42 (puzzle order, not chronological).
- R3-05 section 7 had the same issue plus GAP +01:55 before GAP +01:42.
- **Impact:** Major. Confusing to solvers reading the timeline reconstruction.
- **Resolution:** Fixed in Stage 7 editorial. Both timelines reordered to chronological.
- **Status:** CLOSED.

**BUG-L08: R3-05 binary encoding section self-contradictory**
- Section 6 "Binary Encoding of Final State" started computing binary, then cut itself
  off mid-sentence ("C = ON (0... wait)") to correct course.
- **Impact:** Minor. Developer-facing data section, not solver-facing.
- **Resolution:** Fixed in Stage 7 editorial. Section rewritten as "Answer Encoding"
  with clean explanation.
- **Status:** CLOSED.

**BUG-L09: META-DESIGN.md station-to-roster table is stale**
- META-DESIGN.md R2-META section has a station-to-roster mapping table with positions
  that don't match the actual duty roster (OPS=2, COMPUTER/ENG=6, ENVIRO=4, TAC=1, ENG=5).
- The authored R2-META-response.md uses the correct roster (OPS=0/TORRES, COMPUTER=3/KWON,
  ENVIRO=6/PARK, TAC=1/NAKAMURA, ENG=4/REEVES).
- The final answer (position 3 = KWON) is correct in both, but the intermediate table
  in META-DESIGN.md is from an earlier design draft.
- **Impact:** Low. META-DESIGN.md is a design document, not solver-facing.
- **Fix:** Update META-DESIGN.md table to match the authored roster positions.

**BUG-L10: CLAUDE.md duration estimates are stale (v1 or pre-reference-card)**
- CLAUDE.md "Duration" field says "8-12 hours (team), 15-20 hours (solo)."
- Stage 10 platform test simulation estimates: 1-2h (expert team), 3-5h (novice team), 5-8h (solo).
- The v2 reference-card design makes puzzles significantly faster than v1 classification puzzles.
- **Impact:** Major. Sets incorrect expectations for playtesters and event scheduling.
- **Fix:** Update CLAUDE.md duration to match Stage 10 estimates.

**BUG-L11: HINTS.md R2-02 near-solution says "Comms Computer" for node 7**
- HINTS.md R2-02 near-solution hint: "The breach entry point is node 7 -- the Comms Computer node."
- Should say "Security Computer" per world data and authored puzzle file.
- Extension of BUG-L05 (PUZZLES.md has same issue).
- **Impact:** Major. Solver-facing hint text has wrong label (answer value 7 is correct).
- **Fix:** Update HINTS.md R2-02 near-solution to say "Security Computer."

## Closed

**BUG-L02** — closed 2026-02-28. Resolved in R2-META authoring (parity-based mechanism).
**BUG-L03** — closed 2026-02-28. Resolved in R3-05 authoring.
**BUG-L06** — closed 2026-02-28. Resolved in R2-META authoring (same as BUG-L02).
**BUG-L07** — closed 2026-02-28. Fixed in Stage 7 editorial.
**BUG-L08** — closed 2026-02-28. Fixed in Stage 7 editorial.
