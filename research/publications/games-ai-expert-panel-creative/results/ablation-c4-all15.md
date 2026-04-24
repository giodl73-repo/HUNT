# Ablation C4 — Design Philosophy Only
## All 15 Puzzles — First Encounter

**Condition:** C4 (Design Philosophy only — no biography, no credentials, no review lens, no principles)
**Reviewers:** Dan Katz, Thomas Snyder, Dana Young — Design Philosophy sections only
**Scoring rubric:** Clarity (1–5), Solvability (1–5), Elegance (1–5), Reading Reward (1–5), Fun (1–5), Confirmation (1–5) → /30, pass ≥ 22
**Date:** 2026-02-28

---

## Reviewer Philosophy Summary (C4 Source Material)

**Dan Katz:** Puzzles are contracts. Architecture is load-bearing — every structural decision promises something to the solver. The art is in the whole. Every element must justify its slot. Thematic coherence must be structural, not decorative. Mechanical variety is not decoration — it is the difference between a hunt and a marathon. Story must live in the puzzles themselves, not as set dressing around them.

**Thomas Snyder:** If a computer can generate your puzzle, it is not finished. The constructor's hand must be visible in every decision. Theme is not decoration — a real theme shapes the structure of the puzzle itself. Difficulty must be technique-based, not noise-based. Elegance is the proof the puzzle was finished. Every element must be load-bearing. The solve path should hold one direction and teach the skill it claims to require.

**Dana Young:** The mechanic is not chosen to represent the theme — it is the theme. Start with a world; ask what you would have to do to live there. The answer names the experience the solver just had. Extraction must earn its step. Visual grammar must be consistent and primary. An inevitable answer reframes everything before it; an arbitrary answer is a word the designer happened to land on.

---

## Batch 1 — MIT Mystery Hunt 2023

---

### Puzzle 1: Scicabulary
**Answer:** SHAMROCK
**Mechanism:** Portmanteau deconstruction — fake jargon terms formed from opposite halves of real portmanteaux; 24 terms matched to two-word phrase definitions; indexed letters spell the answer.

#### Philosophical Lens Application

**Katz lens (dominant for structure/contract):** The contract here is sophisticated: two nested layers of wordplay (portmanteau decomposition → definition matching → indexing), and the puzzle delivers on both. Every one of 24 entries justifies its slot — each term must be matched and indexed. The match-then-index structure creates a satisfying binding between the two steps rather than a loose sequential procedure. The 24-item scale is proportionate; no element is vestigial. Thematic coherence (the "fake jargon" conceit) shapes the content throughout. Architecture holds.

**Snyder lens (dominant for construction quality):** The constructor's hand is visible in the generation of each fake portmanteau — these are invented terms, not found objects. The entry point is designed: solvers must realize that jargon terms are built from opposite halves of real portmanteaux (the first aha). That realization is technique-based, not noise-dependent. The two-word phrase definitions are the second layer of precision. The matching step requires actual knowledge (recognizing portmanteaux and their components), not guessing. Load-bearing: every term participates. The solve path is disciplined — recognizing the portmanteau structure unlocks the matching, which unlocks the indexing.

**Young lens (dominant for answer/experience):** The "fake scientific jargon" world is coherent. The mechanic (constructing a vocabulary out of broken portmanteaux) is precisely what you would do in that world. The answer SHAMROCK is not arbitrary — but whether it names the experience of solving is weaker here. SHAMROCK does not obviously reframe the fake-jargon experience. The answer feels like a word the indexing landed on, not the inevitable name for what you just did. This is the weakest dimension under Young's lens.

#### Scores

| Dimension | Katz | Snyder | Young | Avg |
|-----------|------|--------|-------|-----|
| Clarity | 4 | 4 | 4 | **4.0** |
| Solvability | 4 | 4 | 4 | **4.0** |
| Elegance | 4 | 5 | 3 | **4.0** |
| Reading Reward | 4 | 4 | 4 | **4.0** |
| Fun | 4 | 4 | 4 | **4.0** |
| Confirmation | 4 | 4 | 3 | **3.7** |

**Total: 23.7 → PASS**

*Dominant lens: Katz (structural contract) and Snyder (construction craft) both score this highly. Young pulls Elegance and Confirmation down slightly — the answer does not fully land as the name of the experience. The match-then-index architecture is the puzzle's strongest feature under all three lenses.*

---

### Puzzle 2: People Watching
**Answer:** HINDWING
**Mechanism:** Bird identification via anthropomorphized vignettes — 14 social-media-style "tweets" describing people whose behavior encodes a bird species; IBP 4-letter codes extract the answer.

#### Philosophical Lens Application

**Katz lens:** The structural question is whether the vignettes earn their 14-entry count. Each entry is a distinct performance of the same mechanism (observe → identify bird → take IBP code), but the vignettes are varied enough in their encoding (vocal, visual, behavioral, linguistic) that the count feels proportionate. The "people watching" conceit (observing humans who act like birds) is coherent and the title earns its meaning. However, the IBP code extraction step is a late-added external constraint — the mechanism of the puzzle (identify the bird) is complete before the IBP step, and the extraction feels like an administrative layer imposed on top of genuine content. This is the structural weakness.

**Snyder lens:** The constructor's hand is visible in the design of each vignette — the bird calls rendered as human speech, the habitat clues, the visual field marks embedded in clothing and behavior. These are genuine constructions, not arbitrary embeddings. The entry point is accessible (the "people watching" frame invites close reading), and the solving technique (match behavioral/vocal/visual clues to species knowledge) is coherent and consistent. The IBP code step is technique-based (ornithological code system) but demands external knowledge that the puzzle cannot teach during the solve. This creates a difficulty spike that is not fully technique-based — solvers who don't know IBP codes face a hard stop.

**Young lens:** The mechanic is the world — you are watching humans in a crowded space who are actually birds, and you identify them by how they behave. This is strong world-building fidelity. The flavor text ("It can be fun to watch the people flocking to crowded spaces") is genuinely the mechanic, not decorative. The answer HINDWING, however, is a specialized anatomical term that does not name the experience of watching birds in human form. The answer is not inevitable from the world of the puzzle. Young would send this back — the extraction (IBP codes) is a process step, and HINDWING does not reframe the solve.

#### Scores

| Dimension | Katz | Snyder | Young | Avg |
|-----------|------|--------|-------|-----|
| Clarity | 4 | 3 | 4 | **3.7** |
| Solvability | 3 | 3 | 3 | **3.0** |
| Elegance | 3 | 3 | 3 | **3.0** |
| Reading Reward | 5 | 4 | 4 | **4.3** |
| Fun | 4 | 4 | 5 | **4.3** |
| Confirmation | 3 | 3 | 2 | **2.7** |

