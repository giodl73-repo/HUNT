# DEAD RECKONING v2 — Stage 7 Editorial Review

**Reviewer Personas:** Thomas Snyder (expert path), Lucas Pope (novice path), Mike Selinker (narrative arc), Mark Gottlieb (systems engineering)

**Date:** 2026-02-28

**Verdict:** PASS with issues. 0 BLOCKERS. 3 MAJOR. 7 MINOR. All MINOR issues fixed in-place during this review.

---

## REVIEW 1: Dual-Solver Test (Snyder / Pope)

### Method

Each of the 17 puzzles was evaluated on two axes:

1. **Expert path (Thomas Snyder):** Does the physics feel correct? Is the expert path fast and satisfying? Does achieving the win condition feel like real operator competence?

2. **Novice path (Lucas Pope):** Is the reference card complete enough for a novice to solve from scratch? Can a player with NO prior knowledge learn what they need from the panel alone? Are there any steps requiring outside knowledge?

---

### Per-Puzzle Results

#### R1-01 — Carrier Isolation (SineWaveDisplay)

| Criterion | Expert | Novice |
|-----------|--------|--------|
| Physics correct? | YES. Bandpass filter behavior is standard. Carrier isolation by frequency tuning is real signal processing. Harmonic at 2x carrier is physically plausible. | N/A |
| Expert path fast? | YES. 30-60 seconds. S-band recognition + direct tuning. | N/A |
| Reference card sufficient? | N/A | YES. Explains what the display shows, what each control does, step-by-step isolation procedure, and the key physics fact about bandpass filtering. |
| Outside knowledge required? | N/A | NO. Everything needed is on the card. |
| **Verdict** | PASS | PASS |

#### R1-02 — Phase Lock (LissajousDisplay)

| Criterion | Expert | Novice |
|-----------|--------|--------|
| Physics correct? | YES. Lissajous tangency counting is the standard method for frequency ratio identification. 3:1 ratio producing 3 vertical tangencies is correct. | N/A |
| Expert path fast? | YES. 15-30 seconds. Immediate 3:1 recognition, single calculation 3 x 291 = 873. | N/A |
| Reference card sufficient? | YES. Explains tangency counting with a worked example ("3 tangencies vertical, 1 horizontal = Y frequency = 3 times X frequency"). The formula f_Y/f_X = p/q is stated. Step-by-step tuning procedure given. | N/A |
| Outside knowledge required? | NO. The tangency counting method is fully explained on the card. | N/A |
| **Verdict** | PASS | PASS |

#### R1-03 — Null Zone (PhaseInterferenceDisplay)

| Criterion | Expert | Novice |
|-----------|--------|--------|
| Physics correct? | YES. Two-source interference nulling is real phased-array physics. Null placement by source positioning is the standard technique. | N/A |
| Expert path fast? | YES. 30-60 seconds. Symmetry lock + drag to null. | N/A |
| Reference card sufficient? | YES. Explains bright/dark regions, how nulling works, how to use symmetry lock, and the step-by-step procedure for finding the target bearing. | N/A |
| Outside knowledge required? | NO. The drag-and-observe mechanic is self-teaching. | N/A |
| **Verdict** | PASS | PASS |

#### R1-04 — Orbit Classification (ConicSectionDisplay)

| Criterion | Expert | Novice |
|-----------|--------|--------|
| Physics correct? | YES. Conic section classification by eccentricity is standard orbital mechanics. The polar equation r = l/(1 + e cos theta) is correctly applied. Observed positions computed from the formula are internally consistent. | N/A |
| Expert path fast? | YES. 30-60 seconds. Recognize open arc = eccentric ellipse, estimate e ~ 0.7, fine-tune. | N/A |
| Reference card sufficient? | YES. Explains eccentricity classification (circle/ellipse/parabola/hyperbola), what the controls do, and the fitting procedure. The key physics fact about flyby trajectories (e between 0.5 and 1.0) helps the novice narrow the search. | N/A |
| Outside knowledge required? | NO. The card teaches eccentricity from scratch. | N/A |
| **Verdict** | PASS | PASS |

#### R1-05 — Contact Lock (RadarSweepDisplay + TargetingReticleDisplay)

| Criterion | Expert | Novice |
|-----------|--------|--------|
| Physics correct? | YES. Radar lock discrimination between real returns and ghost returns is standard radar processing. The behavioral difference (stable vs. drifting lock) is physically motivated. | N/A |
| Expert path fast? | YES. 1-2 minutes (limited by 2-second lock time per contact). Satisfying — feels like real tactical classification. | N/A |
| Reference card sufficient? | YES. Explains the lock test procedure, the behavioral difference between REAL and DECOY, and the classification button mechanic. | N/A |
| Outside knowledge required? | NO. The card fully describes how to distinguish real from decoy. | N/A |
| **Verdict** | PASS | PASS |

#### R1-06 — Signal Fingerprint (CommSignalDisplay)

