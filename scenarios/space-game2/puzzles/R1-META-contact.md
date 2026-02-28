# R1-META — CONTACT
**Round:** TELEMETRY (Meta)
**Department:** SCIENCE / COMMS
**Panel:** Contact Signature Classification Console — Science Station, Deck 1
**Classification:** RESTRICTED — INCIDENT RECONSTRUCTION

---

## 1. Panel Overview

The six TELEMETRY puzzles each produced a physical measurement — a frequency, a bearing, an eccentricity, a count, a data rate. Six numbers, each wrung from a different instrument. Separately, they are readings. Together, they are a signature.

The Contact Signature Reference Table is a classified database embedded in every starship's science console. It lists known contact types by their observable parameters. The solver maps the six feeder values to the table columns and finds the one row that matches all six. That row names what the ship encountered during the gap. The contact type's class code — the number of sensor modalities required to confirm identification — is the output.

The class code configures the CyclicGroupDisplay: it sets N, the number of points on the commission decoder circle.

**Win condition:** All six feeder values mapped to the Contact Signature Reference Table. One row matches all six: Phase-Locked Echo. The class code readout displays 8. The solver enters 8 into the NumericInput.

**Answer value:** 8 (Phase-Locked Echo class code = N for the CyclicGroupDisplay)

---

## 2. Widget Configuration

### Primary Display — Feeder Value Summary

**Widget:** `IndicatorPanel` (6 rows)
**Config:**
```
{
  rows: 6,
  orientation: "vertical",
  labels: [
    "R1-01  CARRIER FREQUENCY:    _____ GHz",
    "R1-02  HARMONIC RATIO:       _____:1",
    "R1-03  TARGET BEARING:       _____ deg",
    "R1-04  TRAJECTORY ECCENT.:   _____",
    "R1-05  REAL CONTACTS:        _____",
    "R1-06  DATA RATE:            _____ kbps"
  ],
  valueSlots: true,
  valueSource: "round1_feeder_answers",
  autoPopulate: true,
  populatedValues: [
    "2.340",
    "3",
    "213",
    "0.73",
    "5",
    "384"
  ],
  ledColor: "#00CC88",
  blinkOnPopulate: true
}
```

When the solver completes Round 1 puzzles, the feeder values fill in automatically. If the solver arrives at the meta before completing all feeders, unfilled slots show "---".

### Contact Signature Reference Table

**Widget:** `NetworkGridDisplay` variant — tabular reference display
**Config:**
```
{
  displayType: "reference-table",
  title: "CONTACT SIGNATURE REFERENCE — CLASSIFIED",
  columns: [
    "Contact Type",
    "Freq Band",
    "Harmonic Ratio",
    "Approach Bearing",
    "Trajectory",
    "Formation",
    "Data Rate",
    "Class Code"
  ],
  rows: [
    {
      type: "Nav Beacon",
      freqBand: "L-band (1-2 GHz)",
      harmonic: "1:1",
      bearing: "omnidirectional",
      trajectory: "circular (e ~ 0)",
      formation: "1 contact",
      dataRate: "< 1 kbps",
      classCode: 3
    },
    {
      type: "Distress Beacon",
      freqBand: "S-band (2-4 GHz)",
      harmonic: "1:1",
      bearing: "omnidirectional",
      trajectory: "circular",
      formation: "1 contact",
      dataRate: "1-10 kbps",
      classCode: 4
    },
    {
      type: "Commercial Traffic",
      freqBand: "C-band (4-8 GHz)",
      harmonic: "none",
      bearing: "directional",
      trajectory: "elliptical (e = 0.1-0.5)",
      formation: "2-4 contacts",
      dataRate: "> 1 Mbps",
      classCode: 5
    },
    {
      type: "Military Transponder",
      freqBand: "X-band (8-12 GHz)",
      harmonic: "2:1",
      bearing: "directional",
      trajectory: "elliptical",
      formation: "3-6 contacts",
      dataRate: "64-512 kbps",
      classCode: 6
    },
    {
      type: "Subspace Relay",
      freqBand: "Ku-band",
      harmonic: "none",
      bearing: "fixed",
      trajectory: "stationary",
      formation: "1 contact",
      dataRate: "> 10 Mbps",
      classCode: 4
    },
    {
      type: "Stellar Emission",
      freqBand: "broadband",
      harmonic: "none",
      bearing: "isotropic",
      trajectory: "hyperbolic (e > 1)",
      formation: "many",
      dataRate: "N/A",
      classCode: 3
    },
    {
      type: "Sensor Ghost",
      freqBand: "matches own",
      harmonic: "1:1",
      bearing: "matches own",
      trajectory: "matches own",
      formation: "1 contact",
      dataRate: "matches own",
      classCode: 7
    },
    {
      type: "Phase-Locked Echo",
      freqBand: "S-band (2-4 GHz)",
      harmonic: "n:1 (n > 2)",
      bearing: "directional",
      trajectory: "hyperbolic-capture (e = 0.6-0.9)",
      formation: "4-7 contacts",
      dataRate: "128-512 kbps",
      classCode: 8,
      highlight: true
    }
  ],
  matchHighlightColor: "#00FF88",
  noMatchColor: "#661111",
  partialMatchColor: "#FFCC00"
}
```

