# R1-04 — Orbit Classification

## Panel Overview

| Field | Value |
|-------|-------|
| **Puzzle ID** | R1-04 |
| **Title** | Orbit Classification |
| **Round** | 1 — TELEMETRY |
| **Department** | NAV / SCIENCE |
| **Instrument** | Conic section trajectory analyzer |
| **Answer** | 0.73 (eccentricity) |

**Narrative beat:** An object was tracked during the gap. The nav sensors logged a series of position fixes before the records were purged. The raw position data survived in a backup buffer. The solver fits a theoretical conic section to the observed trajectory, determining the object's orbital eccentricity — which classifies its approach dynamics. Was it orbiting? Passing through? Executing a controlled flyby?

**Win condition:** The ConicSectionDisplay theoretical curve passes through all observed position dots simultaneously. The eccentricity NumericStepper reads 0.73.

---

## Widget Configuration

### Display

```
Widget: ConicSectionDisplay
Config:
  - gridSize: 450 x 450 px
  - gridColor: #1A1A2A
  - curveColor: #00CCFF (cyan — theoretical trajectory)
  - curveWidth: 2
  - tracerDot: true
  - tracerColor: #FFFFFF
  - tracerSpeed: 2
  - tracerTrailLength: 80
  - tracerTrailFade: #003355 -> #00CCFF
  - fociMarkers: true (small cross at focus point)
  - fociColor: #666699
  - observedOverlay: true
  - observedDotCount: 12
  - observedDotColor: #FF6644 (orange-red)
  - observedDotRadius: 4
  - observedPositions: (see Data Values — 12 position fixes along an
    eccentric elliptical arc at e=0.73, l=47 km)
  - matchIndicator: true (glows green when all 12 dots fall within
    2 px of the theoretical curve)
  - matchTolerance: 2 px per dot, all 12 must match
  - eccentricityLabel: top-right corner, shows current e value
  - classificationLabel: below eccentricity, auto-updates:
    e < 0.01: "CIRCULAR"
    0.01 <= e < 1.0: "ELLIPTICAL"
    e = 1.0: "PARABOLIC"
    e > 1.0: "HYPERBOLIC"
```

### Controls

```
Widget: NumericStepper
ID: eccentricity
Config:
  - label: "ECCENTRICITY (e)"
  - min: 0.00
  - max: 2.00
  - step: (driven by RotaryDial)
  - default: 0.00
  - decimalPlaces: 2
  - unit: ""
```

```
Widget: RotaryDial
ID: eccentricityStepSize
Config:
  - label: "e STEP"
  - positions: 2
  - positionLabels: ["0.10", "0.01"]
  - positionValues: [0.10, 0.01]
  - default: position 0 (0.10)
  - knobColor: #667788
  - pointerColor: #FFFFFF
```

```
Widget: NumericStepper
ID: semiLatusRectum
Config:
  - label: "SEMI-LATUS RECTUM (l)"
  - min: 1 km
  - max: 100 km
  - step: 1 km
  - default: 30 km
  - decimalPlaces: 0
  - unit: "km"
```

### Layout

```
+--------------------------------------------------+
|  [ NAV — TRAJECTORY ANALYSIS ]                   |
|                                                  |
|  +--------------------------------------------+ |
|  |                              e: 0.00       | |
|  |                              CIRCULAR      | |
|  |                                            | |
|  |         ConicSectionDisplay                | |
|  |    (cyan: theoretical curve with tracer)   | |
|  |    (orange: observed position dots)        | |
|  |    (+ : focus marker)                      | |
|  |                                            | |
|  +--------------------------------------------+ |
|                                                  |
|  e STEP          ECCENTRICITY (e)                |
|  ( o )          [< ] 0.00 [> ]                   |
|  0.10 / 0.01                                     |
|                                                  |
|  SEMI-LATUS RECTUM (l)                           |
|  [< ] 30 km [> ]                                 |
|                                                  |
|  ---- REFERENCE CARD (see below) ----           |
+--------------------------------------------------+
```

---

## Reference Card (Panel Legend)

```
NAV — TRAJECTORY ANALYSIS REFERENCE

WHAT THE DISPLAY SHOWS
  The cyan curve is a theoretical trajectory
  (a conic section). The orange dots are observed
  position fixes from the sensor log. The tracer
  dot moves along the theoretical curve.

  Your goal: adjust the curve until it passes
  through all 12 observed positions.

WHAT ECCENTRICITY MEANS
  Eccentricity (e) describes the shape of the path:
    e = 0.00       Circle (closed orbit)
    0 < e < 1.00   Ellipse (closed, elongated orbit)
    e = 1.00       Parabola (escape trajectory)
    e > 1.00       Hyperbola (flyby trajectory)

  The classification label updates automatically.

CONTROLS
  ECCENTRICITY stepper: Changes the curve shape.
    Low e = round. High e = elongated.
  e STEP dial: 0.10 for coarse sweep, 0.01 for
    fine fitting.
  SEMI-LATUS RECTUM stepper: Changes the curve
    size (scales it to match the observed distances).

HOW TO FIT THE TRAJECTORY
  1. Set e STEP to 0.10. Click ECCENTRICITY upward
     from 0.00. Watch the curve shape change.
  2. At some value, the curve's curvature roughly
     matches the arc of the orange dots. The dots
     form an open arc — not a closed circle.
  3. Switch e STEP to 0.01. Fine-tune until the
     curve passes through every dot.
  4. Adjust SEMI-LATUS RECTUM to scale the curve
     so the distances match. Try values between
     30 and 60 km.
  5. When all 12 dots lie on the curve, the match
     indicator glows green. Read e.

KEY PHYSICS FACT
  An object on a flyby trajectory has e between
  0.5 and 1.0 — highly eccentric. It approaches
  from deep space, curves past under gravitational
  influence, and departs. The eccentricity tells
  you how close the approach was: higher e = more
  direct pass, lower e = more deflected.
```