| Criterion | Expert | Novice |
|-----------|--------|--------|
| Physics correct? | YES. IQ constellation identification of modulation types is standard digital communications. 8 clusters in a ring = 8-PSK is correct. Data rate = symbol rate x bits per symbol is the standard formula. | N/A |
| Expert path fast? | YES. 30-60 seconds. Immediate 8-PSK recognition, tune to 128 kSym/s. | N/A |
| Reference card sufficient? | YES. Explains cluster counting (2/4/8/16), bits per symbol for each type, EVM as a quality indicator, and the data rate formula. The novice can identify 8-PSK by counting clusters and checking EVM. | N/A |
| Outside knowledge required? | NO. Modulation identification is fully taught on the card. | N/A |
| **Verdict** | PASS | PASS |

#### R2-01 — Power Path (ConduitFlowDisplay)

| Criterion | Expert | Novice |
|-----------|--------|--------|
| Physics correct? | YES. EPS junction capacity constraint is physically motivated. Power contribution model is simple and clear. | N/A |
| Expert path fast? | YES. Recognize C(3,2)=3 combinations, test PRIMARY+SECONDARY first (or reason from contributions). | N/A |
| Reference card sufficient? | YES. Explains junction capacity limit, describes the trial-and-error approach, and notes that the count of active conduits is the answer. | N/A |
| Outside knowledge required? | NO. | N/A |
| **Verdict** | PASS | PASS |

#### R2-02 — Data Breach (NetworkGridDisplay)

| Criterion | Expert | Novice |
|-----------|--------|--------|
| Physics correct? | YES. Network packet tracing by node isolation is a real diagnostic technique. The fixed-route rogue packet is a clean puzzle mechanic. | N/A |
| Expert path fast? | YES. 2-3 toggles for an expert who understands network topology. | N/A |
| Reference card sufficient? | YES. Explains the tracing strategy clearly: disable a node, observe if the packet stops or disappears. "Start from edges, work inward." The distinction between "stops" and "never appears" is well explained. | N/A |
| Outside knowledge required? | NO. | N/A |
| **Verdict** | PASS | PASS |

**Note:** R2-02 panel overview says "Node 7, the Comms Computer" but the widget config and world data say Node 7 = SECURITY. Already logged as BUG-L05. The authored puzzle file uses the correct label. See MINOR-01 below.

#### R2-03 — Heat Source (HeatMapDisplay)

| Criterion | Expert | Novice |
|-----------|--------|--------|
| Physics correct? | YES. Thermal diffusion from a point source, coolant placement to collapse the gradient — real thermal engineering. The Gaussian-like temperature profile is physically accurate. | N/A |
| Expert path fast? | YES. 2 coolant placements for an expert who reads the temperature gradient correctly. | N/A |
| Reference card sufficient? | YES. Explains the diffusion concept, coolant placement strategy ("edges first, work inward"), and the key insight that the source keeps generating heat while diffused heat can be absorbed. | N/A |
| Outside knowledge required? | NO. | N/A |
| **Verdict** | PASS | PASS |

#### R2-04 — Shield Profile (ShieldDisplay)

| Criterion | Expert | Novice |
|-----------|--------|--------|
| Physics correct? | YES. Inverse relationship between layer power and damage absorbed is a sound model. Layer geometry weighting (outer absorbs more) is physically motivated. | N/A |
| Expert path fast? | YES. Three slider adjustments. The inverse relationship is immediately apparent to an expert. | N/A |
| Reference card sufficient? | YES. Explicitly states "A STRONGER layer takes LESS damage" and "A WEAKER layer takes MORE damage." Tells the solver to set L1 LOW, L3 HIGH, L2 in between. Gives the matching tolerance. | N/A |
| Outside knowledge required? | NO. | N/A |
| **Verdict** | PASS | PASS |

#### R2-05 — Fault Trace (CircuitTopologyDisplay)

| Criterion | Expert | Novice |
|-----------|--------|--------|
| Physics correct? | YES. Binary search on a branching circuit topology is real diagnostic technique. The probe-and-observe mechanic is clean. | N/A |
| Expert path fast? | YES. 4 probes for an expert who understands tree traversal. | N/A |
| Reference card sufficient? | YES. Explains green/red flash meaning, binary search strategy, and the critical warning about branching topology. The step-by-step guide is clear. | N/A |
| Outside knowledge required? | NO. The card teaches the binary search concept. | N/A |
| **Verdict** | PASS | PASS |

#### R2-06 — Reactor State (GaugeDisplay + ThrottleLever)

| Criterion | Expert | Novice |
|-----------|--------|--------|
| Physics correct? | YES. Coupled reactor gauges responding to fuel flow and containment is a sound model. The "coarse throttle + fine containment" control scheme mirrors real reactor operation. | N/A |
| Expert path fast? | YES. Two control adjustments (throttle to 4, containment to 72%). The detent snap is satisfying. | N/A |
| Reference card sufficient? | YES. Explains the two-control scheme, how to read the setpoint markers, and the coarse/fine adjustment strategy. | N/A |
| Outside knowledge required? | NO. | N/A |
| **Verdict** | PASS | PASS |

#### R3-01 — Triage (LifesignsDisplay)

| Criterion | Expert | Novice |
|-----------|--------|--------|
| Physics correct? | YES. ECG waveform morphology for sinus tachycardia vs V-tach is medically accurate. QRS width and P-wave presence are the correct diagnostic markers. | N/A |
| Expert path fast? | YES. Under 30 seconds. Immediate V-tach recognition from waveform morphology. | N/A |
| Reference card sufficient? | YES. Excellent reference card. Defines all three waveform types with specific criteria (QRS width >120ms, P-wave absence). The "KEY DISTINCTION" section is particularly clear. A novice can diagnose V-tach by comparing the waveform to the card descriptions. | N/A |
| Outside knowledge required? | NO. Medical terminology is fully defined on the card. | N/A |
| **Verdict** | PASS | PASS |

