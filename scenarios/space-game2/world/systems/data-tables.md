# World Data Tables — Stage 2 Supplement

**Created:** 2026-02-27
**Purpose:** Three data tables requested by puzzle authoring team before Stage 2 begins
**Companion to:** `world/WORLD.md`, `world/systems/departments.md`

---

## Table 1: Ship's ODN Topology (Optical Data Network)

The ODN is the ship's nervous system — fiber-optic data pathways connecting every computer, console, and sensor on the ship. All system data flows through the ODN. If the ODN is damaged, systems don't lose power — they lose *information*.

### Network Architecture

```
                    +--------------+
   SENSOR ARRAYS    |              |    OUTPUT SYSTEMS
   ===============> |   PRIMARY    | ===============>
                    |   COMPUTER   |
  +--- Nav Sensors  |   (500 CU)   | --> Navigation Computer --> Helm
  +--- Tac Sensors  |              | --> Tactical Computer --> Weapons
  +--- Sci Sensors  |              | --> Science Computer --> Labs / Medical DB
  +--- Env Sensors  |              | --> Environmental Computer --> Life Support
  +--- Sec Sensors  |              | --> Engine Computer --> Propulsion
                    |              | --> Comms Computer --> Translation --> Comms I/O
  COMMS INPUT       |              | --> Holographic Computer --> Holodeck
  ===============>  |              | --> Diagnostic Computer --> Drone I/O
  +--- Sublight     |              | --> Turbolift Computer
  +--- Subspace     |              |
  +--- Internal     |   Linkage    |
                    |    Node      |
                    |     +--> Replicator Computer
                    |     +--> Transporter Computer
                    |              |
                    +------+-------+
                           |
                    +------+-------+
                    |  SECONDARY   |
                    |  COMPUTER    | --> Security Computer --> Access Control
                    |  (Backup)    | --> Sensor Computer --> Pre-Filters
                    +--------------+
```

### The 15 Computer Nodes

Every major system has its own dedicated computer node on the ODN. These aren't just software partitions — they're physical hardware with their own processing, connected by optical fiber.

| Node | Department | What It Does | Feeds Into |
|------|-----------|-------------|------------|
| **Primary Computer** | CMD | Central processing, coordinates everything | All nodes below |
| **Secondary Computer** | CMD | Backup + security processing | Security, Sensors |
| **Navigation Computer** | NAV | Course plotting, hazard avoidance | Helm console, autopilot |
| **Tactical Computer** | TAC | Firing solutions, threat assessment | Weapons, targeting |
| **Comms Computer** | OPS | Signal processing, encryption, translation | All comm bands, internal comms |
| **Environmental Computer** | OPS | Atmosphere, pressure, temperature control | Life support, zone management |
| **Security Computer** | SEC | Access control, internal sensors, brig | Security consoles, force fields |
| **Engine Computer** | ENG | Propulsion management, fuel tracking | Thrusters, impulse, warp |
| **Science Computer** | SCI | Research data, anomaly analysis | Labs, science sensors |
| **Replicator Computer** | OPS | Pattern storage, matter tracking | All replicator units |
| **Transporter Computer** | OPS | Pattern buffers, targeting, biofilter | Transporter pads |
| **Holographic Computer** | OPS | Holo-environment, projections | Holodecks, EMH |
| **Turbolift Computer** | ENG | Route optimization, traffic management | Turbolift network |
| **Diagnostic Computer** | ENG | System health, maintenance scheduling | Diagnostic labs, drones |
| **Sensor Computer** | NAV | Raw data pre-filtering, calibration | All sensor arrays |

### Data Flow Pattern

Every piece of data follows the same path:

```
  RAW SOURCE --> PRE-FILTER --> ODN --> SPECIALTY COMPUTER --> OUTPUT
```

1. **Raw source** — sensor array, comm antenna, internal monitor
2. **Pre-filter** — removes noise, formats data (each sensor type has its own pre-filter)
3. **ODN** — optical fiber carries filtered data to the correct computer node
4. **Specialty computer** — processes data for its domain (nav computer plots courses, tactical computer calculates firing solutions)
5. **Output** — sends results to the console, display, or automated system that needs it

### ODN Components

