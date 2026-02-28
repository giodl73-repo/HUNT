# Phased Array (Null Steering) — Instrument Reference

**Department:** TAC / COMMS
**Location:** Bridge Tactical console [2], Comms station
**Instrument class:** Antenna beam controller

---

## Controls

| Control | Range | Step | Default |
|---------|-------|------|---------|
| Beam Angle | 0 - 359 degrees | 1 degree | 0 |
| Null Angle | 0 - 359 degrees | 1 degree | OFF |
| Null Depth | -20 / -40 / -60 dB | selector | -40 dB |
| Signal Meter | dBm readout | — | live |
| SNR Display | dB readout | — | live |
| Scan Mode | MANUAL / AUTO-SWEEP | toggle | MANUAL |

## Display

Polar plot: 360-degree antenna pattern showing main beam (high gain) and null(s) (deep notch). Signal strength meter shows received power at current beam angle. SNR display shows signal-to-noise ratio.

---

## Physics

### Beam Steering

A phased array forms a directional beam by adjusting the phase of each element. The beam points in the direction of constructive interference. Signals arriving from the beam direction are amplified. Signals from other directions are attenuated.

### Null Steering

A null is a direction of destructive interference — the array deliberately cancels signals arriving from that direction. Null steering is used to suppress interference from a known direction so that a weaker target signal from a different direction can be received.

```
Place null at interference source -> interference is suppressed
-> noise floor drops in all other directions
-> target signal's SNR improves
```

### Finding the Interference Direction

Procedure: sweep the null angle from 0 to 359 degrees. At each angle, record the SNR for the target signal. The null angle that produces maximum target SNR is the interference direction — because that is where suppressing the interference helps the most.

```
Optimal null angle = bearing of interference source
Maximum SNR at that null angle = best achievable reception
```

### Finding the Target Bearing

Once the interference is nulled, the beam can be steered to maximize received signal power. The beam angle that produces maximum signal strength (with the null still active) is the target signal's bearing.

```
Target bearing = beam angle at maximum signal strength (with null active)
```

### Multiple Interference Sources

With a single null, only one interference source can be suppressed at a time. For N interference sources, N nulls are required. Each additional null costs beam gain (the main beam becomes slightly wider and weaker).

### SNR Calculation

```
SNR = signal_power (dBm) - noise_power (dBm)
```

Where noise_power includes both thermal noise and any unsuppressed interference. Deeper null = more interference suppression = higher SNR.

---

## Operating Procedure: Target Bearing Determination

1. Set Scan Mode = MANUAL.
2. Note current SNR with null OFF. This is baseline.
3. Sweep null angle in 10-degree steps. Record SNR at each step.
4. Identify the null angle with highest SNR. Refine in 1-degree steps around that peak.
5. The optimal null angle = interference bearing.
6. With null locked at optimal angle, sweep beam angle. Maximum signal strength = target bearing.
7. Record target bearing as the answer.

---

## Diagnostic Notes

- If SNR does not improve at any null angle, there is no directional interference — the noise is isotropic (thermal only).
- If two null angles produce similar SNR improvement, there are two interference sources. A single null cannot suppress both simultaneously.
- Null depth of -60 dB is strongest suppression but narrows the null. If interference source is moving, -40 dB is more robust.
- The difference between the null angle (interference) and the beam angle (target) gives the angular separation between interference and target.
