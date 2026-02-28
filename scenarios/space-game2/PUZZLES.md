# DEAD RECKONING v2 — Puzzle Briefs (Widget-First Design)

**Stage 4 complete.** 17 puzzles across 3 rounds. Every puzzle is built around specific named widgets from the console-widget-catalog. The player OPERATES the equipment. The interaction IS the puzzle.

**Answer format:** All answers are numeric values — instrument readouts achieved by correct widget operation.

---

## ROUND 1 — TELEMETRY (6 puzzles)

External signal analysis. Each puzzle uses display widgets from the console catalog paired with input controls. The solver operates the instruments to characterize the contact detected during the six-hour gap.

---

### R1-01 — Carrier Isolation

**Answer:** 2.340 GHz

**Widgets:**
- `SineWaveDisplay` — combined incoming signal (3-component waveform with target overlay)
- `NumericStepper` — filter center frequency (range: 1.000–5.000 GHz, step: configurable)
- `RotaryDial` — step size selector (positions: 1 Hz / 10 Hz / 100 Hz)
- `LCARSButton` — APPLY FILTER (momentary, triggers bandpass filter at current frequency)

**The Situation:**

A combined signal with 3 components arrived during the gap. The SineWaveDisplay shows the raw incoming waveform — a messy composite of three overlapping sinusoids. One is the target carrier. The other two are interference: a broadband noise component and a harmonic artifact.

The ship's bandpass filter isolates signals within a narrow window around the center frequency. The solver tunes the NumericStepper to move the filter center frequency, using the RotaryDial to switch between coarse (100 Hz) and fine (1 Hz) adjustment. When APPLY FILTER is pressed, the SineWaveDisplay redraws showing only the signal content within the filter passband.

A target wave overlay (shown in a contrasting color) represents the theoretical pure carrier. When the filter center frequency is tuned correctly, the filtered signal aligns with the target overlay — the waveforms match in frequency and phase.

**The Work:**
1. Start with coarse adjustment (RotaryDial → 100 Hz). Sweep through the frequency range. The SineWaveDisplay shows filtered output at each setting.
2. When a region of interest appears (filtered waveform begins to resemble the target overlay), switch to fine adjustment (10 Hz, then 1 Hz).
3. At the correct frequency, the filtered waveform locks onto the target overlay. The two waves align visually — same frequency, same shape.
4. The frequency readout at alignment = the answer.

**Win Condition:** Target wave overlay aligns exactly with the filtered waveform. The NumericStepper reads 2.340 GHz.

**Physics:**
- Bandpass filter passes only frequencies near the center frequency
- The carrier is a narrow-bandwidth signal; interference is broad
- When the filter center matches the carrier, the carrier passes through cleanly
- The two interference components are rejected because they are at different frequencies
- SNR improves as the filter narrows around the carrier

**Narrative:** During the gap, someone at the COMMS console was trying to isolate a signal. The filter log was deleted, but the raw combined signal was still in the buffer. The solver redoes the work the original operator performed.

---

### R1-02 — Phase Lock

**Answer:** 873 MHz

