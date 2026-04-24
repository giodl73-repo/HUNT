# Review: Structured Divergence in AI Creative Generation: A Same-Input Study of Puzzle Hunt Pipeline Outputs
**Reviewer:** Julian Togelius | **Round:** 1

## Scores (1-4)
- Novelty: 3/4
- Rigor: 2/4
- Clarity: 4/4
- Significance: 3/4
- **Overall: 3.0/4**

## Overall Assessment
The paper is clearly written, methodologically inventive, and the specification-drives-convergence finding is a genuine empirical contribution with real implications for PCG deployment. The five-level comparison framework is exactly the right kind of structured analysis for a multi-stage generator and is immediately applicable to other systems. My main concern is that the relationship to the PCG diversity literature — particularly expressive range analysis and quality-diversity methods — is treated too briefly and the claimed contrast with that literature is not fully worked out. N=2 is a real problem that the paper cannot reason its way out of, and the threat model for the paradigm (what could produce the observed pattern for reasons other than specification) is not explored.

## Strengths
1. The experimental paradigm is novel in a clean, exportable sense: fix the input, characterise natural output variation across runs, compare at multiple levels of abstraction. This is a generalizable method that PCG researchers can replicate immediately on any multi-stage generator.
2. The five-level comparison framework (structural, pool, assignment, mechanism, micro-design) is well-motivated and the metrics at each level are sensible choices. The decision to set the Jaccard stability threshold at 0.4 rather than a conventional 0.5 is defensible given the domain properties, though more explicit calibration argument would help.
3. Clarity is excellent. The writing is precise and the results tables are clean. The comparison data in the supplementary file (comparison-all-levels.md) is unusually thorough for a short paper — the micro-design comparison table for P1 equivalents is more detailed than many full-length papers provide.
4. The finding that pool organisation principle diverges (domain-by-slot vs. mechanism-by-type) while pool size is stable is a genuinely novel observation about how LLM-based generators impose structure at the pool level. This deserves more prominence.
5. The practical recommendations are specific and useful: "every under-specified decision is a variable decision" is the kind of rule-of-thumb practitioners can actually apply.

## Major Concerns (P1 — blocking)

**P1.1: The contrast with expressive range analysis is claimed but not demonstrated.**
The related work section states: "The QD and expressive range literatures ask: how much diversity can this generator produce? We ask: which components are diverse and which are not, and why?" This is a real and meaningful distinction. However, the paper does not show that expressive range analysis *cannot* answer the authors' question, only that prior work has not asked it this way. An expressive range analysis run over N=100 independent same-input executions, plotting the five-level metrics as behavioral descriptors, would directly characterize the distribution of divergence at each level. The authors should either (a) explain why expressive range analysis is architecturally insufficient for multi-stage creative pipelines (not just that it hasn't been applied this way), or (b) acknowledge that same-input divergence with N≥10 is a specific instance of expressive range analysis applied to a fixed input, and position the contribution accordingly. The claim to novelty relative to the QD/ERA tradition needs sharper grounding.

**P1.2: No exploration of confounds that could produce the specification-drives-convergence pattern for reasons other than what the paper claims.**
The central finding is that specified decisions are stable and unspecified decisions are variable. But there is an alternative explanation: specified decisions are stable because they are *easy* (scale is five puzzles — the model cannot produce six from a scope document that says five), while unspecified decisions are variable because they are genuinely *hard* (many equally valid architectures for round structure). The paper conflates "the specification caused convergence" with "the specification is the only reason for convergence." A system that simply could not violate the specified constraints (because they are formal, checkable properties) would produce the same pattern without any interesting generative mechanism. The discussion should distinguish between convergence due to specification-as-constraint (the model is checked/filtered against the spec) and convergence due to specification-as-prior (the spec shifts the model's probability distribution toward a narrower region of outputs). This distinction has direct implications for the actionable recommendation: if convergence is constraint-based, adding more spec items produces a filter; if it is prior-based, adding more spec items produces a distributional shift — these have different implications for how practitioners should structure their briefs.

## Minor Issues (P2)

**P2.1: The failure modes of the paradigm are not discussed.** What would a "bad" application of same-input divergence look like? If the two runs happened to produce very similar hunts by chance (which N=2 cannot rule out), the analysis would incorrectly classify many decisions as stable. If the clean-room isolation were imperfect (and the paper acknowledges it was procedural rather than architectural), convergence on unspecified decisions could reflect Hunt 1 contamination rather than design-space constraint. The paradigm's validity conditions and failure modes deserve a paragraph in the methodology or limitations section.

**P2.2: The Level 2 pool comparison is underspecified relative to other levels.** The KL divergence threshold of 0.5 nats is stated but not computed in the results section — the text describes the distributions qualitatively and reports Jaccard on selected types, but never reports the actual KL divergence value. If the paper sets a threshold and uses it as a stability criterion, the reported metric should include the numerical value against that threshold.

**P2.3: The "domain semantic gravity" terminology is evocative but not operationalized.** How strong does a semantic proximity need to be to count as "gravity"? Three of five slots is the empirical claim, but the paper does not specify a null model — what proportion of randomly sampled five-word sets from the AoE2 vocabulary would exhibit this level of domain adjacency by chance? Without a null model, 3/5 domain-adjacent pairs could be unremarkable.

**P2.4: Generalisation across domains is a key open question that the paper gestures at but does not explore.** The claim that same-input divergence is a "generalizable experimental paradigm" is supported only by a single domain (AoE2). The call for N=10 runs across three domains in the future work section is appropriate, but the paper should be more cautious about presenting the paradigm as validated rather than proposed.

## Recommendation
Minor Revision
