# SPACEGAMIVERSE — World Overview

**Genre/Aesthetic:** Trek meets Firefly. Real switches, real engine rooms, lived-in ships with gritty utility. Not squeaky-clean utopia — more like a functional military vessel where the bulkheads have scuff marks and the replicator sometimes gives you lukewarm coffee. Dystopian undertones, modern sensibility. The tech is advanced but the experience is physical — you flip switches, you hear fans spin up, you feel the reactor hum through the deck plates.

**Hook:** You are an officer (or a team of officers) aboard a starship, solving whatever the universe throws at you — system failures, scrambled signals, tactical crises, power cascades, and diagnostic puzzles — all from the consoles and stations of a working ship.

**Core systems:**
1. **Ship Systems** — 4 divisions, 16 departments, ~51 systems, full hierarchy from Division down to Module
2. **Power Architecture** — Three power tiers (Battery → Fusion → M/AM), dependency enforcement, EPS vs Electrical grids
3. **Console & Station Model** — Status board with 6 system states, drill-down sub screens, AUTO vs MANUAL operation
4. **Communications** — 8 comm bands (4 short-range, 4 subspace), signal processing, interference taxonomy
5. **Career Progression** — Technician → Rotation → Diagnostician → Senior Officer, with three meta-controls (AUTO/DIAGNOSTIC/CALIBRATE)
6. **Ship Scaling** — Capsule → Fighter → Shuttlepod → Shuttle → Corvette → Cruiser — same architecture at every scale

**Meta target:** Solo play: solve department puzzle sets (comms, tactical, transporter, engineering, etc.) to prove competency in each station. Complete all sets to earn command certification — you can captain your own shuttle. Team play: coordinate across departments to solve ship-wide crises that no single station can handle alone.

---

## Puzzle Type Suggestions

*The universe naturally generates these puzzle types. Organized by department so puzzles feel like they come FROM the ship, not bolted on.*

### By Department

| Department | Puzzle Types | Why It Fits |
|-----------|-------------|-------------|
| **COMMS** | Signal demodulation (pattern matching), frequency ID (deduction), interference classification (taxonomy), encrypted messages (cipher), band selection (category matching) | 8 comm bands with different physics, 6-tier interference taxonomy, each band generates different puzzle types |
| **TACTICAL** | Threat assessment (logic grid), weapons auth chain (sequencing), shield frequency matching (pattern), targeting solution (spatial), sensor contact classification (deduction) | Multiple weapon systems, arming/authorization procedures, shield harmonics, sensor contacts with decoy/real distinction |
| **NAVIGATION** | Course plotting (spatial/math), star chart reading (coordinates), hazard avoidance (pathfinding), time-distance calculation (word problems), fuel budgeting (optimization) | Flight computer with course/guidance/trajectory, fuel management (fusion + M/AM), destination planning |
| **ENGINEERING** | Power load balancing (optimization), fault diagnosis (deduction tree), EPS routing (flow/network), calibration convergence (parameter tuning), reactor sequencing (ordering) | Three power tiers with dependencies, system state transitions, PCU distribution |
| **TRANSPORTER** | Pattern buffer diagnosis (state machine), Heisenberg compensation (precision), biofilter ID (categorization), signal lock puzzles (targeting), matter stream integrity (pattern matching) | Matter-energy conversion, pattern storage, organic vs inorganic transport modes |
| **MEDICAL** | Diagnosis from symptoms (logic grid), triage prioritization (ordering), quarantine protocol (categorization), treatment selection (matching), medical scanner interpretation (data reading) | Diagnosis, surgery, stasis, quarantine — each a different puzzle type |
| **ENVIRONMENTAL** | Atmosphere mix balancing (optimization), pressure breach localization (deduction), thermal routing (flow), gravity setpoint calculation (math), life support triage (prioritization) | Chemistry (atmosphere/pressure/temp) + Physics (gravity/dampening/SIF), one-zone vs multi-zone at different ship scales |
| **SECURITY** | Access code sequencing (cipher), intrusion detection (pattern), clearance level logic (deduction grid), physical/virtual breach classification (taxonomy), brig management (scheduling) | Three-gate security model (console + location + user clearance), cyber security, physical access control |

### Cross-Department (Team Play)

