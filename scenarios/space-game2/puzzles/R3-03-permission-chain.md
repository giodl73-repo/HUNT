# R3-03 — PERMISSION CHAIN
**Round:** CREW RECORD
**Department:** SECURITY / COMPUTER
**Instrument:** CayleyTableDisplay — Group Composition Interface
**Classification:** RESTRICTED — CREW RECORD RECONSTRUCTION

---

## 1. Panel Overview

An unauthorized system override was executed during the gap. The ship's permission system uses group composition: each crew member holds a base permission element, and restricted actions require composing multiple elements to reach a target result. The override that unlocked the classified sensor purge command required composing a known base permission with an unknown authorization token. The Cayley table shows all possible compositions. The solver must find the missing element.

This is the third panel in Round 3. The tone shifts from procedural to accusatory. Round 3 is no longer asking "what happened?" — it is asking "who made it happen?" The answer to this puzzle names a specific person in the authorization chain. The permission structure was not broken. It was used correctly, by the wrong people, for the wrong purpose.

---

## 2. Widget Configuration

### Primary Display

| Widget | Class | Configuration |
|--------|-------|---------------|
| Cayley Table | `CayleyTableDisplay` | 8x8 multiplication table for the group Z_8 (integers modulo 8 under addition). Row headers 0-7 (left), column headers 0-7 (top). Each cell shows the composition result (a + b) mod 8. The table is interactive: when the solver selects an element via the RotaryDial and presses COMPOSE, the corresponding row-column intersection highlights with a bright border. A "TARGET" marker highlights cell value 6 wherever it appears in the table. |

### Input Controls

| Widget | Class | Configuration |
|--------|-------|---------------|
| Element Selector | `RotaryDial` | 8 positions: 0, 1, 2, 3, 4, 5, 6, 7. Each position labeled with its number and roster name: 0=TORRES, 1=NAKAMURA, 2=OKAFOR, 3=KWON, 4=REEVES, 5=VASQUEZ, 6=PARK, 7=CHEN. Label: "AUTHORIZATION TOKEN (X)" |
| Compose | `LCARSButton` | Momentary. Label: "COMPOSE". On press: highlights cell (2, X) in the Cayley table. If the result equals 6 (the OVERRIDE target), the cell flashes green and the display shows "OVERRIDE ACHIEVED." Otherwise the cell flashes amber and shows "INSUFFICIENT — RESULT: [value]". |
| Base Permission Display | `IndicatorPanel` | Single LED readout showing the fixed base permission: "BASE: 2 (OKAFOR)". Always lit cyan. Non-interactive. |
| Target Display | `IndicatorPanel` | Single LED readout showing the target result: "TARGET: 6 (OVERRIDE)". Always lit red. Non-interactive. |

### Panel Layout

```
┌─────────────────────────────────────────────────────────────────────┐
│  SECURITY STATION — PERMISSION COMPOSITION INTERFACE               │
│  Terminal: SEC-AUTH-03-C       Location: COMPUTER CORE, DECK 4    │
│  Data Source: Authorization Chain Log — GAP +01:55                 │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  BASE: 2 (OKAFOR)  ●                TARGET: 6 (OVERRIDE)  ●       │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  CAYLEY TABLE — Z_8 (addition mod 8)                        │   │
│  │                                                              │   │
│  │  ∘ │  0    1    2    3    4    5    6    7                   │   │
│  │  ──┼───────────────────────────────────────                  │   │
│  │  0 │  0    1    2    3    4    5    6    7                   │   │
│  │  1 │  1    2    3    4    5    6    7    0                   │   │
│  │  2 │  2    3    4    5    6    7    0    1     ← BASE ROW   │   │
│  │  3 │  3    4    5    6    7    0    1    2                   │   │
│  │  4 │  4    5    6    7    0    1    2    3                   │   │
│  │  5 │  5    6    7    0    1    2    3    4                   │   │
│  │  6 │  6    7    0    1    2    3    4    5                   │   │
│  │  7 │  7    0    1    2    3    4    5    6                   │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ┌──────────────────────┐  ┌──────────────────┐                    │
│  │ AUTH TOKEN (X)       │  │     COMPOSE      │                    │
│  │ ◯ 0  ◯ 1  ◯ 2  ◯ 3│  │    [ ████ ]      │                    │
│  │ ● 4  ◯ 5  ◯ 6  ◯ 7│  │                   │                    │
│  └──────────────────────┘  └──────────────────┘                    │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  REFERENCE CARD ──────────────────────────────────────────  │   │
│  │  (see Section 3)                                            │   │
│  └─────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 3. Reference Card

Printed on the bezel placard below the Cayley table. Standard security reference material — all officers with system access are briefed on the permission composition model.

```
PERMISSION COMPOSITION — OPERATIONS REFERENCE
═══════════════════════════════════════════════════════════════════

  THE PERMISSION MODEL
    Every officer has a base permission number (0-7).
    Restricted actions require a specific target number.
    To reach the target, the officer's base permission must be
    COMPOSED with an authorization token provided by another
    officer.

  COMPOSITION
    Composition uses modular addition:

      RESULT = (BASE + TOKEN) mod 8

    The Cayley table shows every possible composition.
    Row = base permission. Column = authorization token.
    Cell = result.

  HOW TO USE THE TABLE
    1. Find your BASE PERMISSION on the left (row header).
    2. Scan across that row to find the TARGET value.
    3. Read the column header above that cell.
       That column header is the TOKEN you need.

    Example:
      Base = 2, Target = 5
      Row 2: scan across... cell at column 3 reads "5".
      Token needed: 3.
      Verification: (2 + 3) mod 8 = 5. Correct.

  SOLVING FOR THE UNKNOWN TOKEN
    If you know the BASE and the TARGET:

      TOKEN = (TARGET − BASE) mod 8

    If the subtraction is negative, add 8.

    Example: Base = 2, Target = 6
      6 − 2 = 4.    Token = 4.
      Verification: (2 + 4) mod 8 = 6. Correct.

  ROSTER MAPPING
    Each element (0-7) maps to a duty roster position:
      0=TORRES  1=NAKAMURA  2=OKAFOR  3=KWON
      4=REEVES  5=VASQUEZ   6=PARK    7=CHEN

    The token number identifies which officer provided
    the authorization.

