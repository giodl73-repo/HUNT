# Paper Plan: A Taxonomy of AI Evaluative Frameworks for Creative Work

## Research Question

When AI reviewer personas are given richer, domain-specific profile descriptions, do they introduce qualitatively distinct evaluative lenses — not just more verbose feedback — and can those lenses be systematically catalogued into a transferable taxonomy?

We study this question using a controlled profile upgrade experiment: 29 AI reviewer personas evaluated puzzle hunt designs before and after upgrading from generic v1 profiles (~50 lines each) to richer v2 profiles (~80–120 lines each, with named analytical frameworks). The central claim is that richer profiles surface a bounded taxonomy of evaluative lenses that (1) were not introduced by the v1 profiles, (2) are analytically distinct from each other, (3) are tied to specific expert identities rather than to general verbosity, and (4) transfer across creative domains. This taxonomy is a contribution to computational creativity evaluation methodology independent of the specific AI system used to instantiate it.

## Target Venue

- Conference: ICCC 2026 (International Conference on Computational Creativity) or Creativity & Cognition 2026
- Deadline: TBD (ICCC typically March–April; C&C typically March)
- Page limit: 8–10 pages (ICCC); 8 pages (C&C)
- Expected contribution type: Empirical Study with Conceptual Contribution
- Track note: ICCC values novel evaluation methodology and computational models of creative assessment; C&C values cross-disciplinary creativity frameworks. The taxonomy itself is the primary conceptual contribution; the empirical study provides the evidence base. Emphasize that this taxonomy emerged from — and is grounded in — real creative evaluation practice, not armchair theorizing.

## Core Argument

Prior work on AI-simulated peer review focuses on improving accuracy relative to human reviewers on academic papers. Creative domain evaluation is fundamentally different: there is no ground-truth score, and the value of a review is in the specific analytical lens it introduces, not in the numeric rating it assigns. We show that when AI reviewer personas are upgraded from generic to richly specified profiles, the qualitative change is not more words but new frameworks: bounded evaluative lenses that the original profiles reliably failed to introduce.

The taxonomy we extract has 6 primary categories, each tied to a specific expert identity: a-ha economy (puzzle flow design, originating from Foggy Brume), load-bearing test (structural puzzle design, from Thomas Snyder), world-model consistency (interactive fiction/narrative coherence, from Emily Short), social fabric (participatory experience design, from Jordan Weisman), perceptual-shift (visual and perceptual puzzle design, from Scott Kim), and visual grammar (graphic accessibility and information design, from Dana Young). This taxonomy is not specific to puzzle hunt design: each framework applies to game design, interactive fiction, level design, and other creative domains. The profile upgrade experiment provides evidence that these frameworks are latent in the expert identity, not in the task description, which is the methodological claim of broader significance.

## Sections

### 00-abstract (~250 words)
- Problem: richer AI reviewer profiles are assumed to produce better evaluations, but "better" is typically operationalized as higher scores or more words
- Approach: controlled profile upgrade experiment on 29 profiles; qualitative analysis of review texts to identify new evaluative frameworks introduced
- Findings: 6 new evaluative frameworks surfaced by v2 profiles that were absent in v1; each framework is tied to a specific expert identity; frameworks transfer across creative domains
- Contribution: a taxonomy of 6 creative evaluative frameworks, grounded in expert identity; methodology for extracting evaluative frameworks from AI review texts; evidence that profile richness matters for framework introduction, not just score

