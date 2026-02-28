# DEAD RECKONING v2 — Hints

Each puzzle has a 3-tier hint structure:
- **Nudge** — Points toward the right control to use first
- **Direction** — Explains what physical state to achieve and why
- **Near-solution** — Gives the target value range or the exact control state

---

## Round 1 — TELEMETRY

### R1-01: Carrier Isolation (SineWaveDisplay)

| Tier | Hint |
|------|------|
| Nudge | Start with the RotaryDial on the coarsest step size. Sweep the NumericStepper across the full range and watch for any region where the filtered waveform begins to resemble the target overlay. |
| Direction | A carrier is a single-frequency signal. When your bandpass filter center matches the carrier frequency, the filtered output will be a clean sinusoid matching the target. The two interference components are at different frequencies and will be rejected. Narrow the step size as you get closer. |
| Near-solution | The carrier is in the S-band, between 2.300 and 2.400 GHz. Switch to the finest step size (1 Hz) and tune within that range. The target overlay aligns at exactly 2.340 GHz. |

### R1-02: Phase Lock (LissajousDisplay)

| Tier | Hint |
|------|------|
| Nudge | Count where the Lissajous figure touches its bounding box. The number of tangencies on the vertical edges vs. horizontal edges tells you the frequency ratio. |
| Direction | The figure has 3 tangencies on the vertical axis and 1 on the horizontal. This means f_Y / f_X = 3/1. Since f_X is 291 MHz, the target f_Y = 3 times 291. Tune the Y NumericStepper toward that product. |
| Near-solution | The Y frequency at lock is 873 MHz (exactly 3 times 291 MHz). Start coarse at 870 MHz, then fine-tune. When the LissajousDisplay stops rotating and shows a stable 3-lobed figure, you have phase lock. |

### R1-03: Null Zone (PhaseInterferenceDisplay)

| Tier | Hint |
|------|------|
| Nudge | The dark regions on the heatmap are null zones — places where the two sources cancel each other. Drag the sources to move the null zones. You need a null at the interference direction, not at the target direction. |
| Direction | Turn on symmetry lock first. Then adjust the source frequencies so the null pattern spacing matches the angular separation between the interference source and the target. When the null falls on the interference, the target signal brightens. |
| Near-solution | The target signal is at bearing 213 degrees. Position Source 1 so the null falls on the interference source (opposite side of the display from 213 degrees). When the null suppresses the interference, a bright spot emerges at 213 degrees. The Source 1 bearing readout will confirm 213. |

### R1-04: Orbit Classification (ConicSectionDisplay)

| Tier | Hint |
|------|------|
| Nudge | Start with the RotaryDial on coarse step (0.10). Sweep eccentricity from 0 upward and watch how the theoretical curve shape changes relative to the observed trajectory overlay. |
| Direction | The observed trajectory is an open arc that curves but does not close into an ellipse. This means e is between 0.5 and 1.0 — a highly eccentric ellipse or near-parabolic trajectory. Focus your search in that range. The curve should pass through all the observed position dots simultaneously. |
| Near-solution | Switch to fine step (0.01) around e = 0.70. The curve matches all observed positions at e = 0.73. You may also need to adjust the semi-latus rectum to get the scale right — try values around 40-60 km. |

### R1-05: Contact Lock (RadarSweepDisplay + TargetingReticleDisplay)

| Tier | Hint |
|------|------|
| Nudge | Use the TargetingReticleDisplay to lock onto each contact one at a time. Watch the lock indicator — green means the contact holds steady. Flickering means it is a decoy. |
| Direction | Real contacts maintain consistent bearing and range under targeting lock. Decoys either drift in bearing, fade in signal strength, or cause the lock indicator to flicker. Mark each contact as REAL (green button) or DECOY (red button) after testing it. You are looking for a count. |
| Near-solution | Of the 8 contacts, exactly 5 hold under lock and 3 scatter. The real contacts are the primary object plus four companion objects. Count = 5. |

### R1-06: Signal Fingerprint (CommSignalDisplay)

| Tier | Hint |
|------|------|
| Nudge | Count the number of distinct cluster regions in the IQ constellation. The cluster count identifies the modulation type: 2 = BPSK, 4 = QPSK, 8 = 8-PSK, 16 = 16-QAM. |
| Direction | The constellation shows 8 clusters arranged in a ring (not a grid). This is 8-PSK, which encodes 3 bits per symbol. To find the data rate, you need the symbol rate too. Adjust the NumericStepper until the waveform overlay shows clean transitions — the rate will be a round number. |
| Near-solution | Modulation is 8-PSK (3 bits/symbol). Symbol rate is 128 kSym/s (the waveform sharpens at this value). Data rate = 128 times 3 = 384 kbps. |

---

## Round 2 — SYSTEMS LOG

### R2-01: Power Path (ConduitFlowDisplay)

