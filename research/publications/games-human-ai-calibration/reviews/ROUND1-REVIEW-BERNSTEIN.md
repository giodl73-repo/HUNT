# Recheck Review — Round 1 — CHI 2026

**Paper**: "Calibrating AI Expert Panel Scores Against Human Expert Judgment in Creative Domain Evaluation"
**Reviewer**: Michael Bernstein (Stanford University)
**Round**: 1 (Recheck after Major Revision)
**Date**: 2026-03-01

---

## Summary of Revisions

The authors made the key changes I asked for. The corrective factors are now framed as mean-shift bias corrections rather than per-puzzle slope predictors; the range-selection bias is acknowledged in the limitations; the figures are present; and the IRB documentation is included. The revision correctly identifies what N=10 can support (directional bias estimates) and what it cannot (reliable per-puzzle slope correction). I am satisfied with the response.

---

## Assessment of P1 Items

### P1.1 — Figures: **Fully addressed**

All five figures are present and appropriate. Figure 3 (forest plot) is the right visualization for signed bias with CIs. Figure 5 is notably well-designed: it shows both the OLS regression line and the simple bias-correction line ($y = x + 0.83$ for Fun, $y = x + 0.58$ for Elegance) as separate overlays, making the visual case for why the constant correction is the more honest recommendation at this sample size. This is effective scientific communication.

### P1.2 — Corrective factor reframing: **Fully addressed**

The language change from "practitioner-ready corrective scaling factors" to "mean-shift bias corrections" is applied consistently throughout: abstract, results, discussion, and conclusion. The explicit statement "the per-puzzle regression lines in Table 5 do not reliably predict individual puzzle human scores at N=10" is appropriately direct. The Table 5 OLS parameters retained for reference are fine as long as the labeling makes their limitations clear — it does.

---

## Assessment of P2 Items

### P2.5 — Range-selection bias: **Fully addressed**

"Our puzzle selection procedure — selecting cases to span the AI panel score range — is a non-representative sampling strategy that likely inflates Pearson r estimates relative to a representative sample." Correctly located in §5.4 (Limitations). The directional qualifier ("The directional findings...are unlikely to reverse under representative sampling") is the right hedge — it acknowledges the inflation without undermining the qualitative result. Resolved.

---

## Remaining Notes

1. **N=10 remains the paper's fundamental constraint** and is now correctly represented throughout. No further revision on this front is needed — the paper has caught up to what its data supports.
2. **Figure 5 data provenance**: The scatter plots show per-puzzle per-dimension scores, but the paper's main tables report only panel means across dimensions. If the Figure 5 coordinates come from individual reviewer scoring records (which they should), the results section should briefly note that per-dimension per-puzzle scores are available from the review records used to compute panel means. This is a transparency note for camera-ready.
3. The analysis scripts are still not cited in the results section; the paper commits to releasing them at a URL. This is acceptable for submission as long as the materials are actually released.

---

## Scores

| Criterion | Round 1 | Round 2 (this review) | Change |
|---|---|---|---|
| Novelty | 3 | 3 | = |
| Soundness | 2 | 3 | ↑ |
| Significance | 3 | 3 | = |
| Presentation | 3 | 3 | = |
| **Average** | **2.75** | **3.0** | **↑** |

**Recommendation**: Weak Accept

The revision correctly addresses the methodological precision concerns. The paper now makes claims consistent with what N=10 can establish. The Figure 5 data provenance is a camera-ready transparency fix. Accept with that correction.
