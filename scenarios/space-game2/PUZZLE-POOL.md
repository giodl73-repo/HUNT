# DEAD RECKONING v2 — Puzzle Pool

**Stage 3 complete.** 34 candidates generated across 3 rounds. Panel ranked. 17 selected (6 + 6 + 5).

**Design direction:** Every puzzle is an instrument the solver operates. The answer is not a word — it is a physical measurement: a frequency, a bearing, a time offset, a data rate, a pressure differential, a temperature, a power draw value. The solver achieves the correct instrument state by reasoning through real physics. The instrument's display, at the correct setting, shows the answer value.

---

## ROUND 1 — TELEMETRY (External Signals)

Six instruments, one per puzzle. Each is a real piece of signal analysis equipment described in the world/systems/instruments/ reference files. The solver operates each one to characterize a different aspect of the mystery signal detected during the six-hour gap.

---

### R1-01: CARRIER ACQUISITION (Spectrum Analyzer)

#### Candidate A: Sweep Log Analysis — SELECTED

The spectrum analyzer's automatic sweep log shows 6 frequency sweeps captured at 7-minute intervals during the gap. Center frequency drifted between sweeps — someone was tuning. Each sweep shows a different spectrum. The solver identifies which sweep contains a real carrier (narrow bandwidth, stable amplitude, survives peak hold reasoning) versus noise or interference.

**The work:** The solver applies spectrum analyzer physics:
- Real carriers have bandwidth < RBW at all settings; noise bandwidth tracks RBW
- SNR_gain = 10*log10(span/RBW) — narrowing RBW improves carrier SNR but not noise
- A carrier survives peak hold across sweeps; noise converges to a flat ceiling
- Spurious harmonics appear at integer multiples of the fundamental

The solver determines the correct center frequency and span that would capture the carrier cleanly. The frequency readout IS the answer value.

**Answer type:** Frequency in GHz (e.g., 2.618)
**Difficulty:** Medium
**Physics required:** Spectrum analyzer carrier identification (reference: spectrum-analyzer.md)

**Panel notes:**
- **Thomas Snyder**: The carrier-vs-noise distinction requires understanding what RBW does physically, not just pattern matching. Passes the computer test.
- **Rand Miller**: This IS what a COMMS officer does at the spectrum analyzer. Riven Standard met.
- **Jonathan Blow**: The insight — realizing that one sweep's peak behaves differently from the others — is clean and singular.

#### Candidate B: Harmonic Hunt

Six peaks visible on a single wide-span sweep. The solver determines which is the fundamental and which are harmonics by checking for integer-multiple spacing. The fundamental's frequency is the answer.

**Panel notes:**
- **Wei-Hwa Huang**: Clean constraint — harmonics at n*f0 uniquely determine f0. But the solve is arithmetic, not insight.
- **Dan Katz**: Too computational. "Divide each frequency by each other frequency and check for integers" is a procedure, not a discovery.

#### Candidate C: Noise Floor Excavation

The carrier is below the visible noise floor. The solver must calculate the required RBW to lower the noise floor enough to reveal it, using the noise floor formula. The answer is the RBW setting.

**Panel notes:**
- **Dana Young**: The "invisible signal" premise is strong. The solver proves it exists by changing the instrument, not by looking harder.
- **Mark Gottlieb**: Risk — the formula application is direct computation. No deduction step.

**Consensus: Candidate A.** 10/12 panelists. The multi-sweep log creates a genuine investigation — the solver must figure out which sweep is real and why, not just compute a formula.

---

### R1-02: PHASE LOCK (Oscilloscope / Lissajous)

#### Candidate A: Drift Rate Analysis — SELECTED

The oscilloscope in X-Y mode shows a drifting Lissajous figure. CH1 is the ship's known outbound signal at a stated frequency. The figure is unstable — a slowly rotating pattern. The solver determines the frequency ratio from the figure's topology (counting tangencies), then calculates the exact CH2 frequency from the drift rate (beat frequency = |f1 - f2|).

**The work:**
- Count tangencies on vertical and horizontal axes to determine the frequency ratio (e.g., 3:2)
- Measure the drift rate from stated figure rotation period
- Calculate f_CH2 = f_CH1 * (horizontal_tangencies / vertical_tangencies) +/- drift_rate
- The phase angle at a stable 1:1 sub-ratio encodes additional information

**Answer type:** Frequency in MHz (the exact CH2 frequency at lock)
**Difficulty:** Medium-Hard
**Physics required:** Lissajous figures, frequency ratios, beat frequency (reference: oscilloscope.md)

**Panel notes:**
- **Jonathan Blow**: The two-step solve is elegant — first identify the ratio from shape, then refine with drift rate. Each step uses a different physical concept.
- **Thomas Snyder**: Counting tangencies requires understanding what they represent physically. Not just counting loops.
- **Rand Miller**: The drifting figure IS the puzzle. The solver's task is to freeze it (mentally). Instrument-as-puzzle fully realized.

#### Candidate B: Phase Angle Decoder

A stable 1:1 Lissajous figure at known frequency. The figure is a tilted ellipse. The solver determines the phase angle from the Y-intercept formula, and the phase angle in degrees is the answer.

**Panel notes:**
- **Dan Katz**: One formula, one measurement. Too direct.
- **Wei-Hwa Huang**: The formula application is unambiguous — no deduction required.

#### Candidate C: Multi-Figure Comparison

Five X-Y displays show Lissajous figures from five different sensor pairs. The solver identifies which figure is stable (locked) and which are drifting, then reads the locked figure's parameters.

**Panel notes:**
- **Peter Sarrett**: Comparing multiple displays is more engaging than a single figure. But the "which one is stable" question is binary, not deductive.
- **Lucas Pope**: The identification task is too quick. The meat should be in the analysis, not the selection.

