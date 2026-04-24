# Peer Review — CHI 2026

**Paper**: "Calibrating AI Expert Panel Scores Against Human Expert Judgment in Creative Domain Evaluation"
**Reviewer**: Michael Bernstein (Stanford University)
**Date**: 2026-03-01

---

## Summary

A calibration study comparing AI expert panel scores against human expert judgment for puzzle hunt quality assessment. The design is principled, the per-dimension analysis is the right approach, and the finding that AI panels are reliable on technical dimensions but not hedonic ones is a useful and credible result. My concerns are primarily methodological: the sample sizes (N=10 puzzles, N=5 reviewers) are too small to support the precision of the claims made, particularly the corrective scaling factors, and the puzzle selection procedure introduces a range-selection bias that inflates the reported correlations. These are substantive issues for a CHI empirical paper.

---

## Strengths

**1. The per-dimension ICC-reporting-as-baseline design is methodologically exemplary.**
Reporting human inter-rater reliability as the interpretation ceiling for AI-human correlation is a practice this field should adopt universally, and this paper does it correctly. Many AI-evaluation papers report AI-human correlation in a vacuum; the contextualization against human-human ICC is the right epistemological move.

**2. The profile version comparison is a well-designed within-study control.**
Using the existing v1/v2 score pairs for the AoE puzzles as a natural experiment on profile version effects is elegant. It avoids the need for a separate experiment while providing clean comparative evidence. The null result on Fun — profile richness doesn't help — is exactly the kind of targeted negative finding that calibration studies should provide.

**3. The qualitative analysis of score justifications bridges the quantitative results to a theoretical account.**
The contrast between human reviewers invoking social and temporal experience ("solving with my team, we'd all be groaning") versus AI reviewers invoking structural proxies ("clue density, misdirection surface area") is compelling and memorable. This is the kind of multi-method evidence that CHI values.

---

## Weaknesses

**1. N=10 puzzles is below the threshold for reliable Pearson r estimation at the effect sizes reported.**
The most fundamental methodological concern is the sample size for correlation analysis. With N=10 data points, Pearson r estimates are highly unstable. Simulation-based power analysis for detecting a "moderate" correlation of r=0.70 at 80% power requires N≈29; for detecting r=0.50 (the Fun result) at 80% power requires N≈37. The reported 95% confidence intervals reveal this problem: the Fun correlation CI [0.03, 0.81] is so wide that it is consistent with everything from an essentially null relationship to a strong positive correlation. The paper cannot distinguish between "AI Fun scores have a weak relationship with human Fun scores" and "AI Fun scores have no relationship with human Fun scores" at this sample size.

This is not merely a limitation to be acknowledged — it affects what the paper can conclude. The corrective scaling factor for Fun (LOO-CV R²=0.26) is not distinguishable from zero at this N. The paper's central practical contribution — the corrective scaling factors — rest on sample sizes too small to establish the factors' reliability. The paper should either (a) expand the sample to at least 25–30 puzzles (which appears feasible given the pipeline study's broader puzzle set), or (b) present the corrective factors explicitly as directional estimates with large uncertainty and withdraw the "practitioner-ready" characterization.

**2. Range selection inflates reported correlations.**
The puzzle selection procedure "span the AI panel score range" — that is, puzzles were selected to cover a wide range of AI scores rather than sampled representatively from the pipeline. This is a well-known source of inflation in correlation studies: selecting cases that maximize variance on one variable inflates the correlation with other variables. The reported correlations (Clarity r=0.88, Solvability r=0.84) likely overestimate the correlations that would be observed in a representative sample of puzzles from the production pipeline. The paper should (a) acknowledge this as a methodological limitation and (b) estimate the likely direction and magnitude of the inflation bias.

**3. N=5 human reviewers and the ICC confidence interval problem.**
The human ICC estimates, which serve as the interpretation baseline for all AI-human correlations, have wide confidence intervals of their own. Fun ICC = 0.58, 95% CI [0.44, 0.72]. The lower bound (0.44) would be "poor" reliability; the upper bound (0.72) would be "moderate-good." The AI-human Fun correlation (r=0.52) falls within this ICC CI — meaning we cannot statistically distinguish between "AI performs comparably to human-human reliability" and "AI falls substantially short of human-human reliability" for Fun. The paper's narrative assumes a sharp distinction between AI reliability and human reliability on Fun, but the overlapping confidence intervals undermine this claim.

---

## Minor Issues

1. The puzzle selection procedure specifies "score range spans at least 5 units" but Table 1 shows AI scores ranging from 24.3 to 28.2 — a range of 3.9 units. Either the criterion was not met or there is an inconsistency in reporting.
2. IRB/ethics approval is not mentioned; CHI requires human participant consent documentation.
3. "MARG" is cited as reference [darcy2024marg] but the full reference is not provided in the text.
4. The analysis scripts (calibration.py, icc.py, etc.) are described in plan.md but are not mentioned in the paper's results section. If they exist, they should be cited; if they don't yet exist, remove the reference.

---

## Scores

| Criterion | Score (1-4) |
|---|---|
| Novelty | 3 |
| Soundness | 2 |
| Significance | 3 |
| Presentation | 3 |
| **Average** | **2.75** |

**Recommendation**: Major Revision

The finding is credible and the design is basically sound, but the sample size issues are real and the paper overclaims precision it has not earned. The path to acceptance runs through either (a) expanding the sample to 25–30 puzzles to support the corrective factors, or (b) reframing the corrective factors as directional, population-level bias estimates and withdrawing the per-puzzle correction interpretation. Option (b) is feasible without new data; option (a) would substantially strengthen the paper. Both paths require the paper to address the range-selection bias and present appropriate confidence intervals for the ICC estimates.
