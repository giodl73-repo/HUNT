# REVISION PLAN — Round 1
**Paper:** What Does a Persona Do? Cognitive Trait Decomposition of Authoring Identity in AI Creative Task Generation
**Venue:** ICCC 2026
**Status:** Major Revision
**Panel avg:** 3.3/4 | **Gate:** PASS

---

## Summary of Required Changes

The panel passes the paper on scores but requires four blocking revisions before acceptance. None of the four requires new experiments. The core empirical contribution is accepted; the revisions address theoretical positioning, scope of claims, and methodological transparency.

---

## P1-A: AI Panel Validity and Circularity

**Issue:** The C8 evaluation oracle (Katz, Snyder, Young) is AI-instantiated. The paper uses AI judgment to assess AI creative output without acknowledging the circularity or validating the panel against human expert judgment.

**Required revision:**

Add a subsection to the Methodology section (Section 3, after the current Evaluation Infrastructure subsection) titled "Evaluation Infrastructure Validity." This subsection must:

1. Explicitly acknowledge that the C8 panel consists of AI-instantiated reviewer profiles, not human experts, and that the scores therefore represent the quality judgments of a language model conditioned on those profiles.
2. Cite the companion reviewer-profile depth paper (dellaLibera2025panel) for evidence that the C8 panel's score distributions correlate with the qualitative patterns reported in human expert puzzle design discourse, as the closest available proxy for panel validity.
3. State explicitly what the circularity implies: the quality threshold and tier structure claims are valid to the extent that the C8 panel is a valid proxy for human expert judgment, and this proxy validity is assumed rather than independently verified in this study.
4. In the Conclusion, add one sentence acknowledging human validation as a required future step for establishing that the C8 panel's threshold crossings correspond to human-expert-recognized quality thresholds.

**Do not add a new human validation study.** The above disclosure is sufficient to address the P1-A concern without new data collection.

---

## P1-B: Scope Generalizability Claims

**Issue:** Domain-general claims ("any creative task," "business analysts," "legal drafting," "marketing copy") appear throughout the abstract, introduction, discussion, and conclusion without empirical support outside of puzzle authoring.

**Required revision (affects every section):**

1. **Abstract (lines 39-45):** Replace "generalizes to any creative generation task where audience experience is the primary design constraint" with "represents a domain-agnostic quality stance that we predict, subject to cross-domain replication, will generalize to creative tasks where audience experience is the primary design constraint."

2. **Introduction, Section 1.4 (Broader Significance):** Replace the three-sentence claim about business analysts, marketing copy, legal drafting, and training materials with: "The three-sentence injection (T1, T4, T5) encodes a cognitive stance rather than domain knowledge, and the theoretical grounding of each trait (Norman's user-centered design, Rams's reduction discipline, and implementation hygiene) applies to any authoring context where a human will experience the output. Cross-domain replication is required before this prediction can be reported as a finding; we place it in future work in Section 6."

3. **Discussion, Section 4.2 (Three-Trait Injection and Its Generalization):** The current paragraph beginning "A business analyst applying these three sentences..." must be clearly labeled as a prediction: "We predict that a business analyst applying these three sentences to a deliverable will produce a more audience-oriented artifact, for the structural reasons the three-trait puzzle passes the C8 panel. This is a hypothesis that cross-domain replication would confirm or disconfirm."

4. **Conclusion:** The sentence "We predict that prepending these three sentences to any creative generation prompt... will replicate the threshold-crossing effect" is appropriate as-is because it uses "predict." Verify that all similar claims in the conclusion are marked as predictions rather than findings.

5. **Add a "Scope of Claims" paragraph** to Section 4 (Discussion) or as a dedicated subsection, explicitly noting that the study's empirical scope is: a single creative domain (puzzle hunt authoring), a single answer word (TOWER), a single game world (AoE2), and a single evaluation panel instantiation (C8). State that the minimum sufficient set claim is valid within this scope and that domain-general applicability is a theoretical extrapolation pending replication.

---

## P1-C: Mechanism Novelty — Add a Process Hypothesis

**Issue:** The paper documents that no single trait produces semantic reconstruction while the full artist persona does, but provides no process-level account of why the combination produces a mechanism family escape.

