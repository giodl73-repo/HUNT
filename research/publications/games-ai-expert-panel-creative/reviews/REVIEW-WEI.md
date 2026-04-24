# Peer Review — ICCC 2026

**Paper:** AI-Simulated Expert Panels for Creative Work Evaluation: Profile Depth as the Lever
**Reviewer:** Jason Wei (OpenAI)
**Date:** 2026-02-28

---

## Summary

This paper investigates whether AI-simulated expert panels, instantiated through rich domain-specific reviewer profiles, can provide evaluation quality that meaningfully scales expert feedback in creative domains. The authors ground the study in puzzle hunt design, which provides an appealing research testbed: answers are objectively verifiable (binary correctness) while quality is a continuous, multi-dimensional expert judgment. The core empirical contribution is an eight-condition ablation across three puzzles, systematically varying what reviewer context is present in the prompt, from a bare role label to complete domain-specific profiles. The primary findings are (1) a feedback quality gradient — richer profiles surface named, transferable evaluative frameworks absent from simpler conditions; (2) a calibration inversion — rubric-only evaluation produces inverted tier rankings across a 15-puzzle corpus; and (3) a discrimination gain from adding a weighted "Riven Standard" dimension.

The work is engagingly written and tackles a real, underexplored problem. The experimental design is notably more principled than most LLM evaluation papers. My assessment, however, is that several claims outrun their evidentiary support, and the paper would benefit from sharper separation between what is demonstrated and what is theorized. I detail these concerns below.

---

## Strengths

**1. The experimental design is genuinely principled.**

Most papers studying persona-based prompting compare "with persona" to "without persona" and call it an ablation. This paper goes substantially further. The eight-condition factorial decomposition — separating philosophy from lens, compact principles from full operational tests, adding and removing biographical context — is the right way to study this question. The finding that C3 (principles full) and C4 (philosophy only) both reach 26.7/30 average while representing structurally different forms of context is legitimately interesting. It suggests evaluative signal has multiple valid representations, which has real implications for how we think about prompting strategies.

**2. The defect-detection findings are the paper's most interesting empirical result.**

The discovery that C5 (lens only) uniquely detects the duplicate bracket label in Puzzle II — a verifiable binary construction error — while C6 (full profile) misses it, and that C7 catches a separate blank-count error invisible to C6, constitutes the paper's most provocative finding. These are not soft quality judgments; they are falsifiable, traceable errors. The implication that different prompt configurations function as different classes of instrument (quality calibrator vs. defect detector) is novel and practically useful. The recommended two-pass protocol (C6 for quality, C5 for mechanical defects) is a concrete contribution.

**3. Profile-as-prompting-strategy is a useful framing.**

The paper makes a compelling case that the profile document functions like a chain-of-thought scratchpad for evaluation: rather than asking the model to reason from a role label, the profile externalizes the evaluative reasoning chain as named frameworks, operational tests, and characteristic concerns that the model executes during review. This framing — profile as externalized evaluative reasoning — is more precise than "persona prompting" and positions the contribution correctly. I would encourage the authors to develop this framing more explicitly (see Weaknesses).

**4. The domain selection is well-motivated.**

Puzzle hunt design is an underused research testbed and a good one for this question. The binary answer correctness provides genuine ground truth, the expert community is documented and named, and the evaluative vocabulary is on record in postmortems and interviews. This is a better-motivated domain choice than the synthetic or toy creative tasks common in this literature.

**5. Profile length is explicitly called out as the wrong variable.**

Section 5.4 explicitly states: "profile length itself is not the relevant variable" and attributes the v1-to-v2 quality gain to named frameworks and a structured lens, not to line count. This is the right conclusion and it preempts an obvious confound. It is, however, asserted rather than demonstrated (more on this below).

---

## Weaknesses and Questions

### W1. Profile length vs. profile quality: the key ablation is missing.

The abstract correctly claims that the gain from v1 to v2 comes from quality, not quantity. Section 5.4 repeats this claim. But the experiment does not demonstrate it. The v1 profiles (~50 lines) and v2 profiles (~95 lines) differ simultaneously in length, in the presence of named frameworks, and in the presence of a structured review lens. There is no condition that holds length constant while varying framework specificity, nor one that holds framework specificity constant while varying length. Without these cells, the claim that "profile length is not the relevant variable" cannot be distinguished from "profile length is correlated with the relevant variable but we have not separated the two."

Concretely: a v2-length profile stripped of named frameworks and the review lens (padded to ~95 lines with additional biographical text) would constitute the missing control. If scores and framework emergence under this condition match v1 rather than v2, the specificity claim is confirmed. Without it, the claim is plausible but undemonstrated.

This is the paper's most important missing experiment, given that it directly underlies the primary theoretical claim.

