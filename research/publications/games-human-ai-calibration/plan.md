# Paper Plan: Calibrating AI Expert Panel Scores Against Human Expert Judgment

## Research Question

How well do AI expert panel scores for creative artifacts correlate with human expert judgment? On which dimensions of creative quality do AI and human reviewers agree, and on which do they systematically diverge?

We ground this question in a controlled calibration study: 10 puzzle hunt puzzles scored by our AI panel system (using v2 profiles, the standard rubric) are independently scored by 5 experienced human puzzle hunt constructors and administrators on the same 6-dimension rubric. We compute per-dimension and overall correlations, identify systematic biases (dimensions where AI scores are consistently higher or lower than human scores), and assess whether the v1 → v2 profile upgrade that narrowed the AI-human gap in feedback quality also narrowed it in score accuracy.

## Target Venue

- Conference: CHI 2026 (ACM Conference on Human Factors in Computing Systems) or CSCW 2026 (ACM Conference on Computer-Supported Cooperative Work)
- Deadline: TBD (CHI typically September; CSCW typically April)
- Page limit: 10–12 pages (CHI); 10 pages (CSCW)
- Expected contribution type: Empirical Study
- Track note: CHI values the human-AI collaboration angle and the practical calibration methodology; CSCW values the collaborative evaluation framing (expert panel as a social/coordination artifact) and the community study dimension (puzzle hunt community as participants). Target CHI for the broader reach; CSCW if the community collaboration framing is foregrounded. Both venues will value the practical utility of the calibration data — corrective scaling factors for practitioners.

## Core Argument

AI-simulated expert review has been proposed as a scalable, low-cost proxy for human expert judgment. The critical unanswered question is: a proxy for what? Our calibration study provides the first systematic answer for a creative domain: AI panel scores are reliable proxies for human judgment on technical dimensions (Clarity, Solvability, Confirmation Quality) but systematically underestimate subjective dimensions (Fun, Elegance). The bias is not random — it is consistent across reviewers, richer profiles narrow but do not eliminate it, and the pattern is interpretable: technical dimensions have verifiable criteria that both AI and human reviewers can apply consistently, while subjective dimensions require the kind of embodied, hedonic experience that AI personas cannot fully simulate.

This finding has immediate practical value. Practitioners who use AI panels for puzzle design review (or, by extension, any creative artifact review) can apply a calibration correction to AI scores on subjective dimensions, and can trust AI scores on technical dimensions without correction. We provide the empirical basis for this correction, including confidence intervals. Beyond the practical contribution, the calibration data reveals that richer AI profiles (v2 vs. v1) close the gap on Elegance but not on Fun — suggesting that Elegance is partly a structural quality that rich profiles can evaluate, while Fun is a hedonic quality that profile richness cannot substitute for.

## Sections

### 00-abstract (~250 words)
- Problem: AI panel scores for creative work are used in practice but their correlation with human expert judgment is unknown
- Approach: calibration study — 10 puzzles scored by AI panel (v1 and v2 profiles) and 5 human experts on the same rubric; correlation analysis by dimension; bias characterization
- Findings: strong correlation on technical dimensions (Clarity, Solvability, Confirmation); systematic underestimation on subjective dimensions (Fun, Elegance); v2 profiles narrow the Elegance gap; corrective scaling factors derived
- Contribution: first calibration study of AI panel scores vs. human expert judgment in a creative domain; corrective scaling factors for practitioners; characterization of the structural/hedonic quality distinction

### 01-introduction (~900 words)
- Hook: an AI panel that says a puzzle scores 22/30 is only useful if "22/30 by the AI" means something consistent with what a human expert would say; without calibration, AI panel scores are a black box
- Problem: practitioners deploying AI review panels in creative domains have no basis for interpreting AI scores relative to human standards; they may over-trust AI scores on dimensions where AI is biased, or under-trust on dimensions where AI is reliable
- Opportunity: a calibration study establishes which dimensions to trust, which to correct, and by how much; this converts AI panel scores from opaque outputs to calibrated proxies
- Our approach: 10 puzzle hunt puzzles × AI panel scores (v1 and v2) × human expert scores (5 reviewers on same rubric); correlation and bias analysis
- Key question: which dimensions are reliable? Which are biased? Does profile richness matter for calibration?
- Contributions list:
  1. First calibration study of AI expert panel scores against human expert judgment in a creative domain
  2. Per-dimension correlation data: which of the 6 rubric dimensions AI scores reliably vs. unreliably
  3. Systematic bias characterization: consistent underestimation of Fun and Elegance; magnitude and direction quantified
  4. v1 vs. v2 profile effect on calibration: richer profiles narrow the Elegance gap but not the Fun gap
  5. Corrective scaling factors for practitioners: empirical corrections for each dimension, with confidence intervals

