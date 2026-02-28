# DEAD RECKONING v2 — Puzzle Reference Cards (Print Companion)

**Document type:** Physical reference cards for use alongside the game
**Format:** 3x5 index card size, two per letter page
**Total cards:** 20 (17 feeders + 3 round metas)
**Note:** The final meta (THE COMMISSION) has no print card — it uses round-meta outputs only.

---

## Printing Instructions

- Print double-sided on cardstock (110 lb / 200 gsm recommended)
- Cut to 3" x 5" (standard index card size)
- Two cards per letter page (8.5" x 11"), landscape orientation
- Front: Reference card text with diagram
- Back: Puzzle title, room location, and win condition

---

## ROUND 1 — TELEMETRY

### Card 01: R1-01 Carrier Isolation

**Front**

```
COMMS -- SIGNAL ISOLATION
Room: Communications Station (2-01-01), Deck 2

DISPLAY: Green waveform = filtered signal.
         Gold waveform = target carrier reference.
         Goal: make green match gold.

    signal
    ^       /\      /\      /\
    |  /\  /  \  /\/ >\  /\/ >
    | /  \/    \/      \/
    +-----------------------------> freq
      noise   CARRIER   harmonic
      1.75G   ???GHz    4.68GHz

CONTROLS:
  STEP SIZE dial: 100 / 10 / 1 MHz
  FILTER CENTER stepper: 1.000 - 5.000 GHz
  APPLY FILTER button: redraws display

METHOD: Sweep coarse (100MHz), then fine (10MHz),
then lock (1MHz). At the carrier frequency, green
wave aligns with gold overlay perfectly.
```

**Back**

```
R1-01 -- CARRIER ISOLATION
Communications Station, Deck 2
Round 1: TELEMETRY

Win condition: Filtered waveform matches target
carrier overlay. Read the FILTER CENTER value.

Answer format: frequency in GHz (3 decimal places)
```

---

### Card 02: R1-02 Phase Lock

**Front**

```
COMMS -- PHASE COMPARISON
Room: Science Lab (4-01-01), Deck 4

DISPLAY: Lissajous figure (X-Y oscilloscope).
  CH1 (X) = ship outbound: 291 MHz (fixed)
  CH2 (Y) = incoming signal (adjustable)
  Stable figure = exact frequency ratio.
  Drifting figure = frequencies not locked.

         +---+
        /     \    Count tangencies:
       |       |   Vertical edges = X ratio
       |       |   Horizontal edges = Y ratio
        \     /    3 vert, 1 horiz = Y is 3x X
         +---+

CONTROLS:
  CH2 stepper: 100 - 2000 MHz
  STEP SIZE dial: 10 / 1 / 0.1 MHz

METHOD: Sweep CH2 until figure stops rotating.
Drift indicator shows LOCKED when matched.
Count tangencies to verify ratio.
```

**Back**

```
R1-02 -- PHASE LOCK
Science Lab, Deck 4
Round 1: TELEMETRY

Win condition: Lissajous figure stabilizes.
Drift indicator reads LOCKED. Read CH2 frequency.

Answer format: frequency in MHz (integer)
```

---

### Card 03: R1-03 Null Zone

**Front**

```
TAC -- JAMMING ARRAY
Room: Tactical Control (2-01-02), Deck 2

DISPLAY: Interference heatmap with compass ring.
  Bright = constructive. Dark = null (cancel).
  Hidden target signal at unknown bearing.
  Interference source masking the target.

         0 deg
        . . . .
    270 .      . 90     Drag sources to move
        .      .        null zones. Place a
        . . . .         null on the interference
        180             to reveal the target.

CONTROLS:
  Drag alpha/beta sources on the heatmap
  Source frequency sliders: 100 - 1000 MHz
  SYMMETRY LOCK toggle (mirrors beta to alpha)

METHOD: Turn SYMMETRY LOCK on. Drag alpha until
a null falls on the interference. A red dot
appears at the target bearing. Read TARGET BEARING.
```

