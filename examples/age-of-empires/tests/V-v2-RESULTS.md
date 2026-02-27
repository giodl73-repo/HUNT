# Puzzle V (v2) — Post-Imperial — Test Results

**Puzzle:** Annotated Replay (6 moments in a 1v1, each yields a number, numbers become letters)
**Version:** v2 (redesigned from v1)
**Testers:** Huang, Kenny, Katz
**Answer:** PATROL

---

## v1 Blocking Issues (for regression check)

The v1 test (19.3/30, REDESIGN) identified three blocking failures:

1. **Pure computation, zero deduction.** All six questions were given-input division. Every number was stated inline. A 3-line script solved the puzzle.
2. **Reading Reward 2/5.** Five of six questions provided all numbers in the question itself. No game knowledge required.
3. **Scriptable / Snyder failure.** No ambiguity, no judgment, no lookup. A calculator was the only tool needed.

---

## The Sanitized Puzzle (what testers see)

An annotated replay of a 1v1 on Arabia. Six moments, each poses a question. Each answer is a number 1-26, converted to a letter (A=1 ... Z=26). Read six letters in order.

The replay does not repeat what a student of the game already knows. Costs, training times, tech effects, age-up requirements — look them up.

---

## Tester 1: Wei-Hwa Huang (Deductive Rigor & Solve-Path Quality)

### Solve Attempt

This is much better than v1. The puzzle no longer hands me every number — it describes game situations and expects me to know (or look up) the relevant values. Let me work through each moment.

**Moment 1 — The Resource Dump (19:00)**

Blue has 400 wood and 720 gold. Trains a single unit type from the Archery Range. Both resources hit exactly zero.

I need to find which Archery Range unit costs wood and gold, and which one allows 400W and 720G to be spent with zero remainder. This requires looking up unit costs.

From the world data — Archery Range units that cost wood AND gold:
- Archer: 25W, 45G
- Crossbowman: 25W, 45G (same cost as Archer)
- Cavalry Archer: 40W, 60G
- Skirmisher: 25F, 35W (food + wood, no gold — eliminated)
- Hand Cannoneer: 45F, 50G (food + gold, no wood — eliminated)

Testing the wood-and-gold units:
- Archer/Crossbowman: 400/25 = 16 (wood), 720/45 = 16 (gold). Both divide cleanly to 16. Works.
- Cavalry Archer: 400/40 = 10 (wood), 720/60 = 12 (gold). Mismatch. Eliminated.

Only one cost structure works: 25W/45G. Blue trains **16** units.

The deductive step: I had to identify which units cost wood + gold (not food), then test divisibility. This is constraint satisfaction, not just division.

Answer 1: 16 → **P**

**Moment 2 — The Shortcut (22:00)**

Imperial Age normally requires two Castle Age buildings. But there's a single building type that satisfies the prerequisite on its own. How many does the shortcut require?

From techs.md: Imperial Age requires "2 Castle Age buildings (Siege Workshop, Monastery, University) OR 1 Castle."

The Castle is the shortcut building. You need **1** Castle.

Answer 2: 1 → **A**

**Moment 3 — The Crossbow Rush (23:00 – 32:00)**

Single Archery Range, nonstop for exactly 9 minutes. Crossbowmen. The puzzle explicitly warns: "The Crossbowman does not train at the same speed as the Archer it upgraded from."

This is a lookup trap. A solver who remembers the Archer's 35-second train time and assumes Crossbowmen are the same will get 540/35 ≈ 15.4, which isn't an integer. The hint tells you to look up the actual Crossbowman train time.

From economy.md: Crossbowman train time = 27 seconds. (Archer = 35 seconds.)

540 / 27 = **20**.

Good. The hint is fair — it warns you the obvious number is wrong without telling you the right one. The solver must find the Crossbowman-specific training time.

Answer 3: 20 → **T**

**Moment 4 — The Cavalry Pivot (32:00 – 41:00)**

One Stable, 9 minutes nonstop, Knights.

From economy.md: Knight train time = 30 seconds.

540 / 30 = **18**.

This is the most straightforward moment, but it still requires knowing (or looking up) the Knight train time.

Answer 4: 18 → **R**

