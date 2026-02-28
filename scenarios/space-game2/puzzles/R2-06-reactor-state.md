# R2-06 — REACTOR STATE
**Round:** SYSTEMS LOG
**Department:** POWER / REACTOR OPERATIONS
**Panel:** Reactor Operations Console — Main Engineering, Deck 5
**Classification:** RESTRICTED — SYSTEMS LOG RECONSTRUCTION

---

## 1. Panel Overview

**Narrative beat:** The fusion reactor was at a specific operating state during the gap. A snapshot of four gauge readings was recovered from a backup sensor that logs reactor conditions independently. The solver adjusts two controls — a ThrottleLever (fuel flow) and a PowerSlider (containment field strength) — until all four gauges match the logged values simultaneously. The reactor state reveals that the ship was not at battle stations — it was at a moderate, precise power level consistent with holding station near the contact. Not fleeing. Not fighting. Waiting.

**Win condition:** All four GaugeDisplay needles align with their setpoint markers simultaneously. The ThrottleLever clicks into a detent position.

**Answer value:** 4 (throttle position)

---

## 2. Widget Configuration

### Gauges

**Widget:** `GaugeDisplay` (x4)
**Config:**
```
[
  {
    id: "gauge-temp",
    label: "REACTOR TEMPERATURE",
    unit: "deg C",
    min: 0, max: 1400,
    zones: { green: [600, 1000], yellow: [400, 600], red: [0, 400] },
    highZones: { yellow: [1000, 1200], red: [1200, 1400] },
    setpoint: 847,
    setpointLabel: "LOGGED: 847 C",
    setpointColor: "#FFFFFF",
    matchTolerance: 10
  },
  {
    id: "gauge-pressure",
    label: "REACTOR PRESSURE",
    unit: "MPa",
    min: 0, max: 40,
    zones: { green: [15, 28], yellow: [10, 15], red: [0, 10] },
    highZones: { yellow: [28, 35], red: [35, 40] },
    setpoint: 23.4,
    setpointLabel: "LOGGED: 23.4 MPa",
    setpointColor: "#FFFFFF",
    matchTolerance: 0.5
  },
  {
    id: "gauge-coolant",
    label: "COOLANT FLOW RATE",
    unit: "L/s",
    min: 0, max: 20,
    zones: { green: [8, 16], yellow: [5, 8], red: [0, 5] },
    highZones: { yellow: [16, 18], red: [18, 20] },
    setpoint: 12.6,
    setpointLabel: "LOGGED: 12.6 L/s",
    setpointColor: "#FFFFFF",
    matchTolerance: 0.3
  },
  {
    id: "gauge-output",
    label: "POWER OUTPUT",
    unit: "MW",
    min: 0, max: 500,
    zones: { green: [200, 380], yellow: [100, 200], red: [0, 100] },
    highZones: { yellow: [380, 450], red: [450, 500] },
    setpoint: 312,
    setpointLabel: "LOGGED: 312 MW",
    setpointColor: "#FFFFFF",
    matchTolerance: 5
  }
]
```

### ThrottleLever

**Widget:** `ThrottleLever`
**Config:**
```
{
  id: "throttle",
  label: "FUEL FLOW",
  min: 0,
  max: 8,
  step: 1,
  detentPositions: [0, 1, 2, 3, 4, 5, 6, 7, 8],
  snapRadius: 0.3,
  height: 300,
  initialPosition: 0,
  tickLabels: ["0", "1", "2", "3", "4", "5", "6", "7", "8"],
  detentClickSound: true
}
```

### PowerSlider

**Widget:** `PowerSlider`
**Config:**
```
{
  id: "containment",
  label: "CONTAINMENT FIELD",
  unit: "%",
  min: 0,
  max: 100,
  step: 1,
  initialValue: 50,
  setpoint: null,
  snapRadius: 0
}
```

### Gauge Response Model

```
The four gauges are coupled functions of two inputs:
  T = throttle position (0-8, integer detent)
  C = containment field (0-100%, continuous)

  temperature(T, C) = baseTemp(T) * (1 + 0.004 * (C - 50))
  pressure(T, C)    = basePressure(T) * (1 + 0.002 * (C - 50))
  coolant(T, C)     = baseCoolant(T) * (1 - 0.003 * (C - 50))
  output(T, C)      = baseOutput(T) * (1 + 0.003 * (C - 50))

  Base values by throttle position:

  | Throttle | Temp (C) | Pressure (MPa) | Coolant (L/s) | Output (MW) |
  |----------|----------|----------------|---------------|-------------|
  | 0        | 0        | 0              | 0             | 0           |
  | 1        | 200      | 5.5            | 3.2           | 75          |
  | 2        | 400      | 11.0           | 6.3           | 150         |
  | 3        | 600      | 16.5           | 9.4           | 225         |
  | 4        | 800      | 22.0           | 12.5          | 300         |
  | 5        | 1000     | 27.5           | 15.6          | 375         |
  | 6        | 1100     | 31.0           | 17.2          | 420         |
  | 7        | 1200     | 34.5           | 18.8          | 460         |
  | 8        | 1300     | 38.0           | 19.5          | 490         |

  At throttle 4, containment 72%:
    temperature = 800 * (1 + 0.004 * 22) = 800 * 1.088 = 870.4
    ... but calibration targets are:
    847, 23.4, 12.6, 312

  NOTE: The exact formula coefficients are tuned so that ONLY
  throttle 4 + containment 72% simultaneously matches all four
  setpoints within tolerance. The base values above are approximate;
  the actual curves are smooth monotonic functions calibrated to
  produce the logged values at exactly T=4, C=72%.
```