**Back**

```
R1-03 -- NULL ZONE
Tactical Control, Deck 2
Round 1: TELEMETRY

Win condition: Target signal emerges as bright
dot on compass ring. Read TARGET BEARING readout.

Answer format: bearing in degrees (integer)
```

---

### Card 04: R1-04 Orbit Classification

**Front**

```
NAV -- TRAJECTORY ANALYSIS
Room: Nav Control (2-01-03), Deck 2

DISPLAY: Cyan curve = theoretical trajectory.
  Orange dots (12) = observed position fixes.
  Goal: fit the curve through all dots.

    .  .  .                Conic sections:
     .      .              e = 0: circle
      .       .            0 < e < 1: ellipse
       .        .          e = 1: parabola
        .                  e > 1: hyperbola

CONTROLS:
  ECCENTRICITY stepper: 0.00 - 2.00
  e STEP dial: 0.10 / 0.01
  SEMI-LATUS RECTUM stepper: 1 - 100 km

METHOD: Sweep e at 0.10 steps until curve shape
matches dot arc. Fine-tune at 0.01. Adjust
semi-latus rectum to scale the curve. Match
indicator glows green when all 12 dots fit.
```

**Back**

```
R1-04 -- ORBIT CLASSIFICATION
Nav Control, Deck 2
Round 1: TELEMETRY

Win condition: Theoretical curve passes through
all 12 observed positions. Read ECCENTRICITY.

Answer format: eccentricity (2 decimal places)
```

---

### Card 05: R1-05 Contact Lock

**Front**

```
TAC -- CONTACT CLASSIFICATION
Room: Tactical Control (2-01-02), Deck 2

DISPLAY: Radar scope with 8 contacts (C1-C8).
  Targeting reticle (inset) locks on selected contact.

  REAL contact:  Lock = SOLID (green, steady)
                 Bearing stable (+/- 0.5 deg)
  DECOY contact: Lock = UNSTABLE (flickers)
                 Bearing drifts (+/- 5-15 deg)

    Radar         Reticle
    C3  C1        BRG: 213
      C2  C5      RNG: 60%
    C7  C4        LOCK: SOLID
      C6  C8

CONTROLS: Click contacts on radar to select.
  Classify buttons: OFF > GREEN (REAL) > RED (DECOY)

METHOD: Click each contact. Wait 2 sec for lock.
Observe 3-5 sec. Classify as REAL or DECOY.
Count total REAL contacts.
```

**Back**

```
R1-05 -- CONTACT LOCK
Tactical Control, Deck 2
Round 1: TELEMETRY

Win condition: All 8 contacts classified.
Read REAL COUNT from the display.

Answer format: count of real contacts (integer)
```

---

### Card 06: R1-06 Signal Fingerprint

**Front**

```
COMMS -- SIGNAL ANALYSIS
Room: Communications Station (2-01-01), Deck 2

DISPLAY: IQ constellation diagram.
  Cyan dots = received symbols. Gold circles = ideal.
  Count clusters to identify modulation:
    2 clusters on line  = BPSK (1 bit/sym)
    4 clusters square   = QPSK (2 bits/sym)
    8 clusters ring     = 8-PSK (3 bits/sym)
    16 clusters grid    = 16-QAM (4 bits/sym)

  EVM < 15% = correct modulation type
  EVM > 30% = wrong modulation type

CONTROLS:
  MODULATION dial: BPSK / QPSK / 8-PSK / 16-QAM
  SYMBOL RATE stepper: 1 - 1000 kSym/s

METHOD: Count dot clusters. Select modulation.
Check EVM drops below 15%. Adjust symbol rate
until waveform inset shows clean transitions.
Read DATA RATE (= symbol rate x bits/symbol).
```

**Back**

