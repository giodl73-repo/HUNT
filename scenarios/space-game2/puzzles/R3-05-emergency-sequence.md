# R3-05 — EMERGENCY SEQUENCE
**Round:** CREW RECORD
**Department:** COMMAND / ENGINEERING
**Instrument:** BatSwitch Array — Guarded Emergency Procedure Console
**Classification:** RESTRICTED — CREW RECORD RECONSTRUCTION

---

## 1. Panel Overview

Someone ran an emergency procedure during the gap. Not a standard procedure — a specific sequence that bypasses the CO confirmation step. Every emergency procedure on this ship requires the CO to confirm the final activation. Except one. There is exactly one sequence of switch operations that routes around the confirmation gate: thermal first, then magnetic, then comm, then sensor — with the sensor switch returned to OFF at the end and specific guards closed in a specific order. This sequence was documented in a classified addendum to Contact Protocol Seven. Only three people aboard have read that addendum: the CO, the XO, and the Chief Engineer.

The procedure register counts every physical operation: every guard lift, every switch flip, every guard close. The complete bypass sequence requires exactly 12 operations. That number — 12 — is not arbitrary. It is the count of deliberate, physical actions someone performed at this console during the gap, in the exact order required to bypass the safeguard that exists specifically to prevent unauthorized use.

This is the final feeder puzzle of Round 3. The evidence is complete. By the time the solver reads the procedure register, they know who, they know how, and they know why. What remains is the meta — the CyclicGroupDisplay — which will take the five answers from Round 3 and deliver the name.

---

## 2. Widget Configuration

### Primary Display

| Widget | Class | Configuration |
|--------|-------|---------------|
| Switch Array | `BatSwitch` (x4) | Four guarded toggle switches in a row, each with a red flip-up safety guard. Labels: A = THERMAL, B = MAGNETIC, C = COMM, D = SENSOR. Guards start closed. Switches start OFF. Each guard must be lifted (clicked) before its switch can be flipped. Guards auto-close after 10 seconds if the switch is not flipped. |
| Switch State LEDs | `IndicatorPanel` | 4 LEDs, one per switch. Dark when OFF, lit when ON. Colors: A = orange, B = blue, C = green, D = yellow. |
| Procedure Register | `ScrollingTextDisplay` | Running count of all physical operations performed. Each guard lift, switch flip, and guard close increments the counter. Displays: "OPERATIONS: [count]". Also shows a running log of each operation with timestamp. |
| Sequence Alarm | `MasterAlarm` | Large pulsing red alarm. INACTIVE at start. Activates (pulses at 3 Hz) if the solver performs operations in the wrong order. Clicking the alarm acknowledges and resets the sequence — all switches return to OFF, all guards close, operation counter resets to 0. |
| Automation Gate | `AutoProcedureButton` | Initially LOCKED (greyed out, displays a lock icon). Unlocks only when the correct 12-operation sequence is completed without triggering the MasterAlarm. When unlocked, displays: "PROCEDURE COMPLETE — REGISTER: 12". |

### Input Controls

The BatSwitch guards and toggles are the input controls. The solver physically operates each switch by clicking.

| Interaction | Widget | Action |
|-------------|--------|--------|
| Lift guard | `BatSwitch` guard | Click the red safety cover. It flips up. Operation counter +1. |
| Flip switch ON | `BatSwitch` toggle | Click the switch handle. It moves to ON position. Corresponding LED lights. Operation counter +1. |
| Flip switch OFF | `BatSwitch` toggle | Click the switch handle again. It moves to OFF position. LED goes dark. Operation counter +1. |
| Close guard | `BatSwitch` guard | Click the open safety cover. It flips down. Operation counter +1. |

### Panel Layout

