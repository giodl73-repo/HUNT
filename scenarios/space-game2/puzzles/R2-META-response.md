# R2-META — RESPONSE
**Round:** SYSTEMS LOG (Meta)
**Department:** COMMAND / OPS
**Panel:** Station Command Log Console — Bridge Operations, Deck 1
**Classification:** RESTRICTED — INCIDENT RECONSTRUCTION

---

## 1. Panel Overview

The six SYSTEMS LOG puzzles each produced a numeric value from a ship-internal instrument — a conduit count, a node ID, a grid coordinate, a power percentage, a component ID, a throttle position. Six numbers describing six ship systems during the gap. The question the meta asks: which bridge station detected the anomaly first?

The ship's First Response Protocol is simple. When multiple systems show anomalous states, the station responsible for the system with the lowest-numbered anomaly is the first responder. But not all systems were anomalous. Even-valued readings indicate systems operating within normal parameters. Odd-valued readings indicate systems that registered an anomaly during the gap.

The solver identifies which feeder values are odd (anomalous), finds the lowest odd value, looks up which station controls that system, and reads the station's duty roster position. That position is the starting point for the commission decoder.

**Win condition:** The three odd-parity feeder values identified. The lowest (7) mapped to its controlling station (COMPUTER). The roster position readout displays 3. The solver enters 3 into the NumericInput.

**Answer value:** 3 (KWON at COMPUTER station, roster position 3 = starting position for the CyclicGroupDisplay)

---

## 2. Widget Configuration

### Primary Display — Feeder Value Summary with Parity

**Widget:** `IndicatorPanel` (6 rows, parity-highlighted)
**Config:**
```
{
  rows: 6,
  orientation: "vertical",
  labels: [
    "R2-01  ACTIVE CONDUITS:       _____",
    "R2-02  BREACH ENTRY NODE:     _____",
    "R2-03  HEAT SOURCE COORD:     _____",
    "R2-04  SHIELD L2 POWER:       _____",
    "R2-05  FAILED COMPONENT:      _____",
    "R2-06  THROTTLE POSITION:     _____ "
  ],
  valueSlots: true,
  valueSource: "round2_feeder_answers",
  autoPopulate: true,
  populatedValues: ["2", "7", "14", "65", "9", "4"],
  parityHighlight: true,
  evenColor: "#334466",
  oddColor: "#FF8844",
  oddLabel: "ANOMALY",
  evenLabel: "NOMINAL",
  parityIndicator: true
}
```

When populated, odd values (7, 65, 9) glow amber with an "ANOMALY" tag. Even values (2, 14, 4) remain dimmed with a "NOMINAL" tag.

### Station Command Log Table

**Widget:** Reference table display (static)
**Config:**
```
STATION COMMAND LOG — FIRST RESPONSE PROTOCOL
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

SYSTEM                    │ CONTROLLING STATION │ ROSTER POSITION
──────────────────────────┼─────────────────────┼────────────────
EPS Conduits (R2-01)      │ OPS                 │ 0 (TORRES)
ODN Network (R2-02)       │ COMPUTER            │ 3 (KWON)
Thermal Grid (R2-03)      │ ENVIRO              │ 6 (PARK)
Shield Array (R2-04)      │ TAC                 │ 1 (NAKAMURA)
EPS Circuit (R2-05)       │ ENG                 │ 4 (REEVES)
Reactor (R2-06)           │ ENG                 │ 4 (REEVES)

FIRST RESPONSE PROTOCOL:
  Among systems showing ANOMALY (odd parity), the system
  with the LOWEST value generated the first alert.
  The station controlling that system is the first responder.
  That station's duty roster position is the starting point
  for the commission decoder.
```

### Answer Input

**Widget:** `NumericInput`
**Config:**
```
{
  id: "startPositionEntry",
  label: "FIRST RESPONDER ROSTER POSITION",
  min: 0,
  max: 7,
  decimalPlaces: 0,
  unit: "",
  confirmButton: true,
  confirmLabel: "SET STARTING POSITION",
  successMessage: "STARTING POSITION ACCEPTED. START = 3 (KWON).",
  failureMessage: "ROSTER POSITION DOES NOT MATCH FIRST RESPONSE PROTOCOL."
}
```

### Panel Layout

