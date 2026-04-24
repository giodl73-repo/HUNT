# Ablation Study — Condition C1 (Rubric Only)
**Reviewer context:** Expert reviewer of puzzle design
**Evaluation framework:** 6-dimension rubric only (no profiles, no design principles)
**Date:** 2026-02-28
**Puzzles evaluated:** 15 (3 MIT/Elite + 3 MIT/Elite + 4 Standard Hunt + 5 Games Magazine)

---

## Rubric Reference

| Dimension | Scale | What it measures |
|-----------|-------|-----------------|
| Clarity | 1–5 | Solver understands what to do without help |
| Solvability | 1–5 | Appropriate knowledge reaches correct answer |
| Elegance | 1–5 | Mechanism feels clean, not arbitrary |
| Reading Reward | 1–5 | Domain/puzzle knowledge is load-bearing (not optional) |
| Fun | 1–5 | Enjoyable, creates aha moment |
| Confirmation | 1–5 | Solver knows when they're right |
| **Total** | **/30** | **Pass ≥ 22** |

---

## PUZZLE 1 — Scicabulary
**Source:** MIT Mystery Hunt 2023 | **Tier:** MIT/Elite | **Answer:** SHAMROCK

**Mechanism:** 24 fake jargon terms are each formed from the "opposite halves" of two real portmanteau words. 24 definitions clue two-word phrases. Match terms to phrases, then index the given number into the matched phrase to extract one letter per pair.

| Dimension | Score | Justification |
|-----------|-------|---------------|
| Clarity | 2 | The instructions state the mechanic plainly, but "opposite halves of a portmanteau" is a non-trivial concept that most solvers will need to work several examples before grasping. The hint structure confirms this is a known friction point. |
| Solvability | 4 | Once the portmanteau logic clicks, most steps are tractable: portmanteaus are identifiable, the definitions are humorously fair, indexing is straightforward. The sheer volume (24 pairs) is the main obstacle. |
| Elegance | 4 | The core idea — reverse a portmanteau by swapping its halves — is genuinely novel and internally consistent. The mechanic is clean once understood; no arbitrary exceptions. |
| Reading Reward | 4 | Portmanteau literacy is load-bearing. A solver who knows "brunch," "smog," "infomercial" etc. and can decompose them is substantially faster. Domain knowledge actively rewards rather than merely enables. |
| Fun | 4 | The absurdist definitions (snake that reads your mind, society dominated by Nutella-loving women) generate genuine delight. The aha of recognizing a portmanteau half is repeatable across 24 entries. |
| Confirmation | 3 | SHAMROCK is a real word and checks out as a coherent answer, but the extraction is long enough that a single indexing error could produce a near-miss. No in-puzzle confirming structure beyond the answer itself. |
| **Total** | **21/30** | **FAIL (1 below threshold)** |

**Key finding:** Exceptional elegance undermined by a steep initial clarity hurdle — the "opposite halves" concept demands a worked example before it clicks, leaving solvers stranded at step zero.

---

## PUZZLE 2 — People Watching
**Source:** MIT Mystery Hunt 2023 | **Tier:** MIT/Elite | **Answer:** HINDWING

**Mechanism:** 14 social-media vignettes each anthropomorphize a bird species (via calls, field marks, habitat). Identify each bird, look up IBP 4-letter alpha codes, extract letters to spell HINDWING.

| Dimension | Score | Justification |
|-----------|-------|---------------|
| Clarity | 3 | The flavor text ("flocking," "recall what they said") nudges toward birds but doesn't name the species-identification step. IBP codes are not signposted in the puzzle text — solvers must arrive at that step independently. The overall journey is inferrable but not spelled out. |
| Solvability | 3 | Bird call mnemonics ("The pot! The pot!") and field marks are accurate for experts but opaque to non-birders. The IBP code lookup is findable. The combination of species identification and code retrieval creates two independent knowledge barriers. |
| Elegance | 4 | The anthropomorphized vignettes are an elegant thematic conceit — "people watching" at a café that is also "bird watching." The extraction through professional alpha codes is satisfying once the layer is found. |
| Reading Reward | 5 | This puzzle most heavily rewards domain knowledge of any in the corpus. A birder who knows call mnemonics solves entries in seconds; a non-birder needs research for every entry. Domain expertise is maximally load-bearing. |
| Fun | 4 | The vignettes are charming and witty. The aha of "these people ARE birds" is a strong double-take moment. The reading reward for birders is unusually high. |
| Confirmation | 3 | HINDWING is a real compound noun (part of an insect wing) and feels thematically appropriate for a nature-themed puzzle, which provides mild confirmation. But without an in-puzzle check, a mis-identified bird yields a non-confirming answer. |
| **Total** | **22/30** | **PASS (exactly at threshold)** |

