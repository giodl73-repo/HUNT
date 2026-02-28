# SPACEGAMIVERSE — Systems Detail & Cross-Links

**Companion to:** `world/WORLD.md` (overview)
**Purpose:** Deep detail on each department's systems, how they interconnect, what breaks, and where puzzles emerge from the connections between them.

---

## How to Read This Document

Each department has:
- **What it does** — the department's core function
- **Key systems** — what the player interacts with
- **Depends on** — what upstream systems it needs
- **Feeds into** — what downstream systems rely on it
- **Cross-links** — specific connections to other departments (THIS IS WHERE PUZZLES LIVE)
- **What breaks** — failure modes and their consequences
- **Puzzle hooks** — concrete scenarios for puzzle generation

---

## RED DIVISION — Command

### CMD — Command

**What it does:** Ship-wide authority. Alert status, mission orders, crew assignments, ship's log. The CO's department.

**Key systems:** Alert Management, Self-Destruct, Ship's Log/Records, Mission Planning, Away Team Coordination, Duty Shifts, Personnel Assignments, Training/Promotions

**Depends on:** COMPUTER (for records and displays), COMMS (to issue orders ship-wide), SECURITY (for authorization chains)

**Feeds into:** Everything — CMD sets alert status which affects every department. Red Alert changes power allocation, weapon readiness, shield status, crew positions.

**Cross-links:**
- CMD -> SEC: Self-destruct requires dual authorization (CO + XO). Access codes.
- CMD -> COMMS: Ship-wide announcements use internal comms. Orders to away teams use external comms.
- CMD -> ALL: Alert status changes ripple everywhere. Yellow Alert: shields to READY, weapons LOCKED->READY. Red Alert: shields ONLINE, weapons ONLINE, all non-essential systems to minimum power.
- CMD -> NAV: Mission planning feeds destinations to flight computer.
- CMD -> MEDICAL: Away team coordination includes medical clearance.