### W2. C3 ≈ C4 is surprising and needs more treatment than it receives.

The result that C3 (operational principles with named tests) and C4 (design worldview with no structured lens) both score 26.7/30 is noted in Section 4.2 as "evidence that evaluative signal can be encoded in multiple representational forms." This is a reasonable interpretation. But 26.7 is a single number across three puzzles with one pass/fail verdict each — the variance here is completely unknown. The paper reports no repeated runs, no inter-run variance, and no sensitivity analysis. For a result this surprising, a simple question is unanswered: does C3 ≈ C4 replicate on a second run with temperature 1.0?

At temperature 1.0 (as reported in Section 3.1), score variance across runs will be non-trivial. A difference of even two points on a single puzzle would put C3 ahead of C4 or behind it by a meaningful margin. The tie at 26.7 may be a real finding about representational equivalence, or it may be a temperature artifact. The paper cannot currently distinguish these. At minimum, the authors should report the distribution of outcomes across multiple runs, or acknowledge explicitly that single-run results at temperature 1.0 are unreliable as the basis for claims about structural equivalence.

### W3. Framework emergence is the paper's boldest claim and needs inter-rater reliability.

The paper claims that C6 produces six "domain-emergent" named evaluative frameworks — a-ha economy, load-bearing test, world-model consistency, social fabric, perceptual-shift, visual grammar — that "did not exist in any input text provided to the reviewers." This is presented as the primary qualitative distinction between C2 (retrieved principle labels) and C6 (emergent evaluative tools).

I have two concerns here.

First, framework detection methodology is not described. How were these six frameworks identified? Were reviewers (human or otherwise) given a codebook and asked to mark framework occurrences? Or did the authors read the reviews and identify frameworks themselves? If the latter, this is a post-hoc annotation by the people with the strongest prior about what the paper should find. Inter-rater reliability on framework detection — even a simple second pass by a coder blind to condition — is necessary before "six emergent frameworks" can be treated as a count rather than an impression.

Second, the "not in any input text" claim requires careful verification. The profiles encode design philosophies attributed to named practitioners. Those practitioners' published work, interviews, and public commentary are presumably in the training data for Claude Sonnet 4.6. The model may be reconstructing frameworks from training-data memory of the named person's actual writing, not generating frameworks emergent from the profile's encoded lens. This is the simulation fidelity problem in its sharpest form: when "Thomas Snyder's" C6 review applies the "load-bearing test," is the model reasoning from the profile, or retrieving a known Snyder framework from pretraining? The profile may be functioning as an activation key for training data rather than as a generative context — which is interesting, but a different claim from what the paper makes.

### W4. The simulation fidelity problem is acknowledged but not engaged.

Section 5.4 states that "the evaluations they produce have not been calibrated against those practitioners' actual judgments on the same artifacts" and defers this to future work. This is the right acknowledgment. But the problem runs deeper than calibration: the profiles are constructed from publicly available writings by the named practitioners, and those practitioners are likely represented in the model's training data. The model may be simulating the named experts not because the profile encodes their perspective but because the model already has a representation of those people and the profile is simply activating it.

This has direct implications for the profile-depth argument. If the model's good performance under C6 derives substantially from training-data knowledge of the named practitioners rather than from the profile's structure, then the profile depth finding does not generalize to domains where the "experts" are not in the training data — which is precisely the use case the paper targets (early-stage creative pipelines, scarce domain expertise). The paper should address this confound more directly, even if it cannot resolve it in the current submission.

A straightforward test: generate a C6-equivalent profile for a fictional practitioner with plausible but invented credentials and the same framework structure as a real one. If evaluation quality is similar, the profile structure is doing the work. If it degrades, the named identity is doing the work.

### W5. Temperature 1.0 with single runs makes quantitative claims fragile.

Temperature 1.0 is a reasonable choice for generating diverse, non-degenerate reviews. But combined with single-run evaluation for most conditions, it means the scores in Table 4 are point estimates from a high-variance distribution. C7 scoring Puzzle I at 13/30 versus C0-through-C4 scoring it at 16-21/30 is presented as a meaningful finding about C7's harshness; it may be a single unlucky draw. The paper reports averages across three puzzles but not variance across runs. For a paper centered on quantitative score comparisons, this is a significant gap. Multiple runs with reported variance, or at least a sensitivity analysis, are needed before interpreting the score table as evidence about structural differences between conditions.

### W6. "Framework emergence" vs. "framework retrieval" is the key distinction — but the detection method conflates them.

This follows from W3 but deserves separate attention. The paper's strongest theoretical claim is that C6 profiles cause the model to generate novel evaluative vocabulary from encoded design philosophy, as opposed to C2's retrieval of injected principle labels. The paper supports this with the observation that the six C6 frameworks did not appear in the input text. But "not in the input text" is not the same as "not in the training data." The named frameworks — a-ha economy, load-bearing test — may well appear verbatim in online discussions about puzzle hunt design that the model was trained on. If so, the model is retrieving them from training-data associations with the named practitioner, not generating them from the profile's evaluative stance.

