# Synthesis — Round 1
**Panel average:** 2.98/4
**Gate:** CONDITIONAL PASS (requires major revision on two blocking issues; minor revision on remaining)

---

## Scores

| Reviewer | Novelty | Rigor | Clarity | Significance | Overall |
|----------|---------|-------|---------|--------------|---------|
| Jordanous | 3/4 | 2/4 | 3/4 | 3/4 | 2.8/4 |
| Togelius | 3/4 | 2/4 | 4/4 | 3/4 | 3.0/4 |
| Gervás | 3/4 | 2/4 | 3/4 | 3/4 | 2.8/4 |
| Cook | 3/4 | 2/4 | 4/4 | 3/4 | 3.0/4 |
| Riedl | 3/4 | 2/4 | 4/4 | 4/4 | 3.3/4 |
| **Average** | **3.0/4** | **2.0/4** | **3.6/4** | **3.2/4** | **2.98/4** |

**Consensus summary:** The panel converges on a clear and consistent assessment. Clarity and significance are genuine strengths — the paper is well-written, the paradigm is well-motivated, and the findings have real practical value for practitioners and for the CC/PCG research community. Rigor is the consistent weakness across all five reviewers: the N=2 limitation is not adequately honoured in the inferential language of the body text, the mechanism of the central finding is unestablished, the panel score comparison is confounded by a rubric change, and the "domain semantic gravity" claim lacks operationalization and theoretical grounding. Novelty is solid but requires two corrections to be sustainable: the contribution must be situated more carefully relative to expressive range analysis (Togelius), and it must engage with what "creative" means in a CC venue (Jordanous).

---

## P1 — Blocking Issues

### P1.1: Insufficient CC theoretical grounding for ICCC submission
**Source:** Jordanous (primary), Gervás (secondary)
**Summary:** The paper uses "AI creative generation" and "creative" as unexamined qualifiers throughout without engaging with what creativity means in this context. ICCC requires that claims about creative systems engage with CC criteria (novelty, value, intentionality, process — cf. the SPECS framework). The paper does not ask: is divergence evidence of creativity, or mere stochasticity? Is specification-driven convergence evidence of constraint, or of absence of creativity? Is domain semantic gravity a creative phenomenon or a statistical one?
**Fix:** Add a 200–300 word section (Section 2.5 or Discussion subsection) explicitly addressing the paper's relationship to CC evaluation criteria. The authors have two defensible options: (a) argue that same-input divergence is a proxy for a specific CC property (e.g., it measures the generator's surprise capacity or its ability to produce non-redundant outputs relative to a prior run), anchoring the contribution in SPECS or a related framework; or (b) acknowledge that the study is about pipeline reliability characterisation rather than creativity evaluation per se, and argue that characterising the reliability structure of a creative pipeline is a prerequisite for evaluating the pipeline as a creative system. Option (b) is compatible with the paper's actual content and would make the ICCC submission defensible without requiring the authors to claim more than the data supports.
**Target:** `sections/02-related-work.tex` (new subsection on CC evaluation) and `sections/05-discussion.tex` (brief treatment of the creativity framing question)

### P1.2: Inferential language not honoured throughout body text after N=2 acknowledgment
**Source:** Jordanous (primary), Cook (primary), Togelius (secondary), Gervás (secondary)
**Summary:** Section 5.4 acknowledges the N=2 limitation and calls for N≥10 replication, but the body of the paper consistently uses distributional and categorical language that cannot be supported by two data points: "the panel is measuring a quality dimension that is invariant" (Discussion 5.3), "domain semantic gravity is a tendency" (Discussion 5.2), "the pipeline generates structurally sound puzzles in both runs" (Discussion 5.3), and the practical recommendations in the Conclusion ("run once and trust for explicitly specified decisions"). Cook (P1.1) makes the strongest version of this argument: the practical recommendations rest on a reliability characterisation that has not been demonstrated, and should be qualified as provisional findings from a case study rather than validated rules.
**Fix:** Audit every sentence in Sections 4 and 5 that uses categorical or distributional language (invariant, always, stable, tendency, reliable) and add hedges reflecting the case-study status of the evidence. The recommendations in Section 6 (Conclusion) should be reframed as "in this pipeline and domain, the evidence suggests..." rather than validated deployment rules. This is an editing task, not a conceptual revision, but it must be thorough — selective hedging in the limitations section while leaving the body text uncorrected is not acceptable.
**Target:** `sections/04-results.tex`, `sections/05-discussion.tex`, `sections/06-conclusion.tex`

---

## P2 — Important Issues

