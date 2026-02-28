# R3-04 — BADGE SEQUENCE
**Round:** CREW RECORD
**Department:** SECURITY
**Instrument:** IndicatorPanel — Badge Swipe Log Display
**Classification:** RESTRICTED — CREW RECORD RECONSTRUCTION

---

## 1. Panel Overview

Eighteen badge swipes occurred during the gap. The security hardware buffer recorded every one: who swiped, what door, and whether access was GRANTED, DENIED, or FORCED. FORCED is the anomaly. FORCED means the badge holder did not have standard clearance for that door — but overrode the lock using command authority. A standard officer cannot force a door. Only command-level authority can.

The IndicatorPanel shows 18 LEDs in a row. Most are green (GRANTED) or red (DENIED). A few burn amber. Those amber lights — the FORCED entries — are the ones that matter. Their positions in the sequence, and the badge number they share, name the person who walked through restricted doors by command privilege during the gap.

This is the fourth panel in Round 3. The restraint is gone. The earlier puzzles showed circumstantial evidence — someone in the wrong place, someone with the wrong code, someone providing the wrong authorization. This panel shows direct action. Someone forced their way through restricted doors, repeatedly, using their own badge, during the six missing hours. The badge number is on record.

---

## 2. Widget Configuration

### Primary Display

| Widget | Class | Configuration |
|--------|-------|---------------|
| Badge Sequence LEDs | `IndicatorPanel` | 18 LEDs in a horizontal row, numbered 1-18. Each LED colored by access result: green = GRANTED, red = DENIED, amber = FORCED. Amber LEDs pulse gently (0.5 Hz) to draw attention. Each LED has a small position number above it. |
| Swipe Detail Log | `ScrollingTextDisplay` | Displays detailed log entries for each swipe. Shows: position number, timestamp, badge ID, door name, access result. Entries scroll at reading pace. When the solver clicks an LED on the IndicatorPanel, the ScrollingTextDisplay jumps to that entry. Active entry highlighted with a border. |

### Input Controls

The IndicatorPanel LEDs are clickable — selecting an LED scrolls the log to that entry. No other input controls required. This is a reading/analysis puzzle, not a tuning puzzle.

| Widget | Class | Configuration |
|--------|-------|---------------|
| LED Selection | `IndicatorPanel` | Each LED is clickable. Clicking highlights the LED and scrolls the ScrollingTextDisplay to the corresponding log entry. Previously selected LED returns to its base color. |

### Panel Layout

