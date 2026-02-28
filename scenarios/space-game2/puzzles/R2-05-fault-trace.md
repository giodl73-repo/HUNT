# R2-05 — FAULT TRACE
**Round:** SYSTEMS LOG
**Department:** ENG / EPS DISTRIBUTION
**Panel:** EPS Circuit Diagnostic Console — Main Engineering, Deck 5
**Classification:** RESTRICTED — SYSTEMS LOG RECONSTRUCTION

---

## 1. Panel Overview

**Narrative beat:** An EPS component failed during the gap, cutting power to the sensor recording subsystem. The failure meant the instruments that could have recorded the contact went dark — and the failure itself was never logged because the logging system depended on the same circuit. The solver probes nodes in a branching EPS circuit to find where current stops flowing, isolating the failed component through binary search on a non-linear topology.

**Win condition:** All probe data identifies a single component as the failure point — the last node with current before the circuit goes dead. The solver has traced the fault through the branching topology.

**Answer value:** 9 (failed component ID)

---

## 2. Widget Configuration

### Primary Display

**Widget:** `CircuitTopologyDisplay`
**Config:**
```
{
  nodeCount: 20,
  sourceNode: 1,
  destinationNodes: [16, 17, 18, 19, 20],
  failedComponent: 9,
  currentFlowColor: "#00FF44",
  noCurrentColor: "#661111",
  probeFlashDuration: 800,
  probeGreenColor: "#00FF88",
  probeRedColor: "#FF2222",
  showNodeLabels: true,
  showBranchLabels: true,
  topology: "see below"
}
```

### Circuit Topology

The EPS circuit is not a simple chain. It branches at several points, creating a tree structure. Current flows from the source (node 1) through the tree toward five destination nodes (16-20). The failed component at node 9 blocks current to a specific branch.

```
EPS SENSOR DISTRIBUTION CIRCUIT — TOPOLOGY
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

                            [1] SOURCE
                           / |
                         /   |
                      [2]   [3]
                     / |     |
                   /   |    [6]
                [4]   [5]   / \
                |      |  [12] [13]
               [7]    [8]  |     |
               / \     |  [16]  [17]    ← destinations
             [9] [10] [11]
             |    |     |
            [14] [15]  [18]             ← destinations
             |    |
            [19] [20]                   ← destinations

  BRANCH A: 1 → 2 → 4 → 7 → 9 → 14 → 19  (sensor recording primary)
  BRANCH B: 1 → 2 → 4 → 7 → 10 → 15 → 20  (sensor recording backup)
  BRANCH C: 1 → 2 → 5 → 8 → 11 → 18  (telemetry archive)
  BRANCH D: 1 → 3 → 6 → 12 → 16  (deflector sensors)
  BRANCH E: 1 → 3 → 6 → 13 → 17  (environmental sensors)
```

### Adjacency List

```
{
  1:  [2, 3],
  2:  [4, 5],
  3:  [6],
  4:  [7],
  5:  [8],
  6:  [12, 13],
  7:  [9, 10],
  8:  [11],
  9:  [14],
  10: [15],
  11: [18],
  12: [16],
  13: [17],
  14: [19],
  15: [20]
}
```

### Current Flow State (with component 9 failed)

```
CURRENT PRESENT (green):
  Nodes: 1, 2, 3, 4, 5, 6, 7, 8, 10, 11, 12, 13, 15, 16, 17, 18, 20

CURRENT ABSENT (red):
  Nodes: 9, 14, 19

  Failed component: 9
  Effect: Branch A dead from node 9 onward.
  All other branches unaffected.
```

### Input Controls

**Widget:** `LCARSButton` (x20)
```
{
  buttons: [
    { id: "probe-1",  label: "PROBE 01", mode: "momentary" },
    { id: "probe-2",  label: "PROBE 02", mode: "momentary" },
    { id: "probe-3",  label: "PROBE 03", mode: "momentary" },
    { id: "probe-4",  label: "PROBE 04", mode: "momentary" },
    { id: "probe-5",  label: "PROBE 05", mode: "momentary" },
    { id: "probe-6",  label: "PROBE 06", mode: "momentary" },
    { id: "probe-7",  label: "PROBE 07", mode: "momentary" },
    { id: "probe-8",  label: "PROBE 08", mode: "momentary" },
    { id: "probe-9",  label: "PROBE 09", mode: "momentary" },
    { id: "probe-10", label: "PROBE 10", mode: "momentary" },
    { id: "probe-11", label: "PROBE 11", mode: "momentary" },
    { id: "probe-12", label: "PROBE 12", mode: "momentary" },
    { id: "probe-13", label: "PROBE 13", mode: "momentary" },
    { id: "probe-14", label: "PROBE 14", mode: "momentary" },
    { id: "probe-15", label: "PROBE 15", mode: "momentary" },
    { id: "probe-16", label: "PROBE 16", mode: "momentary" },
    { id: "probe-17", label: "PROBE 17", mode: "momentary" },
    { id: "probe-18", label: "PROBE 18", mode: "momentary" },
    { id: "probe-19", label: "PROBE 19", mode: "momentary" },
    { id: "probe-20", label: "PROBE 20", mode: "momentary" }
  ],
  layout: "grid-4x5",
  probeResultDisplay: "flash-on-node"
}
```

