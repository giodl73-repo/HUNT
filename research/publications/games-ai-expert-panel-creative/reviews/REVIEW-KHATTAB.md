# Peer Review: "AI-Simulated Expert Panels for Creative Evaluation: A Profile-Depth Ablation Study in Puzzle Hunt Design"

**Reviewer**: Omar Khattab (Stanford / Databricks; creator of DSPy)
**Venue**: ICCC 2026
**Submitted**: 2026-02-28

---

## Summary

This paper investigates whether AI-simulated expert review panels, constructed from persistent structured persona profiles, can produce evaluation quality that scales to creative domains where generic rubrics and bare prompting fall short. The authors conduct an eight-condition ablation on three puzzle hunt puzzles, varying the reviewer context from a bare role label through to full 80--120-line domain-specific profiles with named evaluative frameworks. The primary findings are: (1) a feedback quality gradient in which profile depth drives specificity and transferability of diagnostic output more than it drives score magnitude; (2) a calibration inversion in which rubric-only evaluation systematically misjudges elite-tier puzzles by penalizing intentional dimension trade-offs; and (3) a redundancy analysis identifying 3 of 11 principles as adding unique signal beyond well-designed profiles. The authors generalize to an extended 15-puzzle corpus and validate a seven-dimension weighted rubric achieving 24.5 percentage-point tier separation. Profile libraries, rubrics, and transcripts are released.

This is a readable, carefully structured empirical paper with a genuinely useful central finding. It also, from my perspective, represents a significant missed opportunity to connect its findings to the emerging science of LLM program optimization. What the authors have built is an LLM program whose primary tunable artifact is the profile — a context artifact that functions as a soft prompt with interpretable semantics. They have characterized, through careful ablation, what signal different parts of that artifact carry. What they have not done is ask whether that artifact could be optimized automatically, whether the optimization target they are implicitly using (tier separation, framework count, defect detection rate) is the right one, and whether the information-theoretic structure of the redundancy they have measured can be characterized precisely enough to generalize.

My review is organized around five substantive concerns that I believe are both criticisms of the current paper and directions that would substantially increase its contribution.

---

## Strengths

**S1. The ablation is well-designed and the conditions are meaningfully decomposed.** The factorial structure of the eight conditions — separating philosophy from lens, compact principles from full principles, profile alone from profile plus principles — is exactly the kind of causal isolation that makes ablation studies informative rather than merely descriptive. The finding that C4 (philosophy only) and C5 (lens only) achieve comparable score averages by structurally different means is a genuinely interesting result that has implications for how profile sections should be weighted during construction.

**S2. The defect detection result is the paper's most surprising and practically valuable finding.** C5's unique identification of the duplicate bracket label in Puzzle II — a binary correctness failure that no other condition detected — is not a score outcome; it is a mode-of-attention result. The fact that stripping biography and philosophy concentrates attention on visual-mechanical properties precisely enough to surface a specific construction error is a finding I would not have predicted, and it is the kind of result that should be explained theoretically rather than merely reported empirically. This deserves its own section.

**S3. The calibration inversion result has broad implications.** The finding that rubric-only evaluation inverts the tier hierarchy (80% pass rate for Standard Hunt puzzles versus 40% for MIT/Elite-tier puzzles) is a concrete, replicable failure mode of a widely used evaluation design pattern. This will matter to researchers in domains far outside puzzle hunt design. The paper correctly identifies the mechanism — equal-weight rubrics penalize intentional dimension trade-offs — and correctly attributes the fix to expert identity encoding calibration context. This is the paper's most generalizable finding.

**S4. The Riven Standard as a rubric dimension is a non-obvious contribution.** The operationalization of Miller's design principle as a scored dimension — and the empirical demonstration that adding it achieves a substantially better tier-separation gap than double-weighting the two proxy dimensions alone — is a genuine methodological contribution. The interpretation that Elegance and Reading Reward are proxies for a deeper criterion rather than independent dimensions is theoretically motivated and empirically supported.

**S5. The redundancy analysis is underappreciated in the paper's framing.** The finding that 8 of 11 principles are subsumed by well-designed profiles, and that 3 add unique signal, is precisely the kind of structured compression analysis that prompt optimization work needs more of. That the paper frames this only as a practitioner recommendation rather than as an information-theoretic result is, as I discuss below, a missed opportunity.

