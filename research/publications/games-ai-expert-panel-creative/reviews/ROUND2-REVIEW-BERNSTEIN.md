# Peer Review: ICCC 2026 Submission — Round 2 Recheck
## "Expert Panel Simulation for Creative Work Evaluation: Profile Depth and Feedback Quality in Puzzle Hunt Design"

**Reviewer:** Michael Bernstein (Stanford University)
**Area:** Human-AI Collaboration, Crowdsourcing, Human Computation
**Round:** 2 (Major Revision Recheck)
**Prior Recommendation:** Accept with Major Revision

---

## Overview

I gave Major Revision in Round 1. My four principal concerns were: (W1) the self-consistency versus reliability gap — the paper claimed more than its evidence warranted about evaluation quality; (W2) simulation fidelity versus documentation synthesis — the profiles' epistemic status was underexamined; (W3) no practitioner workflow study; and (W4/lead) the calibration inversion was underemphasized as the primary contribution. I also flagged experimental status inconsistencies (W5) and the ground-truth independence problem (Q1).

The revised sections I have read are the abstract, introduction, discussion, and conclusion. I have not re-read the methodology or results sections; my assessment of the revisions is limited to what is visible in these four sections, which are precisely the sections where my Round 1 concerns were concentrated.

The overall assessment: this revision is substantive and largely successful. The two major concerns are resolved or substantially resolved. The moderate concern about practitioner workflow is partially resolved. The calibration inversion is now properly emphasized. The paper is ready for acceptance subject to the remaining outstanding items noted below.

---

## Concern-by-Concern Assessment

### W1: Self-Consistency vs. Reliability Gap

**Round 1 verdict:** Major. The paper claimed "expert-quality evaluation" and "reliable verdicts" without having validated against human expert judgment. Every claim of evaluation quality implied external validity the evidence did not support.

**Round 2 assessment: Resolved.**

The revision addresses this comprehensively and at the right level of structural specificity. The abstract now closes with an explicit scope caveat: "All results are characterized as valid within the authors' constructed evaluation framework; a human calibration study remains future work." This is the correct framing — it appears in the abstract, which is where the misleading claims were most consequential in Round 1.

The introduction reinforces this in Contribution 2: "Note: all quantitative findings are valid within the authors' constructed evaluation framework; the rubric, tier assignments, and thresholds were constructed by the research team and have not been externally calibrated against a held-out human expert standard." Embedding this caveat directly inside the contributions list — rather than deferring it to Section 5.4 — is exactly the structural change I asked for. A reader who stops after the introduction now cannot walk away believing the system has been validated against human judges.

The conclusion opens with the same frame ("All quantitative results reported here are valid within the authors' constructed evaluation framework") before reporting the three headline findings. This sequencing — scope statement before claims — is a meaningful improvement over Round 1, where the headline claims appeared first and the limitations appeared only in a dedicated section that many readers skip.

The one remaining thread of concern is the phrase "reliable verdicts" in the profile design implications section (Section 5.4, third point): "we recommend C6 as the default because it produces richer diagnostic output... even where C3's binary accuracy is marginally better." The word "accuracy" here is used in an internal-consistency sense (agreement with author-assigned tiers), not in an external-validity sense, and this is fine — but the paper should be vigilant that the word "accurate" not appear elsewhere in contexts where external validity is implied. A single pass checking for residual uses of "accuracy," "reliable," and "valid" in the methodology and results sections (which I have not re-read) would be prudent.

Overall, W1 is resolved. The framing change is correctly located and consistently applied in the sections I reviewed.

---

### W2: Simulation Fidelity vs. Documentation Synthesis

**Round 1 verdict:** Major. The paper did not distinguish between "accurately simulating a practitioner's perspective" and "retrieving and reorganizing documented statements that approximate that perspective." This distinction matters for practitioners who will use the system.

**Round 2 assessment: Partially Resolved.**

The revision adds an explicit limitation in Section 5.5 (Limitations, fifth point): "The 29 reviewer profiles were constructed by the research team based on their reading of each practitioner's published work, interviews, and documented design positions — not based on self-descriptions by the named individuals. The profiles synthesize attributed positions rather than directly representing any individual's views, and this limits any claim about simulation fidelity."

This is a meaningful addition and directly responsive to my concern. It names the right distinction and places it appropriately in the limitations section.

However, my Round 1 request was for more than a limitations acknowledgment — I asked for a discussion of what the distinction means for practitioners who receive these reviews. The difference between "simulated idiosyncratic judgment" and "well-organized documentation synthesis" has practical implications: practitioners need to know whether they are receiving reviews that represent how Thomas Snyder would personally respond to their puzzle today, or reviews that represent how Snyder's documented vocabulary applies to the puzzle. These are different things. A profile that synthesizes documentation could be systematically biased toward the concerns Snyder has written about publicly and systematically blind to the concerns he holds but has not published. A practitioner who over-trusts the simulation will act on that bias without knowing it.

