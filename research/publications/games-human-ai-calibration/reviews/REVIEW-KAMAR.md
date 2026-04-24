# Peer Review — CHI 2026

**Paper**: "Calibrating AI Expert Panel Scores Against Human Expert Judgment in Creative Domain Evaluation"
**Reviewer**: Ece Kamar (Microsoft Research)
**Date**: 2026-03-01

---

## Summary

This paper provides empirical calibration data for AI expert panel scores in creative evaluation — specifically, which of six quality dimensions are reliably scored by AI relative to human expert judgment. The central finding (technical dimensions reliable, hedonic dimensions systematically underestimated; richer profiles help with Elegance but not Fun) is well-structured and practically significant for anyone deploying AI review in creative workflows. I find the complementarity framing — "when to trust AI, when to correct, when to require humans" — well-suited to the CHI audience and to the broader human-AI complementarity literature.

---

## Strengths

**1. The complementarity framing is correct and well-executed.**
The paper's most valuable contribution is not the calibration data per se but the translation of that data into a selective-trust framework: use AI scores as-is for technical dimensions, apply corrections for Elegance, require human review for Fun. This is a principled and empirically grounded complementarity design — identifying where human and AI capabilities are substitutable (technical dimensions) and where they are not (hedonic dimensions). The framework is actionable, which is rare in this literature.

**2. The v2 profile result distinguishes two types of hedonic quality.**
The dissociation between Elegance and Fun in the profile version comparison is the most theoretically significant finding in the paper. Elegance improves substantially with richer profiles (r: 0.49→0.68) while Fun does not (r: 0.51→0.52). The paper correctly interprets this as evidence that Elegance has a structural component — accessible to inspection — and Fun has a hedonic component inaccessible to any profile enrichment. This is a productive negative result: it establishes a limit on what persona engineering can achieve for evaluation reliability, which is exactly the kind of system boundary characterization that practitioners need.

**3. The dimensional reliability characterization is the right abstraction for practitioners.**
Providing per-dimension trust guidance rather than a single aggregate reliability figure is the right level of abstraction. A practitioner deploying an AI review pipeline for a gate decision needs to know which dimensions drive the gate behavior and whether to trust each dimension. This paper provides exactly that.

---

## Weaknesses

**1. Static calibration misses the dynamic case: what happens when the AI model or community standards change?**
The paper presents corrective scaling factors as static artifacts ("apply Fun_corrected = 2.01 + 0.68 × Fun_AI"), but AI models change, profile libraries are updated (the v1→v2 transition is the paper's own evidence of this), and community aesthetic standards evolve. A static correction derived today may be substantially miscalibrated after a model update. The paper should address the maintenance question: how often should calibration be re-run, what are the triggers (model update, profile update, community standard shift), and how much data is needed for a reliable recalibration? The current paper treats calibration as a one-time study, but for a system intended for ongoing deployment, it is a recurring maintenance task.

**2. The pass threshold recalibration (22/30 → 20–21/30) ignores dimension weighting.**
The corrected threshold recommendation assumes that all dimensions contribute equally to the quality decision — that a Fun underestimation of 0.83 points is equivalently significant to the same deviation in Clarity. But practitioners may weight Fun or Elegance more heavily in editorial decisions (particularly for FDG-style creative venues where hedonic quality is central). The paper should either (a) provide a dimension-weighted threshold correction that accounts for the quality importance of each dimension, or (b) explicitly note that the threshold correction assumes uniform dimension weights and flag this as a limitation for domains where weights differ.

**3. What should practitioners do when AI and human reviewers disagree on a specific case?**
The calibration analysis supports mean-shift corrections and threshold adjustment, but the most consequential editorial decisions involve specific disagreements: a puzzle the AI panel passes that a human reviewer flags, or a puzzle the AI panel cuts that a human reviewer considers excellent. The paper establishes that disagreements are more likely on Fun and Elegance — good — but does not give practitioners a protocol for handling such disagreements. The complementarity framework should specify the decision rule for the disagreement case: for example, "when AI and human scores on Fun differ by more than 1 point, require a second human reviewer" or "AI Fun scores below X should trigger mandatory human review regardless of total score." Without this, the calibration data improves insight but not decision quality.

---

## Minor Issues

1. The "recalibrated pass threshold" discussion should report the expected false-negative rate at the proposed new threshold (20–21/30) compared to the current threshold (22/30). How many puzzles that would fail the original AI threshold would pass the corrected threshold? What fraction of those would a human expert have rated as genuinely passing?
2. The corrective factors should include practical guidance on when to \emph{not} apply them: e.g., if the correction has wide confidence intervals and the puzzle's AI score is in the middle range, the correction may add more noise than it removes.
3. IRB approval is not mentioned; CHI requires documentation of ethical review for human participant studies.

---

## Scores

| Criterion | Score (1-4) |
|---|---|
| Novelty | 3 |
| Soundness | 3 |
| Significance | 3 |
| Presentation | 3 |
| **Average** | **3.0** |

**Recommendation**: Revise and Resubmit

The core finding is sound and the complementarity framing is genuinely useful. My concerns are primarily about the paper's failure to address the dynamic calibration case and the incomplete practitioner guidance for the disagreement scenario. Neither requires new experiments — both are discussion-level additions. The static-vs-dynamic gap is conceptually important enough that the paper should have a clear section addressing ongoing calibration maintenance. The disagreement protocol is important enough for practitioners that its absence makes the guidance feel incomplete. I'd accept this paper with these additions made.