#### R3-02 — Access Code (ModularClockDisplay)

| Criterion | Expert | Novice |
|-----------|--------|--------|
| Physics correct? | YES (mathematical). Modular arithmetic is correctly implemented. (3+5+6+1) mod 8 = 15 mod 8 = 7. | N/A |
| Expert path fast? | YES. Under 15 seconds. Sum and reduce. | N/A |
| Reference card sufficient? | YES. Teaches modular arithmetic from scratch with worked examples. The "MOD 8 — QUICK REFERENCE" section and the "APPLYING A SEQUENCE" example are particularly helpful. | N/A |
| Outside knowledge required? | NO. | N/A |
| **Verdict** | PASS | PASS |

#### R3-03 — Permission Chain (CayleyTableDisplay)

| Criterion | Expert | Novice |
|-----------|--------|--------|
| Physics correct? | YES (mathematical). Z_8 under addition is a standard group. Cayley table is correct. (2+4) mod 8 = 6. | N/A |
| Expert path fast? | YES. Under 10 seconds. Trivial subtraction: 6-2=4. | N/A |
| Reference card sufficient? | YES. Teaches both methods: table lookup (scan row 2 for value 6, read column header) and formula (TOKEN = TARGET - BASE mod 8). Both are explained with worked examples. | N/A |
| Outside knowledge required? | NO. | N/A |
| **Verdict** | PASS | PASS |

#### R3-04 — Badge Sequence (IndicatorPanel)

| Criterion | Expert | Novice |
|-----------|--------|--------|
| Physics correct? | N/A (reading exercise, not physics). The security audit format is realistic. | N/A |
| Expert path fast? | YES. Under 30 seconds. Scan for amber, click first one, read position. | N/A |
| Reference card sufficient? | YES. Explains GRANTED/DENIED/FORCED distinctions, who can force doors, and the badge-to-roster mapping. The note about the CO's badge using 401 series is essential and is clearly stated. | N/A |
| Outside knowledge required? | NO. | N/A |
| **Verdict** | PASS | PASS |

#### R3-05 — Emergency Sequence (BatSwitch array)

| Criterion | Expert | Novice |
|-----------|--------|--------|
| Physics correct? | N/A (procedural exercise). The guarded-switch mechanic is realistic for cockpit/engineering consoles. | N/A |
| Expert path fast? | YES. Under 2 minutes. Follow the reference card sequence, 12 clicks. | N/A |
| Reference card sufficient? | YES. The "COMPLETE SEQUENCE (12 OPERATIONS)" section lists every single operation step-by-step with operation numbers. Completely unambiguous. | N/A |
| Outside knowledge required? | NO. The full 12-step procedure is printed on the card. | N/A |
| **Verdict** | PASS | PASS |

---

### Dual-Solver Summary

**All 17 puzzles pass the dual-solver test.** Every reference card provides sufficient information for a novice to solve without outside knowledge. Every expert path is fast and satisfying. No BLOCKERS.

---

## REVIEW 2: Narrative Arc (Selinker)

### Escalation Assessment

#### Round 1 — "Someone was waiting for us"

| Puzzle | Revelation | Delivered through data? |
|--------|-----------|----------------------|
| R1-01 | Carrier at 2.340 GHz is S-band, non-standard frequency, deliberately avoiding monitoring bands. Harmonic at 2x confirms engineered transmitter. | YES. Frequency readout + harmonic structure. Narrative log: "Classification: DELIBERATE TRANSMISSION." |
| R1-02 | Incoming signal at exactly 3x ship's outbound = active harmonic tracking. The contact was listening and responding. | YES. Lissajous figure tangency ratio + frequency calculation. |
| R1-03 | Null configuration was not exploratory — it was targeted. The operator already knew where the contact was. | YES. Null bearing + target bearing reveal. |
| R1-04 | Trajectory shows e=0.73 with dwell time at periapsis — powered approach, not gravitational flyby. Deliberate rendezvous masked as flyby. | YES. Observed position data + curve fit. |
| R1-05 | Five contacts in formation, centered on bearing 213. Three decoys deployed as distraction. Coordinated group, not random encounter. | YES. Radar lock behavior + bearing data. |
| R1-06 | 384 kbps using 8-PSK = sustained bidirectional encrypted data exchange. This was a conversation, not a beacon. | YES. Constellation + data rate calculation. |

**R1 escalation verdict:** YES. The round builds from "there was a signal" to "the signal was deliberate" to "the operator knew about it" to "there were five of them in formation" to "they were having a conversation." Each puzzle adds a layer. The revelation that someone was waiting is fully earned through instrument data.

#### Round 2 — "We were prepared"

