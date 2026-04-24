# Peer Review — ICCC 2026 Submission
## "Expert Panels Without Experts: Profile-Based AI Review for Creative Work Evaluation"

**Reviewer:** Percy Liang (Stanford University)
**Date:** 2026-02-28
**Venue:** ICCC 2026

---

## Summary

This paper proposes a system for assembling AI-simulated expert panels using persistent, domain-specific reviewer profiles to evaluate puzzle hunt puzzles. The authors conduct an eight-condition ablation comparing profile depth (bare prompt through full 80-120 line domain-specific profiles) across three puzzles and extend this to a 15-puzzle corpus to demonstrate tier-separation gains from their validated seven-dimension weighted rubric. The central empirical claim is that profile depth produces a feedback quality gradient that is practically more significant than the accompanying score gradient, with the full-profile condition (C6) being the only condition to produce passing scores on all test puzzles and tier-consistent verdicts across the extended corpus.

---

## Strengths

- **The eight-condition factorial design is genuinely informative.** Decomposing profile components (C4: philosophy only vs. C5: lens only vs. C6: full) is methodologically sound and reveals a non-obvious result: the score gradient is non-monotonic, which the authors correctly identify and explain rather than suppress. A paper that reports C2 scoring lower than C1 despite having ten named frameworks is being honest with its data.

- **The calibration inversion finding (Section 5.2) is the paper's strongest empirical contribution.** The demonstration that rubric-only evaluation (C1) inverts the expected tier hierarchy — MIT/Elite puzzles failing at 40% while Standard Hunt puzzles pass at 80% — is a concrete, traceable, practically important result. The mechanistic explanation (elite puzzles sacrifice Clarity for Elegance by design; equal-weight rubrics penalize this) is persuasive and has real implications for AI evaluation system design beyond this specific domain.

- **The defect detection findings (Section 4.3) are the most concrete ground-truth-anchored results in the paper.** The duplicate label error in Puzzle II's bracket diagram and the blank-count error in Puzzle I are objectively verifiable. The observation that C5 detects the first and C7 detects the second — while C6 catches neither — is a falsifiable, domain-independent finding about what different reviewer context configurations are sensitive to. This is exactly the kind of evidence an evaluation methodology paper needs.

- **The authors are unusually transparent about their limitations.** Section 5.4 explicitly states that pass thresholds were "calibrated against the authors' prior experience," that tier assignments are author judgments rather than oracle verdicts, that the corpus is a purposive rather than random sample, and that no human calibration study was conducted. This level of candor is commendable and rare. The limitations section acknowledges the self-referential circularity problem directly: "The verdicts reported here are AI-generated assessments... no external oracle determines which assessments are correct."

- **The cross-condition framework emergence analysis (Section 4.2) makes a legitimate qualitative contribution.** Distinguishing between frameworks that are retrieved (C2 injects principle names and the model applies them) versus emergent (C6 generates vocabulary not present in any input) is a meaningful distinction that bears on the theoretical question of what profile depth is actually doing to the model's behavior.

---

## Weaknesses and Questions

### 1. The scoring system is entirely self-referential (Critical)

The paper's central metric is a score out of 30 (or 45) assigned by AI reviewers instantiated via AI profiles. The pass threshold of 22/30 is "calibrated against the authors' prior experience." The tier assignments used to validate the rubric in Experiment 4 are the authors' judgments about which hunts belong in which tier, not independent expert assessments. The rubric dimensions were authored by the same team that built the profiles.

The result is a closed loop: profiles developed by the authors activate frameworks the authors find compelling, applied against a rubric the authors designed, to produce verdicts calibrated to the authors' intuitions about quality, validated against tier assignments made by the authors. The 24.5pp tier-separation result in Section 5.3 is evidence that the rubric discriminates between tiers as the authors have assigned them — not that the rubric correctly captures what the puzzle hunt community means by quality.

The authors acknowledge this in Section 5.4 but treat it as a limitation rather than a threat to validity. I would argue it is the central unresolved question of the paper. Until a human calibration study is conducted on a shared puzzle set with known community verdicts, the scoring results cannot be interpreted as measuring what the paper claims they measure.

**Required for a strong acceptance:** At minimum, the paper needs to report inter-rater reliability between the AI panel and a sample of human expert judgments on a subset of the 15-puzzle corpus, even informally. The absence of this data is not a minor gap — it determines whether the core claims are about evaluation quality or about self-consistency.