**Total: 21.0 → FAIL**

*Dominant lens: Young's lens is harshest — the answer does not land as the name of the experience, and it governs Confirmation heavily. Snyder flags the IBP code barrier as a noise-adjacent difficulty spike. Katz flags the IBP step as a structural layer imposed on top of a complete mechanism. The vignette-writing is the puzzle's genuine achievement; the extraction step is where it fails all three lenses.*

---

### Puzzle 3: H2No
**Answer:** ENCAMPMENT
**Mechanism:** Date-arithmetic location identification — 6 clues each describe an event offset from a different event at the same location; identify locations, find what they share, extract via "differences."

#### Philosophical Lens Application

**Katz lens:** The architectural elegance here is the flavor-text double meaning: "watch out for any differences" operates as both a warning and the extraction instruction. This is thematic coherence that is structural, not decorative — the flavor text is a puzzle piece, not set dressing. Six clues is a lean count, proportionate to the mechanism. The mechanism (date arithmetic + location identification + "differences" extraction) layers three steps, each necessary. No element is vestigial. The contract is honest: find the locations, find the commonality, use the differences.

**Snyder lens:** The constructor's hand is in the elegance of the "differences" double reading — it is a designed entry point into the extraction. Date arithmetic is technique-based difficulty: the solver must do real historical research and computation, not filter noise. The locations are constructed to share a property, and the "differences" in that property form the extraction. This is a puzzle where the solve path holds one direction. The clue precision required (finding specific historical events at specific locations) is high, which creates difficulty that is genuinely technique-based. The solve teaches the skill of layered historical constraint-matching.

**Young lens:** The world is "these six places share something in common, and the differences between them are what matters." The mechanic of finding locations through historical offsets is unusual and precise — you inhabit a world where you triangulate place through time. The answer ENCAMPMENT: does it name the experience? Encampment as a place (a type of location?) is loosely thematic with the location-finding mechanic, but the connection is not self-evident. The answer lands as a word that was generated by the extraction rather than a word that names what you were doing. Young would find this acceptable but not fully realized — the world is strong, the answer is acceptable but not inevitable.

#### Scores

| Dimension | Katz | Snyder | Young | Avg |
|-----------|------|--------|-------|-----|
| Clarity | 4 | 4 | 4 | **4.0** |
| Solvability | 4 | 4 | 3 | **3.7** |
| Elegance | 5 | 5 | 4 | **4.7** |
| Reading Reward | 4 | 4 | 3 | **3.7** |
| Fun | 4 | 4 | 4 | **4.0** |
| Confirmation | 4 | 4 | 3 | **3.7** |

**Total: 23.8 → PASS**

*Dominant lens: Katz and Snyder both rate this highly for the flavor-text structural elegance and technique-based difficulty. Young rates it lower on Confirmation and Reading Reward because ENCAMPMENT does not land as the inevitable name for the location-difference experience. The architecture is the puzzle's genuine strength.*

---

## Batch 2 — MIT Mystery Hunt 2023

---

### Puzzle 4: Bridge Building
**Answer:** PROTEINPOWDER
**Mechanism:** Hashiwokakero (bridge-building logic puzzle) with chemistry extraction — solve the island-bridge grid, then read the solved pattern as a molecular polymer backbone to identify the answer.

#### Philosophical Lens Application

**Katz lens:** The architectural move here is exceptional: the mechanic (Hashi logic puzzle) is not a vehicle for the theme — it is the theme. A bridge-building puzzle that becomes a polymer backbone is genuinely structural thematic coherence. The title "Bridge Building" carries double meaning at both levels (bridge puzzle genre AND chemical bonds). Every element is load-bearing — the ionic charge superscripts on nodes are structural, not decorative. The puzzle does the contract work of earning two separate insights that arrive in sequence. This is the kind of design Katz defines as a puzzle justifying its slot.

**Snyder lens:** This is a Snyder-ideal construction: a known puzzle type (Hashi) constrained to force a unique solution that also serves as content in a second domain (chemistry). The constructor's hand is visible in the chemical constraints embedded in the grid design — the ionic charges are not ornamental, they disambiguate atoms. The entry point is the title recognition (Hashi), and the second entry point is the polymer realization. The solve path holds one direction. Difficulty is entirely technique-based in both stages: Hashi deduction, then polymer identification. The puzzle teaches both skills by requiring them simultaneously.

**Young lens:** The world is molecular chemistry and the thing you do to live in it is build bridges between atoms. The mechanic is the world. The answer PROTEINPOWDER is the name of a product made from the type of polymer the solved grid encodes. This is not quite the answer naming the experience of solving — "protein powder" is a product category, not a description of bridge-building or polymer chemistry. The answer is functional but not a complete reframing of the experience. The "bridge building = chemical bonding" insight is the experience; PROTEINPOWDER names one result of that chemistry, not the act of doing it. Young would want the answer to land harder.

#### Scores

| Dimension | Katz | Snyder | Young | Avg |
|-----------|------|--------|-------|-----|
| Clarity | 4 | 4 | 4 | **4.0** |
| Solvability | 4 | 5 | 4 | **4.3** |
| Elegance | 5 | 5 | 4 | **4.7** |
| Reading Reward | 5 | 5 | 4 | **4.7** |
| Fun | 5 | 5 | 4 | **4.7** |
| Confirmation | 4 | 4 | 3 | **3.7** |

**Total: 26.1 → PASS**

*Dominant lens: Katz and Snyder are in strong agreement that the dual-domain architecture is the most coherent structural design in the corpus so far. Young pulls Confirmation down — PROTEINPOWDER names a product of the chemistry but not the act of bridge-building-as-bonding. Still the highest scoring puzzle in Batch 2 and the top score overall at this point.*

---

### Puzzle 5: You're Telling Me
**Answer:** CARBONSINK
**Mechanism:** "Shrimp Fried Rice" meme compound-word reinterpretation — 25 rhetorical questions misread compound words/phrases; identify the intended compound; extract letters to reach the environmental-themed answer.

#### Philosophical Lens Application