### Answer Input

**Widget:** `NumericInput`
**Config:**
```
{
  id: "classCodeEntry",
  label: "ENTER CLASS CODE",
  min: 1,
  max: 10,
  decimalPlaces: 0,
  unit: "",
  confirmButton: true,
  confirmLabel: "SUBMIT TO COMMISSION DECODER",
  successMessage: "CLASS CODE ACCEPTED. N = 8. COMMISSION DECODER CONFIGURED.",
  failureMessage: "CLASS CODE DOES NOT MATCH CONTACT SIGNATURE."
}
```

### Panel Layout

```
┌─────────────────────────────────────────────────────────────────────┐
│  SCIENCE STATION — CONTACT SIGNATURE CLASSIFICATION                 │
│  Terminal: SCI-CLASS-01-A      Location: BRIDGE SCIENCE, DECK 1    │
│  Data Source: Round 1 Feeder Measurements                          │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  YOUR MEASUREMENTS:                                                 │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  R1-01  CARRIER FREQUENCY:    2.340 GHz                     │   │
│  │  R1-02  HARMONIC RATIO:       3:1                           │   │
│  │  R1-03  TARGET BEARING:       213 deg                       │   │
│  │  R1-04  TRAJECTORY ECCENT.:   0.73                          │   │
│  │  R1-05  REAL CONTACTS:        5                             │   │
│  │  R1-06  DATA RATE:            384 kbps                      │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  CONTACT SIGNATURE REFERENCE — CLASSIFIED                          │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  Type          │ Freq   │ Harm │ Brg  │ Traj │ Fmt │ Rate │ CC│
│  │  ──────────────┼────────┼──────┼──────┼──────┼─────┼──────┼───│
│  │  Nav Beacon    │ L-band │ 1:1  │ omni │ circ │  1  │ <1k  │ 3 │
│  │  Distress Bcn  │ S-band │ 1:1  │ omni │ circ │  1  │ 1-10k│ 4 │
│  │  Commercial    │ C-band │ none │ dir  │ ell  │ 2-4 │ >1M  │ 5 │
│  │  Military      │ X-band │ 2:1  │ dir  │ ell  │ 3-6 │64-512│ 6 │
│  │  Subsp Relay   │ Ku-bnd │ none │ fixd │ stat │  1  │ >10M │ 4 │
│  │  Stellar Em.   │ broad  │ none │ iso  │ hyp  │ many│ N/A  │ 3 │
│  │  Sensor Ghost  │ match  │ 1:1  │ match│ match│  1  │match │ 7 │
│  │  *PH-LOCK ECHO*│*S-band*│*n:1* │*dir* │*h-c* │*4-7*│128-5k│*8*│
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ENTER CLASS CODE:  [ ___ ]    [ SUBMIT TO COMMISSION DECODER ]    │
│                                                                     │
│  ---- REFERENCE CARD (see below) ----                              │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 3. Reference Card

```
CONTACT SIGNATURE CLASSIFICATION — OPERATOR REFERENCE
═══════════════════════════════════════════════════════════════════

  WHAT THIS PANEL DOES
    Your six Round 1 measurements describe a single contact.
    The Contact Signature Reference Table lists known contact types.
    Each type has a characteristic signature across six parameters.
    Find the type that matches ALL SIX of your measurements.

  HOW TO USE THE TABLE
    1. Look at your first measurement: CARRIER FREQUENCY.
       Find which rows in the table have a frequency band that
       contains your value. (2.340 GHz = S-band, which is 2-4 GHz.)
    2. Check the next column: HARMONIC RATIO. Your value is 3:1.
       Which of the remaining rows match "n:1 where n > 2"?
    3. Continue through all six columns. Most contact types will
       be eliminated after 2-3 columns.
    4. The contact type that matches ALL SIX parameters is the answer.
    5. Read its CLASS CODE from the rightmost column.

  WHAT IS A CLASS CODE?
    The class code indicates how many sensor modalities are needed
    to confirm identification of that contact type. Higher codes
    mean more complex contacts requiring more evidence.

  WHAT TO DO WITH THE CLASS CODE
    Enter it in the numeric input field at the bottom.
    Press SUBMIT. The class code configures the commission decoder
    — it sets the number of points on the decoder circle.

  NOTE ON PARTIAL MATCHING
    If you are missing one or two feeder values, check which
    contact type matches the values you DO have. If only one
    type matches 4 or 5 of your measurements, it is almost
    certainly the correct one. The table is designed so that
    no two contact types match on more than 2 parameters.

