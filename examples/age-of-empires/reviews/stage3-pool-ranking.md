# Stage 3 Review — Pool Ranking

**Reviewers:** Dan Katz (structure & pacing) | Mike Selinker (narrative & experience) | Alex Rosenthal (accessibility & wonder)

**Document reviewed:** PUZZLE-POOL.md (Stage 3 — 15 candidates across 5 slots + 2 cross-slot)

**Date:** 2026-02-27

---

## Scoring Rubric

Each reviewer scores their top pick per slot on four axes (1-5 scale):

| Axis | What it measures |
|------|-----------------|
| **Riven Standard** | Does the puzzle IS the AoE domain, not trivia overlaid on a generic format? |
| **Interlock** | Does it cross-reference with other puzzles or contain internal cross-constraints? |
| **Fun** | Would a solver enjoy this? Does it create a moment? |
| **Accessibility** | Can a casual AoE player (not a competitive player) solve it with the reference sheet? |

---

## Reviewer 1: Dan Katz — Structure & Pacing

### Slot I (Dark Age / Civilizations): I-C — Unique Unit Lineup

The pool description says it best: "This IS what AoE players do." Recognizing units from their stats and abilities, then mapping them to civilizations, is the core identification loop of AoE multiplayer. When you see Mangudai on the field, you know you are fighting Mongols. That is not trivia. That is pattern recognition under pressure.

I-A (Bonus Matcher) is the safer pick — lower difficulty, explicit interlock. But the interlock it claims ("two bonuses reference each other") is weak. Cross-referencing within a single matching puzzle is barely interlock; it is just internal consistency. I-C forces genuine deduction: the stat descriptions are ambiguous until the solver narrows down the unit class, then the special ability, then the civilization. That three-step chain is a real puzzle, not a lookup.

I-B (Wonder Identifier) is charming but brittle. "First letters of civ names spell the answer" is the most transparent extraction mechanism in puzzle design. The solver will see it coming after the second Wonder.

| Axis | Score | Notes |
|------|-------|-------|
| Riven Standard | 5 | Identifying units from stats IS the AoE experience |
| Interlock | 2 | No explicit cross-puzzle connections — weakest axis |
| Fun | 4 | The "police lineup" framing adds personality |
| Accessibility | 3 | Requires knowing unique units specifically, not just generic unit types |
| **Total** | **14/20** | |

**Risk:** Difficulty 3 in the opening slot. The pool author rated this a 3 while the other two candidates are 2s. Opening with the hardest option could stall solvers before they build momentum. Mitigated by the reference sheet (Stage 2 Action Item 2), but still a pacing concern.

### Slot II (Feudal Age / Units): II-B — Rock-Paper-Scissors Tournament

