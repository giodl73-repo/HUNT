# R2-02 — DATA BREACH
**Round:** SYSTEMS LOG
**Department:** COMPUTER / ODN NETWORK
**Panel:** ODN Isolation Console — Main Engineering Computer Section, Deck 5
**Classification:** RESTRICTED — SYSTEMS LOG RECONSTRUCTION

---

## 1. Panel Overview

**Narrative beat:** During the gap, unauthorized data was injected into the ship's Optical Data Network. A rogue packet — visually distinct in red — circulates through the 15-node network, following a fixed route. The solver systematically disables nodes to trace the rogue packet backward to its entry point. The entry node reveals which system was compromised to forge sensor readings.

**Win condition:** The rogue packet's network entry point identified. When the entry node is disabled, the red packet never appears on the network.

**Answer value:** 7 (the breach entry node ID — Node 7, the Comms Computer)

---

## 2. Widget Configuration

### Primary Display

**Widget:** `NetworkGridDisplay`
**Config:**
```
{
  nodeCount: 15,
  nodes: [
    { id: 1,  label: "PRIMARY",    dept: "CMD", x: 400, y: 40 },
    { id: 2,  label: "SECONDARY",  dept: "CMD", x: 250, y: 120 },
    { id: 3,  label: "NAV",        dept: "NAV", x: 100, y: 120 },
    { id: 4,  label: "TACTICAL",   dept: "TAC", x: 50,  y: 220 },
    { id: 5,  label: "COMMS",      dept: "OPS", x: 550, y: 120 },
    { id: 6,  label: "ENVIRO",     dept: "OPS", x: 550, y: 220 },
    { id: 7,  label: "SECURITY",   dept: "SEC", x: 250, y: 220 },
    { id: 8,  label: "ENGINE",     dept: "ENG", x: 100, y: 320 },
    { id: 9,  label: "SCIENCE",    dept: "SCI", x: 250, y: 320 },
    { id: 10, label: "REPLICATOR", dept: "OPS", x: 550, y: 320 },
    { id: 11, label: "TRANSPORT",  dept: "OPS", x: 650, y: 320 },
    { id: 12, label: "HOLOGRAPH",  dept: "OPS", x: 650, y: 420 },
    { id: 13, label: "TURBOLIFT",  dept: "ENG", x: 100, y: 420 },
    { id: 14, label: "DIAGNOSTIC", dept: "ENG", x: 250, y: 420 },
    { id: 15, label: "SENSOR",     dept: "NAV", x: 100, y: 220 }
  ],
  links: [
    { from: 1, to: 2 }, { from: 1, to: 3 }, { from: 1, to: 5 },
    { from: 2, to: 7 }, { from: 2, to: 6 },
    { from: 3, to: 4 }, { from: 3, to: 15 },
    { from: 4, to: 8 },
    { from: 5, to: 6 }, { from: 5, to: 7 },
    { from: 6, to: 10 }, { from: 6, to: 11 },
    { from: 7, to: 9 },
    { from: 8, to: 9 }, { from: 8, to: 13 },
    { from: 9, to: 14 },
    { from: 11, to: 12 },
    { from: 13, to: 14 }
  ],
  normalPacketColor: "#4488FF",
  roguePacketColor: "#FF2222",
  rogueRoute: [7, 5, 1, 3, 4, 8, 9, 7],
  roguePacketSpeed: 1.5,
  nodeLoadFlicker: true
}
```

### Input Controls

**Widget:** `ToggleSwitch` (x15)
```
{
  switches: [
    { id: "node-1",  label: "NODE 01 — PRIMARY",    initialState: true },
    { id: "node-2",  label: "NODE 02 — SECONDARY",  initialState: true },
    { id: "node-3",  label: "NODE 03 — NAV",        initialState: true },
    { id: "node-4",  label: "NODE 04 — TACTICAL",   initialState: true },
    { id: "node-5",  label: "NODE 05 — COMMS",      initialState: true },
    { id: "node-6",  label: "NODE 06 — ENVIRO",     initialState: true },
    { id: "node-7",  label: "NODE 07 — SECURITY",   initialState: true },
    { id: "node-8",  label: "NODE 08 — ENGINE",     initialState: true },
    { id: "node-9",  label: "NODE 09 — SCIENCE",    initialState: true },
    { id: "node-10", label: "NODE 10 — REPLICATOR", initialState: true },
    { id: "node-11", label: "NODE 11 — TRANSPORT",  initialState: true },
    { id: "node-12", label: "NODE 12 — HOLOGRAPH",  initialState: true },
    { id: "node-13", label: "NODE 13 — TURBOLIFT",  initialState: true },
    { id: "node-14", label: "NODE 14 — DIAGNOSTIC", initialState: true },
    { id: "node-15", label: "NODE 15 — SENSOR",     initialState: true }
  ],
  layout: "vertical-scrollable",
  grouping: "by-department"
}
```

