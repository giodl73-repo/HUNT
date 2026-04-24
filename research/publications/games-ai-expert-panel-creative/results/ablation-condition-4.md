# Ablation Study — Condition 4: Design Philosophy Only

**Reviewer context:** Design Philosophy section of each profile, and nothing else.
**Condition:** C4 = no biography, no credentials, no operational review lens, no checklist — worldview only.
**Puzzles reviewed:** I (Dark Age), II (Feudal Age), III (Castle Age)
**Known answers:** I = CASTLE, II = ONAGER, III = LOOM

---

## Puzzle I — Dark Age · C4 (Philosophy Only)

**Reviewer:** Dan Katz — Design Philosophy context

### How the philosophy framed the evaluation

Dan Katz thinks about puzzle hunts as contracts. The structural decision embedded in Puzzle I — eight identification tasks, eight letter extractions, but "not all of them matter" — is a contractual claim. The solver agrees to identify eight civilizations by their bonuses. The designer promises that this effort will yield a word. The question the philosophy asks is: does the contract hold? Is the promise kept cleanly, or does it quietly shift the terms midway through?

His core analytical instinct is architectural. Before anything else: is the whole proportionate? Puzzle I is one puzzle in a five-puzzle hunt. Its job is to contribute CASTLE to the meta. The mechanism — civ identification plus letter extraction — should do that job cleanly, without extra structural cost. The philosophy asks whether the structural decisions serve the experience or impose on it.

The named concept most relevant here is from his body of work on hunt architecture: the distinction between hunts that are one thing versus collections of parts. A puzzle, like a hunt, should be one thing. Puzzle I is a knowledge quiz with an extraction step appended. The question is whether those two things are unified — whether the extraction is inherent to the identification mechanic — or whether the extraction is a structural imposition that lives on top of the quiz like a layer added later.

The "not all letters matter" instruction is the load-bearing architectural choice. It tells the solver: you will do eight identifications, extract eight letters, and then do a second task of pattern-matching or anagramming to find the word. This is two puzzles joined at the extraction step. The contractual question is whether the solver was told they were signing up for that. The flavor text says "Eight letters. Not all of them matter. Find the word." — which is honest disclosure, but does not eliminate the structural friction of having excess letters with no clear rule for discarding them.

Katz's philosophy about hunt length and proportionality applies at the puzzle level too: the right amount of mechanism for the job. Eight clues for a six-letter answer means two identifications produce no payoff. The solver who correctly identifies, say, FRANKS for Bonus G and extracts F — and then finds F is not part of CASTLE — has done real work for nothing. A solver who takes the contract seriously will feel that the terms were slightly uneven.

The philosophy does not condemn this structure outright. Katz's view of puzzle hunts as contracts is not a demand for perfect efficiency — it is a demand for honesty. If the contract says "eight letters, not all matter," the solver should be able to rationally identify which letters matter without guessing. Here, no rule is given for elimination. The solver must try to form a word from a random subset, which introduces a guessing step not previously disclosed.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | The identification mechanic is transparent. The worked example is good. The "not all letters matter" disclosure is honest but creates a second task (find the word from 8 letters, 2 of which are noise) that is underspecified. A solver knows what to do step by step until the very last step. |
| Solvability | 3 | The civ identification layer is high-quality and largely sound — CELTS, JAPANESE, BRITONS, MONGOLS, FRANKS are all clean calls. Two bonuses (Bonus C and Bonus F) have letter-count discrepancies between the most plausible civ answer and the underscores provided: the best AoE2 match for Bonus C (escalating TC/Dock work rate) is PORTUGUESE at 10 letters, not 8; the best match for Bonus F (villagers +5 carry, military 11% faster) is MALIANS at 7 letters, not 6. These represent real blockers for a knowledgeable solver who will write the correct civ name and find it does not fit the blank. The pathway to CASTLE is obscured by these two slots. |
| Elegance | 3 | The mechanic would be elegant if all six extracted letters were used directly. The "not all matter" construction forces a guessing or anagramming step that is structurally separate from the identification work. Two mechanisms where one would do. |
| Reading Reward | 5 | This is the puzzle's real strength. AoE2 knowledge is fully load-bearing. Every bonus is specific to a single civ, stated at game-accurate precision (percentages, unit categories, age restrictions). A solver who has not played AoE2 cannot proceed. The knowledge tested is genuine domain expertise. |
| Fun | 3 | The identification tasks are satisfying — recognizing "infantry moves 15% faster" as CELTS is a genuine aha for a player who knows the civs. The deflation comes at the end: knowing you have eight letters but not knowing which two to discard, with no principle to guide selection, turns the final step into a search problem rather than an insight. |
| Confirmation | 3 | CASTLE is thematically resonant (it is literally the next age up in the game) and self-confirming once found. But because two civ slots have ambiguous answers, the solver cannot confirm the extraction path. The final word can be guessed from partial information, which means confirmation bypasses rather than validates the mechanism. |
| **Total** | **21/30** | |

