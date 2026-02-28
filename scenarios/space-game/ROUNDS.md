# DEAD RECKONING — Structure

## Architecture

```
DEAD RECKONING — 3 rounds, 15 puzzles + 3 round-metas + 1 final meta

Round 1: TELEMETRY — what was out there
├── R1-01: Comm Intercept          (COMMS)      → feeder answer
├── R1-02: Navigation Trace        (NAV)        → feeder answer
├── R1-03: Contact Report          (TAC)        → feeder answer
├── R1-04: Access Audit            (SEC)        → feeder answer
└── R1-05: Anomaly Classification  (SCIENCE)    → feeder answer
         │
         ▼
    R1-META: Telemetry Reconstruction             → WHAT was encountered
         │
         ▼
Round 2: SYSTEMS LOG — what the ship recorded
├── R2-01: Power Consumption Record  (POWER)     → feeder answer
├── R2-02: ODN Pathway Trace         (COMPUTER)  → feeder answer
├── R2-03: Environmental Log         (ENVIRO)    → feeder answer
├── R2-04: Propulsion Output Record  (PROPULSION)→ feeder answer
└── R2-05: Master Diagnostics Report (AUXILIARY) → feeder answer
         │
         ▼
    R2-META: Systems Reconstruction               → HOW the ship responded
         │
         ▼
Round 3: CREW RECORD — what happened to the people
├── R3-01: Medical Intake Log        (MEDICAL)   → feeder answer
├── R3-02: Transporter Usage Log     (TRANSIT)   → feeder answer
├── R3-03: Personnel Badge Record    (SEC)       → feeder answer
├── R3-04: Damage Assessment Report  (DAMAGE CONTROL) → feeder answer
└── R3-05: Replicator Request Log    (REPLICATOR)→ feeder answer
         │
         ▼
    R3-META: Crew Reconstruction                  → WHO ordered the cover-up
         │
         ▼
    FINAL META: The Commission                    → THE INCIDENT (what happened
                                                    and why it was hidden)
```

## Puzzle Count

| Category | Count |
|----------|-------|
| Feeder puzzles | 15 |
| Round metas | 3 |
| Final meta | 1 |
| **Total** | **19** |

## Unlock Structure

- **Round 1** available from start
- **R1-META** unlocks when 4 of 5 Round 1 feeders are solved (80% rule)
- **Round 2** unlocks when R1-META is solved
- **R2-META** unlocks when 4 of 5 Round 2 feeders are solved
- **Round 3** unlocks when R2-META is solved
- **R3-META** unlocks when 4 of 5 Round 3 feeders are solved
- **FINAL META** unlocks when R3-META is solved

Within each round all 5 feeders are available simultaneously.

## Puzzle Slot Detail

### Round 1 — TELEMETRY

| ID | Title | Department | Document type | Puzzle type | Difficulty |
|----|-------|-----------|---------------|-------------|------------|
| R1-01 | Comm Intercept | COMMS | Partial comm log with interference markers | Interference classification — identify the type from waveform data, select removal technique | Medium |
| R1-02 | Navigation Trace | NAV | Sensor array comparison report | Triangulation / sensor deduction — one sensor is drifting; cross-reference three readings to find true position during the gap | Medium |
| R1-03 | Contact Report | TAC | Tactical contact classification sheet | Logic grid — distinguish real contacts from decoys using multi-band sensor data | Medium-Hard |
| R1-04 | Access Audit | SEC | Console access log with clearance gates | Three-gate security deduction — which console, which deck, which clearance level could have made the deletion | Medium |
| R1-05 | Anomaly Classification | SCIENCE | Multi-spectrum sensor sweep | Sensor fusion — combine nav, tactical, and science sensor readings to classify the anomaly type | Hard |
| R1-META | Telemetry Reconstruction | — | Reconstructed signal log | Round-meta → answer drawn from world comm/sensor vocabulary | — |

### Round 2 — SYSTEMS LOG

| ID | Title | Department | Document type | Puzzle type | Difficulty |
|----|-------|-----------|---------------|-------------|------------|
| R2-01 | Power Consumption Record | POWER | EPS load and electrical draw log | Cascade failure trace — power usage during the gap shows a dependency chain that shouldn't have been active | Medium |
| R2-02 | ODN Pathway Trace | COMPUTER | ODN network topology diagram with routing log | Network topology — trace a data packet through the ODN, find the tap or bottleneck that reveals what was running | Medium |
| R2-03 | Environmental Log | ENVIRO | Zone-by-zone environmental adjustment record | Pressure / occupancy deduction — atmospheric adjustments show a section was occupied that the crew roster says was empty | Medium |
| R2-04 | Propulsion Output Record | PROPULSION | Thrust vector and fuel consumption log | Fuel audit / math — propulsion logs don't match the official flight record; calculate the discrepancy and what it implies | Medium-Hard |
| R2-05 | Master Diagnostics Report | AUXILIARY | Partially overwritten master diagnostic output | Reconstruction — the master diagnostic ran during the gap; reconstruct the redacted entries from surviving checksums and system state data | Hard |
| R2-META | Systems Reconstruction | — | Integrated systems report | Round-meta → answer drawn from world power/systems vocabulary | — |

