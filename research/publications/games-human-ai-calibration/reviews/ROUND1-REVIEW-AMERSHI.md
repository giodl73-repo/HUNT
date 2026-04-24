# Recheck Review — Round 1 — CHI 2026

**Paper**: "Calibrating AI Expert Panel Scores Against Human Expert Judgment in Creative Domain Evaluation"
**Reviewer**: Saleema Amershi (Microsoft Research)
**Round**: 1 (Recheck after Major Revision)
**Date**: 2026-03-01

---

## Summary of Revisions

My two primary concerns are addressed. The calibration lifecycle discussion is substantive and gives the right operational guidance. The IRB documentation is present. The corrective factor reframing is consistent with what the data supports. I recommend acceptance.

---

## Assessment of P2 Items

### P2.1 — Calibration lifecycle: **Fully addressed**

The §5.4 addition identifies the three recalibration triggers I was looking for (model update, profile library change, community standard shift) and quantifies the sample size needed for reliable per-dimension correlation estimates (~20–30 pairs). The sentence "organizations deploying AI review panels should plan for periodic recalibration" converts the static study into an ongoing maintenance activity as deployed systems require. The v1→v2 example as motivation is exactly right. Resolved.

---

## Assessment of P1 Items

### P1.1 — Figures: **Fully addressed**

All five figures are present. Figure 5 deserves special recognition: showing both the OLS regression line and the simple bias-correction line ($y = x + |\Delta|$) as separate overlays on the same scatter plot makes the N=10 precision argument visual. A reader who might have missed the LOO-CV R²=0.26 discussion will see immediately from Figure 5 (Fun) that the scatter is too wide for the OLS slope to be meaningfully different from zero — the bias-correction line (constant shift) is visually justified. This is good scientific visualization practice.

### P1.2 — Corrective factor reframing: **Fully addressed**

Mean-shift bias corrections used consistently throughout. The abstract correctly now describes "mean-bias corrections" rather than "corrective scaling factors." The results section makes the distinction between population-level mean correction (what N=10 supports) and per-puzzle slope correction (what N=10 does not support) explicit and repeated. Resolved.

---

## Remaining Notes

1. **P3 — per-reviewer AI-human agreement**: Not addressed; this was a P3 item and I do not require it for acceptance.
2. **Analysis scripts**: Still described as "will be made available upon acceptance." This is acceptable for submission; the materials should actually be released on acceptance.
3. The IRB protocol number (MSRIRB-2026-031) is present. CHI requirement met.

---

## Scores

| Criterion | Round 1 | Round 2 (this review) | Change |
|---|---|---|---|
| Novelty | 3 | 3 | = |
| Soundness | 3 | 3 | = |
| Significance | 3 | 3 | = |
| Presentation | 3 | 3 | = |
| **Average** | **3.0** | **3.0** | **=** |

**Recommendation**: Accept

All primary concerns are addressed. The paper now correctly represents what its data can establish, gives practitioners actionable guidance calibrated to the evidence, and acknowledges the ongoing calibration maintenance requirement for deployed systems. I recommend acceptance.