### Issues identified (worldview-level, not checklist-level)

**Structural unity.** The puzzle is two things: a knowledge quiz and a "find the word among eight letters." These are sequentially joined but not architecturally unified. In Katz's philosophy, a puzzle should be one thing — the extraction should be inherent to the identification mechanic, not appended after it. The cleanest version of this puzzle would have exactly six civilizations, each contributing one letter, reading CASTLE in order. That version is one thing. The current version is two things with a seam between them.

**Contractual incompleteness at the extraction step.** The solver is told "not all letters matter" but given no principle for elimination. This breaks the contract at the last move: the designer has obligated the solver to complete the identification work but has not completed their half of the bargain by providing a clear extraction rule. In Katz's framework, this is the kind of structural decision that promises something to solvers and then quietly shifts the terms. Disclosure ("not all matter") is necessary but not sufficient — the rule for deciding which matter should be available to the solver.

**Blank-count accuracy.** Two bonus slots have word-length mismatches between the most knowledgeable answer and the blank provided. This is not a philosophy-level failure alone — it is an accuracy failure that the philosophy makes visible, because Katz's standard is that the contract must be kept. A solver who knows the game and writes the correct civ name in the wrong-length blank has been failed by the design, not by their knowledge.

### Verdict: FAIL (21/30, below threshold of 22)

The knowledge layer is excellent. The extraction architecture has a structural seam and two accuracy failures that break the contract with the solver.

---

## Puzzle II — Feudal Age · C4 (Philosophy Only)

**Reviewer:** Thomas Snyder — Design Philosophy context

### How the philosophy framed the evaluation

Thomas Snyder's philosophy begins with a diagnostic: if a computer can generate your puzzle, your puzzle isn't finished. The constructor's hand must be visible in every decision. Applied to Puzzle II, this asks: is the bracket constructed, or assembled? Is the specific sequence of matchups — who fights whom, in what order — a deliberate design choice that shapes the extraction path, or an arbitrary arrangement that a randomizer could have produced?

The answer here is that the bracket is constructed. It is not assembled arbitrarily. The designer chose matchups that require the solver to apply specific rules (siege beats archers, cavalry beats siege, anti-cavalry beats cavalry) in a specific order, and then placed one deliberate exception — the Cataphract — exactly where its effect on the extraction is maximized. QF-3 is the load-bearing match: if the solver applies the generic rule (anti-cavalry beats cavalry), they put Halberdier and extract the wrong letter. The puzzle is testing whether the solver knows the Cataphract's specific armor class, not just the general counter hierarchy. That is constructed difficulty, not noise-based difficulty.

Snyder's philosophy draws a sharp line between difficulty that comes from a technique being subtle and difficulty that comes from signal buried in irrelevant information. Puzzle II is entirely on the correct side of this line. The hard part — the Cataphract exception — is hard because it requires genuine knowledge of a specific unit's properties, not because the puzzle is cluttered. The bracket is clean, the counter rules are stated, the exception is flagged by the flavor text ("One matchup breaks the rule you expect"). The construction is honest about where the test lives.

The theme-and-structure relationship that matters most to Snyder is also intact here. Theme is not decoration; a real theme shapes the structure of the puzzle itself. The tournament bracket is not a container for AoE2 content — it is an AoE2 mechanic. Players who have spent any time in team games or build-order practice understand counter logic as the fundamental grammar of unit composition. The bracket is how that grammar gets operationalized. Theme and mechanic are the same thing.

The Just One Cell philosophy is also relevant: isolate one technique, make every other approach useless. Puzzle II isolates counter knowledge. A solver who knows counter logic but not the Cataphract exception cannot fully solve it. A solver who knows the Cataphract exception and proceeds mechanically will extract ONAGER without needing to think about the bracket structure itself. The puzzle rewards exactly the knowledge it claims to require.

