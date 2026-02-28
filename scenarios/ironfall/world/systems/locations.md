# IRONFALL World Map & Locations

Source of truth for the 6 regions, their sub-areas, and connections.

---

## World Map Overview

IRONFALL's world is a single continent with 6 regions arranged in a specific topology. The player starts in Ashveil Forest and must traverse regions in a mostly linear path, though shortcuts and secret passages create alternate routes.

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

---

## Region Details

### Region 1: Ashveil Forest
| Sub-area | Type | Accessible from | Notable features |
|----------|------|----------------|-----------------|
| Ashveil Village | Town | World map | Starting town, Forge, Inn, Shop |
| Ashveil Woods | Dungeon | Ashveil Village | 3 floors, tutorial dungeon |
| Whispering Glade | Secret | Ashveil Woods B2 (hidden wall) | Lore Tablet #1, Moonlight Shard |
| World Map Exit N | Connection | → Glacial Reach | Requires Forest Key |
| World Map Exit E | Connection | → Ember Wastes | Requires Ashveil completion |

### Region 2: Glacial Reach
| Sub-area | Type | Accessible from | Notable features |
|----------|------|----------------|-----------------|
| Frosthold | Town | World map | Ice equipment shop, NPC: Elara |
| Frozen Caverns | Dungeon | Frosthold | 4 floors, sliding ice puzzles |
| Crystalline Chamber | Secret | Frozen Caverns B3 (ice pillar puzzle) | Lore Tablet #2, #3 |
| World Map Exit S | Connection | → Ashveil Forest | |
| World Map Exit N | Connection | → Stormspire Peaks | Requires Ice Key |

### Region 3: Stormspire Peaks
| Sub-area | Type | Accessible from | Notable features |
|----------|------|----------------|-----------------|
| Windrest Camp | Town | World map | NPC: Kael, Thunder equipment |
| Storm Tower | Dungeon | Windrest Camp | 5 floors, lightning timing puzzles |
| Eye of the Storm | Secret | Storm Tower 5F (survive 10 turns) | Lore Tablet #4, #5, #6 |
| World Map Exit S-W | Connection | → Glacial Reach | |
| World Map Exit S-E | Connection | → Sunken Vale | Requires Storm Key |

### Region 4: The Iron Citadel
| Sub-area | Type | Accessible from | Notable features |
|----------|------|----------------|-----------------|
| Iron Gate | Town (ruins) | World map | Final save point, NPC: Morimoto's ghost |
| Citadel Interior | Dungeon | Iron Gate | 7 floors, final dungeon |
| Morimoto's Lab | Secret | Citadel B7 (Developer's Key) | Secret boss fight, Lore Tablet #11, #12 |
| World Map Exit N | Connection | → Ember Wastes | |
| Throne Room | Boss | Citadel 7F | Final boss: The Iron King |

### Region 5: Sunken Vale
| Sub-area | Type | Accessible from | Notable features |
|----------|------|----------------|-----------------|
| Tidepool Village | Town | World map | NPC: Maren, water equipment |
| Drowned Temple | Dungeon | Tidepool Village | 4 floors, water level puzzles |
| Abyssal Grotto | Secret | Drowned Temple B4 (drain sequence) | Lore Tablet #7, #8 |
| World Map Exit N | Connection | → Stormspire Peaks | |
| World Map Exit S | Connection | → Ember Wastes | Requires Tide Key |

### Region 6: Ember Wastes
| Sub-area | Type | Accessible from | Notable features |
|----------|------|----------------|-----------------|
| Cinder Town | Town | World map | NPC: Rook, fire equipment |
| Volcanic Depths | Dungeon | Cinder Town | 5 floors, lava flow puzzles |
| Forge Eternal | Secret | Volcanic Depths B5 (extinguish all torches) | Lore Tablet #9, #10, Master Forge |
| World Map Exit W | Connection | → Ashveil Forest | |
| World Map Exit N | Connection | → Sunken Vale | |
| World Map Exit S | Connection | → Iron Citadel | Requires Ember Key |

---

## Connection Matrix

| From \ To | Ashveil | Glacial | Stormspire | Iron Citadel | Sunken Vale | Ember Wastes |
|-----------|---------|---------|------------|-------------|-------------|-------------|
| Ashveil | -- | Direct | -- | -- | -- | Direct |
| Glacial | Direct | -- | Direct | -- | -- | -- |
| Stormspire | -- | Direct | -- | -- | Direct | -- |
| Iron Citadel | -- | -- | -- | -- | -- | Direct |
| Sunken Vale | -- | -- | Direct | -- | -- | Direct |
| Ember Wastes | Direct | -- | -- | Direct | Direct | -- |

Total connections: 8 (bidirectional). The graph is connected. Shortest path from Ashveil to Iron Citadel: Ashveil → Ember Wastes → Iron Citadel (2 steps). Story path: Ashveil → Glacial → Stormspire → Sunken Vale → Ember Wastes → Iron Citadel (5 steps).

---

## Lore Tablets

12 tablets scattered across secret areas. Each contains a fragment of IRONFALL's backstory.

| # | Location | Content (first line) |
|---|----------|---------------------|
| 01 | Whispering Glade | "Before the Iron King, there was only the forest..." |
| 02 | Crystalline Chamber | "The ice remembers what the living forget..." |
| 03 | Crystalline Chamber | "Elara crossed the frozen sea alone..." |
| 04 | Eye of the Storm | "Lightning split the mountain and the citadel rose..." |
| 05 | Eye of the Storm | "Kael watched the tower fall from the ridge..." |
| 06 | Eye of the Storm | "The storm is not weather. It is memory." |
| 07 | Abyssal Grotto | "Beneath the waves, the old gods sleep..." |
| 08 | Abyssal Grotto | "Maren dove deeper than any before her..." |
| 09 | Forge Eternal | "The first sword was forged in starfire..." |
| 10 | Forge Eternal | "Rook fed the flames for forty years..." |
| 11 | Morimoto's Lab | "I hid the ending where no one would look." |
| 12 | Morimoto's Lab | "The code is not a cheat. It is a promise." |

---

## Secret Areas Access

| Secret Area | Region | How to Access | Required Item |
|-------------|--------|--------------|---------------|
| Whispering Glade | 1 | Hidden wall in Ashveil Woods B2 | None (discoverable) |
| Crystalline Chamber | 2 | Solve ice pillar puzzle in Frozen Caverns B3 | None (puzzle) |
| Eye of the Storm | 3 | Survive 10 turns in Storm Tower 5F boss room | None (endurance) |
| Morimoto's Lab | 4 | Use Developer's Key on hidden door in Citadel B7 | Developer's Key |
| Abyssal Grotto | 5 | Activate drain sequence in Drowned Temple B4 | Tide Key |
| Forge Eternal | 6 | Extinguish all 5 torches in Volcanic Depths B5 | None (puzzle) |

---

## Navigation Affordances

- The connection matrix is asymmetric in narrative (you traverse the world in order) but symmetric mechanically (you can backtrack freely).
- The graph has exactly one cycle: Ashveil → Glacial → Stormspire → Sunken Vale → Ember Wastes → Ashveil.
- The Iron Citadel is a dead end connected only to Ember Wastes.
- Shortest paths between all region pairs are unique (no ties), enabling unambiguous path-finding puzzles.