**Key finding:** The birding domain knowledge requirement is the highest of any puzzle evaluated — a genuine specialist puzzle that rewards expertise maximally but risks complete opacity for generalists.

---

## PUZZLE 3 — H2No
**Source:** MIT Mystery Hunt 2023 | **Tier:** MIT/Elite (filed as Standard Hunt) | **Answer:** ENCAMPMENT

**Mechanism:** Six clues describe a historical event at a location using a date offset from another event at the same location. Identify all six locations, find their common property, extract letters from the "differences" to spell ENCAMPMENT.

| Dimension | Score | Justification |
|-----------|-------|---------------|
| Clarity | 2 | The flavor text ("watch out for any differences") is doing too much work as the extraction hint. The path from "identify six locations" to "extract letters from differences" is under-signposted. The enumerations help identify answer lengths but the extraction logic requires a conceptual leap. |
| Solvability | 3 | The historical events are obscure enough that several will require research (a man digging his own subway, a wooden bridge that inspired a song). The "common property" is resolvable with detective work. The final extraction — what exactly are the "differences"? — remains the puzzle's soft underbelly. |
| Elegance | 3 | The date-arithmetic framing is clean in concept. The flavor text "H2No" (water + no = waterless? or the title contrasting with the extraction) is clever. But the extraction mechanism is opaque enough that "elegant" feels overstated. |
| Reading Reward | 3 | Historical knowledge is useful but the puzzle is resolvable through research. Domain expertise in history accelerates but is not strictly required. |
| Fun | 3 | The historical detective work has genuine appeal, and the moment of recognizing all six locations share a property is satisfying. The extraction step is less fun because it's unclear. |
| Confirmation | 2 | ENCAMPMENT is a plausible English word but provides limited self-confirmation. The "differences" extraction is ambiguous enough that solvers may not trust a correct answer. |
| **Total** | **16/30** | **FAIL** |

**Key finding:** The extraction mechanism — what specifically constitutes the "differences" flagged by flavor text — is insufficiently telegraphed, making the final step feel arbitrary rather than earned.

---

## PUZZLE 4 — Bridge Building
**Source:** MIT Mystery Hunt 2023 | **Tier:** MIT/Elite | **Answer:** PROTEINPOWDER

**Mechanism:** Solve a Hashiwokakero (bridge-building) logic puzzle. The solved grid, with ionic charges on nodes, is then read as a polymer molecular backbone to identify a specific polymer yielding the answer.

| Dimension | Score | Justification |
|-----------|-------|---------------|
| Clarity | 2 | The puzzle title clues the Hashi genre clearly, but the chemistry layer is fully hidden until the logic puzzle is complete. The transition from "solved bridge grid" to "this is a polymer backbone" requires a second, independent conceptual leap with no mid-puzzle signposting. |
| Solvability | 3 | Hashi is a well-defined genre; a competent solver can finish the logic step. The chemistry extraction, however, requires recognizing ionic charges as atom identifiers and connecting them to amino acid or polymer chemistry — a genuine dual-discipline demand. |
| Elegance | 4 | The thematic layering — bridge-building as chemistry bonds, islands as atoms — is conceptually beautiful. The title works at both levels. When the connection clicks, it is deeply satisfying. |
| Reading Reward | 4 | Both Hashi expertise and polymer chemistry knowledge are load-bearing. A chemistry-literate solver sees the molecular structure immediately after completing the logic puzzle; without that knowledge, the extraction is inaccessible. |
| Fun | 4 | Two distinct aha moments: recognizing the genre and completing the logic grid, then recognizing the molecular structure. Double-aha puzzles are intrinsically more fun. |
| Confirmation | 3 | PROTEINPOWDER is a confirming compound noun that fits a chemistry/fitness context, but arriving at it via molecular backbone identification requires confident chemistry reading — the confirmation feels earned only if you trust the extraction. |
| **Total** | **20/30** | **FAIL (2 below threshold)** |

**Key finding:** Conceptually the most ambitious puzzle in the set — two complete disciplines layered sequentially — but the chemistry extraction step is inaccessible without specialist knowledge, making this a team puzzle rather than an individual solve.

