# Module Brief — M4: Patient Zero (Pandemic)

**Assigned to**: The Social
**Game**: Pandemic
**Puzzle title**: Patient Zero
**Answer word**: Author's choice (5-8 letters, thematic, not guessable from topic)

---

## Puzzle Concept

Given the current disease state of 8 cities (cube counts), the infection discard pile, and the knowledge that exactly 2 epidemics have occurred, trace the infection history backward to determine which city was infected first.

## Mechanism

1. Present a board snapshot: 8 cities with their current cube counts (some at 3, some at 1-2).
2. Show the infection discard pile (ordered — top to bottom).
3. State that exactly 2 epidemics have occurred.
4. The solver works backward: the Intensify mechanic (reshuffle discard, place on top) creates recoverable layers.
5. By analyzing which cities have 3 cubes (epidemic bottom-card hits), which were re-infected (post-Intensify), and the layered structure of the discard pile, the solver deduces the original infection order.
6. The first city infected — Patient Zero — combined with extraction from the infection timeline, yields the answer word.

## World Data

Use `world/pandemic.md` for:
- Infection rules and infection rate track
- Epidemic mechanics (Increase, Infect, Intensify)
- Outbreak rules and chain outbreaks
- City list by disease color
- Board geography and connections

## Constraints

| Constraint | Detail |
|-----------|--------|
| **Riven Standard** | The puzzle IS Pandemic infection-deck forensics |
| **Solving = Proving Understanding** | Must genuinely understand the Intensify shuffle to solve |
| **One Aha** | The discard pile has stratigraphy — layers from the Intensify reshuffles |
| **Interlock** | Epidemic timing constrains which cities could have been infected when |
| **Dinner Party Test** | "I forensically reconstructed a disease outbreak" |
| **No Over-Scaffolding** | No step-by-step timeline worksheet |
| **Blame the Player** | All mechanics are in the world data |
| **No Deliberate Errors** | All Pandemic rules must be exactly correct |
| **Surprise the Answer** | The answer should NOT be VIRUS, DISEASE, or PANDEMIC |

## Delivery

- Write `puzzle.md` in your module directory
- Include the puzzle as a solver would see it
- Submit your answer word with the puzzle

## Principles Reference

See `toolkit/PRINCIPLES.md` for full principle definitions and tests.
