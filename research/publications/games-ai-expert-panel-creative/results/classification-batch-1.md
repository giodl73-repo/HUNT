# Classification Panel — Batch 1
**Panel:** Dan Katz, Thomas Snyder, Dana Young (simulated)
**Source:** MIT Mystery Hunt 2023 — The Interesting Things Museum
**Date:** 2026-02-28
**Classifier note:** Puzzles assessed structurally and mechanically WITHOUT solving. No answers were derived from inspection; published answers are used only as reference.

---

## Scicabulary
**Source:** MIT Mystery Hunt 2023 | **Answer:** SHAMROCK

| Dimension | Classification | Notes |
|-----------|---------------|-------|
| Hunt Level | MIT/Elite | The core insight — that portmanteau words have two distinct halves, each of which can be extracted and swapped with the opposite half of another portmanteau — is genuinely novel and requires discovery before any solving can begin. The mechanic is not hinted strongly; the "aha" is the realization of the reversal structure. Indexing is clean and downstream of the insight. |
| Mechanism | Wordplay | Portmanteau deconstruction and letter-index extraction; pure wordplay throughout. |
| Domain Load | 1 | No domain expertise required. The portmanteau words themselves are everyday vocabulary items. |
| Aha Clarity | 3 | One central aha: recognizing that jargon terms are formed from the *wrong* halves of two different portmanteaus. Everything follows from that. The mechanism IS the aha. |
| Paper Suitability | Yes | Full text reproducible. 24 terms with indices and 24 definitions with enumerations render completely in a table. Self-contained. |
| Quality Signal | 3 | Elegant construction. 24 entries all consistent with the mechanism. Definitions are clever and charming without being obtuse. Hint structure is minimal but precisely calibrated. |

**Katz note:** The structural elegance here is that there's exactly one kind of thing the solver has to figure out, and once they figure it out the whole puzzle unrolls. The 24-entry scale is proportionate — enough entries to feel satisfying, not so many that it becomes a slog. The matching step (terms to definitions) is a necessary friction that prevents mechanical throughput; you can't just grind through this. Strong slot-justification.

**Snyder note:** The mechanism has a uniqueness property he'd demand: you cannot complete the matching without understanding the portmanteau reversal logic. There are no shortcuts through enumeration alone — the two-word phrase enumerations confirm but don't unlock. The index numbers are load-bearing; none appear to be decorative. This was finished.

**Young note:** The definitions are delightful — "A snake that can read your mind" (psycho + anaconda = PSYCHONDA, presumably) creates genuine whimsy. The flavor text earns its place: "Is there a way to match these up?" is the exact amount of scaffolding needed. The answer SHAMROCK feels slightly arbitrary at the end (it names nothing the solver was doing), but the journey compensates. Thematic integration with the museum setting is light; would ask whether this fits the round context.

**Use for ablation:** Yes — cleanest single-aha example in the batch. Ideal for testing whether a reviewer can identify mechanism type vs. domain type without solving.

---

## Museum Rules
**Source:** MIT Mystery Hunt 2023 | **Answer:** TOTALLYUNACCEPTABLE