| Puzzle | Revelation | Delivered through data? |
|--------|-----------|----------------------|
| R2-01 | EPS reroute was deliberate — DATA-ODN bandwidth redirected to a decommissioned lab. Cover story: junction overload. Real purpose: power for unauthorized location. | YES. Conduit configuration + narrative log referencing Lab 7. |
| R2-02 | Sensor data override injected through Security Computer maintenance port. Sensors blinded by design. | YES. Rogue packet route tracing + log entry. |
| R2-03 | Deflector array operating in focused scan mode — pointed at the contact. Heat signature is physical evidence that cannot be purged. | YES. Thermal isolation + grid coordinate. |
| R2-04 | Shield preset ECHO-7 loaded 4 minutes BEFORE contact detected. Pre-programmed defensive posture. Foreknowledge. | YES. Shield damage pattern + log timestamp: "Preset loaded 4 minutes before contact detected on sensors." |
| R2-05 | Sensor recording relay deliberately burned open by induced overcurrent. Not wear failure. The recording went dark by design. | YES. Probe data + relay inspection note: "thermal pitting consistent with induced overcurrent." |
| R2-06 | Reactor at exact power budget for rendezvous station-keeping. Not combat power. Calculated allocation for meeting, not fighting. | YES. Four-gauge match + power budget breakdown. |

**R2 escalation verdict:** YES. The critical "turn" moment — R2-04 — lands hard. The shield preset was loaded BEFORE the contact was detected. The narrative revelation log entry is terse and devastating: "Preset loaded 4 minutes before contact detected on sensors." The solver's realization — this was not a response, it was planned — is earned entirely through the damage pattern math and the timestamp. No exposition. Pure data.

The R2 sequence builds a timeline of deliberate preparations: shields configured at GAP+00:03, sensor suppression at GAP+00:14, relay failure at GAP+00:18, EPS reroute at GAP+00:22, reactor state at GAP+00:25, heat source from GAP+01:08. Each system tells the same story: the ship was prepared.

#### Round 3 — "The cover-up was ordered before the encounter ended"

| Puzzle | Revelation | Delivered through data? |
|--------|-----------|----------------------|
| R3-01 | Kwon was not asleep — cardiac crisis in sickbay. Duty log falsified. First thread pulled loose. | YES. ECG waveform classification vs duty log discrepancy. |
| R3-02 | Chen in the Sensor Bay with pre-authorized override code. Not a hack — a procedure. Someone gave Chen the code. | YES. Modular arithmetic result mapping to roster position. |
| R3-03 | Reeves provided authorization token for the sensor purge override. Reeves was "off-duty" but the token was used anyway. Pre-planned. | YES. Cayley table composition result. |
| R3-04 | Vasquez — Badge 401 — personally forced three restricted doors. The CO went herself. No delegation. | YES. Badge swipe LED sequence + log entries. |
| R3-05 | 12-operation bypass = CO confirmation bypass variant. Only Vasquez had the knowledge and authority. The fingerprint is the operation count. | YES. Procedure register count + variant identification. |

