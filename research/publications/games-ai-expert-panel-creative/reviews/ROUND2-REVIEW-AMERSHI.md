# Peer Review — ICCC 2026 — Round 2

**Paper title:** AI-Simulated Expert Panels for Creative Work Evaluation: Profile Depth as a Quality Variable in Puzzle Hunt Design Assessment

**Reviewer:** Saleema Amershi (Microsoft Research)
**Area expertise:** Interactive machine learning, human-in-the-loop AI, responsible AI deployment

**Round 1 recommendation:** Revise and Resubmit

**Scores (1–4 scale, where 4 = strongest):**
- Novelty: 3 (unchanged)
- Soundness: 3 (raised from 2)
- Significance: 3 (unchanged)
- Presentation: 3 (unchanged)

**Recommendation:** Accept with Minor Revisions

---

## Overview of Revision

The authors have made substantive and targeted revisions that address the most critical concerns from Round 1. The paper's core argument is strengthened, its failure modes are now honestly characterized, and the practitioner-facing guidance is materially improved. Several concerns are fully resolved; one is partially resolved with a remaining gap; two are adequately acknowledged without full resolution, which I consider acceptable given the scope of the study. One new concern arose during this reading.

---

## Concern-by-Concern Assessment

### W1. Practitioner Actionability Gap — PARTIAL

**Round 1 request:** Acknowledge that the C6 + seven-dimension rubric recommendation is an offline research result, not a validated practitioner workflow. Identify actionability as a distinct future-work direction.

**What changed:** The abstract now ends with "All results are characterized as valid within the authors' constructed evaluation framework; a human calibration study remains future work." The conclusion opens by foregrounding this caveat explicitly and repeats it before the practitioner recommendation block. The limitations section (Section 5.3, fifth limitation) adds profile construction as a named limitation.

**Assessment:** The epistemological framing is substantially improved. The paper no longer overstates what the results warrant, and the conclusion's practitioner recommendation is now visibly conditional. However, the specific gap I identified — whether a practitioner unfamiliar with the profile architecture can configure and apply this system reliably — is still not named as a distinct actionability concern. The human calibration study is future work. The practitioner usability study (can someone else use this?) remains unacknowledged as a separate gap. These are different questions: calibration asks whether the system produces the right verdicts; actionability asks whether a new user can get any verdicts at all.

The practical guidance in the conclusion is now considerably more concrete and includes explicit warnings about C1 misuse, so this concern is substantially better addressed than in Round 1. But I would like to see one sentence in the future work list explicitly naming practitioner usability (not calibration) as an open question.

**Remaining request (minor):** In the future-work enumeration in Section 6, add a brief item noting that usability of the practitioner workflow — whether domain practitioners new to the system can apply it correctly without expert scaffolding — has not been studied and is a prerequisite for high-stakes deployment. The calibration and usability items can remain separate.

---

### W2. Calibration Inversion Warning Label — RESOLVED

**Round 1 request:** Move the calibration inversion warning to the conclusion's practitioner recommendation section with prominent framing. The finding called "dangerous" in Discussion should be presented as dangerous in the practitioner-facing material.

**What changed:** Section 5.2 is now titled "The Calibration Inversion: Why Rubric-Only Is Dangerous." The conclusion opens with the calibration inversion as its first numbered finding and includes a bolded practitioner-facing sentence: "For practitioners: a bare rubric without expert identity context systematically disadvantages your most ambitious work." The conclusion's practitioner recommendation block ends with an explicit prohibition: "Avoid C1 (rubric only) for any creative work with dimensional trade-offs by design; it will invert your quality hierarchy and systematically disadvantage your most ambitious artifacts."

**Assessment:** This is exactly what I asked for. The calibration inversion is now the conclusion's first finding, it carries bolded practitioner guidance, and the prohibition on C1 use is explicit. The mechanism is stated clearly enough in the conclusion that a practitioner who reads only that section will encounter the warning. The subsection title in Discussion now flags the severity from the heading. I have no further concerns on this item.

---

### W3. AI-Evaluating-AI Circularity — RESOLVED

**Round 1 request:** Add a dedicated Limitations paragraph addressing the AI-evaluating-AI circularity, with reference to GM-04 and GM-05.

**What changed:** The third limitation in Section 5.3 now addresses the ground-truth problem directly: "The verdicts reported here are AI-generated assessments of AI-generated and human-generated puzzles; no external oracle determines which assessments are correct." The language is spare but correct. The paper does not separately flag GM-04 and GM-05 by name in the limitations, which I would have preferred, but the circularity is now explicitly characterized at the structural level.

**Assessment:** The core epistemic disclosure is now present. The paper correctly frames the results as characterizing discriminative power within the authors' framework rather than validated accuracy against an external standard. I accept this resolution. A practitioner reading the limitations section will understand that the evaluation results do not have external validation. The GM-04/GM-05 specific case is not named, but the general disclosure is adequate. I am not escalating this.

---

### W4. Profile Governance — PARTIAL

**Round 1 request:** Address profile governance: profile addition process, update triggers, disclosure to prevent confusion between simulated and actual expert views.

**What changed:** The fifth limitation in Section 5.3 now discloses: "The 29 reviewer profiles were constructed by the research team based on their reading of each practitioner's published work, interviews, and documented design positions — not based on self-descriptions by the named individuals. The profiles synthesize attributed positions rather than directly representing any individual's views, and this limits any claim about simulation fidelity."