```
┌─────────────────────────────────────────────────────────────────────┐
│  SECURITY STATION — BADGE SWIPE LOG                                │
│  Terminal: SEC-BADGE-01-A       Location: SECURITY OFFICE, DECK 1 │
│  Data Source: Security Hardware Buffer — GAP +00:00 to GAP +06:00 │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  BADGE SEQUENCE — 18 SWIPES DURING GAP WINDOW                     │
│                                                                     │
│   1   2   3   4   5   6   7   8   9  10  11  12  13  14  15  16  17  18  │
│  [G] [G] [D] [G] [F] [G] [G] [D] [G] [G] [F] [G] [D] [G] [F] [G] [G] [G] │
│   ●   ●   ●   ●   ◉   ●   ●   ●   ●   ●   ◉   ●   ●   ●   ◉   ●   ●   ● │
│                   ↑                       ↑                   ↑        │
│                 AMBER                   AMBER               AMBER      │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  SWIPE DETAIL LOG                                           │   │
│  │                                                              │   │
│  │  POS  TIME    BADGE   DOOR                      RESULT      │   │
│  │  ──── ─────── ─────── ────────────────────────── ─────────  │   │
│  │  01   00:12   217     Main Corridor Jnct A       GRANTED    │   │
│  │  02   00:18   309     Turbolift Bay 2            GRANTED    │   │
│  │  03   00:25   142     Armory Access              DENIED     │   │
│  │  04   00:31   309     Engineering Deck Access    GRANTED    │   │
│  │  05   01:42   401     Sensor Bay Anteroom        FORCED     │   │
│  │  06   01:48   217     Sickbay Main               GRANTED    │   │
│  │  07   01:56   482     Lab 3 Access               GRANTED    │   │
│  │  08   02:14   142     Bridge Aft Access          DENIED     │   │
│  │  09   02:22   309     Cargo Bay 2                GRANTED    │   │
│  │  10   02:30   217     Quarters Deck 3-A          GRANTED    │   │
│  │  11   02:38   401     Comm Array Junction        FORCED     │   │
│  │  12   02:50   482     Science Dept Corridor      GRANTED    │   │
│  │  13   03:05   142     Computer Core Access       DENIED     │   │
│  │  14   03:18   309     Shuttle Bay Observation    GRANTED    │   │
│  │  15   03:34   401     Computer Core Anteroom     FORCED     │   │
│  │  16   03:52   217     Main Corridor Jnct B       GRANTED    │   │
│  │  17   04:15   482     Quarters Deck 2-C          GRANTED    │   │
│  │  18   04:40   309     Main Engineering           GRANTED    │   │
│  │                                                              │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  REFERENCE CARD ──────────────────────────────────────────  │   │
│  │  (see Section 3)                                            │   │
│  └─────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 3. Reference Card

Printed on the bezel placard below the IndicatorPanel. Standard security briefing material.

```
BADGE SWIPE LOG — ANALYSIS REFERENCE
═══════════════════════════════════════════════════════════════════

  ACCESS RESULT TYPES

    GRANTED (Green LED)
      Standard access. The badge holder has normal clearance for
      this door. Routine. Expected.

    DENIED (Red LED)
      Access refused. The badge holder does not have clearance
      for this door and did not attempt to override. Common for
      crew attempting to enter restricted areas above their
      clearance level. The door did not open.

    FORCED (Amber LED)
      Access OVERRIDDEN. The badge holder did not have standard
      clearance but invoked COMMAND AUTHORITY to bypass the lock.
      The door opened. This is logged as a security event.

      WHO CAN FORCE A DOOR:
        Only officers with Command Authority can force entry.
        Command Authority is held by:
          - The Commanding Officer (CO)
          - The Executive Officer (XO)
          - Officers with temporary command delegation

        Standard officers CANNOT force doors. A FORCED entry
        means a command-level officer personally overrode the
        security system using their own badge.

  WHAT TO LOOK FOR

    In a security audit, FORCED entries are the focus. They
    indicate that a senior officer bypassed normal access control.
    Each FORCED entry records the badge ID of the officer who
    invoked command authority.

    Multiple FORCED entries sharing the same badge ID indicate
    a single officer moving through restricted areas by command
    override — a pattern consistent with a deliberate, planned
    traversal of restricted spaces.

  BADGE ID → ROSTER MAPPING

    Badge IDs are assigned sequentially by roster position:
      Position 1 (NAKAMURA): Badge 1xx
      Position 2 (OKAFOR):   Badge 2xx
      Position 3 (KWON):     Badge 3xx
      Position 4 (REEVES):   Badge 4xx  ← 4xx series
      Position 5 (VASQUEZ):  Badge 5xx  ← would be 5xx...
      ...

    EXCEPTION: The CO's badge uses the 401 series (legacy
    command badge numbering). CO = VASQUEZ = Badge 401.

═══════════════════════════════════════════════════════════════════
  NOTE: The position of the FIRST forced entry in the sequence
  is the answer. This identifies when the override chain began.