| Component | What It Is | Power | Function |
|-----------|-----------|-------|----------|
| **ODN Relay** | Passive signal repeater | 0.5 MW | Passes data through without processing. Long runs need relays every ~50m. |
| **ODN Junction** | Active switching node | 1.0 MW | Routes data between multiple pathways. Can reroute around damage. |
| **Bioneural Gelpack** | Organic processor | 2.0 MW | High-speed pattern recognition. Faster than isolinear arrays. Can get "sick" (biological vulnerability). |
| **Isolinear Array** | Standard storage/compute | 1.0 MW | Reliable, repairable, no biological vulnerabilities. Slower than gelpacks. |
| **Interface Terminal** | Console connection | 0.2 MW | Where ODN meets user. Every console has one. |
| **Diagnostic Port** | Maintenance access | 0.1 MW | Technician access points for testing and repair. |

### ODN Scaling by Ship Class

| Ship Class | ODN Nodes | Specialty Computers | Bioneural? | Notes |
|-----------|-----------|-------------------|------------|-------|
| **Shuttlecraft** | 2 | None | No | One trunk line, minimal routing |
| **Corvette** | 4 | None | No | Simple star topology |
| **Frigate** | 6 | NAV | No | First specialty computer |
| **Destroyer** | 8 | NAV, TAC | No | Two dedicated combat nodes |
| **Cruiser** | 12 | NAV, TAC, ENG, MED, SCI, TRANS | Yes | Full specialty coverage |
| **Battlecruiser** | 16 | All + redundant | Yes | Redundant pathways |
| **Battleship** | 20 | All + HOLO | Yes | Triple-redundant trunk |
| **Carrier** | 28 | All + HOLO | Yes | Separate bay network |
| **Dreadnought** | 36 | All + HOLO | Yes | Isolated combat network |

### Security on the ODN

