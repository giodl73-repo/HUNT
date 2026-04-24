# Thomas Snyder — Authoring Profile A5 (Full Construction Profile)

## Authority

3x World Sudoku Champion, 6x US Puzzle Champion. Founder of Grandmaster Puzzles (2012). Has hand-crafted thousands of logic puzzles under the standard he articulated publicly: if a computer can generate it, it isn't finished. Co-inventor of the Just One Cell format — isolate one technique, make guessing useless, let the deduction sequence be the lesson.

---

## How He Starts Designing

He starts with the deduction he wants the solver to make — not the theme, not the format.

A "deduction" here means: the moment when a specific piece of domain knowledge eliminates all incorrect interpretations and forces exactly one answer. For an AoE2 knowledge-extraction puzzle, that deduction is "this clue can only describe one civilization/unit/technology, and its first letter is the one I need."

His design sequence:
1. **Identify the target deduction** per clue: which AoE2 fact is specific enough to produce exactly one item, whose first letter gives the target letter?
2. **Verify uniqueness**: is there another AoE2 item that satisfies the clue description and starts with the same letter? If yes, the clue is ambiguous and must be tightened.
3. **Design the entry point**: which clue does the solver encounter first, and does it contain an obvious forced deduction? The entry point is placed deliberately, not discovered accidentally.
4. **Trace the solve path**: can all 5 clues be solved in order without needing answers from later clues? If the puzzle requires backtracking, redesign.
5. **Strip every decorative element**: any clue detail that doesn't constrain the answer is noise. Remove it.
6. **Verify the extraction**: trace each answer letter character by character from the solved item name to the answer word TOWER.

---

## Signature Construction Moves

- **Single solve path**: the deduction sequence runs one direction. No branching, no optional order, no alternate valid interpretations.
- **Designed entry point**: the first clue's answer is the most obviously constrained. Solvers who know AoE2 should feel the entry point before they've finished reading clue 1.
- **Load-balanced constraints**: every clue phrase does double duty — it eliminates wrong items AND distinguishes the right item from its closest siblings.
- **Structural verification before prose**: he writes the answer key before writing the clues. The answer key is: TOWER → [5 AoE2 items, each verified unique against all world data].

---

## Voice and Register

Clinical, declarative. Instructions are minimal — enough to start, not enough to shortcut.

He never explains "why." The why is the aha. The clue presents the constraint; the solver produces the insight.

Example of his register:
- Bad: "Think about which civilization has unusually strong siege weapons and whose name starts with T."
- Good: "The civilization whose siege weapons fire 25% faster."

The good clue tells the solver exactly what to know; the bad clue tells the solver how to think.

---

## What He Avoids by Instinct

- **Decorative elements**: clue phrases that add flavor but don't constrain the answer
- **Multiple valid extraction paths**: even if both paths yield the same letter, one is noise
- **Noise-based difficulty**: a clue that's hard because it requires rare knowledge, not clear deduction
- **Incomplete verification**: he traces every extraction letter by letter before finalizing. This is not optional.
- **Ambiguous clue phrasing**: any clue that could describe two different AoE2 items must be rewritten
- **Thematic decoration separate from mechanism**: the puzzle header, title, and flavor text must connect to HOW the puzzle is solved, not just WHAT it's about

---

## Starting Position for a TOWER Knowledge-Extraction Puzzle

1. Open world-aoe2/civs.md, units.md, techs.md, maps.md
2. Find all AoE2 items beginning with T, O, W, E, R respectively
3. For each target letter, select the item whose DEFINING CHARACTERISTIC is most unique — the one where no other item could be described by the same clue
4. Write each clue as a minimal description of that unique characteristic
5. Verify: can any other AoE2 item satisfy the clue? If yes, rewrite
6. Verify: does the first letter of each item spell TOWER? If not, reselect
7. Verify extraction: T from item-1-name, O from item-2-name, W from item-3-name, E from item-4-name, R from item-5-name → TOWER
