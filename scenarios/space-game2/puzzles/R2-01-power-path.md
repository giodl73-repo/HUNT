# R2-01 — POWER PATH
**Round:** SYSTEMS LOG
**Department:** POWER / EPS DISTRIBUTION
**Panel:** EPS Conduit Routing Console — Main Engineering, Deck 5
**Classification:** RESTRICTED — SYSTEMS LOG RECONSTRUCTION

---

## 1. Panel Overview

**Narrative beat:** During the six-hour gap, the ship's EPS routing was reconfigured. Three conduits feed power through a shared junction to three critical destination systems. The junction cannot handle all three conduits simultaneously — it overloads. Someone during the gap found the configuration that keeps all three destinations running. The solver recreates that configuration.

**Win condition:** All three destination system gauges read in the green zone simultaneously. No junction overload warning.

**Answer value:** 2 (the number of active conduits in the working configuration)

---

## 2. Widget Configuration

### Primary Display

**Widget:** `ConduitFlowDisplay`
**Config:**
```
{
  conduitCount: 3,
  conduits: [
    { id: "EPS-PRI",  label: "EPS PRIMARY",   color: "#FF8C00", enabled: true },
    { id: "EPS-SEC",  label: "EPS SECONDARY",  color: "#4488FF", enabled: true },
    { id: "DATA-ODN", label: "DATA-ODN",        color: "#00CCCC", enabled: true }
  ],
  junctionCapacity: 2,
  junctionOverloadFlash: true,
  junctionOverloadColor: "#FF0000",
  junctionLabel: "JCT-5A MAIN BUS",
  destinations: [
    { id: "SHIELDS",      label: "SHIELDS" },
    { id: "IMPULSE",      label: "IMPULSE DRIVE" },
    { id: "LIFESUPPORT",  label: "LIFE SUPPORT" }
  ],
  particleSpeed: 2.0,
  showJunctionLoadBar: true
}
```

### Input Controls

**Widget:** `ToggleSwitch` (x3)
```
[
  { id: "sw-pri",  label: "EPS PRIMARY",   linkedConduit: "EPS-PRI",  initialState: true },
  { id: "sw-sec",  label: "EPS SECONDARY",  linkedConduit: "EPS-SEC",  initialState: true },
  { id: "sw-odn",  label: "DATA-ODN",        linkedConduit: "DATA-ODN", initialState: true }
]
```

### Destination Gauges

**Widget:** `GaugeDisplay` (x3)
```
[
  { id: "gauge-shields",    label: "SHIELDS",       unit: "MW", min: 0, max: 120,
    zones: { red: [0, 30], yellow: [30, 60], green: [60, 120] },
    sourceMap: { "EPS-PRI": 45, "EPS-SEC": 50, "DATA-ODN": 10 } },
  { id: "gauge-impulse",    label: "IMPULSE DRIVE", unit: "MW", min: 0, max: 180,
    zones: { red: [0, 50], yellow: [50, 100], green: [100, 180] },
    sourceMap: { "EPS-PRI": 60, "EPS-SEC": 55, "DATA-ODN": 5 } },
  { id: "gauge-lifesupport", label: "LIFE SUPPORT", unit: "MW", min: 0, max: 40,
    zones: { red: [0, 10], yellow: [10, 20], green: [20, 40] },
    sourceMap: { "EPS-PRI": 10, "EPS-SEC": 12, "DATA-ODN": 18 } }
]
```

### Junction Logic

```
Junction capacity: 2 conduits maximum.
If enabledConduits > 2:
  - Junction overload warning (red flash on JCT-5A label)
  - Lowest-priority conduit (DATA-ODN) power contribution zeroed
  - LIFE SUPPORT gauge drops to red (loses DATA-ODN's 18 MW contribution)
  - Junction load bar pulses red

If enabledConduits <= 2:
  - Junction nominal
  - Each enabled conduit contributes its sourceMap value to each destination gauge
  - Gauge needle animates to sum of contributions from active conduits
```

---

## 3. Reference Card

```
EPS CONDUIT ROUTING — OPERATOR REFERENCE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

WHAT YOU SEE:
  Three horizontal conduits feed power through a central junction
  to three destination systems. Colored particles flow through
  enabled conduits. Static conduits are disabled.

  Three gauges on the right show power delivery to each destination.
  Green zone = system operational. Yellow = degraded. Red = failed.

THE PROBLEM:
  Junction JCT-5A can handle at most 2 active conduits simultaneously.
  With all 3 enabled, the junction overloads and one destination
  loses power.

WHAT TO DO:
  1. Use the three toggle switches to enable/disable conduits.
  2. Watch all three gauges. Your goal: ALL THREE in the green zone.
  3. If the junction flashes red, you have too many conduits active.
     Disable one.
  4. Try each combination. Only one puts all three gauges in green.

READING THE GAUGES:
  Each conduit contributes different amounts to each destination.
  EPS PRIMARY feeds mostly Impulse and Shields.
  EPS SECONDARY feeds mostly Shields and Impulse.
  DATA-ODN feeds mostly Life Support.
  But the combination that works is not obvious — you must test.

SUCCESS STATE:
  All three gauge needles in green. Junction load nominal (no red flash).
  Count the active conduits. That count is your answer.
```