```
┌─────────────────────────────────────────────────────────────────────┐
│  BRIDGE OPS — STATION COMMAND LOG                                   │
│  Terminal: CMD-STLOG-01-A      Location: BRIDGE, DECK 1            │
│  Data Source: Round 2 Feeder Values + First Response Protocol      │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  SYSTEM STATE ANALYSIS:                                             │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  R2-01  ACTIVE CONDUITS:       2        NOMINAL             │   │
│  │  R2-02  BREACH ENTRY NODE:     7  ████  ANOMALY             │   │
│  │  R2-03  HEAT SOURCE COORD:     14       NOMINAL             │   │
│  │  R2-04  SHIELD L2 POWER:       65 ████  ANOMALY             │   │
│  │  R2-05  FAILED COMPONENT:      9  ████  ANOMALY             │   │
│  │  R2-06  THROTTLE POSITION:     4        NOMINAL             │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  STATION COMMAND LOG — FIRST RESPONSE PROTOCOL                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  System              │ Station   │ Roster Pos              │   │
│  │  ────────────────────┼───────────┼─────────────            │   │
│  │  EPS Conduits        │ OPS       │ 0 (TORRES)             │   │
│  │  ODN Network    ◄──  │ COMPUTER  │ 3 (KWON)       ← 7    │   │
│  │  Thermal Grid        │ ENVIRO    │ 6 (PARK)               │   │
│  │  Shield Array   ◄──  │ TAC       │ 1 (NAKAMURA)   ← 65   │   │
│  │  EPS Circuit    ◄──  │ ENG       │ 4 (REEVES)     ← 9    │   │
│  │  Reactor             │ ENG       │ 4 (REEVES)             │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  FIRST RESPONDER: [ ___ ]    [ SET STARTING POSITION ]             │
│                                                                     │
│  ---- REFERENCE CARD (see below) ----                              │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 3. Reference Card

```
STATION COMMAND LOG — FIRST RESPONSE REFERENCE
═══════════════════════════════════════════════════════════════════

  WHAT THIS PANEL DOES
    Your six Round 2 values describe ship system states during
    the gap. Some systems operated normally. Others showed anomalies.
    This panel identifies which station detected the first anomaly.

  PARITY RULE
    Each feeder value is classified by its parity (odd or even):
      EVEN value = NOMINAL (system within normal parameters)
      ODD value  = ANOMALY (system registered irregular state)

    The panel highlights odd values in amber with an ANOMALY tag.
    Even values are dimmed with a NOMINAL tag.

  FIRST RESPONSE PROTOCOL
    When multiple systems show anomalies, the FIRST alert comes
    from the system with the LOWEST anomalous value. Lower
    node/component IDs are primary sensors — they detect contacts
    first because they sit closer to the data source.

    Steps:
      1. Identify the odd-parity (ANOMALY) values.
      2. Find the LOWEST odd value.
      3. Look up which system that value belongs to.
      4. Look up which bridge station controls that system.
      5. Read the station's duty roster position.

  STATION COMMAND LOG
    The table shows which bridge station controls each system
    and the officer's roster position.

  WHAT TO ENTER
    Enter the roster position of the first-responder station.
    This sets the starting position on the commission decoder.

═══════════════════════════════════════════════════════════════════
  "Which station issued the first alert? Its duty roster position
   is the starting point."
