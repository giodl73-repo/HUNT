# DEAD RECKONING v2 — Scene Architecture

**Document type:** Complete Phaser 4 Scene inventory
**Audience:** Game developer implementing the Phaser 4 build
**Companion to:** `GAME-SCRIPT.md` (full narrative script), `ASSET-LIST.md` (asset requirements), `THEME.md` (visual design)

---

## Scene Naming Convention

Scene keys follow the pattern: `Deck{N}_{RoomShortName}`
Narrative scenes use: `Narrative_{SceneID}`

---

## DECK 1 — COMMAND (Division: RED `#8B0000`)

### Scene: `Deck1_BridgeCenter`

| Field | Value |
|-------|-------|
| **Room ID** | 1-01-01 |
| **Room** | Bridge — Center |
| **Puzzle Panels** | R2-META: Station Command Log Console |
| **Widgets** | `IndicatorPanel` (6 rows, parity-highlighted), reference table display, `NumericInput` |
| **Entry Trigger** | Game start (OPENING phase); R2-META console locked until `round2Solved >= 4` |
| **Exit Options** | Bridge Helm (1-01-02), Bridge OPS (1-01-04), Ready Room (1-02-01 — door, always open), Turbolift |
| **Ambient Audio** | Low reactor hum, distant ventilation, periodic console chimes. Quiet, professional. |
| **Lighting** | Warm command red, low ambient. Emergency amber strips during OPENING phase; standard operational lighting after COMMISSION phase. |
| **Special Events** | `ACT1_COLD_OPEN` (game start), `ACT2_R2_META` (R2-META solve), `ACT2_COMPLETE` (Round 2 transition) |

---

### Scene: `Deck1_BridgeHelm`

| Field | Value |
|-------|-------|
| **Room ID** | 1-01-02 |
| **Room** | Bridge — Helm |
| **Puzzle Panels** | None (flavor only — Helm console with navigation readouts) |
| **Widgets** | None interactive |
| **Entry Trigger** | COMMISSION phase (turbolift unlocked) |
| **Exit Options** | Bridge Center (1-01-01), Bridge Tactical (1-01-03) |
| **Ambient Audio** | Bridge ambient (shared with Bridge Center) |
| **Lighting** | Warm command red, navigational display glow |
| **Special Events** | None |

---

### Scene: `Deck1_BridgeTactical`

| Field | Value |
|-------|-------|
| **Room ID** | 1-01-03 |
| **Room** | Bridge — Tactical |
| **Puzzle Panels** | R2-04: Shield Configuration Console |
| **Widgets** | `ShieldDisplay` (3-layer hexagonal), `PowerSlider` (x3), `GaugeDisplay` (threat) |
| **Entry Trigger** | R1-META solved (ROUND_2 phase) |
| **Exit Options** | Bridge Center (1-01-01), Bridge Helm (1-01-02) |
| **Ambient Audio** | Bridge ambient + shield harmonic hum when panel active |
| **Lighting** | Warm command red with tactical overlay glow (amber threat indicators) |
| **Special Events** | `ACT2_R2_04` (shield profile solve — ominous chord on match confirmation, THE TURN preparation) |

---

### Scene: `Deck1_BridgeOPS`

| Field | Value |
|-------|-------|
| **Room ID** | 1-01-04 |
| **Room** | Bridge — OPS |
| **Puzzle Panels** | None (OPS status displays only) |
| **Widgets** | None interactive |
| **Entry Trigger** | COMMISSION phase |
| **Exit Options** | Bridge Center (1-01-01), Bridge Science (1-01-05) |
| **Ambient Audio** | Bridge ambient |
| **Lighting** | Warm command red |
| **Special Events** | None |

---

### Scene: `Deck1_BridgeScience`

| Field | Value |
|-------|-------|
| **Room ID** | 1-01-05 |
| **Room** | Bridge — Science |
| **Puzzle Panels** | R1-META: Contact Signature Classification Console |
| **Widgets** | `IndicatorPanel` (6 feeder values), reference table (8-row Contact Signature Reference), `NumericInput` |
| **Entry Trigger** | `round1Solved >= 4` (R1_META phase) |
| **Exit Options** | Bridge OPS (1-01-04) |
| **Ambient Audio** | Bridge ambient + classification database loading sound on entry |
| **Lighting** | Cool science-station blue overlaid on command red. Science console glow dominates. |
| **Special Events** | `ACT1_R1_META` (contact identified — echo motif first appearance), `ACT1_COMPLETE` (Round 1 transition) |

