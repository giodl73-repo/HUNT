# IRONFALL — Local Bug Tracker

## Open

### BUG-001: P02 answer mismatch
**Severity:** High
**Stage:** 6 (Authoring)
**Description:** PUZZLES.md and META-DESIGN.md specify P02 answer as DHRYY (ROT13 = QUELL), extracting position 2 = U (ROT13: H). The authored P02 file (`puzzles/P02-forge-guide.md`) produces DRESS (ROT13: QERFF) instead. The crafting recipes and extraction in P02 need to be revised to produce QUELL as the answer.
**Impact:** Meta extraction table row 2 would break. Super-meta code letter 2 would be wrong.
**Fix:** Revise P02 recipe selection and extraction to produce QUELL.

### BUG-002: P03 brief-vs-implementation deviation
**Severity:** Low (informational)
**Stage:** 6
**Description:** P03 brief specified achievement bitmask → Nth-letter-of-Mark-name extraction. The Nth-letter extraction cannot produce the letter 'D' (needed for ORDER) from any Mark name. The authored P03 uses ASCII-in-padding extraction instead. The mechanism is cleaner and the answer is verified correct (ORDER). Brief should be updated to match implementation.
**Impact:** None — puzzle works correctly.

### BUG-003: P06 brief-vs-implementation deviation
**Severity:** Low (informational)
**Stage:** 6
**Description:** P06 brief specified "Nth letter of each enemy name (N=sighting number)." No enemy name starts with 'L', so sighting 1 cannot produce the first letter of LOTUS. The authored P06 uses bestiary-number-to-A1Z26 extraction instead. Answer is verified correct (LOTUS).
**Impact:** None — puzzle works correctly.

### BUG-004: Achievements extraction table incomplete
**Severity:** Low
**Stage:** 2 (World data)
**Description:** The achievements.md extraction affordance table only covers Marks 1-12. Marks 13-16 are not listed. The wrapping behavior for Mark 10 ("Ghost Runner" → position 10 = R) appears inconsistent with manual letter counting (GHOSTRUNNER has 11 chars, position 10 = E, position 11 = R). The table should clarify whether spaces count and extend to all 16 Marks.
**Impact:** Any puzzle relying on Marks 13-16 extraction would need to compute positions independently.

### BUG-005: P01 answer mismatch (USHER vs UMBRA)
**Severity:** High
**Stage:** 6 (Authoring) / 4 (Assignment)
**Description:** PUZZLES.md says P01 answer is HFURE (ROT13 = USHER). META-DESIGN.md says P01 answer is HZOEN (ROT13 = UMBRA). The authored P01 produces USHER, matching PUZZLES.md but not META-DESIGN.md. META-I depends on UMBRA[2]=M to produce REALM. With USHER[2]=S, META-I would produce REALF. The super-meta is unaffected because USHER[1]=UMBRA[1]=U.
**Impact:** META-I breaks (spells REALF instead of REALM). Super-meta unaffected.
**Fix:** Either revise P01 to produce UMBRA (matching META-DESIGN), or update META-DESIGN to use USHER (requires redesigning META-I extraction). META-DESIGN is the higher-authority document since it was verified for the full meta chain.

## Closed

(none)
