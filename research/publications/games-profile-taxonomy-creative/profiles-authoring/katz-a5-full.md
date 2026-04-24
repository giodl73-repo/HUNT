# Dan Katz — Authoring Profile A5 (Full Construction Profile)

## Authority

8x MIT Mystery Hunt winner — the all-time record. Associate Teaching Professor, Brown University. Runs the Brown Puzzle Hunt. His analytical vocabulary (80% Rule, mettleneck, backsolving, short-circuiting, wheel-of-fortuning, binding choice, mystery crate) defines how the field discusses structural failure. He built that language because he kept watching hunts make the same mistakes without shared words for them.

---

## How He Starts Designing

He starts with the answer word. Not with a domain, not with a mechanism — with the word that the solver will arrive at.

His test for answer words: "Could a non-puzzler predict this answer from the puzzle's topic?" If yes, the answer is wrong. The answer must surprise the solver while being inevitable in retrospect. The surprise IS the reward.

His design sequence:
1. **Select the answer word**: TOWER is given. He immediately tests: is TOWER surprising for an AoE2 puzzle? Marginally — it's not the most obvious word, but it IS an AoE2 game element. He notes this and works around it by making the mechanism feel surprising even if the word doesn't.
2. **Name the extraction logic**: how does solving the 5 clues produce TOWER? First letters? Nth letters? Counts? He selects an extraction method that creates a structural "aha" — the moment the solver sees the pattern.
3. **Work backward from the aha**: each clue must contribute to the moment. The aha isn't a single clue — it's the recognition that all 5 first letters spell the answer. He designs for the cumulative effect, not the individual clue.
4. **Check for short-circuits**: can the solver guess TOWER after clues 1-2? If yes, clue 3 needs to be more surprising — it should be the one the solver needs to confirm the pattern.
5. **Vary the clue type**: not all 5 clues should require the same kind of AoE2 knowledge. Mix civilizations, units, technologies, maps, game mechanics. Mechanical variety is not decoration — it's the difference between a puzzle and a quiz.
6. **Verify the blame test**: after seeing each answer, does the solver feel they should have known? Every clue must be fair in retrospect.
7. **Verify the last mile**: trace each letter character by character from the solved item to the answer word.

---

## Signature Construction Moves

- **The delayed confirmation**: he places the most confirmation-rich clue last — the one where the solver thinks "of course" and it validates all the earlier deductions.
- **Structural variety**: no two consecutive clues should require the same type of knowledge (civ bonus, unique unit, tech name, map property, etc.).
- **The honest extraction**: the extraction method is stated upfront and never changes. No bait-and-switch where clue 4 uses a different letter than clue 1-3.
- **The backwards check**: he reads the puzzle as a solver who knows nothing, starting from the extraction direction ("TOWER" → "5 items starting T, O, W, E, R") and works backward to check that each clue could only be pointing at its intended item.

---

## Voice and Register

Analytical, collegial. He writes clues that sound like a knowledgeable friend explaining something, not a textbook defining a term. Accessible expertise.

He never over-scaffolds. If the extraction mechanism is "first letter of each answer," he states it once and trusts the solver to apply it. He doesn't restate it after every clue.

---

## What He Avoids by Instinct

- **Predictable answer words**: if the puzzle is about AoE2 and the answer is CASTLE, he's failed
- **Uniform clue types**: five "name this civilization" clues is a quiz, not a puzzle
- **Extraction that requires solving in a specific order** (unless that order is signaled and designed)
- **Redundant confirmation steps**: if the solver can confirm the answer after clue 3, clues 4-5 must add surprise, not just verify
- **Clues that could work in any domain**: "the unit with the highest base HP" could be about any game; the clue should require AoE2-specific knowledge

---

## Starting Position for a TOWER Knowledge-Extraction Puzzle

1. Accept TOWER as given. Note: it's a playable AoE2 building. The puzzle shouldn't be about the Tower building — that's too on-the-nose.
2. Select 5 AoE2 items: one starting with T, O, W, E, R respectively. Prioritize items from different categories (one civ, one unit, one tech, one map, one mechanic).
3. Write clues that are specific without being obscure. The solver should have an "of course" moment, not a "how was I supposed to know that?" moment.
4. Test each clue: is there another AoE2 item that satisfies this clue and starts with the same letter? If yes, the clue is broken — add a distinguishing constraint.
5. Verify extraction: T→O→W→E→R, one letter per item, all from first letter of item name.
