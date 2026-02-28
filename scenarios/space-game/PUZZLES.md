# DEAD RECKONING — Puzzle Registry

**Answer encoding:** ROT13. Plaintext answers stored in `.claude/answers.md` only.
**World reference:** All puzzle facts verified against `world/WORLD.md`, `world/systems/departments.md`, `world/systems/data-tables.md`.

---

## Meta Architecture

| Meta | Answer | Mechanism |
|------|--------|-----------|
| R1-META | PHASE-LOCKED PAIR | 6 interference types exist in the world. 5 feeders each name one. The missing sixth is the answer. |
| R2-META | LOCKOUT | 6 system states exist in the world. 5 feeders each name one. The missing sixth (LOCKED) names the mechanism. |
| R3-META | SENIOR OFFICER | 5 feeders produce crew/system vocabulary; mechanism designed in Stage 5. |
| FINAL | TBD — Stage 5 | Combines PHASE-LOCKED PAIR + LOCKOUT + SENIOR OFFICER to reveal the incident. |

---

## ROUND 1 — TELEMETRY

*What was out there during the missing six hours.*
*Five feeders name five of the six interference types. The missing sixth IS the contact.*

---

### R1-01 — CARRIER WAVE

| Field | Value |
|-------|-------|
| **Department** | COMMS |
| **Instrument** | Frequency spectrum display with band selector dial |
| **Status** | -- |
| **Answer** | (encoded) |
| **Answer note** | see ANSWERS.md |
| **Difficulty** | Medium |

**Brief:**
The solver is at the COMMS console looking at a frequency spectrum display showing six signal traces across the deleted window's comm log. Each trace has a distinct visual signature corresponding to one of the six interference types from the world: Decoys, Shattered Carrier, Hieratic Triplet, Harmonic Echo, Phase-Locked Pair, Absorption Shadow.

The solver uses the band selector dial to tune each trace into focus and classifies it against the interference taxonomy posted on the panel bezel. Five of the six types appear on the display. The sixth — Phase-Locked Pair — is conspicuously absent from the comm log. This is the puzzle's tell: everything is interference except the one thing the comm log doesn't show.

