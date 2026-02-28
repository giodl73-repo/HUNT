# DEAD RECKONING — Puzzle Pool

**Stage 3 complete.** 15 slots × 3 variants = 45 candidates. Panel ranked. Winners selected.

**Design direction:** Every puzzle is experienced AT A CONTROL PANEL. The solver operates real ship instruments — oscilloscopes, switch banks, frequency selectors, gauge arrays, pressure displays, circuit breakers, targeting grids, ODN junction boards, pattern buffer displays, scanner readouts. The puzzle mechanic emerges from what that panel DOES in that room. Not documents — instruments.

---

## ROUND 1 — TELEMETRY

---

## R1-01: CARRIER WAVE (COMMS)

### Variant A: Interference Classifier ✓ SELECTED

The solver faces a **frequency spectrum display** — a horizontal band showing six live signal traces across the comm spectrum. Each trace exhibits a distinct visual signature: paired phase-locked spikes, a shattered irregular carrier, a triplet of harmonic peaks with ritualistic spacing, an absorption notch, etc. The solver turns a **band selector dial** to tune each trace into focus, then classifies it against the six interference types (Decoys, Shattered Carrier, Hieratic Triplet, Harmonic Echo, Phase-Locked Pair, Absorption Shadow) etched into the panel bezel. Once all six are correctly classified, the initial letters of the types spell the answer.

**Panel notes:**
- **Thomas Snyder**: Passes the computer test — a computer could pixel-match spectral shapes, but a human identifies them by *understanding what each interference mechanism physically does*. The Shattered Carrier looks shattered because a carrier wave was physically shattered. That's knowledge, not pattern-match.
- **Rand Miller**: High approval. The spectrum display IS a comm panel; reading interference off it IS what a COMMS officer does. No overlay. Riven Standard met cleanly.
- **Jonathan Blow**: One clean insight — once you realize the six types have distinct visual fingerprints that follow from their names, classification becomes inevitable. No backtracking.

### Variant B: Band Authorization Sequence

The solver faces an **eight-position rotary switch bank** — one switch per comm band, labeled with band name and indicator lights showing ACTIVE / LOCKED / BLOCKED. A log of nine transmission events scrolls on an adjacent screen. The solver determines which bands were actively broadcasting during the deleted window by cross-referencing event codes against a band-status reference table mounted on the panel.

**Panel notes:**
- **Wei-Hwa Huang**: Flags ambiguity — if two events share a band with conflicting status, the intended reading must be unambiguous. Needs an explicit temporal resolution rule.
- **Dan Katz**: Structure concern — "cross-referencing event codes against a status table" adds a lookup layer that distances solver from the panel. Closer to a document puzzle wearing panel clothes.
- **Dana Young**: Accessibility concern — eight bands plus nine events plus status codes is inventory overhead before the puzzle begins.

### Variant C: Subspace Triangulation

The solver operates a **dual-trace oscilloscope** comparing two signal channels simultaneously. Three **vernier knobs** (frequency offset, phase alignment, amplitude scale) are adjusted until the traces lock — indicated by a stable Lissajous figure. Four subspace band configurations produce four lock shapes; each shape encodes a letter.

**Panel notes:**
- **Jonathan Blow**: Risk of multi-step physical manipulation becoming fiddly rather than insightful. The epiphany needs to arrive clearly, not emerge from trial-and-error knob-twisting.
- **Kenny Young**: Buildability concern — Lissajous encoding requires precise analog simulation. Digitally tractable; physically expensive.
- **Rand Miller**: Conceptually strong. Execution risk is the knob-fiddling problem Blow identifies.

**Consensus: Variant A.** 9/12 panelists. Reading interference off a spectrum display IS the COMMS job; classification requires genuine understanding of each interference mechanism and passes Snyder's computer test cleanly.

---

## R1-02: DEAD RECKONING (NAV)

### Variant A: Sensor Comparison Array ✓ SELECTED

The solver faces a **multi-sensor comparison panel** — four rectangular display windows in a 2×2 grid, each showing a different sensor modality (gravitational, electromagnetic, thermal, particle) of the same region of space. Seven contacts appear across the windows but each contact only shows up in some modalities, not all. A real object has a characteristic multi-modal signature (a stellar body appears in all four; a comm buoy appears in EM and thermal only). The solver must identify which contacts are real objects and which are sensor artifacts by reading their cross-modal signature against a sensor-physics reference posted on the panel.

**Panel notes:**
- **Lucas Pope**: High approval. This is Papers Please for sensors — identify from evidence, flag anomalies, build a complete picture from partial information. The multi-modal signature system rewards deductive reasoning.
- **Thomas Snyder**: Passes the computer test — understanding WHY a comm buoy doesn't appear on gravitational sensors requires understanding what gravitational sensors measure.
- **Wei-Hwa Huang**: Requests that sensor physics be internally consistent and complete — no contact should be ambiguous given the reference table.

### Variant B: Navigation Plot Reconstruction

The solver faces a **chart table with a projected star plot** — 2D local space with velocity vectors for 17 tracked objects. The solver extrapolates where each object would be after 6 hours and compares against a current sensor sweep overlay to find matches.

**Panel notes:**
- **Dan Katz**: Strong narrative — this is literally dead reckoning, which is in the hunt title. The mechanic earns its name.
- **Thomas Snyder**: Red flag — vector extrapolation across 17 objects at varying velocities introduces computation. "Compute where this object is after 6 hours at heading 042 mark 7" is a math problem, not a deduction.
- **Mark Gottlieb**: Needs redesign so the solver *identifies* extrapolations rather than *performs* them.

### Variant C: Parallax Rangefinder

The solver operates a **dual-aperture rangefinder panel** — two identical viewfinder screens showing the same region photographed from sensor dishes 400km apart. The solver adjusts a **lateral offset dial** until contacts align in both windows; the dial reading gives distance. Contact distance ranges determine zone brackets (Near/Mid/Far) that encode the answer.

