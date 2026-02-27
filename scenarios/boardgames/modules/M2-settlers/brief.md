# Module Brief — M2: Market Day (Settlers of Catan)

**Assigned to**: The Speedster
**Game**: Settlers of Catan
**Puzzle title**: Market Day
**Answer word**: Author's choice (5-8 letters, thematic, not guessable from topic)

---

## Puzzle Concept

Given a specific hand of resource cards and a sequence of trades/builds, determine what the player built last — which names the answer.

## Mechanism

1. The solver starts with a known hand of resources (e.g., "3 Brick, 2 Lumber, 1 Ore, 2 Grain, 3 Wool").
2. A sequence of 4-5 actions is described (build road, trade at port, build settlement, draw dev card...).
3. The solver must track resources through each action, using building costs from the world data.
4. One step requires DEDUCTION — figuring out which port the player used (from the trade ratio).
5. After all actions, the remaining resources can only build one thing. That structure (or its thematic name) yields the answer word.

## World Data

Use `world/settlers.md` for:
- Building costs (Road, Settlement, City, Development Card)
- Trading ratios (4:1 default, 3:1 generic port, 2:1 specific port)
- Resource types and terrain associations
- Development card types

## Constraints

| Constraint | Detail |
|-----------|--------|
| **Riven Standard** | The puzzle IS Settlers resource management |
| **No Computation Without Deduction** | At least one step requires inferring missing info (which port?) |
| **Interlock** | Each transaction depends on the previous — you must track sequentially |
| **No Over-Scaffolding** | Do NOT provide a resource tracking spreadsheet |
| **One Aha** | The resource math is tight — only one build is possible at the end |
| **Blame the Player** | All building costs and trade ratios are in the world data |
| **No Deliberate Errors** | All resource costs must be exactly correct |
| **Surprise the Answer** | The answer should NOT be SETTLER, CATAN, or RESOURCE |

## Delivery

- Write `puzzle.md` in your module directory
- Include the puzzle as a solver would see it
- Submit your answer word with the puzzle

## Principles Reference

See `toolkit/PRINCIPLES.md` for full principle definitions and tests.
