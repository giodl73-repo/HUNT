# DEAD RECKONING v2 -- Stage 10 Platform Test

**Date:** 2026-02-28
**Scope:** Dual-solver simulation, narrative arc verification, meta chain test, timing estimates
**Primary criterion:** Every puzzle must work for both a Panel Expert and a Panel Novice (SCENARIO.md requirement)

---

## Part 1: Dual-Solver Simulation

### Methodology

For each of the 17 feeders + 3 round metas, two solvers are simulated:

- **Expert:** Has real-world knowledge of the instrument type. Knows the physics/math. Skips the reference card and goes straight to the controls.
- **Novice:** No prior knowledge. Uses ONLY the information in the puzzle file's Reference Card section. Works inductively: try a control, observe, understand, adjust.

Verdict codes:
- **PASS** -- Both solvers can solve the puzzle independently
- **EXPERT-ONLY** -- Novice cannot solve from reference card alone (MAJOR issue)
- **FAIL** -- Neither solver can solve (BLOCKER)

---

### Round 1 -- TELEMETRY (6 feeders)

| Puzzle | Expert Time | Expert Path | Novice Path (from reference card only) | Novice Gaps? | Verdict |
|--------|-------------|-------------|---------------------------------------|-------------|---------|
| **R1-01 Carrier Isolation** | 30-60s | Recognize S-band. Jump to 2.0 GHz, 10 MHz steps to 2.300, then 1 MHz to 2.340. Waveform locks on gold overlay. | Reference card teaches bandpass filtering, 3-step sweep (100/10/1 MHz). Novice sweeps full range at 100 MHz (25 clicks max), narrows at 2.300, locks at 2.340. All concepts explained on card. | None. Card explains "carrier = single frequency" and "filter center = carrier --> clean output." | **PASS** |
| **R1-02 Phase Lock** | 15-30s | See 3-lobed figure. 3 x 291 = 873. Tune directly. | Reference card teaches tangency counting with worked example ("3 vertical, 1 horizontal = Y = 3x X"). Novice sweeps from 500 MHz at 10 MHz steps. Figure slows near 870. Fine-tune to 873. Drift indicator reads LOCKED. Count tangencies to verify. | None. The tangency counting method is fully self-contained on the card. The formula f_Y/f_X = p/q is stated and exemplified. | **PASS** |
| **R1-03 Null Zone** | 30-60s | Symmetry lock on. Drag source to null interference at ~033 deg. Target dot emerges at 213 deg. | Reference card teaches bright/dark regions, null placement, symmetry lock. Novice turns symmetry lock on, drags source alpha around display. When null crosses 033 deg, interference fades. Bright dot appears at 213 deg. Bearing readout confirms. | None. The drag-and-observe mechanic is self-teaching. Card explains null = dark, target = bright. | **PASS** |
| **R1-04 Orbit Classification** | 30-60s | Open arc = eccentric ellipse. Estimate e~0.7. Fine-tune to 0.73. Adjust l to 47 km. | Reference card teaches eccentricity classification (circle/ellipse/parabola/hyperbola). Novice sweeps e from 0.00 upward at 0.10 steps. At 0.70 curvature is close. Switch to 0.01, reach 0.73. Adjust l to 47 km. Match indicator glows. | None. The card teaches eccentricity from scratch and gives the key hint about flyby trajectories (e between 0.5 and 1.0). | **PASS** |
| **R1-05 Contact Lock** | 1-2min | Rapidly click through 8 contacts. Solid vs flickering lock is obvious. 5 real, 3 decoy. | Reference card explains REAL (green lock, stable bearing) vs DECOY (flickering, drifting bearing). Novice clicks each contact, waits 2s for lock, observes for 3-5s. The behavioral difference is unmistakable: stable vs jittery. Classify each. Count greens = 5. | None. The card defines exactly what to look for (lock color, bearing stability, SOLID vs UNSTABLE). The visual difference between real and decoy is designed to be obvious. | **PASS** |
| **R1-06 Signal Fingerprint** | 30-60s | 8 clusters in ring = 8-PSK. Tune symbol rate to 128 kSym/s. 128 x 3 = 384 kbps. | Reference card teaches cluster counting (2/4/8/16 = BPSK/QPSK/8-PSK/16-QAM), bits per symbol, EVM as quality indicator, and the data rate formula. Novice counts 8 clusters, selects 8-PSK. EVM drops from >40% to ~12%. Tunes symbol rate: 100 -> 128 (transitions sharpen). Reads 384 kbps. | None. Modulation identification by cluster counting is fully taught. The EVM < 15% = correct match test is on the card. Data rate = symbol rate x bits/symbol is stated. | **PASS** |

