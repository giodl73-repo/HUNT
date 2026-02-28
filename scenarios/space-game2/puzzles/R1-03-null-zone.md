# R1-03 — Null Zone

## Panel Overview

| Field | Value |
|-------|-------|
| **Puzzle ID** | R1-03 |
| **Title** | Null Zone |
| **Round** | 1 — TELEMETRY |
| **Department** | TAC / COMMS |
| **Instrument** | Phased array jamming display (dual-source interference) |
| **Answer** | 213 degrees |

**Narrative beat:** The ship's jamming array was activated during the gap. Two coherent wave sources were used to create a destructive interference zone — a null — at a specific bearing, suppressing interference from that direction so a target signal from a different bearing could be received cleanly. The array configuration was purged. The solver must reconstruct the null placement that reveals the hidden signal's bearing.

**Win condition:** The PhaseInterferenceDisplay shows a dark null zone suppressing the interference source, and a bright spot emerges at bearing 213 degrees. The target bearing readout confirms 213.

---

## Widget Configuration

### Display

```
Widget: PhaseInterferenceDisplay
Config:
  - gridSize: 400 x 400 px
  - backgroundColor: #000011
  - constructiveColor: #FFCC00 (bright = constructive)
  - destructiveColor: #000022 (dark = destructive)
  - sourceCount: 2 (draggable)
  - source1Label: "alpha"
  - source2Label: "beta"
  - source1DefaultPos: (200, 300) — bottom center
  - source2DefaultPos: (200, 100) — top center
  - symmetryLockAvailable: true
  - bearingOverlay: true (compass ring around the heatmap edge, 0-359 degrees)
  - targetSignalDot: hidden until null is correctly placed
  - targetSignalBearing: 213 degrees (fixed — revealed when interference suppressed)
  - targetSignalColor: #FF3333 (red bright dot at bearing 213 on the edge)
  - interferenceSourceBearing: 33 degrees (the direction to null)
  - interferenceSourcePower: strong (dominates the display until nulled)
  - bearingReadout: bottom of display, shows "TARGET BEARING: ---" until
    target emerges, then "TARGET BEARING: 213 deg"
```

### Controls

```
Widget: LinkedSliderInput
ID: source1Frequency
Config:
  - label: "SOURCE alpha FREQ"
  - min: 100 MHz
  - max: 1000 MHz
  - step: 1 MHz
  - default: 400 MHz
  - unit: "MHz"
```

```
Widget: LinkedSliderInput
ID: source2Frequency
Config:
  - label: "SOURCE beta FREQ"
  - min: 100 MHz
  - max: 1000 MHz
  - step: 1 MHz
  - default: 400 MHz
  - unit: "MHz"
```

```
Widget: ToggleSwitch
ID: symmetryLock
Config:
  - label: "SYMMETRY LOCK"
  - initialState: OFF
  - onColor: #00CC66
  - behavior: when ON, Source beta mirrors Source alpha about the
    center axis. Moving alpha causes beta to move symmetrically.
    Frequency of beta is locked to match alpha.
```

### Layout

```
+--------------------------------------------------+
|  [ TAC — JAMMING ARRAY ]                        |
|                                                  |
|  +--------------------------------------------+ |
|  |              0 deg                         | |
|  |           . . . . .                        | |
|  |       270 .       . 90                     | |
|  |           .  MAP  .                        | |
|  |           . . . . .                        | |
|  |             180                            | |
|  |     (drag alpha/beta to position sources)  | |
|  |                                            | |
|  |     TARGET BEARING: ---                    | |
|  +--------------------------------------------+ |
|                                                  |
|  SOURCE alpha FREQ   SOURCE beta FREQ            |
|  [====O=====] 400    [====O=====] 400            |
|                                                  |
|  SYMMETRY LOCK  [ OFF ]                          |
|                                                  |
|  ---- REFERENCE CARD (see below) ----           |
+--------------------------------------------------+
```

---

## Reference Card (Panel Legend)

