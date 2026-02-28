# Correlator — Instrument Reference

**Department:** COMMS / TAC
**Location:** Bridge Tactical console [2], Comms station
**Instrument class:** Cross-correlation processor

---

## Controls

| Control | Range | Step | Default |
|---------|-------|------|---------|
| Reference Signal | Outbound pulse / Stored template / Manual | selector | Outbound pulse |
| Lag Range | +/- 0.1 ms to +/- 100 ms | selector | +/- 10 ms |
| Lag Resolution | 0.001 / 0.01 / 0.1 ms | selector | 0.01 ms |
| Normalize | ON / OFF | toggle | ON |
| Threshold | 0.0 - 1.0 | 0.05 step | 0.50 |

## Display

Horizontal axis: lag (tau), in milliseconds. Negative left, positive right.
Vertical axis: correlation coefficient R_xy(tau). Range: -1.0 to +1.0 (normalized) or arbitrary units (unnormalized).

Peaks above the threshold are marked with tau value and amplitude.

---

## Physics

### Cross-Correlation

The cross-correlation function measures the similarity between signal x(t) and a time-shifted copy of signal y(t):

```
R_xy(tau) = integral[ x(t) * y(t + tau) ] dt
```

A peak in R_xy at lag tau means the two signals are most similar when y is shifted by tau relative to x.

### Interpreting Lag

| Lag | Meaning |
|-----|---------|
| tau = 0 | Signals are simultaneous. No delay. |
| tau > 0 | Signal y is delayed relative to x. This is an **echo** or **response**: x was transmitted, y arrived tau later. |
| tau < 0 | Signal y precedes x. This is a **precursor**: y existed before x was sent. Physically impossible for a response. Indicates the signal was anticipated or originated independently. |

### Distance from Delay

For a signal that travels at speed c and reflects back:

```
d = c * tau / 2     (round-trip)
d = c * tau          (one-way, if no reflection)
```

Where c = 3 x 10^8 m/s (speed of light in vacuum). For subspace signals, propagation speed varies by band.

### Multiple Peaks

Multiple correlation peaks indicate:

- **Multipath**: the same signal arriving via different paths (direct + reflected off a hull, nebula, etc.). Each path has a different delay.
- **Composite signal**: multiple distinct signals superimposed. Each has its own delay relative to the reference.
- **Harmonics**: correlation at integer multiples of the fundamental lag.

### Peak Characteristics

| Peak property | Meaning |
|--------------|---------|
| Height (amplitude) | Degree of similarity. 1.0 = perfect match. 0.5 = moderate. < threshold = noise. |
| Width | Bandwidth of the correlated component. Narrow peak = narrowband signal. Wide peak = broadband. |
| Polarity | Positive peak = in-phase match. Negative peak = anti-phase match (180-degree inversion). |

---

## Operating Procedure: Echo Detection

1. Set Reference = Outbound pulse (the ship's own transmitted signal).
2. Set Lag Range to expected round-trip time for the distance of interest.
3. Run correlation.
4. Identify peaks above threshold.
5. Positive-tau peaks are echoes. Calculate distance: d = c * tau / 2.
6. Negative-tau peaks are precursors. Flag for investigation.

---

## Diagnostic Notes

- A single peak at tau > 0 with amplitude > 0.8 is a clean echo. Distance is reliable.
- Multiple positive peaks at regular intervals suggest multipath reflection.
- A negative-tau peak above threshold is anomalous. The responding signal existed before the outbound was sent. This rules out echo/response and implies an independent source or a signal that anticipated the outbound.
- If no peaks above threshold appear, the input signal is uncorrelated with the reference. Either the signals are unrelated or the lag range is too narrow.
