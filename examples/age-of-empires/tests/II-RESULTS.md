# Puzzle II — Feudal Age — Test Results

**Puzzle:** Tournament Bracket (8-unit elimination, counter logic, one exception)
**Testers:** Katz, Selinker, Blow
**Answer:** ONAGER

---

## Tester 1: Dan Katz (Structure & Calibration)

### Solve Attempt

Single-elimination bracket, 8 units, counter logic decides outcomes. One unit defies its counter. Let me work through this.

First, classify the units:
- **Siege:** Onager, Mangonel
- **Cavalry:** Knight, Cataphract
- **Archer:** Crossbowman
- **Infantry:** Champion
- **Anti-cavalry infantry:** Pikeman, Halberdier

Now apply counter logic:

**QF-1: Onager vs Crossbowman.** Siege destroys archers. Winner = **Onager**. Letter 1 of "Onager" = **O**.

**QF-2: Pikeman vs Knight.** Anti-cavalry infantry destroys cavalry. Winner = **Pikeman**. Letter 7 of "Pikeman" = **N**.

**QF-3: Cataphract vs Halberdier.** Halberdier is anti-cavalry infantry. Cataphract is cavalry. Normally Halberdier wins. BUT the puzzle says one cavalry unit "does not fear the spear" and "its armor was designed to resist the very bonus that should destroy it." The Cataphract's unique trait in AoE2 is resistance to anti-cavalry bonus damage (trample damage + attack bonus resistance). Winner = **Cataphract**. Letter 2 of "Cataphract" = **A**.

**QF-4: Mangonel vs Champion.** Siege destroys infantry. Winner = **Mangonel**. Letter 4 of "Mangonel" = **G**.

**SF-1: Onager vs Pikeman.** Siege vs infantry. Siege destroys infantry. Winner = **Onager**. Letter 5 of "Onager" = **E**.

**SF-2: Cataphract vs Mangonel.** Cavalry vs siege. Cavalry destroys siege. Winner = **Cataphract**. Letter 7 of "Cataphract" = **R**.

Wait -- the bracket shows a FINAL slot but the worksheet only asks for QF-1 through SF-2 (six letters). The final would be Onager vs Cataphract (siege vs cavalry, cavalry wins), but we don't extract from it. Six letters: O-N-A-G-E-R.

**Answer: ONAGER**

Solve time: ~6 minutes.

The answer is ONAGER -- which is also a unit in the bracket. That's a nice touch. The answer was hiding in plain sight.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | The bracket is well-drawn. Counter logic rules are explicitly stated. The one-exception hint is clear. Minor issue: the bracket shows a FINAL slot but we don't extract from it -- why include it? |
| Solvability | 5 | Every matchup has exactly one correct answer given the rules. The Cataphract exception is well-clued by both the flavor text AND real AoE2 game knowledge. No ambiguity. |
| Elegance | 4 | The mechanism is tight. Eight units, seven matches, one exception. The exception IS the puzzle -- everything else is rule application. The answer being ONAGER (a unit in the bracket itself) is a satisfying loop. |
| Reading Reward | 4 | You need to know counter logic and the Cataphract's special property. This isn't Ctrl+F territory -- you need to understand the rock-paper-scissors of AoE2 combat. |
| Fun | 5 | This is genuinely fun. Filling in a bracket is inherently engaging. The Cataphract twist is the aha moment. I'd tell someone about this one. |
| Confirmation | 4 | ONAGER is clean. Six letters, reads in order. The self-referential nature (the answer is a unit in the puzzle) confirms it. |
| **Total** | **26** | |

### Issues

| Severity | Issue |
|----------|-------|
| Minor | The FINAL slot in the bracket art is drawn but never used for extraction. This is slightly misleading -- a solver might wonder if they're missing a 7th letter. The worksheet clarifies (only QF-1 through SF-2), but the visual promises more than it delivers. |
| Minor | The counter logic rules are stated in the puzzle, which means a solver doesn't strictly need AoE2 knowledge for most matchups. Only the Cataphract exception requires game-specific knowledge. This is arguably a feature (accessibility) but slightly reduces Reading Reward. |

### Suggested Fixes

1. Either extract from the FINAL (adding a 7th letter) or remove the FINAL slot from the bracket art. The visual should match the extraction scope.
2. Consider whether the counter logic rules should be given or discovered. Giving them lowers the difficulty floor (good for puzzle II) but also lowers the Reading Reward.

---

## Tester 2: Mike Selinker (Narrative & Experience)

### Solve Attempt

"Eight warriors enter. One remains." Love the framing. This is a story.

Working through the bracket like a tournament organizer. The counter logic is clean -- siege beats archers/infantry, cavalry beats siege/archers, anti-cav infantry beats cavalry, archers beat infantry. Classic RPS with a twist.

QF-1: Onager crushes Crossbowman. (O)
QF-2: Pikeman skewers Knight. (N)
QF-3: Here's the moment. Cataphract vs Halberdier. By the rules, Halberdier should win. But "a certain cavalry unit does not fear the spear." The Cataphract's entire design is anti-infantry resistance. This is the puzzle's beating heart. Winner = Cataphract. (A)
QF-4: Mangonel flattens Champion. (G)
SF-1: Onager rolls over Pikeman. (E)
SF-2: Cataphract charges down Mangonel. (R)

ONAGER. The war machine that was in the bracket from the start. The answer was in the room the whole time.

**Answer: ONAGER**