---

### Scene: `Deck1_ReadyRoom`

| Field | Value |
|-------|-------|
| **Room ID** | 1-02-01 |
| **Room** | Ready Room |
| **Puzzle Panels** | None |
| **Widgets** | `ScrollingTextDisplay` (commission document) |
| **Entry Trigger** | Scene 1 completes (COMMISSION phase) |
| **Exit Options** | Bridge Center (1-01-01), Turbolift |
| **Ambient Audio** | Console activation hum. Quiet. |
| **Lighting** | Warm amber, formal. Single console glow. |
| **Special Events** | `ACT1_BRIEFING` (commission assignment — player reads and acknowledges) |

---

### Scene: `Deck1_BriefingRoom`

| Field | Value |
|-------|-------|
| **Room ID** | 1-02-02 |
| **Room** | Briefing Room |
| **Puzzle Panels** | FINAL META: Commission Briefing — CyclicGroupDisplay |
| **Widgets** | `CyclicGroupDisplay` (8-point circle, duty roster labels), `NumericInput` (x3 — N, start, step, pre-filled from round metas), `LCARSButton` ("EXECUTE COMMISSION DECODER") |
| **Entry Trigger** | R3-META solved (FINAL phase) — door was LOCKED prior |
| **Exit Options** | Observation Lounge (1-03-01 — unlocks after final meta solved) |
| **Ambient Audio** | Near-silence. Occasional console hum. Formal, tense. |
| **Lighting** | Cool, formal. Central display table illuminated. Four auxiliary console positions dim. |
| **Special Events** | `ACT3_THE_REVEAL` (5-second pointer animation, echo motif at full volume, 5 seconds silence), `ACT3_RECONSTRUCTION` (scrolling incident report), `EPILOGUE_DECISION` (SUBMIT/CLOSE choice) |

---

### Scene: `Deck1_ObservationLounge`

| Field | Value |
|-------|-------|
| **Room ID** | 1-03-01 |
| **Room** | Observation Lounge |
| **Puzzle Panels** | None (epilogue scene) |
| **Widgets** | `ScrollingTextDisplay` (aftermath text — variant based on SUBMIT/CLOSE choice) |
| **Entry Trigger** | Final meta solved (EPILOGUE phase) |
| **Exit Options** | None (game complete — credits overlay) |
| **Ambient Audio** | Deep space ambient. Hull creaks. Vast, empty. Echo motif final iteration — faint, external, from viewport direction. |
| **Lighting** | Dark except viewport. Stars. The vastness of space. |
| **Special Events** | `EPILOGUE_AFTERMATH` (15 seconds text, then hold indefinitely on viewport) |

---

## DECK 2 — OPERATIONS (Division: YELLOW `#DAA520`)

### Scene: `Deck2_Communications`

| Field | Value |
|-------|-------|
| **Room ID** | 2-01-01 |
| **Room** | Communications Station |
| **Puzzle Panels** | R1-01: Bandpass Filter Console (Station 1, left); R1-06: Vector Signal Analyzer Console (Station 2, right) |
| **Widgets — R1-01** | `SineWaveDisplay` (composite waveform + target overlay), `NumericStepper` (filter center), `RotaryDial` (step size), `LCARSButton` (apply filter) |
| **Widgets — R1-06** | `CommSignalDisplay` (IQ constellation), `RotaryDial` (modulation type), `NumericStepper` (symbol rate) |
| **Entry Trigger** | COMMISSION phase (ROUND_1) |
| **Exit Options** | Tactical Control (2-01-02), Transporter Room 1 (2-02-01 — corridor) |
| **Ambient Audio** | Comm static, signal processing tones, data flow. Busier than bridge. Faint static from signal buffer playback. |
| **Lighting** | Amber operations lighting. Console glow from two active stations. |
| **Special Events** | `ACT1_R1_01` (first instrument — 3-second power-up sequence), `ACT1_R1_06` (constellation solve — "384 kbps IS NOT A BEACON") |