The design question Snyder's philosophy forces is about the solve path: does it hold one direction? Here it does. Every match has exactly one correct outcome given the rules provided. There is no branching, no equally valid alternative. The sequence QF-1 through SF-2 is forced. That is the standard.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | The bracket is visually clean. Counter rules are stated explicitly. The extraction instruction is unambiguous (six letters from six slots). The worked example is replaced by the bracket diagram itself, which is sufficient. The flagged exception is disclosed in the flavor text without giving the answer away. Nothing is ambiguous. |
| Solvability | 5 | Every match resolves to one correct outcome. QF-1 (siege beats archer), QF-2 (anti-cavalry beats cavalry — standard case), QF-3 (Cataphract exception, requiring specific knowledge), QF-4 (siege beats infantry), SF-1 (siege beats infantry), SF-2 (cavalry beats siege). Each step is individually verifiable. The path to ONAGER is clean and unambiguous. |
| Elegance | 5 | The extraction is a direct output of the tournament: winner names, indexed letter positions. Every match produces a letter; every letter is used. No noise. No distractors. The exception is structurally necessary — without it, the Cataphract match would be deterministic from general rules alone, and the Cataphract's specific design (armor against anti-cavalry attacks) would go unexercised. The exception is not a trick; it is the point. |
| Reading Reward | 5 | Counter logic is the grammar of AoE2 play. A solver who doesn't know that siege beats archers, that pikemen beat knights, and that the Cataphract is a cavalry unit designed to resist its counter cannot solve this puzzle. The knowledge is load-bearing at every step. ONAGER itself — the answer — is one of the units in the puzzle, which adds a final layer of coherence: the winner of the tournament names the tournament's most powerful entry. |
| Fun | 5 | The tournament format creates natural narrative momentum. Each match is a small payoff. The Cataphract exception is the aha — the moment where knowing something specific about a single unit flips the result. The answer ONAGER lands cleanly. The entire puzzle from first read to final letter extraction is an enjoyable demonstration of AoE2 counter knowledge. |
| Confirmation | 5 | ONAGER is fully confirming at multiple levels. The solver can verify each match independently. The letters O-N-A-G-E-R come from six different slots with no distractors. The answer is a unit name from the game, consistent with the tournament theme. Every step of the derivation is independently checkable. |
| **Total** | **30/30** | |

### Issues identified (worldview-level, not checklist-level)

No structural failures identified. The puzzle holds Snyder's diagnostic: the constructor's hand is visible in every decision. The bracket sequence, the matchup selection, the placement of the Cataphract exception, and the letter indexing all appear to be deliberate choices that produce a specific outcome. A randomized bracket would not produce ONAGER this cleanly — the letter positions are calibrated to the specific units that win each match.

One observation, not a failure: the puzzle requires the solver to understand that the Final match is excluded from extraction. This is explicitly stated ("your extraction uses only the six winners from the Quarterfinal and Semifinal rounds") but creates a mild structural oddity — why run the Final at all if it does not contribute? The Final contributes narrative completion but no letter. In Snyder's framework, elements that can be removed without consequence were not design decisions. The Final could be argued either way: it provides closure (someone wins the tournament) or it is an extra step that adds nothing to the mechanism. This is a minor note, not a blocking issue.

### Verdict: PASS (30/30)

A puzzle that knows exactly what it is testing and gives the solver exactly that. The Cataphract exception is the design — isolating one technique, making every other approach produce the wrong result. This is construction, not assembly.

---

## Puzzle III — Castle Age · C4 (Philosophy Only)

**Reviewer:** Dana Young — Design Philosophy context

### How the philosophy framed the evaluation

Dana Young does not start with a mechanic. She starts with a world. Her question is never "what puzzle type fits this theme?" — it is "what does it feel like to be in this place, and what would you have to do to live there?" Applied to Puzzle III, this asks: does the missing-technology mechanic emerge from the experience of inhabiting AoE2's Castle Age, or was it chosen as a competent format and dressed in AoE2 clothing?

The Castle Age is, in AoE2, the age of decision and accumulation. You queue research. You choose what to advance. The technology tree is not decoration — it is the cognitive architecture of the game. Players who understand the Castle Age understand it as a sequence of choices made under constraint: you cannot skip Forging if you want Blast Furnace. The chain is the game. A puzzle where the mechanic is "identify the missing link in the chain" is not representing this experience — it is this experience. The solver is doing what a Castle Age player does: reading the research tree and identifying what is available at each stage.

Dana's philosophy holds that the answer is the last move in the logic. She designs toward a word that names the experience you just had — not what you produced, but what you were while solving. The answer here is LOOM. LOOM is a technology from the Town Center — specifically the very first technology a player researches in the Dark Age to protect their villagers. It is not a Castle Age technology. This is the moment worth examining: after four chains of Castle Age upgrade research, the answer names a technology that lives in a completely different age and building. Does this reframe the experience, or does it feel arbitrary?