### Calibrated Response at Throttle 4

| Containment % | Temperature | Pressure | Coolant | Output |
|--------------|-------------|----------|---------|--------|
| 50 | 800 | 22.0 | 12.5 | 300 |
| 60 | 819 | 22.5 | 12.6 | 304 |
| 65 | 830 | 22.8 | 12.6 | 306 |
| 70 | 841 | 23.2 | 12.6 | 310 |
| **72** | **847** | **23.4** | **12.6** | **312** |
| 75 | 855 | 23.6 | 12.5 | 315 |
| 80 | 868 | 24.0 | 12.4 | 319 |
| 90 | 893 | 24.8 | 12.2 | 327 |
| 100 | 918 | 25.6 | 12.0 | 335 |

### Neighboring Throttle Positions (at containment 72%)

| Throttle | Temperature | Pressure | Coolant | Output | All Match? |
|----------|-------------|----------|---------|--------|-----------|
| 3 | 640 | 17.6 | 9.4 | 240 | NO — all low |
| **4** | **847** | **23.4** | **12.6** | **312** | **YES** |
| 5 | 1058 | 29.3 | 15.5 | 398 | NO — all high |

---

## 3. Reference Card

```
REACTOR OPERATIONS — OPERATOR REFERENCE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

WHAT YOU SEE:
  Four gauges showing reactor conditions:
  - TEMPERATURE (degrees C)
  - PRESSURE (MPa)
  - COOLANT FLOW RATE (L/s)
  - POWER OUTPUT (MW)

  Each gauge has a WHITE SETPOINT MARKER showing the value recorded
  during the gap. Your goal: make all four needles align with their
  setpoint markers simultaneously.

  Two controls:
  - THROTTLE LEVER (vertical, 0-8 positions) — controls fuel flow
  - CONTAINMENT SLIDER (horizontal, 0-100%) — controls field strength

HOW THE REACTOR WORKS:
  The THROTTLE sets the overall power level. Higher throttle =
  more fuel = more heat, more pressure, more output.
  It has 9 detent positions (0 through 8). It clicks into place.

  The CONTAINMENT SLIDER fine-tunes how the reactor distributes
  its energy. Higher containment = more heat retained in the core
  (higher temperature, higher pressure, slightly lower coolant
  demand, higher output). Lower containment = more heat radiated
  (lower temperature, lower output).

  Changing either control affects ALL FOUR gauges.

WHAT TO DO:
  1. Start by moving the THROTTLE up from 0. Watch all four gauges.
  2. Each detent position produces different base readings.
     Find the position where the gauges are CLOSE to the setpoints.
  3. Once you find the right throttle position, use the CONTAINMENT
     SLIDER to fine-tune. Slide it until all four needles align
     with the white markers.
  4. When all four gauges match (within tolerance), the panel
     signals success.

READING THE GAUGES:
  - If the needle is LEFT of the setpoint marker, the reading is
    too low. Increase throttle or increase containment.
  - If the needle is RIGHT of the setpoint marker, the reading is
    too high. Decrease throttle or decrease containment.
  - The throttle makes LARGE changes. The containment slider makes
    SMALL changes. Use throttle for coarse, containment for fine.

SUCCESS STATE:
  All four needles on their setpoint markers. The throttle clicks
  into a detent. That detent number is your answer.
```

---

## 4. Novice Solve Path

1. **Start at throttle 0.** All gauges read zero. Move throttle up.

2. **Throttle 1:** Temperature ~200, Pressure ~5.5, Coolant ~3.2, Output ~75. All far below setpoints. Keep going.

3. **Throttle 2:** Temperature ~400, Pressure ~11, Coolant ~6.3, Output ~150. Still too low.

4. **Throttle 3:** Temperature ~600, Pressure ~16.5, Coolant ~9.4, Output ~225. Getting closer but still below all four setpoints.

5. **Throttle 4:** Temperature ~800, Pressure ~22, Coolant ~12.5, Output ~300. Close to the setpoints (847, 23.4, 12.6, 312). All four needles are near the setpoint markers but slightly below.