| Scenario | Departments Involved | Puzzle Type |
|----------|---------------------|-------------|
| Ship-wide power failure | ENG + all departments | Sequencing — what order to bring systems back online? |
| Unknown contact | TAC + COMMS + NAV | Deduction — classify contact using sensors, comms intercepts, trajectory analysis |
| Medical emergency during warp | MED + ENG + NAV | Optimization — drop from warp safely while stabilizing patient |
| Boarding action | SEC + TAC + COMMS | Multi-step — lock down sections, arm crew, jam enemy comms |
| Damaged transporter | TRANSIT + ENG + MED | Diagnosis — find the fault, reroute power, verify biofilter before transport |

---

## Tone

This is a ship where the technology is real and the people are human. When you cold-start a fusion reactor, you hear the containment field hum before you see the output gauge climb. The ship comes alive around you — fans, gravity settling, lights brightening. It's physical.

The people are professional but not stiff. Orders come through channels and you respond with proper acknowledgment — "Aye, Commander" — but in your own head you're thinking "great, another diagnostic on the plasma conduit at 0300." Internal monologue is casual, wry, sometimes tired. You care about your ship and your crew, but you also care about getting through your shift.

Single-player tone is casual and personal — you're working through your station, solving problems, building competence. When orders come in from the CO, the register shifts slightly more formal — you're receiving a tasking, not chatting with a friend. But even then, it's not rigid. This isn't a parade ground. It's a working ship.

---

## Physical Spaces — Location-Based Puzzles

Puzzle answers can resolve to PHYSICAL LOCATIONS. "The answer is Deck 3, Compartment 7" — and the player goes there for the next puzzle. This scales differently by ship class:

### Cruiser / Frigate Scale (5 decks, ~28 rooms)

Puzzle targets are **rooms and decks**. The ship has named, navigable locations connected by corridors and turbolifts.

**5 Decks:**

| Deck | Name | Division Color | Key Rooms |
|------|------|-------|-----------|
| 1 | Command | Red | Bridge, Ready Room, Briefing Room, Observation |
| 2 | Operations | Yellow | Transporter, Comms, Nav Control, Tactical Control |
| 3 | Crew | Green | Quarters, Senior Quarters, Mess Hall, Recreation, Sickbay |
| 4 | Support | Blue | Science Lab, Cargo Bay, Armory, Brig, Security |
| 5 | Engineering | Orange | Main Engineering, EPS Control, Env Control, Deflector Control, Computer Core |

**Room ID format:** `{deck}-{section}-{room}` (e.g., `1-01-01` = Deck 1, Section 01, Room 01)

**Each room can contain:**
- Consoles (station screens for specific departments)
- Equipment panels (things you can open, inspect, repair)
- Interactive objects (logs, tools, cargo, evidence)
- NPCs (crew members with dialogue, orders, information)

**83 room types defined** across all ship scales — from 3-deck escorts to 15+ deck flagships. Each room type belongs to a specific department and contains specific equipment.

### Shuttlepod / Shuttle Scale (1-2 rooms)

No decks, no corridors. Puzzle targets are **panels, alcoves, and equipment locations** within one or two compartments.

**Shuttlepod (1 room):**
- Pilot console (front)
- Overhead panel (above pilot)
- Starboard wall — replicator panel, storage
- Port wall — systems access panels, circuit breakers
- Rear wall — transporter alcove, rear hatch
- Under-deck — reactor access, fuel cartridge ports, EPS conduit
- Exterior hull — sensor arrays, nacelle access, antenna

**Shuttle (2 rooms):**
- All of the above, PLUS:
- Rear compartment with 2-4 additional console positions
- More wall panels, more equipment access points
- Possible cargo area

**At this scale, puzzle answers might be:** "Check the port wall circuit breaker panel" or "The frequency is etched on the fuel cartridge housing" — physical things you can point at within the cabin.

---

## Canon Rules

### Ship Architecture
- Every ship from shuttlepod to cruiser runs the SAME systems architecture — 4 divisions (Command RED, Operations YELLOW, Engineering ORANGE, Science BLUE), 16 departments. What changes is SCALE, not structure.
- Ship scaling: Capsule → Fighter → Shuttlepod → Shuttle → Corvette → Cruiser. Each tier adds complexity (more zones, more consoles, more crew) but the core systems are identical.
- Hardware hierarchy: Division → Department → System → Subsystem → Equipment → Component → Module. Software mirrors this: Suite → System Screen → Subsystem Screen → Equipment Screen.
- The Ship's Computer runs everything. At shuttlepod scale, it's one box. At cruiser scale, it's a primary core + secondary core + 13 dedicated computers. Same OS, different hardware.

### The 16 Departments

