# Puzzle IV — Imperial Age — Test Results

**Puzzle:** Resource Map (5 grids, plot resource dots, dots trace letters)
**Testers:** Sarrett, Miller, Dana
**Answer:** TOWER

---

## Tester 1: Peter Sarrett (Experience Design & Physicality)

### Solve Attempt

Five maps. Each has a 5x5 grid and a list of resource positions. Plot the dots. Read the letter.

This is a drawing puzzle. I grab my pencil.

**Map 1 — Arabia:**
Plotting: (1,1)(1,2)(1,3)(1,4)(1,5) fills the entire top row. Then (2,3)(3,3)(4,3)(5,3) fills the center column from row 2 down.
```
Row 1: * * * * *
Row 2: . . * . .
Row 3: . . * . .
Row 4: . . * . .
Row 5: . . * . .
```
Clear **T**. The top crossbar and the vertical stem.

**Map 2 — Arena:**
Plotting: (1,2)(1,3)(1,4) top center. (2,1)(2,5)(3,1)(3,5)(4,1)(4,5) sides. (5,2)(5,3)(5,4) bottom center.
```
Row 1: . * * * .
Row 2: * . . . *
Row 3: * . . . *
Row 4: * . . . *
Row 5: . * * * .
```
Clear **O**. The stone walls framing makes sense -- it's Arena, the walled map.

**Map 3 — Gold Rush:**
Plotting: (1,1)(1,5)(2,1)(2,5)(3,1)(3,3)(3,5)(4,2)(4,4)
```
Row 1: * . . . *
Row 2: * . . . *
Row 3: * . * . *
Row 4: . * . * .
Row 5: . . . . .
```
The two outer columns descend, the center dot at (3,3) connects them, and (4,2)(4,4) are the descending legs. This is a **W**. The "enormous gold pile at center" IS the center of the W.

**Map 4 — Mediterranean:**
Plotting: (1,1)(1,2)(1,3)(2,1)(3,1)(3,2)(4,1)(5,1)(5,2)(5,3)
```
Row 1: * * * . .
Row 2: * . . . .
Row 3: * * . . .
Row 4: * . . . .
Row 5: * * * . .
```
Three horizontal bars on the left side with a vertical backbone. Clear **E**. The "central sea" on the right side of the map is empty -- thematic.

**Map 5 — Baltic:**
Plotting: (1,1)(1,2)(2,1)(2,3)(3,1)(3,2)(4,1)(4,3)(5,1)(5,4)
```
Row 1: * * . . .
Row 2: * . * . .
Row 3: * * . . .
Row 4: * . * . .
Row 5: * . . * .
```
Vertical left column. Top bump at (1,2), middle bump at (3,2) -- these form the two curves of the R. Then (2,3)(4,3)(5,4) form the diagonal leg descending right. Clear **R**.

T-O-W-E-R = **TOWER**

**Answer: TOWER**

Solve time: ~12 minutes (drawing takes time).

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | The instructions are clear. The grid format is consistent. However, the letter recognition step requires squinting at 5x5 grids, which is low-resolution for letter identification. Some letters (W, R) are harder to see than others (T, O, E). |
| Solvability | 4 | Plotting is mechanical. Letter recognition is the challenge -- on a 5x5 grid, letters are blocky. Most players will get T, O, and E immediately. W and R might take a second look. |
| Elegance | 4 | The mechanism is lovely: scouting reports become a drawing exercise, drawings become letters. The resource descriptions add thematic texture -- you're mapping Arabia, Arena, Gold Rush. The fact that the resources are thematically appropriate for each map (Arena has relics, Gold Rush has central gold, Mediterranean has shore fish) is a beautiful touch. |
| Reading Reward | 3 | Here's the weakness: you don't actually need to know what the resources ARE. You just need their coordinates. A solver could ignore the resource column entirely and plot (Row, Col) pairs. The map descriptions and resource names are flavor, not solving material. |
| Fun | 5 | This is a Dinner Party puzzle. "I plotted resources on a map and the dots spelled a word." That's a great hook. The physical act of drawing is engaging. Each map reveal is a mini-aha. |
| Confirmation | 4 | TOWER is thematic (Towers are a key AoE2 building) and clean. Five letters from five maps. |
| **Total** | **24** | |

### Issues

| Severity | Issue |
|----------|-------|
| Major | Reading Reward is low. The resource names are decorative -- you only need the coordinates. A solver could completely ignore whether a dot is "Gold" or "Berries" and still solve the puzzle. The map descriptions are also unused for solving. |
| Minor | The W (Map 3) is the least legible letter. On a 5x5 grid, W requires the bottom row to be empty and the diagonal legs to be implied from (4,2) and (4,4). A solver unfamiliar with blocky pixel fonts might see an M or something ambiguous. |
| Minor | No interlock between maps. Each map is independently solvable. |

### Suggested Fixes

1. Make resource types matter: e.g., "Plot only gold and stone" or "different resource types trace different letters" -- so the solver must distinguish resource categories.
2. Alternatively, have some coordinates be deliberately missing and force the solver to deduce them from the map description ("Berries always spawn near the Town Center").
3. Consider a 7x5 or 6x5 grid for letters that are hard to render at 5x5 (W, R, M).

---

## Tester 2: Rand Miller (World-as-Puzzle)

### Solve Attempt

This feels like scouting. I'm exploring five maps, marking resources, and the land itself speaks to me.

Map 1: T. Map 2: O. Map 3: W. Map 4: E. Map 5: R.