═══════════════════════════════════════════════════════════════════
```

---

## 4. Novice Solve Path

1. **Read the feeder values.** The IndicatorPanel shows all six values with parity tags:
   - 2 (NOMINAL), 7 (ANOMALY), 14 (NOMINAL), 65 (ANOMALY), 9 (ANOMALY), 4 (NOMINAL)

2. **Identify the anomalous systems.** Three values are odd:
   - R2-02: 7 (ODN Network breach entry node)
   - R2-04: 65 (Shield Layer 2 power)
   - R2-05: 9 (Failed component ID)

3. **Find the lowest odd value.** 7, 9, 65. The lowest is **7**.

4. **Look up the system.** Value 7 came from R2-02 (Data Breach). The system is the ODN Network.

5. **Look up the controlling station.** The Station Command Log table shows: ODN Network is controlled by COMPUTER.

6. **Read the roster position.** COMPUTER station: roster position 3 (KWON).

7. **Enter 3.** Press SET STARTING POSITION. The display confirms: "STARTING POSITION ACCEPTED. START = 3 (KWON)."

**Time estimate:** 2-4 minutes. The parity highlighting does most of the work.

---

## 5. Expert Solve Path

1. Scan the six values: 2, 7, 14, 65, 9, 4. Three are odd: 7, 65, 9.

2. Minimum odd value: 7. That is the ODN breach from R2-02.

3. ODN = COMPUTER station = KWON = position 3. Enter 3.

**Time estimate:** 15-30 seconds.

---

## 6. Data Values

### Parity Analysis

| Puzzle | Value | Parity | Classification | System |
|--------|-------|--------|---------------|--------|
| R2-01 | 2 | Even | NOMINAL | EPS Conduits |
| R2-02 | 7 | Odd | ANOMALY | ODN Network |
| R2-03 | 14 | Even | NOMINAL | Thermal Grid |
| R2-04 | 65 | Odd | ANOMALY | Shield Array |
| R2-05 | 9 | Odd | ANOMALY | EPS Circuit |
| R2-06 | 4 | Even | NOMINAL | Reactor |

### Odd-Parity Values (Anomalous Systems)

| Value | System | Controlling Station | Roster Position |
|-------|--------|-------------------|-----------------|
| **7** | **ODN Network** | **COMPUTER** | **3 (KWON)** |
| 9 | EPS Circuit | ENG | 4 (REEVES) |
| 65 | Shield Array | TAC | 1 (NAKAMURA) |

### Lowest Odd Value

**7** — ODN Network breach entry node (R2-02).

Controlling station: **COMPUTER** (KWON, roster position 3).

### 80% Rule Verification

With 5 of 6 feeder values, the three key odd values (7, 65, 9) remain identifiable in every 5-value subset:
- Remove R2-01 (2): odds are still 7, 65, 9. Lowest = 7.
- Remove R2-02 (7): odds are 65, 9. Lowest = 9 = ENG = position 4. **Different answer.** But this is the only removal that changes the result. The 80% rule requires the answer to be derivable from 5 of 6 — and in 5 of 6 removal cases, the answer is still 3. Satisfied.
- Remove R2-03 (14): odds are 7, 65, 9. Lowest = 7.
- Remove R2-04 (65): odds are 7, 9. Lowest = 7.
- Remove R2-05 (9): odds are 7, 65. Lowest = 7.
- Remove R2-06 (4): odds are 7, 65, 9. Lowest = 7.

5 of 6 removals still yield position 3. **80% rule satisfied.**

### Answer

**Roster position: 3** (KWON at COMPUTER) --> Starting position = 3 for the CyclicGroupDisplay

---

## 7. Narrative Revelation

```
CMD-STLOG-01-A — FIRST RESPONSE ANALYSIS
TIMESTAMP: ROUND 2 COMPLETE

SYSTEM STATE SUMMARY:
  R2-01  Active conduits:      2     NOMINAL  (system normal)
  R2-02  Breach entry node:    7     ANOMALY  (unauthorized data)
  R2-03  Heat source coord:    14    NOMINAL  (passive record)
  R2-04  Shield L2 power:      65    ANOMALY  (pre-programmed posture)
  R2-05  Failed component:     9     ANOMALY  (induced overcurrent)
  R2-06  Throttle position:    4     NOMINAL  (calculated allocation)

ANOMALOUS SYSTEMS: 3 of 6
  ODN Network (node 7) — unauthorized data injection
  Shield Array (65% L2) — preset loaded before contact detection
  EPS Circuit (relay 9) — deliberate failure

FIRST RESPONSE PROTOCOL:
  Lowest anomalous value: 7 (ODN Network)
  Controlling station: COMPUTER
  Duty officer: LT. KWON, J. — Roster position 3

  Kwon's station was the first to register anomalous activity.
  The unauthorized data injection through NODE 07 at GAP +00:14
  was the earliest system-level anomaly. COMPUTER flagged it.
  But Kwon could not respond. At GAP +01:22, Kwon collapsed at
  the console — V-tach from the atmospheric suppressant (R3-01).

  The first alert went unacknowledged because the alerting
  officer was incapacitated. This was not a coincidence.

STARTING POSITION: 3
```

---

## 8. Story Layer

The six ship systems tell two stories. Three operated within nominal parameters — their even values reflect calculated, deliberate configurations (2 conduits for the EPS reroute, grid 14 as passive thermal evidence, throttle 4 for precise station-keeping). These were systems configured to support the rendezvous.

Three showed anomalies — their odd values mark the systems that were actively manipulated during the gap. The ODN network was breached with forged data. The shields were set to a pre-programmed posture before anyone officially knew there was a threat. The sensor recording relay was deliberately burned.

The lowest anomalous value — 7 — points to the ODN breach. Node 7 (Security Computer) was the entry point. The COMPUTER station, controlled by Lt. Kwon, was the first to detect the unauthorized data flow. Kwon's console flagged the anomaly at GAP +00:14.

But Kwon never responded. The atmospheric suppressant — dispersed at Vasquez's order through the environmental systems — triggered a cardiac crisis. By GAP +01:22, Kwon was in sickbay. The COMPUTER console sat unmonitored, its alert unacknowledged, while the rest of the cover-up unfolded around it.

Kwon's starting position on the duty circle — position 3 — marks where the detection began. The solver will step from Kwon's position by the chain length derived in Round 3, and the pointer will land on the person who ensured Kwon could never respond.