| Tier | Hint |
|------|------|
| Nudge | Toggle each conduit on and off while watching all 3 destination gauges. Not all conduits can be active simultaneously — the junction overloads. |
| Direction | You need all 3 destination gauges in the green zone at the same time. Try every combination of enabled/disabled conduits (there are only 8 possibilities for 3 switches). One combination works — it uses exactly 2 active conduits. |
| Near-solution | Enable EPS PRIMARY and EPS SECONDARY. Disable DATA-ODN. All 3 gauges go green. The active conduit count is 2. |

### R2-02: Data Breach (NetworkGridDisplay)

| Tier | Hint |
|------|------|
| Nudge | Watch for the red data packet circulating through the network. Toggle nodes OFF one at a time and observe where the red packet stops — it cannot pass through a disabled node. |
| Direction | Use process of elimination. Disable a node. If the red packet still circulates, that node is not on its route. If the packet stops, the disabled node was on the route. Work from the network edges inward to find where the packet enters the network. |
| Near-solution | The breach entry point is node 7 — the Security Computer node. When you disable node 7, the red packet never enters the network at all. All other nodes, when disabled individually, only stop the packet mid-route. |

### R2-03: Heat Source (HeatMapDisplay)

| Tier | Hint |
|------|------|
| Nudge | The warm area on the heatmap has spread out from a hidden source due to thermal diffusion. Place coolant zones at the edges of the warm area to absorb the diffused heat and sharpen the gradient. |
| Direction | Work from the outside in. Each coolant zone you place absorbs nearby heat and shrinks the warm area. After 4-5 coolant zones placed at the periphery, the remaining hot spot reveals the source location. Watch the GaugeDisplay for peak temperature — it will concentrate at the source. |
| Near-solution | The heat source is at grid coordinate 14. Place coolant at coordinates 8, 10, 18, and 20 (surrounding the source). The warm area collapses to a single hot point at 14. |

### R2-04: Shield Profile (ShieldDisplay)

| Tier | Hint |
|------|------|
| Nudge | The ShieldDisplay shows a reference damage pattern from the gap's log. Adjust the 3 PowerSliders until the simulated damage distribution matches the reference overlay. Each layer's power level determines how much damage it absorbs. |
| Direction | A weaker layer absorbs more damage (it fails faster, taking the brunt of the attack). Layer 1 shows the most damage in the reference pattern, so it was set to the lowest power. Layer 3 shows the least damage, so it was strongest. Layer 2 is in between. Adjust Layer 2 until its damage indicator matches the reference. |
| Near-solution | Layer 1 at 30%, Layer 2 at 65%, Layer 3 at 90%. The damage pattern matches the log at these exact values. Layer 2's reading of 65% is the answer. |

### R2-05: Fault Trace (CircuitTopologyDisplay)

| Tier | Hint |
|------|------|
| Nudge | Probe nodes from the source end of the circuit first. Green flash = current is flowing. Red flash = no current. The fault is where current stops. |
| Direction | Use binary search on the circuit topology. Start by probing the source (should be green) and the destination (should be red). Then probe midway nodes. But beware — the circuit branches. You need to follow the actual circuit paths, not just node numbers in order. |
| Near-solution | The fault is at component 9. Nodes upstream of 9 show green (current flowing). Nodes downstream show red (no current). The branch that feeds the sensor recording system passes through component 9 — its failure cut sensor logging. |

### R2-06: Reactor State (GaugeDisplay + ThrottleLever)

| Tier | Hint |
|------|------|
| Nudge | Each gauge has a setpoint marker showing the logged value from the gap. Move the ThrottleLever to different detent positions and watch how all 4 gauges respond. One detent position brings the gauges close to their setpoints. |
| Direction | The ThrottleLever controls fuel flow rate, which drives temperature, pressure, coolant flow, and power output simultaneously. At the correct detent position, all 4 gauges will be near their setpoints. Fine-tune with the PowerSlider (containment field) to match exactly. |
| Near-solution | Throttle position 4 brings all gauges close. Set containment to 72% to match exactly. The throttle detent snaps at position 4 — that is the answer. |

---

## Round 3 — CREW RECORD

### R3-01: Triage (LifesignsDisplay)

| Tier | Hint |
|------|------|
| Nudge | Use the RotaryDial to select each patient and observe their ECG waveform at their logged heart rate. You are looking for one specific waveform pattern: V-tach — wide, regular QRS complexes without P-waves. |
| Direction | Normal stress produces sinus tachycardia — the waveform speeds up but keeps its normal shape (narrow QRS, visible P-waves). V-tach is different: the QRS complexes become wide and blobby, and the P-waves disappear entirely. Compare each patient's waveform to these descriptions. Only one patient matches V-tach. |
| Near-solution | Patient 3 shows V-tach at 168 BPM — wide QRS complexes, no P-waves, regular rhythm. Patients 1 and 2 show sinus tachycardia (stressed but not in crisis). The answer is patient ID 3. |

### R3-02: Access Code (ModularClockDisplay)

