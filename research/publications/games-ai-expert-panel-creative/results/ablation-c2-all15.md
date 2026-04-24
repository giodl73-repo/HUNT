# Ablation Study — Condition C2: Principles Compact (Names and Quotes Only)

**Condition:** C2 — Principles provided as names + quotes only; no tests, no personas, no examples.
**Reviewer context:** "You are an expert reviewer of puzzle design."
**Principles available:** Riven Standard, Solving=Understanding, Blame the Player, No Over-Scaffolding, Surprise the Answer, One Aha, Reading Reward, No Computation Without Deduction, Snyder's Computer Test, Interlock Not Independence, Verify the Last Mile.
**Date:** 2026-02-28
**Puzzles evaluated:** 15 total — Batch 1 (3), Batch 2 (3), Batch 3 (4), Games Magazine (5)

---

## BATCH 1

---

### Puzzle B1-1: Scicabulary
**Answer:** SHAMROCK
**Source:** MIT Mystery Hunt 2023

#### Mechanism Summary
24 fake jargon terms, each formed by combining two opposite halves of two real portmanteau words. 24 definitions clue two-word phrases. Solver matches terms to phrases, then indexes the given number into the matched phrase to extract one letter per pair. All 24 letters yield SHAMROCK.

#### Solving Notes
The puzzle demands two distinct acts of understanding before any indexing can begin. First, the solver must decode what "opposite halves of portmanteau words" means — i.e., that HICED is formed from the back half of "spiked" and the front half of "iced" (or similar), while the definition clues an independent two-word phrase. Second, the solver must determine which two-word phrase belongs to which jargon term, which is not a lookup but a constraint-satisfaction: the enumeration of the two-word phrase and the letter index both constrain the matching. The extraction is clear once matching is established — the index number directly states which letter to take.

The critical aha: realizing that "opposite halves" means the first term uses the back half of portmanteau A and the front half of portmanteau B, while another term uses the front of A and the back of B. This is elegant but genuinely difficult to parse from the flavor text alone. The worked hint ("Notice patterns between [SPACE ATOM] and [HICED]") is required scaffolding without which many solvers will not make the decoding step at all.

SHAMROCK as an answer is arbitrary — it does not comment on the portmanteau mechanism or reward the solver's understanding of the theme. The answer is a word that emerges from the process but does not close a loop.

**Principles fired:**
- **No Over-Scaffolding** — the three-hint progression is necessary scaffolding; without hint 2, the decoding step is likely inaccessible. The hints do not add decoration; they are load-bearing. This is borderline: the puzzle requires them to be solvable at all, suggesting the mechanism is not self-revealing.
- **Solving = Understanding** — to solve this correctly a solver must genuinely understand what "opposite halves of portmanteau words" means, not just apply a formula. Satisfied.
- **Snyder's Computer Test** — a computer could generate this puzzle type given a portmanteau database and an indexing step. The wit is in the definitions (which are playful), but the structure is algorithmic. Partially fires: the definitions are amusing but the mechanism is fully automatable.
- **Verify the Last Mile** — the matching step and the indexing step are two distinct skills. A solver who correctly matches but misidentifies the portmanteau halves gets wrong letters. The last mile (index → letter → final word) is clean; the penultimate mile (match assignment) is harder to verify. Partially fires.
- **Surprise the Answer** — SHAMROCK does not pause the solver. It is an arbitrary real word with no thematic connection to portmanteaux or jargon. Does not satisfy.

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 3 | The flavor text is technically precise but difficult to parse. The mechanism ("opposite halves") takes real effort to decode. The hint chain is necessary scaffolding, not optional polish. |
| Solvability | 4 | Once the decoding aha fires, the matching is constrained and workable. The 24-item scale makes it tedious but not unsolvable. The matching is the hard bottleneck. |
| Elegance | 4 | The construction is genuinely clever — portmanteaux deconstructed in mirror fashion is a clean idea. Slightly undercut by the fact that the definitions feel independent from the portmanteau theme rather than thematically unified. |
| Reading Reward | 3 | Some domain familiarity with portmanteaux (frenemy, brunch, etc.) helps, but the puzzle does not reward deep engagement with any specific domain. The definitions are general-knowledge wordplay. Partial credit. |
| Fun | 4 | The definitions are witty (a snake that can read your mind; a prediction of a snail-storm). Solving individual definitions is pleasurable even before the matching mechanism clicks. |
| Confirmation | 3 | SHAMROCK confirms via word-formation (letters come out), but does not self-explain. A solver who gets a real word is likely correct; a solver who gets a non-word knows something went wrong. Partial confirmation. |
| **Total** | **21/30** | |

**Verdict: FAIL (21 — one point below threshold)**

---

### Puzzle B1-2: People Watching
**Answer:** HINDWING
**Source:** MIT Mystery Hunt 2023

#### Mechanism Summary
14 social-media "observations" each encode a bird species through anthropomorphized behavior, dialogue (representing bird calls), and visual field marks. Solvers identify the bird, take the IBP 4-letter alpha code for each, and extracted letters spell HINDWING.

#### Solving Notes
The puzzle is doing something unusual and successful: it is running an entirely parallel encoding. Each "observation" contains at minimum two convergent channels — vocalization (rendered as human speech), visual markings (colors, physical features), and sometimes behavioral or habitat cues. The solver who recognizes this reads each tweet not as a character sketch but as a field guide entry in disguise. That recognition — that the lady who "screams 'The pot! The pot! The pot!'" and has a white eyebrow is not a person but a Willet (or similar) — is the central aha, and it fires hard.

The IBP code extraction is the secondary mechanism. Codes like WILL (Willet), BADO (Badger), etc. are real ornithological tools, and the puzzle requires knowing or looking up these four-letter codes. The extraction method (codes → letters → HINDWING) is not self-evident; a solver who identifies all birds but does not know about IBP codes is stuck at the last mile.

HINDWING as an answer is a wing part — not obviously a bird part, slightly ironic in a bird puzzle (the wing not of the bird but of the insect order Lepidoptera). This is a minor surprise: HINDWING is a real word, unexpected, and does comment back on the observation theme (watching the creatures, finding what they have in common).