---

## Weaknesses and Questions

**W1. The profile design process is entirely manual, and the paper does not ask whether it could be automated.**

The profiles are the central artifact of this system — they are, in the framing I would use, the learned parameters of an LLM program whose inputs are puzzle artifacts and whose outputs are scored reviews. The paper characterizes what makes a profile effective (named frameworks in the lens section, calibration context in the biography section, three specific non-redundant principles) but treats profile construction as a craft activity performed by the authors. This is the right starting point for a first paper, but the finding immediately raises a question the paper does not pose: could the profiles be automatically optimized toward a defined quality objective?

The authors have, implicitly, exactly the components needed to frame this as an optimization problem. They have a quality metric (tier separation percentage points, which is tractable and differentiable in principle). They have variation in profile content across eight conditions that maps to measurable outcome differences. They have 15 labeled puzzles spanning three tiers. The DSPy framework, or any structured prompt optimization system, could in principle treat the profile as a text-valued parameter and search for profile variants that maximize tier separation on a held-out split. The paper does not attempt this, does not discuss it, and does not identify it as a limitation. In a venue focused on computational creativity, this omission is notable: the paper demonstrates that profile quality matters substantially, then stops before asking how to optimize for it.

A minimum treatment would be to acknowledge that profile design is a prompt engineering problem and to characterize what an optimization target for automated profile search would look like given the paper's own metrics. A stronger treatment would pilot a simple optimization loop on one profile using the paper's own tier-separation signal.

**W2. The rubric is hand-designed, and the paper does not ask whether the right dimensions can be learned from data.**

The six- and seven-dimension rubrics are constructed by the authors from practitioner literature (Snyder, Miller, Juul, Sicart). This is appropriate methodology, but it leaves open the question of whether these dimensions are the right dimensions — not normatively, but empirically, in the sense of explaining variance in tier assignments. The paper validates rubric configurations post-hoc by measuring tier separation, but it does not ask whether an inductively derived rubric (e.g., extracted from the review transcripts via topic modeling or learned from tier labels) would outperform the expert-constructed one.

The paper is in an unusual position here: it has 15 puzzles with tier labels and eight sets of review transcripts per puzzle. This is a small dataset, but it is rich enough to support exploratory analysis of whether the six rubric dimensions explain all the variance in tier assignments or whether the residual variance clusters around dimensions the rubric does not capture. Even a qualitative analysis of the residuals from the tier-separation regression would be informative. The authors note that the Riven Standard captures residual variance that Elegance and Reading Reward do not — which suggests the rubric is not yet closed. The paper should say explicitly whether it makes any claim about rubric completeness, and if not, how a practitioner would know when the rubric is complete enough.

**W3. The redundancy finding should be framed in information-theoretic terms, not just as a practitioner recommendation.**

Section 4.4 contains the paper's most interesting implicit information-theoretic result: 8 of 11 principles are redundant with well-designed profiles, and the 3 non-redundant ones address distinct evaluation modes (extraction-granular verification, solver-retrospective affect, and explicit test application versus reasoning-in-spirit). The paper presents this as a recommendation (add three principles, omit eight) but does not characterize the information structure precisely.

Several questions follow directly from the finding that the authors do not address:

- What is the overlap structure between the 11 principles and the profiles? Are the 8 redundant principles each subsumed by a single specific profile lens item, or are they collectively subsumed by the combination of multiple profile items? This matters because the former implies that redundancy is item-level and can be checked locally, while the latter implies that redundancy is emergent from the profile as a whole.
- The 3 non-redundant principles address qualitatively distinct evaluation modes. Is there a principled way to characterize what kinds of evaluation concerns escape profile encoding? The paper suggests that profiles are designer-centric and that "Blame the Player" adds a solver-retrospective perspective — this is a structural claim about the blind spots of designer-centric profiles that should be made explicitly and tested.
- The paper reports that C7 (profile plus all principles) achieves 14 frameworks but scores worst on Puzzle I. This suggests that adding redundant context does not merely waste tokens — it actively degrades calibration. This is a prompt interference result, not just an efficiency result. It deserves a sharper characterization: is the degradation traceable to specific redundant principles introducing conflicting evaluation criteria, or is it a more general context-length or attention-dilution effect?