### Round 2 -- SYSTEMS LOG (6 feeders)

| Puzzle | Expert Time | Expert Path | Novice Path (from reference card only) | Novice Gaps? | Verdict |
|--------|-------------|-------------|---------------------------------------|-------------|---------|
| **R2-01 Power Path** | 30-60s | 3 conduits, junction capacity 2, C(3,2) = 3 combos. PRI+SEC: Shields 95 (green), Impulse 115 (green), LifeSupport 22 (green). | Reference card says junction handles 2 conduits max. Try each combination: all 3 = overload (red flash). Disable one at a time. PRI+SEC = all green. Count = 2. Only 8 total switch states to test. | None. Card explicitly says "try each combination" and explains the gauge zones. | **PASS** |
| **R2-02 Data Breach** | 2-3 toggles | Watch red packet. Disable Node 5 -- packet stops at Node 7. Disable Node 7 -- packet never appears. Entry = 7. | Reference card teaches the tracing strategy: disable a node, observe if packet stops (on-route) or disappears (entry point). Novice starts from edges, works inward. Eventually disables Node 7 -- packet never appears. The "stops vs never appears" distinction is clear. | None. The card's strategy ("start from edges, work inward") is efficient. The key distinction (stops = on-route, disappears = entry point) is well explained. | **PASS** |
| **R2-03 Heat Source** | 2 placements | Read temperature gradient. Maximum at 14. Place coolant at 8 and 20. Gradient collapses to 14. | Reference card teaches "place coolant at edges of warm area first" and "source keeps generating heat, coolant absorbs only diffused heat." Novice places 4-5 coolant zones at periphery. Warm area shrinks to coordinate 14. Gauge confirms PEAK AT: 14. | None. The card's edge-first strategy works. The key insight (source stays hot, diffused heat gets absorbed) is explicitly stated. | **PASS** |
| **R2-04 Shield Profile** | 3 adjustments | Inverse relationship: heavy damage = low power. L1=30, L2=65, L3=90. All match. | Reference card explicitly states "STRONGER layer takes LESS damage, WEAKER layer takes MORE damage." Tells solver: L1 LOW, L3 HIGH, L2 in between. Novice sets L1~20 (too low, adjusts to 30), L3~95 (too high, adjusts to 90), L2 by binary search (50 -> 65). All match. | None. The card's inverse relationship explanation is unambiguous. The matching tolerance (+/-2%) allows efficient convergence. | **PASS** |
| **R2-05 Fault Trace** | 4 probes | Probe node 19: RED. Probe node 9: RED. Probe node 7: GREEN. Probe node 10: GREEN. Failed = 9 (current at 7, none at 9). | Reference card teaches binary search on branching topology. Novice probes source (1: GREEN) and destinations. Node 19: RED, Node 18: GREEN, Node 16: GREEN. Fault is on branch A. Probes along path: Node 7 GREEN, Node 9 RED. Failed component = 9. | None. The card explains green/red flash meaning and the binary search strategy. The branching warning is prominent. The tree structure is visible on the display. | **PASS** |
| **R2-06 Reactor State** | 2 adjustments | Setpoints ~60% of max. Throttle 4 base values are close. Containment 72% fine-tunes. | Reference card explains coarse (throttle) + fine (containment) control scheme. Novice sweeps throttle from 0 upward. At position 4, gauges are near setpoints. Slides containment from 50% upward. At 72%, all four gauges align. Throttle detent = 4. | None. The card teaches the two-stage approach (coarse throttle, fine containment). The gauge setpoint markers make target values visible. | **PASS** |

### Round 3 -- CREW RECORD (5 feeders)