The extraction mechanism yields the answer DECOYS: the solver identifies that four of the five visible traces are Decoys (the world's first interference type) — the unknown contact used a cloud of decoy signals to mask its actual presence. The single non-Decoy trace identifies the Decoy-masking technique, and the pattern of which type appears confirms the answer word.

**Author notes:**
- The display must clearly show 6 traces with 5 distinct interference type signatures
- Phase-Locked Pair must be visibly absent — its "slot" is empty or static
- The Decoy signatures must be identifiable from their world description (waveform shape follows from what Decoys physically do)
- Extraction: design the display so the solver's classification work yields DECOYS cleanly — e.g., four traces are Decoy variants and identifying them produces the answer
- The absence of Phase-Locked Pair is narrative foreshadowing, not the puzzle answer — that's the round-meta

**Meta role:** Names one of the six interference types (DECOYS). The round-meta uses all five named types to identify the missing sixth.

---

### R1-02 — DEAD RECKONING

| Field | Value |
|-------|-------|
| **Department** | NAV |
| **Instrument** | Four-window multi-modal sensor comparison panel |
| **Status** | -- |
| **Answer** | (encoded) |
| **Answer note** | see ANSWERS.md |
| **Difficulty** | Medium |

**Brief:**
The solver is at the NAV console facing a four-window sensor comparison panel showing the same region of space simultaneously in four modalities: gravitational, electromagnetic, thermal, particle. Seven contacts appear across the windows, each visible in some modalities but not others.

Real objects have consistent multi-modal signatures defined by physics (a vessel appears in EM, thermal, and particle; a nav deflector wake appears in gravitational only; etc.). The solver classifies each contact as real or artifact using a sensor-physics reference on the panel.

The extraction yields SHATTERED CARRIER: the solver discovers that the EM readings for several contacts show a fragmented, broken-carrier waveform pattern — the specific signature of the Shattered Carrier interference type. The contacts the ship was tracking were being obscured by deliberate Shattered Carrier interference injected into the sensor feed.

**Author notes:**
- The four-window layout is the core physical experience — the solver is literally comparing four displays simultaneously
- Sensor physics must be internally consistent and complete (no ambiguous contacts)
- The Shattered Carrier signature should be identifiable in the EM window specifically — broken carrier pattern visible on that modality
- Extraction: the solver identifies the interference type masking the sensor readings as SHATTERED CARRIER
- Reference card on panel must define what each modality detects

**Meta role:** Names one of the six interference types (SHATTERED CARRIER).

---

### R1-03 — CONTACT REPORT

| Field | Value |
|-------|-------|
| **Department** | TAC |
| **Instrument** | Polar targeting display with contact classification sidebar |
| **Status** | -- |
| **Answer** | (encoded) |
| **Answer note** | see ANSWERS.md |
| **Difficulty** | Medium-Hard |

**Brief:**
The solver is at the tactical targeting display — a polar grid with range rings and bearing markers. Eight contacts are plotted as blips, each with four readings in a sidebar: radar cross-section (m²), thermal output (kW), EM emission (dBm), maneuver trace. The solver classifies each as REAL VESSEL or DECOY using the classification standard on the panel bezel.

Six of the eight contacts are decoys; two are real vessels. The real vessels' designations, read by increasing bearing, begin to tell the story of what the ship encountered.

The extraction yields HIERATIC TRIPLET: among the decoy contacts, a specific cluster of three exhibits the exact signature pattern of the Hieratic Triplet interference type — three phantom contacts arranged in a ritualistic spacing pattern that, once classified, identifies the interference type being used to mask the real contacts.

**Author notes:**
- Eight contacts with four readings each — the author should design this so six are clearly Decoys and two are clearly real vessels once the classification standard is applied
- The three Hieratic Triplet contacts should be identifiable as a cluster; their spacing on the polar grid should visually suggest the triplet pattern
- Classification standard on bezel must be readable and complete (no ambiguous classifications)
- The Hieratic Triplet identification is the extraction — the solver names the type, yielding the answer
- Real vessels' details can foreshadow Round 3 narrative elements

**Meta role:** Names one of the six interference types (HIERATIC TRIPLET).

---

### R1-04 — GATE LOG

| Field | Value |
|-------|-------|
| **Department** | SEC |
| **Instrument** | Security access control panel — three indicator columns |
| **Status** | -- |
| **Answer** | (encoded) |
| **Answer note** | see ANSWERS.md |
| **Difficulty** | Medium |

**Brief:**
The solver is at the security access control panel — three vertical columns of indicator lights, one per gate (forward sensor bay, comm array access junction, computer core anteroom). Each light shows a crew access event: timestamp, division color, result (GRANTED / DENIED / FORCED). The deleted six hours appear as REDACTED.

The solver reconstructs who had active access cards and who could have passed all three gates during the redacted period. The reconstruction produces a short list of candidates.

The extraction yields HARMONIC ECHO: buried in the access log, the COMMS console was accessed repeatedly from the same credentials in a pattern that exactly mirrors the comm array's own transmission timestamps — the access events are echoing the outbound signals. This is the Harmonic Echo interference pattern applied to the access system: someone's access credentials were being echoed/replicated by the unknown contact to monitor what was being transmitted.

**Author notes:**
- The three-column layout with GRANTED / DENIED / FORCED is the core instrument
- The REDACTED section must be clearly marked and span all three columns
- The Harmonic Echo pattern emerges from comparing access timestamps against comm transmission timestamps — this cross-system detail is the aha
- Extraction: the solver names the interference type being used against the access system as HARMONIC ECHO
- The card-duration rule (how long access cards remain active) must be posted on the panel

**Meta role:** Names one of the six interference types (HARMONIC ECHO).

---

### R1-05 — ANOMALY STATION

| Field | Value |
|-------|-------|
| **Department** | SCIENCE |
| **Instrument** | Six-band false-color spectral analyzer |
| **Status** | -- |
| **Answer** | (encoded) |
| **Answer note** | see ANSWERS.md |
| **Difficulty** | Hard |

**Brief:**
The solver is at the science console with a six-band spectral analyzer showing the deleted window's anomalous region in radio, microwave, infrared, visible, ultraviolet, and X-ray simultaneously, each as a false-color intensity map. Eight signatures appear; each is visible in some bands but not others.

The solver classifies each signature by its cross-band profile using a spectral reference on the panel. Seven are identifiable astronomical phenomena. The eighth is different: it shows an Absorption Shadow pattern — a specific band combination where signals that should appear are conspicuously absent, as if something is absorbing them at that point in space.

The extraction yields ABSORPTION SHADOW: the solver classifies the eighth anomalous signature and identifies it as an Absorption Shadow — a region where signal absorption exceeds what any natural phenomenon explains. The unknown contact was using an Absorption Shadow technique to make its presence invisible to science sensors.

**Author notes:**
- The six-band false-color display is visually the most striking instrument in Round 1 — give it proper design attention
- Seven of eight signatures should be unambiguously classifiable as astronomical phenomena
- The eighth should show the Absorption Shadow pattern: specific bands show no signal where signal should appear
- The Absorption Shadow classification requires the solver to understand what that type physically is (signal absorption, not interference generation) — this is the knowledge gate
- Extraction: solver names the interference type as ABSORPTION SHADOW

**Meta role:** Names one of the six interference types (ABSORPTION SHADOW).

---

### R1-META — TELEMETRY RECONSTRUCTION

| Field | Value |
|-------|-------|
| **Type** | Round meta |
| **Unlocks** | After 4 of 5 R1 feeders solved |
| **Answer** | (encoded) |
| **Answer note** | see ANSWERS.md |
| **Mechanism** | The world has exactly six interference types. Five feeders each name one: DECOYS, SHATTERED CARRIER, HIERATIC TRIPLET, HARMONIC ECHO, ABSORPTION SHADOW. The sixth — PHASE-LOCKED PAIR — is missing from the telemetry record. It was not interference. It was the signal itself. |

**Brief:**
The solver is presented with a compiled telemetry reconstruction panel showing the five interference types identified across the deleted window. A reference card shows all six types from the world. One type is not represented anywhere in the telemetry data.

The question the panel poses: "The following interference types were identified during the deleted window. One type was not observed as interference. Name it."

The answer is PHASE-LOCKED PAIR — the interference type defined as two signals locked together in perfect synchronization. This was not background noise. This was contact: the unknown entity was using Phase-Locked Pair technology, mirroring the ship's own transmissions so precisely that it looked like a reflection of the ship itself.

**Author notes:**
- The round-meta panel should display the five feeder answers (interference type names) and the complete six-type reference
- The question should be unambiguous: identify the type not present in the record
- The narrative payoff: the solver now knows WHAT was encountered — a phase-locked contact, something that mirrored the ship. This is the "what" of the incident.
- PHASE-LOCKED PAIR unlocks Round 2

---

## ROUND 2 — SYSTEMS LOG

*What the ship's internal systems recorded that wasn't scrubbed.*
*Five feeders name five of the six system states. The missing sixth names the mechanism of the cover-up.*

---

### R2-01 — LOAD SHEDDING

| Field | Value |
|-------|-------|
| **Department** | POWER |
| **Instrument** | EPS distribution panel with load gauges and status indicators |
| **Status** | -- |
| **Answer** | (encoded) |
| **Answer note** | see ANSWERS.md |
| **Difficulty** | Medium |

**Brief:**
The solver is at the EPS distribution panel. Eight systems show current power draw on MW gauges. Three systems show load while their status indicators say OFFLINE. The solver cross-references the phantom draw values against known EPS consumption figures (impulse = 180 MW, shields = 95 MW, transporter = 40 MW, etc.) to identify what was secretly running.

The extraction yields FAULT: the solver discovers that the phantom loads match a specific EPS configuration that can only occur when the PCU is operating in fault-recovery mode — a mode the ship's log doesn't show. The EPS distribution pattern is the signature of a FAULT state being masked by a status override. The answer is the system state the EPS board was actually in: FAULT.

**Author notes:**
- The EPS load panel should show 8 systems with MW gauges — clean, readable
- Three phantom loads must be clearly wrong (MW draw inconsistent with OFFLINE status)
- The consumption figures must be on the panel (the solver shouldn't need to memorize them)
- Extraction: the pattern of phantom loads matches a FAULT-mode EPS configuration — the solver identifies the system state as FAULT
- The FAULT identification is the answer, not the identification of specific systems (though that should also be inferrable as narrative)

**Meta role:** Names one of the six system states (FAULT).

---

### R2-02 — PATHWAY TRACE

| Field | Value |
|-------|-------|
| **Department** | COMPUTER |
| **Instrument** | ODN junction board — 15-node grid with toggle switches |
| **Status** | -- |
| **Answer** | (encoded) |
| **Answer note** | see ANSWERS.md |
| **Difficulty** | Medium |

**Brief:**
The solver is at the ODN junction board — 15 nodes in a grid, each with a toggle switch showing current routing state. The maintenance log shows junction toggles during the deleted window (timestamped, unlabeled). The solver traces data pathways using routing rules to determine which nodes lost service in sequence.

The extraction yields OFFLINE: the solver traces the ODN state changes and determines that several critical nodes were systematically taken OFFLINE during the deleted window — not due to damage, but via deliberate toggle. The sequence of toggled nodes and their final state identifies the system state imposed on those nodes: OFFLINE.

**Author notes:**
- The 15-node grid layout (from `world/systems/data-tables.md` Table 1) should match the canonical ODN topology
- The toggle sequence should be uniquely determined — the solver must reach the same conclusion regardless of order of attack
- Routing rules (active junctions pass data, passive junctions block) must be on the panel or as a reference card
- Extraction: the solver identifies that nodes were put into OFFLINE state — the answer is OFFLINE
- The nodes that went OFFLINE should correspond to specific functions (narrative: communication logs, sensor recordings)

**Meta role:** Names one of the six system states (OFFLINE).

---

### R2-03 — ATMOSPHERIC INCIDENT

| Field | Value |
|-------|-------|
| **Department** | ENVIRO |
| **Instrument** | Environmental chemistry panel — six dial gauges per zone |
| **Status** | -- |
| **Answer** | (encoded) |
| **Answer note** | see ANSWERS.md |
| **Difficulty** | Medium |

**Brief:**
The solver is at the ENVIRO chemistry panel. Six dial gauges per zone: O2%, CO2%, N2%, trace elements, temperature, humidity. Most zones are nominal. Three zones show readings subtly off in specific ways — the multi-gauge signature of something deliberately released into those zones' atmosphere.

The solver identifies the compound from the signature pattern and cross-references it against the replicator's hazardous material table. The cross-reference reveals which department replicated the compound and when.

The extraction yields STANDBY: the compound identified was a sedative/suppressive agent used to induce a STANDBY-like state in biological systems — the same metaphor the ship uses for systems that are sleeping but not off. The ENVIRO system itself was set to STANDBY in those three zones to prevent normal atmospheric alarms from triggering. The solver identifies the system state STANDBY as the mechanism.

**Author notes:**
- Six gauges per zone — the multi-gauge combination is the puzzle's core physical experience
- Three zones should show a distinctive but subtle chemical signature
- The compound cross-reference with the replicator table should be unambiguous (one compound fits the full six-gauge signature)
- The ENVIRO system state in those zones was STANDBY — this is what the solver identifies as the answer
- The compound's nature (biological suppressant) connects narratively to Round 3's medical evidence

**Meta role:** Names one of the six system states (STANDBY).

---

### R2-04 — BURN PATTERN

| Field | Value |
|-------|-------|
| **Department** | PROPULSION |
| **Instrument** | Impulse console strip chart recorder |
| **Status** | -- |
| **Answer** | (encoded) |
| **Answer note** | see ANSWERS.md |
| **Difficulty** | Medium-Hard |

**Brief:**
The solver is at the impulse console looking at a strip chart recorder — a scrolling waveform showing EPS power draw for the impulse drive over the deleted six hours. The curve has eight distinctive events: power-up spikes, cruise plateaus, deceleration dips.

The solver matches each event against the drive's known operating profile (acceleration = 180 MW peak, cruise = 120 MW steady, decel = 60 MW) to identify the maneuver sequence. The official flight log says the ship was stationary during this window. The waveform says otherwise.

The extraction yields ONLINE: the impulse drive was ONLINE and executing maneuvers during the deleted window — a fact the official log set to OFFLINE. The solver identifies the system's true state as ONLINE by reading the waveform against the operating profile.

**Author notes:**
- The strip chart recorder is a horizontally scrolling waveform — the solver reads left to right
- Eight events of three types (power-up, cruise, decel) — the shape of each should be visually distinct
- Operating profile reference (MW values for each maneuver type) must be on or adjacent to the panel
- Extraction: the waveform proves the impulse drive was ONLINE — the answer is the system state ONLINE
- The maneuver sequence revealed by the waveform should encode narrative information (the ship was actively maneuvering to remain in contact with the phase-locked entity)

**Meta role:** Names one of the six system states (ONLINE).

---

### R2-05 — PARTIAL RECOVERY

| Field | Value |
|-------|-------|
| **Department** | AUXILIARY |
| **Instrument** | Auxiliary log terminal — two merged streams |
| **Status** | -- |
| **Answer** | (encoded) |
| **Answer note** | see ANSWERS.md |
| **Difficulty** | Medium |

**Brief:**
The solver is at the auxiliary terminal looking at a log feed that was tampered with — two separate data streams (Electrical grid monitor and EPS grid monitor) were merged into one timestamp-ordered feed with source labels stripped. Each event shows a value (kW or MW), a system name, and a status change.

The solver separates the two streams by knowing which systems belong to which grid (Electrical: computer, comms, sensors, lighting, weapons fire control; EPS: impulse, shields, environmental fields, transporter, replicator). Once separated, each stream tells part of what was happening.

The extraction yields READY: with the streams separated, the EPS stream reveals that several critical combat systems were placed in READY state during the deleted window — weapons to READY, shields to READY — a configuration that only makes sense if the ship was preparing to engage or defend against something. The answer is the system state READY: the ship was standing ready for contact.

**Author notes:**
- The merged feed should have clearly different value scales (kW for Electrical, MW for EPS) to give the solver the first separation clue
- Every system name in the feed should be unambiguously assignable to one grid — no systems that span both
- The Electrical grid reference and EPS grid reference must be available to the solver
- Extraction: the EPS stream shows critical systems were in READY state — the solver identifies the state as READY
- The READY configuration tells the story: someone prepared the ship for contact and then hid it

**Meta role:** Names one of the six system states (READY).

---

### R2-META — SYSTEMS RECONSTRUCTION

| Field | Value |
|-------|-------|
| **Type** | Round meta |
| **Unlocks** | After 4 of 5 R2 feeders solved |
| **Answer** | (encoded) |
| **Answer note** | see ANSWERS.md |
| **Mechanism** | The world has exactly six system states. Five feeders each name one: FAULT, OFFLINE, STANDBY, ONLINE, READY. The sixth — LOCKED — is missing from the systems log. The question the round-meta poses: "What operational procedure does the missing state describe?" The answer: LOCKOUT. |

**Brief:**
The solver is presented with a compiled systems reconstruction panel showing the five system states identified across the internal logs. A reference card shows all six states from the world. One state does not appear in any log entry.

The question: "Five system states appear in the ship's internal records. The sixth was imposed on critical systems without being logged. Name the operational procedure."

The answer is LOCKOUT — the procedure of placing systems in LOCKED state to restrict access while maintaining power availability. The deleted logs removed every record of LOCKED state from the ship's systems. Something was locked away. The lockout was the cover-up's mechanism.

**Author notes:**
- Display the five feeder answers (system state names) and the complete six-state reference
- The panel should make clear that LOCKED state has been scrubbed from every log entry
- The question asks for the OPERATIONAL PROCEDURE (LOCKOUT), not the state name (LOCKED) — this distinction should be on the panel
- Narrative payoff: the solver now knows HOW the ship responded — systems were locked out, access was denied, records were purged. This is the "how" of the cover-up.
- LOCKOUT unlocks Round 3

---

## ROUND 3 — CREW RECORD

*What happened to the people — the traces the cover-up didn't catch.*

---

### R3-01 — VITAL SIGNS

| Field | Value |
|-------|-------|
| **Department** | MEDICAL |
| **Instrument** | Medical scanner display — 20-reading gauge grid |
| **Status** | -- |
| **Answer** | (encoded) |
| **Answer note** | see ANSWERS.md |
| **Difficulty** | Medium |

**Brief:**
The solver is at the medical scanner display — a grid of 20 body-system readings from a scan taken on an unknown crew member during the deleted window. Each reading is a bar gauge against a reference range. Eight gauges show values that cannot coexist in a single healthy individual under normal conditions — but they DO coexist in someone who has been placed in, or recently removed from, a stasis field.

The solver identifies the physiological signature from the inconsistent combination of readings. The medical terminology for this presentation is the answer.

The extraction yields STASIS: the bio-signature is consistent with someone who was placed in emergency stasis during the deleted window. The crew member's readings show the characteristic post-stasis biochemical profile. Someone was stasised — and their medical record was only partially scrubbed.

**Author notes:**
- 20 bar gauges — the visual density is the experience. The solver reads across all 20 and looks for the inconsistency pattern
- Eight gauges should be subtly wrong in a way that a healthy person wouldn't show, but that someone in/from stasis would
- The stasis bio-signature should be internally consistent and derivable from first principles (if the solver understands what stasis does to a biological system, they can identify it)
- The medical reference card on the panel should give enough information to make the identification possible without giving it away
- Extraction: the solver names the condition as STASIS

**Meta role:** One of five Round 3 feeder answers feeding into SENIOR OFFICER.

---

### R3-02 — PATTERN INTEGRITY

| Field | Value |
|-------|-------|
| **Department** | TRANSIT |
| **Instrument** | Transporter targeting screen — spatial grid with reticles |
| **Status** | -- |
| **Answer** | (encoded) |
| **Answer note** | see ANSWERS.md |
| **Difficulty** | Medium |

**Brief:**
The solver is at the transporter targeting screen — a spatial grid with eight targeting reticles, each showing coordinates, a signal-lock quality gauge, and a personnel ID. Seven transports occurred during the deleted window; five completed with normal lock quality, two show degraded status.

The solver determines from the lock quality readings whether any pattern was in the buffer simultaneously with another — which is only possible if the buffer size was artificially expanded. The crew IDs of simultaneous-buffer occupants index into the answer.

The extraction yields BUFFER: the solver confirms that two patterns were held simultaneously in the pattern BUFFER — a deliberate expansion of buffer capacity to allow dual-occupancy transport. The word that describes what was artificially manipulated to make this possible is BUFFER. The simultaneous transport implies two people — or a person and something else — were moved together without a standard transport log entry.

**Author notes:**
- The targeting grid should have a spatial, physical quality — coordinates on both axes, reticles at specific positions
- The signal-lock quality gauge is a circular signal-strength meter — the "degraded" reticles show a partial lock
- The simultaneous-buffer constraint is the key rule: standard buffer holds one pattern; this buffer was expanded
- The solver should be able to determine from the lock quality readings that two patterns overlapped in time
- Extraction: the mechanism that was manipulated is the BUFFER — the answer is the component name

**Meta role:** One of five Round 3 feeder answers feeding into SENIOR OFFICER.

---

### R3-03 — ACCESS RECORD

| Field | Value |
|-------|-------|
| **Department** | SEC (personnel) |
| **Instrument** | Security terminal — badge swipe LED readout |
| **Status** | -- |
| **Answer** | (encoded) |
| **Answer note** | see ANSWERS.md |
| **Difficulty** | Medium |

**Brief:**
The solver is at the security terminal looking at an 18-entry badge swipe log on a scrolling LED readout — door ID, timestamp, personnel ID, result (GRANTED / DENIED / FORCED). Four personnel IDs appear in combinations of rooms that are physically impossible: the same person cannot swipe into two locations three decks apart within 30 seconds.

The solver uses the room ID format ({deck}-{section}-{room}) and the ship's layout to map the impossible swipes and identify which badges were cloned. The cloned badge holders' IDs index into the answer.

The extraction yields FORCED: reviewing the impossible swipes, the solver identifies that every single physically-impossible access event has a result of FORCED — meaning the security system's resistance was overridden by command-level authority. The badge clones aren't the anomaly. The FORCED overrides are. Someone with the highest clearance was systematically forcing access during the deleted window.

**Author notes:**
- 18 entries on a scrolling LED readout — the physical format gives it a real security terminal feel
- The room ID format is key: the solver needs the ship layout (from `world/systems/data-tables.md` Table 2) to determine which rooms are three decks apart
- Four impossible badge sequences — enough to establish the pattern, not so many it becomes exhausting
- The FORCED result appearing consistently on all impossible events is the tell — not the badges themselves
- Extraction: the answer is FORCED (the access result), not a crew member's ID

**Meta role:** One of five Round 3 feeder answers feeding into SENIOR OFFICER.

---

### R3-04 — REPAIR QUEUE

| Field | Value |
|-------|-------|
| **Department** | DAMAGE CONTROL |
| **Instrument** | Damage control board — 5×5 status tile grid |
| **Status** | -- |
| **Answer** | (encoded) |
| **Answer note** | see ANSWERS.md |
| **Difficulty** | Medium-Hard |

**Brief:**
The solver is at the damage control board — a 5×5 grid of system status indicator tiles, one per system, each showing one of the six system states. The board log shows six snapshots across the deleted window.

The standard recovery sequence for a damaged system is: ● FAULT → ◌ STANDBY → ◇ READY → ◉ ONLINE. Any system that instead went ● FAULT → ⊘ LOCKED was deliberately isolated — bypassing standard repair, restricting access. Those systems' designations encode the answer.

The extraction yields LOCKED: the solver identifies several systems that took the anomalous FAULT → LOCKED transition. The state imposed on those systems — the state that represents "power available but deliberately restricted" — is LOCKED.

**Author notes:**
- The 5×5 tile grid is the core visual — each tile uses the world's system state symbols (◉ ◇ ◌ ⊘ ○ ●)
- Six snapshots should show a temporal story — some systems recovering normally, some taking the anomalous LOCKED path
- The standard recovery sequence must be posted on the panel as reference
- The anomalous FAULT → LOCKED transition should appear for 4-6 systems — enough to establish the pattern
- Extraction: the solver identifies the state these systems were put into as LOCKED
- This is the Round 3 parallel of R2-META's LOCKOUT — the damage control board shows WHAT was locked, not WHY

**Meta role:** One of five Round 3 feeder answers feeding into SENIOR OFFICER. Also echoes R2-META thematically.

---

### R3-05 — UNAUTHORIZED REQUEST

| Field | Value |
|-------|-------|
| **Department** | REPLICATOR |
| **Instrument** | Replicator terminal — pattern database display + authorization log |
| **Status** | -- |
| **Answer** | (encoded) |
| **Answer note** | see ANSWERS.md |
| **Difficulty** | Hard |

**Brief:**
The solver is at the replicator terminal looking at 20 replication requests from the deleted window, each showing pattern name, quantity, timestamp, and security flag (OPEN, BIO, HAZ, POISON, SEC, UNK). A second panel shows the authorization log — who approved each request.

The solver checks each request against its required authorization level. Several requests were processed with insufficient authorization. The pattern names of the unauthorized requests provide the extraction.

The extraction yields RESTRICTED: the unauthorized requests are all flagged SEC (Security) — items in the RESTRICTED authorization tier that require Security Chief + CO dual authorization. The requests were approved with only one authorization (not both). The category name for these items — the tier they belong to — is RESTRICTED. The solver identifies the authorization category that was bypassed.

**Author notes:**
- 20 requests with flags and authorization records — the authorization table (from `world/systems/data-tables.md` Table 3) must be on the panel or adjacent
- Several violations should be present — the solver identifies ALL of them, not just one
- The violations should all be the same flag category (SEC / Tier 3 Restricted) to make the pattern clear
- Extraction: the solver names the authorization tier that was bypassed as RESTRICTED
- The specific items replicated (SEC-flagged items requiring dual auth) should be narratively meaningful — weapons? surveillance gear? — pointing toward who did it

**Meta role:** One of five Round 3 feeder answers feeding into SENIOR OFFICER.

---

### R3-META — CREW RECONSTRUCTION

| Field | Value |
|-------|-------|
| **Type** | Round meta |
| **Unlocks** | After 4 of 5 R3 feeders solved |
| **Answer** | (encoded) |
| **Answer note** | see ANSWERS.md |
| **Mechanism** | TBD — Stage 5. Inputs: STASIS, BUFFER, FORCED, LOCKED, RESTRICTED. These five words describe the cover-up's signature: someone with command access (FORCED overrides), highest clearance (RESTRICTED auth bypassed), who locked critical systems (LOCKED), put someone into stasis (STASIS), and manipulated the transporter buffer (BUFFER). The meta puzzle will combine these to identify the rank. |

**Brief (design intent):**
The five feeder answers collectively describe a profile that can only belong to one career tier: SENIOR OFFICER. The meta puzzle will present the five words and ask the solver to identify which rank has access to all five types of actions described. Only a SENIOR OFFICER can: force access gates, bypass RESTRICTED authorization, issue LOCKED state commands without logging, authorize emergency stasis, and expand the transporter buffer.

**Author notes:**
- The meta mechanism will be designed in Stage 5 to use STASIS + BUFFER + FORCED + LOCKED + RESTRICTED → SENIOR OFFICER
- A likely mechanism: each word appears in the career tier definitions in the world — but only the SENIOR OFFICER tier description uses all five
- Or: the five words are matched against the CALIBRATE console command's authorized operations (available only to Senior Officers) — all five are CALIBRATE-level actions
- The career tier reference card (from `world/WORLD.md`) must be available to the solver
- Narrative payoff: the solver now knows WHO gave the order — a Senior Officer. Not named yet. That's the final meta.

---

## FINAL META — THE COMMISSION

| Field | Value |
|-------|-------|
| **Type** | Final meta |
| **Unlocks** | After R3-META solved |
| **Answer** | TBD — Stage 5 |
| **Inputs** | PHASE-LOCKED PAIR (WHAT) + LOCKOUT (HOW) + SENIOR OFFICER (WHO) |

**Design intent:**
The three round-meta answers combine to reconstruct the incident. The final meta answer should be a word or phrase from the SPACEGAMIVERSE world vocabulary that names what happened during those six hours — the thing that was hidden.

**Candidate directions (to be resolved in Stage 5):**
- A world term that means "contact was made and then sealed" — e.g., FIRST WATCH (the first watch officer made first contact and ordered the cover-up)
- A world phrase combining elements of the three answers — PHASE-LOCKED implies a mirror; LOCKOUT implies sealing; SENIOR OFFICER implies authority
- A system state or console command that encapsulates the incident
- The world already contains the right word — the meta mechanism will reveal it

---

## Master Status

Answers in `ANSWERS.md` (ROT13).

| ID | Title | Dept | Status |
|----|-------|------|--------|
| R1-01 | Carrier Wave | COMMS | -- |
| R1-02 | Dead Reckoning | NAV | -- |
| R1-03 | Contact Report | TAC | -- |
| R1-04 | Gate Log | SEC | -- |
| R1-05 | Anomaly Station | SCIENCE | -- |
| R1-META | Telemetry Reconstruction | — | -- |
| R2-01 | Load Shedding | POWER | -- |
| R2-02 | Pathway Trace | COMPUTER | -- |
| R2-03 | Atmospheric Incident | ENVIRO | -- |
| R2-04 | Burn Pattern | PROPULSION | -- |
| R2-05 | Partial Recovery | AUXILIARY | -- |
| R2-META | Systems Reconstruction | — | -- |
| R3-01 | Vital Signs | MEDICAL | -- |
| R3-02 | Pattern Integrity | TRANSIT | -- |
| R3-03 | Access Record | SEC | -- |
| R3-04 | Repair Queue | DC | -- |
| R3-05 | Unauthorized Request | REPLICATOR | -- |
| R3-META | Crew Reconstruction | — | -- |
| FINAL | The Commission | — | -- |
