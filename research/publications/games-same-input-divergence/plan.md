# Paper Plan: Structured Divergence in AI Creative Generation — Same Input, Different Outputs

## Research Question

When the same creative brief is fed into an AI creative pipeline twice, independently, how much do the outputs diverge — and is that divergence structured (predictable) or random? We call this the same-input divergence problem and study it in the context of puzzle hunt creation: the same Age of Empires 2 world data, scope document, and pipeline specification run twice produces two independent hunts. We compare the hunts at the level of puzzle pool selection, answer word choices, meta mechanism design, difficulty curve, and AI panel scores to characterize which design decisions are stable across runs (converge) and which are variable (diverge).

The broader question is: for practitioners using AI creative generators in real pipelines, which outputs can be treated as reliable (run once, use the result) and which require multiple runs and human selection from among the outputs? Understanding this structure is essential for rational use of AI generators in creative production contexts — it tells practitioners where to invest human oversight and where AI can operate autonomously.

## Target Venue

- Conference: ICCC 2026 (International Conference on Computational Creativity) or NeurIPS 2026 Creativity and Design Workshop
- Deadline: TBD (ICCC typically March–April; NeurIPS workshop typically September)
- Page limit: 8–10 pages (ICCC full paper); 4–6 pages (NeurIPS workshop)
- Expected contribution type: Empirical Study
- Track note: ICCC will value the methodological novelty of the same-input divergence framing and the grounded empirical study. NeurIPS workshop is a good fit if we want to reach the ML community with implications for generative model deployment. Target ICCC for full paper; NeurIPS workshop as parallel or fallback venue.

## Core Argument

AI creative generators are stochastic by design, but practitioners lack a principled characterization of which generative decisions are sensitive to this stochasticity and which are robust. Prior work on generation diversity (e.g., quality-diversity algorithms, diversity metrics in PCG) focuses on maximizing or measuring diversity as a goal in itself. We study a different question: given a fixed creative brief, which aspects of the output should be diverse (reflecting the legitimate design space) and which should be stable (reflecting domain knowledge and structural constraints)?

Our empirical study finds that same-input divergence in puzzle hunt generation is structured, not random: puzzle type variety (cryptic, logic, wordplay, trivia) is stable across runs (both runs produce similar distributions of puzzle types); meta mechanism type (word → meta answer extraction) is stable; domain knowledge gating (puzzles built from verifiable AoE2 facts) is stable. But specific answer words, extraction mechanism details within a puzzle type, difficulty ordering within a round, and individual clue selections are highly variable. This structure maps onto a principled distinction: stable decisions are constrained by the domain knowledge and structural specifications; variable decisions are made from a large, equally-valid design space with no constraint reducing the options. This distinction has immediate practical implications: use AI for stable decisions, use human selection (or multiple runs) for variable decisions.

## Sections

### 00-abstract (~250 words)
- Problem: AI creative generators are stochastic; practitioners need to know which design decisions are stable vs. variable across independent runs
- Approach: same-input divergence study — run the AoE2 puzzle hunt pipeline twice from the same world data; compare outputs at multiple levels of abstraction
- Findings: structured divergence — puzzle type variety, meta mechanism, domain gate are stable; answer words, extraction details, difficulty ordering are variable
- Contribution: empirical characterization of same-input divergence structure; practical design recommendations for AI creative pipeline deployment

### 01-introduction (~900 words)
- Hook: two puzzle hunt designers given the same brief would produce different hunts; that's expected and healthy. Two AI systems given the same brief should also produce different hunts — but which differences are meaningful and which are arbitrary?
- Problem: practitioners using AI creative generators need to know when to run once and trust the output, and when to run multiple times and select; current practice is ad hoc
- Opportunity: by running the same pipeline twice and comparing outputs, we can characterize which decisions are stable (domain-constrained) and which are variable (design-space-constrained)
- Our approach: same-input divergence experiment on a complete puzzle hunt pipeline; multi-level comparison (pool, assignment, meta, difficulty curve, panel scores)
- Key question: is divergence random or structured? (answer: structured, and the structure maps onto a principled domain knowledge / design space distinction)
- Contributions list:
  1. The same-input divergence experimental paradigm for characterizing AI creative generator stability
  2. Empirical data on which design decisions are stable vs. variable in puzzle hunt generation
  3. A principled explanation of the divergence structure (domain constraint vs. design space latitude)
  4. Practical recommendations for where to apply human selection in AI creative pipelines
  5. A comparison metric suite for measuring creative artifact divergence at multiple levels