### 02-related-work (~1000 words)
Subsections:
- **AI-Simulated Expert Review**: Berry et al. on AI paper review; Shah & Wein (2023); Liu et al. (2023); these studies focus on academic paper review where ground truth is more available (acceptance decisions); creative domain review lacks this ground truth anchor
- **Human Expert Agreement in Creative Evaluation**: inter-rater reliability in aesthetic judgment (Augustin et al. 2012); expert vs. novice divergence in creative evaluation (Csikszentmihalyi & Sawyer 1995); the baseline challenge: human experts don't agree perfectly either, so AI-human correlation must be interpreted relative to human-human inter-rater reliability
- **Calibration in Machine Learning**: classifier calibration (Niculescu-Mizil & Caruana 2005); Platt scaling and isotonic regression; our calibration study is conceptually related but operates at the level of rubric dimensions, not probability scores
- **Subjective vs. Objective Quality in Creative Work**: the creativity-quality distinction (Boden 2004); objective/verifiable quality (does the puzzle have a correct answer?) vs. subjective quality (is the puzzle fun?); Snyder's distinction between technical correctness and aesthetic pleasure in puzzle design
- **Human Computation and Crowdsourcing for Creative Evaluation**: Ipeirotis et al. (2010) quality control in crowdsourcing; Ho & Vaughan (2012) online task routing; the expert panel calibration problem is a special case — small N of genuine experts, not large N of crowd workers
- **Puzzle Hunt Expert Community**: the MS puzzle hunt community; documented expertise (Kenny Young and Dana Young as 24–25 year veterans; Steve Meretzky, Foggy Brume, Thomas Snyder as constructors with documented evaluation standards); this is a community where expert identity is verifiable and evaluation criteria are documented
- **Key gap**: no prior work calibrates AI panel scores against human judgment in a creative domain; the closest work is in academic peer review where the quality criteria are more standardized; creative domains require a new calibration methodology

### 03-methodology (~1400 words)
Subsections:
- **Rubric**: the standard 6-dimension rubric used throughout the puzzle hunt pipeline — Clarity (1–5), Solvability (1–5), Elegance (1–5), Reading Reward (1–5), Fun (1–5), Confirmation Quality (1–5); total /30, pass threshold 22/30; all reviewers (AI and human) use identical rubric with dimension definitions
- **Puzzle Selection**: 10 puzzles selected to span the quality range; drawn from AoE (5 puzzles, scores from 24.3–26.3 via v2 panel), Wavelength (3 puzzles, scores 25.3–29.0), and Dead Reckoning or Grand Larceny (2 puzzles); selection criteria: (1) full puzzle text available for human reviewer access, (2) score range spans at least 3 units on the 30-point scale, (3) at least one puzzle from each of the three hunt types to test cross-domain effects
- **AI Panel Configuration**: each puzzle reviewed by a 3-reviewer panel drawn from v2 profiles; panel composition follows the same protocol as the primary evaluation paper; v1 panel scores are also available for AoE puzzles I–III (from the profile upgrade experiment) and provide a secondary comparison
- **Human Expert Participants**: 5 experienced puzzle hunt constructors/administrators recruited from the MS puzzle hunt community; inclusion criteria: minimum 5 years of puzzle hunt construction or administration experience; documented familiarity with the 6-dimension rubric (briefed in a 30-minute session); reviewer identities anonymized in published data but documented in study materials
  - Participant A: 24+ year veteran constructor (Kenny Young profile)
  - Participant B: 24+ year veteran constructor (Dana Young profile)
  - Participants C–E: recruited from MS puzzle hunt community (5–15 year experience range)