**Panel notes:**
- **Rand Miller**: Excellent instrument authenticity — parallax rangefinding is exactly what that panel does. Adjusting the offset dial until images align is satisfying physical interaction.
- **Jonathan Blow**: Zone-bracket encoding could be a clean single insight if brackets are unambiguous.
- **Peter Sarrett**: Experience concern — six individual dial-adjustments with no variation may feel repetitive rather than unfolding.

**Consensus: Variant A.** 10/12 panelists. Multi-modal sensor identification teaches the solver what each sensor type actually detects, is internally consistent and fair, and Lucas Pope's "identify from evidence" framework fits the hunt's reconstruction theme exactly.

---

## R1-03: CONTACT REPORT (TAC)

### Variant A: Targeting Grid Classification ✓ SELECTED

The solver faces a **tactical targeting display** — a polar grid (range rings to 50,000km, bearing markers) showing eleven contacts as blips. Each blip has four associated readings: radar cross-section (m²), thermal output (kW), EM emission (dBm), and maneuver pattern trace. The solver classifies each contact as REAL VESSEL or DECOY using a classification standard posted on the panel (e.g., real vessels show thermal output correlated with cross-section; decoys show EM emission disproportionate to cross-section). Real vessels read in bearing order give the answer.

**Panel notes:**
- **Lucas Pope**: Extremely strong. Each contact is a "document" assessed against rules that emerge from physics, not bureaucracy. The correlation between thermal output and cross-section is a rule that makes physical sense.
- **Thomas Snyder**: Passes the computer test decisively — understanding why a decoy's EM is disproportionate requires understanding what a decoy is trying to fake.
- **Dan Katz**: Pacing note — eleven contacts may be one or two too many. Eight gives the same deductive space with less fatigue.

### Variant B: Weapons Lock Disambiguation

A **fire control panel** with target-acquisition switches. A lock/release log of seven events displayed as waveforms shows signal acquisition and dropout. Overlapping locks create interference patterns (Hieratic Triplet when three locks overlap, etc.); the overlap pattern encodes the answer.

**Panel notes:**
- **Wei-Hwa Huang**: Ambiguity risk is high — tracking lock-state across seven events simultaneously will produce incorrect answers that seem correct unless sequencing is extremely tight.
- **Thomas Snyder**: Fails the computer test — a computer can track boolean states across a timeline perfectly. Bookkeeping, not understanding.
- **Dan Katz**: The mechanic doesn't require understanding what a weapons lock *is*.

### Variant C: ECM Signature Strip

The solver faces an **electronic countermeasures analysis panel** — a strip chart recorder showing 12 hours of ECM activity with a gap where the deleted window falls. Contacts before and after the gap have ECM signatures (pulse repetition frequency, modulation type, amplitude envelope). The solver matches pre-gap to post-gap contacts by persistent signature characteristics; matched vessels' designations spell the answer.

**Panel notes:**
- **Rand Miller**: The strip chart recorder is authentic — ECM analysts use continuous paper tape. The mechanic is exactly what that instrument is for. Strong Riven Standard compliance.
- **Peter Sarrett**: Narrative power — the gap in the tape IS the deleted six hours. The solver is physically looking at the evidence.
- **Jonathan Blow**: "Signatures before and after the gap belong to the same contacts" is a clean single insight.

**Consensus: Variant A.** 8/12 panelists. (Variant C received 4 endorsements — strong alternate.) Targeting grid classification wins because the four-parameter system teaches genuine tactical physics and is the cleanest implementation of real-vs-decoy identification.

---

## R1-04: GATE LOG (SEC)

### Variant A: Three-Gate Audit Panel ✓ SELECTED

The solver faces a **security access control panel** — three vertical columns of indicator lights, one per security gate (fore airlock, engineering corridor junction, computer core anteroom). Each indicator is color-coded by division (RED, YELLOW, ORANGE, BLUE) with timestamp and result (GRANTED / DENIED / FORCED). The deleted window appears as REDACTED across all three columns. The solver reconstructs who passed all three gates during the redacted period by identifying which crew members had active access cards — those whose last GRANTED event precedes the gap and whose next post-gap access requires the same authorization level. The sequence encodes the answer.

**Panel notes:**
- **Lucas Pope**: Papers Please applied to security logs. The "who could have passed all three gates" question requires building a model of access permissions from observed behavior, not from a stated ruleset.
- **Dan Katz**: The three-gate constraint creates a natural filter that reduces the candidate set without making the deduction trivial.
- **Wei-Hwa Huang**: Flags that card expiration rules must be explicit — needs a card-duration reference posted on the panel.

### Variant B: Replicator Clearance Matrix

A **replicator terminal panel** — a 6×8 grid of item slots, each showing an item name, security flag, and column of seven crew-clearance indicators. Denied requests during the gap are logged; the solver determines who made each denial by cross-referencing flag against crew clearance profiles.

**Panel notes:**
- **Dana Young**: A 6×8 grid of flags and clearances is heavy inventory before deduction begins.
- **Thomas Snyder**: Borderline — cross-referencing a clearance matrix is something a computer does perfectly. The human understanding element is thin.
- **Mark Gottlieb**: Would be stronger if the solver must infer what some flags mean rather than read them from a legend.

### Variant C: ODN Node Access Trace

The solver faces an **ODN junction board** — 15 nodes as labeled jacks with patch cables, plus a log of which nodes were accessed during the incident. The solver traces the access path through the ODN topology (certain nodes can only be reached through certain others) to determine the origin point, which identifies which crew role initiated the unauthorized access.

**Panel notes:**
- **Jonathan Blow**: Path-tracing offers a strong epiphany — once you realize the topology constrains which routes are possible, the origin is uniquely determined.
- **Mark Gottlieb**: High approval — tracing data flow backward through a network is exactly what a systems engineer does when diagnosing unauthorized access.
- **Kenny Young**: The 15-node topology fits comfortably on a single panel and is buildable.