### P2.1: Mechanism of "specification drives convergence" is unestablished
**Source:** Riedl (P1.1), Gervás (P1.1), Cook (implicit in P1.2)
**Summary:** The paper observes that specified decisions are stable and attributes this to the specification functioning as a constraint on the design space. But at least three mechanisms could produce the same pattern: (a) specification-as-hard-filter (outputs are checked against the spec and rejected if non-compliant — scale is stable because a 6-puzzle output is simply discarded); (b) specification-as-distributional-prior (the spec conditioning narrows the probability distribution at generation time); (c) specification-as-attention-anchor (mentioned items are salient and consequently stable; unmentioned items are underspecified and consequently variable). The practical recommendation "specify more to stabilize" has different design implications depending on which mechanism is operating.
**Fix:** Add a paragraph to Discussion Section 5.1 distinguishing these three mechanisms and identifying which is most consistent with the evidence (or acknowledging that the distinction cannot be made from N=2 and is a target for the N=10 replication). This does not require additional experiments — it requires a theoretical argument about which mechanism is consistent with the data. The finding that narrator voice (stylistic, not formally checkable) is stable alongside scale (formally checkable) suggests that mechanism (a) alone is insufficient and mechanism (b) or (c) is operative.
**Target:** `sections/05-discussion.tex`

### P2.2: Panel score comparison is confounded by rubric change
**Source:** Jordanous (P2.2), Riedl (P2.3), Cook (P2.2), comparison-all-levels.md (explicit documentation)
**Summary:** Hunt 1 used a 6-dimension, 30-point, uniform-weight rubric; Hunt 2 used a 7-dimension, 45-point rubric with double-weighted Elegance and Reading Reward. The +1.2 uplift in Elegance and Reading Reward — presented as a genuine mechanism-level improvement — is partially confounded by (a) the double-weighting of those dimensions in Hunt 2's rubric and (b) the use of different reviewer panels (domain-matched for Hunt 1; fixed Katz/Snyder/Dana panel for Hunt 2). If Katz/Snyder/Dana are systematically more generous on Elegance and Reading Reward than the Hunt 1 domain-matched panels, the uplift is a reviewer calibration effect, not a quality effect.
**Fix:** Add a paragraph in the Methodology (Section 3.4) explicitly acknowledging that the rubric difference is a confound for the dimension-level comparison and that the dimension-level findings (Elegance +1.2, Reading Reward +1.2) should be interpreted as upper bounds on the genuine quality difference. The aggregate normalized score comparison (83% vs. 86%) is more robust because it divides each run's score by its own maximum, but even this comparison is affected by the reviewer-panel change. The Conclusion's treatment of the Elegance/Reading Reward uplift as "consistent with a genuine mechanism-level improvement" should be hedged to "consistent with, but not conclusively attributable to, a genuine mechanism-level improvement."
**Target:** `sections/03-methodology.tex`, `sections/05-discussion.tex`, `sections/00-abstract.tex` (one word change: "systematic +1.2-point uplift" → "observed +1.2-point uplift, partially confounded by rubric weighting change")

### P2.3: Domain semantic gravity lacks operationalization and null model
**Source:** Togelius (P2.3), Jordanous (P2.1), Gervás (P2.1)
**Summary:** The "domain semantic gravity" claim rests on the observation that 3 of 5 feeder slots yield domain-adjacent word pairs. The paper does not specify (a) what threshold of semantic proximity counts as "adjacent," (b) what proportion of randomly sampled 5-word sets from the AoE2 vocabulary would exhibit this level of adjacency by chance (the null model), or (c) whether the phenomenon is distinguishable from the known behaviour of LLM sampling from a semantically structured training distribution (Jordanous). Without a null model, 3/5 domain-adjacent pairs could be unremarkable given the structured AoE2 domain vocabulary.
**Fix:** Add an operationalization of "domain-adjacent" to Section 3.3 (Level 3 methodology) — at minimum, specify the criterion used to judge adjacency (co-occurrence in the same AoE2 semantic category, as defined by the world data taxonomy). Then, in Section 4.3 (Level 3 results), add a brief null model argument: if both runs sample independently and uniformly from the AoE2 vocabulary, what is the expected number of domain-adjacent pairs by chance? Even an informal estimate based on the vocabulary structure (e.g., "the AoE2 vocabulary has approximately 150 gameplay-relevant terms distributed across 8 semantic categories; if both runs sample uniformly, the expected number of same-category slot pairs is approximately X") would make the 3/5 observation more interpretable.
**Target:** `sections/03-methodology.tex`, `sections/04-results.tex`

