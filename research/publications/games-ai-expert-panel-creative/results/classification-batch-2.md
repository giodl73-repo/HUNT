# Classification Panel — Real Puzzles Batch 2
**Source Hunt:** MIT Mystery Hunt 2023 — The Interesting Things Museum
**Panel:** Dan Katz, Thomas Snyder, Dana Young
**Classified:** 2026-02-28

---

## Bridge Building
**Source:** MIT Mystery Hunt 2023 | **Answer:** PROTEINPOWDER

| Dimension | Classification | Notes |
|-----------|---------------|-------|
| Hunt Level | MIT/Elite | Requires fluency in two independent technical domains: Hashiwokakero grid logic AND organic chemistry (polymer backbone, ionic charges, carbon valence rules). Neither alone would reach this answer — the aha is at the seam between them. |
| Mechanism | Logic-Constraint + Knowledge-extraction (Hybrid) | Phase 1: constraint logic (Hashi rules force a unique bridge configuration). Phase 2: knowledge extraction (read solved grid as molecular structure via chemistry domain knowledge). |
| Domain Load | 3 — Essential | Solving the Hashi grid is doable without chemistry knowledge. But reading the result as a polymer backbone — interpreting (+/−) superscripts as ionic charges, applying carbon valence (max 4 bonds), ignoring hydrogen — requires genuine chemistry. Domain knowledge is not just helpful; it is the extraction mechanism. |
| Aha Clarity | 3 — Single clean aha | The aha is the reframe: "the solved logic puzzle IS a molecular diagram." The bridge-building metaphor doubles as a chemistry metaphor (chemical bonds are bridges). Title, logic genre, and extraction are unified by a single insight. |
| Paper Suitability | Yes | Exemplary two-phase hybrid. Phase boundary is clean. Domain layers are separable for analysis. Canned hints are well-staged and reveal the intended solving path clearly. |
| Quality Signal | 3 — Exceptional | The title is doing triple duty (puzzle genre, construction activity, molecular bonding). The thematic unity is structural, not decorative. |

**Katz note:** The hunt contract is honored — solvers who bring both logic skill and chemistry knowledge get a faster, more satisfying solve. Solvers who have only one domain get the other half as a learning experience. Neither half is wasted. The puzzle earns its slot by requiring a specific pairing of disciplines that no other puzzle in the round replicates.

**Snyder note:** The Hashi logic is the entry point and it is designed, not discovered — the ionic charge superscripts constrain the node identities before chemistry begins, giving the logic phase a clean unique solution. The transition from grid to molecule is the pivot point: does the constructor test it? The canned hints suggest yes — "bond types needed for the polymer backbone" implies the solve path was traced. This is craftsmanship.

**Young note:** The visual reframe — same diagram, two readings — is the puzzle's best moment. A Hashi grid that resolves into a molecular structure is a moment of visual revelation: the solver literally sees the structure change registers. The title is perfectly dry. The extraction is earned because it reframes what you were looking at, not just what you do with it.

**Use for ablation:** Yes — clean two-phase hybrid with separable domain layers; ideal for testing whether AI-generated puzzles can achieve genuine mechanism integration rather than juxtaposition.

---

## Natural Transformation
**Source:** MIT Mystery Hunt 2023 | **Answer:** TANGRAM

| Dimension | Classification | Notes |
|-----------|---------------|-------|
| Hunt Level | MIT/Elite | Meta-level deduction: solvers must reverse-engineer the rule system before applying it. No single entry point — constraint propagation across diagrams is required. The final extraction (gram vs. g distinction + mathematical expression as word) is itself a puzzle-within-the-puzzle. |
| Mechanism | Logic-Constraint + Transformation (Hybrid) | Solvers reconstruct transformation rules from examples (logic), then apply rules (transformation), then perform a linguistic/mathematical extraction. Three distinct mechanism layers. |
| Domain Load | 2 — Helpful | Category knowledge (animal collective nouns, mathematical vocabulary, what "derivative" means in non-calculus contexts) helps but is partially inferrable from the diagram constraints. The rules are deducible from the example diagrams even without prior domain knowledge. |
| Aha Clarity | 2 — Aha present but not central | There is no single aha moment. The experience is progressive: each arrow type cracked is a local aha. The final extraction (mathematical expression that sounds like a word → TANGRAM) is the culminating moment, but it requires prior aha accumulation. Not a single clean insight. |
| Paper Suitability | Yes | Excellent example of rule-induction mechanism. Three-layer construction (learn rules, apply rules, extract answer) is analytically distinct from standard lookup or wordplay puzzles. |
| Quality Signal | 3 — Exceptional | The title "Natural Transformation" is a category theory term — a transformation between functors — which is itself a kind of arrow-system with consistent rules. The puzzle's content (arrows between sets with consistent rules) is exactly what a natural transformation describes in mathematics. The thematic embedding runs deeper than decoration. |

