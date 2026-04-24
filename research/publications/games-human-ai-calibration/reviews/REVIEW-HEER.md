# Peer Review — CHI 2026

**Paper**: "Calibrating AI Expert Panel Scores Against Human Expert Judgment in Creative Domain Evaluation"
**Reviewer**: Jeffrey Heer (University of Washington)
**Date**: 2026-03-01

---

## Summary

A calibration study quantifying AI-human score agreement for puzzle hunt evaluation, with per-dimension correlation and bias analysis. The finding that technical dimensions are AI-reliable while hedonic dimensions are not is credible; the profile version comparison sharpening the Elegance/Fun distinction is methodologically clean. My concerns focus on (1) the absence of the five key figures described in the paper, which are essential for communicating the calibration data at a CHI venue; (2) the practical precision problem with the corrective scaling factors, which have uncertainty ranges too wide to be used as per-puzzle corrections; and (3) the pass threshold recalibration recommendation, which is presented as a point value when it should be a decision-theoretic analysis.

---

## Strengths

**1. The figure design plan is well-conceived.**
The five figures described in the plan — forest plot for bias, per-dimension correlation bar chart with CIs, scatter plots for Fun and Elegance corrective scaling, v1/v2 grouped bar chart — are exactly the right visualizations for this data. The forest plot with horizontal reference at zero is the standard and appropriate format for signed-bias presentation; the scatter plots with OLS regression overlays are the right format for calibration factor visualization. The figure plan demonstrates strong visualization judgment.

**2. The table structure effectively communicates the per-dimension pattern.**
Tables 3 (correlation) and 4 (bias) are well-organized and present the data at the right granularity. The "Reliability" classification column in Table 3 (Strong/Moderate/Weak) and the significance annotation in Table 4 make the key pattern (technical reliable, hedonic not) immediately readable from the tables.

**3. The qualitative analysis adds essential interpretive depth.**
The contrast between human reviewers' experiential justification language ("solving with my team, we'd all be groaning in the best way") and AI reviewers' structural proxy language is memorable and communicatively effective. For a CHI audience accustomed to mixed-methods work, the qualitative bridge to the quantitative finding is appropriate and well-executed in this section.

---

## Weaknesses

**1. The five key figures are absent from the draft.**
This is the most significant presentational concern. CHI is a venue that evaluates papers in part on how they communicate visually; the five described figures — forest plot, correlation bar chart, v1/v2 comparison, scaling scatter plots — are not in the submitted paper. They are described accurately in the paper (what each will show) and planned in accompanying materials, but the figures themselves are not present. A CHI submission without its figures cannot be fully evaluated for visual communication quality, and the figures are essential for communicating the calibration data to a practitioner audience. This must be resolved before submission.

**2. The corrective scaling factors' practical utility is overstated given the uncertainty.**
The Fun correction ($\hat{FN}_{corrected} = 2.01 + 0.68 \cdot FN_{AI}$) has a 95% CI for the slope of [0.09, 1.27]. A slope of 0.09 would imply that AI Fun scores provide essentially no predictive information about human Fun scores (the correction would simply add a constant ~2.01 to all AI Fun scores regardless of their value); a slope of 1.27 would imply substantial predictive value. The paper cannot distinguish between these interpretations at N=10. As currently presented, applying the Fun correction to a specific puzzle's AI Fun score amounts to adding approximately 2.01 points to the AI score — not because the slope is meaningfully different from zero, but because the intercept dominates.

The paper should reframe the corrective factors for Fun and Elegance as bias corrections (add approximately 0.83 to AI Fun scores and 0.58 to AI Elegance scores) rather than as slope-adjusted linear models, until a larger sample can establish the slope reliably. This is both more honest and more practically useful.

**3. The pass threshold recommendation should be a decision analysis, not a point value.**
"Lower the AI-only pass threshold from 22/30 to approximately 20–21/30" presents a policy recommendation as a single number without error bars. The correct framing is a decision-theoretic analysis: what is the false negative rate (puzzles that would pass human review but fail AI review) at threshold 22/30, and how does it change at 20/30 and 21/30? Given the variability in the Fun and Elegance corrections, what is the probability that a puzzle scoring exactly 21/30 by the AI panel would score above 22/30 by a human panel? This analysis is feasible with the data in hand and would be far more actionable for practitioners than a point estimate.

---

## Minor Issues

1. The "LOO-CV R²" values in Table 5 should be presented with confidence intervals or standard errors, not as point estimates only — cross-validated R² at N=10 has very high variance.
2. The correlation CI for Fun [0.03, 0.81] spans such a wide range that the narrative claim that the AI-human Fun relationship is "weak" cannot be distinguished from "essentially zero" or "moderate" at this sample size. The presentation should acknowledge this.
3. IRB/ethics documentation absent; required for CHI submissions involving human participants.
4. The "[URL]" placeholder in the conclusion should specify the repository platform and what materials will be withheld (puzzle solutions, participant identities).

---

## Scores

| Criterion | Score (1-4) |
|---|---|
| Novelty | 3 |
| Soundness | 2 |
| Significance | 3 |
| Presentation | 2 |
| **Average** | **2.5** |

**Recommendation**: Major Revision

The absent figures drop the presentation score to 2 — this is a CHI paper that cannot be assessed for its visual communication without them. The soundness concerns about the Fun correction are real but fixable by reframing. My revision asks are: (a) add the five figures; (b) reframe Fun and Elegance corrective factors as bias corrections rather than slope-adjusted linear models; (c) replace the point-estimate pass threshold recommendation with a decision analysis; (d) document IRB approval. All are resolvable without new data.
