# Peer Review — FDG 2026

**Paper**: "A Structured Generative Pipeline for Puzzle Hunt Creation: Empirical Characterization Across Five Scenarios"
**Reviewer**: Mark Nelson (Adobe Research / Falmouth University)
**Date**: 2026-02-28

---

## Summary

This paper presents an 11-stage pipeline for AI-assisted puzzle hunt creation, instantiated across five thematic scenarios and 52 puzzles, and reports a 90.4% first-pass editorial pass rate. The central claims are that (1) stage structure with explicit review gates improves multi-artifact generation quality over unstructured approaches, (2) failures concentrate at Stage 8 rather than at gate stages, confirming gate efficacy, and (3) the pipeline transfers across domains without per-scenario modification. The paper situates itself at the intersection of PCG, AI pipeline orchestration, and puzzle design, though the system itself is most accurately characterized as an AI co-authorship workflow with structured editorial review rather than a procedural content generator in the sense the PCG literature has established.

---

## Strengths

**1. The world lock mechanism is a genuine and underexplored contribution.**
The canonical world document committed as read-only before any generative stage is a clean, practical solution to coherence drift in multi-session, multi-artifact generation. The authors note that no post-lock canon violations were detected across nine scenarios, compared to "multiple coherence errors per scenario" in pilot runs without the lock — a before/after comparison that is the most convincing empirical result in the paper. This mechanism is underrepresented in the AI-assisted content creation literature and deserves the attention it receives. Practitioners in adjacent domains (tabletop campaign generation, interactive fiction pipelines) could adopt this pattern immediately. The analog to the "single source of truth" pattern in software engineering is obvious in retrospect, but its formalization as a pipeline mechanism for AI generation is novel.

**2. The cost-staging rationale for gate placement is the paper's strongest conceptual contribution.**
The argument that creative pipeline failures are not equally expensive at all stages, and that gate placement should be calibrated to failure cost rather than to failure probability, is well-articulated and empirically supported. The explicit cost model — brief only at Stage 3, authoring effort at Stage 7, integration and delivery at Stage 8 — gives practitioners a principled basis for deciding where to invest evaluation effort. This is a practical contribution the PCG community has not formalized in this way, even though the underlying logic is implicit in generate-and-test paradigms.

**3. The failure analysis is specific and honest about its own process gaps.**
The categorization of the five failures into mechanism failure, answer coordination failure, construction error, and delivery gap — and the direct acknowledgment that the Ironfall answer-coordination failures were caught at Stage 8 only because Stage 7 review did not cross-reference META.md — reflects well on the paper's intellectual honesty. The authors do not claim the gates are perfect; they identify the specific process gap and propose a concrete fix. This is the kind of failure analysis that makes a systems paper useful to practitioners.

**4. Cross-domain transferability is demonstrated, not merely claimed.**
The invariance of the skill library across five scenarios — observable from commit history showing no per-scenario modifications to skill files — is a stronger form of evidence than author assertions typically provide. The point that commit messages are domain-agnostic is a nice way to make the claim concrete without requiring the reviewer to inspect every file.

---

## Weaknesses

**1. The "generative" framing is imprecise and risks misleading the PCG community.**
The paper repeatedly describes this as a "generative pipeline" and situates itself in relation to PCG research (Shaker et al., Smith and Whitehead, Summerville et al.), but the system is not a PCG system in any technically precise sense. PCG, as the literature defines it, involves algorithmic content generation through search, constraint satisfaction, grammar-based expansion, or machine learning — processes that generate content computationally with defined formal properties. What this paper describes is an AI co-authorship workflow: a human operator invokes LLM prompts at each of eleven stages, reviews structured output, and decides whether to proceed. The review gates are editorial checkpoints, not constraint filters in a generative system. The paper's own autonomy coding acknowledges this: Stage 1 is "AI-primary, human confirms," Stage 11 is "human-primary," and even the fully autonomous stages are LLM invocations against structured briefs, not algorithmic generation in the PCG sense. The paper should either (a) adopt more precise terminology — "AI-assisted editorial pipeline," "AI co-authorship framework," or "structured LLM orchestration pipeline" — or (b) explicitly argue for an extended definition of PCG that encompasses prompted LLM workflows, engaging with the definitional debate rather than eliding it. The current framing will read as category confusion to PCG researchers who have invested significant effort in distinguishing between generative and editorial approaches to content creation.

**2. The 90.4% pass rate is self-referential and cannot be interpreted as an external quality signal.**
The rubric used by the AI expert panel, the scoring thresholds (33/45 at Stage 3, 22/30 at Stage 6), the tier assignments, and the reviewer profiles were all constructed by the authors. The 90.4% first-pass pass rate measures the fraction of author-generated puzzles that meet author-constructed quality standards as evaluated by an author-configured AI panel. This is not a measure of whether the puzzles are good by any external criterion — human solver enjoyment, community standards, fairness to solvers, or even conventional puzzle craftsmanship as assessed by experienced human constructors. The Limitations section acknowledges that "pass rates throughout this paper are assessed by AI expert panel review, not by human play-testing," which is an appropriate acknowledgment, but the paper's framing does not consistently apply this caveat. Section 3 describes the gate thresholds as "empirically grounded" and "calibrated in the rubric validation experiment of [companion paper]," but the companion paper is cited, not reproduced, and the reader cannot assess what "calibration" against "three quality tiers of published puzzle hunt puzzles" actually demonstrates. Until the AI panel verdicts are validated against human assessments at an acceptable correlation threshold, the 90.4% figure supports only the claim that the pipeline produces output that the authors' own system considers acceptable — which is not the claim the paper makes.