TOWER.

**Answer: TOWER**

Solve time: ~10 minutes.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | Clean format. Consistent grid size. The resource tables are well-structured. |
| Solvability | 4 | Plotting is straightforward. Letter recognition is 90% unambiguous (W is the hardest). |
| Elegance | 5 | This is the most diegetic puzzle in the set. You ARE a scout marking resources on a map. The letter emerges from the act of exploration. The world IS the puzzle -- the resource placement spells the answer. This is Riven-quality integration of mechanism and world. Each map name (Arabia, Arena, Gold Rush, Mediterranean, Baltic) is a real AoE2 map, and the resource descriptions match their character. |
| Reading Reward | 3 | The resources are correctly themed for each map, but the solver doesn't need to UNDERSTAND the maps to solve. The coordinates alone suffice. |
| Fun | 5 | The drawing is the experience. Each grid completion is a reveal moment. Five mini-ahas building to the final word. |
| Confirmation | 4 | TOWER is clean and thematic. |
| **Total** | **25** | |

### Issues

| Severity | Issue |
|----------|-------|
| Minor | The map descriptions ("Open terrain. Scattered resources. No natural walls.") are atmospheric but unused for solving. In a Riven-style puzzle, the descriptions would contain clues. Here they're flavor text. |
| Minor | The resource types are correctly themed but functionally irrelevant. This is a missed opportunity to make the world-knowledge load-bearing. |

### Suggested Fixes

1. Make the map descriptions functional: "One resource is hidden behind the treeline" -- the solver must determine its position from the map description rather than a coordinate table.
2. Have one map where a resource type is ambiguous and the map name helps disambiguate (e.g., "This resource only appears on water maps").

---

## Tester 3: Dana Young (Craft & Presentation)

### Solve Attempt

The visual design is critical here. Five grids, five tables, five maps. The layout needs to be clean enough that plotting is pleasant, not tedious.

The grids are well-drawn with ASCII art. The coordinate tables are consistent (Row, Col format). The resource names add texture.

Plotting all five: T, O, W, E, R.

**Answer: TOWER**

Solve time: ~11 minutes.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | The presentation is the best in the set. Each map has: flavor text, resource table, blank grid. The format is consistent across all five. The grids have row and column labels. Nothing is ambiguous. |
| Solvability | 4 | Mechanical plotting is simple. Letter recognition on 5x5 grids requires some pattern-recognition skill. The W and R are harder than T, O, E. |
| Elegance | 4 | The mechanism is inherently elegant: dots on a grid form letters. The thematic integration (real AoE2 maps, real resource types) is well-done. Five maps for a five-letter word is the right ratio. |
| Reading Reward | 3 | The resource names and map descriptions are atmospheric but not functionally required. Coordinates alone solve the puzzle. |
| Fun | 5 | This is the most visually satisfying puzzle in the set. The drawing activity is engaging. Each completed grid is a reveal. The cumulative effect of five letters emerging from resource maps is delightful. |
| Confirmation | 4 | TOWER is clean and thematic. Five letters, five maps, no ambiguity. |
| **Total** | **25** | |

### Issues

| Severity | Issue |
|----------|-------|
| Major | Reading Reward is the consensus weakness. The resource names could be replaced with "Dot 1, Dot 2, Dot 3" and the puzzle would still work. The AoE2 content is cosmetic, not structural. |
| Minor | The W letter relies on the solver seeing a W in a pattern that could also read as an upside-down A or M. Adding one more dot (e.g., at (5,1) or (5,5)) would make the W more legible, though it might require adjusting the resource narrative. |

### Suggested Fixes

1. Make resource types structural: assign different symbols to different resources (Gold = filled dot, Stone = open circle, Food = X). Then the letter is only visible when you use the right symbol.
2. Alternatively, have one grid where a resource position is given relative to another resource (not as absolute coordinates), requiring the solver to use the resource table more carefully.

---

## Synthesis

| Dimension | Sarrett | Miller | Dana | Average |
|-----------|---------|--------|------|---------|
| Clarity | 4 | 4 | 5 | 4.3 |
| Solvability | 4 | 4 | 4 | 4.0 |
| Elegance | 4 | 5 | 4 | 4.3 |
| Reading Reward | 3 | 3 | 3 | 3.0 |
| Fun | 5 | 5 | 5 | 5.0 |
| Confirmation | 4 | 4 | 4 | 4.0 |
| **Total** | **24** | **25** | **25** | **24.7** |

### Verdict: PASS (24.7/30)

The most fun puzzle in the set. The drawing mechanic is inherently engaging, the thematic integration is strong, and the five-grid-to-five-letter payoff is satisfying. The critical weakness is Reading Reward (all three testers scored it 3/5): the resource names and map descriptions are cosmetic, not structural. The coordinates alone solve the puzzle. This is fixable without changing the core mechanism.

### Consensus Issues

1. **Reading Reward** (Major): All three testers scored this 3/5. Resource names and map descriptions are decorative, not functional. The puzzle works identically with unlabeled coordinate pairs.
2. **W legibility** (Minor): Two testers flagged the W (Map 3) as the hardest letter to recognize on a 5x5 grid.
3. **Highest fun** (Positive): All three testers scored Fun at 5/5. This is the Dinner Party puzzle of the set.
4. **Strong diegetic quality** (Positive): Miller specifically praised this as the most diegetic puzzle in the set -- "you ARE a scout marking resources."
