# Stage 2 Review — Wololo: An Age of Empires Puzzle Hunt

**Reviewers:** Dan Katz (structure & pacing) | Mike Selinker (narrative & experience) | Alex Rosenthal (accessibility & wonder)

**Documents reviewed:** SCOPE.md (Stage 1), ROUNDS.md (Stage 2)

**Date:** 2026-02-27

---

## Reviewer 1: Dan Katz — Structure & Pacing

### Score: 7/10

### What Works

**The size is right.** 5+1 is an excellent scale for a toolkit validation run. No bloat, no filler. Every puzzle covers a distinct domain (Civilizations, Units, Technologies, Maps, Strategy) with zero overlap. This is the kind of discipline I wish more Mystery Hunt rounds had. You could solve this in a single sitting without the experience collapsing under its own weight.

**The difficulty curve is intentional.** 2-2-3-2-3 is a smart shape. Starting with identification (match bonuses to civs) is the right call — it is the lowest-friction entry point for any AoE player. Dipping back to difficulty 2 for Maps after the Castle Age tech tree puzzle at 3 gives a breather before the final build-order puzzle. This is pacing that respects the solver.

**The 80% rule is acknowledged.** The ROUNDS doc explicitly states the crossword meta is solvable with 4 of 5 answers. Good. But "acknowledged" is not the same as "verified." See below.

### Issues

**Issue 1 (HIGH): The crossword meta at 5 words is structurally fragile.**
A 5-word crossword is barely a crossword. With only 5 answer words, you have at most ~10 crossing points. WOLOLO needs 6 highlighted squares (W-O-L-O-L-O). For the 80% rule to hold (solvable with 4 of 5 answers), you need enough crossing constraints that the missing word is deducible from its crossing letters alone. With 5 words, the grid is almost certainly sparse — each word crosses maybe 2-3 others. Losing one answer might leave 2-3 uncrossed letters in adjacent words, which may not be enough to recover.

**Recommendation:** Before Stage 3, build a proof-of-concept grid with 5 placeholder words and verify that removing any single word still yields WOLOLO through crossing constraints. If the grid cannot support the 80% rule, consider (a) adding a 6th feeder puzzle to increase grid density, or (b) switching the meta mechanism to something more forgiving — an acrostic (first letters of answers spell WOLOLO) or a drop-quote where partial completion still reveals the phrase.

**Issue 2 (MEDIUM): WOLOLO as the meta answer — does it survive scrutiny?**
The 80% rule has a second dimension: could a solver guess the meta answer early? WOLOLO is the single most iconic sound in Age of Empires. The hunt is called "Wololo." The narrator is a Monk. The theme is conversion. Any solver who has played AoE2 for more than 10 hours will suspect the answer is WOLOLO before solving a single puzzle. This is not necessarily fatal — knowing the destination does not ruin the journey if the mechanism is satisfying. But it means the meta mechanism itself must be the reward, not the surprise of the answer. The crossword grid must be elegant enough that filling it in and watching WOLOLO emerge letter by letter is the payoff.

**Recommendation:** Lean into the transparency. Do not try to hide that WOLOLO is the answer. Instead, make the meta grid visually satisfying — the moment the solver fills in the last crossing letter and sees the highlighted squares spell it out should feel like a confirmation, not a revelation. Consider whether the grid shape itself can evoke something (a monk, a cross, a tech tree).

**Issue 3 (LOW): Puzzle types may be too uniform in extraction.**
All 5 puzzles appear to produce a single answer word. That is standard. But if all 5 extraction mechanisms are "take first letters" or "read highlighted cells," the hunt will feel repetitive at the extraction layer even if the puzzle content varies. Stage 3 should ensure varied extraction: one puzzle gives a word directly, one uses indexing, one uses initial letters, etc.

### Summary

The architecture is sound. 5+1 is the right size, the difficulty curve is well-shaped, and the domain coverage is clean. The main structural risk is the crossword meta at 5 words — it is on the edge of being too small to support the 80% rule. Build the proof-of-concept grid early. Everything else is solvable in Stage 3.

---

## Reviewer 2: Mike Selinker — Narrative & Experience

### Score: 6/10

### What Works