═══════════════════════════════════════════════════════════════════
  "Enter the contact's class code as the number of points
   on the commission decoder."
═══════════════════════════════════════════════════════════════════
```

---

## 4. Novice Solve Path

1. **Read the feeder values.** The IndicatorPanel shows all six measurements from Round 1.

2. **Start with frequency.** 2.340 GHz. Look at the Freq Band column. L-band is 1-2 GHz (too low). S-band is 2-4 GHz — 2.340 falls in this range. Two rows have S-band: Distress Beacon and Phase-Locked Echo.

3. **Check harmonic ratio.** 3:1. Distress Beacon says 1:1. Phase-Locked Echo says n:1 (n > 2). 3 > 2, so Phase-Locked Echo matches. Distress Beacon eliminated.

4. **Verify remaining columns for Phase-Locked Echo:**
   - Bearing: 213 deg = directional. Phase-Locked Echo says directional. Match.
   - Trajectory: e = 0.73, which is in range 0.6-0.9. Phase-Locked Echo says hyperbolic-capture (e = 0.6-0.9). Match.
   - Formation: 5 contacts. Phase-Locked Echo says 4-7. Match.
   - Data rate: 384 kbps. Phase-Locked Echo says 128-512 kbps. Match.

5. **All six match.** Contact type: Phase-Locked Echo. Class code: **8**.

6. **Enter 8** in the NumericInput. Press SUBMIT. The display confirms: "CLASS CODE ACCEPTED. N = 8."

**Time estimate:** 3-5 minutes. Mostly reading the table and cross-referencing.

---

## 5. Expert Solve Path

1. Glance at the feeder values. S-band + 3:1 harmonic immediately identifies Phase-Locked Echo — no other contact type has n:1 harmonic with n > 2 in the S-band.

2. Confirm with a quick scan: e = 0.73 (hyperbolic-capture range), 5 contacts (4-7), 384 kbps (128-512). All match.

3. Class code 8. Enter and submit.

**Time estimate:** 30-60 seconds. The harmonic ratio is the instant discriminator.

---

## 6. Data Values

### Feeder Value Matching

| Parameter | Feeder Value | Phase-Locked Echo Range | Match? |
|-----------|-------------|------------------------|--------|
| Carrier Frequency | 2.340 GHz | S-band (2-4 GHz) | YES |
| Harmonic Ratio | 3:1 | n:1 (n > 2) | YES |
| Target Bearing | 213 deg | directional | YES |
| Trajectory Eccentricity | 0.73 | 0.6-0.9 | YES |
| Real Contacts | 5 | 4-7 | YES |
| Data Rate | 384 kbps | 128-512 kbps | YES |

### Match Count by Contact Type

| Contact Type | Matches (of 6) |
|-------------|----------------|
| Nav Beacon | 0 |
| Distress Beacon | 1 (freq band only) |
| Commercial Traffic | 0 |
| Military Transponder | 2 (bearing + data rate range overlap) |
| Subspace Relay | 0 |
| Stellar Emission | 0 |
| Sensor Ghost | 2 (freq band if interpreted as "matching own S-band" + formation) |
| **Phase-Locked Echo** | **6** |

No other type matches more than 2 of the 6 parameters.

### Class Code Table

| Contact Type | Class Code | Meaning |
|-------------|-----------|---------|
| Nav Beacon | 3 | 3 sensor modalities to confirm |
| Distress Beacon | 4 | 4 sensor modalities |
| Commercial Traffic | 5 | 5 sensor modalities |
| Military Transponder | 6 | 6 sensor modalities |
| Subspace Relay | 4 | 4 sensor modalities |
| Stellar Emission | 3 | 3 sensor modalities |
| Sensor Ghost | 7 | 7 sensor modalities (hard to distinguish from real) |
| **Phase-Locked Echo** | **8** | **All 8 modalities required (most complex contact)** |

### 80% Rule Verification

With any 5 of 6 feeder values, Phase-Locked Echo still matches all 5. No other type matches more than 2 of any 5-value subset. The identification is robust to one missing feeder.

### Answer

**Class code: 8** (Phase-Locked Echo) --> N = 8 for the CyclicGroupDisplay

---

## 7. Narrative Revelation

```
SCI-CLASS-01-A — CONTACT SIGNATURE CLASSIFICATION
TIMESTAMP: ROUND 1 COMPLETE

