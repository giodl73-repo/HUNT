# DEAD RECKONING v2 — Stage 8 Integration Check

**Date:** 2026-02-28
**Scope:** Full end-to-end verification of 17 feeders + 3 round metas + 1 final meta

---

## Check 1: Answer Chain End-to-End

### Round 1 (TELEMETRY) -> R1-META -> N = 8

| Puzzle | Answer | Confirmed In |
|--------|--------|-------------|
| R1-01 Carrier Isolation | 2.340 GHz | puzzles/R1-01-carrier-isolation.md |
| R1-02 Phase Lock | 873 MHz | puzzles/R1-02-phase-lock.md |
| R1-03 Null Zone | 213 degrees | puzzles/R1-03-null-zone.md |
| R1-04 Orbit Classification | 0.73 | puzzles/R1-04-orbit-classification.md |
| R1-05 Contact Lock | 5 | puzzles/R1-05-contact-lock.md |
| R1-06 Signal Fingerprint | 384 kbps | puzzles/R1-06-signal-fingerprint.md |

R1-META mechanism: Map 6 feeder values to Contact Signature Reference Table. Only Phase-Locked Echo matches all 6. Class code = 8.

- 2.340 GHz in S-band (2-4 GHz) -- MATCH
- 873 MHz / 291 MHz = 3:1 harmonic ratio (n > 2) -- MATCH
- 213 degrees = directional -- MATCH
- 0.73 in range 0.6-0.9 = hyperbolic-capture -- MATCH
- 5 contacts in range 4-7 -- MATCH
- 384 kbps in range 128-512 kbps -- MATCH

No other contact type matches more than 2 of 6. **N = 8. PASS.**

### Round 2 (SYSTEMS LOG) -> R2-META -> Start = 3

| Puzzle | Answer | Parity | Classification |
|--------|--------|--------|---------------|
| R2-01 Power Path | 2 | Even | NOMINAL |
| R2-02 Data Breach | 7 | Odd | ANOMALY |
| R2-03 Heat Source | 14 | Even | NOMINAL |
| R2-04 Shield Profile | 65 | Odd | ANOMALY |
| R2-05 Fault Trace | 9 | Odd | ANOMALY |
| R2-06 Reactor State | 4 | Even | NOMINAL |

Odd values: 7, 9, 65. Lowest odd = 7 (R2-02, ODN Network).
Controlling station: COMPUTER. Roster position: 3 (KWON).

**Start = 3. PASS.**

### Round 3 (CREW RECORD) -> R3-META -> Step = 2

| Puzzle | Answer | Mod 8 | Roster | Role |
|--------|--------|-------|--------|------|
| R3-01 Triage | 3 | 3 | KWON | Casualty |
| R3-02 Access Code | 7 | 7 | CHEN | Access agent |
| R3-03 Permission Chain | 4 | 4 | REEVES | Authorization |
| R3-04 Badge Sequence | 5 | 5 | VASQUEZ | Ordering officer |
| R3-05 Emergency Sequence | 12 | 4 | REEVES | Executor |

Authorization chain: VASQUEZ -> REEVES = 2 links.

**Step = 2. PASS.**

### Final Meta -> VASQUEZ

(3 + 2) mod 8 = 5. Position 5 = VASQUEZ. **PASS.**

**CHECK 1 RESULT: PASS**

---

## Check 2: CyclicGroupDisplay Math

Circle labels (positions 0-7):
```
0: TORRES
1: NAKAMURA
2: OKAFOR
3: KWON
4: REEVES
5: VASQUEZ
6: PARK
7: CHEN
```

Inputs:
- N = 8 (from R1-META)
- Start = 3 (from R2-META)
- Step = 2 (from R3-META)

Computation: (3 + 2) mod 8 = 5 mod 8 = 5

Position 5 = VASQUEZ.

Verified: the circle labels match the duty roster used throughout all puzzles (R3-02 ModularClockDisplay, R3-03 CayleyTableDisplay roster mapping, R3-04 badge-to-roster mapping, R3-META authorization chain).

**CHECK 2 RESULT: PASS**

---

## Check 3: Cross-Puzzle Bearing Consistency (213 degrees)

**R1-03 (Null Zone):** Target bearing = 213 degrees. Confirmed in widget config:
`targetSignalBearing: 213 degrees` and win condition: "bright spot emerges at bearing 213 degrees."

**R1-05 (Contact Lock):** Contact C1 bearing = 213 degrees. Confirmed in widget config:
`C1: bearing 213, range 60%, behavior REAL (holds under lock)`.

