# Peer Review — CHI 2026

**Paper**: "Calibrating AI Expert Panel Scores Against Human Expert Judgment in Creative Domain Evaluation"
**Reviewer**: Ben Shneiderman (University of Maryland)
**Date**: 2026-03-01

---

## Summary

This paper addresses an important and underexplored problem: when AI-simulated expert panels issue quality scores for creative artifacts, how do those scores relate to what human experts would say? The study design — 10 puzzles scored by an AI panel and five human experts on an identical rubric — is methodologically clear, and the per-dimension analysis yields an interpretable finding: AI panels are calibrated on technically verifiable dimensions (Clarity, Solvability, Confirmation) and systematically underestimate hedonic dimensions (Fun, Elegance). The principle — that AI evaluation reliability tracks criterion verifiability — is a useful, actionable contribution for practitioners deploying AI review in creative workflows.

---

## Strengths

**1. Practitioner-relevant framing.**
The paper is organized around a practical question: where should a practitioner trust AI scores, where should they apply corrections, and where should they require human review? This is the right question for a CHI audience concerned with human-AI collaboration in professional contexts. The practitioner guidance section in the discussion is concrete and actionable: use AI scores as-is for Clarity/Solvability/Confirmation; apply corrections for Fun and Elegance; lower the pass threshold for AI-only pipelines. This operational framing distinguishes the paper from purely descriptive calibration studies.

**2. The dimension-reliability dissociation is genuine and theoretically grounded.**
The finding that technical dimensions are AI-reliable while hedonic dimensions are not is more than a correlational observation — the discussion grounds it in a principled account of what AI reviewers can and cannot access (structural inspection vs. embodied experience of solving). The profile version result (v2 improves Elegance but not Fun) sharpens this claim considerably: the dissociation within hedonic dimensions between partially-structural Elegance and purely-experiential Fun is a theoretically informative detail that the paper handles well.

**3. The human ICC baseline is the correct methodological choice.**
Reporting human inter-rater reliability as the interpretation ceiling for AI-human correlation is the right design decision, and the paper executes it correctly. Many AI-human comparison studies hold AI to a higher standard than humans; this paper appropriately asks whether AI performs within the range of human-level reliability on each dimension.

---

## Weaknesses

**1. The corrective scaling factors are called "practitioner-ready" but the evidence does not support this characterization.**
The Fun correction has LOO-CV R² = 0.26. This means that 74% of the variance in human Fun scores is unexplained by AI Fun scores even after applying the linear correction. Applying the Fun correction to a specific puzzle's AI Fun score will predict that puzzle's human Fun score to approximately the same accuracy as just predicting the human panel mean for all puzzles. This is a population-level mean correction, not a per-puzzle predictor. The paper should clearly distinguish between (a) the mean-shift finding — AI Fun scores are on average 0.83 points lower than human Fun scores — which is actionable for threshold recalibration, and (b) the per-puzzle correction — using AI Fun scores to predict human Fun scores on individual puzzles — which is essentially unreliable at N=10. The "practitioner-ready corrective scaling factors" framing overstates what the data supports.

**2. The generalizability claim rests on a single domain and a single expert community.**
The conclusion introduces "a generalizable principle" based on one calibration study in one creative domain using experts from one community. The gesture toward academic paper review and interactive fiction in the Discussion is illustrative but not evidential. This is a hypothesis about generalizability, not a demonstration. The paper should either limit the claim to the puzzle hunt domain ("our results suggest a principle that may generalize...") or invest in the multi-domain framing with comparative evidence. The current construction overstates the evidential basis for a general principle.

**3. The paper does not address what practitioners should do when AI and human reviewers disagree on a specific puzzle.**
The calibration analysis gives mean corrections — useful for threshold adjustment — but does not address the disagreement problem. In practice, an AI panel passing a puzzle that a human reviewer later flags, or vice versa, is the decision point that matters most. The paper establishes that disagreements are more likely on Fun and Elegance than on technical dimensions, which is useful. But it does not give guidance on how to handle specific disagreements: who has the higher burden of proof, what triggers mandatory human review, how disagreements should be adjudicated. For a CHI paper about human-AI collaboration, this gap in the practical implications is notable.

---

## Minor Issues

1. The "first calibration study in a creative domain" claim should be hedged more carefully — the lit review covers analogous work in academic peer review and creative writing but does not fully survey the adjacent creative evaluation literature (game level evaluation, music evaluation, design critique). The claim may be accurate but needs to be defended more specifically.
2. IRB or equivalent ethical review is not mentioned despite human participant involvement. CHI requires ethical clearance documentation.
3. The conclusion commits to a "[URL]" artifact release but does not name the repository or specify which materials will be withheld.

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

The core finding is genuine and the framing is appropriate for CHI. The soundness concern is primarily about the "practitioner-ready" overclaim for the Fun corrective factor and the over-broad generalizability claim. Both are fixable without new experiments: the first requires reframing the corrective factors as mean-shift estimates rather than per-puzzle predictors; the second requires downgrading the "generalizable principle" to a "generalizable hypothesis" with appropriate qualification. The study design is sound within its N constraints; the paper needs to be more honest about what N=10 can and cannot establish.