In the DSPy framing: the "compilation" of a profile-plus-principles context into a minimal sufficient context is exactly the problem of finding a maximally informative, non-redundant prompt. The paper has done this empirically for one domain. The question is whether the principle (add what addresses blind spots, remove what is subsumed) generalizes to other domains, and whether the blind-spot characterization can be derived from the profile structure without running the full ablation.

**W4. The C8 recommendation is manual prompt compression, and the paper should frame it as such.**

The paper's practical recommendation — use "C8 format" profiles at approximately 75 lines rather than the full 80--120-line v2 profiles — is described as a profile design implication but is operationally a prompt compression decision. The paper recommends trimming biography to essential calibration context, preserving the full lens section, and appending three non-redundant principles. This is exactly the kind of token-efficiency / information-density trade-off that the prompt compression literature studies formally.

The paper should report the token counts of each condition explicitly (not just line counts, which are a proxy for tokens but not the right unit of analysis in this context), and should characterize the Pareto frontier of information content versus token cost across the eight conditions. The move from C6 (full profile) to the recommended C8 configuration should be described not just as "trimmed profiles" but as maximizing information density — the signal-per-token ratio — by removing biography text whose calibration content is carried redundantly by other profile sections or by the model's own prior.

The C5 condition (lens only) is particularly interesting from a compression perspective: it achieves the highest defect detection rate of any condition at presumably the lowest token cost. If the paper's goal is to characterize the optimal prompting strategy for creative evaluation, the token-efficiency of C5 relative to C6 for the specific task of defect detection should be quantified and framed as a task-conditional prompt selection finding.

**W5. Persona simulation fidelity is not evaluated, and this gap is more consequential than the paper acknowledges.**

The paper's most significant unresolved limitation is stated clearly in Section 5.4: the AI-simulated reviewers have not been calibrated against actual reviews by the named human practitioners on the same artifacts. The paper correctly identifies this as a gap, but I want to be more direct about its implications.

The paper's central empirical claim is that domain-specific profiles produce better evaluation than generic profiles or bare prompting. "Better" is measured by tier separation, framework count, and defect detection — all of which are structural properties of the review output, not properties relative to a human expert ground truth. This means the paper can claim that C6 is better than C0 in a well-defined internal sense, but it cannot claim that C6 approximates what an actual Thomas Snyder or Dana Young would say about these puzzles. The latter claim, which the paper's framing strongly implies, is not supported by the data.

This is not a minor limitation — it is the gap between "this LLM program produces structured, specific, internally coherent output" and "this LLM program produces output that agrees with human domain experts." These are empirically distinct properties, and the paper only establishes the former. The DSPy paradigm is relevant here: if ground-truth human reviews were available as a training signal, the profile could be optimized toward actual agreement with human experts rather than toward structural proxies for quality. The paper should be more explicit that its evaluation methodology validates structural quality, not human-expert agreement, and should discuss what the difference between these two validation modes means for deployment in high-stakes evaluation contexts.

A related concern: the profiles encode evaluative frameworks "attributed to named human experts" (Section 3.2), but the attributions are constructed from published writings and interviews, not from the practitioners' self-descriptions of their review criteria. The profile for Thomas Snyder, for example, encodes the load-bearing test as a named framework based on the authors' reading of his published work. There is no guarantee that Snyder would recognize this as an accurate characterization of his review process, or that he would apply the test in the way the profile specifies. The paper treats the profiles as grounded in documented practitioner perspectives but does not validate that the grounding is accurate. This is a significant gap for a system that claims to simulate specific named individuals.

---

## Minor Points

**MP1.** The paper reports line counts (50 lines for v1 profiles, 80--120 for v2) as the primary quantitative characterization of profile size. Token counts should be reported alongside line counts. In current LLM practice, the relevant cost unit is tokens, not lines, and lines-per-profile is a noisy proxy for token cost that varies substantially with line length.