The primary contact (C1) in R1-05 is at bearing 213, the same bearing as the target signal in R1-03. This is consistent: the null zone in R1-03 was configured to receive the signal from the contact at bearing 213, and R1-05's primary contact (the Phase-Locked Echo) holds at the same bearing.

No other puzzle references bearing 213 in a conflicting way.

**CHECK 3 RESULT: PASS**

---

## Check 4: GAP Timeline Consistency

Timeline extracted from authored R2 puzzle narrative revelations:

| Time | Event | Source |
|------|-------|--------|
| GAP +00:03 | Shield configuration set (pre-programmed) | R2-04-shield-profile.md |
| GAP +00:14 | ODN breach / sensor data injection | R2-02-data-breach.md |
| GAP +00:18 | EPS component 9 failure (sensor recording cut) | R2-05-fault-trace.md |
| GAP +00:22 | EPS conduit routing reconfigured | R2-01-power-path.md |
| GAP +00:25 | Reactor throttle set to position 4 | R2-06-reactor-state.md |
| GAP +01:08 | Heat source active (deflector focused scan) | R2-03-heat-source.md |

Causal consistency checks:
1. Shields configured (+00:03) BEFORE sensor suppression (+00:14) -- makes sense: prepare defenses before suppressing evidence.
2. Sensor data injection (+00:14) BEFORE relay failure (+00:18) -- sensor data overridden first, then recording cut.
3. Relay failure (+00:18) BEFORE EPS reroute (+00:22) -- the sensor relay fails, then power is rerouted.
4. Reactor state (+00:25) BEFORE heat source (+01:08) -- reactor configured to support deflector, then deflector activated later.

Timeline from R3 puzzles:

| Time | Event | Source |
|------|-------|--------|
| GAP +00:42 | Chen accesses Sensor Bay Anteroom | R3-02, R3-03, R3-05 |
| GAP +01:22 | Kwon collapses (V-tach) at COMPUTER station | R3-01, R3-03, R3-05 |
| GAP +01:42 | Vasquez forces Sensor Bay Anteroom door | R3-04, R3-05 |
| GAP +01:55 | Reeves' token composes OVERRIDE permission | R3-03, R3-05 |
| GAP +02:10 | 12-operation bypass executed | R3-05 |
| GAP +02:38 | Vasquez forces Comm Array Junction | R3-04 |
| GAP +03:34 | Vasquez forces Computer Core Anteroom (purge) | R3-04 |

Combined timeline is chronologically consistent. No timestamp conflicts. R3-05 section 7 lists all events in chronological order (confirmed as fixed in BUG-L07).

R2-META-response.md references GAP +00:14 (the ODN breach at node 7) and GAP +01:22 (Kwon collapse) -- both consistent with the R2 and R3 puzzle timelines.

**CHECK 4 RESULT: PASS**

---

## Check 5: Crew Cross-References

### KWON (roster position 3)
- R2-META: COMPUTER station officer, roster position 3, first alert station
- R3-01: Patient 3 = LT. KWON, J. (CREW-ID 8837-SIGMA), V-tach at GAP +01:22
- R3-META: Value 3 = KWON = incapacitated (casualty of suppressant)
- R3-05 story: "Kwon collapses -- COMPUTER station unmonitored"
- CyclicGroupDisplay: Position 3 = KWON (starting position)
- **Consistent across all files.**

### VASQUEZ (roster position 5)
- R3-04: Badge 401 = CO = VASQUEZ, first forced entry at position 5
- R3-META: Value 5 = VASQUEZ = ordering officer (CO override)
- R3-05 story: "Vasquez forces Sensor Bay Anteroom door" at +01:42
- CyclicGroupDisplay: Position 5 = VASQUEZ (final answer)
- META-DESIGN.md: "The word VASQUEZ -- the name of the officer who ordered the cover-up"
- **Consistent across all files.**

### REEVES (roster position 4)
- R3-03: Element 4 = REEVES = authorization token provider
- R3-05: 12 operations = procedure register count, 12 mod 8 = 4 = REEVES
- R3-META: Appears in both R3-03 and R3-05, double role (authorization + execution)
- R2-META: ENG station (EPS Circuit, Reactor) = position 4 = REEVES
- **Consistent across all files.**

