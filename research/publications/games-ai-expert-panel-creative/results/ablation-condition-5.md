# Ablation Study — Condition 5: Lens Only (No Biography, No Philosophy)

**Condition**: Reviewer persona restricted to the "## Review Lens" section only. No name context
used beyond attribution label. No biography, credentials, design philosophy, or principles.
Operational checklist only.

**Puzzles reviewed**:
- Puzzle I: Dark Age (answer: CASTLE)
- Puzzle II: Feudal Age (answer: ONAGER)
- Puzzle III: Castle Age (answer: LOOM)

**Date**: 2026-02-28

---

## Puzzle I — Dark Age — C5 (Lens Only)

**Reviewer**: Dan Katz's Review Lens — no biography, no philosophy

### Lens questions applied

The lens contains 12 operational questions. Each is applied in sequence. Questions that operate
at hunt scope (meta architecture, unlock structure, hint economics, audience size) are flagged
as out of scope for a single feeder puzzle but noted where they reveal partial signal.

**Q1 — Does the hunt size match the audience?**
Out of scope for individual puzzle. No signal.

**Q2 — Can the meta be short-circuited?**
Out of scope for a feeder. No signal.

**Q3 — Is the meta a pattern-recognition problem or a full puzzle?**
Out of scope. No signal.

**Q4 — Are the hint economics honest?**
The hunt has no hints (documented in scenario CLAUDE.md). Out of scope. No signal.

**Q5 — Does every puzzle justify its slot?**
FIRES. This is the entry puzzle in a 5-feeder hunt. The mechanism is: identify a civilization
from its unique bonus, write the name, index the numbered letter. The content is accurate AoE2
knowledge (civ bonuses are distinct and verifiable). The puzzle earns its slot as a direct test
of civilization familiarity — it does something the other four puzzles (units, technologies,
tournament bracket, tech chains) do not.

**Q6 — Is the unlock structure creating mystery crates?**
Out of scope. No signal.

**Q7 — Are the mechanisms varied enough?**
Partially fires. Internally, the puzzle uses one mechanism for all eight clues with no variation.
This is not a defect in isolation but would register as a uniformity risk if adjacent puzzles in
the hunt share the same structure (identify-and-index format). No evidence of that from the other
two puzzles reviewed here.

**Q8 — Is the narrative encountered while solving or reported afterward?**
FIRES. The Monk's flavor text ("Eight civilizations stand before you. Each one carries a gift
no other has. Name them.") precedes the puzzle rather than inhabiting it. The solving experience
is: read bonus, recall civ, write name, circle letter. The narrative plays no role in any step.
This is set dressing, not structural narrative. The "Two of these bonuses concern infantry. Read
them closely." hint is closer to structural because it guides the solver toward a potential
ambiguity (Aztecs vs. Japanese both have infantry bonuses), but it still lives in the frame
text rather than emerging from the mechanic itself.

**Q9 — Are the testers calibrated correctly?**
Cannot assess from puzzle text. No signal.

**Q10 — Is the thematic coherence structural, not decorative?**
FIRES — positive result. The civilization bonus IS the puzzle mechanism. The solver does not
look up AoE trivia and then do a separate puzzle step; knowing the civilizations IS the
solve. Theme and mechanism are unified at the content level. This is the lens question that
fires most cleanly in the puzzle's favor.

**Q11 — Are free answers being deployed honestly?**
No free answers. Out of scope. No signal.

**Q12 — Would he want to solve this?**
FIRES — with significant concern. The extraction instruction reads: "Eight letters. Not all of
them matter. Find the word." This is a structurally broken step. The puzzle extracts eight
letters (one per civilization), then asks the solver to find a word from a subset of them with
no mechanism for determining which subset. The only valid approach is:

1. Collect all eight letters.
2. Anagram subsets until a word emerges.
3. Confirm CASTLE.

This is wheel-of-fortuning (to use the vocabulary the checklist implies): the solver is
guessing an English word from partial information rather than deriving a unique answer through
the puzzle's logic. A solver who cannot spell CASTLE from their letter set has no principled
way to confirm they are done. A solver who misspells one civilization name will get a wrong
letter and will not be able to identify the error, because they don't know which letters are
supposed to matter.