This is the correct pick by a wide margin. A single-elimination bracket is a self-contained deduction engine. The solver is not just recalling counter relationships — they are APPLYING them across seven matchups, tracking exceptions, and handling the trick matchup where a unique unit breaks the normal rules. That trick is the aha. The extraction (winners' unit-type initials) is clean and non-obvious until the solver looks for it.

II-A (Counter Circle) has a nice broken-link concept, but six units in a circle is small. The solver traces the circle, finds the break, done. There is not enough surface area for real deduction. II-C (Upgrade Path) is pure recall — fill in missing names from known sequences. That is a quiz, not a puzzle.

| Axis | Score | Notes |
|------|-------|-------|
| Riven Standard | 5 | Counter logic IS the tactical layer of AoE |
| Interlock | 3 | The trick matchup forces the solver to hold exceptions in mind across the bracket |
| Fun | 5 | Tournament brackets are inherently engaging; the trick matchup adds drama |
| Accessibility | 4 | Basic counters (infantry < cavalry < archers) are AoE 101; the trick is the challenge |
| **Total** | **17/20** | |

**Strongest candidate in the entire pool.** The mechanism is varied (bracket, not matching or gap-fill), the aha is clear (the rule-breaking unit), and it passes both the Riven Standard and Snyder's Computer Test (the trick matchup requires judgment, not just formula application).

### Slot III (Castle Age / Technologies): III-A — Tech Tree Gap-Fill

The tech tree IS AoE. Every player has stared at it. Navigating dependency chains (Fletching -> Bodkin Arrow -> ???) IS what you do in Castle Age when you are deciding your research path. The internal interlock — two techs whose dependencies constrain each other — means the solver cannot brute-force each blank independently. They must reason about the tree as a system.

III-B (Research Cost Detective) is too narrow. Knowing that Wheelbarrow costs 200F 100G is deep memorization, not understanding. III-C (Age Advancement Requirements) is a reasonable puzzle but has a lower ceiling — matching three transitions to their requirements is a small puzzle that will feel slight after the tournament bracket.

| Axis | Score | Notes |
|------|-------|-------|
| Riven Standard | 5 | Navigating the tech tree IS Castle Age gameplay |
| Interlock | 4 | Two mutually constraining dependencies create real internal interlock |
| Fun | 3 | Competent but not spectacular; filling blanks in a tree is workmanlike |
| Accessibility | 3 | Tech tree knowledge is deeper than unit counters — reference sheet essential |
| **Total** | **15/20** | |

### Slot IV (Imperial Age / Maps): IV-B — Resource Map

This is where I defer to Selinker. Drawing on the page is a mechanism no other puzzle in this hunt uses. The pool has matching (Slot I), brackets (Slot II), gap-fill (Slot III), and either math or sequencing (Slot V). If Slot IV is also an identification puzzle (IV-A) or a pathfinding puzzle (IV-C), we have a mechanism monotony problem. IV-B breaks the pattern by introducing a physical/spatial mode. That alone earns it the slot.

The mechanism is also genuinely clever. Resource spawn positions on a circular map are not random — they follow known patterns. Plotting them reveals letter shapes. That is an aha that rewards AoE knowledge with a visual surprise.

IV-A (Map Identifier) is too thin — five identifications with first-letter extraction. IV-C (Terrain Puzzle) has interlock potential ("the path depends on which unit type you're moving — connects to Puzzle II") but the mechanism (pathfinding on an ASCII grid) is generic. It could be any game with terrain movement rules.

| Axis | Score | Notes |
|------|-------|-------|
| Riven Standard | 4 | Resource spawns are AoE-specific, but the "plot dots that spell letters" mechanic is general |
| Interlock | 2 | No explicit connections to other puzzles |
| Fun | 5 | Drawing, spatial reasoning, visual reveal — this is the hunt's physical moment |
| Accessibility | 3 | Requires knowing resource spawn patterns for specific maps |
| **Total** | **14/20** | |

### Slot V (Post-Imperial / Strategy): V-B — Economy Puzzle

V-A (Build Order) is a trap. It requires competitive AoE knowledge that most players do not have. Rosenthal flagged this in Stage 2 and the pool author agrees. V-C (Historical Battle) is low-interlock trivia — "identify campaign scenarios from descriptions" is identification again, a mechanism we already have in Slots I and IV-A.

V-B (Economy) is the right choice. Villager allocation under resource constraints IS AoE strategy at its most universal. Every player — competitive or casual — has faced the question "how many villagers do I put on gold?" The math is the puzzle, but the math is not just computation: the 40-villager constraint means the solver must OPTIMIZE, not just calculate. That optimization step is the aha. The A1Z26 extraction is clean.

However, V-B carries risk. The resource math must be carefully calibrated so the numbers work out to valid A1Z26 values (1-26). If the optimal allocation is 17 on food, 14 on gold, 9 on stone, those map to Q, N, I — which must form a word. The extraction is downstream of the math, meaning the puzzle author must reverse-engineer the resource costs and gather rates to produce a specific answer. This is a construction challenge, not a design problem.

| Axis | Score | Notes |
|------|-------|-------|
| Riven Standard | 5 | Resource allocation IS the strategic core of AoE |
| Interlock | 2 | Self-contained math puzzle — no cross-puzzle links |
| Fun | 3 | Satisfying to solve but not a showstopper |
| Accessibility | 4 | Any player who has managed an economy understands the problem |
| **Total** | **14/20** | |

### Cross-Slot: X-1 — Taunt Decoder

The nostalgia factor is real. "11" (the universal AoE taunt for "Don't resign" or, more commonly, a taunt used to mock opponents) is AoE cultural knowledge that every player shares. But this is a pure lookup puzzle. There is no deduction, no interlock, no aha. The solver either knows the taunts or they do not. It fails the No Computation Without Deduction principle and Snyder's Computer Test.

**Verdict: Do not include X-1 in the main 5.** Consider it as a bonus/warm-up if the hunt needs a low-friction entry point, or fold a taunt reference into another puzzle (e.g., one of the Monk's puzzle intros could reference a taunt number that becomes relevant in the meta).

### Katz Lineup

| Slot | Pick | Score |
|------|------|-------|
| I | I-C (Unique Unit Lineup) | 14/20 |
| II | II-B (Tournament Bracket) | 17/20 |
| III | III-A (Tech Tree Gap-Fill) | 15/20 |
| IV | IV-B (Resource Map) | 14/20 |
| V | V-B (Economy Puzzle) | 14/20 |

**Mechanism variety check:** Identification-from-stats (I) / Bracket deduction (II) / Dependency gap-fill (III) / Spatial drawing (IV) / Optimization math (V). Five distinct mechanisms. **Passes.**

**Concern:** Interlock across the hunt is weak. Only III-A has strong internal interlock. No puzzle explicitly feeds information to another. The crossword meta is the only cross-puzzle link. This was flagged in Stage 2 (principle compliance: AT RISK). For a 5-puzzle hunt this is acceptable — over-interlocking would feel forced — but it means the meta must do more structural work.

---

## Reviewer 2: Mike Selinker — Narrative & Experience

### Slot I (Dark Age / Civilizations): I-A — Bonus Matcher

I will take the other side of Katz on this one. The opening puzzle sets the emotional register for the entire hunt. Starting at difficulty 3 with "identify units from raw stat blocks" is starting with homework. The Monk says "Eight civilizations stand before you. Each one carries a gift no other has. Name them." That voice calls for elegance, not forensics.

I-A (Bonus Matcher) is the elegant choice. "Shepherds work 25% faster" — the solver reads that and thinks "I know this civilization." The aha is recognition, not analysis. The interlock is real: two bonuses that reference each other create a satisfying "wait, those are connected" moment. And the difficulty 2 rating means the solver builds confidence before the hunt ramps up.

I-C is a fine puzzle. It is not a fine opening.

| Axis | Score | Notes |
|------|-------|-------|
| Riven Standard | 4 | Civ bonuses are iconic but matching is a common format |
| Interlock | 4 | Two cross-referencing bonuses create a genuine constraint |
| Fun | 4 | Recognition puzzles are inherently satisfying — "I KNOW this" |
| Accessibility | 5 | Most AoE players remember at least some civ bonuses |
| **Total** | **17/20** | |

### Slot II (Feudal Age / Units): II-B — Rock-Paper-Scissors Tournament

Agreement with Katz. The tournament bracket is the clear winner. But let me add the narrative angle: a bracket is a STORY. Each matchup is a scene. The solver watches cavalry charge into pikemen and knows the outcome. They watch the trick matchup and feel uncertainty. The bracket has rising tension — winner's bracket narrows, stakes increase, the finals matter. This is not just a deduction engine. It is a narrative engine.

The "Dinner Party" potential noted in the pool is real. "I predicted an 8-unit tournament bracket using Age of Empires counter logic" — that is a sentence that works at a dinner party. But it is a B+ dinner party story, not an A. The real Dinner Party puzzle is in Slot IV.

| Axis | Score | Notes |
|------|-------|-------|
| Riven Standard | 5 | Counter logic is the tactical heart of AoE |
| Interlock | 3 | Internal bracket logic; trick matchup creates a "hold that thought" moment |
| Fun | 5 | Brackets are universally engaging — everyone has filled out March Madness |
| Accessibility | 4 | Basic counters accessible; the trick is learnable |
| **Total** | **17/20** | |

### Slot III (Castle Age / Technologies): III-A — Tech Tree Gap-Fill

Agreement with Katz, but I will note the narrative opportunity. The Monk's intro — "The research queue is broken. Six technologies are missing from the tree. The Castle Age waits." — frames this as repair, not recall. The solver is fixing something. That is a more engaging narrative than "fill in the blanks." The broken tech tree is a story: something went wrong, and you are the one who knows how to fix it.

The internal interlock (two techs constraining each other) maps to a narrative beat: "Wait, if THIS tech goes here, then THAT one can only go..." — the solver experiences a cascade of realizations, not a sequence of lookups.

| Axis | Score | Notes |
|------|-------|-------|
| Riven Standard | 5 | The tech tree IS AoE at the strategic layer |
| Interlock | 4 | Mutual dependencies create narrative tension |
| Fun | 3 | Satisfying but not spectacular |
| Accessibility | 3 | Requires tech tree knowledge; reference sheet helps |
| **Total** | **15/20** | |

### Slot IV (Imperial Age / Maps): IV-B — Resource Map

**This is the Dinner Party puzzle.** Full stop. Let me explain why.

The pool description says "same as the Star Chart in the reference project." The Star Chart was the single most discussed puzzle in the Joker test runs. People said: "I drew the answer in the sky." That is a sentence that makes non-puzzlers lean forward. Now transpose it: "I plotted resource spawns on Age of Empires maps and the dots spelled a word." Same energy. Same physical engagement. Same visual reveal.

Here is what makes it work as narrative. The Monk tells you where resources are. You draw dots. You do not know what you are drawing until you step back and see letter shapes emerge from the resource positions. The aha is VISUAL. It happens to your eyes, not to your brain. You look at what you drew and realize you have been writing a word without knowing it. That is magic.

No other puzzle in this pool does this. I-A is recognition. II-B is deduction. III-A is completion. V-B is optimization. IV-B is DISCOVERY. It is the hunt's physical, tactile, visual centerpiece.

The concern is difficulty 3. Drawing puzzles can go wrong if the instructions are ambiguous or the letter shapes are not clean. The construction must be immaculate — resource positions must unambiguously form recognizable letters. A sloppy "is that a B or a D?" moment would kill the experience.

| Axis | Score | Notes |
|------|-------|-------|
| Riven Standard | 4 | Resource spawns are AoE-authentic; the letter-spelling is an overlay |
| Interlock | 2 | Self-contained — no cross-puzzle links |
| Fun | 5 | The visual reveal is the hunt's peak moment (besides the meta) |
| Accessibility | 3 | Requires knowing map types well enough to trust the plotted positions |
| **Total** | **14/20** | |

The score does not capture the qualitative weight. This puzzle's fun-per-minute is the highest in the pool. It is the one solvers will photograph and share.

### Slot V (Post-Imperial / Strategy): V-B — Economy Puzzle

V-B is the right pick. The economy puzzle has a satisfying mechanical texture — the solver is playing AoE at the strategic level, making allocation decisions under constraints. The 40-villager cap means you cannot just do the math; you must find the balance point.

But I have a narrative note. The Post-Imperial slot is the climax before the meta. The Monk's voice should be at its most intense here. An economy puzzle — "how many villagers on each resource" — is strategic but not dramatic. Consider whether the framing can be elevated: instead of "build 10 Knights and a Castle," the scenario could be "the enemy is rushing you. You have 10 minutes. Allocate." The math is identical; the narrative pressure is real.

| Axis | Score | Notes |
|------|-------|-------|
| Riven Standard | 5 | Resource allocation IS strategy |
| Interlock | 2 | Self-contained |
| Fun | 3 | Satisfying but cerebral, not visceral |
| Accessibility | 4 | Every player understands the problem even if the math takes work |
| **Total** | **14/20** | |

### Cross-Slot: X-1 — Taunt Decoder

I want this in the hunt. The nostalgia is too strong to leave out. Every AoE player knows "11." Every multiplayer game ends with a taunt war. This is COMMUNITY knowledge — the shared language of AoE multiplayer.

But Katz is right that it is a pure lookup. My compromise: fold taunt references into the Monk's narration. Let the Monk say "14" instead of "Start the game already" in the intro. Let "30" ("Wololo") appear somewhere thematic. The taunts become discovered, never explained (Principle 12). The solver who recognizes them feels a secret kinship. The solver who does not misses nothing.

**Verdict: Do not include X-1 as a standalone puzzle. Weave taunt numbers into the Monk's voice as an easter egg layer.**

### Selinker Lineup

| Slot | Pick | Score |
|------|------|-------|
| I | I-A (Bonus Matcher) | 17/20 |
| II | II-B (Tournament Bracket) | 17/20 |
| III | III-A (Tech Tree Gap-Fill) | 15/20 |
| IV | IV-B (Resource Map) | 14/20 |
| V | V-B (Economy Puzzle) | 14/20 |

---

## Reviewer 3: Alex Rosenthal — Accessibility & Wonder

### Slot I (Dark Age / Civilizations): I-A — Bonus Matcher

I agree with Selinker over Katz on this one, and here is the accessibility argument.

This is the first thing the solver sees. It sets the difficulty expectation for the entire hunt. If the solver opens Puzzle I and cannot identify a single unit from its stats (I-C), they will assume the hunt is not for them. That is a catastrophic first impression. I-A (Bonus Matcher) has a softer landing: even a casual player can match 3-4 of the 8 bonuses from memory. The reference sheet (Stage 2 Action Item 2) fills in the rest. The solver gets early wins, builds confidence, and enters Puzzle II ready to work harder.

I-C is a better puzzle in isolation. I-A is a better Puzzle I.

Also note: I-B (Wonder Identifier) deserves mention for the "wonder" factor (identifying civilizations from architectural descriptions of their Wonders), but its extraction mechanism (first letters spell the answer) is too transparent. The solver would see it immediately and work backward from the letter constraint rather than forward from the architecture. That inverts the puzzle's logic.

| Axis | Score | Notes |
|------|-------|-------|
| Riven Standard | 4 | Civ bonuses are core AoE identity |
| Interlock | 4 | Two cross-referencing bonuses add real constraint |
| Fun | 4 | "I know this one" moments are rewarding |
| Accessibility | 5 | Most accessible option in the slot — perfect for Puzzle I |
| **Total** | **17/20** | |

### Slot II (Feudal Age / Units): II-B — Rock-Paper-Scissors Tournament

All three reviewers agree. The tournament bracket is the strongest candidate in the entire pool. I will add one accessibility note: the pool says "one matchup has a trick — a unique unit that breaks the normal counter rules." This trick must be FAIR. The solver must be able to figure out that this unit is special from information given in the puzzle or on the reference sheet. If the trick relies on obscure knowledge of a specific unique unit's anti-counter ability, it fails the Blame the Player principle. The trick should be deducible — perhaps the unit's description hints at its rule-breaking nature, or the bracket structure creates a contradiction that forces the solver to re-evaluate.

| Axis | Score | Notes |
|------|-------|-------|
| Riven Standard | 5 | Counter logic is AoE at its most fundamental |
| Interlock | 3 | Bracket creates internal cross-constraints |
| Fun | 5 | Tournament brackets are intrinsically compelling |
| Accessibility | 4 | Basic counters well-known; trick must be fair |
| **Total** | **17/20** | |

### Slot III (Castle Age / Technologies): III-A — Tech Tree Gap-Fill

Agreement, with one reservation. This is the puzzle most dependent on the reference sheet. A casual player cannot fill "Fletching -> Bodkin Arrow -> ???" from memory. The reference sheet must include a condensed tech tree — not the full 200+ technologies, but enough of the dependency chains that the solver can navigate them.

III-C (Age Advancement Requirements) is more accessible but less interesting. The matching format is a repeat of Slot I. III-B (Research Cost Detective) is too niche — only players who have memorized costs could engage with it.

| Axis | Score | Notes |
|------|-------|-------|
| Riven Standard | 5 | The tech tree IS the AoE Castle Age experience |
| Interlock | 4 | Mutual dependencies are genuine deduction |
| Fun | 3 | Competent, not thrilling |
| Accessibility | 2 | Heavily reference-dependent; casual players need the sheet |
| **Total** | **14/20** | |

### Slot IV (Imperial Age / Maps): IV-B — Resource Map

I love this concept. Drawing on the page transforms the solver from reader to participant. The visual reveal — resource dots forming letters — is the "wonder moment" I asked for in the Stage 2 review. This is the answer to "where is the visual flair?"

My accessibility concern: "Plot resources on a blank circular map" requires spatial reasoning that some solvers find intimidating. The blank circle can feel like a math test. Two mitigations: (1) provide the circular map pre-printed with compass directions and clock positions clearly labeled, so the solver is plotting on a scaffold rather than a blank page; (2) make the first map an easy one (Arabia or a simple layout) so the solver understands the mechanic before the harder maps.

| Axis | Score | Notes |
|------|-------|-------|
| Riven Standard | 4 | Resource spawns are AoE-authentic |
| Interlock | 2 | Self-contained |
| Fun | 5 | Drawing + visual reveal = wonder |
| Accessibility | 3 | Pre-printed maps and a gentle first example are essential |
| **Total** | **14/20** | |

### Slot V (Post-Imperial / Strategy): V-B — Economy Puzzle

This is the slot I flagged in Stage 2, so let me be precise about what changed.

V-A (Build Order) was my original concern: "only competitive players know build orders." The pool author heard this and offered V-B (Economy) as the accessible alternative. Is it accessible enough?

Yes — with calibration. The problem statement ("you need X units, here are gather rates, you have 40 villagers, allocate them") is comprehensible to any player who has managed an economy. The math is middle-school arithmetic, not optimization theory. The solver does not need to know the "correct" competitive allocation — they need to do the resource math and find the allocation that fits the constraint.

V-C (Historical Battle) is the most accessible option but the least interesting. Campaign scenario identification is trivia, not strategy. It does not pass the Riven Standard for the Strategy slot — it is history identification wearing a strategy costume.

| Axis | Score | Notes |
|------|-------|-------|
| Riven Standard | 5 | Resource allocation IS AoE strategy |
| Interlock | 2 | Self-contained |
| Fun | 3 | Cerebral satisfaction, not visceral delight |
| Accessibility | 4 | The math is straightforward; the constraint adds the puzzle |
| **Total** | **14/20** | |

### Cross-Slot: X-1 — Taunt Decoder

I want to amplify Selinker's suggestion. Taunts as an easter egg layer — embedded in the Monk's narration — is brilliant. It turns cultural knowledge into a discovery moment (Principle 12: Discovered, Never Explained). A solver who recognizes "14" in the Monk's voice and realizes it means "Start the game already" gets a private wink. That is accessibility done right: the layer exists for those who know it, and is invisible to those who do not.

As a standalone puzzle, X-1 fails. It is a lookup table. No deduction, no aha.

**Verdict: Easter egg, not puzzle.**

### Rosenthal Lineup

| Slot | Pick | Score |
|------|------|-------|
| I | I-A (Bonus Matcher) | 17/20 |
| II | II-B (Tournament Bracket) | 17/20 |
| III | III-A (Tech Tree Gap-Fill) | 14/20 |
| IV | IV-B (Resource Map) | 14/20 |
| V | V-B (Economy Puzzle) | 14/20 |

---

## Synthesis: Consensus Lineup

### The Vote

| Slot | Katz | Selinker | Rosenthal | Consensus |
|------|------|----------|-----------|-----------|
| I | I-C (Unique Unit) | **I-A (Bonus Matcher)** | **I-A (Bonus Matcher)** | **I-A** (2-1) |
| II | **II-B (Tournament)** | **II-B (Tournament)** | **II-B (Tournament)** | **II-B** (3-0) |
| III | **III-A (Tech Tree)** | **III-A (Tech Tree)** | **III-A (Tech Tree)** | **III-A** (3-0) |
| IV | **IV-B (Resource Map)** | **IV-B (Resource Map)** | **IV-B (Resource Map)** | **IV-B** (3-0) |
| V | **V-B (Economy)** | **V-B (Economy)** | **V-B (Economy)** | **V-B** (3-0) |
| X-1 | No (standalone) | Easter egg | Easter egg | **Easter egg** |

### The Dissent (Slot I)

Katz argues I-C (Unique Unit Lineup) is a stronger puzzle in isolation — higher Riven Standard score, more deduction required, richer solve path. He is right on the merits. But Selinker and Rosenthal argue I-A (Bonus Matcher) is a stronger *opening* puzzle — gentler difficulty curve, more accessible, sets the right emotional register. In a 5-puzzle hunt with no warm-up round, Puzzle I is doing double duty as both the first puzzle and the on-ramp.

**Resolution:** I-A opens the hunt. I-C's best element — the "identify units from stat descriptions" concept — can be folded into II-B's tournament bracket. Instead of naming the 8 tournament units outright, describe some by their stats/abilities. This adds a layer of identification to the bracket puzzle without front-loading it into Slot I.

### The Final Five

```
I.   BONUS MATCHER (Civilizations) — Difficulty 2
     Match 8 unique civ bonuses to their civilizations.
     Two bonuses cross-reference each other. Extraction by indexing.
     Mechanism: Identification + internal interlock.

II.  ROCK-PAPER-SCISSORS TOURNAMENT (Units) — Difficulty 2
     Predict winners in an 8-unit single-elimination bracket
     using AoE counter logic. One trick matchup breaks the rules.
     Mechanism: Bracket deduction + exception discovery.

III. TECH TREE GAP-FILL (Technologies) — Difficulty 3
     Six technologies missing from a dependency tree.
     Two blanks mutually constrain each other.
     Mechanism: Dependency reasoning + mutual constraint.

IV.  RESOURCE MAP (Maps) — Difficulty 3
     Plot resource spawn positions on 5 blank circular maps.
     Plotted dots trace letter shapes.
     Mechanism: Spatial drawing + visual discovery.
     *** DINNER PARTY PUZZLE ***

V.   ECONOMY PUZZLE (Strategy) — Difficulty 3
     Allocate 40 villagers across resources to afford a specific army.
     Optimal allocation numbers map to letters via A1Z26.
     Mechanism: Constrained optimization + encoding.
```

### Difficulty Curve (revised)

```
Difficulty
3 |                   ██    ██    ██
2 |    ██    ██
1 |
  +----+-----+-----+-----+-----+--
    I     II    III   IV    V
```

The curve now ramps from 2-2-3-3-3 instead of the original 2-2-3-2-3. This is slightly front-weighted toward difficulty at the end, which is appropriate for Post-Imperial gameplay. The solver builds confidence in Slots I-II, then the hunt demands more from Slot III onward.

**Katz note:** The 2-2-3-3-3 curve means the back half is uniformly hard. Consider whether IV could stay at difficulty 2 (simplify the map set — use 4 well-known maps and 1 moderate one instead of 5 moderate maps). A 2-2-3-2-3 dip-and-rise is better pacing than a staircase.

### Mechanism Variety Audit

| Slot | Primary Mechanism | Physical Mode | Aha Moment |
|------|-------------------|---------------|------------|
| I | Matching with interlock | Read + write answers | Two bonuses reference each other |
| II | Bracket deduction | Read + trace bracket | The trick unit that breaks counter rules |
| III | Dependency gap-fill | Read + fill tree | Two blanks constraining each other |
| IV | Spatial plotting | **Draw on page** | Dots form letter shapes |
| V | Constrained optimization | Calculate + encode | The constraint forces a unique solution |

**Verdict (Katz):** Five distinct mechanisms. No two puzzles use the same solve mode. The physical mode varies: reading, tracing, filling, drawing, calculating. The aha moments are all different types: recognition, exception, cascade, visual, constraint. **Passes the variety test.**

### Principle Compliance

| Principle | Status | Notes |
|-----------|--------|-------|
| Riven Standard | **PASS** | Every puzzle IS its AoE domain. Tournament bracket IS unit tactics. Tech tree IS research. Resource map IS map knowledge. |
| Solving = Proving Understanding | **PASS** | Answers demonstrate AoE comprehension, not keyword matching. |
| Dinner Party Test | **PASS** | IV-B (Resource Map) is the Dinner Party puzzle. "I plotted resource spawns and the dots spelled a word." |
| The Book Test | **PASS** | Pencil and printed pages. IV-B requires drawing but only on provided blank maps. |
| Blame the Player | **TBD** | Depends on construction. The trick matchup in II-B and the mutual constraints in III-A must be fair. |
| 80% Rule | **Carried from Stage 2** | Meta mechanism still needs proof-of-concept. |
| No Over-Scaffolding | **PASS** | No puzzle provides step-by-step instructions. IV-B gives positions, not drawing instructions. |
| Surprise the Answer | **TBD** | Answer words not yet chosen. They must not be predictable from the domain. |
| One Aha | **PASS** | Each puzzle has exactly one aha (see Mechanism Variety table). |
| No Deliberate Errors | **TBD** | All AoE facts (bonuses, counters, tech trees, resource spawns, gather rates) must be verified against game data. |
| Interlock, Not Independence | **PARTIAL** | III-A has strong internal interlock. I-A has moderate internal interlock. Cross-puzzle interlock remains weak — meta is the only link. Acceptable for 5-puzzle hunt. |
| No Computation Without Deduction | **PASS** | V-B has computation but the constraint (40 villagers) forces optimization, not just calculation. II-B requires judgment on the trick matchup. |
| Snyder's Computer Test | **PASS** | II-B (trick matchup), III-A (mutual constraints), IV-B (spatial recognition of letter shapes) all require human judgment. |
| Voice Rules | **Carried from Stage 2** | Monk voice established; escalation guide still needed. |

### Action Items from this Review

| # | Priority | Action | Source |
|---|----------|--------|--------|
| 1 | **P1** | Construct IV-B (Resource Map) proof-of-concept: pick 5 maps, define resource positions, verify the plotted dots form legible letters. This is the highest-risk puzzle — if the letters are not clean, the Dinner Party moment dies. | Selinker |
| 2 | **P1** | Verify II-B trick matchup is fair. The rule-breaking unique unit must be deducible from information in the puzzle or reference sheet. Identify the unit, document why the solver can figure it out. | Katz |
| 3 | **P2** | Calibrate V-B (Economy) math so that the optimal allocation maps to valid A1Z26 values that spell a real word. Reverse-engineer the resource costs and gather rates. Test with 2-3 different army compositions to find one that works. | Katz |
| 4 | **P2** | Design the reference sheet to support III-A. Include condensed tech tree dependency chains (not all 200+ techs — just the chains used in the puzzle plus enough decoys that the solver must still reason). | Rosenthal |
| 5 | **P2** | Pre-print blank circular maps for IV-B with compass directions and clock positions labeled. The solver should be plotting on a scaffold, not a blank page. Include a gentle first map (Arabia or similar). | Rosenthal |
| 6 | **P3** | Fold taunt numbers into Monk narration as easter eggs. Identify 3-5 taunt numbers to embed. Document which taunts, where they appear, and what a solver who recognizes them gains (flavor, not mechanical advantage). | Selinker |
| 7 | **P3** | Consider borrowing I-C's "identify from stats" concept for 2-3 units in the II-B bracket (describe units by abilities instead of naming them). Adds an identification layer without front-loading difficulty. | Katz |

### Eliminated Candidates — Summary

| Candidate | Why eliminated |
|-----------|---------------|
| I-B (Wonder Identifier) | Extraction too transparent (first letters). Solver would reverse-engineer from letter constraints. |
| I-C (Unique Unit Lineup) | Strong puzzle, wrong slot. Too hard for the opener. Best elements can be folded into II-B. |
| II-A (Counter Circle) | Too small (6 units in a circle). Not enough surface for real deduction. |
| II-C (Upgrade Path) | Pure recall — fill in known sequences. A quiz, not a puzzle. |
| III-B (Research Cost Detective) | Too niche. Memorizing tech costs is not understanding the tech tree. |
| III-C (Age Advancement) | Matching format repeats Slot I. Lower ceiling. |
| IV-A (Map Identifier) | Thin — five identifications. No physical/spatial element. |
| IV-C (Terrain Puzzle) | Generic pathfinding. Could be any game with terrain movement. |
| V-A (Build Order) | Competitive knowledge only. Alienates casual players. Rosenthal's original flag holds. |
| V-C (Historical Battle) | Trivia identification wearing a strategy costume. Does not pass Riven Standard for the Strategy slot. |
| X-1 (Taunt Decoder) | Pure lookup. No deduction, no aha. Better as easter egg layer. |
| X-2 (Sound Effects) | Not seriously considered. Audio descriptions on paper lose the sensory element. |

---

## Final Verdict: APPROVED

The consensus lineup of I-A / II-B / III-A / IV-B / V-B is approved by all three reviewers. Four of five picks are unanimous (3-0). The Slot I dissent (Katz preferring I-C) is resolved by folding I-C's best concept into II-B.

The lineup provides:
- **Clean difficulty curve:** 2-2-3-3-3 (or 2-2-3-2-3 if IV-B is simplified per Katz's suggestion)
- **Five distinct mechanisms:** Matching, bracket deduction, dependency reasoning, spatial drawing, constrained optimization
- **One Dinner Party puzzle:** IV-B (Resource Map) — the physical, visual centerpiece
- **One showstopper mechanism:** II-B (Tournament Bracket) — the most purely fun puzzle in the pool
- **Strong Riven Standard compliance:** Every puzzle IS its AoE domain
- **Taunt easter eggs:** X-1 repurposed as a hidden layer in the Monk's narration

Proceed to Stage 4: individual puzzle construction.

---

*Review conducted under the 3-reviewer panel model. Document: PUZZLE-POOL.md. Panel: Katz (structure), Selinker (narrative), Rosenthal (accessibility).*
