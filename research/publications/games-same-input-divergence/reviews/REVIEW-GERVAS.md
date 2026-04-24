# Review: Structured Divergence in AI Creative Generation: A Same-Input Study of Puzzle Hunt Pipeline Outputs
**Reviewer:** Pablo Gervás | **Round:** 1

## Scores (1-4)
- Novelty: 3/4
- Rigor: 2/4
- Clarity: 3/4
- Significance: 3/4
- **Overall: 2.8/4**

## Overall Assessment
The paper raises a question of genuine theoretical interest — whether the stable/variable partition in a multi-stage AI creative generator is principled or incidental — and provides empirical data that supports a principled account. The specification-drives-convergence finding is the right answer to a real question. My concern is that the paper uses the language of "design decisions" throughout without engaging with what decision-making means for a pipeline that has no deliberate cognitive process. The specification-drives-convergence claim is empirically supported but theoretically underspecified: it conflates the pipeline making a decision with the pipeline satisfying a constraint, and this conflation leads to a practical recommendation that is correct but undertheorized.

## Strengths
1. The same-input divergence paradigm is a principled complement to prompt sensitivity analysis and the introduction positions it clearly and compellingly. The distinction between divergence-as-instrument and diversity-as-goal is the paper's clearest theoretical contribution.
2. The five-level comparison framework is well-motivated by the structure of the generative artifact. The decision to compare at the level of extraction mechanism (Level 4) rather than only at puzzle type is a methodologically important choice that reveals the partial convergence at Slot 1 (indexed letter extraction) that a coarser analysis would miss.
3. The design space theory anchoring (Simon, Lawson, Gero FBS model) is appropriate and the mapping is substantive. The claim that "stable decisions are over-determined by functional requirements" maps correctly onto the Gero framework. Few empirical CC papers engage with design cognition theory at this level of specificity.
4. The domain semantic gravity finding is genuinely novel in the sense that it was not anticipated by the study design. The observation that three of five feeder slots yield domain-adjacent word pairs across zero-Jaccard runs is the kind of empirical surprise that warrants theoretical development, even if the current account is underspecified.
5. The acknowledgment that Hunt 2's Elegance and Reading Reward uplift (+1.2) reflects a genuine mechanism-level improvement rather than rubric noise is appropriately careful and the argument is plausible.

## Major Concerns (P1 — blocking)

**P1.1: The paper uses "decision" language for a pipeline that makes no decisions in any meaningful cognitive sense.**
Throughout the paper, the pipeline "decides" round count, "chooses" meta mechanism type, and "selects" answer words. The design space theory section draws on Lawson's work on *human* designers who deliberately identify constrained versus free decisions. But the pipeline does none of this: it generates outputs by sampling from a conditional distribution. The "decision" that is "stable" is not a decision at all — it is a region of the output space that the conditional distribution concentrates probability mass on given the input specification. The "decision" that is "variable" is a region of the output space where the conditional distribution is flat given the input.

This is not a pedantic distinction. It matters for the specification-drives-convergence claim. The paper's recommendation — "specify more in the creative brief to stabilize pipeline outputs" — is correct, but the theoretical account of *why* it works is wrong if the mechanism is statistical (specification shifts the conditional distribution) rather than cognitive (specification helps the pipeline deliberate more effectively). The paper should either (a) explicitly adopt a functionalist stance and argue that the cognitive vocabulary is useful as a description of computational behaviour regardless of underlying mechanism, or (b) reframe the theoretical account in distributional terms without the decision-making language. The design cognition citations (Lawson, Simon) support option (a) if explicitly argued; currently they are invoked implicitly without the argument being made.

**P1.2: The specification-drives-convergence claim does not distinguish between convergence caused by the specification and convergence caused by the domain.**
The paper claims that "domain knowledge is neither necessary nor sufficient for stability; specification is both." But the evidence for this claim is that (a) specified properties are stable and (b) domain-level properties (answer words, mechanisms) are variable despite being drawn from a constrained AoE2 vocabulary. This supports the claim that domain knowledge is not *sufficient*, but it does not support the claim that domain knowledge is not *necessary*. If the domain were replaced with an arbitrary sequence of tokens with no semantic structure, would specification still drive convergence? The paper cannot answer this from N=2 in a single domain. The claim about necessity should be hedged to reflect that it is speculative relative to the evidence.

## Minor Issues (P2)

**P2.1: The "domain semantic gravity" claim would benefit from a cognitive account of the mechanism.** The paper identifies the phenomenon (domain-adjacent word pairs in 3/5 slots despite Jaccard 0.00) but does not propose a mechanism. Is gravity a property of the LLM's internal representation of the AoE2 domain (semantic clustering in the embedding space)? Is it a property of the structured nature of the puzzle hunt task (both runs sample from the same finite vocabulary of valid AoE2 answer words)? Is it a property of the generative pipeline's prompt structure (both runs are told to select "thematically resonant" answer words from the world data)? These are distinguishable mechanisms with different implications for the gravity phenomenon. Even a brief treatment of which account is most consistent with the data would strengthen the theoretical contribution.

**P2.2: The discussion of panel score stability would benefit from explicit treatment of the reviewer calibration confound.** Hunt 1 used domain-matched panels (different reviewer profiles per puzzle) and Hunt 2 used a fixed panel (Katz, Snyder, Dana). If the Katz/Snyder/Dana profiles are systematically more generous on Elegance and Reading Reward than the Hunt 1 domain-matched profiles, the +1.2 uplift is partially or wholly a reviewer calibration effect rather than a genuine quality improvement. The paper acknowledges the rubric difference but does not address this reviewer-selection confound directly.

**P2.3: The paper's contribution claim list is longer than the contributions warrant.** Five contributions are listed in the Introduction: the paradigm, the empirical data, the specification-drives-convergence finding, domain semantic gravity, and the practical recommendations. Contributions 3 and 5 are not independent (the practical recommendations follow directly from the finding), and contribution 2 (empirical data) is a description of the study rather than a claim about the world. Three clean contributions — the paradigm, the specification-drives-convergence finding, and domain semantic gravity — would be sharper and more defensible.

**P2.4: The paper could more directly address whether the specification-drives-convergence finding is a property of this specific pipeline or of LLM-based creative pipelines in general.** The paper positions the finding as generalizable but the evidence is domain-specific and pipeline-specific. A brief argument for what properties of the pipeline (multi-stage, specification-grounded, LLM-based) are responsible for the finding — and which pipelines with different properties might not exhibit it — would sharpen the scope of the contribution.

## Recommendation
Major Revision