---

## PUZZLE 5 — You're Telling Me
**Source:** MIT Mystery Hunt 2023 | **Tier:** MIT/Elite | **Answer:** CARBONSINK

**Mechanism:** 25+ rhetorical questions each misparse a compound word/phrase (Shrimp Fried Rice format). Identify the intended compound in each case, extract letters to reach the answer.

| Dimension | Score | Justification |
|-----------|-------|---------------|
| Clarity | 4 | The meme format is widely known and the rhetorical-question framing makes each clue immediately comprehensible. Solvers can engage with entry 1 without understanding the full puzzle structure. High accessibility at the clue level. |
| Solvability | 4 | The compounds are varied in difficulty but none requires specialist knowledge. The "shrimp fried rice" parsing is a documented internet meme; most solvers who have encountered it will solve entries quickly. Even without meme familiarity, the rhetorical questions are logically transparent. |
| Elegance | 4 | The meme format applied consistently to 25+ entries is elegant in its uniformity. The environmental theme of the answer (CARBONSINK = carbon + sink) is a satisfying thematic cap. |
| Reading Reward | 3 | Knowledge of the meme accelerates solving significantly but is not strictly required — the rhetorical logic works even without meme context. Mildly load-bearing rather than maximally so. |
| Fun | 4 | Individually funny entries, escalating variety, and the collective pleasure of a long satisfying list. The "you're telling me" format generates humor reliably. |
| Confirmation | 3 | CARBONSINK (carbon dioxide sink, also a kitchen sink pun) is thematically apt, but the extraction mechanism from 25+ entries doesn't have an in-puzzle confirming structure. Length and thematic fit provide informal confirmation. |
| **Total** | **22/30** | **PASS (exactly at threshold)** |

**Key finding:** The most accessible MIT-tier puzzle in the corpus — high clarity, low knowledge barrier, reliable fun — but relies on volume (25+ entries) rather than depth, and lacks a self-confirming extraction mechanism.

---

## PUZZLE 6 — Dropypasta
**Source:** MIT Mystery Hunt 2023 | **Tier:** Standard Hunt | **Answer:** VINDICATIONISLAND

**Mechanism:** Fill in dropquote rows (all from Super Smash Bros.), collecting one missing letter per row. Then match two characters per named Smash stage; the stage's win condition determines which character's letter advances.

| Dimension | Score | Justification |
|-----------|-------|---------------|
| Clarity | 3 | Dropquotes are a known format. The stage-win-condition rules (e.g., "earlier original video game release," "higher tier list position") are clearly defined per stage. However, the chain — fill quotes, identify missing letters, run character match, apply stage condition — has multiple steps that are stated rather than intuited. |
| Solvability | 3 | Super Smash Bros. quote identification requires franchise familiarity. The stage win conditions are quirky (Kirby inhale damage, character weight) and require game-specific research. A non-Smash solver will be slowed significantly at every step. |
| Elegance | 3 | The dropquote-to-character-match chain is inventive but the connection between the two steps feels somewhat arbitrary — why do these specific stage conditions govern the letters? The justification is thematic rather than necessary. |
| Reading Reward | 4 | Smash Bros. franchise knowledge is heavily load-bearing: recognizing quotes, knowing character weights, tier positions, and inhale damage values are all domain-specific. Experts gain real advantage. |
| Fun | 3 | Fun for Smash players; opaque for others. The stage win conditions are delightfully arcane for the audience they target. VINDICATIONISLAND as an answer is absurdly long and funny upon reveal. |
| Confirmation | 3 | VINDICATIONISLAND is unusual enough to feel either definitely right or definitely wrong. The length itself serves as informal confirmation. |
| **Total** | **19/30** | **FAIL** |

**Key finding:** A tight thematic package for Smash Bros. fans that fails on solvability for general audiences — the franchise knowledge requirement exceeds what "Standard Hunt" tier typically demands.

---

## PUZZLE 7 — Information Relay
**Source:** Huntinality III | **Tier:** MIT/Elite (filed as Standard Hunt) | **Answer:** FINISH LINE

**Mechanism:** Five Bear family members each give clues in a broken style (homophones, sandwiches, video game refs, unhelpfully literal, unrelated). Identify 6 target words, determine which bear gave which clue, extract via Grandpa's clue positions, then apply Grandma's method (homophone) to Grandpa's clues to reach FINISH LINE.