```
R1-06 -- SIGNAL FINGERPRINT
Communications Station, Deck 2
Round 1: TELEMETRY

Win condition: Correct modulation and symbol rate.
DATA RATE readout displays the answer.

Answer format: data rate in kbps (integer)
```

---

### Card 07: R1-META Contact

**Front**

```
SCIENCE -- CONTACT SIGNATURE CLASSIFICATION
Room: Bridge Science Station (1-01-05), Deck 1

DISPLAY: Six feeder values from Round 1.
  Contact Signature Reference Table (8 types).
  Each type has 6 parameter ranges.

  YOUR VALUES:         Match each to the table:
  Carrier freq         -> Freq Band column
  Harmonic ratio       -> Harmonic column
  Target bearing       -> Bearing column
  Trajectory eccent.   -> Trajectory column
  Real contacts        -> Formation column
  Data rate            -> Data Rate column

METHOD:
  1. Check which table rows match your first value
  2. Check which of those match your second value
  3. Continue until one row matches ALL SIX
  4. Read its CLASS CODE from the rightmost column
  5. Enter the class code as the answer

NOTE: No two contact types match on more than
2 of 6 parameters. The correct match is unique.
```

**Back**

```
R1-META -- CONTACT
Bridge Science Station, Deck 1
Round 1 Meta: WHAT was encountered?

Win condition: All six values matched to one
contact type. Enter its CLASS CODE.

Answer format: class code (integer)
Output: Sets N (number of points) on the
commission decoder circle.
```

---

## ROUND 2 — SYSTEMS LOG

### Card 08: R2-01 Power Path

**Front**

```
EPS -- CONDUIT ROUTING
Room: Main Engineering Power (5-01-01), Deck 5

DISPLAY: 3 conduits feed through junction JCT-5A
  to 3 destinations. Gauges show power delivery.
  Green zone = operational. Red = failed.

  [EPS-PRI]---+
  [EPS-SEC]---JCT-5A---[SHIELDS]
  [DATA-ODN]--+         [IMPULSE]
                        [LIFE SUPPORT]

  JUNCTION LIMIT: Max 2 conduits at once.
  With 3 active: junction overloads, one drops.

CONTROLS: 3 toggle switches (one per conduit)

METHOD: Start with all 3 on (junction overloads).
Try each 2-conduit combination:
  PRI + SEC:  check all 3 gauges
  PRI + ODN:  check all 3 gauges
  SEC + ODN:  check all 3 gauges
Only one puts ALL THREE gauges in green.
Count the active conduits.
```

**Back**

```
R2-01 -- POWER PATH
Main Engineering, Deck 5
Round 2: SYSTEMS LOG

Win condition: All three destination gauges in
green zone simultaneously. No junction overload.

Answer format: count of active conduits (integer)
```

---

### Card 09: R2-02 Data Breach

**Front**

```
ODN -- NETWORK ISOLATION
Room: Main Eng Computer (5-01-03), Deck 5

DISPLAY: 15-node network. Blue packets = normal.
  Red packet = rogue data on a fixed route.

  Toggle nodes OFF to trace the rogue packet:
    Node OFF, red packet still flows
      -> that node is NOT on the route
    Node OFF, red packet STOPS at another node
      -> disabled node is downstream of stop
    Node OFF, red packet NEVER APPEARS
      -> that node is the ENTRY POINT

    [1]---[2]---[4]---[7]---[9]
     |     |          |     [10]
    [3]   [5]---[7]
     |           |
    [6]---[12]  [9]---[14]

CONTROLS: 15 toggle switches (one per node)

METHOD: Start from outer nodes, work inward.
Find the one node whose disabling prevents
the red packet from ever appearing.
That node ID is the answer.
```

**Back**

```
R2-02 -- DATA BREACH
Main Engineering Computer, Deck 5
Round 2: SYSTEMS LOG

Win condition: Identify the node where disabling
it prevents the rogue packet from appearing.

Answer format: node ID number (integer)
```

