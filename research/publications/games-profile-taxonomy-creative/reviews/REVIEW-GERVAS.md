# Review: Profile Depth and the Reviewing-to-Authoring Transfer Problem: Evidence from AI Expert Personas in Puzzle Hunt Design
**Reviewer:** Pablo Gervás
**Expertise:** Computational creativity, narrative generation, cognitive models of creative cognition, authoring/reviewing as cognitive processes
**Venue:** ICCC 2026

## Scores (1-4)
- Novelty: 3/4
- Rigor: 3/4
- Clarity: 4/4
- Significance: 3/4
- **Overall: 3.25/4**

## Overall Assessment

This is one of the more carefully designed empirical studies I have reviewed in this area. The authoring ablation is well-constructed, the symmetric-gradient finding is both credible and interesting, and the lens-type model makes a genuine contribution to understanding how reviewing and authoring relate as cognitive activities. My main reservation is that the paper invokes a rich cognitive framework in the related work — Csikszentmihalyi's systems model, Weisberg on expert creative performance, Polanyi on tacit knowledge — and then does not follow through on these theoretical resources in the discussion. The lens taxonomy is not grounded in the cognitive model that the related work introduces. This is the paper's primary gap: the theory and the empirics are adjacent but not integrated.

## Overall Assessment

The paper writes clearly and thinks carefully about the cognitive distinction between reviewing and authoring. The three-lens taxonomy (construction, operational, perceptual) is the paper's most distinctive contribution. The symmetric gradient is an important empirical result. Both contributions would be strengthened if the paper completed the cognitive grounding it begins in Section 2.3.

## Strengths

- The authoring/reviewing cognitive distinction is introduced with genuine theoretical care in Section 2.3. The Csikszentmihalyi/Weisberg/Polanyi framework is correctly applied and sets up the paper's central question well.
- The A2 inversion result (domain data without framework) is a clean empirical finding with a clear cognitive interpretation: factual knowledge without procedural structure does not activate domain competence in the generative direction.
- The A4 approximately A5 result (worldview alone captures most of the value of the full profile) is cognitively interesting: it suggests that the generative starting position — where a designer begins — is the cognitively loaded variable, with the checklist functioning as post-hoc verification.
- Clarity throughout is high. The paper explains its conditions, rubric, and findings in language that is precise without being unnecessarily technical. Section 4 is particularly clear.
- The Young A6 finding — forced substitution through format constraint — is the most cognitively interesting result and deserves the interpretive prominence the paper gives it.

## Major Concerns (P1 — blocking)

**P1.1: The cognitive model introduced in Section 2.3 is not operationalized in the lens taxonomy.**
Section 2.3 makes a meaningful theoretical claim: reviewing profiles operationalize "knowing-that" (Polanyi) while authoring profiles must encode "knowing-how." The lens taxonomy (Section 3.3) introduces three lens types, but the classification is not connected to this knowing-that/knowing-how distinction. Are operational lenses closer to knowing-how? Is the key property that they can be converted to procedural instructions rather than remaining as declarative checks? The paper asserts that operational tests "invert cleanly" but does not explain what makes an inversion clean in cognitive terms. Similarly, the claim that perceptual lenses are "terminal" should connect to the knowing-that / knowing-how distinction: perceptual checks are purely declarative (they describe a desired state of a finished artifact) in a way that operational checks are not. Making this connection explicit would unify the related work and the findings sections and give the lens taxonomy a cognitive foundation rather than a purely empirical one.

**P1.2: The knowing-how claim is untested by the experimental design.**
The paper's most important theoretical claim — that reviewing profiles operationalize knowing-that while authoring profiles require knowing-how — is never directly tested. The experimental evidence shows that some reviewing profiles transfer better than others when repurposed for authoring, but it does not measure whether the transferred content has knowing-how character. The paper infers the cognitive property from the output quality, which is circular: operational profiles transfer because they have authoring-compatible content, and we know they have authoring-compatible content because they transfer. A direct measure of the profiles' procedural versus declarative content (e.g., by tagging profile statements as prescriptive/procedural vs. evaluative/declarative) would break this circularity and ground the cognitive claim empirically.

**P1.3: The Young A6 finding is interpreted as "forced substitution" but the mechanism is not modeled.**
The paper's most striking result is that Young's A6 mode outperformed A5 because blocking visual grammar checks forced a narrative substitution. This is presented as a design implication rather than a cognitive finding. But the cognitive question is important: why does blocking one generative route activate another? Is this an instance of the iterative reformulation process that Weisberg describes as central to expert creative problem-solving? Is it related to constraint satisfaction in cognitive models of creativity? The paper raises this question by invoking the relevant theoretical framework in Section 2.3 and then does not return to it in the discussion. Section 5.2 describes the mechanism but does not interpret it in cognitive terms.

## Minor Issues (P2 — important but not blocking)

**P2.1: The Csikszentmihalyi "domain actor / field actor" framing in Section 2.3 is used to set up the authoring/reviewing distinction but is not returned to.** The systems model distinguishes creator, domain, and field — but the paper's experimental conditions conflate these. The AI system acting as author is a domain actor; the AI system acting as reviewer is a field actor. But both are simulated by the same underlying model with different profiles. The implications of this for the cognitive interpretation of the results — specifically, what it means for a field actor (reviewer profile) to successfully simulate a domain actor (author profile) — are not discussed.

**P2.2: The "tacit knowledge" argument in Section 2.3 is left hanging.** Polanyi's distinction between knowing-how and knowing-that is introduced as a theoretical resource but never operationalized in the study design or results interpretation. If tacit procedural knowledge cannot be fully recovered from declarative statements about quality standards (as Section 2.3 argues), how does the paper explain that some reviewing profiles do transfer productively? Either the tacit knowledge claim needs qualification or the paper needs to explain why operational profiles are exceptions to it.

**P2.3: Figure 1 (gradient comparison) is referenced but not described in the main text.** The paper references "Figure 1" at the end of Section 4.1 as evidence for the symmetric-gradient claim, but provides no caption or description of what the figure shows. Even a brief description in the text of what the reader should observe in Figure 1 would help.

**P2.4: The "starting orientation" divergence across the three A5 designers (Section 3.2) is mentioned but not analyzed as a result.** Snyder starts with the target deduction; Katz with the answer word; Young with the world. This divergence is presumably connected to the different lens types — but the paper does not make this connection explicit. If starting orientation predicts lens type, that would be a useful finding.

## Recommendations (P3 — suggested improvements)

**P3.1:** Add a paragraph in Section 5.2 that maps the three lens types explicitly onto the knowing-that/knowing-how distinction introduced in Section 2.3. Operational lenses carry procedural content that can be applied during construction; construction lenses are already procedural by nature; perceptual lenses are purely declarative and cannot be applied before the artifact exists. This mapping would unify the theoretical and empirical contributions.

**P3.2:** In Section 5.2, engage briefly with the cognitive creativity literature on reformulation under constraint (Weisberg, Ward, or Finke et al.) to ground the Young A6 "forced substitution" finding in a cognitive model rather than treating it as an anomaly.

**P3.3:** Consider adding a methodological note on the procedural-versus-declarative character of the profiles used, even informally. A table showing which profile statements are prescriptive (do X during construction) versus evaluative (assess whether X is true) would provide evidence for the cognitive claims that currently rest on inference from output quality.

## Recommendation
Minor Revision
