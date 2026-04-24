# Peer Review — CHI 2026

**Paper**: "Calibrating AI Expert Panel Scores Against Human Expert Judgment in Creative Domain Evaluation"
**Reviewer**: Saleema Amershi (Microsoft Research)
**Date**: 2026-03-01

---

## Summary

This paper provides a controlled calibration study of AI expert panel scores against human expert judgment in puzzle hunt quality evaluation. The design is rigorous within its constraints, the findings are interpretable, and the practitioner implications are direct. The per-dimension reliability characterization — technical dimensions reliable, hedonic dimensions not; profile richness helps with structural-hedonic Elegance but not experiential Fun — is a genuine contribution to the AI-assisted review literature. My primary concern is that the paper treats calibration as a static product rather than an ongoing process, which limits the paper's applicability to real deployment scenarios where the AI model, profile library, and community standards all evolve.

---

## Strengths

**1. The study design's key methodological choices are correct.**
Using the same rubric for AI and human reviewers (rather than having human reviewers evaluate AI reviewers' verdicts) is the right choice: it ensures the comparison is against a shared quality standard, not against human-generated reviews of AI-generated reviews. Reporting ICC as the human reliability baseline before reporting AI-human correlation is correct. The qualitative analysis of justification texts as a bridge to mechanism is appropriate for a CHI multi-method paper.

**2. The v2 profile result is the paper's most original finding.**
The profile version comparison is a clean natural experiment, and the null result for Fun is theoretically sharp. It is not merely that AI underestimates Fun on average — it is that a substantial investment in richer, more elaborated AI personas that explicitly include frameworks for evaluating Fun (a-ha economy, misdirection structure) does not close the Fun calibration gap at all. This is a useful negative result that says something specific about the nature of hedonic evaluation that no amount of persona engineering can substitute for.

**3. The "when human review is mandatory" guidance is the right operational output.**
Section 5.3 (Implications for AI Panel Deployment) gives concrete guidance: use AI as-is for technical dimensions, apply corrections for hedonic dimensions, require human review when Fun and Elegance drive the decision. For a practitioner deploying an AI review pipeline, this is the paragraph they will actually use.

---

## Weaknesses

**1. The paper presents calibration as a one-time study rather than an ongoing maintenance process.**
The v1→v2 profile upgrade that is the paper's evidence for profile version effects is itself evidence that calibration is not a one-time activity. The v1 calibration is now outdated; a new calibration was needed after the upgrade. The paper should address the calibration lifecycle: what events trigger recalibration (model update, profile library change, community standard shift), what sample size is sufficient for a reliable recalibration, and how practitioners should manage the validity window of a calibration study. Without this, the paper's practical guidance has a shelf life that is not specified.

**2. The study does not address per-reviewer AI-human agreement, only panel-mean AI-human agreement.**
The analysis computes the correlation between AI panel mean and human panel mean across puzzles. This is the right aggregate measure, but it obscures an important structural question: does the AI panel agree with the human panel on the same puzzles that humans agree with each other on? Or does AI agreement track a different dimension of puzzle quality than human agreement? Analyzing per-reviewer AI-human agreement (how often does the AI panel rank the same puzzle highest that Reviewer A ranks highest?) would reveal whether AI reliability is orthogonal to or aligned with human reliability patterns.

**3. The corrective factors are treated as static inputs rather than as outputs of a learning system.**
A natural extension of this work — and one that would be practically significant — is a system where human reviewer feedback on AI-scored puzzles updates the corrective factors dynamically, improving calibration over time. The paper mentions in the limitations that "the calibration result is specific to the v2 profile library" and recommends treating calibration as a "recurring maintenance task," but does not discuss what a principled online calibration protocol would look like. For a CHI paper in the interactive ML tradition, this gap is notable: the paper describes the problem but does not take the obvious next step of describing the solution architecture.

---

## Minor Issues

1. The analysis scripts referenced in the plan (calibration.py, icc.py, scaling.py) are not mentioned in the paper's results — either they have been run and should be cited, or they haven't been run and should not be referenced.
2. IRB documentation is absent; CHI requires ethical review documentation for human participant studies.
3. Section 4.4 describes leave-one-out cross-validation as "10 folds, one puzzle held out per fold" — this is standard LOOCV and correctly described, but the results (LOO-CV R²=0.26 for Fun) should be presented with appropriate uncertainty bounds, not just as point estimates.
4. The conclusion commits to releasing "analysis scripts" at a URL that is not specified. If the scripts are not yet available, this should be framed as "will be made available upon acceptance."

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

The paper is solid and addresses a genuinely important question. My revision requests are primarily additions rather than corrections: (a) a discussion of the calibration lifecycle and maintenance protocol; (b) a brief analysis of per-reviewer AI-human agreement patterns; (c) IRB documentation. None requires new experiments. The sample size limitations are acknowledged appropriately in the paper; the corrective factor framing (mean-shift estimates rather than per-puzzle predictors) needs modest sharpening, consistent with what Shneiderman and Bernstein flag. I would accept this paper with these additions.
