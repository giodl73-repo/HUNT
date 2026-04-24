# Review: What Does a Persona Do? Cognitive Trait Decomposition of Authoring Identity in AI Creative Task Generation
**Reviewer:** Tony Veale | **Round:** 1

## Scores (1-4)
Novelty: 3 | Rigor: 3 | Clarity: 4 | Significance: 3 | **Overall: 3.25**

## Overall Assessment
The paper presents a carefully designed ablation study of persona prompt effects on AI creative output, with a clear central finding: experience-first orientation (T1) is architecturally irreplaceable, elegance drive (T4) and rigor (T5) are load-bearing, and the combination T1+T4+T5 constitutes a minimum sufficient set for passing quality on a domain-expert rubric. The writing is unusually clear and the experimental design is among the most systematic I have seen applied to this question. My concern is about scope: the paper's claims range from the specific and well-supported (this trait decomposition, this domain, this rubric) to the broad and unsubstantiated (any creative task, any domain, business documents, legal drafting). I am skeptical that puzzle hunt authoring is a sufficiently representative creative domain to warrant these claims, and the paper needs to either empirically support them or substantially retract them.

## Strengths

1. **The three-phase design is genuinely elegant.** The combination of isolation, leave-one-out, and reconstruction addresses the same question from three complementary angles and produces converging evidence. This is good science. The reconstruction curve (Table 3) in particular is a clean and informative visualization of the causal structure.

2. **The T6 (systematic thinking) below-baseline result is theoretically interesting.** A trait that operationalizes expertise-style structured problem decomposition (Chi et al.) scores *below* the null baseline in isolation. This is consistent with Sawyer's improvisation account, and the paper connects these dots. The observation that proceduralism is the opposite of what the highest-weighted rubric dimensions (Elegance, Reading Reward) reward is a non-trivial finding about the relationship between domain expertise and creative quality.

3. **The elegance paradox is handled with theoretical sophistication.** The distinction between transparency (elimination of the solver's journey) and clarity (an unobstructed journey) is conceptually sharp and adds genuine value to the discussion of what "elegance" means in a creative artifact context.

4. **Clear, direct prose.** The paper says what it means. The analysis of each leave-one-out condition is specific and diagnostic --- the paper explains exactly how each puzzle fails, not merely that it fails.

## P1 — Blocking Issues

1. **The generalizability claim is the paper's main theoretical contribution but has no empirical support.** The abstract claims the minimum sufficient set "generalizes to any creative generation task where audience experience is the primary design constraint." The introduction adds "marketing copy, product design specifications, legal plain-language drafting, training material authoring." The discussion states "A business analyst applying these three sentences to a deliverable will produce a more audience-oriented, less cluttered, and more accurate artifact." None of these claims is tested. The entire empirical basis is a single creative domain (puzzle hunt authoring), a single answer word (TOWER), a single game world (Age of Empires II), and a single evaluation panel (C8). Puzzle hunt authoring is an unusually constrained creative task with a specific measurable quality criterion (solvability by an expert solver). The claim that T1+T4+T5 generalizes to legal plain-language drafting requires at minimum a theoretical argument for why the rubric dimensions that T1+T4+T5 address in puzzle authoring are the same dimensions that determine quality in legal writing. The paper does not provide this argument. The generalizability claims must be reframed as hypotheses to be tested, not findings to be reported.

2. **"Domain-naive artistic orientation outperforms domain-specific expertise" is overclaimed.** The comparison is between two bare two-to-four word labels ("You are an artist" vs. "You are a puzzle designer"). This is not a comparison between artistic orientation and domain expertise --- it is a comparison between the LLM's prior associations with two different identity tokens. The artist condition outperforms not because artistic training instills certain cognitive habits, but because the model's training data associates "artist" with the six trait dispositions the paper identifies, while "puzzle designer" activates template-reproduction behaviors. The paper conflates the model's associative properties with a general claim about artistic versus domain-specific cognition. This conflation needs to be addressed: the findings are about prompt semantics in this model, not about artistic versus domain cognitive orientations in general.

3. **The evaluation oracle is an AI panel.** The three "expert reviewers" (Katz, Snyder, Young) are LLM-instantiated from markdown profiles. A paper about what makes AI creative output good must not use AI judgment as its validity ground. This is circular. Either the paper needs a human validation component or it needs a much more careful epistemic discussion of what the C8 panel is actually measuring and why its scores should be trusted as proxies for the quality judgments that the generalizability claim requires.

## P2 — Important Issues

1. **A single answer word and a single game world severely limit internal validity.** All conditions design a puzzle with answer TOWER using AoE2 data. This means the differences between conditions may partially reflect differences in how the answer word TOWER interacts with each trait disposition, not purely how the trait dispositions affect creative quality in general. T3 (visual thinking) scoring above T1 (experience-first) in isolation may be an artifact of TOWER's visual iconicity as a concept. Replication with at least two or three answer words would substantially improve the internal validity argument.

2. **The comparison to "the gap between any two adjacent reviewer ablation conditions" is self-referential.** The paper's second sentence claims the artist advantage "is larger than the gap between any two adjacent reviewer ablation conditions in the companion reviewer-profile depth study." This comparison grounds the significance of the 15.6-point gap against a benchmark from papers by the same author. For an independent reader, this benchmark is opaque without more context about the distribution of score gaps in that companion study.

## Recommendation: Major Revision