**Required revision:**

Add a new subsection to Section 4 (Discussion), after Section 4.1, titled "A Hypothesis on Mechanism Family Escape." This subsection (200-300 words) should:

1. State explicitly that the paper does not provide a mechanistic account of the damaged-inscription escape, only an interpretive account.
2. Offer the following hypothesis, clearly labeled as a hypothesis to be tested: T1 (experience-first) functions as a gating condition for mechanism family access. In isolation, T1 redirects the problem frame from "what letters yield TOWER" to "what experience reveals TOWER," but without T4 and T5, the generative process lacks the reduction discipline and factual grounding to sustain a novel mechanism to completion. The acrostic/indexing family is the default attractor because it satisfies T1's experiential reorientation at minimum cost (the solver does "find" the answer, just through letter counting). T4's reduction constraint combined with T1's experiential frame may rule out explicit extraction tables as insufficiently elegant, and T3 (visual thinking) then generates a spatial metaphor (damaged inscription, missing piece) that instantiates T1's "what does the solver do?" question as a physical act of reconstruction. This would predict that T1+T3 alone might partially escape the acrostic family before T4 and T5 are added.
3. Note that this hypothesis can be tested with two additional conditions (T1+T3 and T1+T2) without the full Phase 3 infrastructure.
4. Connect the hypothesis to Pérez y Pérez's MEXICA model's engagement/reflection phases: T1 drives engagement-phase reframing; T4+T5 constitute a compressed reflection phase that prunes and verifies the reframed output.

---

## P1-D: Trait Taxonomy — Cognitive Grounding

**Issue:** The six traits are derived through introspective analysis cross-referenced against creativity theory, but the taxonomy lacks a principled account of exhaustiveness, independence, and theoretical grounding.

**Required revision:**

1. **Section 3.2 (Trait Taxonomy), expand the derivation paragraph:** Add 150-200 words explicitly positioning the six traits within an established creativity framework. The preferred framing is Boden's three types of conceptual space transformation (combinatorial, exploratory, transformational):
   - T1 (experience-first) and T4 (elegance drive) are exploratory constraints: they define the boundaries of the relevant conceptual space (audience experience as primary constraint; minimum form as aesthetic boundary).
   - T2 (surprise instinct) and T3 (visual thinking) are combinatorial operators: they combine existing concepts (domain knowledge + experiential frame) in unexpected configurations.
   - T5 (rigor/verification) is a transformational constraint: it prunes outputs that violate factual accuracy, which is a necessary condition for the exploration to produce artifacts that are surprising rather than merely wrong.
   - T6 (systematic thinking) does not fit cleanly into Boden's framework, which is consistent with its below-baseline isolation score: it is a problem-solving competency imported into the taxonomy as a check on whether domain-expertise-style traits contribute to creative quality (they do not, in this domain).

2. **Add a footnote or parenthetical** noting that the six-trait set is not claimed to be exhaustive: it represents the trait space accessible through introspective analysis of a single-word persona label, and additional traits may be identifiable through analysis of richer persona specifications or through application to other creative domains.

3. **Add a brief comparison to SPECS** (Jordanous 2012): note that the six traits span several SPECS components (intentionality, domain competence, active generation, originality) and that the minimum sufficient set (T1+T4+T5) maps primarily to intentionality and evaluation competence --- consistent with SPECS's emphasis on these components as foundational rather than supplemental.

---

## P2 Revisions (Strongly Recommended)

### P2-A: Single-Run Sensitivity Analysis

**Target section:** Methodology (Section 3.5, Evaluation Infrastructure), and Results (footnote to Table 3).

**Action:** Add a paragraph in Section 3.5 acknowledging the single-run limitation and estimating expected generation variance. Note that the 2-point AA session drift (40.3 → 41.7 → 42.3) suggests session variance of approximately ±1-1.5 points. Add a footnote to Table 3 noting that R3 (35.7) sits 2.7 points above threshold, that R2 (30.3) sits 2.7 points below threshold, and that given estimated session variance of ±1.5 points, the R2/R3 threshold crossing is robust but not definitive. The minimum sufficient set claim is supported contingently on single-run conditions; multi-run replication is identified as a robustness check for future work.

