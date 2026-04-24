# Paper Plan: Expert AI Panels for Creative Work Evaluation

## Research Question

Can AI persona panels — groups of LLM instances embodying distinct domain expert identities — provide reliable, high-quality evaluation of creative artifacts? Does increasing profile specificity (richer persona descriptions, more reviewers) improve evaluation quality in measurable ways?

We ground this question in puzzle hunt design, a domain with well-defined quality criteria, measurable scoring rubrics, and a community of recognized expert practitioners whose evaluative styles are documented and distinctive.

## Target Venue

- Conference/journal: ICCC 2026 (International Conference on Computational Creativity)
- Deadline: TBD (typically March–April)
- Page limit: 8–10 pages
- Expected contribution type: Empirical Study with System Description
- Track note: ICCC values novelty of creative process and evaluation methodology; emphasize that this is a novel application domain (game/puzzle design) and that the empirical rigor distinguishes it from position papers

## Core Argument

Existing work on AI-simulated review (e.g., Berry et al. on AI paper review, persona-based prompting) focuses on academic paper evaluation. We extend this to creative domain evaluation, where the criteria are less standardized and require richer domain knowledge. Our system uses persistent expert profiles — structured markdown documents capturing each reviewer's design philosophy, characteristic concerns, evaluation lens, and voice — and shows that profile quality materially affects evaluation quality, not just verbosity.

Key empirical finding: upgrading from 12 generic profiles to 29 richer, domain-specific profiles produces an average score delta of +0.57 across puzzle quality dimensions, but more importantly produces more specific, transferable, and actionable feedback. The new profiles surface analytical frameworks (a-ha economy, load-bearing test, world-model consistency, social fabric, perceptual-shift) that the original profiles did not introduce.

## Sections

### 00-abstract (~250 words)
- Problem: creative work evaluation is hard, subjective, expensive
- Approach: AI expert panels with rich domain-specific profiles
- Testbed: puzzle hunt design (well-defined criteria, recognized experts)
- Findings: profile quality → feedback specificity; cross-domain generalizability
- Contribution: empirical method + open profiles + replication data