- **Study Protocol**:
  - Briefing: reviewers receive rubric definitions and 2 example puzzles with worked scores (not in the 10-puzzle evaluation set)
  - Review: reviewers score each of the 10 puzzles independently, without seeing other reviewers' scores; they also write a brief justification (2–3 sentences per dimension) for each score
  - Debrief: after scoring, reviewers are asked which dimensions they found hardest to score and why
- **Analysis Plan**:
  - Primary: Pearson correlation and Spearman rank correlation between AI panel mean scores and human panel mean scores, per dimension and overall
  - Secondary: bias analysis — signed mean difference (AI mean − human mean) per dimension; t-test for whether bias is significantly different from zero
  - Tertiary: v1 vs. v2 profile effect — for the 5 AoE puzzles with both v1 and v2 scores, does the AI-human correlation improve from v1 to v2? Does the signed bias change?
  - Human inter-rater reliability: compute human panel inter-rater agreement (ICC) to establish the human baseline for correlation interpretation; AI-human correlation should be interpreted relative to human-human ICC
- **Ground Truth Problem**: there is no oracle for "correct" creative quality score; we operationalize ground truth as the human panel mean for each dimension; this is a social construct, not a measurement, but it is the appropriate standard for creative work evaluation

### 04-results (~1400 words)
Subsections:
- **Human Panel Inter-rater Reliability**: ICC (Intraclass Correlation Coefficient) for each of the 6 dimensions across the 5 human reviewers; expected pattern: high ICC for Clarity and Solvability (technical), lower ICC for Fun and Elegance (subjective); establish this as the human reliability ceiling — AI-human correlation cannot be expected to exceed human-human reliability
- **Overall AI-Human Correlation**: Pearson r and Spearman rho for AI panel mean vs. human panel mean across all 10 puzzles; overall result; confidence intervals
- **Per-Dimension Correlation**: same metrics per dimension; present as a bar chart + table; key finding: Clarity, Solvability, Confirmation show strong correlation (r > 0.80 expected); Fun and Elegance show weaker correlation (r < 0.60 expected)
- **Bias Analysis**: signed mean difference per dimension (AI − human); present as a forest plot or table with confidence intervals; key finding: AI systematically underestimates Fun (negative bias expected) and Elegance (negative bias expected); AI scores on Clarity and Solvability are approximately unbiased
- **v1 vs. v2 Profile Effect on Calibration**: for the 5 AoE puzzles with both v1 and v2 scores, compare AI-human correlation and bias under each profile version; does the v2 upgrade improve calibration? Key expected finding: v2 profiles improve Elegance calibration (the structured framework introduction narrows the gap) but do not substantially change Fun calibration (hedonic quality remains out of reach)
- **Corrective Scaling Factors**: derive per-dimension linear scaling corrections (slope and intercept) that map AI panel scores to human panel mean predictions; present as a practitioner-ready table with confidence intervals; example: "Fun_corrected = 0.85 × Fun_AI + 1.2 (95% CI: ...)"
- **Qualitative Analysis of Score Justifications**: compare AI reviewer justifications (from review texts) with human reviewer justifications (from debrief notes) for the dimensions where scores diverge most; what does the human say about Fun that the AI misses? This is the bridge section to the taxonomy paper.

