# Peer Review: ICCC 2026 Submission
## "Expert Panel Simulation for Creative Work Evaluation: Profile Depth and Feedback Quality in Puzzle Hunt Design"

**Reviewer:** Michael Bernstein (Stanford University)
**Area:** Human-AI Collaboration, Crowdsourcing, Human Computation
**Conflicts:** None declared

---

## Summary

This paper investigates whether AI-simulated expert panels, instantiated through rich domain-specific reviewer profiles, can produce higher-quality evaluative feedback on creative artifacts than bare prompting or rubric-only conditions. The testbed is puzzle hunt design — a domain with verifiable correct answers and contested quality judgments — making it a reasonable empirical choice for this kind of study. The authors compare eight conditions in a factorial ablation, varying what reviewer context is present (nothing, rubric only, philosophy only, lens only, full profile, and combinations thereof). The primary contributions claimed are: (1) evidence that profile depth produces a feedback quality gradient more practically significant than its score gradient; (2) identification of a "calibration inversion" in which rubric-only evaluation disadvantages elite-tier puzzles relative to accessible ones; and (3) a generalized rubric dimension (the Riven Standard) that achieves substantially better tier separation than equal-weight baselines.

The paper is carefully written and the ablation design is more sophisticated than most work in this space. The calibration inversion finding is genuinely novel and practically important. The framework emergence result — that full profiles surface named evaluative concepts absent from any input — is the kind of qualitative finding that often gets buried in appendices but is arguably the paper's most important contribution.

That said, the paper's central claim — that this system provides "expert-quality" evaluation — is not established by the evidence presented. The study demonstrates internal coherence, domain specificity, and calibration relative to the authors' own tier assignments. It does not demonstrate correspondence to human expert judgment. This is a significant gap, acknowledged in the limitations section but not adequately foregrounded in the abstract, introduction, and conclusion where claims about evaluation quality are strongest. I recommend acceptance conditional on significant revision to the framing of claims, with the calibration inversion finding elevated as the primary contribution.

---

## Strengths

**1. The calibration inversion is a novel, important, and generalizable finding.**

The result that rubric-only evaluation (C1) inverts the expected quality tier hierarchy — passing 80% of Standard Hunt puzzles while failing 60% of MIT/Elite-tier puzzles — is not obvious, not previously documented, and has implications well beyond puzzle hunts. The mechanism is traced clearly: elite puzzles intentionally trade Clarity and Solvability for Elegance and Reading Reward, and an equal-weight rubric treats that trade-off as a defect. The generalization in Section 5.2 ("any creative domain where quality trades off across multiple dimensions") is warranted and could anchor a standalone contribution. This finding should be the paper's headline, not its third result.

**2. The eight-condition ablation design is rigorous within its constraints.**

Decomposing the profile into philosophy-only (C4) and lens-only (C5) conditions is smart experimental design. The finding that C5 uniquely detects a verifiable binary defect (the duplicate bracket label in Puzzle II) — while C6 does not — is an important result that cuts against the authors' preferred condition being the universal optimum. The paper deserves credit for reporting this honestly rather than normalizing it away. The recommendation of a two-pass protocol (C6 for quality calibration, C5 for defect detection) follows naturally and is operationally useful.

**3. The defect detection analysis is honest and practically grounded.**

Sections 4.3 and 4.4 report that C5 and C7 find real, verifiable construction errors (duplicate diagram label, wrong blank count) that C6 misses. The paper does not claim the full profile condition is superior on all dimensions; it characterizes a trade-off between calibration accuracy and mechanical error detection. This is the kind of nuanced finding that makes the ablation worthwhile.

**4. The redundancy analysis (Section 4.4) is practically useful.**