**Katz note:** This puzzle is a small hunt within a hunt — the first diagrams function as an onboarding level before the actual puzzle begins. The solver's contract includes "you must build the ruleset yourself." This is legitimate MIT-tier design: the instructions are in the puzzle, not before it. The constraint-propagation phase prevents short-circuiting.

**Snyder note:** The entry point is not a single designed moment — it is a system, and the solver enters where they find traction. This has the risk Snyder flags: "two routes to the same answer" at the rule-induction stage. If a solver incorrectly infers an arrow rule from the examples but it still produces consistent outputs downstream, the puzzle has a soundness problem. The canned hints ("try solving later diagrams first") suggest this ambiguity was known.

**Young note:** The visual grammar of the arrow diagrams is precise and consistent — same element type, same meaning. Dana would note that the diagram layout is doing real communicative work: the key diagrams must visually precede the application diagrams, and the flow is readable. The extraction is the weakest moment: "gram versus g" and "mathematical expression that sounds like a word" require a specific interpretive leap that may not feel earned without the hints.

**Use for ablation:** Yes — rule-induction mechanism is rare in AI-generated puzzles; excellent test case for whether AI can construct internally consistent logical grammars rather than surface wordplay.

---

## Extrasensory
**Source:** MIT Mystery Hunt 2023 | **Answer:** DEODORANT

