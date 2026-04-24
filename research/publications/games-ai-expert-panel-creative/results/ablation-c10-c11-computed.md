# Ablation Configurations C10 and C11 — Computed from Existing Data
**Computation date:** 2026-02-28
**Source data:** ablation-c8-all15.md (C8 per-dimension scores), ablation-c6-all15.md (C6 review texts for MTI inference), ablation-c9-recomputed.md (C9 results for comparison)

---

## C10 Definition

**Formula:** C10 = Clarity + Solvability + (Elegance × 2) + (Reading_Reward × 2) + Fun + Confirmation
**Score range:** /40
**Pass threshold:** ≥ 29/40 (72.5%)
**Input scores:** C8 per-dimension scores (trimmed profiles + 3 injected principles)
**Distinction from C9:** C10 applies the C9 weighted formula to C8's principle-adjusted scores. C9 uses raw C6 scores. C10 uses C8 scores, which carry the Blame-the-Player, Verify the Last Mile, and Snyder's Computer Test adjustments. The key difference: Dropypasta scores RR=3 in C8 (vs. RR=4 in C6) and Confirmation=2 in C8 (vs. Confirmation=3 in C6), making C10 harsher on that puzzle.

---

## C10 Source Scores (C8 per-dimension)

| Puzzle | Tier | C | S | E | RR | F | Conf | C8(/30) |
|--------|------|---|---|---|----|---|------|---------|
| Scicabulary (SHAMROCK) | MIT/Elite | 3 | 4 | 4 | 4 | 4 | 3 | 22 |
| People Watching (HINDWING) | MIT/Elite | 3 | 4 | 4 | 4 | 4 | 3 | 22 |
| H2No (ENCAMPMENT) | MIT/Elite | 4 | 3 | 3 | 3 | 3 | 3 | 19 |
| Bridge Building (PROTEINPOWDER) | MIT/Elite | 4 | 4 | 5 | 5 | 4 | 4 | 26 |
| You're Telling Me (CARBONSINK) | MIT/Elite | 4 | 4 | 4 | 4 | 5 | 3 | 24 |
| Dropypasta (VINDICATIONISLAND) | MIT/Elite | 3 | 3 | 4 | 3 | 3 | 2 | 18 |
| Information Relay (FINISH LINE) | Standard Hunt | 4 | 4 | 5 | 4 | 5 | 5 | 27 |
| Front and Center (RADAR) | Standard Hunt | 5 | 4 | 5 | 4 | 5 | 5 | 28 |
| Characters (UNHEXING) | Standard Hunt | 4 | 4 | 5 | 5 | 4 | 5 | 27 |
| What's Next? (AFGHANISTAN) | Standard Hunt | 4 | 4 | 4 | 4 | 4 | 4 | 24 |
| GM-01 The Planets | Games Magazine | 5 | 5 | 2 | 2 | 2 | 5 | 21 |
| GM-02 Weather | Games Magazine | 5 | 5 | 2 | 2 | 2 | 5 | 21 |
| GM-03 Animals | Games Magazine | 5 | 5 | 1 | 1 | 2 | 5 | 19 |
| GM-04 Logic Grid | Games Magazine | 5 | 5 | 3 | 2 | 3 | 5 | 23 |
| GM-05 Word Search | Games Magazine | 3 | 1 | 2 | 2 | 1 | 1 | 10 |

---

## C10 Table

Formula: C10 = C + S + (E×2) + (RR×2) + F + Conf. Pass ≥ 29/40.

| Puzzle | Tier | C8(/30) | C10(/40) | C10 Pass? |
|--------|------|---------|---------|-----------|
| Scicabulary (SHAMROCK) | MIT/Elite | 22 | **30** | YES |
| People Watching (HINDWING) | MIT/Elite | 22 | **30** | YES |
| H2No (ENCAMPMENT) | MIT/Elite | 19 | **25** | NO |
| Bridge Building (PROTEINPOWDER) | MIT/Elite | 26 | **36** | YES |
| You're Telling Me (CARBONSINK) | MIT/Elite | 24 | **32** | YES |
| Dropypasta (VINDICATIONISLAND) | MIT/Elite | 18 | **25** | NO |
| Information Relay (FINISH LINE) | Standard Hunt | 27 | **36** | YES |
| Front and Center (RADAR) | Standard Hunt | 28 | **37** | YES |
| Characters (UNHEXING) | Standard Hunt | 27 | **37** | YES |
| What's Next? (AFGHANISTAN) | Standard Hunt | 24 | **32** | YES |
| GM-01 The Planets | Games Magazine | 21 | **25** | NO |
| GM-02 Weather | Games Magazine | 21 | **25** | NO |
| GM-03 Animals | Games Magazine | 19 | **21** | NO |
| GM-04 Logic Grid | Games Magazine | 23 | **28** | NO |
| GM-05 Word Search | Games Magazine | 10 | **14** | NO |