---

## Novice Solve Path

1. Read the reference card. Understand: adjust e and l to make the cyan curve match the orange dots. The dots form an open arc, not a closed shape.
2. Start with e = 0.00 (circle), l = 30 km. The cyan curve is a circle. The orange dots clearly do not lie on a circle — they form a curved arc that opens out. The classification reads "CIRCULAR".
3. RotaryDial on 0.10. Click e upward: 0.10, 0.20, ..., 0.50. The curve elongates into progressively flattened ellipses. The dots start to approach the curve but the curvature doesn't match yet.
4. At e = 0.70, the curve's shape is close — the arc curvature is similar to the dots. Several dots are near the curve. Classification: "ELLIPTICAL".
5. Switch RotaryDial to 0.01. Click: 0.71, 0.72, 0.73. At 0.73, more dots line up.
6. Now adjust l (semi-latus rectum). At l = 30 km, the curve is too small. Click upward: 40, 45, 47. At l = 47 km, the curve scales correctly and all 12 dots sit on it. The match indicator glows green.
7. Read the eccentricity: 0.73.

**Time estimate:** 4-6 minutes. The coarse e sweep is 7 clicks. The fine tuning is 3 clicks. The l adjustment is ~17 clicks.

---

## Expert Solve Path

1. See the observed position overlay: an open arc, not closing. Immediately classify as eccentric ellipse or near-parabolic (0.5 < e < 1.0).
2. Estimate from the arc's curvature: it is well open but not hyperbolic (the dots curve back slightly). Guess e ~ 0.7.
3. Set e = 0.70 at coarse step. Switch to 0.01. Three clicks to 0.73. Curve matches curvature.
4. Adjust l from 30 to ~47 km (estimate scale from how far the dots are from the focus). Match indicator glows green at l = 47.
5. Read: 0.73. Done.

**Time estimate:** 30-60 seconds.

---

## Data Values

| Parameter | Value | Source/Justification |
|-----------|-------|---------------------|
| Eccentricity | 0.73 | META-DESIGN.md R1-04 answer |
| Semi-latus rectum | 47 km | Chosen to produce an approach distance consistent with the incident (closest approach ~27 km at e=0.73 using r_min = l/(1+e) = 47/1.73 = 27.2 km) |
| Observed position count | 12 | Enough to constrain the fit unambiguously |
| Observed arc extent | ~120 degrees of true anomaly | Shows enough of the trajectory to distinguish e=0.73 from nearby values |
| Focus location | Center of display | Ship is at the focus of the conic section |
| Default eccentricity | 0.00 | Forces solver to sweep upward |
| Default semi-latus rectum | 30 km | Undersized — forces solver to also adjust scale |
| Periapsis distance | 27.2 km | r_min = l/(1+e) = 47/(1+0.73) = 27.17 km. Contact came within ~27 km. |
| Classification at answer | ELLIPTICAL | 0 < 0.73 < 1.0 |

**Observed position coordinates** (in polar: r in km, theta in degrees from periapsis):

| Fix # | True anomaly (deg) | r (km) |
|-------|-------------------|--------|
| 1 | -60 | 34.4 |
| 2 | -50 | 31.8 |
| 3 | -40 | 29.8 |
| 4 | -30 | 28.2 |
| 5 | -20 | 27.4 |
| 6 | -10 | 27.2 |
| 7 | 0 (periapsis) | 27.2 |
| 8 | 10 | 27.2 |
| 9 | 20 | 27.4 |
| 10 | 30 | 28.2 |
| 11 | 40 | 29.8 |
| 12 | 60 | 34.4 |

All computed from r = l / (1 + e * cos(theta)) with l = 47, e = 0.73.

---

## Narrative Revelation

```
[NAV] GAP+01:12:08 — Trajectory fit complete. Object eccentricity: 0.73.
Classification: highly eccentric elliptical approach. Periapsis: 27 km.
Trajectory consistent with gravitational capture — object entered sensor
range, held station at closest approach, then departed on exit arc.
```

---

## Story Layer

Eccentricity 0.73 is not a natural orbit. Natural objects in eccentric orbits around a gravitating body follow predictable Keplerian paths — they do not "hold station" at periapsis. But the position data shows the contact spent an unusually long time near closest approach (fixes 5-8 are nearly equidistant from the ship at ~27 km). This is consistent with a powered trajectory that used gravitational deflection for approach but applied thrust at periapsis to maintain proximity. The contact was not just passing by — it slowed down, held position for a period, and then departed. A deliberate rendezvous masked as a gravitational flyby. The eccentricity classifies the approach dynamics, but the dwell time at periapsis reveals the intent.