```
┌─────────────────────────────────────────────────────────────────────┐
│  ENGINEERING STATION — EMERGENCY PROCEDURE CONSOLE                 │
│  Terminal: ENG-EMPROC-01-A    Location: MAIN ENGINEERING, DECK 5  │
│  Data Source: Procedure Register — GAP +02:10 to GAP +02:28      │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│         [!]                                                         │
│      MASTER ALARM               OPERATIONS: [ 0 ]                  │
│      (inactive)                                                     │
│                                                                     │
│  ┌────────────┐ ┌────────────┐ ┌────────────┐ ┌────────────┐     │
│  │  ████████  │ │  ████████  │ │  ████████  │ │  ████████  │     │
│  │  [GUARD]   │ │  [GUARD]   │ │  [GUARD]   │ │  [GUARD]   │     │
│  │            │ │            │ │            │ │            │     │
│  │    ┃ OFF  │ │    ┃ OFF  │ │    ┃ OFF  │ │    ┃ OFF  │     │
│  │    ┃      │ │    ┃      │ │    ┃      │ │    ┃      │     │
│  │ THERMAL   │ │ MAGNETIC   │ │   COMM     │ │  SENSOR    │     │
│  │    (A)    │ │    (B)     │ │    (C)     │ │    (D)     │     │
│  └────────────┘ └────────────┘ └────────────┘ └────────────┘     │
│       ○              ○              ○              ○               │
│    (dark)         (dark)         (dark)         (dark)            │
│                                                                     │
│  ┌──────────────────────────────────────┐  ┌────────────────────┐ │
│  │  PROCEDURE LOG                       │  │  AUTO-PROCEDURE    │ │
│  │                                      │  │     ⊘ LOCKED       │ │
│  │  (operations will appear here)       │  │                    │ │
│  │                                      │  │                    │ │
│  └──────────────────────────────────────┘  └────────────────────┘ │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  REFERENCE CARD ──────────────────────────────────────────  │   │
│  │  (see Section 3)                                            │   │
│  └─────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 3. Reference Card

Printed on the bezel placard, top edge of the console. Standard emergency procedure reference — posted at all guarded-switch stations per engineering regulation.

```
EMERGENCY PROCEDURE CONSOLE — OPERATIONS REFERENCE
═══════════════════════════════════════════════════════════════════

  GUARDED SWITCHES
    Each switch has a red safety guard (flip-up cover).
    The guard MUST be lifted before the switch can be toggled.
    The guard can be closed manually at any time.
    If left open, the guard auto-closes after 10 seconds.

  OPERATION COUNTING
    The procedure register counts every physical action:
      - Lifting a guard    = 1 operation
      - Flipping a switch  = 1 operation
      - Closing a guard    = 1 operation

    Each action is logged with a timestamp.

  CORRECT SEQUENCE
    The procedure log recovered from the gap shows the following
    clues about the correct sequence:

      "thermal before magnetic"
      "magnetic before comm"
      "sensor last"
      "sensor re-secured after scan complete"
      "thermal and magnetic guards stowed for duration"

    This means:
      1. Engage switches in order: A → B → C → D
      2. After D (sensor) is engaged and the scan completes,
         return D to OFF and close its guard
      3. Close guards A and B (thermal and magnetic stowed
         for long-duration operation)

  WRONG SEQUENCE
    If any switch is engaged out of order, the MasterAlarm
    activates. Click the alarm to acknowledge and reset.
    All switches return to OFF, all guards close, the
    procedure register resets to 0.

  COMPLETE SEQUENCE (12 OPERATIONS)
    The full correct sequence of physical actions:

      Phase 1 — Engage (8 operations):
        1. Lift guard A          (op 1)
        2. Flip A → ON           (op 2)
        3. Lift guard B          (op 3)
        4. Flip B → ON           (op 4)
        5. Lift guard C          (op 5)
        6. Flip C → ON           (op 6)
        7. Lift guard D          (op 7)
        8. Flip D → ON           (op 8)

      Phase 2 — Sensor re-secure (2 operations):
        9. Flip D → OFF          (op 9)
       10. Close guard D          (op 10)

      Phase 3 — Stow thermal and magnetic (2 operations):
       11. Close guard A          (op 11)
       12. Close guard B          (op 12)

    Total: 12 operations.

  PROCEDURE REGISTER READING
    When all 12 operations are completed in the correct order
    without triggering the alarm, the register reads 12 and
    the AutoProcedureButton unlocks.

═══════════════════════════════════════════════════════════════════
  NOTE: The 12-operation sequence is the CO confirmation bypass.
  Standard emergency procedures require 14 operations (the
  additional 2 are CO confirmation + CO guard close). The
  12-operation variant omits the CO step. This variant is
  documented only in the classified addendum to Contact
  Protocol Seven.