The revision acknowledges this limitation exists but does not develop its implications. Section 5.5 states that the synthesis limitation "limits any claim about simulation fidelity" — which is correct as far as it goes — but does not connect this to how practitioners should weight the feedback they receive. The practitioner recommendation in the conclusion (use C6 as default) does not mention this caveat, so a reader who reads only the conclusion for the practical takeaway will not see it.

The fix is modest: one sentence in the conclusion's practitioner recommendations paragraph noting that profile-generated feedback represents a synthesis of documented practitioner positions, not real-time practitioner judgment, and should be treated accordingly. This is not a new thought for the authors — the language is already in Section 5.5 — it just needs to be surfaced at the point of practical recommendation.

W2 is partially resolved: the limitation is acknowledged in the right place, but its implications for practice are not yet carried forward to the conclusion's recommendations.

---

### W3: No Practitioner Workflow Study

**Round 1 verdict:** Moderate. The paper recommends a specific deployment configuration without studying whether practitioners use it, whether it changes their behavior, or whether it improves the creative work they produce.

**Round 2 assessment: Partially Resolved.**

The revision adds a paragraph in Section 5.5 (Limitations) — visible in the discussion section I reviewed — that explicitly flags the practitioner calibration study as future work: "A calibration study — in which the same puzzles are evaluated both by the AI panel and by the named human experts or their recognized proxies — is necessary before the system's verdicts can be treated as proxies for human expert review." The discussion also acknowledges the gap in the conclusion's future work bullets ("Human calibration. The panel's verdicts have not been compared against assessments by the named human practitioners on the same artifacts").

This is better than Round 1, where the practitioner workflow gap was not explicitly named. However, the specific concern I raised was not just about calibration against human experts — it was about the crowd systems deployment gap: tools that perform well in ablations often fail in deployment because they do not fit workflows, arrive at the wrong stage of iteration, or are not trusted by practitioners regardless of performance metrics. The revision addresses the calibration dimension of this concern but not the workflow and trust dimensions.

The conclusion's practitioner recommendations section is detailed and specific (use C6, supplement with C5, add three principles, avoid C1) — more detailed than most tool papers. But it presents these recommendations as if they are established best practices, when they are established only as optimal configurations within a controlled ablation. A practitioner reader might reasonably expect that "use C6 as default" is backed by deployment experience; it is backed by ablation performance. The distinction should be named.

I am not requesting a deployment study — that would exceed the scope of a revision. I am requesting one sentence in the practitioner recommendations acknowledging that these are ablation-derived recommendations that have not been validated in iterative creative pipelines. This is a small addition that prevents practitioners from over-trusting the recommendations.

W3 is partially resolved: the calibration dimension is acknowledged, the workflow and trust dimensions are not yet surfaced at the point of practical recommendation.

---

### W4: Calibration Inversion Underemphasized

**Round 1 verdict:** Major. The calibration inversion was the paper's most important finding and its practical headline. It was positioned as the second result in the discussion section and not foregrounded in the abstract or introduction as the lead contribution.

**Round 2 assessment: Resolved.**

This is the most successful change in the revision, and it is executed correctly throughout all four sections I reviewed.

The abstract now opens with a direct statement of the finding: "a rubric applied without expert identity context systematically inverts the quality hierarchy, producing higher scores for accessible work than for ambitious work." The mechanism is stated in the abstract ("elite puzzles deliberately sacrifice Clarity and Solvability for Elegance and Reading Reward, and an equal-weight rubric treats that trade-off as a defect"), not deferred to the discussion. The practical implication is explicit.

The introduction lists the calibration inversion as Contribution 1, with a full mechanistic explanation and an explicit practitioner warning: "This finding is a concrete warning for practitioners — rubric-only evaluation appears objective but is miscalibrated for ambitious creative work." This is precisely the framing I asked for: the finding as a warning, not as a capability claim.

The discussion gives the calibration inversion its own subsection (5.2, "The Calibration Inversion: Why Rubric-Only Is Dangerous") with its own dedicated header — more prominent than the Round 1 treatment. The section ends with the strongest claim the paper makes: "The rubric-only condition is, empirically, the most dangerous condition in the ablation: it appears objective, it produces structured output, and it misleads." This is well-phrased.

The conclusion leads with the calibration inversion as the first of three headline findings, and the practitioner recommendations explicitly name C1 avoidance as the terminal recommendation: "Avoid C1 (rubric only) for any creative work with dimensional trade-offs by design; it will invert your quality hierarchy and systematically disadvantage your most ambitious artifacts."