Identifying which of the eleven tested principles add unique evaluative signal (three do: Verify the Last Mile, Blame the Player, Snyder's Computer Test) and which are subsumed by well-designed profiles (eight do not) is a contribution that practitioners can immediately use. The characterization of why Snyder's Computer Test as an injected principle triggers application rather than reasoning-in-spirit is a subtle but important observation about how LLMs use context.

**5. Puzzle hunt as a testbed is well-motivated.**

The combination of verifiable correct answers and contested quality judgments makes this domain unusually tractable for empirical creative evaluation research. The authors make this case clearly and specifically. The comparison to superficially similar domains (trivia, narrative fiction) is useful framing for readers outside the community.

---

## Weaknesses and Questions

**W1 [Major]: The paper cannot claim the AI panels are "reliable" — it can only claim they are self-consistent.**

This is the central methodological problem, and it is understated throughout the paper. The abstract states the system investigates whether AI panels "can provide evaluation quality that scales without sacrificing the specificity that makes expert feedback actionable." The introduction presents the system as producing feedback that is "more specific, more transferable, and more actionable." The conclusion recommends practitioners "use C6 profiles with the seven-dimension weighted rubric as the default evaluation condition." All of these claims implicitly assume the AI panel output tracks what human experts would say.

But the authors have not shown this. The tier assignments used as ground truth were made by the authors themselves, not by independent human experts. The profiles are constructed from published writings, not validated against reviews the named practitioners would actually give on these specific puzzles. The pass thresholds were "calibrated against the authors' prior experience with hunt quality," not against a held-out set of human expert ratings.

What the paper has actually demonstrated is: (a) that richer profiles produce output with more domain-specific vocabulary; (b) that full profiles produce tier-consistent verdicts according to the authors' own tier assignments; and (c) that rubric-only evaluation inverts that hierarchy. These are meaningful findings. But they establish internal coherence, not external validity. A reviewer who is internally coherent but systematically wrong is a liability, not an asset.

The fix is not to run the human calibration study (that is future work, acknowledged and appropriate). The fix is to revise every claim about "evaluation quality," "expert-quality feedback," and "reliable verdicts" to accurately reflect what was measured. The system produces evaluations that are self-consistent, domain-specific in vocabulary, and tier-concordant with the authors' assessments. That is a real contribution — but it is a different claim than is currently being made.

**W2 [Major]: The profile system raises a simulation fidelity question the paper does not adequately address.**

The profiles are constructed from published writings, interviews, and documented design work by named practitioners (Thomas Snyder, Dan Katz, Dana Young, Emily Short, etc.). This is a principled approach. But the paper does not distinguish between two very different things a profile might be doing: (1) accurately simulating a practitioner's evaluative perspective, or (2) retrieving and reorganizing documented statements that approximate that perspective.

This distinction matters for the paper's core claim. If the profiles are functioning as accurate simulations, then the named frameworks they surface (a-ha economy, load-bearing test) are genuinely the evaluative concepts those practitioners would apply. If the profiles are retrieving and recombining documented vocabulary, then the frameworks are a sophisticated interpolation of things the practitioners have said in public — which is still valuable, but differently so.

The paper actually gestures at this distinction in Section 4.2 when comparing C2 (principle names retrieved from injected context) with C6 (frameworks emergent from profile-encoded philosophy): "C2 names principle-labels and C6 names analytical tools generated from evaluative experience." This is insightful. But the same critique applies, at a higher level of abstraction, to C6 itself: the profiles are themselves derived from documented sources, and the "emergent" frameworks they produce are ultimately a function of what those sources contain. The paper should engage this more directly and honestly.

Specifically: what is the difference between a profile that "simulates" Thomas Snyder's evaluative perspective and one that "synthesizes" his documented views into a review checklist? For practitioners considering using this system, this distinction matters: they need to know whether the reviews they receive represent idiosyncratic expert judgment or well-organized documentation synthesis.

**W3 [Moderate]: The practitioner workflow is proposed but not studied.**

The paper concludes with specific practitioner recommendations: use C6 as default, supplement with C5 for defect detection, add three non-redundant principles for maximum coverage. This is a useful contribution. But the paper does not study whether practitioners would actually use the system this way, whether the feedback changes their behavior, or whether the evaluations they receive improve the creative work they produce.

In the crowd systems literature, this is a persistent gap: tools that produce better outputs in controlled experiments often fail to change practitioner behavior in deployment because they do not fit existing workflows, the feedback arrives at the wrong stage of development, or practitioners do not trust automated evaluation regardless of its quality. The paper notes in the abstract that expert feedback is "most needed during early-stage iteration" — but does not study whether the system is actually used in that context or whether it produces different outcomes than the existing alternatives (self-review, generalist peer review, delayed expert review).

This limitation should be stated explicitly in the discussion. The system is proposed as a tool, but its utility as a tool — as opposed to its technical performance in an ablation — is not evaluated.

**W4 [Moderate]: The Games Magazine / hunt paradigm distinction is interesting — how generalizable is it?**

Section 5.2 discusses the calibration inversion in terms of a distinction between puzzle paradigms: Games Magazine puzzles optimize for consistent accessibility, while MIT/Elite puzzles intentionally sacrifice accessibility for concentrated excellence. This is a well-observed distinction, and the rubric design implications (double-weighting Elegance and Reading Reward, adding the Riven Standard) follow from it.

But the paper proposes the calibration inversion result generalizes to "any creative domain where quality trades off across multiple dimensions." This is a strong generalization from a single domain. What is the analog in, say, short fiction, where literary magazine stories may sacrifice plot clarity for stylistic ambition in the same way elite hunt puzzles sacrifice Clarity for Elegance? What about music production, where lo-fi aesthetic choices might score poorly on a production-quality rubric while being artistically superior? The claim is plausible, but the paper makes it as an assertion rather than as a hypothesis requiring investigation. The limitations section should flag this explicitly.

**W5 [Minor]: The experimental status is inconsistently signaled.**

Table 3 in the methodology section describes Experiments 2 and 3 as "in progress at the time of submission." Experiment 4 (rubric validation) operates on existing score data from Experiment 3, which is itself pending. Yet Section 5.2 (calibration inversion) and Section 5.3 (Riven Standard) both report the 15-puzzle corpus and 24.5pp tier separation results as findings, and the conclusion treats them as established contributions. If the 15-puzzle corpus data is available and the rubric validation results are real, the methodology section should not describe these experiments as pending. If they are partially complete, the paper should be clearer about which results are from complete data and which are from partial analysis.

The result reported in the abstract — "+0.57/30 average score delta" — comes from what is described as Experiment 1, which is "the Condition~1-to-Condition~2 arm of Experiment~0, for which complete data is already available." This is fine, but the framing in the abstract ("three evaluated puzzles") makes the study sound smaller than it apparently is across all experiments.

**Q1: On the ground-truth problem.**

Section 5.4 (limitations) states that "tier assignments are themselves judgments made by the authors based on puzzle provenance and community consensus, not independent oracle verdicts." This is the right acknowledgment. But it raises a question the paper does not fully answer: if the tier assignments are the authors' judgments, and the AI panel's tier-consistent verdicts are measured against those assignments, what exactly does "tier separation" measure? It may be measuring agreement between the AI system and the authors' own evaluative framework — which would be unsurprising if the profiles were constructed by the same authors who made the tier assignments.

The paper should clarify the independence of the tier assignments from the profile construction. Were tier assignments made before profiles were developed? By the same people? This matters for interpreting the tier separation results.

**Q2: On the three-reviewer panel size.**

Why three reviewers? The paper justifies this as "computationally tractable" and notes that the three-reviewer design provides "lens diversity (structural rigor, narrative/accessibility, domain specificity)." But the ablation uses the same three reviewers (Dan Katz, Thomas Snyder, Dana Young) across all eight conditions. What happens with different reviewer compositions? Does panel composition significantly affect results, or is the profile depth the dominant variable? The paper mentions inter-reviewer variance (high spread reported as range rather than average) but does not systematically analyze whether different panel compositions produce different verdicts on the same puzzles.

**Q3: On profile authorship and its implications.**

The profiles are described as "structured markdown documents" constructed from "publicly available writings, interviews, and published work by the named practitioners." Who constructed them, using what process? Were they reviewed for accuracy by the named practitioners or by recognized members of the hunt community? The paper mentions that the profiles encode "authentic expert perspectives" — but authenticity cannot be assessed without either practitioner validation or community review. For a paper presenting profiles as a reusable artifact for practitioners to use in high-stakes evaluation contexts, the profile construction process deserves more transparency.

---

## Scores

| Criterion | Score (1–4) | Notes |
|-----------|-------------|-------|
| Novelty | 3 | Calibration inversion and Riven Standard findings are new. The basic profile-depth-matters claim extends prior work incrementally. |
| Soundness | 2 | The ablation design is rigorous, but the core claim (AI panels provide expert-quality evaluation) is not established by the evidence. The ground-truth and simulation-fidelity problems are real and underacknowledged in the body of the paper. |
| Significance | 3 | The calibration inversion result is significant and applicable across creative domains. The practitioner recommendations are specific and actionable. The full-profile condition as a design artifact (29 profiles + rubric + replication package) is a genuine contribution to the community. |
| Presentation | 3 | Well-written throughout. The experimental design is clearly described. The limitations section is honest, but its content needs to migrate further into the abstract, introduction, and conclusion to avoid misleading readers who read only those sections. |

**Overall recommendation: Accept with Major Revision**

---

## Recommendation

Accept with major revision. The paper makes three genuine contributions — the calibration inversion finding, the ablation design methodology, and the released profile library — and presents them in a well-written, carefully organized paper. The recommendation to revise is not about the work itself but about the claims made for it.

The revisions I consider essential:

1. **Revise the abstract, introduction, and conclusion** to accurately characterize what was measured. "Internally coherent and domain-specific feedback that is tier-concordant with the authors' assessments" is a real contribution but is different from "expert-quality evaluation." The system produces the former; the latter requires human calibration that has not been done.

2. **Foreground the calibration inversion result** as the primary contribution. This finding is novel, generalizable, and has immediate practical implications. It is currently the second finding in the discussion section. It should be the headline.

3. **Add a discussion of the simulation fidelity question** in Section 5.4 (Limitations). The distinction between "simulating a practitioner's perspective" and "synthesizing their documented views" is real and matters for practitioners using the system. The paper's own comparison of C2 (retrieved principle-labels) versus C6 (emergent frameworks) establishes the conceptual vocabulary for this discussion; it should be applied reflexively to the profiles themselves.

4. **Clarify the experimental status** of the 15-puzzle corpus results throughout the paper, and clarify the independence of tier assignments from profile construction.

5. **Add a paragraph to the discussion** on the practitioner workflow gap: what would a deployment study look like, what are the risks of practitioners over-trusting AI panel verdicts, and what safeguards does the system provide against false confidence?

The calibration inversion finding alone is worth the price of admission. The paper would be significantly stronger if it were the paper's primary claim rather than its supporting act.

---

*Review submitted for ICCC 2026. Reviewer identity disclosed per author request.*
