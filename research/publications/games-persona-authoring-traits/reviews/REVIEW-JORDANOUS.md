# Review: What Does a Persona Do? Cognitive Trait Decomposition of Authoring Identity in AI Creative Task Generation
**Reviewer:** Anna Jordanous | **Round:** 1

## Scores (1-4)
Novelty: 4 | Rigor: 3 | Clarity: 4 | Significance: 3 | **Overall: 3.5**

## Overall Assessment
This paper makes a genuine empirical contribution to the computational creativity literature by decomposing a holistic persona prompt into six independently testable cognitive traits and identifying the minimum sufficient subset for quality-threshold creative output. The three-phase experimental design (isolation, leave-one-out, reconstruction) is well-conceived and yields a coherent causal story. However, the paper does not adequately situate its contributions within computational creativity theory, particularly the SPECS framework, and the mechanism novelty finding --- the artist's "damaged inscription" escaping the acrostic gravity of every other condition --- remains underexplained at the theoretical level despite being the study's most striking empirical result.

## Strengths

1. **Three-phase experimental design is methodologically sound.** The combination of trait isolation (Phase 1), leave-one-out ablation (Phase 2), and cumulative reconstruction (Phase 3) is an unusually rigorous approach to causal decomposition in a creative generation context. The triangulation across all three phases strengthens the minimum sufficient set claim substantially.

2. **Mechanism novelty finding is genuinely novel.** No prior computational creativity study has empirically documented a case where a domain-naive persona escapes the mechanism-family gravity of domain-specific personas. The artist condition producing semantic reconstruction (damaged inscription) while all six trait-isolation conditions remain within syntactic extraction (indexing, acrostic) is striking and theoretically important.

3. **Trait formulations are carefully grounded.** Linking each trait to an established theoretical source (Norman, Rams, Snyder, Berlyne, Arnheim, Chi) before operationalizing it as an imperative sentence is good practice. The operationalization as domain-agnostic behavioral directives is principled.

4. **The elegance paradox is clearly analysed.** The observation that T4 in isolation drops below the null baseline while T4-removal from the full persona drops the score below threshold (i.e., T4 is harmful alone but structurally load-bearing in combination) is a theoretically important finding about trait interaction, and the paper explains it well in the discussion.

## P1 — Blocking Issues

1. **No engagement with SPECS or other CC evaluation frameworks.** SPECS (Sonnenschein et al. 2020, extending Jordanous 2012) decomposes creative competence into 14 components including intentionality, domain competence, active generation, and originality. The six traits in this paper are empirically derived from a single-system persona prompt, but there is no analysis of how they relate to or map onto established CC component frameworks. Is T1 (experience-first) an instantiation of intentionality? Does T5 (rigor/verification) correspond to evaluation competence? The paper cannot claim a principled cognitive basis for the trait taxonomy without this mapping. Without it, the reviewer cannot determine whether the taxonomy adds to CC theory or rediscovers existing components in new vocabulary.

2. **The mechanism novelty finding lacks a causal explanation.** The paper documents that no single trait produces the damaged-inscription mechanism, that the full artist persona does, and that T1+T4+T5 is the minimum sufficient set. But it does not explain *why* the combination produces a categorically different mechanism family. The discussion section (4.1) argues from the domain-labels-vs.-artistic-orientation distinction, but this is interpretive rather than mechanistic. For ICCC, the central question for any system claiming a creative output is: what is the generative process by which the novel mechanism was produced? The paper provides a post-hoc account (experience-first asks "what does the solver do?") but not a process account. Without this, the mechanism novelty claim rests on the evaluation result rather than on an understanding of the computational process.

3. **Single-domain, single-task design limits generalizability claims.** The introduction and conclusion make strong generalizability claims ("any creative generation task where audience experience is the primary design constraint," "marketing copy, product design specifications, legal plain-language drafting"). These claims are not supported by a single-domain study. The paper acknowledges this in the future work section but continues to make the claims as findings rather than predictions throughout the body. Generalizability claims must be either scoped to the demonstrated domain or moved fully to the future work section.

## P2 — Important Issues

1. **The C8 panel is AI-instantiated, not human.** Three reviewers (Katz, Snyder, Young) are described as "domain-expert reviewer profiles ... instantiated from 80-120 line structured markdown profiles." This means the evaluation oracle is itself an AI system, not human experts. The paper does not discuss the reliability or validity of this evaluation infrastructure relative to actual human expert judgment. For a paper whose central claim is about creative quality, the validity of the quality measure is foundational. A brief human-validation comparison (even for a subset of conditions) would substantially strengthen confidence in the rubric scores.

2. **Score variance across sessions is unaddressed.** The paper reports AA scoring 40.3 in the motivating study, 41.7 in Phase 2, and 42.3 in Phase 3 --- a 2-point drift across evaluation sessions. The paper attributes this to "reviewer variance" but does not model or quantify it. A drift of 2 points against a threshold gap of 2.7 points (R2 at 30.3 vs. threshold at 33) means session variance is nontrivially large relative to the detection required. The conclusion that T1+T4+T5 is the minimum sufficient set may be an artifact of session-specific calibration.

## Recommendation: Major Revision