---

### Scene: `Deck2_TacticalControl`

| Field | Value |
|-------|-------|
| **Room ID** | 2-01-02 |
| **Room** | Tactical Control |
| **Puzzle Panels** | R1-03: Phased Array Jamming Console (Station 1, left); R1-05: Tactical Radar Console (Station 2, right) |
| **Widgets — R1-03** | `PhaseInterferenceDisplay` (dual-source heatmap, draggable sources), `LinkedSliderInput` (x2 source frequencies), `ToggleSwitch` (symmetry lock) |
| **Widgets — R1-05** | `RadarSweepDisplay` (8 contacts), `TargetingReticleDisplay` (lock test inset), `LightedButton` (x8 classify) |
| **Entry Trigger** | COMMISSION phase (ROUND_1) |
| **Exit Options** | Communications (2-01-01), Nav Control (2-01-03), Transporter Room 2 (2-02-02 — corridor) |
| **Ambient Audio** | Radar sweep pings, interference hum. Tactical operations feel. |
| **Lighting** | Amber operations. Radar green glow from scope. |
| **Special Events** | `ACT1_R1_03` (null zone — clean signal tone emerges when null placed), `ACT1_R1_05` (contact lock — "FORMATION SIZE: 5. THIS WAS NOT A LONE VESSEL.") |

---

### Scene: `Deck2_NavControl`

| Field | Value |
|-------|-------|
| **Room ID** | 2-01-03 |
| **Room** | Nav Control |
| **Puzzle Panels** | R1-04: Trajectory Analysis Console |
| **Widgets** | `ConicSectionDisplay` (conic curve + 12 observed position dots), `NumericStepper` (eccentricity), `RotaryDial` (e step size), `NumericStepper` (semi-latus rectum) |
| **Entry Trigger** | COMMISSION phase (ROUND_1) |
| **Exit Options** | Tactical Control (2-01-02) |
| **Ambient Audio** | Comm static background. Soft plotting clicks. |
| **Lighting** | Amber operations. Star chart display backdrop. |
| **Special Events** | `ACT1_R1_04` (trajectory fit — "THE OBJECT DECELERATED AND HELD STATION.") |

---

### Scene: `Deck2_TransporterRoom1`

| Field | Value |
|-------|-------|
| **Room ID** | 2-02-01 |
| **Room** | Transporter Room 1 |
| **Puzzle Panels** | None (flavor only) |
| **Widgets** | None interactive |
| **Entry Trigger** | COMMISSION phase |
| **Exit Options** | Communications (2-01-01) |
| **Ambient Audio** | Transporter idle hum |
| **Lighting** | Amber operations |
| **Special Events** | None |

---

### Scene: `Deck2_TransporterRoom2`

| Field | Value |
|-------|-------|
| **Room ID** | 2-02-02 |
| **Room** | Transporter Room 2 |
| **Puzzle Panels** | None (flavor only) |
| **Widgets** | None interactive |
| **Entry Trigger** | COMMISSION phase |
| **Exit Options** | Tactical Control (2-01-02) |
| **Ambient Audio** | Transporter idle hum |
| **Lighting** | Amber operations |
| **Special Events** | None |

---

## DECK 3 — CREW (Division: GREEN `#2E8B57`)

### Scene: `Deck3_Sickbay`

| Field | Value |
|-------|-------|
| **Room ID** | 3-01-01 |
| **Room** | Sickbay — Main |
| **Puzzle Panels** | R3-01: LifesignsDisplay ECG Monitor |
| **Widgets** | `LifesignsDisplay` (3-channel ECG), `RotaryDial` (patient selector), `LinkedSliderInput` (heart rate), `IndicatorPanel` (3 BPM LEDs) |
| **Entry Trigger** | R2-META solved (ROUND_3 phase) |
| **Exit Options** | Crew Quarters (3-02-01), Senior Quarters (3-01-02) |
| **Ambient Audio** | ECG beeps (three distinct rhythms, Patient 3 irregular and accelerating). Clinical. No music. |
| **Lighting** | Clinical white. Three biobeds visible. Sterile, bright. |
| **Special Events** | `ACT3_R3_01` (V-tach identification — "THE DUTY LOG IS WRONG.") |

---

