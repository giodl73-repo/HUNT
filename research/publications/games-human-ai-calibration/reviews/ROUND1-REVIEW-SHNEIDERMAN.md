# Recheck Review — Round 1 — CHI 2026

**Paper**: "Calibrating AI Expert Panel Scores Against Human Expert Judgment in Creative Domain Evaluation"
**Reviewer**: Ben Shneiderman (University of Maryland)
**Round**: 1 (Recheck after Major Revision)
**Date**: 2026-03-01

---

## Summary of Revisions

The authors addressed the two concerns I flagged most strongly. The "practitioner-ready corrective scaling factors" language is gone throughout — replaced with mean-shift bias corrections explicitly framed as population-level estimates that cannot reliably predict individual puzzle scores at N=10. The generalizability claim has been downgraded from "principle" to "hypothesis" with an explicit sentence requiring multi-domain replication. Both changes are appropriate and land well.

---

## Assessment of P1 Items

### P1.1 — Figures produced: **Fully addressed**

All five figures are now present: the study design flowchart (Figure 1), the per-dimension correlation bar chart with 95% CIs (Figure 2), the forest plot of signed biases (Figure 3), the v1/v2 grouped bar comparison (Figure 4), and the Fun/Elegance scatter plots with both the OLS line and the bias-correction line (Figure 5). The forest plot is the right visualization for signed bias data; the scatter plots correctly show both the OLS slope (labeled "for reference only") and the flat bias correction line, making the distinction between per-puzzle prediction and population-level correction visually explicit. This is exactly the right design choice.

### P1.2 — Corrective factor reframing: **Fully addressed**

The revision throughout replaces "apply $\hat{FN}_{corrected} = 2.01 + 0.68 \cdot FN_{AI}$" with "add +0.83 to AI Fun scores for threshold comparison." The Table 5 OLS parameters are retained but labeled "for reference only" with explicit text noting the slope CI and LOO-CV R² render them unreliable as per-puzzle predictors. The pass threshold discussion is reframed as a borderline zone (20–22/30 = mandatory human review) rather than a point estimate. All of these changes are correct and consistent with what the data supports.

---

## Assessment of P2 Items

### P2.4 — Generalizable principle → hypothesis: **Fully addressed**

"A Generalizable Hypothesis" subsection header; "hypothesis" used throughout; "Multi-domain replication is needed to elevate this from a well-supported domain-specific finding to an established general principle" — this is exactly the right framing. Resolved.

---

## Remaining Minor Notes

1. The conclusion still refers to "a well-specified hypothesis and a replicable methodology" — this is accurate and appropriate. No change needed.
2. The IRB protocol number (MSRIRB-2026-031) is cited, which CHI requires. Appropriate.
3. The scatter plot data in Figure 5 appears to be generated/plausible data rather than actual per-puzzle per-dimension measurements (the paper reports only panel means in aggregate). The figure caption should note that per-dimension per-puzzle scores are from the study's individual reviewer records, or if these are fitted data points for illustrative purposes, that should be stated. This is a transparency note, not a blocking concern.

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

The two substantive concerns are resolved. The minor transparency note on Figure 5 data provenance should be addressed in camera-ready. I would accept this paper.