**Katz lens:** The mechanism is elegant in concept (misreading compound words is a single, learnable procedure) but the 25-entry count is the structural pressure point. Katz's concern here is proportionality: does the list earn its 25-entry scale, or is this a mechanism that burns itself out before the end? The first dozen entries establish and reinforce the mechanic; the remaining entries feel like execution against a template rather than discovery. The answer CARBONSINK has a thematic connection (environmental theme) but the puzzle's mechanism is linguistic wordplay, not environmental content — the theme is cosmetic rather than structural. "Thematic coherence that is decorative, not structural" is precisely Katz's failure mode.

**Snyder lens:** The compound-word misreading mechanic is well-designed as a single technique: identify what the clue-question is misreading, find the intended compound, extract. The entry point is accessible (the "Shrimp Fried Rice" frame is explained by the mechanism immediately). However, 25 entries means that the last third of the puzzle is technique already mastered — there is no additional deduction depth, only execution. The difficulty is evenly distributed rather than building, which means the solve teaches the technique in the first few entries and then runs it as a procedure. Snyder distinguishes "puzzle" from "procedure" — a long list that applies the same operation repeatedly trends toward procedure. CARBONSINK: "carbon" (coal) + "sink" (faucet) is a weak final step — the compound-word connection does not land as the culmination of the wordplay experience.

**Young lens:** The world of this puzzle is linguistic — a world where compound words have two valid parsings. The mechanic (inhabiting the wrong parsing and correcting it) is a complete activity. However, the answer CARBONSINK imports an environmental theme that has no basis in the compound-word world — it is labeled onto the puzzle rather than grown from it. The answer does not name the experience of misreading compound words; it names a concept from a domain that the puzzle does not inhabit. This is Young's clearest failure mode: the answer is not the last move in the logic. The extraction adds process (identifying environmental compounds) without adding meaning.

#### Scores

| Dimension | Katz | Snyder | Young | Avg |
|-----------|------|--------|-------|-----|
| Clarity | 4 | 4 | 4 | **4.0** |
| Solvability | 4 | 4 | 4 | **4.0** |
| Elegance | 3 | 3 | 2 | **2.7** |
| Reading Reward | 3 | 3 | 3 | **3.0** |
| Fun | 3 | 3 | 3 | **3.0** |
| Confirmation | 3 | 3 | 2 | **2.7** |

**Total: 19.4 → FAIL**

*Dominant lens: Young's lens is harshest — the answer imports an external theme that does not emerge from the puzzle's world. All three lenses agree that the scale (25 entries) trends toward procedure, and the environmental theme is decorative rather than structural. The compound-word misreading mechanism itself is sound but under-concluded.*

---

### Puzzle 6: Dropypasta
**Answer:** VINDICATIONISLAND
**Mechanism:** Dropquote puzzle (letters fill down from above into a quote) with character-matching extraction — fill in missing letters from Super Smash Bros. quotes, then apply stage-specific win conditions to determine which character contributes each extracted letter.

#### Philosophical Lens Application

**Katz lens:** The two-layer design is interesting architecturally: dropquote (a known fill-in puzzle type) feeds into a Smash Bros. character-matchup layer that uses stage-specific win conditions as extraction rules. The win conditions (alphabetical, release date, tier list rank, Kirby damage, proximity on character select screen, weight) are varied and require six different types of knowledge — this is genuine mechanical variety. However, the win conditions feel like a list of arbitrary rules rather than a unified system: the six stages each have a different criterion with no internal logic connecting them. A solver cannot deduce the criteria — they must be given. This is close to Katz's "mystery crate" concern: the rules are revealed rather than discoverable. The answer VINDICATIONISLAND is long (17 letters) and specific; it is not immediately evocative as a culminating idea.

**Snyder lens:** The dropquote fill step is clean and technique-based — solvers reconstruct quotes from their structure. The Smash Bros. layer demands specific game knowledge (tier lists, character weights, Kirby inhale damage) that is external and non-deducible. The six win conditions are heterogeneous — they are not six variations of the same skill, they are six unrelated facts about the game. This is difficulty-through-domain-knowledge rather than difficulty-through-technique: a solver who does not know Smash Bros. deeply is stopped not because they cannot reason but because they lack facts. The construction does not teach the skill; it tests possession of facts. Snyder would identify this as a solve path that branches by domain knowledge rather than holding one deductive direction.

**Young lens:** The world of Super Smash Bros. stages is inhabitable — the stage-specific win conditions do attempt to make each stage a distinct world with its own rules. The dropquote mechanic (filling in quotes from the game) immerses you in that world. The answer VINDICATIONISLAND is a specific island in the game — but it does not name the experience of applying six different stage rules. The answer names a location in the game; it does not reframe what you were doing. Young would also note that the six win conditions introduce inconsistency in the visual grammar: each stage operates by a different rule, which breaks the uniform reading of the extraction procedure.

#### Scores

| Dimension | Katz | Snyder | Young | Avg |
|-----------|------|--------|-------|-----|
| Clarity | 3 | 3 | 3 | **3.0** |
| Solvability | 3 | 3 | 3 | **3.0** |
| Elegance | 3 | 2 | 2 | **2.3** |
| Reading Reward | 4 | 3 | 3 | **3.3** |
| Fun | 3 | 3 | 4 | **3.3** |
| Confirmation | 3 | 2 | 2 | **2.3** |

**Total: 17.2 → FAIL**

*Dominant lens: Snyder is harshest — the six heterogeneous win conditions represent domain-knowledge difficulty, not technique-based difficulty, and the solve path fragments by game knowledge. Katz flags the mystery-crate quality of the extraction rules. Young finds the answer non-reframing. The dropquote fill step is sound; the extraction layer undermines it.*

---

## Batch 3 — Huntinality III + Teammate Hunt 2021

---

### Puzzle 7: Information Relay
**Answer:** FINISH LINE
**Mechanism:** Telephone game framing — five Bear family members each give clues for a target word in a broken way (homophones, sandwich clues, game references, technically-correct-but-useless, completely unrelated); identify target words; the extraction meta-step applies Grandma's method (homophones) to Grandpa's clues to reach the answer.

#### Philosophical Lens Application

**Katz lens:** The architecture here is genuinely exceptional: the puzzle has a meta-layer that is internal to itself. Identifying the six target words is Stage 1; identifying which Bear gave each set of clues is Stage 2; the extraction instruction (HOW NANA HEARS GRAMPS) tells you to apply Grandma's homophone method to Grandpa's mismatched clues — Stage 3. Each stage is necessary and none is vestigial. The "telephone game" conceit is structurally coherent: the unreliable narrators are the puzzle's mechanism, not its decoration. Every Bear's method is load-bearing in Stage 1 (distinguishing them is the skill), and Grandpa's method becomes load-bearing in Stage 3. This is Katz's ideal: thematic coherence that is structural, not decorative. The contract is fully honored.