---

### Card 10: R2-03 Heat Source

**Front**

```
ENVIRO -- DECK THERMAL MONITOR
Room: Environmental Control (5-02-02), Deck 5

DISPLAY: 5x5 thermal grid. Colors: blue (cool)
  to red (hot). A heat source has diffused outward.
  Multiple cells are warm. Source is hidden in
  the gradient.

  +----+----+----+----+----+
  | 1  | 2  | 3  | 4  | 5  |  Place COOLANT at
  +----+----+----+----+----+  edges of the warm
  | 6  | 7  | 8  | 9  |10  |  area to absorb
  +----+----+----+----+----+  diffused heat.
  |11  |12  |13  |14* |15  |
  +----+----+----+----+----+  The source keeps
  |16  |17  |18  |19  |20  |  generating heat
  +----+----+----+----+----+  and stays warm.
  |21  |22  |23  |24  |25  |
  +----+----+----+----+----+

CONTROLS: Click cell + PLACE COOLANT (max 6)

METHOD: Place coolant at warm edges (not center).
As peripheral heat is absorbed, warm area shrinks.
Last remaining hot cell = source. Read PEAK AT.
```

**Back**

```
R2-03 -- HEAT SOURCE
Environmental Control, Deck 5
Round 2: SYSTEMS LOG

Win condition: Peak temperature isolated to a
single grid coordinate. Read PEAK AT readout.

Answer format: grid coordinate number (integer)
```

---

### Card 11: R2-04 Shield Profile

**Front**

```
TAC -- SHIELD CONFIGURATION
Room: Bridge Tactical (1-01-03), Deck 1

DISPLAY: 3-layer hexagonal shield with damage.
  White overlay = reference damage from the gap.
  Your simulated damage changes with sliders.

  REFERENCE PATTERN:
    Layer 1 (Outer):  78% damage  HEAVY
    Layer 2 (Middle): 42% damage  MODERATE
    Layer 3 (Inner):  14% damage  LIGHT

  Higher power = less damage (shield absorbs more)
  Lower power  = more damage (attack passes through)

CONTROLS: 3 PowerSliders (Layer 1/2/3 power %)

METHOD: Heavy damage = weak layer (set LOW).
  Light damage = strong layer (set HIGH).
  Set L1 low, L3 high, L2 in between.
  Fine-tune until all 3 damage indicators match
  the white reference overlay. Match tolerance: +/-2%.
  Read the Layer 2 slider value.
```

**Back**

```
R2-04 -- SHIELD PROFILE
Bridge Tactical, Deck 1
Round 2: SYSTEMS LOG

Win condition: All three damage indicators match
the reference overlay. Read Layer 2 power %.

Answer format: power percentage (integer)
```

---

### Card 12: R2-05 Fault Trace

**Front**

```
ENG -- EPS CIRCUIT DIAGNOSTIC
Room: EPS Control (5-02-01), Deck 5

DISPLAY: 20-node EPS circuit (tree structure).
  Current flows from SOURCE (node 1) to 5 dest.
  One component FAILED -- current stops there.

            [1]
           / \            PROBE a node:
         [2]  [3]         Green = current present
        / \     \         Red = no current
      [4] [5]  [6]
       |    |   / \       The FAILED node is where
      [7] [8] [12][13]   current enters but does
      / \   |   |   |    NOT exit (green parent,
    [9][10][11][16][17]   red child).
     |   |   |
   [14][15][18]
    |    |
   [19][20]

CONTROLS: 20 PROBE buttons (one per node)

METHOD: Probe destinations to find dead branch.
Binary search: probe mid-path nodes.
Green = fault is downstream. Red = fault is upstream.
Find adjacent green-red pair. Green node = answer.
```

**Back**

```
R2-05 -- FAULT TRACE
EPS Control, Deck 5
Round 2: SYSTEMS LOG

Win condition: Identify the failed component --
the node where current enters but does not exit.

Answer format: node ID number (integer)
```