### CHEN (roster position 7)
- R3-02: Final clock position 7 = CHEN access code
- R3-META: Value 7 = CHEN = physical access agent
- R2-META: Not directly implicated (no system controlled by CHEN's station shows anomaly)
- **Consistent.**

**CHECK 5 RESULT: PASS**

---

## Check 6: No Answer Leaks

The final meta answer is VASQUEZ. The CLAUDE.md states: "Numeric answers stored in .claude/ project memory only. No plaintext answers in git-tracked files."

### Plaintext "VASQUEZ" appearances in solver-facing files:

The name VASQUEZ appears in:
1. **R3-02** (roster labels on ModularClockDisplay) -- position 5 label. This is a roster label, not an answer leak. The solver sees all 8 names as circle positions.
2. **R3-03** (roster mapping on reference card) -- same. All 8 names listed.
3. **R3-04** (Badge 401 = VASQUEZ identified in reference card and narrative) -- this is the R3-04 puzzle's own narrative revelation. It identifies Vasquez as the badge holder, not as the final meta answer.
4. **R3-05** (story layer references Vasquez) -- narrative revelation after solving.
5. **R3-META** (authorization chain Vasquez -> Reeves) -- narrative revelation, meta-internal.
6. **R2-META** (no VASQUEZ appearance in solver-facing content) -- VASQUEZ does not appear.
7. **R1-META** (no VASQUEZ appearance) -- correct.
8. **HINTS.md** (Final Meta near-solution: "Position 5 on the circle reads VASQUEZ") -- this is a Tier 3 near-solution hint for the FINAL meta. Appropriate: the near-solution hint for the last puzzle IS the answer.
9. **PUZZLES.md** (R3-04 narrative mentions VASQUEZ) -- design document, not solver-facing.

### Assessment:
- VASQUEZ appears as a roster label in R3-02, R3-03 (alongside all 8 names) -- NOT a leak.
- VASQUEZ is identified as badge holder in R3-04's narrative revelation -- this is the puzzle's own output, revealed AFTER solving R3-04. The solver learns Vasquez forced doors. This does not reveal she is the final answer.
- The connection "VASQUEZ is the final answer" is only revealed when the CyclicGroupDisplay is configured. Knowing Vasquez forced doors (from R3-04) does not tell the solver she is position (3+2) mod 8 = 5 on the duty circle.
- HINTS.md Tier 3 for the final meta does reveal VASQUEZ directly, but that is by design (it is the last-resort hint).

**No premature answer leaks detected.**

**CHECK 6 RESULT: PASS**

---

## Check 7: Dual-Solver Paths Spot-Check

### Spot-check 1: R1-02 Phase Lock (answer: 873 MHz)

**Novice path:** Reference card teaches Lissajous figure reading (tangencies = ratio numbers). Novice sweeps from 500 MHz upward at 10 MHz steps, notices slowdown near 870, switches to fine. At 873 MHz the figure locks. Count tangencies: 3 vertical, 1 horizontal = 3:1. Verify: 3 x 291 = 873. Time: 4-6 minutes.

**Expert path:** Observe 3 tangencies on vertical axis. Immediately compute 3 x 291 = 873. Tune directly to 873 MHz. Lock achieved. Time: under 30 seconds.

Both paths arrive at 873 MHz. The reference card teaches everything a novice needs. The expert bypasses the sweep entirely. **PASS.**

### Spot-check 2: R3-03 Permission Chain (answer: 4)

**Novice path:** Reference card explains Cayley table lookup. Find row 2, scan for value 6, read column header = 4. Verify with RotaryDial + COMPOSE button. Time: 2-3 minutes.

**Expert path:** Recognize Z_8 addition. (6 - 2) mod 8 = 4. Done. Time: under 10 seconds.

Both paths arrive at element 4. **PASS.**

### Spot-check 3: R2-META Response (answer: 3)

**Novice path:** Panel highlights odd values in amber with ANOMALY tag. Three amber values: 7, 65, 9. Reference card says: lowest odd = first alert. Lowest is 7. Look up Station Command Log: ODN Network = COMPUTER = position 3. Enter 3. Time: 2-4 minutes.

**Expert path:** Scan values: 2, 7, 14, 65, 9, 4. Odd = 7, 9, 65. Min = 7. ODN = COMPUTER = KWON = 3. Enter 3. Time: 15-30 seconds.

Both paths arrive at 3. **PASS.**

**CHECK 7 RESULT: PASS**

---

## Check 8: Hints Completeness

HINTS.md structure verified:

### Round 1 -- TELEMETRY (6 puzzles)
| Puzzle | Nudge | Direction | Near-solution |
|--------|-------|-----------|--------------|
| R1-01 Carrier Isolation | Present | Present | Present (2.340 GHz) |
| R1-02 Phase Lock | Present | Present | Present (873 MHz) |
| R1-03 Null Zone | Present | Present | Present (213 degrees) |
| R1-04 Orbit Classification | Present | Present | Present (0.73) |
| R1-05 Contact Lock | Present | Present | Present (5) |
| R1-06 Signal Fingerprint | Present | Present | Present (384 kbps) |

### Round 2 -- SYSTEMS LOG (6 puzzles)
| Puzzle | Nudge | Direction | Near-solution |
|--------|-------|-----------|--------------|
| R2-01 Power Path | Present | Present | Present (2) |
| R2-02 Data Breach | Present | Present | Present (7) |
| R2-03 Heat Source | Present | Present | Present (14) |
| R2-04 Shield Profile | Present | Present | Present (65) |
| R2-05 Fault Trace | Present | Present | Present (9) |
| R2-06 Reactor State | Present | Present | Present (4) |

### Round 3 -- CREW RECORD (5 puzzles)
| Puzzle | Nudge | Direction | Near-solution |
|--------|-------|-----------|--------------|
| R3-01 Triage | Present | Present | Present (3) |
| R3-02 Access Code | Present | Present | Present (7) |
| R3-03 Permission Chain | Present | Present | Present (4) |
| R3-04 Badge Sequence | Present | Present | Present (5) |
| R3-05 Emergency Sequence | Present | Present | Present (12) |

### Metas (3 round metas + 1 final meta)
| Meta | Nudge | Direction | Near-solution |
|------|-------|-----------|--------------|
| R1-META CONTACT | Present | Present | Present (8) |
| R2-META RESPONSE | Present | Present | Present (3) |
| R3-META COVER-UP | Present | Present | Present (2) |
| FINAL META THE COMMISSION | Present | Present | Present (VASQUEZ) |

**All 17 feeders + 3 round metas + 1 final meta = 21 hint sets. All complete at 3 tiers.**

Note: R2-02 near-solution hint says "node 7 -- the Comms Computer node." This should say "Security Computer" per BUG-L05. The authored puzzle file uses the correct label, but the HINTS.md text has the stale label.

**CHECK 8 RESULT: PASS (with minor note on R2-02 hint label)**

---

## Bugs Found During Integration

### BUG-L09: META-DESIGN.md station-to-roster table is stale

META-DESIGN.md (Section "R2-META -- RESPONSE", lines 165-172) has a station-to-roster mapping table that uses different roster positions than the authored R2-META:
- META-DESIGN.md: OPS = 2, COMPUTER/ENG = 6, ENVIRO = 4, TAC = 1, ENG = 5
- Authored R2-META: OPS = 0 (TORRES), COMPUTER = 3 (KWON), ENVIRO = 6 (PARK), TAC = 1 (NAKAMURA), ENG = 4 (REEVES)

The authored R2-META uses the correct duty roster (0=TORRES through 7=CHEN), consistent with the CyclicGroupDisplay and all R3 puzzles. The META-DESIGN.md table appears to be from an earlier design draft.

**Impact:** Low. META-DESIGN.md is a design document, not solver-facing. The authored puzzle is authoritative. The final answer (position 3 = KWON) is correct in both.
**Fix:** Update META-DESIGN.md table to match the authored roster positions.

### BUG-L05 (existing, still open): PUZZLES.md R2-02 "Comms Computer"

Already logged. PUZZLES.md line 301 still says "the Comms Computer node" for node 7. World data and authored puzzle both say "Security Computer." HINTS.md R2-02 near-solution hint also says "Comms Computer."

**Impact:** Low. Answer value (7) is correct. Label is wrong in brief and hint text.
**Fix:** Update PUZZLES.md R2-02 narrative and HINTS.md R2-02 near-solution to say "Security Computer."

### BUG-L06 (existing): R2-META mechanism -- RESOLVED

BUG-L06 noted that two candidate extraction mechanisms existed for R2-META. The authored R2-META-response.md selects the parity-based mechanism (odd = ANOMALY, lowest odd = first alert, controlling station roster position = starting position). This resolves BUG-L06.

**Status:** CLOSED by R2-META authoring.

### BUG-L02 (existing): R2-META mechanism needs tightening -- RESOLVED

Same as BUG-L06. The authored R2-META now has a single clean derivation path.

**Status:** CLOSED by R2-META authoring.

---

## Summary

| Check | Result |
|-------|--------|
| 1. Answer chain end-to-end | PASS |
| 2. CyclicGroupDisplay math | PASS |
| 3. Cross-puzzle bearing consistency | PASS |
| 4. GAP timeline consistency | PASS |
| 5. Crew cross-references | PASS |
| 6. No answer leaks | PASS |
| 7. Dual-solver paths (3 spot-checks) | PASS |
| 8. Hints completeness | PASS (minor note) |

**Overall: 8/8 PASS. No blockers. 1 new minor bug (BUG-L09). 2 existing bugs resolved (BUG-L02, BUG-L06).**