═══════════════════════════════════════════════════════════════════
```

---

## 4. Novice Solve Path

A solver unfamiliar with guarded switches can solve this from the reference card and the physical feedback of the console.

1. **Read the reference card.** It describes the three phases: engage A-B-C-D, re-secure D, stow A and B. Total: 12 operations. It even lists them step by step.

2. **Phase 1: Engage.** Click guard A (it flips up — operation 1). Click switch A (it flips ON, LED A lights orange — operation 2). Click guard B (operation 3). Click switch B (LED B lights blue — operation 4). Click guard C (operation 5). Click switch C (LED C lights green — operation 6). Click guard D (operation 7). Click switch D (LED D lights yellow — operation 8). Register reads 8.

3. **Phase 2: Sensor re-secure.** Click switch D again (it flips OFF, LED D goes dark — operation 9). Click guard D (it closes — operation 10). Register reads 10.

4. **Phase 3: Stow.** Click guard A (it closes — operation 11). Click guard B (it closes — operation 12). Register reads 12.

5. **Confirm.** The AutoProcedureButton unlocks. It displays: "PROCEDURE COMPLETE — REGISTER: 12". The MasterAlarm did not trigger. The sequence was correct.

6. **Read the answer.** Procedure register = **12**.

If the solver makes a mistake (wrong order), the MasterAlarm triggers, everything resets, and they try again. The reference card has the full sequence, so recovery is straightforward.

---

## 5. Expert Solve Path

A solver who reads the reference card and recognizes the structure:

1. The sequence is A-B-C-D engagement, D off, D guard close, A guard close, B guard close. Each guard lift + switch flip = 2 ops per switch. Four switches = 8. Plus 2 for D re-secure + 2 for A/B stow = 12 total.

2. Execute the 12 operations rapidly. No wrong-order risk if you follow A-B-C-D.

3. Register reads 12. Done.

A solver who already knows Contact Protocol Seven (because they are deeply immersed in the fiction): recognizes the 12-operation bypass variant immediately and understands its significance before even touching the switches.

Time for expert: under 2 minutes (physical execution of 12 clicks).

---

## 6. Data Values

### Operation Sequence

| Op # | Action | Switch | Result | Register |
|------|--------|--------|--------|----------|
| 1 | Lift guard A | THERMAL | Guard open | 1 |
| 2 | Flip A ON | THERMAL | LED A lit | 2 |
| 3 | Lift guard B | MAGNETIC | Guard open | 3 |
| 4 | Flip B ON | MAGNETIC | LED B lit | 4 |
| 5 | Lift guard C | COMM | Guard open | 5 |
| 6 | Flip C ON | COMM | LED C lit | 6 |
| 7 | Lift guard D | SENSOR | Guard open | 7 |
| 8 | Flip D ON | SENSOR | LED D lit | 8 |
| 9 | Flip D OFF | SENSOR | LED D dark | 9 |
| 10 | Close guard D | SENSOR | Guard closed | 10 |
| 11 | Close guard A | THERMAL | Guard closed | 11 |
| 12 | Close guard B | MAGNETIC | Guard closed | 12 |

### Final Switch State

| Switch | Position | Guard | LED |
|--------|----------|-------|-----|
| A (THERMAL) | ON | CLOSED | Lit (orange) |
| B (MAGNETIC) | ON | CLOSED | Lit (blue) |
| C (COMM) | ON | OPEN (not closed in sequence) | Lit (green) |
| D (SENSOR) | OFF | CLOSED | Dark |

### Answer Encoding

The answer is the **procedure register reading** — the total count of physical operations performed (12). This is not a binary encoding of the final switch states. The register counts every guard lift, switch flip, and guard close. The 12-operation sequence is the CO confirmation bypass variant; standard procedure would produce 14.

### Answer

**Procedure state register: 12**

---

## 7. Narrative Revelation

```
ENG-EMPROC-01-A — PROCEDURE REGISTER RECONSTRUCTION
TIMESTAMP: GAP +02:10 to GAP +02:28

PROCEDURE EXECUTED: DEFLECTOR ARRAY RECONFIGURATION
VARIANT: CO CONFIRMATION BYPASS — 12-OPERATION SEQUENCE
PROTOCOL: CONTACT PROTOCOL SEVEN — CLASSIFIED ADDENDUM

OPERATION COUNT: 12
SEQUENCE VERIFIED: THERMAL → MAGNETIC → COMM → SENSOR → RE-SECURE