**Moment 5 — The Upgrade Question (41:00)**

Divide the Cavalier upgrade's gold cost by the Militia's per-unit gold cost.

Two separate lookups required:
- From techs.md: Cavalier upgrade = 300F 300G. Gold cost = 300.
- From economy.md: Militia = 60F 20G. Per-unit gold cost = 20.

300 / 20 = **15**.

This moment requires the solver to distinguish between upgrade costs (techs.md) and training costs (economy.md). The Cavalier upgrade cost is NOT the same as the Cavalier training cost (60F 75G). A solver who confuses them gets 75/20 = 3.75, which isn't an integer — a useful self-check.

Answer 5: 15 → **O**

**Moment 6 — The Conversion Range (50:00)**

Monk's conversion range after Block Printing.

From techs.md: Monks have base conversion range of 9. Block Printing gives +3 range. 9 + 3 = **12**.

Answer 6: 12 → **L**

**P-A-T-R-O-L = PATROL**

**Answer: PATROL**

Solve time: ~10 minutes. Substantially longer than v1's 2-3 minutes.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Each moment is a clean scenario. The extraction rule (A=1...Z=26) is standard. The replay framing is natural. The Moment 3 hint about Crossbowman train time is a model of fair warning. |
| Solvability | 4 | Every answer is a clean integer. Non-integer results serve as self-checks (wrong unit cost, wrong train time). The only risk is Moment 1, where a solver might not think to test all Archery Range units — but the puzzle explicitly says "Several Archery Range units cost wood and gold. Only one allows Blue to spend both stockpiles exactly to zero," which points you to the method. |
| Elegance | 4 | The replay framing gives narrative coherence — you're watching a game unfold chronologically. Each moment tests a different type of game knowledge: unit costs (M1), age-up requirements (M2), training times (M3, M4), upgrade costs vs. training costs (M5), tech effects (M6). Moment 1 is the standout — it's genuine constraint satisfaction (find the unit where two divisibility conditions meet). Moments 4 and 6 are simpler (single lookup + arithmetic), which keeps the difficulty curve varied rather than monotonous. |
| Reading Reward | 4 | This is the critical improvement. Every moment requires at least one lookup in the world data. M1 requires scanning the full Archery Range unit table and testing divisibility. M2 requires knowing the Imperial Age prerequisite alternatives. M3 requires knowing the Crossbowman-specific train time (not the Archer's). M5 requires distinguishing upgrade costs from training costs across two different reference tables. M6 requires base range + tech effect. Only M4 is a pure single-lookup. |
| Fun | 3 | The replay framing is engaging. Moment 1 is genuinely satisfying — the constraint-satisfaction step feels like detective work. But Moments 4 and 6 are still fundamentally "look up a number and divide/add." The puzzle is now a good puzzle, not a great one. It's solid middle-of-pack material, not a Dinner Party entry. |
| Confirmation | 4 | PATROL is thematic (a unit command in AoE2). Six letters from six moments. The replay narrative makes the answer feel like a command you'd issue at the end of the game — "patrol your units post-Imperial." |
| **Total** | **24** | |

### Issues

| Severity | Issue |
|----------|-------|
| Minor | Moments 3 and 4 are structurally identical: (time / train_time = count). The train times differ, and M3 has the Crossbowman trap, but the operation is the same. One more moment with a different mathematical structure would help variety. |
| Minor | Moment 6 is simple addition (9 + 3 = 12). It's the weakest moment deductively. It works as a cool-down after M5, but a more interesting final moment would strengthen the ending. |
| Positive | Moment 1 is a genuine deduction step. The solver must enumerate unit costs, test two divisibility constraints simultaneously, and eliminate non-matching options. This is the kind of "No Computation Without Deduction" fix the v1 needed. |
| Positive | The Crossbowman ≠ Archer train time trap in M3 is excellent design. It rewards the solver who looks up the actual value and punishes the solver who assumes. Fair, well-signaled, and thematic. |

---

## Tester 2: Kenny Young (Infrastructure & Buildability)

### Solve Attempt

The replay conceit is nice. I'm watching a game, and each moment is a question about game mechanics. Let me see if the world data gives me everything I need.

**M1:** Archery Range units that cost W+G. From the unit table: Archer (25W 45G), Crossbow (25W 45G), Cav Archer (40W 60G). Skirms and HC don't cost W+G. Testing 400W/720G: Archer/Xbow gives 16/16 (match), Cav Archer gives 10/12 (no match). Answer: **16** → P.

This is the best moment. I had to go through the unit list, identify candidates, and test each. That's a real puzzle step.

**M2:** Imperial Age requires 2 Castle Age buildings or 1 Castle. The "shortcut" is the Castle. Requires **1**. → A.

Quick lookup, but the "shortcut" framing makes it a knowledge question, not a gimme. A solver who doesn't know the Castle exception has to find it in the tech reference.

**M3:** 9 minutes of Crossbowmen. The hint warns me the Crossbow isn't the same speed as the Archer. Crossbow = 27 sec. 540/27 = **20** → T.

Good trap. Without the hint, I'd probably try 35 seconds first and get a non-integer. The hint saves frustration without giving the answer.

**M4:** 9 minutes of Knights. Knight = 30 sec. 540/30 = **18** → R.

Straightforward, but I need to know the Knight train time. Fair.

**M5:** Cavalier upgrade gold / Militia gold per unit. Cavalier upgrade = 300G (from techs). Militia = 20G (from economy). 300/20 = **15** → O.

I like this one. Upgrade costs and training costs live in different tables. The solver has to navigate between techs.md and economy.md. The word "upgrade's gold cost" vs "per-unit gold cost" signals this distinction clearly.

**M6:** Monk range after Block Printing. Base 9, Block Printing +3 = **12** → L.

Simple lookup, but the Monk/Monastery data is in techs.md, not economy.md. The solver has to know where to look.

**PATROL.**

**Answer: PATROL**

Solve time: ~8 minutes.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Each moment is well-framed. The replay format is natural. The Crossbowman hint is the right amount of guidance — warns about a trap without solving it. |
| Solvability | 4 | Clean integers throughout. Non-integer results flag wrong inputs. M1 is the hardest — a solver might miss that Archers and Crossbows have the same cost and both work. But the puzzle says "only one allows" which refers to the cost structure, not the specific unit. Slightly ambiguous, but the answer is unambiguous. |
| Elegance | 4 | The seed is "replay moments as game-knowledge questions." Six moments, six different aspects of the game: unit economics (M1), age advancement (M2), production timing (M3, M4), upgrade economics (M5), tech effects (M6). The chronological replay gives it narrative structure. M1 is the clear standout. |
| Reading Reward | 4 | Every moment requires consulting the reference data. M1 requires scanning the unit cost table and testing multiple candidates. M3 requires finding a specific train time that differs from the obvious one. M5 requires cross-referencing two different tables. M2 and M6 are single lookups but in non-obvious locations (the Castle exception, the Monk range note). Only M4 is a truly trivial lookup. |
| Fun | 3 | The replay is a good frame. M1 is satisfying. M3's trap is well-designed. But the overall experience is "six lookups with some arithmetic," which is solid but not exciting. This won't be anyone's favorite puzzle in the set, but no one will hate it either. It's a reliable middle-difficulty puzzle. |
| Confirmation | 4 | PATROL is clean. Thematic. The word itself is a payoff — you've been watching the replay, and the answer is a command to issue. |
| **Total** | **24** | |

### Issues

| Severity | Issue |
|----------|-------|
| Minor | M1 could cause a brief confusion: "both Archer and Crossbowman have the same cost (25W 45G), so aren't there two unit types that work?" The puzzle says "only one allows Blue to spend both stockpiles exactly to zero," which is true — only one cost structure (25W/45G) works, not two different cost structures. But a pedantic solver might flag this. |
| Minor | The replay timestamps (19:00, 22:00, etc.) add flavor but aren't used for solving. If timestamps were load-bearing (e.g., "how much time passed between M3 and M4?"), they'd add interlock. Currently they're cosmetic. |
| Positive | The v1→v2 improvement is night and day. v1 handed you every number. v2 makes you find them. The same answer (PATROL) with a completely different solving experience. |

---

## Tester 3: Dan Katz (Structure & Calibration)

### Solve Attempt

Let me see if this redesign fixes the three blocking issues from v1.

M1: 400W, 720G, Archery Range. I scan the unit table. Wood+gold Archery Range units: Archer 25W/45G, Crossbow 25W/45G, Cav Archer 40W/60G. (Skirm is F+W, HC is F+G.) Test: 400/25=16 and 720/45=16. Match. 400/40=10 and 720/60=12. No match. Answer: **16** → P.

This is constraint satisfaction. The solver must enumerate, filter, test. That's deduction. v1 issue #1: fixed.

M2: Castle as Imperial Age shortcut. Requires **1** → A.

Game knowledge question. You need to know the Castle exception OR find it in techs.md. v1 issue #2: partially addressed — this is a single lookup but it's non-trivial knowledge.

M3: Crossbow train time ≠ Archer train time. 540/27 = **20** → T.

The trap is well-designed. A solver who doesn't look it up gets a non-integer and knows to check. The hint is fair.

M4: Knight = 30 sec. 540/30 = **18** → R.

Standard.

M5: Cross-reference: Cavalier upgrade gold (techs.md: 300G) / Militia training gold (economy.md: 20G) = **15** → O.

Good: two different tables, two different cost types (upgrade vs. training). Forces navigation of the reference material.

M6: Base 9 + Block Printing 3 = **12** → L.

Simple. But the solver needs to find the Monk base range and the Block Printing effect, which are both in techs.md's Monastery section. Not obvious to a non-expert.

**PATROL.**

**Answer: PATROL**

Solve time: ~8 minutes.

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | The replay frame is clean. Each moment is self-contained. The extraction is standard A=1. The Crossbowman hint is well-calibrated — it warns without solving. |
| Solvability | 4 | All answers are clean integers. The self-check mechanism (non-integer = wrong lookup) is built into every moment. M1 is the only step with meaningful ambiguity (multiple unit candidates), and the puzzle explicitly tells you to test divisibility. |
| Elegance | 3 | The replay frame adds coherence, and M1 is genuine constraint satisfaction. But Moments 3-4-5-6 are still fundamentally "look up value, do arithmetic." The variety is in WHAT you look up (train time, upgrade cost, tech effect), not in HOW you reason. There's one deduction step (M1) and five lookup-and-compute steps. I'd want at least two moments that require deduction, not just one. Compared to the v1 (elegance 1.3), this is a significant improvement. But it's not at the level of Puzzles I-IV in this set. |
| Reading Reward | 4 | The critical fix. Every moment requires a lookup. M1 requires scanning and filtering. M3 requires finding a non-obvious value (Crossbow ≠ Archer train time). M5 requires cross-referencing two tables. M2 and M6 require finding specific rules in the tech reference. Only M4 is a trivial single lookup. This is a 2-point improvement over v1. |
| Fun | 3 | M1 is satisfying. The Crossbowman trap is clever. The replay frame is engaging. But the overall experience is "six questions, mostly lookup + arithmetic." It's a competent puzzle, not an exciting one. It's calibrated right for difficulty 3 now — it's harder than v1 by a wide margin — but it doesn't have the Dinner Party quality of Puzzle IV. |
| Confirmation | 4 | PATROL remains a strong answer word. Thematic and surprising. |
| **Total** | **23** | |

### Issues

| Severity | Issue |
|----------|-------|
| Minor | Only one moment (M1) involves genuine deduction. The rest are lookup-then-compute. A second deduction moment would push elegance from 3 to 4. For example: M6 could present a scenario where the solver must deduce WHICH Monastery tech was researched from observable behavior, rather than being told "Block Printing." |
| Minor | The six moments have no interlock. Solving M1 gives no information about M2. Each is independent. v1 had the same issue. Adding one moment where a previous answer constrains the current one would improve structure. |
| Minor | Difficulty calibration is now correct. This is harder than Puzzles I and II but not crushingly so. The positioning as Post-Imperial (endgame) puzzle makes sense — it tests broad game knowledge across multiple systems. |
| Positive | The v1 blocking issues are resolved. (1) M1 requires deduction. (2) Reading Reward is now 4/5 across the board. (3) A script cannot solve this without access to the game data tables, and M1 requires testing multiple candidates — not just evaluating a formula. |

### v1 Blocking Issue Regression Check

| v1 Issue | Status | Evidence |
|----------|--------|----------|
| Pure computation, zero deduction | **Fixed** | M1 requires enumeration + dual-divisibility testing. M3 has a deliberate trap that punishes assumption. M5 requires distinguishing upgrade costs from training costs. |
| Reading Reward 2/5 | **Fixed** | All three testers score 4/5. Every moment requires at least one lookup. No numbers are given inline. |
| Scriptable / Snyder failure | **Fixed** | M1 requires enumerating unit candidates and testing two constraints. A script needs the full unit cost table AND a search strategy. This is no longer a 3-line solution. |

---

## Synthesis

| Dimension | Huang | Kenny | Katz | Average |
|-----------|-------|-------|------|---------|
| Clarity | 5 | 5 | 5 | 5.0 |
| Solvability | 4 | 4 | 4 | 4.0 |
| Elegance | 4 | 4 | 3 | 3.7 |
| Reading Reward | 4 | 4 | 4 | 4.0 |
| Fun | 3 | 3 | 3 | 3.0 |
| Confirmation | 4 | 4 | 4 | 4.0 |
| **Total** | **24** | **24** | **23** | **23.7** |

### Verdict: PASS (23.7/30)

The redesign resolves all three v1 blocking issues. Reading Reward jumps from 2.0 to 4.0. Elegance jumps from 1.3 to 3.7. The puzzle is now a legitimate game-knowledge challenge that requires consulting the world data, not a worksheet of inline division.

### v1 → v2 Comparison

| Dimension | v1 Average | v2 Average | Delta |
|-----------|-----------|-----------|-------|
| Clarity | 5.0 | 5.0 | — |
| Solvability | 5.0 | 4.0 | -1.0 |
| Elegance | 1.3 | 3.7 | **+2.4** |
| Reading Reward | 2.0 | 4.0 | **+2.0** |
| Fun | 2.0 | 3.0 | **+1.0** |
| Confirmation | 4.0 | 4.0 | — |
| **Total** | **19.3** | **23.7** | **+4.4** |

The solvability drop (-1.0) is expected and desirable — v1 was trivially solvable (5.0), which was a symptom of the problem, not a feature. The puzzle is now appropriately calibrated: challenging enough to require engagement, clean enough to avoid frustration.

### Consensus Issues

1. **Single deduction moment** (Minor): All three testers noted that only M1 involves genuine constraint satisfaction. M2-M6 are lookup-then-compute. A second deduction moment would push elegance higher. Not blocking — the puzzle passes — but a polish opportunity.
2. **No interlock** (Minor): The six moments remain independent. Each can be solved without reference to the others. This is the same structure as v1, but now acceptable because the individual moments are more engaging.
3. **M1 is the star** (Positive): All three testers praised M1's dual-divisibility constraint as genuine deduction. This is the moment that elevates the puzzle from "quiz" to "puzzle."
4. **Crossbowman trap in M3** (Positive): The deliberate mismatch between Archer and Crossbowman train times, with a fair warning hint, is well-crafted design. It rewards careful lookup and punishes assumption.
5. **Cross-table navigation in M5** (Positive): Requiring the solver to distinguish upgrade costs (techs.md) from training costs (economy.md) is exactly the kind of Reading Reward the v1 was missing.
6. **PATROL preserved** (Positive): The answer word survived the redesign intact. It remains thematic and well-confirmed.

### Blocking Issue Regression (all three testers agree)

| v1 Blocking Issue | v2 Status |
|-------------------|-----------|
| Was pure computation → now requires deduction? | **YES.** M1 is constraint satisfaction. M3 has a deliberate trap. M5 requires cross-table reasoning. |
| Was zero Reading Reward → now requires world/ lookups? | **YES.** Every moment requires at least one lookup. No values are given inline. Reading Reward: 2.0 → 4.0. |
| Was scriptable → now requires domain knowledge? | **YES.** M1 requires enumerating Archery Range units and testing divisibility. A solver (or script) needs the full unit table and a search strategy. |
