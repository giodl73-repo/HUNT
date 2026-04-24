# Peer Review — ICCC 2026

**Paper title:** AI-Simulated Expert Panels for Creative Work Evaluation: Profile Depth as a Quality Variable in Puzzle Hunt Design Assessment

**Reviewer:** Saleema Amershi (Microsoft Research)
**Area expertise:** Interactive machine learning, human-in-the-loop AI, responsible AI deployment

**Scores (1–4 scale, where 4 = strongest):**
- Novelty: 3
- Soundness: 2
- Significance: 3
- Presentation: 3

**Recommendation:** Revise and Resubmit

---

## Summary

This paper investigates whether AI-simulated expert reviewer panels, instantiated through rich domain-specific profiles, produce higher-quality evaluation of creative work than generic prompting or rubric-only conditions. The testbed is puzzle hunt design, chosen for its unusual combination of binary answer-correctness (a useful ground truth anchor) and contested multi-dimensional quality judgments. The primary contribution is an eight-condition ablation study across 15 puzzles, comparing reviewer contexts ranging from a bare role label (C0) through a complete profile plus injected design principles (C7). The main findings are: (1) a feedback quality gradient — richer profiles produce more specific, transferable, and actionable diagnostic output, even though score increases are modest; (2) a calibration inversion — rubric-only evaluation (C1) systematically disadvantages the highest-quality work by failing to recalibrate dimension weights for puzzles that deliberately sacrifice accessibility for elegance; and (3) a Riven Standard discriminator effect — adding a seventh dimension that asks whether a puzzle's mechanic is inseparable from its domain achieves substantially better tier separation (24.5pp vs. 10.9pp). The paper recommends C8 profiles with the seven-dimension weighted rubric as the default practitioner configuration.

The work is ambitious, carefully constructed, and makes a genuine contribution to AI-mediated creative evaluation. Several concerns, however, limit the claims that can currently be supported, and one finding — the calibration inversion — carries practical safety implications that the paper does not adequately surface for practitioners.

---

## Strengths

**1. The core research question is well-motivated and correctly framed.**
The insight that persona quality, not persona presence, is the lever that matters is a meaningful advance over the binary persona/no-persona treatments dominant in the prior literature (Zhang et al., Park et al., Dang et al.). The decision to decompose profile components — philosophy only, lens only, full profile, full profile plus principles — is exactly the right experimental design for isolating what is actually doing the work. This is a contribution even if the results were less clear than they are.

**2. The calibration inversion finding is the paper's most important result.**
The observation that C1 (rubric only) produces a pass rate of 40% for MIT/Elite-tier puzzles versus 80% for Standard Hunt puzzles — inverting the expected tier hierarchy — is striking, concrete, and has clear practical implications. The mechanistic explanation is credible: equal-weight rubrics penalize designs that deliberately trade accessibility for elegance, and the rubric lacks the calibration context to distinguish intended trade-offs from design failures. This is the kind of negative finding that practitioners need to know.

**3. The defect detection results are genuinely useful.**
The finding that C5 (lens only) uniquely detects the duplicate label error in Puzzle II, while C7 (full profile plus principles) detects the blank-count error in Puzzle I, points toward a two-pass evaluation protocol with distinct roles for different conditions. The recommendation to run C6 for quality calibration and C5 for mechanical verification is actionable and grounded in the data.