**What breaks:** Authorization chain compromise (someone has codes who shouldn't). Alert system stuck (can't change alert level). Log corruption (ship's record unreliable).

**Puzzle hooks:**
- Reconstruct a corrupted ship's log from fragments across multiple departments
- Determine the correct alert level from a set of conflicting sensor readings
- Trace an unauthorized command back through the authorization chain
- Crew scheduling conflict — who was on duty when the incident happened?

---

### NAV — Navigation

**What it does:** Getting from A to B. Sensors for knowing where you are, deflector for not dying on the way, computer for plotting the course, maneuvers for flying it.

**Key systems:** Proximity Sensors, Short-Range Nav Sensors, Long-Range Nav Sensors, Nav Deflector (debris clearing), Course Plotting, Guidance, Trajectory Calculation, Docking, Combat Maneuvers, Landing, Evasive Patterns

**Depends on:** POWER (sensors on electrical, nav deflector needs M/AM), COMPUTER (flight computer is a nav module inside ship's computer), PROPULSION (NAV plots the course, PROPULSION flies it)

**Feeds into:** PROPULSION (course data drives thrust vectors), TAC (sensor data shared for targeting), COMMS (position data for signal triangulation)

**Cross-links:**
- NAV -> PROPULSION: NAV says "go there," PROPULSION makes it happen. Course changes require recalculating fuel burn.
- NAV -> TAC: Nav sensors and tactical sensors share some hardware but different processing. Nav sees rocks and stars. Tactical sees threats. Same antenna, different question.
- NAV -> ENVIRO: Speed changes affect inertial dampening load. Course through a nebula affects sensor range.
- NAV -> COMMS: Position affects which comm bands work (line-of-sight for optical, range for subspace).
- NAV -> POWER: Fuel budgeting ties directly to power — how much fusion fuel for impulse, how much M/AM fuel for warp, what's the TTF at this speed?

**What breaks:** Sensor calibration drift (you think you're here but you're actually there). Nav deflector failure at warp (debris impacts). Flight computer crash (manual piloting only). Guidance lock loss (course veering).

**Puzzle hooks:**
- Calculate the correct course given fuel constraints, hazards, and time pressure
- Sensor readings don't match — is the sensor miscalibrated or is something hiding?
- Nav deflector power fluctuation — is it the deflector or the M/AM reactor feeding it?
- Triangulate a signal source from multiple sensor readings at different positions
- Identify which nav sensor is drifting by comparing readings across the array

---

### TAC — Tactical

**What it does:** Threat detection, defense, and offense. Sensors to find threats, deflector for active defense, weapons to shoot, electronic warfare to hide or jam.

**Key systems:** Tactical Sensors, Tactical Deflector (energy pulse, science-coupled scan, modified modes, overload management), Tactical Computer, Phaser Arrays (6 arcs), Torpedo Launchers, Weapon Arming & Authorization, Engagement Modes, Electronic Warfare, Cloaking, Countermeasures

**Depends on:** POWER (weapons and shields on EPS, sensors on electrical), COMPUTER (targeting solutions), NAV (position and vector data for firing solutions), CMD (weapons authorization)

**Feeds into:** CMD (threat assessments inform alert level), NAV (evasive patterns require nav coordination), COMMS (jamming/counter-jamming)

**Cross-links:**
- TAC -> CMD: Weapons LOCKED until authorized. Authorization chain: CO grants weapons free, TAC officer arms and fires. Two-person process.
- TAC -> NAV: Firing solution = target position + own ship vector + weapon speed. NAV data is half the equation.
- TAC -> COMMS: Electronic warfare uses comm frequencies. Jamming an enemy means knowing their comm bands. Counter-jamming means protecting yours.
- TAC -> POWER: Phaser arrays draw from EPS. Full phaser volley can brownout other EPS systems (shields, impulse). Power allocation is a tactical decision.
- TAC -> SCIENCE: Tactical deflector has a "science-coupled scan" mode — borrowed for deep scans. Tactical and science share deflector time.
- TAC -> SHIELDS: Shield frequency rotation. If the enemy knows your frequency, they can punch through. Changing frequency mid-battle is a puzzle.

**What breaks:** Phaser array arc failure (blind spot in coverage). Torpedo launcher jam. Shield frequency compromise (enemy adapted). Tactical sensor ghost contacts (decoys vs real). Targeting computer drift (shots missing).

**Puzzle hooks:**
- Six phaser arcs cover 360 degrees — one is down. Which arc? Rotate the ship to compensate?
- Shield frequency has been compromised — deduce the new frequency from the enemy's attack pattern
- Distinguish real contacts from decoys using sensor data across multiple bands
- Weapons authorization chain: verify the order is legitimate (not spoofed)
- Power allocation: full phasers OR full shields, not both. What's the right split for this scenario?
- Electronic warfare: identify which comm band the enemy is using from signal characteristics

---

### SEC — Security

**What it does:** Access control (who can go where, who can use what), intrusion detection (physical and cyber), secure facilities (brig, armory).

**Key systems:** Security Sensors, Virtual Systems (cyber security, console access control, authentication), Physical Spaces (force fields, door locks, restricted areas), Brig/Detention, Armory/Weapons Storage

**Depends on:** COMPUTER (authentication databases, access logs), POWER (force fields on EPS), COMMS (internal comms for security alerts)

**Feeds into:** CMD (security status informs command decisions), ALL (access control gates every console on the ship)

**Cross-links:**
- SEC -> COMPUTER: Three-gate security model. Gate 1: what the console allows. Gate 2: where on the ship it is. Gate 3: who's logged in. All three are checked against the computer's auth database.
- SEC -> CMD: Alert status changes security posture. Red Alert: armory opens, security teams deploy. Intruder Alert: force fields activate, doors lock.
- SEC -> ENVIRO: Environmental can be weaponized for security — vent atmosphere from a compartment to neutralize intruders. Emergency option.
- SEC -> TRANSIT: Transporter can bypass physical security (beam past locked doors). Transporter security protocols prevent unauthorized transport.
- SEC -> ALL CONSOLES: Every console on the ship checks security clearance. A puzzle answer might be hidden behind a clearance gate — you need the right rank to access the right screen.

**What breaks:** Authentication database corruption (nobody can log in, or everyone can). Force field generator failure (can't contain prisoners or intruders). Access log gaps (who was where?). Console lockout (legitimate user denied access).

**Puzzle hooks:**
- Reconstruct an access log to determine who entered a restricted area
- Three-gate security: figure out which gate is blocking access (wrong console? wrong deck? wrong rank?)
- Intruder has spoofed credentials — find the discrepancy in the auth records
- Armory inventory puzzle — weapons checked out don't match weapons returned
- Cyber intrusion: trace unauthorized access through the ODN data network
- Force field routing: which sections can you isolate with the remaining working generators?

---

## YELLOW DIVISION — Operations

### COMMS — Communications

**What it does:** All communication — ship-to-ship, ship-to-ground, ship-to-fleet, internal. 8 comm bands with different physics and ranges. Encryption, translation, emergency beacons.

**Key systems:**
- **Short-range (4 bands):** Low-Wave EM (UHF/VHF) — broadband, noisy, everyone hears it. Standard Radio — workhorse, moderate range. Optical LaserComm — line-of-sight only, very secure, can't intercept what you can't see. Tightbeam Microwave — directional, long-range point-to-point.
- **Subspace (4 bands):** Subspace Pulse — standard fleet comms, reliable, moderate speed. Hyperwave Channel — fast, expensive, reserved for priority traffic. Neutrino Burst — passes through anything, nearly unblockable, hard to aim. Gravitic Modulation — exotic, longest range, requires specialized equipment.
- **Infrastructure:** Universal Translator, Encryption, Emergency Beacon/Transponder, Internal Comms

**Depends on:** POWER (electrical grid for all comms), COMPUTER (signal processing, encryption/decryption), NAV (position data for directional comms)

**Feeds into:** CMD (receiving and relaying orders), TAC (electronic warfare, signal intelligence), SEC (internal comms for alerts), NAV (position broadcasts, distress signals)

**Cross-links:**
- COMMS -> TAC: Electronic warfare lives in the overlap. Jamming = transmitting noise on enemy bands. Interception = listening on enemy bands. Counter-jamming = protecting your own bands. Know the band, know the physics, know the countermeasure.
- COMMS -> NAV: Optical LaserComm needs line-of-sight — position matters. If the target is behind a planet, laser won't work. Switch bands. Tightbeam needs precise aiming — nav data feeds the antenna pointing.
- COMMS -> SEC: Encrypted channels. Who has the encryption keys? Key rotation schedule. A compromised key means the enemy reads your traffic.
- COMMS -> SCIENCE: Subspace phenomena affect comm bands differently. A subspace anomaly might block Subspace Pulse but leave Neutrino Burst clear. Science sensor data tells comms which band will work.
- COMMS -> MEDICAL: Emergency medical consultations over comms. Away team medic needs guidance from sickbay. Band quality matters — you don't want static during surgery instructions.

**What breaks:** Antenna damage (some bands lost, others work). Encryption key compromise. Universal Translator glitch (mistranslation). Subspace interference (natural phenomena blocking specific bands). Signal echo/ghost (hearing your own transmission bounced back, mistaking it for a reply).

**Puzzle hooks:**
- 8 bands, each with different physics — which band works for THIS situation? (Line of sight blocked? Need to penetrate a nebula? Need to avoid detection?)
- Interference classification: what type of interference is this? (Decoy, shattered carrier, harmonic echo, etc.) — each requires different removal technique
- Encrypted message with partial key — deduce the missing key segments
- Signal triangulation: three listening posts, three signal readings, find the source
- Universal Translator failure: reconstruct meaning from partial/garbled translation
- Comm band selection under constraints: enemy is jamming Standard Radio, planet blocks LaserComm, subspace storm blocks Subspace Pulse — which band gets the message through?

---

### ENVIRO — Environmental Control

**What it does:** Two jobs. **Chemistry:** keep the air breathable (atmosphere, pressure, humidity, temperature). **Physics:** keep the reference frame stable (gravity, inertial dampening, SIF).

**Key systems:**
- **Chemistry side:** O2 Generation / CO2 Scrubbing, Pressure Modulation, Humidity Control, Emergency Venting, Temperature Regulation, Circulation Fans, HEPA Filtration
- **Physics side:** Inertial Dampening, Artificial Gravity (setpoint-based plating), Structural Integrity Field (SIF — hull reinforcement at warp)

**Depends on:** POWER (chemistry on electrical, physics on EPS — fields need plasma-level power)

**Feeds into:** ALL (every department needs breathable air and stable gravity to function), PROPULSION (SIF required for warp, dampening required for impulse)

**Cross-links:**
- ENVIRO -> PROPULSION: You can't go to warp without SIF. You can't do impulse maneuvers without dampening. Propulsion depends on ENVIRO fields.
- ENVIRO -> MEDICAL: Atmosphere composition matters for patients. Quarantine requires isolated environmental zones. Alien species may need different atmosphere mix.
- ENVIRO -> SEC: Emergency venting is a security tool — decompress a section to neutralize intruders. Dangerous, irreversible, last resort.
- ENVIRO -> DAMAGE CONTROL: Hull breach = pressure loss = ENVIRO emergency. Damage control patches the hull, ENVIRO repressurizes.
- ENVIRO -> SCIENCE: Multi-species support requires different atmospheric zones. Science labs might need specific atmospheres for experiments.
- ENVIRO -> POWER: Chemistry side runs on electrical (low power). Physics side runs on EPS (high power). If EPS fails, you keep air but lose gravity and dampening.

**What breaks:** CO2 scrubber failure (air slowly becomes toxic — timer puzzle). Pressure breach (hull damage, need to find and seal). Gravity plating malfunction (some sections 0g, others 2g). Dampening failure at speed (crew injury from acceleration). Temperature regulation failure (overheating or freezing, depending on where the ship is relative to a star).

**Puzzle hooks:**
- Atmosphere balancing: CO2 rising, scrubber at 60% — which sections to vent, which to seal, to buy time for repair?
- Gravity setpoint calculation: planet is 0.7g, station is spinning at 0.9g, ship needs 1.0g — what does the plating output?
- Pressure breach localization: pressure dropping in multiple sections — trace the breach by reading pressure sensors across the deck
- Temperature routing: reactor waste heat usually warms the ship, but reactor is down — reroute heat from remaining sources to prevent freezing
- Life support triage: power is limited, can only run full ENVIRO on 3 of 5 decks — which decks get priority and why?
- Multi-species atmosphere: alien ambassador needs different O2 mix — which sections can be isolated and recomposed?

---

### TRANSIT — On/Off Ship Operations

**What it does:** Everything that moves people or cargo on or off the ship. Airlocks, escape pods, EVA, transporter, shuttle bay, cargo bay.

**Key systems:** Airlock, Escape Pods, EVA Suits, Transporter (organic/inorganic/cargo modes), Shuttle Bay Ops, Shuttle Operations, Shuttle Maintenance, Cargo Bay Operations, Cargo Loading/Inventory

**Depends on:** POWER (transporter on EPS, others on electrical), ENVIRO (atmosphere for shuttle bay and cargo bay), SEC (transporter security protocols, cargo manifests)

**Feeds into:** CMD (away team deployment), MEDICAL (medical emergency transport), SEC (transporter as bypass vector)

**Cross-links:**
- TRANSIT -> MEDICAL: Transporter biofilter scans for pathogens, weapons, explosives. Medical database feeds the biofilter's detection library. New pathogen? Update the biofilter or miss it.
- TRANSIT -> SEC: Transporter can bypass locked doors and force fields. Security protocols prevent unauthorized beaming. Hacking the transporter = hacking security.
- TRANSIT -> NAV: Transporter needs a sensor lock on the target — nav sensors provide position data. Shuttle departure needs nav clearance and flight path.
- TRANSIT -> POWER: Transporter is a massive EPS draw. Beaming during a power crisis means something else goes dark. Cargo transport draws even more.
- TRANSIT -> ENVIRO: Shuttle bay must be pressurized for crew, depressurized for launch. Cycling the atmosphere takes time.
- TRANSIT -> COMMS: Shuttle operations need comms coordination. Away team transport needs comm lock for emergency beam-out.

**What breaks:** Transporter pattern buffer degradation (person stuck in the buffer — timer puzzle). Biofilter miss (something got aboard that shouldn't have). Shuttle bay door jammed (can't launch or recover). Cargo manifest discrepancy (what's actually in the cargo bay vs what the records say). Airlock seal failure.

**Puzzle hooks:**
- Transporter pattern buffer: signal is degrading, you have X minutes to resolve the problem or lose the pattern
- Biofilter identification: unknown organism detected — match it against the medical database to determine threat level
- Cargo manifest audit: inventory doesn't match manifest — what's missing, what's extra, was it theft or error?
- Shuttle bay pressure cycle: need to launch a shuttle but bay is pressurized with crew working — sequence the evacuation, depressurization, and launch
- Emergency beam-out: away team in danger, but there's interference on the signal lock — boost from which system?
- Transporter + security: someone beamed aboard without authorization — trace the transport log, find the origin coordinates, identify the intruder

---

### REPLICATOR

**What it does:** Matter-energy conversion. Makes food, tools, replacement parts, medical supplies — anything the pattern library contains. Also de-replicates waste back into raw matter.

**Key systems:** Organic Replicator (food, biological materials), Inorganic Replicator (tools, parts, equipment), Large/Cargo Replicator (big items), De-Replication/Waste Processing

**Depends on:** POWER (EPS — matter-energy conversion needs plasma power), COMPUTER (pattern library)

**Feeds into:** MEDICAL (medical supplies on demand), DAMAGE CONTROL (replacement parts), ALL (food, tools, daily needs)

**Cross-links:**
- REPLICATOR -> MEDICAL: Medical supplies on demand — but does the replicator have the pattern? Exotic medications might not be in the library.
- REPLICATOR -> DAMAGE CONTROL: Replacement parts for repairs. Replicator quality vs original parts — replicated components may be lower tolerance.
- REPLICATOR -> POWER: Big EPS draw. Replicating during power rationing is a luxury vs necessity decision.
- REPLICATOR -> SEC: Weapons replication is restricted. Security lockouts prevent unauthorized replication of weapons, explosives, restricted materials.
- REPLICATOR -> SCIENCE: New patterns require science analysis — scan an object, analyze its composition, create a replication pattern.

**What breaks:** Pattern library corruption (item replicates wrong). Power fluctuation during replication (item partially formed). De-replication failure (waste backing up). Restricted item bypass (security lockout failed).

**Puzzle hooks:**
- Pattern library corruption: replicated item has defects — compare to spec, identify which pattern parameters are wrong
- Resource rationing: limited power, list of needed items — prioritize what to replicate
- Reverse engineering: scan an unknown object, analyze its composition, create a replication pattern from sensor data
- Security audit: someone replicated a restricted item — trace the authorization override

---

### HOLODECK

**What it does:** Holographic simulation. Environmental projection, object replication, interactive scenarios. Used for training, recreation, and tactical planning.

**Key systems:** Holo-Environmental (simulate any environment), Holo-Projection (visual/auditory/tactile), Holo-Replication (temporary physical objects)

**Depends on:** POWER (EPS for projection), COMPUTER (massive processing for real-time simulation), ENVIRO (atmosphere within holodeck)

**Cross-links:**
- HOLODECK -> COMPUTER: Holodeck is the biggest single processing load on the ship's computer. Running a complex simulation while the ship is in crisis = resource conflict.
- HOLODECK -> MEDICAL: Medical training simulations. Surgical practice. Therapy programs.
- HOLODECK -> TAC: Tactical simulations. Battle planning. Crew combat training.
- HOLODECK -> SEC: Holodeck safeties. When safeties fail, holographic weapons become real. Security emergency.

**What breaks:** Safety protocol failure (holographic weapons can injure). Program corruption (simulation deviating from parameters). Power draw conflict (holodeck vs critical systems). Exit blocked (crew trapped in simulation).

**Puzzle hooks:**
- Holodeck safety failure: determine which safety protocol failed and how to restore it from outside
- Simulation glitch: the holographic scenario has contradictions — identify what's wrong with the program
- Resource conflict: holodeck is drawing too much power during a crisis — find the program and shut it down

---

## ORANGE DIVISION — Engineering

### POWER

**What it does:** Generate and distribute all power for the ship. Two generators (fusion, M/AM), two distribution grids (electrical, EPS), battery backup, and the PCU that splits it all.

**Key systems:** Fusion Reactor, M/AM Reactor, Batteries, EPS Distribution (plasma), Electrical Distribution, Power Conditioning Unit (PCU)

**Depends on:** Nothing — POWER is the root. Everything depends on it.

**Feeds into:** EVERYTHING. Every system on the ship traces back to POWER.

**Cross-links:**
- POWER -> ALL: The dependency tree IS the ship's nervous system. Battery feeds electrical. Fusion feeds PCU. PCU splits to electrical + EPS. M/AM feeds Warp EPS. Every department connects here.
- POWER -> NAV: Fuel management. Fusion fuel for impulse operations. M/AM fuel for warp. Fuel level determines range and options.
- POWER -> TAC: Power allocation in combat. Full phasers + full shields exceeds output — something has to give. Power routing is a tactical decision.
- POWER -> ENVIRO: Chemistry runs on electrical (low), physics runs on EPS (high). Power loss affects them differently.
- POWER -> DAMAGE CONTROL: Reactor damage is the most critical failure. Damage control priority #1 is always power.

**What breaks:** Reactor SCRAM (emergency shutdown — ship on battery). EPS conduit rupture (plasma leak — dangerous). PCU failure (reactor running but can't distribute). Battery depletion (no backup). Fuel contamination. Coolant leak.

**Puzzle hooks:**
- Power load balancing: total demand exceeds reactor output — which systems get power and which go dark?
- EPS routing: conduit ruptured on Deck 3 — reroute plasma through backup conduits to maintain power to critical sections
- Reactor diagnostics: output is fluctuating — is it the reactor, the PCU, or a downstream short?
- Fuel audit: fuel consumption doesn't match the flight log — is there a leak or was there unauthorized use?
- Battery management: reactor is down, battery is at 40% — calculate TTF for each possible system configuration
- Cascade failure: one system failure triggers others through the dependency tree — trace the cascade and find the root cause

---

### COMPUTER

**What it does:** Processing, storage, and data networking for the entire ship. Primary core, secondary/auxiliary core, 13 dedicated computers (one per major system), and the ODN data network connecting everything.

**Key systems:** Primary Core, Secondary/Auxiliary Core, 13 Dedicated Computers (nav, tactical, medical, science, environmental, transporter, weapons, engineering, comms, shuttle, cargo, security, holodeck), ODN (Optical Data Network), Data Pathways

**Depends on:** POWER (electrical grid)

**Feeds into:** EVERYTHING — every system uses the computer for processing, storage, and control

**Cross-links:**
- COMPUTER -> ALL: The computer IS the glue. Nav computer plots courses. Tactical computer calculates firing solutions. Medical computer maintains patient records. All of them talk through the ODN.
- COMPUTER -> SEC: Authentication databases live here. Console access control. Intrusion detection. If the computer is compromised, security is compromised.
- COMPUTER -> COMMS: Signal processing for all 8 comm bands. Encryption/decryption. Universal Translator. All comm functions are software running on the computer.
- COMPUTER -> HOLODECK: Biggest processing load. A holodeck simulation can consume 40% of available processing.

**What breaks:** Core failure (switch to backup core — reduced capacity). ODN pathway damage (some consoles lose data, others fine — network topology puzzle). Dedicated computer crash (one system loses its specialized processing). Data corruption. Processing overload (too many demands, system prioritization).

**Puzzle hooks:**
- ODN routing: data pathway damaged — which consoles lose connectivity? Can you reroute through backup pathways?
- Processing allocation: three critical systems all need priority processing — rank them and allocate
- Dedicated computer failure: the nav computer crashed — what functions are lost and what can the primary core take over?
- Data forensics: corrupted database — reconstruct records from fragments across multiple cores
- Network topology: trace a data packet from source to destination through the ODN — find the bottleneck or the tap

---

### DAMAGE CONTROL

**What it does:** Assess damage, prioritize repairs, coordinate repair teams, track maintenance schedules. The firefighters of the ship.

**Key systems:** Damage Control (assessment, prioritization, team dispatch), Maintenance (scheduled upkeep, parts inventory, repair history)

**Depends on:** POWER (for everything), COMPUTER (damage reports, repair tracking), COMMS (coordination with repair teams), REPLICATOR (replacement parts)

**Feeds into:** ALL — damage control repairs whatever breaks in any department

**Cross-links:**
- DAMAGE CONTROL -> ALL: Any system can break. Damage control needs to understand EVERY department well enough to assess and repair. Cross-department knowledge is the core skill.
- DAMAGE CONTROL -> POWER: Power restoration is always priority one. No power = no repairs to anything else.
- DAMAGE CONTROL -> REPLICATOR: Need a replacement EPS conduit? Replicate one. Replicator down? Now you're scavenging parts from non-critical systems.
- DAMAGE CONTROL -> ENVIRO: Hull breach = pressure loss. Damage control seals the breach, ENVIRO repressurizes. Coordinated response.
- DAMAGE CONTROL -> SEC: Damage from attack vs damage from malfunction — security needs to know which.

**What breaks:** Damage control itself rarely "breaks" — but it can be overwhelmed. Too many simultaneous failures. Repair teams injured. Parts unavailable. Prioritization errors (fixing the wrong thing first).

**Puzzle hooks:**
- Damage prioritization: 5 systems damaged, 2 repair teams — what order, what assignment?
- Parts scavenging: needed part can't be replicated — which non-critical system has a compatible component?
- Repair sequencing: System A needs System B working to repair it, but System B needs System C — find the repair order
- Damage assessment: multiple casualty reports coming in — which are critical, which can wait, which are duplicates?
- Maintenance puzzle: scheduled maintenance was skipped — predict which system will fail first based on maintenance history

---

### PROPULSION

**What it does:** Move the ship. Three tiers: thrusters (slow/precise), impulse (fast/in-system), warp (FTL/between systems).

**Key systems:**
- **Thrusters:** Ion RCS, Matter RCS, Microthruster Arrays, Lateral Thruster Pods — electrical power, fine maneuvering
- **Impulse Drive:** Main Drive Nozzles, Exhaust Assemblies, Nozzle Collimators, Thrust Vectoring — EPS power, in-system travel
- **Warp Drive:** Warp Core (fuel intake, matter/antimatter injection, dilithium matrix, intermix chamber, coolant, plasma distribution), Nacelles (Bussard collector, warp field coils, field geometry control, signature dampening) — M/AM power, FTL travel

**Depends on:** POWER (thrusters=electrical, impulse=EPS, warp=M/AM), NAV (course data, guidance), ENVIRO (SIF required for warp, dampening for impulse)

**Feeds into:** NAV (propulsion executes what nav plots), POWER (warp core IS the M/AM reactor)

**Cross-links:**
- PROPULSION -> NAV: NAV says where, PROPULSION says how. Speed changes affect fuel consumption, arrival time, sensor effectiveness.
- PROPULSION -> ENVIRO: Impulse acceleration requires inertial dampening. Warp requires SIF. No fields = no safe propulsion.
- PROPULSION -> POWER: Impulse draws from EPS. Warp has its own dedicated Warp EPS. Full impulse + full shields can max out the fusion reactor.
- PROPULSION -> DAMAGE CONTROL: Nacelle damage is critical at warp. Warp field asymmetry from damaged coils can tear the ship apart.
- PROPULSION -> TAC: Speed and maneuverability matter in combat. Evasive patterns are propulsion + nav working together.

**What breaks:** Thruster misfire (ship drifts). Impulse drive nozzle misalignment (thrust vector off). Warp field geometry failure (asymmetric field — dangerous). Dilithium matrix degradation (losing warp efficiency). Coolant leak (reactor overheating). Bussard collector failure (can't scoop supplemental fuel at warp).

**Puzzle hooks:**
- Warp field geometry: coils are misaligned — calculate the correct power distribution across coils to restore symmetric field
- Fuel optimization: limited fuel, multiple destinations — what speed and route minimizes consumption?
- Thruster diagnostics: ship is drifting — which thruster pod is misfiring? Read the RCS output logs.
- Impulse nozzle calibration: thrust vector is off by X degrees — calculate the correction
- Dilithium matrix degradation: efficiency dropping over time — project when the matrix will fail and plan the replacement window
- Emergency propulsion: main impulse is down — can you limp on thrusters only? How long to reach the station?

---

### AUXILIARY SYSTEMS

**What it does:** Miscellaneous ship systems that don't fit neatly into other departments. Turbolift, tractor beam, probes, towing, coolant, master diagnostics.

**Key systems:** Turbolift (crew transport between decks), Tractor Beam (grab/hold/tow objects), Probe Launchers (deploy sensor probes), Towing (structural towing of other vessels), Coolant System (thermal management for all reactors and high-power systems), Master Diagnostics

**Depends on:** POWER (EPS for tractor/towing, electrical for turbolift/probes), COMPUTER (diagnostics, turbolift routing)

**Feeds into:** NAV (probes extend sensor range), POWER (coolant keeps reactors operating), ALL (turbolift moves crew, master diagnostics evaluates any system)

**Cross-links:**
- AUXILIARY -> POWER: Coolant is critical for reactor operation. Coolant failure = reactor overheating = SCRAM. The coolant system is POWER's lifeline.
- AUXILIARY -> NAV: Probes extend sensor range. Launch a probe ahead into a nebula to scout what nav sensors can't see.
- AUXILIARY -> TRANSIT: Tractor beam for shuttle recovery, cargo handling, docking assistance. Towing for disabled vessels.
- AUXILIARY -> ALL: Master Diagnostics can run diagnostics on ANY system on the ship. It's the meta-tool — the diagnostic that diagnoses other diagnostics.

**What breaks:** Turbolift stuck (crew stranded between decks). Tractor beam power fluctuation (dropping a held object). Coolant leak (reactor temperature climbing). Probe launcher jam. Diagnostic false positive/negative.

**Puzzle hooks:**
- Coolant routing: leak detected in coolant loop — trace the loop, find the leak segment, reroute through backup
- Master diagnostic: system X shows FAULT but passes individual tests — use master diagnostics to find the intermittent failure
- Probe data analysis: probe sent into a nebula returns garbled data — reconstruct the sensor readings
- Tractor beam calculation: need to tow a vessel of mass X with your tractor rated for Y — can you do it? At what speed?
- Turbolift routing: turbolift stuck, crew injured — which maintenance access shaft reaches that location?

---

## BLUE DIVISION — Science

### MEDICAL

**What it does:** Keep the crew alive and healthy. Diagnosis, treatment, surgery, quarantine, medical research, pharmaceutical production.

**Key systems:** Diagnosis, Surgery, Stasis (for cases beyond current treatment), Quarantine, Medical Computer, Medical Sensors, Medical Scanners, Medical Research, Medical Production (drugs, compounds — replicator makes it faster but the CAPABILITY is the department's), Morgue

**Depends on:** POWER (electrical for scanners/computer, EPS for surgical equipment), COMPUTER (medical database, patient records), REPLICATOR (medical supplies), TRANSIT (emergency transport of patients)

**Feeds into:** CMD (crew fitness reports, away team medical clearance), TRANSIT (biofilter database), SEC (autopsy/forensics), SCIENCE (biological research)

**Cross-links:**
- MEDICAL -> TRANSIT: Biofilter. Medical database tells the transporter what to scan for. New pathogen discovered by medical? Update the biofilter immediately or next transport brings it aboard.
- MEDICAL -> ENVIRO: Quarantine requires isolated environmental zones. Alien patient needs different atmosphere mix — ENVIRO adjusts.
- MEDICAL -> SCIENCE: Medical research + science research overlap. Unknown alien biology requires both departments.
- MEDICAL -> COMMS: Remote medical consultation. Away team medic needs guidance — comm quality matters for surgical instruction.
- MEDICAL -> CMD: Crew fitness determines duty assignments. Medical can pull someone from duty. CO needs to know.
- MEDICAL -> SEC: Forensic analysis. Cause of death. Toxicology. Autopsy results feed security investigations.

**What breaks:** Scanner malfunction (misdiagnosis). Medical database corruption (wrong treatment protocol). Stasis unit failure (patient reviving prematurely). Quarantine containment breach. Replicator can't produce needed compound (pattern not in library).

**Puzzle hooks:**
- Diagnosis: patient presents symptoms matching multiple conditions — use scanner data, medical history, environmental factors to narrow down
- Triage: multiple casualties, limited surgical capacity — prioritize by severity and survivability
- Quarantine logic: pathogen detected — which sections were exposed? Model the spread, determine isolation boundaries
- Unknown pathogen: symptoms don't match any known disease — cross-reference with science sensor data, away team logs, environmental readings
- Toxicology: crew member poisoned — identify the substance from scanner readings, trace the source (replicator? cargo? sabotage?)
- Biofilter update: new threat identified — define the scanning parameters to add to the transporter biofilter

---

### SCIENCE

**What it does:** Research, analysis, and sensor operation for scientific purposes (as opposed to nav or tactical sensors). The "figure out what this IS" department.

**Key systems:** Science Operations (coordination, prioritization), Science Research (labs, experiments, analysis), Science Sensors (long-range, deep-scan, multi-spectrum)

**Depends on:** POWER (electrical for sensors, EPS for high-energy experiments), COMPUTER (massive processing for analysis), TAC (shared deflector time for deep scans)

**Feeds into:** ALL — science analysis informs every department's understanding of what they're dealing with

**Cross-links:**
- SCIENCE -> TAC: Tactical deflector has a science-coupled scan mode. Science borrows the deflector dish for deep-space surveys. Scheduling conflict: tactical needs the deflector for defense, science needs it for research.
- SCIENCE -> MEDICAL: Unknown biological samples. Alien life forms. Shared analysis.
- SCIENCE -> NAV: Stellar phenomena analysis feeds navigation — that nebula's radiation profile tells nav whether it's safe to fly through.
- SCIENCE -> COMMS: Subspace phenomena affect comm bands. Science analysis of a subspace anomaly tells comms which bands will work.
- SCIENCE -> ENVIRO: Planetary atmosphere analysis for away team preparation. Multi-species environmental requirements.
- SCIENCE -> REPLICATOR: Scanning unknown objects to create replication patterns. Analysis before reproduction.

**What breaks:** Sensor calibration drift. Lab containment failure (experiment escapes containment). Processing overload (analysis taking too long, queuing behind other demands). Sensor array damage (blind in specific spectra).

**Puzzle hooks:**
- Unknown substance: sensor readings across multiple spectra — cross-reference to identify composition
- Anomaly analysis: subspace readings are unusual — categorize the anomaly type and predict its behavior
- Shared deflector scheduling: science needs a 2-hour deep scan, tactical needs the deflector on standby — negotiate or sequence
- Stellar cartography: map an unknown system from sensor data — star type, planet count, habitable zone, hazards
- Experiment gone wrong: lab readings are off — determine what changed and whether containment is holding
- Sensor fusion: combine data from nav sensors, tactical sensors, and science sensors to build a complete picture that no single sensor type can provide

---

## Cross-Department Connection Map

Every line below is a potential puzzle that spans two or more departments:

```
                         +---- CMD ----+
                         |  (authority) |
                         +--+---+---+--+
                            |   |   |
              +-------------+   |   +-------------+
              v                 v                 v
           +-----+          +-----+           +-----+
           | SEC |<-------->| TAC |<--------->| NAV |
           |     |          |     |           |     |
           +--+--+          +--+--+           +--+--+
              |                |                 |
              |           +----+----+            |
              |           v         v            |
              |      +--------+ +-------+        |
              |      |WEAPONS| |SHIELDS|        |
              |      +--------+ +-------+        |
              |                                  |
    +---------+----------------------------------+---------+
    |                    POWER (root)                        |
    |  Battery -> Electrical -> Fusion -> EPS -> M/AM -> Warp |
    +---+--------+--------+--------+--------+--------+-----+
        |        |        |        |        |        |
        v        v        v        v        v        v
    COMPUTER  COMMS    ENVIRO  TRANSIT  PROPULSION  AUX
        |        |        |        |        |        |
        |        |        |        |        |        |
        v        v        v        v        v        v
    +----+   +----+   +----+  +------+  +----+  +----+
    | ODN|   |8   |   |CHEM|  |TRANS-|  |WARP|  |COOL|
    |DATA|   |COMM|   |+   |  |PORT +|  |CORE|  |ANT |
    |NET |   |BAND|   |PHYS|  |CARGO |  |    |  |    |
    +----+   +----+   +----+  +------+  +----+  +----+
                                |
                         +------+------+
                         v             v
                    +--------+    +--------+
                    |MEDICAL |    |SCIENCE |
                    |(biofltr|    |(deflctr|
                    | + diag)|    | + labs)|
                    +--------+    +--------+
```

**Every arrow is a puzzle.** Power feeds everything (dependency sequencing). Comms links to tactical (electronic warfare). Medical links to transit (biofilter). Science links to tactical (shared deflector). Security links to computer (auth databases). Environmental links to propulsion (fields required for speed). The web of connections means a fault in one department cascades to others — and tracing that cascade IS the puzzle.