| Dimension | Score | Justification |
|-----------|-------|---------------|
| Clarity | 2 | The setup is elaborate: five bear types, six clue styles, two extraction steps. The initial framing (Exquisite Froot) is charming but masks the functional complexity. The instruction to apply "one bear's method to another bear's clues" is a meta-step that requires fully understanding the first layer before the second becomes visible. |
| Solvability | 3 | Individual target words are reasonably gettable. The bear identification step is deductive and fair. The final extraction — Grandpa's clues as Grandma's homophones — is a satisfying click once the intermediate cluephrase is read, but requires both layers to work. |
| Elegance | 5 | Structurally the most elegant puzzle in the corpus. The telephone-game framing, the five distinct but internally consistent bear styles, and the second-order extraction (apply one bear's broken method to another's clues) form a genuinely beautiful self-referential system. |
| Reading Reward | 3 | Video game references and bear-style identification reward puzzle-hunt familiarity more than domain expertise. The value is in structural reasoning rather than specialist knowledge. |
| Fun | 4 | The bear voices are delightful. "HOW NANA HEARS GRAMPS" as the intermediate cluephrase is an aha moment of the highest order — it explains itself through its own content. |
| Confirmation | 4 | The two-step extraction terminates with FINISH LINE, a phrase that is both phonetically clean and thematically complete. The homophony of FINNISH/FINISH is a strong confirming signal. |
| **Total** | **21/30** | **FAIL (1 below threshold)** |

**Key finding:** The most elegantly constructed puzzle in the corpus by a significant margin — the meta-recursive extraction is a masterclass — but the two-layer mechanism imposes a clarity cost that keeps it one point below threshold.

---

## PUZZLE 8 — Front and Center
**Source:** Huntinality III | **Tier:** Standard Hunt | **Answer:** RADAR

**Mechanism:** Five newspaper clippings from "SEMI TIMES" have blacked-out palindrome headlines (word-length enumerations given, subtitles describe the story). Reconstruct each palindrome headline, extract the center letter; five center letters spell RADAR (itself a palindrome).

| Dimension | Score | Justification |
|-----------|-------|---------------|
| Clarity | 4 | The newspaper format, enumerated word lengths, and descriptive subtitles give solvers clear handholds. "SEMI TIMES" as a palindrome is visible immediately. "Front and Center" as a title hints at the extraction step without giving it away. Clarity is high once the palindrome nature of headlines is recognized. |
| Solvability | 4 | Reconstructing palindromes from word lengths and subtitles is tractable with wordplay skill. The subtitles are precise enough to suggest specific content ("Cuban export" = cigar, "fruit-mammal hybrid" = banana bat). The palindrome search space is constrained by both enumeration and content. |
| Elegance | 5 | Near-perfect elegance: palindrome newspaper from a palindrome source (SEMI TIMES), palindrome headlines, center letter extraction, palindrome answer (RADAR). Every element reinforces the theme recursively. The "SEMI TIMES" crease bisecting the T is a physical confirmation of the theme. |
| Reading Reward | 3 | Wordplay skill and palindrome familiarity are load-bearing. No specialist domain knowledge required, but the puzzle rewards puzzle-constructors and word-game enthusiasts. |
| Fun | 5 | Individual palindromes are delightful ("MR OWL ATE MR METAL WORM," "WAS IT A BANANA BAT I SAW?"). The escalating absurdity of the subtitles is genuinely funny. The aha of RADAR as the palindrome answer is a perfect cap. |
| Confirmation | 5 | RADAR is itself a palindrome — the strongest possible self-confirming answer for a palindrome puzzle. The theme is complete, recursive, and undeniable. |
| **Total** | **26/30** | **PASS** |

**Key finding:** The strongest puzzle in the corpus by total score. Elegance and confirmation are both maximal; the recursive palindrome theme achieves complete self-referential closure.

---

## PUZZLE 9 — Characters
**Source:** Huntinality III | **Tier:** Standard Hunt | **Answer:** UNHEXING

**Mechanism:** Eight clues each yield a word composed only of letters A–F (valid hex). Perform arithmetic on the hex values, decode results as ASCII. The eight ASCII values spell UNHEXING.

