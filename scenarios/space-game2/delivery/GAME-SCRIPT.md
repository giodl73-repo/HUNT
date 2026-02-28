# DEAD RECKONING v2 — Game Architecture & Scene Script

**Document type:** Blueprint for React/TypeScript game developer
**Audience:** The person building the interactive experience
**Covers:** Ship architecture, navigation system, full scene script, state machine, multi-ship future

---

## Section 1: World Architecture Overview

### Ship Identity

| Field | Value |
|-------|-------|
| Class | Cruiser |
| Decks | 5 |
| Rooms | ~28 navigable spaces |
| Divisions | 4 (RED Command, YELLOW Operations, ORANGE Engineering, BLUE Science) |
| Departments | 16 |
| Power | Fusion reactor (primary), M/AM reactor (warp), Battery (backup) |
| Computer Nodes | 15 on the ODN (Optical Data Network) |
| Crew (named) | 8 bridge officers on duty roster |

### Deck Layout

#### Deck 1 — COMMAND (Division: RED)

| Room ID | Room Name | Consoles | Puzzle Assignments |
|---------|-----------|----------|--------------------|
| 1-01-01 | Bridge — Center | COMMAND_CHAIR (x1) | R2-META (Station Command Log Console) |
| 1-01-02 | Bridge — Helm | HELM (x1) | — |
| 1-01-03 | Bridge — Tactical | TACTICAL (x1) | R2-04 Shield Profile |
| 1-01-04 | Bridge — OPS | OPS (x1) | — |
| 1-01-05 | Bridge — Science | SCIENCE (x1) | R1-META (Contact Signature Classification Console) |
| 1-02-01 | Ready Room | AUXILIARY (x1) | — |
| 1-02-02 | Briefing Room | AUXILIARY (x4) | FINAL META (Commission Briefing — CyclicGroupDisplay) |
| 1-03-01 | Observation Lounge | — | Epilogue scene |

#### Deck 2 — OPERATIONS (Division: YELLOW)

| Room ID | Room Name | Consoles | Puzzle Assignments |
|---------|-----------|----------|--------------------|
| 2-01-01 | Communications Station | COMMS (x2) | R1-01 Carrier Isolation, R1-06 Signal Fingerprint |
| 2-01-02 | Tactical Control | TACTICAL (x1) | R1-03 Null Zone, R1-05 Contact Lock |
| 2-01-03 | Nav Control | HELM (x1) | R1-04 Orbit Classification |
| 2-02-01 | Transporter Room 1 | TRANSPORTER (x2) | — |
| 2-02-02 | Transporter Room 2 | TRANSPORTER (x2) | — |

#### Deck 3 — CREW (Division: GREEN)

| Room ID | Room Name | Consoles | Puzzle Assignments |
|---------|-----------|----------|--------------------|
| 3-01-01 | Sickbay — Main | MEDICAL (x3) | R3-01 Triage |
| 3-01-02 | Senior Quarters | AUXILIARY (x1) | — |
| 3-02-01 | Crew Quarters | AUXILIARY (x1) | — |
| 3-03-01 | Mess Hall | AUXILIARY (x2) | — |
| 3-03-02 | Recreation | — | — |

#### Deck 4 — SUPPORT (Division: BLUE)

| Room ID | Room Name | Consoles | Puzzle Assignments |
|---------|-----------|----------|--------------------|
| 4-01-01 | Science Lab | SCIENCE (x4) | R1-02 Phase Lock |
| 4-01-02 | Security Office | SECURITY (x3) | R3-02 Access Code, R3-04 Badge Sequence, R3-META Cover-Up |
| 4-02-01 | Armory | SECURITY (x1) | — |
| 4-02-02 | Brig | SECURITY (x1) | — |
| 4-03-01 | Cargo Bay | OPS (x1) | — |

#### Deck 5 — ENGINEERING (Division: ORANGE)

| Room ID | Room Name | Consoles | Puzzle Assignments |
|---------|-----------|----------|--------------------|
| 5-01-01 | Main Engineering — Power | ENGINEERING (x4) | R2-01 Power Path, R2-06 Reactor State |
| 5-01-02 | Main Engineering — Propulsion | ENGINEERING (x4) | — |
| 5-01-03 | Main Engineering — Computer | ENGINEERING (x3) | R2-02 Data Breach |
| 5-01-04 | Main Engineering — Damage Control | ENGINEERING (x3) | — |
| 5-02-01 | EPS Control | ENGINEERING (x2) | R2-05 Fault Trace |
| 5-02-02 | Environmental Control | ENVIRONMENTAL (x2) | R2-03 Heat Source |
| 5-02-03 | Deflector Control | ENGINEERING (x2) | R3-05 Emergency Sequence |
| 5-03-01 | Computer Core | — | R3-03 Permission Chain |

### Complete Puzzle-to-Room Map

| Puzzle | Title | Room | Deck | Console/Panel Name |
|--------|-------|------|------|--------------------|
| R1-01 | Carrier Isolation | 2-01-01 Communications Station | 2 | Bandpass Filter Console |
| R1-02 | Phase Lock | 4-01-01 Science Lab | 4 | Lissajous Display Console |
| R1-03 | Null Zone | 2-01-02 Tactical Control | 2 | Phased Array Jamming Console |
| R1-04 | Orbit Classification | 2-01-03 Nav Control | 2 | Trajectory Analysis Console |
| R1-05 | Contact Lock | 2-01-02 Tactical Control | 2 | Tactical Radar Console |
| R1-06 | Signal Fingerprint | 2-01-01 Communications Station | 2 | Vector Signal Analyzer Console |
| R1-META | Contact | 1-01-05 Bridge — Science | 1 | Contact Signature Classification Console |
| R2-01 | Power Path | 5-01-01 Main Eng — Power | 5 | EPS Conduit Routing Console |
| R2-02 | Data Breach | 5-01-03 Main Eng — Computer | 5 | ODN Isolation Console |
| R2-03 | Heat Source | 5-02-02 Environmental Control | 5 | Deck Thermal Monitoring Console |
| R2-04 | Shield Profile | 1-01-03 Bridge — Tactical | 1 | Shield Configuration Console |
| R2-05 | Fault Trace | 5-02-01 EPS Control | 5 | EPS Circuit Diagnostic Console |
| R2-06 | Reactor State | 5-01-01 Main Eng — Power | 5 | Reactor Operations Console |
| R2-META | Response | 1-01-01 Bridge — Center | 1 | Station Command Log Console |
| R3-01 | Triage | 3-01-01 Sickbay — Main | 3 | LifesignsDisplay ECG Monitor |
| R3-02 | Access Code | 4-01-02 Security Office | 4 | Modular Key Rotation Interface |
| R3-03 | Permission Chain | 5-03-01 Computer Core | 5 | Group Composition Interface |
| R3-04 | Badge Sequence | 4-01-02 Security Office | 4 | Badge Swipe Log Display |
| R3-05 | Emergency Sequence | 5-02-03 Deflector Control | 5 | Guarded Emergency Procedure Console |
| R3-META | Cover-Up | 4-01-02 Security Office | 4 | Authorization Chain Analysis Console |
| FINAL | The Commission | 1-02-02 Briefing Room | 1 | CyclicGroupDisplay Commission Decoder |

### Duty Roster (8 Bridge Officers)

The CyclicGroupDisplay uses these labels. The duty roster is canonical — it appears on multiple consoles and is the key to the final meta.

| Position | Name | Station | Rank | Division |
|----------|------|---------|------|----------|
| 0 | TORRES | OPS | Lt. Cmdr. | YELLOW |
| 1 | NAKAMURA | TAC | Lt. | RED |
| 2 | OKAFOR | HELM | Lt. | RED |
| 3 | KWON | COMPUTER | Lt. | ORANGE |
| 4 | REEVES | ENG | Lt. Cmdr. | ORANGE |
| 5 | VASQUEZ | CMD | Capt. | RED |
| 6 | PARK | ENVIRO | Lt. | YELLOW |
| 7 | CHEN | COMMS | Ens. | YELLOW |

### ODN Topology (15 Nodes)

The ODN is the ship's data nervous system. Node numbering matters for R2-02 (Data Breach). The 15 nodes are:

| Node ID | Computer | Department | Deck |
|---------|----------|------------|------|
| 0 | Primary Computer | CMD | 5 |
| 1 | Secondary Computer | CMD | 5 |
| 2 | Navigation Computer | NAV | 1 |
| 3 | Tactical Computer | TAC | 1 |
| 4 | Comms Computer | OPS | 2 |
| 5 | Environmental Computer | OPS | 5 |
| 6 | Engine Computer | ENG | 5 |
| 7 | Security Computer | SEC | 4 |
| 8 | Science Computer | SCI | 4 |
| 9 | Replicator Computer | OPS | 3 |
| 10 | Transporter Computer | OPS | 2 |
| 11 | Holographic Computer | OPS | 3 |
| 12 | Turbolift Computer | ENG | 5 |
| 13 | Diagnostic Computer | ENG | 5 |
| 14 | Sensor Computer | NAV | 1 |

### Power Architecture

```
BATTERY -> Electrical Grid (kW)
  Runs: computer, comms, sensors, weapons fire control, lighting,
        environmental chemistry, thrusters, fusion reactor startup

FUSION REACTOR -> PCU -> EPS Grid (MW)
  Runs: impulse, shields, env fields, transporter, replicator,
        M/AM reactor startup

M/AM REACTOR -> Warp EPS
  Runs: warp drive, nav deflector
  Also bridges to PCU for supplemental ship power
```

### System States

Six states visible on every status board:

| State | Meaning | Visual |
|-------|---------|--------|
| ONLINE | Running (ACTIVE = working, IDLE = monitoring) | Green |
| READY | Power available, can activate | Blue |
| STANDBY | Was running, instant wake | Amber |
| LOCKED | Power available but restricted | Red outline |
| OFFLINE | No power, dependency not met | Grey |
| FAULT | Hardware failure | Red flash |

---

## Section 2: Navigation System

### Turbolift Network

The turbolift is the primary navigation mechanism. Every deck has at least one turbolift stop. The turbolift interface shows available destinations; locked destinations appear greyed out with a lock icon.

| Turbolift Stop | Deck | Rooms Accessible |
|----------------|------|------------------|
| BRIDGE | 1 | Bridge (all stations), Ready Room, Briefing Room, Observation Lounge |
| OPERATIONS | 2 | Communications Station, Tactical Control, Nav Control, Transporter Rooms |
| CREW | 3 | Sickbay, Quarters, Mess Hall, Recreation |
| SUPPORT | 4 | Science Lab, Security Office, Armory, Brig, Cargo Bay |
| ENGINEERING | 5 | Main Engineering (all sections), EPS Control, Environmental Control, Deflector Control, Computer Core |

### Corridor Navigation

Within each deck, rooms are connected by corridors. The player moves between rooms on the same deck without the turbolift. Movement is point-and-click on a simplified deck map.