**Consensus: Candidate A.** 11/12 panelists. The two-step ratio-then-drift solve is the strongest instrument puzzle in the pool. Both steps require genuine physical reasoning.

---

### R1-03: CORRELATION PEAK (Correlator)

#### Candidate A: Echo vs. Precursor — SELECTED

The correlator output shows 5 peaks at different lag values. Some are at positive tau (echoes — physically plausible responses). One is at negative tau (precursor — the signal existed before the outbound was sent, which is physically impossible for a response). The solver identifies which peaks are real echoes, which is the anomalous precursor, and calculates the distance to the dominant echo source using d = c*tau/2.

**The work:**
- Separate positive-tau peaks (echoes) from negative-tau peaks (precursors)
- The precursor implies the signal was NOT a response to the ship's outbound — it was anticipated or independent
- Among the positive-tau peaks, identify the dominant one (highest amplitude)
- Calculate distance: d = c * tau / 2 where c = 3e8 m/s
- The distance in km is the answer

**Answer type:** Distance in km (e.g., 51,000)
**Difficulty:** Medium
**Physics required:** Cross-correlation, echo delay, precursor anomaly (reference: correlator.md)

**Panel notes:**
- **Lucas Pope**: The precursor is the aha. "This signal came before ours was sent" recontextualizes everything. It's not an echo — it's something else entirely.
- **Wei-Hwa Huang**: The positive/negative tau distinction is physically grounded and unambiguous. Clean deduction.
- **Mike Selinker**: Narrative power — the precursor tells a story. Someone knew we were going to transmit before we did.

#### Candidate B: Multipath Triangulation

Three correlation measurements from three different sensor positions, each showing different tau values. The solver triangulates the source position from the three distances.

**Panel notes:**
- **Thomas Snyder**: Three-distance triangulation is geometry, not correlator physics. The instrument becomes a distance-measuring tool for a geometry puzzle.
- **Dan Katz**: The correlator is incidental. Any rangefinder gives the same data.

#### Candidate C: Composite Signal Decomposition

The correlation shows 8 peaks. The solver identifies which peaks come from the same source (regular spacing = multipath) and which are independent signals (irregular spacing). The number of independent sources is the answer.

**Panel notes:**
- **Jonathan Blow**: Counting independent sources is a classification task, not an instrument operation. The answer (a count) doesn't feel like a measurement.

**Consensus: Candidate A.** 10/12 panelists. The echo-vs-precursor distinction is the strongest single insight in Round 1 — a physical impossibility that demands explanation.

---

### R1-04: NULLING (Phased Array)

#### Candidate A: Steering Log Reconstruction — SELECTED

The phased array log shows 8 steering attempts during the gap. For each attempt: beam angle, null angle, and resulting SNR. The solver analyzes the 8 data points to determine which null angle produced maximum SNR improvement (= interference direction), then identifies the target signal's bearing as the beam angle that produced maximum signal strength with that null active.

**The work:**
- Plot or tabulate the 8 (null_angle, SNR) pairs
- The null angle with highest SNR = interference bearing
- With that null locked, the corresponding beam angle = target bearing
- The target bearing in degrees is the answer

**Answer type:** Bearing in degrees (e.g., 247)
**Difficulty:** Medium
**Physics required:** Null steering, interference rejection (reference: phased-array.md)

**Panel notes:**
- **Rand Miller**: The solver is literally replaying the original operator's attempts and figuring out what they were looking for. Instrument archaeology.
- **Dan Katz**: Eight data points is the right density. Enough to show the pattern, few enough to be tractable.
- **Kenny Young**: The tabulation is straightforward. No exotic math. The insight is recognizing that max SNR at a null angle means the interference IS at that angle.

#### Candidate B: Two-Source Nulling

Two interference sources at different angles. One null can only suppress one. The solver determines both directions and which one the operator chose to null, then infers the target bearing.

**Panel notes:**
- **Wei-Hwa Huang**: Two interference sources doubles the data and adds ambiguity. The "which one was nulled" question introduces a meta-layer that distances the solver from the instrument.
- **Thomas Snyder**: Overscoped. The single-source version is already a complete puzzle.

#### Candidate C: Dynamic Null Tracking

The interference source is moving. The log shows null angle adjustments over time. The solver extrapolates the interference trajectory and predicts its future bearing.

**Panel notes:**
- **Dan Katz**: Trajectory extrapolation is computation, not instrument reasoning. Fails the Riven test — the puzzle is math, not array operation.

**Consensus: Candidate A.** 11/12 panelists. The 8-attempt log is a clean dataset that rewards understanding of null steering without overcomplicating.

---

### R1-05: SPECTRAL DECOMPOSITION (FFT Analyzer)

#### Candidate A: Hidden Carrier Detection — SELECTED

The raw time-domain signal looks like noise. The solver configures the FFT analyzer (window function, sample rate, N) to reveal a hidden periodic signal buried in the noise floor. The correct configuration makes a carrier peak appear above the noise in a specific frequency bin. The bin number times the frequency resolution gives the carrier frequency.

**The work:**
- Select sample rate for the expected frequency range
- Select N for sufficient frequency resolution
- Select window function: Blackman if a weak signal is near a strong one; Hanning for general use
- Apply averaging to reduce noise variance (noise drops by 10*log10(n_avg) dB)
- Identify the bin with the carrier peak
- Calculate: f_signal = k_peak * fs / N
- The carrier frequency is the answer

**Answer type:** Frequency in kHz (e.g., 488.3)
**Difficulty:** Hard
**Physics required:** FFT parameters, window functions, averaging (reference: fft-analyzer.md)