**3. Stage timing from commit timestamps conflates elapsed time with human work time in ways the paper does not fully account for.**
The paper reports that Scenario 1 completed Stages 1–10 in 3 hours 17 minutes "as measured by git commit timestamps." The Limitations section correctly notes that "commit timestamps capture wall-clock intervals between artifact commits rather than active human-minutes per stage," but the paper's body text does not apply this caveat consistently. Section 3's framing — "the first fully-executed scenario completed in 3h 17m from scope to ship-ready artifacts" — appears in the Introduction, Abstract, and Contributions as an unqualified throughput claim. A commit timestamp for Stage 6 at 11:06 following Stage 3 commits at 10:05–10:17 does not establish that a human was actively working on Stage 6 for those 49 minutes, or that Stage 5 (meta design, coded "embedded in Stage 6" with no separate measurement) was addressed during that window at all. The parallelism of Scenarios 3–5 compounds the problem: if multiple sessions were running simultaneously, the two-day scope makes the "two active days" framing ambiguous as a production time estimate. The paper should either report only what the timestamps can support — elapsed wall-clock time for one scenario in one session — or instrument future runs to capture actual human-active minutes per stage. The distinction matters considerably if practitioners are evaluating whether to adopt the pipeline based on time-to-ship claims.

**4. The PCG related work is cited but not engaged as a framework for analysis.**
Section 2 cites Shaker et al. and Smith and Whitehead but does not apply their frameworks analytically to the system under description. The paper notes that "our pipeline is constraint-based in the same spirit" as PCG-via-search, but this is asserted without formal argument. A PCG-trained reader would immediately observe that: (a) the system has no search process — stages execute forward without backtracking; (b) constraints are enforced editorially (human review) rather than computationally; (c) the world lock is a hard constraint imposed by process, not by algorithmic enforcement; and (d) the "gate conditions" are human decision points, not automated fitness thresholds. The paper's own system would be more precisely situated as an instance of human-in-the-loop orchestration — closer to mixed-initiative design (Yannakakis et al., Liapis et al.) than to PCG-via-search — but this placement is not made explicit. Engaging seriously with where the system sits relative to the mixed-initiative literature, and what it adds to that tradition, would strengthen the paper's claims considerably and situate the world lock and gate-staging contributions more precisely.

**5. The improvement arc across scenarios is a significant confound that the paper acknowledges but does not adequately address.**
Section 4.5 notes that the pipeline was in "active improvement" during the study, with the Age of Empires scenario (first run, 83% pass) and Dead Reckoning (last run, 100% verified) bookending the arc. Bug fixes from Scenario 1 propagated forward to later scenarios. Reviewer profiles grew from 12 to 29 during the two-day run. Skills were added reactively. The paper acknowledges that separating pipeline refinement effects from pool-size effects from accumulated experience is a "confound the current study cannot fully disentangle." This is an honest and accurate statement, but it has a more serious implication than the paper treats it as having: the 90.4% aggregate pass rate is not a stable measure of a static system's quality. It is a blended measure of a system that changed materially between its first and last runs. Reporting a single pass rate across an actively evolving system overstates measurement precision. The paper should either analyze early and late scenarios separately to bracket the improvement effect, or report the aggregate as an order-of-magnitude estimate rather than a precise figure ("approximately 90%") that implies more stability than the data supports.

---

## Scores (1–4 scale, 4 = strongest)

| Dimension | Score | Rationale |
|---|---|---|
| Novelty | 3 | The world lock mechanism and cost-staged gate design are genuine contributions. The pipeline architecture itself is novel as a specified artifact, though adjacent to existing multi-agent and mixed-initiative work. |
| Soundness | 2 | The self-referential evaluation methodology and the conflation of elapsed time with work time are substantive issues. The main empirical claims hold within their stated assumptions, but those assumptions carry more weight than the paper acknowledges. |
| Significance | 3 | If the AI panel quality proxy is validated in the companion paper, the pipeline specification has real practical value for the puzzle hunt community and for adjacent multi-artifact creative system practitioners. |
| Presentation | 3 | Writing is clear and well-organized. The failure analysis table and autonomy gradient are useful. The gate-cost argument is well-paced. The PCG framing introduces terminology confusion that the prose does not resolve. |

---

## Recommendation

**Weak Accept** — with revision requested on two points before camera-ready.

The paper makes a genuine contribution with the world lock mechanism and the cost-staged gate design. The failure analysis is specific and honest. The cross-domain transferability claim is supported by observable commit evidence rather than author assertion. These are the marks of a paper worth accepting.

However, the PCG framing is imprecise in a way that will read as category confusion to the FDG audience, and the 90.4% pass rate headline is given more evidentiary weight than the self-referential evaluation methodology can support. Revision should address both. Specifically:

1. Adopt precise terminology for the system type: "AI-assisted editorial pipeline" or "structured LLM orchestration workflow" rather than "generative pipeline" in contexts where PCG connotations attach. If the authors wish to make a claim about extending the PCG concept to encompass prompted LLM workflows, make that argument explicitly rather than importing PCG framing by citation.

2. Apply the timing caveat from the Limitations section consistently throughout the body — including in the Abstract and Introduction. The 3h17m figure should be presented as elapsed wall-clock time for one session, not as a human-effort estimate.

These revisions are scoped and do not require new experiments. The core contribution is not in dispute.