The answer is: LOOM reframes the experience. A solver who finishes the four chains — ranged attack, melee attack, lumber efficiency, fortification — and extracts LOOM will have a specific recognition if they know AoE2: this is the first thing you research in every game. The Castle Age puzzle ends with a word that names the foundation, the beginning. That is a meaningful payoff. It does not name what the solver was doing during the solve (researching Castle Age upgrades), but it names something structurally adjacent to it — the original protection, the earliest commitment. Whether this qualifies as "inevitable" rather than "arbitrary" in Dana's terms depends on whether the solver experiences it as a reframe or as a random word.

Her thinking about visual structure is also relevant here. The chains are rendered as ASCII diagrams with arrows. The visual grammar states the same thing four times: a sequence with a gap. This is consistent. The solver learns the format once and applies it three more times. The visual structure does not mislead — it communicates exactly what is present and absent. Each gap is the same kind of gap. Dana's standard is that the visual grammar must hold consistently, and here it does.

The extraction instruction — "ignore spaces when counting" — is the one moment where the visual design requires a verbal correction. Chain 3 (BOW SAW) and Chain 2 (IRON CASTING) both have spaces in their names that the solver must mentally remove before counting. The instruction handles this, but it is a place where the visual and the logic briefly diverge: the names as written and the names as counted are different objects. This is a minor seam, not a failure.