**4. The redundancy analysis is unusually transparent.**
Identifying which of the eleven design principles add unique signal versus which are subsumed by well-designed profiles is exactly the kind of ablation that practitioners need to configure systems efficiently. The finding that only three principles (Verify the Last Mile, Blame the Player, Snyder's Computer Test) are non-redundant is specific and saves practitioners from context overhead that doesn't pay.

**5. The choice of testbed is well-justified.**
Puzzle hunt design is a genuinely good creative evaluation testbed for the stated reasons — verifiable answers anchor the ground truth while quality is contested and multi-dimensional, and the expert community is documented enough to ground profiles in authentic perspectives rather than invented ones. The argument is made carefully.

---

## Weaknesses and Questions

### W1. The practitioner recommendation lacks actionability evidence

The conclusion's core recommendation — "use C6 profiles with the seven-dimension weighted rubric as the default evaluation condition" — is not evaluated against the dimension that matters most for practitioners: can they actually apply it correctly?

The paper shows that the C6 + seven-dimension rubric produces better tier separation and more actionable feedback in the authors' hands, with the authors' profiles, evaluated by the authors' choice of puzzles. It does not study whether a practitioner new to the system can load these profiles, apply this rubric to their own creative work, and get reliable results. This gap is not acknowledged as a limitation. The system could be arbitrarily difficult to configure for a new domain, the profiles could be brittle to paraphrase or partial loading, and the rubric's pass threshold (22/30, calibrated to the authors' prior experience) could produce systematically wrong verdicts when applied to unfamiliar work.

From an interactive ML perspective, the recommendation is an offline result deployed into a use context that has not been studied. A minimum acknowledgment would be: "we do not know whether practitioners unfamiliar with the profile architecture can configure and apply this system reliably, and we identify this as a required evaluation before high-stakes deployment."

A stronger version would sketch what a usability study of the practitioner workflow would look like: what instructions would accompany the profile library, what would constitute correct versus incorrect application, and how you would detect miscalibration in deployment.

**Request:** Add explicit acknowledgment that the recommendation is an offline research result, not a validated practitioner workflow. Identify actionability as a distinct future-work direction separate from the human calibration study.

### W2. The calibration inversion needs a warning label, not just a finding

Section 5.2 identifies the calibration inversion as a finding and includes the sentence "Practitioners who adopt structured rubrics for creative work assessment without equally structured expert identity context risk systematically disadvantaging their most ambitious artifacts." This sentence is correct and important. It is also buried in a discussion subsection, framed as a generalization, and not repeated in the conclusion's practitioner recommendation.

The conclusion (Section 6) does include the sentence "Avoid C1 (rubric only) for any creative work that deliberately trades some dimensions for others; it will systematically disadvantage the most ambitious artifacts in your corpus." But the word "systematically" undersells what the data shows. The data shows that in the 15-puzzle corpus, C1 produces the lowest MIT/Elite pass rate (40%) of any condition, fails Information Relay — one of the strongest puzzles in the corpus — at 21/30, and nearly collapses the MIT/Elite vs. Games Magazine gap to 0.6 points. The most straightforward interpretation is: if you naively apply this rubric without expert profiles, your evaluations of your best work will be indistinguishable from evaluations of generic crosswords.

This is not a minor calibration issue. It is the kind of systematic error that propagates invisibly through a creative pipeline. A practitioner who trusts the rubric and notices their best puzzles are scoring lower than expected will likely conclude the puzzles need to be made more accessible, which is the opposite of the correct intervention. They will optimize against the rubric's failure mode.

The paper identifies the problem clearly enough to name it "dangerous." It should then be presented to practitioners as clearly dangerous, with explicit guidance on when C1 is never appropriate and what the symptoms of calibration inversion look like in practice.

**Request:** Move the calibration inversion warning to the conclusion's practitioner recommendation section with prominent framing. Consider a named callout box or Warning paragraph. The current presentation places the most safety-relevant finding in a location practitioners are likely to skim.

### W3. The AI-evaluating-AI-generated puzzles circularity (GM-04 and GM-05) needs explicit framing

The classification-games-magazine document — which informs the ablation corpus construction — explicitly notes that GM-04 (Logic Grid) and GM-05 (Word Search) are "constructed as canonical format examples" and identifies strong AI construction signals in both: placeholder names from CS/logic pedagogy (Alice, Bob, Carol), obvious-set vocabulary, template-pattern clue structures, and in GM-05's case a backward-design failure characteristic of generative systems.

The paper's Section 5.3 (Limitations) acknowledges that "the verdicts reported here are AI-generated assessments of AI-generated and human-generated puzzles" but does not specifically flag the GM-04/GM-05 case or discuss whether AI-generated content being evaluated by AI-simulated reviewers introduces a systematic bias. This is not a fatal flaw — the GM tier's purpose is to establish a floor, and both puzzles are correctly classified as Games Magazine tier in every condition — but the circularity has at least two implications the paper should address:

First, AI-generated puzzles may have artifacts (template structures, placeholder vocabulary, incomplete backward design) that AI evaluators are differentially good or bad at detecting. The paper's data shows GM-05 scoring 13/30 in C6, the lowest score in the entire corpus, which suggests the profiles are sensitive to the construction error — but it is unclear whether they are detecting the error because it is an error or because they recognize AI construction patterns. These are different things with different implications for the system's use on human-generated work.

Second, and more practically: practitioners using this system to evaluate their own AI-generated creative work need to know whether AI-evaluating-AI has different reliability properties than AI-evaluating-human. The paper's corpus mixes these cases without studying them separately.

**Request:** Add a dedicated paragraph in the Limitations section (Section 5.3) addressing the AI-evaluating-AI circularity specifically, with reference to GM-04 and GM-05. Clarify whether the authors believe the evaluation reliability differs between human-generated and AI-generated artifacts, and if so, what the implications are for practitioners working with AI-generated creative content.

### W4. Profile governance: addition, update, and drift

The 29 profiles are the system's core infrastructure. The paper describes their construction — six sections, 80–120 lines, grounded in publicly available writings and interviews — and explains why domain-specific profiles outperform generic ones. It does not address three governance questions that matter for deployment:

**Profile addition:** The paper's profiles encode perspectives of named puzzle hunt practitioners. What is the process for adding new profiles as new practitioners emerge? The puzzle hunt community is small and relatively slow-changing, but in adjacent domains (game design, interactive fiction, tabletop games), the expert community turns over faster. The profile architecture requires that a new profile meet whatever quality bar produces the evaluation improvements observed here — but that quality bar is not operationalized as a rubric for profile construction. The paper gives guidance on what profile sections to include but not on how to verify that a new profile produces the evaluative specificity that makes the system work.

**Profile update:** The profiles encode the evaluative philosophies of specific named individuals as of the time of their construction. Real practitioners evolve: they write new postmortems, revise their positions, move to new domains. A profile grounded in a practitioner's 2018 interview may not reflect their 2026 views. The paper treats profiles as stable artifacts ("authored once, refined over time") but gives no guidance on when and how profiles should be updated, what triggers a revision, or how to detect that a profile has drifted from its source.

**Attributed views:** The profiles make claims in first-person voice about named individuals' evaluative philosophies. The paper appropriately does not claim these are the individuals' actual views, but a practitioner using the system could reasonably confuse the AI's simulation of Rand Miller's views with Rand Miller's actual views — particularly if the profiles are widely distributed. This is at minimum a disclosure issue and potentially a reputational risk for the named individuals.

**Request:** Add a subsection in Section 5 (or an appendix) addressing profile governance: what process should practitioners follow when adding profiles, what triggers a profile update, and what disclosures should accompany distributed profiles to prevent confusion between simulated and actual expert views.

### W5. Riven Standard inter-rater consistency requires empirical support

Section 5.3 states that "reviewers in C11 conditions produce consistent Riven Standard scores with lower inter-reviewer variance than most other dimensions." This is a strong claim about the tractability of a philosophically subtle criterion, and it is the claim that most needs empirical grounding.

The Riven Standard scale (1 = "theme is decorative; any content could substitute" to 5 = "mechanic is inseparable from the domain and could not exist in any other context") requires a reviewer to make a judgment about the structural relationship between a puzzle's mechanism and its subject matter. This is not a factual determination; it is an interpretive one. The paper reports that Riven Standard scores are inferred from the C6 review texts for the Experiment 4 analysis — meaning the scores are derived by the authors from existing review language, not produced independently by multiple reviewers on the same rubric items.

For the inter-reviewer consistency claim to be credible, the paper needs to show Riven Standard scores produced independently by at least two of the three panel reviewers on the same puzzles, with inter-reviewer agreement reported (Krippendorff's alpha or similar). As currently presented, the consistency claim is an author inference from a derived scoring exercise, not an empirical measurement of rater agreement.

The concern is not trivial. If the Riven Standard dimension is high-variance across reviewers, the 24.5pp tier separation result may be largely driven by the double-weighting of Elegance and Reading Reward rather than by the Riven Standard score itself. The decomposed contribution of each factor is not reported.

**Request:** Report Riven Standard inter-reviewer agreement statistics (ideally from at least a subset of the 15-puzzle corpus where multiple reviewers scored the same puzzles on this dimension). If this data is not available, qualify the consistency claim appropriately and report what fraction of the 24.5pp tier separation is attributable to the Riven Standard dimension alone versus the double-weighting of the proxy dimensions.

### W6. C3 outperforms C6 on reliability metrics without being the recommended condition — the explanation needs strengthening

Table 4a in the ablation master results shows C3 (Full Principles, no profiles) and C7 (Full Profiles + Principles) as the most reliable conditions (0 and 1 wrong verdicts respectively), while C6 (Full Profiles) achieves 2 wrong verdicts and a lower overall pass rate (53% vs. 73%). The conclusion recommends C6 as the default condition when principles are unavailable.

The paper's argument for C6 over C3 is that C6 produces richer qualitative feedback and domain-emergent frameworks. This is consistent with the framework-count data. But from a practitioner perspective, the tradeoff is: C3 is more reliable on binary pass/fail verdicts but less rich in qualitative output; C6 is less reliable but more informative. This tradeoff is not made explicit in the recommendation.

More concerning: C3 is never recommended as the default, even though it produces the best reliability in the corpus. The practical reason appears to be that C3's feedback is "retrieved principle labels applied as criteria" rather than domain-emergent frameworks — but this distinction, while analytically important, does not obviously translate to a worse practitioner experience. A practitioner who uses C3 gets better binary verdicts and gets principle-labeled feedback. The argument that C6 feedback is qualitatively superior needs to be made more directly against the C3 alternative, not just against C0.

**Request:** Add an explicit comparison of C3 and C6 in the practitioner recommendation section, acknowledging that C3 achieves better binary reliability and explaining specifically why the qualitative advantages of C6 feedback outweigh the reliability disadvantage in the recommended use case. If the recommendation depends on the nature of the downstream task (binary gate versus iterative creative feedback), say so.

---

## Minor Comments

- The abstract claims "richer profiles surface six analytical frameworks... that generic profiles never introduce." The body text reveals this is a comparison between v2 (C6) and v1 profiles, not between C6 and all conditions including C3/C7. C2 surfaces ten frameworks (though retrieved rather than emergent). The abstract's framing of this as a binary result oversimplifies; please revise.

- The pass threshold of 22/30 is described as "calibrated to the quality level at which puzzles in well-regarded MIT Mystery Hunt and Microsoft Puzzlehunt rounds have shipped." The master results data shows that C6 fails H2No (20/30) and Dropypasta (21/30) at this threshold. H2No is classified as Standard Hunt (not MIT/Elite) in the classification master table — its failure at C6 may indicate the threshold is set too conservatively for Standard Hunt work, or that H2No is genuinely below threshold. This ambiguity should be addressed.

- The paper's experimental design note states that Experiments 2, 3, and 4 are "in progress" or "pending" at submission. The conclusion and abstract are written as if these experiments are complete. The abstract does not flag incompleteness. Submitting with multiple pending experiments requires either narrowing the abstract's claims or providing preliminary data with appropriate uncertainty estimates. The current presentation suggests complete results where there are partial ones.

- Section 4.2 (Calibration Inversion) is placed in the Discussion section rather than the Evaluation section. Given that it presents quantitative findings from the 15-puzzle corpus (a result, not an interpretation), it would be more appropriate in Section 4. The current placement means the paper's second most important empirical finding is not in the evaluation section.

- The paper does not discuss the computational cost of the recommended configuration. Practitioners choosing between C3 (no profiles needed, good binary reliability) and C6 (full profiles, better qualitative output) will care about token overhead, API cost, and evaluation latency. A brief cost comparison would improve the practical guidance.

---

## Summary Assessment

This paper makes three genuine contributions: an empirically grounded argument that persona quality (not presence) is the variable that matters for AI-simulated review; a named and mechanistically explained calibration inversion that practitioners should actively avoid; and a Riven Standard rubric dimension that operationalizes a generalizable creative quality criterion. These contributions are real and would be useful to practitioners building AI evaluation pipelines for creative work.

The paper is limited by insufficient attention to the practitioner experience: whether the recommended configuration is actually usable by someone new to the system, whether the calibration inversion warning is positioned prominently enough to prevent the error it describes, whether the circularity of AI-evaluating-AI-generated content is properly characterized, and whether the profile governance questions have defensible answers. These are not nitpicks — they are the difference between a system that supports meaningful human oversight of creative AI evaluation and one that installs a sophisticated-looking evaluation pipeline with systematically wrong failure modes.

The work is publishable with revisions that address at minimum W1, W2, and W5. W3 and W4 strengthen the paper substantially and address legitimate concerns that reviewers and practitioners will raise.

---

*Review submitted for ICCC 2026. Conflict of interest: none declared.*
