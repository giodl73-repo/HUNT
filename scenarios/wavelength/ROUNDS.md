# WAVELENGTH — Round Structure

## Architecture

```
Single round — 6 puzzles + 1 meta

   P1 (Track Listing)  ─┐
   P2 (Music Theory)   ─┤
   P3 (Album ID)       ─┤──→  META (The Final Broadcast)
   P4 (Chart Positions) ─┤
   P5 (Lyrics)         ─┤
   P6 (Artist Names)   ─┘
```

## Round: The Long Wave (Vee's Final Broadcast)

One round. Six sets. Each set is a puzzle themed around a different facet of music. Between sets, Vee speaks one sentence — flavor text that contextualizes the puzzle.

**Unlock model:** All 6 puzzles available from the start (parallel). Meta unlocks when solvers choose to attempt it (they need feeder answers to make progress).

**Difficulty curve:**
| Puzzle | Difficulty | Rationale |
|--------|-----------|-----------|
| P1 | 2/5 (Easy) | Track listing extraction — straightforward once the mechanism is identified |
| P2 | 3/5 (Medium) | Music theory encoding — requires knowing or learning note names A-G |
| P3 | 3/5 (Medium) | Album identification — requires deduction from descriptions |
| P4 | 3/5 (Medium) | Chart positions — lookup and convert |
| P5 | 4/5 (Hard) | Lyrics hidden words — requires careful reading and pattern recognition |
| P6 | 4/5 (Hard) | Artist name wordplay — requires lateral thinking |
| META | 4/5 (Hard) | Combines all 6 answer words into the DJ's final message |

**Target solve time:** 2-3 hours for a team of 2-4.

## Puzzle Slots

| ID | Working Title | Type | Answer Length | Domain |
|----|--------------|------|---------------|--------|
| P1 | "Side A" | Track listing extraction | 6 letters | Albums, track listings |
| P2 | "Notation" | Music theory encoding | 5 letters | Note names, notation |
| P3 | "Deep Cuts" | Album identification | 6 letters | Album art, descriptions |
| P4 | "Chart Toppers" | Chart position patterns | 6 letters | Billboard Hot 100 |
| P5 | "Between the Lines" | Lyrics hidden words | 5 letters | Song lyrics |
| P6 | "Name That Band" | Artist name wordplay | 6 letters | Band/artist names |
| META | "Sign Off" | Final broadcast message | Phrase (TBD) | All of the above |

## Numbering

Puzzles are numbered by their position on the DJ's set list: Set 1 through Set 6. No encoded numbering — the radio station frame is structural enough.

## Meta Architecture

The meta takes the 6 answer words from the feeder puzzles. The extraction mechanism will be designed in Stage 5 after answer words are coordinated. Target: a short phrase that serves as Vee's final message.

**The 80% Rule:** The meta should be solvable with 5 of 6 feeder answers (any 5). This means the mechanism must have enough redundancy or pattern-recognition hooks that one missing answer does not block progress.

## Narrator Integration

Each puzzle is introduced by a one-sentence note from Vee — written on the playlist card, as if she scrawled it in the booth during the broadcast. These are not instructions; they are flavor that sets the mood for each set.

Example format:
> *Set 1: "I played these songs in this order for a reason. I always did."*
