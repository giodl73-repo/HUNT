# SYNTHESIS — games-human-ai-calibration
**Round:** 1 | **Reviewers:** Shneiderman, Bernstein, Kamar, Amershi, Heer
**Average score:** 2.80/4 | **Gate:** PASS | **Consensus:** Major Revision

---

## Score Summary

| Reviewer | Novelty | Soundness | Significance | Presentation | Avg | Rec |
|---|---|---|---|---|---|---|
| Shneiderman (UMD / HCAI) | 3 | 2 | 3 | 3 | 2.75 | Major Revision |
| Bernstein (Stanford / crowdsourcing) | 3 | 2 | 3 | 3 | 2.75 | Major Revision |
| Kamar (MSR / complementarity) | 3 | 3 | 3 | 3 | 3.0 | Revise & Resubmit |
| Amershi (MSR / interactive ML) | 3 | 3 | 3 | 3 | 3.0 | Revise & Resubmit |
| Heer (UW / visualization) | 3 | 2 | 3 | 2 | 2.5 | Major Revision |
| **Average** | **3.0** | **2.4** | **3.0** | **2.8** | **2.80** | |

**Gate check:** avg 2.80/4 ≥ 2.5 ✓ | no score < 2/4 ✓ → **PASS**

---

## Consensus Strengths

- **The dimension-reliability dissociation is genuine and credible** (all 5): All five reviewers accept the core finding — technical dimensions are AI-reliable, hedonic dimensions are not. The finding is considered credible because (a) it is internally consistent across the correlation, bias, and profile version analyses; (b) it has a principled mechanistic account (structural inspection vs. embodied experience); (c) the negative result on Fun profile improvement is a sharp, theoretically informative point.

- **The v2 profile result on Elegance/Fun is the paper's most original contribution** (Amershi, Kamar, Bernstein): The dissociation within hedonic dimensions — richer profiles close the Elegance gap but not the Fun gap — is called out across three reviews as the most novel and theoretically productive finding. Kamar: "A substantial investment in richer personas that explicitly include frameworks for Fun does not help at all — this is exactly the kind of system boundary characterization practitioners need." This result establishes a principled limit on persona engineering for evaluation reliability.

- **ICC-as-baseline is methodologically exemplary** (Bernstein, Shneiderman, Amershi): Reporting human inter-rater reliability as the interpretation ceiling for AI-human correlation is recognized across three reviews as the correct epistemological design. Papers in this space routinely fail to do this; this paper does it right.

- **The complementarity/selective-trust framing is well-suited to CHI** (Kamar, Amershi, Shneiderman): The translation of calibration data into per-dimension trust guidance (use AI as-is / apply correction / require human review) is recognized as the right practical output for a CHI audience. The framework is actionable, which is rare.

- **Qualitative analysis bridges quantitative finding to mechanism** (Heer, Amershi): The human vs. AI justification contrast ("solving with my team, we'd all be groaning" vs. "clue density, misdirection surface area") is considered memorable and appropriately multi-method for CHI.

---

## Consensus Concerns

- **N=10 puzzles is too small for reliable corrective scaling factors** (Bernstein, Shneiderman, Heer): The most serious methodological concern, flagged by three reviewers. Bernstein: "With N=10, the Fun correlation CI [0.03, 0.81] spans from essentially null to strong — we cannot distinguish 'weak relationship' from 'no relationship' at this sample size." Heer: "The Fun slope CI [0.09, 1.27] means the correction is consistent with adding a constant ~2.01 to all AI Fun scores regardless of value — i.e., the slope is statistically indistinguishable from zero." The corrective scaling factors for Fun and Elegance should be reframed as **bias corrections** (add 0.83 and 0.58 respectively) rather than slope-adjusted linear models, until a larger sample can establish the slopes.

- **Figures are absent** (Heer, implicitly noted by others): Heer drops the presentation score to 2 because the five key figures described in the paper are not present. For CHI, where visual communication is primary, this is a blocking omission. All five figures must be produced.

- **"Practitioner-ready corrective scaling factors" overclaims precision** (Shneiderman, Bernstein, Heer): The Fun LOO-CV R²=0.26 means 74% of variance in human Fun scores is unexplained. The current presentation implies per-puzzle precision that the data cannot support. The paper should distinguish (a) mean-shift corrections — AI Fun scores are on average 0.83 points lower, so lower the threshold — from (b) per-puzzle slope corrections — given AI scores X, predict human score Y — and be explicit that only (a) is warranted at this sample size.

- **Calibration lifecycle not addressed** (Kamar, Amershi): Two reviewers flag that static calibration is insufficient for a deployed system. The paper should address: what triggers recalibration (model update, profile change, community drift), how much data is needed for a reliable recalibration, and how practitioners should manage the validity window. Amershi notes this is particularly notable given that the paper's own v1→v2 evidence demonstrates that calibration goes stale after profile updates.

- **Pass threshold recalibration should be a decision analysis, not a point estimate** (Heer, Kamar): "22/30 → 20–21/30" is presented as a policy recommendation without uncertainty bounds. The correct formulation is a decision-theoretic analysis: what is the false-negative rate at each threshold, and how does it depend on the uncertainty in the Fun and Elegance corrections?