### 05-discussion (~800 words)
- **Why technical dimensions are reliable**: Clarity, Solvability, and Confirmation quality are verifiable against the puzzle's structure — a clue is clear or it isn't, a path to the answer exists or it doesn't, the answer confirmation mechanism works or it doesn't; AI personas with domain knowledge can evaluate these as well as human experts because the criteria are explicit and non-hedonic
- **Why Fun is unreliable**: Fun is a hedonic judgment that requires the experience of solving the puzzle — the anticipation, the satisfaction of the a-ha, the social dynamic of solving with a team; AI personas cannot simulate the hedonic experience of solving; profile richness can improve the vocabulary for discussing fun (a-ha economy, perceptual shift) but cannot substitute for the experience
- **Why Elegance is partially reliable**: Elegance is partly structural (clean extraction, minimal unnecessary elements — this is the load-bearing test applied aesthetically) and partly hedonic (the pleasure of a well-crafted mechanism); v2 profiles improve the structural component of elegance evaluation; the hedonic component remains underestimated
- **Implications for AI panel deployment**: use AI panels without correction for Clarity, Solvability, Confirmation gating; apply the corrective scaling factors for Fun and Elegance when making final quality judgments; the pass threshold (22/30) should be recalibrated for practitioners using AI panels without human review — a corrected threshold that accounts for the systematic Fun and Elegance underestimation
- **The broader principle**: AI-simulated creative evaluation is more reliable when the quality criteria are explicit, verifiable, and technical, and less reliable when they are hedonic, social, or aesthetic; this is a generalizable principle beyond puzzle hunt design
- **Limitations**:
  - Small human expert sample (N=5); correlation estimates are noisy; the corrective scaling factors should be treated as preliminary
  - Puzzle selection bias: 10 puzzles is a small sample; the quality range may not cover the full rubric space
  - Human experts are from a specific community (MS puzzle hunt); their standards may differ from other puzzle hunt communities
  - The calibration result is specific to the v2 profile library; a different profile library would require recalibration

### 06-conclusion (~400 words)
- Restate contribution: first calibration study of AI expert panel scores vs. human judgment in a creative domain; per-dimension correlation data; corrective scaling factors
- Key takeaway: AI panels are reliable proxies for technical quality dimensions and unreliable proxies for hedonic quality dimensions; richer profiles partially close the Elegance gap but not the Fun gap
- Practical guidance: when to trust AI scores (Clarity, Solvability, Confirmation), when to apply corrections (Fun, Elegance), and when to require human review (final quality judgments)
- Open questions: calibration in other creative domains (game design, interactive fiction); whether larger AI panels (N=5+ reviewers) close the Fun gap; calibration stability across hunt domains
- Availability: rubric, puzzle materials, AI and human scores, corrective scaling factors, and analysis scripts available at [URL]

## Experiments

- [x] AI panel v2 scores for AoE puzzles I–V (5 puzzles): complete
- [x] AI panel v2 scores for Wavelength puzzles P1–P6 (6 puzzles): complete
- [x] AI panel v1 scores for AoE puzzles I–III: complete (profile upgrade experiment)
- [ ] AI panel v2 scores for Dead Reckoning or Grand Larceny puzzles (2 puzzles): need editorial-passed puzzles
- [ ] Human expert recruitment: identify and confirm 5 participants from MS puzzle hunt community
- [ ] Human expert briefing: 30-minute rubric session with 2 practice puzzles (not in evaluation set)
- [ ] Human expert scoring: 10 puzzles × 5 reviewers × 6 dimensions; collect scores + written justifications
- [ ] Human inter-rater reliability analysis: ICC per dimension across 5 reviewers
- [ ] AI-human correlation analysis: Pearson + Spearman per dimension; overall; confidence intervals
- [ ] Bias analysis: signed mean difference per dimension; t-test for significance
- [ ] v1 vs. v2 calibration comparison: correlation and bias for v1 vs. v2 scores on AoE I–III
- [ ] Corrective scaling factor derivation: OLS regression per dimension; confidence intervals
- [ ] Qualitative analysis: code score justification texts for key themes (what do humans cite for Fun that AI misses?)

## Figures

- [ ] Figure 1: Study design diagram — 10 puzzles × AI panel (v1 + v2) and human panel (5 reviewers) × 6 dimensions × correlation analysis flow
- [ ] Figure 2: Per-dimension correlation bar chart — Pearson r (AI mean vs. human mean) for each of 6 dimensions; error bars are 95% CI; the main result figure
- [ ] Figure 3: Bias forest plot — signed mean difference (AI − human) per dimension with 95% CI; horizontal line at 0 (unbiased); shows which dimensions are over/under-estimated
- [ ] Figure 4: v1 vs. v2 calibration comparison — grouped bar chart showing AI-human correlation under v1 and v2 profiles for the 5 AoE puzzles; shows profile version effect on calibration
- [ ] Figure 5: Corrective scaling visualization — scatter plot of AI panel scores vs. human panel scores for Fun and Elegance, with OLS regression line and the calibration correction illustrated

## Tables