---

### Card 13: R2-06 Reactor State

**Front**

```
POWER -- REACTOR OPERATIONS
Room: Main Engineering Power (5-01-01), Deck 5

DISPLAY: 4 gauges with white setpoint markers.
  TEMPERATURE: logged value (deg C)
  PRESSURE:    logged value (MPa)
  COOLANT:     logged value (L/s)
  OUTPUT:      logged value (MW)

  Needle LEFT of marker = too low (increase)
  Needle RIGHT of marker = too high (decrease)

CONTROLS:
  THROTTLE lever: 0-8 detent positions (coarse)
  CONTAINMENT slider: 0-100% (fine-tune)

METHOD:
  1. Move throttle up from 0. Watch gauges.
  2. Find the detent where gauges are CLOSE
     to the setpoints (within the right range).
  3. Fine-tune with containment slider until
     all 4 needles sit on the white markers.
  4. Read the THROTTLE POSITION (detent number).
```

**Back**

```
R2-06 -- REACTOR STATE
Main Engineering, Deck 5
Round 2: SYSTEMS LOG

Win condition: All four gauge needles aligned
with setpoint markers. Throttle in a detent.

Answer format: throttle position (integer 0-8)
```

---

### Card 14: R2-META Response

**Front**

```
COMMAND -- STATION COMMAND LOG
Room: Bridge Center (1-01-01), Deck 1

DISPLAY: Six Round 2 values with parity tags.
  EVEN value = NOMINAL (system normal)
  ODD value  = ANOMALY (irregular state)

  Station Command Log table maps each system
  to its controlling bridge station and the
  station officer's duty roster position.

METHOD:
  1. Identify the ODD-parity (ANOMALY) values
  2. Find the LOWEST odd value among them
  3. Look up which system that value belongs to
  4. Look up which station controls that system
  5. Read the station's DUTY ROSTER POSITION

  FIRST RESPONSE PROTOCOL: Lowest anomalous
  value = first alert. Lower node/component IDs
  are primary sensors -- they detect first.

  Enter the roster position of the first-
  responder station.
```

**Back**

```
R2-META -- RESPONSE
Bridge Center, Deck 1
Round 2 Meta: HOW did the ship respond?

Win condition: First-alert station identified.
Enter its duty roster position.

Answer format: roster position (integer 0-7)
Output: Sets starting position on the
commission decoder circle.
```

---

## ROUND 3 — CREW RECORD

### Card 15: R3-01 Triage

**Front**

```
MEDICAL -- LIFESIGNS MONITOR
Room: Sickbay Main (3-01-01), Deck 3

DISPLAY: 3 ECG channels (3 patients).
  Each shows a morphing waveform at a heart rate.

  NORMAL SINUS (60-100 BPM):
    Narrow peaks. P-waves visible.

  SINUS TACHYCARDIA (100-150 BPM):
    Narrow peaks (fast). P-waves present.
    Stressed, NOT cardiac crisis.

  V-TACH (>150 BPM):
    WIDE, blobby peaks. NO P-waves.
    Cardiac crisis. Life-threatening.

  KEY: Peak WIDTH is the diagnostic marker.
    Narrow peaks = sinus (normal pathway)
    Wide peaks = V-tach (abnormal pathway)

CONTROLS:
  Patient selector dial (P1/P2/P3)
  Heart rate slider: 60-200 BPM

METHOD: Observe each patient's waveform.
Find the one with WIDE QRS and NO P-waves.
That patient ID is the answer.
```

**Back**

```
R3-01 -- TRIAGE
Sickbay, Deck 3
Round 3: CREW RECORD

Win condition: Identify the patient in
cardiac crisis (V-tach). Read PATIENT ID.

Answer format: patient number (integer 1-3)
```

---

### Card 16: R3-02 Access Code

**Front**

