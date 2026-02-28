# DEAD RECKONING — World Canon Lock

**Locked:** 2026-02-27
**Status:** LOCKED — no world changes after this point without updating this log

All puzzle facts must be verified against the files listed below. If a fact isn't in these files, it doesn't exist in this hunt's canon.

---

## Locked Files

| File | What It Contains | Locked |
|------|-----------------|--------|
| `world/WORLD.md` | Ship systems overview, 16 departments, power architecture (3 tiers, dependency tree), system states (6), comm bands (8), interference taxonomy (6 types), console model (3 meta-controls), career progression (4 tiers), ship scaling (6 classes), division colors | ✓ |
| `world/systems/departments.md` | Per-department detail for all 16 departments — key systems, dependencies, cross-links, failure modes, puzzle hooks | ✓ |
| `world/systems/data-tables.md` | ODN topology (15 nodes, data flow, components, scaling, security), room/console layout (console types, room bindings, access depth, shuttlepod), replicator security flags (6 categories, 5 auth tiers, skill progression, hard limits) | ✓ |

---

## What Is Canon

### Ship Architecture
- 4 divisions: RED (Command), YELLOW (Operations), ORANGE (Engineering), BLUE (Science)
- 16 departments as listed in WORLD.md
- Hardware hierarchy: Division → Department → System → Subsystem → Equipment → Component → Module
- Same architecture at every ship scale; only complexity changes

### Power
- Three tiers: Battery → Fusion Reactor → M/AM Reactor
- Electrical Grid (kW): computer, comms, sensors, lighting, environmental, weapons fire control
- EPS Grid (MW): impulse, shields, env fields, transporter, replicator
- Warp EPS: warp drive, nav deflector (M/AM only)
- PCU splits fusion output into Electrical + EPS

### System States (6)
◉ ONLINE, ◇ READY, ◌ STANDBY, ⊘ LOCKED, ○ OFFLINE, ● FAULT

### Comm Bands (8)
Short-range: Low-Wave EM, Standard Radio, Optical LaserComm, Tightbeam Microwave
Subspace: Subspace Pulse, Hyperwave Channel, Neutrino Burst, Gravitic Modulation

### Interference Taxonomy (6 types)
Decoys, Shattered Carrier, Hieratic Triplet, Harmonic Echo, Phase-Locked Pair, Absorption Shadow

### Console Meta-Controls (3)
AUTO, DIAGNOSTIC, CALIBRATE

### Career Progression (4 tiers)
Technician → Rotation → Diagnostician → Senior Officer

### ODN (15 nodes)
Primary Computer, Secondary Computer, Navigation, Tactical, Comms, Environmental, Security, Engine, Science, Replicator, Transporter, Holographic, Turbolift, Diagnostic, Sensor

### Replicator Security Categories (6)
OPEN (Standard), BIO (Bio-Container), HAZ (Hazardous), POISON (Poisonous), SEC (Security), UNK (Unknown)

---

## Canon Lock Log

| Date | What changed | Why |
|------|-------------|-----|
| 2026-02-27 | Initial lock | All source files imported, Stage 2 complete |

---

## Lock Protocol

If a puzzle author needs a fact that isn't in these files:
1. Check all three world files first — it may be there under a different term
2. If genuinely missing, flag it here before authoring
3. A new fact can be added ONLY if it doesn't contradict existing canon
4. All additions must be logged in the Canon Lock Log above
5. Once a puzzle is authored against a fact, that fact is immutable