**Snyder lens:** The constructor's hand is visible in the design of each Bear's clue method — these are five distinct techniques, each carefully calibrated so that solvers can identify them. The entry point is the Grandma method (the most recognizable — homophones), and from there the others become distinguishable by elimination and consistency. The solve path holds one direction: identify words → identify Bears → apply meta-instruction → reach final homophones. The difficulty is entirely technique-based: understanding how each Bear's method works and applying the correct method in the extraction. The puzzle teaches its own skill during the solve — by the time you reach the extraction, you have already practiced Grandma's method on live examples. This is Snyder's ideal: the deduction sequence is the lesson.

**Young lens:** The world of "Exquisite Froot" — a broken telephone game played by a family of bears who all hear differently — is vivid and specific. The mechanic (inhabiting each Bear's broken communication style) is what you do to live in that world. The answer FINISH LINE: "how Nana hears Gramps" — you apply the Grandma homophone method to Grandpa's mismatched clues to arrive at homophones that become FINISH LINE. The answer names the act of reaching the end of the relay — the finish line of the information relay. This is Young's ideal: the answer names the experience you just had. The world, the mechanic, and the answer are unified.

#### Scores

| Dimension | Katz | Snyder | Young | Avg |
|-----------|------|--------|-------|-----|
| Clarity | 4 | 4 | 4 | **4.0** |
| Solvability | 4 | 5 | 4 | **4.3** |
| Elegance | 5 | 5 | 5 | **5.0** |
| Reading Reward | 5 | 5 | 5 | **5.0** |
| Fun | 5 | 5 | 5 | **5.0** |
| Confirmation | 5 | 5 | 5 | **5.0** |

**Total: 28.3 → PASS**

*Dominant lens: All three lenses converge at the top of the scale. Katz identifies the internal meta-layer as ideal thematic-structural coherence. Snyder identifies the teaching-through-solving structure as ideal construction. Young identifies the answer FINISH LINE as the name of the experience of completing a broken-telephone relay. This is the highest-scoring puzzle in the corpus.*

---

### Puzzle 8: Front and Center
**Answer:** RADAR
**Mechanism:** Five newspaper clippings with blacked-out palindrome headlines; reconstruct each headline from the subtitle description and word-length enumeration; extract the center letter of each palindrome; the answer is itself a palindrome.

#### Philosophical Lens Application

**Katz lens:** The structure is elegant in its self-containment: five palindromes → five center letters → a fifth palindrome as the answer. The "SEMI TIMES" masthead is itself a palindrome, and the crease bisecting its T hints at "front and center." Every element is load-bearing — the five subtitles are necessary (they constrain reconstruction), the word-length enumerations are necessary (they constrain palindrome recovery), and the center-letter extraction is necessary (it is the mechanism). The conceit (a palindrome newspaper) is structurally coherent. No element is vestigial. The answer RADAR confirms the theme through its own form — this is a puzzle whose answer demonstrates its own mechanism. Katz would find this architectural.

**Snyder lens:** The entry point is the newspaper framing and the realization that SEMI TIMES is a palindrome — a designed entry point that unlocks the mechanic. The reconstruction technique (build a palindrome matching the subtitle description and word-length enumeration) is a genuine deductive challenge requiring both linguistic knowledge (knowing palindromes like "A Santa at Nasa") and constraint-matching. The solve path holds one direction: reconstruct → extract center → verify with confirmation palindrome. The difficulty is technique-based: palindrome construction under constraint. The puzzle teaches palindrome recovery — the solver who finishes this puzzle has deepened their palindrome fluency. The answer RADAR is itself a demonstration of mastery: the final item proved the technique correct.

**Young lens:** The world is a palindrome newspaper. The thing you do to live there is read backwards as easily as forwards. The mechanic (reconstructing blacked-out headlines that must be palindromes) is what a palindrome-world inhabitant does. The answer RADAR: it names the newspaper's detection instrument — a palindrome that scans in both directions. More than that, RADAR is the central instrument of a world where reading in both directions is fundamental. The answer is not just a palindrome — it is a word that describes the sensing apparatus of the world you just inhabited. Young would find this approaches the ideal: answer as the name of the experience's instrument.

#### Scores

| Dimension | Katz | Snyder | Young | Avg |
|-----------|------|--------|-------|-----|
| Clarity | 5 | 5 | 5 | **5.0** |
| Solvability | 4 | 4 | 4 | **4.0** |
| Elegance | 5 | 5 | 5 | **5.0** |
| Reading Reward | 4 | 5 | 5 | **4.7** |
| Fun | 5 | 5 | 5 | **5.0** |
| Confirmation | 5 | 5 | 5 | **5.0** |

**Total: 28.7 → PASS**

*Dominant lens: All three lenses converge again. The self-demonstrating answer (RADAR is a palindrome confirming a palindrome-extraction mechanism) satisfies Snyder's construction standard, Katz's architectural standard, and Young's answer-as-experience standard simultaneously. Highest Clarity score in the corpus — the mechanism is transparent and self-confirming. Tied with Information Relay for the highest score.*

---

### Puzzle 9: Characters
**Answer:** UNHEXING
**Mechanism:** Hex arithmetic + ASCII extraction — clues yield words whose letters are all A–F (valid hex); hex arithmetic produces ASCII values; decoded values spell the answer.

#### Philosophical Lens Application

**Katz lens:** The architecture is lean and clean: eight clues, one per ASCII character, the mechanism uniform throughout. The contract is: recognize the A–F constraint, compute hex arithmetic, decode ASCII. Each clue earns its slot by producing a distinct hex string. The flavor text ("find their base and then understand their code") carries genuine double meaning: "base" = hexadecimal base; "code" = ASCII code. This is structural flavor, not decorative. However, the eight-entry format is more of a procedure than a puzzle: once the hex/ASCII mechanism is understood, the remaining entries are execution. Katz would find this proportionate in scale but limited in architectural ambition — it is a well-formed single-layer puzzle, not a multi-stage structure.

**Snyder lens:** The "only A–F letters" constraint is the puzzle's central construction insight — the constructor's hand is visible in selecting only words composed of hex digits, which required genuine skill and search. The entry point is designed: the title "Characters" combined with "find their base" and "understand their code" is a three-part hint that converges on the realization. The hex arithmetic is technique-based: no guessing, just careful computation and ASCII lookup. The difficulty is clean — the challenge is recognizing the constraint, not filtering noise. However, the solve path produces no additional deductions after the mechanism is found; each entry is independent. The puzzle teaches one technique and applies it eight times. Snyder would find this finished but limited in structural depth.

**Young lens:** The world is one where characters are hex-encoded and you identify them by decoding arithmetic. The flavor text ("some characters in the forest — find their base and understand their code") is the world: a coded forest. The answer UNHEXING names the act of decoding the hex characters you just performed — "unhexing" is precisely the activity of the puzzle. This is Young's ideal in miniature: the answer is the verb for what you were doing. The world (coded forest) → the mechanic (hex arithmetic) → the answer (UNHEXING) forms a tight circuit. Young would score this highly on Confirmation because the answer is completely inevitable once you understand what you were doing.

#### Scores

| Dimension | Katz | Snyder | Young | Avg |
|-----------|------|--------|-------|-----|
| Clarity | 4 | 4 | 4 | **4.0** |
| Solvability | 4 | 4 | 5 | **4.3** |
| Elegance | 4 | 4 | 5 | **4.3** |
| Reading Reward | 3 | 4 | 4 | **3.7** |
| Fun | 4 | 4 | 4 | **4.0** |
| Confirmation | 4 | 4 | 5 | **4.3** |

**Total: 24.6 → PASS**

*Dominant lens: Young gives this the highest score of the three — UNHEXING as "the verb for what you were doing" is a near-perfect answer landing. Snyder scores it above average but flags the single-technique execution pattern. Katz finds it proportionate but not architecturally ambitious. Reading Reward is the weak dimension: the domain (hex arithmetic + ASCII) is narrow, limiting how much of the solving experience rewards prior knowledge.*

---

### Puzzle 10: What's Next?
**Answer:** AFGHANISTAN
**Mechanism:** "We Didn't Start the Fire" image identification + indexing — images depict events from the Billy Joel song; find the *next* item in the lyric sequence; index into it; extracted letters spell a phrase that is itself a lyric; answer is the word that follows it in the song.

#### Philosophical Lens Application

**Katz lens:** The double-aha structure is architecturally sound: first aha (recognize the song), second aha (realize the extracted phrase is itself a lyric and the answer is what follows). The second aha is the mechanism's most elegant feature — the extracted letters do not spell the answer directly; they spell a clue to the answer that uses the song's own logic. This is thematic coherence that is structural: the song's sequential logic is used twice, at different scales. The indexing step (find next item, confirmed by B = letter count) is clean and verification-built-in. The image-identification first step has inherent accessibility challenges — some images are more ambiguous than others — but this is a standard hunt difficulty modulation, not a structural problem.

**Snyder lens:** The entry point is the song recognition (the first forced deduction that unlocks the mechanism). The solve path holds one direction: identify event → find next lyric → verify letter count → index → collect letters → recognize extracted phrase → find next word. Each step is necessary and load-bearing. The technique-based difficulty is song knowledge (identifying events in "We Didn't Start the Fire") and the recognition that the extracted phrase is itself a lyric — a moment of genuine deduction that was not visible before. The index-confirmation built into each entry (B = letter count) is Snyder's ideal: each step can be independently verified before proceeding. The solver cannot proceed on wrong information without it surfacing.

**Young lens:** The world is the song "We Didn't Start the Fire" — a documentary sequence of historical events. The thing you do to live in that world is know what comes next. The answer AFGHANISTAN: "RUSSIANS IN AFGHANISTAN" is indeed the next line. But does AFGHANISTAN name the experience of playing "what comes next" in a sequence of historical events? The answer names a country that appears in the song, not the experience of the sequential game. Young would find the first two layers strong (inhabiting the song's sequential logic) and the answer acceptable — it is the word that follows the extracted phrase, so it is inevitable from the mechanism — but it does not reframe the experience. The answer is mechanically inevitable without being experientially resonant.

#### Scores

| Dimension | Katz | Snyder | Young | Avg |
|-----------|------|--------|-------|-----|
| Clarity | 4 | 4 | 4 | **4.0** |
| Solvability | 4 | 4 | 3 | **3.7** |
| Elegance | 4 | 5 | 4 | **4.3** |
| Reading Reward | 5 | 4 | 4 | **4.3** |
| Fun | 5 | 4 | 4 | **4.3** |
| Confirmation | 4 | 5 | 3 | **4.0** |

**Total: 24.6 → PASS**

*Dominant lens: Snyder scores this highest — the built-in confirmation mechanism (letter count B) and the two-level sequential logic represent strong construction craft. Katz scores it highly for the second-aha architectural elegance. Young pulls Solvability and Confirmation down slightly — the final answer is mechanically inevitable but not fully experientially resonant. The double-aha structure is the puzzle's defining achievement.*

---

## Games Magazine Tier — GM-01 through GM-05

*Note: These are simple standalone crosswords and canonical puzzle formats. The philosophical lenses are applied but the structural gaps are large and consistent across all five entries.*

---

### Puzzle 11: GM-01 — The Planets (Crossword)
**Answer:** Grid completion (PLUTO, NEPTUNE, JUPITER, MERCURY, SATURN, VENUS, EARTH, MARS)

#### Philosophical Lens Application

**Katz lens:** A crossword is a contract: fill in the grid using the clues and crossing constraints. The Planets crossword honors this contract with no deception — every clue is a direct definition, every answer is a planet or former planet name. The puzzle justifies its eight entries because crossword fill requires them as intersecting constraints. However, the mechanism has no second layer, no aha, no structural depth beyond the grid fill. Katz's standard for "does every puzzle justify its slot?" is met at the most basic level — this is a functioning crossword — but "would he want to solve this?" presses hard against a puzzle with no discovery, no misdirection, and no payoff beyond completion. The "no longer officially considered a planet" clue for PLUTO is the only moment of editorial wit; everything else is definition retrieval.

**Snyder lens:** A crossword grid is a constrained construction: the crossing letters require that every answer be compatible at every intersection. The constructor's decisions here are minimal — the grid topology and the clues are both simple. The entry point is "planet or former planet in the solar system," which is the entire puzzle. The difficulty is recall, not technique. There is no designed entry point in the Snyder sense — any clue can be attempted in any order, and the constraint propagation is trivial because the domain is small (8 planets + 1 dwarf planet). The puzzle was not "finished" in Snyder's sense because finishing it required no genuine construction choices — a grid-generation algorithm could have produced this.

**Young lens:** The world is the solar system. The mechanic (fill in the names of planets) is the most obvious thing you could do in that world — it has zero fidelity. The PLUTO clue ("no longer officially considered a planet") is the only moment where the puzzle does something with the world instead of just labeling it. The answer is not a single word that names the experience — there is no extraction, no reframing. The crossword completion IS the answer, and the experience of completing it has no culminating moment. Young would find this below the threshold of puzzle design: it is a vocabulary exercise with a grid constraint.

#### Scores

| Dimension | Katz | Snyder | Young | Avg |
|-----------|------|--------|-------|-----|
| Clarity | 5 | 5 | 5 | **5.0** |
| Solvability | 5 | 5 | 5 | **5.0** |
| Elegance | 1 | 1 | 1 | **1.0** |
| Reading Reward | 1 | 1 | 2 | **1.3** |
| Fun | 2 | 2 | 2 | **2.0** |
| Confirmation | 4 | 4 | 4 | **4.0** |

**Total: 18.3 → FAIL**

*Dominant lens: All three agree. Maximum Clarity and Solvability because the mechanism is transparent. Minimum Elegance because there is no construction craft beyond grid fill. The PLUTO clue elevates Reading Reward slightly above 1. Fun is low because completion is not discovery.*

---

### Puzzle 12: GM-02 — Weather (Crossword)
**Answer:** Grid completion (THUNDERSTORM, EVAPORATION, WEATHER, CONDENSATION, TORNADO, GROUNDWATER, HURRICANE, CLIMATE, HUMIDITY, METEOROLOGISTS, FLOOD)

#### Philosophical Lens Application

**Katz lens:** Slightly stronger than The Planets because the clues are full-sentence definitions rather than label fragments, and the self-referential WEATHER clue ("The state of the air at a certain time and place" when WEATHER is the puzzle's theme) introduces a mild moment of recursive coherence. The METEOROLOGISTS answer (longest in the grid at 15 letters) provides a satisfying constraint anchor. Still: no aha, no extraction, no second layer. The mechanism is the same as GM-01 and the structural depth is the same.

**Snyder lens:** Same assessment as GM-01 with a slight upward adjustment for clue sophistication. METEOROLOGISTS requires specific vocabulary and the 15-letter constraint creates genuine crossing difficulty. The self-referential WEATHER clue is a small instance of the constructor's hand — it required noticing that the puzzle's theme word could be both the subject and an answer, and exploiting that. Not architectural, but a sign of some construction attention.

**Young lens:** The WEATHER clue is the one moment of world-inhabiting — the word "weather" is what the puzzle is about and also what you do to the puzzle's world. This is a fragment of Young's ideal, in miniature: the theme word as the experience of the puzzle. However, it is a single clue in an 11-answer grid, not a structural principle. The world of meteorology is present in the vocabulary but not in the mechanic.

#### Scores

| Dimension | Katz | Snyder | Young | Avg |
|-----------|------|--------|-------|-----|
| Clarity | 5 | 5 | 5 | **5.0** |
| Solvability | 5 | 5 | 5 | **5.0** |
| Elegance | 1 | 2 | 2 | **1.7** |
| Reading Reward | 2 | 2 | 2 | **2.0** |
| Fun | 2 | 2 | 2 | **2.0** |
| Confirmation | 4 | 4 | 4 | **4.0** |

**Total: 19.7 → FAIL**

*Dominant lens: Marginally higher than GM-01 due to the self-referential WEATHER clue and the METEOROLOGISTS anchor. Still well below the pass threshold. The gap between this and any MIT Hunt puzzle illustrates the difference between vocabulary-exercise construction and mechanism-driven construction.*

---

### Puzzle 13: GM-03 — Animals (Crossword)
**Answer:** Grid completion (ZEBRA, GIRAFFE, KANGAROO, CAT, PIGS, CHEETAH, FROG, DOG, ELEPHANTS, LION, RHINO)

#### Philosophical Lens Application

**Katz lens:** "Man's best friend" for DOG. "Likes to chase mice" for CAT. "King of the jungle" for LION. These are crossword clichés — they are not clues, they are verbal labels stored in collective memory. A puzzle that depends on solver familiarity with clichés rather than genuine clue-to-answer reasoning is not honoring any contract except "you know these animals and these phrases." No element of the construction demonstrates a choice. Katz would describe this as a puzzle that exists because someone made it, not because the hunt needs it — except there is no hunt here, and the puzzle still fails the test.

**Snyder lens:** Zero constructor's hand visible. Every clue could have been generated by asking "what is commonly said about [animal]?" and selecting the first result. No design decision is present. "Hopping Australian marsupial" for KANGAROO is accurate but could have been written by anyone. This is the clearest possible example of Snyder's diagnostic failing: a computer could have generated this puzzle. The fact that it is not in a grid Snyder cares about is irrelevant — the absence of intentionality is the failure.

**Young lens:** The world is "animals." The mechanic is "name them." There is no inhabiting this world, no action you perform to live there — you recall names. The answer is not a word that reframes anything; completion of the grid is the end state. This is below puzzle design for Young: it is a vocabulary list with crossing constraints.

#### Scores

| Dimension | Katz | Snyder | Young | Avg |
|-----------|------|--------|-------|-----|
| Clarity | 5 | 5 | 5 | **5.0** |
| Solvability | 5 | 5 | 5 | **5.0** |
| Elegance | 1 | 1 | 1 | **1.0** |
| Reading Reward | 1 | 1 | 1 | **1.0** |
| Fun | 2 | 1 | 2 | **1.7** |
| Confirmation | 4 | 4 | 4 | **4.0** |

**Total: 17.7 → FAIL**

*Dominant lens: Unanimous — this is the floor. The cliché cluing ("Man's best friend," "King of the jungle") represents the absence of construction. The lowest Elegance and Reading Reward scores in the corpus, tied with Dropypasta.*

---

### Puzzle 14: GM-04 — Logic Grid (Simple)
**Answer:** Alice=Top/Cat, Carol=Middle/Dog, Bob=Ground/Fish

#### Philosophical Lens Application

**Katz lens:** A logic grid puzzle is a constraint-satisfaction problem. This one has five clues, three people, three floors, three pets, and a unique solution. It honors the contract: the clues are sufficient, non-redundant (each clue is necessary to reach uniqueness), and the solution is verifiable. No clue exists that could be removed without introducing multiple solutions — or if one is removable, it would indicate loose construction. The mechanism is clean. The puzzle does not pretend to be more than it is. Katz would find this appropriately scaled — it is a small puzzle with a small scope, honestly representing itself.

**Snyder lens:** The logic grid is Snyder's native territory. Five clues for a 3×6 solution space: the construction question is whether every clue is load-bearing (contributes to uniqueness). Clue 5 ("Alice and the person with the dog do not live on the same floor") is a negative constraint that, combined with Clue 2 (cat person on top) and Clue 3 (Bob directly below dog person), creates the chain that forces uniqueness. The entry point is Clue 2 (the most constrained — a direct placement), and the solve path flows from there. This is a correctly constructed minimal logic grid: the constraints are tight and technique-based. Snyder would find this finished.

**Young lens:** The world is "three people, three floors, three pets." The mechanic is logical elimination. There is no world to inhabit, no experience of the space — the "characters" are placeholders named Alice, Carol, and Bob with no personality. The answer is a grid assignment, not a word. There is no extraction, no culminating moment. Young's standard — does the answer name the experience you just had? — is undefined here because the answer is not a word. The puzzle is competent but has no world.

#### Scores

| Dimension | Katz | Snyder | Young | Avg |
|-----------|------|--------|-------|-----|
| Clarity | 5 | 5 | 5 | **5.0** |
| Solvability | 5 | 5 | 5 | **5.0** |
| Elegance | 2 | 3 | 1 | **2.0** |
| Reading Reward | 1 | 2 | 1 | **1.3** |
| Fun | 3 | 3 | 2 | **2.7** |
| Confirmation | 5 | 5 | 4 | **4.7** |

**Total: 20.7 → FAIL**

*Dominant lens: Snyder scores this highest of the three GM-tier puzzles because the logic grid is correctly constructed — every clue is load-bearing and the solve path flows. Katz finds it honest and proportionate. Young finds it worldless. Confirmation is high (grid completion is unambiguous). The strongest of the Games Magazine tier in construction craft, still failing on Elegance and Reading Reward.*

---

### Puzzle 15: GM-05 — Word Search with Hidden Message
**Answer:** SECRET MESSAGE (told to the solver in advance)

#### Philosophical Lens Application

**Katz lens:** The structural problem is the Confirmation dimension: the hidden message is told to the solver before they begin ("The remaining letters, read left to right, top to bottom, spell a two-word phrase"). A puzzle that announces its answer before you solve it has broken the contract. The "aha" is pre-emptied. The word search itself is a completion task, not a discovery task, and the hidden message layer cannot function as discovery when disclosed in advance. The puzzle treats the solver as someone who needs to be reassured rather than someone capable of discovery. Beyond this, the grid given in the file is broken — the "remaining letters" form a string (ABSFJGHINO ABIJKQD) that does not spell a readable phrase, indicating incomplete construction.

**Snyder lens:** A word search is a search procedure: find these words, then read what's left. The "remaining letters" mechanic is the only construction layer. But pre-announcing the answer ("spell a two-word phrase") means the constraint that was supposed to be deductive is given away. The grid is additionally broken — the remaining letters in the published version do not form SECRET MESSAGE, indicating the constructor did not verify the extraction. For Snyder, this is a puzzle that was not finished: the extraction was not checked. The most basic standard of construction (verify that the mechanism produces the intended answer) was not met.

**Young lens:** The world is a grid of letters hiding eight puzzle-related words. The mechanic of finding them and reading what's left could have been a discovery. But pre-announcing the phrase ("spell a two-word phrase") eliminates the discovery. The answer SECRET MESSAGE, told in advance, is the most anti-Young construction possible: it is the opposite of "the answer names the experience you just had" — here, the answer names an outcome that is declared before the experience begins. The word list (PUZZLE, HUNT, CLUE, SOLVE, GAME, WORD, FIND, PLAY) is thematically coherent with the puzzle's world, but that coherence is unrealized because the puzzle was not finished.

#### Scores

| Dimension | Katz | Snyder | Young | Avg |
|-----------|------|--------|-------|-----|
| Clarity | 4 | 3 | 3 | **3.3** |
| Solvability | 4 | 4 | 4 | **4.0** |
| Elegance | 1 | 1 | 1 | **1.0** |
| Reading Reward | 1 | 1 | 1 | **1.0** |
| Fun | 2 | 2 | 2 | **2.0** |
| Confirmation | 1 | 1 | 1 | **1.0** |

**Total: 12.3 → FAIL**

*Dominant lens: All three lenses give maximum penalty on Confirmation — the answer is pre-announced, eliminating the only element of discovery. Snyder additionally penalizes for the broken grid (extraction does not verify). This is the lowest-scoring puzzle in the corpus. Clarity is above minimum because the word-search mechanic is legible; the failure is entirely in the construction decision to announce the answer and in the unverified extraction.*

---

## Master Summary

### All 15 Puzzles — Scores and Pass/Fail

| # | Puzzle | Answer | Clarity | Solvability | Elegance | Reading Reward | Fun | Confirmation | Total | Result |
|---|--------|--------|---------|-------------|----------|----------------|-----|--------------|-------|--------|
| 1 | Scicabulary | SHAMROCK | 4.0 | 4.0 | 4.0 | 4.0 | 4.0 | 3.7 | **23.7** | PASS |
| 2 | People Watching | HINDWING | 3.7 | 3.0 | 3.0 | 4.3 | 4.3 | 2.7 | **21.0** | FAIL |
| 3 | H2No | ENCAMPMENT | 4.0 | 3.7 | 4.7 | 3.7 | 4.0 | 3.7 | **23.8** | PASS |
| 4 | Bridge Building | PROTEINPOWDER | 4.0 | 4.3 | 4.7 | 4.7 | 4.7 | 3.7 | **26.1** | PASS |
| 5 | You're Telling Me | CARBONSINK | 4.0 | 4.0 | 2.7 | 3.0 | 3.0 | 2.7 | **19.4** | FAIL |
| 6 | Dropypasta | VINDICATIONISLAND | 3.0 | 3.0 | 2.3 | 3.3 | 3.3 | 2.3 | **17.2** | FAIL |
| 7 | Information Relay | FINISH LINE | 4.0 | 4.3 | 5.0 | 5.0 | 5.0 | 5.0 | **28.3** | PASS |
| 8 | Front and Center | RADAR | 5.0 | 4.0 | 5.0 | 4.7 | 5.0 | 5.0 | **28.7** | PASS |
| 9 | Characters | UNHEXING | 4.0 | 4.3 | 4.3 | 3.7 | 4.0 | 4.3 | **24.6** | PASS |
| 10 | What's Next? | AFGHANISTAN | 4.0 | 3.7 | 4.3 | 4.3 | 4.3 | 4.0 | **24.6** | PASS |
| 11 | GM-01: The Planets | Grid fill | 5.0 | 5.0 | 1.0 | 1.3 | 2.0 | 4.0 | **18.3** | FAIL |
| 12 | GM-02: Weather | Grid fill | 5.0 | 5.0 | 1.7 | 2.0 | 2.0 | 4.0 | **19.7** | FAIL |
| 13 | GM-03: Animals | Grid fill | 5.0 | 5.0 | 1.0 | 1.0 | 1.7 | 4.0 | **17.7** | FAIL |
| 14 | GM-04: Logic Grid | Grid assignment | 5.0 | 5.0 | 2.0 | 1.3 | 2.7 | 4.7 | **20.7** | FAIL |
| 15 | GM-05: Word Search | SECRET MESSAGE | 3.3 | 4.0 | 1.0 | 1.0 | 2.0 | 1.0 | **12.3** | FAIL |

---

### Tier Averages

#### MIT Mystery Hunt Tier (Puzzles 1–6)
| Puzzle | Total |
|--------|-------|
| Scicabulary | 23.7 |
| People Watching | 21.0 |
| H2No | 23.8 |
| Bridge Building | 26.1 |
| You're Telling Me | 19.4 |
| Dropypasta | 17.2 |
| **Tier Average** | **21.9** |

*Pass rate: 3/6 (50%). The MIT tier shows high variance — the three failures (People Watching, You're Telling Me, Dropypasta) all fail primarily on Elegance and Confirmation, where the answer does not land as the inevitable culmination of the mechanism. The three passes (Scicabulary, H2No, Bridge Building) succeed because their structures are load-bearing throughout and the extraction is architecturally integrated.*

#### Huntinality III / Teammate Hunt Tier (Puzzles 7–10)
| Puzzle | Total |
|--------|-------|
| Information Relay | 28.3 |
| Front and Center | 28.7 |
| Characters | 24.6 |
| What's Next? | 24.6 |
| **Tier Average** | **26.6** |

*Pass rate: 4/4 (100%). All four puzzles from this tier pass, with two scoring in the top two positions in the entire corpus. The Huntinality III / Teammate Hunt tier demonstrates consistent construction quality: in all four puzzles, the answer is mechanically integrated with the mechanism (RADAR as palindrome confirmation, FINISH LINE as relay endpoint, UNHEXING as the verb for the operation, AFGHANISTAN as the next lyric). The answer-as-culmination principle is honored throughout this tier.*

#### Games Magazine Tier (Puzzles 11–15)
| Puzzle | Total |
|--------|-------|
| GM-01: The Planets | 18.3 |
| GM-02: Weather | 19.7 |
| GM-03: Animals | 17.7 |
| GM-04: Logic Grid | 20.7 |
| GM-05: Word Search | 12.3 |
| **Tier Average** | **17.7** |

*Pass rate: 0/5 (0%). All five GM-tier puzzles fail. The failure mode is consistent: maximum scores on Clarity and Solvability (these puzzles are easy to understand and solve) but minimum scores on Elegance, Reading Reward, and — most critically — Fun and Confirmation as discovery experiences. GM-04 (Logic Grid) scores highest in the tier because it is correctly constructed (load-bearing constraints, clean solve path) even though it has no extraction, no world, and no answer-as-experience. GM-05 (Word Search) scores lowest because it pre-announces its answer, eliminating the only possible discovery moment.*

---

### Dimension Averages Across All 15 Puzzles

| Dimension | MIT (6) | Hunt III/TM (4) | Games Mag (5) | Overall (15) |
|-----------|---------|-----------------|---------------|--------------|
| Clarity | 4.1 | 4.3 | 4.7 | **4.3** |
| Solvability | 3.7 | 4.1 | 4.8 | **4.1** |
| Elegance | 3.6 | 4.7 | 1.3 | **3.1** |
| Reading Reward | 3.8 | 4.4 | 1.3 | **3.1** |
| Fun | 3.9 | 4.6 | 2.0 | **3.4** |
| Confirmation | 3.0 | 4.6 | 3.5 | **3.6** |
| **Total** | **22.1** | **26.6** | **14.0** | **21.5** |

---

### Key Findings — C4 Condition

**1. Elegance is the tier-separating dimension.** The gap between Games Magazine (avg 1.3) and Huntinality III (avg 4.7) is largest on Elegance. Under the design-philosophy lenses, Elegance measures whether the mechanism is architecturally integrated — not whether the puzzle is sophisticated, but whether every element earns its presence. GM-tier puzzles score near minimum on Elegance because they have no construction choices; hunt puzzles score high when every element is load-bearing.

**2. Confirmation is the answer-quality dimension.** Puzzles where the answer names the experience (FINISH LINE, RADAR, UNHEXING) score 4.7–5.0 on Confirmation. Puzzles where the answer is a word the indexing landed on (HINDWING, VINDICATIONISLAND, CARBONSINK) score 2.3–2.7. The Young lens drives this dimension most strongly — her "inevitable vs. arbitrary answer" standard maps directly onto Confirmation.

**3. Clarity and Solvability invert across tiers.** GM-tier puzzles score highest on Clarity and Solvability (5.0) and lowest on Elegance/Reading Reward. Hunt puzzles score lower on Clarity (the mechanism is not transparent at entry) but higher on everything else. This is the correct signature: a well-constructed hunt puzzle should be harder to enter but more rewarding to solve. The C4 condition (design philosophy only) captures this correctly because the three philosophies all value construction depth over accessibility.

**4. The three lenses converge at extremes.** On the two highest-scoring puzzles (Front and Center, Information Relay) all three reviewers give near-identical high scores. On the lowest-scoring puzzle (GM-05: Word Search) all three give near-identical low scores. The lenses disagree most on puzzles where mechanism and answer are partially aligned — e.g., What's Next? (Snyder high/Young low) or Bridge Building (Katz/Snyder high/Young medium). This suggests the lenses are complementary: Katz catches architectural failures, Snyder catches construction failures, Young catches answer-quality failures.

**5. C4 pass rate: 7/15 (47%).** The full corpus splits cleanly: all Huntinality III / Teammate Hunt puzzles pass, the MIT Hunt tier splits 50/50, and all GM-tier puzzles fail. The threshold at 22/30 correctly distinguishes puzzles with genuine construction craft (the three passes in the MIT tier have scores of 23.7, 23.8, and 26.1) from puzzles that function without craft (all GM-tier puzzles fall between 12.3 and 20.7).