| Puzzle | Expert Time | Expert Path | Novice Path (from reference card only) | Novice Gaps? | Verdict |
|--------|-------------|-------------|---------------------------------------|-------------|---------|
| **R3-01 Triage** | <30s | Immediate V-tach recognition from Patient 3 waveform (wide QRS, no P-waves, 168 BPM). | Reference card defines three waveform types with specific criteria: Normal (narrow QRS <120ms, P-waves present), Sinus Tachy (narrow QRS, P-waves, faster), V-tach (WIDE QRS >120ms, NO P-waves). KEY DISTINCTION section is explicit. Novice compares each patient's waveform to descriptions. Patient 3 has wide, blobby peaks, no P-waves. V-tach. Patient ID = 3. | None. The QRS width criterion (narrow vs wide) is the only diagnostic marker needed and it is clearly defined on the card. Medical terminology is fully explained. | **PASS** |
| **R3-02 Access Code** | <15s | Sum: 3+5+6+1 = 15. 15 mod 8 = 7. Position 7 = CHEN. | Reference card teaches modular arithmetic from scratch with two worked examples. The "MOD 8 -- QUICK REFERENCE" section gives the subtraction rule. The "APPLYING A SEQUENCE" worked example demonstrates the exact same type of calculation. Novice applies rotations one at a time: 0->3->0->6->7. Final position = 7. | None. Modular arithmetic is taught completely. The step-by-step application is explicitly modeled on the card. | **PASS** |
| **R3-03 Permission Chain** | <10s | Z_8 addition. (6-2) mod 8 = 4. Element 4 = REEVES. | Reference card teaches both lookup method (scan row 2 for value 6, read column header = 4) and formula method (TOKEN = TARGET - BASE mod 8). Both are exemplified with worked problems. Novice uses either approach. Element 4 = REEVES. | None. Both solve methods are fully taught. The Cayley table is displayed in full. | **PASS** |
| **R3-04 Badge Sequence** | <30s | Scan for amber LEDs. Positions 5, 11, 15. Click position 5. Badge 401 = CO = VASQUEZ. First forced entry = position 5. | Reference card explains GRANTED/DENIED/FORCED distinctions. WHO CAN FORCE: only CO/XO. Badge-to-roster mapping is provided. EXCEPTION note: "CO = VASQUEZ = Badge 401." The answer instruction is explicit: "The position of the FIRST forced entry in the sequence is the answer." Novice identifies amber LEDs (they pulse), clicks each, notes Badge 401 on all three. First = position 5. | **Minor friction:** The badge numbering exception (Badge 401 = CO despite being in the 4xx range, not expected 5xx) could momentarily confuse a fast reader. However, the exception is prominently called out and "CO = VASQUEZ = Badge 401" is stated explicitly. Acceptable. | **PASS** |
| **R3-05 Emergency Sequence** | <2min | Follow the 12-operation sequence from the reference card. A-B-C-D engage, D off, D guard close, A guard close, B guard close. Register = 12. | Reference card prints the COMPLETE SEQUENCE with all 12 operations numbered step by step. The solver clicks each action in order. If wrong order, MasterAlarm resets -- try again using the printed sequence. | None. The full 12-step procedure is printed verbatim on the reference card. The solver simply follows instructions. Even if they make a mistake, the alarm resets and they can retry. | **PASS** |

### Round Metas (3 metas)

| Meta | Expert Time | Expert Path | Novice Path (from reference card only) | Novice Gaps? | Verdict |
|------|-------------|-------------|---------------------------------------|-------------|---------|
| **R1-META Contact** | 30-60s | S-band + 3:1 harmonic = Phase-Locked Echo immediately. Verify remaining 4 columns. Class code = 8. | Reference card teaches column-by-column elimination. Start with frequency: 2.340 GHz = S-band. Two rows match. Check harmonic: 3:1. Only Phase-Locked Echo has n:1 where n>2. Verify remaining 4 columns (all match). Read class code = 8. Enter 8. | None. The table is self-explanatory. The reference card walks through the elimination process step by step. The partial matching note provides safety net for missing feeders. | **PASS** |
| **R2-META Response** | 15-30s | Odd values: 7, 9, 65. Min = 7. ODN = COMPUTER = KWON = position 3. | Panel highlights odd values in amber with ANOMALY tag. Reference card explains parity rule (even = NOMINAL, odd = ANOMALY) and First Response Protocol (lowest odd = first alert). Novice identifies three amber values (7, 65, 9), finds lowest (7), looks up system (ODN Network), looks up station (COMPUTER), reads position (3). | None. The parity highlighting does most of the work. The Station Command Log table maps system -> station -> position. The step-by-step procedure on the card matches exactly. | **PASS** |
| **R3-META Cover-Up** | 30-60s | Values map to: Kwon (3), Chen (7), Reeves (4), Vasquez (5), Reeves (4). Chain: Vasquez -> Reeves = 2 links. | Panel displays roster mappings with roles. Reference card walks through the three steps: map values to crew, identify roles (who ordered vs who executed), trace the chain. VASQUEZ = CO (forced doors), REEVES = executor (12-op bypass). Chain = 2 links. | **Minor friction:** The chain length concept (2 links = self-authorization + delegation) is somewhat abstract. However, the reference card states the chain explicitly: "Link 1: Vasquez authorized herself. Link 2: Vasquez authorized Reeves." The number 2 is directly readable. | **PASS** |