```
TAC — JAMMING ARRAY REFERENCE

WHAT THE DISPLAY SHOWS
  The heatmap shows an interference pattern from
  two wave sources (alpha and beta). Bright regions
  are constructive interference (signals add).
  Dark regions are destructive interference — these
  are NULL ZONES where signals cancel out.

  A compass ring around the edge shows bearings
  (0-359 degrees). A target signal is arriving from
  an unknown bearing. An interference source is
  arriving from a different bearing. The target
  signal is hidden behind the interference.

HOW NULLING WORKS
  Place a null zone at the bearing of the
  interference source. This suppresses the
  interference. The target signal, arriving from a
  DIFFERENT bearing, is not affected by the null.
  With the interference gone, the target emerges as
  a bright dot on the compass ring.

CONTROLS
  Drag sources: Click and drag alpha and beta on
    the heatmap. This moves the null zones.
  Source frequencies: Adjust the slider to change
    the wavelength. This changes the spacing between
    null zones.
  Symmetry lock: When ON, beta mirrors alpha. This
    simplifies positioning — only move alpha.

HOW TO FIND THE TARGET BEARING
  1. Turn SYMMETRY LOCK on.
  2. Drag Source alpha slowly around the display.
     Watch the heatmap pattern shift.
  3. When a null zone falls on the interference
     source direction, the interference is
     suppressed. A bright dot will appear at the
     target signal's bearing.
  4. Adjust source frequency if the null zone is
     too wide or too narrow to suppress the
     interference cleanly.
  5. Read the TARGET BEARING from the readout.

KEY PHYSICS FACT
  Two coherent sources create alternating bright
  and dark bands. The position of the dark bands
  (nulls) depends on source positions and frequency.
  Nulls fall where the path difference from the two
  sources equals half a wavelength. Moving the
  sources moves the nulls.
```

---

## Novice Solve Path

1. Read the reference card. Understand: drag sources to move null zones. Goal is to suppress interference so the target appears.
2. Turn SYMMETRY LOCK on. Now only need to move Source alpha — beta follows.
3. The default position shows a vertical interference pattern. No target visible. The bearing readout shows "---".
4. Drag Source alpha clockwise around the display. The interference pattern rotates. Watch the compass ring for a bright dot appearing.
5. As alpha moves toward a position that places a null at bearing 33 degrees (the interference source direction), the interference fades. A bright red dot begins to appear at bearing 213 degrees.
6. Fine-tune the source position. The target dot brightens as the null deepens on the interference.
7. Adjust source frequency if needed — increasing frequency tightens the null spacing, producing a sharper null. A frequency around 450-550 MHz typically produces good null depth at the required bearing.
8. When the null is well-placed, the display shows a clear dark band at ~33 degrees and a bright dot at 213 degrees. The readout updates: "TARGET BEARING: 213 deg".
9. Read: 213 degrees. Done.

**Time estimate:** 4-7 minutes. Mostly spent learning the drag-and-observe mechanic.

---

## Expert Solve Path

1. Recognize: two-source interference, need to null an interference source. The null must be placed at the interference bearing.
2. Turn SYMMETRY LOCK on. The interference source is roughly opposite the target (a common jamming geometry).
3. Drag Source alpha to create a null at the strongest interference direction. The interference pattern shifts. A red dot appears at 213 degrees almost immediately once the null crosses ~33 degrees.
4. Fine-tune position for maximum target brightness.
5. Read: TARGET BEARING: 213 deg. Done.

**Time estimate:** 30-60 seconds.

---

## Data Values

| Parameter | Value | Source/Justification |
|-----------|-------|---------------------|
| Target signal bearing | 213 degrees | META-DESIGN.md R1-03 answer |
| Interference source bearing | 33 degrees | 213 - 180 = 33 degrees (interference from near-opposite direction, consistent with the contact approaching from port quarter while interference arrives from starboard) |
| Default source frequency | 400 MHz | Mid-range starting point |
| Optimal source frequency | ~500 MHz | Produces null spacing appropriate for the angular separation between interference (33 deg) and target (213 deg) |
| Source frequency range | 100-1000 MHz | Wide enough for experimentation |
| Heatmap resolution | 400x400 px | Sufficient for 1-degree bearing precision on the compass ring |
| Target signal power | -80 dBm (weak) | Hidden behind interference at -60 dBm until null is placed |
| Interference power | -60 dBm (strong) | 20 dB above target — completely masks it without nulling |

---

## Narrative Revelation

```
[TAC] GAP+00:47:23 — Null zone placed at bearing 033. Interference
suppressed. Target signal acquired at bearing 213. Source bearing
confirmed: port quarter approach, consistent with sensor contact track.
```

---

## Story Layer

The jamming array was configured to suppress interference from bearing 033 — the direction of a nearby navigational beacon whose routine transmissions were masking the contact's signal. With the beacon nulled, the contact's signal emerged cleanly at bearing 213. This bearing is consistent with the contact approaching from the port quarter — an approach vector that keeps the contact in the ship's sensor shadow for as long as possible before detection. The fact that someone knew to null bearing 033 to reveal bearing 213 means they already knew approximately where the contact was. The null configuration was not exploratory — it was targeted. The operator at this console was expecting the contact.