SEQUENCE ANALYSIS:
  This is the 12-operation variant that omits CO confirmation.
  Standard procedure requires 14 operations (12 + CO confirm + close).
  The 12-operation bypass exists for a single scenario: when the CO
  is the one executing the procedure. The CO does not confirm to
  herself. The bypass was designed for exactly this case.

  The classified addendum to Contact Protocol Seven documents this
  variant. Distribution: CO, XO, Chief Engineer. Three people.
  The XO was off-ship during the gap (shore leave, verified).
  The Chief Engineer (Reeves) was logged as off-duty.

  One person had the knowledge, the authority, and the physical
  presence to execute the 12-operation bypass: Captain Vasquez.

TIMELINE RECONSTRUCTION (ROUND 3 — CHRONOLOGICAL):
  GAP +00:42   Chen enters Sensor Bay — pre-authorized code
  GAP +01:22   Kwon collapses — COMPUTER station unmonitored
  GAP +01:42   Vasquez forces Sensor Bay Anteroom door
  GAP +01:55   Reeves' token composes OVERRIDE permission
  GAP +02:10   12-operation bypass executed — deflector reconfigured
  GAP +02:38   Vasquez forces Comm Array Junction
  GAP +03:34   Vasquez forces Computer Core Anteroom — purge initiated

  The bypass at GAP +02:10 reconfigured the deflector for a focused
  scan of the contact. Thermal systems absorbed the heat. Magnetic
  containment stabilized the array. The comm relay opened an
  encrypted channel. The sensor array performed the scan and was
  then re-secured — switched OFF and guard closed — to prevent
  the scan data from being logged by normal channels.

  The 12-operation count is the fingerprint. Standard procedure
  leaves 14 in the register. Only the CO bypass leaves 12.
```

---

## 8. Story Layer — The Circle Closes

The five puzzles of Round 3 have named every participant in the cover-up and traced every link in the chain:

**R3-01 (Patient 3 = KWON):** The COMPUTER officer was incapacitated by the atmospheric suppressant. This was not an accident — the suppressant was dispersed through environmental systems at Vasquez's order. Kwon's collapse left the COMPUTER station unmonitored, disabling the automatic anomaly detection that would have flagged the unauthorized operations that followed.

**R3-02 (Clock position 7 = CHEN):** The COMMS officer entered the Sensor Bay Anteroom using a pre-authorized Contact Protocol Seven override code. Chen had the code because Chen is a department head. Chen used the code because Chen was ordered to.

**R3-03 (Element 4 = REEVES):** The Chief Engineer's authorization token was composed with Okafor's base permission to achieve the OVERRIDE result. Reeves provided the token — either willingly (pre-planned) or through authorization cache extraction. Either way, Reeves is in the chain.

**R3-04 (Badge position 5 = VASQUEZ):** The CO personally forced entry to three restricted compartments along the exact route required to intercept sensor data, relay it over encrypted comm, and purge the records. Badge 401. Command authority. No delegation.

**R3-05 (Register 12 = CO BYPASS):** The emergency deflector reconfiguration was executed using the 12-operation sequence that bypasses CO confirmation — because the CO was the one at the console. Only three people knew this sequence existed. The XO was off-ship. Reeves was "off-duty." Vasquez was the only one present with the knowledge and authority.

---

### The CyclicGroupDisplay

The five answers from Round 3 — 3, 7, 4, 5, 12 — feed the R3-META. The meta traces the authorization chain from detection to execution and produces the step size for the final CyclicGroupDisplay:

**Chain length = 2** (Vasquez authorized herself, then authorized Reeves to execute).

The CyclicGroupDisplay — the ship's classified duty rotation decoder — accepts three inputs:
- N = 8 (from Round 1: the Phase-Locked Echo class code)
- Starting position = 3 (from Round 2: the first-alert station, KWON's position)
- Step size = 2 (from Round 3: the authorization chain length)

Computation: (3 + 2) mod 8 = **5**.

Position 5 on the circle: **VASQUEZ**.

The circle was always there, built into the ship's console. A tool for looking up duty rotation. The solver enters three numbers derived from seventeen puzzles' worth of physical evidence — signal characteristics, system states, personnel records — and the pointer lands on the officer who ordered the cover-up.

The instruments do not lie. Someone wished they did.