Two specialty computers are **firewalled** — isolated from the main network with controlled access:
- **Science Computer** — research data, experiments (firewalled to prevent contamination of main systems)
- **Holographic Computer** — holo-environment control (firewalled because holodeck malfunctions shouldn't propagate)

The **Replicator** and **Transporter** computers connect through a **Linkage Node** — an intermediary that adds an extra security/validation layer. These systems convert energy to matter and vice versa; the linkage node is a safety gate.

### Puzzle Hooks

- **ODN damage:** A relay fails — which consoles lose data? Can you reroute through backup junctions?
- **Data tracing:** Trace a suspicious data packet from source to destination through the ODN. Where was it intercepted? Where was it modified?
- **Network bottleneck:** Too many systems drawing compute. Which node is overloaded? What can be deprioritized?
- **Cyber threat:** A worm spreading through the ODN. Which junctions can you isolate to contain it?
- **Gelpack failure:** Bioneural gelpack is "sick" (biological infection). Can't just swap it — need to identify the pathogen and treat or replace.
- **Encryption layer:** Someone broke the encryption on a comm channel. Trace which ODN segment was compromised.
- **Pre-filter bypass:** Raw unfiltered data flooding a computer node. Find and repair the broken pre-filter.

---

## Table 2: Room / Console Layout

### Console Types and Default Monitors

Every console has 3 monitor positions. The default layout depends on what type of console it is:

| Console Type | [0] Main Work | [1] Support | [2] Reference |
|-------------|---------------|-------------|---------------|
| COMMAND_CHAIR | Ship Status | Alert Board | Mission Brief |
| HELM | Navigation | Propulsion | Nav Sensors |
| TACTICAL | Weapons Control | Shield Status | Tactical Sensors |
| OPS | Power Overview | System Status | Damage Report |
| ENGINEERING | Reactor Ops | EPS Grid | Coolant / Battery |
| COMMS | Channel Mgmt | Signal Processing | Subspace Status |
| SCIENCE | Active Experiment | Sensor Data | Lab Status |
| MEDICAL | Patient Status | Lab Results | Pharmacy |
| SECURITY | Patrol Monitor | Internal Sensors | Brig Status |
| ENVIRONMENTAL | Zone Control | Atmosphere | Life Support |
| TRANSPORTER | Pad Operations | Pattern Buffer | Transceiver Array |
| AUXILIARY | General Access | Configurable | Configurable |

### Room-to-Console Bindings (Cruiser Scale)

What consoles live in which rooms, and what software suite they run by default:

| Room | Deck | Consoles | Default Suite |
|------|------|----------|---------------|
| **Bridge — Center** | 1 | COMMAND_CHAIR (x1) | Command Suite — ship status, alerts, crew |
| **Bridge — Helm** | 1 | HELM (x1) | Navigation Suite — sensors, deflector, course |
| **Bridge — Tactical** | 1 | TACTICAL (x1) | Tactical Suite — sensors, weapons, shields |
| **Bridge — OPS** | 1 | OPS (x1) | OPS Suite — comms, enviro, transit overview |
| **Bridge — Science** | 1 | SCIENCE (x1) | Science Suite — science ops, sensors |
| **Ready Room** | 1 | AUXILIARY (x1) | Command Suite (private) |
| **Briefing Room** | 1 | AUXILIARY (x4) | Configurable per meeting |
| **Main Engineering — Power** | 5 | ENGINEERING (x4) | Reactor, EPS, batteries, distribution |
| **Main Engineering — Propulsion** | 5 | ENGINEERING (x4) | Thrusters, impulse, warp core, nacelles |
| **Main Engineering — Computer** | 5 | ENGINEERING (x3) | Computer core, ODN, diagnostics |
| **Main Engineering — Damage Control** | 5 | ENGINEERING (x3) | DC coordination, repair teams, maintenance |
| **EPS Control** | 5 | ENGINEERING (x2) | Power distribution grid |
| **Environmental Control** | 5 | ENVIRONMENTAL (x2) | Life support, atmosphere, gravity |
| **Deflector Control** | 5 | ENGINEERING (x2) | Nav deflector operations |
| **Transporter Room 1** | 2 | TRANSPORTER (x2) | Pad A ops, Pad B ops |
| **Transporter Room 2** | 2 | TRANSPORTER (x2) | Pad C ops, Pad D ops |
| **Emergency Transport Bay** | 5 | TRANSPORTER (x2) | Emergency pads (20-person) |
| **Sickbay — Main** | 3 | MEDICAL (x3) | Diagnosis, treatment, stasis |
| **Science Lab** | 4 | SCIENCE (x4) | Research, analysis, experiments |
| **Security Office** | 4 | SECURITY (x3) | Internal sensors, access control, brig |
| **Brig** | 4 | SECURITY (x1) | Containment fields, prisoner vitals |
| **Armory** | 4 | SECURITY (x1) | Weapons locker, equipment checkout |
| **Cargo Bay** | 4 | OPS (x1) | Cargo manifest, environmental monitoring |
| **Personal Quarters** | 3 | AUXILIARY (x1) | Personal Suite — messages, schedule, logs |
| **Senior Quarters** | 3 | AUXILIARY (x1) | Personal Suite (expanded access) |
| **Mess Hall** | 3 | AUXILIARY (x2) | Public Terminal — replicator, directory, schedule |
| **Corridor Terminals** | All | AUXILIARY (xvaries) | Public Terminal — limited access |
| **Turbolift** | All | — | Turbolift Interface — destination selection only |

### Secure Console Restrictions

Some systems can ONLY be operated from specific physical locations. You can monitor them from anywhere, but operational control requires being at the right console:

| System | Operational Control ONLY From | Monitor-Only Everywhere Else |
|--------|-------------------------------|------------------------------|
| **Weapons fire control** | Bridge Tactical, Weapons Bay | Yes |
| **Reactor core operations** | Main Engineering, Reactor Room | Bridge can monitor, not operate |
| **Self-destruct** | Bridge Command, Ready Room | Not visible elsewhere |
| **Brig containment** | Security Office, Brig Station | Not accessible elsewhere |
| **Navigation override** | Bridge Helm, Aux Bridge | Not accessible elsewhere |

### Console Access Depth by Location

Not all consoles can reach all systems. Physical location determines how deep you can drill:

| Location Tier | Home Dept Access | Ship-Wide Access | Example |
|--------------|-----------------|-----------------|---------|
| **Bridge** | Full | Full or -1 level | Command chair sees everything |
| **Main Engineering** | Full | Full or -1 level | Engineering sees all ENG, monitors others |
| **Department Station** | Full | -2 levels | Sickbay sees all MED, limited ENG view |
| **Auxiliary Bridge** | Full | -1 level | Backup command, slightly less depth |
| **General Area** | -2 levels | -3 levels | Mess hall terminal — surface info only |
| **Personal Quarters** | -3 levels | -4 levels | Personal stuff, minimal ship data |
| **Turbolift** | -4 levels | -5 levels | Destination selection, that's it |

### Shuttlepod Console Layout (1 room)

At shuttlepod scale, there's one console with access to everything:

| Position | Equipment | What's There |
|----------|----------|-------------|
| **Pilot Console** (front) | Main console, 3 monitors | All ship systems — one person, all departments |
| **Overhead Panel** (above pilot) | Circuit breakers, manual switches | Emergency cutoffs, manual overrides |
| **Starboard Wall** | Replicator panel, storage | Food/beverage replication, supply storage |
| **Port Wall** | Systems access panels | EPS conduit access, circuit breaker panel |
| **Rear Wall** | Transporter alcove, rear hatch | Phone-booth transporter, entry/exit |
| **Under-Deck** | Reactor access, fuel ports | Reactor maintenance, fuel cartridge swap |
| **Exterior Hull** | Sensor arrays, nacelle access, antenna | External maintenance and repair (EVA) |

### Puzzle Hooks

- **Location-based answers:** "The fault is on Deck 4" — player navigates there. "Check the port wall breaker panel" — player examines it.
- **Console access puzzles:** You need to fire weapons but you're in Engineering. How do you get to Bridge Tactical? Or can you convince someone already there?
- **Secure system bypass:** Can you hack reactor control from a corridor terminal? (Answer: no, but you could reroute ODN data to fake a reading...)
- **Console density:** Main Engineering has 14+ consoles across 4 sections. Which section has the console you need?
- **Cross-reference:** The answer to a puzzle is a room ID. Room IDs follow format `{deck}-{section}-{room}` (e.g., `5-02-03` = Deck 5, Section 02, Room 03).

---

## Table 3: Replicator Security Flag Categories

### How Replication Works

Replicators convert stored energy into matter using pattern templates from a database. The Replicator Computer stores thousands of patterns (food recipes, tool designs, component blueprints). To replicate something, the system needs:

1. **A pattern** — the molecular blueprint (stored in the pattern database)
2. **Raw matter** — energy/matter reserves to convert
3. **Authorization** — security clearance for the requested item category

### The Six Container Categories

Every item in the replicator pattern database is tagged with a security container category. The category determines what safety systems engage during replication and what authorization is required:

| Category | Flag | What It Covers | Safety System | Auth Required |
|----------|------|---------------|--------------|--------------|
| **Standard** | OPEN | Food, beverages, clothing, personal items, basic tools, furniture, dishes | None | None — any crew member, any replicator |
| **Bio-Container** | BIO | Biological samples, organic compounds, growth media, tissue cultures | Biofilter scan pre-replication | Science or Medical clearance |
| **Hazardous** | HAZ | Corrosive chemicals, radioactive materials, volatile compounds, plasma canisters | Pattern buffer containment field | Engineering clearance + logged |
| **Poisonous** | POISON | Toxins, controlled pharmaceuticals, anesthetics, lethal compounds | Biofilter assembly active screening | Medical clearance only — CMO authorization for lethal quantities |
| **Security** | SEC | Weapons, explosives, restraints, surveillance equipment, encryption devices | Security lockout — dual authorization | Security Chief + CO authorization |
| **Unknown** | UNK | Unclassified patterns, alien artifacts, experimental compounds, field-captured patterns | Full containment — waveguide instruction verification | Science Officer + department head review |

### Authorization Tiers (Who Can Replicate What)

| Tier | Who | What They Can Replicate | Where |
|------|-----|------------------------|-------|
| **Tier 0 — Open** | Any crew member | Standard items (food, clothing, basic tools) | Any replicator (mess hall, quarters, corridor) |
| **Tier 1 — Department** | Trained department personnel | Bio-containers (science), Hazardous (engineering) | Department replicators only (lab, workshop) |
| **Tier 2 — Specialist** | Certified specialists (Level 5-6 training) | Poisonous materials, medical compounds | Medical replicator (Sickbay), secured lab replicator |
| **Tier 3 — Restricted** | Security Chief + CO dual auth | Weapons, explosives, surveillance gear | Armory replicator or secure engineering bay |
| **Tier 4 — Unknown** | Science Officer + department head | Unclassified/alien patterns, experimental items | Isolated containment replicator only |

### Replicator Skill Progression (Crew Training)

Crew members advance through certification levels to access higher-tier replication:

| Level | Certification | Unlocks |
|-------|-------------|---------|
| 1 | Inorganic Replication (basic) | Standard tool and component patterns |
| 2 | Inorganic Recycling | De-assembly operations (matter reclamation) |
| 3 | Inorganic Buffer Operations | Pattern buffer management, custom modifications |
| 3 | Organic Recycling | Food waste reclamation |
| 4 | Inorganic Replication (advanced) | Complex engineering components |
| 4 | Organic Replication (basic) | Food preparation, basic biological materials |
| 5 | Organic Buffer Operations (advanced) | Complex organic patterns, tissue samples |
| 5-6 | **Organic Replication — Security** | Security-flagged organic materials |
| 6a | **Bio-Medical Replication** | Pharmaceutical compounds, medical devices |
| 6b | **Adv. Engineering Replication** | Specialized engineering components |
| 6c | **Weapons Replication — Personal** | Personal weapons (requires Level 7 cert) |
| 7 | Basic Transporter Operations | Crossover to transporter systems |

### Replicator Types by Location

Different replicator units have different capabilities and security levels:

| Replicator Type | Location | Categories Available | Access |
|----------------|----------|---------------------|--------|
| **Food/Beverage** | Mess hall, quarters, corridors | Standard only | Public — any crew |
| **Engineering Workshop** | Main Engineering bay | Standard + Hazardous | Sealed — engineering dept |
| **Science Lab** | Science laboratory | Standard + Bio + Unknown | Sealed — science dept |
| **Medical** | Sickbay | Standard + Bio + Poisonous | Public area, medical auth for Tier 2+ |
| **Armory** | Armory (Security) | Standard + Security | Restricted — Security + CO dual auth |
| **De-Assembly Bay** | Auxiliary replicator bay | All categories (reverse process) | Restricted — highest security |
| **Shuttlepod** | Starboard wall panel | Standard only (limited pattern library) | Pilot authorization |

### What CAN'T Be Replicated (Hard Limits)

Some things are beyond replicator capability regardless of authorization:

| Item | Why Not |
|------|---------|
| **Dilithium crystals** | Molecular structure too complex for replication — must be mined |
| **Latinum** | Replication-resistant by nature (why it's used as currency) |
| **Living organisms** | Replicators create dead matter — transporter handles living patterns |
| **Antimatter** | Produced by dedicated antimatter generators, not replicators |
| **Other replicators** | Self-replication is hardcoded impossible (safety interlock) |
| **Classified weapon designs** | Patterns not in database — can't replicate what you don't have |

### Puzzle Hooks

- **Security audit:** Someone replicated a restricted item. Trace the authorization override — who approved it? Was it forged?
- **Pattern database:** An unknown pattern appeared in the replicator computer. Where did it come from? ODN intrusion? Transferred from a station?
- **Container mismatch:** An item flagged as Standard is actually Hazardous. The biofilter didn't engage. Who changed the flag?
- **Skill verification:** A crewmember claims they can replicate medical compounds. Check their certification level — do they actually have Level 6a?
- **Dual-auth puzzle:** To replicate a weapon, you need Security Chief AND CO. One is unavailable. Is there a legitimate override path?
- **De-assembly forensics:** Something was de-assembled (recycled). What was it? The matter signature might tell you what was destroyed.
- **Shuttlepod limitation:** You need a component that's not in the shuttlepod's limited pattern library. Can you download it from the cruiser via subspace? Do you have the authorization?

---

## Table 4: EPS Grid Power Consumption Reference

Canonical MW draw values for EPS-grid systems. Electrical grid systems draw in kW (not listed here).

**Electrical Grid (kW) — runs: computer, comms, sensors, lighting, environmental sensors, weapons fire control**

**EPS Grid (MW) — runs: impulse, shields, environmental fields, transporter, replicator, holodeck**

| System | Grid | Rated Draw | Notes |
|--------|------|-----------|-------|
| Impulse Drive — Acceleration | EPS | 180 MW | Peak draw during thrust-up |
| Impulse Drive — Cruise | EPS | 120 MW | Steady state at cruising speed |
| Impulse Drive — Deceleration | EPS | 60 MW | Thrust reversal / braking |
| Shields (full power) | EPS | 95 MW | Combat or standby shield |
| Environmental Fields (all zones) | EPS | 25 MW | Gravity + dampening + SIF combined |
| Transporter | EPS | 40 MW | Per active transport cycle |
| Replicator | EPS | 35 MW | Standard replication cycle |
| Holodeck | EPS | 55 MW | Active holographic simulation |

**Added:** 2026-02-27 — canonized from R2-01 puzzle data during integration check.