**Assessment:** The attribution disclosure is now present and appropriately scoped. This addresses the most acute concern — that distributed profiles could be mistaken for the individuals' actual views. The profile addition process and update triggers are still not addressed. I consider this an acceptable gap for the current publication: the paper is a research contribution, not a deployment guide, and governance questions of that operational specificity belong in a companion systems paper or release documentation. The disclosure that profiles are attributed rather than self-described is the minimum required for responsible publication, and it is now present. The concern is resolved for publication purposes.

---

### W5. Riven Standard Inter-Rater Consistency — NOT ADDRESSED

**Round 1 request:** Report Riven Standard inter-reviewer agreement statistics. If data is unavailable, qualify the consistency claim and decompose the 24.5pp tier separation into contributions from the Riven Standard dimension versus the double-weighted proxy dimensions.

**What changed:** Section 5.3 still states that "reviewers in C11 conditions produce consistent Riven Standard scores with lower inter-reviewer variance than most other dimensions." The abstract still treats the 24.5pp result as a unified finding without decomposition. No inter-reviewer agreement statistics appear anywhere in the revised text.

**Assessment:** This concern is not addressed. The claim that Riven Standard scores are more consistent across reviewers than other dimensions remains empirically unsupported in the text. The paper does add a note in Discussion (Section 5.1) that inter-rater reliability for framework coding "was not assessed; this represents a limitation on the framework-count comparisons." That acknowledgment is relevant and welcome, but it is about framework emergence coding, not Riven Standard score consistency. These are distinct claims and the Riven Standard claim is the stronger one.

The 24.5pp tier separation result is the paper's most striking quantitative finding. Its credibility depends on whether the Riven Standard dimension, as scored, is reliable across reviewers or whether the result is primarily attributable to the double-weighting of Elegance and Reading Reward (which would have been achievable without the new dimension). The paper does not report the decomposed contributions.

I am maintaining this as a required revision. The options are:
1. Report inter-reviewer agreement on Riven Standard scores (preferred).
2. Report the tier separation achieved by double-weighting Elegance and Reading Reward alone (C9 configuration) versus C11, and acknowledge that the incremental contribution of the Riven Standard score itself is not independently validated for rater consistency.
3. Qualify the consistency claim as an observation ("Riven Standard scores appeared to cluster within a narrower range than other dimensions") and move the stronger consistency claim to future work.

Any of these would be acceptable. None is currently present.

---

### W6. C3 vs. C6 Recommendation — RESOLVED

**Round 1 request:** Explicitly compare C3 and C6 in the practitioner recommendation section, acknowledging C3's better binary reliability and explaining why C6 is recommended for the stated use case.

**What changed:** Section 5.4 (Profile Design Implications) now includes: "We recommend C6 as the default because it produces richer diagnostic output — named frameworks, reviewer-specific concerns, voice — that C3 cannot generate, even where C3's binary accuracy is marginally better." The conclusion's practitioner recommendation includes the same comparison and makes the trade-off explicit.

**Assessment:** The trade-off is now named and the recommendation is explicitly conditional. The paper acknowledges C3's reliability advantage without hiding it. This is adequate.

---

## New Concern (Round 2)

### N1. Abstract Calibration Caveat Creates a New Ambiguity

The revised abstract now ends with: "All results are characterized as valid within the authors' constructed evaluation framework; a human calibration study remains future work." This disclosure is correct and important. However, the placement — as the abstract's final sentence, after three paragraphs of specific quantitative claims — creates an interpretive problem: the sentence applies equally to all claims in the abstract, but a reader moving from abstract to introduction may not carry forward that the 24.5pp result, the 80% vs. 40% inversion result, and the six emergent frameworks are all qualified by this sentence.

This is a structural issue, not a factual one. The qualification is present; it may not read as applying to the specific numbers that precede it.

**Minor request:** Consider opening the abstract's concluding sentence rather than closing with it, or rephrase to make clear the sentence qualifies the quantitative results above it: "These quantitative results are characterized as valid within the authors' constructed evaluation framework (author-designed rubric, author-assigned tiers, author-calibrated thresholds); a human calibration study is identified as future work before practical deployment."

---

## Summary Assessment

The revised paper is substantially improved. The calibration inversion warning is now properly foregrounded as a practitioner danger, not buried in discussion. The profile construction limitation is now disclosed, adequately scoped, and appropriately framed. The AI-evaluating-AI circularity is acknowledged at the structural level. The C3 vs. C6 trade-off is now explicit. The paper's epistemic posture — results valid within an internally constructed framework, external validation pending — is now consistent from abstract through conclusion.

One concern from Round 1 remains unaddressed (W5: Riven Standard inter-rater consistency), and I consider it a required revision before acceptance. The 24.5pp tier separation is the paper's headline quantitative result; the consistency claim about the dimension that produces it needs either empirical support or appropriate qualification. This is a bounded fix.

One new minor concern (N1) involves abstract structure, not content, and is easy to address.

The actionability gap (W1) remains partially open but is substantially better characterized than in Round 1. I accept the current treatment as adequate for publication given the paper's scope, with the single sentence request about usability as distinct from calibration.

The paper's contributions — calibration inversion as a named failure mode, profile depth as a quality variable, Riven Standard as a generalizable discriminator, redundancy analysis of design principles — are real, carefully documented, and useful to the community. These contributions survive the limitations now honestly disclosed. I recommend acceptance conditional on addressing W5 and N1.

---

*Round 2 review submitted for ICCC 2026. Conflict of interest: none declared.*