| Tier | Hint |
|------|------|
| Nudge | The access log shows a sequence of rotation amounts. Apply them one at a time to the ModularClockDisplay using the NumericStepper and APPLY button. The clock has 8 positions (0-7) and wraps around. |
| Direction | Each rotation adds to the current position modulo 8. Start at position 0. Apply +3 to get to 3. Apply +5 to get to (3+5) mod 8 = 0. Apply +6 to get to 6. Apply +1 to get to 7. Track the running total carefully — the modular wrap is easy to miss. |
| Near-solution | The rotation sequence is +3, +5, +6, +1. Running total: 0 → 3 → 0 → 6 → 7. The final clock position is 7. |

### R3-03: Permission Chain (CayleyTableDisplay)

| Tier | Hint |
|------|------|
| Nudge | The Cayley table shows all compositions. The OVERRIDE result is element 6. You know the base permission is element 2. Find which element X satisfies 2 composed with X = 6. |
| Direction | In the Cayley table, find row 2. Scan across the columns until you find the cell containing 6. The column header of that cell is the missing element X. This is modular addition: 2 + X = 6 (mod 8). |
| Near-solution | Row 2, scanning across: column 0 gives 2, column 1 gives 3, column 2 gives 4, column 3 gives 5, column 4 gives 6. The answer is element 4. |

### R3-04: Badge Sequence (IndicatorPanel)

| Tier | Hint |
|------|------|
| Nudge | Look at the IndicatorPanel for amber LEDs — those are the FORCED entries, where someone overrode security. The position number of the first amber LED in the sequence is significant. |
| Direction | FORCED entries (amber) represent command-authority overrides of normal security. Read the ScrollingTextDisplay for each amber entry to see the badge ID and door. All forced entries share the same badge number — that is the person who forced their way in. The position of the first forced entry in the 18-swipe sequence encodes the answer. |
| Near-solution | Amber LEDs appear at positions 5, 11, and 15. All share badge 401. The first FORCED entry is at position 5. The answer is 5. |

### R3-05: Emergency Sequence (BatSwitch)

| Tier | Hint |
|------|------|
| Nudge | The 4 guarded switches must be flipped in a specific order. The procedure log has clues about the correct sequence. Lift the guard first, then flip. Wrong order triggers the MasterAlarm. |
| Direction | The procedure log says "thermal before magnetic, magnetic before comm, sensor last." This means the order is: THERMAL (A), MAGNETIC (B), COMM (C), SENSOR (D). Flip them A-B-C-D. After completing the sequence, note that the log shows SENSOR was toggled back OFF at the end. |
| Near-solution | Correct sequence: A, B, C, D. After completing the sequence, the sensor switch (D) is toggled back OFF and its guard closed, then guards A and B are closed. Count every operation: 4 guard lifts + 4 flips + 1 re-toggle + 1 guard close + 2 cleanup closes = 12 total operations. The procedure register reads 12. |

---

## Meta Hints

### R1-META: CONTACT

| Tier | Hint |
|------|------|
| Nudge | Your 6 measurement values describe a single contact. The Contact Signature Reference Table has rows for different contact types. Map each value to its column and find the row that matches all 6. |
| Direction | Look at the frequency band first: 2.340 GHz is S-band. That eliminates most rows. Then check the harmonic ratio (3:1 means n>2). Only one contact type has both S-band AND n>2 harmonic ratio: Phase-Locked Echo. Verify all 6 columns match. |
| Near-solution | Phase-Locked Echo matches all 6 values. Its class code is 8. Enter 8 as the number of points on the commission decoder. |

### R2-META: RESPONSE

| Tier | Hint |
|------|------|
| Nudge | The 6 system values tell you about the ship's response. The meta asks which station issued the first alert. Cross-reference each system with its controlling bridge station. |
| Direction | The first alert during a contact event comes from the station whose system showed anomaly first. Look at your 6 values and the station assignments. The station at roster position 3 was first. |
| Near-solution | The starting position for the commission decoder is 3. |

### R3-META: COVER-UP

| Tier | Hint |
|------|------|
| Nudge | Your 5 crew record values trace an authorization chain. How many distinct links does it take to get from the detecting officer to the commanding officer? |
| Direction | The authorization chain goes: detection at station 3 (Kwon) → authorization provided by element 4 (Reeves) → orders from badge position 5 (Vasquez). The chain has 2 links. |
| Near-solution | The step size for the commission decoder is 2. |

### FINAL META: THE COMMISSION

| Tier | Hint |
|------|------|
| Nudge | The CyclicGroupDisplay has 3 inputs: number of points, starting position, and step size. You have all 3 from the round metas. Configure the circle and read where the pointer lands. |
| Direction | N = 8 points (from R1-META class code). Start at position 3 (from R2-META first alert station). Step by 2 (from R3-META chain length). Calculate: (3 + 2) mod 8 = 5. Read the label at position 5. |
| Near-solution | Position 5 on the circle reads VASQUEZ. Captain Vasquez ordered the cover-up. |