| Dimension | Score | Justification |
|-----------|-------|---------------|
| Clarity | 4 | The flavor text ("find their base," "understand their code") provides pointed hints. The title "Characters" refers to both storybook characters and ASCII characters. The mechanism is stated obliquely but the hints converge. Once the A–F constraint is noticed, the path is clear. |
| Solvability | 4 | The clues are fair wordplay (BABA from "Baba Is You," CAFE as "small restaurant"). The hex arithmetic is straightforward with a calculator. ASCII decoding is a standard puzzle-hunt skill. No step requires unusual knowledge. |
| Elegance | 5 | A beautiful three-layer mechanism (wordplay → hex → ASCII) where every constraint is necessary. The fact that the clue answers happen to use only A–F is the core insight, and it follows naturally from the English words involved. UNHEXING as the answer is self-describing — the puzzle is literally un-hexing hex values. |
| Reading Reward | 4 | Knowledge of hex and ASCII is load-bearing but well within the standard puzzle-hunt vocabulary. The "Baba Is You" reference rewards gaming knowledge. Domain fluency genuinely accelerates. |
| Fun | 5 | The aha cascade is exceptional: noticing A–F, recognizing hex, performing arithmetic, getting ASCII values, watching UNHEXING emerge. Each layer is its own small reward. The answer's self-description is the final delight. |
| Confirmation | 5 | UNHEXING is self-describing (the puzzle literally un-hexes hex-encoded ASCII). The answer confirms the mechanism, creating complete thematic closure. |
| **Total** | **27/30** | **PASS** |

**Key finding:** The highest-scoring puzzle in the corpus. The self-describing answer (UNHEXING = what the puzzle does) achieves the rarest form of puzzle confirmation — the answer explains itself.

---

## PUZZLE 10 — What's Next?
**Source:** Huntinality III | **Tier:** Standard Hunt | **Answer:** AFGHANISTAN

**Mechanism:** Ten images depict events/people from Billy Joel's "We Didn't Start the Fire." Find the *next* item in the song's sequence, use the provided A/B numbers to index into it. Extracted letters spell "RUSSIANS IN" — itself a lyric — and the answer is the next word: AFGHANISTAN.

| Dimension | Score | Justification |
|-----------|-------|---------------|
| Clarity | 4 | The title "What's Next?" is the mechanism stated as a question. The A/B number format (index/length) provides implicit confirmation for each step. Clear entry point; the song-identification step is the only implicit requirement. |
| Solvability | 3 | Identifying all ten events from images requires "We Didn't Start the Fire" familiarity or research. The song is well-known and the lyrics are indexed extensively online, but identifying specific images as specific song references can be ambiguous without the actual image content (which is not fully reproduced here). |
| Elegance | 4 | The "what comes next" framing applies recursively: each step asks for what comes next in the song, and the final step asks for what comes next after the extracted phrase. Double recursion on the same concept. |
| Reading Reward | 4 | "We Didn't Start the Fire" lyric knowledge is directly load-bearing. A solver who has the song memorized moves significantly faster. The second aha (recognizing "RUSSIANS IN" as a lyric) requires song familiarity. |
| Fun | 4 | Two aha moments: the song identification and the realization that the extracted phrase is itself a lyric requiring one more "next" step. The answer AFGHANISTAN feels satisfying given the song's Cold War context. |
| Confirmation | 3 | AFGHANISTAN follows "RUSSIANS IN" in the song — confirming requires knowing the song. A solver who has verified the extraction trusts the answer; without song knowledge, confirmation requires a lookup. |
| **Total** | **22/30** | **PASS (exactly at threshold)** |

**Key finding:** Elegant recursive construction ("what's next?" at every layer) with strong reading reward for song-familiar solvers, but image-dependent identification creates a solvability ceiling for non-song-knowers.

---

## PUZZLE 11 — GM-01: The Planets
**Source:** Crossword Labs | **Tier:** Games Magazine | **Answer:** Grid fill (PLUTO, NEPTUNE, JUPITER, MERCURY, SATURN, VENUS, EARTH, MARS)

**Mechanism:** Standard themed crossword. Clues are direct definitions of solar system bodies.

| Dimension | Score | Justification |
|-----------|-------|---------------|
| Clarity | 5 | Nothing to clarify — fill the crossword. Format is universally understood. |
| Solvability | 5 | Every answer is middle-school science vocabulary. No solver capable of reading should fail to complete this puzzle. |
| Elegance | 2 | Purely functional. "Famous for its rings" for SATURN is a crossword cliché. No misdirection, no wordplay, no twist. "No longer officially considered a planet" for PLUTO is the one moment of mild cleverness. |
| Reading Reward | 1 | Solar system knowledge is elementary-school level. No specialist knowledge rewarded. The domain is load-bearing only in the trivial sense that you must know planets exist. |
| Fun | 2 | Completion satisfaction but no aha. Pleasant in the way that any successful recall is pleasant. |
| Confirmation | 5 | Crossword grid confirms instantly: crossing letters validate, and completion leaves no ambiguity. |
| **Total** | **20/30** | **FAIL** |