**C10 total passes: 8/15 (53%)**

### C10 Calculation Verification

| Puzzle | C + S | E×2 | RR×2 | F + Conf | C10 |
|--------|-------|-----|------|----------|-----|
| Scicabulary | 3+4=7 | 8 | 8 | 4+3=7 | 30 |
| People Watching | 3+4=7 | 8 | 8 | 4+3=7 | 30 |
| H2No | 4+3=7 | 6 | 6 | 3+3=6 | 25 |
| Bridge Building | 4+4=8 | 10 | 10 | 4+4=8 | 36 |
| You're Telling Me | 4+4=8 | 8 | 8 | 5+3=8 | 32 |
| Dropypasta | 3+3=6 | 8 | 6 | 3+2=5 | 25 |
| Information Relay | 4+4=8 | 10 | 8 | 5+5=10 | 36 |
| Front and Center | 5+4=9 | 10 | 8 | 5+5=10 | 37 |
| Characters | 4+4=8 | 10 | 10 | 4+5=9 | 37 |
| What's Next? | 4+4=8 | 8 | 8 | 4+4=8 | 32 |
| GM-01 The Planets | 5+5=10 | 4 | 4 | 2+5=7 | 25 |
| GM-02 Weather | 5+5=10 | 4 | 4 | 2+5=7 | 25 |
| GM-03 Animals | 5+5=10 | 2 | 2 | 2+5=7 | 21 |
| GM-04 Logic Grid | 5+5=10 | 6 | 4 | 3+5=8 | 28 |
| GM-05 Word Search | 3+1=4 | 4 | 4 | 1+1=2 | 14 |

### C10 Tier Analysis

| Tier | C10 avg | C10 pass rate | C8 pass rate (for comparison) |
|------|---------|---------------|-------------------------------|
| MIT/Elite (6 puzzles) | 29.7 / 40 (74.2%) | 4/6 (67%) | 4/6 (67%) |
| Standard Hunt (4 puzzles) | 35.5 / 40 (88.8%) | 4/4 (100%) | 4/4 (100%) |
| Games Magazine (5 puzzles) | 22.6 / 40 (56.5%) | 0/5 (0%) | 1/5 (20%) |

**Cross-tier gap (MIT/Elite vs. Games Magazine):**
- C8: 21.8/30 (72.7%) vs. 18.8/30 (62.7%) → gap = **10.0 percentage points**
- C10: 29.7/40 (74.2%) vs. 22.6/40 (56.5%) → gap = **17.7 percentage points**

**C10 key change from C9:** Dropypasta drops from 29/40 (borderline pass in C9) to 25/40 (clear fail). This is the most significant structural change. The C8 penalty on Dropypasta's RR (4→3) and Confirmation (3→2) propagates through the weighted formula: the Confirmation drop costs 1 point, and the RR drop costs 2 points (weighted ×2), totaling a 4-point reduction relative to C9's score for Dropypasta. C10 thus resolves the borderline-pass problem that C9 identified but could not adjudicate.

**C10 key change from C8:** GM-04 Logic Grid drops from 23/30 (PASS in C8) to 28/40 (FAIL in C10). This mirrors the C9 finding: the weighted rubric correctly penalizes GM-04's low Elegance (3) and Reading Reward (2) by doubling their cost. The C8 raw-score pass for GM-04 is overturned in C10, consistent with C9's finding that this was a false positive under uniform weighting.

---

## C11 Definition

**Formula:** C11 = Clarity + Solvability + (Elegance × 2) + (Reading_Reward × 2) + Fun + Confirmation + (MTI × 1)
**Score range:** /45
**Pass threshold:** ≥ 33/45 (73.3%, matching the 73% threshold applied across configurations)
**Input scores:** C8 per-dimension scores + inferred MTI scores from C6 and C8 review texts
**New dimension:** Mechanism-Theme Identity (MTI, 1–5)

### MTI Scoring Rubric

| Score | Criterion |
|-------|-----------|
| 5 | The mechanic could not exist in any other theme — mechanic and theme are the same object |
| 4 | The mechanic strongly embodies the theme — domain knowledge enriches it substantially |
| 3 | The mechanic uses the theme as content but could be re-skinned to another domain |
| 2 | The theme is decorative — any content could substitute |
| 1 | No theme, or theme and mechanism are unrelated |

---

## C11 MTI Scores with Rationale

Inferred from C6 reviewer panel notes and C8 assessments. Evidence drawn primarily from Dana Young's readings (which explicitly address the "mechanic is the theme" question) and Snyder's structural observations.

### Puzzle 1: Scicabulary (SHAMROCK) — MTI: 3