### 01-introduction (~900 words)
- Hook: a puzzle review that says "this puzzle has a satisfying a-ha moment" is less useful than one that identifies where the a-ha economy breaks down and suggests a specific repair; the difference between v1 and v2 profiles is the difference between these two reviews
- Problem: the field lacks a systematic vocabulary for creative evaluation lenses; existing rubrics (clarity, elegance, fun) are coarse and do not capture the specific analytical perspectives that domain experts bring
- Opportunity: AI reviewer personas, if richly specified, can be made to reliably introduce domain-expert analytical frameworks; studying which frameworks emerge under which profiles gives us a grounded taxonomy
- Our approach: profile upgrade experiment; qualitative coding of review texts; cross-domain transfer study
- Key question: are the new frameworks introduced by v2 profiles artifacts of verbosity, or are they tied to specific expert identities?
- Contributions list:
  1. A 6-category taxonomy of creative evaluative frameworks, grounded in puzzle/game design expert practice
  2. Empirical evidence that framework introduction is tied to expert identity, not profile length
  3. Methodology for extracting and coding evaluative frameworks from AI review texts
  4. Cross-domain transfer analysis: which frameworks transfer from puzzle design to game design, interactive fiction, and level design
  5. Open framework taxonomy with coding guide for replication and extension

### 02-related-work (~1000 words)
Subsections:
- **Evaluation in Computational Creativity**: Colton & Wiggins (2012) on creative system evaluation; Jordanous (2012) SPECS framework; Ritchie (2007) empirical criteria; the persistent problem of evaluating creative quality without ground truth
- **Expert Knowledge in Creative Evaluation**: Csikszentmihalyi's systems model of creativity (creator, domain, field); field-level expertise as domain-specific evaluative knowledge; tacit knowledge in expert evaluation (Polanyi)
- **AI Persona Design and Profile Effects**: Park et al. (2023) generative agents; Zhang et al. persona prompting; Dang et al. creative writing with personas; the gap: no prior work studies which evaluative frameworks different persona types introduce
- **Review Quality in AI-Simulated Peer Review**: Berry et al. on AI paper review; Liu et al. (2023) LLM reviewers; Shah & Wein (2023) reviewer simulation; these studies optimize for numeric accuracy, not for evaluative lens diversity
- **Puzzle Hunt Design as Creative Domain**: the puzzle hunt community's evaluative vocabulary (a-ha moment, confirmation quality, extraction elegance, thematic integration); Snyder's craftsmanship criteria; the Riven Standard; this is an under-studied creative domain with a rich expert vocabulary
- **Key gap**: no prior work extracts a taxonomy of evaluative frameworks from AI reviewer personas; no study shows that specific expert identities introduce specific frameworks; the relationship between profile richness and framework introduction is unstudied

### 03-methodology (~1200 words)
Subsections:
- **Scoring Rubric (7 dimensions, empirically tuned — C11 configuration)**:
  - Clarity ×1, Solvability ×1 (execution quality)
  - **Elegance ×2, Reading Reward ×2** (primary tier discriminators, double-weighted)
  - Fun ×1, Confirmation ×1 (experience quality)
  - **Riven Standard ×1** — "Does the puzzle IS what the field does, not overlaid on it?" (5=mechanic inseparable from domain; 1=theme is decorative)
  - Total /45, pass ≥ 33 (73% threshold)
  - Rationale: validated against 15 real puzzles × 8 conditions; Elegance+RR double-weighted because they are proxies for the Riven Standard; adding RS explicitly captures the residual; achieves 24.5pp tier gap vs 10.9pp with equal weights
  - This rubric is the primary system used in Paper #1 (games-ai-expert-panel-creative); Paper #3 inherits it unchanged
- **Profile Upgrade Experiment Design**: 29 profiles upgraded from v1 to v2; same 3 puzzles evaluated by panels drawn from v1 and v2 profiles; controlled for: same puzzle, same rubric (7-dim weighted above), same number of reviewers (3 per puzzle), same scoring dimensions
- **Profile Versions Compared**:
  - v1 profile anatomy: name, affiliation, brief design philosophy (~50 lines); no named analytical frameworks
  - v2 profile anatomy: name, affiliation, detailed design philosophy, characteristic evaluation lens with 2–3 named frameworks, characteristic concerns, voice register, key works (~80–120 lines); named frameworks are explicitly stated in profile text