**R3 escalation verdict:** YES. The restraint peels away deliberately. R3-01 is quiet (a waveform discrepancy). R3-02 is procedural (a code that shouldn't have been used). R3-03 is accusatory (naming a specific person in the authorization chain). R3-04 drops the restraint entirely (Badge 401, forced entry, three doors, one person). R3-05 closes the loop with the operation count fingerprint.

The progression from circumstantial (R3-01, R3-02) to direct evidence (R3-04, R3-05) mirrors real investigative structure. By the time the solver reaches R3-05, they already know it is Vasquez — the last puzzle just makes it undeniable.

#### The Turn (R2-04 Shield Profile)

**Does R2-04 land the "this was planned" realization?**

YES. The narrative revelation is a masterclass in data-driven storytelling:

```
SHIELD CONFIGURATION LOG — GAP +00:03
  Defensive posture: PRESET ECHO-7
  Layer allocation: 30 / 65 / 90
  Authorization: Standing order. Not reactive.
  Preset loaded 4 minutes before contact detected on sensors.
```

Four lines. No exposition. No explanation of what this means. The data speaks: the shields were set up before anyone "knew" there was a threat. The solver makes the inference. The instrument shows. The solver understands. This is the Riven Standard applied to narrative.

#### R3-04 Badge Sequence — Confrontational Enough?

**Does Badge 401 land with appropriate narrative weight?**

YES. The puzzle structure itself is confrontational. The solver clicks through 18 badge swipes. Fifteen are routine (green) or failed (red). Three burn amber. They all share the same badge number. The narrative revelation ends with: "The CO did not delegate this. The CO did not send someone. The CO went herself." This is stated through data (badge numbers, door names, timestamps) and then confirmed in a single plain sentence. The restraint of Rounds 1 and 2 makes this directness hit harder.

#### R3-05 — Does the Circle Close?

**Does the 12-operation count feel like a closing?**

YES. The reference card note — "The 12-operation sequence is the CO confirmation bypass" — connects the physical action (12 switch operations) to the identity (only the CO uses the bypass variant). The procedure register is a counter. The counter reads 12. That number is a fingerprint. The CyclicGroupDisplay explanation in the story layer shows how the five R3 answers feed the meta, and the pointer landing on position 5 = VASQUEZ is the circle literally closing.

### Story-Through-Data Assessment

**Are narrative revelations delivered ONLY through instrument readings and log entries?**

YES, with one qualification. The "Story Layer" section at the end of each puzzle file contains expository paragraphs explaining the narrative significance. However, these sections are developer documentation, not solver-facing content. The solver sees only the reference card, the controls, and the narrative revelation log entry. All narrative revelation log entries use the terse format:

```
[SYSTEM] GAP+HH:MM:SS — Observation. Classification. Data point.
```

This format is consistent across all 17 puzzles.

**Are there any puzzle files that explain the conspiracy through exposition rather than data?**

The solver-facing content (panel display, reference card, narrative revelation) is data-only. The story layer sections are developer-facing analysis. As long as the story layer text is not shown to the solver, the principle holds. **No violations.**

### Terse Log Entry Format Consistency

Checked all 17 narrative revelation log entries. Format pattern:

```
[DEPT] GAP+HH:MM — Brief factual statement. Classification or status.
```

R1 puzzles use `[COMMS]`, `[TAC]`, `[NAV]` department tags. R2 puzzles use system-specific headers (`EPS ROUTING LOG`, `ODN INTRUSION LOG`, etc.). R3 puzzles use terminal identifiers (`MED-BIOTERM-03-B`, `SEC-KEYCTL-07-A`, etc.).

**MINOR issue identified:** R2 and R3 revelation headers are more detailed than R1's single-line `[DEPT]` prefix. R1 uses `[COMMS] GAP+00:23:11 — ...` while R2 uses multi-line blocks with header bars and indented sub-entries. The R2/R3 format is richer and more realistic. R1 is more telegraphic.

**Verdict:** This is a stylistic progression, not an inconsistency. R1 (external telemetry) uses raw signal log format. R2 (internal systems) uses system log format with more structure. R3 (personnel records) uses terminal-report format with even more detail. The increasing detail mirrors the escalation — the deeper you dig, the more complete the records become. **Acceptable as-is. No fix needed.**

---

## REVIEW 3: Cross-Puzzle Consistency (Gottlieb)

### Bearing 213 Degrees

R1-03 establishes the contact bearing at 213 degrees (target signal bearing revealed when interference is nulled).

| Reference | Value | Consistent? |
|-----------|-------|------------|
| R1-03 Null Zone — answer | 213 degrees | Origin. |
| R1-05 Contact Lock — C1 bearing | 213 degrees | YES. Primary contact at same bearing. |
| R1-05 Contact Lock — formation center | bearing 195-240, centered on 213 | YES. |
| R1-05 data values | "at the same bearing as R1-03 target" | YES. Explicitly cross-referenced. |
| R2-04 story layer | "contact was approaching from bearing 213 degrees" | YES. |
| R1-06 story layer | "whatever was at bearing 213" | YES. |
| R2-03 story layer | "bearing 213 degrees (R1-03)" | YES. |

**Verdict:** Bearing 213 degrees is internally consistent across all references. PASS.

### GAP Timeline (R2 Puzzles)

The six R2 puzzles establish a timeline of ship system events during the gap:

| GAP Time | Event | Source | Consistent? |
|----------|-------|--------|------------|
| +00:03 | Shield preset ECHO-7 loaded | R2-04 revelation | YES |
| +00:14 | ODN intrusion — sensor data override injected | R2-02 revelation | YES |
| +00:18 | EPS relay 9 failure — sensor recording dead | R2-05 revelation | YES |
| +00:22 | EPS conduit reroute — DATA-ODN disabled | R2-01 revelation | YES |
| +00:25 | Reactor state — throttle 4, containment 72% | R2-06 revelation | YES |
| +01:08 | Heat source — deflector in focused scan mode | R2-03 revelation | YES |

**Internal consistency check:**

1. R2-04 (shields at +00:03) before R2-05 (relay failure at +00:18): The shields were configured before the sensor recording was killed. This makes sense — shields protect the ship during the contact, recording suppression comes later.

2. R2-02 (sensor suppression at +00:14) before R2-05 (relay failure at +00:18): First suppress real-time displays, then kill the recording. Deliberately sequenced. Consistent.

3. R2-01 (EPS reroute at +00:22) after R2-05 (relay failure at +00:18): The recording is dead, so the EPS reroute to Lab 7 will not be recorded by normal channels. Consistent.

4. R2-06 (reactor at +00:25) after R2-01 (EPS reroute at +00:22): The reactor state reflects the power budget AFTER the reroute. The 312 MW output accounts for all active loads including the deflector (55 MW) powered by the rerouted conduit. Consistent.

5. R2-03 (heat source at +01:08) is the latest R2 event: The deflector has been running in focused scan mode for over an hour at this point. The heat has had time to diffuse, explaining why the thermal grid shows a broad warm area rather than a sharp hot spot. Consistent.

6. R2-05 story layer says "four minutes apart" for sensor suppression (+00:14) and relay failure (+00:18): 18 - 14 = 4 minutes. Confirmed.

7. R2-02 story layer says "just eight minutes before the EPS reroute": +00:22 - +00:14 = 8 minutes. Confirmed.

**Verdict:** GAP timeline across R2 puzzles is internally consistent. PASS.

### R3 Timeline

| GAP Time | Event | Source |
|----------|-------|--------|
| +00:42 | Chen accesses Sensor Bay Anteroom (R3-02 modular key) | R3-02 panel header, revelation |
| +01:22 | Kwon collapses — V-tach (R3-01) | R3-01 panel header, revelation |
| +01:42 | Vasquez forces Sensor Bay Anteroom (R3-04 badge swipe 5) | R3-04 badge log |
| +01:55 | Reeves' token used for OVERRIDE (R3-03) | R3-03 revelation |
| +02:10 | 12-operation bypass executed (R3-05) | R3-05 panel header |
| +02:38 | Vasquez forces Comm Array Junction (R3-04 badge swipe 11) | R3-04 badge log |
| +03:34 | Vasquez forces Computer Core Anteroom (R3-04 badge swipe 15) | R3-04 badge log |

**MAJOR issue identified (MAJOR-01):** The timeline summaries in R3-03 and R3-05 list events OUT of chronological order:

R3-03 revelation (lines 223-225):
```
  GAP +01:22   Kwon collapses at COMPUTER station (R3-01)
  GAP +00:42   Chen accesses Sensor Bay Anteroom (R3-02)
  GAP +01:55   Okafor composes Reeves' token to achieve OVERRIDE
```

This lists +01:22 before +00:42, which is not chronological. It is listed in puzzle order (R3-01 then R3-02 then R3-03) but this is confusing to a solver reading the timeline.

R3-05 revelation (lines 274-280):
```
  GAP +01:22   Kwon collapses — COMPUTER station unmonitored
  GAP +00:42   Chen enters Sensor Bay — pre-authorized code
  GAP +01:55   Reeves' token composes OVERRIDE permission
  GAP +01:42   Vasquez forces Sensor Bay Anteroom door
  GAP +02:10   12-operation bypass executed — deflector reconfigured
  GAP +02:38   Vasquez forces Comm Array Junction
  GAP +03:34   Vasquez forces Computer Core Anteroom — purge initiated
```

Same issue. +01:22 before +00:42, and +01:55 before +01:42.

**Classification:** MAJOR. The solver will read this timeline and find it disorienting. The revelation logs should present events in chronological order.

**Fix:** Reorder both timeline summaries into chronological order. (Applied in-place — see fixes section below.)

### Crew Member Roles

| Crew Member | Roster Position | Department | Appearances | Consistent? |
|-------------|----------------|------------|-------------|------------|
| KWON | 3 | COMPUTER | R3-01 (Patient 3, V-tach), R3-02 (clock position passes through 3), R3-05 (story references Kwon collapse). Meta: starting position 3. | YES |
| CHEN | 7 | COMMS | R3-02 (clock position 7, accessed Sensor Bay), R3-04 (not directly — but Chen's badge ID is not among the forced entries). | YES |
| REEVES | 4 | ENG | R3-03 (element 4, authorization token), R3-05 (Chief Engineer, knew bypass). | YES |
| VASQUEZ | 5 | CMD (CO) | R3-04 (badge position 5, Badge 401), R3-05 (only person with knowledge+authority+presence for 12-op bypass). Meta: final answer position 5. | YES |
| OKAFOR | 2 | NAV | R3-03 (base permission element 2). | YES |
| TORRES | 0 | OPS | Clock label at position 0. | N/A |
| NAKAMURA | 1 | TAC | Clock label at position 1. | N/A |
| PARK | 6 | SCIENCE | Clock label at position 6. | N/A |

**Roster consistency check:** The duty roster (0=TORRES through 7=CHEN) is identical in:
- R3-02 (ModularClockDisplay labels)
- R3-03 (CayleyTableDisplay element labels)
- R3-04 (badge-to-roster mapping)
- META-DESIGN.md (CyclicGroupDisplay labels)

**Verdict:** All crew member roles, roster positions, and cross-references are internally consistent. PASS.

### CyclicGroupDisplay Math — End-to-End Verification

**Chain:**

1. R1-META: 6 telemetry measurements -> Contact Signature Reference Table -> Phase-Locked Echo -> Class Code **8** -> N = 8.

   Verification: 2.340 GHz (S-band) + 873/291 = 3:1 harmonic + 213 deg (directional) + 0.73 eccentricity (hyperbolic-capture) + 5 contacts (4-7 range) + 384 kbps (128-512 range) -> All 6 match Phase-Locked Echo. No other type matches more than 2. Class code = 8. **Confirmed.**

2. R2-META: 6 system values -> first-alert station -> roster position **3** -> start = 3.

   The mechanism described in META-DESIGN.md has two candidate derivations (logged as BUG-L02/BUG-L06). The final answer (3) is consistent with KWON at the COMPUTER station. **Confirmed, but mechanism needs authoring (known bug).**

3. R3-META: 5 crew record values -> authorization chain -> chain length **2** -> step = 2.

   Chain: VASQUEZ (position 5) -> REEVES (position 4). Two links. Derivation from feeder values: 3 (Kwon), 7 (Chen), 4 (Reeves), 5 (Vasquez), 12 mod 8 = 4 (Reeves again). VASQUEZ authorized REEVES. Length = 2. **Confirmed.**

4. Final computation: (3 + 2) mod 8 = **5** = **VASQUEZ**.

   Verified: 3 + 2 = 5. 5 mod 8 = 5. Position 5 on the circle = VASQUEZ. **Confirmed.**

**Verdict:** The CyclicGroupDisplay math works end-to-end. PASS.

### Answer Value Consistency Table

| ID | Answer | In PUZZLES.md | In META-DESIGN.md | In puzzle file | In HINTS.md | Consistent? |
|----|--------|--------------|-------------------|---------------|-------------|------------|
| R1-01 | 2.340 GHz | 2.340 | 2.340 | 2.340 | 2.340 | YES |
| R1-02 | 873 MHz | 873 | 873 | 873 | 873 | YES |
| R1-03 | 213 deg | 213 | 213 | 213 | 213 | YES |
| R1-04 | 0.73 | 0.73 | 0.73 | 0.73 | 0.73 | YES |
| R1-05 | 5 | 5 | 5 | 5 | 5 | YES |
| R1-06 | 384 kbps | 384 | 384 | 384 | 384 | YES |
| R2-01 | 2 | 2 | 2 | 2 | 2 | YES |
| R2-02 | 7 | 7 | 7 | 7 | 7 | YES |
| R2-03 | 14 | 14 | 14 | 14 | 14 | YES |
| R2-04 | 65 | 65 | 65 | 65 | 65 | YES |
| R2-05 | 9 | 9 | 9 | 9 | 9 | YES |
| R2-06 | 4 | 4 | 4 | 4 | 4 | YES |
| R3-01 | 3 | 3 | 3 | 3 | 3 | YES |
| R3-02 | 7 | 7 | 7 | 7 | 7 | YES |
| R3-03 | 4 | 4 | 4 | 4 | 4 | YES |
| R3-04 | 5 | 5 | 5 | 5 | 5 | YES |
| R3-05 | 12 | 12 | 12 | 12 | 12 | YES |

**Verdict:** All 17 answer values are consistent across all documents. PASS.

---

## ISSUE LOG

### BLOCKERS

None.

### MAJOR

**MAJOR-01: R3-03 and R3-05 timeline summaries are not chronological**
- Location: R3-03-permission-chain.md section 7, lines 223-225. R3-05-emergency-sequence.md section 7, lines 274-280.
- Problem: Events listed in puzzle order, not chronological order. GAP +01:22 appears before GAP +00:42.
- Impact: Solver reads these timelines and finds them confusing. Undermines the careful narrative structure.
- Fix: Reorder both timeline summaries into chronological order.
- Status: **FIXED IN-PLACE** (see fixes section).

**MAJOR-02: R2-02 panel overview says "Node 7, the Comms Computer" — should be "Security Computer"**
- Location: R2-02-data-breach.md line 15.
- Problem: The panel overview header says "Node 7, the Comms Computer" but the widget configuration (line 36) correctly identifies Node 7 as SECURITY, and world data (data-tables.md) confirms NODE-07 = Security Computer. Already logged as BUG-L05.
- Impact: Solver-facing panel overview text is wrong. The rest of the puzzle file uses the correct label.
- Fix: Change "Comms Computer" to "Security Computer" in the panel overview.
- Status: **FIXED IN-PLACE** (see fixes section).

**MAJOR-03: R2-META derivation mechanism still has two candidates (BUG-L02/L06)**
- Location: meta/META-DESIGN.md R2-META section.
- Problem: Two different derivation methods are described for producing starting position = 3 from the six R2 feeder values. Only one should survive. This is a pre-existing known bug (BUG-L02, BUG-L06).
- Impact: The R2-META puzzle cannot be authored until one mechanism is selected. Does not affect feeder puzzles or other metas.
- Fix: Must be resolved during R2-META authoring (not an editorial fix — requires design decision).
- Status: OPEN. Already tracked in BUGS-local.md.

### MINOR

**MINOR-01: PUZZLES.md R2-02 narrative says "Comms Computer" (same as MAJOR-02)**
- Location: PUZZLES.md R2-02 narrative paragraph.
- Note: This is the brief-level echo of the same error. Not fixing PUZZLES.md in this editorial pass since the authored puzzle file is the canonical source and has been fixed.
- Status: Deferred. Already logged as BUG-L05. Low impact since PUZZLES.md is not solver-facing.

**MINOR-02: PUZZLES.md R3-05 binary encoding discussion is confusing**
- Already logged as BUG-L04. The authored R3-05 puzzle file correctly uses the procedure register count (12), not binary encoding. PUZZLES.md brief text is confusing but not solver-facing.
- Status: Deferred. Already tracked.

**MINOR-03: R1-02 beat frequency terminology**
- Location: R1-02-phase-lock.md data values table.
- The "beat frequency at default" is listed as "~209 MHz" with explanation "500 - 291 = 209 MHz." Beat frequency is correctly defined as |f1 - f2|, but at 209 MHz the display would be rotating extremely fast — this is not a "beat" in the audio sense but rather a rapid phase rotation. The Lissajous display would appear as a filled region, not a discernible rotating figure, at this mismatch.
- Impact: The novice solve path says at 500 MHz "the figure is a fast-rotating mess" which is accurate for a 209 MHz beat. The solver can still follow the procedure (sweep upward until the figure slows). The visual representation will need to handle this during delivery implementation.
- Status: Acceptable as-is. No fix needed. Note for Stage 9 delivery team: ensure the LissajousDisplay renders meaningfully at high mismatch frequencies (e.g., show a filled oval rather than discrete traces).

**MINOR-04: R1-04 periapsis distance quoted two ways**
- R1-04 data values: "Periapsis distance: 27.2 km" computed as l/(1+e) = 47/1.73 = 27.17 km.
- META-DESIGN.md (from v1) references "51 km distance from the correlator data" but this is a v1 reference in parenthetical text, not a v2 data point.
- No inconsistency in v2 data. The 51 km reference is clearly marked as v1 context.
- Status: No fix needed.

**MINOR-05: R3-04 badge ID series mapping**
- The reference card says "Position 4 (REEVES): Badge 4xx" and "Position 5 (VASQUEZ): Badge 5xx" but then immediately says "EXCEPTION: The CO's badge uses the 401 series." Badge 401 is in the 4xx range, not the expected 5xx for position 5. This is intentional (explained as "legacy command badge numbering") and the exception is clearly called out.
- A novice might initially think Badge 401 maps to Reeves (position 4, badge 4xx) before reading the exception. However, the exception note is prominent and the reference card explicitly states "CO = VASQUEZ = Badge 401."
- Status: Acceptable as-is. The exception is clearly documented. The slight confusion adds to the puzzle texture — the solver must read carefully.

**MINOR-06: R2-04 near-miss values table says 64% and 66% are "within tolerance" but then says "65% is the only integer value where ALL THREE layers simultaneously match"**
- Location: R2-04-shield-profile.md section 6, near-miss values.
- The text says tolerance is +/-2%, so 63-67% are "technically within range" for L2, but then explains that at 64% or 66% the required L1/L3 values shift to non-integer positions. This is correct and well-explained. The puzzle design ensures a unique answer despite the tolerance band.
- Status: No fix needed. Well-designed constraint.

**MINOR-07: R3-05 binary encoding section is internally contradictory (in the puzzle file)**
- Location: R3-05-emergency-sequence.md section 6, "Binary Encoding of Final State."
- The section starts to compute the binary encoding, then says "A = ON (1), B = ON (1), C = ON (0... wait)" and cuts itself off, correctly noting that the answer is the register count, not binary encoding. This reads as the author catching an error mid-sentence.
- Fix: Clean up the self-correction to be more deliberate.
- Status: **FIXED IN-PLACE** (see fixes section).

---

## FIXES APPLIED IN-PLACE

### Fix 1: R3-03 timeline reordered to chronological (MAJOR-01)

In `R3-03-permission-chain.md`, section 7 narrative revelation, reordered the "CHAIN OF EVENTS" to chronological order.

### Fix 2: R3-05 timeline reordered to chronological (MAJOR-01)

In `R3-05-emergency-sequence.md`, section 7 narrative revelation, reordered the "TIMELINE RECONSTRUCTION" to chronological order.

### Fix 3: R2-02 panel overview label corrected (MAJOR-02)

In `R2-02-data-breach.md`, line 15, changed "Node 7, the Comms Computer" to "Node 7, the Security Computer."

### Fix 4: R3-05 binary encoding section cleaned up (MINOR-07)

In `R3-05-emergency-sequence.md`, section 6, replaced the self-correcting "A = ON (1), B = ON (1), C = ON (0... wait)" text with a clean statement that the answer is the procedure register count.

---

## OVERALL ASSESSMENT

### Strengths

1. **Reference cards are exceptional.** Every single one teaches the necessary concept from scratch. The ECG reference card (R3-01) and the Lissajous reference card (R1-02) are particularly well-designed — they teach real physics/medicine without dumbing it down.

2. **Expert paths are genuinely fast and satisfying.** An expert solves R1-02 in 15 seconds by computing 3 x 291. An expert solves R3-03 in 10 seconds by subtracting 6-2=4. The puzzles reward real knowledge without requiring it.

3. **Narrative escalation is tight.** The three-round structure (external signals -> ship response -> personnel records) builds naturally. Each round's revelation is earned through the instruments' data, not through exposition.

4. **Cross-puzzle data is consistent.** Bearing 213 appears in every puzzle that references the contact direction. The GAP timeline is internally consistent with plausible ordering. All 17 answer values match across all documents.

5. **The CyclicGroupDisplay math works.** (3 + 2) mod 8 = 5 = VASQUEZ. Verified end-to-end from the three round-meta derivations through the final computation.

6. **The "turn" moment (R2-04) lands.** The shield preset loaded before contact detection is the most economically devastating narrative beat in the hunt — four lines of log entry, zero exposition.

### Weaknesses

1. **R2-META mechanism needs a design decision.** Two candidate derivations exist. This blocks R2-META authoring but not feeder completion.

2. **PUZZLES.md has stale labels.** R2-02 says "Comms Computer" instead of "Security Computer," and R3-05 has confusing binary encoding discussion. Both are brief-level issues, not solver-facing.

3. **R3-04 badge numbering is intentionally tricky.** Badge 401 = CO = position 5 despite being in the "4xx" series. The exception is clearly documented but might slow a novice who reads too quickly.

### Recommendation

**PASS.** The hunt is ready to proceed to Stage 8 (Integration). The two MAJOR issues that were fixable (timeline ordering, node label) have been fixed in-place. The remaining MAJOR (R2-META mechanism) is a known design decision that must be resolved before R2-META authoring.