**Principles fired:**
- **Riven Standard** — the puzzle IS birdwatching as a cognitive activity: field marks, vocalizations, behavioral cues. The solving process is exactly what a birder does. Strongly satisfied.
- **Solving = Understanding** — a solver who correctly identifies all 14 birds has demonstrated genuine understanding of bird vocalizations and field marks. Strongly satisfied.
- **Blame the Player** — the clues are fair. Each observation contains multiple confirming channels. A solver who misidentifies a bird has genuinely misread the field marks. Satisfied.
- **Reading Reward** — birding knowledge is load-bearing. Non-birders can partially solve via internet lookup, but the puzzle genuinely rewards solvers who recognize calls. Satisfied.
- **Interlock Not Independence** — the 14 observations do not cross-reference each other; each is solved independently. The IBP codes do not inform each other. Independence concern: this is 14 independent lookups with a final letter extraction. The crossword constraint (multiple codes must cooperate to form HINDWING) provides mild interlock at the extraction stage. Partially fires — mild concern.
- **Verify the Last Mile** — IBP code knowledge is a separate skill from bird identification. A solver who identifies all birds but does not know what IBP codes are cannot extract. Fires — this is a genuine two-skill gap.

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | The flavor text ("recall what they said") is an elegant pointer. The IBP hint in the canned hints is necessary and clear. The presentation as social media tweets is evocative and effective. |
| Solvability | 4 | Bird identification via vocalizations is specialized but Internet-accessible. The IBP codes are lookup-able. The path is not blocked; it requires knowing where to look. |
| Elegance | 5 | The double-encoding (field marks + vocalizations rendered as human behavior) is beautifully constructed. Every tweet carries multiple confirming signals. The flavor text is thematically unified. |
| Reading Reward | 5 | A birder gets this faster and with more pleasure than anyone else. The vocalizations are transcribed with genuine accuracy. Domain knowledge is not just helpful — it is delightful. |
| Fun | 5 | The tweet format is charming. Individually, each observation is a small puzzle of recognition and joy. The moment of "wait, she's not a person, she's a bird" fires as a genuine aha. |
| Confirmation | 3 | HINDWING is a real word and unexpected enough to feel satisfying. However, a solver cannot confirm the word is correct without external verification — it does not self-confirm within the bird domain. |
| **Total** | **26/30** | |

**Verdict: PASS (26)**

---

### Puzzle B1-3: H2No
**Answer:** ENCAMPMENT
**Source:** MIT Mystery Hunt 2023

#### Mechanism Summary
Six clues each describe a location through a date-arithmetic relationship between two historical events at the same place (e.g., "124 years after a glassware company was founded by Neumann"). Solver identifies all six locations, determines what they have in common (the shared property), then extracts letters from the "differences" between how each location expresses that property.

#### Solving Notes
The date-arithmetic is load-bearing: for each clue, the solver must identify the anchor event, compute the offset year, then identify a second historical event at the same location in that target year. Both events must be co-located — this double constraint severely limits the solution space and makes false positives rare. The mechanism is sound.

The critical extraction step depends on the flavor text: "watch out for any differences." This is the puzzle's weakest link. What "differences" means is not stated, and the extraction mechanism is obscured behind intentionally vague language. A solver who correctly identifies all six locations and their shared property must then independently discover what differentiates them and how those differences encode letters. The flavor text is coy rather than mysterious — it withholds necessary mechanism rather than offering elegant ambiguity.

ENCAMPMENT as an answer does not obviously connect to water, history, or the shared property of the locations. The title H2No suggests "water" as the unifying theme (locations with notable water-related history or NO water — a water denial), which is clever but not confirmed by the materials available.

**Principles fired:**
- **No Computation Without Deduction** — computing years from offsets is arithmetic, not puzzle-solving. The deduction is the co-location constraint and the identification of events. The arithmetic is a scaffold for the deduction, not the answer. Partially fires: the arithmetic is lightweight enough not to dominate.
- **Verify the Last Mile** — the extraction step ("watch for differences") is structurally obscured. A solver completing the identification step correctly has no clear path to the final answer. Fires strongly.
- **Blame the Player** — the vague flavor text makes blaming the player for missing the extraction step unfair. The puzzle does not give the solver enough information to know what kind of "difference" to look for. Fires against the puzzle.
- **Snyder's Computer Test** — the historical co-location mechanism could be generated algorithmically from a historical events database. The puzzle's wit is in the selection of locations and events, but the structure is automatable. Fires.

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 2 | The date-arithmetic clues are clear individually. The extraction step ("watch for differences") is critically underspecified. A solver who completes the identification step has no reliable guide to the extraction. |
| Solvability | 3 | The identification step is hard but resolvable with research. The extraction step is where solvers are likely to stall completely. A solver who accidentally stumbles onto the "differences" encoding has no way to verify they found the intended mechanism. |
| Elegance | 3 | Date-arithmetic with co-location constraints is clever. The "differences" extraction, if the intended mechanism is what I believe it is (variation in how the shared water-related property manifests), is elegant in concept. In execution, the opacity of the flavor text undermines it. |
| Reading Reward | 3 | Local and historical knowledge helps but is not deeply rewarded. The puzzle is more about research than domain expertise. |
| Fun | 3 | The individual date puzzles are satisfying to crack. The fun collapses at the extraction bottleneck. |
| Confirmation | 2 | ENCAMPMENT does not self-confirm. A solver cannot verify arrival at the correct answer through internal puzzle logic. |
| **Total** | **16/30** | |

**Verdict: FAIL (16)**

---

## BATCH 2

---

### Puzzle B2-1: Bridge Building
**Answer:** PROTEINPOWDER
**Source:** MIT Mystery Hunt 2023

#### Mechanism Summary
A Hashiwokakero (island-bridge) logic puzzle whose solved grid, when read as a molecular structure (polymer backbone with ionic charge superscripts on nodes), identifies a specific polymer — protein powder's constituent amino acid chain — yielding the answer PROTEINPOWDER.

#### Solving Notes
The puzzle is doing two entirely separate and competent things in sequence. The first is a standard Hashi logic puzzle — a well-defined genre with clear rules, and the title directly signals the genre. A solver who enjoys Hashi can complete this step independently of the chemistry. The second step — reading the solved grid as a molecular backbone — is a genuinely surprising second layer.

