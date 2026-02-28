# R3-02 — ACCESS CODE
**Round:** CREW RECORD
**Department:** SECURITY
**Instrument:** ModularClockDisplay — Modular Key Rotation Interface
**Classification:** RESTRICTED — CREW RECORD RECONSTRUCTION

---

## 1. Panel Overview

A restricted compartment was accessed during the gap. The door uses the ship's modular key system: a sequence of rotations on an 8-position clock. The access log survived in the security hardware buffer — it recorded the rotation sequence but not who entered it and not the final result. The solver applies the logged rotations to determine where the pointer lands.

The answer is not a code. It is a position on the duty roster. When the clock stops turning, it points at a name.

This is the second panel in Round 3. The tone shifts from clinical to procedural. Someone had a code they should not have had. The system worked exactly as designed — which means the access was authorized. The question is: authorized by whom?

---

## 2. Widget Configuration

### Primary Display

| Widget | Class | Configuration |
|--------|-------|---------------|
| Modular Clock | `ModularClockDisplay` | N = 8 positions (0-7), arranged clockwise on a circular face. Each position labeled with its number and a corresponding duty roster name: 0 = TORRES, 1 = NAKAMURA, 2 = OKAFOR, 3 = KWON, 4 = REEVES, 5 = VASQUEZ, 6 = PARK, 7 = CHEN. A pointer (arrow) starts at position 0. Pointer animates smoothly when rotated. Current position displayed in a central numeric readout. |

### Input Controls

| Widget | Class | Configuration |
|--------|-------|---------------|
| Rotation Amount | `NumericStepper` | Range: 0-7, step: 1. Sets the rotation amount for the next APPLY operation. Label: "ROTATION (+)" |
| Apply Rotation | `LCARSButton` | Momentary. Label: "APPLY ROTATION". On press: pointer advances clockwise by the stepper value, modulo 8. The central readout updates to the new position. Button flashes green on valid application. |
| Sequence Log | `ScrollingTextDisplay` | Displays the access log showing the rotation sequence to be applied. Read-only. Each entry shows a rotation amount and a timestamp. A marker tracks which rotation the solver has applied so far (dimming completed entries). |

### Panel Layout

```
┌─────────────────────────────────────────────────────────────────────┐
│  SECURITY STATION — MODULAR KEY INTERFACE                          │
│  Terminal: SEC-KEYCTL-07-A      Location: DECK 2 CORRIDOR, FR 14  │
│  Data Source: Security Hardware Buffer — GAP +00:42 to +00:47     │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│         0:TORRES                                                    │
│        /         \              ┌──────────────────────────┐       │
│  7:CHEN           1:NAKAMURA    │  ACCESS SEQUENCE LOG     │       │
│  |                |             │                          │       │
│  6:PARK    [ 0 ]  2:OKAFOR     │  ENTRY 1: ROTATE +3     │       │
│  |          ↑     |             │  ENTRY 2: ROTATE +5     │       │
│  5:VASQUEZ        3:KWON        │  ENTRY 3: ROTATE +6     │       │
│        \         /              │  ENTRY 4: ROTATE +1     │       │
│         4:REEVES                │                          │       │
│                                 └──────────────────────────┘       │
│  ┌──────────────┐  ┌──────────────────┐                            │
│  │ ROTATION (+) │  │  APPLY ROTATION  │                            │
│  │ ◄  [ 3 ]  ► │  │     [ ████ ]     │                            │
│  └──────────────┘  └──────────────────┘                            │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  REFERENCE CARD ──────────────────────────────────────────  │   │
│  │  (see Section 3)                                            │   │
│  └─────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 3. Reference Card

Printed on the bezel placard, port side of the ModularClockDisplay. Standard security reference material — all personnel with restricted-area access are briefed on the modular key system during orientation.

```
MODULAR KEY SYSTEM — OPERATIONS REFERENCE
═══════════════════════════════════════════════════════════════════

  THE CLOCK
    The modular key uses an 8-position clock (positions 0-7).
    Positions are arranged clockwise around the dial face.
    The pointer starts at position 0 for every new access attempt.

  ROTATION
    Each rotation adds a number to the current position.
    The result wraps around: if you go past 7, you return to 0.

    Formula:  NEW POSITION = (CURRENT + ROTATION) mod 8

    Example:  Current = 5, Rotation = +4
              5 + 4 = 9
              9 mod 8 = 1
              New position = 1

  MOD 8 — QUICK REFERENCE
    If the sum is 0-7:   the answer IS the sum
    If the sum is 8-15:  subtract 8
    If the sum is 16+:   subtract 8 again (rare for single rotations)

  APPLYING A SEQUENCE
    The access log may show multiple rotations. Apply them in order.
    Each rotation starts from wherever the previous rotation ended.

    Example sequence: +2, +5, +3
      Start at 0.
      +2 → (0+2) mod 8 = 2
      +5 → (2+5) mod 8 = 7
      +3 → (7+3) mod 8 = 2
      Final position: 2

  THE FINAL POSITION
    The position where the pointer rests after all rotations have
    been applied is the access code result. This position corresponds
    to a duty roster entry — the officer whose authorization was
    used (or borrowed) to gain access.

═══════════════════════════════════════════════════════════════════
  NOTE: The sum of all rotations, modulo 8, always produces the
  same result regardless of the starting position. The sequence
  is deterministic. There is exactly one correct final position.