**Key finding:** The highest-confirmation, lowest-elegance puzzle in the corpus — the Games Magazine floor establishes that mechanical soundness without construction depth scores below threshold despite perfect clarity and solvability.

---

## PUZZLE 12 — GM-02: Weather
**Source:** Crossword Labs | **Tier:** Games Magazine | **Answer:** Grid fill (THUNDERSTORM, EVAPORATION, WEATHER, CONDENSATION, TORNADO, GROUNDWATER, HURRICANE, CLIMATE, HUMIDITY, METEOROLOGISTS, FLOOD)

**Mechanism:** Standard themed crossword. Clues are full-sentence definitions of meteorological terms.

| Dimension | Score | Justification |
|-----------|-------|---------------|
| Clarity | 5 | Standard crossword format; full-sentence clues eliminate ambiguity entirely. |
| Solvability | 5 | Middle-school earth science vocabulary. METEOROLOGISTS (15 letters) is the most demanding but entirely accessible. |
| Elegance | 2 | Slightly stronger than The Planets: the WEATHER answer in a weather-themed puzzle creates a mild self-referential moment. Full-sentence definitions are more considered than fragments. Still no wordplay or misdirection. |
| Reading Reward | 1 | Elementary-school science. No specialist knowledge engaged or rewarded. |
| Fun | 2 | Completion is the pleasure. METEOROLOGISTS is satisfying to fill in for its length alone. |
| Confirmation | 5 | Standard crossword confirmation: crossings validate, completion is unambiguous. |
| **Total** | **20/30** | **FAIL** |

**Key finding:** Marginally better clue craft than The Planets (fuller definitions, one self-referential answer) but the same structural profile — maximum clarity/confirmation, minimum elegance/reading reward.

---

## PUZZLE 13 — GM-03: Animals
**Source:** Crossword Labs | **Tier:** Games Magazine | **Answer:** Grid fill (ZEBRA, GIRAFFE, KANGAROO, CAT, PIGS, CHEETAH, FROG, DOG, ELEPHANTS, LION, RHINO)

**Mechanism:** Standard themed crossword. Clues are the most conventional possible animal definitions.

| Dimension | Score | Justification |
|-----------|-------|---------------|
| Clarity | 5 | Crossword format, unambiguous clues, zero friction. |
| Solvability | 5 | "Man's best friend" for DOG and "Likes to chase mice" for CAT are crossword clichés — the puzzle could be completed purely from knowing crossword conventions. |
| Elegance | 1 | The lowest-elegance puzzle in the corpus. Every clue is either a cliché ("Man's best friend," "King of the jungle") or a direct physical description. No wit, no misdirection, no construction interest. |
| Reading Reward | 1 | Knowledge of common animals is universal. No domain expertise rewarded or required. |
| Fun | 1 | Minimal engagement. The puzzle offers no discovery, no surprise, no aha. Completion is its only reward. |
| Confirmation | 5 | Crossword confirmation is immediate and unambiguous. |
| **Total** | **18/30** | **FAIL** |

**Key finding:** The absolute floor of the corpus — achieves perfect clarity/solvability/confirmation by the paradoxical method of having nothing to figure out. "Elegant" is not achievable when every element is conventional by definition.

---

## PUZZLE 14 — GM-04: Logic Grid (Simple)
**Source:** Canonical Games Magazine format | **Tier:** Games Magazine | **Answer:** Alice=Top/Cat, Carol=Middle/Dog, Bob=Ground/Fish

**Mechanism:** Three-variable elimination logic grid (person/floor/pet). Five clues constrain the solution to one assignment.

| Dimension | Score | Justification |
|-----------|-------|---------------|
| Clarity | 5 | Grid format is self-explanatory; clues are natural-language constraints with no ambiguity. |
| Solvability | 5 | Three variables, five clues, one solution. Any solver who has encountered a logic grid can complete this in under two minutes. |
| Elegance | 3 | Logic grids are inherently elegant as a format — systematic elimination is clean. This instance is minimal (3×3) but internally consistent. No unnecessary clues; every clue contributes. |
| Reading Reward | 1 | No domain knowledge required. Logical reasoning alone suffices. |
| Fun | 3 | The "click" of completing a logic grid is a genuine pleasure, even at minimal complexity. The satisfaction of the unique solution is real. |
| Confirmation | 5 | The unique solution and completed grid leave no ambiguity. |
| **Total** | **22/30** | **PASS (exactly at threshold)** |