### Scene: `Deck3_SeniorQuarters`

| Field | Value |
|-------|-------|
| **Room ID** | 3-01-02 |
| **Room** | Senior Quarters |
| **Puzzle Panels** | None |
| **Widgets** | None interactive |
| **Entry Trigger** | ROUND_3 phase |
| **Exit Options** | Sickbay (3-01-01) |
| **Ambient Audio** | Soft hum, life support sounds. Residential, calm. |
| **Lighting** | Warm green residential |
| **Special Events** | None |

---

### Scene: `Deck3_CrewQuarters`

| Field | Value |
|-------|-------|
| **Room ID** | 3-02-01 |
| **Room** | Crew Quarters |
| **Puzzle Panels** | None |
| **Widgets** | None interactive |
| **Entry Trigger** | COMMISSION phase |
| **Exit Options** | Sickbay (3-01-01), Mess Hall (3-03-01) |
| **Ambient Audio** | Soft hum, occasional intercom. Residential. |
| **Lighting** | Warm green residential |
| **Special Events** | None |

---

### Scene: `Deck3_MessHall`

| Field | Value |
|-------|-------|
| **Room ID** | 3-03-01 |
| **Room** | Mess Hall |
| **Puzzle Panels** | None |
| **Widgets** | None interactive |
| **Entry Trigger** | COMMISSION phase |
| **Exit Options** | Crew Quarters (3-02-01), Recreation (3-03-02) |
| **Ambient Audio** | Soft hum, ambient galley sounds |
| **Lighting** | Warm green |
| **Special Events** | None |

---

### Scene: `Deck3_Recreation`

| Field | Value |
|-------|-------|
| **Room ID** | 3-03-02 |
| **Room** | Recreation |
| **Puzzle Panels** | None |
| **Widgets** | None interactive |
| **Entry Trigger** | COMMISSION phase |
| **Exit Options** | Mess Hall (3-03-01) |
| **Ambient Audio** | Quiet residential hum |
| **Lighting** | Warm green |
| **Special Events** | None |

---

## DECK 4 — SUPPORT (Division: BLUE `#1E3A5F`)

### Scene: `Deck4_ScienceLab`

| Field | Value |
|-------|-------|
| **Room ID** | 4-01-01 |
| **Room** | Science Lab |
| **Puzzle Panels** | R1-02: Lissajous Display Console (Station 2) |
| **Widgets** | `LissajousDisplay` (X-Y mode, drift/lock), `NumericStepper` (X freq — read-only, 291 MHz), `NumericStepper` (Y freq), `RotaryDial` (step size) |
| **Entry Trigger** | COMMISSION phase (ROUND_1) |
| **Exit Options** | Security Office (4-01-02), Cargo Bay (4-03-01 — corridor) |
| **Ambient Audio** | Lab equipment hum, oscilloscope sweep tone (chirp matching drift rate — pitch changes as player adjusts frequency) |
| **Lighting** | Cool science blue. Four workstations in semicircle. Station 2 illuminated. |
| **Special Events** | `ACT1_R1_02` (phase lock — "THE INCOMING SIGNAL IS TRACKING US. HARMONIC LOCK.") |

---

### Scene: `Deck4_SecurityOffice`

| Field | Value |
|-------|-------|
| **Room ID** | 4-01-02 |
| **Room** | Security Office |
| **Puzzle Panels** | R3-02: Modular Key Rotation Interface (Station 1); R3-04: Badge Swipe Log Display (Station 2); R3-META: Authorization Chain Analysis Console (Station 3) |
| **Widgets — R3-02** | `ModularClockDisplay` (8-position, roster labels), `NumericStepper` (rotation), `LCARSButton` (apply), `ScrollingTextDisplay` (sequence log) |
| **Widgets — R3-04** | `IndicatorPanel` (18 LEDs, G/R/F), `ScrollingTextDisplay` (swipe detail log) |
| **Widgets — R3-META** | Authorization chain display, `NumericInput` |
| **Entry Trigger** | R2-META solved (ROUND_3 phase); R3-META console locked until `round3Solved >= 3` |
| **Exit Options** | Science Lab (4-01-01), Armory (4-02-01), Brig (4-02-02 — SECURED until ROUND_3) |
| **Ambient Audio** | Security beeps. Badge swipe playback sounds when R3-04 active. |
| **Lighting** | Cool blue institutional. Three workstations. Security monitoring displays on walls. |
| **Special Events** | `ACT3_R3_02` (access code), `ACT3_THE_DROP` (badge sequence — THE DROP: sustained low tone, three FORCED clunks, silence), `ACT3_R3_META` (authorization chain — two metallic impacts), `ACT3_COMPLETE` (Round 3 transition, briefing room unlocks) |