- **Generalizability claim overstates the evidence** (Shneiderman): The "generalizable principle" introduced in the conclusion and Discussion rests on a single domain study. The principle should be reframed as a hypothesis with appropriate qualification: "our results suggest a principle that may generalize..."

- **IRB/ethics documentation absent** (Shneiderman, Bernstein, Kamar, Amershi): All four reviewers who noted it flag this as a CHI requirement. Ethics documentation must be added.

- **Range-selection inflates reported correlations** (Bernstein): Puzzle selection to "span the AI panel score range" is a non-representative sampling strategy that inflates Pearson r estimates. This should be acknowledged as a limitation and the likely direction of the inflation bias should be estimated.

---

## P1 — Blocking

### P1.1 Figures must be produced

Heer's presentation score of 2 reflects the absence of all five key figures. For a CHI submission, figures are not optional: the forest plot (bias), per-dimension correlation chart, v1/v2 comparison bar chart, and Fun/Elegance scatter plots with OLS overlays are the primary visual communication of the paper's results. All five must be present before submission.

**Target:** figures/fig1-study-design.pdf, figures/fig2-correlation-bars.pdf, figures/fig3-bias-forest.pdf, figures/fig4-v1v2-comparison.pdf, figures/fig5-scaling-scatter.pdf

### P1.2 Reframe corrective factors as bias corrections, not slope-adjusted predictors

The Fun slope CI [0.09, 1.27] and LOO-CV R²=0.26 render the per-puzzle slope correction unreliable at N=10. The practical correction that the data supports is a mean-shift: AI Fun scores are biased low by 0.83 points on average, so (a) add ~0.83 to AI Fun scores for threshold comparison, or (b) lower the pass threshold by the expected bias. The slope-adjusted formula ($\hat{FN}_{corrected} = 2.01 + 0.68 \cdot FN_{AI}$) suggests per-puzzle precision that the data does not support. Reframe consistently throughout.

**Target:** sections/04-results.tex (§4.5), sections/05-discussion.tex (§5.3), sections/06-conclusion.tex

---

## P2 — Important

### P2.1 Calibration lifecycle: add discussion of maintenance protocol

The paper should add a substantive discussion of what triggers recalibration, how much data is needed, and how practitioners should manage validity windows. This is a practical necessity for any deployed calibration system and is doubly important given the paper's own evidence of v1→v2 drift.

**Target:** sections/05-discussion.tex (new subsection or extended §5.3)

### P2.2 Pass threshold: replace point estimate with decision-theoretic analysis

Replace "approximately 20–21/30" with a probabilistic analysis: at each candidate threshold (19, 20, 21), what is the expected false-negative rate given the mean bias and its uncertainty? This is computable from the existing data and is far more actionable for practitioners.

**Target:** sections/04-results.tex (§4.5), sections/05-discussion.tex (§5.3)

### P2.3 IRB/ethics documentation

Add a paragraph in §3 (Study Design) documenting that appropriate ethical review was obtained and that informed consent was obtained from all five participants.

**Target:** sections/03-methodology.tex

### P2.4 "Generalizable principle" → "Generalizable hypothesis"

Downgrade the principle framing to a hypothesis, consistent with the evidential basis (N=1 domain). Add explicit qualification: "our results in the puzzle hunt domain suggest a principle that we hypothesize may generalize across creative evaluation contexts; multi-domain replication is needed to establish this."

**Target:** sections/05-discussion.tex (§5.4), sections/06-conclusion.tex

### P2.5 Range-selection bias: add to limitations

Acknowledge that puzzle selection to span the AI score range inflates Pearson r estimates relative to a representative sample, and note that the reported correlations likely overestimate population-level AI-human correlation.

**Target:** sections/05-discussion.tex (Limitations)

---

## P3 — Nice to Have

### P3.1 Per-reviewer AI-human agreement (Amershi)
Analyze whether AI reliability aligns with human reliability patterns at the reviewer level, not just the panel mean level.

### P3.2 Disagreement protocol (Kamar, Shneiderman)
Add guidance on what practitioners should do when AI and human reviewers disagree on a specific puzzle, not just on aggregate calibration.

### P3.3 Analysis scripts cited in results section (Amershi, Heer)
Either reference the scripts in the results section (if run) or remove the plan.md reference.

---

## Summary Assessment

This is a competent and genuinely useful calibration study. The core finding — technical dimensions AI-reliable, hedonic dimensions not; profile richness helps with structural-hedonic Elegance but not experiential Fun — is credible, interpretable, and actionable. The path to acceptance requires two categories of work: (1) **figures**: all five must be produced; (2) **precision recalibration**: the corrective factors and pass threshold must be reframed as mean-shift estimates rather than per-puzzle predictors, consistent with what N=10 can actually support. The P2 items are all discussion-level additions requiring no new data. The paper will be stronger if it addresses the calibration lifecycle gap (P2.1) and the generalizability overstatement (P2.4).
