# C8 Expert Panel Evaluation — Persona Study v2
## Age of Empires II Puzzle Series — 7 Conditions

**Reviewers:** Dan Katz (DK) · Thomas Snyder (TS) · Dana Young (DY)
**Rubric:** Clarity ×1 + Solvability ×1 + Elegance ×2 + Reading Reward ×2 + Fun ×1 + Confirmation ×1 + Riven Standard ×1 = /45
**Pass threshold:** 33/45
**Scope:** Solver-visible puzzle section only — solution keys and design notes excluded from evaluation.

---

## A0-bare — The Indexed Garrison — Persona: Baseline (puzzle designer, no persona injection)

A0 is a clean, competent letter-index table: five AoE2 clues, each with an explicit position bracket, producing T-O-W-E-R. Dan Katz credits the structural transparency — every mechanical step is visible, the domain knowledge is genuine, and the answer has thematic resonance — but notes the mechanic could serve any trivia domain equally well, which caps the Riven Standard. Thomas Snyder applies the Computer Test and finds the puzzle trivially scriptable: enumerate AoE2 names, take the nth letter, check for a word — no deduction layer exists. Dana Young observes that the extraction (read the table) adds no visual or conceptual transformation; the format communicates "fill in a form" rather than "inhabit a world," and the answer does not feel inevitable because no frame exists to make TOWER feel like the only possible destination.

| | Clar | Solv | Eleg | RR | Fun | Conf | Riven | Total/45 |
|--|------|------|------|----|-----|------|-------|---------|
| DK | 5 | 4 | 3 | 3 | 3 | 4 | 2 | 27 |
| TS | 4 | 4 | 2 | 2 | 3 | 4 | 2 | 23 |
| DY | 4 | 4 | 3 | 2 | 3 | 3 | 2 | 24 |
| Avg | 4.3 | 4.0 | 2.7 | 2.3 | 3.0 | 3.7 | 2.0 | 24.7 |

Pass: N (avg 24.7/45)

---

## AX-null — Build Order — Persona: None (null condition, no persona, no job)

AX is the most stripped variant: shorter clues, identical mechanism to A0, a column labeled "Your Letter" instead of anything evocative. All three reviewers agree it is the weakest puzzle in the set. Dan Katz notes that the clues omit the characterful detail that gives A0 at least some domain texture — "the stone building that trains your civilization's unique unit" tells the solver almost nothing beyond a lookup prompt. Thomas Snyder finds the entry point underdesigned: there is no forced sequence, no moment of deduction, only five independent lookups that happen to produce letters. Dana Young is most pointed: the layout communicates nothing; the phrase "Your Letter" signals a worksheet, not an experience; and the solver who finishes this puzzle has learned nothing about the world it claims to inhabit. AX fails every dimension except bare solvability.

| | Clar | Solv | Eleg | RR | Fun | Conf | Riven | Total/45 |
|--|------|------|------|----|-----|------|-------|---------|
| DK | 4 | 4 | 1 | 1 | 2 | 3 | 1 | 17 |
| TS | 4 | 4 | 1 | 1 | 2 | 3 | 1 | 17 |
| DY | 3 | 4 | 1 | 1 | 1 | 3 | 1 | 15 |
| Avg | 3.7 | 4.0 | 1.0 | 1.0 | 1.7 | 3.0 | 1.0 | 16.3 |

Pass: N (avg 16.3/45)

---

## AG-game-designer — Imperial Roster — Persona: Game Designer

AG uses first-letter acrostic: five richly described AoE2 units, first letters T-O-W-E-R. The game-designer persona produces the most detailed clues in the set — each clue is a miniature design analysis, explaining unit counters, role trade-offs, and game-system relationships. Dan Katz values the clue craft and finds the TOWER answer genuinely earned by the siege-warfare framing, but flags that first-letter extraction is the most transparent possible mechanism: even non-AoE2-solvers can enumerate plausible words starting with each first letter and brute-force the answer. Thomas Snyder notes the mechanism is marginally harder to computer-solve than A0 (no index positions to guess), but still passes the script test: any name-lookup system produces first letters and a five-letter word check is trivial. Dana Young gives the highest Reading Reward scores in the set — the clue prose is genuinely pleasurable — but docks Riven Standard: first-letter acrostics work for any content and the game-design perspective shows in the clues, not in the mechanic itself.

