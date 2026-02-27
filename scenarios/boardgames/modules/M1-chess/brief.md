# Module Brief — M1: Reconstructed (Chess)

**Assigned to**: The Methodical
**Game**: Chess
**Puzzle title**: Reconstructed
**Answer word**: Author's choice (5-8 letters, thematic, not guessable from topic)

---

## Puzzle Concept

Reconstruct a chess position from a set of overlapping clues, then identify the only legal move — which yields the answer.

## Mechanism

1. Present 8 clues that describe constraints on piece placement:
   - "A bishop controls the long diagonal"
   - "The king is on a light square adjacent to the e-file"
   - etc.
2. Each clue eliminates possible positions. The clues INTERLOCK — solving one constrains others.
3. The unique valid arrangement has exactly one legal move.
4. The notation of that move, combined with extracted letters from the clue-solving process, yields the answer word.

## World Data

Use `world/chess.md` for:
- Piece movements and capture rules
- Board geometry (light/dark squares, file/rank naming)
- Algebraic notation
- Special moves (castling conditions, en passant)

## Constraints

| Constraint | Detail |
|-----------|--------|
| **Riven Standard** | The puzzle IS chess reasoning — piece movement, board geometry, notation |
| **Interlock** | At least 3 clues must cross-reference each other (solving one helps solve another) |
| **No Over-Scaffolding** | Do NOT provide a pre-drawn board. The solver draws their own. |
| **One Aha** | The position is uniquely determined — there's only one arrangement satisfying all clues |
| **Blame the Player** | All clues must be fair and verifiable against the world data |
| **No Deliberate Errors** | Every chess fact must be correct (square colors, piece movements, notation) |
| **Surprise the Answer** | The answer should NOT be CHECKMATE, CHECK, or KING |

## Delivery

- Write `puzzle.md` in your module directory
- Include the puzzle as a solver would see it (no answer key inline)
- Keep working notes in `notes.md` if desired
- Submit your answer word with the puzzle

## Principles Reference

See `toolkit/PRINCIPLES.md` for full principle definitions and tests.