---

### Scene: `Deck4_Armory`

| Field | Value |
|-------|-------|
| **Room ID** | 4-02-01 |
| **Room** | Armory |
| **Puzzle Panels** | None |
| **Widgets** | None interactive |
| **Entry Trigger** | ROUND_3 phase |
| **Exit Options** | Security Office (4-01-02) |
| **Ambient Audio** | Security ambient |
| **Lighting** | Cool blue |
| **Special Events** | None |

---

### Scene: `Deck4_Brig`

| Field | Value |
|-------|-------|
| **Room ID** | 4-02-02 |
| **Room** | Brig |
| **Puzzle Panels** | None (flavor only — empty cells, evidence of gap activity) |
| **Widgets** | None interactive |
| **Entry Trigger** | R2-META solved (ROUND_3 phase) — door was SECURED |
| **Exit Options** | Security Office (4-01-02) |
| **Ambient Audio** | Quiet, containment field hum |
| **Lighting** | Cool blue, dim. Cells dark. |
| **Special Events** | None |

---

### Scene: `Deck4_CargoBay`

| Field | Value |
|-------|-------|
| **Room ID** | 4-03-01 |
| **Room** | Cargo Bay |
| **Puzzle Panels** | None |
| **Widgets** | None interactive |
| **Entry Trigger** | COMMISSION phase |
| **Exit Options** | Science Lab (4-01-01) |
| **Ambient Audio** | Cargo bay echoes, industrial hum |
| **Lighting** | Cool blue, industrial overhead |
| **Special Events** | None |

---

## DECK 5 — ENGINEERING (Division: ORANGE `#CC5500`)

### Scene: `Deck5_MainEngPower`

| Field | Value |
|-------|-------|
| **Room ID** | 5-01-01 |
| **Room** | Main Engineering — Power |
| **Puzzle Panels** | R2-01: EPS Conduit Routing Console; R2-06: Reactor Operations Console |
| **Widgets — R2-01** | `ConduitFlowDisplay` (3 conduits, junction, destinations), `ToggleSwitch` (x3), `GaugeDisplay` (x3 destination) |
| **Widgets — R2-06** | `GaugeDisplay` (x4 reactor gauges with setpoints), `ThrottleLever` (fuel flow, 9 detents), `PowerSlider` (containment field) |
| **Entry Trigger** | R1-META solved (ROUND_2 phase) |
| **Exit Options** | Main Eng Propulsion (5-01-02), EPS Control (5-02-01 — corridor) |
| **Ambient Audio** | Loud reactor hum, plasma conduit flow sounds, periodic status chimes. Industrial, alive. Reactor hum deepens as throttle moves. |
| **Lighting** | Burnt orange industrial. Overhead power distribution diagram. Four console stations visible. Commission-marked consoles outlined amber. |
| **Special Events** | `ACT2_ENG_ARRIVAL` (engineering first visit), `ACT2_R2_01` (power path), `ACT2_THE_TURN` (R2-06 solve — reactor at station-keeping, echo motif returns, "WAITING.") |

---

### Scene: `Deck5_MainEngPropulsion`

| Field | Value |
|-------|-------|
| **Room ID** | 5-01-02 |
| **Room** | Main Engineering — Propulsion |
| **Puzzle Panels** | None (flavor — propulsion monitoring) |
| **Widgets** | None interactive |
| **Entry Trigger** | ROUND_2 phase |
| **Exit Options** | Main Eng Power (5-01-01), Main Eng Computer (5-01-03) |
| **Ambient Audio** | Engineering industrial |
| **Lighting** | Burnt orange |
| **Special Events** | None |

---

### Scene: `Deck5_MainEngComputer`