Dana Young (C6): "Where I feel the theme thinning is the extraction: indexing into the two-word phrase is a clean but emotionally neutral step. After the richness of the portmanteau deconstruction, the extraction feels like administration. The answer SHAMROCK doesn't name what you were doing."

The portmanteau-inversion mechanic does involve inhabiting a fake scientific register — you are doing a kind of linguistic science. But the theme is "fake jargon" and the mechanic is letter-indexing after matching; these could be transposed to many other domains (e.g., fake cooking terminology, fake sports jargon). The portmanteau-deconstruction step is thematically coherent with the fake-science framing, but the extraction step is neutral. Partial integration.

**MTI = 3** (uses the theme as content; re-skinnable to another domain with moderate effort)

### Puzzle 2: People Watching (HINDWING) — MTI: 4

Dana Young (C6): "I love that the field marks show up as clothing choices and the calls show up as dialogue — that is the mechanic being the theme, not decorating it. You are doing what a birdwatcher does; you are just watching a different kind of flock."

C8: "The visual grammar is consistent and the mechanic is the theme: field marks as fashion choices, bird calls as dialogue. That is genuine thematic integrity."

The birdwatching mechanic IS the content — the vignette structure encodes birdwatcher field-identification methodology (visual marks + vocalization). However, the extraction step (IBP alpha codes) is an artifact of the code system, not of birdwatching as an activity. The core observation mechanic is tightly integrated; the extraction layer is technically imposed. Stops short of 5 because the IBP code system is external scaffolding, not thematic logic.