═══════════════════════════════════════════════════════════════════
```

---

## 4. Novice Solve Path

A solver unfamiliar with security logs can solve this from the IndicatorPanel and reference card alone.

1. **Observe the IndicatorPanel.** 18 LEDs. Most are green. Three are red. Three pulse amber. The amber ones stand out visually — they pulse.

2. **Read the reference card.** Amber = FORCED. FORCED means a command-level officer overrode the door lock. Only the CO or XO can do this. Focus on the amber entries.

3. **Identify the amber positions.** Count across the LED row: positions 5, 11, and 15 are amber. Click each one to read the details in the ScrollingTextDisplay.

4. **Read the three FORCED entries:**
   - Position 5: Badge 401, Sensor Bay Anteroom, 01:42, FORCED
   - Position 11: Badge 401, Comm Array Junction, 02:38, FORCED
   - Position 15: Badge 401, Computer Core Anteroom, 03:34, FORCED

5. **Note the pattern.** All three FORCED entries share badge ID 401. The reference card says Badge 401 = CO = VASQUEZ. One person forced three restricted doors over a two-hour window.

6. **Find the answer.** The reference card says: "The position of the FIRST forced entry in the sequence is the answer." The first amber LED is at position **5**.

7. **Verify the mapping.** Position 5 in the badge sequence. The reference card maps roster position 5 to VASQUEZ. The badge ID (401) confirms: VASQUEZ.

---

## 5. Expert Solve Path

A solver who understands security audit patterns:

1. Scan the IndicatorPanel for amber LEDs. Three: positions 5, 11, 15.
2. Click position 5. Badge 401 — that is the CO's badge.
3. First forced entry = position 5. Answer: **5**.
4. Badge 401 = Vasquez. The CO personally forced restricted doors.

Time for expert: under 30 seconds.

---

## 6. Data Values

### Complete Badge Swipe Log

| Pos | Time | Badge | Door | Result | LED |
|-----|------|-------|------|--------|-----|
| 1 | 00:12 | 217 | Main Corridor Jnct A | GRANTED | Green |
| 2 | 00:18 | 309 | Turbolift Bay 2 | GRANTED | Green |
| 3 | 00:25 | 142 | Armory Access | DENIED | Red |
| 4 | 00:31 | 309 | Engineering Deck Access | GRANTED | Green |
| **5** | **01:42** | **401** | **Sensor Bay Anteroom** | **FORCED** | **Amber** |
| 6 | 01:48 | 217 | Sickbay Main | GRANTED | Green |
| 7 | 01:56 | 482 | Lab 3 Access | GRANTED | Green |
| 8 | 02:14 | 142 | Bridge Aft Access | DENIED | Red |
| 9 | 02:22 | 309 | Cargo Bay 2 | GRANTED | Green |
| 10 | 02:30 | 217 | Quarters Deck 3-A | GRANTED | Green |
| **11** | **02:38** | **401** | **Comm Array Junction** | **FORCED** | **Amber** |
| 12 | 02:50 | 482 | Science Dept Corridor | GRANTED | Green |
| 13 | 03:05 | 142 | Computer Core Access | DENIED | Red |
| 14 | 03:18 | 309 | Shuttle Bay Observation | GRANTED | Green |
| **15** | **03:34** | **401** | **Computer Core Anteroom** | **FORCED** | **Amber** |
| 16 | 03:52 | 217 | Main Corridor Jnct B | GRANTED | Green |
| 17 | 04:15 | 482 | Quarters Deck 2-C | GRANTED | Green |
| 18 | 04:40 | 309 | Main Engineering | GRANTED | Green |

### Badge-to-Officer Mapping

| Badge Series | Officer | Roster Position |
|-------------|---------|-----------------|
| 142 | (crew, insufficient clearance — all DENIED) | — |
| 217 | crew member (routine access) | — |
| 309 | crew member (routine access) | — |
| **401** | **VASQUEZ (CO)** | **5** |
| 482 | crew member (routine access) | — |

### Forced Entry Summary

| Sequence Position | Time | Door | Badge | Officer |
|-------------------|------|------|-------|---------|
| 5 | 01:42 | Sensor Bay Anteroom | 401 | VASQUEZ |
| 11 | 02:38 | Comm Array Junction | 401 | VASQUEZ |
| 15 | 03:34 | Computer Core Anteroom | 401 | VASQUEZ |

### Answer

**Badge position: 5** (first FORCED entry, mapping to roster position 5 = VASQUEZ)

---

## 7. Narrative Revelation

```
SEC-BADGE-01-A — HARDWARE BUFFER RECONSTRUCTION
PERIOD: GAP +00:00 to GAP +06:00

18 BADGE SWIPES RECORDED.
3 FORCED ENTRIES DETECTED.

FORCED ENTRY ANALYSIS:
  POS 05: Badge 401 → Sensor Bay Anteroom       — GAP +01:42
  POS 11: Badge 401 → Comm Array Junction        — GAP +02:38
  POS 15: Badge 401 → Computer Core Anteroom     — GAP +03:34

ALL FORCED ENTRIES: BADGE 401 — CAPT. VASQUEZ, R.
AUTHORITY INVOKED: COMMAND OVERRIDE — CO PRIVILEGE

ROUTE RECONSTRUCTION:
  01:42  Vasquez enters Sensor Bay Anteroom (raw sensor hardware)
  02:38  Vasquez enters Comm Array Junction (encrypted comm relay)
  03:34  Vasquez enters Computer Core Anteroom (data purge terminal)

  Three restricted compartments. Three forced entries. One badge.
  The route traces from the sensor hardware to the comm relay to
  the data purge terminal — the exact path required to intercept
  sensor data, relay it via encrypted channel, and then delete
  the original records.

  The CO did not delegate this. The CO did not send someone.
  The CO went herself.
```

---

## 8. Story Layer

The circumstantial evidence has run out. This is direct.

R3-01 showed Kwon neutralized — incapacitated by an adverse reaction to the atmospheric suppressant, removed from the COMPUTER station where anomalous data would have been flagged. R3-02 showed Chen entering the Sensor Bay with a pre-authorized override code. R3-03 showed Reeves' authorization token composing the OVERRIDE permission. Those three puzzles established that other officers were involved — but they could have been following orders, or duped, or compromised.

R3-04 leaves no room. Captain Vasquez, Badge 401, personally forced entry to three restricted compartments between GAP +01:42 and GAP +03:34. She did not delegate. She used her own badge. She invoked command authority at each door. The route she took — Sensor Bay to Comm Array Junction to Computer Core — is the exact sequence required to intercept raw sensor data, transmit it over an encrypted external channel, and then purge the records.

Vasquez went in person because this could not be delegated. The sensor data, the encrypted transmission, the purge — she handled each step herself. Kwon was unconscious. Chen opened the door. Reeves provided the authorization token. But Vasquez walked the corridor. Vasquez forced the doors. Vasquez touched the hardware.

The name is almost spoken. One puzzle remains.