**Consensus: Variant A.** 9/12 panelists. The three-gate structure is clean, the GRANTED/DENIED/FORCED system provides the right information density, and the three-gate filter naturally limits the answer space.

---

## R1-05: ANOMALY STATION (SCIENCE)

### Variant A: Multi-Spectrum Anomaly Classifier ✓ SELECTED

The solver faces a **science station with a six-band spectral analyzer** — the same anomalous region shown simultaneously in six wavelength bands (radio, microwave, infrared, visible, ultraviolet, X-ray), each as a false-color intensity map in its own viewport. Eight anomalous signatures appear; each shows up in some bands but not others, and the combination of bands in which a signature appears identifies its physical type. The solver classifies each signature using a spectral reference on the panel, then uses the classification names to extract the answer.

**Panel notes:**
- **Thomas Snyder**: Strong — classification requires understanding what each wavelength band detects, which is genuine astrophysical knowledge. A computer can cross-reference band combinations, but understanding *why* a neutron star doesn't emit visible light requires understanding neutron stars.
- **Alex Rosenthal**: Accessibility and wonder both served — the false-color multi-band display is visually arresting, and the moment when a solver understands *why* a signature appears in those specific bands is a genuine learning moment.
- **Rand Miller**: The spectral analyzer IS what that station does; reading anomaly signatures IS the science officer's job. Riven Standard met cleanly.

### Variant B: Probe Telemetry Reconstruction

A **probe control terminal** showing telemetry from four deployed probes, each with four instrument readouts — several readings missing or damaged. The solver reasons backward from partial telemetry to determine probe trajectories; the trajectories plotted on an overlay grid form a shape encoding the answer.

**Panel notes:**
- **Jonathan Blow**: Needs a clean single entry point — one reading that uniquely constrains one trajectory, unlocking the others. Without this, solvers will try random assignments.
- **Dan Katz**: Four probes × four instruments × missing readings creates a large underdetermined system. Risk of multiple valid solutions.
- **Wei-Hwa Huang**: Flags ambiguity — needs a proof that exactly one trajectory assignment satisfies all constraints.

### Variant C: Sensor Fusion Calibration Board

A **sensor calibration panel** — eight instrument modules each measuring a different physical quantity, each with a reading and a calibration offset dial. The anomaly produces internally inconsistent readings; some instruments are miscalibrated due to anomaly interference. The solver identifies truly-reading instruments by checking internal consistency; correctly-reading instruments in panel order spell the answer.

**Panel notes:**
- **Jonathan Blow**: Clean insight — once you understand that the anomaly interferes with specific sensor types (mapping to interference taxonomy vocabulary), identifying miscalibrated instruments follows inevitably.
- **Mark Gottlieb**: Sensor fusion and calibration consistency checking is real engineering practice.
- **Thomas Snyder**: Strong — a computer can check numerical consistency, but understanding *why* a gravitational anomaly affects neutrino sensors differently than EM sensors requires understanding the anomaly's physical nature.

**Consensus: Variant A.** 9/12 panelists. (Variant C received 3 endorsements — strong alternate.) The multi-spectrum classifier wins because the false-color six-band display is the most visually distinctive instrument in Round 1, the spectral classification system is grounded in real physics, and the wonder criterion is met.

---

## Round 1 Summary

| Slot | Selected | Score | Key endorsement |
|------|---------|-------|----------------|
| R1-01 | Interference Classifier (spectrum display) | 9/12 | Rand Miller + Snyder: reading the display IS the COMMS job |
| R1-02 | Sensor Comparison Array (4-window panel) | 10/12 | Lucas Pope + Snyder: multi-modal ID rewards genuine sensor knowledge |
| R1-03 | Targeting Grid Classification (polar grid) | 8/12 | Lucas Pope + Blow: four-parameter system teaches tactical physics |
| R1-04 | Three-Gate Audit Panel (indicator columns) | 9/12 | Lucas Pope + Katz: three-gate filter is clean; GRANTED/DENIED/FORCED is right density |
| R1-05 | Multi-Spectrum Classifier (false-color analyzer) | 9/12 | Miller + Rosenthal: six-band display is visually distinctive; Riven Standard met |

**Instrument variety:** frequency spectrum display, multi-sensor comparison array, polar tactical grid, security indicator column bank, false-color spectral analyzer. No instrument type repeats.

---

## ROUND 2 — SYSTEMS LOG

---

## R2-01: LOAD SHEDDING (POWER)

### Variant A: Cascade Trace

The EPS load board shows 12 distribution nodes as illuminated **rocker switches**, each with an MW readout gauge. The fault log shows timestamps of which nodes tripped offline in sequence but not causes. The solver reads the trip sequence and cross-references MW values against EPS grid capacity rules (impulse draws first, then shields, then environmental fields) to reconstruct the cascade and identify which single node initiated the failure.

**Panel notes:**
- **Wei-Hwa Huang**: The deductive chain is tight — each elimination step is forced, not guessed.
- **Mark Gottlieb**: EPS capacity math is real systems engineering, not decoration.
- **Kenny Young**: 12 rocker switches with gauges is buildable at a table with paper instruments.

### Variant B: Ghost Load ✓ SELECTED

The EPS distribution panel shows current draw values for eight systems, but three systems show loads that don't match their operating states — drawing power while listed as OFFLINE. The solver maps the "phantom" draw values against known EPS consumption tables (impulse = 180 MW, shields = 95 MW, etc.) to identify what was actually running secretly; the three hidden systems' ODN node designations index into the answer.

**Panel notes:**
- **Rand Miller**: The discrepancy between the panel's stated status and actual numbers is the Riven standard — the instrument itself reveals the lie.
- **Thomas Snyder**: The solver must understand what each system consumes to name it, not just pattern-match numbers.
- **Jonathan Blow**: The aha is realizing the ship was running something it denied running. Clean single insight.