═══════════════════════════════════════════════════════════════════
```

---

## 4. Novice Solve Path

A solver who has never encountered modular arithmetic can solve this panel from the reference card alone.

1. **Read the reference card.** It explains the clock has 8 positions (0-7). Each rotation adds to the current position and wraps around using "mod 8" (subtract 8 if the sum exceeds 7). The pointer starts at 0.

2. **Read the access sequence log.** Four rotations: +3, +5, +6, +1.

3. **Apply rotation 1.** Set the NumericStepper to 3. Press APPLY ROTATION. The pointer moves clockwise from 0 to position 3 (KWON). Central readout shows 3. The first log entry dims.

4. **Apply rotation 2.** Set the NumericStepper to 5. Press APPLY ROTATION. The solver can verify: 3 + 5 = 8. The reference card says "if the sum is 8-15, subtract 8." So 8 - 8 = 0. Pointer moves to position 0 (TORRES). Central readout shows 0. Second log entry dims.

5. **Apply rotation 3.** Set the NumericStepper to 6. Press APPLY ROTATION. 0 + 6 = 6. Sum is 0-7, so the answer IS the sum. Pointer moves to position 6 (PARK). Central readout shows 6. Third log entry dims.

6. **Apply rotation 4.** Set the NumericStepper to 1. Press APPLY ROTATION. 6 + 1 = 7. Pointer moves to position 7 (CHEN). Central readout shows 7. Fourth log entry dims. All entries complete.

7. **Read the final position.** The pointer rests at position **7**. The label reads CHEN.

---

## 5. Expert Solve Path

A solver comfortable with modular arithmetic can skip the step-by-step:

1. Read the rotation sequence: +3, +5, +6, +1.
2. Sum: 3 + 5 + 6 + 1 = 15.
3. 15 mod 8 = 7.
4. Position 7 = CHEN. Done.

The reference card's note confirms: "The sum of all rotations, modulo 8, always produces the same result." The expert adds once and takes the remainder.

Time for expert: under 15 seconds.

---

## 6. Data Values

### Access Sequence

| Step | Rotation | Running Total | Mod 8 | Position | Label |
|------|----------|---------------|-------|----------|-------|
| Start | - | 0 | 0 | 0 | TORRES |
| 1 | +3 | 3 | 3 | 3 | KWON |
| 2 | +5 | 8 | 0 | 0 | TORRES |
| 3 | +6 | 14 | 6 | 6 | PARK |
| 4 | +1 | 15 | 7 | 7 | CHEN |

### Duty Roster (Clock Labels)

| Position | Officer | Department |
|----------|---------|------------|
| 0 | TORRES | OPS |
| 1 | NAKAMURA | TAC |
| 2 | OKAFOR | NAV |
| 3 | KWON | COMPUTER |
| 4 | REEVES | ENG |
| 5 | VASQUEZ | CMD (CO) |
| 6 | PARK | SCIENCE |
| 7 | CHEN | COMMS |

### Answer

**Clock position: 7** (CHEN)

---

## 7. Narrative Revelation

```
SEC-KEYCTL-07-A — HARDWARE BUFFER RECONSTRUCTION
TIMESTAMP: GAP +00:42 to GAP +00:47

DOOR ACCESSED: SENSOR BAY ANTEROOM — DECK 2, FRAME 14
ACCESS METHOD: MODULAR KEY ROTATION — 4-STEP SEQUENCE
ROTATION SEQUENCE: +3, +5, +6, +1
TOTAL ROTATION: 15 ≡ 7 (mod 8)

FINAL POSITION: 7
ROSTER MATCH: LT. CHEN, M. — COMMS OFFICER — POSITION 7

ACCESS AUTHORIZATION: PRE-LOADED EMERGENCY OVERRIDE
SEQUENCE TYPE: CONTACT PROTOCOL SEVEN — FOUR-ROTATION VARIANT

SECURITY NOTE:
  This rotation sequence matches a pre-authorized emergency override
  code — not a forced entry, not a brute-force attempt. Someone
  entered the correct four-rotation sequence on the first attempt,
  without error, in under five minutes. They had the code.

  Contact Protocol Seven override codes are distributed only to
  department heads and above. Lt. Chen is a department head.
  Lt. Chen was logged as on-duty at the COMMS console during this
  window. The COMMS console is on Deck 1. The Sensor Bay Anteroom
  is on Deck 2.

DISCREPANCY: Chen accessed the Sensor Bay Anteroom using a
pre-authorized override. Chen's duty station is on a different deck.
```

---

## 8. Story Layer

The first puzzle showed someone was not where the log claimed (Kwon: "sleeping" but in sickbay). This puzzle shows someone was where they should not have been.

The Sensor Bay Anteroom is a restricted compartment — it houses the raw sensor recording hardware, the physical substrate of the ship's memory. During the gap, someone entered this compartment using a pre-authorized emergency override code. Not a hack. Not a forced entry. A code that was known, entered correctly, on the first attempt.

The code resolves to position 7 on the duty roster: Chen, the COMMS officer. Chen was supposed to be at the COMMS console on Deck 1. Instead, Chen was on Deck 2, entering a restricted compartment that houses the sensor recording system — the same system that was later purged.

This does not yet prove that Chen deleted the records. It proves that Chen had access, had the code, and was physically present at the hardware location where the deletion would later occur. The access was authorized by Contact Protocol Seven — a protocol that only senior officers know exists.

Chen did not improvise. Chen was following a procedure. The question Round 3 is building toward: whose procedure?