| Dimension | Classification | Notes |
|-----------|---------------|-------|
| Hunt Level | MIT/Elite | Requires genuine US legal/regulatory trivia research — knowing obscure jurisdiction-specific laws — combined with a second-level extraction step (state's "highest power" = governor or supreme court, yielding a specific name to index into). Neither step is mechanical; both require domain investigation. |
| Mechanism | Knowledge extraction | Identify forbidden-activity jurisdictions → extract from highest legal authority name. Two-step knowledge chain. |
| Domain Load | 3 | Essential. Non-expert cannot identify obscure jurisdiction-specific laws without research. Even with research, the laws described are deliberately arcane (the humor of the puzzle depends on their obscurity). |
| Aha Clarity | 2 | Two ahas: (1) recognizing the activities map to specific prohibited jurisdictions, (2) recognizing "highest power" means the named authority (governor/supreme court chief justice) from whose name you extract. Neither is as singular as a pure single-aha puzzle; they're sequential. |
| Paper Suitability | No | Core mechanic requires the illustrated page-flipping book interface. The activities are delivered via images in the interactive book — not text. In text format, the puzzle loses its primary content delivery mechanism. A text transcript might approximate it, but the puzzle as designed cannot be fairly reproduced in markdown. |
| Quality Signal | 2 | Well-constructed for what it is, but the interactive delivery is load-bearing in ways that make it format-dependent. The extraction (letter from a specific official's name) is somewhat indirect and feels like it could produce ambiguity depending on title conventions. |

**Katz note:** This is structurally a two-phase knowledge extraction puzzle with an interactive wrapper. The "highest power" extraction is the kind of step that can break for teams who know the jurisdictions but don't think of the authority naming convention — a potential mettleneck if the phrase isn't parsed correctly. The title TOTALLYUNACCEPTABLE is a clever payoff. Slot justification is fine — the museum setting earns the tourist-rule-breaking frame.

**Snyder note:** The interactive book interface is doing structural work — the page presentation presumably sequences or contextualizes the activities in ways that a flat list would not. This is a case where the delivery medium IS part of the puzzle design. That's a construction choice he respects, but it means the text representation loses integrity. The extraction step (from official names) requires verifying that there is exactly one valid reading for each state, which he would demand to see proven in testing.

**Young note:** The tourist frame is charming and the "everything they're doing is illegal somewhere" setup creates genuine discovery delight. The interactive flip-book is a strong visual choice — the puzzle inhabits its world (museum exhibit about rules). However, the extraction from authority names feels like it exits the tourist-rule frame abruptly; there's a thematic seam where the world-building ends and the cipher begins. She'd push to make that step feel more diegetic.

**Use for ablation:** Conditional — useful for testing Paper Suitability assessment specifically, but the No rating on that dimension means it can't serve as an example of paper-reproducible hunt content.

---

## People Watching
**Source:** MIT Mystery Hunt 2023 | **Answer:** HINDWING

| Dimension | Classification | Notes |
|-----------|---------------|-------|
| Hunt Level | MIT/Elite | Requires knowing bird calls as rendered in human mnemonic form, visual field marks translated into human descriptors, and the IBP 4-letter banding code system — three distinct knowledge layers all functioning simultaneously. The encoding of bird calls as human speech is elegant and unusual. |
| Mechanism | Knowledge extraction | Bird species identification from anthropomorphized behavioral/visual/vocal cues → IBP alpha-code extraction. |
| Domain Load | 3 | Essential. Bird call mnemonics, field mark vocabulary, and the IBP code system are all specialist knowledge. A non-birder cannot complete this without substantial research, and even with research the call-to-mnemonic mappings require field guide familiarity. |
| Aha Clarity | 2 | The aha is recognizing that the "people" are birds (their speech = calls, appearance = field marks, habitat = location). Once that's clear, the IBP code step is a second, cleaner revelation. Two ahas in sequence, with the first being the central one. |
| Paper Suitability | Yes | 14 prose vignettes are fully text-reproducible. The puzzle as transcribed here is entirely self-contained in markdown format. |
| Quality Signal | 3 | The construction is meticulous — each vignette encodes call mnemonics, visual field marks, and behavioral/habitat cues simultaneously. The like-count conceit (popular = common species, rare species = few likes) is an elegant additional layer that rewards noticing. Thematic coherence is exceptional: the museum "people watching" frame holds all the way through extraction. |

**Katz note:** This is a model puzzle for thematic integration — the delivery mechanism (social media vignettes) is so internally coherent that you can't separate form from content. The like-count correlation with species commonality is exactly the kind of added layer that justifies the slot without extending the solving time. Mechanically varied from everything else in any round it would sit in.

**Snyder note:** The dual-coding (visual + vocal + behavioral) in each vignette is demanding construction work. Each entry has to satisfy multiple constraints simultaneously: the call mnemonic has to be renderable as plausible human dialogue, the field mark has to be describable as human appearance, and the scene has to be socially coherent. That's real craft. He'd want to verify that each entry has exactly one valid bird identification — the like-count layer suggests the constructor was thinking about disambiguation carefully.

**Young note:** This is the puzzle in the batch that most successfully makes the solver inhabit a world. You are not solving about birds; you are watching people and realizing they are birds, which is a completely different experience. The flavor text ("if only you can recall what they said") sets up the call-mnemonic layer without over-scaffolding it. The IBP code extraction is the one moment where the world-frame breaks — "IBP 4-letter alpha code" is clinical in a way the vignettes aren't. She'd push to find a more diegetic way to introduce that step.

**Use for ablation:** Yes — strong example of Domain Load 3 with genuine mechanism elegance. Also useful for testing whether reviewers distinguish "domain-heavy but clean" from "domain-heavy and sloppy."

---

## Much Ado About Nothing
**Source:** MIT Mystery Hunt 2023 | **Answer:** RANDOMDECISION

| Dimension | Classification | Notes |
|-----------|---------------|-------|
| Hunt Level | MIT/Elite | Requires computing Pearson correlation coefficients, t-scores, and p-values against scene-derived datasets from a specific Shakespeare play — then applying a significance threshold to filter to exactly 7 results and A1Z26-encoding variable indices. The barrier to entry is genuine mathematical statistics expertise, not general intelligence. |
| Mechanism | Logic/Constraint | Statistical deduction chain: dataset derivation → correlation computation → significance filtering → letter extraction. The "logic" here is mathematical rather than grid-based, but the structure is forced deduction at every step. |
| Domain Load | 3 | Essential. Requires working knowledge of Pearson correlation, t-score conversion, and p-value interpretation. Also requires access to and familiarity with *Much Ado About Nothing* text (specifically scene-by-scene line data). A non-statistician cannot complete this without substantial research and computation. |
| Aha Clarity | 1 | No single aha — this is a procedure puzzle. The steps are specified in the flavor text and hints. The "insight" is executing each step correctly. There is no moment of revelation; there is a sequence of computations. |
| Paper Suitability | Partial | The procedure is text-reproducible, but the actual dataset (derived from Shakespeare's text) and the variable definitions (X_i, Y_j) are apparently specified in the puzzle interface and are not fully described in the available text. Without knowing what X_i and Y_j actually are, the puzzle cannot be replicated from this description. |
| Quality Signal | 2 | Well-constructed in the sense that the procedure is internally consistent and yields a unique result. But the puzzle confuses computation with deduction — it's an exercise in applied statistics, not puzzle-solving. The title/theme connection (Shakespeare + statistics = "much ado about nothing" pun) is clever, but the mechanism does not embody the theme. |

**Katz note:** This is the puzzle in the batch that most clearly violates the "no computation without deduction" principle. The entire mechanism is specified in the flavor text. A solver who knows statistics can grind through the procedure; there is no moment where understanding something changes what you do next. The p < 0.05 threshold is arbitrary from a puzzle standpoint — the solver has to trust it rather than derive it. He would vote to cut this puzzle from any hunt he was editing.

**Snyder note:** The construction question he'd ask: is there a genuine forced path here, or is the procedure just followed until an answer emerges? Pearson correlation computation is not a deduction — it's an algorithm. The "exactly seven" significant correlations is either a construction artifact (the dataset was built to produce seven) or a discovery (unlikely). If it's constructed, the puzzle is doing work he respects; if it's discovered, it's not a puzzle. He cannot determine which from the description. He'd want to see the variable definitions and dataset before assessing further.

**Young note:** The Shakespeare frame is a pun, not a world. The solver is not inside *Much Ado About Nothing*; they're using it as a data source. The answer RANDOMDECISION does not reframe the experience of solving — it doesn't name anything the solver was doing or feeling. She'd call this a failed thematic integration: the title promises something the puzzle doesn't deliver.

**Use for ablation:** Conditional — useful as an example of MIT-level complexity that lacks Aha Clarity and has thematic problems. The Aha 1 rating and Quality 2 combination makes it a useful negative example.

---

## Brain Freeze
**Source:** MIT Mystery Hunt 2023 | **Answer:** GEOMETRICSNOW

| Dimension | Classification | Notes |
|-----------|---------------|-------|
| Hunt Level | Standard Hunt | The mechanism — anagram identification → mathematical number type → corresponding song → lyric → acrostic — is a multi-step chain without a singular aha. Each step is clear once you find the entry point (all words are mathematical number types). The domain knowledge (number type names + specific artist's songs) is accessible with moderate research. |
| Mechanism | Hybrid | Anagram (wordplay) + knowledge extraction (math number types + song identification) + acrostic (transformation). A genuine multi-mechanism hybrid. |
| Domain Load | 2 | Helpful. Knowing the mathematical number sequence names (triangular, square, lucky, happy, etc.) speeds the solve significantly. The specific artist's songs require research if you don't know them, but the artist identification (once the connection is seen) narrows the search substantially. |
| Aha Clarity | 2 | The aha of recognizing that all scrambled words are mathematical number type names is present and satisfying, but it's one of three or four steps rather than the central experience. The "artists share a common theme" realization is a second aha, but downstream of the first. |
| Paper Suitability | Partial | The puzzle is described as image-based — the word list is presented visually and some elements (bolded words, alphabetically ordered "meaningless letters") depend on visual formatting that may not survive markdown conversion. A careful text reconstruction would approximate it, but precision could be lost. |
| Quality Signal | 2 | The mechanism is well-conceived; mathematical number type names as the unscrambling target is a clean choice. However, the multi-step chain (anagram → number type → song → lyric → acrostic) accumulates enough steps that any one of them could break the chain for a solver. The round placement (Science) makes the math connection appropriate. |

**Katz note:** Structurally, this is a pipeline puzzle — each step feeds the next without significant backsolving opportunity. If the song identification step breaks, the acrostic is inaccessible. He'd want to verify that the "common theme" among artists is distinctive enough to confirm when found and that the lyric-to-acrostic step is uniquely determined. Multi-step pipelines need to be tested for chain fragility; this one has at least one potentially fragile joint (the "bolded words relate to blanks" step, which sounds ambiguous in the description).

**Snyder note:** The anagram step is the cleanest construction in this puzzle — all words being mathematical number types is a well-bounded set that makes confirmation possible. The subsequent steps involve more interpretive work (which artist? which song? what do "bolded words" mean in context?). Without seeing the actual puzzle image, he cannot assess whether the entry point (the initial scrambled word list) is visually clear or requires sorting through noise.

**Young note:** The "Brain Freeze" title and Science round placement work together — mathematical sequences encoded as scrambled cold words, payoff GEOMETRICSNOW, is thematically coherent. The acrostic step (reading first letters) is the extraction step she'd push on: does it feel earned, or is it mechanical? If the words chosen for the acrostic are arbitrary (any words that happen to yield the right first letters), the extraction is a tax, not a payoff. The answer GEOMETRICSNOW does feel thematically arrived at — it reframes what you were doing.

**Use for ablation:** Yes — good example of a multi-step hybrid that sits below MIT/Elite. Useful for testing whether reviewers can identify "Standard Hunt" when complexity is high but aha clarity is moderate.

---

## World's Smallest Logic Puzzles
**Source:** MIT Mystery Hunt 2023 | **Answer:** CHAOSDOMAIN

| Dimension | Classification | Notes |
|-----------|---------------|-------|
| Hunt Level | MIT/Elite | Requires identifying and correctly solving approximately 40-46 distinct Nikoli-style logic puzzle types at miniature scale, using cross-grid equality/inequality constraints as the additional layer. The puzzle-type identification layer (matching unlabeled grids to type names) is a meta-challenge that presupposes broad familiarity with the Nikoli tradition. |
| Mechanism | Logic/Constraint | Nikoli-style deduction puzzles + type identification + index extraction + grid overlay. The core is logic/constraint solving, but the meta-challenge of type identification adds an identification layer. |
| Domain Load | 3 | Essential. Recognizing Dosun-Fuwari, Rukkuea, Nanbaboru, Gaidoaro, and Yokibunkatsu (among 40+ types) requires deep familiarity with the Nikoli puzzle tradition. A solver unfamiliar with even common types like Masyu or Nonogram cannot begin. |
| Aha Clarity | 2 | The central aha — that each set has one completely blank grid, and those blank grids serve as overlays onto a master letter grid — is present and satisfying. But it follows from solving, not from insight. The type-identification challenge is a sustained competence test more than a moment of revelation. |
| Paper Suitability | Partial | The puzzle depends on visual grid representations of miniature logic puzzles. A text description cannot reproduce the actual grids. The extraction step (overlay onto letter grid, highlight cells 2 spaces from neighbor positions) requires the actual spatial layout. This puzzle cannot be meaningfully reconstructed in markdown. |
| Quality Signal | 3 | The ambition of this construction is exceptional — 40+ miniature puzzles across 8 sets, each with cross-puzzle constraints, yielding a multi-layer extraction. The blank-grid overlay mechanism is a genuinely novel extraction technique. This is citeable as exemplary construction work within the logic puzzle tradition. |

**Katz note:** This is a puzzle for a specific audience — solvers with significant Nikoli experience — and it's honest about that. The type-identification challenge is the barrier: if your team has a Nikoli expert, this puzzle falls; if you don't, it becomes a research puzzle of a different kind. He'd want to verify that the cross-grid equality/inequality constraints actually provide uniqueness without the type identification — or whether type ID is required for solvability. The answer CHAOSDOMAIN is pleasingly opposite to the orderly logic puzzle grid aesthetic.

**Snyder note:** This is exactly the kind of puzzle he would want to have constructed. The choice to use miniature grids forces construction elegance — at small scale, every cell is load-bearing, and there's no room for extraneous constraints. The 40+ type breadth is a statement about the scope of the logic puzzle tradition. He'd scrutinize the cross-grid equality/inequality constraints carefully: those are the mechanism by which sets achieve uniqueness, and if any are under-constrained, the whole set breaks. He'd verify each set in test.

**Young note:** The puzzle does not inhabit a "world" in her sense — it's a collection of puzzles organized by a clever meta-mechanism. But the "World's Smallest" frame is committed: every element is miniature, the puzzle is about demonstrating breadth through compression. She'd ask about the solver experience of 40+ puzzles in sequence — at what point does this become endurance rather than discovery? The blank-grid overlay at the end is a satisfying visual payoff that she'd call a moment of genuine visual resolution.

**Use for ablation:** Yes — exemplary for Quality Signal 3 and Domain Load 3 in the logic tradition. Contrasts sharply with Much Ado About Nothing as a case where high complexity and high construction quality coexist.

---

## One of the Puzzles of All Time
**Source:** MIT Mystery Hunt 2023 | **Answer:** NORMALBUNDLE

| Dimension | Classification | Notes |
|-----------|---------------|-------|
| Hunt Level | Standard Hunt | The "morbing" anagram mechanism is accessible once explained; the underlying clue-solving (16 reviews each yielding one word) is standard cryptic-adjacent cluing. The pairing step (critic + audience reviews with shared letters) is the main puzzle challenge. The internet-meme framing makes the entry point either immediate (if you know the meme) or irrelevant (if you don't — the mechanism still functions). |
| Mechanism | Hybrid | Wordplay (clue-solving to yield one word per review) + transformation (anagram/"morbing" + letter-matching). |
| Domain Load | 2 | Helpful. Recognizing the *Morbius* "It's morbin' time" meme speeds orientation but isn't required — the mechanism functions without it. Individual review clues reference specific items (Jiangdu by area, helium-neon laser, Zeeland province) that require research but are individually googleable. |
| Aha Clarity | 2 | The aha of recognizing that "morbing" means anagramming is central but must be arrived at through the meme context. The pairing step (critic + audience words with shared letters) is a second discovery. Two ahas in sequence, neither fully clean in isolation. |
| Paper Suitability | Yes | The puzzle as described is fully text-reproducible. 16 reviews with their clue content render in prose. The mechanism description is complete in the text. |
| Quality Signal | 2 | Well-constructed in its moment — the *Morbius* meme was at peak cultural saturation in early 2022, making the puzzle's humor precisely timed. The mechanism (anagramming = "morbing") is a clever lexicalization of a specific internet phenomenon. However, the cultural specificity ages it: solvers who don't know the meme miss the entry framing entirely. The answer NORMALBUNDLE is mathematical (normal bundle from differential geometry), connecting to the Science round, but the connection to the meme is arbitrary. |

**Katz note:** This puzzle earns its slot through commitment to a bit — the entire thing is a sustained meme joke, and it executes that joke at puzzle-hunt quality. The structural question is whether the clue-solving step (16 individual words from reviews) is uniquely determined — if two reviews could yield synonyms that both pair plausibly with audience reviews, the pairing step has ambiguity. He'd want to know the specific words yielded by each review before assessing tightness.

**Snyder note:** The anagram step is elegantly self-described ("morbing" = scrambling letters, so the puzzle names its own mechanism). The matching step concerns him: if "many letters in common" is not precisely defined, the pairing produces ambiguous results. He'd want to see a uniqueness proof for the critic/audience pairing — is there exactly one valid matching, or does the anagram step disambiguate after the fact? Construction is solid but the pairing criterion needs tightening in his assessment.

**Young note:** The *Morbius* meme frame is a world — a specific cultural moment rendered as a puzzle. The puzzle doesn't just use the meme; it performs the meme while using it. That's thematic integration at the level she cares about. The answer NORMALBUNDLE is a miss: it exits the meme-world into mathematical jargon and doesn't reframe what the solver was doing. She'd push for an answer that names the experience of "morbing" rather than a science-round requirement.

**Use for ablation:** Yes — good example of culturally-specific puzzle construction. Tests whether reviewers can assess Quality Signal independently of personal familiarity with source material.

---

## H2No
**Source:** MIT Mystery Hunt 2023 | **Answer:** ENCAMPMENT

| Dimension | Classification | Notes |
|-----------|---------------|-------|
| Hunt Level | Standard Hunt | The date-arithmetic mechanism is clear and procedural — find an anchor event, compute a year, identify the location. The "what do these locations have in common" step and the differential extraction are additional layers that elevate this above pure trivia, but the overall experience is research-forward rather than insight-forward. |
| Mechanism | Knowledge extraction | Historical date arithmetic → location identification → property identification → differential letter extraction. |
| Domain Load | 3 | Essential. Each clue requires knowing a specific historical event at a specific location (Neumann glassware company, the London Bridge "replacement" song, a one-day state capital, etc.). Without the ability to research or know these events, no locations can be identified and extraction is impossible. |
| Aha Clarity | 2 | Two ahas: (1) recognizing that "what they have in common" is the shared property (likely a water-related prohibition, suggested by "H2No"), and (2) the "differences" flavor text pointing to the differential extraction. Neither is as singular as the top-tier aha puzzles. |
| Paper Suitability | Yes | Six text clues with enumerations are fully text-reproducible in markdown. Self-contained. |
| Quality Signal | 2 | The flavor text ("watch out for any differences") is elegant misdirection — "differences" operates simultaneously as a warning about variation and a pointer to the subtraction/differential mechanism. The H2No title signals water/no-water prohibition without giving it away. Well-constructed at the clue level; the extraction mechanism (using "differences" from a shared property) is clever but depends on the actual puzzle content (what is the shared property? what are the differences?) being well-chosen enough to yield ENCAMPMENT unambiguously. |

**Katz note:** This is a research puzzle with a clever extraction frame. The "differences" flavor text is doing real structural work — it's both a misdirect (watch for variations that trip you up) and a direct instruction (use the differences as the extraction key). That dual reading is the construction quality here. He'd want to verify that the six locations are findable without excessive research strain and that the shared property is narrow enough to be confirmable once all six are identified.

**Snyder note:** The six clues need to each have exactly one valid location — the date arithmetic must be precise enough that multiple candidate events don't produce the same year at different locations. He'd verify each clue's uniqueness independently. The extraction mechanism ("differences" from a shared property) needs to produce exactly 10 letters in the right order; the ordering mechanism (presumably by clue number) needs to be confirmed as unambiguous.

**Young note:** H2No is a clever title that promises a world (water prohibition, prohibition locations) without fully delivering one. The tourist flavor ("these locations have something in common") gestures at the Museum setting but the puzzle doesn't inhabit that world — it's a research exercise in historical geography. The answer ENCAMPMENT doesn't reframe the solving experience in a way she'd find satisfying. She'd call this well-constructed but not fully realized thematically.

**Use for ablation:** Yes — clean example of Knowledge Load 3 at Standard Hunt level. Contrasts usefully with People Watching (also Load 3, but MIT/Elite and thematically integrated).

---

## Batch 1 Summary

| Puzzle | Hunt Level | Mechanism | Domain | Aha | Paper OK | Quality | Use? |
|--------|-----------|-----------|--------|-----|----------|---------|------|
| Scicabulary | MIT/Elite | Wordplay | 1 | 3 | Yes | 3 | Yes |
| Museum Rules | MIT/Elite | Knowledge extraction | 3 | 2 | No | 2 | Conditional |
| People Watching | MIT/Elite | Knowledge extraction | 3 | 2 | Yes | 3 | Yes |
| Much Ado About Nothing | MIT/Elite | Logic/Constraint | 3 | 1 | Partial | 2 | Conditional |
| Brain Freeze | Standard Hunt | Hybrid | 2 | 2 | Partial | 2 | Yes |
| World's Smallest Logic Puzzles | MIT/Elite | Logic/Constraint | 3 | 2 | Partial | 3 | Yes |
| One of the Puzzles of All Time | Standard Hunt | Hybrid | 2 | 2 | Yes | 2 | Yes |
| H2No | Standard Hunt | Knowledge extraction | 3 | 2 | Yes | 2 | Yes |

---

## Panel Discussion Notes

**Distribution observations:**

- 5 of 8 puzzles classify as MIT/Elite — consistent with this batch drawing exclusively from MIT Mystery Hunt 2023. The remaining 3 (Brain Freeze, One of the Puzzles of All Time, H2No) are Standard Hunt, which the panel views as "strong hunt puzzles" rather than simple ones.

- No puzzle scores Aha Clarity 3 except Scicabulary. This may reflect the MIT Mystery Hunt tendency toward multi-step mechanism chains rather than single-aha designs. Alternatively it may reflect the inherent difficulty of achieving clean single-aha construction at elite difficulty — complexity and aha clarity may trade off.

- Domain Load is heavily weighted toward 3 (Essential). Six of eight puzzles require domain expertise. This is expected for MIT Mystery Hunt but worth noting for ablation: the batch may not represent the full range of Domain Load distribution across hunt types.

- Paper Suitability is split: 4 Yes, 2 Partial, 1 No. The No (Museum Rules) and Partials (Brain Freeze, Brain Freeze, World's Smallest Logic Puzzles) all involve image-dependent or interactive delivery. The remaining 4 are fully text-reproducible. This creates a natural two-group comparison for any study of paper-reproducibility.

**Inter-reviewer disagreements noted:**

- **Much Ado About Nothing (Hunt Level):** Katz would argue for downgrading to Standard Hunt precisely because the absence of aha clarity makes this more of an exercise than a puzzle. The panel maintained MIT/Elite based on barrier-to-entry (statistics expertise + dataset generation), but noted his objection.

- **World's Smallest Logic Puzzles (Aha Clarity):** Snyder argued for Aha 3 based on the blank-grid overlay revelation being a genuine visual aha; the panel stayed at 2 because the aha follows from solving rather than preceding it.

- **One of the Puzzles of All Time (Quality Signal):** Young argued for 3 on grounds of thematic commitment; Katz and Snyder stayed at 2 on grounds of pairing-criterion ambiguity and cultural aging. Majority ruled.

**Methodological note for research:**

The panel assessed WITHOUT solving. For puzzles where the mechanism is partially opaque from description alone (Museum Rules, Brain Freeze), assessments carry higher uncertainty. Researchers using this classification should note which puzzles were assessed from complete vs. incomplete mechanism descriptions.