The ionic charge superscripts (+/−) are the aha-gating device: they are anomalous within Hashi (which uses only integer node counts) and signal to a chemically literate solver that the nodes represent atoms with charges. The connection between a bridge-building puzzle grid and a polymer backbone is conceptually elegant: both consist of nodes connected by bonds/bridges, and the Hashi constraint (maximum 4 bridges per node, matching carbon's valence) is structurally apt.

However, the chemistry step is extremely demanding. Identifying a specific polymer from a solved Hashi grid requires knowledge of amino acid charges, polymer backbone notation, and the ability to map a 2D puzzle grid onto a 3D chemical structure. This is not "Reading Reward" in the usual sense — it is a domain barrier that most puzzle solvers (who are not chemists) cannot cross without the canned hints explicitly pointing them to chemistry.

PROTEINPOWDER as an answer is satisfying in a slightly comic way — it is a gym-culture object, not a scientific term, which creates a deliberate mismatch with the rigorous chemistry that precedes it. The mismatch is intentional (this is a museum exhibit puzzle) and funny.

**Principles fired:**
- **Riven Standard** — the puzzle grid IS a molecular structure, not a structure overlaid on a molecular metaphor. The Hashi grid is literally isomorphic to the chemistry representation. Strongly satisfied.
- **One Aha** — there are two distinct ahas: (1) solve the Hashi, (2) read as chemistry. Whether this is "one aha" depends on framing. Each step has its own mechanism and satisfaction. Fires as "two ahas" concern.
- **Snyder's Computer Test** — the Hashi portion is fully computer-generatable and solvable. The chemistry reading is what a computer cannot do without chemical domain knowledge. The chemistry layer saves this from failing. Partially fires.
- **Verify the Last Mile** — identifying a specific polymer from a grid requires chemistry knowledge that most puzzle solvers lack. The last mile is a domain barrier. Fires.

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 3 | The Hashi instructions are implicitly clear (genre is titled). The chemistry step is unannounced and requires the solver to notice the anomalous charge superscripts. The hints are necessary to gate the chemistry interpretation. |
| Solvability | 3 | The Hashi portion is solvable by any logic puzzle fan. The chemistry portion requires specialized knowledge that most hunt solvers do not have. The domain barrier is high. |
| Elegance | 5 | The structural isomorphism between Hashi grids (nodes, max-4 connections) and molecular backbones (atoms, carbon valence) is genuinely beautiful. The puzzle earns its chemistry because the fit is real, not decorative. |
| Reading Reward | 4 | A chemist gets enormous reward from this puzzle. The grid-as-molecule reading is a delight for the right audience. General solvers are excluded from this reward, which is a cost. |
| Fun | 4 | The Hashi is satisfying independently. The chemistry aha (for those who get it) is a genuine delight. The answer PROTEINPOWDER is comic and deflating in a fun way. |
| Confirmation | 4 | A chemist who identifies the polymer and sees PROTEINPOWDER knows they are right. Non-chemists cannot self-confirm at the chemistry step. |
| **Total** | **23/30** | |

**Verdict: PASS (23)**

---

### Puzzle B2-2: You're Telling Me
**Answer:** CARBONSINK
**Source:** MIT Mystery Hunt 2023

#### Mechanism Summary
25 (plus one — 26 total entries) rhetorical questions each describe a compound word or phrase read in a non-standard "Shrimp Fried Rice" meme style. Solvers identify the intended compound, then extract letters to form the answer CARBONSINK.

#### Solving Notes
The mechanism is elegant and accessible. The "Shrimp Fried Rice" meme format is well-known enough to be recognizable but not over-exposed, and the puzzle applies it with genuine wit. Each clue is a misreading: "Can a two-dimensional space even contain a gas?" reads FLATBED (flat + bed, but parsed as "flat space for a bed" rather than the truck), or more precisely, it targets a compound where one word changes meaning depending on parsing position.

The 26 clues are sorted alphabetically by their answers, which is an unusual and initially disorienting presentation choice. The sorting removes the linear reading order as a scaffold, forcing solvers to work from the content of each clue rather than from positional context. This is a legitimate difficulty calibration.

CARBONSINK as an answer is thematically unified: "carbon" and "sink" (as in carbon sink, an environmental concept) reframes the compound word game as environmentally themed. The answer does comment back on the puzzle domain in a mild way.

The main concern is the scale: 26 clues with alphabetical sorting creates a heavy extraction step. If the extraction mechanism is straightforward (e.g., first letter of each answer in alphabetical order), the scale is manageable. If it requires additional steps, the load is high.

**Principles fired:**
- **One Aha** — the single aha is "these are Shrimp Fried Rice misreadings." Once this fires, all 26 clues become tractable by the same method. Single aha, cleanly delivered. Satisfied.
- **No Over-Scaffolding** — the presentation does not explain the meme or provide worked examples. Solvers must recognize the format or derive it from context. Appropriately lean. Satisfied.
- **Snyder's Computer Test** — a computer could generate compound-word misreadings from a dictionary. The wit in these specific clues (especially "Isn't scaring a reptile animal abuse?" for ALLIGATOR CLIP or similar) is human, but the mechanism is automatable. Fires as partial concern.
- **Interlock Not Independence** — the 26 clues are solved independently; there is no cross-reference between solutions. The alphabetical sorting does not create interlock — it is a sequencing choice. Fires: this is 26 independent lookups with extraction.
- **Surprise the Answer** — CARBONSINK is mildly surprising (the environmental pun lands), though "carbon sink" is a known environmental concept. Partial credit.

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | The meme format is self-identifying once recognized. The rhetorical-question structure makes each clue unambiguous in what kind of misreading is intended. The alphabetical sorting is confusing until the solver realizes answers are sorted, not clues. |
| Solvability | 4 | Each clue is individually tractable once the format clicks. The alphabetical sorting is an obstacle that resolves on first successful identification. Scale (26 clues) is manageable for a hunt team. |
| Elegance | 4 | The meme-as-puzzle-mechanism is a neat conceit. The environmental answer is a satisfying thematic coherence. The alphabetical ordering adds mild elegance (solvers get self-confirmation when their answer fits alphabetically). |
| Reading Reward | 3 | No specific domain knowledge is required beyond compound word knowledge and meme recognition. Pleasant but not deeply rewarding for expertise. |
| Fun | 4 | Individually the clues are witty and enjoyable. The format allows rapid "got it!" moments. The alphabetical ordering provides ongoing confirmation that answers are correct. |
| Confirmation | 4 | Alphabetical ordering of answers provides natural ongoing verification. CARBONSINK lands cleanly. |
| **Total** | **23/30** | |

**Verdict: PASS (23)**

---

### Puzzle B2-3: Dropypasta
**Answer:** VINDICATIONISLAND
**Source:** MIT Mystery Hunt 2023

#### Mechanism Summary
A dropquote puzzle where all quotes come from the Super Smash Bros. universe. Each row has one letter missing from the top; that letter is extracted. Extracted letters are assigned to two characters per Smash Bros. stage; stage-specific win conditions (alphabetical, release date, tier ranking, Kirby inhale damage, proximity to Pokémon on select screen, weight) determine which character "wins" and contributes their extracted letter to the final answer VINDICATIONISLAND.

#### Solving Notes
The puzzle has three distinct layers of mechanism. Layer one (dropquote filling + Smash universe identification) is straightforward for fans and impenetrable for non-fans. Layer two (extracting the dropped letters) is mechanical and clean. Layer three (the stage win conditions) is where the puzzle becomes unusual: each stage imposes a completely different win condition, requiring the solver to know or research six entirely different facts about Smash Bros. characters. The tier list condition requires knowing the current competitive tier list; the Kirby inhale damage condition requires very detailed game knowledge; the proximity-on-select-screen condition requires either memory or access to the game UI.

VINDICATIONISLAND is a long answer — 17 letters — and each of its letters is gated behind a different win condition. The diversity of win conditions prevents routine: the solver must engage differently with each stage. This is intentional and clever, but the cognitive load is high.

The answer VINDICATIONISLAND is amusing (vindication is what a challenger feels after winning; "island" may reference the fighting game stage type). It is a compound of two meaningful words that creates an unexpected combination.

**Principles fired:**
- **One Aha** — there is no single aha. The puzzle requires six different ahas (one per win condition) plus the initial dropquote identification. This is well above "one aha is fun, three ahas is a slog." Fires as concern.
- **Interlock Not Independence** — the six stages are solved independently; the win conditions do not inform each other. Fires: six independent win-condition lookups.
- **Reading Reward** — Smash Bros. knowledge is deeply load-bearing. Knowing tier rankings, release dates, Kirby mechanics, and select screen layout all reward domain expertise. Satisfied for the target audience.
- **Snyder's Computer Test** — the dropquote filling and character-matching steps are automatable. The win conditions are domain-specific lookups. Partially fires.
- **Verify the Last Mile** — six different win conditions mean six different potential error points. A solver who gets five of six win conditions correct cannot detect the error from the answer alone. Fires.

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 3 | The dropquote mechanism is clear. The stage win conditions are individually clear but collectively overwhelming — six different rules requiring six different types of knowledge. |
| Solvability | 3 | Completely dependent on Smash Bros. expertise. The tier list condition is time-sensitive (tier lists change). The Kirby inhale damage condition is obscure even within Smash knowledge. |
| Elegance | 3 | The diversity of win conditions is creative but excessive. Each individual condition is elegant; the combination is more of a trivia gauntlet than a unified puzzle. |
| Reading Reward | 4 | For Smash Bros. fans, this puzzle is a rewarding test of deep knowledge across multiple game mechanics. The audience specificity is high. |
| Fun | 3 | Fun for deep Smash fans, alienating for others. The dropquote layer is entertaining independent of Smash knowledge. |
| Confirmation | 3 | VINDICATIONISLAND is distinctive enough to feel like a correct answer. The length and meaning create mild self-confirmation. |
| **Total** | **19/30** | |

**Verdict: FAIL (19)**

---

## BATCH 3

---

### Puzzle B3-1: Information Relay
**Answer:** FINISH LINE
**Source:** Huntinality III 2023

#### Mechanism Summary
A "telephone game" puzzle where five bears (Papa, Mama, Child, Grandma, Grandpa) each give broken clues for six target words in their own characteristic ways. Solvers identify which bear gave each pair of clues, deduce the target words, then apply one bear's interpretation method to another's clues to extract the answer. The final extraction: "HOW NANA HEARS GRAMPS" means apply Grandma's method (interpret as homophones) to Grandpa's clues, yielding FINNISH LINE → homophone: FINISH LINE.

#### Solving Notes
This is a structurally sophisticated puzzle. The meta-layer — applying one bear's method to another's clues — requires the solver to hold the entire mechanism in mind simultaneously. The six target words and their clue sets are the verification layer: solve the words to confirm which bear uses which method, then use that confirmed understanding to crack the extraction.

The extraction step ("HOW NANA HEARS GRAMPS" → apply Grandma's method to Grandpa's clues) is genuinely elegant. It requires understanding both bears' methods: Grandpa's clues are completely unrelated (he misunderstood the game), and Grandma clues homophones. Applying Grandma's method to Grandpa's clues means treating Grandpa's clue answers as homophones of the target word, then using those targets to spell the answer. FINNISH LINE → homophone → FINISH LINE is a satisfying final twist.

The aha hierarchy is clean: (1) identify the bear methods, (2) solve the target words to confirm, (3) read the extraction instruction from indexed letters, (4) apply the correct method to the correct clues. This is a "one aha" puzzle in the sense that each step gates naturally to the next, though there are four stages. The ahas are sequential rather than parallel, which mitigates the slog risk.

**Principles fired:**
- **Solving = Understanding** — a solver who correctly extracts FINISH LINE has demonstrably understood all five bear methods, the telephone game mechanic, and the meta-application instruction. Strongly satisfied.
- **Riven Standard** — the puzzle IS the telephone game as a cognitive structure: each bear's broken transmission method is what the puzzle asks you to understand. Satisfied.
- **One Aha** — four sequential stages, but the structure is telescoping rather than independent. Acceptable.
- **Blame the Player** — the bear methods are deducible from the clue examples. A solver who misidentifies a bear's method has misread the evidence. Fair.
- **Surprise the Answer** — FINISH LINE is a mild surprise: the homophone step (FINNISH LINE) is funny and the racing metaphor is apt for a relay-game puzzle. Satisfied.

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | The puzzle's structure (five bears, different methods) is entertainingly clear. The extraction instruction, once extracted ("HOW NANA HEARS GRAMPS"), is unambiguous in what it requires. |
| Solvability | 4 | Each bear's method is deducible from examples. The target word identification provides self-verification. The final extraction requires understanding both relevant bear methods. High but fair difficulty. |
| Elegance | 5 | The telescoping structure — understand the bears to confirm the words, read the words to get the extraction instruction, apply the instruction to get the answer — is beautifully unified. The answer is a homophone of something in the puzzle. |
| Reading Reward | 3 | No specific domain expertise is required beyond general knowledge and puzzle-thinking. The bears are fictional; the game references are accessible. |
| Fun | 5 | The bear characters are charming and their distinct "voices" make clue identification entertaining. The final twist (FINNISH LINE → FINISH LINE) earns a laugh. |
| Confirmation | 4 | FINISH LINE is immediately recognizable as a real phrase. The racing metaphor is apt. The homophone chain provides internal verification. |
| **Total** | **25/30** | |

**Verdict: PASS (25)**

---

### Puzzle B3-2: Front and Center
**Answer:** RADAR
**Source:** Huntinality III 2023

#### Mechanism Summary
Five newspaper clippings from "SEMI TIMES" each have blacked-out palindrome headlines. Word-length grids for each headline are provided. Subtitles describe the story content. Solvers reconstruct the palindrome headlines, take the center letter of each, and spell RADAR — itself a palindrome.

#### Solving Notes
This puzzle is a masterclass in economy. Every element serves double duty: the newspaper name (SEMI TIMES) is a palindrome, hinting at the mechanism before the solver even reads the clippings. The "front and center" title directly instructs what to do (take the center letter). The subtitles are genuinely clue-ful: "Paradise damaged due to errant throw of Cuban export, situation described as 'extremely awful'" cleanly clues CIGAR TOSS IN EDEN IS SO TRAGIC without ambiguity. The word-length grids provide confirmation as reconstruction proceeds.

The answer RADAR is the puzzle's best trick: the answer is itself a palindrome, which is not a coincidence but the puzzle closing a loop. A solver who extracts RADAR immediately sees the elegance — the answer IS what the headlines ARE, not just a word that emerges.

Five palindromes is the right scale. One palindrome is trivial; ten would be exhausting. Five allows a difficulty gradient (STAR RATS STATS STAR RATS is easier than MR. OWL ATE MR. METAL WORM) while keeping the solving time appropriate.

**Principles fired:**
- **Riven Standard** — the puzzle IS palindrome construction: the solving process (reconstructing palindromes from word-length grids and descriptive subtitles) is what a palindromist does. Strongly satisfied.
- **Solving = Understanding** — a solver who reconstructs all five headlines understands palindrome structure. Satisfied.
- **Surprise the Answer** — RADAR pauses the solver: "the answer is a palindrome" is the final recognition, and it is earned. Strongly satisfied.
- **One Aha** — one aha (these are palindromes, take center letters), cleanly delivered, with the bonus second recognition that the answer is also a palindrome. Satisfied with grace notes.
- **Blame the Player** — the word-length grids, title, and newspaper name provide sufficient scaffolding. A solver who misses the palindrome recognition has missed multiple signals. Fair.
- **No Over-Scaffolding** — the puzzle does not explain what a palindrome is. It expects solvers to recognize the structure. Lean and appropriate. Satisfied.

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Title, newspaper name, and word-length grids are all doing productive work. The subtitle clues are fair and effective. No wasted elements. |
| Solvability | 4 | Palindrome reconstruction from word-length grids and subtitles is a pleasant and tractable challenge. Some headlines (STAR RATS STATS STAR RATS) are very easy; others (A SANTA DEVIL NEVER EVEN LIVED AT NASA) require more construction. The difficulty gradient is well-calibrated. |
| Elegance | 5 | Every element in the puzzle contributes meaningfully. The answer closes the loop. The newspaper name is a cheeky in-joke that rewards attention. |
| Reading Reward | 3 | No domain expertise required. General linguistic fluency and wordplay familiarity are helpful but not specialized. The pleasure is construction-joy, not domain knowledge. |
| Fun | 5 | Palindrome reconstruction is inherently delightful. The five subtitles are themselves amusing ("consumes iron wiggler colleague"). The RADAR reveal is a clean payoff. |
| Confirmation | 5 | RADAR is immediately recognizable as a palindrome. A solver who extracts RADAR knows they are right — the answer confirms itself through being the thing the puzzle is about. |
| **Total** | **27/30** | |

**Verdict: PASS (27)**

---

### Puzzle B3-3: Characters
**Answer:** UNHEXING
**Source:** Huntinality III 2023

#### Mechanism Summary
Eight equations use clues to identify words made only of the letters A–F. These words are treated as hexadecimal strings. Arithmetic on those hex values produces decimal numbers that are read as ASCII codes, spelling UNHEXING.

#### Solving Notes
The puzzle requires recognizing three nested encodings simultaneously: that clue answers contain only A–F, that A–F = hexadecimal, and that decimal results are ASCII characters. The flavor text ("find their base and then understand their code") tips both revelations without stating them.

The eight equations are well-constructed: the individual clues (BABA from *Baba Is You*, CAFE for "small restaurant", ACE for "highest-ranking card") are accessible and even charming, and the discovery that they all contain only hex digits is the primary aha. The hex arithmetic is straightforward; the ASCII decoding is a standard puzzle step.

UNHEXING as an answer is perfect: it is a self-referential word (un-hexing = removing the hex), the process of what the solver just did (decoded from hex), and a play on "unhexing" as removing a curse. The answer is doing multiple things at once.

The main concern is the "No Computation Without Deduction" principle: the solver does arithmetic (hex subtraction, multiplication) to get decimal values. The arithmetic is not the deduction — the deduction is recognizing the nested encoding scheme. But the arithmetic is unavoidable and somewhat mechanical. Whether this fires depends on whether the arithmetic is the "puzzle" or merely a tool for expressing the puzzle.

**Principles fired:**
- **Riven Standard** — the puzzle IS hexadecimal encoding and decoding. The solving process is computing in base 16 and reading ASCII. Strongly satisfied.
- **Surprise the Answer** — UNHEXING is a genuine surprise: it self-describes the process, reverses the hex encoding, and plays on "unhexing as a curse." Strongly satisfied.
- **Solving = Understanding** — a solver who completes this understands hex arithmetic and ASCII encoding. Satisfied.
- **No Computation Without Deduction** — the arithmetic here is the vehicle for the deduction, not a substitute for it. The deduction is: "these words are in base 16, the results are ASCII." The arithmetic follows from that deduction. Borderline — fires weakly.
- **One Aha** — one aha (everything is hex/ASCII), cleanly delivered. Satisfied.

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | The flavor text pointers ("find their base," "understand their code") are elegant without over-explaining. The equation format is clean. |
| Solvability | 4 | Identifying hex-only words from the clues is the hard step, but examples like BABA and CAFE are accessible. The arithmetic is mechanical once the encoding is identified. |
| Elegance | 5 | Self-referential answer + nested encoding layers + flavor text that tips both without stating either = very tight construction. |
| Reading Reward | 3 | Benefit from knowing ASCII codes and hex arithmetic helps, but these are general technical knowledge rather than domain expertise. |
| Fun | 4 | The individual clues (especially BABA, CAFE, DEAD) are charming. The final reveal of UNHEXING is a genuine delight. |
| Confirmation | 5 | UNHEXING is self-confirming: it describes the process, it contains only letters A–U (not constrained to A–F but clearly a real word), and it is immediately recognizable as intentionally self-referential. |
| **Total** | **25/30** | |

**Verdict: PASS (25)**

---

### Puzzle B3-4: What's Next?
**Answer:** AFGHANISTAN
**Source:** Huntinality III 2023

#### Mechanism Summary
Ten images depict events/people from Billy Joel's "We Didn't Start the Fire." Each image is paired with numbers A/B. Solver identifies the event, finds the *next* item in the song's lyric sequence, uses A as the index and B as the letter count (confirmation). Extracted letters spell RUSSIANS IN, which is a lyric from the song. The word that follows "RUSSIANS IN" in the song is AFGHANISTAN.

#### Solving Notes
The puzzle has a satisfying double-aha structure that does not trip the "three ahas is a slog" concern because the ahas are hierarchically nested: (1) recognize the song and find the "next item," (2) recognize that the extracted phrase is itself a lyric, and the answer is what follows. The second aha is specifically rewarding because it asks the solver to apply the same "what comes next" logic to the extracted phrase itself — a recursive application of the puzzle's own mechanism.

AFGHANISTAN as an answer is maximally satisfying for this puzzle: it is the next item in the song, so the solver who understands the mechanism immediately recognizes it as correct without any external verification.

The ten images are the hard step: visual identification of historical events from a dense mid-century history song requires either prior knowledge of "We Didn't Start the Fire" or significant research. The B/letter-count confirmation numbers provide a useful check that prevents false positives.

**Principles fired:**
- **Solving = Understanding** — a solver who extracts AFGHANISTAN has demonstrated understanding of the song's structure and the recursive "next item" mechanism. Strongly satisfied.
- **Surprise the Answer** — AFGHANISTAN is an unexpected answer word (not a puzzle concept, not a meta-reference, but a geopolitical entity from a song about history). The surprise is mild but real. Satisfied.
- **One Aha** — two ahas, but hierarchically nested and mutually reinforcing. The second aha applies the same mechanism as the first. Acceptable.
- **Reading Reward** — "We Didn't Start the Fire" knowledge is load-bearing. Fans of the song get enormous advantage. Satisfied.
- **Blame the Player** — the B/letter-count confirmation provides a built-in check. A solver who identifies the wrong event will see a letter-count mismatch. Fair.
- **Riven Standard** — the puzzle IS the song: the solving process is navigating the song's historical narrative. Satisfied.

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | The format (image → next item → index) is clear. The recursive second step is not stated but is fair to derive. The B-confirmation numbers are helpful guardrails. |
| Solvability | 4 | "We Didn't Start the Fire" familiarity is a major advantage but not mandatory (the song's Wikipedia article is accessible). The visual identification step is the hard bottleneck. |
| Elegance | 5 | The recursive application of the puzzle's own mechanism to the extracted phrase is genuinely elegant. The confirmation numbers prevent false positives without over-scaffolding. |
| Reading Reward | 4 | Billy Joel fans and music history buffs get real reward. The song's dense historical reference structure makes genuine familiarity advantageous. |
| Fun | 4 | Image identification is engaging. The final recursive step is a clean payoff. AFGHANISTAN landing in place is satisfying. |
| Confirmation | 5 | AFGHANISTAN is self-confirming: it is the next word in the song after RUSSIANS IN, and a solver who knows this immediately knows they are right. |
| **Total** | **26/30** | |

**Verdict: PASS (26)**

---

## GAMES MAGAZINE TIER

---

### Puzzle GM-01: The Planets
**Answer:** (crossword fill — PLUTO, NEPTUNE, JUPITER, MERCURY, SATURN, VENUS, EARTH, MARS)
**Source:** Crossword Labs

#### Mechanism Summary
A themed crossword with 9 entries on the topic of the solar system. All clues are direct definitions. No extraction step. Solving = filling the grid correctly.

#### Solving Notes
This is the definitional floor of puzzle construction. Every clue has exactly one answer, the answer is the end of the process, and completing the grid constitutes solving. There is no aha, no transformation, no extraction. The crossword constraint (crossing letters confirm each answer) is the only cognitive mechanism beyond vocabulary recall.

The strongest clue — "No longer officially considered a planet" for PLUTO — is the only moment of any wit, as it references the 2006 IAU reclassification. The other clues are pure definitions. The puzzle does what it advertises and nothing more.

**Principles fired:**
- **Snyder's Computer Test** — a computer can generate and solve this trivially. Fires.
- **No Over-Scaffolding** — there is no scaffolding because there is no mechanism to scaffold. The puzzle is entirely recall + letter-count. Fires in reverse: the puzzle has no aha to scaffold, which means it cannot violate this principle — but it also cannot satisfy it.
- **Interlock Not Independence** — crossing letters provide mild interlock, but each clue is solved independently with the grid providing only confirmation. Fires as concern.
- **Reading Reward** — no domain reward beyond elementary school science. Fires.
- **One Aha** — no aha. Fires as "no ahas is dull."
- **Riven Standard** — the puzzle does not IS anything about astronomy. It tests recall of planet names. Fires.

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Perfectly clear. Every clue is unambiguous. |
| Solvability | 5 | Any adult can solve this. |
| Elegance | 1 | No construction sophistication. Direct definitions throughout. The PLUTO clue is the single moment of wit. |
| Reading Reward | 1 | No domain reward beyond recall. |
| Fun | 2 | Pleasant completion exercise. No surprises. |
| Confirmation | 5 | When the grid is full and all crossings match, you are done and correct. |
| **Total** | **19/30** | |

**Verdict: FAIL (19)**

---

### Puzzle GM-02: Weather
**Answer:** (crossword fill — THUNDERSTORM, EVAPORATION, WEATHER, CONDENSATION, TORNADO, GROUNDWATER, HURRICANE, CLIMATE, HUMIDITY, METEOROLOGISTS, FLOOD)
**Source:** Crossword Labs

#### Mechanism Summary
A themed crossword with 11 entries on meteorology. All clues are full-sentence definitions. No extraction step. One mild self-referential elegance: the clue for WEATHER within a puzzle titled "Weather" is "The state of the air at a certain time and place."

#### Solving Notes
This puzzle is marginally better than The Planets on two counts: the clues are more elaborate (full sentences rather than fragments), and the self-referential WEATHER clue creates a mild moment of recognition. METEOROLOGISTS as an answer is satisfying for its length and specificity. Otherwise, the analysis is identical to The Planets: no aha, no extraction, direct definitions throughout.

**Principles fired:** Same as GM-01. The WEATHER self-reference is a minor Riven Standard success — the puzzle's title IS the topic and the topic IS in the grid. But this is structural coincidence, not construction intent, and it is mild.

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Clear, full-sentence definitions. Accessible vocabulary. |
| Solvability | 5 | Any adult with middle-school science knowledge can solve this. |
| Elegance | 2 | Slightly better cluing than The Planets. The WEATHER self-reference is a small moment of elegance. METEOROLOGISTS is a satisfying answer length. |
| Reading Reward | 1 | No domain reward beyond recall. |
| Fun | 2 | Slightly more engaging than The Planets due to clue length and METEOROLOGISTS. |
| Confirmation | 5 | Grid completion = done. |
| **Total** | **20/30** | |

**Verdict: FAIL (20)**

---

### Puzzle GM-03: Animals
**Answer:** (crossword fill — ZEBRA, GIRAFFE, KANGAROO, CAT, PIGS, CHEETAH, FROG, DOG, ELEPHANTS, LION, RHINO)
**Source:** Crossword Labs

#### Mechanism Summary
A themed crossword with 11 entries on common animals. All clues are clichéd or generic definitions. No extraction step.

#### Solving Notes
This is the absolute floor of the tier. "Man's best friend" for DOG and "Likes to chase mice" for CAT are crossword clichés that require no thought at all. "Hopping Australian marsupial" for KANGAROO is accurate but utterly generic. No clue demonstrates wit, misdirection, or surprise. The only decision a solver makes is letter count, which is provided.

**Principles fired:** All negative principles fire fully: Snyder's Computer Test (trivially generatable and solvable), no Aha, no Reading Reward, no Riven Standard.

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Maximally clear. |
| Solvability | 5 | Accessible to children. |
| Elegance | 1 | Clichéd clues throughout. No construction sophistication. |
| Reading Reward | 1 | No reward for any domain knowledge. |
| Fun | 1 | Mechanical completion. No moments of pleasure. |
| Confirmation | 5 | Grid completion = done. |
| **Total** | **18/30** | |

**Verdict: FAIL (18)**

---

### Puzzle GM-04: Logic Grid (Simple)
**Answer:** Alice=Top/Cat, Carol=Middle/Dog, Bob=Ground/Fish
**Source:** Canonical Games Magazine format

#### Mechanism Summary
A 3×3 constraint-satisfaction logic puzzle (three friends, three floors, three pets). Five clues eliminate possibilities until a unique solution is determined.

#### Solving Notes
The logic grid is a well-calibrated simple example of its type. The clues chain correctly: clue 2 (cat → top floor), clue 5 (Alice ≠ dog floor), clue 3 (Bob directly below dog owner) work together to place all three people uniquely. The solution is non-trivial: it requires at least two rounds of inference.

This is a legitimate puzzle type with genuine cognitive content — constraint satisfaction is real reasoning, not just recall. It is, however, fully mechanical: there is no aha, no surprise, no theme beyond "friends and pets." The mechanism is transparent from the start.

**Principles fired:**
- **No Computation Without Deduction** — the deduction IS the solving process here. No arithmetic required. Satisfied in letter.
- **Snyder's Computer Test** — a constraint-solver can solve this trivially. Fires.
- **One Aha** — no aha. The solution emerges from methodical elimination. "No ahas is dull." Fires.
- **Riven Standard** — the puzzle does not embody any specific domain. Friends and pets are props, not content. Fires.

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | The format is conventional and self-explanatory. The grid template makes tracking straightforward. |
| Solvability | 4 | Requires genuine multi-step reasoning. Not trivial to complete without the grid template. |
| Elegance | 2 | Correct logic but no construction artistry. The clues are efficiently minimal, which is a form of elegance within the type's conventions. |
| Reading Reward | 1 | No domain reward. Friends, floors, and pets are placeholders. |
| Fun | 2 | The chain of deductions has mild satisfaction. No surprise or delight. |
| Confirmation | 5 | Unique solution = done. All cells filled = correct. |
| **Total** | **19/30** | |

**Verdict: FAIL (19)**

---

### Puzzle GM-05: Word Search with Hidden Message
**Answer:** SECRET MESSAGE
**Source:** Canonical Games Magazine format

#### Mechanism Summary
Find 8 hidden words in a 6×8 grid. Remaining letters spell a two-word phrase. The answer is told to the solver ("The remaining letters, read left to right, top to bottom, spell a two-word phrase").

#### Solving Notes
The fatal flaw is stated in the puzzle notes: "the hidden message is told to the solver, not discovered." This puzzle tells the solver what will happen before the solver does it. The mechanism has no surprise, no misdirection, no discovery. The solver knows from the instructions that remaining letters spell something; they do not discover this independently.

The word-search step is pure visual scanning — mechanical, computer-generatable, and devoid of deduction. The hidden message extraction is signposted. The "SECRET MESSAGE" answer is thematically appropriate (it is itself a message that was hidden) but the self-referential loop is weak because the solver was told to expect it.

**Principles fired:**
- **Snyder's Computer Test** — fully computer-generatable and solvable. Fires completely.
- **No Over-Scaffolding** — the puzzle scaffolds away its own reveal by telling the solver what the extraction mechanism is. Fires.
- **One Aha** — no aha. Fires as "no ahas is dull."
- **Riven Standard** — word search does not embody any domain. Fires.
- **Surprise the Answer** — the solver is told to expect a hidden message; SECRET MESSAGE is the expected thing. Fires.

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 5 | Instructions are explicit. Cannot be misunderstood. |
| Solvability | 5 | Word search + sequential letter reading. Accessible to all ages. |
| Elegance | 1 | Mechanism is disclosed before solving begins. No construction artistry. |
| Reading Reward | 1 | No domain reward. |
| Fun | 1 | Visual scanning + expected payoff. No moments of surprise. |
| Confirmation | 5 | When words are found and remaining letters read left-to-right, completion is certain. |
| **Total** | **18/30** | |

**Verdict: FAIL (18)**

---

## MASTER SUMMARY TABLE

| # | Puzzle | Clarity | Solvability | Elegance | Reading Reward | Fun | Confirmation | Total | Pass/Fail |
|---|--------|---------|-------------|----------|----------------|-----|--------------|-------|-----------|
| B1-1 | Scicabulary | 3 | 4 | 4 | 3 | 4 | 3 | 21 | FAIL |
| B1-2 | People Watching | 4 | 4 | 5 | 5 | 5 | 3 | 26 | PASS |
| B1-3 | H2No | 2 | 3 | 3 | 3 | 3 | 2 | 16 | FAIL |
| B2-1 | Bridge Building | 3 | 3 | 5 | 4 | 4 | 4 | 23 | PASS |
| B2-2 | You're Telling Me | 4 | 4 | 4 | 3 | 4 | 4 | 23 | PASS |
| B2-3 | Dropypasta | 3 | 3 | 3 | 4 | 3 | 3 | 19 | FAIL |
| B3-1 | Information Relay | 4 | 4 | 5 | 3 | 5 | 4 | 25 | PASS |
| B3-2 | Front and Center | 5 | 4 | 5 | 3 | 5 | 5 | 27 | PASS |
| B3-3 | Characters | 4 | 4 | 5 | 3 | 4 | 5 | 25 | PASS |
| B3-4 | What's Next? | 4 | 4 | 5 | 4 | 4 | 5 | 26 | PASS |
| GM-01 | The Planets | 5 | 5 | 1 | 1 | 2 | 5 | 19 | FAIL |
| GM-02 | Weather | 5 | 5 | 2 | 1 | 2 | 5 | 20 | FAIL |
| GM-03 | Animals | 5 | 5 | 1 | 1 | 1 | 5 | 18 | FAIL |
| GM-04 | Logic Grid | 5 | 4 | 2 | 1 | 2 | 5 | 19 | FAIL |
| GM-05 | Word Search | 5 | 5 | 1 | 1 | 1 | 5 | 18 | FAIL |

---

## TIER AVERAGES

| Tier | Puzzles | Avg Clarity | Avg Solvability | Avg Elegance | Avg Reading Reward | Avg Fun | Avg Confirmation | Avg Total | Pass Rate |
|------|---------|-------------|-----------------|--------------|--------------------|---------|--------------------|-----------|-----------|
| Batch 1 (MIT Hunt) | 3 | 3.0 | 3.7 | 4.0 | 3.7 | 4.0 | 2.7 | 21.0 | 1/3 (33%) |
| Batch 2 (MIT Hunt) | 3 | 3.3 | 3.3 | 4.0 | 3.7 | 3.7 | 3.7 | 21.7 | 2/3 (67%) |
| Batch 3 (Huntinality/Teammate) | 4 | 4.3 | 4.0 | 5.0 | 3.3 | 4.5 | 4.8 | 25.8 | 4/4 (100%) |
| Games Magazine | 5 | 5.0 | 4.8 | 1.4 | 1.0 | 1.6 | 5.0 | 18.8 | 0/5 (0%) |
| **All 15** | **15** | **4.0** | **4.0** | **3.3** | **2.7** | **3.2** | **4.1** | **21.9** | **7/15 (47%)** |

---

## OBSERVATIONS

### Tier Separation
The Games Magazine tier (18.8 avg) falls clearly below the pass threshold (22), while Batch 3 (25.8 avg) sits comfortably above it. Batch 1 and 2 straddle the threshold (21.0 and 21.7), with quality determined by individual puzzle construction choices rather than tier-level factors.

### Dimension Profiles
The Games Magazine tier achieves maximum scores on Clarity, Solvability, and Confirmation — these dimensions do not distinguish quality. The separating dimensions are Elegance, Reading Reward, and Fun. Hunt-tier puzzles that score well on these three but poorly on Clarity or Confirmation (H2No, Scicabulary, Dropypasta) are the interesting failure cases: they have real construction ambition but insufficient polish at extraction.

### Principle Discrimination
Under Condition C2 (names + quotes only), the principles that fired most discriminatively were:

1. **Riven Standard** — cleanly separated People Watching (27) and Front and Center (27) from H2No (16). The "IS the domain" test is the sharpest single discriminator.
2. **Surprise the Answer** — RADAR and UNHEXING scored 5/5 on Confirmation partly because the answers are self-revealing. SHAMROCK and ENCAMPMENT scored 2–3 because the answers are arbitrary relative to mechanism.
3. **One Aha** — Dropypasta's six win conditions and Scicabulary's multi-step matching both triggered "three ahas is a slog" concerns and depressed Fun scores.
4. **Verify the Last Mile** — H2No's obscured extraction and Dropypasta's six different verification methods were the sharpest solvability deductions.
5. **Snyder's Computer Test** — fired on all five Games Magazine puzzles and partially on several Hunt puzzles, but alone insufficient to fail a Hunt puzzle; it requires companion principle failures.

### Notable Findings

**Front and Center (27/30)** is the highest-scoring puzzle. It satisfies Riven Standard, One Aha, Surprise the Answer, Blame the Player, and No Over-Scaffolding simultaneously, and its answer is self-confirming in the deepest possible way (RADAR is itself what the puzzle is about).

**H2No (16/30)** is the lowest-scoring Hunt puzzle. The extraction mechanism ("watch for differences") fails Blame the Player and Verify the Last Mile. The puzzle's strongest asset — the date-arithmetic co-location constraint — is undermined by opacity at the finish.

**The Games Magazine tier** does not fail because the puzzles are bad; they fail because the rubric is designed to measure puzzle-hunt construction values. These puzzles are appropriate and well-executed for their intended audience and format. Their failure is a category result, not a quality defect.

---

*Condition C2 evaluation complete. 15 puzzles scored. 7 pass, 8 fail. Threshold: 22/30.*