**Panel notes:**
- **Thomas Snyder**: The solver must understand what each parameter does to choose correctly. Wrong window = signal masked. Wrong N = insufficient resolution. Wrong sample rate = signal aliased. Three independent choices, each physics-grounded.
- **Jonathan Blow**: "The signal is there but you can't see it yet" is a powerful premise. The solver changes their instrument, not the signal.
- **Mark Gottlieb**: Three-parameter configuration space is rich enough for a hard puzzle. The solver navigates the space by understanding, not by brute force.

#### Candidate B: Harmonic Fingerprint

The FFT shows a forest of peaks. The solver identifies which sets of peaks form harmonic series (bins at k, 2k, 3k), counts independent fundamental frequencies, and uses their bin numbers as the answer.

**Panel notes:**
- **Dan Katz**: Harmonic identification in FFT is the same skill as in the spectrum analyzer. Overlap with R1-01.
- **Wei-Hwa Huang**: The "forest of peaks" risks being visually overwhelming without clear entry point.

#### Candidate C: Window Function Shootout

The same signal analyzed with three different window functions. The solver compares the three results to determine which reveals the true signal and which are artifacts of leakage.

**Panel notes:**
- **Rand Miller**: Comparing three displays is strong instrument work. But the answer (which window is correct) is qualitative, not a measurement.
- **Dana Young**: Would work better as a step within a puzzle than as the entire puzzle.

**Consensus: Candidate A.** 9/12 panelists. The three-parameter configuration space (fs, N, window) creates a puzzle where every choice matters and all are physics-grounded.

---

### R1-06: VECTOR ANALYSIS (Vector Signal Analyzer)

#### Candidate A: Modulation Identification — SELECTED

The vector signal analyzer shows a constellation diagram of the mystery signal. The constellation is distorted: rotated by an unknown phase offset, scattered by noise (moderate EVM), and the modulation type is not immediately obvious. The solver determines the modulation type from the constellation geometry, applies phase correction to derotate, reads the symbol rate, and calculates the data rate.

**The work:**
- Identify modulation type from cluster count: 2 = BPSK, 4 = QPSK, 8 = 8-PSK, 16 = 16-QAM
- Apply phase correction to align constellation to standard orientation
- Read EVM to assess signal quality
- Determine bits/symbol from modulation type
- Calculate data rate = symbol_rate * bits_per_symbol
- The data rate in kbps is the answer

**Answer type:** Data rate in kbps (e.g., 128)
**Difficulty:** Medium-Hard
**Physics required:** IQ constellation, modulation types, data rate calculation (reference: vector-signal-analyzer.md)

**Panel notes:**
- **Lucas Pope**: The distorted constellation is a document that must be "read" correctly — rotated back to standard, identified, measured. Papers-Please applied to signal processing.
- **Thomas Snyder**: Modulation identification from a noisy constellation requires understanding what each modulation looks like. The scatter and rotation are physical distortions, not arbitrary encoding.
- **Alex Rosenthal**: The constellation diagram is visually striking and unfamiliar to most solvers. The learning moment — "these clusters represent bits" — is genuine.

#### Candidate B: Phase Offset Chain

Five signals received sequentially, each with a different phase offset. The offsets form a pattern (arithmetic sequence, etc.). The solver derotates each, identifies the pattern, and extracts the answer from the sequence.

**Panel notes:**
- **Dan Katz**: Pattern extraction from a sequence of offsets is a puzzle-hunt mechanic overlaid on instrument data. Not Riven Standard.
- **Jonathan Blow**: The phase offsets should mean something physically, not encode a pattern.

#### Candidate C: Multi-Signal Constellation

The constellation shows two overlapping modulation schemes (e.g., QPSK + BPSK on the same carrier). The solver separates the two signals by their different symbol rates and identifies each independently.

**Panel notes:**
- **Wei-Hwa Huang**: Signal separation is a real technique but the required analysis is very advanced. Accessibility concern for a puzzle hunt.
- **Kenny Young**: Overlapping constellations would be extremely confusing visually without specialized training.

**Consensus: Candidate A.** 10/12 panelists. The derotate-identify-calculate sequence is exactly how a COMMS officer reads an unknown signal. Clean, self-contained, assessable.

---

## Round 1 Summary

| Slot | Selected | Score | Answer type | Key physics |
|------|---------|-------|-------------|-------------|
| R1-01 | Sweep Log Analysis (Spectrum Analyzer) | 10/12 | Frequency (GHz) | Carrier vs. noise, RBW, peak hold |
| R1-02 | Drift Rate Analysis (Oscilloscope) | 11/12 | Frequency (MHz) | Lissajous tangencies, beat frequency |
| R1-03 | Echo vs. Precursor (Correlator) | 10/12 | Distance (km) | Cross-correlation lag, d = c*tau/2 |
| R1-04 | Steering Log Reconstruction (Phased Array) | 11/12 | Bearing (degrees) | Null steering, interference rejection |
| R1-05 | Hidden Carrier Detection (FFT) | 9/12 | Frequency (kHz) | FFT parameters, window functions |
| R1-06 | Modulation Identification (VSA) | 10/12 | Data rate (kbps) | IQ constellation, modulation type |

**Instrument variety:** Six distinct instruments, six distinct physics domains. No overlap.

---

## ROUND 2 — SYSTEMS LOG (Ship-Internal Instruments)

Ship-internal instruments using the v1 world data (power architecture, ODN topology, environmental systems, propulsion, damage control, navigation). Same mechanism as Round 1 — operate the instrument, achieve a target state, read the numeric answer — but the instruments are shipboard systems rather than signal analysis tools.

---

### R2-01: POWER AUDIT (EPS Distribution Panel)