═══════════════════════════════════════════════════════════════════
```

---

## 4. Novice Solve Path

A solver who has never seen a Cayley table can solve this from the reference card alone.

1. **Read the panel indicators.** Base permission = 2 (OKAFOR). Target = 6 (OVERRIDE).

2. **Read the reference card.** It explains: find your base on the left (row 2), scan across until you find the target value (6), read the column header.

3. **Look at the Cayley table, row 2.** The entries are: 2, 3, 4, 5, 6, 7, 0, 1. Scan for the value 6. It appears under column 4.

4. **Read the column header.** Column 4. The token is element 4.

5. **Verify using the RotaryDial.** Set the dial to position 4. Press COMPOSE. The cell at row 2, column 4 highlights. Its value is 6. The display shows "OVERRIDE ACHIEVED." Confirmed.

6. **Map to the roster.** Element 4 = REEVES.

7. **Answer: element index 4** (REEVES).

Alternatively, the solver can use the formula from the reference card: TOKEN = (6 - 2) mod 8 = 4. Either approach works. The table lookup is more visual; the formula is faster.

---

## 5. Expert Solve Path

A solver comfortable with modular arithmetic:

1. Read: base = 2, target = 6.
2. Token = (6 - 2) mod 8 = 4.
3. Element 4 = REEVES. Done.

A solver who recognizes Z_8 needs no reference card at all. The group operation is addition mod 8. The unknown is trivially solvable.

Time for expert: under 10 seconds.

---

## 6. Data Values

### Composition Data

| Parameter | Value |
|-----------|-------|
| Group | Z_8 (integers 0-7 under addition mod 8) |
| Base Permission (known) | 2 (OKAFOR) |
| Authorization Token (unknown) | **4 (REEVES)** |
| Target Result | 6 (OVERRIDE) |
| Composition | (2 + 4) mod 8 = 6 |

### Full Cayley Table (Z_8 addition)

```
∘ │  0   1   2   3   4   5   6   7
──┼─────────────────────────────────
0 │  0   1   2   3   4   5   6   7
1 │  1   2   3   4   5   6   7   0
2 │  2   3   4   5  [6]  7   0   1    ← base row; [6] = target cell
3 │  3   4   5   6   7   0   1   2
4 │  4   5   6   7   0   1   2   3
5 │  5   6   7   0   1   2   3   4
6 │  6   7   0   1   2   3   4   5
7 │  7   0   1   2   3   4   5   6
```

### Answer

**Element index: 4** (REEVES)

---

## 7. Narrative Revelation

```
SEC-AUTH-03-C — AUTHORIZATION CHAIN LOG RECONSTRUCTION
TIMESTAMP: GAP +01:55

ACTION REQUESTED: SENSOR PURGE OVERRIDE — CLASSIFIED
AUTHORIZATION LEVEL REQUIRED: ELEMENT 6 (OVERRIDE)

INITIATING OFFICER: LT. OKAFOR, A. — NAV OFFICER — ELEMENT 2
AUTHORIZATION TOKEN COMPOSED: ELEMENT 4

COMPOSITION: 2 ∘ 4 = 6 (mod 8)
RESULT: OVERRIDE — GRANTED

ELEMENT 4 MAPS TO: LT. CMDR. REEVES, D. — CHIEF ENGINEER — POSITION 4
REEVES STATUS AT GAP +01:55: LOGGED AS OFF-DUTY

CHAIN OF EVENTS:
  GAP +01:22   Kwon collapses at COMPUTER station (R3-01)
  GAP +00:42   Chen accesses Sensor Bay Anteroom (R3-02)
  GAP +01:55   Okafor composes Reeves' token to achieve OVERRIDE

  Reeves was listed as off-duty. Reeves' authorization token was
  used anyway. Either Reeves provided the token before going
  off-shift — meaning the override was pre-planned — or the
  token was taken from Reeves' authorization cache without
  Reeves' knowledge.

  Either way, Reeves is in the chain. The override was not
  improvised. It was composed from specific permissions held by
  specific officers. This was planned.
```

---

## 8. Story Layer

The tension escalates. R3-01 showed a person in the wrong place (Kwon: not asleep). R3-02 showed a person at the wrong door (Chen: with the right code). Now R3-03 shows a person providing authorization they should not have had reason to give.

Reeves — the Chief Engineer, roster position 4 — provided the authorization token that, when composed with Okafor's base permission, unlocked the OVERRIDE command. This is the permission that enabled the sensor purge. Without Reeves' element 4, the composition fails: 2 + anything-other-than-4 does not equal 6 (mod 8).

Reeves was logged as off-duty. But Reeves' authorization token appeared in the chain at GAP +01:55. The token was either provided willingly (pre-planned cooperation) or extracted from Reeves' authorization cache (compromised credentials). Neither explanation is innocent.

The picture forming across Round 3: Kwon was neutralized (R3-01). Chen had the physical access (R3-02). Reeves provided the authorization (R3-03). Three officers, each contributing a necessary piece. None of them are senior enough to have ordered this alone. Someone coordinated them. The next two panels will make it clear who.