The thematic coherence across all four chains merits attention. Chains 1 and 2 both live at the Blacksmith ("that building forges both swords and arrows"). The flavor text announces this as a deliberate structural element. Dana's philosophy values theme that shapes structure — and this annotation does shape the solve: the solver who internalizes "two chains at the Blacksmith" knows to look for the paired relationship when identifying missing technologies. The building location is load-bearing, not decorative.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | The chain format is instantly readable. Each chain has a clear direction, a clear gap, and a clear instruction. The "ignore spaces when counting" rule is stated once and applies consistently. The worksheet maps directly to the puzzle structure. No step is ambiguous. |
| Solvability | 5 | The four missing technologies are identifiable by any player who knows the AoE2 tech tree: FLETCHING (ranged attack, Feudal), IRON CASTING (melee attack, Castle), BOW SAW (lumber, Castle), MASONRY (fortification, Castle). Each chain provides enough context (preceding and following technology, description of the upgrade's effect, age labels) to identify the missing link. The path to LOOM is clean: L(FLETCHING[2]), O(IRONCASTING[3]), O(BOWSAW[2]), M(MASONRY[1]). |
| Elegance | 5 | Four chains, four letters, four contributions, zero noise. Every identified technology contributes a letter. The "ignore spaces" rule is a minor roughness but does not disrupt the mechanism. The chain structure is the natural grammar of the AoE2 tech tree — it is not imposed on the content; it is how the content is organized in the game. |
| Reading Reward | 5 | AoE2 knowledge is fully load-bearing. A solver who does not know that FLETCHING precedes Bodkin Arrow, or that IRON CASTING sits between Forging and Blast Furnace, cannot solve the puzzle. The technology names are precise and match actual in-game names. The game knowledge tested is specific to the research tree — not general AoE2 awareness but actual tree navigation. |
| Fun | 4 | The chain format produces consistent small satisfactions: each missing technology is a click of recognition for a player who knows the tree. The answer LOOM lands as a genuine surprise with retroactive meaning (it is the earliest technology, completing a kind of loop from Dark Age foundation to Castle Age advancement). A score of 4 rather than 5 because the four chains are structurally identical — the format teaches itself once and then repeats. The solver's experience does not change between Chain 1 and Chain 4. A fifth chain of a different type, or a structural variation between chains, would elevate the fun further. |
| Confirmation | 5 | LOOM is fully confirming. It is a real AoE2 technology with specific meaning (Dark Age Town Center upgrade, protects villagers). The solver can verify all four chains independently. The letters L-O-O-M come from four different chains with no distractors. The answer's non-Castle-Age origin is surprising but not arbitrary — it names a specific technology with a specific place in the game's logic. |
| **Total** | **29/30** | |

### Issues identified (worldview-level, not checklist-level)

**Structural uniformity.** All four chains use the same format: a gap in a sequence, identify the missing technology, extract one letter. The puzzle does what it does four times in the same way. In Dana's philosophy, a mechanic should be chosen because it is the form this world deserves — and four identical instances is a claim that this one form is sufficient to render the entire Castle Age research experience. It is a reasonable claim, but it is also the puzzle's ceiling. A solver who finishes Chain 4 is doing the same thing they did in Chain 1. The experience does not deepen or shift. This is not a failure — the puzzle works cleanly — but it represents an opportunity for structural variation that was not taken.

**The answer's relationship to the theme.** LOOM names the experience from outside the Castle Age. Dana's philosophy asks whether the answer is inevitable — whether it reframes everything before it. LOOM is surprising (it is not a Castle Age technology) and meaningful (it is a foundational game technology), but the connection to the Castle Age framing is not directly stated. The solver must supply the contextual reasoning: "I researched four Castle Age technology chains and the answer is the oldest technology in the game." Whether this is an elegant contrast or a loose connection depends on the solver. The puzzle does not explicitly bridge this — it relies on the solver's AoE2 knowledge to experience the payoff. For solvers with that knowledge, the answer will land. For solvers at the edge of AoE2 familiarity, LOOM may feel like a competent answer rather than an inevitable one.

### Verdict: PASS (29/30)

The mechanic is the content, not a representation of it. The castle Age research tree is rendered as its own grammar, and the solver navigates it the way a player navigates it. The answer LOOM arrives with genuine thematic resonance for a player who knows the game. One point deducted for structural uniformity across four identical chain formats.

---

## Summary Table

| Puzzle | Reviewer | Score | Verdict |
|--------|----------|-------|---------|
| I — Dark Age | Dan Katz (philosophy) | 21/30 | FAIL |
| II — Feudal Age | Thomas Snyder (philosophy) | 30/30 | PASS |
| III — Castle Age | Dana Young (philosophy) | 29/30 | PASS |

---

## Named Frameworks Count

Philosophy-derived frameworks invoked across all three reviews:

1. **Puzzle hunt as contract** (Katz) — structural promise made to solvers; disclosure vs. completeness distinction
2. **Architectural proportionality** (Katz) — right amount of mechanism for the job; two things where one would do
3. **If a computer can generate it, it isn't finished** (Snyder) — diagnostic for construction vs. assembly
4. **Constructed vs. noise-based difficulty** (Snyder) — technique difficulty vs. signal-buried-in-clutter distinction
5. **Theme and structure as the same thing** (Snyder/Young) — theme that shapes structure vs. theme as label
6. **Just One Cell isolation principle** (Snyder) — isolate one technique, make every other approach useless
7. **The world is the puzzle** (Young) — mechanic is the content, not a representation of it
8. **The answer names the experience** (Young) — answer as the last move in the logic, reframing what came before
9. **Fidelity, not versatility** (Young) — every world deserves the form that lets you inhabit it; structural uniformity as ceiling

Total named frameworks: **9**

---

## Actionable Fixes Count

| Puzzle | Fix | Priority |
|--------|-----|----------|
| I | Verify Bonus C: blank count (8) vs. most plausible AoE2 answer (PORTUGUESE, 10 letters) — correct the blank or replace the bonus | Critical |
| I | Verify Bonus F: blank count (6) vs. most plausible AoE2 answer (MALIANS, 7 letters) — correct the blank or replace the bonus | Critical |
| I | Restructure extraction to eliminate "not all letters matter" by using exactly 6 civs, all contributing in order | Design |
| II | Consider whether the Final match is structurally necessary or can be removed/retained as narrative closure | Minor |
| III | Consider structural variation between chains to prevent four-identical-instances uniformity | Design |

Total actionable fixes: **5** (2 critical, 2 design, 1 minor)

---

## Language Register Note

Condition 4 produces evaluations written from philosophical standpoints — worldview claims about what design is, what puzzles should do, what the solver is owed. The language is analytical but abstract: "contractual incompleteness," "architectural unity," "constructed difficulty," "theme and mechanic as the same thing." These phrases come directly from the Design Philosophy sections and reflect how each reviewer thinks about design at the level of principle rather than at the level of specific checks.

This register has a specific character: it identifies failures at the level of design orientation (what kind of puzzle is this trying to be?) rather than at the level of operational detail (does this clue lead to the correct answer?). The result is that worldview-level failures are clearly named — Puzzle I's contractual seam, Puzzle III's structural uniformity ceiling — while the operational failures that could only be caught by a systematic checklist (specific blank-count mismatches, exact letter-position verification) are either noticed incidentally or noted with less precision.

The contrast with Conditions 1–3 is predictable: C4 identifies the same critical Bonus C and Bonus F issues in Puzzle I (because they are visible to anyone who attempts the puzzle in good faith), but frames them as a contract failure rather than a verification failure. The diagnosis is compatible but the vocabulary is different. Puzzle II and III are evaluated with high scores in C4 because both puzzles are structurally sound at the worldview level — the mechanisms are the content, the difficulty is technique-based, the answers name the experience. These are exactly the properties that philosophy-level analysis is calibrated to detect.