#### Candidate A: Ghost Load — SELECTED

The EPS distribution panel shows current draw values for 8 systems during a snapshot from the gap. Three systems show loads that don't match their stated operating states — drawing power while listed as OFFLINE. The solver maps the phantom draw values against the canonical EPS consumption table (Impulse 180/120/60 MW, Shields 95 MW, Env Fields 25 MW, Transporter 40 MW, Replicator 35 MW, Holodeck 55 MW) to identify what was secretly running. The total phantom load in MW is the answer.

**Answer type:** Power in MW (e.g., 170)
**Difficulty:** Medium
**Physics required:** EPS power consumption reference (data-tables.md Table 4), system states

**Panel notes:**
- **Rand Miller**: The discrepancy between stated status and actual power draw — the instrument reveals the lie. Riven Standard.
- **Thomas Snyder**: The solver must understand what each system consumes to name it. Not pattern matching — systems knowledge.
- **Jonathan Blow**: "The ship was running something it denied running." Clean singular insight.

#### Candidate B: Cascade Trace

12 distribution nodes with trip-sequence timestamps. Reconstruct the cascade failure to find the initiating node.

**Panel notes:**
- **Mark Gottlieb**: Strong engineering puzzle but more complex than needed for this slot. Better suited to a harder position.

#### Candidate C: Battery Countdown

Battery at 40%, reactor down. Calculate time-to-failure for different system configurations. Optimal configuration's remaining time is the answer.

**Panel notes:**
- **Dan Katz**: Pure optimization computation. No deduction.

**Consensus: Candidate A.** 9/12. The phantom load mechanic elegantly uses the EPS reference table as the instrument.

---

### R2-02: DATA TRACE (ODN Junction Board)

#### Candidate A: Routing Reconstruction — SELECTED

The ODN junction board shows 15 nodes with current routing states (active/passive). A redacted maintenance log provides toggle-count timestamps but not labels. The solver traces data pathways using ODN routing rules (Primary Computer must maintain Navigation; active junctions pass data, passive block) to determine which nodes lost service and in what order. The final surviving data rate through the degraded network (in Gbps) is the answer.

**Answer type:** Data rate in Gbps (e.g., 4.7)
**Difficulty:** Medium-Hard
**Physics required:** ODN topology (data-tables.md Table 1), routing rules

**Panel notes:**
- **Wei-Hwa Huang**: Routing rules create a genuine constraint-satisfaction problem. The board state plus toggle count uniquely determines the sequence.
- **Lucas Pope**: Pure deductive identification. The answer cannot be guessed, only proved.
- **Mark Gottlieb**: Tracing data flow through a network is exactly how unauthorized access is diagnosed.

#### Candidate B: Gelpack Infection

Three sick gelpacks, determine which data pathways are compromised. Compromised systems' combined processing load is the answer.

**Panel notes:**
- **Thomas Snyder**: The organic/mechanical distinction is meaningful but the puzzle is mostly pathfinding, not instrument operation.

#### Candidate C: Bottleneck Identification

Processing load data for all 15 nodes during the gap. One node is overloaded. Identify it from queue depth data.

**Panel notes:**
- **Dan Katz**: Finding the max value in a list is not a puzzle.

**Consensus: Candidate A.** 10/12. Junction routing is the most authentic "operating the ODN board" experience.

---

### R2-03: PRESSURE MAP (Environmental Control Panel)

#### Candidate A: Breach Localization — SELECTED

The environmental control panel shows pressure readings (in kPa) for all 28 rooms across 5 decks, captured during the gap. Most rooms show nominal 101.3 kPa. Several rooms show reduced pressure in a pattern radiating from a single point. The solver traces the pressure gradient to localize the breach — the room with the lowest pressure (closest to vacuum) is the breach origin. But one set of rooms shows pressure drop that does NOT fit the radial pattern — those rooms were deliberately depressurized (emergency venting for security). The solver must separate breach damage from intentional venting.

**The work:**
- Map all 28 pressure readings onto the deck layout
- Identify the radial pressure gradient (breach: pressure decreases toward the source)
- Identify the anomalous pressure drop (venting: sudden, uniform drop in non-adjacent rooms)
- The breach room's pressure differential from nominal (in kPa) is the answer

**Answer type:** Pressure differential in kPa (e.g., 47.2)
**Difficulty:** Medium
**Physics required:** Pressure gradients, room layout (data-tables.md Table 2)

**Panel notes:**
- **Kenny Young**: 28 rooms is tractable. The radial gradient is visual and buildable.
- **Rand Miller**: Reading pressure sensors to find a hull breach IS environmental control. Riven Standard.
- **Lucas Pope**: The secondary pattern (intentional venting) adds depth — the solver must distinguish two different causes of the same symptom.

#### Candidate B: Atmosphere Composition

CO2/O2 ratio data across decks. The solver calculates scrubber efficiency and identifies the failing zone.

**Panel notes:**
- **Dana Young**: Ratio calculation without deductive step. Accessible but thin.

#### Candidate C: Gravity Setpoint Audit

Gravity plating readings across zones. Some zones show anomalous g-force. The solver calculates expected vs. actual and identifies the malfunctioning plate.

**Panel notes:**
- **Jonathan Blow**: Gravity setpoint is a strong concept but the puzzle is "find the number that doesn't match." Thin.

**Consensus: Candidate A.** 9/12. The breach-vs-venting distinction is a genuine two-layer deduction using the environmental panel as intended.

---

### R2-04: THRUST VECTOR (Propulsion Output Panel)

#### Candidate A: Course Reconstruction — SELECTED