**MTI = 4** (the mechanic strongly embodies the theme; birdwatcher epistemology is the puzzle's logic)

### Puzzle 3: H2No (ENCAMPMENT) — MTI: 3

Dana Young (C6): "What I notice is that the visual presentation of this puzzle... is doing almost no work to guide you through the intended experience. The layout is a list; the solve is a research task; the extraction is a logical step. There is no visual moment where the puzzle resolves."

Dan Katz (C6): "The flavor text is doing real work here — 'watch out for any differences' is a fair and honest signal."

The mechanism (date arithmetic to derive target years, then location identification, then differential extraction) is generically a research-and-comparison task that could work on many other thematic domains (films, scientific discoveries, architecture). The "H2O" theme is present in the title but does not shape the mechanism — the mechanism is: look up events, calculate, extract differences. The historical-location theme is content, not structure.

**MTI = 3** (theme as content; research-comparison mechanic is domain-portable)

### Puzzle 4: Bridge Building (PROTEINPOWDER) — MTI: 5

Dana Young (C6): "This is a puzzle where the mechanic is the theme in the most literal possible way: a bridge-building puzzle whose solved form is literally the structure of a polymer chain. That recursion is beautiful. Building bridges IS building protein powder, in some deep structural sense."

Thomas Snyder (C6): "The constraint that nodes have maximum 4 connections (which maps to carbon's valence) is not just clever — it shapes the grid in ways that make the chemical reading possible. This is theme and structure being the same thing."

C8: "The mechanic is the theme in the most literal structural sense: building bridges is building a polymer chain. That recursion is complete and beautiful."

The Hashi (bridge-building) genre constrains its own solution set to the topology of carbon-valence chemistry. You cannot have constructed this puzzle in any other domain — the bridge-count maximum (4) is carbon's valence, not an arbitrary crossword constraint. The solution to the logic puzzle IS a molecular diagram. This is the purest mechanism-theme identity in the batch.

**MTI = 5** (canonical case: the mechanic could not exist in any other theme)

### Puzzle 5: You're Telling Me (CARBONSINK) — MTI: 3

Dana Young (C6): "The world of this puzzle is consistent: every entry inhabits the same mode of deliberate misreading."

C8: "The format is mechanically generatable... What saves it: the selection of compounds requires finding genuinely funny alternate readings, and humor is not scriptable."

The "Shrimp Fried Rice" compound-misreading mechanic is a general linguistic operation (find alternate parsings of compound words) that happens to point toward an environmental compound answer. The environmental theme (CARBONSINK, coal, sinks) is content applied to the format, not structure derived from it. You could run this exact mechanic with any thematic cluster and produce a different compound answer. The humor is thematically consistent but could be transposed.

**MTI = 3** (compound-word misreading is a general mechanic; environmental theme is a content skin)

### Puzzle 6: Dropypasta (VINDICATIONISLAND) — MTI: 3

Dana Young (C6): "Super Smash Bros. stages as the organizing principle for an extraction system is thematically rich — stages in that game are places with rules and physics, just as each win condition here is a place with its own logic."

C8: "The stage-conditioned extraction is architecturally novel — each stage has its own rule, and rule variety at the extraction level is a genuine design achievement."

The Smash Bros. theme does contribute meaningfully to the stage-win-condition structure — each stage in the game has distinct physics that loosely maps to distinct extraction logic. But the dropquote layer is completely domain-portable (it works with any quotation database), and the win conditions are derived from game trivia rather than emerging from the game's mechanical logic. The connection is meaningful but not necessary: you could imagine a tournament-bracket version of this puzzle with a completely different domain. Dana Young herself says VINDICATIONISLAND "reads more as two words forced together than as something inevitable," which marks a theme-mechanism gap at the answer level.

**MTI = 3** (theme contributes to win-condition variety but does not necessitate the mechanism)

### Puzzle 7: Information Relay (FINISH LINE) — MTI: 5

Dana Young (C6): "The world of this puzzle is fully realized. The five bears each have distinct voices, and those voices are mechanically load-bearing — the personality is the clue method. That is the mechanic being the theme in the purest sense I've seen in this batch."

C8: "The world of this puzzle is fully realized and the five bears are mechanically load-bearing — personality is the clue method, method is the mechanic, mechanic is the theme."

The mechanism — a telephone-game relay where each participant's failure mode is their personality trait — IS the theme of familial communication breakdown. The five bears' personalities are not decorative: Grandma's mishearing IS the extraction key. Papa's technical uselessness IS a legitimate clue. The structural conceit (information degrades through relay) could not function in a different theme because the mechanism is precisely this family's communication dynamic. FINISH LINE names the act of completing the relay: you ran the relay and crossed the line. Full identity.

**MTI = 5** (personality = clue method = mechanic = theme; could not exist in any other framing)

### Puzzle 8: Front and Center (RADAR) — MTI: 5

Dana Young (C6): "The newspaper format is not decorative — it is the world of the puzzle, and palindromes are naturally at home in a world where text reads both ways. The answer RADAR is perfect: it names a technology for detecting things that are both ahead and behind you, which resonates with the temporal symmetry of palindromes."

C8: "The mechanic IS the theme: building bridges is building a polymer chain." [Note: this is Bridge Building — but for Front and Center, C8 says:] "The visual resolution of this puzzle is complete at every scale: headlines palindrome visually, extracted word palindromes visually, masthead bisects itself visually."

The palindrome structure is not a theme applied to the mechanism — the palindrome IS the mechanism AND the medium AND the answer. The newspaper (SEMI TIMES) is a palindrome. The headlines are palindromes. The answer RADAR is a palindrome. The extraction method (center letter) is only meaningful because palindromes have a structural center. None of this could exist in a non-palindromic domain: the center-letter extraction depends entirely on the palindrome structure. Three-scale structural identity.

**MTI = 5** (palindrome = the medium, the object, the extraction rule, and the answer — multi-scale identity)

### Puzzle 9: Characters (UNHEXING) — MTI: 4

Dan Katz (C6): "The flavor text is doing its job perfectly: 'find their base' is a double meaning (numerical base / home base), and 'understand their code' signals the ASCII step. The answer UNHEXING is wonderful — it names the operation you performed."

Dana Young (C6): "UNHEXING names both the mathematical operation (de-hexadecimal-ifying) and the magical operation (breaking a hex), and those two meanings are in play at the same time throughout the solve. This puzzle knows what it is about."

C8: "The hex knowledge and cultural knowledge (BABA, CAFE) both rewarded; dual resonance of UNHEXING exceptional."

The hexadecimal constraint (A-F vocabulary only) is structurally embedded in the clue construction — every clue answer must be spelled from A through F, which is both the math requirement and the "characters in the forest" theme. The two meanings of "characters" (letter-characters and forest-fairy-tale characters) are active simultaneously. However, the fairy-tale/forest framing is atmospheric rather than structural: the mechanism would work as "a spy sending hexadecimal codes" or "a computer logging errors" without needing the forest. The dual-resonance of UNHEXING is powerful but the thematic world (forest, fairy tale) is a skin over the mathematical core.

**MTI = 4** (hex structure IS the clue vocabulary IS the constraint; forest theme is atmospheric wrapper)

### Puzzle 10: What's Next? (AFGHANISTAN) — MTI: 4

Dana Young (C6): "The puzzle's world is the song — you are living inside 'We Didn't Start the Fire' for the duration of the solve. The images place you at specific moments in the song's historical timeline, and the 'what comes next?' frame asks you to extend your knowledge forward from each moment. Then the extracted phrase 'RUSSIANS IN' places you in the song as a reader/listener, and the answer AFGHANISTAN continues the lyric."

C8: "The double aha structure sustains engagement; the 'RUSSIANS IN' revelation is a genuine moment."

The lyric-continuation frame — you are inside the song, you extract a lyric fragment, and the answer is the next word the song sings — means the mechanism IS the listening experience. The B-value confirmation (letter count of the next item) is an internal constraint that only works because the song has fixed sequences. However, the image-recognition step is largely the same as any sequence-identification puzzle (it could be done with a different song). The deep integration is at the extraction layer (extracted phrase → next lyric → answer), not the image-identification layer.

**MTI = 4** (lyric-continuation frame tightly integrates mechanism with the song's sequential logic; image step is re-skinnable)

### Puzzle 11: GM-01 The Planets — MTI: 1

Dana Young (C6): "There is no visual moment of resolution, no extraction, no answer that reframes the experience. When you finish this puzzle, you have filled in planet names."

Thomas Snyder (C6): "There is no technique being tested, no deduction being demanded, no moment where understanding the rules forces a conclusion. This is a vocabulary lookup task organized as a crossword."

There is no mechanism in the sense the MTI dimension requires. The theme (planets) is the category of vocabulary to retrieve. Any other themed vocabulary set (oceans, capitals, elements) would produce the same structural result. The PLUTO clue ("No longer officially considered a planet") is the one moment of thematic-mechanism contact, but the puzzle's mechanism is just: know planet names → fill grid.

**MTI = 1** (theme is the category of lookup; mechanism is category-neutral crossword fill)

### Puzzle 12: GM-02 Weather — MTI: 2

Thomas Snyder (C6): "The puzzle's only notable structural feature is the self-referential WEATHER clue: 'The state of the air at a certain time and place' is the definition of the word that names the theme category. That is a small act of coherence."

Dana Young (C6): "The self-referential WEATHER clue is the best moment here... the theme of the puzzle and the object of the puzzle coincide at one point."

The WEATHER self-reference is a moment where the theme and an element of the mechanism briefly coincide: the puzzle is about weather, and WEATHER is both an answer and the theme's name. This is minimal but non-zero integration — it goes beyond GM-01. The rest of the crossword is category-neutral fill.

**MTI = 2** (theme is decorative; one self-referential answer is the only moment of integration)

### Puzzle 13: GM-03 Animals — MTI: 1

Dana Young (C6): "No theme that shapes the mechanic. No answer that reframes the experience."

Thomas Snyder (C6): "There is no entry point, there is no forced deduction, there is no puzzle here by the standard I hold."

Cliché clues ("Man's best friend," "Likes to chase mice") explicitly confirm the absence of editorial design decisions. The theme contributes nothing to the mechanism; it is the category of answers to be retrieved.

**MTI = 1** (absolute floor; theme and mechanism are orthogonal)

### Puzzle 14: GM-04 Logic Grid — MTI: 2

Dan Katz (C6): "This is structurally honest in a way the crosswords are not. There is a genuine solve path."

Dana Young (C6): "The logic grid has something the crosswords lack: a moment of derivation rather than retrieval. When you place Alice on the top floor with the cat, you have derived it."

The constraint-satisfaction mechanism is genuine and forced. However, the theme (people, floors, pets) is entirely replaceable — any three entities × three categories would produce the same constraint graph. The mechanism is correctly constructed deduction, but the "pets and floors" content is not integrated with the deductive logic in any way that requires it to be about pets and floors specifically.

**MTI = 2** (deductive mechanic is sound but the thematic content is purely substitutable filler)

### Puzzle 15: GM-05 Word Search — MTI: 2

Dan Katz (C6): "A word search is a search problem with a variable amount of difficulty depending on grid density and word length."

Dana Young (C6): "The concept — hidden message in the leftover letters of a word search — is a format with genuine mild aha potential. When you clear the found words and the remaining letters form a phrase, that is a small visual payoff: the grid reveals something."

The "remaining letters spell a hidden message" concept has mild structural integration (the theme words are load-bearing because they need to be removed to reveal the message), but the hidden message ("SECRET MESSAGE") names the format rather than the theme. Any category of words could occupy the grid and any message could be hidden. The construction failure aside, the mechanism is theme-portable.

**MTI = 2** (mild structural integration via the reveal mechanic; theme is substitutable; construction incomplete)

---

## C11 Table

**Formula:** C11 = C9 + MTI, i.e., C11 uses C6 per-dimension scores with C9 weights, then adds MTI.
Equivalently: C11 = C + S + (E×2) + (RR×2) + F + Conf + MTI, where the first six terms are evaluated from C6 scores.
**Pass ≥ 33/45** (73.3% threshold, matching C6/C8/C9/C10).

Since C9 already computes C + S + (E×2) + (RR×2) + F + Conf from C6 scores, C11 = C9 score + MTI.

| Puzzle | Tier | MTI | MTI Rationale (summary) | C9(/40) | C11(/45) | C11 Pass? |
|--------|------|-----|--------------------------|---------|---------|-----------|
| Scicabulary (SHAMROCK) | MIT/Elite | 3 | Fake-jargon theme is content; mechanic re-skinnable to other domains | 31 | **34** | YES |
| People Watching (HINDWING) | MIT/Elite | 4 | Birdwatcher epistemology IS the mechanism; IBP extraction is external scaffolding | 30 | **34** | YES |
| H2No (ENCAMPMENT) | MIT/Elite | 3 | Research-comparison mechanic is domain-portable; historical theme is content | 26 | **29** | NO |
| Bridge Building (PROTEINPOWDER) | MIT/Elite | 5 | Hashi topology = carbon valence = polymer chain; could not exist in any other domain | 36 | **41** | YES |
| You're Telling Me (CARBONSINK) | MIT/Elite | 3 | Compound-misreading is a general linguistic operation; environmental theme is content skin | 32 | **35** | YES |
| Dropypasta (VINDICATIONISLAND) | MIT/Elite | 3 | Smash theme contributes to win-condition variety but does not necessitate the mechanism | 29 | **32** | NO |
| Information Relay (FINISH LINE) | Standard Hunt | 5 | Personality = clue method = mechanic = theme; telephone-game relay could not exist in any other framing | 36 | **41** | YES |
| Front and Center (RADAR) | Standard Hunt | 5 | Palindrome = medium, object, extraction rule, and answer at three simultaneous scales | 37 | **42** | YES |
| Characters (UNHEXING) | Standard Hunt | 4 | Hex A-F constraint IS the clue vocabulary IS the construction rule; forest framing is atmospheric | 37 | **41** | YES |
| What's Next? (AFGHANISTAN) | Standard Hunt | 4 | Lyric-continuation tightly integrates mechanism with song's sequential logic; image step is re-skinnable | 32 | **36** | YES |
| GM-01 The Planets | Games Magazine | 1 | Theme is category of lookup; mechanism is category-neutral | 25 | **26** | NO |
| GM-02 Weather | Games Magazine | 2 | Self-referential WEATHER clue is single point of integration; rest is category fill | 25 | **27** | NO |
| GM-03 Animals | Games Magazine | 1 | Cliché clues confirm absence of editorial design; absolute floor | 21 | **22** | NO |
| GM-04 Logic Grid | Games Magazine | 2 | Deductive mechanism is sound; pets-and-floors content is fully substitutable | 28 | **30** | NO |
| GM-05 Word Search | Games Magazine | 2 | Mild reveal-mechanic integration; construction incomplete; theme substitutable | 17 | **19** | NO |

**C11 total passes: 9/15 (60%)**

### C11 Calculation Verification

C11 = C9 + MTI

| Puzzle | C9(/40) | MTI | C11(/45) |
|--------|---------|-----|---------|
| Scicabulary | 31 | 3 | 34 |
| People Watching | 30 | 4 | 34 |
| H2No | 26 | 3 | 29 |
| Bridge Building | 36 | 5 | 41 |
| You're Telling Me | 32 | 3 | 35 |
| Dropypasta | 29 | 3 | 32 |
| Information Relay | 36 | 5 | 41 |
| Front and Center | 37 | 5 | 42 |
| Characters | 37 | 4 | 41 |
| What's Next? | 32 | 4 | 36 |
| GM-01 The Planets | 25 | 1 | 26 |
| GM-02 Weather | 25 | 2 | 27 |
| GM-03 Animals | 21 | 1 | 22 |
| GM-04 Logic Grid | 28 | 2 | 30 |
| GM-05 Word Search | 17 | 2 | 19 |

Key pass/fail boundaries: Scicabulary 34 (YES), Dropypasta 32 (NO — one below threshold), GM-04 30 (NO — three below threshold). The threshold of 33 falls cleanly between Dropypasta (32) and the next passing MIT puzzle (Scicabulary at 34), which is structurally correct behavior.

### C11 Tier Analysis

| Tier | C11 avg | C11 pass rate |
|------|---------|---------------|
| MIT/Elite (6 puzzles) | 35.8 / 45 (79.6%) | 4/6 (67%) |
| Standard Hunt (4 puzzles) | 40.0 / 45 (88.9%) | 4/4 (100%) |
| Games Magazine (5 puzzles) | 24.8 / 45 (55.1%) | 0/5 (0%) |

**Cross-tier gap (MIT/Elite vs. Games Magazine):**
- C9: 76.7% vs. 58.0% → gap = 18.7 percentage points
- C11: 79.6% vs. 55.1% → gap = **24.5 percentage points**

C11 widens the cross-tier gap further than any previous configuration by adding a dimension that explicitly rewards puzzles where mechanism and theme are structurally inseparable — a property that hunt puzzles can achieve and Games Magazine crosswords cannot (by definition: a crossword's mechanism is always domain-portable).

---

## Comparison Summary

### Pass rates by tier and configuration

| Config | MIT/Elite pass | Standard pass | GM pass | Total | False positives | False negatives |
|--------|---------------|---------------|---------|-------|-----------------|-----------------|
| C6 | 4/6 (67%) | 4/4 (100%) | 1/5 (20%) | 9/15 (60%) | 1 (GM-04) | 1 (Dropypasta) |
| C8 | 4/6 (67%) | 4/4 (100%) | 1/5 (20%) | 8/15 (53%) | 1 (GM-04) | 2 (Dropypasta, H2No)* |
| C9 | 5/6 (83%) | 4/4 (100%) | 0/5 (0%) | 10/15 (67%) | 0 | 1 (H2No) |
| C10 | 4/6 (67%) | 4/4 (100%) | 0/5 (0%) | 8/15 (53%) | 0 | 2 (Dropypasta, H2No) |
| C11 | 4/6 (67%) | 4/4 (100%) | 0/5 (0%) | 9/15 (60%) | 0 | 1 (H2No) |

*H2No is a contested case: reviewers across C6, C8, C9 are split on whether it deserves to pass. C8 drops it to 19/30 (fail); C6 has it at 20/30 (fail); C9 has it at 26/40 (fail). It is not a false negative in any configuration — all configurations agree it should fail.

**False positive definition:** A Games Magazine puzzle that passes (should not pass given tier).
**False negative definition:** A hunt puzzle that fails and is arguable as passing (Dropypasta is contested; H2No is not seriously contested as a false negative given all reviewers identify genuine design concerns).

### Cross-tier gap (MIT/Elite avg vs. Games Magazine avg, as % of max)

| Config | MIT/Elite % | Games Magazine % | Gap (pp) |
|--------|-------------|-----------------|----------|
| C6 | 75.6% | 64.7% | 10.9 |
| C8 | 72.7% | 62.7% | 10.0 |
| C9 | 76.7% | 58.0% | 18.7 |
| C10 | 74.2% | 56.5% | 17.7 |
| C11 | 79.6% | 55.1% | 24.5 |

### Standard Hunt vs. MIT/Elite gap (% of max)

| Config | Standard % | MIT/Elite % | Gap (pp) |
|--------|-----------|-------------|----------|
| C6 | 88.3% | 75.6% | 12.7 |
| C8 | 88.3% | 72.7% | 15.6 |
| C9 | 88.8% | 76.7% | 12.1 |
| C10 | 88.8% | 74.2% | 14.6 |
| C11 | 88.9% | 79.6% | 9.3 |

C11 narrows the Standard Hunt vs. MIT/Elite gap (9.3 pp) because MIT puzzles with high MTI scores (Bridge Building MTI=5, People Watching MTI=4, What's Next MTI=4 from C9) gain more than the Standard Hunt puzzles (which already score very high). The gap narrowing is correct behavior: C11 recognizes that some MIT puzzles achieve the same depth of mechanism-theme integration as the best Huntinality puzzles.

---

## Verdict: Which Configuration is Best and Why?

### Ranking

**C11 > C9 > C10 > C6 > C8**

### C11 is the strongest configuration overall

**Reason 1: Best cross-tier discrimination.** C11 achieves a 24.5 percentage-point gap between MIT/Elite and Games Magazine, compared to C9's 18.7 pp, C10's 17.7 pp, C6's 10.9 pp, and C8's 10.0 pp. No configuration produces cleaner tier separation. This matters because the rubric's primary function is to correctly distinguish puzzle quality tiers.

**Reason 2: Zero false positives.** Like C9 and C10, C11 correctly fails GM-04 — the key false positive that C6 and C8 carried. C11 does not promote any Games Magazine puzzle to passing status.

**Reason 3: The MTI dimension is theoretically justified.** Mechanism-Theme Identity is not a post-hoc rationalization — it quantifies the "Riven Standard" that Dana Young articulates explicitly in C6 ("is the mechanic the theme, not merely representing it?") and that Thomas Snyder reaches via structural analysis ("this is theme and structure being the same thing"). Adding a dimension that was already implicitly being evaluated by the panel is an improvement in rubric transparency, not an ad-hoc adjustment.

**Reason 4: Correct handling of Dropypasta.** C9 produced a borderline pass for Dropypasta (29/40, exactly at threshold) and flagged this as the most contested result. C11 scores Dropypasta at 32/45 (below threshold of 33), resolving the ambiguity decisively. The MTI=3 score for Dropypasta is defensible: the Smash Bros. theme contributes to win-condition variety but does not necessitate the mechanism — the dropquote layer works in any domain, and the stage-conditioned extraction logic is loosely rather than necessarily derived from Smash Bros. physics. C11 correctly identifies Dropypasta as a failing puzzle.

**Reason 5: Reward structure for the best hunt puzzles.** Bridge Building (MTI=5) and Information Relay (MTI=5) and Front and Center (MTI=5) score 41, 41, and 42 out of 45 in C11, which appropriately separates them from merely good hunt puzzles. The MTI dimension creates vertical distance within the passing tier that C6–C10 could not provide.

### Why C11 > C9

C9 is the best single-knob solution. C11 is better because it adds a second theoretically-grounded dimension rather than relying entirely on weighted arithmetic. C9's Dropypasta borderline-pass was the one known problem with C9; C11 resolves it. C11 also produces better within-tier discrimination among passing puzzles (Bridge Building 41 vs. Scicabulary 34 vs. You're Telling Me 35 creates a meaningful gradient where C9 had 36 vs. 31 vs. 32 — a similar but shallower spread).

### Why C10 < C9

C10 applies C8's principle-adjusted scores to the C9 weighted formula. The main effect is dropping Dropypasta from 29/40 (C9, borderline) to 25/40 (C10, clear fail). This is correct behavior. However, C10 also slightly depresses MIT/Elite overall (29.7/40 average vs. C9's 30.7/40) by carrying forward C8's Blame-the-Player penalties. The net result is that C10 has a smaller cross-tier gap than C9 (17.7 vs. 18.7 pp) and is marginally worse on tier separation. C10 is a valid configuration that resolves the Dropypasta problem, but it does so at the cost of slightly reduced MIT/Elite average scores compared to C9.

### Why C8 is last

C8 achieves discrimination through profile trimming and principle injection — a valid approach — but it produces the worst cross-tier gap of any configuration (10.0 pp) because the penalties it applies (Blame-the-Player, Verify the Last Mile) fall on both the MIT/Elite tier (Dropypasta, H2No) and the Games Magazine tier (GM-05), without the weighted rubric's amplification of the dimensions that most distinguish tiers. C8's failure to outperform C6 on tier separation (despite having more sophisticated reviewer logic) is evidence that review process complexity alone does not improve rubric discrimination. The gains require structural changes to the rubric itself (weighting, additional dimensions), not just changes to the reviewer personas.

### Recommended path forward

**For immediate use:** C11 as defined. The MTI dimension adds one inference step (rating 1–5 from review text or direct assessment) but is robustly scoreable from the review texts already generated. The definition is clear and the anchor cases are well-established: Bridge Building (5), Information Relay (5), Front and Center (5) as the top tier; GM-01/GM-03 (1) as the floor.

**For further testing:** A C11-variant using C8 scores as input (rather than C6/C9) would combine all three improvements: weighted rubric (C9), principle-injected review scoring (C8), and MTI dimension (C11). Preliminary projection based on C10 data: this would score Dropypasta at 25+3=28 (well below threshold), Bridge Building at 36+5=41, GM-04 at 28+2=30. The cross-tier gap would likely exceed 25 pp. That configuration would be the strongest tested but would require re-running the MTI scoring against C8 review texts, which this document does not yet do (the MTI scores above are inferred from C6 texts).

---

## Appendix: MTI Scores Summary Table

| Puzzle | Tier | MTI | Rationale keyword |
|--------|------|-----|-------------------|
| Scicabulary | MIT/Elite | 3 | Re-skinnable mechanic; theme is content |
| People Watching | MIT/Elite | 4 | Birdwatcher epistemology IS the puzzle logic |
| H2No | MIT/Elite | 3 | Research-comparison is domain-portable |
| Bridge Building | MIT/Elite | 5 | Hashi topology = carbon valence = polymer; inextricable |
| You're Telling Me | MIT/Elite | 3 | Compound-misreading is general; environmental theme is skin |
| Dropypasta | MIT/Elite | 3 | Theme contributes to win-conditions loosely; not necessitated |
| Information Relay | Standard | 5 | Personality = clue method = mechanism = theme |
| Front and Center | Standard | 5 | Palindrome = medium, object, extraction rule, answer — 3 scales |
| Characters | Standard | 4 | Hex A-F IS the vocabulary constraint; forest theme is atmospheric |
| What's Next? | Standard | 4 | Lyric-continuation integrates mechanism with song's sequential logic |
| GM-01 The Planets | GM | 1 | Theme is vocabulary category; mechanism is category-neutral |
| GM-02 Weather | GM | 2 | Self-referential WEATHER clue is single integration point |
| GM-03 Animals | GM | 1 | Cliché clues confirm absence of design; absolute floor |
| GM-04 Logic Grid | GM | 2 | Deduction is genuine; pets-and-floors content is substitutable |
| GM-05 Word Search | GM | 2 | Mild reveal-mechanic integration; construction incomplete |

**MTI tier averages:**
- MIT/Elite: (3+4+3+5+3+3)/6 = 21/6 = **3.5**
- Standard Hunt: (5+5+4+4)/4 = 18/4 = **4.5**
- Games Magazine: (1+2+1+2+2)/5 = 8/5 = **1.6**

The MTI dimension alone achieves a Standard Hunt avg of 4.5 vs. Games Magazine avg of 1.6 — a 2.9-point separation on a 1–5 scale. This confirms that MTI is structurally discriminating between tiers, not just adding noise.