### Variant C: Power Tier Ladder

A **three-tier power display** shows Battery, Fusion, and M/AM outputs with bar gauges. The deleted logs show power tier transitions. The solver reasons about which systems require which tiers and reconstructs the operational sequence from tier-change timestamps; each transition contributes a letter.

**Panel notes:**
- **Dan Katz**: Clean structure — tier transitions are discrete events, making enumeration tractable.
- **Dana Young**: Accessible if the table is provided, and the deduction chain is still real.
- **Alex Rosenthal**: The three-tier visual is genuinely beautiful when the solver grasps the full power architecture at once.

**Consensus: Variant B.** 9/12 panelists. The phantom load mechanic makes the instrument itself the evidence of deception, satisfying the Riven standard while demanding genuine systems knowledge.

---

## R2-02: PATHWAY TRACE (COMPUTER)

### Variant A: Junction Reroute ✓ SELECTED

The **ODN junction board** displays 15 nodes as a physical grid with toggle switches at each active routing junction. The board shows final states; the maintenance log provides toggle timestamps without labels. The solver traces data pathways through the grid using ODN routing rules (Primary Computer must maintain Navigation; active junctions pass data, passive junctions block) to determine which nodes lost service in what order, spelling the answer from two-letter node codes.

**Panel notes:**
- **Wei-Hwa Huang**: The routing rules create a genuine constraint-satisfaction problem — the board state plus toggle count uniquely determines the sequence.
- **Lucas Pope**: Pure deductive identification — the answer cannot be guessed, only proved.
- **Mark Gottlieb**: ODN topology is real systems-engineering; tracing data flow backward through a network is exactly how unauthorized access is diagnosed.

### Variant B: Gelpack Infection