The propulsion output panel shows thrust vector data and fuel consumption logs during the gap. The official flight record says the ship was stationary. But the fuel consumption doesn't match — fuel was burned. The solver calculates the actual acceleration, duration, and resulting displacement from the propulsion data to determine where the ship actually went. The displacement distance in km is the answer.

**The work:**
- Read thrust values (in kN), burn duration (in seconds), and fuel consumption (in kg)
- Cross-reference against ship mass to calculate acceleration: a = F/m
- Calculate displacement: d = 0.5 * a * t^2
- The official record claims d = 0 but the fuel says otherwise
- The actual displacement in km is the answer

**Answer type:** Distance in km (e.g., 2,340)
**Difficulty:** Medium-Hard
**Physics required:** F = ma, displacement kinematics, fuel consumption reference (departments.md PROPULSION)

**Panel notes:**
- **Dan Katz**: The discrepancy between the official record and the physical evidence IS dead reckoning. The hunt title earns its name.
- **Thomas Snyder**: Real physics (F=ma, kinematics) applied to real data. The solver uses freshman physics, not puzzle-hunt tricks.
- **Mark Gottlieb**: Ship mass must be provided. Fuel burn rate must be canonical. All data from world files.

#### Candidate B: Thruster Misfire Diagnosis

RCS thruster output logs show drift. The solver identifies which thruster pod is misfiring from the rotational drift vector.

**Panel notes:**
- **Wei-Hwa Huang**: Clean deduction but the answer (a thruster ID) is not a numeric measurement in the same sense.

#### Candidate C: Warp Field Asymmetry

Nacelle power distribution data. The solver calculates coil-by-coil power needed to restore symmetric field geometry.

**Panel notes:**
- **Kenny Young**: Warp field geometry is complex. Risk of overscoping for a single puzzle.

**Consensus: Candidate A.** 10/12. The fuel-vs-record discrepancy is thematically perfect for DEAD RECKONING and uses real kinematics.

---

### R2-05: DIAGNOSTIC RECONSTRUCTION (Master Diagnostics Terminal)

#### Candidate A: Redacted Diagnostic — SELECTED

The master diagnostic ran during the gap and produced a full-ship health report. Most of the report was subsequently redacted. But system checksums survive — every system has a stored healthy-state checksum and a current checksum. The solver compares checksums to identify which systems changed state during the gap (checksum mismatch = state changed). The number of systems that changed state is the answer.

**The work:**
- 15 systems listed with before/during/after checksums (hex strings)
- Before checksum = known healthy state
- During checksum = state during the gap
- After checksum = current state
- Systems where during != before changed state during the gap
- Some "during" checksums match "after" = changes were NOT reverted (permanent)
- Some "during" checksums differ from both before and after = changes occurred and were then partially reverted
- The count of systems with before != during is the answer

**Answer type:** Count (e.g., 7)
**Difficulty:** Hard
**Physics required:** System state model (WORLD.md), checksum comparison (conceptual)

**Panel notes:**
- **Mark Gottlieb**: Checksum forensics is real engineering practice. The solver reads the diagnostic output as an engineer would.
- **Jonathan Blow**: The three-state comparison (before/during/after) is the insight. A system that changed and reverted is suspicious. A system that changed and stayed is evidence.
- **Lucas Pope**: This is Obra Dinn for system states — reconstruct what happened from the surviving evidence.

#### Candidate B: Diagnostic Contradiction

The diagnostic report says FAULT on two systems, but those systems are currently ONLINE. The solver determines whether the fault was real (and subsequently repaired) or the diagnostic was spoofed.

**Panel notes:**
- **Dan Katz**: Binary determination (real fault vs. spoofed) is too thin for a hard puzzle.

#### Candidate C: Cascading Health Check

The diagnostic tests systems in dependency order. A failure at one level cascades to dependent systems. The solver identifies the root cause from the cascade pattern.

**Panel notes:**
- **Thomas Snyder**: Cascade tracing from the power dependency tree. Strong but overlaps with R2-01 (power audit).

**Consensus: Candidate A.** 9/12. The three-phase checksum comparison is the most forensically interesting approach and rewards careful systematic comparison.

---

### R2-06: NAVIGATION DRIFT (Helm Console)

#### Candidate A: Sensor Calibration Audit — SELECTED

The helm console displays navigation sensor readings from three independent sensor arrays during the gap. The arrays should agree but don't — one array is drifting. The solver cross-references the three arrays' readings (bearing, range, velocity) for 5 known reference objects to identify which array is miscalibrated and by how much. The calibration offset (in degrees of bearing error) is the answer.

**The work:**
- Three sensor arrays each report bearing, range, and velocity for 5 reference objects
- Two arrays agree within tolerance; one is consistently offset
- The offset is systematic (constant bearing error across all objects)
- The solver identifies the drifting array by majority agreement
- The bearing offset in degrees is the answer

**Answer type:** Bearing offset in degrees (e.g., 3.7)
**Difficulty:** Medium
**Physics required:** Sensor comparison, systematic error identification (departments.md NAV)

**Panel notes:**
- **Wei-Hwa Huang**: Three-array comparison with one outlier is a clean statistical deduction. The "majority rules" principle is physically grounded — two independent sensors agreeing is stronger than one disagreeing.
- **Kenny Young**: 5 reference objects x 3 arrays = 15 data points. Tractable on paper.
- **Dana Young**: Accessible — the solver doesn't need signal processing knowledge, just careful comparison.

#### Candidate B: Star Chart Drift

Two star chart snapshots (before and after the gap). The solver identifies which stars moved more than expected, implying the ship moved.

**Panel notes:**
- **Thomas Snyder**: Elegant concept but requires astrometry knowledge that isn't in the world files.

#### Candidate C: Dead Reckoning Fix

