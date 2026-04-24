# Review: Profile Depth and the Reviewing-to-Authoring Transfer Problem: Evidence from AI Expert Personas in Puzzle Hunt Design
**Reviewer:** Tony Veale
**Expertise:** Computational creativity, creative language, conceptual blending, AI generativity
**Venue:** ICCC 2026

## Scores (1-4)
- Novelty: 3/4
- Rigor: 2/4
- Clarity: 3/4
- Significance: 2/4
- **Overall: 2.5/4**

## Overall Assessment

The paper reports a well-controlled empirical study with a genuinely interesting finding — that the profile-depth gradient for AI reviewers mirrors the gradient for AI authors — and the Young perceptual-lens result is striking. My concern is that the theoretical framing conflates what is really a prompt-engineering finding with a claim about creativity and creative cognition. The paper's strongest contribution is empirical (the symmetric gradient), but its theoretical ambition (the lens-type transfer model as an account of reviewing-to-authoring transfer) exceeds what the evidence can support, and the creativity question — whether any of these outputs are genuinely creative — goes unasked.

## Overall Assessment

The paper reports a careful ablation study and introduces a three-category lens taxonomy that, even if not yet fully validated, offers practitioners a useful vocabulary for thinking about the generativity of reviewer profiles. However, the paper's relationship to the ICCC core question — what does this tell us about computational creativity? — remains underdeveloped. The most interesting creativity-theoretic claim (that forcing a format mismatch produces productive substitution in Young's case) is treated as a design implication rather than explored as a model of creative cognition.

## Strengths

- The experimental design is unusually clean for this kind of study: same answer word, same mechanism, same rubric, same evaluation panel across all conditions. The ablation logic is sound.
- The A2 domain-inversion effect is the most creativity-theoretically interesting result: adding raw content without a framework degrades rather than improves creative output. This is a meaningful empirical fact about how LLMs behave under persona prompting.
- The Young A6 finding — that blocking the default approach (visual grammar) forces a productive structural substitution — is a genuine insight about creative problem-solving under constraint. It deserves more theoretical treatment than it receives.
- The six-framework taxonomy in Section 4.3 is original and useful. The frameworks name phenomena that puzzle hunt practitioners recognize but have not previously codified.
- The paper is honest about what it is not measuring (mechanism novelty, genuine domain-native creativity) and where the ceiling is.

## Major Concerns (P1 — blocking)

**P1.1: The paper does not engage with the question of whether any of the outputs are creative.**
ICCC is a creativity-focused conference. The paper studies AI authoring quality, but "quality" is defined by rubric score, and the rubric measures craft properties (clarity, solvability, elegance, reading reward) rather than creative properties in any theoretically grounded sense. Whether a puzzle that scores 38.0/45 on this rubric is creative in any meaningful sense — whether it is novel, surprising, valuable, or authentic in Wiggins' sense — is never asked. The paper would be stronger at ICCC if it either: (a) argued that the rubric dimensions are proxies for creativity-relevant properties and linked them to established computational creativity frameworks, or (b) acknowledged explicitly that the paper is about craft quality rather than creativity per se, and adjusted its ICCC positioning accordingly. As written, the paper reads as a prompt-engineering study that has been submitted to a creativity conference without clearly articulating why its findings matter for creativity theory.

**P1.2: The symmetric-gradient claim is not differentiated from a simpler story.**
The paper argues that the same structural properties govern profile depth in both reviewing and authoring directions, implying something theoretically interesting about the symmetry. But the simpler story is: more informative prompts produce better outputs, with the same diminishing-returns pattern that has been documented across dozens of prompting studies. What does the symmetry tell us that this simpler account does not? The paper asserts that "the mechanisms that determine profile quality are not specific to the evaluation task" — but the mechanism it describes (framework > content) is exactly what the prompting literature already says. The symmetric-gradient finding needs a stronger theoretical differentiation from what is already known.

**P1.3: The lens-type taxonomy is proposed as a classification scheme without a generative principle.**
Construction/operational/perceptual are labels applied after the fact to three designers' results. The paper does not explain what underlying cognitive or computational principle distinguishes these lens types, nor does it say how a reader would classify a new designer's profile without having already observed their A5-to-A6 gap. A taxonomy that can only be applied retrospectively is descriptive rather than predictive. For the model to function as claimed ("predicts which reviewer profiles can be safely repurposed"), it needs a classification procedure that operates on profile content rather than on observed transfer outcomes.

## Minor Issues (P2 — important but not blocking)

**P2.1: The Young A6 "forced substitution" finding needs more theoretical treatment.** The mechanism Young uses — second-person present-tense narrative that places the solver inside a defensive scenario — is genuinely interesting. Why does blocking the visual grammar approach produce a more creative outcome? This is reminiscent of work on creative constraints (Stokes) and the role of obstacle-induced reformulation in creativity. The paper would benefit from engaging with this literature even briefly rather than treating the finding as a practical note about perceptual lenses.

**P2.2: The six evaluative frameworks in Table 4 are summarized but not analyzed for ICCC.** A-ha economy, load-bearing test, world-model consistency, social fabric, perceptual-shift, and visual grammar are the paper's most interesting qualitative contribution, but they appear in Section 4.3 without connection to computational creativity theory. At minimum, mapping them to established constructs (e.g., perceptual-shift connects to Wiggins' notion of exploratory creativity; social fabric connects to collaborative creativity models) would significantly increase their theoretical value.

**P2.3: The phrase "the profile's framework determines what the model does with the information it holds" is doing significant theoretical work but is never supported.** This claim appears in Section 5.1 and is presented as an explanatory mechanism for the symmetric gradient. What is the model actually doing when it "processes" a framework-enriched profile versus a content-only profile? Without at least a hypothesis about the mechanism, the claim risks being circular (the framework works because it is a framework).

**P2.4: No discussion of whether results generalize beyond LLMs.** The paper is about AI expert personas, but its claims are framed at the level of "profiles" and "authoring quality" without specifying that these are LLM-specific behaviors. ICCC reviewers will want to know whether the results speak to computational creativity systems more broadly or only to prompted LLMs.

## Recommendations (P3 — suggested improvements)

**P3.1:** Add a section or substantial paragraph connecting the lens-type model to computational creativity frameworks — specifically to the question of generativity: what makes a representation generative rather than terminal? This is the ICCC audience's natural question and the paper's results speak to it directly.

**P3.2:** Engage with at least two or three creativity-theoretic citations that ground the "forced substitution" result — Stokes on creative constraints, Ward on path of least resistance in creative cognition, or Boden on exploratory versus transformational creativity — to show that Young's A6 result is not an anomaly but an instance of a known creative mechanism.

**P3.3:** Consider reframing the abstract's contribution statement to foreground what the ICCC audience will find valuable: not "practical guidance for constructing authoring profiles" (a software engineering claim) but "evidence that profile depth is a structural property of how constrained generation systems engage complex tasks" (a creativity systems claim).

## Recommendation
Major Revision