**Widgets:**
- `LissajousDisplay` — X-Y parametric curve with fade trail and leading dot
- `NumericStepper` — Y frequency (range: 100–2000 MHz, step: configurable)
- `NumericStepper` — X frequency (fixed readout: 291 MHz — the ship's known outbound)
- `RotaryDial` — step size selector (positions: 0.1 / 1 / 10 MHz)

**The Situation:**

The oscilloscope is in X-Y mode. CH1 (X axis) is the ship's known outbound signal at 291 MHz (displayed as a fixed readout, not adjustable). CH2 (Y axis) is the incoming mystery signal at an unknown frequency. The LissajousDisplay shows a drifting spiral — the two signals are near-matched but not locked. The figure rotates slowly, indicating a beat frequency (small frequency mismatch).

The solver adjusts the Y frequency NumericStepper until the Lissajous figure stabilizes. A stable figure means the frequency ratio is exactly rational (p:q in small integers). The figure's topology (counted tangencies) reveals the ratio.

**The Work:**
1. Observe the drifting figure. Count tangencies on the bounding box edges when the figure momentarily stabilizes (or use persistence to reveal the envelope).
2. The figure shows 3 tangencies on the vertical axis and 1 on the horizontal → frequency ratio is 3:1.
3. Therefore f_Y = 3 × f_X = 3 × 291 = 873 MHz.
4. Tune the Y NumericStepper toward 873 MHz using coarse steps (10 MHz), then fine steps (1 MHz, 0.1 MHz).
5. As the stepper approaches 873 MHz, the LissajousDisplay figure slows its rotation and stabilizes into a clean 3:1 pattern.
6. At exactly 873 MHz, the figure locks — no rotation, stable shape.

**Win Condition:** LissajousDisplay shows a stable 3:1 Lissajous figure (3 vertical tangencies, 1 horizontal). The Y frequency NumericStepper reads 873 MHz.

**Physics:**
- Stable Lissajous figures occur when f_Y / f_X = p/q (rational ratio of small integers)
- Tangencies on vertical axis = numerator (f_X), horizontal axis = denominator (f_Y)
- Drift rate = beat frequency = |f_actual - f_set|
- When drift rate → 0, the figure stabilizes (phase lock achieved)
- A 3:1 ratio means the contact was transmitting at exactly 3× the ship's outbound — harmonic locking behavior, characteristic of a phase-locked echo

**Narrative:** The incoming signal was phase-locked to the ship's own transmission at a 3:1 harmonic ratio. This is not natural. Something was actively matching the ship's frequency.

---

### R1-03 — Null Zone

**Answer:** 213 degrees

**Widgets:**
- `PhaseInterferenceDisplay` — two draggable wave sources on a heatmap grid; bright = constructive, dark = destructive interference
- `LinkedSliderInput` — Source 1 frequency (range: 100–1000 MHz)
- `LinkedSliderInput` — Source 2 frequency (range: 100–1000 MHz)
- `ToggleSwitch` — symmetry lock (when ON, sources mirror each other about the center axis)

**The Situation:**

The ship's jamming array uses two wave sources to create destructive interference (a null zone) at a specific bearing. During the gap, the array was used to null interference at an unknown bearing so a hidden signal could be received. The solver must recreate the null configuration.

The PhaseInterferenceDisplay shows a live heatmap. Bright regions = constructive interference (signal enhanced). Dark regions = destructive interference (signal canceled). The solver positions the two wave sources (by dragging them on the display) and adjusts their frequencies until a dark null zone appears at a specific bearing.

A target bearing is marked on the heatmap edge with a small indicator: "TARGET SIGNAL BEARING: ???°". A separate indicator shows "NULL ZONE BEARING: [calculated from current source positions]". When the null zone falls on the interference source direction (not the target), the target signal emerges — shown as a bright spot at the target bearing.

**The Work:**
1. The heatmap shows the current interference pattern. A faint target signal is marked at an unknown bearing.
2. The solver adjusts Source 1 position (dragging on the display) and Source 1 frequency to create a null zone.
3. With symmetry lock ON, Source 2 mirrors Source 1.
4. When the null zone is placed at the correct bearing (the interference source direction), the interference is suppressed and the target signal becomes visible.
5. The PhaseInterferenceDisplay reveals the target bearing: the bright spot appears at 213°.
6. Source 1's position bearing (the bearing from center to where Source 1 is placed) = the answer.

**Win Condition:** Heatmap shows a dark null at the interference bearing, and the target signal emerges as a bright spot at 213°. Source 1 position indicates bearing 213° for the revealed target.

**Physics:**
- Two coherent sources create an interference pattern
- Destructive interference (null) occurs where path length difference = half wavelength
- Positioning sources to null a specific direction suppresses signals from that direction
- The target signal, coming from a different direction, is not affected by the null
- Source frequency determines the spacing of the null pattern

**Narrative:** The jamming array was configured to null interference from one direction so the contact's signal from bearing 213° could be received cleanly. The solver reconstructs that configuration.

---

### R1-04 — Orbit Classification

**Answer:** 0.73 (eccentricity)

**Widgets:**
- `ConicSectionDisplay` — polar conic curve with tracer dot, showing observed trajectory overlay
- `NumericStepper` — eccentricity (range: 0.00–2.00, step: 0.01)
- `NumericStepper` — semi-latus rectum (range: 1–100 km, step: 1)
- `RotaryDial` — step size for eccentricity (positions: 0.01 / 0.10)

**The Situation:**

An object was tracked during the gap. Its trajectory data is plotted on the ConicSectionDisplay as a series of position points (an overlay of observed positions). The solver adjusts the eccentricity and semi-latus rectum until the theoretical conic curve matches the observed trajectory overlay.

The ConicSectionDisplay morphs smoothly as eccentricity changes: circle (e=0) → ellipse (0<e<1) → parabola (e=1) → hyperbola (e>1). A tracer dot moves along the theoretical curve. The observed trajectory overlay is a set of fixed dots from the sensor log.

**The Work:**
1. Observe the overlay of tracked positions. They form an arc — clearly not a circle, not a full ellipse.
2. Start with RotaryDial on coarse (0.10 step). Sweep eccentricity from 0.00 upward.
3. At e ≈ 0.70, the theoretical curve begins to match the observed arc shape.
4. Switch to fine step (0.01). At e = 0.73, the curve passes through all observed position points.
5. Adjust semi-latus rectum to scale the curve correctly (fitting the distance scale of the observations).
6. When both parameters are correct, all observed positions lie on the theoretical curve.

**Win Condition:** ConicSectionDisplay theoretical curve matches observed trajectory overlay at e = 0.73. The eccentricity readout = the answer.

**Physics:**
- Conic sections describe orbital/trajectory paths under gravity
- e = 0: circular orbit (bound, periodic)
- 0 < e < 1: elliptical orbit (bound, periodic)
- e = 1: parabolic trajectory (unbound, escape)
- e > 1: hyperbolic trajectory (unbound, flyby)
- e = 0.73: a highly eccentric ellipse — the object was in a capture trajectory, approaching from deep space and being gravitationally deflected but not permanently captured. This is characteristic of a flyby contact that briefly enters sensor range.

**Narrative:** The contact's trajectory was a hyperbolic-capture approach — it came in close, held station briefly (consistent with the 51 km distance from the correlator data in v1), then departed. The eccentricity characterizes its approach dynamics.

---

### R1-05 — Contact Lock

**Answer:** 5 (real contacts)

**Widgets:**
- `RadarSweepDisplay` — rotating sweep with 8 contact blips moving autonomously
- `TargetingReticleDisplay` — crosshair with lock indicator (green when locked)
- `LightedButton` array — 8 buttons, one per contact (toggle: REAL = green lit / DECOY = red lit)

**The Situation:**

The radar scope shows 8 contacts detected during the gap. Some are real objects; others are sensor ghosts (decoys). Real contacts maintain consistent bearing under targeting lock — when the TargetingReticleDisplay locks onto them, they hold steady. Decoys scatter or fade when locked — the targeting lock disrupts the ghost pattern.

The solver locks each contact using the TargetingReticleDisplay and observes its behavior:
- **Real contact:** Holds bearing and range. Lock indicator stays green. Bearing readout is stable.
- **Decoy:** Bearing drifts or blip fades. Lock indicator flickers. Bearing readout wanders.

After testing each contact, the solver marks it as REAL (green) or DECOY (red) using the corresponding LightedButton.

**The Work:**
1. The RadarSweepDisplay shows 8 blips at various bearings and ranges.
2. Select contact 1. The TargetingReticleDisplay tracks it. Observe: bearing stable, lock green → REAL. Press LightedButton 1 to green.
3. Select contact 2. Lock it. Observe: bearing drifts, lock flickers → DECOY. Press LightedButton 2 to red.
4. Repeat for all 8 contacts.
5. Count the green (REAL) buttons. The count = the answer.

**Win Condition:** All 8 contacts classified. 5 buttons lit green (REAL), 3 lit red (DECOY). Count of REAL = 5.

**Physics:**
- Radar returns from real objects have consistent Doppler and bearing signatures
- Ghost returns (multipath reflections, sidelobe artifacts) have inconsistent signatures
- Targeting lock tests consistency — a real return maintains coherence under scrutiny
- Decoys may appear strong on the initial sweep but fail under lock

**Narrative:** During the gap, 8 contacts appeared on radar. The original operator had to distinguish real from decoy. Five were real — the primary contact and four escort/companion objects.

---

### R1-06 — Signal Fingerprint

**Answer:** 384 kbps

**Widgets:**
- `CommSignalDisplay` — IQ constellation diagram + waveform overlay
- `RotaryDial` — modulation type selector (positions: BPSK / QPSK / 8-PSK / 16-QAM)
- `NumericStepper` — symbol rate (range: 1–1000 kSym/s, step: 1)

**The Situation:**

The CommSignalDisplay shows the IQ constellation of the mystery signal. The constellation points are scattered — rotated from standard orientation and noisy. The solver must identify the modulation type and symbol rate to calculate the data rate.

The RotaryDial selects modulation type. When the correct type is selected, the CommSignalDisplay overlays the ideal constellation positions — the scattered points cluster around the ideal positions (low EVM). When the wrong type is selected, the clusters don't align with the ideal positions (high EVM).

The NumericStepper sets the assumed symbol rate. When the correct rate is entered, the CommSignalDisplay's waveform overlay shows clean symbol transitions. When incorrect, the transitions blur.

**The Work:**
1. Observe the constellation. Count approximate cluster regions: there appear to be 8 clusters arranged in a ring → likely 8-PSK.
2. Turn RotaryDial to 8-PSK. The ideal constellation overlay appears. The clusters tighten around the 8 ideal positions. EVM readout drops to < 15%.
3. Check other modulation types: BPSK shows only 2 ideal positions (clusters don't fit), QPSK shows 4 (clusters don't fit), 16-QAM shows a grid (clusters don't fit).
4. With 8-PSK confirmed (3 bits/symbol), adjust the symbol rate NumericStepper.
5. The waveform overlay sharpens when symbol rate = 128 kSym/s.
6. Data rate = symbol_rate × bits_per_symbol = 128 × 3 = **384 kbps**.
7. The CommSignalDisplay shows the calculated data rate readout.

**Win Condition:** RotaryDial set to 8-PSK, NumericStepper set to 128 kSym/s. Data rate readout = 384 kbps.

**Physics:**
- IQ constellation geometry identifies modulation type (2 points = BPSK, 4 = QPSK, 8 = 8-PSK, 16 = 16-QAM)
- Bits per symbol: BPSK=1, QPSK=2, 8-PSK=3, 16-QAM=4
- Data rate = symbol_rate × bits_per_symbol
- Correct modulation selection minimizes EVM (error vector magnitude)
- Correct symbol rate produces clean eye diagram / waveform transitions

**Narrative:** The contact was transmitting at 384 kbps using 8-PSK modulation — a moderately complex scheme indicating deliberate, structured communication rather than a simple beacon.

---

## ROUND 2 — SYSTEMS LOG (6 puzzles)

Ship-internal instruments. Each puzzle reconstructs an aspect of the ship's response during the gap by operating console widgets that display internal system states.

---

### R2-01 — Power Path

**Answer:** 2 (active conduits)

**Widgets:**
- `ConduitFlowDisplay` — 3 parallel power conduits (EPS PRIMARY / EPS SECONDARY / DATA-ODN) with flowing particles and junction nodes
- `ToggleSwitch` (×3) — one per conduit (enable/disable)
- `GaugeDisplay` (×3) — one per destination system (Shields, Impulse, Life Support), showing power delivery level

**The Situation:**

EPS conduits were damaged during the gap. Only some conduits can be active simultaneously without exceeding junction capacity. The solver must enable/disable the 3 conduits to route power to 3 critical systems (Shields, Impulse, Life Support) simultaneously, keeping all 3 GaugeDisplay needles in the green zone.

The ConduitFlowDisplay shows flowing particles in enabled conduits and static conduits when disabled. Each conduit feeds specific destination systems through junction nodes. The gauges show real-time power delivery.

**The Work:**
1. Start with all 3 conduits enabled. Observe: junction overload warning appears (red flash at the junction node). One gauge drops to red — the junction can't handle all 3 conduits at once.
2. Disable EPS SECONDARY. Gauges: Shields drops to yellow (insufficient), Impulse green, Life Support green. Not all in green.
3. Re-enable SECONDARY, disable DATA-ODN. Gauges: Shields green, Impulse green, Life Support drops to yellow. Not all green.
4. Try: EPS PRIMARY + EPS SECONDARY enabled, DATA-ODN disabled. Gauges: all three in green zone. Junction load nominal.
5. Try other combinations: PRIMARY only = two gauges red. SECONDARY only = one gauge red.
6. Only the combination PRIMARY + SECONDARY (2 active conduits) puts all 3 gauges in green.

**Win Condition:** All 3 destination gauges in green zone simultaneously. Active conduit count = 2.

**Narrative:** During the gap, the EPS routing had to be reconfigured to maintain critical systems after conduit damage. The two-conduit configuration reveals which conduit was sacrificed and why.

---

### R2-02 — Data Breach

**Answer:** 7 (breach entry node ID)

**Widgets:**
- `NetworkGridDisplay` — 15-node ODN network with traveling data packets and per-node load indicators
- `ToggleSwitch` (×15) — one per node (enable/disable), allowing the solver to isolate nodes

**The Situation:**

Unauthorized data was routed through the ODN during the gap. The NetworkGridDisplay shows the 15 ODN nodes with data packets flowing between them. A rogue data packet (shown in red, distinct from normal blue packets) is circulating through the network, following a specific route.

The solver toggles nodes OFF to isolate segments of the network. When a node on the rogue packet's route is disabled, the red packet stops at the previous node (it can't pass through a disabled node). By systematically disabling nodes and observing where the red packet stops, the solver traces the rogue route backward to find the entry point.

**The Work:**
1. Observe the red packet's path. It circulates through several nodes.
2. Disable node 15. Red packet still flows → node 15 is not on the route.
3. Disable node 10. Red packet stops at node 12 → node 10 is on the route, between 12 and the next node.
4. Re-enable node 10. Disable node 7. Red packet stops at the network edge → node 7 is the entry point (the packet can't enter the network).
5. Verify: enable all nodes except 7. Red packet never appears → confirmed entry at node 7.

**Win Condition:** Node 7 identified as the breach entry point. The solver has traced the rogue packet to its origin node.

**Narrative:** Someone injected unauthorized data into the ODN through node 7 — the Comms Computer node. This was used to forge sensor readings during the gap, making it appear that no contact was detected.

---

### R2-03 — Heat Source

**Answer:** 14 (grid coordinate)

**Widgets:**
- `HeatMapDisplay` — thermal diffusion grid showing temperature distribution across a deck section
- `LCARSButton` (×2) — PLACE COOLANT / REMOVE COOLANT (toggles coolant zones on the grid)
- `GaugeDisplay` — peak temperature readout

**The Situation:**

An anomalous heat source appeared during the gap. The HeatMapDisplay shows the current thermal state — heat has diffused outward from the source, creating a warm gradient across the grid. The source location is hidden within the diffused pattern.

The solver places coolant zones (by clicking grid positions and pressing PLACE COOLANT) to absorb heat and isolate the source. When coolant is placed optimally, the diffused heat is drawn away from peripheral areas, and the source location becomes visible as the hottest remaining point.

**The Work:**
1. The HeatMapDisplay shows a broad warm area (yellow-orange gradient) across the grid. No obvious single hot spot.
2. Place coolant at the edges of the warm area. The heat map updates — coolant absorbs nearby heat, and the gradient sharpens.
3. Place more coolant zones, working inward. The warm area shrinks.
4. After placing 4-5 coolant zones, a clear hot spot emerges at grid coordinate 14.
5. The GaugeDisplay confirms: peak temperature is at coordinate 14.
6. Removing coolant causes the heat to diffuse again, obscuring the source.

**Win Condition:** Peak temperature isolated to grid coordinate 14. The GaugeDisplay shows the source location.

**Narrative:** The heat source was the deflector array operating in a non-standard mode — a focused scan directed at the contact. The deflector generated excess thermal energy at grid position 14 (Deflector Control, Deck 5).

---

### R2-04 — Shield Profile

**Answer:** 65 (Layer 2 power percentage)

**Widgets:**
- `ShieldDisplay` — 3-layer concentric hexagonal shield with per-layer strength indicators
- `PowerSlider` (×3) — one per shield layer (range: 0–100%, step: 1%)
- `GaugeDisplay` — incoming threat power readout (shows simulated attack matching the gap's damage pattern)

**The Situation:**

An attack profile hit the ship during the gap. The damage pattern is recorded in the log — specific layers took specific damage amounts. The solver must recreate the exact shield power allocation that produced the logged damage pattern.

The ShieldDisplay shows the 3 layers. The solver adjusts each layer's PowerSlider to set its strength. The GaugeDisplay shows the incoming threat power. When the shield configuration matches the logged state, the resulting damage distribution matches the recorded pattern — the ShieldDisplay shows damage indicators matching the log exactly.

**The Work:**
1. The logged damage pattern is displayed as a reference overlay on the ShieldDisplay: Layer 1 took heavy damage, Layer 2 took moderate damage, Layer 3 took light damage.
2. If all layers at 100%: the damage would be distributed evenly. Doesn't match — log shows uneven damage.
3. Adjust Layer 1 to lower power (it took the most damage, so it was weakest). Try 30%.
4. Layer 2 at moderate power. Try 65%.
5. Layer 3 at high power. Try 90%.
6. At 30/65/90, the ShieldDisplay damage pattern matches the log overlay. The damage indicators align.
7. Experimenting with nearby values (64%, 66%) shows the match is exact only at 65% for Layer 2.

**Win Condition:** Shield power allocation produces damage pattern matching the log. Layer 2 PowerSlider reads 65%.

**Narrative:** The shield allocation during the gap was deliberately asymmetric — Layer 1 was weakened to redirect power to other systems, Layer 2 was set at 65% as a compromise, Layer 3 was strengthened to protect the aft section where the contact was approaching.

---

### R2-05 — Fault Trace

**Answer:** 9 (failed component ID)

**Widgets:**
- `CircuitTopologyDisplay` — EPS circuit with 20 nodes and current flow visualization
- `LCARSButton` (×20) — PROBE buttons, one per node (momentary press, tests for current at that node)

**The Situation:**

An EPS component failed during the gap. The CircuitTopologyDisplay shows the EPS circuit with 20 nodes connected by conduits. Current flows from the source node through the circuit. At the failed component, current stops — nodes downstream of the failure show no current.

The solver probes nodes by pressing the corresponding LCARSButton. Each probe test shows whether current is present (green flash) or absent (red flash) at that node.

**The Work:**
1. Probe the source node (node 1). Green flash — current is entering the circuit.
2. Probe the final destination node (node 20). Red flash — no current reaching the end.
3. The failure is somewhere between node 1 and node 20. Binary search.
4. Probe node 10. Green — current reaches node 10. Failure is downstream.
5. Probe node 15. Red — no current at node 15. Failure is between 10 and 15.
6. Probe node 12. Green. Probe node 13. Red.
7. Failure is between node 12 and node 13. But the topology branches here.
8. Probe node 9 (which feeds node 13 via a branch). Green at 9, red at the next node downstream.
9. The failed component is at node 9's output junction — component ID 9.

**Win Condition:** All probe data identifies component 9 as the failure point — the last node with current before it disappears.

**Note:** The circuit topology is not a simple linear chain — it has branches and junctions, making the binary search non-trivial. The solver must trace the actual circuit topology to determine which nodes are upstream/downstream.

**Narrative:** Component 9 is an EPS relay in the sensor distribution circuit. Its failure cut power to the sensor recording subsystem — and the failure was not logged because the logging system itself depended on that relay. The failure may have been induced deliberately.

---

### R2-06 — Reactor State

**Answer:** 4 (throttle position)

**Widgets:**
- `GaugeDisplay` (×4) — reactor temperature, reactor pressure, coolant flow rate, power output
- `ThrottleLever` — fuel flow (range: 0–8, detent positions at integers)
- `PowerSlider` — containment field strength (range: 0–100%)

**The Situation:**

The reactor was at a specific state during the gap — a snapshot of four gauge readings was recovered from a backup sensor. The solver adjusts the ThrottleLever (fuel flow) and PowerSlider (containment) until all 4 GaugeDisplay readings match the logged values simultaneously.

**The Work:**
1. The logged values are displayed as setpoint markers on each gauge: temperature = 847°C, pressure = 23.4 MPa, coolant flow = 12.6 L/s, power output = 312 MW.
2. Start with ThrottleLever at 0. All gauges read zero. Move throttle up.
3. At throttle position 2: temperature 400°C, pressure 11 MPa. Too low.
4. At throttle position 6: temperature 1100°C, pressure 34 MPa. Too high.
5. At throttle position 4: temperature ~840°C, pressure ~23 MPa. Close.
6. Adjust PowerSlider to fine-tune. At containment 72%: temperature = 847°C, pressure = 23.4 MPa. Two gauges match.
7. Check coolant flow and power output: both match at this setting.
8. All 4 gauges at setpoint markers. The lever clicks into the detent at position 4.

**Win Condition:** All 4 gauges match logged values. ThrottleLever reads position 4.

**Physics:**
- Fuel flow rate (throttle) determines heat generation rate
- Containment field strength determines how much heat is retained vs. radiated
- The four gauge readings are coupled — changing one control affects multiple gauges
- Only one combination of throttle + containment produces all four target values simultaneously
- The detent snap confirms the correct position

**Narrative:** The reactor was at throttle 4 during the gap — a moderate power level, not the maximum. This is consistent with the ship maintaining station near the contact rather than maneuvering aggressively. The containment setting (72%) diverted some heat to the deflector array.

---

## ROUND 3 — CREW RECORD (5 puzzles)

Personnel and biological data. Each puzzle uses console widgets to read people, not signals. Medical biometrics, access systems, permission structures, security logs, emergency procedures.

---

### R3-01 — Triage

**Answer:** 3 (patient ID)

**Widgets:**
- `LifesignsDisplay` — 3-channel ECG trace showing morphing waveforms for 3 patients
- `LinkedSliderInput` (×3) — heart rate adjustment for each patient (range: 60–200 BPM)
- `RotaryDial` — active patient selector (positions: Patient 1 / Patient 2 / Patient 3)

**The Situation:**

Three crew members logged unusual vital signs during the gap. The LifesignsDisplay shows 3 ECG channels simultaneously. Each channel shows a waveform that changes morphology as heart rate changes — normal PQRST at 72 BPM, degrading to V-tach at high rates.

The solver's task: identify which patient's waveform matches ventricular tachycardia (V-tach) — the crisis waveform. V-tach is characterized by wide, regular QRS complexes with no discernible P-waves, at rates above 150 BPM.

**The Work:**
1. Observe the 3 channels. All show elevated heart rates (above resting baseline).
2. Use RotaryDial to select Patient 1. Adjust LinkedSliderInput to see how the waveform responds. At the logged BPM, the waveform shows narrow QRS complexes with visible P-waves → sinus tachycardia (stress, not crisis).
3. Select Patient 2. At logged BPM, waveform shows slightly widened QRS but with P-waves → borderline, but not V-tach.
4. Select Patient 3. At logged BPM (168 BPM), waveform shows wide QRS complexes, no P-waves, regular rhythm → V-tach. Crisis confirmed.
5. Patient 3's ID number = 3.

**Win Condition:** Patient 3 identified as V-tach. Patient ID = 3.

**Physics:**
- Normal ECG: narrow QRS, visible P-waves, rate 60-100 BPM
- Sinus tachycardia: narrow QRS, visible P-waves, rate > 100 BPM (stress response)
- V-tach: wide QRS (>120ms), no P-waves, rate > 150 BPM (life-threatening)
- The LifesignsDisplay morphs realistically based on rate — it doesn't just speed up, it changes shape

**Narrative:** Patient 3 is Lt. Kwon (the COMPUTER officer, roster position 3). Kwon had a V-tach episode during the gap — the atmospheric suppressant caused an adverse cardiac reaction. This is why Kwon's station (COMPUTER, position 3) was the first to show anomaly — Kwon collapsed, and the unmonitored COMPUTER station's alert went unacknowledged.

---

### R3-02 — Access Code

**Answer:** 7 (final clock position)

**Widgets:**
- `ModularClockDisplay` — a clock face with N positions (here N=8, positions 0-7), with a pointer that can be rotated
- `NumericStepper` — rotation amount (range: 0–7, step: 1)
- `LCARSButton` — APPLY ROTATION (momentary, rotates the pointer by the stepper amount)

**The Situation:**

A restricted door was accessed during the gap using the ship's modular key system. The key system uses modular arithmetic: a sequence of rotations on an 8-position clock. The access log shows the rotation sequence that was applied, but not the starting position. The solver applies the logged rotations in order to determine where the pointer ends up.

**The Work:**
1. The access log shows a sequence of rotations: +3, +5, +6, +1.
2. The ModularClockDisplay starts at position 0 (default for a reset lock).
3. Apply rotation +3: pointer moves to position 3. Press APPLY.
4. Apply rotation +5: pointer moves to (3+5) mod 8 = 0. Press APPLY.
5. Apply rotation +6: pointer moves to (0+6) mod 8 = 6. Press APPLY.
6. Apply rotation +1: pointer moves to (6+1) mod 8 = 7. Press APPLY.
7. Final position = 7.

**Win Condition:** After applying all 4 rotations, the ModularClockDisplay pointer rests at position 7.

**Physics (mathematical):**
- Modular arithmetic: (a + b) mod N
- The clock has 8 positions (0-7)
- Each rotation adds to the current position modulo 8
- The sequence is deterministic — given the starting position and rotation sequence, the final position is unique

**Narrative:** The rotation sequence +3, +5, +6, +1 sums to 15 ≡ 7 (mod 8). Position 7 on the duty roster is CHEN. The door accessed was the sensor bay anteroom — CHEN's access code was used (or borrowed) to enter.

---

### R3-03 — Permission Chain

**Answer:** 4 (element index)

**Widgets:**
- `CayleyTableDisplay` — a multiplication table for a small group (here: the group Z_8 under addition mod 8)
- `RotaryDial` — element selector (positions: 0, 1, 2, 3, 4, 5, 6, 7)
- `LCARSButton` — COMPOSE (momentary, composes the selected element with the running total)

**The Situation:**

The ship's permission system uses group composition. Permissions are represented as elements of a group. To perform a restricted action, a crew member must compose their base permission with one or more authorization tokens. The unauthorized action during the gap required composing specific permissions to reach the OVERRIDE result.

The CayleyTableDisplay shows the full 8×8 composition table. The solver must find which element, when composed with the known authorization sequence, produces the target result.

**The Work:**
1. The target result is OVERRIDE, which corresponds to element 6 in the group.
2. The known authorization chain: element 2 (base permission) composed with element X = element 6.
3. Look up in the Cayley table: 2 ∘ X = 6.
4. In Z_8 addition: 2 + X ≡ 6 (mod 8), so X = 4.
5. Select element 4 on the RotaryDial. Press COMPOSE.
6. The CayleyTableDisplay highlights the cell (2, 4) → 6. Confirmed.

**Win Condition:** Element 4 identified as the missing authorization token. The composition 2 ∘ 4 = 6 (OVERRIDE) is highlighted in the Cayley table.

**Narrative:** Element 4 corresponds to REEVES (roster position 4). Reeves provided the authorization token that, combined with the base permission (element 2, held by OKAFOR), produced the OVERRIDE permission. Reeves was the intermediary in the authorization chain.

---

### R3-04 — Badge Sequence

**Answer:** 5 (forced entry position)

**Widgets:**
- `IndicatorPanel` — 18 LEDs in a row, each colored: green (GRANTED), red (DENIED), amber (FORCED)
- `ScrollingTextDisplay` — log entries with timestamps, badge IDs, door names, and access results

**The Situation:**

18 badge swipes occurred during the gap. The IndicatorPanel shows the result of each swipe: GRANTED (green), DENIED (red), or FORCED (amber). FORCED entries indicate that security was overridden by command authority — the badge holder did not have standard clearance but forced entry anyway.

The ScrollingTextDisplay shows the log details for each swipe. The solver must read the forced entries (amber LEDs) in order and identify a pattern in their positions.

**The Work:**
1. The IndicatorPanel shows: G, G, D, G, F, G, G, D, G, G, F, G, D, G, F, G, G, G (where G=green, D=red, F=amber).
2. Forced entries (amber) appear at positions 5, 11, and 15.
3. The ScrollingTextDisplay for these entries shows:
   - Position 5: Badge 401, Sensor Bay Anteroom, FORCED at 0142
   - Position 11: Badge 401, Comm Array Junction, FORCED at 0238
   - Position 15: Badge 401, Computer Core Anteroom, FORCED at 0334
4. All three forced entries share badge 401. The first forced entry is at position **5**.
5. Badge 401 = Vasquez (CO). The first forced entry at position 5 places the first override at the Sensor Bay Anteroom at 0142 — 15 minutes into the gap.

**Win Condition:** The first FORCED entry identified at position 5 in the badge sequence. The solver confirms badge 401 = the CO.

**Narrative:** Position 5 in the badge sequence corresponds to roster position 5 — VASQUEZ. The first forced entry initiated the chain of unauthorized access. All forced entries share badge 401.

---

### R3-05 — Emergency Sequence

**Answer:** 12 (final switch state configuration)

**Widgets:**
- `BatSwitch` (guarded, ×4) — 4 protected toggle switches, each with a flip-up safety guard
- `AutoProcedureButton` — automation button (initially locked)
- `MasterAlarm` — large pulsing red alarm (activates on wrong sequence)
- `IndicatorPanel` — 4 LEDs showing each switch state (ON/OFF)

**The Situation:**

Someone ran an emergency procedure during the gap using 4 protected switches. The BatSwitches are guarded — the safety cover must be lifted before each switch can be flipped. The correct sequence must be followed; wrong order triggers the MasterAlarm (indicating the procedure was aborted in the real event and restarted).

The solver must reconstruct the correct sequence from clues in the procedure log.

**The Work:**
1. The procedure log (displayed on the panel) shows 4 entries with timestamps but the switch identifiers are redacted. The entries show: "Guard lifted", "Switch engaged", "System confirmed", "Guard closed" — repeated 4 times.
2. Clue: the procedure references "thermal before magnetic, magnetic before comm, sensor last."
3. The 4 switches are labeled: THERMAL (Switch A), MAGNETIC (Switch B), COMM (Switch C), SENSOR (Switch D).
4. Correct order: A, B, C, D (thermal → magnetic → comm → sensor).
5. Lift guard on A. Flip A. Indicator 1 lights. No alarm.
6. Lift guard on B. Flip B. Indicator 2 lights. No alarm.
7. Lift guard on C. Flip C. Indicator 3 lights. No alarm.
8. Lift guard on D. Flip D. Indicator 4 lights. No alarm.
9. All 4 indicators lit. The AutoProcedureButton unlocks (the solver has completed the manual procedure correctly).
10. The final switch configuration is encoded as a binary number: A=ON, B=ON, C=ON, D=OFF → 1100 = 12? No:

**Encoding:** The procedure register counts total operations performed during the emergency sequence. Each guard lift = 1 operation. Each switch toggle = 1 operation. The complete sequence:
1. Lift guard A (+1), flip A (+1) = 2 ops
2. Lift guard B (+1), flip B (+1) = 2 ops
3. Lift guard C (+1), flip C (+1) = 2 ops
4. Lift guard D (+1), flip D (+1) = 2 ops
5. Flip D back OFF (+1), close guard D (+1) = 2 ops (sensor re-secured)
6. Close guard A (+1), close guard B (+1) = 2 ops (cleanup)

Total: 12 operations. The procedure register readout = **12**.

**Win Condition:** Correct sequence executed without triggering MasterAlarm. Final switch state = 1100 binary = 12.

**Narrative:** The emergency procedure was the deflector array reconfiguration — thermal systems first (to handle the heat), magnetic containment second, comm relay third (to open the encrypted channel), sensor array last (to scan the contact). The sensor switch was returned to OFF after the scan completed. Configuration 12 (binary 1100) encodes the final state: thermal and magnetic still active, comm still active, sensor secured.

---

## ANSWER SUMMARY TABLE

| ID | Title | Primary Widget | Answer | Unit |
|----|-------|---------------|--------|------|
| R1-01 | Carrier Isolation | SineWaveDisplay | 2.340 | GHz |
| R1-02 | Phase Lock | LissajousDisplay | 873 | MHz |
| R1-03 | Null Zone | PhaseInterferenceDisplay | 213 | degrees |
| R1-04 | Orbit Classification | ConicSectionDisplay | 0.73 | eccentricity |
| R1-05 | Contact Lock | RadarSweepDisplay + TargetingReticleDisplay | 5 | count |
| R1-06 | Signal Fingerprint | CommSignalDisplay | 384 | kbps |
| R2-01 | Power Path | ConduitFlowDisplay | 2 | conduit count |
| R2-02 | Data Breach | NetworkGridDisplay | 7 | node ID |
| R2-03 | Heat Source | HeatMapDisplay | 14 | grid coord |
| R2-04 | Shield Profile | ShieldDisplay | 65 | power % |
| R2-05 | Fault Trace | CircuitTopologyDisplay | 9 | component ID |
| R2-06 | Reactor State | GaugeDisplay (×4) | 4 | throttle position |
| R3-01 | Triage | LifesignsDisplay | 3 | patient ID |
| R3-02 | Access Code | ModularClockDisplay | 7 | clock position |
| R3-03 | Permission Chain | CayleyTableDisplay | 4 | element index |
| R3-04 | Badge Sequence | IndicatorPanel | 5 | badge position |
| R3-05 | Emergency Sequence | BatSwitch (×4) | 12 | switch config |

---

## WIDGET USAGE SUMMARY

| Widget | Used In | Role |
|--------|---------|------|
| SineWaveDisplay | R1-01 | Combined signal + target overlay |
| LissajousDisplay | R1-02 | X-Y parametric frequency lock |
| PhaseInterferenceDisplay | R1-03 | Null zone placement heatmap |
| ConicSectionDisplay | R1-04 | Trajectory curve fitting |
| RadarSweepDisplay | R1-05 | Contact detection |
| TargetingReticleDisplay | R1-05 | Contact locking |
| CommSignalDisplay | R1-06 | IQ constellation + waveform |
| ConduitFlowDisplay | R2-01 | EPS routing visualization |
| NetworkGridDisplay | R2-02 | ODN topology + packet flow |
| HeatMapDisplay | R2-03 | Thermal diffusion isolation |
| ShieldDisplay | R2-04 | Multi-layer shield damage |
| CircuitTopologyDisplay | R2-05 | EPS circuit current flow |
| GaugeDisplay | R2-01, R2-03, R2-04, R2-06 | Meter readouts |
| LifesignsDisplay | R3-01 | ECG waveform morphology |
| ModularClockDisplay | R3-02 | Modular arithmetic rotation |
| CayleyTableDisplay | R3-03 | Group composition table |
| IndicatorPanel | R3-04, R3-05 | LED status array |
| ScrollingTextDisplay | R3-04 | Badge swipe log |
| BatSwitch | R3-05 | Guarded emergency switches |
| AutoProcedureButton | R3-05 | Earned automation control |
| MasterAlarm | R3-05 | Wrong-sequence feedback |
| NumericStepper | R1-01, R1-02, R1-04, R1-06, R3-02 | Precision value adjustment |
| RotaryDial | R1-01, R1-02, R1-04, R1-06, R3-01, R3-03 | Mode/step selection |
| LCARSButton | R1-01, R2-03, R2-05, R3-02, R3-03 | Action confirmation |
| ToggleSwitch | R1-03, R2-01, R2-02 | Enable/disable toggle |
| LinkedSliderInput | R1-03, R3-01 | Linked slider + numeric |
| PowerSlider | R2-04, R2-06 | Continuous range adjustment |
| ThrottleLever | R2-06 | Detent-notch vertical lever |
| LightedButton | R1-05 | Toggle with status LED |

**Total unique widgets used:** 21 (of ~28 in catalog). Every puzzle uses at least 2 widgets. Maximum is 4 (R1-01, R2-06, R3-05).

---

## NARRATIVE CONSISTENCY

All numeric answers are internally consistent with the incident:

- **R1 values** describe the contact: S-band carrier at 2.340 GHz, 3:1 harmonic lock at 873 MHz, bearing 213°, hyperbolic-capture trajectory (e=0.73), 5 real contacts, 384 kbps data rate.
- **R2 values** describe the ship's response: 2 active conduits (reduced power routing), ODN breach at node 7 (Comms), heat source at grid 14 (deflector), shield Layer 2 at 65%, component 9 failed (sensor relay), reactor throttle at 4 (moderate power).
- **R3 values** describe the personnel involved: Patient 3 = Kwon (V-tach), clock position 7 = Chen's access code, element 4 = Reeves' authorization token, badge position 5 = Vasquez's first forced entry, switch config 12 = deflector reconfiguration.
- **R1-03 null zone bearing (213°)** is consistent with the contact bearing established in the v1 meta (previously 147°, now updated to 213° for the v2 narrative — the contact approached from port quarter).
- **R1-05 contact count (5)** is consistent with the primary contact plus 4 companion objects — a small formation.