### Isolation Logic

```
When a node is toggled OFF:
  - Node dims on NetworkGridDisplay
  - All links to/from that node become dashed (inactive)
  - Normal blue packets reroute around disabled nodes
  - Rogue red packet CANNOT reroute — it follows a fixed path
    - If the disabled node is on the rogue route, the packet STOPS
      at the last enabled node before the disabled one
    - If the disabled node is the ENTRY POINT (node 7), the rogue
      packet never appears at all

When a node is toggled back ON:
  - Node brightens, links reactivate
  - Rogue packet resumes circulation if its full route is clear
```

---

## 3. Reference Card

```
ODN ISOLATION CONSOLE — OPERATOR REFERENCE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

WHAT YOU SEE:
  A network of 15 computer nodes connected by data links.
  Blue packets = normal ship data traffic.
  Red packet = unauthorized rogue data, circulating on a fixed route.

  Each node has a toggle switch. Flipping a switch OFF disables
  that node — it goes dark, its links deactivate.

THE PROBLEM:
  A rogue data packet was injected into the ODN during the gap.
  It follows a fixed route through certain nodes. You need to
  find which node is the ENTRY POINT — where the packet enters
  the network.

HOW TO TRACE:
  1. Watch the red packet. Note which nodes it visits.
  2. Disable a node (toggle OFF). Two things can happen:
     a) Red packet still circulates → that node is NOT on its route.
     b) Red packet stops at a node or disappears → the disabled
        node IS on the route.
  3. If you disable a node and the red packet STOPS at another node,
     the disabled node is DOWNSTREAM of where the packet stopped.
  4. If you disable a node and the red packet NEVER APPEARS,
     that node is the ENTRY POINT.
  5. Re-enable nodes between tests to let the packet flow again.

STRATEGY:
  Start from the edges of the network. Disable outer nodes first.
  If the packet still flows, those nodes are not on the route.
  Work inward. The entry point is the node where the packet
  originates — disabling it prevents the packet from ever appearing.

SUCCESS STATE:
  You have found the one node whose disabling prevents the red
  packet from appearing at all. That node's ID number is the answer.
```

---

## 4. Novice Solve Path

1. **Observe:** Watch the red packet circulate. It visits several nodes but moves quickly. Hard to track the full route visually, but the solver can see it passing through the middle of the network.

2. **Test outlying nodes first:**
   - Disable Node 12 (HOLOGRAPH). Red packet still flows. Not on route. Re-enable.
   - Disable Node 10 (REPLICATOR). Still flows. Not on route. Re-enable.
   - Disable Node 11 (TRANSPORT). Still flows. Not on route. Re-enable.
   - Disable Node 6 (ENVIRO). Still flows. Not on route. Re-enable.
   - Disable Node 15 (SENSOR). Still flows. Not on route. Re-enable.

3. **Test deeper nodes:**
   - Disable Node 13 (TURBOLIFT). Still flows. Re-enable.
   - Disable Node 14 (DIAGNOSTIC). Still flows. Re-enable.
   - Disable Node 2 (SECONDARY). Still flows. Re-enable.

4. **Test mid-network nodes:**
   - Disable Node 9 (SCIENCE). Red packet stops at Node 8. Node 9 is on the route, downstream of 8. Re-enable.
   - Disable Node 8 (ENGINE). Red packet stops at Node 4. Node 8 is on the route, downstream of 4. Re-enable.
   - Disable Node 4 (TACTICAL). Red packet stops at Node 3. Node 4 is on the route, downstream of 3. Re-enable.
   - Disable Node 3 (NAV). Red packet stops at Node 1. Node 3 is on the route, downstream of 1. Re-enable.
   - Disable Node 1 (PRIMARY). Red packet stops at Node 5. Node 1 is on the route, downstream of 5. Re-enable.
   - Disable Node 5 (COMMS). Red packet stops at Node 7. Node 5 is on the route, downstream of 7. Re-enable.

5. **Test Node 7:**
   - Disable Node 7 (SECURITY). Red packet never appears. Node 7 is the entry point.

