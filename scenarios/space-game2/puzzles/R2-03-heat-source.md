# R2-03 — HEAT SOURCE
**Round:** SYSTEMS LOG
**Department:** ENVIRO / THERMAL MANAGEMENT
**Panel:** Deck Thermal Monitoring Console — Environmental Control, Deck 5
**Classification:** RESTRICTED — SYSTEMS LOG RECONSTRUCTION

---

## 1. Panel Overview

**Narrative beat:** An anomalous heat source appeared on the ship during the gap. The thermal grid shows diffused heat spread across a 5x5 deck section — the source has been radiating long enough for the heat to blur outward. The solver places coolant zones to absorb peripheral heat and collapse the gradient, revealing the hidden source location as the last remaining hot spot.

**Win condition:** Peak temperature isolated to a single grid coordinate. The GaugeDisplay peak temperature indicator points to one cell with no ambiguity.

**Answer value:** 14 (grid coordinate of the heat source)

---

## 2. Widget Configuration

### Primary Display

**Widget:** `HeatMapDisplay`
**Config:**
```
{
  gridWidth: 5,
  gridHeight: 5,
  cellSize: 80,
  cellLabels: true,
  labelFormat: "coordinate",
  coordinateMap: [
    [ 1,  2,  3,  4,  5],
    [ 6,  7,  8,  9, 10],
    [11, 12, 13, 14, 15],
    [16, 17, 18, 19, 20],
    [21, 22, 23, 24, 25]
  ],
  colorScale: {
    cold:  "#1a237e",
    cool:  "#2196f3",
    warm:  "#ffeb3b",
    hot:   "#ff9800",
    peak:  "#f44336"
  },
  diffusionRate: 0.15,
  decayRate: 0.02,
  heatSources: [
    { coordinate: 14, output: 100, fixed: true, hidden: true }
  ],
  coolantZones: [],
  maxCoolantZones: 6,
  coolantAbsorption: 40,
  showCellTemperatures: true,
  temperatureUnit: "C"
}
```

### Initial Thermal State (Post-Diffusion)

The heat source at coordinate 14 has been radiating for the equivalent of several hours. Diffusion has spread the heat outward. Initial temperatures (degrees C above ambient) when the solver first sees the panel:

```
┌──────┬──────┬──────┬──────┬──────┐
│  1   │  2   │  3   │  4   │  5   │
│ +2.1 │ +3.4 │ +4.8 │ +6.2 │ +3.1 │
├──────┼──────┼──────┼──────┼──────┤
│  6   │  7   │  8   │  9   │ 10   │
│ +3.8 │ +6.5 │ +9.1 │+12.7 │ +7.4 │
├──────┼──────┼──────┼──────┼──────┤
│ 11   │ 12   │ 13   │ 14   │ 15   │
│ +5.2 │ +8.8 │+14.3 │+18.6 │+11.9 │
├──────┼──────┼──────┼──────┼──────┤
│ 16   │ 17   │ 18   │ 19   │ 20   │
│ +3.5 │ +6.1 │ +9.4 │+12.2 │ +7.1 │
├──────┼──────┼──────┼──────┼──────┤
│ 21   │ 22   │ 23   │ 24   │ 25   │
│ +1.8 │ +3.0 │ +4.5 │ +5.8 │ +2.9 │
└──────┴──────┴──────┴──────┴──────┘
```

Note: The hottest cell is 14 (+18.6), but cells 13 (+14.3), 9 (+12.7), and 19 (+12.2) are also warm. The gradient is broad. Without coolant intervention, the source location is ambiguous — it could plausibly be any of the cells in the 13-14-9-19 cluster.

### Coolant Controls

**Widget:** `LCARSButton` (x2)
```
[
  { id: "btn-place",  label: "PLACE COOLANT",  mode: "momentary",
    behavior: "Player clicks a grid cell, then presses PLACE COOLANT to deploy a coolant zone at that cell." },
  { id: "btn-remove", label: "REMOVE COOLANT", mode: "momentary",
    behavior: "Player clicks an existing coolant zone, then presses REMOVE COOLANT to retract it." }
]
```

### Peak Temperature Gauge

**Widget:** `GaugeDisplay`
```
{
  id: "gauge-peak",
  label: "PEAK TEMPERATURE",
  unit: "deg C above ambient",
  min: 0,
  max: 25,
  zones: {
    green:  [0, 5],
    yellow: [5, 12],
    red:    [12, 25]
  },
  source: "max(gridTemperatures)",
  showCoordinate: true,
  coordinateLabel: "PEAK AT: [coord]"
}
```

