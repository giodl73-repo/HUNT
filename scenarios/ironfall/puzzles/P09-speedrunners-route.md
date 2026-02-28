# P09 — The Speedrunner's Route

*Author: The Lurker*

---

> SpeedQueen timed every segment. The route is the only one that works. The times are the message.

---

## ironfall-archive.net/forum/threads/fastest-route-iron-citadel

**SpeedQueen (2001):** "What is the fastest route from Ashveil Forest to the Iron Citadel that allows you to fight Morimoto's Shadow?"

**Requirements:**
- Start at Ashveil Forest.
- End at Iron Citadel, Morimoto's Lab (Citadel B7).
- Each region-to-region connection requires that region's transit key (obtained by clearing the region's dungeon).
- The Warden's Key (dropped by Peak Warden in Stormspire Peaks) is needed to open the door to Citadel B7.
- Minimize region transitions.

---

## The World Map

```
                    ┌─────────────────┐
                    │  STORMSPIRE      │
                    │  PEAKS           │
                    │  (Region 3)      │
                    └────┬───────┬────┘
                         │       │
              ┌──────────┘       └──────────┐
              │                              │
    ┌─────────┴─────────┐        ┌──────────┴────────┐
    │  GLACIAL REACH    │        │  SUNKEN VALE      │
    │  (Region 2)       │        │  (Region 5)       │
    └─────────┬─────────┘        └──────────┬────────┘
              │                              │
    ┌─────────┴─────────┐        ┌──────────┴────────┐
    │  ASHVEIL FOREST   │────────│  EMBER WASTES     │
    │  (Region 1)       │        │  (Region 6)       │
    └───────────────────┘        └──────────┬────────┘
                                            │
                                 ┌──────────┴────────┐
                                 │  IRON CITADEL     │
                                 │  (Region 4)       │
                                 └───────────────────┘
```

**Connections:** Ashveil↔Glacial, Glacial↔Stormspire, Stormspire↔Sunken, Sunken↔Ember, Ember↔Ashveil, Ember↔Iron Citadel.

---

## Dependency Analysis

The naive shortest path from Ashveil to Iron Citadel is: Ashveil → Ember → Iron Citadel (2 transitions, using the direct Ashveil-Ember connection).

This fails. The Warden's Key is obtained in Stormspire. Without visiting Stormspire, the solver cannot open Citadel B7.

**Check whether reaching Stormspire then shortcutting to Iron Citadel saves transitions.**

Route A: Ashveil → Glacial → Stormspire → (backtrack) → Glacial → Ashveil → Ember → Iron Citadel = 6 transitions. Backtracking adds cost.

Route B: Ashveil → Glacial → Stormspire → Sunken → Ember → Iron Citadel = 5 transitions. The story path. No backtracking.

Route C: Ashveil → Ember → Sunken → Stormspire → Glacial → Ashveil → Ember → Iron Citadel = 7 transitions. Even worse.

**Route B is optimal: 5 transitions.**

No route can do better. Stormspire must be visited. Stormspire connects only to Glacial and Sunken. Reaching Stormspire from Ashveil requires at least 2 transitions (via Glacial). Reaching Iron Citadel from Stormspire requires at least 3 transitions (Stormspire → Sunken → Ember → Iron Citadel). Total minimum: 2 + 3 = 5.

---

## SpeedQueen's Segment Times

SpeedQueen recorded the optimal run. Five transitions, five timed segments.

```
╔══════════════════════════════════════════════════════════╗
║  SPEEDRUN LOG — SpeedQueen — June 8, 2001               ║
╠══════════════════════════════════════════════════════════╣
║                                                          ║
║  Segment 1:  Ashveil Forest  →  Glacial Reach            ║
║              Time:  0:07  (7 minutes)                    ║
║                                                          ║
║  Segment 2:  Glacial Reach  →  Stormspire Peaks          ║
║              Time:  0:12  (12 minutes)                   ║
║                                                          ║
║  Segment 3:  Stormspire Peaks  →  Sunken Vale            ║
║              Time:  0:05  (5 minutes)                    ║
║                                                          ║
║  Segment 4:  Sunken Vale  →  Ember Wastes                ║
║              Time:  0:01  (1 minute)                     ║
║                                                          ║
║  Segment 5:  Ember Wastes  →  Iron Citadel               ║
║              Time:  0:13  (13 minutes)                   ║
║                                                          ║
║  Total:  0:38  (38 minutes)                              ║
║                                                          ║
║  Note: "The times are not random. Convert them.          ║
║   1=A, 2=B, ... 26=Z. Read the segments."               ║
║                                                          ║
╚══════════════════════════════════════════════════════════╝
```

---

## Solution

### Step 1: Determine the optimal route

The solver must confirm that Route B (Ashveil → Glacial → Stormspire → Sunken → Ember → Iron Citadel) is optimal by:
- Recognizing that Stormspire is required (Warden's Key).
- Verifying that no shortcut avoids the 5-transition minimum.
- Confirming that the Ashveil → Ember shortcut does not help because you must still reach Stormspire.

### Step 2: Convert segment times

SpeedQueen states: "1=A, 2=B, ... 26=Z."

| Segment | Minutes | A1Z26 |
|---------|---------|-------|
| 1 | 7 | G |
| 2 | 12 | L |
| 3 | 5 | E |
| 4 | 1 | A |
| 5 | 13 | M |

**Answer: GLEAM** (ROT13: TYRNZ)

### Extraction Verification

Answer: GLEAM. Puzzle slot: Act II, position 4. Super-meta extracts position 4: GLEAM[4] = A. A in ROT13 = N. Matches the extraction table.

---

*The world map is in the archive. The connections do not change.*