| Field | Value |
|-------|-------|
| **Room ID** | 5-01-03 |
| **Room** | Main Engineering — Computer |
| **Puzzle Panels** | R2-02: ODN Isolation Console |
| **Widgets** | `NetworkGridDisplay` (15-node ODN topology, rogue packet), `ToggleSwitch` (x15 node disable) |
| **Entry Trigger** | R1-META solved (ROUND_2 phase) |
| **Exit Options** | Main Eng Propulsion (5-01-02), Main Eng Damage Control (5-01-04), Computer Core (5-03-01 — SECURED until ROUND_3) |
| **Ambient Audio** | Network data flow (rapid clicking), red packet has distinct harsh tone |
| **Lighting** | Burnt orange, network display glow |
| **Special Events** | `ACT2_R2_02` (data breach — "THE FORGED SENSOR READINGS WERE PLANTED FROM INSIDE.") |

---

### Scene: `Deck5_MainEngDamageControl`

| Field | Value |
|-------|-------|
| **Room ID** | 5-01-04 |
| **Room** | Main Engineering — Damage Control |
| **Puzzle Panels** | None (flavor — damage control board) |
| **Widgets** | None interactive |
| **Entry Trigger** | ROUND_2 phase |
| **Exit Options** | Main Eng Computer (5-01-03) |
| **Ambient Audio** | Engineering industrial |
| **Lighting** | Burnt orange |
| **Special Events** | None |

---

### Scene: `Deck5_EPSControl`

| Field | Value |
|-------|-------|
| **Room ID** | 5-02-01 |
| **Room** | EPS Control |
| **Puzzle Panels** | R2-05: EPS Circuit Diagnostic Console |
| **Widgets** | `CircuitTopologyDisplay` (20-node branching tree), `LCARSButton` (x20 probe buttons) |
| **Entry Trigger** | R1-META solved (ROUND_2 phase) |
| **Exit Options** | Main Eng Power (5-01-01), Environmental Control (5-02-02 — corridor) |
| **Ambient Audio** | EPS conduit hum, probe electrical test tones |
| **Lighting** | Burnt orange, circuit diagram backlit |
| **Special Events** | `ACT2_R2_05` (fault trace — "SOMEONE BURNED THE RECORDING SYSTEM ON PURPOSE.") |

---

### Scene: `Deck5_EnvironmentalControl`

| Field | Value |
|-------|-------|
| **Room ID** | 5-02-02 |
| **Room** | Environmental Control |
| **Puzzle Panels** | R2-03: Deck Thermal Monitoring Console |
| **Widgets** | `HeatMapDisplay` (5x5 grid, diffusion simulation), `LCARSButton` (place/remove coolant), `GaugeDisplay` (peak temperature) |
| **Entry Trigger** | R1-META solved (ROUND_2 phase) |
| **Exit Options** | EPS Control (5-02-01), Deflector Control (5-02-03 — corridor) |
| **Ambient Audio** | Environmental systems hum, temperature click sounds |
| **Lighting** | Burnt orange, thermal display color wash |
| **Special Events** | `ACT2_R2_03` (heat source — "EQUIPMENT THAT WAS NOT AUTHORIZED TO BE POWERED.") |

---

### Scene: `Deck5_DeflectorControl`

| Field | Value |
|-------|-------|
| **Room ID** | 5-02-03 |
| **Room** | Deflector Control |
| **Puzzle Panels** | R3-05: Guarded Emergency Procedure Console |
| **Widgets** | `BatSwitch` (x4, guarded: A=THERMAL, B=MAGNETIC, C=COMM, D=SENSOR), `IndicatorPanel` (4 switch state LEDs + procedure register), `ScrollingTextDisplay` (procedure log + classified addendum excerpt), `MasterAlarm`, `AutoProcedureButton` |
| **Entry Trigger** | R2-META solved (ROUND_3 phase) |
| **Exit Options** | Environmental Control (5-02-02) |
| **Ambient Audio** | Guard lift clicks, switch flips, counter ticks. MasterAlarm if wrong sequence. |
| **Lighting** | Burnt orange. Red safety guard covers visible on the four switches. |
| **Special Events** | `ACT3_R3_05` (bypass sequence — "ONLY THREE PEOPLE HAVE READ IT: THE CO, THE XO, AND THE CHIEF ENGINEER. THE XO WAS OFF-SHIP. THAT LEAVES TWO.") |