The ship's last known position, heading, and speed before the gap, plus its position after. The solver calculates the trajectory during the gap.

**Panel notes:**
- **Dan Katz**: Direct dead reckoning calculation. Thematically on point but computationally thin.

**Consensus: Candidate A.** 10/12. Three-array sensor comparison is genuine navigation work and avoids computational heaviness.

---

## Round 2 Summary

| Slot | Selected | Score | Answer type | Key physics |
|------|---------|-------|-------------|-------------|
| R2-01 | Ghost Load (EPS Panel) | 9/12 | Power (MW) | EPS consumption reference |
| R2-02 | Routing Reconstruction (ODN Board) | 10/12 | Data rate (Gbps) | ODN topology, routing rules |
| R2-03 | Breach Localization (Enviro Panel) | 9/12 | Pressure (kPa) | Pressure gradients, deck layout |
| R2-04 | Course Reconstruction (Propulsion) | 10/12 | Distance (km) | F=ma, displacement kinematics |
| R2-05 | Redacted Diagnostic (Master Diag) | 9/12 | Count | Checksum comparison, system states |
| R2-06 | Sensor Calibration Audit (Helm) | 10/12 | Bearing offset (degrees) | Sensor comparison, systematic error |

**Instrument variety:** EPS panel, ODN junction board, environmental zone display, propulsion output log, diagnostic terminal, helm sensor array. All ship-internal, all distinct.

---

## ROUND 3 — CREW RECORD (Personnel/Biological Data)

Five puzzles using personnel and biological data. The instruments read people, not signals. Medical biometrics, access logs, duty rosters, comm records, transporter logs. Same mechanism: operate the readout until the evidence resolves. The answer is a numeric measurement.

---

### R3-01: VITAL SIGNS (Medical Scanner)

#### Candidate A: Stress Biometric Analysis — SELECTED

The medical scanner shows biometric readings for 12 crew members during the gap: heart rate (bpm), cortisol level (nmol/L), blood pressure (mmHg), and galvanic skin response (microsiemens). Normal baselines are provided for each crew member. The solver identifies which crew members show stress signatures (elevated vitals above their personal baselines) that are consistent with a specific stressor pattern — not just general anxiety but the physiological profile of someone who knows they are doing something unauthorized. The peak heart rate delta (max BPM above baseline) among the stress-positive crew is the answer.

**Answer type:** Heart rate delta in BPM (e.g., 34)
**Difficulty:** Medium
**Physics required:** Biometric baselines, stress physiology (departments.md MEDICAL)

**Panel notes:**
- **Lucas Pope**: Medical intake logs as identification documents. Each crew member's biometrics are a "passport" the solver reads. Obra Dinn for bodies.
- **Dana Young**: The personal baseline comparison makes this accessible — you don't need to know normal heart rates, just which readings are abnormal for each person.
- **Mike Selinker**: "The body doesn't lie" — even if the crew won't talk, their vital signs tell the story.

#### Candidate B: Triage Sequencing

Multiple casualties from the gap. Severity scores and arrival times. The solver determines optimal triage order.

**Panel notes:**
- **Dan Katz**: Optimization, not deduction. The answer (a sequence) is not a numeric measurement.

#### Candidate C: Toxicology Screen

One crew member's blood work shows an unexpected compound. The solver identifies it from the medical database.

**Panel notes:**
- **Wei-Hwa Huang**: Compound identification from a database is a lookup, not a deduction.

**Consensus: Candidate A.** 10/12. Biometric baseline comparison is genuine medical scanner work and produces a clean numeric answer.

---

### R3-02: TRANSPORT LOG (Transporter Panel)

#### Candidate A: Phase Residue Decode — SELECTED

The transporter log was scrubbed, but transport events leave a phase residue in the pattern buffer — a decaying energy signature with a half-life of ~4 hours. The buffer shows residue levels at known times post-gap. The solver uses the decay curve to back-calculate when the transport occurred during the gap, then uses the phase frequency (in Hz) to identify the transport mode (organic/inorganic/cargo — each has a different phase frequency signature). The transport timestamp offset from the start of the gap (in minutes) is the answer.

**Answer type:** Time offset in minutes (e.g., 127)
**Difficulty:** Medium-Hard
**Physics required:** Exponential decay, half-life calculation, transport modes (departments.md TRANSIT)

**Panel notes:**
- **Jonathan Blow**: Exponential decay is freshman physics but applied to transporter phase residue. The solver uses real math on fictional data — the physics is real, the context is sci-fi. Perfect blend.
- **Thomas Snyder**: The half-life calculation requires understanding decay curves, not just plugging numbers. The solver must figure out which data points to use.
- **Rand Miller**: The transporter panel showing residue decay IS what the instrument does between transport events. Riven Standard met.

#### Candidate B: Coordinate Reconstruction

Pattern buffer retains partial destination coordinates. The solver reconstructs the full coordinates from fragments and error correction codes.

**Panel notes:**
- **Wei-Hwa Huang**: Error correction code reconstruction is a coding theory puzzle. Not instrument operation.

#### Candidate C: Biofilter Anomaly

The biofilter flagged something during a gap transport. The solver matches the biofilter reading against the medical database to identify the flagged substance.

**Panel notes:**
- **Dana Young**: Database matching is a lookup. Thin for a standalone puzzle.

**Consensus: Candidate A.** 10/12. Phase residue decay is a beautiful application of real physics (exponential decay) to the transporter instrument.

---

### R3-03: ACCESS MAP (Security Console)

#### Candidate A: Badge Swipe Triangulation — SELECTED