**The Monk narrator is a great character.** Short sentences. Present tense. No exclamation marks. Speaks as if the game world is real. This is a genuine voice, not a gimmick. The sample intros in ROUNDS.md are strong — "Eight civilizations stand before you. Each one carries a gift no other has. Name them." That is clean, atmospheric writing. I can hear this character. I believe in this character.

**The age-progression framing creates real narrative arc.** Dark Age through Post-Imperial is not just a difficulty ramp — it is a story. The solver is advancing through the ages, just as they would in a game. The meta as "Wonder Victory" (the game's infinite win condition) is a satisfying structural metaphor. Numbering puzzles with roman numerals for Ages and the meta as infinity is elegant.

**"Wololo" as the final moment is genuinely theatrical.** The Monk has been narrating the whole hunt, never saying his catchphrase. The meta intro reads: "You have advanced through all five ages. One sound remains. You know what it is." The solver fills in the meta grid. WOLOLO appears. The Monk has finally converted you. That is a payoff. That is a moment someone would describe to a friend.

### Issues

**Issue 1 (HIGH): There is no Dinner Party puzzle in the feeder set.**
The meta reveal (WOLOLO) is the Dinner Party moment. Good. But what about the five feeder puzzles? Match bonuses to civs (identification). Trace counter chains (logic). Fill missing techs (completion). Identify maps from descriptions (identification again). Sequence a build order (sequencing). These are all clean, well-typed puzzles. None of them will make someone say "you will not believe what I just did." They are solid B-material. The hunt needs at least one A-material feeder.

**Recommendation:** Promote one feeder to Dinner Party status. The best candidate is Puzzle IV (Maps). Instead of "identify 5 maps from terrain descriptions," make the solver DRAW or RECONSTRUCT a map from clues — perhaps overlaying fragments that, when correctly assembled, reveal a word written in the terrain itself. A spatial-visual puzzle where the answer is literally hidden in the landscape of an AoE map would be the kind of thing someone describes at dinner. Alternatively, the counter chain (Puzzle II) could be elevated: instead of a simple "what beats what" chain, make it a circular chain where the solver must discover the one unit that breaks the cycle — and that unit's name is the answer.

**Issue 2 (MEDIUM): The Monk's voice is established but not tested under pressure.**
The sample intros are great. But the voice rules will be tested in the body of each puzzle — the clue text, the flavor lines between sections, the hints. Five puzzles at ~500 words each means ~2,500 words of Monk voice. Maintaining "short sentences, present tense, no exclamation marks, speaks as if the game is real" across that volume is harder than it looks. One slip ("Check the tech tree!") and the voice breaks.

**Recommendation:** Write a 200-word Monk voice sample that covers all the hard cases: giving a hint without breaking character, acknowledging the solver is stuck, transitioning between puzzle sections. Establish the voice under pressure before Stage 3 drafting begins.

**Issue 3 (MEDIUM): The narrative does not escalate tension.**
Advancing through the Ages is a good metaphor, but the Monk's relationship to the solver does not change. In AoE, the game gets more intense as you age up — more units, more conflict, more at stake. The Monk's intros should reflect this. Dark Age should feel quiet and uncertain. Imperial Age should feel urgent. The meta should feel like a siege. Right now all five intros have the same temperature.

**Recommendation:** Write a narrative arc for the Monk's voice across the five puzzles. Dark Age: calm, observational. Feudal: slightly more direct. Castle: tension rising. Imperial: stakes explicit. Post-Imperial: quiet confidence. Meta: the conversion. This costs nothing to implement and adds real emotional shape to the experience.

### Summary

The Monk is a strong character and WOLOLO is a strong payoff. The narrative framing (Ages as progression, Wonder Victory as meta, the Monk's withheld catchphrase) is genuinely good. But the five feeder puzzles are all B-grade experiences — competent but not memorable. At least one needs to be elevated to Dinner Party status. And the Monk's voice needs a deliberate emotional arc across the five ages, not a flat register throughout.

---

## Reviewer 3: Alex Rosenthal — Accessibility & Wonder

### Score: 7/10

### What Works

**The TED-Ed pitch is clean.** "A puzzle hunt about Age of Empires where you advance through the ages and the final answer is the monk conversion sound — WOLOLO." I can say that in one sentence. A non-puzzler who has played AoE would immediately grin. That is the test. This passes.

**The audience is well-defined.** "Solo puzzler who has played AoE (any version)." That is millions of people. AoE2 Definitive Edition has sold over 10 million copies. The nostalgia factor is real and broad. This is not a niche puzzle hunt for competitive AoE players — it is for anyone who remembers villagers saying "Rogan?" and monks going "Wololo."

**2-3 hours is the right length.** Long enough to feel like an event, short enough to complete in one sitting. For a solo experience targeting casual AoE fans, this is exactly right. You do not want someone putting this down and never coming back.

**The format is accessible.** Printable PDF or Markdown. No special tools. No apps. No login. This is "download, print, solve with a pencil." The Book Test (from PRINCIPLES.md) is satisfied — except this is not even a book, it is a few pages. Lower barrier than a book.

### Issues

**Issue 1 (HIGH): Knowledge calibration is the biggest risk.**
The hunt says "the solver's memory IS the library." This is a departure from the toolkit's encyclopedia model, and it is the single biggest accessibility risk. Consider a casual AoE player who played AoE2 in 2002 and bought Definitive Edition in 2020 but only plays occasionally:

- Could they match 8 civilization bonuses to civilizations? Maybe 4-5 of 8. The rest would require looking things up.
- Could they trace unit counter chains? Spearmen beat cavalry, skirmishers beat archers — yes. But detailed counter chains with specific unit names? Unlikely without reference.
- Could they fill missing techs in a research chain? Almost certainly not from memory.
- Could they identify maps from terrain descriptions? Arabia, Black Forest, Islands — yes. Specific details? No.
- Could they sequence an optimal build order? Only competitive players know build orders.

The hunt is optimized for someone who plays AoE regularly and knows the tech tree. That is a much smaller audience than "anyone who has played AoE."

**Recommendation:** Provide a reference sheet. A one-page "AoE Quick Reference" with civilization bonuses, unit counter chart, and tech tree summary. This does not make the puzzles easier — the solver still has to figure out which information is relevant and how to use it. It just ensures that imperfect memory is not a barrier. The toolkit's Reading Reward principle says "solving should require understanding the material," not "solving should require memorizing the material." A reference sheet turns the hunt from "memory test" to "reasoning test."

**Issue 2 (MEDIUM): Puzzle V (build order) may alienate casual players.**
Build orders are competitive AoE knowledge. A casual player who turtles behind walls and booms to Imperial Age has never optimized a Castle Age timing. This puzzle is not "AoE knowledge" — it is "competitive AoE knowledge." There is a difference. A puzzle about strategy that does not require meta-gaming expertise would be more accessible.

**Recommendation:** Replace or reframe Puzzle V. Instead of "sequence 8 steps to reach Castle Age optimally" (which has a single correct answer that only competitive players know), consider "put historical events in chronological order" using the in-game civilization histories, or "match famous AoE strategies to their names" (Fast Castle, Flush, Boom, etc.) as an identification puzzle rather than a sequencing one. The domain is still Strategy, but the knowledge requirement shifts from competitive to cultural.

**Issue 3 (LOW): The "wonder" moment needs to be visual.**
Puzzle hunts that create wonder tend to do it visually — the moment you tilt the page and see the word, the moment the star chart reveals a constellation that spells the answer. WOLOLO appearing as highlighted squares in a crossword grid is satisfying but not visually dramatic. For a hunt about a video game known for its visual and audio identity, there is an opportunity for something more.

**Recommendation:** Consider making the meta grid shaped like a monastery or a monk sprite. Or have the highlighted squares trace the path of a conversion animation. Small visual touches that make the final reveal feel like it belongs in the AoE universe, not just in a generic crossword.

### Summary

The pitch is great, the length is right, and the format is accessible. The biggest risk is knowledge calibration — the hunt currently requires deeper AoE knowledge than a casual player has. A reference sheet solves this cleanly. The build-order puzzle (V) leans too competitive and should be reframed. And the meta reveal, while narratively strong, would benefit from visual flair that matches the game's aesthetic.

---

## Synthesis

### Scores

| Reviewer | Score | Verdict |
|----------|-------|---------|
| Dan Katz (structure) | 7/10 | Sound architecture, meta needs proof-of-concept |
| Mike Selinker (narrative) | 6/10 | Strong character, needs a Dinner Party puzzle |
| Alex Rosenthal (accessibility) | 7/10 | Great pitch, knowledge calibration at risk |
| **Average** | **6.7/10** | |

### Overall Verdict: PROCEED to Stage 3 — with 4 action items

The design is fundamentally sound. The size is right (5+1), the narrative framing works (Ages as progression, Monk as narrator, WOLOLO as payoff), and the pitch is clean enough for broad appeal. No structural redesign is needed. But four specific issues must be resolved before or during Stage 3 drafting.

### Consensus Issues

All three reviewers agree on the following:

1. **The meta mechanism is the load-bearing risk.** A 5-word crossword is small. It must be proven (not assumed) that it supports the 80% rule. If it does not, the meta mechanism must change before any puzzles are drafted.

2. **Knowledge calibration needs a solution.** The hunt cannot rely on perfect AoE memory. Either a reference sheet or in-puzzle lookups must be provided so that reasoning — not recall — is the gating factor.

3. **The feeder puzzles are competent but not memorable.** The meta reveal (WOLOLO) is the emotional climax. But the journey to get there needs at least one feeder that creates its own moment of delight.

### Action Items for Stage 3

| # | Priority | Action | Owner | Source |
|---|----------|--------|-------|--------|
| 1 | **P1** | Build a proof-of-concept crossword grid with 5 placeholder words. Verify that removing any single word still yields WOLOLO through crossing constraints. If it fails, switch to acrostic or drop-quote mechanism. | Meta designer | Katz |
| 2 | **P1** | Create a one-page AoE Quick Reference sheet (civ bonuses, unit counters, tech tree summary). Include it as part of the hunt materials. | Content author | Rosenthal |
| 3 | **P2** | Elevate one feeder puzzle to Dinner Party status. Best candidates: Puzzle IV (Maps — make it spatial/visual) or Puzzle II (Units — make the counter chain circular with a rule-breaking discovery). | Puzzle designer | Selinker |
| 4 | **P2** | Reframe Puzzle V (Strategy) from competitive build-order sequencing to something accessible to casual players. Options: historical chronology using in-game civ histories, or strategy-name identification. | Puzzle designer | Rosenthal |
| 5 | **P3** | Write a Monk voice escalation guide: emotional temperature for each Age (Dark=calm, Feudal=direct, Castle=tense, Imperial=urgent, Meta=quiet power). Test with a 200-word sample. | Narrative author | Selinker |
| 6 | **P3** | Design the meta grid with visual intent — monastery shape, monk sprite outline, or conversion-path trace. The reveal should feel like AoE, not like a generic crossword. | Meta designer | Rosenthal |

### Principle Compliance Check

| Principle | Status | Notes |
|-----------|--------|-------|
| Riven Standard | PASS | Every puzzle IS AoE knowledge, not trivia overlaid on a generic format |
| Solving = Proving Understanding | PASS | Answers require understanding game systems, not just keyword lookup |
| Dinner Party Test | AT RISK | Meta passes; no feeder currently passes (Action Item 3) |
| The Book Test | PASS | Pencil and printed pages only |
| Blame the Player | TBD | Cannot evaluate until puzzles are drafted (Stage 3) |
| 80% Rule | AT RISK | Crossword at 5 words needs proof-of-concept (Action Item 1) |
| No Over-Scaffolding | TBD | Stage 3 |
| Surprise the Answer | PARTIAL | WOLOLO is expected but emotionally satisfying; feeder answers TBD |
| One Aha | TBD | Stage 3 |
| No Deliberate Errors | TBD | Stage 3 — AoE facts must be verified against game data |
| Interlock | AT RISK | 5 independent puzzles feeding a crossword — interlock is only at meta level |
| No Computation Without Deduction | TBD | Stage 3 |
| Snyder's Computer Test | TBD | Stage 3 |
| Voice Rules | PASS | Monk voice is well-defined; needs pressure-testing (Action Item 5) |

---

*Review conducted under the 3-reviewer panel model. Proceed to Stage 3 (individual puzzle design) with the above action items addressed.*
