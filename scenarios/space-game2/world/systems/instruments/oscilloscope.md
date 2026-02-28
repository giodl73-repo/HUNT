# Oscilloscope (X-Y Mode) — Instrument Reference

**Department:** COMMS / SCIENCE
**Location:** Bridge Science console, Comms station, Science Lab
**Instrument class:** Time-domain / Lissajous display

---

## Controls

| Control | Range | Step | Default |
|---------|-------|------|---------|
| CH1 Source | Internal / External | selector | Internal |
| CH2 Source | Internal / External | selector | External |
| Display Mode | Y-T / X-Y | toggle | Y-T |
| Timebase (Y-T mode) | 1 us - 1 s/div | decade | 1 ms/div |
| CH1 Scale | 10 mV - 10 V/div | decade | 1 V/div |
| CH2 Scale | 10 mV - 10 V/div | decade | 1 V/div |
| Persistence | OFF / 1s / 5s / INF | selector | OFF |

## Display

**Y-T mode:** Horizontal = time, Vertical = voltage. Standard oscilloscope view.

**X-Y mode:** CH1 drives horizontal axis. CH2 drives vertical axis. The trace draws the parametric curve (CH1(t), CH2(t)).

---

## Physics: Lissajous Figures

When two sinusoidal signals are applied in X-Y mode, the display shows a Lissajous figure. The figure's shape encodes the frequency ratio and phase relationship between the two signals.

### Frequency Ratio from Figure Shape

For signals x = A*sin(a*t + d) and y = B*sin(b*t):

```
Frequency ratio = a/b = (tangencies on vertical axis) / (tangencies on horizontal axis)
```

Count the number of times the figure touches its bounding box:
- Tangencies on the **vertical** axis (left and right edges) = numerator (f_CH1)
- Tangencies on the **horizontal** axis (top and bottom edges) = denominator (f_CH2)

### Phase Angle

For a 1:1 frequency ratio, the figure is an ellipse. The phase angle is:

```
phase = arcsin(2 * Y0 / Y_max)
```

Where Y0 is the Y-axis intercept (where the figure crosses the vertical center line) and Y_max is the maximum Y excursion.

- phase = 0: straight line, slope +1 (in phase)
- phase = 90: circle (quadrature)
- phase = 180: straight line, slope -1 (anti-phase)

### Drift Rate

If the frequencies are not exactly in ratio, the figure rotates. The rotation rate equals the beat frequency:

```
drift rate = |f1 - f2| (for near-1:1 ratio)
```

A stable figure means the frequencies are locked in exact ratio. A slowly rotating figure means they are close but not locked. A rapidly rotating figure means they are far from any simple ratio.

### Reference Table: Stable Figures

| Ratio (CH1:CH2) | Figure | Tangencies V | Tangencies H |
|-----------------|--------|-------------|-------------|
| 1:1 | Ellipse / circle / line | 1 | 1 |
| 2:1 | Figure-eight (horizontal) | 2 | 1 |
| 1:2 | Figure-eight (vertical) | 1 | 2 |
| 3:1 | Three loops horizontal | 3 | 1 |
| 3:2 | Pretzel (3 vertical, 2 horizontal tangencies) | 3 | 2 |
| 4:3 | Complex knot (4 vertical, 3 horizontal) | 4 | 3 |
| 5:3 | Dense knot (5 vertical, 3 horizontal) | 5 | 3 |

### Identifying Unknown Frequency

If CH1 frequency is known (f_known) and the figure is stable:

```
f_CH2 = f_known * (tangencies_horizontal / tangencies_vertical)
```

If the figure is drifting at rate R Hz near a p:q ratio:

```
f_CH2 = f_known * (q/p) +/- R
```

---

## Operating Procedure: Frequency Determination

1. Set CH1 = known reference signal. Set CH2 = unknown signal.
2. Switch to X-Y mode.
3. Count tangencies on each axis to determine frequency ratio.
4. If figure is drifting, measure drift rate. Calculate exact unknown frequency.
5. Read phase angle from Y-intercept if 1:1 ratio.

---

## Diagnostic Notes

- A line means in-phase or anti-phase. A circle means quadrature. Anything between is a tilted ellipse.
- If the figure changes shape over time, the frequency ratio is not stable. This indicates a free-running oscillator, not a locked source.
- Persistence ON with a drifting figure will fill the screen — this reveals the bounding box for tangency counting.