### Coolant Placement Logic

```
When a coolant zone is placed at coordinate C:
  - Cell C temperature drops by coolantAbsorption (40 units effective)
  - Cell C turns blue (coolant marker visible)
  - Adjacent cells (orthogonal) lose 15 units of effective heat
  - Diagonal cells lose 8 units of effective heat
  - The HeatMapDisplay recalculates diffusion equilibrium
  - Peak temperature may shift to a different coordinate

When a coolant zone is removed:
  - Heat from the source re-diffuses into the uncooled region
  - Temperature gradient relaxes back toward the diffused state
  - The peak may spread again

Maximum 6 coolant zones at once. The solver can reposition them.
```

---

## 3. Reference Card

```
DECK THERMAL MONITOR — OPERATOR REFERENCE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

WHAT YOU SEE:
  A 5x5 grid showing thermal readings across a deck section.
  Colors range from blue (cool) to red (hot). Each cell shows
  its grid coordinate number and temperature above ambient.

  A broad warm region covers the center-right of the grid.
  The heat has DIFFUSED outward from a single source, spreading
  over time. The source location is hidden within the gradient.

THE PROBLEM:
  Find the exact grid coordinate of the heat source.
  The diffused gradient is too broad to identify the source
  by inspection alone.

WHAT TO DO:
  1. Click a grid cell, then press PLACE COOLANT.
     Coolant absorbs heat from that cell and its neighbors.
  2. Place coolant at the EDGES of the warm area first.
     This removes diffused heat without affecting the source.
  3. As peripheral heat is absorbed, the warm area shrinks.
     The source remains hot because it is actively generating heat.
  4. After 4-5 coolant placements, only the source cell will
     remain warm. All other cells will have been cooled.
  5. Read the GaugeDisplay — it shows the peak temperature and
     the coordinate of the peak. When only the source remains,
     that coordinate is your answer.

KEY INSIGHT:
  Coolant absorbs DIFFUSED heat (heat that has spread from the
  source). The source ITSELF continues generating heat and cannot
  be fully cooled by a single coolant zone. The source cell stays
  warm even when surrounded by coolant.

  Place coolant at coordinates AWAY from the suspected source.
  Do not waste coolant directly on the source — you want to
  REVEAL it, not cool it.

SUCCESS STATE:
  Peak temperature gauge shows a single coordinate as the hottest
  point. The warm area has collapsed to one cell. That cell's
  coordinate number is the answer.
```

---

## 4. Novice Solve Path

1. **Initial observation:** The heatmap shows a broad warm region. Multiple cells are warm. The gauge reads "PEAK AT: 14" but cells 13, 9, and 19 are nearly as hot. The solver might wonder if 14 is already the answer, but the instructions say to isolate the source, and the gradient is ambiguous.

2. **First coolant — outer ring:** Place coolant at coordinate 5 (upper right corner, +3.1). The corner cools. Nearby cells 4 and 10 drop slightly. Not much change in the center.

3. **Second coolant — opposite edge:** Place coolant at coordinate 21 (lower left, +1.8). The lower-left corner cools. Still broad warm center.

4. **Third coolant — north of warm center:** Place coolant at coordinate 8 (row 2 center, +9.1). This absorbs significant heat from the upper part of the gradient. Cells 3, 7, 8, 9, 13 drop noticeably. The warm region tightens.

5. **Fourth coolant — south of warm center:** Place coolant at coordinate 18 (row 4 center, +9.4). Cells 17, 18, 19, 23, 13 drop. The warm area collapses further. Now only cells 14 and 9 remain significantly above ambient.

6. **Fifth coolant — east edge:** Place coolant at coordinate 10 (right edge, +7.4). Cells 9, 10, 15, 5 drop. Cell 9 is now cooled. The warm spot is clearly at coordinate 14.

7. **Gauge confirmation:** PEAK AT: 14. Temperature at 14 is still elevated (+16.2 with nearby coolant absorbing some diffusion, but the source keeps generating). Cell 13 has dropped to +4.1. Cell 9 has dropped to +2.8. Cell 19 has dropped to +3.3. The isolation is clear.

8. **Answer:** 14.

---

## 5. Expert Solve Path

1. Read the initial temperature grid. Note the asymmetry: the gradient is not centered on the grid. The hottest region skews toward column 4 (coordinates 4, 9, 14, 19, 24). Within that column, the peak is clearly row 3 (coordinate 14 at +18.6).

2. An expert recognizes that in a diffusion field from a point source, the source is at the maximum of the temperature distribution. Coordinate 14 is already the peak. But the puzzle requires isolation — the gradient must collapse to confirm.