### 2. The "feedback quality" claim is not operationally grounded (High severity)

The paper's primary claim is that C6 produces "better" feedback than C0-C5 by surfacing domain-emergent frameworks with names like "a-ha economy" and "load-bearing test." The evidence offered is: (a) these framework names do not appear in the input text, so they are not retrieved; (b) practitioners reading the reviews would find them more actionable.

Neither (a) nor (b) is a validated measure of feedback quality. The claim that a framework is "emergent" rather than "retrieved" is established negatively (it does not appear in the input) but not positively (there is no evidence the framework corresponds to a genuine evaluative concept used by the expert community). The claim that the feedback is more "actionable" is the authors' judgment, not a user study or practitioner survey. The word "better" requires a definition that is not provided.

To be precise: the paper shows that C6 produces reviews containing phrases like "load-bearing test" and "a-ha economy." It does not show that reviews containing these phrases cause constructors to produce better puzzles, or that the diagnoses are accurate, or that the named frameworks correspond to the diagnostic categories used by the practitioners whose names appear on the profiles. The gap between what is shown and what is claimed is large.

### 3. Statistical power is insufficient for the conclusions drawn (High severity)

The primary ablation (Experiment 0) evaluates three puzzles across eight conditions. Experiment 4's rubric validation uses 15 puzzles. The paper draws conclusions about "tier separation," "calibration inversion," pass rates by tier, and the comparative diagnostic value of eight conditions from these sample sizes.

Three puzzles from a single hunt (Age of Empires) cannot support claims about the general behavior of condition-condition differences. The C3 vs. C4 convergence (both 26.7/30), the C6 pass-on-all-three result, the C7 harsh-on-Puzzle-I result: these are patterns in a three-data-point sample. The 15-puzzle corpus improves this, but the tier cells contain five puzzles each, drawn from a purposive sample the authors selected. At N=5 per tier, a single outlier puzzle accounts for 20% of the pass rate.

The paper does not report any measures of uncertainty (confidence intervals, bootstrapped ranges, or even simple variance within tiers). The pass-rate comparisons (80% vs. 40% for Standard vs. MIT/Elite under C1) are presented as findings rather than as estimates with associated uncertainty from a small, non-random sample.

### 4. Tier assignment validity is assumed rather than established (High severity)

The rubric validation in Experiment 4 treats tier membership (MIT/Elite vs. Standard Hunt vs. Games Magazine) as ground truth. The 24.5pp tier-separation result is interpreted as evidence that the C11 rubric configuration correctly captures quality — because it discriminates more strongly between tiers.

This reasoning is circular if tier assignment is not independently validated. The paper states that assignments were "made by the authors based on puzzle provenance and community consensus" (Section 5.4). Provenance (where a puzzle appeared) is a reasonable proxy for community judgment but is not identical to it: puzzles appear in MIT Mystery Hunt that experienced constructors consider poorly designed, and Games Magazine puzzles exist that practitioners consider elegant. Using provenance as a quality proxy, then validating a rubric against the provenance-derived tier, then claiming the rubric captures quality, moves through a chain of inference at each step of which the argument could fail.

What would a valid tier-separation validation look like? It would require independent expert quality ratings of the 15 puzzles from practitioners who did not know the rubric or the paper's hypotheses, followed by a comparison of those ratings to tier assignments. The authors acknowledge this is future work; I would argue it is the prerequisite for interpreting Experiment 4's results as supporting the paper's claims.

### 5. The "named frameworks as quality signal" claim conflates mechanism with quality (Moderate severity)

Section 4.2 and the discussion treat the count of named evaluative frameworks as a measure of review quality, with C6 producing six domain-emergent frameworks and C0-C1 producing zero. But the paper also reports that C7 produces 14 frameworks and produces the harshest (potentially least calibrated) verdicts in the ablation, and that C2 produces 10 frameworks but scores lower than C1 despite the framework count.

The non-monotonic relationship between framework count and calibration quality (C2 < C1 despite higher framework count; C7 harshest despite highest framework count) is correctly noted by the authors. But the paper does not revise the claim that framework emergence is the primary signal. If the metric predicts quality for C6 but not for C2 or C7, it is not a reliable quality signal — it is a correlate in a particular range of conditions. The claim should be scoped accordingly.

### 6. Profile construction raises a potential confound the paper does not address (Moderate severity)

