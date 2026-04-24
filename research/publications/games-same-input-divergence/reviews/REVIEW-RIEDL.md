# Review: Structured Divergence in AI Creative Generation: A Same-Input Study of Puzzle Hunt Pipeline Outputs
**Reviewer:** Mark Riedl | **Round:** 1

## Scores (1-4)
- Novelty: 3/4
- Rigor: 2/4
- Clarity: 4/4
- Significance: 4/4
- **Overall: 3.3/4**

## Overall Assessment
This paper identifies a practically important and theoretically underexplored question — where in a multi-stage AI creative pipeline does stochasticity matter and where does it not? — and provides credible preliminary evidence for a clean answer. The domain semantic gravity finding is the most original theoretical contribution and deserves more careful development than it receives. My main concern is that the paper's central framing — "specification drives convergence" — is correct as a description of the observed data but is offered as a causal claim without the mechanism being established. The implications for controlling AI creative systems depend on whether the mechanism is specification-as-constraint, specification-as-prior, or specification-as-attention-anchor, and these have very different practical consequences.

## Strengths
1. The significance of the paper's contribution is high for anyone working on deployed AI creative systems. The question "which outputs can I trust from a single run, and which do I need to sample and select?" is live and practical, and the paper provides a concrete empirical answer for a well-characterised pipeline. This is the kind of systems-facing empirical work that the AI narrative/creativity community needs more of.
2. The domain semantic gravity finding is independently interesting and has implications beyond puzzle hunts. An AI story generator seeded with a domain (e.g., Norse mythology) will cluster character names, locations, and plot events around domain-salient concepts across independent runs even when specific choices diverge. If this is a general property of domain-grounded LLM generation, it has consequences for any creative system where downstream content depends on specific named entities — which includes virtually all narrative AI.
3. The five-level comparison is methodologically clean and the stability/variable classification at each level is applied consistently. The finding that Level 1 (structural) and Level 3 (assignment) diverge completely while the panel-score dimension of Level 5 (micro-design quality) is stable is a non-obvious result that the multi-level analysis is necessary to reveal.
4. The connection between the panel score stability finding and the practical deployment question (can I use the AI panel aggregate as a reliable gate?) is exactly the right question to ask and the answer — stable at pass/fail, variable at dimension level — is a usable result.
5. The paper's writing is consistently clear and the results tables are well-constructed. The comparison-all-levels data (available in supplementary materials) is thorough and the micro-design comparison of Hunt 1 P1 versus Hunt 2 P1 is the kind of granular analysis that makes the general claims concrete.

## Major Concerns (P1 — blocking)

**P1.1: The mechanism of "specification drives convergence" is asserted but not established, and the different possible mechanisms have different implications.**
The paper observes that specified decisions are stable and unspecified decisions are variable, and attributes this to the specification functioning as a constraint on the design space. But there are at least three distinct mechanisms that could produce the same observed pattern:

(a) *Specification-as-constraint*: The pipeline checks its outputs against the specification and rejects outputs that violate it. Scale (5+1 puzzles) is stable because the pipeline cannot produce 6 puzzles when the spec says 5 — it is a hard filter.

(b) *Specification-as-prior*: The specification shifts the model's conditional distribution toward outputs that satisfy the specified properties, not by filtering but by narrowing the probability mass. Narrator voice (The Monk) is stable because the conditioning on "Monk voice rules" is strong enough that both runs sample from a narrow distribution around Monk-voice outputs.

(c) *Specification-as-attention-anchor*: The specification makes certain output dimensions salient in the model's context, focusing generation attention on those dimensions and implicitly leaving unmentioned dimensions under-constrained. Scale and format are stable because they are mentioned first and most prominently; round structure is variable because it is not mentioned and the model's attention is not directed toward it.

These are not the same mechanism. Mechanism (a) is a filter that can be applied post-hoc; mechanism (b) requires the specification to be in the model's context during generation; mechanism (c) depends on position and salience in the prompt. For practitioners designing creative briefs, the difference is significant: if (a), any formally checkable constraint can be made stable; if (b), the specification must be in context at the generation step for each decision; if (c), decision salience in the brief matters, not just decision presence. The paper should either distinguish these mechanisms and identify which is responsible, or explicitly acknowledge that the mechanism is unknown and the recommendation is heuristic.

**P1.2: The domain semantic gravity finding is presented as a discovery but not theorised at the level its importance warrants.**
Domain semantic gravity — that domain-grounded generators produce thematically coherent outputs across independent runs even when exact lexical choices diverge — is the paper's most original theoretical contribution. It is also the finding with the broadest implications for controlling AI creative systems (see Strength 2 above). Yet the paper gives it roughly one and a half pages of discussion, offers no theoretical account of the mechanism, and does not attempt to connect it to the extensive literature on semantic structure in LLM representations (e.g., conceptual clustering, domain adaptation, few-shot semantic priming). If this finding is real and general, it deserves either (a) its own empirical investigation or (b) a substantially expanded theoretical treatment that positions it relative to known LLM behaviours. As currently presented, it is an observation in search of a theory, and the paper's positioning of it as a "newly identified phenomenon" is defensible only if it is distinguished from what could be predicted from standard LLM semantic structure theory.

## Minor Issues (P2)

**P2.1: The paper should more carefully specify what "steering" the pipeline means in light of the findings.** The conclusion states that practitioners should "specify explicitly" decisions that must converge. But for decisions at Level 3 (answer words), domain semantic gravity means that even explicit specification of *semantic category* may not be sufficient to guarantee a specific word, because the semantic basin narrows the distribution without fixing a point. Practitioners who need PATROL (not MARCH) for a mechanism that depends on a specific letter pattern cannot achieve this by specifying "military verb" — they need to specify the word exactly. The paper's recommendation should be more precise about the granularity of specification required to guarantee convergence at each level.

**P2.2: The paper gestures at "evaluative-language stability" as a future direction in the conclusion but does not adequately motivate it.** The claim that AI panel reasoning patterns might exhibit the same structure as AI panel scores — stable in aggregate, variable in specific dimensions — is plausible but presented without argument. This should either be developed briefly (what is the theoretical basis for expecting evaluative-language stability?) or dropped from the conclusion.

**P2.3: The rubric change between Hunt 1 and Hunt 2 is a methodological problem that the paper treats as a limitation but does not fully address.** If the goal is to compare panel score stability across runs, the rubric should be fixed. The addition of the Riven Standard dimension in Hunt 2 and the reweighting of Elegance and Reading Reward make the panel score comparison substantially less clean than the paper represents. This is acknowledged in the supplementary data but not foregrounded in the main results section, where the +1.2 uplift in Elegance is presented as a clean finding rather than a potentially confounded one.

## Recommendation
Minor Revision