```
Deck 1 (Command):
  Bridge Center <-> Bridge Helm <-> Bridge Tactical
  Bridge Center <-> Bridge OPS <-> Bridge Science
  Bridge Center --> Ready Room (door, always open)
  Bridge Center --> Briefing Room (door, locked until Act 3)
  Briefing Room --> Observation Lounge (door, locked until Epilogue)

Deck 2 (Operations):
  Communications Station <-> Tactical Control <-> Nav Control
  Communications Station --> Transporter Room 1 (corridor)
  Tactical Control --> Transporter Room 2 (corridor)

Deck 3 (Crew):
  Sickbay <-> Crew Quarters <-> Senior Quarters
  Crew Quarters <-> Mess Hall <-> Recreation

Deck 4 (Support):
  Science Lab <-> Security Office <-> Armory
  Security Office --> Brig (secured door, opens Round 3)
  Science Lab --> Cargo Bay (corridor)

Deck 5 (Engineering):
  Main Eng Power <-> Main Eng Propulsion <-> Main Eng Computer
  Main Eng Computer <-> Main Eng Damage Control
  Main Eng Power --> EPS Control (corridor)
  Main Eng Computer --> Computer Core (secured door)
  EPS Control --> Environmental Control (corridor)
  Environmental Control --> Deflector Control (corridor)
```

### Door States

Doors control puzzle accessibility. Each door has one of four states:

| State | Visual | Player Action |
|-------|--------|---------------|
| OPEN | Green frame, open animation | Walk through |
| LOCKED | Red frame, lock icon | Cannot enter; tooltip shows unlock condition |
| SECURED | Amber frame, keypad icon | Requires authorization event (story-driven unlock) |
| FORCED | Red frame, damage marks | Flavor only — shows someone forced entry during gap |

### Navigation State Machine

The game progresses through phases. Each phase unlocks new decks, rooms, and puzzles.

| Phase | Trigger | Unlocks | Locks |
|-------|---------|---------|-------|
| OPENING | Game start | Deck 1: Bridge (Center, Helm, OPS), Ready Room | All other rooms |
| COMMISSION | Complete opening cutscene | Turbolift to all 5 decks; all Round 1 rooms | Briefing Room, Observation Lounge |
| ROUND_1 | Commission briefing ends | R1 puzzle rooms: 2-01-01, 2-01-02, 2-01-03, 4-01-01 | R1-META room until 4+ R1 feeders solved |
| R1_META | 4 of 6 R1 feeders solved | 1-01-05 (Bridge Science) for R1-META | — |
| ROUND_2 | R1-META solved (N=8 accepted) | R2 puzzle rooms: 5-01-01, 5-01-03, 5-02-01, 5-02-02, 1-01-03 | R2-META room until 4+ R2 feeders solved |
| R2_META | 4 of 6 R2 feeders solved | 1-01-01 (Bridge Center) for R2-META | — |
| ROUND_3 | R2-META solved (start=3 accepted) | R3 puzzle rooms: 3-01-01, 4-01-02, 5-03-01, 5-02-03 | R3-META room until 3+ R3 feeders solved |
| R3_META | 3 of 5 R3 feeders solved | 4-01-02 (Security Office) for R3-META | — |
| FINAL | R3-META solved (step=2 accepted) | 1-02-02 (Briefing Room) for final meta | — |
| EPILOGUE | Final meta solved (VASQUEZ named) | 1-03-01 (Observation Lounge) | All puzzle rooms (soft lock — revisitable but greyed) |

### Locked Door Schedule

| Door | Initial State | Unlocks At | Condition |
|------|--------------|------------|-----------|
| Briefing Room (1-02-02) | LOCKED | FINAL phase | All 3 round metas solved |
| Observation Lounge (1-03-01) | LOCKED | EPILOGUE phase | Final meta solved |
| Brig (4-02-02) | SECURED | ROUND_3 phase | R2-META solved |
| Computer Core (5-03-01) | SECURED | ROUND_3 phase | R2-META solved |
| Bridge Science (1-01-05) | LOCKED | R1_META phase | 4+ R1 feeders solved |
| Bridge Center CMD console | LOCKED | R2_META phase | 4+ R2 feeders solved |
| Security Office AUTH console | LOCKED | R3_META phase | 3+ R3 feeders solved |

### Map UI

Each deck is a top-down 2D map. Rooms are labeled rectangles. Corridors are lines connecting them. The player clicks a room to move there (if accessible). Inaccessible rooms are darkened with a lock overlay.

The turbolift appears as a button panel at the edge of each deck map. Clicking a deck number transitions (with a brief elevator animation) to that deck's map.

A minimap in the corner shows all 5 decks as colored strips (RED/YELLOW/GREEN/BLUE/ORANGE) with a dot for the player's current position.

---

## Section 3: Full Scene Script

### Notation

Each scene specifies:
- **ID**: Unique scene identifier
- **Location**: Room where the scene plays
- **Trigger**: What causes the scene to start
- **Display**: What the player sees (console text, panel state, ambient visuals)
- **Audio**: Sound design notes (ambient, SFX, music)
- **Duration**: How long the scene plays before player regains control
- **Exits**: What the player can do next

---

### ACT 1 — TELEMETRY (Scenes 1-12)

*The signals from outside are too deliberate. Someone was waiting for you.*

---

#### Scene 1: COLD OPEN

**ID:** `ACT1_COLD_OPEN`
**Location:** 1-01-01 Bridge — Center
**Trigger:** Game start (new game)
**Display:**

The bridge is dim. Emergency lighting only — amber strips along the floor, red status lights on every console. The main viewscreen shows stars, static, then goes dark. Three lines of text appear on the center console:

```
SHIP STATUS: CONDITION YELLOW
MAIN TELEMETRY LOG: DELETED — 0347 UTC
BACKUP INSTRUMENTS: RECORDING — 6HR GAP DETECTED
```

A pause. Then:

```
COMMISSION ORDER #2247
OFFICER ASSIGNED: [PLAYER]
CLASSIFICATION: RESTRICTED
REPORT TO BRIEFING — DECK 1
```

**Audio:** Low reactor hum. Distant ventilation. A single console chime. No music.
**Duration:** 12 seconds (text appears line by line, 2 seconds per line)
**Exits:** Ready Room door illuminates green. Player clicks to proceed.

---

#### Scene 2: THE BRIEFING (Commission Assignment)

**ID:** `ACT1_BRIEFING`
**Location:** 1-02-01 Ready Room
**Trigger:** Player enters Ready Room
**Display:**

The Ready Room console activates. The screen shows a formal commission document, scrolling slowly:

```
STARSHIP COMMISSION — INCIDENT RECONSTRUCTION

TO: [PLAYER], Officer of the Watch
FROM: Fleet Operations, Sector 7 Command
RE: Telemetry Gap — Hours 1822-0022 UTC

The commanding officer has been relieved. Official cause:
navigational incident. Six hours of main telemetry deleted at
0347 UTC. Backup instruments continued recording. A sealed order
will purge backup logs in 72 hours.

You have been assigned to reconstruct what happened during the gap
using surviving instrument data. Begin with external telemetry.
The instruments in Communications, Tactical Control, Nav Control,
and the Science Lab contain signal data from the gap.

Operate each instrument. Determine what it recorded.
Report your findings to the Science Station on the Bridge.

The instruments do not lie. Someone wishes they did.

COMMISSION ORDER #2247 — AUTHORIZED
```

**Audio:** Console activation hum. Paper-on-screen scroll sound. A brief authentication chime at "AUTHORIZED."
**Duration:** Player-paced (scrolling text, player clicks ACKNOWLEDGE to proceed)
**Exits:** Turbolift unlocks. Navigation phase enters COMMISSION. All 5 deck stops available. Round 1 puzzle rooms open.

---

#### Scene 3: FIRST INSTRUMENT (R1-01 Carrier Isolation)

**ID:** `ACT1_R1_01`
**Location:** 2-01-01 Communications Station
**Trigger:** Player enters Communications Station for the first time
**Display:**

The room lights come up to operational brightness. Two console stations are visible. The left station's main monitor activates with:

```
COMMS — SIGNAL ANALYSIS WORKSTATION
Terminal: COMMS-SIG-01-A
Data Source: Gap Signal Buffer (RAW)
Status: READY — AWAITING OPERATOR

Three overlapping signal components detected in buffer.
Filter configuration deleted. Reconstruction required.
```

The SineWaveDisplay widget initializes — showing the composite waveform (three overlapping signals). The NumericStepper and RotaryDial controls appear below. A reference card panel slides out on the right side.

**Audio:** Console power-up sequence (3-second fan spin, display flicker, then steady hum). Faint static from the signal buffer playback.
**Duration:** 3 seconds for power-up, then player has full control.
**Exits:** Player operates instrument to solve R1-01 (answer: 2.340 GHz). On correct answer, the console displays:

```
CARRIER ISOLATED: 2.340 GHz
S-BAND. SUSTAINED TRANSMISSION. NOT A BEACON.
MEASUREMENT LOGGED TO COMMISSION FILE.
```

---

#### Scene 4: LISSAJOUS (R1-02 Phase Lock)

**ID:** `ACT1_R1_02`
**Location:** 4-01-01 Science Lab
**Trigger:** Player enters Science Lab for the first time
**Display:**

Science Lab — four workstations arranged in a semicircle. Station 2 activates:

```
SCIENCE — DUAL-CHANNEL OSCILLOSCOPE (X-Y MODE)
Terminal: SCI-SCOPE-02-A
CH1: Ship outbound — 291 MHz (stable)
CH2: Incoming signal — frequency unknown
Mode: X-Y (Lissajous)
Status: DRIFT DETECTED — AWAITING LOCK
```

The LissajousDisplay shows a slowly rotating, drifting spiral — the figure is unstable. The Y-frequency NumericStepper and fine-tune RotaryDial appear. The reference card shows what stable Lissajous ratios look like (1:1 circle, 2:1 figure-8, 3:1 pretzel).

**Audio:** Oscilloscope sweep tone (faint periodic chirp matching the drift rate). Tone pitch changes as player adjusts frequency.
**Duration:** 3 seconds for power-up.
**Exits:** Player achieves 3:1 lock at 873 MHz. The figure freezes, perfectly stable:

```
PHASE LOCK ACQUIRED.
RATIO: 3:1 (CH2 = 3 x CH1)
Y FREQUENCY: 873 MHz
THE INCOMING SIGNAL IS TRACKING US. HARMONIC LOCK.
```

---

#### Scene 5: NULL STEERING (R1-03 Null Zone)

**ID:** `ACT1_R1_03`
**Location:** 2-01-02 Tactical Control
**Trigger:** Player enters Tactical Control for the first time
**Display:**

Two stations in Tactical Control. Station 1 (left) activates for R1-03:

```
TAC/COMMS — PHASED ARRAY INTERFERENCE DISPLAY
Terminal: TAC-ARRAY-01-A
Mode: Dual-Source Jamming
Target: Isolate signal from interference
Array Configuration: DELETED — RECONSTRUCTION REQUIRED
```

The PhaseInterferenceDisplay shows a polar interference pattern with two overlapping wave sources. The LinkedSliderInput controls source separation and phase offset. A ToggleSwitch selects active/passive nulling mode. The pattern has bright and dark zones — the solver must place a null on the interference source to reveal the target signal's bearing.

**Audio:** Low-frequency interference hum that shifts as the player adjusts null placement. When null locks onto interference, the hum drops to silence and a clean signal tone emerges.
**Duration:** 3 seconds for power-up.
**Exits:** Target bearing resolves to 213 degrees:

```
NULL ZONE PLACED. INTERFERENCE SUPPRESSED.
TARGET SIGNAL BEARING: 213 DEGREES
PORT QUARTER APPROACH. DELIBERATE CLOSING VECTOR.
```

---

#### Scene 6: TRAJECTORY (R1-04 Orbit Classification)

**ID:** `ACT1_R1_04`
**Location:** 2-01-03 Nav Control
**Trigger:** Player enters Nav Control for the first time
**Display:**

```
NAV/SCIENCE — CONIC SECTION TRAJECTORY ANALYZER
Terminal: NAV-TRAJ-01-A
Data Source: Position fix backup (8 fixes, 47-minute window)
Status: READY — CURVE FIT REQUIRED
```

The ConicSectionDisplay shows a Cartesian grid with 8 position dots (the tracked object's fixes during the gap). The theoretical conic curve (adjustable) does not yet match. The NumericStepper controls eccentricity (0.00-2.00). The RotaryDial controls semi-major axis. A reference card shows conic section types: circle (e=0), ellipse (0<e<1), parabola (e=1), hyperbola (e>1).

**Audio:** Soft plotting clicks as each position fix appears. A confirmation tone when the curve passes through all 8 dots.
**Duration:** 3 seconds for power-up.
**Exits:** Eccentricity = 0.73. All dots on curve:

```
CURVE FIT: COMPLETE
ECCENTRICITY: 0.73
CLASSIFICATION: HYPERBOLIC-CAPTURE
NOT PASSING THROUGH. NOT ORBITING.
THE OBJECT DECELERATED AND HELD STATION.
```

---

#### Scene 7: RADAR LOCK (R1-05 Contact Lock)

**ID:** `ACT1_R1_05`
**Location:** 2-01-02 Tactical Control
**Trigger:** Player accesses Station 2 in Tactical Control (or first visit if R1-03 already done)
**Display:**

Station 2 (right) in Tactical Control activates:

```
TAC — TACTICAL RADAR + TARGETING LOCK
Terminal: TAC-RADAR-02-A
Contacts: 8 DETECTED (unclassified)
Classification Log: DELETED — RECLASSIFICATION REQUIRED
```

The RadarSweepDisplay shows a circular radar scope with 8 blips at various positions. The sweep line rotates. The TargetingReticleDisplay overlay appears when the player clicks a contact. Eight LightedButton controls (one per contact) allow classification: click to mark REAL (green) or DECOY (red).

**Audio:** Radar sweep ping (once per rotation). Lock-on tone when reticle settles on a contact. Different confirmation tones for REAL vs DECOY classification.
**Duration:** 3 seconds for power-up.
**Exits:** 5 marked REAL, 3 marked DECOY:

```
CLASSIFICATION COMPLETE.
REAL CONTACTS: 5 (PRIMARY + 4 ESCORTS)
DECOY CONTACTS: 3 (MULTIPATH/ECM)
FORMATION SIZE: 5. THIS WAS NOT A LONE VESSEL.
```

---

#### Scene 8: CONSTELLATION (R1-06 Signal Fingerprint)

**ID:** `ACT1_R1_06`
**Location:** 2-01-01 Communications Station
**Trigger:** Player accesses Station 2 in Communications (or first visit if R1-01 already done)
**Display:**

Right-side console in Communications activates:

```
COMMS — VECTOR SIGNAL ANALYZER
Terminal: COMMS-VSA-01-B
Data Source: IQ Recording (Gap Signal Buffer)
Modulation: UNKNOWN
Symbol Rate: UNKNOWN
Status: CONSTELLATION DISTORTED — IDENTIFICATION REQUIRED
```

The CommSignalDisplay shows a scattered, rotated IQ constellation. The RotaryDial selects modulation type (BPSK, QPSK, 8-PSK, 16-QAM). The NumericStepper sets symbol rate. As the player selects the correct modulation, the constellation points snap into a recognizable pattern.

**Audio:** Digital data chirp (garbled at wrong settings, clean at correct settings). EVM meter clicks.
**Duration:** 3 seconds for power-up.
**Exits:** 8-PSK selected, 128 kSym/s, data rate = 384 kbps:

```
MODULATION: 8-PSK
SYMBOL RATE: 128 kSym/s
DATA RATE: 384 kbps
EVM: 12% (CLEAN SIGNAL)
384 kbps IS NOT A BEACON. NOT A DISTRESS CALL.
THIS WAS A CONVERSATION.
```

---

#### Scene 9: ROUND 1 PROGRESS (Auto-Tracking)

**ID:** `ACT1_PROGRESS`
**Location:** Any (HUD overlay)
**Trigger:** Each R1 feeder solved
**Display:**

A small commission progress panel in the top-right corner updates:

```
TELEMETRY RECONSTRUCTION
━━━━━━━━━━━━━━━━━━━━━━━
R1-01 CARRIER:     2.340 GHz    ■
R1-02 HARMONIC:    873 MHz      ■
R1-03 BEARING:     213 deg      ■
R1-04 TRAJECTORY:  0.73         ■
R1-05 FORMATION:   5 contacts   ■
R1-06 DATA RATE:   384 kbps     ■
━━━━━━━━━━━━━━━━━━━━━━━
FEEDERS COMPLETE: [X]/6
```

Unsolved puzzles show `---` and an empty box. When 4+ are solved, a new line appears:

```
CONTACT CLASSIFICATION AVAILABLE.
REPORT TO BRIDGE SCIENCE STATION.
```

**Audio:** Chime on each feeder solve. A different, deeper tone when 4+ threshold reached.
**Duration:** 3-second overlay, then fades.
**Exits:** Player navigates to Bridge Science for R1-META.

---

#### Scene 10: R1-META (Contact Signature Classification)

**ID:** `ACT1_R1_META`
**Location:** 1-01-05 Bridge — Science
**Trigger:** Player enters Bridge Science with 4+ R1 feeders solved
**Display:**

```
SCIENCE STATION — CONTACT SIGNATURE CLASSIFICATION
Terminal: SCI-CLASS-01-A
Data Source: Round 1 Feeder Measurements
Classification: RESTRICTED — INCIDENT RECONSTRUCTION
```

The IndicatorPanel displays all 6 feeder values (solved values filled, unsolved show "---"). Below it, the Contact Signature Reference Table appears — a classified database of 8 known contact types, each with 6 parameter columns plus a Class Code column. Phase-Locked Echo is the last row.

The NumericInput widget appears at the bottom: "ENTER CLASS CODE."

**Audio:** Classification database loading sound (brief data scroll). When values match a row, that row highlights with a soft lock-on tone.
**Duration:** 3 seconds for display build.
**Exits:** Player enters 8 (Phase-Locked Echo class code):

```
CLASS CODE ACCEPTED. N = 8.
COMMISSION DECODER CONFIGURED: 8-POINT CIRCLE.

CONTACT IDENTIFIED: PHASE-LOCKED ECHO
A Phase-Locked Echo is not a phenomenon. It is an entity.
It chose to respond. It chose the harmonic. It chose to hold formation.
The ship did not encounter a Phase-Locked Echo during the gap.
A Phase-Locked Echo encountered the ship.
```

**Audio:** Classification confirmed — three ascending tones. A low, resonant pulse (the "echo" motif — first appearance). Silence for 3 seconds after the revelation text.

---

#### Scene 11: ROUND 1 COMPLETE (Transition)

**ID:** `ACT1_COMPLETE`
**Location:** 1-01-05 Bridge — Science
**Trigger:** R1-META solved
**Display:**

The Science Station display transitions:

```
TELEMETRY RECONSTRUCTION: COMPLETE
CONTACT: PHASE-LOCKED ECHO (CLASS 8)

COMMISSION ORDER #2247 — UPDATE
Proceed to internal systems analysis.
The ship's response during the gap is recorded in
Engineering, EPS, Environmental, and Tactical consoles.

Round 2: SYSTEMS LOG
Access authorized: Deck 5 Engineering, Deck 1 Tactical
```

**Audio:** Commission update chime. The ambient hum shifts slightly — a subtle key change indicating narrative progression.
**Duration:** 8 seconds, then player regains control.
**Exits:** Round 2 rooms unlock. Navigation phase enters ROUND_2.

---

#### Scene 12: NARRATIVE BEAT — "Someone Knew"

**ID:** `ACT1_NARRATIVE`
**Location:** Turbolift (between Decks 1 and 5)
**Trigger:** First turbolift ride after R1-META solved
**Display:**

During the turbolift transition animation, text appears on the turbolift's small status display:

```
TURBOLIFT — DECK 5

NOTE TO COMMISSION FILE:
The six telemetry measurements describe an entity that
locked onto the ship's own signal at a harmonic ratio.
It held formation at 51 km for 47 minutes.
It transmitted 384 kbps of structured data.

The contact was waiting. The question is no longer
what was out there. The question is: did the ship
know it was coming?
```

**Audio:** Turbolift movement sound (mechanical hum, deceleration). Text appears over the movement audio.
**Duration:** Turbolift transit time (~5 seconds). Text remains readable for 3 seconds after arrival.
**Exits:** Deck 5 map opens.

---

### ACT 2 — SYSTEMS LOG (Scenes 13-23)

*The ship's response was calm, coordinated, and prepared. Someone on this ship knew the encounter was coming.*

---

#### Scene 13: ENGINEERING ARRIVAL

**ID:** `ACT2_ENG_ARRIVAL`
**Location:** 5-01-01 Main Engineering — Power
**Trigger:** Player enters Main Engineering for the first time (during Round 2)
**Display:**

Main Engineering is large — four console stations visible in the Power section alone. The overhead display shows the ship's power distribution diagram. Status board reads:

```
MAIN ENGINEERING — POWER SECTION
4 CONSOLE STATIONS ACTIVE
REACTOR: FUSION — ONLINE (OUTPUT 847 MW)
EPS GRID: NOMINAL
COMMISSION ACCESS: AUTHORIZED — ROUND 2
```

Two consoles highlight with commission markers (amber outlines): the EPS Conduit Routing Console (R2-01) and the Reactor Operations Console (R2-06).

**Audio:** Engineering ambient — louder reactor hum, plasma conduit flow sounds, periodic status chimes. Distinctly different from bridge ambient.
**Duration:** 5 seconds for environment build.
**Exits:** Player approaches either highlighted console.

---

#### Scene 14: EPS ROUTING (R2-01 Power Path)

**ID:** `ACT2_R2_01`
**Location:** 5-01-01 Main Engineering — Power
**Trigger:** Player accesses EPS Conduit Routing Console
**Display:**

```
POWER — EPS CONDUIT ROUTING CONSOLE
Terminal: ENG-EPS-ROUTE-01
Data Source: EPS Junction Log (Gap Snapshot)
Status: JUNCTION OVERLOAD DETECTED — RECONFIGURATION REQUIRED
```

The ConduitFlowDisplay shows three conduits feeding through a shared junction to three destination systems. All three conduits active = junction overload (red warning). Three ToggleSwitch controls (one per conduit). Three GaugeDisplay readouts (one per destination system — must all be green).

**Audio:** EPS hum. Overload warning buzzer when junction overloads. Gauge click-into-green sounds.
**Duration:** 3 seconds for power-up.
**Exits:** 2 conduits active, all destinations green:

```
CONFIGURATION FOUND. ACTIVE CONDUITS: 2.
ALL DESTINATION SYSTEMS POWERED.
SOMEONE FOUND THIS BALANCE DURING THE GAP.
TWO CONDUITS. NOT THREE. PRECISE ALLOCATION.
```

---

#### Scene 15: ODN TRACE (R2-02 Data Breach)

**ID:** `ACT2_R2_02`
**Location:** 5-01-03 Main Engineering — Computer
**Trigger:** Player enters Computer section of Main Engineering
**Display:**

```
COMPUTER — ODN ISOLATION CONSOLE
Terminal: ENG-ODN-ISO-01
Network: 15 nodes active
Alert: ROGUE PACKET DETECTED (unauthorized data injection)
Status: ENTRY POINT UNKNOWN — ISOLATION REQUIRED
```

The NetworkGridDisplay shows the 15-node ODN topology. A red packet animates along a fixed route through the network. 15 ToggleSwitch controls (one per node — click to disable). When a node is disabled, the red packet either reroutes or vanishes (if the disabled node was on its path).

**Audio:** Network data flow (rapid clicking). Red packet has a distinct harsh tone. Node disable = power-down sound.
**Duration:** 3 seconds for power-up.
**Exits:** Node 7 (Security Computer) identified as entry point:

```
ENTRY NODE: 7 — SECURITY COMPUTER
UNAUTHORIZED DATA INJECTION ORIGINATED FROM
THE SECURITY SYSTEM ITSELF.
THE FORGED SENSOR READINGS WERE PLANTED FROM INSIDE.
```

---

#### Scene 16: THERMAL TRACE (R2-03 Heat Source)

**ID:** `ACT2_R2_03`
**Location:** 5-02-02 Environmental Control
**Trigger:** Player enters Environmental Control
**Display:**

```
ENVIRO — DECK THERMAL MONITORING CONSOLE
Terminal: ENV-THERM-01
Data Source: Thermal Grid Snapshot (Gap +03:14)
Alert: ANOMALOUS HEAT SOURCE DETECTED
Status: SOURCE LOCATION MASKED BY THERMAL DIFFUSION
```

The HeatMapDisplay shows a 5x5 grid of temperature readings, all elevated — heat has diffused outward from the source. LCARSButton controls place coolant zones (up to 4). GaugeDisplay shows peak temperature.

**Audio:** Environmental systems hum. Temperature click sounds as coolant zones absorb heat. A "lock" tone when source is isolated.
**Duration:** 3 seconds for power-up.
**Exits:** Grid coordinate 14 isolated:

```
HEAT SOURCE ISOLATED: GRID COORDINATE 14
LOCATION: SENSOR BAY, DECK 4
SOMEONE WAS RUNNING EQUIPMENT IN THE SENSOR BAY
DURING THE GAP. EQUIPMENT THAT GENERATES HEAT.
EQUIPMENT THAT WAS NOT AUTHORIZED TO BE POWERED.
```

---

#### Scene 17: SHIELD ANALYSIS (R2-04 Shield Profile)

**ID:** `ACT2_R2_04`
**Location:** 1-01-03 Bridge — Tactical
**Trigger:** Player accesses Shield Configuration Console on Bridge Tactical
**Display:**

```
TAC — SHIELD CONFIGURATION CONSOLE
Terminal: TAC-SHIELD-01
Data Source: Shield Damage Pattern (Gap Backup Sensor)
Alert: DAMAGE PATTERN INCONSISTENT WITH REACTIVE DEFENSE
Status: PRESET POSTURE DETECTED — RECONSTRUCTION REQUIRED
```

The ShieldDisplay shows a three-layer shield diagram with damage indicators. A reference overlay shows the recorded damage pattern from the gap. Three PowerSlider controls adjust power allocation per layer (total = 100%). As the player adjusts, simulated damage redistributes across layers.

**Audio:** Shield harmonic hum. Power redistribution sounds (plasma flow shifting). Match confirmation when all three layers align with reference.
**Duration:** 3 seconds for power-up.
**Exits:** Layer 2 power = 65%:

```
SHIELD POSTURE MATCHED.
L1: 20%  L2: 65%  L3: 15%
THIS IS NOT A REACTIVE POSTURE. THIS IS A PRESET.
SOMEONE LOADED THIS CONFIGURATION BEFORE THE CONTACT
WAS OFFICIALLY DETECTED. THE SHIELDS WERE READY
BEFORE ANYONE SHOULD HAVE KNOWN TO RAISE THEM.
```

**Audio:** A low, ominous chord on the match confirmation. This is THE TURN preparation — the first strong evidence of foreknowledge.

---

#### Scene 18: CIRCUIT FAULT (R2-05 Fault Trace)

**ID:** `ACT2_R2_05`
**Location:** 5-02-01 EPS Control
**Trigger:** Player enters EPS Control
**Display:**

```
ENG — EPS CIRCUIT DIAGNOSTIC CONSOLE
Terminal: ENG-EPS-DIAG-01
Data Source: EPS Circuit Topology (Sensor Recording Subsystem)
Alert: COMPONENT FAILURE — SENSOR RECORDING LOST
Status: FAULT LOCATION UNKNOWN — TRACE REQUIRED
```

The CircuitTopologyDisplay shows a branching circuit tree. LCARSButton controls probe individual nodes (showing current/no-current at each point). The player performs binary search through the branches to find where current stops flowing.

**Audio:** Probe sounds (electrical test tone — high for current, dead silence for no-current). Branch selection clicks.
**Duration:** 3 seconds for power-up.
**Exits:** Component 9 identified:

```
FAULT TRACE COMPLETE. FAILED COMPONENT: 9.
RELAY 9 IS THE JUNCTION FEEDING THE SENSOR RECORDER.
WHEN RELAY 9 FAILED, THE INSTRUMENTS WENT DARK.
THIS FAILURE WAS NOT RANDOM. RELAY 9 SHOWS OVERCURRENT
DAMAGE CONSISTENT WITH A DELIBERATE SURGE.
SOMEONE BURNED THE RECORDING SYSTEM ON PURPOSE.
```

---

#### Scene 19: THE TURN (R2-06 Reactor State)

**ID:** `ACT2_THE_TURN`
**Location:** 5-01-01 Main Engineering — Power
**Trigger:** Player accesses Reactor Operations Console
**Display:**

```
POWER — REACTOR OPERATIONS CONSOLE
Terminal: ENG-REACTOR-01
Data Source: Backup Reactor Sensor (Independent Logger)
Snapshot: Gap +02:47
Status: FOUR GAUGE READINGS RECOVERED — MATCH REQUIRED
```

Four GaugeDisplay widgets show target setpoint markers from the gap snapshot. ThrottleLever and PowerSlider controls adjust fuel flow and containment. All four needles must align simultaneously.

**Audio:** Reactor hum deepens as throttle moves. Containment field oscillation as PowerSlider adjusts. Four distinct "lock" tones as each gauge aligns.
**Duration:** 3 seconds for power-up.
**Exits:** Throttle position 4. All gauges matched:

```
REACTOR STATE MATCHED.
THROTTLE: POSITION 4 — MODERATE SUSTAINED OUTPUT
CONTAINMENT: NOMINAL
TEMPERATURE: STABLE AT 847 MW

THIS IS NOT BATTLE POWER. THIS IS NOT FLIGHT POWER.
THIS IS STATION-KEEPING. THE SHIP WAS HOLDING POSITION.
NOT FLEEING. NOT FIGHTING.
WAITING.
```

**Audio:** The reactor hum drops to a steady, low tone. Three seconds of silence. Then the "echo" motif from Scene 10 returns — quiet, distant, but unmistakable. The contact was 51 km away. The ship was holding station. This was a rendezvous.

This is THE TURN. The narrative shifts from "what happened?" to "who ordered this?" The player now knows the ship's response was not defensive. It was prepared, precise, and deliberate. The next round will name names.

---

#### Scene 20: R2 PROGRESS (Auto-Tracking)

**ID:** `ACT2_PROGRESS`
**Location:** Any (HUD overlay)
**Trigger:** Each R2 feeder solved
**Display:**

```
SYSTEMS LOG RECONSTRUCTION
━━━━━━━━━━━━━━━━━━━━━━━━━
R2-01 CONDUITS:    2           ■
R2-02 BREACH NODE: 7           ■
R2-03 HEAT COORD:  14          ■
R2-04 SHIELD L2:   65%         ■
R2-05 FAULT COMP:  9           ■
R2-06 THROTTLE:    4           ■
━━━━━━━━━━━━━━━━━━━━━━━━━
FEEDERS COMPLETE: [X]/6
```

When 4+ solved:

```
STATION COMMAND LOG AVAILABLE.
REPORT TO BRIDGE COMMAND STATION.
```

**Audio:** Same pattern as Act 1 progress tracking.
**Duration:** 3-second overlay.
**Exits:** Player navigates to Bridge Center for R2-META.

---

#### Scene 21: R2-META (Station Command Log)

**ID:** `ACT2_R2_META`
**Location:** 1-01-01 Bridge — Center
**Trigger:** Player accesses Command console with 4+ R2 feeders solved
**Display:**

```
BRIDGE OPS — STATION COMMAND LOG
Terminal: CMD-STLOG-01-A
Data Source: Round 2 Feeder Values + First Response Protocol
Classification: RESTRICTED — INCIDENT RECONSTRUCTION
```

The IndicatorPanel shows all 6 R2 values with parity highlighting: even values (2, 14, 4) dim with "NOMINAL" tags; odd values (7, 65, 9) glow amber with "ANOMALY" tags. Below, the Station Command Log table maps systems to controlling stations and roster positions.

The NumericInput: "FIRST RESPONDER ROSTER POSITION."

**Audio:** Parity sorting sound (a soft binary click pattern). Amber highlighting accompanied by a warning undertone.
**Duration:** 3 seconds for display build.
**Exits:** Player enters 3 (KWON at COMPUTER):

```
STARTING POSITION ACCEPTED. START = 3 (KWON).

KWON'S STATION WAS THE FIRST TO REGISTER ANOMALOUS ACTIVITY.
THE UNAUTHORIZED DATA INJECTION THROUGH NODE 07 AT GAP +00:14
WAS THE EARLIEST SYSTEM-LEVEL ANOMALY. COMPUTER FLAGGED IT.
BUT KWON COULD NOT RESPOND. AT GAP +01:22, KWON COLLAPSED
AT THE CONSOLE — V-TACH FROM THE ATMOSPHERIC SUPPRESSANT.

THE FIRST ALERT WENT UNACKNOWLEDGED BECAUSE THE ALERTING
OFFICER WAS INCAPACITATED. THIS WAS NOT A COINCIDENCE.

STARTING POSITION: 3
```

**Audio:** The KWON revelation is accompanied by a heartbeat monitor sound — brief, then flatline. Silence.

---

#### Scene 22: ROUND 2 COMPLETE (Transition)

**ID:** `ACT2_COMPLETE`
**Location:** 1-01-01 Bridge — Center
**Trigger:** R2-META solved
**Display:**

```
SYSTEMS LOG RECONSTRUCTION: COMPLETE
FIRST RESPONDER: KWON (POSITION 3)

COMMISSION ORDER #2247 — UPDATE
Proceed to crew record analysis.
Personnel and biological data from the gap survive in
Sickbay, Security Office, Computer Core, and Deflector Control.

Round 3: CREW RECORD
Access authorized: Deck 3 Sickbay, Deck 4 Security,
                   Deck 5 Computer Core, Deck 5 Deflector Control
```

**Audio:** Commission update chime. Ambient shifts — darker, more tense. The investigation has become personal.
**Duration:** 8 seconds.
**Exits:** Round 3 rooms unlock. Computer Core secured door opens. Brig becomes accessible. Navigation phase enters ROUND_3.

---

#### Scene 23: NARRATIVE BEAT — "Someone on This Ship"

**ID:** `ACT2_NARRATIVE`
**Location:** Turbolift (between decks)
**Trigger:** First turbolift ride after R2-META solved
**Display:**

```
TURBOLIFT — TRANSIT

NOTE TO COMMISSION FILE:
Three systems showed anomalies during the gap.
The breach came through the Security Computer.
The shields were set to a preset posture before detection.
The sensor recorder was deliberately burned.
The reactor was at station-keeping power.

The ship was not attacked. It was not surprised.
The response was coordinated and prepared.

Someone on this ship knew the encounter was coming.
The crew records will say who.
```

**Audio:** Turbolift transit. A brief return of the "echo" motif, closer now.
**Duration:** Turbolift transit (~5 seconds) + 3 seconds hold.
**Exits:** Destination deck map opens.

---

### ACT 3 — CREW RECORD (Scenes 24-33)

*The cover-up was ordered before the encounter ended. Whoever deleted the logs already knew what was in them.*

---

#### Scene 24: SICKBAY (R3-01 Triage)

**ID:** `ACT3_R3_01`
**Location:** 3-01-01 Sickbay — Main
**Trigger:** Player enters Sickbay for the first time (during Round 3)
**Display:**

Sickbay lighting is clinical white. Three biobeds visible. The ECG monitor console activates:

```
MEDICAL — MULTI-CHANNEL ECG MONITOR
Terminal: MED-ECG-01
Data Source: Passive Sensor Buffer (Gap Period)
Patients: 3 ADMITTED DURING GAP
Status: CARDIAC ANOMALY DETECTED — CLASSIFICATION REQUIRED
```

The LifesignsDisplay shows three simultaneous ECG traces (cyan, amber, red). The RotaryDial selects the active patient for analysis. The LinkedSliderInput adjusts temporal zoom and baseline.

**Audio:** ECG beeps — three distinct rhythms. One (Patient 3) is irregular, accelerating. The clinical beeping creates tension without music.
**Duration:** 3 seconds for power-up.
**Exits:** Patient 3 identified as cardiac crisis (V-tach):

```
CRISIS PATIENT: 3 — KWON, J. (LT.)
WAVEFORM: VENTRICULAR TACHYCARDIA
ONSET: GAP +01:22
CAUSE: CONSISTENT WITH ATMOSPHERIC SUPPRESSANT EXPOSURE

THE DUTY LOG SAYS KWON WAS ASLEEP IN QUARTERS.
THE ECG SAYS KWON WAS IN CARDIAC CRISIS AT THE CONSOLE.
THE DUTY LOG IS WRONG.
```

**Audio:** V-tach alarm tone. Brief, then muted. The clinical tone of the room makes the accusation hit harder.

---

#### Scene 25: KEY ROTATION (R3-02 Access Code)

**ID:** `ACT3_R3_02`
**Location:** 4-01-02 Security Office
**Trigger:** Player enters Security Office for the first time (during Round 3)
**Display:**

Security Office — three workstations. Station 1 activates:

```
SECURITY — MODULAR KEY ROTATION INTERFACE
Terminal: SEC-KEY-01-A
Data Source: Hardware Security Buffer (Door Access Log)
Compartment: SENSOR BAY (RESTRICTED)
Status: ACCESS SEQUENCE RECOVERED — FINAL POSITION REQUIRED
```

The ModularClockDisplay shows an 8-position clock face labeled with duty roster names (0=TORRES through 7=CHEN). A pointer starts at position 0. The NumericStepper applies rotation steps. An LCARSButton confirms each rotation. A ScrollingTextDisplay shows the logged rotation sequence.

**Audio:** Clock mechanism sounds (gear click on each rotation). Roster name pronunciation on final position.
**Duration:** 3 seconds for power-up.
**Exits:** Final clock position = 7 (CHEN):

```
FINAL POSITION: 7 — CHEN, M. (ENS.)
CHEN ACCESSED THE RESTRICTED SENSOR BAY DURING THE GAP.
THE ACCESS WAS AUTHORIZED. THE CODE WAS VALID.
THE QUESTION IS: WHO DISTRIBUTED THAT CODE?
```

---

#### Scene 26: PERMISSION CHAIN (R3-03 Permission Chain)

**ID:** `ACT3_R3_03`
**Location:** 5-03-01 Computer Core
**Trigger:** Player enters Computer Core (newly accessible in Round 3)
**Display:**

The Computer Core is a vertical room — racks of isolinear arrays floor to ceiling, blue status lights blinking. One workstation:

```
SECURITY/COMPUTER — GROUP COMPOSITION INTERFACE
Terminal: COMP-PERM-01
Data Source: Permission Cache (Sensor Purge Authorization)
Alert: UNAUTHORIZED OVERRIDE DETECTED
Status: AUTHORIZATION TOKEN UNKNOWN — COMPOSITION REQUIRED
```

The CayleyTableDisplay shows an 8x8 multiplication table (Z_8 under addition mod 8). A RotaryDial selects elements. An LCARSButton performs composition. A "TARGET" marker highlights the required result (element 6 — the OVERRIDE permission). The known base permission is element 2 (held by the person who initiated the purge).

**Audio:** Computing sounds (data processing hum). Composition confirmation clicks. A ratcheting tension sound as the solver narrows possibilities.
**Duration:** 3 seconds for power-up.
**Exits:** Element 4 (REEVES) identified:

```
AUTHORIZATION TOKEN: ELEMENT 4 — REEVES, K. (LT. CMDR.)
COMPOSITION: 2 + 4 = 6 (mod 8) = OVERRIDE
REEVES' TOKEN COMPOSED THE OVERRIDE PERMISSION.
REEVES IS THE CHIEF ENGINEER. ONE OF THREE PEOPLE
WHO KNEW THE CLASSIFIED ADDENDUM TO CONTACT PROTOCOL SEVEN.
```

---

#### Scene 27: THE DROP (R3-04 Badge Sequence)

**ID:** `ACT3_THE_DROP`
**Location:** 4-01-02 Security Office
**Trigger:** Player accesses Station 2 in Security Office
**Display:**

Station 2 in Security Office activates:

```
SECURITY — BADGE SWIPE LOG DISPLAY
Terminal: SEC-BADGE-01-B
Data Source: Hardware Security Buffer (Badge Reader Log)
Entries: 18 SWIPES RECORDED DURING GAP
Alert: FORCED ENTRIES DETECTED
Status: BADGE IDENTIFICATION REQUIRED
```

The IndicatorPanel shows 18 LEDs in a row. Most are green (GRANTED) or red (DENIED). Three burn amber, pulsing — the FORCED entries. A ScrollingTextDisplay below shows the full swipe log. When the player identifies the pattern, the badge number resolves.

**Audio:** Badge swipe sounds play in sequence as the ScrollingTextDisplay advances. GRANTED = clean beep. DENIED = harsh buzz. FORCED = a heavy metallic clunk (door being overridden). The three FORCED entries are viscerally distinct.
**Duration:** 3 seconds for power-up.
**Exits:** Badge 401 identified at position 5 (VASQUEZ):

```
FORCED ENTRIES: POSITIONS 5, 11, 16
BADGE: 401 — VASQUEZ, R. (CAPT.)
ACCESS: COMMAND OVERRIDE (CO PRIVILEGE)
THREE RESTRICTED DOORS. FORCED OPEN. SAME BADGE.

THE COMMANDING OFFICER WALKED THROUGH THREE RESTRICTED
DOORS DURING THE GAP. NOT GRANTED ACCESS. FORCED.
USING HER OWN BADGE. UNDER HER OWN AUTHORITY.
```

**Audio:** This is THE DROP. A sustained low tone. The three FORCED clunks replay in rapid succession. Then silence. The ambient shifts — the investigation is no longer abstract. The CO forced doors. The player now knows who.

---

#### Scene 28: BYPASS SEQUENCE (R3-05 Emergency Sequence)

**ID:** `ACT3_R3_05`
**Location:** 5-02-03 Deflector Control
**Trigger:** Player enters Deflector Control
**Display:**

```
COMMAND/ENGINEERING — GUARDED EMERGENCY PROCEDURE CONSOLE
Terminal: ENG-EMRG-PROC-01
Data Source: Procedure Register (Hardware Counter)
Alert: CO CONFIRMATION BYPASS DETECTED
Status: SEQUENCE RECONSTRUCTION REQUIRED
```

Four BatSwitch controls in a row, each with a red safety guard. Labels: A=THERMAL, B=MAGNETIC, C=COMM, D=SENSOR. An IndicatorPanel shows the procedure register (counts physical operations). A ScrollingTextDisplay shows the classified addendum excerpt. A MasterAlarm indicator. An AutoProcedureButton for guided execution.

**Audio:** Guard lift sounds (mechanical click-clack). Switch flip (heavy toggle). Guard close (snap). Each operation increments the procedure register with an audible counter tick. The MasterAlarm sounds if the wrong sequence is attempted.
**Duration:** 3 seconds for power-up.
**Exits:** 12-operation bypass sequence completed:

```
PROCEDURE REGISTER: 12 OPERATIONS
SEQUENCE: THERMAL-MAGNETIC-COMM-SENSOR(ON)-SENSOR(OFF)
WITH GUARDS IN SPECIFIC ORDER
CO CONFIRMATION STEP: BYPASSED

THIS SEQUENCE IS DOCUMENTED IN THE CLASSIFIED ADDENDUM
TO CONTACT PROTOCOL SEVEN. ONLY THREE PEOPLE HAVE READ IT:
THE CO, THE XO, AND THE CHIEF ENGINEER.
THE XO WAS OFF-SHIP. THAT LEAVES TWO.
```

---

#### Scene 29: R3 PROGRESS (Auto-Tracking)

**ID:** `ACT3_PROGRESS`
**Location:** Any (HUD overlay)
**Trigger:** Each R3 feeder solved
**Display:**

```
CREW RECORD RECONSTRUCTION
━━━━━━━━━━━━━━━━━━━━━━━━━
R3-01 PATIENT:     3 (KWON)    ■
R3-02 ACCESS:      7 (CHEN)    ■
R3-03 AUTH TOKEN:  4 (REEVES)  ■
R3-04 BADGE:       5 (VASQUEZ) ■
R3-05 PROCEDURE:   12          ■
━━━━━━━━━━━━━━━━━━━━━━━━━
FEEDERS COMPLETE: [X]/5
```

When 3+ solved:

```
AUTHORIZATION CHAIN ANALYSIS AVAILABLE.
REPORT TO SECURITY OFFICE.
```

**Audio:** Same pattern. The chime on completion is lower, more ominous.
**Duration:** 3-second overlay.
**Exits:** Player navigates to Security Office for R3-META.

---

#### Scene 30: R3-META (Authorization Chain)

**ID:** `ACT3_R3_META`
**Location:** 4-01-02 Security Office
**Trigger:** Player accesses Station 3 in Security Office with 3+ R3 feeders solved
**Display:**

```
SECURITY — AUTHORIZATION CHAIN ANALYSIS
Terminal: SEC-AUTH-CHAIN-01
Data Source: Round 3 Feeder Values + Ship Authorization Protocol
Classification: RESTRICTED — INCIDENT RECONSTRUCTION
```

The Authorization Chain Display shows all 5 feeder values mapped to crew roster positions (with R3-05's value of 12 reduced mod 8 to 4). The Action-Authorization Matrix shows each person's role. The chain visualization draws arrows: VASQUEZ --> REEVES.

The NumericInput: "AUTHORIZATION CHAIN LENGTH."

**Audio:** Chain link sounds (metallic clicks) as each connection draws. A ratcheting tension that builds as the chain forms.
**Duration:** 3 seconds for display build.
**Exits:** Player enters 2 (chain length):

```
STEP SIZE ACCEPTED. STEP = 2.

AUTHORIZATION CHAIN:
  VASQUEZ (Commanding Officer)
    ├── Self-authorized: CO privilege invoked at 3 restricted doors
    └── Authorized REEVES: token provided for OVERRIDE composition
        └── REEVES executed: 12-operation deflector bypass

  Chain: VASQUEZ ──> REEVES
  Length: 2 links

VASQUEZ GAVE THE ORDER. REEVES EXECUTED IT.
TWO LINKS. TWO PEOPLE WHO MADE IT HAPPEN.

STEP SIZE: 2
```

**Audio:** Two heavy metallic impacts (the "links"). Then the "echo" motif returns, now close, now clear.

---

#### Scene 31: ROUND 3 COMPLETE / FINAL META UNLOCK

**ID:** `ACT3_COMPLETE`
**Location:** 4-01-02 Security Office
**Trigger:** R3-META solved
**Display:**

```
CREW RECORD RECONSTRUCTION: COMPLETE
AUTHORIZATION CHAIN: VASQUEZ ──> REEVES (2 LINKS)

COMMISSION ORDER #2247 — FINAL UPDATE
All evidence collected. The commission decoder is configured.
Report to the Briefing Room, Deck 1, for final reconstruction.

N = 8 (PHASE-LOCKED ECHO)
START = 3 (KWON — FIRST ALERT)
STEP = 2 (AUTHORIZATION CHAIN LENGTH)

THE BRIEFING ROOM IS UNLOCKED.
```

**Audio:** Three ascending tones (one per round meta value: 8, 3, 2). Then a door unlock sound — the Briefing Room.
**Duration:** 10 seconds.
**Exits:** Briefing Room (1-02-02) unlocks. Navigation phase enters FINAL.

---

#### Scene 32: THE REVEAL (Final Meta — The Commission)

**ID:** `ACT3_THE_REVEAL`
**Location:** 1-02-02 Briefing Room
**Trigger:** Player enters Briefing Room
**Display:**

The Briefing Room is formal. Four auxiliary consoles surround a central display table. The central display activates:

```
COMMISSION DECODER — CLASSIFIED
Terminal: CMD-DECODER-01
Mode: CyclicGroupDisplay
Configuration: AWAITING PARAMETERS
```

The CyclicGroupDisplay renders: an 8-point circle appears on the central display. Labels populate clockwise:

```
        0: TORRES
      /            \
  7: CHEN            1: NAKAMURA
  |                    |
  6: PARK            2: OKAFOR
  |                    |
  5: VASQUEZ          3: KWON
      \            /
        4: REEVES
```

Three parameter slots appear:

```
N (CIRCLE SIZE):       8    [CONFIRMED — PHASE-LOCKED ECHO]
START (POSITION):      3    [CONFIRMED — KWON / FIRST ALERT]
STEP (INCREMENT):      2    [CONFIRMED — CHAIN LENGTH]
```

An LCARSButton: "EXECUTE COMMISSION DECODER."

The player presses the button.

**Animation sequence (5 seconds):**

1. A pointer appears at position 3 (KWON). KWON's label brightens. (1 second)
2. The pointer begins to move clockwise. It passes position 4 (REEVES). REEVES brightens briefly, then dims. (2 seconds)
3. The pointer lands on position 5. (1 second)
4. Position 5 label enlarges: **VASQUEZ**. The entire circle dims except position 5. (1 second)

**Final display:**

```
COMMISSION DECODER — RESULT
━━━━━━━━━━━━━━━━━━━━━━━━━━

  START:  3 (KWON)
  STEP:   +2
  RESULT: (3 + 2) mod 8 = 5

  ████████████████████████
  ██                    ██
  ██     VASQUEZ        ██
  ██     Position 5     ██
  ██     Commanding     ██
  ██     Officer        ██
  ██                    ██
  ████████████████████████

THE COMMISSION DECODER NAMES THE OFFICER WHO
ORDERED THE COVER-UP. THE INSTRUMENTS DO NOT LIE.
THE CHAIN DOES NOT BREAK.

VASQUEZ.
```

**Audio:** This is THE REVEAL. When the player presses EXECUTE:
- A clock mechanism winds up (1 second).
- The pointer movement is accompanied by a slow, resonant tick.
- When the pointer passes REEVES, a brief echo of the chain link sound.
- When the pointer lands on VASQUEZ, the "echo" motif plays one final time — full volume, clear, unmistakable. Not distant. Not faint. Present.
- Then complete silence for 5 seconds. The name sits on screen.

---

#### Scene 33: THE RECONSTRUCTION (Post-Reveal Narrative)

**ID:** `ACT3_RECONSTRUCTION`
**Location:** 1-02-02 Briefing Room
**Trigger:** 5 seconds after THE REVEAL animation completes
**Display:**

The central display transitions. The CyclicGroupDisplay shrinks to a corner reference. The main screen fills with a scrolling commission reconstruction:

```
COMMISSION RECONSTRUCTION — INCIDENT REPORT
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

WHAT HAPPENED:
  At hour 1822, the ship was contacted by a Phase-Locked Echo —
  an entity that locked onto the ship's outbound signal at a 3:1
  harmonic ratio. It held formation at 51 km with 4 escort contacts.
  It transmitted structured data at 384 kbps for 47 minutes.

  The ship did not encounter the Phase-Locked Echo by accident.
  The Echo encountered the ship. And the ship was ready.

HOW THE SHIP RESPONDED:
  The response was pre-planned. The shields were set to a defensive
  posture BEFORE the contact was officially detected. The EPS grid
  was reconfigured for station-keeping power. The reactor was held
  at throttle 4 — moderate, precise, waiting.

  Lt. Kwon at COMPUTER detected the first anomaly at GAP +00:14.
  At GAP +01:22, Kwon collapsed — atmospheric suppressant, dispersed
  through environmental systems. The first alert went unacknowledged.
  This was not a coincidence.

WHO ORDERED THE COVER-UP:
  Captain Vasquez (Badge 401) forced three restricted doors during
  the gap. She authorized herself via CO privilege. She authorized
  Lt. Cmdr. Reeves to execute the 12-operation emergency deflector
  bypass — a classified procedure from the addendum to Contact
  Protocol Seven that bypasses the CO confirmation step.

  Chen accessed the restricted Sensor Bay using a pre-distributed
  code. The ODN breach originated from the Security Computer.
  The sensor recording relay was deliberately burned.

  Every thread traces to Vasquez. She knew the encounter was coming.
  She prepared the ship. She incapacitated the one officer who would
  have detected the unauthorized activity. She forced her way through
  restricted areas. She ordered the logs deleted before the encounter
  ended.

  Vasquez did not encounter the Phase-Locked Echo.
  Vasquez met it.

COMMISSION FINDING:
  Captain R. Vasquez ordered the cover-up.
  The authorization chain has 2 links.
  The instruments do not lie.
```

**Audio:** Scrolling text is accompanied by a quiet, steady tone — no music, just the hum of the ship's systems. As the text reaches "Vasquez met it," the "echo" motif plays its final iteration — this time harmonized, resolved, at rest. The motif was the contact's signature all along.
**Duration:** Player-paced scrolling (approximately 60 seconds to read). Player clicks ACKNOWLEDGE at the bottom.
**Exits:** EPILOGUE phase begins.

---

### EPILOGUE (Scenes 34-35)

---

#### Scene 34: THE DECISION

**ID:** `EPILOGUE_DECISION`
**Location:** 1-02-02 Briefing Room
**Trigger:** Player clicks ACKNOWLEDGE on the reconstruction
**Display:**

The reconstruction text clears. A final commission prompt appears:

```
COMMISSION ORDER #2247 — FINAL ACTION

Your reconstruction is complete. The evidence names
Captain Vasquez as the officer who ordered the cover-up.

The sealed order to purge backup instrument logs
executes in [COUNTDOWN: 71:42:18].

You have two options:

  [SUBMIT FINDINGS]     [CLOSE COMMISSION FILE]

SUBMIT: Your findings are transmitted to Sector Command.
        The backup logs are preserved. Vasquez faces tribunal.
        Whatever the Phase-Locked Echo is, it becomes known.

CLOSE:  Your findings are sealed. The backup logs are purged
        on schedule. No one knows what you found.
        Whatever the Phase-Locked Echo is, it stays between
        Vasquez and the silence.

This decision has no correct answer.
The instruments told you what happened.
They cannot tell you what to do about it.
```

Two LCARSButtons: SUBMIT FINDINGS (amber) and CLOSE COMMISSION FILE (grey).

**Audio:** Ship ambient only. The quietest moment in the game. No music. No motifs. Just the reactor hum and the player's choice.
**Duration:** Player-paced. No timeout.
**Exits:** Either button leads to Scene 35 (different text variants).

---

#### Scene 35: THE AFTERMATH

**ID:** `EPILOGUE_AFTERMATH`
**Location:** 1-03-01 Observation Lounge
**Trigger:** Player makes their choice in Scene 34
**Display:**

The Observation Lounge door opens. The room is dark except for the viewport — stars, the vastness of space.

**If SUBMIT FINDINGS was chosen:**

```
COMMISSION ORDER #2247 — FILED

Your findings have been transmitted to Sector 7 Command.
The backup instrument logs have been flagged for preservation.
A tribunal convening order has been issued for Captain Vasquez.

The Phase-Locked Echo held formation at 51 km for 47 minutes.
It transmitted 384 kilobits per second. It chose the harmonic.
It chose to respond. It is still out there.

And now, because of what you did, someone else will know.

DEAD RECKONING
Incident reconstruction complete.
```

**If CLOSE COMMISSION FILE was chosen:**

```
COMMISSION ORDER #2247 — SEALED

Your commission file has been sealed.
The backup instrument logs will be purged in 71 hours.
No findings will be transmitted.

The Phase-Locked Echo held formation at 51 km for 47 minutes.
It transmitted 384 kilobits per second. It chose the harmonic.
It chose to respond. It is still out there.

Vasquez knows. And now, so do you.
No one else ever will.

DEAD RECKONING
Incident reconstruction complete.
```

**Audio:** Viewport ambient — a deep, wide space sound. If SUBMIT: a distant comm transmission chime (the findings going out). If CLOSE: the sound of a file seal (a sharp, final click). Then silence. Then the "echo" motif one last time — but this time it is outside the ship, coming from the viewport direction. Very faint. Very far. Still there.

**Duration:** 15 seconds of text, then the game holds on the viewport indefinitely until the player exits.
**Exits:** Credits overlay. Game complete.

---

## Section 4: React State Machine

### TypeScript Interface

```typescript
// ============================================================
// DEAD RECKONING v2 — Game State Machine
// ============================================================

// --- Enums ---

enum GamePhase {
  OPENING = "OPENING",
  COMMISSION = "COMMISSION",
  ROUND_1 = "ROUND_1",
  R1_META = "R1_META",
  ROUND_2 = "ROUND_2",
  R2_META = "R2_META",
  ROUND_3 = "ROUND_3",
  R3_META = "R3_META",
  FINAL = "FINAL",
  EPILOGUE = "EPILOGUE",
  COMPLETE = "COMPLETE",
}

enum RoomState {
  OPEN = "OPEN",
  LOCKED = "LOCKED",
  SECURED = "SECURED",
  FORCED = "FORCED",
}

enum PuzzleState {
  LOCKED = "LOCKED",       // Not yet accessible
  AVAILABLE = "AVAILABLE", // Room open, puzzle ready
  IN_PROGRESS = "IN_PROGRESS", // Player has interacted
  SOLVED = "SOLVED",       // Correct answer submitted
}

enum SystemState {
  ONLINE = "ONLINE",
  READY = "READY",
  STANDBY = "STANDBY",
  LOCKED = "LOCKED",
  OFFLINE = "OFFLINE",
  FAULT = "FAULT",
}

enum EpilogueChoice {
  NONE = "NONE",
  SUBMIT = "SUBMIT",
  CLOSE = "CLOSE",
}

// --- Puzzle Answer Types ---

interface PuzzleAnswer {
  puzzleId: string;
  value: number;
  unit: string;
  timestamp: number; // ms since game start
}

// --- Room State ---

interface RoomInfo {
  roomId: string;         // e.g., "1-01-01"
  name: string;           // e.g., "Bridge — Center"
  deck: number;           // 1-5
  state: RoomState;
  puzzleIds: string[];    // puzzles accessible from this room
  consoleCount: number;
  unlockCondition: string | null; // human-readable, null if always open
}

// --- Player State ---

interface PlayerState {
  currentRoom: string;    // roomId
  currentDeck: number;    // 1-5
  visitedRooms: Set<string>;
  hintsUsed: Map<string, number>; // puzzleId -> hint tier (1/2/3)
}

// --- Meta State ---

interface MetaState {
  r1Meta: {
    n: number | null;            // 8 when solved
    contactType: string | null;  // "PHASE_LOCKED_ECHO" when solved
    solved: boolean;
  };
  r2Meta: {
    startPosition: number | null;  // 3 when solved
    firstResponder: string | null; // "KWON" when solved
    solved: boolean;
  };
  r3Meta: {
    stepSize: number | null;     // 2 when solved
    chainLength: number | null;  // 2 when solved
    solved: boolean;
  };
  finalMeta: {
    result: number | null;       // 5 when solved
    officerName: string | null;  // "VASQUEZ" when solved
    solved: boolean;
  };
}

// --- Commission Decoder ---

interface CyclicGroupConfig {
  n: number | null;         // circle size (8)
  start: number | null;     // starting position (3)
  step: number | null;      // step size (2)
  labels: string[];         // ["TORRES","NAKAMURA","OKAFOR","KWON","REEVES","VASQUEZ","PARK","CHEN"]
  result: number | null;    // landing position (5)
  executed: boolean;
}

// --- Scene Tracking ---

interface SceneState {
  currentScene: string | null;   // scene ID
  scenesPlayed: Set<string>;     // all scene IDs that have been shown
  cutsceneActive: boolean;       // true during non-interactive sequences
}

// --- Hint System ---

interface HintState {
  available: Map<string, number>;  // puzzleId -> max tier available
  revealed: Map<string, number>;   // puzzleId -> highest tier revealed
}

// --- Audio State ---

interface AudioState {
  ambientTrack: string;          // current ambient loop ID
  echoMotifPlayed: number;       // count of echo motif plays
  lastSfx: string | null;
}

// --- Master Game State ---

interface GameState {
  // Core progression
  phase: GamePhase;
  startTime: number;                  // Unix ms
  elapsedTime: number;                // ms since start

  // Puzzle tracking
  puzzles: Map<string, PuzzleState>;  // puzzleId -> state
  answers: Map<string, PuzzleAnswer>; // puzzleId -> answer

  // Round completion
  round1Solved: number;               // count of R1 feeders solved (0-6)
  round2Solved: number;               // count of R2 feeders solved (0-6)
  round3Solved: number;               // count of R3 feeders solved (0-5)

  // Meta state
  meta: MetaState;
  decoder: CyclicGroupConfig;

  // Navigation
  rooms: Map<string, RoomInfo>;       // roomId -> room info
  player: PlayerState;

  // Narrative
  scenes: SceneState;
  epilogueChoice: EpilogueChoice;

  // Support systems
  hints: HintState;
  audio: AudioState;

  // Save/load
  saveSlot: number;
  lastSave: number;                   // Unix ms
}
```

### State Transitions

Every state transition is triggered by a specific event. Below is the complete transition table.

#### Phase Transitions

| Current Phase | Event | Next Phase | Side Effects |
|---------------|-------|------------|--------------|
| OPENING | Scene 1 completes | COMMISSION | Ready Room door opens |
| COMMISSION | Scene 2 ACKNOWLEDGE | ROUND_1 | Turbolift unlocks all decks; R1 rooms open |
| ROUND_1 | round1Solved >= 4 | R1_META | Bridge Science console unlocks |
| R1_META | R1-META solved (n=8) | ROUND_2 | R2 rooms unlock; narrative beat triggers |
| ROUND_2 | round2Solved >= 4 | R2_META | Bridge Center CMD console unlocks |
| R2_META | R2-META solved (start=3) | ROUND_3 | R3 rooms unlock; Computer Core/Brig open |
| ROUND_3 | round3Solved >= 3 | R3_META | Security Office AUTH console unlocks |
| R3_META | R3-META solved (step=2) | FINAL | Briefing Room unlocks |
| FINAL | Final meta solved (VASQUEZ) | EPILOGUE | Observation Lounge unlocks |
| EPILOGUE | Player chooses SUBMIT or CLOSE | COMPLETE | Credits |

#### Puzzle State Transitions

| Current State | Event | Next State | Condition |
|---------------|-------|------------|-----------|
| LOCKED | Phase advances | AVAILABLE | Room containing puzzle is now open |
| AVAILABLE | Player enters room | IN_PROGRESS | First interaction with console |
| IN_PROGRESS | Correct answer | SOLVED | Answer matches expected value |
| IN_PROGRESS | Wrong answer | IN_PROGRESS | Error message shown; retry allowed |

#### Room Unlock Triggers

| Room | Unlocked When | Phase |
|------|---------------|-------|
| 1-01-01 Bridge Center | Game start | OPENING |
| 1-02-01 Ready Room | Scene 1 completes | COMMISSION |
| 2-01-01 Communications | Scene 2 ACKNOWLEDGE | ROUND_1 |
| 2-01-02 Tactical Control | Scene 2 ACKNOWLEDGE | ROUND_1 |
| 2-01-03 Nav Control | Scene 2 ACKNOWLEDGE | ROUND_1 |
| 4-01-01 Science Lab | Scene 2 ACKNOWLEDGE | ROUND_1 |
| 1-01-05 Bridge Science | round1Solved >= 4 | R1_META |
| 5-01-01 Main Eng Power | R1-META solved | ROUND_2 |
| 5-01-03 Main Eng Computer | R1-META solved | ROUND_2 |
| 5-02-01 EPS Control | R1-META solved | ROUND_2 |
| 5-02-02 Environmental Control | R1-META solved | ROUND_2 |
| 1-01-03 Bridge Tactical | R1-META solved | ROUND_2 |
| 1-01-01 Bridge Center (CMD) | round2Solved >= 4 | R2_META |
| 3-01-01 Sickbay | R2-META solved | ROUND_3 |
| 4-01-02 Security Office | R2-META solved | ROUND_3 |
| 5-03-01 Computer Core | R2-META solved | ROUND_3 |
| 5-02-03 Deflector Control | R2-META solved | ROUND_3 |
| 4-01-02 Security AUTH console | round3Solved >= 3 | R3_META |
| 1-02-02 Briefing Room | R3-META solved | FINAL |
| 1-03-01 Observation Lounge | Final meta solved | EPILOGUE |

#### Answer Validation

```typescript
// Answer validation — exact match for integers, tolerance for decimals
const ANSWER_KEY: Record<string, { value: number; tolerance: number; unit: string }> = {
  "R1-01": { value: 2.340, tolerance: 0.005, unit: "GHz" },
  "R1-02": { value: 873,   tolerance: 0,     unit: "MHz" },
  "R1-03": { value: 213,   tolerance: 0,     unit: "deg" },
  "R1-04": { value: 0.73,  tolerance: 0.005, unit: "" },
  "R1-05": { value: 5,     tolerance: 0,     unit: "contacts" },
  "R1-06": { value: 384,   tolerance: 0,     unit: "kbps" },
  "R1-META": { value: 8,   tolerance: 0,     unit: "" },
  "R2-01": { value: 2,     tolerance: 0,     unit: "conduits" },
  "R2-02": { value: 7,     tolerance: 0,     unit: "nodeID" },
  "R2-03": { value: 14,    tolerance: 0,     unit: "gridCoord" },
  "R2-04": { value: 65,    tolerance: 0,     unit: "%" },
  "R2-05": { value: 9,     tolerance: 0,     unit: "componentID" },
  "R2-06": { value: 4,     tolerance: 0,     unit: "position" },
  "R2-META": { value: 3,   tolerance: 0,     unit: "" },
  "R3-01": { value: 3,     tolerance: 0,     unit: "patientID" },
  "R3-02": { value: 7,     tolerance: 0,     unit: "position" },
  "R3-03": { value: 4,     tolerance: 0,     unit: "element" },
  "R3-04": { value: 5,     tolerance: 0,     unit: "position" },
  "R3-05": { value: 12,    tolerance: 0,     unit: "operations" },
  "R3-META": { value: 2,   tolerance: 0,     unit: "links" },
  "FINAL":  { value: 5,    tolerance: 0,     unit: "position" },
};

function validateAnswer(puzzleId: string, submitted: number): boolean {
  const key = ANSWER_KEY[puzzleId];
  if (!key) return false;
  return Math.abs(submitted - key.value) <= key.tolerance;
}
```

#### Save/Load Contract

```typescript
interface SaveData {
  version: number;          // schema version for migration
  gameState: GameState;     // full serializable state
  checksum: string;         // integrity check
  timestamp: number;        // Unix ms
}

// Auto-save triggers:
// - After each puzzle solved
// - On phase transition
// - On room change
// - Every 5 minutes of active play
// - Before epilogue choice

// Save slots: 3 manual + 1 auto
```

### Widget Component Map

Each puzzle maps to specific React components. The widget names match the console-widget-catalog.

| Widget Class | React Component | Used By |
|-------------|----------------|---------|
| SineWaveDisplay | `<SineWaveDisplay />` | R1-01 |
| LissajousDisplay | `<LissajousDisplay />` | R1-02 |
| PhaseInterferenceDisplay | `<PhaseInterferenceDisplay />` | R1-03 |
| ConicSectionDisplay | `<ConicSectionDisplay />` | R1-04 |
| RadarSweepDisplay | `<RadarSweepDisplay />` | R1-05 |
| TargetingReticleDisplay | `<TargetingReticleDisplay />` | R1-05 |
| CommSignalDisplay | `<CommSignalDisplay />` | R1-06 |
| ConduitFlowDisplay | `<ConduitFlowDisplay />` | R2-01 |
| NetworkGridDisplay | `<NetworkGridDisplay />` | R2-02 |
| HeatMapDisplay | `<HeatMapDisplay />` | R2-03 |
| ShieldDisplay | `<ShieldDisplay />` | R2-04 |
| CircuitTopologyDisplay | `<CircuitTopologyDisplay />` | R2-05 |
| GaugeDisplay | `<GaugeDisplay />` | R2-01, R2-03, R2-06 |
| ThrottleLever | `<ThrottleLever />` | R2-06 |
| LifesignsDisplay | `<LifesignsDisplay />` | R3-01 |
| ModularClockDisplay | `<ModularClockDisplay />` | R3-02 |
| CayleyTableDisplay | `<CayleyTableDisplay />` | R3-03 |
| IndicatorPanel | `<IndicatorPanel />` | R1-META, R2-META, R3-04, R3-05 |
| BatSwitch | `<BatSwitch />` | R3-05 |
| ScrollingTextDisplay | `<ScrollingTextDisplay />` | R3-02, R3-04, R3-05 |
| CyclicGroupDisplay | `<CyclicGroupDisplay />` | FINAL META |
| NumericStepper | `<NumericStepper />` | R1-01, R1-02, R1-04, R1-06, R3-02 |
| RotaryDial | `<RotaryDial />` | R1-01, R1-02, R1-04, R1-06, R3-01, R3-03 |
| ToggleSwitch | `<ToggleSwitch />` | R1-03, R2-01, R2-02 |
| PowerSlider | `<PowerSlider />` | R2-04, R2-06 |
| LCARSButton | `<LCARSButton />` | R2-03, R2-05, R3-03, FINAL |
| LightedButton | `<LightedButton />` | R1-05 |
| LinkedSliderInput | `<LinkedSliderInput />` | R1-03, R3-01 |
| NumericInput | `<NumericInput />` | R1-META, R2-META, R3-META |
| MasterAlarm | `<MasterAlarm />` | R3-05 |
| AutoProcedureButton | `<AutoProcedureButton />` | R3-05 |

---

## Section 5: Multiple Ships / Future Missions

### Ship Classes in the SPACEGAMIVERSE

The same systems architecture scales across 6 ship classes. Each class adds complexity but never changes the fundamental structure (4 divisions, 16 departments, same power tiers, same console model).

| Class | Decks | Rooms | Crew | ODN Nodes | Puzzle Scale |
|-------|-------|-------|------|-----------|--------------|
| Capsule | 0 | 1 cabin | 1 | 0 | Tutorial only |
| Fighter | 0 | 1 cockpit | 1-2 | 1 | 2-3 puzzles |
| Shuttlepod | 0 | 1 compartment | 1-4 | 2 | 4-6 puzzles |
| Shuttle | 1 | 2 compartments | 2-8 | 3 | 6-10 puzzles |
| Corvette | 3 | ~12 rooms | 10-30 | 6 | 10-15 puzzles |
| **Cruiser** | **5** | **~28 rooms** | **50-200** | **12** | **17-25 puzzles** |

DEAD RECKONING v2 runs on a Cruiser. Future missions can use any class.

### Career Progression

The SPACEGAMIVERSE career model provides a natural difficulty arc across missions:

| Rank | Console Access | Meta-Controls | Mission Type |
|------|---------------|---------------|-------------|
| Technician | AUTO only | AUTO | Tutorial missions — follow procedures |
| Rotation | New stations unlocked | AUTO | Cross-training — learn multiple departments |
| Diagnostician | DIAGNOSTIC unlocked | AUTO + DIAGNOSTIC | Investigation missions — find what is wrong |
| Senior Officer | CALIBRATE unlocked | All three | Command missions — make decisions |

DEAD RECKONING v2 operates at the **Diagnostician** level — the player is investigating, diagnosing, reconstructing. A future mission could operate at Senior Officer level, where the player must calibrate systems and make command decisions under pressure.

### Mission Architecture

Each mission (scenario) uses the same structural pattern:

```
MISSION = {
  ship: ShipClass,
  setting: NarrativeFrame,
  rounds: Round[],        // 2-4 rounds, each with feeders + meta
  finalMeta: FinalMeta,   // combines round-meta outputs
  epilogue: Epilogue,     // player choice or revelation
}

ROUND = {
  name: string,
  theme: string,          // what kind of evidence
  feeders: Puzzle[],      // 4-8 puzzles
  meta: MetaPuzzle,       // combines feeder values
}

PUZZLE = {
  id: string,
  room: RoomId,
  widgets: WidgetConfig[], // from console-widget-catalog
  answer: NumericAnswer,
  referenceCard: string,   // always present
  solvePathNovice: string,
  solvePathExpert: string,
}
```

### Future Mission Concepts

The engine built for DEAD RECKONING v2 supports any scenario in the SPACEGAMIVERSE:

| Mission Concept | Ship Class | Rank Level | Rounds | Core Mechanic |
|----------------|------------|------------|--------|---------------|
| DEAD RECKONING v2 | Cruiser | Diagnostician | 3 (6+6+5) | Instrument operation |
| COLD START | Shuttlepod | Technician | 2 (3+3) | Power-up sequence (bring systems online in order) |
| BLIND SPOT | Corvette | Rotation | 2 (4+4) | Sensor calibration (find what the sensors miss) |
| CHAIN OF COMMAND | Cruiser | Senior Officer | 4 (5+5+5+5) | Authorization and triage under crisis |
| FIRST CONTACT | Shuttle | Diagnostician | 2 (4+3) | Signal decoding (classify unknown transmission) |
| GHOST SHIP | Cruiser | Diagnostician | 3 (5+6+4) | System forensics (reconstruct crew timeline) |

### Shared Infrastructure

All missions share:

1. **Widget library** — the console-widget-catalog is ship-agnostic. SineWaveDisplay works on a shuttlepod or a cruiser.
2. **Navigation system** — scales from "click panel positions in one room" (shuttlepod) to "turbolift between 5 decks" (cruiser).
3. **State machine** — same GamePhase enum, same transition model. More or fewer rounds, but same structure.
4. **Answer validation** — always numeric. Always instrument-derived.
5. **Save system** — same SaveData contract.
6. **Hint system** — 3-tier hints per puzzle (Nudge, Direction, Near-Solution).
7. **Audio system** — ambient per deck/room, SFX per widget interaction, motif per narrative arc.

### Implementation Priority

For the React/TypeScript build, implement in this order:

1. **Navigation engine** — deck maps, room transitions, turbolift, door states
2. **Widget library** — build each widget as an isolated React component with standardized props
3. **Puzzle container** — wraps widgets + reference card + answer validation + solve tracking
4. **State machine** — GameState management, phase transitions, save/load
5. **Scene system** — cutscene renderer, text display, animation sequences
6. **Audio engine** — ambient loops, SFX triggers, motif system
7. **HUD** — progress tracker, minimap, hint button
8. **Polish** — transitions, loading states, accessibility, responsive layout

### Component Architecture

```
<App>
  <GameStateProvider>
    <AudioEngine />
    <HUD>
      <ProgressTracker />
      <MiniMap />
      <HintButton />
    </HUD>
    <NavigationEngine>
      <DeckMap deck={currentDeck}>
        <Room roomId={currentRoom}>
          <PuzzleContainer puzzleId={activePuzzle}>
            <WidgetPanel>
              {/* Display widgets */}
              <SineWaveDisplay config={...} />
              {/* Input controls */}
              <NumericStepper config={...} />
              <RotaryDial config={...} />
            </WidgetPanel>
            <ReferenceCard content={...} />
            <AnswerValidator puzzleId={...} onSolve={...} />
          </PuzzleContainer>
        </Room>
      </DeckMap>
    </NavigationEngine>
    <SceneRenderer scene={currentScene} />
    <SaveManager />
  </GameStateProvider>
</App>
```

---

## Appendix A: Audio Design Notes

### Ambient Tracks (per location)

| Location | Ambient | Notes |
|----------|---------|-------|
| Bridge | Low reactor hum, distant ventilation, periodic console chimes | Quiet, professional |
| Operations (Deck 2) | Comm static, signal processing tones, data flow | Busier than bridge |
| Crew (Deck 3) | Softer hum, occasional intercom, life support sounds | Residential, calm |
| Support (Deck 4) | Security beeps, lab equipment, cargo bay echoes | Functional, institutional |
| Engineering (Deck 5) | Loud reactor, plasma conduit flow, heavy equipment | Industrial, alive |
| Turbolift | Mechanical movement, deceleration | Transit only |
| Briefing Room | Near-silence, occasional console hum | Formal, tense |
| Observation Lounge | Deep space ambient, hull creaks | Vast, empty |

### Motif: The Echo

The Phase-Locked Echo has a musical signature — a short tonal phrase that appears at key narrative moments:

| Occurrence | Scene | Volume | Quality |
|-----------|-------|--------|---------|
| 1st | Scene 10 (R1-META solve) | Quiet | Distant, faint |
| 2nd | Scene 19 (THE TURN) | Low | Closer, clearer |
| 3rd | Scene 23 (Act 2 narrative) | Medium | Present, recognizable |
| 4th | Scene 30 (R3-META solve) | Medium-high | Close, clear |
| 5th | Scene 32 (THE REVEAL) | Full | Resolved, complete |
| 6th | Scene 33 (Reconstruction) | Full, harmonized | At rest |
| 7th | Scene 35 (Aftermath) | Faint, external | Outside the ship |

The motif is a 3:1 harmonic relationship (matching the contact's signal ratio). It consists of two tones: a base note and a note at 3x the frequency. As the motif recurs, it gains harmonic richness — additional overtones, longer sustain, warmer timbre — mirroring the solver's growing understanding.

---

## Appendix B: Accessibility Notes

- All puzzle states must be describable in text (for screen readers)
- Color is never the sole indicator of state (always paired with shape, label, or pattern)
- All interactive controls must be keyboard-navigable (Tab, Enter, Arrow keys)
- Reference cards are always available as expandable text panels
- The text fallback mode (described in SCOPE.md) serves as the accessibility-first version
- Audio cues are supplementary, never required for puzzle solving
- High-contrast mode for all console displays (toggle in settings)
- Adjustable text size for all narrative and reference content
