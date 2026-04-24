# Review: Structured Divergence in AI Creative Generation: A Same-Input Study of Puzzle Hunt Pipeline Outputs
**Reviewer:** Michael Cook | **Round:** 1

## Scores (1-4)
- Novelty: 3/4
- Rigor: 2/4
- Clarity: 4/4
- Significance: 3/4
- **Overall: 3.0/4**

## Overall Assessment
This is a practical and well-executed case study of pipeline reliability with a genuinely useful central finding. The paper is clearly written and the authors are refreshingly honest about the N=2 limitation and the incomplete Hunt 2. My primary concern is that N=2 is not merely a limitation to be acknowledged and moved past — it is a structural problem for most of the paper's inferential claims, and the paper's practical recommendations rest on a reliability characterisation that has not been demonstrated. The micro-design comparison at Level 5 is the strongest section because it is richest in detail; the structural comparison at Level 1 is the weakest because two observations cannot establish which architectural choices are "stable" versus "variable" at a pipeline level.

## Strengths
1. The same-input divergence paradigm is a direct response to a real practitioner problem. I have encountered exactly the "run once and trust vs. run many times and select" dilemma in my own creative system work, and the paper's framing of it — how do you know which decisions are reliable? — is precisely the right question.
2. The five-level analysis gives the study a depth that a single-level comparison would lack. The fact that Slot 1 mechanism families partially converge (both use indexed-N letter extraction) despite diverging at every other mechanism slot is the kind of granular finding that would be missed by an aggregate diversity metric.
3. The pool organisation finding (Level 2) is one of the paper's most interesting observations and is underappreciated: two runs from the same content library produce pools organised on fundamentally different principles (domain-by-slot vs. mechanism-by-type). This is not a surface-level randomness effect but a genuine difference in generative strategy. This deserves more discussion than it receives.
4. The distinction between exploitation (running once and trusting stable decisions) and exploration (running multiple times and selecting from variable decisions) maps directly onto a practical deployment architecture. The paper's recommendation to use specification completeness as the lever for controlling reliability is immediately actionable.
5. The honest acknowledgment that clean-room isolation was procedural rather than architectural is the kind of methodological transparency that is rare and valuable.

## Major Concerns (P1 — blocking)

**P1.1: The practical recommendations in Section 5.1 and the Conclusion are stated with more confidence than the evidence warrants given N=2.**
The paper recommends: "run once and trust for explicitly specified decisions; run multiple times and select for unspecified ones." This recommendation presupposes that the stability pattern observed across two runs is representative of the pipeline's general behaviour. But from two runs, we cannot distinguish:
- A pipeline where specified decisions are *always* stable (the paper's claim)
- A pipeline where specified decisions are *usually* stable, with occasional failures that two runs would not reveal
- A pipeline where the two runs happened to converge on specified decisions because of correlated training data patterns that could equally produce correlated divergence on unspecified decisions in other domains

The recommendation should be qualified: "in this pipeline and domain, the evidence suggests that specified decisions may be more reliable than unspecified ones, but this should be verified with N≥5 runs before deployment decisions are based on it." The current phrasing reads as a validated rule, not a hypothesis supported by preliminary evidence.

**P1.2: The paper does not discuss what happens when the pipeline generates outputs that are variable in ways that matter for downstream stages.**
The central practical problem is not that answer words vary — it is that answer words constrain the meta mechanism, which constrains the overall hunt architecture. The paper notes this briefly (Discussion Section 5.2, "practitioners who need specific answer word properties...must either specify explicitly or run multiple times"), but does not develop the architectural implication: in a multi-stage creative pipeline where stage N's output is stage N+1's input, variability at any stage *propagates* to all downstream stages. The paper treats each level's variability as independent, but levels 3, 4, and 5 are causally coupled: Hunt 2's uniform 5-letter answer words directly determined its vowel-count meta mechanism (Level 4), which in turn shaped the extraction logic for each feeder puzzle (micro-design, Level 5). A single "variable" decision at Level 3 induced correlated variable decisions at Levels 4 and 5 — this is a structural property of multi-stage pipelines that the same-input divergence paradigm should address. The paper's current framing treats the levels as parallel measurements on the same artifact rather than as causally ordered stages in a generative process.

## Minor Issues (P2)

**P2.1: The two-run comparison leaves open the question of which run is better.** The paper's panel score analysis shows Hunt 2 scoring higher on Elegance and Reading Reward (+1.2). If the first-run performance is systematically worse — because the pipeline "learns" (or more accurately, because the pipeline's prompts were revised between runs to address documented weaknesses) — then the study is not comparing two independent samples from the same pipeline but two runs from different versions of the pipeline. The paper acknowledges that "Hunt 2's puzzle designs produce higher structural elegance and stronger aha-moment quality *by construction*" (emphasis added, Section 5.3), which implies that Hunt 2's generation was shaped by knowledge of Hunt 1's weaknesses. If this is the case, it violates the clean-room assumption more deeply than the procedural isolation caveat acknowledges.

**P2.2: The Level 2 pool comparison would benefit from a direct quantitative test.** The paper reports Jaccard on selected types (0.11) but not the KL divergence between the full 15-candidate type distributions, which was the metric specified in Table 1 (threshold: KL < 0.5 nats). Reporting the specified metric rather than substituting an alternative strengthens the analysis.

**P2.3: The "partial stability" classification for domain-adjacent word pairs (Level 3) is a useful observation but the threshold for "adjacency" is not operationalized.** The paper identifies three domain-adjacent pairs based on the authors' judgment of semantic proximity. This is reasonable for a case study but would not be reproducible at scale. Even a brief description of an operationalizable criterion (e.g., "both words appear in the same semantic category in the AoE2 wiki taxonomy") would improve the analysis.

## Recommendation
Minor Revision