**RED — Command:**
- CMD (Command) — ship command, mission command, crew management
- NAV (Navigation) — sensors, nav deflector, flight computer, maneuvers
- TAC (Tactical) — tactical sensors, tactical deflector, weapons, electronic warfare
- SEC (Security) — security sensors, access control (virtual + physical), brig, armory

**YELLOW — Operations:**
- COMMS (Communications) — 4 short-range + 4 subspace bands, internal comms, encryption, universal translator
- ENVIRO (Environmental) — atmosphere/pressure, thermal/circulation, field control (gravity, dampening, SIF)
- TRANSIT (On/Off Ship Ops) — airlock, escape pods, EVA, transporter, shuttle bay, cargo bay
- REPLICATOR — organic, inorganic, large/cargo replication, de-replication/waste
- HOLODECK — holo-environmental, holo-projection, holo-replication

**ORANGE — Engineering:**
- POWER — fusion reactor, M/AM reactor, batteries, EPS distribution, electrical distribution
- COMPUTER — primary/secondary cores, 13 dedicated computers, ODN data network
- DAMAGE CONTROL — damage control, maintenance
- PROPULSION — thrusters (RCS), impulse drive, warp drive (core + nacelles)
- AUXILIARY — turbolift, tractor beam, probes, towing, coolant, master diagnostics

**BLUE — Science:**
- MEDICAL — diagnosis, surgery, stasis, quarantine, medical computer/sensors/scanners, research, production, morgue
- SCIENCE — science operations, research, science sensors

### Power
- Three power tiers: Battery (electrical only, limited), Fusion Reactor (electrical + EPS plasma), M/AM Reactor (warp-level power + bridge to ship power).
- Electrical Grid (kW scale) runs the brains — computer, comms, sensors, lighting, environmental, weapons fire control.
- EPS Grid (MW scale) runs the muscle — impulse, shields, env fields, transporter, replicator.
- Nav Deflector and Warp Drive require M/AM power. Everything else runs on fusion.
- Power Conditioning Unit (PCU) splits fusion output into Electrical Grid + EPS Grid.

**Dependency tree (determines what can be online):**
```
  BATTERY → Electrical Grid
  │   Runs: computer, comms, sensors, weapons, environmental,
  │         flight computer, lighting, thrusters, fusion reactor startup
  │
  FUSION REACTOR → PCU
  │   Unlocks: EPS Grid → impulse, shields, env fields,
  │            transporter, replicator, M/AM reactor startup
  │
  M/AM REACTOR
      Unlocks: Warp EPS → warp drive, nav deflector
      Also bridges to PCU for supplemental ship power
```

### System States
- Six states visible on status board:
  - **◉ ONLINE** — running (ACTIVE = doing work, IDLE = monitoring, zero output)
  - **◇ READY** — power available, can be activated
  - **◌ STANDBY** — was running, put to sleep, instant wake
  - **⊘ LOCKED** — power available but restricted, requires deliberate action to unlock (e.g., weapons locked while docked)
  - **○ OFFLINE** — no power, upstream dependency not met
  - **● FAULT** — hardware failure, needs repair
- READY means "you can turn this on." OFFLINE means "you can't yet." LOCKED means "you could but you need to take steps first."

### Environmental
- ENVIRO has two halves: **Chemistry** (atmosphere, pressure, humidity, temperature — keeps you alive) and **Physics** (gravity, inertial dampening, SIF — keeps you functional).
- Gravity plating is setpoint-based. Set 1.0g, it measures ambient field, outputs the delta. Seamless transitions between docked/free-flight/landed.
- Gravity + Inertial Dampening work together as one physics system. Gravity sets "down." Dampening absorbs external forces. Together they keep physics normal inside the cabin at rest or during combat maneuvers.

### Console Model
- Three universal meta-controls on EVERY station: **AUTO** (system handles it), **DIAGNOSTIC** (investigate), **CALIBRATE** (tune/adjust). These frame whatever activity is on screen.
- Career progression through these controls: Technician (AUTO only) → Rotation (new stations) → Diagnostician (DIAGNOSTIC unlocked) → Senior Officer (CALIBRATE unlocked, triage and command).
- Console security: three gates — (1) what the console allows, (2) where on the ship it is, (3) who's logged in. Clearance determines depth and breadth of access.

### Communications
- 8 comm bands across two categories:
  - **Short-range (4):** Low-Wave EM (UHF/VHF), Standard Radio, Optical LaserComm, Tightbeam Microwave
  - **Subspace (4):** Subspace Pulse, Hyperwave Channel, Neutrino Burst, Gravitic Modulation