### Round 3 — CREW RECORD

| ID | Title | Department | Document type | Puzzle type | Difficulty |
|----|-------|-----------|---------------|-------------|------------|
| R3-01 | Medical Intake Log | MEDICAL | Sickbay intake and triage record | Diagnosis deduction — patient symptoms during the gap window don't match the stated cause; cross-reference scanner data and timeline to find the discrepancy | Medium |
| R3-02 | Transporter Usage Log | TRANSIT | Pattern buffer log with signal data | State machine / pattern buffer — transport occurred during the gap; the log was scrubbed but transport leaves a phase residue; decode the destination coordinates | Medium |
| R3-03 | Personnel Badge Record | SEC | Badge swipe log with clearance timestamps | Access log reconstruction — reconstruct who was where during the gap from incomplete badge records and known crew positions | Medium |
| R3-04 | Damage Assessment Report | DAMAGE CONTROL | Post-incident damage report | Repair sequencing / contradiction — damage logged after the gap doesn't match the ship's current state; find what was repaired that wasn't officially broken | Medium-Hard |
| R3-05 | Replicator Request Log | REPLICATOR | Itemized replicator usage record with security flags | Security audit — replicator requests during the gap reveal what was being prepared; match requests against the restricted items list to identify the anomaly | Hard |
| R3-META | Crew Reconstruction | — | Personnel incident summary | Round-meta → answer drawn from world rank/personnel vocabulary | — |

## Difficulty Curve

Round 1 (TELEMETRY): Entry → Medium → Hard
Round 2 (SYSTEMS LOG): Medium → Medium → Hard (technical, requires ship system knowledge)
Round 3 (CREW RECORD): Medium → Medium → Hard (most deductive, most ambiguous evidence)

The difficulty increases round-to-round by design. Round 1 is observation. Round 2 is inference. Round 3 is reconstruction against active resistance — someone tried to hide this.

## Estimated Solve Times

| Element | Time estimate |
|---------|--------------|
| Per feeder (average) | 20-35 min |
| Per round-meta | 15-25 min |
| Final meta | 20-30 min |
| **Total (full team)** | **~5-6 hours** |

## Meta Answer Architecture

Each round-meta produces one answer word or phrase. All answers drawn from the SPACEGAMIVERSE world vocabulary — the world already contains the right words. Exact words resolved in Stage 5.

| Meta | Answers | Word type |
|------|---------|-----------|
| R1-META | WHAT was encountered | Comm/sensor/anomaly vocabulary |
| R2-META | HOW the ship responded | System state / console command vocabulary |
| R3-META | WHO ordered the cover-up | Rank / career tier vocabulary |
| FINAL META | THE INCIDENT | A phrase or word from world canon that lands as a revelation |

## Numbering — In-Universe Display

| ID | In-universe label |
|----|-----------------|
| R1-01 through R1-05 | TELEMETRY LOG — ITEM 01 through 05 |
| R1-META | TELEMETRY RECONSTRUCTION |
| R2-01 through R2-05 | SYSTEMS LOG — ITEM 01 through 05 |
| R2-META | SYSTEMS RECONSTRUCTION |
| R3-01 through R3-05 | CREW RECORD — ITEM 01 through 05 |
| R3-META | CREW RECONSTRUCTION |
| FINAL | INCIDENT REPORT — FINAL |

## Canon Lock Required Before Authoring

World files must be locked (no further changes) before Stage 6. The following need to exist and be stable:

| What | Where | Status |
|------|-------|--------|
| Ship layout (decks, rooms, console locations) | `world/systems/data-tables.md` — Table 2 | PRESENT |
| ODN topology (which computer feeds which console) | `world/systems/data-tables.md` — Table 1 | PRESENT |
| Replicator security flag categories | `world/systems/data-tables.md` — Table 3 | PRESENT |
| Power dependency tree (exact systems list) | `world/WORLD.md` | PRESENT |
| Comm band physics and interference taxonomy | `world/WORLD.md` | PRESENT |
| Career rank ladder and clearance levels | `world/WORLD.md` | PRESENT |

All world data required for authoring is now present. World is ready to lock.