### P2-B: Session Variance in AA Reference Score

**Target section:** Results (Section 4, opening paragraph).

**Action:** Add a sentence explicitly noting the AA drift (40.3, 41.7, 42.3 across sessions), attributing it to panel calibration variance across sessions rather than output variance (the same puzzle text was re-evaluated), and providing the session variance estimate. Note that cross-session comparisons (Phase 1 vs. Phase 2 isolation score for AA) may be subject to this variance and interpret accordingly.

### P2-C: T6 Theoretical Status Clarification

**Target section:** Section 3.2 (Trait Taxonomy).

**Action:** Add a sentence clarifying that T6 (systematic thinking) is included as a domain-general competency control rather than a candidate creative trait in the Boden/Finke-Ward-Smith sense. Its below-baseline isolation score is then not a surprising empirical finding but an expected result that validates the taxonomy boundary: creative quality in this domain is not served by problem-solving competencies applied in isolation.

### P2-D: Single Answer Word Limitation

**Target section:** Section 5.4 (Implications for AI Creative Pipeline Design) or a new Limitations paragraph.

**Action:** Add a dedicated Limitations subsection at the end of Section 5 or as a brief Section 5.5. Include: (1) single answer word (TOWER) and game world (AoE2) as scope limitations; (2) acknowledgment that TOWER's visual-architectural semantic properties may interact with T3 (visual thinking) in ways that would not generalize to abstract or linguistic answer words; (3) identification of multi-answer-word replication as a near-term robustness check.

### P2-E: Creative Process Model Connection

**Target section:** Section 5.2 (Three-Trait Injection and Its Generalization) or the new Section 4.x on mechanism hypothesis.

**Action:** Add a 100-word paragraph connecting the T1+T4+T5 dependency chain to MEXICA's engagement/reflection dual-phase structure and to the four-stage Wallas model (T1 drives preparation/incubation toward an experiential target; T4+T5 constitute a compressed illumination/verification phase). Note this as a suggestive parallel rather than a claimed equivalence, and identify testing the parallel against narrative generation and story design tasks as a future direction.

---

## Revision Priorities

| Priority | Item | Effort | Risk if skipped |
|---|---|---|---|
| 1 | P1-B: Scope all domain-general claims | Low-medium (text revision throughout) | Rejection on resubmit |
| 2 | P1-A: AI panel validity disclosure | Low (one new subsection, ~200 words) | Rejection on resubmit |
| 3 | P1-C: Mechanism hypothesis subsection | Medium (~250 words + citations) | Weak revision response |
| 4 | P1-D: Taxonomy grounding (Boden + SPECS) | Medium (~200 words + 2 citations) | Weak revision response |
| 5 | P2-A/B: Variance acknowledgment | Low (footnotes + paragraph) | Credibility concern |
| 6 | P2-C: T6 clarification | Low (1-2 sentences) | Theoretical confusion |
| 7 | P2-D: Limitations subsection | Low (~150 words) | Reviewer trust |
| 8 | P2-E: Process model connection | Low (~100 words) | ICCC positioning |

---

## What Does NOT Need to Change

- The experimental design, conditions, and results are accepted as reported. No re-running of experiments is required.
- The three-tier structure (irreplaceable / load-bearing / quality-amplifying) is accepted by all reviewers.
- The minimum sufficient set (T1+T4+T5) finding is accepted as valid within the study's scope.
- The mechanism novelty finding (artist escapes acrostic family) is accepted as an important empirical result.
- The practical three-sentence injection (T1+T4+T5) is accepted as a directly useful contribution.
- The clarity and directness of the writing is praised by all reviewers; do not substantially restructure the prose.

---

## Estimated Revision Scope

The full revision requires approximately 800-1,000 words of new or substantially modified text distributed across:
- 1 new methodology subsection (~200 words, P1-A)
- 1 new discussion subsection (~300 words, P1-C)
- Distributed scope-scoping edits across abstract, introduction, discussion, and conclusion (~250 words, P1-B)
- Taxonomy paragraph expansion (~150 words, P1-D)
- Footnotes and acknowledgment paragraphs (~100 words, P2-A/B/C/D/E)

No section needs to be deleted or substantially restructured. The revision is additive and clarifying.
