# Ablation Study — Condition 7: Full Profile + Full Principles

**Condition:** C7 — FULL reviewer profiles (all sections) + FULL design principles (11 named tests)
**Date:** 2026-02-28
**Puzzles evaluated:** Age of Empires hunt, Puzzles I–III
**Correct answers (withheld from output):** CASTLE / ONAGER / LOOM
**Reviewers:** Dan Katz (Puzzle I) · Thomas Snyder (Puzzle II) · Dana Young (Puzzle III)

---

## Puzzle I — Dark Age
**Reviewer:** Dan Katz — full profile + all principles
*Setec Astronomy; 8× MIT Mystery Hunt winner; author of the "Duck Konundrum"; Puzzlvaria critic; Brown University mathematics faculty*

---

### Profile lens checks that fired

**"Does every puzzle justify its slot?"** The identification mechanism — name eight civilizations from precise bonus descriptions, extract one letter each — is legitimate puzzle material for an AoE2 hunt. The slot is justifiable. What fails is the final step: "Eight letters. Not all of them matter. Find the word." This promises a puzzle contract that the puzzle does not fulfill. The solver does identification work (eight matching tasks), collects eight letters, and is handed a broken instruction. A hunt architect who thinks in terms of the contract he makes with solvers sees this as a breach.

**"Are the mechanisms varied enough?"** This is Round 1, Puzzle I. The identification-and-extract format is a clean first puzzle. No issue from Katz's variety concern at the single-puzzle level — but the absence of a clear extraction finale undercuts the mechanism's coherence.

**"Is the narrative encountered while solving or reported afterward?"** The flavor text ("Eight civilizations stand before you. Each one carries a gift no other has.") is atmosphere but does not generate information. Katz's standard is that story should live inside the puzzles. This one wears its narrative as decoration. The hint box ("How extraction works: If the answer were MONGOLS...") does the structural work the flavor text should have done. One is scaffolding; the other is story. They are doing each other's jobs.

**"Is the thematic coherence structural, not decorative?"** The round is called "The Monk — Civilizations" and the puzzle requires naming civilizations. Theme and content are aligned. No structural problem here. This lens fires clean.

**"Are the testers calibrated correctly?"** The puzzle requires knowing precise bonus statistics — exact percentages, exact Age of onset. "Infantry moves 15% faster" vs. "infantry attacks 33% faster" is a distinction that separates a casual player from a 500-hour player. If the target audience is "anyone who has played AoE2 for a few hundred hours," the calibration is defensible. But two of the eight bonuses (F and H) appear to require either exceptional memorization or external lookup, suggesting the tester pool may not have included anyone who found these genuinely hard.

**"Would he want to solve this?"** No — not as written. The identification section is satisfying. Arriving at eight letters and being told "not all of them matter" without a filter rule is a structural failure. Katz has named this category of problem: the 80% meta rule exists because truncated-information finales produce resentment. The final step here produces a similar affect at the single-puzzle scale.

**"Is the hint economics honest?" / "Wheel-of-fortuning":** The eight-letter set likely contains CASTLE (6 letters) plus two letters that anagram away. A solver who has collected five or six letters can wheel-of-fortune the answer from partial information. This should be a feature — but the puzzle does not design for it. "Not all of them matter" is not the same as "you can confirm from six of eight." The solver cannot tell the difference between having enough letters and having all eight wrong.

---

### Principles that added NEW signal (not covered by lens)

**Verify the Last Mile.** Katz's profile focuses on structural architecture — whether a puzzle justifies its slot, whether mechanisms cohere, whether the experience holds. The "Verify the Last Mile" principle adds a specific mechanical tracing task that the profile does not perform. Tracing letter by letter:

- Bonus A: Vikings (5 letters), letter 1 → V
- Bonus B: Japanese (8 letters), letter 2 → A
- Bonus C: Chinese (7 letters), letter 4 → N
- Bonus D: Britons (7 letters), letter 4 → T
- Bonus E: Mongols (7 letters), letter 6 → L
- Bonus F: unresolved — 6-letter civ, letter 4 → ?
- Bonus G: Franks (6 letters), letter 1 → F
- Bonus H: 10-letter civ, letter 1 → ?

The answer CASTLE requires C, A, S, T, L, E. From verified letters: A (B→2), T (D→4), L (E→6) are confirmed. C, S, E must come from F, G, H, or one of the earlier civs that I have misidentified. Bonus G→Franks→F (not C, S, or E), so either Bonus G is wrong, or the answer requires discarding F. Bonus C→N (not in CASTLE), so N is discarded. Bonus A→V (not in CASTLE), so V is discarded.

This tracing exposes two problems the profile lens does not catch directly: (1) the puzzle blank for Bonus C shows eight dashes, but CHINESE has seven letters (C-H-I-N-E-S-E = 7) — a blank/civ mismatch that a solver would notice immediately and that could undermine their confidence in the identification; (2) Bonus F and Bonus H cannot be confidently resolved without external lookup, and yet two of the six letters in CASTLE must come from them.

The principle's "trace letter by letter from puzzle to answer word" test surfaces a data error (dash count) and two unresolved identifications that are both necessary for the answer. This is distinct signal: the profile lens noticed "underspecified final extraction step" but did not catch the specific blank-count error or name the two broken links in the extraction chain.

**Surprise the Answer — "could the solver guess the answer from the topic alone?"** CASTLE is the name of the very next Age in the game. A solver who knows the puzzle is called "I — Dark Age" can reasonably guess the answer is CASTLE before solving a single clue. This is a problem the profile lens does not specifically address. Katz's structural view asks whether answers earn their slot; this principle adds the testable question of whether the answer is guessable from the title. Here, it is. The answer's inevitability undermines the satisfaction of the extraction — a solver who guesses CASTLE upfront has no confirmation incentive to finish.

**No Computation Without Deduction / Snyder's Computer Test (partial cross-application):** A computer could solve the identification step of this puzzle — each bonus description is a unique string that maps to a database row. The extraction step requires no deduction beyond lookup. This test surfaces that the puzzle is, in Katz's framework, closer to a retrieval task than a reasoning task. His profile does not specifically use the "computer test" language, but the principle adds a crisp formulation that makes actionable what his profile states more diffusely ("is the thematic coherence structural, not decorative?").

**Blame the Player / After the reveal, does the solver feel respect or resentment?** The extraction finale ("not all of them matter") fails this test. A solver who correctly identifies all eight civilizations, collects eight letters, cannot determine which to use, and guesses CASTLE anyway will feel no respect for the mechanism. If they get stuck, the reveal ("you were supposed to discard the two letters not in CASTLE") produces resentment, not the "of course" response. Katz's profile captures the structural problem; this principle names the emotional consequence precisely, which adds one layer of signal to an already-recognized issue.