6. **Verify:** Re-enable all nodes except 7. Red packet absent. Enable 7, disable anything else on the route — packet stops but does not disappear. Only disabling 7 kills the packet entirely.

7. **Answer:** Node 7 — the breach entry point.

---

## 5. Expert Solve Path

1. Watch the red packet for one full circuit. Observe it visits approximately 7 nodes in a loop. Note the general path runs through the central/left portion of the network.

2. Pick a node near the center of the observed route — say Node 5 (COMMS). Disable it. Red packet stops at Node 7. This means the route goes 7 → 5 → ... somewhere.

3. Node 7 might be the entry point. Disable Node 7 (SECURITY) instead. Red packet never appears.

4. Confirmed in 2-3 toggles. Answer: 7.

---

## 6. Data Values

### Rogue Packet Route (fixed, non-reroutable)

```
Node 7 (SECURITY) → Node 5 (COMMS) → Node 1 (PRIMARY) →
Node 3 (NAV) → Node 4 (TACTICAL) → Node 8 (ENGINE) →
Node 9 (SCIENCE) → Node 7 (SECURITY) [loop]
```

Route length: 7 nodes. Cycle time: approximately 12 seconds at default speed.

### Nodes ON the Route

| Node | Label | On Route? |
|------|-------|-----------|
| 1 | PRIMARY | YES |
| 2 | SECONDARY | no |
| 3 | NAV | YES |
| 4 | TACTICAL | YES |
| 5 | COMMS | YES |
| 6 | ENVIRO | no |
| 7 | SECURITY | YES (entry point) |
| 8 | ENGINE | YES |
| 9 | SCIENCE | YES |
| 10 | REPLICATOR | no |
| 11 | TRANSPORT | no |
| 12 | HOLOGRAPH | no |
| 13 | TURBOLIFT | no |
| 14 | DIAGNOSTIC | no |
| 15 | SENSOR | no |

### Node Isolation Behavior

| Node Disabled | Red Packet Behavior |
|--------------|-------------------|
| Node 7 (entry) | Packet NEVER APPEARS |
| Node 5 | Stops at Node 7 |
| Node 1 | Stops at Node 5 |
| Node 3 | Stops at Node 1 |
| Node 4 | Stops at Node 3 |
| Node 8 | Stops at Node 4 |
| Node 9 | Stops at Node 8 |
| Any off-route node | Packet continues normally |

### Network Topology (Adjacency)

```
 1: [2, 3, 5]
 2: [1, 6, 7]
 3: [1, 4, 15]
 4: [3, 8]
 5: [1, 6, 7]
 6: [2, 5, 10, 11]
 7: [2, 5, 9]
 8: [4, 9, 13]
 9: [7, 8, 14]
10: [6]
11: [6, 12]
12: [11]
13: [8, 14]
14: [9, 13]
15: [3]
```

---

## 7. Narrative Revelation

On identifying Node 7 as the breach entry point, the following log entry appears on the NetworkGridDisplay status bar:

```
ODN INTRUSION LOG — GAP +00:14
  Rogue packet origin: NODE 07 — SECURITY COMPUTER
  Payload class: SENSOR DATA OVERRIDE
  Target: NODE 15 (SENSOR COMPUTER) via PRIMARY relay
  Effect: External contact readings suppressed at pre-filter stage.
  Authorization: Maintenance port access. No badge record.
```

---

## 8. Story Layer

**The anomaly:** The rogue data entered through Node 7 — the Security Computer. This node controls access logs, internal sensors, and the brig. The packet's payload was a sensor data override, routed through the Primary Computer to reach the Sensor Computer (Node 15). Its purpose: suppress the contact readings at the pre-filter stage so that no other console on the ship would display the contact during the gap.

**What this means:** Someone with physical access to the Security Computer's maintenance port injected forged data into the ODN. The maintenance port has no badge reader — it is a hardwired diagnostic interface inside the Security Office, accessible only to security personnel and senior officers. The injection occurred at GAP +00:14, just eight minutes before the EPS reroute to Lab 7. The sequence was: suppress the sensors first, then power up the unauthorized lab.

**Connection to conspiracy:** The breach came through Security, not through Engineering or Comms. This means the person who ordered the cover-up had security-level physical access. Combined with the forced badge entries (R3-04), this points to someone in the command chain — not a rogue technician. The data was not stolen. It was planted. The sensors were not blinded by accident. They were blinded by design.
