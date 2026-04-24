# Recheck Review — Round 1 — CHI 2026

**Paper**: "Calibrating AI Expert Panel Scores Against Human Expert Judgment in Creative Domain Evaluation"
**Reviewer**: Ece Kamar (Microsoft Research)
**Round**: 1 (Recheck after Major Revision)
**Date**: 2026-03-01

---

## Summary of Revisions

Both of my P2 concerns are addressed. The calibration lifecycle discussion is present and gives practitioners concrete maintenance guidance. The pass threshold recommendation is now a borderline zone with two operational options rather than a single recalibrated number. The figures are present. I recommend acceptance.

---

## Assessment of P2 Items

### P2.1 — Calibration lifecycle: **Fully addressed**

The revised §5.4 adds: recalibration triggers (model update, profile library change, community standard shift), a specific sample size requirement (~20–30 puzzle-reviewer pairs for stable per-dimension estimates), and the v1→v2 transition itself as the motivating evidence that calibration goes stale. This is the right structure. The quantification ("20–30 pairs") converts an abstract recommendation into an actionable operational target. The acknowledgment that N=10 is "a minimum for directional estimates, not a sufficient sample for production use" correctly manages expectations. Resolved.

### P2.2 — Pass threshold: **Substantially addressed**

The revision replaces the point estimate with a borderline zone: "treat puzzles scoring 20–22/30 by AI as requiring mandatory human review" (option b) or "lower the threshold to 20/30" (option a). This is more operationally honest than "20–21/30" and acknowledges the uncertainty in the correction. I note that option (b) — mandatory human review for the borderline zone — is the stronger practical recommendation for contexts where human review capacity permits; option (a) is appropriate for high-throughput pipelines. The paper correctly presents both.

I would have preferred a table showing expected false-negative rates at each threshold, but the borderline zone framing is a legitimate and actionable alternative. My remaining concern here is minor and does not warrant further revision.

---

## Remaining Notes

1. The disagreement protocol (P3.2 from my original review) is not addressed — it was a P3 item and I do not require it. The paper's guidance for the "borderline zone" partially addresses this concern.
2. The figures are well-designed. Figure 4 (v1/v2 grouped bars) makes the Fun null result visually stark — the two bars for Fun are nearly identical height while all other dimensions show meaningful v2 improvement.

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

The paper's core contributions are sound, the practical guidance is now appropriately calibrated to the evidence, and the lifecycle discussion addresses the deployment reality that calibration is an ongoing maintenance activity. I recommend acceptance.