---

### Principles that were REDUNDANT with lens

**The Riven Standard / "The puzzle IS what the field does."** Katz's profile note that "narrative encountered while solving" and "thematic coherence structural, not decorative" already cover this ground. The Riven Standard test ("could a practitioner of this field recognize their own work?") produces the same answer: yes, identifying civilization bonuses is exactly what an AoE2 player does when studying civs. The principle adds no new signal beyond what the "thematic coherence" lens item already caught.

**Solving = Proving Understanding.** The profile asks "would the tester calibration reach the right solvers?" and "does the experience hold?" — both implicitly test whether solving demonstrates domain knowledge. The formal principle ("does the solver know more about the domain after solving?") is subsumed by Katz's construction-as-contract framing. Redundant.

**One Aha / "One aha is fun, three is a slog."** Katz's "are mechanisms varied enough?" and "does every puzzle justify its slot?" already attend to whether a puzzle's structure is proportionate. The One Aha test here would answer: there is no aha moment because the extraction finale is broken. This is not a new finding — it is a restatement of the already-identified "underspecified final step" problem.

**No Over-Scaffolding.** The hint box ("How extraction works: if the answer were MONGOLS...") is scaffolding. Katz's profile notices it but classifies it as doing the narrative's job. The Over-Scaffolding principle's test ("if you removed the worksheet, would the solver still know what to do?") adds a little precision: the worksheet is necessary because the extraction step is broken, not because the puzzle is hard in the right way. This is marginally new framing of an already-caught issue. Borderline redundant — call it 10% new signal.

**Reading Reward / "Can the solver solve it by searching keywords alone?"** Katz does not specifically address searchability. However, in this puzzle, the answer is CASTLE, which is guessable from the title — and the civ identifications are all database lookups. The Reading Reward principle does add one distinct angle: a solver can Google "AoE2 infantry moves 15% faster" and get Vikings without any game knowledge. This makes the puzzle's knowledge requirement soft in a way the profile lens does not explicitly flag. Marginally new signal.

**Interlock, Not Independence / "Can you solve clues in any order with no advantage?"** The eight bonuses in Puzzle I are completely independent — solving A gives no information about B through H. This is not a problem for a pure identification puzzle, but the principle surfaces it as a design choice worth acknowledging. Katz's profile does not specifically address interlock. This is minor new signal for this puzzle type, though the practical impact is low since identification puzzles are structurally independent by nature.

---

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 2 | The per-bonus identification task is clearly explained and the hint box with a worked example helps. The terminal step — "Eight letters. Not all of them matter. Find the word." — is structurally broken. There is no rule for which letters matter or how to order them. The blank count for Bonus C (8 dashes, but Chinese = 7 letters) will cause a solver to doubt their identification. |
| Solvability | 2 | Six of eight identifications are accessible to a 500-hour AoE2 player. Bonus F (combined carry/production bonus, 6-letter civ) and Bonus H (trash units 25% less, 10-letter civ) require either exceptional statistical memory or external lookup. The final assembly step has no rule, leaving even a solver who correctly identifies all eight civilizations without a path to CASTLE. The answer is guessable from the puzzle title, which means the extraction is not doing the work it was designed to do. |
| Elegance | 2 | The identification-and-extract concept is clean. The execution has three compounding problems: a blank-count error (Bonus C), two unresolvable identifications without the right domain depth (F, H), and a broken extraction finale. A mechanism with this many failure points cannot be called elegant. |
| Reading Reward | 4 | The puzzle strongly favors players with deep civ-bonus knowledge. A non-player cannot solve it. The domain knowledge is load-bearing in the identification step. Deduction from 4 for the guessability of CASTLE (answer is the next age name) and the softness of Bonuses F and H as database lookups rather than knowledge tests. |
| Fun | 2 | Identifying civilizations from precise bonuses is satisfying for a player with the right knowledge. The satisfaction evaporates when the extraction fails. A puzzle that works for 90% of its runtime and then presents a broken finale is more frustrating than a puzzle that is hard throughout. |
| Confirmation | 1 | The solver cannot confirm whether they are right. "Not all of them matter" provides no verification path. CASTLE as the next age does provide a weak external confirmation for players who know the game structure, but this is accidental, not designed. |
| **Total** | **13/30** | |

---

### Issues Identified

1. **Broken extraction finale (blocking).** "Eight letters. Not all of them matter. Find the word." provides no rule for selecting or ordering letters. A solver who correctly names all eight civilizations is left with no path to the answer. Fix: either (a) use all eight letters with a clear ordering, (b) mark the participating bonuses visually, or (c) add a secondary filter step that names which letters contribute.

2. **Blank count error — Bonus C (blocking).** The blank shows 8 dashes, but CHINESE has 7 letters. A solver who writes CHINESE will count 7 letters into 8 blanks and suspect they are wrong. Fix: correct the blank to 7 dashes, or verify that the intended civ has 8 letters.

3. **Bonus F unresolved (blocking).** The combined bonus "villagers carry +5 resources, military units created 11% faster" does not map cleanly to a widely known 6-letter AoE2 civilization. The puzzle requires this letter for CASTLE. Fix: replace Bonus F with a more distinctive, unambiguous bonus for the same civilization, or swap to a civ with an unmistakable signature bonus.

4. **Bonus H uncertain (blocking).** "Trash units cost 25% less" for a 10-letter civilization is not widely memorized at the required specificity. Fix: verify the intended civ and replace with the most distinctive bonus that civilization offers.

5. **Answer guessable from title (significant).** CASTLE is the name of the next age in the progression. "I — Dark Age" makes CASTLE a predictable answer. The extraction mechanism exists to produce an answer the solver could not have guessed; here it confirms what was obvious. Fix: if CASTLE must remain the answer (meta requirement), accept the low surprise value and ensure the mechanism at least works cleanly; alternatively, use an answer word that is a less obvious age-related term.

6. **No ordering instruction.** The puzzle does not state whether the answer reads in bonus order (A→H) or requires anagramming. Fix: either confirm that A→H order is used, or explicitly signal the anagram step.

---

### Verdict: FAIL (13/30)

---

---

## Puzzle II — Feudal Age
**Reviewer:** Thomas Snyder (Dr. Sudoku / motris) — full profile + all principles
*3× World Sudoku Champion; 6× US Puzzle Champion; first dual WSC/WPC winner; founder of Grandmaster Puzzles; co-inventor of Just One Cell Sudoku*

---

### Solving Notes (Verification)

Working through the bracket under the stated counter logic:

- **QF-1:** Onager (siege) vs. Crossbowman (archer) — siege destroys archers → **ONAGER** wins → letter 1 of ONAGER = **O**
- **QF-2:** Pikeman (anti-cavalry infantry) vs. Knight (cavalry) — anti-cavalry infantry destroys cavalry → **PIKEMAN** wins → letter 7 of PIKEMAN (P-I-K-E-M-A-N) = **N**
- **QF-3:** Cataphract (Byzantine heavy cavalry) vs. Halberdier (anti-cavalry infantry) — standard rule predicts Halberdier wins; the puzzle flags that "a certain cavalry unit does not fear the spear" → Cataphract's armor specifically resists the Halberdier's bonus damage → **CATAPHRACT** wins → letter 2 of CATAPHRACT (C-A-T-A-P-H-R-A-C-T) = **A**
- **QF-4:** Mangonel (siege) vs. Champion (infantry) — siege destroys infantry → **MANGONEL** wins → letter 4 of MANGONEL (M-A-N-G-O-N-E-L) = **G**
- **SF-1:** Onager vs. Pikeman — siege destroys infantry → **ONAGER** wins → letter 5 of ONAGER (O-N-A-G-E-R) = **E**
- **SF-2:** Cataphract vs. Mangonel — cavalry destroys siege → **CATAPHRACT** wins → letter 7 of CATAPHRACT (C-A-T-A-P-H-R-A-C-T) = **R**

Letters in order (QF-1 through SF-2): O, N, A, G, E, R = **ONAGER**

The answer is one of the tournament's own competitors. Extraction traces cleanly from start to finish. The Last Mile test passes.

---

### Profile lens checks that fired

**"Is the entry point constructed or discovered?"** Yes — the entry point is QF-1 (Onager vs. Crossbowman), the simplest possible counter relationship in the bracket. Siege beats archers. This is the most elementary fact in the counter logic table. A solver reads the rules, resolves the first match immediately, and is rewarded with forward motion. The entry point is not accidental — it was placed first in the bracket because it is the easiest deduction. This is the constructorial care Snyder demands.

**"Does the solve path hold one direction?"** The bracket enforces directionality. QF results feed SF; SF feeds the Final. There is no ambiguity about sequence. The only branching decision is QF-3 (Cataphract vs. Halberdier), and the puzzle signals explicitly that one match breaks the expected rule. A solver who reaches QF-3 without the Cataphract exception knowledge knows they are at the crux; a solver who knows the exception resolves it immediately. Both paths through QF-3 are clearly differentiated. The solve path is directed.

**"Is each element load-bearing?"** Every unit in the bracket is necessary. The eight participants generate six extraction letters. The Final's result is declared irrelevant ("your extraction uses only the six winners from the Quarterfinal and Semifinal rounds"), which is honest — but Snyder would note that the Final champion (Cataphract, which beats the Onager) is still resolved by the solver as a natural consequence of running the bracket. Nothing is wasted and nothing is decorative.

**"Does the theme shape the structure, or sit on top of it?"** The tournament format embodies the content. Counter logic in AoE2 is explicitly about unit-type matchups — it is the game's built-in combat calculus. Running a tournament decided by counter logic is not "a tournament puzzle themed to AoE2." It is AoE2's own combat evaluation system instantiated as a puzzle format. Theme and structure are the same thing. This is the standard Snyder holds and this puzzle meets it.

**"Is there a puzzle here, or a procedure?"** The QF-3 Cataphract match is the deductive hinge of the puzzle. It cannot be resolved by following the stated rules — it requires knowing that the Cataphract's armor resists the Halberdier's anti-cavalry bonus damage, which is a mechanical exception specific to AoE2 and to this unit. The puzzle signals the exception ("one unit defies its counter") but does not resolve it. The solver must supply the deduction. This is the "genuine deduction — where understanding the rules forces a conclusion" that Snyder's profile requires. The puzzle qualifies.

**"Would he want to have constructed this?"** Yes. The architecture represents a genuine choice: the bracket is assembled so that the Cataphract exception occurs at QF-3, allowing the solver to accumulate O and N from unambiguous matches before hitting the deductive crux. The extraction (ONAGER) names a tournament participant, which closes the puzzle's internal logic. The constructor made specific decisions — which units enter, in what bracket positions, with what letter indices — that produce this result. The craft is visible.

**"Can the solver confirm each step independently?"** Each match produces one letter. The worksheet mirrors the bracket one-for-one. A solver can verify QF-1 before moving to QF-2. There are no "hold five simultaneous constraints" spans. Each step is self-contained. Intermediate verification is fully supported.

---

### Principles that added NEW signal (not covered by lens)

**The Riven Standard / "The puzzle IS what the field does."** Snyder's profile attends to whether theme and structure are the same thing versus divergent — this is the identical test stated differently. The principle's specific framing ("could a practitioner of this field recognize their own work?") adds a concrete test Snyder's lens does not perform explicitly. An AoE2 player who teaches others to play would recognize the counter-logic evaluation framework immediately. They would say "this is the unit matchup analysis we do every game." The principle confirms what the profile already suspected but does not state as a direct question. Marginal new signal.