The checklist question fires: there is a structural problem that cannot be fixed by individual
puzzle quality. The mechanism breaks at the extraction step.

**Secondary structural issue**: The blanks for civilization names provide implicit letter counts
(e.g., "_ _ _ _ _" = 5 letters), which is a significant solving aid. This partly offsets the
extraction ambiguity but also means a solver who doesn't know a civ bonus cannot work backward
from the blank count alone without extra knowledge. The blanks are a useful confirmation tool,
not a solve path.

**Verification (answer derivation)**:

Working through the eight bonuses against known AoE2 data:

| Bonus | Civilization | Letters | Index | Extracted |
|-------|-------------|---------|-------|-----------|
| A. Infantry 15% faster (Feudal) | AZTECS (5) | A-Z-T-E-C-S | 1 | A |
| B. Infantry attacks 33% faster (Feudal) | JAPANESE (8) | J-A-P-A-N-E-S-E | 2 | A |
| C. TC/Dock work rate +10/+15/+20% | CHINESE (7) | C-H-I-N-E-S-E | 4 | N |
| D. Foot archers +1 range Castle, +1 Imperial | BRITONS (7) | B-R-I-T-O-N-S | 4 | T |
| E. Cavalry archers fire 25% faster | MONGOLS (7) | M-O-N-G-O-L-S | 6 | L |
| F. Villagers +5 carry, military 11% faster | INCAS (5) | I-N-C-A-S | 4 | A |
| G. Cavalry +20% HP | FRANKS (6) | F-R-A-N-K-S | 1 | F |
| H. Trash units (Skirmishers, Pikemen, Light Cav) cost 25% less | GOTHS (5)? | — | 1 | ? |

Collected letters in order: A, A, N, T, L, A, F, ?

The letters A-A-N-T-L-A-F-? do not contain C, A, S, T, L, E as a subset unless the
civilization identifications above are wrong. CASTLE requires C + A + S + T + L + E.

Checking: T (Bonus D) and L (Bonus E) are present. A is present (multiple). C, S, E are
not present in the derivation above.

This indicates either: (a) several civilization identifications above are incorrect, or (b)
the blanks' letter counts serve as the primary verification path and the derivation above
has errors. The blank counts provided are:

- A: 5 letters → AZTECS (6 letters — MISMATCH). This is the first problem. AZTECS is 6
  letters, not 5. This either means the identification is wrong or the blank count is
  wrong. If the blank for A shows 5 underscores and the answer is 6 letters, the puzzle
  has a factual error.

Rechecking: "_ _ _ _ _" = 5 blanks. AZTECS = A-Z-T-E-C-S = 6 letters. This is a direct
contradiction. Either the civilization is wrong (a different 5-letter civ has infantry 15%
faster from Feudal) or the blank count is incorrect.

Alternative for 5-letter infantry-speed civ: No standard AoE2 civ has this exact bonus
at exactly 5 letters. MALAY? (5 letters, but their bonus is faster age-up, not infantry
speed.) This appears to be a factual/construction error in the puzzle.

The letter count mismatch for Bonus A is a confirmed construction defect.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 3 | Instructions are clearly stated; the "not all matter" extraction is a deliberate choice but creates ambiguity about which letters count |
| Solvability | 2 | Blank count for Bonus A contradicts AZTECS (6 letters, 5 blanks shown); if the civ ID is wrong the full answer chain is unverifiable; anagram-subset extraction has no unique path |
| Elegance | 2 | Identify-and-index is a clean format but the "not all matter" instruction breaks the elegance of the mechanism; no wit in the structure |
| Reading Reward | 4 | AoE2 civ bonus knowledge is fully load-bearing; solver must know the game to proceed; theme is structural |
| Fun | 3 | Identification of civ bonuses from clues is satisfying for a knowledgeable AoE player; extraction step undercuts the payoff |
| Confirmation | 2 | Solver cannot confirm which letters matter; a misspelled civilization corrupts one letter with no error-detection mechanism; blank count inconsistency for Bonus A removes one confirmation path |
| **Total** | **16/30** | |

### Issues identified (checklist-level, specific)

