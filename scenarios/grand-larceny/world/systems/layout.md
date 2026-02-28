# Hotel Layout — The Grand Hotel

## Overview

Three floors plus basement. Art Deco landmark. The building is roughly rectangular, oriented north-south. Main entrance faces east onto Grand Avenue.

---

## Basement Level (B)

| Room | Code | Position | Notes |
|------|------|----------|-------|
| Wine Cellar | B01 | NW corner | Temperature-controlled. Staff-only access. |
| Laundry | B02 | North center | Industrial equipment. Staff entrance from alley. |
| Boiler Room | B03 | NE corner | Maintenance access. Connects to service shaft. |
| Storage | B04 | South center | Hotel archives, spare furniture. |
| Staff Corridor | B-HALL | East-west | Connects all basement rooms. Stair at east end to kitchen (1F). |
| Service Elevator | B-ELEV | Center | Goes B to 3F. Key-operated. Staff only. |

## First Floor (1F) — Public Spaces

| Room | Code | Position | Notes |
|------|------|----------|-------|
| Grand Lobby | 100 | Center east | Main entrance. Reception desk. Marble floor. |
| Restaurant Aurore | 101 | SE corner | 40 seats. Open for gala dinner 7:00-9:00 PM. |
| The Brass Bar | 102 | NE corner | Cocktails. Open throughout gala. |
| Kitchen | 103 | North center | Staff only. Connects to B-HALL staircase. |
| Cloakroom | 104 | South of lobby | Staffed during gala. |
| Telephone Room | 105 | SW corner | 3 booths. Guest long-distance calls. |
| Manager's Office | 106 | NW corner | Mr. Hargrove's office. Locked after 6 PM. |
| Main Staircase | 1F-STAIR | Center west | Grand staircase to 2F and 3F. |
| Service Elevator | 1F-ELEV | Center | Same shaft as B-ELEV. |
| East Corridor | 1F-EAST | East wing | Connects lobby to bar and restaurant. |
| West Corridor | 1F-WEST | West wing | Connects lobby to telephone room and manager's office. |

## Second Floor (2F) — Gallery and Guest Rooms

| Room | Code | Position | Notes |
|------|------|----------|-------|
| Sargent Gallery | 200 | Center east | Where "Lady in Vermillion" hung. Open during gala 7:30-10:00 PM. |
| Reading Room | 201 | NE corner | Quiet room. Unlocked. |
| Room 202 | 202 | North center | Guest room — Mrs. Ashworth |
| Room 203 | 203 | NW corner | Guest room — Mr. Fontaine |
| Room 204 | 204 | SW corner | Guest room — Mr. Kessler |
| Room 205 | 205 | South center | Guest room — Miss Delacroix |
| Linen Closet | 206 | SE corner | Staff access. Contains maintenance supplies. |
| Second Floor Hall | 2F-HALL | U-shaped | Connects all 2F rooms. Access from main staircase. |
| Service Elevator | 2F-ELEV | Center | Same shaft. |

## Third Floor (3F) — Suites and Roof Access

| Room | Code | Position | Notes |
|------|------|----------|-------|
| Penthouse Suite | 300 | North half | Occupied by hotel owner, Mr. Hargrove. |
| Suite 301 | 301 | SW corner | VIP suite — empty night of gala. |
| Suite 302 | 302 | SE corner | VIP suite — Senator Morgan (not a suspect). |
| Roof Access | 303 | NE corner | Locked door to flat roof. Fire escape on north face. |
| Third Floor Hall | 3F-HALL | Central corridor | Short hallway, 3 doors. |
| Service Elevator | 3F-ELEV | Center | Top of shaft. |

---

## Connections and Paths

### Vertical connections
- **Main staircase** (1F-STAIR): 1F lobby ↔ 2F hall ↔ 3F hall
- **Service elevator** (B-ELEV through 3F-ELEV): B ↔ 1F ↔ 2F ↔ 3F. Key-operated. Only staff have keys.
- **Kitchen staircase**: 1F kitchen (103) ↔ B staff corridor (B-HALL). Narrow, unlit.
- **Fire escape**: External. North face. 3F roof (303) ↔ ground level alley.

### Key adjacencies (doors or direct connections)
- 100 (lobby) → 1F-EAST → 101 (restaurant), 102 (bar)
- 100 (lobby) → 1F-WEST → 105 (telephone), 106 (manager)
- 100 (lobby) → 104 (cloakroom)
- 100 (lobby) → 1F-STAIR → 2F-HALL
- 103 (kitchen) → 1F-EAST (via service door)
- 103 (kitchen) → B-HALL (via staircase)
- 2F-HALL → 200, 201, 202, 203, 204, 205, 206
- 200 (gallery) → 206 (linen closet) — connecting door, usually locked
- 3F-HALL → 300, 301, 302, 303

### The path that matters (P3 design)

The thief's route through the hotel can be traced on the floor plan. The rooms visited in order, when their room codes are read, encode the answer word LOCATION. Specifically:

**Path:** The thief entered via the fire escape (303), took the service elevator (3F-ELEV) down to 2F, crossed to the gallery (200), exited through the linen closet (206), took the service elevator (2F-ELEV) down to B, went through storage (B04), and exited via the staff corridor to the laundry (B02) and out the alley door.

The rooms on the path, in order: 303 → 3F-ELEV → 200 → 206 → 2F-ELEV → B04 → B02

The extraction mechanism for P3 uses the first letter of each room NAME on the path:
- **R**oof Access → L? No...

[NOTE: The extraction mechanism is defined in the puzzle brief. The spatial path is designed here; the letter extraction is coordinated in PUZZLES.md during Stage 4.]

---

## Floor Plan Design Notes (for P3)

The floor plan should be drawn in architectural drafting style:
- Clean lines, labeled rooms
- North arrow, scale bar
- Three separate floor diagrams (B, 1F, 2F, 3F) arranged vertically on the page
- Service elevator shaft marked on each floor
- Fire escape marked on 3F
- The connecting door between gallery (200) and linen closet (206) should be shown but not labeled as a passage — the solver discovers this

The hotel key card prop (hotel-key) has a room number on it that indicates the starting point of the path. The solver uses the key card + floor plan + clues from the police report to trace the thief's route.