### 01-introduction (~800 words)
- Hook: puzzle hunt design as a creative domain with measurable quality
- Problem: evaluating creative work requires domain expertise that is scarce and expensive
- Opportunity: LLMs can simulate domain experts IF the persona is rich enough
- Our approach: persistent expert profiles + structured scoring rubric
- Key question: does profile quality matter? (answer: yes, and here's how to measure it)
- Contributions list:
  1. Profile-based AI expert panel system for creative work evaluation
  2. Empirical comparison of profile versions (12 → 29 profiles, v1 → v2)
  3. Cross-hunt generalizability study across 3 creative domains
  4. Open-source profile library for puzzle/game design evaluation
  5. Taxonomy of evaluation frameworks surfaced by richer profiles

### 02-related-work (~1000 words)
Subsections:
- **AI-Simulated Expert Review**: cite panel-reviewer-profiles (Berry et al.), panel-review-methodology, LLM-as-reviewer literature (Shah & Wein 2023, Liu et al. 2023)
- **Persona-Based Prompting**: Zhang et al. personas in dialogue, Park et al. generative agents, Dang et al. personas for creative writing
- **Creative Evaluation Methodology**: computational creativity evaluation (Colton & Wiggins 2012), puzzle design principles (Snyder's craftsmanship criteria, Riven Standard from Rand Miller), game design quality metrics
- **Puzzle Hunt as a Research Domain**: position the hunt as a legitimate creative artifact with measurable properties; distinguish from trivia (domain knowledge IS the mechanism)
- **Key gap**: no prior work applies AI expert panels to creative domain evaluation; no empirical study of profile quality effects on creative evaluation

### 03-methodology (~1200 words)
Subsections:
- **System Architecture**: profiles as persistent markdown artifacts; structured scoring rubric (6 dimensions, 1-5 scale, /30 total, pass threshold 22/30); panel assembly (3 reviewers per puzzle, drawn from profile library)
- **Profile Design**: what a profile contains (design philosophy, evaluation lens, characteristic concerns, voice, key works); difference between v1 (12 profiles, ~50 lines each) and v2 (29 profiles, ~80-120 lines each, with specific analytical frameworks named)
- **Scoring Rubric**: define all 6 dimensions (Clarity, Solvability, Elegance, Reading Reward, Fun, Confirmation) with examples; explain why these dimensions matter for creative evaluation specifically
- **Testbed: Puzzle Hunt Design**: describe the domain; explain why it's appropriate (verifiable correct answers, documented expert community, well-defined quality bar, multiple hunt types provide natural domain variation)
- **Experimental Design**:
  - Experiment 0 (Profile Ablation): same 3 puzzles × 3 conditions (no profile, v1, v2); full gradient from baseline to domain-specific; measures score, feedback specificity, named frameworks, actionable fixes
  - Experiment 1 (Profile Upgrade, subsumed by E0): the Condition 1→2 arm of Experiment 0 with data already collected
  - Experiment 2 (Cross-Hunt Generalizability): extend re-testing to 25 puzzles across 3 hunt types
  - Experiment 3 (Same-Input New Hunt): generate a second hunt from same world data; compare structural decisions

### 04-evaluation (~1500 words)
Subsections:
- **Experiment 0 Results** (profile ablation, 3 conditions × 3 puzzles):
  - Three-condition score table: No Profile vs. v1 vs. v2 per puzzle per dimension
  - Score gradient: expected monotonic increase; v1→v2 delta already known (+0.57 average)
  - Qualitative gradient: No Profile produces generic criteria ("is it fun?"); v1 produces domain labels ("Reading Reward is weak"); v2 produces specific mechanisms ("Bonus B requires back-solving, which is fairness-category failure, not difficulty")
  - Named frameworks per review: ~0 (Condition 0) → ~1-2 (Condition 1) → ~3-5 (Condition 2)
  - Actionable fixes per review: sparse → general → specific (the key contribution)
  - Key finding: score delta modest across all conditions; feedback quality delta is the signal
- **Experiment 2 Results** (cross-hunt generalizability, 25 puzzles):
  - Extend re-testing across AoE (5 puzzles), Wavelength (6 puzzles), Dead Reckoning (3-5 puzzles)
  - Does the +0.57 average hold across domains? Table of results by hunt type
  - Which analytical frameworks transfer across domains?
  - Hunt-type variation: game-knowledge vs. music vs. sci-fi
- **Experiment 3 Results** (same-input, new hunt):
  - Run full pipeline twice on AoE2 world; compare puzzle pool selection, meta design, answer words
  - Structural divergence: which choices are stable vs. variable?
  - Quality comparison: are second-run puzzles better, worse, or different?
- **Reliability Analysis**: intra-panel agreement; do different 3-reviewer subsets from v2 profiles reach same verdicts?

### 05-discussion (~800 words)
- **Why feedback quality matters more than score delta**: a richer panel that names specific problems is more useful than one that scores higher but vaguely
- **The profile as a design artifact**: profiles encode evaluative frameworks; the framework taxonomy (a-ha economy etc.) is a contribution in itself
- **Limitations**:
  - AI personas are not human experts; scores may not predict human reception
  - Domain specificity: puzzle hunt design may not generalize to all creative domains
  - Ground truth problem: no oracle for "correct" creative quality score
- **Implications for AI-assisted creative pipelines**: where AI expert panels fit (pre-human review, brainstorm gating, comparison across design alternatives)
- **Future work**: ablation of profile components; human expert validation study; other creative domains (game design, interactive fiction, level design)

### 06-conclusion (~400 words)
- Restate contribution: empirical evidence that profile quality matters for AI creative evaluation
- Key takeaway: the feedback quality gap between v1 and v2 profiles is more practically important than the score gap
- Open questions: what else can you evaluate this way? What makes a good profile?
- Availability: profiles, scoring rubric, and replication data available at [URL]

## Experiments

- [ ] Experiment 0 (NEW — Ablation): Profile depth ablation — same 3 puzzles (AoE I, II, III), three conditions:
  - Condition 0: No profiles (bare prompt — "You are an expert reviewer. Score this puzzle on 6 dimensions.")
  - Condition 1: Generic profiles v1 (12 profiles, ~50 lines, general design expertise)
  - Condition 2: Domain-specific profiles v2 (29 profiles, ~80-120 lines, puzzle-specific frameworks)
  - Measure: score per dimension, feedback word count, named frameworks per review, actionable fixes identified
  - Expected result: monotonic improvement across conditions; largest jump at Condition 1→2 (domain specificity)
- [x] Experiment 1 (completed): Profile upgrade comparison — 3 puzzles (AoE I, II, III), v1 vs. v2 panel
  - NOTE: Experiment 1 is now the Condition 1 → Condition 2 arm of Experiment 0
- [ ] Experiment 2: Cross-hunt generalizability — extend to 25 puzzles across AoE (5), Wavelength (6), Dead Reckoning (4)
  - Script: run parallel re-test agents per puzzle, collect scores, compute deltas
- [ ] Experiment 3: Same-input new hunt — re-run AoE2 pipeline, compare pool/meta/answers
  - Generate second hunt → compare PUZZLE-POOL.md selections, ROUNDS.md structure, meta answer
- [ ] Reliability analysis: run same puzzle (AoE II) through 3 different 3-reviewer subsets from v2 profiles; measure variance

## Figures

- [ ] Figure 1: System architecture diagram — profile → panel assembly → review → synthesis → score
- [ ] Figure 2: Ablation bar chart — score by condition (No Profile / v1 / v2) per dimension across 3 puzzles (replaces simple v1 vs v2)
- [ ] Figure 3: Feedback quality taxonomy — 6 new analytical frameworks, with puzzle-hunt examples
- [ ] Figure 4: Cross-hunt generalizability — score delta by hunt type (AoE / Wavelength / Dead Reckoning)
- [ ] Figure 5: Same-input divergence diagram — two hunts from same world, showing structural variation

## Tables

- [ ] Table 1: Profile comparison (v1 vs v2) — line count, sections, frameworks per profile
- [ ] Table 2: Scoring rubric — 6 dimensions with definitions and examples
- [ ] Table 3: Experiment 1 results — full score breakdown by puzzle and reviewer
- [ ] Table 4: Cross-hunt results — score delta by hunt type and puzzle type
- [ ] Table 5: Actionable fixes surfaced — puzzle, original panel verdict, new panel verdict, fix type

## Analysis Scripts

- [ ] scripts/compute_deltas.py — given two sets of scores, compute per-dimension and overall deltas
- [ ] scripts/framework_taxonomy.py — extract named frameworks from review texts, classify, count
- [ ] scripts/reliability.py — compute inter-rater agreement (Cohen's kappa equivalent) for AI panels
- [ ] scripts/hunt_comparison.py — diff two PUZZLE-POOL.md and ROUNDS.md files, compute structural divergence

## Quality Checkpoints

- [ ] Word count: 5500–7500 words (ICCC typical 8 pages)
- [ ] References: 30+ citations (mix of HCI, NLP, game design, creativity)
- [ ] Figures: 4–5 figures
- [ ] Tables: 4–5 tables
- [ ] All empirical claims backed by data from experiments
- [ ] Profile library described and linked
- [ ] Replication package described

## Key Data Available Now

### Experiment 1 (complete)
| Puzzle | v1 Score | v2 Score | Delta | New Frameworks Introduced |
|--------|----------|----------|-------|--------------------------|
| AoE I (Dark Age) | 24.3/30 | 24.7/30 | +0.4 | World-model consistency, double-confirmation |
| AoE II (Feudal) | 26.0/30 | 26.3/30 | +0.3 | A-ha economy, load-bearing test |
| AoE III (Castle) | 24.0/30 | 25.0/30 | +1.0 | Social fabric, perceptual-shift, visual grammar |
| **Average** | **24.8** | **25.3** | **+0.57** | |

### Puzzles ready for Experiment 2 (25 total)
- Age of Empires: I, II, III, IV, V-v2 (5 puzzles) — all have baseline scores
- Wavelength: P1–P6 (6 puzzles) — scores 25.3–29.0
- Ironfall: P01–P10 (10 puzzles) — verified, all PASS
- Dead Reckoning: R1-01, R1-02, R1-03 (3 puzzles, more available) — new hunt, no baselines yet

## Dependencies

- Experiment 2 depends on: parallel re-test runs (parallel agents)
- Figure 4 depends on: Experiment 2 results
- Table 4 depends on: Experiment 2 results
- Experiment 3 depends on: second AoE2 hunt generation (run toolkit pipeline again)
- Figure 5 depends on: Experiment 3
- Reliability analysis depends on: running AoE II through 3 different reviewer subsets
- Results section depends on: all experiments
- Discussion depends on: results section

## Related Papers in This Repository

- `panel/panel-reviewer-profiles`: Token-efficient persona simulation — our profile system extends this
- `panel/panel-reviewer-calibration`: Calibrating AI reviewer personas — our domain application
- `panel/panel-review-methodology`: AI-simulated expert review — foundational methodology we build on