```
SECURITY -- MODULAR KEY INTERFACE
Room: Security Office (4-01-02), Deck 4

DISPLAY: 8-position clock (0-7) with pointer.
  Positions labeled with duty roster names.
  Pointer starts at position 0.

        0:TORRES
       /         \
  7:CHEN          1:NAKAMURA
  |        [ptr]        |
  6:PARK          2:OKAFOR
       \         /
        4:REEVES    3:KWON    5:VASQUEZ

ROTATION: new = (current + rotation) mod 8
  If sum > 7, subtract 8.

ACCESS SEQUENCE LOG shows rotation amounts.
Apply each rotation in order.

CONTROLS:
  Rotation stepper: 0-7
  APPLY ROTATION button

METHOD: Set stepper to first rotation amount.
Press APPLY. Repeat for each entry in the log.
Read the FINAL POSITION after all rotations.
```

**Back**

```
R3-02 -- ACCESS CODE
Security Office, Deck 4
Round 3: CREW RECORD

Win condition: All rotations applied. Read
the final clock position.

Answer format: clock position (integer 0-7)
Shortcut: sum all rotations, take mod 8.
```

---

### Card 17: R3-03 Permission Chain

**Front**

```
SECURITY -- PERMISSION COMPOSITION
Room: Computer Core (5-03-01), Deck 5

DISPLAY: Cayley table (8x8) for Z_8 addition.
  BASE permission = known (fixed, shown on panel)
  TARGET result = known (shown on panel)
  AUTH TOKEN = unknown (you find this)

  Composition: RESULT = (BASE + TOKEN) mod 8

  +  | 0  1  2  3  4  5  6  7
  ---+-------------------------
  0  | 0  1  2  3  4  5  6  7
  1  | 1  2  3  4  5  6  7  0
  2  | 2  3  4  5 [6] 7  0  1  <- base row
  ...

METHOD:
  1. Find BASE on the left (row header)
  2. Scan across that row for the TARGET value
  3. Read the column header = TOKEN
  Or: TOKEN = (TARGET - BASE) mod 8

  Each element maps to a roster position:
  0=TORRES 1=NAKAMURA 2=OKAFOR 3=KWON
  4=REEVES 5=VASQUEZ  6=PARK   7=CHEN
```

**Back**

```
R3-03 -- PERMISSION CHAIN
Computer Core, Deck 5
Round 3: CREW RECORD

Win condition: Identify the authorization token
that composes with the base to reach the target.

Answer format: element index (integer 0-7)
```

---

### Card 18: R3-04 Badge Sequence

**Front**

```
SECURITY -- BADGE SWIPE LOG
Room: Security Office (4-01-02), Deck 4

DISPLAY: 18 LEDs in a row (badge swipes).
  Green = GRANTED (normal access)
  Red   = DENIED (insufficient clearance)
  Amber = FORCED (command override)

  FORCED = only CO or XO can do this.
  Multiple FORCED entries with same badge ID
  = one senior officer moving through
  restricted areas by command authority.

  1  2  3  4 [5] 6  7  8  9 10[11]12 13 14[15]16 17 18
  G  G  D  G  F  G  G  D  G  G  F  G  D  G  F  G  G  G

CONTROLS: Click any LED to see swipe details
  (badge ID, door name, time, result).

METHOD: Find the AMBER (FORCED) entries.
Note the badge ID. Check if all FORCED entries
share the same badge. The POSITION of the
FIRST forced entry in the sequence is the answer.
```

**Back**

```
R3-04 -- BADGE SEQUENCE
Security Office, Deck 4
Round 3: CREW RECORD

Win condition: Identify the first FORCED entry.
Read its position number in the sequence.

Answer format: sequence position (integer 1-18)
```

---

### Card 19: R3-05 Emergency Sequence

**Front**

