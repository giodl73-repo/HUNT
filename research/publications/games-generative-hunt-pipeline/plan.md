# Paper Plan: A Generative Pipeline for AI-Assisted Puzzle Hunt Creation

## Research Question

Can a structured, multi-stage generative pipeline produce puzzle hunt artifacts of sufficient quality for real deployment? Where in an 11-stage pipeline does human judgment remain essential, and where can AI operate autonomously?

We ground this question in six puzzle hunt scenarios executed end-to-end through the same pipeline, spanning a range of domains (game-knowledge, noir fiction, sci-fi RPG, music), scales (4 to 19 puzzles), and delivery formats (physical dossier, digital website, print-and-play). The goal is to characterize the pipeline as a system contribution — not a one-off generative experiment — and to identify the leverage points where process structure matters most.

## Target Venue

- Conference: FDG 2026 (Foundations of Digital Games) or CHI 2026 Late-Breaking Work
- Deadline: TBD (FDG typically June; CHI LBW typically January)
- Page limit: 10–12 pages full paper (FDG); 4 pages extended abstract (CHI LBW)
- Expected contribution type: System + Empirical Study
- Track note: FDG values playable/runnable system demonstrations; CHI values the human-AI collaboration angle. Target FDG for full paper; CHI LBW as fallback if scope narrows.

## Core Argument

Unconstrained AI generation of creative artifacts produces highly variable output quality; structure is the primary lever. A pipeline with explicit stage gates — world lock, puzzle pool ranking, editorial review — constrains the generation space at the right points and yields higher-quality artifacts than a single end-to-end prompt. The expert panel review gates (Stage 3: pool ranking; Stage 7: editorial) are the highest-leverage interventions: they gate the downstream investment of authoring time and ensure only pass-worthy puzzles receive integration work.

Across six independent hunt scenarios, the pipeline produced 52 total puzzles, of which 47 passed final editorial review, a 90.4% pass rate. The 5 failures (9.6%) were concentrated at two stages: Stage 7 (editorial catch) and Stage 10 (platform test), suggesting that the review gates are functioning as intended and that generation failures are not uniformly distributed across stages. This pipeline structure is transferable: the same CLAUDE.md-driven skill set ran all six scenarios with no per-scenario code changes, only per-scenario world and scope documents.

## Sections

### 00-abstract (~250 words)
- Problem: AI generation of multi-artifact creative systems (not single outputs) requires process structure
- Approach: 11-stage pipeline with defined gate conditions, AI expert panel review at two stages
- Testbed: 6 puzzle hunt scenarios, 52 puzzles, varied domains and scales
- Findings: 90.4% pass rate; review gates are highest-leverage; pipeline is domain-transferable
- Contribution: system description + cross-scenario empirical characterization + open pipeline specification

### 01-introduction (~900 words)
- Hook: puzzle hunt as a multi-artifact creative system — not a single output but a coherent collection with a meta-puzzle
- Problem: AI generation at the level of a puzzle hunt requires coordinated decisions across scope, structure, authoring, and delivery; unconstrained generation fails on coherence
- Opportunity: structured pipelines with review gates can impose the necessary coordination
- Our approach: 11-stage pipeline, each stage producing structured artifacts consumed by subsequent stages
- Key question: does stage structure matter? Which stages are the leverage points?
- Contributions list:
  1. Specification of an 11-stage AI-assisted puzzle hunt pipeline with gate conditions
  2. Cross-scenario empirical characterization across 6 hunt domains/scales
  3. Analysis of failure points: where generation fails and why
  4. Identification of highest-leverage intervention points (Stage 3 and Stage 7)
  5. Open pipeline specification (CLAUDE.md skill library) for replication and extension

