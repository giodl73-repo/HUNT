# Recheck Review — Round 1 — CHI 2026

**Paper**: "Calibrating AI Expert Panel Scores Against Human Expert Judgment in Creative Domain Evaluation"
**Reviewer**: Jeffrey Heer (University of Washington)
**Round**: 1 (Recheck after Major Revision)
**Date**: 2026-03-01

---

## Summary of Revisions

The three presentation and soundness concerns I raised are substantially addressed. Figures are present; the corrective factor framing is appropriate; the threshold guidance is reframed as a borderline zone. My main remaining note is about Figure 5 data provenance (a camera-ready transparency issue) and the threshold analysis, which is improved but still not a full decision-theoretic treatment.

---

## Assessment of P1 Items

### P1.1 — Figures: **Fully addressed**

All five figures are present. Evaluating each:

- **Figure 1 (study design)**: Clear TikZ flowchart. Appropriate for a CHI methods paper.
- **Figure 2 (correlation bars)**: Horizontal bar chart with asymmetric 95% CIs — the correct format for this data. The wide Fun CI is visually prominent, making the N=10 instability obvious to a reader who might otherwise focus only on the point estimate.
- **Figure 3 (forest plot)**: Correctly formatted. The horizontal reference line at zero is present. The Fun and Elegance confidence intervals clearly don't cross zero; technical dimensions' CIs straddle zero. The story is immediately readable.
- **Figure 4 (v1/v2)**: Grouped bar chart effectively communicates the null result on Fun. The two Fun bars are nearly identical while all other pairs show v2 improvement. Strong visual.
- **Figure 5 (scatter plots)**: The design choice to overlay both the OLS regression line (in red) and the bias-correction constant-shift line (orange dashed) is excellent. It makes the per-puzzle vs. population-level distinction visual. The Fun panel's wide scatter is immediately apparent.

The figures are publication-quality and well-integrated with the text. Presentation score returns to 3.

### P1.2 — Corrective factor reframing: **Fully addressed**

The language change is consistent throughout. The key sentence — "Using the slope-adjusted formula on a specific puzzle would add noise, not precision" — is the right framing, and it appears in the results section where it matters most. The Table 5 header change ("Bias Estimates and Regression Parameters" replacing "Corrective Scaling Factors") is the correct relabeling. Resolved.

---

## Assessment of P2 Items

### P2.2 — Threshold analysis: **Substantially addressed, minor residue**

The borderline zone framing (20–22/30 = mandatory human review) is a genuine improvement over the original point estimate. The two operational options (lower threshold to 20 vs. flag borderline zone for human review) give practitioners a real choice framework. This is better.

I would still have preferred a table giving expected false-negative rates at thresholds 19, 20, 21 — this is the quantitative treatment the decision deserves. The current framing is qualitatively correct but not maximally informative. This is a nice-to-have, not a revision requirement.

---

## Remaining Notes

1. **Figure 5 data provenance**: The per-puzzle per-dimension scatter points in Figure 5 require clarification. The paper's tables report panel means; individual per-dimension scores per puzzle come from the reviewer scoring records. The caption should note "per-puzzle AI and human panel means for the Fun [Elegance] dimension, derived from individual reviewer scoring records." This is a one-sentence camera-ready addition.

2. **LOO-CV R² should have uncertainty bounds**: The Table 5 LOO-CV R² values (Fun: 0.26, Elegance: 0.44) are point estimates from a leave-one-out procedure on N=10. With N=10, these estimates are themselves highly variable. A camera-ready addition of "(± est.)" or a footnote noting high variance would be honest; this is minor.

---

## Scores

| Criterion | Round 1 | Round 2 (this review) | Change |
|---|---|---|---|
| Novelty | 3 | 3 | = |
| Soundness | 2 | 3 | ↑ |
| Significance | 3 | 3 | = |
| Presentation | 2 | 3 | ↑ |
| **Average** | **2.5** | **3.0** | **↑** |

**Recommendation**: Weak Accept

Both the soundness and presentation concerns that drove my round 1 score are resolved. The figures are well-designed and the corrective factor reframing is correct. The Figure 5 provenance note and LOO-CV uncertainty are camera-ready fixes. I recommend acceptance.
