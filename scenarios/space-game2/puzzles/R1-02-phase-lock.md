# R1-02 — Phase Lock

## Panel Overview

| Field | Value |
|-------|-------|
| **Puzzle ID** | R1-02 |
| **Title** | Phase Lock |
| **Round** | 1 — TELEMETRY |
| **Department** | COMMS / SCIENCE |
| **Instrument** | Dual-channel oscilloscope in X-Y (Lissajous) mode |
| **Answer** | 873 MHz |

**Narrative beat:** The ship's outbound signal at 291 MHz was active during the gap. An incoming signal was detected on CH2. The oscilloscope was in X-Y mode — someone was comparing the two signals in real time. The Lissajous figure was drifting, which means the frequencies were close but not locked. The solver must determine the exact incoming frequency that stabilizes the figure, revealing the harmonic relationship between the ship's transmission and the contact's response.

**Win condition:** The LissajousDisplay shows a stable 3:1 figure (3 tangencies on the vertical axis, 1 on the horizontal). No rotation. The Y frequency NumericStepper reads 873 MHz.

---

## Widget Configuration

### Display

```
Widget: LissajousDisplay
Config:
  - xFrequency: 291 MHz (fixed — ship's outbound, driven by fixed readout)
  - yFrequency: (driven by NumericStepper, default 500 MHz)
  - phaseOffset: 0 degrees
  - trailLength: 200 points
  - trailFadeColor: #003322 -> #00FF88
  - dotColor: #FFFFFF (bright leading dot)
  - dotRadius: 4
  - speed: 3
  - gridColor: #1A2A2A
  - boundingBoxVisible: true (faint dashed box showing tangency edges)
  - tangencyMarkers: true (small tick marks on bounding box edges where
    the figure touches — visible only when figure is near-stable)
  - stabilityIndicator: bottom-right corner, shows "DRIFT: XX Hz" when
    figure rotates, shows "LOCKED" in green when drift < 0.1 MHz
```

### Controls

```
Widget: NumericStepper
ID: xFrequencyReadout
Config:
  - label: "CH1 (X) — SHIP OUTBOUND"
  - value: 291 MHz (FIXED — not adjustable, display-only)
  - unit: "MHz"
  - disabled: true
  - backgroundColor: #222233 (dimmed to show read-only)
```

```
Widget: NumericStepper
ID: yFrequency
Config:
  - label: "CH2 (Y) — INCOMING SIGNAL"
  - min: 100 MHz
  - max: 2000 MHz
  - step: (driven by RotaryDial)
  - default: 500 MHz
  - decimalPlaces: 1
  - unit: "MHz"
```

```
Widget: RotaryDial
ID: stepSizeSelector
Config:
  - label: "STEP SIZE"
  - positions: 3
  - positionLabels: ["10 MHz", "1 MHz", "0.1 MHz"]
  - positionValues: [10.0, 1.0, 0.1]
  - default: position 0 (10 MHz)
  - knobColor: #667788
  - pointerColor: #FFFFFF
```

### Layout

```
+--------------------------------------------------+
|  [ COMMS — PHASE COMPARISON ]                    |
|                                                  |
|  +--------------------------------------------+ |
|  |                                            | |
|  |         LissajousDisplay                   | |
|  |    (green trail with white leading dot)    | |
|  |    (faint bounding box with tangency ticks)| |
|  |                                            | |
|  |                          DRIFT: 209.0 Hz  | |
|  +--------------------------------------------+ |
|                                                  |
|  CH1 (X) — SHIP OUTBOUND     CH2 (Y) — INCOMING |
|  [ 291.0 MHz ] (fixed)       [< ] 500.0 MHz [>] |
|                                                  |
|  STEP SIZE                                       |
|  ( o )                                           |
|  10 / 1 / 0.1 MHz                               |
|                                                  |
|  ---- REFERENCE CARD (see below) ----           |
+--------------------------------------------------+
```

---

## Reference Card (Panel Legend)