The v2 profiles are "constructed from publicly available writings, interviews, and published work by the named practitioners." The underlying model (Claude Sonnet 4.6) was trained on substantial amounts of text from and about these practitioners, including their publicly documented evaluative frameworks. When C6 produces a review that names "load-bearing test" and attributes it to Thomas Snyder, this may reflect the model's training-data knowledge of Snyder's published framework rather than reasoning derived from the profile content.

The paper claims to distinguish emergent frameworks from retrieved ones by checking whether the framework name appears in the profile input. But the relevant comparison is not "does it appear in the profile" but "does it appear in the model's training data." If "load-bearing test" is Snyder's documented vocabulary in sources the model was trained on, then C6's use of that label is retrieval from training data, not framework emergence from profile content — and the profile's causal role in producing the output is unclear.

An ablation that tested C6 against a condition with identical profile context but a model with no prior knowledge of the named practitioners would begin to disentangle these confounds. This is not a feasible experiment within a single paper, but the confound deserves explicit acknowledgment.

---

## Scores

| Dimension | Score (1-4) | Rationale |
|---|---|---|
| Novelty | 3 | The profile-as-evaluative-instrument framing and the calibration inversion finding are genuinely new. Persona-based prompting is not new; the contribution is the systematic ablation over profile components and the non-obvious finding that profile depth produces qualitative rather than quantitative gains. |
| Soundness | 2 | The experimental design is more careful than typical in this space, and the authors are transparent about limitations. However, the core self-referentiality problem (AI evaluating AI against thresholds the authors set, validated against tier assignments the authors made) is not resolved by the paper and is not merely a limitation — it determines what the quantitative results actually mean. The defect-detection results (verifiable errors found by specific conditions) are sound; the broader scoring claims are not. |
| Significance | 3 | The calibration inversion finding is significant and generalizable. The profile component decomposition (lens vs. philosophy, redundant vs. non-redundant principles) provides practical guidance with a reasonably solid empirical basis. The field needs exactly this kind of systematic ablation over evaluation system design choices. |
| Presentation | 3 | The paper is clearly written, the experimental design is well-explained, and the limitations section is more candid than average. The non-monotonic score table is reported and explained rather than suppressed. Some claims in the abstract and conclusion are stronger than the underlying evidence supports (see weaknesses 1 and 2). |

**Overall weighted assessment:** The paper makes a genuine methodological contribution and reports the data honestly. Its central weakness is not dishonesty but insufficiency: the claims about evaluation quality require grounding outside the system that produced them, and the current paper does not provide that grounding.

---

## Recommendation

**Major Revision**

The calibration inversion finding and the defect-detection analysis are worth publishing. The profile component ablation is a useful empirical contribution. However, the paper's primary claim — that C6 produces "better" evaluation than alternative conditions — cannot be evaluated without at least a partial human calibration study. The current submission documents an internally consistent and carefully designed system but does not establish that the system's outputs correspond to actual expert judgment in any verifiable sense.

For revision, I ask the authors to:

1. Conduct or report even a minimal human calibration: have one or two genuine puzzle hunt constructors (not the paper's authors) evaluate a subset of the 15 corpus puzzles on the same rubric, and report agreement with the AI panel's verdicts. Even N=5 puzzles with two human raters would provide an external anchor that the current paper completely lacks.

2. Qualify the "feedback quality gradient" claim specifically: the paper shows that C6 reviews contain domain-specific named frameworks not present in the input. It does not show that these frameworks are diagnostically accurate or that they improve constructor outcomes. The claim should be scoped to what is demonstrated.

3. Report uncertainty on the small-sample statistics. Pass rates computed over N=5 per tier should be accompanied by at minimum a statement of what a single-puzzle difference would do to the numbers. The paper currently presents 80% vs. 40% as if these were stable estimates rather than 4/5 vs. 2/5 counts.

4. Discuss the training-data confound for named framework attribution. The observation that C6 produces "load-bearing test" (Snyder's documented vocabulary) is not cleanly attributable to the profile rather than the model's prior training without evidence that the framework label would not appear under a condition with the profile removed and the model's training-data knowledge of Snyder preserved.

The paper is closer to publishable than many submissions in this space, and I expect the revision to be tractable. The honest treatment of limitations in Section 5.4 suggests the authors understand the gap; the revision should close it rather than acknowledge it.