### P2.4: Level 2 KL divergence is the specified metric but is not reported
**Source:** Togelius (P2.2), Cook (P2.2)
**Summary:** Table 1 specifies KL divergence (threshold < 0.5 nats) as the primary stability metric for Level 2 pool comparison. The results section (Section 4.2) reports Jaccard on selected types (0.11) but does not report the KL divergence value. The specified metric should be computed and reported; the authors may also choose to report Jaccard but the KL divergence is required by the methodology specification.
**Fix:** Compute KL divergence between the two 15-candidate type distributions and report it in Section 4.2. Given the distribution data already present in comparison-all-levels.md, this is a calculation task. The KL divergence between the two distributions (identification-heavy vs. chain/path-heavy) is expected to be well above 0.5 nats, which would reinforce the VARIABLE classification for Level 2.
**Target:** `sections/04-results.tex`

### P2.5: Propagation of variability across causally ordered pipeline stages is not addressed
**Source:** Cook (P1.2)
**Summary:** The paper treats the five comparison levels as parallel measurements on the same artifact. But in a multi-stage pipeline, Level 3 (answer words) causally constrains Level 4 (mechanisms) and Level 5 (micro-design). Hunt 2's uniform 5-letter answer words were not independent of its vowel-count meta mechanism — the mechanism required uniform-length words and the word selection was shaped by that requirement. The paper's implication that each level's variability is independently controllable through specification is not quite right.
**Fix:** Add a paragraph to Discussion Section 5.1 (or a new subsection) noting that same-input divergence in multi-stage pipelines exhibits causal propagation: variable decisions at upstream stages (Level 3 assignment) induce correlated variable decisions at downstream stages (Levels 4 and 5). This does not undermine the specification-drives-convergence finding, but it refines the practical recommendation: specifying upstream decisions (e.g., answer words) has a multiplicative effect on downstream stability, because it constrains the design space available to later pipeline stages. This is actually a stronger argument for complete specification than the paper currently makes.
**Target:** `sections/05-discussion.tex`

---

## P3 — Suggestions (non-blocking)

**P3.1: The pool organisation principle finding deserves more prominence.** Both Cook and Togelius note that the observation in Section 4.2 — that the two runs organised their 15-candidate pools on different principles (domain-by-slot vs. mechanism-by-type) — is one of the most interesting findings in the paper. It suggests that the pipeline's generative strategy at the pool stage is itself variable in a principled way. Consider elevating this observation to a named phenomenon or at least a dedicated paragraph in the discussion.

**P3.2: Contribution 2 in the Introduction (empirical data) should be reframed.** Gervás notes that "we provide empirical data" is a description of the study rather than a claim about the world. Replace with a claim: "We establish that complete lexical divergence (Jaccard 0.00) and complete pass/fail convergence can coexist in a multi-stage AI creative pipeline" or similar.

**P3.3: The future work section could be strengthened by specifying the N=10 replication design more concretely.** The paper calls for N=10 runs across three domains; a brief protocol sketch (same SCOPE.md, same toolkit, automated Level 4 coding using the mechanism taxonomy from Table 2, automated Level 3 analysis using the AoE2 vocabulary semantic categories) would make the replication invitation more concrete and increase the likelihood that the community takes it up.

**P3.4: The paper's title front-loads "Structured Divergence" but could be more specific.** "Specification Drives Convergence: A Same-Input Divergence Study of AI Puzzle Hunt Generation" would better reflect the central finding and key term. Current title is accurate but generic.

**P3.5: Riedl's point about evaluative-language stability as future work should be clarified or dropped from the Conclusion.** If it is worth raising, it needs a sentence of motivation. If the argument is too thin, the sentence should be cut.

---

## Recommendation

**Decision: Major Revision Required**

The paper has real strengths — a novel and exportable paradigm, clean and practically useful central finding, excellent clarity — but the two P1 issues (CC grounding for ICCC venue, and inferential language not honoured after N=2 acknowledgment) are both blocking. Neither requires new experiments; both are fixable through targeted revision. The P2 issues are individually manageable and several (P2.3, P2.4, P2.5) can be addressed through computation and editing rather than conceptual overhaul.

The panel recommends resubmission after revision. The authors should provide a response-to-reviewers document addressing each P1 and P2 item and indicating the specific textual changes made in response.

Estimated revision scope: Section 2 (new 200-300 word subsection), Section 3 (methodology qualification on rubric confound and operationalization of domain adjacency), Section 4 (KL divergence computation, hedge language), Section 5 (mechanism clarification, rubric confound acknowledgment, causal propagation paragraph), Section 6 (hedge all recommendations as provisional). Total estimated: 400–600 words of new or revised text, plus hedge-language edits throughout.