The security console shows badge swipe records for all crew during the gap. Most records are intact but the computer core anteroom log (the most sensitive location) was wiped. The solver reconstructs who entered the computer core by cross-referencing badge timestamps at adjacent checkpoints — if a crew member badged through Corridor Junction 5-A at T1 and appeared at EPS Control at T2, and the only route between those points passes through the computer core anteroom, then they entered the anteroom between T1 and T2. The transit time between the last-known and next-known badge locations (in seconds) is the answer.

**Answer type:** Transit time in seconds (e.g., 340)
**Difficulty:** Medium
**Physics required:** Ship layout, room adjacency (data-tables.md Table 2), personnel movement

**Panel notes:**
- **Lucas Pope**: Reconstructing a path from partial evidence. Each badge swipe is a timestamp that constrains where the person could have been. This is exactly what security does.
- **Dan Katz**: The ship layout becomes the constraint system. Adjacent rooms limit possible paths. Clean topology.
- **Peter Sarrett**: The "missing middle" — the wiped log — creates the mystery. The surrounding records tell the story.

#### Candidate B: Clearance Level Deduction

Console access logs show what each crew member queried. The solver determines their clearance level from their access pattern.

**Panel notes:**
- **Thomas Snyder**: Clearance deduction from access patterns is good but the answer (a clearance level) is categorical, not numeric.

#### Candidate C: Duty Roster Contradiction

The duty roster says one thing. Badge records say another. The solver identifies who was off-roster.

**Panel notes:**
- **Mark Gottlieb**: Binary comparison. Thin for a standalone puzzle.

**Consensus: Candidate A.** 11/12. Badge swipe triangulation through the ship layout is the strongest Round 3 candidate — topology constrains the deduction, and the wiped log creates the mystery.

---

### R3-04: COMMS DURATION (Internal Comms Log)

#### Candidate A: Call Pattern Analysis — SELECTED

The internal comms log shows all intra-ship calls during the gap: caller, recipient, duration (in seconds), and comm channel. The solver identifies an anomalous call pattern — a series of encrypted calls between two crew members that doesn't match the normal duty communication pattern. The calls are short, clustered in time, and use a non-standard encryption channel. The total duration of the anomalous call cluster (in seconds) is the answer.

**Answer type:** Duration in seconds (e.g., 847)
**Difficulty:** Medium
**Physics required:** Internal comms structure (departments.md COMMS), encryption channels

**Panel notes:**
- **Peter Sarrett**: The call pattern is a behavioral fingerprint. The solver identifies conspiracy from communication metadata, not from content. Relevant and contemporary.
- **Mike Selinker**: "Who called whom, when, and for how long" — the narrative emerges from dry data. Strong.
- **Kenny Young**: The normal-vs-anomalous distinction must be clear. Baseline call patterns need to be established in the data.

#### Candidate B: Encryption Key Audit

Key rotation timestamps show someone accessed an encryption key out of sequence. The solver traces the key access chain.

**Panel notes:**
- **Wei-Hwa Huang**: Key rotation analysis is niche. Accessibility concern.

#### Candidate C: Universal Translator Glitch

A partially garbled translation from the gap. The solver reconstructs the original message.

**Panel notes:**
- **Dan Katz**: Translation reconstruction is a linguistic puzzle, not an instrument operation.

**Consensus: Candidate A.** 9/12. Communication metadata analysis is accessible, thematically strong, and produces a clean numeric answer.

---

### R3-05: REPLICATOR AUDIT (Replicator Terminal)

#### Candidate A: Restricted Item Trace — SELECTED

The replicator log shows 24 replication events during the gap. Each event lists: item name, security category (OPEN/BIO/HAZ/POISON/SEC/UNK), requesting crew member, authorization status, and power draw (MW). Most events are normal (food, tools, routine supplies). Several events are anomalous: security-flagged items replicated with authorization overrides, items replicated at unusual times, or items whose power draw doesn't match their stated category (a "Standard" item drawing 35 MW when standard items draw < 5 MW). The total anomalous power draw (MW sum of all flagged events) is the answer.

**Answer type:** Power in MW (e.g., 142)
**Difficulty:** Hard
**Physics required:** Replicator security categories (data-tables.md Table 3), authorization tiers, EPS power reference

**Panel notes:**
- **Lucas Pope**: Every replication event is a document to be assessed. Authorization override + wrong power draw + wrong time of day = three independent red flags. Papers Please for replicator logs.
- **Thomas Snyder**: The solver needs to understand what each security category means and what authorization is required. A BIO item replicated without science clearance is a problem. Understanding the system IS the puzzle.
- **Jonathan Blow**: The power draw mismatch is the hidden layer — even if the authorization looks correct, the physics (power consumption) reveals the truth. The instrument doesn't lie.

#### Candidate B: Pattern Library Forensics

An unknown pattern appeared in the replicator database. The solver traces its origin through the ODN.

**Panel notes:**
- **Mark Gottlieb**: ODN tracing overlaps with R2-02. Different instrument, same skill.

#### Candidate C: De-Assembly Audit

Something was de-replicated (recycled) during the gap. The solver identifies what was destroyed from the residual matter-energy signature.

**Panel notes:**
- **Rand Miller**: Strong concept but "residual matter-energy signature" isn't documented in the world files. Would need new canon.

**Consensus: Candidate A.** 10/12. The three-layer audit (authorization + category + power draw) is the richest deductive puzzle in Round 3 and uses the replicator security system as designed.

---

### Additional Round 3 Candidate: R3-06: DUTY ROSTER (Personnel Database)

#### Candidate A: Scheduling Anomaly

The duty roster during the gap shows crew assignments that violate staffing rules (minimum crew per department, required certifications, rest period regulations). The solver identifies which assignments are illegal and determines how many crew-hours of violations occurred.

