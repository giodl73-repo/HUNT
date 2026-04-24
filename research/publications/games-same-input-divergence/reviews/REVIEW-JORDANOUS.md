# Review: Structured Divergence in AI Creative Generation: A Same-Input Study of Puzzle Hunt Pipeline Outputs
**Reviewer:** Anna Jordanous | **Round:** 1

## Scores (1-4)
- Novelty: 3/4
- Rigor: 2/4
- Clarity: 3/4
- Significance: 3/4
- **Overall: 2.8/4**

## Overall Assessment
This paper introduces a genuinely interesting experimental paradigm and reports a clean empirical result — specification drives convergence — that has real practical resonance for AI creative pipeline deployment. However, it consistently uses the vocabulary of creativity without engaging seriously with the conceptual question of whether any of what is being studied constitutes creative behaviour in a theoretically grounded sense. The phenomenon labeled "domain semantic gravity" is presented as a discovery but lacks the theoretical grounding that would distinguish it from a known artefact of training data distribution. At N=2, the empirical base is insufficient to support the distributional language used throughout.

## Strengths
1. The same-input divergence paradigm is a well-motivated inversion of prompt sensitivity analysis and fills a real gap: prior work either maximises diversity, controls it, or measures it holistically. The clean articulation of what this paradigm is and what it is not (Section 3.1) is the strongest methodological writing in the paper.
2. The five-level comparison framework is principled and sensibly ordered from coarse to fine. The decision to set thresholds against domain calibration rather than conventional values is appropriate, even if the calibration choices could be better defended.
3. The specification-drives-convergence finding is crisp, practically actionable, and empirically clean. The fact that it overturns the authors' own domain-constraint hypothesis is a mark of intellectual honesty.
4. The connection to design space theory (Simon, Lawson, Gero) is a genuine theoretical anchor, not mere citation decoration, and the mapping from the FBS model onto the stable/variable partition is substantive.
5. The paper clearly distinguishes between aggregate-level and dimension-level panel score stability, which is the right distinction to draw. The finding that Elegance and Reading Reward shift systematically (+1.2) while Clarity is stable is meaningful.

## Major Concerns (P1 — blocking)

**P1.1: No engagement with what creativity means in this context.**
The abstract and conclusion repeatedly use the phrase "AI creative generation" and the word "creative" appears throughout as an unexamined qualifier. The paper is presented at a computational creativity venue. Jordanous (2012, SPECS framework) and other CC evaluation work require that claims about creative systems engage with criteria for creativity — novelty, value, intentionality, process. This paper never asks: is the divergence it measures evidence of *creativity* in either run? Is the convergence evidence of *constraint* or of *lack of creativity*? Is "domain semantic gravity" a creative phenomenon or a statistical one? Without this engagement, the paper is a sound engineering study of pipeline reliability that has been framed in CC vocabulary without CC grounding. The authors should either (a) explicitly argue that same-input divergence is a relevant proxy for a CC property (e.g., surprise, novelty relative to a generator's prior outputs), or (b) acknowledge that the paper is about pipeline characterisation rather than creativity evaluation and situate it accordingly. Submitting to ICCC without this framing is a mismatch of venue.

**P1.2: N=2 is acknowledged but the implications for inferential language are not followed through.**
Section 5.4 (Limitations) notes the N=2 problem and calls for N≥10 replication. But the paper continues, after this acknowledgment, to use language that implies distributional knowledge: "the pipeline produces structurally sound puzzles in both runs" (Discussion), "the panel is measuring a quality dimension that is invariant" (Discussion), "domain semantic gravity is a tendency" (Discussion). "Tendency," "invariant," and similar distributional claims cannot be established from two data points. Every sentence making a distributional inference must be hedged to reflect that these are provisional observations from a single case study. The limitations section acknowledges this but the body of the paper does not consistently honour the caveat.

## Minor Issues (P2)

**P2.1: "Domain semantic gravity" requires a more careful theoretical account.** The phenomenon — that thematically adjacent but lexically distinct words appear in corresponding slots across runs — is interesting. But it is precisely what one would predict from any LLM trained on a corpus in which the AoE2 domain occurs coherently: the conditional distribution over valid AoE2 words clusters around domain-salient concepts. The paper presents this as a newly identified phenomenon without asking whether it is distinguishable from the known behaviour of LLM sampling from a semantically structured training distribution. A brief treatment of what would falsify "domain semantic gravity" as a distinct phenomenon — e.g., showing it does not occur in domains without coherent semantic clustering, or that it occurs at rates above chance given the vocabulary size — would substantially strengthen the claim.

**P2.2: The rubric incompatibility problem in panel score comparison is mentioned but not adequately addressed.** The comparison-all-levels.md data reveals that Hunt 1 used a 6-dimension, 30-point, uniform-weight rubric while Hunt 2 used a 7-dimension, 45-point rubric with double-weighted Elegance and Reading Reward. Section 3.4 notes that scores are "normalized to percentage of maximum," but normalisation does not solve the weighting difference: if Elegance is double-weighted in Hunt 2 and Hunt 2 scores higher on Elegance, the normalized aggregate difference partially reflects a rubric artefact rather than a quality difference. The +1.2 uplift in Elegance and Reading Reward, reported as a genuine mechanism-level improvement (Discussion, Section 5.3), may be partially confounded by the rubric change. This needs to be acknowledged explicitly and either disentangled or treated as a limitation on the panel stability claim.

**P2.3: The paper would benefit from a brief operational definition of "creative brief" as used in the specification-drives-convergence claim.** The SCOPE.md document specifies scale, narrator, audience, format, solve time, and domain. But these are largely logistical or stylistic constraints rather than creative directives. A stronger claim would distinguish between creative specification (what aesthetic choices the brief makes) and logistical specification (what formal properties the brief fixes), and ask whether the convergence is on creative properties or merely on formal ones.

## Recommendation
Major Revision