### Probe Logic

```
When PROBE [n] is pressed:
  - The CircuitTopologyDisplay highlights node [n]
  - If current is present at node [n]:
    → Green flash on the node (800ms)
    → Status line: "NODE [n]: CURRENT DETECTED — [component name]"
  - If current is absent at node [n]:
    → Red flash on the node (800ms)
    → Status line: "NODE [n]: NO CURRENT — DOWNSTREAM OF FAULT"

Probe results are cumulative — the display remembers all probed nodes
(green glow for current, red glow for no current) until the panel is reset.
```

---

## 3. Reference Card

```
EPS CIRCUIT DIAGNOSTIC — OPERATOR REFERENCE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

WHAT YOU SEE:
  An EPS circuit with 20 nodes connected in a tree structure.
  Current flows from the SOURCE (node 1) through the tree toward
  5 destination nodes (16-20). One component has FAILED — current
  stops at that node and everything downstream goes dark.

  Twenty PROBE buttons let you test each node for current.
  Green flash = current is flowing at that node.
  Red flash = no current (you are downstream of the fault).

THE PROBLEM:
  Find the failed component. It is the node where current STOPS —
  the last node upstream that has current, whose downstream child
  does NOT have current.

HOW THE CIRCUIT BRANCHES:
  The circuit is NOT a straight line. It BRANCHES:
  - Node 1 splits into nodes 2 and 3
  - Node 2 splits into nodes 4 and 5
  - Node 7 splits into nodes 9 and 10
  - Node 6 splits into nodes 12 and 13

  The fault affects only ONE branch. Other branches still carry current.

HOW TO FIND THE FAULT:
  1. Probe the source (node 1). Should be GREEN (current present).
  2. Probe a destination node (e.g., node 19). If RED, the fault
     is somewhere on the path from node 1 to node 19.
  3. Use BINARY SEARCH on that path:
     - Probe the middle node on the path.
     - If GREEN, the fault is further downstream. Search the lower half.
     - If RED, the fault is upstream. Search the upper half.
  4. When you find two adjacent nodes where one is GREEN and the
     next is RED — the GREEN node is the failed component.
     (Current enters it but does not exit.)

  CAUTION: The circuit branches. Make sure you are tracing the
  correct path. If a destination node shows GREEN, the fault is
  NOT on that destination's branch.

SUCCESS STATE:
  You can identify the single node where current enters but does not
  exit. That node's ID number is the answer.
```

---

## 4. Novice Solve Path

1. **Start at the extremes:**
   - Probe node 1 (SOURCE): GREEN. Current enters the circuit.
   - Probe node 19: RED. No current at destination. Fault is somewhere on path 1 → 2 → 4 → 7 → 9 → 14 → 19.
   - Probe node 20: RED. Path 1 → 2 → 4 → 7 → 10 → 15 → 20 also dead? Wait...

2. **Check other destinations to narrow the branch:**
   - Probe node 16: GREEN. Branch D is fine (1 → 3 → 6 → 12 → 16).
   - Probe node 17: GREEN. Branch E is fine (1 → 3 → 6 → 13 → 17).
   - Probe node 18: GREEN. Branch C is fine (1 → 2 → 5 → 8 → 11 → 18).

3. **The fault is on the branch serving nodes 19 and 20.** Both are RED. That branch goes through nodes 1, 2, 4, 7, then splits at 7 into 9 (→19) and 10 (→20).

4. **Wait — check node 20 more carefully:**
   - Probe node 10: GREEN. Current reaches node 10.
   - Probe node 15: GREEN. Current reaches node 15.
   - Probe node 20: GREEN. Destination 20 is actually fine.

5. **Revise:** Only node 19 is dead. Path to 19: 1 → 2 → 4 → 7 → 9 → 14 → 19. But 20 is fine, meaning 1, 2, 4, 7 all have current (they feed the 10 → 15 → 20 branch too).

6. **Binary search on the dead branch (7 → 9 → 14 → 19):**
   - Already know node 7 has current (implied by node 10 being GREEN).
   - Probe node 9: RED. No current at node 9.
   - Probe node 7: GREEN (confirming).

7. **Fault identified:** Node 7 has current. Node 9 does not. Node 9 is the direct child of node 7. The failed component is **node 9**.

8. **Verify:** Probe node 14: RED. Probe node 19: RED. Everything downstream of 9 is dead. Everything else is fine.

9. **Answer:** 9.