```
ENGINEERING -- EMERGENCY PROCEDURE
Room: Deflector Control (5-02-03), Deck 5

DISPLAY: 4 guarded switches (A-D) with LEDs.
  A = THERMAL  B = MAGNETIC  C = COMM  D = SENSOR
  Each has a red safety guard (must lift first).
  Procedure register counts every operation.

  CORRECT SEQUENCE (12 operations):
  Phase 1 - Engage:
    Lift A, flip A ON, lift B, flip B ON,
    lift C, flip C ON, lift D, flip D ON  (8 ops)
  Phase 2 - Sensor re-secure:
    Flip D OFF, close guard D             (2 ops)
  Phase 3 - Stow:
    Close guard A, close guard B          (2 ops)

  Wrong order triggers MASTER ALARM (resets all).

CONTROLS: Click guards to lift/close.
  Click switches to flip ON/OFF.

METHOD: Follow the 12-step sequence exactly.
Read the PROCEDURE REGISTER when complete.
```

**Back**

```
R3-05 -- EMERGENCY SEQUENCE
Deflector Control, Deck 5
Round 3: CREW RECORD

Win condition: All 12 operations completed in
correct order. AutoProcedure button unlocks.

Answer format: procedure register count (integer)
The 12-op variant = CO confirmation bypass.
```

---

### Card 20: R3-META Cover-Up

**Front**

```
SECURITY -- AUTHORIZATION CHAIN ANALYSIS
Room: Security Office (4-01-02), Deck 4

DISPLAY: Five Round 3 values mapped to crew.
  Each value is a duty roster index (mod 8).

  Map values to crew:
    R3-01: ___ -> position ___ = ___________
    R3-02: ___ -> position ___ = ___________
    R3-03: ___ -> position ___ = ___________
    R3-04: ___ -> position ___ = ___________
    R3-05: ___ -> mod 8 = ___ = ___________

  Identify roles:
    Who was incapacitated? (casualty)
    Who had physical access? (agent)
    Who provided authorization? (enabler)
    Who invoked command authority? (principal)

METHOD:
  1. Map all 5 values to roster positions
  2. Note which officer appears TWICE
  3. Identify the ordering officer (CO authority)
  4. Identify the executing officer
  5. Count AUTHORIZATION LINKS between them
  Enter the chain length.
```

**Back**

```
R3-META -- COVER-UP
Security Office, Deck 4
Round 3 Meta: WHO ordered the cover-up?

Win condition: Authorization chain traced.
Enter the chain length (number of links).

Answer format: chain length (integer)
Output: Sets step size on the commission
decoder circle.
```

---

## CARD LAYOUT GUIDE (for printing)

### Page 1 (Letter, landscape)
- Top: Card 01 (R1-01 Carrier Isolation)
- Bottom: Card 02 (R1-02 Phase Lock)

### Page 2
- Top: Card 03 (R1-03 Null Zone)
- Bottom: Card 04 (R1-04 Orbit Classification)

### Page 3
- Top: Card 05 (R1-05 Contact Lock)
- Bottom: Card 06 (R1-06 Signal Fingerprint)

### Page 4
- Top: Card 07 (R1-META Contact)
- Bottom: Card 08 (R2-01 Power Path)

### Page 5
- Top: Card 09 (R2-02 Data Breach)
- Bottom: Card 10 (R2-03 Heat Source)

### Page 6
- Top: Card 11 (R2-04 Shield Profile)
- Bottom: Card 12 (R2-05 Fault Trace)

### Page 7
- Top: Card 13 (R2-06 Reactor State)
- Bottom: Card 14 (R2-META Response)

### Page 8
- Top: Card 15 (R3-01 Triage)
- Bottom: Card 16 (R3-02 Access Code)

### Page 9
- Top: Card 17 (R3-03 Permission Chain)
- Bottom: Card 18 (R3-04 Badge Sequence)

### Page 10
- Top: Card 19 (R3-05 Emergency Sequence)
- Bottom: Card 20 (R3-META Cover-Up)