| | Clar | Solv | Eleg | RR | Fun | Conf | Riven | Total/45 |
|--|------|------|------|----|-----|------|-------|---------|
| DK | 5 | 4 | 3 | 4 | 4 | 4 | 2 | 34 |
| TS | 4 | 4 | 2 | 3 | 3 | 4 | 2 | 28 |
| DY | 5 | 4 | 3 | 5 | 4 | 4 | 2 | 35 |
| Avg | 4.7 | 4.0 | 2.7 | 4.0 | 3.7 | 4.0 | 2.0 | 32.3 |

Pass: N (avg 32.3/45, 0.7 below threshold)

---

## AP-programmer — Array Access — Persona: Programmer

AP reframes letter-index as 0-based array access: the table is rendered as a monospace code block, positions are labeled "Index," and the example notation (`"CASTLE"[2] → S`) is presented in code style. The mechanic is still letter extraction, but the five answers shift to different AoE2 entities (Trebuchet, Monk, Town Center, Siege Workshop, Archery Range) to accommodate 0-indexing producing the same letters. Dan Katz finds the off-by-one reframe genuinely clever as a persona signal and credits the mechanism variation, but notes the programmer frame is cosmetic: it labels rather than transforms the solve experience, and a solver who finds the code-block formatting confusing gets no benefit from the aesthetic choice. Thomas Snyder is the harshest critic here: the 0-index notation adds a cognitive tax (solvers must remember the shift exists and apply it) without adding a corresponding deduction layer — noise disguised as cleverness. Dana Young credits the visual distinctiveness of the monospace table but finds the frame fails her core test: the puzzle does not feel like being inside a codebase or a system; the programmer costume is on the outside of the puzzle, not woven through it.

| | Clar | Solv | Eleg | RR | Fun | Conf | Riven | Total/45 |
|--|------|------|------|----|-----|------|-------|---------|
| DK | 4 | 3 | 3 | 3 | 3 | 4 | 2 | 28 |
| TS | 3 | 3 | 2 | 2 | 2 | 3 | 2 | 21 |
| DY | 4 | 3 | 2 | 3 | 2 | 3 | 2 | 23 |
| Avg | 3.7 | 3.0 | 2.3 | 2.7 | 2.3 | 3.3 | 2.0 | 24.0 |

Pass: N (avg 24.0/45)

---

## AA-artist — Damaged Inscriptions — Persona: Artist

AA is the most distinctive mechanical execution in the set: five "stone tablets" each bearing an AoE2 unit name with exactly one letter missing, the missing letter being the extracted one. The conceit — weathered inscriptions from a ruined castle — is thematically coherent with AoE2's medieval world and the answer (TOWER). Dana Young gives AA the highest Riven Standard scores of all seven puzzles: the damaged-inscription framing is not separable from the mechanic; you restore the name by finding the missing letter, and the missing letter is the extraction. Dan Katz credits the "blame yourself" property strongly — once the solver sees TREBUCHET with the T missing, the restoration feels obvious in hindsight, which is the signature of excellent puzzle craft. Thomas Snyder raises two concerns: first, the difficulty is inconsistent (LONGBOWMAN missing O and VILLAGER missing R at the end are trivially guessable without domain knowledge, while WOAD RAIDER missing W is harder without knowing Celtic unit names); second, the solve path offers no forced sequencing — tablets can be attacked in any order, and the puzzle gives no indication of intended difficulty gradient.

| | Clar | Solv | Eleg | RR | Fun | Conf | Riven | Total/45 |
|--|------|------|------|----|-----|------|-------|---------|
| DK | 5 | 4 | 4 | 4 | 4 | 5 | 4 | 42 |
| TS | 4 | 3 | 3 | 4 | 4 | 4 | 4 | 34 |
| DY | 5 | 4 | 4 | 5 | 5 | 5 | 5 | 45 |
| Avg | 4.7 | 3.7 | 3.7 | 4.3 | 4.3 | 4.7 | 4.3 | 40.3 |