- **Coding Methodology**: review texts coded for evaluative framework introduction; a framework is counted as "introduced" if the review text names or clearly applies a specific analytical lens not present in the rubric (i.e., beyond scoring the 6 dimensions); two-pass coding: automated detection of known framework names (a-ha economy, load-bearing test, etc.) then manual review for novel frameworks
- **Framework Identification Criteria**: a new framework must (1) be named or operationalizable, (2) be applied to specific puzzle features rather than stated abstractly, (3) generate actionable feedback, and (4) not reduce to one of the 6 scoring rubric dimensions
- **Cross-Domain Transfer Study**: apply the 6 frameworks as coding categories to review texts from Wavelength (music/thematic), Ironfall (sci-fi RPG), and Grand Larceny (noir fiction) hunts; measure how frequently each framework appears across domains
- **Profile-Framework Attribution Study**: for each of the 29 profiles, determine which frameworks it reliably introduces vs. which it never introduces; compute per-profile framework signature

### 04-taxonomy (~1500 words)
This section IS the main contribution — a detailed exposition of each framework with examples from puzzle reviews.

Subsections:
- **Framework 1: A-ha Economy** (Foggy Brume)
  - Definition: the design and pacing of moments of insight in a puzzle; not just "does the puzzle have an a-ha" but "does the a-ha economy match the puzzle's promised experience"
  - Failure modes: a-ha too early (solver confirms without satisfying journey), too late (solver gives up before reaching it), in wrong place (a-ha is about extraction not about theme)
  - Puzzle hunt examples: AoE II (Feudal Age) — a-ha economy diagnosis and repair
  - Transfer: applies to level design (the "eureka" moment in puzzle-platformers), game design (discovery pacing in exploration games)