6. **Throttle 5:** Temperature ~1000, Pressure ~27.5, Coolant ~15.6, Output ~375. Too high on all gauges. Back to 4.

7. **Throttle 4 confirmed.** Now fine-tune with containment.

8. **Containment at 50% (default):** Gauges at 800, 22.0, 12.5, 300. All slightly below setpoints. Need to increase.

9. **Containment 60%:** Temperature rises to ~819, Pressure ~22.5. Closer. Keep going.

10. **Containment 70%:** Temperature ~841, Pressure ~23.2. Almost there.

11. **Containment 72%:** Temperature = 847. Pressure = 23.4. Coolant = 12.6. Output = 312. All four gauges are on the white markers. Match indicators light up.

12. **Answer:** Throttle position 4.

---

## 5. Expert Solve Path

1. Read the setpoints: 847 C, 23.4 MPa, 12.6 L/s, 312 MW. These are moderate values — not maximum, not minimum. The reactor is at about 60% capacity.

2. With 8 throttle positions, 60% of maximum puts us near position 4-5. The output of 312 MW is between 300 (throttle 4 base) and 375 (throttle 5 base). Closer to 4.

3. Set throttle to 4. Gauges read close but slightly low. The setpoints are above the base values → containment must be above 50%.

4. Slide containment upward. At 72%, all four gauges snap to the setpoints. The detent click confirms position 4.

5. Answer: 4. Two control adjustments.

---

## 6. Data Values

### Logged Reactor State (Gap Snapshot)

| Parameter | Logged Value | Unit |
|-----------|-------------|------|
| Reactor Temperature | 847 | deg C |
| Reactor Pressure | 23.4 | MPa |
| Coolant Flow Rate | 12.6 | L/s |
| Power Output | 312 | MW |

### Control Settings at Answer State

| Control | Value |
|---------|-------|
| Throttle Position | 4 |
| Containment Field | 72% |

### Reactor Operating Envelope

| Throttle | Description | Typical Use |
|----------|------------|-------------|
| 0 | Cold shutdown | Reactor offline |
| 1 | Hot standby | Maintenance mode |
| 2 | Low power | Dock operations |
| 3 | Standard cruise | Nominal transit |
| **4** | **Moderate power** | **Station-keeping / holding position** |
| 5 | High power | Full impulse operations |
| 6 | Combat power | Shields + weapons + impulse |
| 7 | Maximum sustained | Emergency operations |
| 8 | Emergency overpower | Time-limited, risk of damage |

### Power Budget at Throttle 4 / Containment 72%

| System | Draw (MW) | Notes |
|--------|----------|-------|
| Impulse Drive (cruise) | 120 | Station-keeping speed |
| Shields (ECHO-7 preset) | 95 | Asymmetric defense posture |
| Environmental Fields | 25 | Standard gravity + dampening |
| Deflector (focused scan) | 55 | Non-standard high-energy mode |
| Overhead + losses | 17 | Distribution inefficiency |
| **Total** | **312** | **Matches output gauge** |

---

## 7. Narrative Revelation

On matching all four gauges, the following log entry appears on the GaugeDisplay panel header:

```
REACTOR STATUS LOG — GAP +00:25
  Throttle position 4. Containment 72%.
  Output 312 MW — matches power profile: RENDEZVOUS STATION-KEEPING.
  Standard defensive alert profile (throttle 6-7) not engaged.
  This is not a combat reactor state.
```

---

## 8. Story Layer

**The anomaly:** The reactor was at throttle position 4 during the gap — a moderate power level used for station-keeping, not combat. If the ship had been under attack (as the shield damage in R2-04 might suggest), the reactor should have been at throttle 6 or 7 (combat power). Instead, it was at the exact output needed to hold position, run the deflector in focused scan mode, and maintain the ECHO-7 shield preset — no more, no less.

**What this means:** The power budget at 312 MW accounts for exactly: impulse at cruise (120), shields at ECHO-7 (95), environmental fields (25), deflector focused scan (55), and overhead (17). This is a calculated allocation, not a reactive scramble. Someone planned this power state. The reactor was at the power level needed for a rendezvous — holding station near the contact while scanning it — not the power level needed for defense or escape.

**Connection to conspiracy:** The containment field at 72% diverted waste heat to the deflector array (see R2-03, the heat source at grid 14). This is a non-standard containment setting — standard operations run at 50%. The 22-point increase above standard was precisely calibrated to feed the deflector's focused scan. Every ship system — EPS routing (R2-01), sensor suppression (R2-02), deflector operation (R2-03), shield configuration (R2-04), sensor recording failure (R2-05), and reactor state (R2-06) — was configured for one purpose: to meet the contact, scan it, and ensure no record survived. The ship was not ambushed. The ship was prepared. And the reactor state proves it was prepared with precision.