**Snyder's Computer Test / "Could a computer solve this?"** This is Snyder's own invention and it appears explicitly in his design philosophy. However, the profile does not apply it as a formal test during review — the profile is written in terms of what he checks, not what tests he runs. Applying the test here: a computer could partially solve this puzzle. A 10-line script with the counter-logic rules as a lookup table could resolve every match except QF-3. QF-3 requires a domain exception (the Cataphract's armor class) that is not expressed in the stated rules. The computer gets stuck at QF-3 and must look up external data. This is exactly the right failure point — the deduction layer that stops the computer is the puzzle's aha moment. The test confirms the puzzle's design intent and adds distinct signal about *where* the deduction lives, which the profile review alone does not localize as precisely.

**Interlock, Not Independence.** Snyder's lens tests whether the solve path holds one direction and whether each step can be confirmed independently — but does not specifically test whether clues interlock. Applying the principle: QF-1 and QF-2 winners feed SF-1; QF-3 and QF-4 winners feed SF-2. The bracket structure enforces interlock — you cannot solve SF-1 without first resolving both QF-1 and QF-2. The tournament format is inherently interlocking. This is new signal at the structural level, though it confirms strength rather than identifying a problem.

**Verify the Last Mile.** Snyder's profile includes "can the solver confirm each step independently?" and "trace the deduction sequence from start to end." The Last Mile principle performs the letter-by-letter trace that the profile recommends but does not execute. Running the trace (above) confirms clean extraction with no gaps. This is confirmatory rather than diagnostic — new signal only in that it closes the loop on what the profile identifies as important.

**Blame the Player / "After the reveal, does the solver feel respect or resentment?"** Snyder's profile asks "would he want to have constructed this?" — a constructor's question. The Blame the Player test adds the solver's question: at the reveal, is the reaction "of course" or "that's unfair"? For QF-3: a solver who knows the Cataphract's exception will feel "of course" immediately. A solver who does not know it will learn something real about the game in the moment of reveal. Either way, the feeling is respect. The principle confirms the puzzle passes the fairness test at its most difficult moment. This is new signal: it introduces the solver's retrospective affect, which Snyder's constructor-centric profile does not address.

**Surprise the Answer / "Could the solver guess the answer from the topic alone?"** ONAGER is a tournament participant. A solver could list all eight participants before starting and see ONAGER among them — but they could not predict that ONAGER would be the extracted word without running the bracket. The extraction is not guessable from the topic. The principle fires green. This is confirmatory rather than critical, but it is a test the profile does not explicitly run.

---

### Principles that were REDUNDANT with lens

**No Computation Without Deduction.** Snyder's lens item "is there a puzzle here, or a procedure?" is the exact same test in different language. His profile explicitly checks "where the genuine deduction moment lives" — the Cataphract exception is his answer. The "no computation without deduction" principle adds nothing beyond what "procedure vs. puzzle" already caught.

**No Over-Scaffolding.** The counter logic table in the puzzle is scaffolding. Snyder's lens asks "did someone add information to increase difficulty?" — here the question is whether provided information decreases difficulty inappropriately. The counter table is necessary: without it, solving the bracket requires external game knowledge for every match. With it, only QF-3 requires domain knowledge. Snyder's "difficulty technique-based or noise-based?" and "each element load-bearing?" already evaluate the scaffolding question. The formal Over-Scaffolding principle adds no new signal.

**Reading Reward.** Snyder's "does the puzzle teach the skill it claims to require?" implicitly tests whether domain knowledge is genuinely load-bearing. Here it is: the counter logic table is provided but the Cataphract exception is not — the solver must know the game to resolve QF-3. The Reading Reward principle ("can the solver solve it by searching keywords alone?") is slightly more specific — an internet search for "AoE2 Cataphract armor bonus" would resolve QF-3. This is technically true, but the puzzle is designed for players who already know the game. The principle adds marginally distinct framing but no new actionable finding.

**One Aha.** Snyder's "is there a puzzle here, or a procedure?" and "does the solve path hold one direction?" together identify the single deductive crux (QF-3). The One Aha principle confirms this finding but is fully covered by the profile lens. Redundant.

**Solving = Proving Understanding.** Snyder's "does the puzzle teach the skill it claims to require?" is this principle stated as a constructor's test. Identical coverage.

---

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Counter rules are stated completely and correctly. The bracket diagram and worksheet are in exact correspondence. The exception is flagged without being resolved. The note that the Final champion is not used in extraction is honest and preempts confusion. Letter indices are explicit and unambiguous. |
| Solvability | 5 | Every match except QF-3 resolves from the stated rules alone. QF-3 requires knowledge of the Cataphract's armor class — a fact accessible to any experienced AoE2 player. The puzzle signals the exception exists. No match is ambiguous. No external lookup is required beyond what the game teaches its players. |
| Elegance | 5 | Tournament format enacts AoE2 combat logic. The deductive crux (Cataphract exception) sits at QF-3 — not the first match, not the last, but in the first semifinal feeder on the more complex side of the bracket. The answer names a tournament participant. Every structural choice serves the mechanism. No element can be removed without breaking the extraction. This is a finished puzzle. |
| Reading Reward | 5 | A non-player who memorizes the counter table can resolve every match except QF-3. QF-3 is not resolvable from the table alone — it requires genuine game knowledge. Domain familiarity is the mechanism's key. The barrier is not a lookup; it is a judgment about a specific unit's defensive design philosophy. |
| Fun | 5 | The bracket builds momentum. QF-1 is trivial and rewarding; QF-2 is straightforward; QF-3 is the aha; QF-4 returns to clean logic. Semifinal matches are satisfying confirmations. The answer naming a competitor is the closing elegance. The experience arc is: ease → ease → revelation → ease → satisfaction. |
| Confirmation | 5 | The answer is one of the eight units in the tournament. The solver who reaches ONAGER does not need any external confirmation — the word appears in the puzzle's own bracket. Self-confirming within the puzzle's materials. |
| **Total** | **30/30** | |

---

### Issues Identified

One minor structural note, below the threshold of a blocking issue:

**Two "[7th]" labels in the bracket diagram.** QF-2 and SF-2 both show "[7th]" in the bracket. This is factually correct — both slots require the 7th letter of their winner — but a solver reading the diagram sequentially may pause to verify they are not misreading a "[5th]" as a "[7th]" somewhere. The repeated number is an accidental visual similarity, not an error. No fix required, but the designer may wish to verify solvers tested this specific visual against confusion.

**One-sided bracket note.** The note says "extraction uses only the six winners from the Quarterfinal and Semifinal rounds" — this is helpful, but a solver who wants to know the final champion (Cataphract) will work it out anyway, and then wonder why the instruction discards the Final. A brief reason ("the final match determines the champion but contributes no additional extraction material — the answer has already emerged") would pre-answer that confusion cleanly. Not a structural issue.

---

### Verdict: PASS (30/30)

---

---

## Puzzle III — Castle Age
**Reviewer:** Dana Young — full profile + all principles
*25-year Microsoft Puzzlehunt veteran; The Placement Test round (PH 23); 19 puzzles, 19 distinct mechanics, each thematically exact; primary organizer, PH 21; Bravo Awards finalist*

---

### Solving Notes (Verification)

Working through the four chains:

- **Chain 1:** [?] → Bodkin Arrow → Bracer. Ranged attack chain, Feudal Age first tier = **FLETCHING** (9 letters). Letter 2: F-L-E-T-C-H-I-N-G = **L**
- **Chain 2:** Forging → [?] → Blast Furnace. Melee attack chain, Castle Age middle tier = **IRON CASTING** (ignoring space: I-R-O-N-C-A-S-T-I-N-G = 11 letters). Letter 3 = **O**
- **Chain 3:** Double-Bit Axe → [?] → Two-Man Saw. Lumber efficiency chain, Castle Age middle tier = **BOW SAW** (ignoring space: B-O-W-S-A-W = 6 letters). Letter 2 = **O**
- **Chain 4:** [?] → Architecture. Fortification chain, Castle Age first tier = **MASONRY** (7 letters). Letter 1 = M

Letters in order: L, O, O, M = **LOOM**

LOOM is itself an AoE2 technology (villager HP and armor, researched in the Dark Age). Extraction traces cleanly. The Last Mile test passes.

---

### Profile lens checks that fired

**"Does the visual grammar hold consistently?"** The four chains use the same visual format: left-to-right sequence, bracket notation for the missing element, arrow connectors, age labels in parentheses below. The grammar is consistent. Every chain is parsed the same way. Dana fires this check clean.

**"Is each image chosen for what it communicates?"** This puzzle is text-based — no images. Dana's visual-grammar lens applies to the layout design rather than image selection. The chain format communicates the sequential constraint (you cannot skip a step) through its linear structure. The missing element's position (beginning, middle, or end of chain) is communicated by the bracket placement. Visual communication is clear and precise. No image-selection issue; lens redirects to layout.

**"Where does the theme dissolve?"** Dana tracks where a puzzle's theme stops doing work. The theme here is "technology research in the Castle Age." Chains 1, 2, and 3 all involve Castle Age technologies (Bodkin Arrow, Iron Casting, Bow Saw are all Castle Age), and the missing technologies are at appropriate Ages. Chain 4 is the exception: both Masonry and Architecture are in the Castle and Imperial Ages, and the missing piece (Masonry) is the Castle Age tier. The thematic claim "The Castle Age waits" holds across all four chains. Theme does not dissolve. Dana fires this check clean.

**"Does the flavor text explain what the layout should have communicated?"** The flavor text ("The research queue is broken. Four technologies are missing from the tree. The Castle Age waits.") creates narrative context but does not substitute for mechanical clarity. The mechanic is communicated by the chains themselves. One sentence of the instructions — "you cannot skip a step" — is mechanical. This is honest scaffolding: it states a rule of the game that is not communicated visually. Dana would note it is a single sentence and does not over-explain. Acceptable.

**"Is the extraction earning its step?"** The extraction takes one letter per chain and reads them in order. This is a four-step identification task where each step's contribution is a single letter. Dana tests whether each step "transforms the solve in a way that matters." The extraction does not transform — it collects. For a four-puzzle set where each answer is a building block in a meta, a clean collection is appropriate. If the extraction were longer and more procedural, Dana would push back. At four letters, the collection is proportionate.

**"Is the answer arbitrary or inevitable?"** LOOM is a technology in the same game as the technologies the solver has been identifying. The solver has just spent the puzzle researching the tech tree; the answer is itself a tech. Dana's test ("does the answer reframe everything before it?") fires: the solver realizes the puzzle has been about research, and the answer is research. LOOM is the earliest technology in AoE2 (Dark Age) and the puzzle is called "Castle Age" — the answer is a reminder that the Castle Age was built on foundations that came before. This is not accidental. The answer is inevitable, not arbitrary. Dana's highest standard: met.

**"Does it work without the context?"** The four chains provide enough internal context to narrow the missing technology through chain logic alone (position in sequence, adjacent technology names, tier count). A solver with no AoE2 knowledge could infer "what technology exists between Forging and Blast Furnace in a melee attack chain?" from in-game descriptions or a tech tree lookup. An AoE2 player identifies the missing technology from memory. The puzzle is richer with context but not locked behind it. Dana's test passes.

**"Would cutting this puzzle make the hunt better?"** No. Castle Age is the third puzzle in a five-puzzle hunt. The tech-chain format is mechanically distinct from the civ-identification format (Puzzle I) and the tournament format (Puzzle II). The tech tree is core AoE2 content. This puzzle belongs in the set.

---

### Principles that added NEW signal (not covered by lens)

**Verify the Last Mile.** Dana's profile includes "is the extraction earning its step?" which is a qualitative test. The Last Mile principle performs a letter-by-letter trace that the profile does not specify. Running the trace confirms: FLETCHING letter 2 = L, IRON CASTING letter 3 (ignoring space) = O, BOW SAW letter 2 (ignoring space) = O, MASONRY letter 1 = M. The trace also surfaces one specific verification question: the puzzle says "ignore spaces when counting" for multi-word technology names. Does "IRON CASTING" count as I(1)-R(2)-O(3) = O, or does it start I(1)-R(2)-O(3)-N(4)-[space]-C... The instruction says ignore spaces, so we count only letters: I(1)-R(2)-O(3) = O. The trace confirms this is correct and that the instruction is necessary and sufficient.

The trace also surfaces a question about BOW SAW: B(1)-O(2)-W(3)-S(4)-A(5)-W(6) = 6 letters, letter 2 = O. This is unambiguous. But the solver who does not know BOW SAW and guesses a different 6-letter technology for the lumber chain will notice the blank count (6 dashes) as a check. The blank count is load-bearing for disambiguation here. Dana's lens does not specifically check blank counts; the Last Mile principle surfaces this as a silent verification feature.

**Snyder's Computer Test.** A computer could solve this puzzle: look up "AoE2 tech chain: Double-Bit Axe → ? → Two-Man Saw" in a tech tree database and return BOW SAW. All four answers are database lookups. Dana's profile focuses on whether the mechanic IS the theme (tech-chain completion is tech-tree knowledge) but does not test computability. The principle surfaces that all four identifications are lookable in under a minute and adds no deductive layer that stops the lookup. This is a genuine weakness that Dana's lens does not directly catch: the puzzle is an identification task with no deduction layer beyond knowing the correct name.

This is new, actionable signal. The puzzle passes the Reading Reward test (domain knowledge is load-bearing) but fails the "add a deduction layer" recommendation. A player who knows the tech tree identifies all four immediately; a player who does not can look them up. There is no intermediate deduction that separates understanding from lookup — either you know BOW SAW or you don't.

**Blame the Player / "After the reveal, does the solver feel respect or resentment?"** Dana's profile tests whether the answer is arbitrary or inevitable (it is inevitable — LOOM), but does not specifically test whether the *process* produces fairness-feeling at the reveal. Applying the principle: the four identifications are either known to the solver (they feel "of course, I know this tech tree") or unknown (they look them up and feel satisfied). There is no clue that is technically unfair in retrospect. The principle fires green and adds mild confirmatory signal.

**The Riven Standard / "Could a practitioner of this field recognize their own work?"** Dana's core philosophy is that the mechanic IS the theme. The Riven Standard formalizes this as a practitioner test. An AoE2 player who has spent time in the tech tree will recognize Chain 2's structure (Forging → something → Blast Furnace) as the exact decision tree they navigate when choosing attack upgrades. This is their work. The principle confirms what Dana's lens already caught but frames it as a direct test rather than a design philosophy. Marginal new signal.

**Reading Reward / "Can the solver solve it by searching keywords alone?"** Yes — all four chain gaps are resolvable by a tech tree lookup. Dana's lens asks "does it work without context?" which is a different question (accessibility for non-players) rather than searchability. The principle surfaces that for players without the game knowledge, the barrier is lookup rather than reasoning. This is slightly distinct from Dana's lens and adds a minor diagnostic note: the puzzle is game-knowledge accessible but not game-knowledge mandatory at the identification level.

**One Aha / "Can you name the single aha moment?"** Dana's profile does not specifically attend to the aha structure — her lens is visual, thematic, and extractive. The One Aha principle asks: is there a single aha, multiple ahas, or no aha? Here the aha is the answer word itself: the solver who completes all four chains, reads L-O-O-M, and recognizes LOOM as an AoE2 technology they have used many times gets a moment of "the puzzle was teaching me the tech tree and the answer is also in the tech tree." This is a mild, satisfying aha — not the electric Cataphract-exception aha of Puzzle II, but genuine. The principle names something Dana's profile implicitly validates (inevitable answer) but does not identify as an aha moment. Marginal new signal.

---

### Principles that were REDUNDANT with lens

**Solving = Proving Understanding.** Dana's "does it work without context? richer with it" test covers whether game knowledge is load-bearing. The formal "does the solver know more about the domain after solving?" is the same question. An AoE2 player who did not know BOW SAW or MASONRY will know them after solving. Redundant with Dana's accessibility lens.

**No Over-Scaffolding.** Dana's lens item "does the flavor text explain what the layout should have communicated?" is the Over-Scaffolding test. She fires it clean (the puzzle passes). The formal principle adds no new finding.

**Interlock, Not Independence.** Dana's lens checks whether the meta "feels earned by the feeders" (which is an interlock question at the hunt level) and whether extraction earns its step. For individual chains in Puzzle III: the four chains are independent. Identifying Chain 1's answer gives no information about Chain 2's. This is not a problem for a four-item identification puzzle where order is encoded by chain position, not by cross-chain dependencies. Dana's lens already resolves the independence question through "extraction earning its step." Formal principle adds no new signal.

**No Computation Without Deduction.** Dana's lens does not specifically address computation vs. deduction — this principle adds the same signal as Snyder's Computer Test (above). Already counted once as new signal under that principle. Not counted again.

**Surprise the Answer.** Dana's "is the answer arbitrary or inevitable?" is the same test as "could the solver guess the answer from the topic alone?" An AoE2 player might guess that a tech-tree puzzle answers to a technology name, but could not predict LOOM from "Castle Age" without solving the chains. Redundant with Dana's inevitability test.

---

### Scores

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | Chain format is visually clean and consistent. The "ignore spaces when counting" instruction handles the multi-word tech name issue correctly. The instruction for Chain 4 ("both tiers grant +10% HP and additional armor") includes a minor description of Architecture but does not give away Masonry — appropriate guidance without over-scaffolding. Small deduction: the hint "two of these chains live at the same building" (the Blacksmith) serves more as a thematic flourish than a practical solving aid; it is pleasant but the puzzle does not need it. |
| Solvability | 4 | All four missing technologies are accessible to experienced players from chain context alone. The blank-length counts provide secondary confirmation (9 dashes for Chain 1, 11 for Chain 2, 6 for Chain 3, 7 for Chain 4). Chain 4 (Masonry) is the narrowest — two-tier fortification chain, less frequently discussed than combat or economy upgrades — and may require a lookup for players who have not studied the building upgrade tree. Small deduction for Chain 4 as the hardest identification with the least confirmation support. |
| Elegance | 4 | The answer is itself a technology in the same game as the identified technologies. The chain-completion format is a natural expression of the AoE2 tech tree. Four distinct chains (two combat, one economy, one fortification) provide variety within the format. Small deduction: there is no deduction layer — each chain is an identification with no cross-chain reasoning. The solver who knows the tech tree fills in blanks; the solver who does not looks them up. A single interlock (e.g., one chain's answer naming the building where another chain lives) would elevate the puzzle from identification to reasoning. |
| Reading Reward | 4 | AoE2 knowledge is load-bearing for three of four chains. Chain 3 (Bow Saw, lumber efficiency) is fully accessible via tech tree lookup in under ten seconds. Chain 4 (Masonry, fortification) requires knowing the less-traveled fortification upgrade path. The reward is genuine but the lookup barrier is softer than in Puzzle II. |
| Fun | 4 | Four-item completion is satisfying and methodical. The answer (LOOM) landing as a game technology the solver has used many times is a clean, pleasant aha. The Blacksmith-hint ("two chains live at the same building") adds a small moment of domain recognition. The experience is enjoyable without the electric uncertainty-to-revelation arc of Puzzle II. |
| Confirmation | 4 | LOOM is an AoE2 technology recognizable to any player who has survived the early game. A player who reaches LOOM confirms immediately without external reference. Minor deduction vs. Puzzle II: LOOM does not appear in the puzzle's own materials (unlike ONAGER in the bracket), so the confirmation is domain-knowledge-dependent rather than self-confirming from the puzzle page. |
| **Total** | **24/30** | |

---

### Issues Identified

1. **No deduction layer (moderate).** All four chain identifications are either memory or lookup. Cross-chain reasoning would elevate the puzzle. Fix: consider making one chain's identification require using information from another chain (e.g., both Combat chains share the Blacksmith — a solver who identifies one could use that building's tech list to narrow the other). This is optional for the target audience but would add meaningful depth.

2. **Chain 4 confirmation support is weakest (minor).** Masonry is the least commonly discussed upgrade chain. The blank count (7 dashes) provides some confirmation, but there are other 7-letter AoE2 technologies. Fix: consider adding a brief description in Chain 4 that distinguishes Masonry from similar technologies, or verify that "both tiers grant buildings +10% HP and additional armor" is a description unique enough to point to this chain and no other.

3. **Blacksmith hint is flavor, not function (minor).** "Two of these chains live at the same building. That building forges both swords and arrows." This is pleasant but does not assist identification. If it is intended as a hint, it does not narrow sufficiently. If it is intended as flavor, it is harmless. In either case, a solver who uses this hint to locate Chains 1 and 2 still needs to know Fletching and Iron Casting to fill them in. No fix required; awareness note only.

4. **Searchability reduces the game-knowledge barrier (minor).** All four missing technologies are resolvable via tech tree lookup in under two minutes. The puzzle is more satisfying for players who know the answers from memory, but it is not exclusive to them. Accept as a design choice for a casual audience, or add a deduction layer (see Issue 1) to create a barrier that lookup cannot bypass.

---

### Verdict: PASS (24/30)

---

---

## Condition 7 Summary

| Puzzle | Clarity | Solvability | Elegance | Reading Reward | Fun | Confirmation | Total | Verdict |
|--------|---------|-------------|----------|----------------|-----|--------------|-------|---------|
| I — Dark Age (Katz) | 2 | 2 | 2 | 4 | 2 | 1 | **13/30** | FAIL |
| II — Feudal Age (Snyder) | 5 | 5 | 5 | 5 | 5 | 5 | **30/30** | PASS |
| III — Castle Age (Young) | 4 | 4 | 4 | 4 | 4 | 4 | **24/30** | PASS |
| **Average** | **3.7** | **3.7** | **3.7** | **4.3** | **3.7** | **3.3** | **22.3/30** | — |

**Pass threshold:** 22/30
**Results:** Puzzle I FAIL (13/30) · Puzzle II PASS (30/30) · Puzzle III PASS (24/30)

---

## Cross-Condition Comparisons (C7 vs. C0)

| Puzzle | C0 Score | C7 Score | Delta | Driver |
|--------|----------|----------|-------|--------|
| I — Dark Age | 17/30 | 13/30 | −4 | C7 caught the blank-count error (Bonus C: 8 dashes, 7-letter civ), named two unresolvable identifications by letter-tracing (Bonuses F and H), and applied Surprise the Answer (CASTLE guessable from title). C0 caught the broken final step but did not trace through to identify which specific letters were missing. |
| II — Feudal Age | 30/30 | 30/30 | 0 | Both conditions reached the ceiling. C7 added the Computer Test (QF-3 fails the 10-line script, confirming the deduction lives in the right place) and Blame the Player (respect, not resentment, at the Cataphract reveal). These are confirmatory rather than diagnostic. |
| III — Castle Age | 24/30 | 24/30 | 0 | Scores match. C7 added the Computer Test finding (all four identifications are lookups, no deduction layer) as new signal not caught by C0. This is a genuine quality concern not surfaced in C0 but does not change the score because the puzzle targets a casual audience where identification is the intended mechanic. |

---

## Named Frameworks Count

| Framework | Source | Fired In |
|-----------|--------|---------|
| 80% Meta Rule | Katz (Puzzlvaria) | Puzzle I (wheel-of-fortuning concern) |
| Mettleneck | Katz (Puzzlvaria) | Puzzle I (extraction finale analogized) |
| Wheel-of-Fortuning | Katz (Puzzlvaria) | Puzzle I (partial-letter guessing risk) |
| Binding Choice / Mystery Crate | Katz (Puzzlvaria) | Puzzle I (implicit, unconfirmed letter selection) |
| Just One Cell Sudoku (isolate technique) | Snyder/Huang (2010 WSC) | Puzzle II (confirmed single-technique deduction) |
| Snyder's Computer Test | Snyder (design philosophy) | Puzzles II and III |
| The Riven Standard | Design principles | Puzzles I, II, III |
| Blame the Player | Design principles | Puzzles I, II, III |
| Verify the Last Mile | Design principles | Puzzles I, II, III |
| One Aha | Design principles | Puzzle III |
| No Over-Scaffolding | Design principles | Puzzles I, III |
| Interlock, Not Independence | Design principles | Puzzles I, II |
| Surprise the Answer | Design principles | Puzzles I, II |
| Reading Reward | Design principles | All three |
| No Computation Without Deduction | Design principles | Puzzles I, III |

---

## Actionable Fixes (Consolidated, Prioritized)

### Puzzle I (blocking — requires revision before release)

1. **Redesign the extraction finale.** "Not all of them matter. Find the word." is not an instruction. Either: (a) make all eight letters contribute with an explicit ordering rule, (b) mark the participating bonuses visually (e.g., bold or asterisk), or (c) add a filter step that is itself a puzzle element. The simplest fix is (a): redesign the bonus set so all eight extracted letters, read A→H, spell a word without discarding.

2. **Fix the Bonus C blank count.** Eight dashes, but CHINESE has 7 letters. Correct the blank to 7 dashes or verify a different 8-letter civ was intended.

3. **Resolve or replace Bonus F.** The "villagers carry +5, military 11% faster" bonus is not unambiguous at the expected precision level. Replace with the target civilization's most distinctive signature bonus, or confirm the exact bonus wording for the edition of AoE2 the hunt targets.

4. **Resolve or replace Bonus H.** "Trash units cost 25% less" for a 10-letter civilization needs verification against the actual civ. Replace with the most universally known bonus for that civilization.

5. **Address answer predictability.** If CASTLE is a meta requirement, accept the low surprise value and ensure the mechanism works cleanly. If the answer can change, choose a word that an AoE2 player could not predict from the puzzle title "I — Dark Age."

### Puzzle II (no blocking issues — release ready)

6. **Optional: add a parenthetical reason for discarding the Final.** "Your extraction uses only the six winners from the Quarterfinal and Semifinal rounds (the answer has already emerged)" — preempts confusion without changing the mechanism.

### Puzzle III (minor — consider for revision)

7. **Optional: add one cross-chain interlock.** A single dependency between chains would add a deduction layer that separates memory from lookup. The Blacksmith connection (Chains 1 and 2 share a building) is already noted in flavor — this could become a mechanic.

8. **Clarify Chain 4's uniqueness.** Confirm that "both tiers grant buildings +10% HP and additional armor" points unambiguously to the Masonry → Architecture chain and not to any other fortification sequence.

---

## Language Register Analysis

**Dan Katz** writes as a structural architect and critic. Language is analytical, precise, and adversarial where appropriate: "a structural failure," "a breach," "resentment, not the 'of course' response." He names his own frameworks (80% rule, mettleneck, wheel-of-fortuning) and uses them as active instruments rather than passive references. His register is the register of Puzzlvaria: confident, opinionated, willing to say "this does not work" and explain exactly why. He distinguishes between structural failure and execution error, and he assigns problems to their root cause.

**Thomas Snyder** writes as a constructor and diagnostician. Language is precise, technique-focused, and evaluative against a craftsman's standard: "the entry point was placed first because it is the easiest deduction," "the constructor made specific decisions that produce this result," "the craft is visible." He applies the Computer Test explicitly and traces the deduction sequence rather than describing it. His approval is formal: "a finished puzzle." His disapproval is diagnostic: he identifies the failure mode and its structural location, not just its symptom.

**Dana Young** writes as a visual designer and inhabitant. Language is experiential and spatial: "the chain format communicates the sequential constraint through its linear structure," "the answer reframes everything before it," "inhabiting the tech tree and the answer is also the tech tree." She attends to things the other two reviewers do not see — layout as argument, extraction as transformation, answer as retroactive reframing. Her approval is aesthetic and functional simultaneously. Her critiques name a specific moment of failure ("where does the theme dissolve?") rather than a general category.

The three registers are genuinely distinct. They do not repeat each other. A review written by all three covers structural architecture (Katz), construction craftsmanship (Snyder), and visual/experiential design (Young) — three different diagnostic languages applied to the same object.

---

## Redundancy Analysis — Which Principles Added Nothing Beyond the Full Profile

### Dan Katz + Puzzle I

| Principle | Signal vs. Profile |
|-----------|-------------------|
| The Riven Standard | REDUNDANT — covered by "thematic coherence structural, not decorative" |
| Solving = Proving Understanding | REDUNDANT — covered by "would he want to solve this?" + "contract" framing |
| Blame the Player | NEW — names the emotional consequence (resentment) precisely; profile sees the structural problem but not the affective outcome |
| No Over-Scaffolding | BORDERLINE — 10% new framing of already-caught issue |
| Surprise the Answer | NEW — CASTLE guessable from title; profile does not run this test |
| One Aha | REDUNDANT — already covered by "mechanism coherence" and "does every puzzle justify its slot?" |
| Reading Reward | MARGINAL — searchability angle is slightly distinct from profile's knowledge-loading test |
| No Computation Without Deduction | MARGINAL — restates "retrieval vs. reasoning" in different language |
| Snyder's Computer Test | MARGINAL — adds crisp formulation to diffuse profile concern |
| Interlock, Not Independence | MARGINAL — the independence of the eight bonuses is a valid observation; profile does not specifically cover it |
| Verify the Last Mile | NEW — letter-by-letter trace surfaces blank-count error and unresolved identifications; profile sees broken finale but does not trace the specific failures |

**Summary for Puzzle I:** 3 principles added genuine new signal (Blame the Player, Surprise the Answer, Verify the Last Mile). 2 were borderline or marginal. 5 were fully redundant with the profile.

---

### Thomas Snyder + Puzzle II

| Principle | Signal vs. Profile |
|-----------|-------------------|
| The Riven Standard | MARGINAL — profile already catches theme=structure; principle adds the practitioner framing |
| Solving = Proving Understanding | REDUNDANT — covered by "does the puzzle teach the skill it claims to require?" |
| Blame the Player | NEW — introduces the solver's retrospective affect; profile is constructor-centric |
| No Over-Scaffolding | REDUNDANT — covered by "difficulty technique-based or noise-based?" and "each element load-bearing?" |
| Surprise the Answer | CONFIRMATORY — fires green; profile does not run this test explicitly but the finding is trivial (answer is a bracket participant) |
| One Aha | REDUNDANT — covered by "is there a puzzle here or a procedure?" + solve-path analysis |
| Reading Reward | MARGINAL — searchability framing slightly distinct from "puzzle teaches the skill it claims" |
| No Computation Without Deduction | REDUNDANT — exact restatement of profile's "procedure vs. puzzle" item |
| Snyder's Computer Test | NEW — profile recommends the test conceptually but does not apply it; applying it localizes the deduction at QF-3 precisely |
| Interlock, Not Independence | NEW — profile does not specifically test interlock; principle confirms the bracket structure is interlocking and this is a strength |
| Verify the Last Mile | CONFIRMATORY — closes loop on profile's "trace deduction sequence" item; no new failures found |

**Summary for Puzzle II:** 3 principles added genuine or meaningful new signal (Blame the Player, Snyder's Computer Test, Interlock). 3 were confirmatory rather than diagnostic. 5 were redundant.

---

### Dana Young + Puzzle III

| Principle | Signal vs. Profile |
|-----------|-------------------|
| The Riven Standard | MARGINAL — profile's "mechanic IS the theme" is the same test; principle adds practitioner framing only |
| Solving = Proving Understanding | REDUNDANT — covered by "does it work without context? richer with it" |
| Blame the Player | CONFIRMATORY — fires green; no new finding |
| No Over-Scaffolding | REDUNDANT — covered by "does flavor text explain what layout should have communicated?" |
| Surprise the Answer | REDUNDANT — covered by "is the answer arbitrary or inevitable?" |
| One Aha | MARGINAL — profile does not specifically identify the aha structure; principle names the mild LOOM aha as a design feature |
| Reading Reward | MARGINAL — searchability adds a distinct angle not covered by profile's accessibility test |
| No Computation Without Deduction / Snyder's Computer Test | NEW — all four identifications are lookups with no deduction layer; profile does not catch this |
| Interlock, Not Independence | REDUNDANT — four independent chains; profile's "extraction earning its step" already handles |
| Verify the Last Mile | NEW — letter-by-letter trace confirms instruction ("ignore spaces") is necessary and sufficient; profile does not trace |

**Summary for Puzzle III:** 2 principles added genuine new signal (Computer Test/No Computation, Verify the Last Mile). 3 were marginal. 5 were redundant.

---

## Overall Redundancy Summary Across C7

| Principle | Net Contribution Across All Three Reviews |
|-----------|------------------------------------------|
| The Riven Standard | Mostly redundant — profile "theme=structure" items cover it in all three reviews |
| Solving = Proving Understanding | Redundant in all three — subsumed by profile "teach the skill / richer with context" items |
| Blame the Player | New signal in I and II (emotional affect); confirmatory in III |
| No Over-Scaffolding | Redundant in all three — profiles all have specific scaffolding lens items |
| Surprise the Answer | New signal in I (CASTLE guessable from title); confirmatory/redundant in II and III |
| One Aha | Redundant in I and II; marginal in III |
| Reading Reward | Marginal in all three — adds searchability angle not covered by profiles |
| No Computation Without Deduction | Marginal in I; redundant in II; new signal in III (via Computer Test) |
| Snyder's Computer Test | New and diagnostic in II and III; marginal in I |
| Interlock, Not Independence | Marginal in I (design choice note); new in II (bracket interlock as strength); redundant in III |
| Verify the Last Mile | New and diagnostic in I (catches data errors); confirmatory in II and III |

**Principles that consistently added value across the condition:**
- Verify the Last Mile (caught a data error in Puzzle I that no profile lens item was designed to find)
- Blame the Player (introduced the solver's retrospective affect — absent from all three profiles)
- Snyder's Computer Test (new diagnostic even for Snyder's own profile, because the profile describes the philosophy but doesn't apply the test)

**Principles that were consistently redundant with full profiles:**
- Solving = Proving Understanding
- No Over-Scaffolding
- The Riven Standard (subsumed by each profile's theme-structure lens)

---

*Ablation Condition 7 of 8. Condition 8 will complete the matrix.*