**Answer type:** Crew-hours (e.g., 18)
**Difficulty:** Medium
**Physics required:** Career progression, duty rules (departments.md CMD)

**Panel notes:**
- **Dan Katz**: Scheduling constraint violations are tractable and the "illegal assignment" framing is thematically strong.
- **Dana Young**: Good accessibility — the rules are stated, the violations are findable.
- **Wei-Hwa Huang**: Risk of being a "check all rules against all assignments" sweep. Needs a structural entry point.

**Not selected** — Round 3 is 5 puzzles. This is the strongest alternate if a selected puzzle is cut.

---

### Additional Round 3 Candidate: R3-07: AUTOPSY REPORT (Medical Forensics)

#### Candidate A: Time of Death Reconstruction

A crew member was found deceased after the gap. The medical scanner provides body temperature curve data. The solver uses Newton's Law of Cooling to back-calculate time of death. The time offset from the start of the gap (in minutes) is the answer.

**Answer type:** Time offset in minutes
**Difficulty:** Hard
**Physics required:** Newton's Law of Cooling, body temperature, ambient temperature

**Panel notes:**
- **Jonathan Blow**: Newton's Law of Cooling applied to forensics. Real physics, grim but effective.
- **Thomas Snyder**: The computation is direct but the solver must identify which data points to use and account for ambient temperature changes (the ship's environmental logs affect the cooling rate).
- **Mike Selinker**: Dark tone. Does it fit the hunt's "wry, tired, competent" register? A dead crew member escalates significantly.

**Not selected** — tone concern. Strong alternate for a darker-toned variant.

---

## Round 3 Summary

| Slot | Selected | Score | Answer type | Key physics |
|------|---------|-------|-------------|-------------|
| R3-01 | Stress Biometric Analysis (Medical Scanner) | 10/12 | Heart rate delta (BPM) | Biometric baselines, stress physiology |
| R3-02 | Phase Residue Decode (Transporter) | 10/12 | Time offset (minutes) | Exponential decay, half-life |
| R3-03 | Badge Swipe Triangulation (Security) | 11/12 | Transit time (seconds) | Ship layout, room adjacency |
| R3-04 | Call Pattern Analysis (Internal Comms) | 9/12 | Duration (seconds) | Comms metadata, pattern recognition |
| R3-05 | Restricted Item Trace (Replicator) | 10/12 | Power (MW) | Replicator security, authorization tiers |

**Instrument variety:** Medical scanner, transporter buffer readout, security badge console, internal comms log, replicator terminal. All personnel-focused, all distinct.

---

## Full Selection: 17 Puzzles

| ID | Title | Instrument | Answer Type | Difficulty |
|----|-------|-----------|-------------|------------|
| R1-01 | Carrier Acquisition | Spectrum Analyzer | Frequency (GHz) | Medium |
| R1-02 | Phase Lock | Oscilloscope (Lissajous) | Frequency (MHz) | Medium-Hard |
| R1-03 | Correlation Peak | Correlator | Distance (km) | Medium |
| R1-04 | Nulling | Phased Array | Bearing (degrees) | Medium |
| R1-05 | Spectral Decomposition | FFT Analyzer | Frequency (kHz) | Hard |
| R1-06 | Vector Analysis | Vector Signal Analyzer | Data rate (kbps) | Medium-Hard |
| R2-01 | Power Audit | EPS Distribution Panel | Power (MW) | Medium |
| R2-02 | Data Trace | ODN Junction Board | Data rate (Gbps) | Medium-Hard |
| R2-03 | Pressure Map | Environmental Panel | Pressure (kPa) | Medium |
| R2-04 | Thrust Vector | Propulsion Output | Distance (km) | Medium-Hard |
| R2-05 | Diagnostic Reconstruction | Master Diagnostics | Count | Hard |
| R2-06 | Navigation Drift | Helm Console | Bearing offset (degrees) | Medium |
| R3-01 | Vital Signs | Medical Scanner | Heart rate delta (BPM) | Medium |
| R3-02 | Transport Log | Transporter Panel | Time offset (minutes) | Medium-Hard |
| R3-03 | Access Map | Security Console | Transit time (seconds) | Medium |
| R3-04 | Comms Duration | Internal Comms Log | Duration (seconds) | Medium |
| R3-05 | Replicator Audit | Replicator Terminal | Power (MW) | Hard |

---

## Difficulty Distribution

| Difficulty | Count | Positions |
|-----------|-------|-----------|
| Medium | 7 | R1-01, R1-03, R1-04, R2-01, R2-03, R2-06, R3-01, R3-03, R3-04 |
| Medium-Hard | 5 | R1-02, R1-06, R2-02, R2-04, R3-02 |
| Hard | 3 | R1-05, R2-05, R3-05 |

Each round has exactly one Hard puzzle as its capstone. Difficulty ramps within rounds and across them.

---

## Answer Type Diversity

| Answer Type | Count | Puzzles |
|------------|-------|---------|
| Frequency | 3 | R1-01 (GHz), R1-02 (MHz), R1-05 (kHz) |
| Distance | 2 | R1-03 (km), R2-04 (km) |
| Bearing/Angle | 2 | R1-04 (degrees), R2-06 (degrees) |
| Data rate | 2 | R1-06 (kbps), R2-02 (Gbps) |
| Power | 2 | R2-01 (MW), R3-05 (MW) |
| Pressure | 1 | R2-03 (kPa) |
| Count | 1 | R2-05 |
| Heart rate | 1 | R3-01 (BPM) |
| Time | 2 | R3-02 (minutes), R3-04 (seconds) |
| Transit time | 1 | R3-03 (seconds) |

No answer type appears more than 3 times. Sufficient diversity for meta architecture.