**Key finding:** The Logic Grid outperforms the three themed crosswords despite being equally simple — the constraint-satisfaction format provides a genuine reasoning pleasure that direct-definition crosswords lack, and the clean confirmation of a unique solution tips it to passing.

---

## PUZZLE 15 — GM-05: Word Search with Hidden Message
**Source:** Canonical Games Magazine format | **Tier:** Games Magazine | **Answer:** SECRET MESSAGE

**Mechanism:** Find 8 hidden words in a 6×8 grid. Remaining letters, read left to right, spell a two-word phrase. The hidden message is announced in the instructions.

| Dimension | Score | Justification |
|-----------|-------|---------------|
| Clarity | 5 | Word search format is universally understood. The hidden message mechanic is explicitly stated ("remaining letters... spell a two-word phrase"). |
| Solvability | 4 | Word lists are short, common words. The hidden message mechanism is straightforward if the grid is correctly constructed — the only failure mode is a poorly filled residual, which is a constructor problem not a solver problem. |
| Elegance | 2 | The hidden-message mechanism is a documented word-search enhancement, not an original construction. The meta-layer (remaining letters spell something) is a standard variant. The puzzle tells the solver what to expect rather than letting them discover it. |
| Reading Reward | 1 | Finding PUZZLE, HUNT, CLUE etc. in a grid requires no domain knowledge. |
| Fun | 2 | Finding hidden words is pleasant. The residual message is pre-announced, removing the discovery element — the solver is completing a task rather than making a discovery. |
| Confirmation | 4 | The residual message confirms the grid is complete; any word not found will corrupt the message, providing built-in error detection. |
| **Total** | **18/30** | **FAIL** |

**Key finding:** Pre-announcing the hidden message eliminates the aha moment that would otherwise be the puzzle's best feature — confirmation and fun are directly traded off by the design choice to telegraph the mechanism.

---

## MASTER SUMMARY TABLE

| # | Puzzle | Tier | CL | SV | EL | RR | FN | CF | Total | Result |
|---|--------|------|----|----|----|----|----|-----|-------|--------|
| 1 | Scicabulary | MIT/Elite | 2 | 4 | 4 | 4 | 4 | 3 | **21** | FAIL |
| 2 | People Watching | MIT/Elite | 3 | 3 | 4 | 5 | 4 | 3 | **22** | PASS |
| 3 | H2No | MIT/Elite | 2 | 3 | 3 | 3 | 3 | 2 | **16** | FAIL |
| 4 | Bridge Building | MIT/Elite | 2 | 3 | 4 | 4 | 4 | 3 | **20** | FAIL |
| 5 | You're Telling Me | MIT/Elite | 4 | 4 | 4 | 3 | 4 | 3 | **22** | PASS |
| 6 | Dropypasta | Standard Hunt | 3 | 3 | 3 | 4 | 3 | 3 | **19** | FAIL |
| 7 | Information Relay | Standard Hunt | 2 | 3 | 5 | 3 | 4 | 4 | **21** | FAIL |
| 8 | Front and Center | Standard Hunt | 4 | 4 | 5 | 3 | 5 | 5 | **26** | PASS |
| 9 | Characters | Standard Hunt | 4 | 4 | 5 | 4 | 5 | 5 | **27** | PASS |
| 10 | What's Next? | Standard Hunt | 4 | 3 | 4 | 4 | 4 | 3 | **22** | PASS |
| 11 | GM-01: Planets | Games Magazine | 5 | 5 | 2 | 1 | 2 | 5 | **20** | FAIL |
| 12 | GM-02: Weather | Games Magazine | 5 | 5 | 2 | 1 | 2 | 5 | **20** | FAIL |
| 13 | GM-03: Animals | Games Magazine | 5 | 5 | 1 | 1 | 1 | 5 | **18** | FAIL |
| 14 | GM-04: Logic Grid | Games Magazine | 5 | 5 | 3 | 1 | 3 | 5 | **22** | PASS |
| 15 | GM-05: Word Search | Games Magazine | 5 | 4 | 2 | 1 | 2 | 4 | **18** | FAIL |