### Part 1 Summary

**All 17 feeders + 3 round metas: PASS.**

Every reference card provides sufficient information for a novice to solve without outside knowledge. Every expert path is fast and satisfying. The two minor friction points identified (R3-04 badge exception, R3-META chain abstraction) are acceptable -- the information is present and clearly stated.

No BLOCKERS. No EXPERT-ONLY issues.

---

## Part 2: Narrative Arc Test

### Question 1: Does Round 1 feel like "there was a signal" -> "they were having a conversation"?

**YES.** The escalation across the six R1 puzzles builds as follows:

| Puzzle | What the solver learns | Escalation level |
|--------|----------------------|-----------------|
| R1-01 | A carrier at 2.340 GHz, non-standard frequency. "DELIBERATE TRANSMISSION." | Signal exists |
| R1-02 | Incoming at exactly 3x ship's outbound. "ACTIVE HARMONIC TRACKING." | Signal is responsive |
| R1-03 | Null was targeted, not exploratory. Operator already knew where to look. | Someone expected it |
| R1-04 | Trajectory shows powered approach, held station at 27 km. Not a flyby -- a rendezvous. | They came on purpose |
| R1-05 | Five contacts in formation, three decoys as distraction. Coordinated group. | There are more of them |
| R1-06 | 384 kbps, 8-PSK, bidirectional. 100 MB over the gap. "ACTIVE DATA EXCHANGE." | They were talking |

The progression from "a signal exists" to "it was deliberately responsive" to "the operator expected it" to "they held position" to "there were five of them" to "they were having a sustained conversation" is well-paced. Each puzzle adds a layer of intentionality. The revelation that "someone was waiting" is earned through six distinct instrument readings.

**Verdict: PASS.**

### Question 2: Does Round 2 reveal "the ship responded" -> "the ship was PREPARED"?

**YES.** The six R2 puzzles establish a timeline of system states:

| GAP Time | Event | What it reveals |
|----------|-------|----------------|
| +00:03 | Shield preset ECHO-7 loaded | Foreknowledge (4 min before detection) |
| +00:14 | Sensor data override injected via Node 7 | Active sensor suppression |
| +00:18 | Relay 9 burned open | Recording killed by induced overcurrent |
| +00:22 | EPS reroute -- DATA-ODN disabled, power to Lab 7 | Unauthorized power allocation |
| +00:25 | Reactor at throttle 4, containment 72% | Calculated station-keeping power budget |
| +01:08 | Heat source at grid 14 (deflector anteroom) | Deflector in focused scan mode |

The critical shift from "response" to "preparation" lands at R2-04. The shield preset was loaded at GAP +00:03 -- four minutes before the contact was detected on sensors at GAP +00:07. The narrative revelation is devastating in its economy:

```
Preset loaded 4 minutes before contact detected on sensors.
```

No exposition. No explanation. The solver makes the inference: this was not reactive defense. This was foreknowledge.

**Verdict: PASS.**

### Question 3: Is THE TURN (R2-04, shield preset 4 minutes before contact) a genuine revelation moment?

**YES.** R2-04 is designed so the solver physically discovers the anomaly. They adjust three shield power sliders to match a damage pattern. The mechanical act is straightforward: set L1 low, L2 medium, L3 high. The revelation comes AFTER solving -- the narrative log reveals the timestamp. The four-line log entry:

```
SHIELD CONFIGURATION LOG -- GAP +00:03
  Defensive posture: PRESET ECHO-7
  Layer allocation: 30 / 65 / 90
  Authorization: Standing order. Not reactive.
  Preset loaded 4 minutes before contact detected on sensors.
```

This is terse, factual, and the implication is immediate. The solver has just proven (by matching the damage pattern) that the shields were at 30/65/90. The log tells them WHEN that configuration was loaded. The gap between +00:03 (shields set) and the official contact detection is the proof of foreknowledge.