| Dimension | Classification | Notes |
|-----------|---------------|-------|
| Hunt Level | Standard Hunt | A themed crossword with letter extraction. The core mechanism (fill themed crossword, extract anomalous letters) is accessible and well-understood. The theme content (six senses) is clever but not opaque. MIT placement reflects density/content knowledge, not mechanism novelty. |
| Mechanism | Wordplay + Knowledge-extraction | The crossword clues require domain knowledge (MIT-specific references, specific cultural knowledge). The extraction (identify letters that "don't belong") is a standard post-crossword mechanism. |
| Domain Load | 2 — Helpful | MIT-specific clues (Course 9-5, popular MIT course) favor MIT solvers. Several clues (Highlander, Sanhedrin, AOP) require specific cultural/religious/pop-culture knowledge. Domain knowledge accelerates solve but most clues are reachable with general knowledge. |
| Aha Clarity | 2 — Aha present but not central | The "six senses" theme (the clues reference extra-sensory phenomena) plus the extraction mechanism ("extrasensory" = perceiving what doesn't belong) is a satisfying thematic link. But it functions more as retrospective elegance than a solving pivot. |
| Paper Suitability | Partial | The puzzle is mechanically competent but represents a well-established construction category. It adds little mechanically novel. Useful as a baseline or comparison point: standard crossword-with-extraction at the MIT difficulty level. |
| Quality Signal | 2 — Standard | The thematic link between puzzle title, clue theme, and extraction mechanism is present and works. Construction appears clean. The reordered clue numbering (19, 7, 11, 4...) adds an unusual presentational layer — presumably meaningful to the extraction, though the mechanism description doesn't fully clarify why. |

**Katz note:** This puzzle's slot is justified — it provides a more accessible entry point in a difficult hunt. The crossword mechanism is immediately parseable to any solver. The extraction is not novel but is competent. It passes the slot-justification test because "themed crossword with elegant title-extraction link" is a legitimate hunt function, not padding.

**Snyder note:** The reordered clue numbering is either thematically motivated or an editorial mistake. If motivated — if the numbering sequence encodes something, or if solving in a non-standard order is part of the mechanism — this is an interesting construction choice. If it is simply the order they appear on the page (arbitrary), it adds noise without design. The difference between these readings matters significantly for construction evaluation.

**Young note:** The extraction instruction "identify letters that don't belong" depends on the solver seeing the pattern — which requires having solved enough of the crossword to perceive the anomaly. Dana would ask: is the anomaly visually immediate when the grid is filled, or does it require careful cell-by-cell comparison? The experiential quality of that moment determines whether the puzzle lands. A theme that holds through fill and extraction is well-built; one that requires analytical post-processing after the grid is done is less satisfying.

**Use for ablation:** Conditional — useful as a baseline representative of the standard crossword-with-extraction category. Not a strong positive example; better as the control condition for mechanism complexity comparisons.

---

## A Trip to the Museum
**Source:** MIT Mystery Hunt 2023 | **Answer:** REPLICA

| Dimension | Classification | Notes |
|-----------|---------------|-------|
| Hunt Level | MIT/Elite | Physical location requirement is the defining feature. Five sub-puzzles embedded in real exhibits at the MIT Museum during Mystery Hunt weekend. This is site-specific puzzle design — a genre that does not exist below the MIT/Elite level. |
| Mechanism | Physical-Location (primary) + Logic-Constraint + Transformation | Each sub-puzzle uses a distinct mechanism (loom manipulation, cell-logic on leaf triangulation, counting additions to Last Supper, letter-tracking through starshade animation, partial-order reconstruction to extract path letters). The final extraction (five one-word answers forming a cluephrase) is standard. |
| Domain Load | 1 — Not required | Domain knowledge is not the barrier. Physical presence is. The sub-puzzle mechanisms are solvable on-site with the exhibits as primary reference material. Cultural knowledge (Last Supper as a reference point) helps but is broadly accessible. |
| Aha Clarity | 1 — No clear singular aha | Five sub-puzzles, each with its own local aha (shaded cells form letters, path tracing extracts letters, etc.). The final cluephrase is a satisfying synthesis, but the experience is segmented rather than unified under a single insight. |
| Paper Suitability | Partial | The physical-location mechanism is important to document but essentially non-reproducible for AI analysis — no amount of text description replicates the experience of standing at an exhibit. Useful as a classification anchor for "Physical-Location" category and for discussing the limits of text-reproducible puzzle analysis. |
| Quality Signal | 3 — Exceptional | The integration of puzzle design into actual museum exhibits is ambitious and executed with specific, tested sub-puzzles. The sub-puzzle variety (five distinct mechanisms across three exhibit spaces) is evidence of genuine construction effort. The answer REPLICA — which is what museum artifacts often are — achieves the "answer names the experience" standard. |

**Katz note:** This is a puzzle that justifies its location in a hunt that takes place at MIT, in January, with physical events at campus buildings. It could not exist outside its context. The sub-puzzle variety prevents the physical scavenger hunt from becoming a search problem: each exhibit stop requires a different cognitive operation. This is structural variety deployed correctly.

**Snyder note:** The sub-puzzle in Part 2 (cell logic on triangulated leaf) is closest to Snyder's domain — a logic constraint applied to a physical artifact. The question is whether the cell logic has a unique solution independent of which leaf the solver picks. If the outline matching step is ambiguous, the logic puzzle downstream is unreliable. Construction soundness of a physical puzzle requires additional verification steps that text-based puzzles do not.

**Young note:** The answer REPLICA is exemplary. In a museum about "interesting things," the answer naming the relationship between the exhibit and the real artifact is thematically exact. Dana would note that this puzzle requires the solver to inhabit the museum — not just visit it — and the answer captures that the solver has been encountering replicas all along. The moment of recognition reframes the entire experience. This is the standard she would want to hold other puzzles to.

**Use for ablation:** No — physical-location mechanism is not reproducible in text format. Useful only as a category boundary example (showing what the "Physical-Location" classification means at the extreme). Cannot be used in AI-panel ablation without stripping the mechanism that defines the puzzle.

---

## Street Smarts
**Source:** MIT Mystery Hunt 2023 | **Answer:** ROADTOHELL

| Dimension | Classification | Notes |
|-----------|---------------|-------|
| Hunt Level | MIT/Elite | Physical presence required at a specific Boston location (Commonwealth Avenue Mall). The puzzle is time-locked to Mystery Hunt weekend (snow and leaves may obscure plaques). Location-specific knowledge (Boston geography, the specific donors named) cannot be obtained from memory — physical verification is mandatory. |
| Mechanism | Physical-Location (primary) + Knowledge-extraction | The mechanism is fundamentally a location-based scavenger hunt: identify 29 real-world physical plaques/benches/trees from clues, then use block position and positional index to extract letters. The clue-writing (puns, misdirections, historical references) layers wordplay over the physical search. |
| Domain Load | 2 — Helpful | Boston history, the specific named figures on Commonwealth Ave (known abolitionists, city figures, Taylor Swift song titles on plaques), and street geography knowledge helps. But the physical plaques are the authoritative source — domain knowledge reduces search time, it doesn't substitute for presence. |
| Aha Clarity | 2 — Aha present but not central | The extraction mechanism (block number / position index spelled out in each clue) is elegant once discovered — the clue format encodes its own indexing. ROADTOHELL as an answer for a puzzle about street plaques is a satisfying thematic pun. But neither moment is a single clean aha that restructures the puzzle; they are cumulative satisfactions. |
| Paper Suitability | No | Physical presence is mandatory. No solver can complete this puzzle from a text description — they must be in Boston, at the Mall, in January, possibly moving snow to read plaques. The puzzle is not text-reproducible in any meaningful sense. Category value only: anchor for Physical-Location at the MIT level. |
| Quality Signal | 3 — Exceptional | The clue writing is dense with layered references — historical figures, Taylor Swift songs, Seth Carr's role, mathematicians, the "YOLO" paraphrase of a plaque. This is skilled crossword-clue craft applied to a scavenger hunt format. The indexing scheme (n/m notation in each clue encoding block number and position) is elegant infrastructure. |

**Katz note:** The clue writing doubles as entertainment — reading the clues is pleasurable independent of whether you can solve them. The "(3/3)" notation embedded in each clue is the best kind of hunt infrastructure: it is hiding in plain sight. The solver who notices it mid-hunt has a structural aha that retroactively improves every clue they've already read. This is the mechanism elegance Katz associates with MIT-quality construction.

**Snyder note:** The 29-clue set has no entry point problem — any clue that a solver can identify provides a letter, and the 29-letter answer presumably has redundancy built in. Unlike a logic constraint puzzle where a single wrong step cascades, here the mechanism is parallelizable and error-tolerant. From a construction standpoint, the clues must each have exactly one valid match in the physical world, and the positional indices must extract correctly. Field verification — actually visiting the Mall — is the only way to certify this.

**Young note:** The answer ROAD TO HELL for a puzzle conducted on a Boston street is funny, specific, and thematically exact. Dana would ask: does the experience of walking the Mall solve the puzzle, or does the puzzle give you a reason to walk the Mall differently than you would otherwise? The second is better, and based on the clue structure — which makes the plaques visible in a new way — this puzzle achieves it. The extraction adds meaning rather than just process.

**Use for ablation:** No — physical-location requirement makes AI panel evaluation impossible at the mechanism level. Text description can classify it but not reproduce the solve experience.

---

## Dropypasta
**Source:** MIT Mystery Hunt 2023 | **Answer:** VINDICATIONISLAND

| Dimension | Classification | Notes |
|-----------|---------------|-------|
| Hunt Level | Standard Hunt | The dropquote mechanism is a known puzzle format (fill missing letters from a "falling letters" grid). The Super Smash Bros. domain knowledge layer makes it MIT-hunt appropriate, but the core mechanism is accessible. The extraction (stage win conditions for character matchups) elevates it toward MIT tier. |
| Mechanism | Knowledge-extraction + Transformation (Hybrid) | Phase 1: fill dropquotes (standard format, requires recognizing SSBU quotes). Phase 2: character matchup determination by stage-specific rules (alphabetical, release date, tier list position, Kirby inhale damage, CSP proximity, weight). Phase 3: letter extraction from winners. |
| Domain Load | 3 — Essential | Two domain layers, both essential: (1) Super Smash Bros. quote recognition — without knowing the source material, the dropquote fill is impossible. (2) SSBU game mechanics — tier list standing, Kirby inhale damage values, character weights, CSP positions, game release dates — these are not inferrable without domain knowledge. |
| Aha Clarity | 2 — Aha present but not central | The stage win conditions are individually clever (Battlefield = alphabetical, Dream Land = earlier game, Final Destination = tier list, etc.). Each condition is its own local aha. VINDICATIONISLAND as an answer for a Smash Bros. puzzle may have thematic resonance within the hunt context (museum → island). But there is no single restructuring insight — it is a sequence of domain-knowledge queries. |
| Paper Suitability | Yes | The two-domain structure (text fill + game mechanics lookup) is analytically interesting. The stage win conditions are a clean example of multi-criteria extraction: each stage uses a different dimension of the same game's knowledge base. This is worth documenting as a "domain-stratified extraction" mechanism type. |
| Quality Signal | 2 — Standard | The construction is competent. The stage-condition variety (six different rule types across six stages) prevents the extraction from becoming rote. The SSBU genre choice produces specific, verifiable answers — tier lists and weights are documented, making the puzzle certifiably fair. But the mechanism juxtaposition (dropquote + character tournament) has a seam: filling the dropquotes and the stage matchups feel like two adjacent puzzles rather than one integrated one. |

**Katz note:** The mechanism seam is the structural concern. A dropquote that yields characters, and then a separate stage-tournament system that processes those characters — these are two puzzle ideas occupying one puzzle slot. The connection (Smash Bros. quotes → Smash Bros. character matchups) is topical rather than mechanical. Katz would ask whether the extraction earns the fill, or whether the fill is just a long way to get a list of character names that could have been provided directly.

**Snyder note:** The six stage win conditions are the puzzle's most interesting construction decision. Each condition tests a different kind of knowledge: alphabetical order tests the fill itself; release date and weight test factual game knowledge; tier list tests community meta-knowledge; Kirby inhale damage tests mechanical game knowledge; CSP proximity tests visual game knowledge. The variety is intentional and demonstrates range. Snyder's concern: are all six conditions independently certified as having unique outcomes for each matchup? A tie in any condition (two characters equal weight, two on same CSP column) creates an ambiguity.

**Young note:** "Dropypasta" is a clever title: dropquote + pasta (copypasta, and SSBU has famous community copypastas like the "smash hit" speech). The title earns its pun. The extraction experience — determining "who wins" each stage — imports a tournament logic that SSBU players find natural. For the target audience, this is thematically immersive. For solvers outside the domain, it reads as pure mechanical lookup. Dana would flag this as a difficulty-curve honesty problem: the puzzle is accessible to SSBU players and nearly opaque to everyone else, with no gradient between them.

**Use for ablation:** Yes — the two-phase domain-stratified extraction is analytically useful. The mechanism seam (topical rather than mechanical integration) is exactly the kind of construction flaw the ablation study should be able to detect.

---

## You're Telling Me
**Source:** MIT Mystery Hunt 2023 | **Answer:** CARBONSINK

| Dimension | Classification | Notes |
|-----------|---------------|-------|
| Hunt Level | MIT/Elite | The "Shrimp Fried Rice" meme format is a known internet reference, but deploying it across 25+ compound-word clues with consistent format and a thematic extraction reaching an environment-themed answer (CARBONSINK) requires solver fluency in compound-word parsing, meme culture, and environmental vocabulary. The clue writing quality is elite. |
| Mechanism | Wordplay (primary) + Transformation | Each clue re-parses a compound word or phrase by applying the meme logic (read the compound differently). This is pure wordplay — no domain knowledge required beyond English compound word familiarity and the meme format. The extraction mechanism (from solved compounds to CARBONSINK) is the transformation layer. |
| Domain Load | 1 — Not Required | The mechanism is fully accessible if you understand the meme format (which the puzzle presumably provides as context). Each compound word clue is solvable from general English vocabulary. CARBONSINK is an environmental science term but is deducible from CARBON + SINK as separate words. |
| Aha Clarity | 3 — Single clean aha | The aha is the meme format recognition: once a solver sees that each clue describes a compound word that has been "read wrong," all 25+ clues become immediately approachable using the same single insight. The format is self-teaching from the first solved example. CARBONSINK as the answer (which IS a compound word that could be read as "carbon" + "sink") retroactively confirms the theme. |
| Paper Suitability | Yes | The meme-as-mechanism approach is analytically significant: a single cultural format (the Shrimp Fried Rice meme) generates a complete puzzle structure with consistent grammar. This is a case study in format-as-mechanism. The 25-clue scale tests whether the mechanism holds at length — the solver's relationship to the format evolves across the solve. |
| Quality Signal | 3 — Exceptional | The clue writing is consistently high ("Are you telling me you want me to make a timepiece panic?" for ALARM CLOCK; "Is that like a blunt weapon for beating Canada or France up?" for presumably FRENCH PRESS or similar). The thematic coherence extends to the extraction answer — CARBONSINK is itself a compound that can be parsed the "wrong" way (carbon that sinks, versus a place carbon sinks to). The self-referential elegance is a mark of elite construction. |

**Katz note:** This puzzle has structural elegance: it teaches itself in one example and sustains the mechanism across 25+ entries without degrading. The solvers' relationship to the format evolves — early entries feel like puzzle-solving, later entries feel like the solver is the one doing the meme. That progressive shift is what separates a puzzle from a worksheet. The answer CARBONSINK is also a "Shrimp Fried Rice" compound if you think about it, which makes the ending feel like the puzzle winking at itself.

**Snyder note:** The construction challenge here is calibration: 25+ clues must each have exactly one satisfying compound-word answer, and the misreading must be precisely specific enough to point to it. Too broad a misdirection and multiple answers fit; too narrow and the clue reads like a straight definition. This is clue-craft at a high level. The enumeration grid (part of speech labels alongside word lengths) provides a useful constraint layer that narrows the answer space without giving it away — intelligent scaffolding.

**Young note:** The humor is doing real work. Each clue is funny on its own terms ("I'm pretty sure stones are made of elements too dense for fusion" for ROCK FUSION or similar; "Isn't scaring a reptile animal abuse?" for LIZARD FRIGHTENING or SNAKE STARTLING). A puzzle that makes you laugh while you solve it is creating an experiential register that pure logic puzzles cannot achieve. Dana would note that this is the mechanism being thematically exact: the meme format is inherently comedic, so the puzzle's affect is correct to its form. The final answer being environmental-themed reads as a tonal pivot — the humor lands, then the answer is serious. Whether that tonal shift is intentional or incidental determines whether it is elegance or accident.

**Use for ablation:** Yes — strong positive example. The meme-format mechanism is an aha-first structure (understand the format, then all entries are solvable) that tests AI generation on a very different axis than knowledge retrieval or logic constraint. Excellent test of whether AI can generate consistently in a comedic wordplay register.

---

## Apples Plus Bananas
**Source:** MIT Mystery Hunt 2023 | **Answer:** HARVESTFESTIVAL

| Dimension | Classification | Notes |
|-----------|---------------|-------|
| Hunt Level | MIT/Elite | Cryptarithmetic at the MIT level: the constraint "total cannot be reduced" (sum is prime) over PLU codes (real-world 4–5 digit grocery barcode numbers) requires both the solve-the-constraint logic and the domain knowledge of what PLU codes are and which produce varieties they map to. Variety-name extraction adds a third layer. |
| Mechanism | Logic-Constraint + Knowledge-extraction (Hybrid) | Phase 1: identify produce from emoji, look up real PLU codes. Phase 2: satisfy the primality constraint across two lists of 18 emoji sequences. Phase 3: extract letters from variety names of correctly assigned produce based on positional or frequency counts. |
| Domain Load | 3 — Essential | PLU codes are real-world grocery infrastructure knowledge — specific 4–5 digit numbers assigned by the International Federation for Produce Standards. Without knowing (or looking up) that a Gala apple is PLU 3283, a Fuji apple is PLU 4131, etc., the puzzle cannot be entered. The emoji must be identified as specific varieties, not just categories. |
| Aha Clarity | 2 — Aha present but not central | The aha is the wordplay on "cannot be reduced" (supermarket-receipt language that means "prime" in the mathematical sense). That's a single, elegant pun. But it is a setup aha, not a solving aha — understanding it tells you the constraint, not how to satisfy it. The primality constraint is an optimization problem without a clean logical forced path. |
| Paper Suitability | Yes | Analytically rich: (1) emoji-as-lookup-key is a novel encoding layer; (2) primality constraint over domain-specific codes is an unusual hybrid of real-world lookup and mathematical constraint; (3) the variety-name extraction adds a third layer. This is worth documenting as a "multi-layer constraint + extraction" pattern with domain-essential lookup at the base. |
| Quality Signal | 3 — Exceptional | The flavor text pun ("your total cannot be reduced") is elegant and immediately parseable to anyone who has scanned groceries. The erratum (missing enumeration 7 8) suggests the puzzle was complex enough to generate an error in production — not a negative, but evidence of ambition. The thematic anchor (produce emoji representing specific grocery varieties with real barcodes) is a completely original conceit. |

**Katz note:** The primality constraint is mathematically precise but computationally messy — satisfying a primality condition over sums of 4–5 digit numbers across 36 emoji total is not a clean logical forced path. This risks the "computation without deduction" failure mode. If the intended solving path is "look up PLU codes, sum them, check primality," that is calculation, not insight. If the intended path is "the primality constraint eliminates most PLU code assignments, forcing specific varieties," that is deduction. The puzzle's quality depends entirely on which path is intended and whether it actually works.

**Snyder note:** The construction question is whether the primality constraint produces a unique solution. Given that PLU codes for produce can vary by variety (Fuji vs. Gala vs. Honeycrisp all different PLU codes for apple), and that the same produce emoji could represent different varieties, the constraint must eliminate all but one assignment. If it does: exceptional construction. If it does not — if multiple PLU assignments satisfy the primality condition — the puzzle has a uniqueness failure at its core. This is not detectable without actually computing it.

**Young note:** The emoji are the visual entry point — 16 produce emoji in two identical lists are visually playful and immediately recognizable as a grocery receipt aesthetic. The flavor text ("You've picked a variety of produce. Please scan your items") commits fully to the register. The answer HARVEST FESTIVAL (a seasonal produce event) closes the thematic loop without being reductive about it. Dana would note that this puzzle inhabits its world — the grocery checkout — completely, from flavor text through mechanism through answer. That is her standard: the mechanic is the world, not placed inside it.

**Use for ablation:** Yes — multi-layer hybrid with domain-essential lookup at base and mathematical constraint as extraction criterion. Tests AI generation on a register where pun, domain knowledge, and mathematical structure must all co-exist. The potential uniqueness question (does primality constrain uniquely?) is also a useful ablation test point.

---

## Summary Table

| Puzzle | Hunt Level | Mechanism | Domain Load | Aha Clarity | Paper Suitability | Quality Signal | Ablation |
|--------|-----------|-----------|-------------|-------------|------------------|---------------|---------|
| Bridge Building | MIT/Elite | Logic-Constraint + Knowledge (Hybrid) | 3 | 3 | Yes | 3 | Yes |
| Natural Transformation | MIT/Elite | Logic-Constraint + Transformation (Hybrid) | 2 | 2 | Yes | 3 | Yes |
| Extrasensory | Standard Hunt | Wordplay + Knowledge | 2 | 2 | Partial | 2 | Conditional |
| A Trip to the Museum | MIT/Elite | Physical-Location + Logic + Transformation | 1 | 1 | Partial | 3 | No |
| Street Smarts | MIT/Elite | Physical-Location + Knowledge | 2 | 2 | No | 3 | No |
| Dropypasta | Standard Hunt | Knowledge + Transformation (Hybrid) | 3 | 2 | Yes | 2 | Yes |
| You're Telling Me | MIT/Elite | Wordplay + Transformation | 1 | 3 | Yes | 3 | Yes |
| Apples Plus Bananas | MIT/Elite | Logic-Constraint + Knowledge (Hybrid) | 3 | 2 | Yes | 3 | Yes |

---

## Panel Notes — Batch-Level Observations

**Katz:** The batch is heavily weighted toward MIT/Elite — 6 of 8 puzzles require either physical presence or domain knowledge at the essential level. The two physical-location puzzles (A Trip to the Museum, Street Smarts) are incommensurable with text-based analysis and should be excluded from the paper's main corpus. Of the remaining six, Bridge Building and You're Telling Me are the strongest examples of the aha-mechanism alignment the paper is investigating. Dropypasta has a mechanism seam that makes it a useful negative example.

**Snyder:** Four puzzles in this batch are true hybrid mechanisms (Bridge Building, Natural Transformation, Dropypasta, Apples Plus Bananas). The quality differentiator between the strong hybrids (Bridge Building, Apples Plus Bananas) and the weaker one (Dropypasta) is whether the two mechanism phases are integrated or juxtaposed. Integrated: the first phase creates the raw material that only the second phase can read (molecular diagram from Hashi grid; prime constraint over PLU codes). Juxtaposed: the first phase produces a list that the second phase processes independently (character names from dropquotes, then character matchups). This distinction should be operationalized in the paper's mechanism taxonomy.

**Young:** The best answers in this batch are REPLICA and CARBONSINK. Both name the experience rather than just labeling the content. ROADTOHELL and HARVESTFESTIVAL are thematically apt but arrive at the end rather than reframing what came before. PROTEINPOWDER and VINDICATIONISLAND feel arbitrary — neither is inevitable from its puzzle's logic. DEODORANT and TANGRAM are somewhere between: functional, but not revelatory. Answer inevitability should be a scored dimension in the paper.
