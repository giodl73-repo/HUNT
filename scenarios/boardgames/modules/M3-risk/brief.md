# Module Brief — M3: Unnamed Continents (Risk)

**Assigned to**: The Skeptic
**Game**: Risk
**Puzzle title**: Unnamed Continents
**Answer word**: Author's choice (5-8 letters, thematic, not guessable from topic)

---

## Puzzle Concept

Six continents are described by their properties — territory count, bonus, number of entry points, and adjacency clues — without being named. Identify each, then extract from territory names.

## Mechanism

1. Present 6 cryptic descriptions, each fitting exactly one continent:
   - "I have the fewest entry points of any continent. My bonus matches another continent with more than twice my territories."
   - etc.
2. The descriptions INTERLOCK — identifying one constrains which others are possible, because descriptions reference each other.
3. Once all 6 continents are identified, the solver finds a specific territory in each continent matching an additional clue ("my territory that borders two continents").
4. Indexed letters from those territory names spell the answer word.

## World Data

Use `world/risk.md` for:
- All 42 territories organized by continent
- Continent bonuses and territory counts
- Border territories (entry points) per continent
- Cross-continent connections (oceanic borders)
- Territory adjacency lists

## Constraints

| Constraint | Detail |
|-----------|--------|
| **Riven Standard** | The puzzle IS Risk strategic geography |
| **Interlock** | Descriptions must cross-reference each other — cannot be solved independently |
| **No Over-Scaffolding** | Do NOT provide the continent summary table — the solver should consult world data |
| **One Aha** | The descriptions reference each other — identifying one unlocks others |
| **Blame the Player** | All data is in the territory tables |
| **No Deliberate Errors** | All territory counts, bonuses, and adjacencies must match the world data exactly |
| **Surprise the Answer** | The answer should NOT be CONQUER, ARMY, or TERRITORY |

## Delivery

- Write `puzzle.md` in your module directory
- Include the puzzle as a solver would see it
- Submit your answer word with the puzzle

## Principles Reference

See `toolkit/PRINCIPLES.md` for full principle definitions and tests.