---

## 5. Expert Solve Path

1. Probe two destinations to find the dead branch:
   - Node 19: RED.
   - Node 18: GREEN. (The fault is not on the 5 → 8 branch.)

2. Node 19 is on branch A (through node 9). Probe node 9: RED.

3. Probe node 7 (parent of 9): GREEN. Also probe node 10 (sibling of 9): GREEN.

4. Node 7 has current, node 9 does not, node 10 does. Failed component = node 9.

5. Answer: 9. Four probes total.

---

## 6. Data Values

### Circuit Node Reference

| Node | Component Name | Branch | Current? |
|------|---------------|--------|----------|
| 1 | EPS Source Junction | root | GREEN |
| 2 | Primary Relay A | A/B/C | GREEN |
| 3 | Primary Relay B | D/E | GREEN |
| 4 | Secondary Relay A | A/B | GREEN |
| 5 | Secondary Relay C | C | GREEN |
| 6 | Secondary Relay B | D/E | GREEN |
| 7 | Tertiary Relay A | A/B | GREEN |
| 8 | Tertiary Relay C | C | GREEN |
| **9** | **Sensor Distribution Relay** | **A** | **RED (FAILED)** |
| 10 | Sensor Backup Relay | B | GREEN |
| 11 | Telemetry Archive Relay | C | GREEN |
| 12 | Deflector Sensor Relay | D | GREEN |
| 13 | Environmental Sensor Relay | E | GREEN |
| 14 | Recording Primary Bus | A | RED (downstream) |
| 15 | Recording Backup Bus | B | GREEN |
| 16 | Deflector Sensor Array | D (dest) | GREEN |
| 17 | Environmental Sensor Array | E (dest) | GREEN |
| 18 | Telemetry Archive Unit | C (dest) | GREEN |
| 19 | Sensor Recording Primary | A (dest) | RED (downstream) |
| 20 | Sensor Recording Backup | B (dest) | GREEN |

### Branch Status Summary

| Branch | Path | Destinations | Status |
|--------|------|-------------|--------|
| A | 1→2→4→7→9→14→19 | Node 19 (Sensor Recording Primary) | DEAD at node 9 |
| B | 1→2→4→7→10→15→20 | Node 20 (Sensor Recording Backup) | LIVE |
| C | 1→2→5→8→11→18 | Node 18 (Telemetry Archive) | LIVE |
| D | 1→3→6→12→16 | Node 16 (Deflector Sensors) | LIVE |
| E | 1→3→6→13→17 | Node 17 (Environmental Sensors) | LIVE |

### Diagnostic Observations

- Only Branch A is dead. One component failure — node 9 (Sensor Distribution Relay).
- Sensor Recording Backup (Branch B, node 20) is LIVE. This means sensor data could theoretically have been recorded through the backup path.
- But sensor recording primary was the active recording system. The backup requires manual activation.
- The backup was never activated during the gap. With the primary recording dead, the sensors ran but their data was not stored.

---

## 7. Narrative Revelation

On identifying node 9 as the failed component, the following log entry appears on the CircuitTopologyDisplay status overlay:

```
EPS FAULT REPORT — GAP +00:18
  Component: NODE 09 — SENSOR DISTRIBUTION RELAY
  Failure mode: OPEN CIRCUIT — relay contacts fused open.
  Effect: Sensor Recording Primary (NODE 19) — unpowered.
  Backup path (NODE 20): Available but not activated.
  Relay inspection note: Contact surfaces show thermal pitting
  consistent with induced overcurrent, not wear failure.
```

---

## 8. Story Layer

**The anomaly:** Component 9 is the Sensor Distribution Relay — the single point in the EPS circuit that feeds the primary sensor recording system. Its failure at GAP +00:18 meant that the ship's sensors continued to operate (they have their own power path through the NAV and TAC circuits), but the sensor DATA was not being recorded. The instruments saw the contact. The recordings did not capture it.

**What this means:** The failure was not random. The relay inspection note says "thermal pitting consistent with induced overcurrent, not wear failure." An induced overcurrent means someone deliberately pushed excess current through the relay to burn it open. The backup recording path (Branch B, through node 10) was available and functional — but it requires manual activation from the Engineering console. Nobody activated it. Nobody reported the failure. The sensor recording went dark and stayed dark for six hours.

**Connection to conspiracy:** This is the mechanism of the cover-up. The sensor data override injected through the Security Computer (R2-02, GAP +00:14) suppressed real-time contact readings on the bridge displays. The EPS relay failure (R2-05, GAP +00:18) ensured no recording was made. Four minutes apart. The order is deliberate: first suppress the displays so nobody sees the contact, then kill the recording so even if someone checked later, there would be nothing to find. The sealed order to wipe the backup instrument logs (the hook from the opening narrative) targets the one remaining copy — the passive buffers that the relay failure could not reach.