A **computer health panel** shows ODN component status. Three Bioneural Gelpacks show degraded performance (response latency gauges reading high — they're "sick"). The solver determines which data pathways route through all three sick gelpacks; compromised functions' names provide the extraction.

**Panel notes:**
- **Thomas Snyder**: The organic vs. mechanical distinction is meaningful — a solver who doesn't know gelpacks are biological cannot solve this correctly. Exactly the right knowledge gate.
- **Mike Selinker**: "The computer has an infection" is a vivid narrative beat.
- **Jonathan Blow**: The epiphany — realizing organic components have organic failure modes — recontextualizes the entire board.

### Variant C: Access Depth Map

A **console access panel** shows location-coded data queries from the deleted window, each tagged with an access depth level (-2, -3, -4). The solver cross-references the access depth table (Bridge=Full, Department Station=-2, General Area=-3, Personal Quarters=-4) to identify which locations each query could have originated from, using elimination to pin each query to a specific room; room IDs encode the answer.

**Panel notes:**
- **Dan Katz**: Room ID format ({deck}-{section}-{room}) is elegant enumeration — each resolved room contributes exactly one element.
- **Dana Young**: Access depth concept needs a clear reference table, but once provided it's accessible.
- **Peter Sarrett**: The feeling of triangulating an intruder through their console footprint is a compelling experience frame.

**Consensus: Variant A.** 10/12 panelists. The junction toggle trace is the cleanest "operating the ODN board" experience — the solver is literally routing data, not reading about routing.

---

## R2-03: ATMOSPHERIC INCIDENT (ENVIRO)

### Variant A: Zone Pressure Sequence

An **environmental zone control panel** — eight compartments as a grid of analog pressure gauges, each with a needle and a colored nominal-range band. Pressure deviations during the deleted window (above-nominal vs. below-nominal) identify which zones were affected. The zone designations provide the answer.

**Panel notes:**
- **Kenny Young**: Analog gauge arrays are a classic physical prop — eight gauges on a grid is buildable and tactile.
- **Thomas Snyder**: Binary deviation (above/below nominal) is elegant — the solver must understand what nominal pressure IS before classifying each gauge.
- **Wei-Hwa Huang**: The spatial pattern of affected zones is the deductive constraint.

### Variant B: Atmospheric Chemistry Panel ✓ SELECTED

The **ENVIRO chemistry panel** shows six atmospheric sensors per zone: O2 percentage, CO2 percentage, N2 percentage, trace elements, temperature, and humidity — all as dial gauges. Three zones show readings subtly off-nominal in specific ways suggesting something was deliberately released. The solver identifies the chemical signature from the multi-gauge readings and matches it against the replicator's hazardous material table (Table 3), with the matched compound names providing extraction.

**Panel notes:**
- **Rand Miller**: The multi-instrument signature that only makes sense when you read ALL the gauges together is the Riven standard applied to chemistry.
- **Mark Gottlieb**: The ENVIRO-to-REPLICATOR cross-system connection is authentic ship engineering — atmosphere contamination and replicator security flags genuinely interact.
- **Jonathan Blow**: The epiphany is recognizing a compound from its atmospheric signature, not from being told what it is.

### Variant C: SIF Field Collapse

The physics side of the ENVIRO panel shows Structural Integrity Field (SIF) status for each deck as percentage gauges with warning lights. SIF fields dropped in sequence during the deleted window; the solver maps the drop sequence against the ship's deck layout and EPS routing to derive the answer from deck numbers.

**Panel notes:**
- **Dan Katz**: Deck-number enumeration is clean and the SIF drop sequence is temporally ordered.
- **Dana Young**: Solvers need to know SIF runs on EPS, not electrical — fair but a real knowledge gate.
- **Alex Rosenthal**: The visual of an entire deck's structural field collapsing in sequence is dramatic and makes stakes feel real.

**Consensus: Variant B.** 9/12 panelists. Reading a six-gauge atmospheric signature to identify a compound satisfies the Riven standard maximally — the solver must read six instrument types simultaneously, which is exactly what an ENVIRO station operator does.

---

## R2-04: BURN PATTERN (PROPULSION)

### Variant A: Thrust Vector Recovery

A **circular thrust vector display** — four RCS thruster pairs as directional indicators with burn-duration gauges. Eight burns logged during the deleted window as durations and directions, but with corrupted vector labels. The solver reconstructs the ship's actual trajectory by composing thrust vectors; the course plotted on a corner star chart spells a letter sequence in constellation-point notation.

**Panel notes:**
- **Thomas Snyder**: Vector composition is genuine deduction — you cannot guess the answer without understanding what each thruster pair does.
- **Wei-Hwa Huang**: The step-by-step forced reconstruction is the kind of chain where each element is uniquely determined.
- **Kenny Young**: The circular thruster grid is a striking prop, but the star chart extraction layer may be one step too many.

### Variant B: Fuel Consumption Audit

The **propulsion fuel console** shows three tanks (RCS propellant, Fusion fuel, Antimatter) with precision consumption gauges. The solver divides total six-hour consumption by known burn rates to determine how many hours each propulsion mode was active; the mode sequence (Thrusters/Impulse/Warp → T/I/W) combined with hour-counts encodes the answer.

**Panel notes:**
- **Mark Gottlieb**: The three-tier propulsion architecture (electrical/EPS/M/AM) makes this a direct application of power tier knowledge.
- **Dan Katz**: Clean structure — three tanks, six hours, forced division.
- **Peter Sarrett**: The experience of auditing a fuel log like an investigator is compelling, but the arithmetic may feel like computation without deduction unless burn-rate tables are elegantly designed.

### Variant C: Impulse Signature Trace ✓ SELECTED

The **impulse console** shows the EPS power draw graph for the impulse drive — a **strip chart recorder** with a scrolling display of MW consumption over the deleted window. The consumption curve has eight distinctive events: power-up spikes, cruise plateaus, deceleration dips. The solver matches each event type against the impulse drive's operating profile (acceleration = 180 MW peak, cruise = 120 MW steady, decel = 60 MW) to identify the maneuver sequence; the initial letters of the maneuver names spell the answer.

**Panel notes:**
- **Jonathan Blow**: The strip chart recorder is a physically satisfying instrument — the solver watches the ship's behavior encoded in a line.
- **Thomas Snyder**: Matching consumption curves to maneuver types requires understanding impulse drive physics, not just reading labels.
- **Rand Miller**: The instrument IS the record of what the ship did — Riven Standard.

**Consensus: Variant C.** 10/12 panelists. The strip chart recorder is the most instrument-native mechanic — the solver reads behavior from a continuous waveform, exactly what an engineering console provides, and the deduction chain from MW values to maneuver names is tight.

---

## R2-05: PARTIAL RECOVERY (AUXILIARY)

### Variant A: Diagnostic Terminal Reconstruction

The **master diagnostics terminal** shows a partial system health scan — 12 system entries, each with status indicators across five diagnostic fields (power draw, data throughput, error count, temperature, last-accessed timestamp). Eight entries are fully readable; four have corrupted fields. The solver uses known relationships between fields (ONLINE state must show nonzero data throughput; LOCKED state must show zero last-accessed timestamp) to fill in the corrupted fields; reconstructed entries in row order provide the answer.

**Panel notes:**
- **Lucas Pope**: Pure deductive identification from system-state rules — the solver fills in blanks by logical necessity, not by guessing.
- **Wei-Hwa Huang**: The six system states and their behavioral implications are the constraint set, and using them correctly is the test.
- **Dana Young**: Requires knowing all six states' behavioral implications — the reference card must be on the puzzle.

### Variant B: Interleaved Log Fragment ✓ SELECTED

The **auxiliary terminal** shows two interleaved log streams — one from the Electrical grid monitor, one from the EPS grid monitor — merged into a single timestamp-ordered feed with source labels stripped. Each event is marked only with a value (kW or MW), a system name, and a status change. The solver separates the two streams by reasoning about which systems run on which grid (computer/comms/sensors/lighting/environmental/weapons-fire-control = Electrical; impulse/shields/env-fields/transporter/replicator = EPS), then reads each stream independently to reconstruct the event sequence.

**Panel notes:**
- **Mark Gottlieb**: The Electrical vs. EPS split is authentic systems engineering — knowing which systems belong to which grid is genuine world knowledge.
- **Thomas Snyder**: The separation step is a clean deductive gate; the solver cannot proceed without correctly classifying every system.
- **Dan Katz**: Two interleaved streams is a classic puzzle structure, and the knowledge required to separate them makes it more than pattern matching.

### Variant C: Console Access Archaeology

The master diagnostics terminal shows which console (by location) accessed which system node at what time — but console location labels are replaced with access-depth codes (-2, -3, -4, Full). The solver reasons backward: given the system accessed and the depth required, identify specific rooms that were active; the room-ID sequence encodes the answer.

**Panel notes:**
- **Peter Sarrett**: Forensic archaeology through a diagnostic terminal is the right frame for the reconstruction theme.
- **Jonathan Blow**: Backward-reasoning from capability to location is a genuine epiphany structure.
- **Kenny Young**: Logic chain is sound but needs careful enumeration to avoid multiple valid solutions.

**Consensus: Variant B.** 9/12 panelists. Separating two interleaved log streams by system classification is the clearest expression of "what did the ship's systems record?" and rewards genuine knowledge of the Electrical/EPS architecture.

---

## Round 2 Summary

| Slot | Selected | Score | Key endorsement |
|------|---------|-------|----------------|
| R2-01 | Ghost Load (EPS panel with phantom draws) | 9/12 | Miller: instrument reveals deception; Snyder: must know consumption values |
| R2-02 | Junction Reroute (ODN toggle board) | 10/12 | Huang + Pope: routing constraint is uniquely determined, not guessable |
| R2-03 | Atmospheric Chemistry (6-gauge per zone) | 9/12 | Miller: multi-gauge signature; Gottlieb: cross-system ENVIRO↔REPLICATOR authenticity |
| R2-04 | Impulse Signature Trace (strip chart) | 10/12 | Snyder + Blow: waveform demands understanding of impulse physics |
| R2-05 | Interleaved Log Fragment (EPS/Electrical merge) | 9/12 | Gottlieb + Snyder: Electrical/EPS separation is authentic systems knowledge |

---

## ROUND 3 — CREW RECORD

---

## R3-01: VITAL SIGNS (MEDICAL)

### Variant A: Biometric Sequence

The **sickbay diagnostic display** shows six biometric monitoring panels — heart rate, blood pressure, neural activity, body temperature, O2 saturation, and respiration — each as a scrolling waveform display. Recordings from the deleted window for six crew members show anomaly signatures; two waveforms per patient indicate physical trauma vs. pharmacological intervention. The solver identifies the anomaly type from the waveform shape and matches intervention timing against the sickbay **medication log panel** (LED indicators per drug).

**Panel notes:**
- **Thomas Snyder**: Reading anomaly type from waveform shape requires understanding what normal and abnormal waveforms look like — a genuine knowledge test.
- **Peter Sarrett**: Sickbay is the most human room on the ship. Making it about what happened to people through their vital signs creates real emotional stakes.
- **Mike Selinker**: The combination of waveform panel plus medication log makes the solver feel like a ship's doctor.

### Variant B: Scanner Calibration Anomaly ✓ SELECTED

The **medical scanner display** shows a calibration panel — a grid of 20 body-system readings from a single scan taken on an unknown crew member during the deleted window, each as a bar gauge against a reference range. Eight gauges show values that cannot coexist in a single healthy individual, but DO coexist in someone who has received a specific treatment or experienced a specific event. The solver identifies the physiological signature from the inconsistent combination of readings; the medical terminology for that condition spells the answer.

**Panel notes:**
- **Wei-Hwa Huang**: The inconsistency-detection structure is rigorous — find the subset of gauges that conflict and determine what condition explains all conflicts simultaneously.
- **Jonathan Blow**: The epiphany of recognizing a condition from its symptom constellation — rather than from being told — is the ideal puzzle experience.
- **Lucas Pope**: Deductive identification in its purest form.

### Variant C: Surgical Log Display

A **sickbay surgical terminal** shows a procedure log — eight procedures logged as sequences of tool activations on a touchscreen panel (each tool a labeled button that illuminates when active). Procedure names are missing; only tool activation orders remain. The solver identifies each procedure from its tool sequence (bone-knitter + dermal regenerator = wound closure; neural stimulator + cortical monitor = neurological intervention, etc.); the procedure sequence in chronological order spells the answer.

**Panel notes:**
- **Dana Young**: The surgical tool vocabulary is learnable and the identification task is approachable once the tool glossary is provided.
- **Kenny Young**: A touchscreen panel with illuminated tool buttons is a clean physical prop.
- **Dan Katz**: Eight procedures from tool sequences is potentially underconstrained — needs clear tool-to-procedure mappings.

**Consensus: Variant B.** 10/12 panelists. Reading a physiological signature from an inconsistent gauge array is exactly what a medical scanner display does, and the condition-identification epiphany satisfies both the Riven standard and Blow's epiphany design criterion.

---

## R3-02: PATTERN INTEGRITY (TRANSIT)

### Variant A: Targeting Grid Lock-On ✓ SELECTED

The **transporter targeting screen** shows a spatial grid with eight targeting reticles — each with coordinates, a signal-lock quality gauge (circular signal-strength display), and a personnel ID tag. Seven transports are logged during the deleted window; five completed normally, two show "lock degraded" status. The solver determines from lock quality readings and coordinates whether each transport originated on-ship or off-ship, and whether any pattern was simultaneously in the buffer with another — only possible if buffer size was artificially expanded. The crew IDs of simultaneous-buffer occupants index into the answer.

**Panel notes:**
- **Mike Selinker**: Two patterns in the buffer simultaneously is a chilling narrative beat — someone was deliberately obscured in transit.
- **Rand Miller**: The targeting grid coordinates plus lock quality gauges give the solver everything they need without telling them what to look for.
- **Thomas Snyder**: The simultaneous-buffer constraint is a clear rule that makes the deduction forced.

### Variant B: Pattern Buffer Decay

A **pattern buffer display** shows 12 storage cells, each with a signal integrity gauge (0–100%), a timestamp, and a decay-rate indicator. Three cells show integrity values inconsistent with their timestamps and the standard decay rate — decayed faster or slower than physics allows. The solver identifies tampered cells and determines whether faster or slower decay indicates active interference; cell numbers of tampered patterns provide extraction.

**Panel notes:**
- **Mark Gottlieb**: Pattern buffer decay is a real engineering parameter, and inconsistency with the decay formula is a genuine fault condition.
- **Wei-Hwa Huang**: The three-category classification (faster/slower/normal) combined with timestamp arithmetic is a tight constraint.
- **Lucas Pope**: The solver must calculate expected integrity from first principles to identify anomalies.

### Variant C: Bio-Filter Log

A **transporter bio-filter panel** — a grid of filter threshold gauges, one per transport event, each with a dial indicating sensitivity level. In three transports, the bio-filter was manually set below standard threshold. The solver determines what passes at below-standard threshold, cross-references with the diagnostic log (shows material class detected but not blocked); passed materials' names encode the answer.

**Panel notes:**
- **Peter Sarrett**: Deliberately lowering a bio-filter to smuggle something through is a vivid act of agency — the solver is reconstructing a specific choice.
- **Jonathan Blow**: The connection between filter threshold and material class requires understanding what filters do, not just reading values.
- **Dan Katz**: Extraction from material class names is clean and enumerable.

**Consensus: Variant A.** 9/12 panelists. The simultaneous-buffer-occupancy mechanic is the most compelling narrative beat of the three variants, and the targeting grid gives solvers a genuinely spatial, instrument-native experience.

---

## R3-03: ACCESS RECORD (SEC — personnel)

### Variant A: Badge Swipe Chronology ✓ SELECTED

The **security terminal** displays a badge swipe log as a scrolling LED readout — each entry shows a door ID, timestamp, personnel ID, and result (GRANTED/DENIED). During the deleted window, 18 swipes are logged. Four personnel IDs appear in combinations of rooms that are physically impossible given the ship's layout — the same person cannot swipe into Room A and Room B within 30 seconds if those rooms are three decks apart. The solver maps the impossible swipes using room IDs ({deck}-{section}-{room}) to identify cloned badges; cloned badge holders' personnel ID numbers index into the answer.

**Panel notes:**
- **Lucas Pope**: Badge-clone detection from physical impossibility is pure deductive identification — the solver cannot guess, only prove.
- **Wei-Hwa Huang**: The room-ID spatial constraint is tight and unambiguous.
- **Dan Katz**: 18 swipes is the right density — enough to need careful tracking, not so many it becomes a slog.

### Variant B: Force Field Matrix

A **security force field panel** — a 6×6 grid of corridor segments, each with a status indicator (ACTIVE/INACTIVE) and toggle switch. The force field log shows the grid state at six timestamps. The solver traces open paths at each timestamp and determines which compartments were accessible; the intersection of "accessible" and "occupied" narrows down which crew members could have reached which locations.

**Panel notes:**
- **Mike Selinker**: The force field grid as a physical toggle panel is exactly the right tactile experience for a security puzzle.
- **Thomas Snyder**: Path-tracing through a binary grid is genuinely solvable without computation by a careful human.
- **Kenny Young**: A 6×6 grid with toggle switches is a clean, producible prop.

### Variant C: Biometric Verification Log

A **security verification terminal** shows a biometric scan log — each entry is a radar-chart display of six biometric parameters (retinal, voice, fingerprint, gait, heartbeat, neural pattern) with match-confidence bars. Eight verifications occurred during the deleted window; two show unusual parameter combinations (high confidence on some metrics, anomalously low on others). The solver identifies which parameters were spoofed (live person = correlated biometrics; spoofed = uncorrelated) and the spoofed parameters' names provide the answer.

**Panel notes:**
- **Jonathan Blow**: The correlation structure — biometrics that should covary but don't — is a beautiful epiphany gate.
- **Rand Miller**: The radar chart makes spoofing visually apparent once the solver understands what correlation should look like. Riven Standard.
- **Peter Sarrett**: The experience of catching an identity fraud through biometric inconsistency is uniquely satisfying.

**Consensus: Variant A.** 9/12 panelists. Badge-clone detection through physical impossibility is the tightest deductive structure and makes the room-ID system feel essential rather than decorative.

---

## R3-04: REPAIR QUEUE (DAMAGE CONTROL)

### Variant A: Priority Board State ✓ SELECTED

The **damage control board** shows a 5×5 grid of system status indicator tiles — each cycling through the six system states (◉ ◇ ◌ ⊘ ○ ●). The board log shows six snapshots from the deleted window. The solver traces each system's state transition across snapshots and identifies systems that followed an unusual sequence. The standard recovery sequence is ● FAULT → ◌ STANDBY → ◇ READY → ◉ ONLINE; any system that went ● FAULT → ⊘ LOCKED skipped standard repair and was deliberately isolated. The deliberately isolated systems' designations encode the answer.

**Panel notes:**
- **Mark Gottlieb**: The six system states and their expected transition sequences are real operational knowledge, and detecting violations of the expected sequence is genuine systems engineering reasoning.
- **Thomas Snyder**: The standard sequence vs. anomalous sequence distinction is a clean binary that makes each system classifiable without ambiguity.
- **Wei-Hwa Huang**: Six snapshots × 25 cells is the right density for a constraint-satisfaction problem that remains humanly tractable.

### Variant B: Repair Crew Routing

A **damage control dispatch panel** — eight crew teams displayed as movable tokens on a ship schematic, each with a current-room display and a task queue. Eight dispatch events show teams assigned, rerouted, or stood down. The solver reconstructs where each team was at each moment using room-ID and travel-time constraints; unassigned jobs' system names provide the answer.

**Panel notes:**
- **Dan Katz**: Routing-with-time-constraints is classic logic puzzle territory, and the ship schematic gives it spatial grounding.
- **Kenny Young**: Movable token dispatch boards are buildable and physically engaging.
- **Peter Sarrett**: Following repair crews through a crisis — watching what got fixed and what got ignored — is emotionally resonant.

### Variant C: Structural Integrity Display

The damage control board shows hull integrity readings for 12 hull sections as analog gauges with rate-of-change indicators. At six timestamps, the solver identifies which sections gained vs. lost integrity; the repair attention pattern projected onto the hull geometry spells a letter.

**Panel notes:**
- **Jonathan Blow**: Deriving a letter from hull repair patterns viewed geometrically is a pure visual epiphany.
- **Rand Miller**: The data IS the answer, decoded only by understanding the spatial structure. Riven Standard applied to damage assessment.
- **Alex Rosenthal**: The image of a ship's hull showing the outline of a hidden message in its repair scars is memorable.

**Consensus: Variant A.** 10/12 panelists. The system-state transition anomaly detection is the most rigorous application of the six-state vocabulary and rewards exactly the operational knowledge the hunt's world is built on.

---

## R3-05: UNAUTHORIZED REQUEST (REPLICATOR)

### Variant A: Security Flag Audit ✓ SELECTED

The **replicator terminal** shows a pattern database display — 20 replication requests from the deleted window, each showing a pattern name, quantity, timestamp, and security flag (OPEN, BIO, HAZ, POISON, SEC, UNK). A second panel shows the authorization log (who approved each flag). The solver checks each request against the authorization requirements (OPEN = none; BIO = medical auth; HAZ = department chief; POISON = CMO + Captain; SEC = security clearance; UNK = Science Officer review) and identifies which requests were processed with insufficient authorization. The pattern names of unauthorized requests provide the answer.

**Panel notes:**
- **Dana Young**: The flag-to-authorization mapping is a clear rule set that can be placed directly on the puzzle as a reference card, making the logic accessible while keeping the deduction real.
- **Mark Gottlieb**: This directly tests the replicator security flag system, world-specific knowledge at its most granular.
- **Dan Katz**: "Find the violations" is a clean puzzle structure with unambiguous right answers.

### Variant B: Pattern Substitution

A **replicator pattern database terminal** — each entry has a molecular pattern ID, display name, and flag. Eight patterns were replicated during the gap; three show discrepancies between display name and molecular pattern ID. The pattern IDs use a structured naming convention (molecular class prefix + compound suffix) that reveals what was actually replicated; the solver decodes the actual compound from the ID syntax and matches it against the security flag database.

**Panel notes:**
- **Thomas Snyder**: Decoding a naming convention is a genuine deductive task — the solver must understand the syntax to identify the substitution.
- **Jonathan Blow**: Discovering that display names lie but molecular IDs don't is a clean epiphany.
- **Lucas Pope**: Substitution detection is pure logical identification, and the pattern-ID syntax is learnable.

### Variant C: Replication Queue Reconstruction

A **replicator terminal** with a numeric readout for total mass replicated and a per-material energy-consumption gauge. The solver uses the total energy consumed and known energy-density values to work backward and determine which combination of materials accounts for the full energy budget; the materials' names provide the answer.

**Panel notes:**
- **Kenny Young**: Energy-budget arithmetic is clean to build around — the total constrains the solution uniquely if the material list is carefully designed.
- **Wei-Hwa Huang**: The purest forced-deduction variant — the numbers uniquely determine the answer.
- **Peter Sarrett**: Reconstructing a deleted record from its energy footprint is a satisfying forensic frame.

**Consensus: Variant A.** 11/12 panelists. The security flag audit is the most direct engagement with the replicator's world-specific vocabulary, tests all six flag types, and creates a clear investigative frame where the solver is operating the actual authorization system.

---

## Round 3 Summary

| Slot | Selected | Score | Key endorsement |
|------|---------|-------|----------------|
| R3-01 | Scanner Calibration Anomaly (gauge grid) | 10/12 | Blow + Pope: condition-identification epiphany; pure deductive identification |
| R3-02 | Targeting Grid Lock-On (spatial grid) | 9/12 | Selinker: simultaneous-buffer narrative; Miller: grid gives everything needed |
| R3-03 | Badge Swipe Chronology (LED readout) | 9/12 | Pope: physical impossibility is irrefutable; Huang: room-ID constraint tight |
| R3-04 | Priority Board State (6-state tile grid) | 10/12 | Gottlieb + Snyder: state-transition anomaly tests operational world knowledge |
| R3-05 | Security Flag Audit (pattern database + auth log) | 11/12 | Young + Gottlieb: accessible; direct engagement with world-specific flag vocabulary |

---

## Master Summary — All 15 Selected

| ID | Title | Dept | Instrument | Puzzle type | Score |
|----|-------|------|-----------|-------------|-------|
| R1-01 | Carrier Wave | COMMS | Frequency spectrum display | Interference classification | 9/12 |
| R1-02 | Dead Reckoning | NAV | Multi-sensor comparison array | Cross-modal contact ID | 10/12 |
| R1-03 | Contact Report | TAC | Polar targeting grid | Real vs. decoy classification | 8/12 |
| R1-04 | Gate Log | SEC | Security indicator columns | Three-gate access deduction | 9/12 |
| R1-05 | Anomaly Station | SCI | False-color spectral analyzer | Multi-band signature classification | 9/12 |
| R2-01 | Load Shedding | POWER | EPS load board | Phantom draw identification | 9/12 |
| R2-02 | Pathway Trace | COMPUTER | ODN junction toggle board | Network routing trace | 10/12 |
| R2-03 | Atmospheric Incident | ENVIRO | 6-gauge chemistry panel | Multi-gauge compound signature | 9/12 |
| R2-04 | Burn Pattern | PROPULSION | Strip chart recorder | Waveform-to-maneuver matching | 10/12 |
| R2-05 | Partial Recovery | AUXILIARY | Auxiliary terminal | Interleaved stream separation | 9/12 |
| R3-01 | Vital Signs | MEDICAL | Medical scanner gauge grid | Physiological signature ID | 10/12 |
| R3-02 | Pattern Integrity | TRANSIT | Transporter targeting screen | Buffer-state deduction | 9/12 |
| R3-03 | Access Record | SEC | Badge swipe LED readout | Physical impossibility detection | 9/12 |
| R3-04 | Repair Queue | DC | Damage control status tiles | State-transition anomaly detection | 10/12 |
| R3-05 | Unauthorized Request | REPLICATOR | Pattern database terminal | Security flag audit | 11/12 |

**Instrument variety:** 15 puzzles, 15 distinct instrument types. No instrument repeats across the hunt.

**Meta vocabulary seeded by the pool:**
- Round 1 (WHAT): interference type names, sensor modality names, contact classification terms → candidate answer words: HARMONIC, ABSORPTION, PHASE-LOCKED, HIERATIC
- Round 2 (HOW): system state names, EPS/Electrical grid vocabulary, ODN node names → candidate answer words: STANDBY, LOCKOUT, ISOLATION, REROUTED, CALIBRATE
- Round 3 (WHO): security flag categories, crew rank vocabulary, system state vocabulary → candidate answer words: DIAGNOSTICIAN, COMMANDANT, ADJUTANT, PROVOST