**MP2.** The claim that six domain-emergent frameworks "did not appear in any input text provided to the reviewers" (Section 4.2) requires qualification. The profile encodes concepts such as the load-bearing test in operational language; the model generating the review is drawing on both the encoded profile content and its own training-data knowledge of Snyder's published work. The framework labels may be "emergent" in the sense that the reviewer generated the label text rather than copying it from the input, but they are not emergent in the sense of arising from novel reasoning rather than retrieval. The paper should be more careful about this distinction.

**MP3.** The experimental design treats three reviewers (Dan Katz, Thomas Snyder, Dana Young) as fixed across all eight conditions. This controls for reviewer selection effects within the ablation, but it limits the generalizability of the findings to the specific profile designs the authors chose for these three reviewers. The paper's claims about "what makes a profile effective" are derived from a three-reviewer ablation; whether they generalize to profiles for other practitioners is not tested.

**MP4.** Experiments 2, 3, and 4 are partially incomplete at submission time, and the paper explicitly flags this. For a published record, the authors should clarify which quantitative claims rest on complete versus in-progress data. The tier-separation results from Experiment 4 appear complete; the cross-hunt generalizability results from Experiment 2 appear partial. The distinction should be made explicit in the results text, not only in the experimental design section.

---

## Scores (1--4 scale, where 4 = outstanding)

| Dimension | Score | Rationale |
|-----------|-------|-----------|
| **Novelty** | 3 | The profile-depth-as-variable framing is new. The creative evaluation domain is well-chosen. The calibration inversion and defect detection findings are genuinely surprising. Docked one point for not connecting to the LLM program optimization literature, where closely related questions about prompt component contribution have been studied formally. |
| **Soundness** | 2 | The empirical methodology is careful within its scope, but the central claim (domain-specific profiles produce expert-quality evaluation) rests on structural proxies rather than human-expert validation. The absence of any ground-truth calibration against actual expert reviews limits how far the soundness claim extends. The results are internally consistent; the connection to the stated goal is not fully established. |
| **Significance** | 3 | The calibration inversion finding and the defect detection result are both practically significant and likely to matter to researchers beyond the puzzle hunt domain. The Riven Standard as a rubric dimension is a transferable methodological contribution. The profile library release is a concrete community contribution. Docked one point for leaving the optimization question unaddressed. |
| **Presentation** | 4 | The paper is clearly written, well-organized, and genuinely enjoyable to read. The named evaluative frameworks are introduced with enough context to be understood without domain knowledge. The conditions table and ablation table are well-designed. The limitations section is unusually frank. |

---

## Recommendation

**Weak Accept** (suitable for publication with revisions).

The paper makes real contributions: the calibration inversion result, the defect detection finding, the Riven Standard rubric validation, and the profile redundancy analysis are all useful and non-obvious. The writing is good and the ablation design is careful. I would vote to accept this paper.

However, I want to register clearly that the paper is missing a connection that would substantially increase its contribution. The profiles are LLM program parameters. The ablation has characterized which components carry what signal. The tier-separation metric is an optimization target. The pipeline is a system that could, in principle, be compiled rather than hand-crafted. None of this machinery is invoked, and the paper is correspondingly weaker than it could be.

The revisions I would ask for are:

1. Report token counts for each condition alongside line counts.
2. Add a brief section framing profile design as a prompt optimization problem, identifying what an automated optimization target would look like given the paper's own metrics, and discussing why this direction is or is not tractable given the dataset size.
3. Sharpen the characterization of the redundancy finding: what structural property of a principle predicts whether it will be subsumed by a well-designed profile? The designer-centric versus solver-retrospective distinction for "Blame the Player" is a start; generalize it.
4. Clarify the distinction between validating structural quality (which the paper does) and validating human-expert agreement (which it does not). The framing in the abstract and introduction oversells the connection to human expert review; the limitations section is appropriately cautious but too far from the claims it is qualifying.
5. Address the profile attribution concern: the profiles are constructed from the authors' reading of practitioners' published work, not from the practitioners' self-descriptions. This is a reasonable methodology, but it should be flagged as a methodological choice with acknowledged limits, not treated as grounding that makes the profiles straightforwardly accurate simulations of the named individuals.

---

*Review prepared for ICCC 2026. Reviewer conflict of interest: none declared.*