3. Place coolant at two strategic positions:
   - Coordinate 8 (absorbs heat from the north half of the gradient)
   - Coordinate 20 (absorbs heat from the south-east, the second-warmest quadrant)

4. The gradient collapses. Only coordinate 14 remains elevated. Gauge confirms: PEAK AT: 14.

5. Answer: 14. Two coolant placements, done.

---

## 6. Data Values

### Heat Source Parameters

| Parameter | Value |
|-----------|-------|
| Source coordinate | 14 |
| Source output | 100 (arbitrary thermal units) |
| Source type | Fixed, continuous, hidden |
| Diffusion rate | 0.15 per timestep |
| Decay rate | 0.02 per timestep |

### Initial Temperature Grid (deg C above ambient)

| | Col 1 | Col 2 | Col 3 | Col 4 | Col 5 |
|---|-------|-------|-------|-------|-------|
| **Row 1** | 2.1 | 3.4 | 4.8 | 6.2 | 3.1 |
| **Row 2** | 3.8 | 6.5 | 9.1 | 12.7 | 7.4 |
| **Row 3** | 5.2 | 8.8 | 14.3 | **18.6** | 11.9 |
| **Row 4** | 3.5 | 6.1 | 9.4 | 12.2 | 7.1 |
| **Row 5** | 1.8 | 3.0 | 4.5 | 5.8 | 2.9 |

### Temperature Profile (Column 4, all rows — maximum gradient axis)

| Coordinate | Temperature | Distance from Source |
|-----------|------------|---------------------|
| 4 | +6.2 | 2 cells |
| 9 | +12.7 | 1 cell |
| **14** | **+18.6** | **0 (source)** |
| 19 | +12.2 | 1 cell |
| 24 | +5.8 | 2 cells |

The profile follows an approximate Gaussian centered on coordinate 14. The slight asymmetry (9 at 12.7 vs 19 at 12.2) reflects the grid boundary effects — heat diffuses more freely upward (toward the larger connected deck space) than downward (toward a bulkhead boundary).

### Coolant Zone Placement Guide

| Strategy | Placements | Coolant Used | Isolation Quality |
|----------|-----------|-------------|-------------------|
| Surround source | 8, 9, 13, 15, 18, 19 | 6 | Overkill — wastes coolant on source-adjacent cells |
| Edge-first | 5, 21, 8, 18, 10 | 5 | Clean — removes peripheral heat first |
| Expert minimum | 8, 20 | 2 | Sufficient — collapses gradient to single peak |
| Random scatter | varies | varies | May work with 4-6 placements depending on positions |

### Deck Section Reference

```
DECK 5 — SECTION FRAME 10-14 THERMAL GRID
  Coordinate 14 = Deflector Control anteroom
  Coordinate 13 = Deflector Control main
  Coordinate 9  = EPS Control junction
  Coordinate 19 = Environmental Control access
```

---

## 7. Narrative Revelation

On isolating the heat source to coordinate 14, the following log entry appears on the HeatMapDisplay status overlay:

```
THERMAL ANOMALY LOG — GAP +01:08
  Source isolated: Grid 14 — Deflector Control anteroom, Deck 5.
  No scheduled activity in this section during gap window.
  Thermal profile consistent with deflector array in FOCUSED SCAN mode.
  Deflector mode log: [PURGED]
```

---

## 8. Story Layer

**The anomaly:** Grid coordinate 14 corresponds to the Deflector Control anteroom on Deck 5. The ship's schedule shows no activity in this section during the gap — the deflector was supposed to be in standard navigational mode, which generates minimal heat. The thermal signature matches the deflector array operating in focused scan mode — a high-energy directed beam used for deep-space surveys. This mode generates significant waste heat at the anteroom junction where the power conduits feed the array.

**What this means:** Someone activated the deflector in focused scan mode during the gap without logging it. The deflector mode log was purged (consistent with the broader log deletion). But heat cannot be purged. The thermal diffusion grid is a passive system — it records what it measures, and it was never designed to be tampered with. The heat signature is physical evidence that the deflector was pointed at something.

**Connection to conspiracy:** The focused scan was aimed at the contact — the Phase-Locked Echo detected at bearing 213 degrees (R1-03). The deflector was reconfigured as part of the emergency procedure (R3-05). The power for this scan came from the DATA-ODN conduit that was disabled in the EPS reroute (R2-01). Three separate ship systems — EPS, thermal grid, deflector — tell the same story: someone pointed the most powerful sensor on the ship at the contact, and then tried to erase every trace of having done so.