The retrieval-vs.-emergence distinction is foundational to the paper's positioning. It needs a test that distinguishes the two, not just the observation that the text was absent from the immediate context window.

### W7. Ground-truth reliance on tier labels is acknowledged but underweighted in the main text.

The calibration inversion finding — that C1 inverts the tier hierarchy across the 15-puzzle corpus — is presented as a concrete warning for practitioners. But as Section 5.4 acknowledges, the tier assignments (MIT/Elite, Standard Hunt, Games Magazine) are the authors' judgments based on puzzle provenance, not independent oracle verdicts. The finding that C1 produces "wrong" tier rankings presupposes that the tier labels are right. If those labels are uncertain, the calibration inversion becomes less crisp. The main text treats the finding with more confidence than the limitations section warrants.

---

## On the Chain-of-Thought Analogy

I want to flag something I find genuinely interesting that the paper does not develop: the profile-as-CoT framing.

Chain-of-thought prompting works, at least in part, by externalizing reasoning steps that would otherwise be implicit, providing the model with a scratchpad to reason through intermediate inferences before producing a final answer. The v2 profiles — with their operational review lens specifying named tests and the operations that apply them ("strip individual constraints and test whether the puzzle still forces a unique solution") — are doing something structurally similar for evaluation: they externalize the evaluative reasoning chain, providing the model with a named procedure rather than leaving it to reconstruct the procedure from a role label.

This framing would strengthen the paper's positioning in the NLP literature and connect it to a well-studied mechanism with empirical backing. It would also sharpen the W1 concern: if the profile is functioning as CoT, the relevant ablation is "operational tests" vs. "operational tests expressed without named frameworks" vs. "operational tests stripped of their names," analogous to CoT variants in the prompting literature. The paper is currently positioned as a persona-prompting contribution; the CoT framing would additionally connect it to the structured-prompting literature and potentially to mechanistic work on why externalized reasoning helps.

---

## Scores (1--4 scale, 4 = highest)

| Dimension    | Score | Rationale |
|--------------|-------|-----------|
| Novelty      | 3     | The profile-depth variable is understudied; the eight-condition decomposition and defect-detection finding are genuinely novel. The underlying idea (persona depth matters) is anticipated in the related work but not rigorously tested there. |
| Soundness    | 2     | The experimental design is principled, but single-run results at temperature 1.0, missing inter-rater reliability for framework counts, and the absent length-vs.-quality control leave the primary claims insufficiently supported. The simulation fidelity confound is acknowledged but not addressed. |
| Significance | 3     | The calibration inversion finding and the two-pass protocol recommendation are directly actionable for practitioners. The defect-detection result (C5 uniquely catching a verifiable error) is the kind of concrete finding that influences system design. |
| Presentation | 3     | Well-written and well-organized; the ablation table is clear; the distinction between score gradient and feedback quality gradient is made cleanly. The claims in the main text periodically outrun the evidence more than the limitations section acknowledges. |

**Overall: 2.75 (round to 3 for the program)**

---

## Recommendation

**Accept with major revisions.**

The paper has the right question, a notably principled experimental design, and two findings strong enough to stand on their own: the defect-detection result (C5 catching a verifiable extraction error invisible to all other conditions) and the calibration inversion (C1 inverting tier rankings). These are concrete, falsifiable contributions worth publishing at ICCC.

The primary weaknesses are evidentiary, not conceptual:

1. The length-vs.-quality control (W1) is the most critical missing experiment. Without it, the paper's central theoretical claim is plausible but undemonstrated.
2. Framework detection needs inter-rater reliability coding (W3), even a simple one. "Six frameworks emerged" is not a reliable observation without it.
3. Temperature-1.0 single-run results need either variance reporting across multiple runs, or explicit acknowledgment that the score comparisons are suggestive rather than reliable.
4. The simulation fidelity confound — named practitioners in training data vs. profile structure driving behavior — needs to be addressed directly, not just deferred.

The CoT-analogy framing is a strengthening suggestion, not a requirement for acceptance.

If the authors can add (1) repeated runs with variance for the three key puzzles, (2) a brief inter-rater reliability check on framework coding, and (3) a more direct discussion of the training-data confound, the soundness score rises to 3 and the paper is clearly above the bar. The framework-emergence claim, properly qualified, is still interesting. The calibration inversion result, which does not depend on those concerns, is the paper's most robust and generalizable finding — it deserves more prominence in the abstract and conclusion than it currently receives.

---

*Reviewed in the capacity of external reviewer. I have no conflicts of interest with the authors.*