### 02-related-work (~1000 words)
Subsections:
- **AI Creative Generation and Diversity**: quality-diversity algorithms (Mouret & Clune 2015 MAP-Elites; Gravina et al. 2019 QD-survey); diversity metrics in PCG (Smith et al. 2010, Shaker & Abou-Zleikha 2014); the goal in these works is maximizing diversity; our goal is characterizing the structure of divergence
- **Stochasticity and Reliability in Language Model Output**: temperature effects on LLM output diversity (Ficler & Goldberg 2017); prompt sensitivity in LLMs (Jiang et al. 2020, Zhao et al. 2021); our study differs by fixing the prompt and measuring output divergence across runs at a structured, multi-level artifact level
- **Reproducibility in Generative Systems**: reproducibility in ML (Pineau et al. 2021); creative system reproducibility (Colton et al. 2011); seed control in PCG; our study deliberately varies the seed/run to measure natural stochasticity, not to eliminate it
- **Puzzle Hunt and Creative Artifact Comparison**: no prior work compares two independent puzzle hunt generations from the same brief; we establish the comparison methodology here
- **Creative Design Space Theory**: Simon's design space as a search problem (Simon 1969 Sciences of the Artificial); Lawson's design cognition (Lawson 1997); the distinction between constrained and unconstrained design decisions as a theoretical framing for the stable/variable finding
- **Key gap**: no prior work characterizes the structure of same-input divergence in AI creative generation; the quality-diversity literature assumes divergence is uniformly desirable; practitioners lack principled guidance on which decisions to trust from a single run