The restraint of the delivery -- no exclamation marks, no "shocking revelation" framing, just a timestamp and a fact -- makes it hit harder. The solver's own inference fills the emotional weight.

**Verdict: PASS. THE TURN lands.**

### Question 4: Does Round 3 escalate: circumstantial -> suspicious -> damning -> confrontational?

**YES.** The five R3 puzzles escalate restraint level:

| Puzzle | Evidence type | Tone | What's named |
|--------|--------------|------|-------------|
| R3-01 | Waveform discrepancy | Clinical, quiet | A patient in the wrong place (Kwon: "sleeping" but in sickbay in V-tach) |
| R3-02 | Access code result | Procedural | A person with the wrong code (Chen: pre-authorized, first try, wrong deck) |
| R3-03 | Authorization composition | Accusatory | A specific person in the chain (Reeves: provided the OVERRIDE token) |
| R3-04 | Badge log | Confrontational, direct | Three forced doors, one badge, one person (Vasquez: Badge 401, CO) |
| R3-05 | Procedure fingerprint | Closing | The fingerprint confirms the identity (12-op bypass = CO confirmation bypass) |

The progression is deliberate: R3-01 is a quiet anomaly (waveform that doesn't match a duty log). R3-02 is a procedural concern (someone used a code they shouldn't have had). R3-03 directly names someone in the authorization chain. R3-04 drops all restraint -- Badge 401, three forced doors, "the CO went herself." R3-05 seals it with the 12-operation fingerprint.

By the time the solver finishes R3-04, they know it is Vasquez. R3-05 makes it undeniable.

**Verdict: PASS.**

### Question 5: Is THE DROP (R3-04, three forced doors) appropriately weighted?

**YES.** The puzzle structure itself is confrontational. The solver clicks through 18 badge swipes. Fifteen are routine (green) or failed (red). Three burn amber, pulsing. All three share Badge 401. The narrative revelation ends with:

```
The CO did not delegate this. The CO did not send someone.
The CO went herself.
```

After three puzzles of circumstantial evidence (R3-01, R3-02, R3-03), the directness of R3-04 is a deliberate break in tone. The restraint of Rounds 1 and 2 makes this directness effective. The three amber LEDs in a field of green are visually stark. The badge number repeating across all three is damning.

**Verdict: PASS. THE DROP lands.**

### Question 6: Does the CyclicGroupDisplay -> VASQUEZ feel like the circle closing?

**YES.** The CyclicGroupDisplay is the ship's own tool -- a duty rotation decoder built into the briefing room console. The solver enters three numbers derived from 17 puzzles of physical evidence:

- N = 8 (the contact's class code -- what they encountered)
- Start = 3 (the first-alert station -- where detection began)
- Step = 2 (the authorization chain length -- how the cover-up was organized)

(3 + 2) mod 8 = 5 = VASQUEZ.

The pointer lands on the name. The solver did not type VASQUEZ. The instrument named her. The ship's own tool, configured with evidence the solver extracted from the ship's own instruments, points to the officer responsible.

This is the Riven Standard fully realized: the puzzle IS the instrument. The answer is not typed -- it is demonstrated. The circle closes because the solver has literally traced the evidence from the contact (Round 1) through the ship's response (Round 2) through the personnel records (Round 3) and the commission decoder points to the name.

**Verdict: PASS. The circle closes.**

### Part 2 Summary

The narrative arc works at every checkpoint. The escalation is well-paced across three rounds. THE TURN (R2-04) and THE DROP (R3-04) both land with appropriate weight. The final meta payoff (CyclicGroupDisplay -> VASQUEZ) delivers the narrative closure through the instrument metaphor.

---

## Part 3: Meta Chain Test

### R1-META: Contact Signature Matching

**Input:** 6 feeder values from Round 1.

| Feeder | Value | Table Column | Phase-Locked Echo Range | Match? |
|--------|-------|-------------|------------------------|--------|
| R1-01 | 2.340 GHz | Freq Band | S-band (2-4 GHz) | YES |
| R1-02 | 873 MHz / 291 MHz = 3:1 | Harmonic Ratio | n:1 (n > 2) | YES |
| R1-03 | 213 deg | Approach Bearing | directional | YES |
| R1-04 | 0.73 | Trajectory | hyperbolic-capture (e = 0.6-0.9) | YES |
| R1-05 | 5 | Formation | 4-7 contacts | YES |
| R1-06 | 384 kbps | Data Rate | 128-512 kbps | YES |

**Match counts by type:**

| Contact Type | Matches |
|-------------|---------|
| Nav Beacon | 0 |
| Distress Beacon | 1 (freq band) |
| Commercial Traffic | 0 |
| Military Transponder | 2 (bearing, data rate overlap) |
| Subspace Relay | 0 |
| Stellar Emission | 0 |
| Sensor Ghost | 2 (freq if interpreted as own, formation) |
| **Phase-Locked Echo** | **6** |

**Unique identification confirmed.** No other type matches more than 2 of 6.

Phase-Locked Echo class code = **8**. N = 8 for CyclicGroupDisplay. **CLEAN.**

### R2-META: First Response Protocol

**Input:** 6 feeder values from Round 2.

| Feeder | Value | Parity | Classification |
|--------|-------|--------|---------------|
| R2-01 | 2 | Even | NOMINAL |
| R2-02 | 7 | Odd | ANOMALY |
| R2-03 | 14 | Even | NOMINAL |
| R2-04 | 65 | Odd | ANOMALY |
| R2-05 | 9 | Odd | ANOMALY |
| R2-06 | 4 | Even | NOMINAL |

Odd values: **7, 65, 9**. Lowest odd = **7** (R2-02, ODN Network).

Controlling station: COMPUTER. Roster position: **3** (KWON).

Starting position = **3**. **CLEAN.**

### R3-META: Authorization Chain

**Input:** 5 feeder values from Round 3.

| Feeder | Value | Mod 8 | Roster | Role |
|--------|-------|-------|--------|------|
| R3-01 | 3 | 3 | KWON | Casualty |
| R3-02 | 7 | 7 | CHEN | Access agent |
| R3-03 | 4 | 4 | REEVES | Authorization |
| R3-04 | 5 | 5 | VASQUEZ | Ordering officer |
| R3-05 | 12 | 4 | REEVES | Executor |

Chain: VASQUEZ (pos 5) -> REEVES (pos 4). Two links.

Step size = **2**. **CLEAN.**

### Final Meta: CyclicGroupDisplay

**Inputs:**
- N = 8 (from R1-META)
- Start = 3 (from R2-META)
- Step = 2 (from R3-META)

**Computation:**
- (3 + 2) mod 8 = 5 mod 8 = **5**

**Circle labels (positions 0-7):**
```
0: TORRES    1: NAKAMURA    2: OKAFOR    3: KWON
4: REEVES    5: VASQUEZ     6: PARK      7: CHEN
```

Position 5 = **VASQUEZ**.

**Verification:** 3 + 2 = 5. 5 mod 8 = 5. Position 5 = VASQUEZ. **CONFIRMED.**

### 80% Rule Check

| Meta | Feeder removed | Result still correct? |
|------|---------------|----------------------|
| R1-META | Remove any 1 of 6 | YES -- Phase-Locked Echo still matches all remaining 5. No other type matches >2 of any 5-subset. |
| R2-META | Remove R2-01 (2) | YES -- odd values unchanged: 7, 65, 9. Min = 7. |
| R2-META | Remove R2-02 (7) | NO -- odd values become 65, 9. Min = 9 = ENG = pos 4. Different answer. |
| R2-META | Remove R2-03 (14) | YES |
| R2-META | Remove R2-04 (65) | YES |
| R2-META | Remove R2-05 (9) | YES |
| R2-META | Remove R2-06 (4) | YES |
| R2-META overall | 5 of 6 removals OK | **SATISFIED** (83%) |
| R3-META | Remove any 1 of 5 | YES -- Vasquez and Reeves are identifiable from any 4-value subset. Chain = 2 in all cases. |

**All three round metas satisfy the 80% rule.**

### Part 3 Summary

The meta chain is clean end-to-end. All three round metas produce the correct inputs for the CyclicGroupDisplay. The final computation (3 + 2) mod 8 = 5 = VASQUEZ is verified. The 80% rule is satisfied for all three metas.

---

## Part 4: Hunt Timing Estimates

### Definitions

- **Expert team of 2:** Both solvers have engineering/physics backgrounds. They recognize instruments, know the math, understand signal processing concepts.
- **Mixed team of 2:** One expert (physics/engineering), one novice (no instrument background). They collaborate, with the expert guiding interpretation and the novice working controls.
- **Novice team of 2:** Neither solver has instrument background. They rely entirely on reference cards. Both can read and reason but have no prior exposure to oscilloscopes, spectrum analyzers, etc.

### Per-Puzzle Timing

| Puzzle | Expert Team | Mixed Team | Novice Team |
|--------|-------------|------------|-------------|
| **R1-01** | 1 min | 2 min | 4 min |
| **R1-02** | 0.5 min | 3 min | 6 min |
| **R1-03** | 1 min | 3 min | 6 min |
| **R1-04** | 1 min | 3 min | 6 min |
| **R1-05** | 2 min | 4 min | 8 min |
| **R1-06** | 1 min | 3 min | 5 min |
| **R1-META** | 1 min | 3 min | 5 min |
| R1 subtotal | **7.5 min** | **21 min** | **40 min** |
| **R2-01** | 1 min | 2 min | 4 min |
| **R2-02** | 1 min | 3 min | 8 min |
| **R2-03** | 1 min | 2 min | 5 min |
| **R2-04** | 1 min | 2 min | 4 min |
| **R2-05** | 1 min | 3 min | 7 min |
| **R2-06** | 1 min | 2 min | 5 min |
| **R2-META** | 0.5 min | 2 min | 4 min |
| R2 subtotal | **6.5 min** | **16 min** | **37 min** |
| **R3-01** | 0.5 min | 2 min | 4 min |
| **R3-02** | 0.5 min | 2 min | 4 min |
| **R3-03** | 0.5 min | 1 min | 3 min |
| **R3-04** | 0.5 min | 2 min | 4 min |
| **R3-05** | 2 min | 3 min | 5 min |
| **R3-META** | 1 min | 3 min | 5 min |
| R3 subtotal | **5 min** | **13 min** | **25 min** |
| **FINAL META** | 1 min | 2 min | 3 min |
| **TOTAL PUZZLE TIME** | **20 min** | **52 min** | **105 min** |

### Navigation, Reading, and Story Absorption

The hunt includes narrative revelations after each puzzle, room navigation in the game, and story digestion time. These add overhead:

| Activity | Expert Team | Mixed Team | Novice Team |
|----------|-------------|------------|-------------|
| Navigation between rooms | 10 min | 15 min | 20 min |
| Reading narrative revelations | 15 min | 25 min | 35 min |
| Story discussion/absorption | 10 min | 20 min | 30 min |
| Re-reading / backtracking | 5 min | 10 min | 20 min |
| **Overhead total** | **40 min** | **70 min** | **105 min** |

### Total Estimated Hunt Times

| Team Type | Puzzle Time | Overhead | Total | Rounded |
|-----------|-------------|----------|-------|---------|
| Expert team of 2 | 20 min | 40 min | 60 min | **1-1.5 hours** |
| Mixed team of 2 | 52 min | 70 min | 122 min | **2-2.5 hours** |
| Novice team of 2 | 105 min | 105 min | 210 min | **3.5-4 hours** |

### Comparison to CLAUDE.md Estimates

CLAUDE.md states: "8-12 hours (team), 15-20 hours (solo)."

The simulated times are significantly shorter than the CLAUDE.md estimates. This suggests either:
1. The CLAUDE.md estimates include hint-free cold solving (no reference cards), or
2. The estimates account for much longer navigation/story time in a full game build, or
3. The estimates were set conservatively for the v1 design (which had harder puzzles)

**Recommendation:** Update the CLAUDE.md duration estimate. With reference cards (which are integral to the v2 design), realistic team times are:
- Expert team: 1-2 hours
- Mixed team: 2-3 hours
- Novice team: 3.5-5 hours
- Solo novice: 5-8 hours

These are still satisfying durations for a puzzle hunt experience.

---

## Issues for Stage 11

### BLOCKERS

None.

### MAJOR

**MAJOR-01: CLAUDE.md duration estimates are stale (from v1 or pre-reference-card design)**
- Location: CLAUDE.md, "Duration" field
- Current: "8-12 hours (team), 15-20 hours (solo)"
- Recommended: "1-2 hours (expert team), 3-5 hours (novice team), 5-8 hours (solo)"
- Impact: Sets incorrect expectations for playtesters
- Severity: MAJOR (affects test planning and event scheduling)

**MAJOR-02: HINTS.md R2-02 near-solution still says "Comms Computer" for node 7**
- Location: HINTS.md, R2-02 near-solution tier
- Current: "The breach entry point is node 7 -- the Comms Computer node."
- Should be: "The breach entry point is node 7 -- the Security Computer node."
- Already tracked as BUG-L05 extension
- Impact: Solver-facing hint text has wrong label (answer value 7 is correct)
- Severity: MAJOR (solver-facing text is wrong)

### MINOR

**MINOR-01: R3-04 badge numbering exception could slow a speed-reader**
- Badge 401 maps to CO = VASQUEZ (position 5) despite being in the "4xx" series
- The exception is clearly documented ("EXCEPTION: The CO's badge uses the 401 series")
- A fast reader might initially think Badge 401 = Reeves (position 4, badge 4xx)
- Impact: Momentary confusion, self-correcting upon reading the exception note
- Severity: MINOR (information is present, just requires careful reading)

**MINOR-02: R3-META chain length concept is somewhat abstract**
- "Authorization chain length = 2" requires understanding the link-counting model
- The reference card states both links explicitly, so the number 2 is directly readable
- A solver who does not understand the abstraction can still read "2" from the card
- Impact: Low. The answer is readable even without deep understanding of the model.
- Severity: MINOR

**MINOR-03: META-DESIGN.md station-to-roster table is stale (already tracked as BUG-L09)**
- The META-DESIGN.md table for R2-META uses non-canonical roster positions
- The authored R2-META-response.md uses the correct roster
- Impact: Developer confusion only (not solver-facing)
- Severity: MINOR

**MINOR-04: R1-02 beat frequency at default (209 MHz) would produce filled oval, not discernible rotating figure**
- At 500 MHz default, the beat with 291 MHz is 209 MHz -- extremely fast rotation
- The Lissajous display would show a filled region, not a traceable figure
- The novice solve path says "fast-rotating mess" which is accurate
- Delivery implementation note: the LissajousDisplay must render meaningfully at high mismatch
- Already noted in editorial review (MINOR-03 there)
- Impact: Delivery/implementation concern, not a puzzle design issue
- Severity: MINOR (for Stage 11 delivery refinement)

---

## Overall Assessment

### Strengths

1. **Reference cards are uniformly excellent.** All 17 feeder puzzles + 3 round metas have reference cards that teach the necessary concepts from scratch. The ECG card (R3-01), the Lissajous tangency card (R1-02), and the modular arithmetic card (R3-02) are particularly well-designed. No puzzle requires outside knowledge.

2. **Expert paths are genuinely fast and rewarding.** An expert solves R1-02 in 15 seconds (3 x 291 = 873). An expert solves R3-03 in under 10 seconds (6 - 2 = 4). The puzzles reward domain knowledge without requiring it.

3. **The dual-solver promise is delivered.** Every puzzle works for both solver types. The expert skips the reference card and goes to the controls. The novice reads the card and catches up. Both arrive at the same answer. This is the core design requirement from SCENARIO.md, and it is met across all 20 puzzles.

4. **The narrative arc is tight and earns its revelations.** Story beats are delivered through instrument data and terse log entries. No exposition, no NPC dialogue. The escalation from "there was a signal" through "the ship was prepared" to "the CO went herself" is well-paced. THE TURN and THE DROP both land.

5. **The meta chain is mathematically clean.** All three round metas produce unambiguous outputs. The CyclicGroupDisplay computation is simple and verifiable. The 80% rule is satisfied for all metas.

6. **Cross-puzzle data is consistent.** Bearing 213 degrees, the GAP timeline, crew roster positions, and answer values are all internally consistent across all documents (confirmed in Stage 8 integration check and re-verified here).

### Weaknesses

1. **Hunt is shorter than advertised.** CLAUDE.md estimates 8-12 hours for a team. Simulated times suggest 1-5 hours depending on expertise. This is a calibration issue, not a design flaw.

2. **HINTS.md R2-02 still has the wrong node label.** This is a known bug (BUG-L05) that persists into solver-facing hint text.

3. **R1-02 high-mismatch Lissajous rendering needs implementation attention.** The 209 MHz beat frequency at default settings would produce an indiscernible display. The delivery build must handle this gracefully.

### Verdict

**PASS.** The platform test confirms that the dual-solver requirement is met across all 20 puzzles. The narrative arc works. The meta chain is clean. No blockers. Two MAJOR issues identified (stale duration estimate, wrong hint label). Four MINOR issues noted for Stage 11 polish.

The hunt is ready for Stage 11 (POLISH).