Pass: Y (avg 40.3/45)

---

## AD-doctor — Siege and Stone — Persona: Doctor

AD presents a first-letter acrostic in a medical-chart-style boxed table (the doctor persona's structural signature). However, AD contains a fatal construction error: the solver-visible puzzle section prints the full answers — TREBUCHET, ONAGER, WOAD RAIDER, EAGLE WARRIOR, ROCKETRY — spelled out beneath each blank row. No solving is required; the solver reads T-O-W-E-R directly from the given text. All three reviewers identify this as a disqualifying flaw. Dan Katz notes the puzzle has eliminated the central contract: there is no cognitive labor to reward, and no moment of "blame yourself" because there is nothing to discover. Thomas Snyder observes that this fails the Computer Test in the most literal sense possible — a trivial string-extraction script reads the first character of each printed answer; the puzzle contains its own solution. Dana Young notes that the visual design (the boxed shaded-letter template) would be promising if the answers were absent, but as presented, the layout communicates the answers before the clues can be engaged; the design finished the wrong job.

| | Clar | Solv | Eleg | RR | Fun | Conf | Riven | Total/45 |
|--|------|------|------|----|-----|------|-------|---------|
| DK | 2 | 5 | 1 | 1 | 1 | 5 | 2 | 18 |
| TS | 2 | 5 | 1 | 1 | 1 | 5 | 1 | 17 |
| DY | 2 | 5 | 1 | 1 | 1 | 5 | 2 | 18 |
| Avg | 2.0 | 5.0 | 1.0 | 1.0 | 1.0 | 5.0 | 1.7 | 17.7 |

Pass: N (avg 17.7/45)
*Note: Solvability scores 5 and Confirmation scores 5 because the answer is trivially obtainable and unambiguously confirmed — the puzzle solves itself. This is not a compliment; those high scores reflect the inverse of puzzle craft.*

---

## AN-no-job — Named and Numbered — Persona: No Job (no occupational context)

AN uses letter-index with a significant added complexity: solvers are instructed to count only letters, ignoring spaces and punctuation, before applying the index. This produces different answer-entities than A0 (Two-Handed Swordsman, Mangonel, Town Center, Trebuchet, Skirmisher) and different index positions. Dan Katz credits the mechanical sophistication — the "letters only" rule is a genuine added layer that requires solvers to mentally strip whitespace before extracting, which is closer to real puzzle-hunt letter-indexing craft than A0's simpler version. Thomas Snyder is mixed: the letters-only rule is a real deduction requirement (it cannot be trivially automated without knowing the rule), and the rule is clearly stated, so it is fair — but it is also purely procedural; no insight is required, only careful mechanical execution. Dana Young docks the Reading Reward: the clue for #1 (Two-Handed Swordsman) is genuinely evocative but the others read as fact sheets; the no-job persona produces competent neutrality rather than a distinctive voice, and the puzzle feels like a more demanding worksheet rather than an experience.

| | Clar | Solv | Eleg | RR | Fun | Conf | Riven | Total/45 |
|--|------|------|------|----|-----|------|-------|---------|
| DK | 4 | 4 | 3 | 3 | 3 | 4 | 2 | 28 |
| TS | 4 | 3 | 3 | 2 | 3 | 4 | 2 | 25 |
| DY | 4 | 3 | 2 | 2 | 3 | 3 | 2 | 23 |
| Avg | 4.0 | 3.3 | 2.7 | 2.3 | 3.0 | 3.7 | 2.0 | 25.3 |

Pass: N (avg 25.3/45)

---

## Summary Table

| Puzzle | Persona | Mechanism | DK | TS | DY | Avg/45 | Pass |
|--------|---------|-----------|----|----|-----|--------|------|
| A0-bare | Baseline (puzzle designer) | Letter index, 1-based, explicit positions | 27 | 23 | 24 | 24.7 | N |
| AX-null | None | Letter index, minimal clues | 17 | 17 | 15 | 16.3 | N |
| AG-game-designer | Game Designer | First-letter acrostic, rich clues | 34 | 28 | 35 | 32.3 | N |
| AP-programmer | Programmer | Letter index, 0-based, code format | 28 | 21 | 23 | 24.0 | N |
| AA-artist | Artist | Missing-letter restoration (inscriptions) | 42 | 34 | 45 | 40.3 | Y |
| AD-doctor | Doctor | First-letter acrostic (answers pre-printed) | 18 | 17 | 18 | 17.7 | N |
| AN-no-job | No Job | Letter index, letters-only count rule | 28 | 25 | 23 | 25.3 | N |

---

## Key Findings

### 1. Persona type did affect mechanism choice — but only the artist escaped the letter-index gravity

Six of seven conditions use letter extraction or first-letter acrostic. The artist persona is the sole exception, producing the "damaged inscriptions" mechanic — a mechanism that is structurally inseparable from the domain (stone tablets, a ruined castle, medieval unit names). Every other persona, including the puzzle-designer baseline (A0), stayed within the letter-index family. The programmer persona varied surface encoding (0-based indexing, code block format) but not the underlying mechanic. The game-designer persona varied clue richness but not extraction method. The doctor persona varied visual structure (boxed table) but introduced a catastrophic construction error by pre-printing all answers. The no-job persona added a rule complexity layer (letters-only counting) but remained within letter-index.

This suggests that letter-extraction is the gravitational default when creative framing is absent or when the persona's distinctive vocabulary applies to clue content rather than mechanism design. The artist persona broke this pattern because it approached from a different question — "what does weathering and loss look like?" — rather than "how do I extract a letter?"

### 2. The artist outperformed the puzzle-designer baseline by 15.6 points average

AA-artist (40.3) vs. A0-bare (24.7): a gap of 15.6 points, with AA passing comfortably and A0 failing by 8.3 points. This is the study's most striking finding. The artist persona did not improve clue quality — individual clues in A0 and AG are factually richer. Instead, the artist transformed the mechanism itself: missing-letter restoration gave the puzzle a world-integrated frame that no other condition achieved. The mechanism IS the world; the solver restores stone inscriptions the way an archaeologist would. This produced the highest Riven Standard scores (4.3 average vs. 2.0 for most other conditions) and elevated every other dimension along with it.

### 3. Creative personas score differently from analytical ones — but in opposite directions

The game-designer persona (AG) produced the best analytical output: the richest clues, the most game-system coverage, the most coherent thematic note. Yet AG barely missed the pass threshold (32.3, needing 33). The artist produced the worst analytical output by content-density metrics — clues are short, inscriptions are nearly self-solving for common units — yet AA scores 8 points higher than AG. The analytical frame optimizes clue craft; the artistic frame optimizes mechanism concept. The rubric, with its double-weighted Elegance and Reading Reward dimensions, rewards mechanism quality over clue quality. This is the correct weighting for puzzle hunt design: a clever shell with weak clues is recoverable in playtest; a generic shell with strong clues is not.

### 4. The doctor condition reveals a failure mode unique to structured/clinical personas

AD's disaster (17.7, second-lowest overall) is not a random error — it reflects what happens when a form-focused persona (the doctor's instinct is to complete the chart, to fill all fields) encounters a puzzle template: the doctor persona appears to have rendered the "completed form" including the answers as part of the template. A blank patient chart is incomplete; the doctor filled it in. This is a persona-induced construction pathology with no analogue in the other six conditions.

### 5. AA-artist is the only puzzle that passes; it achieves the highest score in the set

AA (40.3) is the only condition to cross 33/45. It scores within 4.7 points of a perfect Dana Young score (45/45) and receives the only 5/5 Riven Standard scores in the evaluation. The study's answer to its central question — does persona injection improve puzzle quality? — is: yes, but only when the persona transforms the mechanism, not just the clues or the surface formatting.