**CL** = Clarity | **SV** = Solvability | **EL** = Elegance | **RR** = Reading Reward | **FN** = Fun | **CF** = Confirmation

---

## TIER AVERAGES

| Tier | N | Avg CL | Avg SV | Avg EL | Avg RR | Avg FN | Avg CF | **Avg Total** | Pass Rate |
|------|---|--------|--------|--------|--------|--------|--------|---------------|-----------|
| MIT/Elite | 5 | 2.6 | 3.4 | 3.8 | 3.8 | 3.8 | 2.8 | **20.2** | 2/5 (40%) |
| Standard Hunt | 5 | 3.4 | 3.4 | 4.4 | 3.6 | 4.2 | 4.0 | **23.0** | 4/5 (80%) |
| Games Magazine | 5 | 5.0 | 4.8 | 2.0 | 1.0 | 2.0 | 4.8 | **19.6** | 1/5 (20%) |
| **All 15** | 15 | 3.7 | 3.9 | 3.4 | 2.8 | 3.3 | 3.9 | **20.9** | 7/15 (47%) |

---

## CROSS-TIER ANALYSIS

### Pass rates
Standard Hunt tier passes at 80%; MIT/Elite at 40%; Games Magazine at 20%. Counterintuitively, Standard Hunt outperforms MIT/Elite on this rubric.

### Why MIT/Elite underperforms Standard Hunt on this rubric
MIT/Elite puzzles trade Clarity and Solvability for Elegance and Reading Reward. The rubric penalizes this trade: Scicabulary (21), Bridge Building (20), and H2No (16) all fail due to steep clarity or solvability costs. The rubric as written does not adequately credit the ambition of the MIT/Elite construction — it treats a puzzle that 5% of solvers can complete as equivalent in quality to one 95% of solvers can complete.

### Why Games Magazine nearly matches MIT/Elite in total score
Games Magazine crosswords score 5/5 on Clarity, 4–5/5 on Solvability, and 5/5 on Confirmation. This floors the total around 14–15 before elegance or fun are scored, which partially offsets their near-zero scores on Elegance and Reading Reward. The rubric is not strongly discriminating between Games Magazine and MIT/Elite as tiers.

### The dimension most diagnostic of tier
Reading Reward is the most discriminating dimension: MIT/Elite averages 3.8, Standard Hunt 3.6, Games Magazine 1.0. A puzzle that scores 4–5 on Reading Reward is almost certainly not Games Magazine tier. A puzzle that scores 1–2 on Reading Reward is almost certainly not MIT/Elite tier.

### The dimension least diagnostic of tier
Clarity is inversely correlated with tier: MIT/Elite averages 2.6, Games Magazine averages 5.0. Under this rubric, difficulty of entry is scored as a quality defect, which inverts the typical puzzle-hunt value hierarchy (where high-concept puzzles are valued over low-barrier ones).

### Standout puzzles
- **Characters (27/30):** Best total score; self-describing answer achieves maximal confirmation through thematic closure.
- **Front and Center (26/30):** Best elegance score (tied with Characters); palindrome recursion achieves structural perfection.
- **H2No (16/30):** Lowest scoring of all 15 puzzles, below the Games Magazine crosswords — the extraction mechanism is underspecified enough to score near-basement on Clarity and Confirmation.

### GM-04 Logic Grid anomaly
The Logic Grid puzzle (22/30, PASS) outscores three MIT/Elite puzzles and ties three Standard Hunt puzzles. This is a rubric calibration signal: the format's inherent clarity, solvability, and confirmation properties combine to produce a passing score without any of the construction depth the rubric ostensibly values. The rubric does not adequately penalize low construction ambition.

---

## METHODOLOGICAL NOTE FOR ABLATION

**C1 condition findings:** Without design principles or puzzle-hunt profiles as context, the rubric:
1. Fails to distinguish construction ambition from pedagogical accessibility (Logic Grid passes; Bridge Building fails)
2. Overweights Clarity and Confirmation (both 5-point scales where Games Magazine maxes out easily)
3. Underweights the relationship between mechanism depth and Reading Reward (which is the most diagnostic dimension but only one-sixth of the total score)
4. Produces a counter-tier result: Standard Hunt > MIT/Elite > Games Magazine by pass rate, which inverts the intended quality hierarchy

The 6-dimension rubric without supporting context is a necessary but insufficient evaluation framework for tier discrimination. C2 and C3 conditions should show measurably better tier alignment if profiles and principles provide the calibration context this rubric lacks.