- Each band has different physics, range, reliability, and interference characteristics. Band selection switches the puzzle type — same instrument, different thinking.
- 6-tier interference taxonomy: Decoys, Shattered Carrier, Hieratic Triplet, Harmonic Echo, Phase-Locked Pair, Absorption Shadow. Each is a different KIND of puzzle, not a harder version of the same one.
- Scanner has two modes: SCAN (ping detection, full band sweep) → FT (Fourier Transform analysis, zoomed on specific contact).

### Transporter
- At shuttlepod/shuttle scale: wall alcove, phone-booth size, one person. At cruiser scale: full transporter room with pad.
- Three modes: Organic Transport, Inorganic Transport, Large/Cargo Transport.
- Runs on EPS (plasma power). Cannot operate on battery alone.

---

## World Systems Index

| System | What it contains | Puzzle design purpose |
|--------|-----------------|----------------|
| Ship Systems Tree | 4 divisions, 16 departments, ~51 systems, ~111 subsystems | Categorization, hierarchy puzzles, "what department owns this?" |
| Power Architecture | Three tiers, dependency tree, system states | Sequencing, dependency logic, "what must be online first?" |
| Comm Bands | 8 bands with physics, ranges, interference types | Signal puzzles, band identification, interference classification |
| Interference Taxonomy | 6 interference types, each a different puzzle mechanic | Pattern matching, deduction, waveform analysis |
| Console Security | Three-gate security, clearance levels, access rules | Access logic puzzles, clearance deduction |
| Ship Scaling | What exists at each vessel size (capsule → cruiser) | Comparison puzzles, "given these systems, identify the ship class" |
| Career Progression | 4 tiers, 3 meta-controls, skill gating | Difficulty tiers, "what rank can access this?" |
| Equipment Database | 14 categories, 45+ fields per entry (serial ID, type, location, components) | Component identification, repair puzzles, part matching |
| Ship Layout (Cruiser) | 5 decks, ~28 rooms, corridors, deck assignments, room IDs | Location-based puzzles, "go to Deck 3 Room 5" |
| Room Types | 83 room types with department assignments and equipment | Room identification, "what type of room has this equipment?" |
| Console-Room Bindings | What equipment lives in each room, console types | Equipment location puzzles, "where is the EPS junction?" |
| Shuttlepod Cabin | Panel locations, alcoves, wall-mounted equipment positions | Small-craft puzzles, "check the port wall breaker panel" |
| Division Colors | RED=Command, YELLOW=Ops, ORANGE=Engineering, BLUE=Science | Visual identification, categorization, color-coded deduction |

---

## Data Design Notes

- The 8 comm bands map 1:1 to puzzle types — each band's physics drives its interference, which drives a different puzzle mechanic. This is by design ("lateral gameplay" — same instrument, different thinking).
- The power dependency tree has exactly three tiers with clean cascading: battery → fusion → M/AM. Each tier unlocks a specific set of systems. Good for ordered logic puzzles.
- System states (ONLINE/READY/STANDBY/LOCKED/OFFLINE/FAULT) form a state machine with defined transitions. Good for state-based puzzles and diagnosis.
- The 4-division color scheme (RED/YELLOW/ORANGE/BLUE) maps to 16 departments. Clean categorization for sorting/matching puzzles.
- Ship scaling from shuttlepod (1 console, 1 room) to cruiser (dedicated consoles per subsystem, hundreds of compartments) means the same puzzle can appear at different complexity levels by changing the ship class context.
- Career progression (Technician → Rotation → Diagnostician → Senior Officer) provides natural difficulty tiers for puzzle sets.
- The ENVIRO chemistry/physics split and gravity setpoint model could generate calculation puzzles (compensate for planet gravity, balance atmosphere for alien species, calculate dampening load at given acceleration).
- **Puzzle answers can be physical locations.** On a cruiser: "Deck 3, Room 5" — player navigates there. On a shuttlepod: "port wall circuit breaker panel" — player examines it. Same puzzle mechanic, different spatial scale.
- The 5-deck layout has clean department separation by deck — Command on Deck 1, Engineering on Deck 5. Good for deduction puzzles ("this system fault is on the same deck as the computer core").
- Room ID format (`1-01-01`) is machine-readable and could encode puzzle information in the ID structure itself.
- 83 room types across all ship scales. Room types map to departments — if you know the room type, you know the department, and vice versa.