1. **Blank count mismatch, Bonus A**: The blank shows 5 underscores. AZTECS is 6 letters. These
   cannot both be correct. Either the blank count is wrong (should be 6) or the intended
   civilization is a different 5-letter civ. If AZTECS is wrong, the identification of the
   entire infantry-speed bonus must be reconsidered. This is the most severe defect — it
   corrupts the first extracted letter.

2. **Unstructured subset extraction**: "Eight letters. Not all of them matter. Find the word."
   gives the solver no mechanism for determining which 6 of 8 letters to use. The only valid
   approach is anagramming a subset, which is guessing, not solving. The puzzle needs either
   (a) a rule that specifies which letters are "real" (e.g., "the letters from civilian
   civilizations" or "the letters from civilizations that appear in the Dark Age"), or (b) a
   redesign that produces exactly the right number of letters.

3. **No error detection for wrong civilization identifications**: If a solver identifies the
   wrong civilization for any bonus, they get a wrong letter with no way to know which step
   failed. The blank letter counts partially address this (wrong answer = wrong letter count),
   but only for cases where the solver can count; a close-count civ (e.g., 7 vs. 6 letters)
   may not trigger the check.

4. **Narrative is decorative**: "Eight civilizations stand before you. Each one carries a
   gift no other has." adds nothing to the mechanic. This is a checklist observation, not a
   scoring defect, but the lens flags it.

### Verdict: FAIL (16/30, threshold 22)

---

## Puzzle II — Feudal Age — C5 (Lens Only)

**Reviewer**: Thomas Snyder's Review Lens — no biography, no philosophy

### Lens questions applied

The lens contains 12 operational questions oriented toward construction precision, solve path
integrity, and the relationship between visual structure and logic. Applied in sequence.

**Q1 — Is the entry point constructed or discovered?**
FIRES. The entry point is QF-1 (Onager vs. Crossbowman). The counter logic is clearly stated:
siege destroys archers. This match has a definite forced result (Onager wins) that is
derivable from the rules given. The entry point is deliberate — the easiest match in the
bracket is placed first, giving the solver a clean first deduction. This fires in the puzzle's
favor.

**Q2 — Does the solve path hold one direction?**
FIRES — with a structural concern. The intended solve path is: resolve QF-1 → QF-2 → QF-3 →
QF-4 → SF-1 → SF-2, extracting letters in order. However, QF-3 (Cataphract vs. Halberdier)
requires knowledge of the Cataphract's specific anti-anti-cavalry armor bonus, which the
puzzle signals with: "One unit in this bracket defies its counter." This signal is given once
and applies to one match, which is clean. The deduction sequence is forced and runs one
direction. No branching. This question fires in the puzzle's favor.

**Q3 — Is each element load-bearing?**
FIRES. Each of the six extraction matches is necessary:
- QF-1 feeds SF-1 winner and provides letter O.
- QF-2 feeds SF-1 winner and provides letter N.
- QF-3 feeds SF-2 winner and provides letter A.
- QF-4 feeds SF-2 winner and provides letter G.
- SF-1 provides letter E.
- SF-2 provides letter R.

The Final match is explicitly excluded from extraction ("your extraction uses only the six
winners from the Quarterfinal and Semifinal rounds"), which is correct — the Final is not
load-bearing for the answer. Every element included in the extraction is necessary. This
fires in the puzzle's favor.

**Q4 — Does the theme shape the structure, or sit on top of it?**
FIRES — positive. The tournament bracket IS the AoE2 unit counter system. The solver is not
solving a puzzle "about" unit counters — they are applying unit counter logic to derive a
result. The bracket format is not decorative; it is the mechanism. Theme and structure are
the same thing here. This fires strongly in the puzzle's favor.

**Q5 — Is the difficulty technique-based or noise-based?**
FIRES. The only difficulty is:
(a) Knowing or reasoning through unit counter relationships.
(b) Knowing the Cataphract exception specifically.
Both are technique-based (AoE2 knowledge). There is no noise. The counter logic table is
provided and is concise. This fires in the puzzle's favor.

**Q6 — Can the solver confirm each step independently?**
FIRES — with a concern. Each QF match produces a winner whose name's letter is extracted.
The solver can confirm each winner independently before proceeding. However, the Cataphract
exception in QF-3 requires specific knowledge. The puzzle signals the exception exists
("One matchup breaks the rule you expect") but does not identify which matchup until the
solver encounters it. A solver who does not know Cataphract's special armor will apply the
standard rule (Halberdier beats Cataphract) and get A → letter 2 of HALBERDIER = A, which
is still A. This means the puzzle accidentally self-corrects on this step: whether the solver
knows the exception or not, they extract the same letter from the match if their error gives
HALBERDIER. Let me check: letter 2 of HALBERDIER = H(1)A(2) = A. Letter 2 of CATAPHRACT =
C(1)A(2) = A. The puzzle inadvertently produces the same extracted letter regardless of
whether the solver knows the Cataphract exception. This is a structural redundancy — the
"exception" clue does not actually change the extracted letter, making the hint necessary
for tournament integrity but irrelevant to answer derivation. This is a design flaw: the
most interesting knowledge-test in the puzzle (knowing the Cataphract exception) has no
consequence for the answer.

**Q7 — Does the puzzle teach the skill it claims to require?**
FIRES — negative. The puzzle claims (via the exception hint) that Cataphract counter knowledge
is required. It is not — as demonstrated above, the extracted letter is A regardless. The
puzzle does not teach this knowledge; it gestures at it. A solver who completes the puzzle
without knowing the exception gets the correct answer. This is a missed opportunity at
minimum, and a constructional defect at most (the hint creates a misleading difficulty signal).

**Q8 — Are there multiple valid solving paths that obscure the intended one?**
FIRES — on QF-3 specifically. There are two valid solving paths for QF-3:
- Path A (correct): Cataphract has anti-anti-cavalry armor → Cataphract wins → letter 2 of
  CATAPHRACT = A.
- Path B (incorrect counter logic): Halberdier counters cavalry → Halberdier wins → letter
  2 of HALBERDIER = A.
Both paths yield the same extracted letter. The intended path is masked by the incidental
equivalence. This fires as a construction failure: the Cataphract exception cannot be
verified by the solver because the answer does not distinguish between the two paths.

**Q9 — Does the visual design communicate structure, or just fill space?**
FIRES — negative. The ASCII bracket diagram contains a direct error:

```
├─ Winner [5th] ──┐
│
├─ Winner [7th] ──┘
│
├─ Winner [7th] ──┘   ← This line shows [7th] twice
```

In the bracket as printed, both SF-1 (upper semifinal) and SF-2 (lower semifinal) are labeled
"Winner [7th]" in the ASCII art. The worksheet table correctly shows SF-1 as "5th" and SF-2
as "7th." The bracket diagram contradicts the worksheet. A solver using the diagram alone
would extract letter 7 from both SF winners, producing a wrong answer. The visual design
communicates wrong information and must be corrected before the puzzle is usable. This is the
most severe defect in the puzzle.

**Q10 — Did someone add information to increase difficulty?**
Does not fire. The counter logic table is necessary and appropriately scoped. No excess
constraints.

**Q11 — Is there a puzzle here, or a procedure?**
FIRES. There is a genuine deduction moment: recognizing that the Cataphract defies its
counter. The bracket resolution requires reasoning through multiple sequential counter
relationships. This is not a pure procedure — it requires knowledge and application.
However, as noted under Q7, the most interesting deduction (Cataphract exception) has no
mechanical consequence for the answer, weakening the puzzle's claim to requiring it. Fires
weakly in the puzzle's favor on the procedural question; fires as a concern on the
knowledge-consequence question.

**Q12 — Would he want to have constructed this?**
FIRES — with reservation. The bracket-as-counter-system is a genuinely elegant architecture
with a clean 6-letter extraction. The Cataphract exception is the right kind of knowledge-
test for this puzzle type. The construction failure is that the exception produces the same
letter regardless of solver knowledge, meaning the puzzle cannot actually test what it
claims to test. A puzzle that cannot verify its own most interesting fact has not been
finished.

**Verification (answer derivation)**:

| Match | Unit A | Unit B | Counter rule applied | Winner | Index | Letter |
|-------|--------|--------|---------------------|--------|-------|--------|
| QF-1 | Onager | Crossbowman | Siege destroys archers | Onager | 1st | O |
| QF-2 | Pikeman | Knight | Anti-cav infantry destroys cavalry | Pikeman | 7th | N |
| QF-3 | Cataphract | Halberdier | Exception: Cataphract resists anti-cav bonus | Cataphract | 2nd | A |
| QF-4 | Mangonel | Champion | Siege destroys infantry | Mangonel | 4th | G |
| SF-1 | Onager | Pikeman | Siege destroys infantry | Onager | 5th | E |
| SF-2 | Cataphract | Mangonel | Cavalry destroys siege | Cataphract | 7th | R |

Letters in order: O-N-A-G-E-R = ONAGER. Confirmed correct.

**Critical note on QF-3 equivalence**: CATAPHRACT letter 2 = A; HALBERDIER letter 2 = A.
A solver applying wrong counter logic for QF-3 still extracts A and arrives at ONAGER.
The puzzle is self-confirming but not self-correcting on its most important knowledge point.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 3 | Instructions are clear; the bracket ASCII art contains a direct error ([7th] shown twice instead of [5th]/[7th]) that will actively mislead solvers using the diagram |
| Solvability | 4 | Answer is derivable; counter logic is clearly stated; Cataphract exception is signaled; bracket works correctly in the worksheet even where the diagram fails |
| Elegance | 3 | Bracket-as-counter-system is elegant in concept; the Cataphract exception being consequence-free for letter extraction breaks the elegance claim |
| Reading Reward | 4 | AoE2 unit counter knowledge is genuinely load-bearing for five of six matches; the exception knowledge is decorative but still thematically rewarding |
| Fun | 4 | Tournament format is engaging; working through the bracket has a satisfying cascade; the Cataphract reveal is a nice moment even if it changes nothing |
| Confirmation | 3 | Solver can confirm each match independently via the counter logic table; bracket diagram error creates a false confirmation path if solver uses diagram instead of worksheet |
| **Total** | **21/30** | |

### Issues identified (checklist-level, specific)

1. **Bracket diagram shows "[7th]" for both SF slots**: In the ASCII bracket, both semifinal
   winner slots are labeled "[7th]". The worksheet correctly shows SF-1 as "5th" and SF-2
   as "7th". Any solver who resolves the bracket from the diagram alone will extract the
   wrong letter from SF-1 (letter 7 of ONAGER = S, not letter 5 = E), producing a wrong
   answer. This is a print/layout defect that must be corrected before distribution.
   Fix: change the upper SF bracket label from "[7th]" to "[5th]".

2. **Cataphract exception produces same letter regardless of solver knowledge**: Letter 2
   of both CATAPHRACT (correct winner) and HALBERDIER (wrong winner under standard counter
   logic) is A. The puzzle's most interesting knowledge-test (the exception rule) has no
   consequence for answer derivation. A solver who does not know the exception arrives at
   the correct answer anyway. Options: (a) accept this as a lucky robustness feature, or
   (b) redesign to place the Cataphract extraction at a position where CATAPHRACT and
   HALBERDIER diverge (e.g., position 3: T vs. L).

3. **Final match is unused but unexplained until footnote**: The puzzle includes a Final
   bracket slot with a CHAMPION label, resolves it, and then clarifies in a note that
   the Final is not used for extraction. This creates momentary confusion. The note handles
   it adequately but the bracket would be cleaner if the Final were visually distinguished
   (e.g., a dashed box or explicit "Not used" label in the diagram).

### Verdict: FAIL (21/30, threshold 22 — one point below; bracket error is the decisive defect)

---

## Puzzle III — Castle Age — C5 (Lens Only)

**Reviewer**: Dana Young's Review Lens — no biography, no philosophy

### Lens questions applied

The lens contains 12 operational questions oriented toward visual grammar, thematic
consistency, layout direction, extraction meaning, and answer inevitability. Applied in
sequence.

**Q1 — Does the visual grammar hold consistently?**
FIRES — positive. Each chain follows the same visual format:
- Left-to-right arrow sequence.
- Missing technology shown as [? ? ?] with dotted representation in the blank.
- Age labels in parentheses below each node.
- Circle letter instruction in brackets: [N].

The grammar is consistent across all four chains. The [? ? ?] notation for the missing
technology is applied identically each time. The format does not shift between chains.
This fires in the puzzle's favor.

**Q2 — Is each image chosen for what it communicates, not just what it depicts?**
Partially applicable. This puzzle is text-based (technology chain names), not image-based.
The closest analog is the node labeling: age labels ("Feudal," "Castle," "Imperial") are
placed consistently below each node and communicate temporal ordering, which is precisely
what the solver needs. The content choices are appropriate. Fires neutrally.

**Q3 — Where does the theme dissolve?**
FIRES. The hint "Two of these chains live at the same building. That building forges both
swords and arrows." is thematically precise (the Blacksmith in AoE2 handles both melee
attack upgrades and ranged attack upgrades). Chains 1 and 2 share the Blacksmith. The hint
is structurally relevant — it signals that those two chains may require distinguishing
between attack types (melee vs. ranged) rather than just upgrade sequence knowledge.

However: Chain 4 (Fortification: Masonry → Architecture) and Chain 3 (Lumber Efficiency:
Double-Bit Axe → Bow Saw → Two-Man Saw) do not receive equivalent building identification
hints. If the building hint is useful for disambiguation in Chains 1-2, its absence in
Chains 3-4 is a consistency gap. This fires mildly as a thematic coherence concern: the
hint structure is not applied uniformly.

**Q4 — Does the layout direct the solver, or leave them searching?**
FIRES — positive. The four chains are visually separated, labeled by domain, and presented
in the same spatial format. The eye moves left-to-right along the arrow sequence, lands on
the missing node (marked [? ? ?]), and finds the circle instruction. The worksheet below
mirrors the chain order. The solver knows exactly where to look. This fires in the puzzle's
favor.

**Q5 — Is there a moment where the puzzle resolves visually?**
Does not fire strongly. This is a text-based puzzle; the visual resolution moment is weak.
The closest equivalent is the worksheet filling out to produce L-O-O-M, which is a
satisfying phonetic recognition (LOOM = a real AoE2 technology). There is no visual
confirmation — only textual. This fires as a mild negative: the visual grammar is clean
but the resolution is not a visual event.

**Q6 — Does it work without the context?**
FIRES — concern. The puzzle requires:
(a) Knowing the technology names (FLETCHER, IRON CASTING, BOW SAW, MASONRY).
(b) Knowing which technologies belong to which chains and ages.

A solver with no AoE2 knowledge cannot proceed. However, the lens asks "richer with it,
not locked behind it." For a puzzle explicitly built around AoE2 knowledge in an AoE2
puzzle hunt, being locked behind game knowledge is the design intent. The question fires
as confirmation of design intent rather than as a defect. The puzzle is appropriately
inaccessible to non-AoE players and appropriately accessible to AoE players.

**Q7 — Does the flavor text explain what the layout should have communicated?**
FIRES — positive. The flavor text ("The research queue is broken. Four technologies are
missing from the tree. The Castle Age waits.") does not explain any mechanic. The layout
communicates the mechanic (missing node in a chain → name it → index letter) without
assistance from the flavor text. The hint about the shared Blacksmith building adds
information the layout could not provide without adding building labels to every node.
This fires in the puzzle's favor with one caveat: the Blacksmith hint is thematically
useful but also creates an asymmetry (Chains 1-2 get a hint, Chains 3-4 do not).

**Q8 — Is the extraction earning its step?**
FIRES. Each extraction indexes a specific letter from the missing technology's name. The
letter positions are:
- Chain 1: FLETCHER, letter 2 = L
- Chain 2: IRON CASTING, letter 3 (ignoring spaces) = O
- Chain 3: BOW SAW, letter 2 (ignoring spaces) = O
- Chain 4: MASONRY, letter 1 = M

The instruction "ignore spaces when counting" is stated in the puzzle. This is a necessary
clarification (IRON CASTING has a space; BOW SAW has a space) that prevents ambiguity.
The extraction produces four letters — L, O, O, M — that read directly as LOOM with no
anagramming required. The extraction earns its step: each letter is uniquely forced by the
identification of the correct technology.

This is in direct contrast to Puzzle I's "not all letters matter" extraction, which requires
subset-anagramming. Puzzle III's extraction is clean and unambiguous.

**Q9 — Is the difficulty curve honest?**
FIRES — positive. The four chains range from straightforward (Chain 3: lumber efficiency
is a basic tree; Chain 4: Masonry/Architecture is a well-known fortification chain) to
moderately specific (Chain 1: knowing FLETCHER is the first-tier ranged attack upgrade;
Chain 2: knowing IRON CASTING specifically as the Castle-tier melee attack upgrade). No
chain requires arcane knowledge — each is part of standard AoE2 tech tree knowledge that
a player with moderate game time would have. The difficulty is consistent across all four
chains. This fires in the puzzle's favor.

**Q10 — Is the answer arbitrary or inevitable?**
FIRES — strongly in the puzzle's favor. LOOM is a real AoE2 technology — it is the
first research available at the Town Center, grants villagers +15 HP and +1/+2 armor, and
is universally recommended as a first-minute tech. The answer names a technology from
the game (fitting the "Technologies" theme of the puzzle) and is one of the most iconic
early-game technologies in AoE2. The answer is inevitable: it is a word that lives in
the game, belongs to the technology category the puzzle tests, and resonates thematically.
This is the strongest result from the lens in these three puzzles.

**Q11 — Does the meta feel earned by the feeders?**
Out of scope for individual feeder evaluation.

**Q12 — Would cutting this puzzle make the hunt better?**
Does not fire. The puzzle is compact (four chains, four letters, clean extraction), tests
a distinct knowledge domain (technology chains vs. civ bonuses vs. unit counters), and
produces an answer with strong thematic resonance. It belongs in the set.

**Verification (answer derivation)**:

| Chain | Domain | Sequence | Missing Technology | Letter | Extracted |
|-------|--------|----------|--------------------|--------|-----------|
| 1 | Ranged Attack | [?] → Bodkin Arrow → Bracer | FLETCHER (Feudal blacksmith) | 2nd | L |
| 2 | Melee Attack | Forging → [?] → Blast Furnace | IRON CASTING (Castle blacksmith) | 3rd (no spaces) | O |
| 3 | Lumber Efficiency | Double-Bit Axe → [?] → Two-Man Saw | BOW SAW (Castle lumber camp) | 2nd (no spaces) | O |
| 4 | Fortification | [?] → Architecture | MASONRY (Castle University) | 1st | M |

Letters: L-O-O-M = LOOM. Confirmed correct.

**Note on Chain 4 building**: The Fortification chain (Masonry/Architecture) is researched
at the University, not the Blacksmith. The puzzle's hint "Two of these chains live at the
same building. That building forges both swords and arrows." correctly implies Chains 1
and 2 are at the Blacksmith and Chains 3 and 4 are elsewhere. However, Chain 3 (Lumber:
Bow Saw) is at the Lumber Camp and Chain 4 (Fortification: Masonry) is at the University —
two different buildings. The hint does not mislead, but a solver who expects Chains 3-4 to
also share a building may be confused when they don't. Minor: does not affect solvability.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Visual grammar is consistent; layout directs the solver clearly; "ignore spaces" instruction prevents ambiguity; extraction path is unambiguous |
| Solvability | 5 | All four technology identifications are within standard AoE2 knowledge; extraction is deterministic; answer is confirmed by reading L-O-O-M directly |
| Elegance | 4 | Four chains, four letters, direct read — clean mechanism; slight asymmetry in the building hint (Chains 1-2 get it, 3-4 don't) prevents a perfect score |
| Reading Reward | 5 | AoE2 technology chain knowledge is fully load-bearing; solver must know the tech tree to proceed; LOOM as the answer names an iconic technology from the same domain |
| Fun | 5 | The aha moment when LOOM resolves is strong — it's a word in the game, it's a technology, and it's one of the first things any AoE2 player learns to research; inevitable in the best sense |
| Confirmation | 4 | Solver can confirm each chain independently; answer reads directly without anagramming; the "ignore spaces" instruction needed for IRON CASTING and BOW SAW is handled correctly; one point off because Chains 3-4 have no building confirmation path if a solver is uncertain |
| **Total** | **28/30** | |

### Issues identified (checklist-level, specific)

1. **Hint asymmetry between chain pairs**: The Blacksmith hint applies to Chains 1-2 and
   helps solvers distinguish ranged vs. melee attack upgrades. Chains 3 (Lumber Camp) and
   4 (University) share no building and receive no equivalent disambiguation hint. This is
   not a solvability problem but is a minor consistency gap in the hint structure. Consider
   adding "The other two chains live in different places — one near the forest, one inside
   your walls" or similar if the asymmetry is felt in playtesting.

2. **Chain 4 building ambiguity in hint framing**: The puzzle says "Two of these chains
   live at the same building." A solver might momentarily expect Chains 3 and 4 to also
   share a building since the hint introduces building-sharing as a category. When they
   don't, this could create brief confusion. Very minor.

3. **No visual resolution moment**: The puzzle resolves as text (four letters in a
   worksheet table). There is no visual aha — the confirmation is linguistic (recognizing
   LOOM as a technology name). For a solver who does not immediately recognize LOOM, the
   answer could feel like a random letter string. The connection is strong for AoE2 players
   but relies entirely on game knowledge for the confirmation moment.

### Verdict: PASS (28/30, well above threshold 22)

---

## Summary Table

| Puzzle | Reviewer Lens | Total | Pass/Fail | Named frameworks used | Actionable fixes | Register |
|--------|--------------|-------|-----------|----------------------|-----------------|---------|
| I — Dark Age | Dan Katz lens | 16/30 | FAIL | Wheel-of-fortuning (from checklist vocabulary) | 3 specific fixes | Diagnostic, structural |
| II — Feudal Age | Thomas Snyder lens | 21/30 | FAIL | None (lens is question-based, not framework-named) | 3 specific fixes | Constructional precision |
| III — Castle Age | Dana Young lens | 28/30 | PASS | None (lens is question-based, not framework-named) | 2 minor refinements | Visual grammar, experience |

**Named frameworks invoked**: 1 (wheel-of-fortuning, from the Katz lens checklist implicit
vocabulary — the lens does not name it but the behavior it describes maps directly to it).

**Total actionable fixes across three puzzles**: 8 specific items (3 + 3 + 2).

**Language register**: All three reviews operate in diagnostic mode — specific, operational,
checklist-level. No biographical authority is claimed. No design philosophy is invoked
beyond what the lens questions themselves encode. The reviews identify what fires, what
does not fire, and what the firing implies for the puzzle as submitted.

---

## Condition Notes (for ablation study)

This condition (C5) applied only the "## Review Lens" bulleted checklist from each profile.
No biographical context, credential framing, or design philosophy was used to interpret or
weight the lens questions. The only identity context retained is the attribution label
required for output format.

Observable effects of this restriction:

1. **Framework vocabulary is impoverished**: The Katz lens implies terms like
   "wheel-of-fortuning" and "mettleneck" through its behavioral descriptions but does not
   name them. Without the biography and published criticism context, these concepts must be
   paraphrased rather than named. The review for Puzzle I circles the wheel-of-fortuning
   problem without being able to name it precisely.

2. **Lens questions at hunt scope are systematically inapplicable**: The Katz lens contains
   4 of 12 questions operating at hunt scope (meta short-circuiting, hint economics, unlock
   structure, audience size). With only a single feeder puzzle as input, these questions
   produce no signal. The Snyder and Young lenses are more feeder-portable (11 of 12 and
   10 of 12 questions fire on individual puzzles, respectively).

3. **Scoring is consistent with expectations**: The highest-quality puzzle (III) receives
   the highest score under any condition. The systematic defect in Puzzle II (bracket
   diagram error) is caught cleanly by Snyder's visual design question. The structural
   failure in Puzzle I (subset extraction) is caught cleanly by Katz's "would he want to
   solve this" question. The lens checklist, without any biography, is sufficient to
   identify all three major defects found in the full-profile conditions.

4. **Tone is flatter without biographical framing**: Without the credential context, the
   reviews read as operational assessments rather than expert judgments. The conclusions
   are the same; the rhetorical authority differs.