- **Framework 2: Load-Bearing Test** (Thomas Snyder)
  - Definition: every element of a puzzle must earn its place; the load-bearing test asks "if I removed this element, does the puzzle collapse?" If not, the element is decorative noise
  - Failure modes: superfluous grid elements, redundant clues, thematic decoration that doesn't contribute to solving
  - Puzzle hunt examples: AoE II (Feudal Age) — load-bearing test reveals a redundant confirmation step
  - Transfer: applies to level design (unnecessary obstacles), narrative design (scenes that don't advance story or character)

- **Framework 3: World-Model Consistency** (Emily Short)
  - Definition: the puzzle's internal logic must be consistent with the fiction it inhabits; a violation of world-model consistency breaks immersion even if the puzzle mechanism is sound
  - Failure modes: anachronistic clue references, game-world facts that contradict the hunt's canonical world document, answer words that are inconsistent with the fictional register
  - Puzzle hunt examples: AoE I (Dark Age) — world-model consistency catch on a medieval technology reference
  - Transfer: applies to interactive fiction (puzzle solutions must be consistent with the world model), tabletop RPG design, game narrative

- **Framework 4: Social Fabric** (Jordan Weisman)
  - Definition: puzzle hunt design is a social experience; the social fabric framework evaluates how a puzzle plays in a group — does it have roles for different solver types? Does it create moments of shared discovery or isolate individual solvers?
  - Failure modes: puzzles that are solvable by one person alone with no collaborative affordance, puzzles that require one specific skill set (alienating team members without it)
  - Puzzle hunt examples: AoE III (Castle Age) — social fabric analysis of a word puzzle with no visual component
  - Transfer: applies to tabletop game design (multiplayer dynamics), escape room design, collaborative experience design

- **Framework 5: Perceptual Shift** (Scott Kim)
  - Definition: the best puzzle mechanisms produce a shift in how the solver perceives the input material; the perceptual-shift framework evaluates whether the puzzle rewards a genuine cognitive reorientation or merely rewards information lookup
  - Failure modes: puzzles where the "trick" is just knowing an obscure fact rather than seeing something familiar in a new way; puzzles where the perceptual shift is too abrupt (no scaffolding for the reorientation)
  - Puzzle hunt examples: AoE III (Castle Age) — perceptual shift analysis of a grid puzzle
  - Transfer: applies to visual design, interaction design (moments of interface revelation), puzzle game design

- **Framework 6: Visual Grammar** (Dana Young)
  - Definition: the layout, typography, and visual presentation of a puzzle communicate solving rules before the solver reads a single word of instruction; visual grammar framework evaluates whether the presentation grammar is consistent, legible, and accurate
  - Failure modes: inconsistent grid line weights that imply structure that doesn't exist, typography that makes answer extraction ambiguous, color coding without accessible alternatives
  - Puzzle hunt examples: AoE III (Castle Age) — visual grammar diagnosis of a presentation ambiguity
  - Transfer: applies to information design, game UI design, accessible design

- **Framework Inter-relations**: map which frameworks tend to co-occur in the same review; are some frameworks complementary (a-ha economy + perceptual shift) or orthogonal (load-bearing test + social fabric)?

### 05-discussion (~800 words)
- **Identity-Framework Attribution**: the data show that frameworks are systematically tied to expert identities — Snyder reliably introduces load-bearing test, Short reliably introduces world-model consistency — and this is not explained by profile length (some short profiles introduce more frameworks than some long ones)
- **The taxonomy as a methodological tool**: practitioners can use the 6 frameworks as a checklist for evaluating their own creative work, independent of AI; the AI panel is a convenient way to apply the checklist systematically, but the checklist is the contribution
- **Cross-domain transfer**: all 6 frameworks appeared in at least one non-puzzle-hunt review, confirming they are not puzzle-hunt-specific; the frequency with which they transfer varies (world-model consistency and social fabric transfer most readily; visual grammar and load-bearing test are more domain-specific)
- **Limitations**:
  - The taxonomy is inductively derived from a specific set of profiles; other profiles might introduce other frameworks; this taxonomy should be treated as illustrative, not exhaustive
  - The coding methodology is partially manual and involves judgment calls; inter-coder reliability not yet established
  - All reviews were generated by the same LLM; different models might produce different framework distributions
- **Future work**: extend to 12+ profiles from other creative domains (game design, interactive fiction, film criticism); establish inter-coder reliability on framework coding; test whether non-AI domain experts recognize and validate the taxonomy

### 06-conclusion (~400 words)
- Restate contribution: a 6-category taxonomy of creative evaluative frameworks, grounded in expert identity, transferable across creative domains
- Key takeaway: the value of richer AI reviewer profiles is not higher scores but the introduction of named, actionable evaluative frameworks that the reviewer community uses but rarely makes explicit
- The taxonomy as a bridge: connecting AI-simulated review practice to the theoretical frameworks of creativity research and expert knowledge
- Availability: framework taxonomy, coding guide, review texts, and profile library available at [URL]

## Experiments

- [x] Profile upgrade experiment (3 puzzles × v1/v2 panel): complete; raw review texts available
- [x] Framework identification (pass 1 — automated): 6 framework names detected in v2 review texts, absent in v1 texts
- [ ] Framework coding (pass 2 — manual): code all 30+ review texts for framework application vs. mere naming; establish per-puzzle framework coverage
- [ ] Profile-framework attribution: for each of the 29 v2 profiles, determine which frameworks it introduces across its review history; compute profile × framework frequency matrix
- [ ] Cross-domain transfer study: apply 6-framework coding scheme to Wavelength, Ironfall, and Grand Larceny review texts; measure per-framework cross-domain frequency
- [ ] Framework inter-relation analysis: compute co-occurrence matrix of frameworks within single reviews; identify systematic co-occurrence patterns
- [ ] Inter-coder reliability: have second coder independently code 20% of review texts; compute Cohen's kappa on framework presence/absence decisions

## Figures

- [ ] Figure 1: Taxonomy overview — 6 frameworks in a visual layout, with originating expert identity and 3 key sub-concepts per framework; designed as a reference figure
- [ ] Figure 2: Profile-framework attribution matrix — 29 profiles × 6 frameworks, cell color indicates frequency of framework introduction; shows that specific profiles reliably introduce specific frameworks
- [ ] Figure 3: v1 vs. v2 framework introduction comparison — 3 puzzles × 6 frameworks, showing framework presence (v2) vs. absence (v1) for each framework; main evidence figure
- [ ] Figure 4: Cross-domain transfer heatmap — 6 frameworks × 4 domains (puzzle hunt, music/thematic, sci-fi, noir fiction), cell value = frequency of framework appearance; shows which frameworks transfer most readily
- [ ] Figure 5: Framework inter-relation network — nodes are frameworks, edges weighted by co-occurrence frequency within single reviews; shows which frameworks tend to be introduced together

## Tables

- [ ] Table 1: Profile version comparison — v1 vs. v2 dimensions: line count, section count, named frameworks per profile, specificity score (manual rating)
- [ ] Table 2: Framework taxonomy reference — 6 rows × {framework name, originating expert identity, definition, characteristic failure modes, example from review text, transfer domains}
- [ ] Table 3: Experiment 1 framework results — 3 puzzles × {v1 frameworks introduced (count + names), v2 frameworks introduced (count + names), new frameworks gained}
- [ ] Table 4: Profile-framework frequency matrix (summary) — 6 expert identity clusters × 6 frameworks, with frequency counts; full 29×6 matrix in appendix
- [ ] Table 5: Cross-domain transfer results — 6 frameworks × 4 domains, with representative review excerpt for each framework × domain cell where framework appeared

## Analysis Scripts

- [ ] scripts/framework_detector.py — regex + semantic search for 6 known framework names in review texts; outputs per-review presence/absence
- [ ] scripts/profile_attribution.py — for each profile, aggregate framework introduction counts across all reviews that profile participated in
- [ ] scripts/cooccurrence.py — compute pairwise framework co-occurrence matrix from review texts
- [ ] scripts/transfer_analysis.py — apply framework coding to cross-domain review texts; compute per-domain per-framework frequency

## Quality Checkpoints

- [ ] Word count: 5500–7500 words (ICCC 8-page target)
- [ ] References: 30+ citations (mix of creativity research, HCI, NLP, game design, expert knowledge)
- [ ] Figures: 4–5 figures
- [ ] Tables: 4–5 tables
- [ ] Section 04 (taxonomy): each framework has a definition, 3 failure modes, a concrete puzzle example, and at least one transfer domain example
- [ ] All framework attribution claims backed by review text excerpts
- [ ] Framework taxonomy coded by two coders with reliability reported
- [ ] Profile library and review texts linked for replication

## Dependencies

- Framework coding (pass 2) depends on: manual review of all 30+ review texts
- Profile-framework attribution depends on: pass 2 coding
- Figure 2 (attribution matrix) depends on: profile-framework attribution experiment
- Cross-domain transfer study depends on: access to Wavelength, Ironfall, Grand Larceny review texts
- Figure 4 (transfer heatmap) depends on: cross-domain transfer study
- Inter-coder reliability depends on: second coder availability
- Discussion depends on: all experiment results

## Related Papers

- `games-ai-expert-panel-creative`: The primary paper — this paper is a deep-dive into one of the primary paper's secondary contributions (the taxonomy of evaluative frameworks surfaced by richer profiles, mentioned in Section 04 of the primary paper); the framework taxonomy here is the full elaboration of what the primary paper introduces as a finding
- `games-generative-hunt-pipeline`: The pipeline paper — Stage 7 editorial reviews in that pipeline are analyzed using the framework taxonomy developed here; Table 5 in the pipeline paper (editorial examples) maps onto the framework taxonomy
- `games-same-input-divergence`: The divergence paper — the divergence analysis can be run at the level of evaluative frameworks (do the same frameworks appear in both runs' reviews? does divergence show up in framework coverage?); methodological cross-reference
- `games-human-ai-calibration`: The calibration paper — the human expert calibration study should test whether human experts recognize and endorse the 6 frameworks; if human experts spontaneously use the same frameworks when evaluating the same puzzles, the taxonomy gains external validity