Solve time: ~5 minutes.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | Bracket format is intuitive. Rules are explicit. The exception is well-telegraphed. The unused FINAL is a small blemish. |
| Solvability | 5 | Clean deduction at every step. The Cataphract exception is the only non-trivial decision, and it's well-clued. |
| Elegance | 5 | This is beautifully crafted. The mechanism IS the content. The exception IS the aha. The answer IS a unit in the bracket. Three layers of self-reference. This is the kind of puzzle I'd put in a book. |
| Reading Reward | 4 | Counter logic is core AoE2 knowledge. The Cataphract's anti-bonus-damage trait is the deeper cut. |
| Fun | 5 | Dinner Party Test: passed. "I had to fill in a tournament bracket using counter logic, and the twist was that one cavalry unit was immune to its counter. The answer was ONAGER -- one of the units in the bracket." That's a story worth telling. |
| Confirmation | 4 | ONAGER is unambiguous. Self-referential confirmation (the answer is in the puzzle). |
| **Total** | **27** | |

### Issues

| Severity | Issue |
|----------|-------|
| Minor | The unused FINAL bracket slot. It's narrative deadweight -- "one remains" but we don't learn who the champion is. The Cataphract would win (cavalry beats siege), but we never extract from it. |

### Suggested Fixes

1. Either use the FINAL for extraction (7-letter answer) or have the FINAL serve a narrative purpose: "The champion is ___" as a bonus confirmation. The bracket promises a champion and should deliver one, even if it's not part of the letter extraction.

---

## Tester 3: Jonathan Blow (Epiphany & Information Content)

### Solve Attempt

The counter logic rules are given. That means most matchups are pure execution -- apply the rule, get the winner. That's concerning.

QF-1: Rule application. Onager wins. (O)
QF-2: Rule application. Pikeman wins. (N)
QF-3: This is where the puzzle lives. The rules say Halberdier should win. But the puzzle says one unit defies its counter. Which one? "Its armor was designed to resist the very bonus that should destroy it." The Cataphract. This is the single moment of genuine insight. (A)
QF-4: Rule application. Mangonel wins. (G)
SF-1: Rule application. Onager wins. (E)
SF-2: Rule application. Cataphract wins. (R)

ONAGER.

**Answer: ONAGER**

Solve time: ~4 minutes.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Unambiguous at every step. The bracket format is universally understood. |
| Solvability | 5 | One decision point (QF-3), rest is mechanical. Highly solvable. |
| Elegance | 4 | The Cataphract exception is a genuine aha -- "one unit defies its counter" is the puzzle. Everything else is scaffolding. The scaffolding is clean, but 5/6 matches are trivial. I'd prefer more matches that require insight. |
| Reading Reward | 3 | The counter logic rules are GIVEN in the puzzle. Only the Cataphract exception requires game knowledge. If you removed the AoE2 theme entirely and kept the abstract counter rules, the puzzle would work identically for 5 of 6 matches. That's a Reading Reward problem. |
| Fun | 4 | The bracket format is inherently satisfying. The Cataphract twist is good. But the low insight density (1 aha out of 6 decisions) dampens the overall experience. |
| Confirmation | 4 | Clean extraction. Self-referential answer. |
| **Total** | **25** | |

### Issues

| Severity | Issue |
|----------|-------|
| Major | Reading Reward is borderline. The counter logic rules are explicitly provided in the puzzle, which means a non-AoE2-player could solve 5 of 6 matchups. Only the Cataphract exception requires domain knowledge. This pushes the puzzle toward "logic exercise with one trivia question" rather than "AoE2 knowledge test." |
| Minor | 5 of 6 extraction letters come from mechanical rule application. The insight density is low. A 10-line script could solve everything except QF-3. |

### Suggested Fixes

1. Consider removing or trimming the explicit counter logic rules. Let the solver discover the rock-paper-scissors through the matchups themselves. This would increase both Reading Reward and insight density.
2. Alternatively, add a second exception or ambiguous matchup where the solver must apply deeper game knowledge (e.g., a unit that counters differently than expected due to an upgrade or unique tech).

---

## Synthesis

| Dimension | Katz | Selinker | Blow | Average |
|-----------|------|----------|------|---------|
| Clarity | 4 | 4 | 5 | 4.3 |
| Solvability | 5 | 5 | 5 | 5.0 |
| Elegance | 4 | 5 | 4 | 4.3 |
| Reading Reward | 4 | 4 | 3 | 3.7 |
| Fun | 5 | 5 | 4 | 4.7 |
| Confirmation | 4 | 4 | 4 | 4.0 |
| **Total** | **26** | **27** | **25** | **26.0** |

### Verdict: PASS (26.0/30)

The strongest puzzle in the set so far. The bracket format is inherently fun, the Cataphract exception is a well-crafted aha moment, and the self-referential answer (ONAGER is in the bracket) is elegant. The main weakness is Reading Reward -- the explicit counter logic rules mean most matchups don't require AoE2 knowledge. Blow's critique is valid: consider whether the rules should be discovered rather than given.

### Consensus Issues

1. **Unused FINAL slot** (Minor): All three testers noted the bracket shows a FINAL but extraction stops at semifinals. Either use it or remove it.
2. **Reading Reward** (Major, per Blow): Counter logic rules are given explicitly, reducing the need for game knowledge. Consider trimming or removing the rules section to increase Reading Reward.
3. **Self-referential answer** (Positive): All three testers noted ONAGER being a bracket unit as an elegant touch. This is a strength.