FEEDER MEASUREMENTS:
  Carrier frequency:     2.340 GHz     (S-band)
  Harmonic ratio:        3:1           (active tracking)
  Target bearing:        213 deg       (port quarter approach)
  Trajectory eccentricity: 0.73        (hyperbolic-capture)
  Formation size:        5 contacts    (primary + 4 escorts)
  Data rate:             384 kbps      (sustained data exchange)

SIGNATURE MATCH: PHASE-LOCKED ECHO
  Match confidence: 6 of 6 parameters within range.
  No other contact type matches more than 2 parameters.

CLASS CODE: 8
  Commission decoder configured: 8-point circle.

CLASSIFICATION NOTE:
  A Phase-Locked Echo is not a phenomenon. It is an entity.
  It locks onto a ship's own transmission at a harmonic ratio,
  holds station within sensor range, and maintains sustained
  bidirectional data exchange. It was not detected by the ship.
  The ship was detected by it. It chose to respond. It chose
  the harmonic. It chose to hold formation.

  The ship did not encounter a Phase-Locked Echo during the gap.
  A Phase-Locked Echo encountered the ship.
```

---

## 8. Story Layer

The six instruments of Round 1 each captured one dimension of the contact's signature. The spectrum analyzer found the carrier. The oscilloscope proved the harmonic lock. The phased array revealed the bearing. The trajectory analyzer showed the approach dynamics. The radar confirmed the formation. The vector signal analyzer measured the data rate.

Each measurement alone tells a story fragment. The carrier at 2.340 GHz says: someone was transmitting. The 3:1 harmonic says: they were listening to us first. The bearing says: they came from the port quarter. The eccentricity says: they slowed down and held station. The formation says: there were five of them. The data rate says: they were talking.

Together, the six measurements match exactly one entry in the Contact Signature Reference Table: Phase-Locked Echo. An entity that mirrors your own signal, announces itself through harmonic resonance, and then waits. The class code — 8 — reflects its complexity. All eight sensor modalities are required to confirm because a Phase-Locked Echo is the hardest contact to distinguish from a sensor artifact. It looks like your own signal reflected back. Only the harmonic ratio and the trajectory dynamics prove it is something else.

The class code 8 feeds the CyclicGroupDisplay as N. Eight bridge officers on the duty circle. Eight points on the commission decoder. The contact has set the size of the circle that will ultimately name the person who ordered the cover-up.
