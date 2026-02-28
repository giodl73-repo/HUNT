# DEAD RECKONING v2 — Hints

**Status:** Skeleton. Placeholders only. Hints will be filled in during Stage 6 authoring when puzzle data is finalized.

Each puzzle has a 3-tier hint structure:
- **Nudge** — Points the solver toward the right approach without revealing mechanism
- **Direction** — Reveals the key physical concept or instrument feature to use
- **Near-solution** — Gives the specific procedure, stopping just short of the answer

---

## Round 1 — TELEMETRY

### R1-01: Carrier Acquisition (Spectrum Analyzer)

| Tier | Hint |
|------|------|
| Nudge | [TBD — suggest looking at how peaks behave differently across sweeps] |
| Direction | [TBD — explain carrier vs. noise behavior with RBW changes] |
| Near-solution | [TBD — identify the specific sweep and explain peak-hold reasoning] |

### R1-02: Phase Lock (Oscilloscope / Lissajous)

| Tier | Hint |
|------|------|
| Nudge | [TBD — suggest counting where the figure touches its bounding box] |
| Direction | [TBD — explain tangency counting for frequency ratio] |
| Near-solution | [TBD — give the ratio and explain how drift rate refines the exact frequency] |

### R1-03: Correlation Peak (Correlator)

| Tier | Hint |
|------|------|
| Nudge | [TBD — suggest paying attention to the sign of tau (positive vs. negative)] |
| Direction | [TBD — explain echo vs. precursor distinction] |
| Near-solution | [TBD — identify the dominant echo peak and walk through d = c*tau/2] |

### R1-04: Nulling (Phased Array)

| Tier | Hint |
|------|------|
| Nudge | [TBD — suggest focusing on which steering attempt produced the best SNR] |
| Direction | [TBD — explain that the optimal null angle IS the interference direction] |
| Near-solution | [TBD — identify the null angle and the corresponding target bearing] |

### R1-05: Spectral Decomposition (FFT Analyzer)

| Tier | Hint |
|------|------|
| Nudge | [TBD — suggest that the signal is there but the current settings can't show it] |
| Direction | [TBD — explain window function selection and averaging] |
| Near-solution | [TBD — give the correct FFT configuration and explain why it works] |

### R1-06: Vector Analysis (Vector Signal Analyzer)

| Tier | Hint |
|------|------|
| Nudge | [TBD — suggest the constellation is rotated from its standard orientation] |
| Direction | [TBD — explain modulation type identification from cluster count] |
| Near-solution | [TBD — give the modulation type and walk through data rate calculation] |

---

## Round 2 — SYSTEMS LOG

### R2-01: Power Audit (EPS Distribution Panel)

| Tier | Hint |
|------|------|
| Nudge | [TBD — suggest comparing stated system states to actual power readings] |
| Direction | [TBD — explain that OFFLINE systems should draw 0 MW; match draws to the EPS table] |
| Near-solution | [TBD — identify the phantom loads and their total] |

### R2-02: Data Trace (ODN Junction Board)

| Tier | Hint |
|------|------|
| Nudge | [TBD — suggest tracing which nodes can reach which through active junctions] |
| Direction | [TBD — explain ODN routing rules and which pathways are severed] |
| Near-solution | [TBD — walk through the surviving data pathway and calculate throughput] |

### R2-03: Pressure Map (Environmental Control Panel)

| Tier | Hint |
|------|------|
| Nudge | [TBD — suggest mapping pressure readings by physical location on the deck plan] |
| Direction | [TBD — explain radial pressure gradient vs. deliberate venting pattern] |
| Near-solution | [TBD — identify the breach room and calculate the differential] |

### R2-04: Thrust Vector (Propulsion Output Panel)

| Tier | Hint |
|------|------|
| Nudge | [TBD — suggest that the fuel consumption doesn't match a stationary ship] |
| Direction | [TBD — explain F=ma and displacement calculation from thrust data] |
| Near-solution | [TBD — provide the acceleration calculation and resulting displacement] |

### R2-05: Diagnostic Reconstruction (Master Diagnostics Terminal)

| Tier | Hint |
|------|------|
| Nudge | [TBD — suggest comparing the three checksum columns systematically] |
| Direction | [TBD — explain that before != during means the system changed during the gap] |
| Near-solution | [TBD — list which systems changed and give the count] |

### R2-06: Navigation Drift (Helm Console)

| Tier | Hint |
|------|------|
| Nudge | [TBD — suggest that one sensor array disagrees with the other two] |
| Direction | [TBD — explain systematic error vs. random error in sensor comparison] |
| Near-solution | [TBD — identify the drifting array and calculate the offset] |

---

## Round 3 — CREW RECORD

### R3-01: Vital Signs (Medical Scanner)

| Tier | Hint |
|------|------|
| Nudge | [TBD — suggest comparing each crew member's readings to their personal baselines] |
| Direction | [TBD — explain which vitals indicate stress vs. normal activity] |
| Near-solution | [TBD — identify the stress-positive crew and the peak BPM delta] |

### R3-02: Transport Log (Transporter Panel)

| Tier | Hint |
|------|------|
| Nudge | [TBD — suggest the residue levels are decaying over time from a single event] |
| Direction | [TBD — explain exponential decay and half-life back-calculation] |
| Near-solution | [TBD — walk through the decay curve and identify the transport time] |

### R3-03: Access Map (Security Console)

| Tier | Hint |
|------|------|
| Nudge | [TBD — suggest the ship layout constrains which paths are possible between badge swipes] |
| Direction | [TBD — explain that adjacent-room topology limits where someone could have been] |
| Near-solution | [TBD — trace the specific path and calculate the transit time] |

### R3-04: Comms Duration (Internal Comms Log)

| Tier | Hint |
|------|------|
| Nudge | [TBD — suggest looking for calls that don't match normal duty patterns] |
| Direction | [TBD — explain how to identify anomalous call clusters by channel and timing] |
| Near-solution | [TBD — identify the anomalous calls and sum their duration] |

### R3-05: Replicator Audit (Replicator Terminal)

| Tier | Hint |
|------|------|
| Nudge | [TBD — suggest cross-referencing item categories with required authorizations] |
| Direction | [TBD — explain the three red flags: wrong auth, wrong category, wrong power draw] |
| Near-solution | [TBD — identify all anomalous events and sum the power draw] |
