# Module Brief — M5: Spymaster (Codenames)

**Assigned to**: The Lurker
**Game**: Codenames
**Puzzle title**: Spymaster
**Answer word**: Author's choice (5-8 letters, thematic, not guessable from topic)

---

## Puzzle Concept

You ARE the spymaster. Given a 5x5 grid of words and a key card, find the single best one-word clue that connects exactly 3 of your team's words while avoiding the assassin.

## Mechanism

1. Present a full 25-word grid with the key card (showing Blue team = 9, Red = 8, Bystander = 7, Assassin = 1).
2. The solver plays spymaster for the Blue team.
3. They must find a one-word clue that:
   - Links exactly 3 of their 9 team words
   - Is unambiguous — the 3 connected words must be clearly the best matches
   - Does NOT relate to the assassin word
4. The clue word itself IS the answer.

## World Data

Use `world/codenames.md` for:
- Grid layout (5x5, 25 words)
- Key card structure (9/8/7/1 distribution)
- Clue rules (one word, relates to meaning, not spelling/position)
- Guessing rules (number + 1 maximum guesses)
- Assassin avoidance (touching assassin = instant loss)

## Constraints

| Constraint | Detail |
|-----------|--------|
| **Riven Standard** | The puzzle IS spymastering — finding the connecting word |
| **One Aha** | The moment the solver sees the unexpected thematic link between 3 seemingly unrelated words |
| **Surprise the Answer** | The connecting word should be non-obvious (not the first word you'd think of) |
| **Dinner Party Test** | "I was the Codenames spymaster and the answer was my clue" |
| **Blame the Player** | The grid is fully visible; the connection is real and fair |
| **No Deliberate Errors** | All clue rules must be correctly stated |
| **No Over-Scaffolding** | Do NOT sort words by team or pre-group them |

## Delivery

- Write `puzzle.md` in your module directory
- Include the puzzle as a solver would see it
- Submit your answer word with the puzzle

## Principles Reference

See `toolkit/PRINCIPLES.md` for full principle definitions and tests.