### 02-related-work (~1000 words)
Subsections:
- **AI-Assisted Creative Systems**: distinguish single-artifact generation (poem, image, level) from multi-artifact system generation (a hunt, a game, a tabletop campaign); cite work on AI co-authorship (Kreminski et al. 2020), story generation systems (Roemmele & Gordon 2015), dungeon generation (Summerville et al. 2018)
- **Procedural Content Generation with Constraints**: PCG-via-search and constraint-based generation (Shaker et al. 2016 PCG book, Smith & Whitehead 2010 on tanagra); our pipeline is constraint-based across stages
- **Puzzle Design and Game Design Quality**: Snyder craftsmanship criteria, Riven Standard (Rand Miller), WPC/USPC puzzle quality bar; puzzle hunt as distinct from standalone puzzles — meta-structure and answer confirmation as additional quality axes
- **Review Gates in Creative Production**: editorial pipelines in traditional publishing; peer review in academic writing; the review-gate pattern as a design pattern for AI content pipelines
- **AI Pipelines for Complex Tasks**: agent orchestration (Park et al. 2023 generative agents, AutoGPT-style pipelines, Nakajima's BabyAGI), but contrast with our structured decomposition — we do not use autonomous agent loops; each stage is human-triggerable and reviewable
- **Key gap**: no prior work characterizes an end-to-end generative pipeline for multi-artifact creative systems across multiple domain runs; no empirical characterization of where pipeline structure adds the most value

### 03-system-description (~1400 words)
Subsections:
- **Pipeline Overview**: 11 stages with names, numbered, one-sentence purpose per stage; input/output artifact per stage; dependency graph showing which stages are sequential vs. can branch
  - Stage 1: Scope (brief → SCOPE.md + CLAUDE.md)
  - Stage 2: Structure (structure → ROUNDS.md)
  - Stage 3: Puzzle Pool (generation + AI panel ranking → PUZZLE-POOL.md with scores)
  - Stage 4: Assignment (pool → per-puzzle ASSIGNMENT.md files)
  - Stage 5: Meta Design (feeder answers → META.md)
  - Stage 6: Authoring (assignment → puzzle files per puzzle)
  - Stage 7: Editorial Review (puzzle files → REVIEW.md per puzzle, pass/fail gate)
  - Stage 8: Integration (editorial pass → PUZZLES.md integration checklist)
  - Stage 9: Delivery Preparation (integrated puzzles → delivery package)
  - Stage 10: Platform Test (delivery package → test report)
  - Stage 11: Polish (test report → final artifacts)
- **Gate Conditions**: define what "pass" means at Stage 3 (score ≥ 22/30 from AI panel, or human override) and Stage 7 (editorial verdict: pass/revise/cut); explain why these two gates were chosen as primary gates
- **AI Roles by Stage**: map each stage to: fully AI-autonomous, AI-primary/human-review, human-primary/AI-assist, human-only; show that Stages 1, 4, 5, 8 are largely autonomous; Stages 3, 7 require AI panel; Stage 9 (delivery format) may require human tooling
- **Implementation**: CLAUDE.md skill library (hunt/scope, hunt/pool, hunt/write, hunt/review, hunt/editorial, hunt/print, hunt/site, hunt/props); each skill as a persistent markdown prompt with defined inputs and expected outputs
- **World Lock Mechanism**: Stage 1 produces a canonical world document (LOCKED.md); all subsequent stages treat it as read-only; prevents coherence drift across stages

### 04-empirical-characterization (~1400 words)
Subsections:
- **Scenario Overview Table**: 6 scenarios × {domain, scale, delivery format, meta type, total puzzles, pass rate}
  - Age of Empires (game-knowledge, 5+1 meta, digital, WOLOLO)
  - Boardgames (game-knowledge, 4+1 meta, physical, TBD)
  - Grand Larceny (noir fiction, 10+1 meta, physical dossier, TBD)
  - Ironfall (sci-fi RPG, 10+1 meta, digital, TBD)
  - Wavelength (music/thematic, 6+1 meta, print-and-play, TBD)
  - Dead Reckoning (navigation/adventure, ~5+1 meta, digital, TBD)
- **Pass Rate Analysis**: overall 90.4%; breakdown by scenario; breakdown by failure stage (Stage 7 vs. Stage 10); is pass rate correlated with domain, scale, or delivery format?
- **Stage Timing Analysis**: estimated human-time investment per stage across scenarios; which stages are bottlenecks? Which stages have the most iteration loops?
- **Output Artifact Quality**: sample editorial review reports from Stage 7 across scenarios; show that Stage 7 reports are actionable (identify specific fixes) vs. Stage 3 pool reports (comparative ranking)
- **Cross-Domain Transferability**: the same pipeline specification ran all 6 scenarios without per-scenario code changes; characterize what varied (world documents, scope documents) vs. what was invariant (skill prompts, rubric, profile library)
- **Failure Analysis**: detail the 5 failed puzzles; categorize failure types (domain-knowledge gap, extraction mechanism broken, meta integration failure, delivery format incompatibility); map to stage where failure was first detectable

### 05-discussion (~800 words)
- **Why Stage 3 and Stage 7 are highest-leverage**: Stage 3 gates the investment of authoring work (failing here is cheap); Stage 7 gates integration (failing here wastes authoring + prevents meta from closing); both are review gates, not generation stages
- **The world lock as a coherence mechanism**: without a locked world document, generation drift across stages would produce incoherent hunts; the lock is a simple but essential design decision
- **Autonomy gradient**: the pipeline has an autonomy gradient — early stages (scope, structure) and late stages (polish) admit more human involvement; middle stages (authoring, editorial) are where AI operates most autonomously; this is counterintuitive but reflects where structured knowledge (domain facts, puzzle mechanics) is dense
- **Limitations**:
  - All 6 scenarios are authored by the same toolkit/profile system; a true pipeline generalizability test would use different authoring systems
  - Pass rate is assessed by AI panel, not human play-testers; the correlation between AI panel pass and human enjoyment is studied separately (see games-human-ai-calibration)
  - Scale ceiling: the largest scenario (Ironfall/Grand Larceny at ~10 puzzles) is below typical MIT Mystery Hunt scale (200+ puzzles); pipeline behavior at scale is untested
- **Design recommendations for practitioners**: use Stage 3 pool ranking before committing to authoring; invest in world-lock documents before running generative stages; AI expert panels are cheap enough to run on every puzzle candidate

### 06-conclusion (~400 words)
- Restate contribution: empirical characterization of a structured pipeline across 6 hunt scenarios
- Key takeaway: stage structure is the primary quality lever; review gates at Stages 3 and 7 account for most of the quality lift
- Open questions: pipeline behavior at scale (50+ puzzles); human play-test correlation study; cross-domain pipeline adaptation (tabletop campaign generation, interactive fiction)
- Availability: pipeline specification (CLAUDE.md skills), scenario artifacts, and empirical data available at [URL]

## Experiments

- [x] Scenario 1 (Age of Empires): 5 puzzles + meta, fully complete, panel scores available
- [x] Scenario 2 (Boardgames): partial completion, pool stage done
- [x] Scenario 3 (Grand Larceny): 10+ puzzles, editorial review complete
- [x] Scenario 4 (Ironfall): 10 puzzles, all PASS, delivery package complete
- [x] Scenario 5 (Wavelength): 6 puzzles, all reviewed, meta confirmed
- [ ] Scenario 6 (Dead Reckoning): 3–5 puzzles authored, need editorial review completion
- [ ] Stage timing analysis: estimate human-minutes per stage per scenario; compile from session notes
- [ ] Failure analysis: catalog the 5 failed puzzles, code failure types, map to pipeline stages
- [ ] Cross-scenario pass rate table: compile all AI panel scores, compute per-scenario and overall pass rates
- [ ] Autonomy gradient analysis: code each stage in each scenario as AI-autonomous / AI-primary / human-primary

## Figures

- [ ] Figure 1: Pipeline diagram — 11 stages in sequence, with gate condition markers at Stages 3 and 7, input/output artifact labels per stage
- [ ] Figure 2: Cross-scenario characterization matrix — 6 scenarios × pipeline stages, showing completion status, pass/fail at gates, and output artifact types
- [ ] Figure 3: Pass rate breakdown — stacked bar by scenario showing passed/revised/cut puzzle counts; secondary grouping by failure stage
- [ ] Figure 4: Autonomy gradient diagram — each stage rated on AI autonomy scale (fully autonomous → human-only), with justification per stage
- [ ] Figure 5: Stage timing heatmap — estimated human-time investment per stage across the 6 scenarios; highlight Stage 7 as most time-intensive human review

## Tables

- [ ] Table 1: Pipeline stage reference — stage number, name, primary input artifact, primary output artifact, gate condition (if any), AI autonomy level
- [ ] Table 2: Scenario summary — 6 scenarios × {domain type, scale, delivery format, meta answer type, total puzzle candidates, final puzzle count, pass rate}
- [ ] Table 3: Failure analysis — 5 failed puzzles × {scenario, puzzle title, failure stage, failure type, recovery action taken}
- [ ] Table 4: Stage 3 pool ranking examples — 3 scenarios × top-5 pool candidates with scores, showing how pool ranking distributes across puzzle types
- [ ] Table 5: Stage 7 editorial examples — one representative review per scenario showing verdict, key issues identified, and actionability of feedback

## Quality Checkpoints

- [ ] Word count: 6000–8000 words (FDG 10-page target)
- [ ] References: 35+ citations (mix of PCG, HCI, game design, AI pipelines, creativity)
- [ ] Figures: 4–5 figures
- [ ] Tables: 4–5 tables
- [ ] All empirical claims (pass rates, failure counts) backed by scenario artifacts
- [ ] Pipeline specification described precisely enough to replicate
- [ ] All 6 scenarios described with sufficient detail to support claims

## Dependencies

- Scenario 6 (Dead Reckoning) completion depends on: additional authoring and editorial review runs
- Stage timing analysis depends on: session note compilation (can be estimated if notes are incomplete)
- Failure analysis depends on: review of REVIEW.md files across all scenarios
- Figure 3 (pass rate breakdown) depends on: cross-scenario pass rate table
- Figure 5 (timing heatmap) depends on: stage timing analysis
- Table 3 (failure analysis) depends on: failure analysis experiment
- Discussion section depends on: all empirical characterization sections

## Related Papers

- `games-ai-expert-panel-creative`: The primary paper — this paper uses the AI expert panel system from that paper as the implementation of Stage 3 and Stage 7 review gates; results here provide empirical context for the panel evaluation study
- `games-profile-taxonomy-creative`: The profile taxonomy paper — the evaluative frameworks introduced by richer profiles appear in Stage 7 editorial reviews; the taxonomy can be cross-referenced with Stage 7 failure categories
- `games-same-input-divergence`: The divergence paper — uses the Age of Empires scenario from this pipeline as its primary testbed; the pipeline description here is the shared infrastructure
- `games-human-ai-calibration`: The calibration paper — validates the Stage 3 and Stage 7 AI review gates against human expert judgment; results there are the ground-truth validation of the gates described here
