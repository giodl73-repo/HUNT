# Rounds — Game Night

**Stage 3: STRUCTURE**

---

## Round Structure

One round. Five puzzles. One meta. Each puzzle is one board game, one module, one author.

---

## The Host's Opening

*The board is out. The pieces are sorted. Mostly. Five games, five puzzles. You don't play them tonight. Tonight you solve them. The snacks are on the counter. The timer is running. Nobody asked for a timer but it felt right.*

---

## Round 1: Game Night

| Slot | Game | Module | Author | Difficulty | Solve time (target) |
|------|------|--------|--------|------------|-------------------|
| ♟ Chess | Chess | M1 | The Professor | 3 | 20-30 min |
| ⬡ Settlers | Settlers of Catan | M2 | The Speedrunner | 2 | 15-20 min |
| 🎲 Risk | Risk | M3 | The Rebel | 2 | 15-20 min |
| 🦠 Pandemic | Pandemic | M4 | The Collaborator | 3 | 20-30 min |
| 🔍 Codenames | Codenames | M5 | The Silent One | 2 | 15-20 min |
| ★ Meta | All five | M6 | Admin | 3 | 20-30 min |

**Total target solve time: 2-3 hours** (including reading, discussion, wrong turns).

---

## Ordering

No fixed solve order. The Host sets all five games out at once. Solvers pick their own order. This is a game night — you play what catches your eye.

*Five boxes on the table. You could start anywhere. The chess set looks serious. The Catan box has a dent in it. Someone spilled something on the Risk map. The Pandemic box is still sealed — nobody wanted to open it again. The Codenames grid is already laid out. Your call.*

---

## Difficulty Calibration

| Level | Meaning | Target audience |
|-------|---------|-----------------|
| 1 | Warm-up — almost no puzzle experience needed | Non-puzzlers |
| 2 | Approachable — casual board game knowledge + light deduction | Casual gamers |
| 3 | Moderate — requires real engagement with the game's mechanics | Serious board gamers |
| 4 | Hard — expert knowledge or multi-step deduction | Puzzle hunt veterans |
| 5 | Brutal — competition-level | MIT Mystery Hunt |

**This hunt targets 2-3.** Casual board gamers should be able to solve everything. Serious gamers should feel rewarded. Nobody should need to have played competitively.

---

## Narrator: The Host

**Voice**: Dry. Organized. Slightly anxious.
**Tone**: Present tense. Short sentences. No exclamation marks. Addresses "you" directly.
**Anxiety source**: Worried the games won't be fun enough. Worried someone will flip the board. Worried the snacks will run out before the puzzles do.

### Voice Examples

*The rules are in the box. Most of them. Some of them fell behind the shelf last time. You'll manage.*

*This one requires dice. They're in the bag. If you can't find the bag, check the couch cushions. There's always dice in the couch cushions.*

*Someone already opened the Codenames box and sorted the cards. That someone was me. At 2 AM. It felt productive at the time.*

### What the Host Does NOT Do

- Does not explain the games (the solver is assumed to know them)
- Does not give hints (too anxious to be helpful)
- Does not comment on the meta until all five are done
- Does not use exclamation marks (anxiety precludes enthusiasm)

---

## Module Independence

Each puzzle is self-contained. The solver does not need to solve Chess before Pandemic. The author of M3-Risk does not need to read M1-Chess. The only dependency is:

```
M1 ──┐
M2 ──┤
M3 ──┼──→ M6 (Meta)
M4 ──┤
M5 ──┘
```

The meta (M6) cannot be designed until all 5 answer words are locked. This is intentional — it tests Goal G5 (can the admin build a meta from 5 uncoordinated answers?).

---

## Meta: TBD

The meta mechanism is designed AFTER all 5 authors submit their puzzles and answer words are locked. The admin (M6 author) receives:

1. The 5 answer words (chosen independently by each author)
2. The 5 puzzles (for thematic integration)
3. The constraint: the meta answer must be board-game-related

The admin must find a mechanism that:
- Uses all 5 answer words as inputs
- Produces a single final answer
- Feels like a natural capstone to the hunt
- Satisfies the 80% Rule (solvable with 4 of 5 answers)

*There's a sixth box on the table. You hadn't noticed it before. It doesn't have a label. It doesn't have rules. It has five blank spaces and a question mark. The Host is watching you from the kitchen. Pretending not to.*

---

## Answer Word Constraints

Each author picks their own answer word. Constraints:

1. Must be a real English word (no proper nouns)
2. Should be thematically connected to their game
3. 5-8 letters preferred (for meta flexibility)
4. No coordination between authors (this is the realistic case)
5. Author submits answer word with their puzzle draft

The admin will work with whatever 5 words arrive. That's the test.
