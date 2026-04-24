# Peer Review — ICCC 2026 Submission (Round 2 Recheck)
## "Expert Panels Without Experts: Profile-Based AI Review for Creative Work Evaluation"

**Reviewer:** Percy Liang (Stanford University)
**Date:** 2026-02-28
**Round:** 2 (Recheck after Major Revision)
**Venue:** ICCC 2026

---

## Percy Liang — Round 2 Recheck

### P1.1 Self-referential validation: Partially Resolved

The authors have made a genuine and meaningful change here. The abstract now closes with an explicit disclaimer ("All results are characterized as valid within the authors' constructed evaluation framework; a human calibration study remains future work"), the contributions list in Section 1 embeds a bolded caveat directly into the system description ("Note: all quantitative findings are valid within the authors' constructed evaluation framework; the rubric, tier assignments, and thresholds were constructed by the research team and have not been externally calibrated against a held-out human expert standard"), the methodology section repeats the threshold-calibration caveat in the rubric description, the conclusion opens by foregrounding it, and the limitations section (Section 5.4) is more expansive and precise than before.

This is the right rhetorical move and it substantially reduces the risk of overclaiming. However, the underlying problem — that no external anchor exists — has not been closed; it has been clearly labeled. That is an improvement, but what I asked for in Round 1 was a minimal human calibration, not better labeling of its absence. The revised paper is more honest about what it is; it has not become what I asked it to be. I accept that the authors judged such a study infeasible within revision scope. The disclaimers are appropriately pervasive and specific enough that a careful reader will not mistake internal consistency for external validation. I am willing to treat this as a partial resolution given the venue and scope constraints, but the gap remains open.

### P1.2 Feedback quality claim: Partially Resolved

The revision makes two substantive changes. First, Section 5.1 now explicitly distinguishes what is established ("these frameworks appear in C6 reviews and not in C0/C1 reviews, and they identify specific, repair-pointing issues that rubric vocabulary does not surface") from what is not established (that the frameworks are diagnostically accurate, that they improve constructor outcomes, or that they causally derive from the profile rather than the model's training prior). Second, the conclusion's statement of the feedback quality gradient is now hedged: "Whether this reflects profile-driven emergence or retrieval from the model's training prior is not established by this study."

This is the qualification I asked for. The claim has been scoped to what is demonstrable — differential appearance of named frameworks across conditions — and the authors now explicitly acknowledge the training-data confound rather than treating profile-emergence as established. The word "better" is not resolved by a user study (which I did not expect at this stage), but the claim no longer rests on an unacknowledged equivocation. This is sufficient for the venue.

### P1.3 Experiment status: Partially Resolved

The experimental design section now explicitly states upfront: "We report results from three experiments, one of which has complete data and two of which are in progress at the time of submission." Each incomplete experiment (Experiments 2, 3) is now labeled with a statement that data collection is ongoing and that results presented reflect available data at submission time. Future figures and tables are described as planned ("Figure 4 will present...") rather than presented as if complete.

My Round 1 concern was that incomplete experiments were presented as findings. The revision has converted those presentations into clearly-labeled in-progress reports, which is an adequate correction. What remains is that the paper still publishes results from experiments with partial data — the cross-hunt generalizability and divergence results are incomplete — and the reader must hold that qualification throughout. The correction is structural rather than substantive: the underlying incompleteness is now disclosed rather than obscured. This is acceptable for a conference venue with the understanding that the complete study remains future work.

### P2 items addressed

**Training-data confound (P2, Weakness 6):** Explicitly acknowledged in Section 5.1 and the limitations section. A control using fictional practitioner profiles is proposed as future work. The confound is now disclosed at the point of the claim rather than deferred entirely to limitations.

**Statistical uncertainty on small-N pass rates (P2, Weakness 3):** The rubric description now notes that "Both thresholds correspond to approximately 73% of the maximum" and that scores represent "point estimates without sampling variance," but the 80% vs. 40% pass-rate comparisons in the discussion still appear without any statement of the underlying counts (4/5 vs. 2/5) or what a single-puzzle change would do to the numbers. This remains incompletely addressed; I had asked for it explicitly and it is not present.

**Framework count vs. quality conflation (P2, Weakness 5):** Section 5.1 now explicitly states "The quality of named frameworks is the signal, not the count" and explains the C2 vs. C6 distinction. This is adequately addressed.

**Tier assignment validity (P2, Weakness 4):** The limitations section now explicitly states that tier assignments are "judgments made by the authors based on puzzle provenance and community consensus, not independent oracle verdicts." The circularity remains structurally present but is accurately characterized.

---

### Updated Scores

| Dimension | Round 1 | Round 2 | Change |
|---|---|---|---|
| Novelty | 3 | 3 | No change — the contribution profile is unchanged; the revision did not add findings |
| Soundness | 2 | 2 | No change — the self-referential structure remains; the improvement is in how it is disclosed, not in its resolution. The missing pass-rate uncertainty reporting (my explicit Round 1 ask #3) is a continuing gap. |
| Significance | 3 | 3 | No change — the calibration inversion finding remains the paper's most significant contribution and is unaffected by the revision |
| Presentation | 3 | 3 | Marginally improved by the pervasive disclaimers and clearer experiment-status labeling, but not enough to move the score |

---

### Updated Recommendation: Minor Revision

The revision is a genuine improvement. The authors have taken the self-referential validation problem seriously and embedded disclosure of it at every point in the paper where overclaiming was previously possible — the abstract, the contributions list, the rubric description, the conclusion, and a substantially expanded limitations section. The feedback quality claim has been scoped to what is demonstrable. The incomplete experiments are now labeled as such. These changes do not resolve the underlying gaps, but they correctly characterize the paper's epistemic situation, which is what I can reasonably expect at this stage without a new study.

Two items remain. The pass-rate comparisons (80% vs. 40%) still lack any uncertainty reporting — I asked explicitly for a statement of the underlying counts (4/5 vs. 2/5) and what a one-puzzle change would do to the numbers, and this has not appeared. This is a minor presentation fix that the authors can make without new analysis. And the incomplete experiments remain incomplete; the paper is publishing partial results from Experiments 2 and 3 with appropriate labels, but those results are not yet ready to support strong conclusions. I recommend acceptance conditional on adding the N-counts and single-outlier sensitivity language to the pass-rate discussion. The paper is now honestly scoped to what it has actually done, and what it has done — the calibration inversion finding, the profile component ablation, the training-data confound disclosure, the Riven Standard rubric contribution — is worth publishing.