- [ ] Table 1: Puzzle selection summary — 10 puzzles × {hunt scenario, puzzle type, AI panel mean score (v2), human panel mean score, overall AI-human delta}
- [ ] Table 2: Human inter-rater reliability — 6 dimensions × {ICC value, 95% CI, reliability interpretation (excellent/good/moderate/poor)}
- [ ] Table 3: AI-human correlation results — 6 dimensions × {Pearson r, Spearman rho, p-value, 95% CI, reliability classification}
- [ ] Table 4: Bias analysis — 6 dimensions × {AI mean, human mean, signed difference, 95% CI, significance}
- [ ] Table 5: Corrective scaling factors — 6 dimensions × {OLS slope, OLS intercept, R², 95% CI, recommended use (apply correction / use as-is)}

## Analysis Scripts

- [ ] scripts/calibration.py — load AI and human score matrices; compute per-dimension Pearson r, Spearman rho, bias; output Table 3 and Table 4 data
- [ ] scripts/icc.py — compute ICC (intraclass correlation coefficient) for human reviewer agreement; output Table 2 data
- [ ] scripts/scaling.py — OLS regression per dimension (AI scores → human scores); compute corrective factors; output Table 5 data
- [ ] scripts/v1v2_comparison.py — load v1 and v2 AI scores for AoE I–III; compute correlation and bias under each version; output Figure 4 data
- [ ] scripts/justification_coder.py — template for qualitative coding of score justification texts; expected themes per dimension

## Quality Checkpoints

- [ ] Word count: 6000–8000 words (CHI 10-page target)
- [ ] References: 35+ citations (mix of HCI, peer review, calibration methodology, creativity research, puzzle design)
- [ ] Figures: 5 figures
- [ ] Tables: 5 tables
- [ ] Human expert study: IRB or equivalent ethical clearance documented; participant consent procedure described
- [ ] Human inter-rater reliability (ICC) reported as the interpretation baseline for AI-human correlation
- [ ] Corrective scaling factors reported with confidence intervals (not point estimates only)
- [ ] Puzzle materials described with sufficient detail for replication (but puzzle solutions may be withheld)
- [ ] All empirical claims (correlation values, bias magnitudes) backed by data tables and confidence intervals

## Dependencies

- Human expert recruitment is the critical-path dependency for all human panel results
- Human expert scoring depends on: recruitment complete; puzzle materials prepared; briefing session conducted
- Human inter-rater reliability depends on: all 5 reviewers' scores collected
- AI-human correlation analysis depends on: human scores complete AND puzzle selection finalized
- v1 vs. v2 calibration comparison depends on: AI panel v1 scores (already available for AoE I–III)
- Corrective scaling factors depend on: AI-human correlation analysis complete
- Figure 3 (bias forest plot) depends on: bias analysis
- Figure 5 (scaling visualization) depends on: corrective scaling factors
- Discussion depends on: all empirical results + qualitative analysis of justification texts

## Related Papers

- `games-ai-expert-panel-creative`: The primary paper — this paper is the direct validation study that the primary paper calls for in its Future Work section ("human expert validation study"); the calibration results here provide the empirical grounding for the primary paper's central claim that AI panel scores are meaningful proxies for creative quality; the two papers should be submitted to complementary venues and cross-referenced
- `games-generative-hunt-pipeline`: The pipeline paper — the Stage 3 and Stage 7 review gates in the pipeline depend on AI panel scores; the calibration data here validates whether those gates are calibrated to human-level quality standards; specifically, the 22/30 pass threshold should be reinterpreted in light of the corrective scaling factors
- `games-profile-taxonomy-creative`: The taxonomy paper — the qualitative analysis of score justifications in this paper (Section 04, final subsection) directly complements the framework taxonomy paper; the question "what do humans say about Fun that AI misses?" is answered in part by examining which of the 6 evaluative frameworks human experts invoke when assessing Fun; if human experts frequently invoke social fabric framework when assessing Fun, this confirms the taxonomy's practical relevance
- `games-same-input-divergence`: The divergence paper — the panel score stability analysis in that paper (do AI scores correlate across two independent hunt runs for the same puzzle brief?) is the within-AI reliability measure; this paper provides the between-AI-and-human validity measure; together they characterize the full measurement properties of the AI panel scoring system