W4 is resolved. The calibration inversion is now the paper's headline finding, consistently foregrounded in abstract, introduction, discussion, and conclusion. My Round 1 request to make this the lead contribution has been fully honored.

---

### W5: Experimental Status Inconsistencies

**Round 1 verdict:** Minor. The methodology section described Experiments 2 and 3 as "in progress at the time of submission" while the abstract and discussion reported results from those experiments as established findings.

**Round 2 assessment: Not Assessed.**

This concerned the methodology section, which I have not re-read in this round. I note that the abstract and conclusion sections consistently report 15-puzzle corpus results as established findings, which is appropriate if the experiments are now complete. If the methodology section still contains "in progress" language for completed experiments, that would be a minor copyedit catch the authors should make. I flag this without scoring it, as I cannot assess it from the sections reviewed.

---

### Q1: Ground-Truth Independence

**Round 1 verdict:** Question. Were tier assignments made before profiles were developed, by the same people? This matters for interpreting tier separation results.

**Round 2 assessment: Addressed.**

The limitations section explicitly acknowledges: "The paper treats the extended-corpus tier assignments (MIT/Elite, Standard Hunt, Games Magazine) as ground truth against which panel verdicts can be assessed, but these assignments are themselves judgments made by the authors based on puzzle provenance and community consensus, not independent oracle verdicts." This is the correct acknowledgment. Whether tier assignments were made before or after profile development is not explicitly stated, but the paper's characterization of the results as "valid within the authors' constructed evaluation framework" appropriately scopes the claim such that independence of the tier assignments from profile construction is less critical than it would be under a stronger external validity claim.

---

## Updated Scores

| Criterion | Round 1 Score | Round 2 Score | Notes |
|-----------|---------------|---------------|-------|
| Novelty | 3 | 3 | Unchanged. Calibration inversion and Riven Standard remain the novel contributions; the profile-depth claim remains incremental relative to prior work. |
| Soundness | 2 | 3 | The claim scope is now accurately matched to the evidence. The paper no longer asserts external validity it cannot establish. The internal consistency of the methodology is well-supported. The remaining gap (simulation fidelity implications not carried to practitioner recommendations) is minor. |
| Significance | 3 | 3 | Unchanged. The calibration inversion result is the significant practical contribution; its prominence is now proportionate to its importance. |
| Presentation | 3 | 3 | Improved in the revised sections — the structural changes (caveat in abstract, inversion as Contribution 1, conclusion opening with scope statement) are meaningful. No change to overall score because I have not re-read the full paper. |

**Updated overall recommendation: Accept with Minor Revision**

---

## Recommendation

The paper is ready for acceptance pending two small additions:

1. **One sentence in the conclusion's practitioner recommendations paragraph** noting that the W2 simulation fidelity caveat applies to the feedback practitioners receive: profile-generated reviews represent a synthesis of documented practitioner positions, not real-time practitioner judgment, and should be weighted accordingly. The language is already in Section 5.5; it needs to be present where practitioners will read it.

2. **One sentence in the conclusion's practitioner recommendations paragraph** noting that the C6/C5/three-principles recommendation stack derives from ablation performance, not from deployment validation, and that practitioners in iterative creative pipelines should treat these as starting configurations rather than established workflow standards.

Neither of these requires rethinking the paper's claims or structure. Both are sentences, not paragraphs. The core revisions — reframing all claims within the authors' constructed evaluation framework, foregrounding the calibration inversion as the primary contribution, adding the simulation fidelity limitation in the body — are well executed and correctly located.

The calibration inversion finding is now where it belongs: the paper's headline. That change alone substantially improves the paper's utility for practitioners and for researchers in adjacent creative evaluation domains who will encounter this work.

---

## Summary Table

| Concern | Round 1 Severity | Round 2 Status | Notes |
|---------|-----------------|----------------|-------|
| W1: Self-consistency vs. reliability gap | Major | Resolved | Scope caveat in abstract, intro (Contribution 2), and conclusion opening. |
| W2: Simulation fidelity vs. documentation synthesis | Major | Partially Resolved | Limitation acknowledged in Section 5.5; implications not carried to practitioner recommendations. |
| W3: No practitioner workflow study | Moderate | Partially Resolved | Calibration gap acknowledged; workflow/trust gap not surfaced at point of recommendation. |
| W4: Calibration inversion underemphasized | Major | Resolved | Now Contribution 1, Section 5.2 headline, and terminal recommendation in conclusion. |
| W5: Experimental status inconsistency | Minor | Not Assessed | Methodology section not re-read; abstract/conclusion consistent. |
| Q1: Ground-truth independence | Question | Addressed | Tier assignment limitations explicitly scoped in Section 5.5. |

---

*Review submitted for ICCC 2026, Round 2. Reviewer identity disclosed per author request.*