### 03-methodology (~1200 words)
Subsections:
- **The Same-Input Divergence Paradigm**: definition; relation to reproducibility studies (we don't want to eliminate divergence, we want to characterize it); relation to ablation studies (we're not removing components, we're repeating with full specification); relation to sensitivity analysis (we're not varying inputs, we're measuring output variation from fixed inputs)
- **Experimental Setup**:
  - Hunt 1 (AoE "Wololo"): 5 puzzles + meta, answer WOLOLO, fully complete; this is the primary scenario from the pipeline paper
  - Hunt 2 (AoE "New Run"): generated fresh from the same world data (SCOPE.md, world knowledge base, AoE2 facts), same skill library, same profile library; all generative stages run independently without access to Hunt 1 artifacts
  - Control: same pipeline specification (toolkit CLAUDE.md skills), same profile library (29 v2 profiles), same world data; only stochastic variation is from LLM generation
- **Comparison Levels**: five levels of comparison, from coarse to fine
  - Level 1 (Structural): number of puzzles per round, round names, meta type
  - Level 2 (Pool): puzzle type distribution in pool, top-10 pool selections, score distribution
  - Level 3 (Assignment): answer words chosen, feeder answer set for meta
  - Level 4 (Mechanism): within each puzzle type, specific mechanism used (e.g., which cryptic clue type, which logic grid constraint)
  - Level 5 (Micro-design): individual clue formulations, flavor text, grid content
- **Comparison Metrics**:
  - Structural: Jaccard similarity on round structure; exact match on meta type
  - Pool: puzzle type distribution KL divergence; top-10 set overlap; score distribution correlation
  - Assignment: answer word set overlap; semantic similarity of answer word sets (embedding distance); meta feeder word overlap
  - Mechanism: coded by puzzle type (requires manual coding); mechanism similarity matrix
  - Micro-design: edit distance on equivalent puzzle sections; semantic similarity of clue formulations
- **Stability Classification**: a decision is classified as "stable" if the comparison metric exceeds a threshold across the two runs; "variable" if below threshold; thresholds calibrated per metric type

### 04-results (~1400 words)
Subsections:
- **Level 1 — Structural Stability**: Hunt 1 and Hunt 2 round structure; meta type; how similar are the two hunts at the top level?
  - Expected: high stability — both produce 5 puzzles + meta, 3 rounds, word-extraction meta type
  - Report: exact match on meta type; round structure similarity; any structural divergence at this level
- **Level 2 — Pool Stability**: puzzle type distribution comparison; top-10 pool overlap; score distributions
  - Expected: moderate-high stability — both pools drawn from same AoE2 domain, same puzzle type diversity constraints
  - Report: KL divergence on type distribution; top-10 overlap count; score distribution comparison (means, variance)
- **Level 3 — Assignment Variability**: answer word overlap; semantic distance of answer word sets; meta feeder word overlap
  - Expected: moderate-low stability — many equally valid answer words exist for a given AoE2 concept; specific word choice is arbitrary within the valid set
  - Report: answer word set overlap (exact match); semantic similarity; meta feeder set comparison; this is expected to be the most variable level
- **Level 4 — Mechanism Variability**: per-puzzle-type mechanism comparison
  - Expected: moderate variability — within a puzzle type (e.g., cryptic crossword), many mechanism variants are equally valid; the choice of which variant to use is unconstrained
  - Report: manual coding of mechanism types per puzzle; similarity matrix; are any mechanism types stable (always chosen) or variable (randomly distributed)?
- **Level 5 — Micro-design Variability**: clue formulation, flavor text, grid content
  - Expected: high variability — individual clue formulations are highly sensitive to stochastic generation
  - Report: edit distance on equivalent sections; semantic similarity of clue sets
- **Stability Structure Summary**: table of 5 levels × {stability classification, expected, observed, explanation}; the key finding presented as a clean table

### 05-discussion (~800 words)
- **The principled explanation**: stable decisions (Level 1 structural, Level 2 pool distribution) are constrained by domain knowledge requirements and pipeline specifications; the AoE2 domain mandates that any valid hunt reference verifiable AoE2 facts, and the diversity constraint mandates multiple puzzle types — these constraints reduce the design space to a small set of equally-valid structures. Variable decisions (Level 3 answer words, Level 4 mechanisms, Level 5 micro-design) are made from a large, equally-valid design space where no constraint privileges one option over another.
- **Practical implications for AI pipeline deployment**: run-once and trust for stable decisions; run-multiple and select for variable decisions; specifically: trust the pipeline for round structure and puzzle type variety (stable); use human selection or multiple runs for specific answer words and extraction mechanisms (variable)
- **Implications for evaluation**: if AI panel scores are stable across runs, the pipeline's quality guarantee is meaningful; if scores are variable, the panel is sensitive to arbitrary design choices (which would be concerning). Report panel score stability and interpret.
- **The divergence structure as a quality signal**: variable decisions that produce consistently high-quality outputs in both runs are "safe" variable decisions (the design space is large but all-good); variable decisions that produce quality differences between runs are "risky" variable decisions (the design space is large and quality-uneven). Identify which of our variable decisions fall into each category.
- **Limitations**:
  - Only two runs; a true characterization of divergence structure requires many more runs (N≥10 at minimum); these two runs provide a case study, not a statistical result
  - Only one domain (AoE2); other domains may have different stability structures
  - The "same world data" control is imperfect — the world data is a text document, and LLM processing of that document varies run-to-run in addition to the generative decisions we measure
- **Future work**: replicate with N=10 runs across 3 domains; develop automated stability metrics that don't require manual coding; extend to other AI creative pipelines (tabletop campaign generation, interactive fiction)

### 06-conclusion (~400 words)
- Restate contribution: empirical characterization of structured divergence in AI puzzle hunt generation
- Key takeaway: divergence is not random — it maps predictably onto the domain-constraint/design-space distinction
- Practical guidance: which decisions to run-once-and-trust vs. run-multiple-and-select
- The same-input divergence paradigm as a generalizable method for characterizing any AI creative generator
- Availability: both hunt artifacts (PUZZLE-POOL.md, ROUNDS.md, puzzle files, panel scores), comparison scripts, and methodology description available at [URL]

## Experiments

- [x] Hunt 1 (AoE "Wololo"): complete — 5 puzzles + meta, answer WOLOLO, all AI panel scores available
- [ ] Hunt 2 (AoE "New Run"): generate fresh run of full pipeline from same world data without access to Hunt 1 artifacts; document all generative choices at each stage
- [ ] Level 1 comparison (structural): compare round structure and meta type between Hunt 1 and Hunt 2
- [ ] Level 2 comparison (pool): compute puzzle type distribution, top-10 overlap, score distributions for both pools
- [ ] Level 3 comparison (assignment): answer word overlap, semantic similarity, meta feeder comparison
- [ ] Level 4 comparison (mechanism): manual coding of mechanism types per puzzle per run; similarity matrix
- [ ] Level 5 comparison (micro-design): edit distance and semantic similarity on clue formulations
- [ ] Stability classification: apply thresholds to classify each level as stable vs. variable
- [ ] Panel score stability: run AI panel on Hunt 2 puzzles; compare score distribution to Hunt 1

## Figures

- [ ] Figure 1: Same-input divergence paradigm diagram — same world data + pipeline → Hunt 1 and Hunt 2 (parallel outputs); 5-level comparison hierarchy shown as layers
- [ ] Figure 2: Level-by-level stability visualization — radar chart or divergence profile showing stability score at each of the 5 levels; key figure showing the stable-high to variable-high gradient
- [ ] Figure 3: Pool comparison — side-by-side puzzle type distribution (Hunt 1 vs. Hunt 2) and top-10 pool score distributions; shows Level 2 stability
- [ ] Figure 4: Answer word divergence — Hunt 1 and Hunt 2 feeder answer sets visualized as overlapping circles (Venn) with semantic distance annotation; shows Level 3 variability
- [ ] Figure 5: Stability structure summary — conceptual figure mapping 5 levels onto the domain-constraint vs. design-space-latitude axis; the theoretical contribution in visual form

## Tables

- [ ] Table 1: Comparison methodology — 5 levels × {comparison unit, metric, threshold, expected direction}
- [ ] Table 2: Level-by-level results — 5 levels × {Hunt 1 value, Hunt 2 value, comparison metric, stability classification}
- [ ] Table 3: Answer word comparison — Hunt 1 answer words vs. Hunt 2 answer words; semantic similarity score; meta feeder overlap
- [ ] Table 4: Panel score comparison — Hunt 1 puzzle scores (by dimension) vs. Hunt 2 puzzle scores (by dimension); score stability analysis
- [ ] Table 5: Stability structure summary — 5 levels × {stability classification, domain-constraint or design-space-latitude, practical recommendation for practitioners}

## Analysis Scripts

- [ ] scripts/structural_compare.py — diff ROUNDS.md files from two hunt runs; output structural similarity metrics
- [ ] scripts/pool_compare.py — diff PUZZLE-POOL.md files; compute type distribution KL divergence, top-10 overlap, score distribution stats
- [ ] scripts/answer_compare.py — compare answer word sets; compute exact overlap, semantic similarity via embedding distance
- [ ] scripts/score_stability.py — load panel scores from two hunt runs; compute per-dimension correlation and overall distribution comparison
- [ ] scripts/mechanism_coder.py — template for manual mechanism coding; outputs mechanism similarity matrix given two sets of coded mechanism labels

## Quality Checkpoints

- [ ] Word count: 5500–7500 words (ICCC 8-page target)
- [ ] References: 30+ citations (mix of PCG, ML reproducibility, design cognition, game AI, creativity)
- [ ] Figures: 5 figures
- [ ] Tables: 5 tables
- [ ] Hunt 2 generation: must be run without access to Hunt 1 artifacts (clean-room condition documented)
- [ ] All comparison metrics operationalized and thresholds justified
- [ ] Stability classification covers all 5 comparison levels
- [ ] Practical recommendations section derived from stability data, not from speculation

## Dependencies

- Hunt 2 generation (clean-room) is the blocking dependency for all comparisons
- All Level comparisons (1–5) depend on: Hunt 2 generation complete
- Panel score stability depends on: Hunt 2 panel scores (requires running AI panel review on Hunt 2 puzzles)
- Stability classification depends on: all Level comparisons complete
- Figure 2 (stability visualization) depends on: stability classification
- Discussion depends on: all Level comparison results and stability classification

## Related Papers

- `games-ai-expert-panel-creative`: The primary paper — this paper's Experiment 3 (same-input new hunt) is the seed of this paper; the full divergence analysis here is the complete study of what the primary paper describes only as a comparison of "structural differences" and "quality comparison"
- `games-generative-hunt-pipeline`: The pipeline paper — Hunt 1 (AoE "Wololo") is the Age of Empires scenario from that paper; this paper uses the same scenario as a testbed and extends it with a second run; both papers should cross-reference the Hunt 1 characterization
- `games-profile-taxonomy-creative`: The taxonomy paper — the divergence analysis can include an evaluative-framework-level comparison: do the same evaluative frameworks appear in both runs' AI panel reviews, or do the frameworks diverge too? This would be a secondary finding adding depth to the taxonomy paper's cross-domain transfer claim
- `games-human-ai-calibration`: The calibration paper — if AI panel scores are stable across the two runs (same puzzle brief → similar scores even with different specific puzzles), this supports the claim that AI panel scores are a reliable quality signal; the calibration data provides the human baseline to check whether the score stability reflects genuine quality stability or systematic AI bias