---

## 4. Novice Solve Path

1. **Initial state:** All three conduits enabled. Junction overload warning flashes red. Life Support gauge drops to red (junction is shedding the lowest-priority conduit's contribution). Shields and Impulse read in green.

2. **Observation:** The junction cannot handle 3 conduits. The solver notices the red flash and the Life Support gauge in red.

3. **First attempt — disable EPS PRIMARY:**
   - Active: EPS SECONDARY + DATA-ODN (2 conduits, junction OK)
   - Gauges: Shields = 50 + 10 = 60 MW (borderline green). Impulse = 55 + 5 = 60 MW (yellow — below 100 MW threshold). Life Support = 12 + 18 = 30 MW (green).
   - Result: Impulse in yellow. Not all green.

4. **Second attempt — disable EPS SECONDARY instead:**
   - Active: EPS PRIMARY + DATA-ODN (2 conduits, junction OK)
   - Gauges: Shields = 45 + 10 = 55 MW (yellow). Impulse = 60 + 5 = 65 MW (yellow). Life Support = 10 + 18 = 28 MW (green).
   - Result: Shields and Impulse both in yellow. Worse.

5. **Third attempt — disable DATA-ODN:**
   - Active: EPS PRIMARY + EPS SECONDARY (2 conduits, junction OK)
   - Gauges: Shields = 45 + 50 = 95 MW (green). Impulse = 60 + 55 = 115 MW (green). Life Support = 10 + 12 = 22 MW (green).
   - Result: All three gauges in green. Junction nominal.

6. **Answer:** 2 active conduits (EPS PRIMARY + EPS SECONDARY).

---

## 5. Expert Solve Path

1. Recognize that the junction capacity is 2. There are only C(3,2) = 3 possible two-conduit combinations, plus 3 single-conduit states and the zero state. Only need to check the three two-conduit combos.

2. Read the source contribution values from the gauge labels or infer from initial gauge positions:
   - PRIMARY feeds Impulse (60) and Shields (45) heavily, Life Support (10) lightly.
   - SECONDARY feeds Shields (50) and Impulse (55) heavily, Life Support (12) lightly.
   - DATA-ODN feeds Life Support (18) heavily, others lightly.

3. PRIMARY + SECONDARY: Shields = 95 (green), Impulse = 115 (green), Life Support = 22 (green, just above 20 threshold). Only this combination clears all three green thresholds.

4. Confirm with one toggle. Answer: 2.

---

## 6. Data Values

### Conduit Power Contributions (MW per destination)

| Conduit | Shields | Impulse | Life Support |
|---------|---------|---------|-------------|
| EPS PRIMARY | 45 | 60 | 10 |
| EPS SECONDARY | 50 | 55 | 12 |
| DATA-ODN | 10 | 5 | 18 |

### Gauge Green Thresholds

| Destination | Green Zone Min | Green Zone Max |
|-------------|---------------|----------------|
| Shields | 60 MW | 120 MW |
| Impulse Drive | 100 MW | 180 MW |
| Life Support | 20 MW | 40 MW |

### All Two-Conduit Combinations

| Active Conduits | Shields | Impulse | Life Support | All Green? |
|----------------|---------|---------|-------------|-----------|
| PRI + SEC | 95 | 115 | 22 | YES |
| PRI + ODN | 55 | 65 | 28 | NO (Shields yellow, Impulse yellow) |
| SEC + ODN | 60 | 60 | 30 | NO (Impulse yellow) |

### Junction Constraint

- Maximum simultaneous conduits: 2
- At 3 conduits: junction overloads, DATA-ODN contribution zeroed (lowest priority)
- At 1 conduit: insufficient power for at least 2 destinations
- At 0 conduits: all gauges at zero

---

## 7. Narrative Revelation

On achieving the win condition, the following log entry appears on the ConduitFlowDisplay status line:

```
EPS ROUTING LOG — GAP +00:22
  JCT-5A reconfigured. DATA-ODN conduit disabled.
  EPS PRIMARY + EPS SECONDARY active. All destinations nominal.
  Excess ODN bandwidth rerouted to Lab 7 (Deck 4, Frame 12).
  Lab 7 current manifest status: DECOMMISSIONED.
```

---

## 8. Story Layer

**The anomaly:** The DATA-ODN conduit was not disabled to protect the junction. It was disabled so its bandwidth could be rerouted elsewhere — to Lab 7, a compartment listed as DECOMMISSIONED in the ship's manifest. Someone needed power in a place that officially does not exist.

**What this means:** The EPS reconfiguration during the gap was not an emergency response to conduit damage. It was a deliberate reroute. Whoever made this change knew the junction limitation and used it as cover — "we had to disable DATA-ODN because of the overload" — while the real purpose was to power an unauthorized location.

**Connection to conspiracy:** Lab 7 is where the deflector array was reconfigured for the focused scan of the contact (see R2-03 Heat Source, R3-05 Emergency Sequence). The power had to come from somewhere. The DATA-ODN conduit was sacrificed to provide it. The operator who made this change knew what was in Lab 7 and what it needed power for.