---

### Scene: `Deck5_ComputerCore`

| Field | Value |
|-------|-------|
| **Room ID** | 5-03-01 |
| **Room** | Computer Core |
| **Puzzle Panels** | R3-03: Group Composition Interface |
| **Widgets** | `CayleyTableDisplay` (8x8 Z_8 multiplication table), `RotaryDial` (element selector, 8 positions with roster names), `LCARSButton` (compose), `IndicatorPanel` (base permission + target) |
| **Entry Trigger** | R2-META solved (ROUND_3 phase) — door was SECURED |
| **Exit Options** | Main Eng Computer (5-01-03) |
| **Ambient Audio** | Computing sounds, data processing hum, composition clicks |
| **Lighting** | Blue isolinear glow from floor-to-ceiling racks. Single workstation illuminated. Vertical room. |
| **Special Events** | `ACT3_R3_03` (permission chain — "REEVES IS THE CHIEF ENGINEER. ONE OF THREE PEOPLE WHO KNEW THE CLASSIFIED ADDENDUM.") |

---

## NARRATIVE SCENES (Non-Room)

### Scene: `Narrative_Turbolift`

| Field | Value |
|-------|-------|
| **Location** | Turbolift interior (transit between decks) |
| **Puzzle Panels** | None |
| **Widgets** | Deck selection button panel, small status display for narrative text |
| **Trigger** | Player clicks turbolift stop; narrative beats trigger on first ride after round-meta solves |
| **Duration** | ~5 seconds transit + 3 seconds text hold |
| **Ambient Audio** | Mechanical movement hum, deceleration |
| **Lighting** | Functional, neutral. Simple interior. |
| **Special Events** | `ACT1_NARRATIVE` (first ride after R1-META — "did the ship know it was coming?"), `ACT2_NARRATIVE` (first ride after R2-META — "Someone on this ship knew the encounter was coming.") |

---

### Scene: `Narrative_HUD`

| Field | Value |
|-------|-------|
| **Location** | Overlay on any room scene |
| **Puzzle Panels** | None |
| **Widgets** | Progress tracker panel (top-right corner), minimap (corner, 5 deck strips) |
| **Trigger** | Each feeder solve; threshold events (4/6 R1, 4/6 R2, 3/5 R3) |
| **Duration** | 3-second overlay fade |
| **Ambient Audio** | Chime on each feeder solve. Deeper tone on threshold. |
| **Lighting** | N/A (overlay) |
| **Special Events** | `ACT1_PROGRESS`, `ACT2_PROGRESS`, `ACT3_PROGRESS` |

---

## SCENE COUNT SUMMARY

| Category | Scenes | Details |
|----------|--------|---------|
| Deck 1 — Command | 7 | BridgeCenter, BridgeHelm, BridgeTactical, BridgeOPS, BridgeScience, ReadyRoom, BriefingRoom, ObservationLounge |
| Deck 2 — Operations | 5 | Communications, TacticalControl, NavControl, TransporterRoom1, TransporterRoom2 |
| Deck 3 — Crew | 5 | Sickbay, SeniorQuarters, CrewQuarters, MessHall, Recreation |
| Deck 4 — Support | 5 | ScienceLab, SecurityOffice, Armory, Brig, CargoBay |
| Deck 5 — Engineering | 8 | MainEngPower, MainEngPropulsion, MainEngComputer, MainEngDamageControl, EPSControl, EnvironmentalControl, DeflectorControl, ComputerCore |
| Narrative | 2 | Turbolift, HUD overlay |
| **Total** | **32** | 28 navigable rooms + 2 transporter flavor + 2 narrative |

### Puzzle Distribution by Deck

| Deck | Puzzles | IDs |
|------|---------|-----|
| 1 | 3 | R1-META, R2-META, R2-04, FINAL META |
| 2 | 4 | R1-01, R1-03, R1-04, R1-05, R1-06 |
| 3 | 1 | R3-01 |
| 4 | 4 | R1-02, R3-02, R3-03 (Deck 5 room), R3-04, R3-META |
| 5 | 8 | R2-01, R2-02, R2-03, R2-05, R2-06, R3-03, R3-05 |