```
COMMS — PHASE COMPARISON REFERENCE

WHAT THE DISPLAY SHOWS
  The oscilloscope is in X-Y mode. The ship's
  outbound signal (291 MHz) drives the horizontal
  axis. The incoming mystery signal drives the
  vertical axis. The resulting curve is called a
  Lissajous figure.

  A STABLE figure (not rotating) means the two
  frequencies are in an exact whole-number ratio.
  A DRIFTING figure (rotating slowly) means the
  frequencies are close to a ratio but not locked.

HOW TO READ THE FIGURE
  Count where the figure touches its bounding box:
    - Tangencies on the VERTICAL edges (left/right)
      = the X frequency's ratio number
    - Tangencies on the HORIZONTAL edges (top/bottom)
      = the Y frequency's ratio number

  Example: 3 tangencies vertical, 1 horizontal
           means Y frequency = 3 times X frequency.

  The tangency markers (tick marks on the box edges)
  appear when the figure is near-stable. Count them.

CONTROLS
  CH2 (Y) stepper: Adjusts the assumed Y frequency.
    As you approach the correct value, the figure
    slows its rotation and stabilizes.
  STEP SIZE (dial): 10 MHz for coarse search,
    1 MHz for approach, 0.1 MHz for final lock.

KEY PHYSICS FACT
  Stable Lissajous figures occur when
    f_Y / f_X = p / q
  where p and q are small whole numbers.
  The drift indicator reads "LOCKED" when the
  figure has fully stabilized.

HOW TO FIND THE UNKNOWN FREQUENCY
  1. Set STEP SIZE to 10 MHz. Sweep CH2 frequency.
  2. Watch the figure. When it slows its rotation,
     you are near a stable ratio.
  3. Switch to 1 MHz, then 0.1 MHz. Tune until
     the figure stops rotating and the drift
     indicator shows LOCKED.
  4. Count tangencies to confirm the ratio.
  5. The CH2 frequency at lock is the answer.

READING THE DISPLAY AT HIGH MISMATCH
  When CH2 is far from a harmonic, the figure
  rotates rapidly and may appear as a filled oval
  or a smeared region. This is normal — it means
  you are not near a stable ratio yet.
  As you approach the correct frequency, the
  rotation slows to a gentle drift and the figure
  resolves into a clear lobed shape. A slow drift
  means you are close. Keep fine-tuning.
```

---

## Novice Solve Path

1. Read the reference card. Understand: the figure rotates when frequencies are mismatched. The goal is to stop the rotation by finding the correct incoming frequency.
2. Start at 500 MHz, RotaryDial on 10 MHz. The figure is a fast-rotating mess — far from any simple ratio with 291 MHz.
3. Click upward: 510, 520, ..., 580 MHz. At 580 the figure rotates. At 590, still rotating. At 870, the figure slows noticeably. The drift indicator drops.
4. Switch to 1 MHz. At 873 MHz, the figure nearly stops. Drift reads small. Switch to 0.1 MHz.
5. At 873.0 MHz, the figure stabilizes completely. Drift indicator: LOCKED.
6. Count tangencies: 3 on the vertical edges, 1 on the horizontal. The ratio is 3:1.
7. Verify mentally: 3 x 291 = 873. Consistent.
8. The CH2 frequency at lock is 873 MHz.

**Time estimate:** 4-6 minutes. The coarse sweep from 500 to 870 is ~37 clicks at 10 MHz. Fine tuning is 3-5 clicks.

---

## Expert Solve Path

1. See X-Y mode, f_X = 291 MHz. Immediately look at the drifting figure. It has a 3-lobed structure suggesting 3:1 ratio.
2. Calculate: 3 x 291 = 873 MHz.
3. Set stepper to 870 MHz (a few 10 MHz clicks from 500, or type directly if numeric input allows). Switch to 1 MHz. Three clicks to 873.
4. Figure locks. Drift: LOCKED. Three vertical tangencies, one horizontal. Confirmed.
5. Read: 873 MHz. Done.

**Time estimate:** 15-30 seconds.

---

## Data Values

| Parameter | Value | Source/Justification |
|-----------|-------|---------------------|
| Ship outbound frequency (CH1/X) | 291 MHz | Chosen so that 3x = 873 MHz, a clean integer. Within UHF band range. |
| Incoming signal frequency (CH2/Y) | 873 MHz | META-DESIGN.md R1-02 answer. Exactly 3x outbound. |
| Frequency ratio at lock | 3:1 | META-DESIGN.md: "3:1 harmonic ratio at lock" |
| Vertical tangencies | 3 | Matches the ratio numerator per oscilloscope physics reference |
| Horizontal tangencies | 1 | Matches the ratio denominator per oscilloscope physics reference |
| Default Y frequency | 500 MHz | Forces a search — 500/291 is irrational, figure rotates rapidly |
| Y frequency range | 100-2000 MHz | Wide enough to contain 1:1 (291), 2:1 (582), 3:1 (873), etc. |
| Beat frequency at default | ~209 MHz | 500 - 291 = 209 MHz. Rapid rotation at start. |
| Lock tolerance | 0.1 MHz | Matches finest step size. Within 0.1 MHz, figure visually stabilizes. |

---

## Narrative Revelation

```
[COMMS] GAP+00:31:47 — Phase lock achieved. Incoming frequency: 873 MHz.
Ratio to outbound: 3:1 (third harmonic). Source is frequency-locked to
ship's own transmission. Classification: ACTIVE HARMONIC TRACKING.
```

---

## Story Layer

The incoming signal was not at a random frequency. It was at exactly three times the ship's own outbound transmission. A 3:1 harmonic lock means the contact was actively tracking the ship's frequency and responding at a precise integer multiple. Natural echoes return at 1:1. Subspace reflections introduce Doppler shift but maintain the fundamental. Only a device actively synthesizing from the received signal produces a clean harmonic lock at n:1 where n > 2. The contact was listening to the ship's outbound, multiplying the frequency, and transmitting back. This is the signature behavior of a Phase-Locked Echo — a contact that mirrors your own signal at a harmonic ratio to announce its presence without using an independent carrier. The implication: whatever was out there during the gap was aware of the ship and was responding deliberately.
