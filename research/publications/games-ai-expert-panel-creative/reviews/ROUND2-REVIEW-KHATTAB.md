# Round 2 Review: "AI-Simulated Expert Panels for Creative Evaluation: A Profile-Depth Ablation Study in Puzzle Hunt Design"

**Reviewer**: Omar Khattab (Stanford / Databricks; creator of DSPy)
**Venue**: ICCC 2026
**Round**: 2 (Recheck)
**Original Recommendation**: Weak Accept (3/4 avg)
**Round 1 Score**: Novelty 3, Soundness 2, Significance 3, Presentation 4

---

## Overview

I have read the revised methodology, discussion, and conclusion sections against my Round 1 concerns. The revision is substantive in places and evasive in others. The authors have made real improvements to the limitations treatment and the redundancy discussion, but have declined to engage with the two requests I considered most important: automated profile optimization and information-theoretic framing of redundancy. The improvements to soundness framing are genuine and address something I flagged sharply. The non-responses to W1, W3, and W4 leave the paper in a position I would characterize as refined-but-unchanged at its core.

My position has strengthened modestly, not from the revision itself, but because I am now more confident that what the authors have submitted is the paper they intended to submit — a practitioner-facing empirical contribution that makes no claim to be a computational optimization paper — and that reading it against a DSPy framework was, in part, my lens superimposed on their object. Accepting that, the revision handles the soundness concern (W5) credibly and the profile attribution concern (W5 subsidiary) in a way I can accept. The W1/W3/W4 gap remains, but I am less certain than in Round 1 that it represents the paper's primary deficiency rather than a missed opportunity in a neighboring research space.

---

## Concern-by-Concern Assessment

### W1: Manual profile design — no automation framing
**Status: Partial**

The revision adds one sentence to Section 6 under the bullet point "Profile design as a research problem": "Which sections of a profile are necessary for calibration, which are sufficient for framework emergence, and whether profiles can be constructed to surface specific evaluative frameworks on demand are questions that bear on both the efficiency and the reliability of the system."

This is the minimum acknowledgment I asked for in my required revisions item 2. It frames profile design as a research question. What it does not do is what I asked for in that same item: characterize what an automated optimization target would look like given the paper's own metrics, or discuss why automated optimization is or is not tractable given the dataset size. The question is named; it is not engaged.

The Round 1 ask was: "Add a brief section framing profile design as a prompt optimization problem, identifying what an optimization target would look like given the paper's own metrics, and discussing why this direction is or is not tractable." The revision adds a bullet point naming the question. This is Partial credit — the omission is now explicit rather than silent, which is better, but the tractability analysis I requested is absent.

### W2: Hand-designed rubric, no learned-dimension alternative
**Status: Not Addressed**

I asked whether the rubric dimensions could be inductively derived from the data — specifically whether topic modeling of review transcripts, or regression on tier labels, would recover the six dimensions or surface additional ones. The revision makes no change to the rubric discussion beyond language that was already present in the original submission. The authors maintain the six- and seven-dimension rubrics without engaging the question of whether the residual variance is closed.

This is the concern I am now most willing to set aside, because the Riven Standard discussion already demonstrates that the authors are iteratively refining the rubric toward completeness, and the conclusion's acknowledgment that "all quantitative results reported here are valid within the authors' constructed evaluation framework" makes the constructedness of the rubric legible. The limitation is acknowledged if not addressed. This does not satisfy my Round 1 request but it lands in a more defensible position than it did before.

### W3: Redundancy finding should be framed in information-theoretic terms
**Status: Partial**

This is the most substantive content revision. The discussion section (Section 4.4, as reported in the discussion) has been expanded with structural characterization of the three non-redundant principles. The authors now explicitly name the structural property that distinguishes the redundant from the non-redundant principles:

- "Blame the Player" is non-redundant because it is solver-retrospective and profiles are designer-centric — a structural claim about the blind spot category.
- "Verify the Last Mile" is non-redundant because it operates at character-level extraction granularity, finer than any profile lens.
- "Snyder's Computer Test" is non-redundant because it requires explicit test application, not reasoning-in-spirit.

This is the generalization I asked for in Round 1 item 3: "what structural property of a principle predicts whether it will be subsumed by a well-designed profile?" The designer-centric versus solver-retrospective distinction is now made explicitly, and the granularity-level argument for "Verify the Last Mile" is new and precise.

What is still missing: the overlap structure between the 8 redundant principles and specific profile items (do they map one-to-one to lens items, or are they collectively subsumed?), and a formal characterization of what "subsumption" means in this context — whether it is behavioral (the profile produces equivalent output) or informational (the profile carries the information the principle encodes). The C7 degradation result — that adding redundant principles hurts calibration on Puzzle I — is still described as an empirical observation without a mechanism. Is this a context-length effect? Attention dilution? Conflicting evaluative criteria? The paper still does not say.

The structural taxonomy is substantially better. The information-theoretic framing is still absent. Partial.

### W4: C8 framing should be prompt compression
**Status: Not Addressed**

The paper's practical recommendation remains framed as "profile design implication." Token counts are not reported alongside line counts in any revised section I can find. The Pareto frontier of information density versus token cost across the eight conditions is not characterized. The C5 condition's token efficiency relative to C6 for defect detection specifically is not quantified.

The conclusion does introduce the phrase "C8 format, ~75 lines" in the artifact release description. This is not prompt compression framing — it is a line-count description. The token-count gap I identified in MP1 has also not been addressed in the sections I reviewed. This concern is Not Addressed.

I will note, having read the revision carefully, that the absence of token counts is now more conspicuous rather than less: the paper recommends a specific format ("C8 format, approximately 75 lines") in its conclusion and artifact release, which makes the absence of the corresponding token characterization more glaring, not less.

### W5: Persona simulation fidelity not evaluated
**Status: Resolved**

This is where the revision earns the most credit. The authors have made three substantive improvements:

First, the conclusion is rewritten to lead with a framing that I consider appropriate and honest: "All quantitative results reported here are valid within the authors' constructed evaluation framework (author-designed rubric, author-assigned tiers, author-calibrated thresholds); they characterize discriminative power within that framework and should not be read as validated against an external human expert standard." This is exactly the clarification I asked for in Round 1 item 4. The abstract-to-limitations gap that I identified in Round 1 — where the framing implied human-expert agreement that the data did not support — is now closed by making the qualification structural rather than relegated to a limitations section.

Second, the limitations section (Section 5.5, "Profile Construction") now explicitly states: "The profiles synthesize attributed positions rather than directly representing any individual's views, and this limits any claim about simulation fidelity." This is Round 1 item 5 addressed directly and cleanly.

Third, the discussion section adds the "fictional practitioner profiles" control suggestion: "A control using fictional practitioner profiles with structurally equivalent lens sections would disentangle these contributions; this is reserved for future work." This is methodologically the right response to my MP2 concern about training-data retrieval versus profile-driven reasoning.

The human-expert calibration gap is not closed — it cannot be without running the calibration study — but the framing of the claims is now appropriately bounded. I am satisfied that the paper no longer oversells the connection to human expert review.

---

## Minor Points: Status

**MP1 (Token counts)**: Not Addressed. Line counts remain the only reported size metric. This is a real gap given the C8 recommendation.

**MP2 (Framework emergence versus retrieval)**: Substantially addressed in the discussion via the fictional practitioner control proposal and the explicit qualification that the framework term may reflect profile-driven reasoning or training-prior retrieval.

**MP3 (Three-reviewer design generalizability)**: Not addressed in the revised sections I reviewed. The limitation is acknowledged in the original submission's experimental design section and has not been strengthened.

**MP4 (Experiments in progress)**: The conclusion and methodology sections maintain the qualification language from the original. No new data appears to have been added. The in-progress experiments remain flagged appropriately.

---

## Updated Scores

| Dimension | Round 1 | Round 2 | Change | Rationale |
|-----------|---------|---------|--------|-----------|
| **Novelty** | 3 | 3 | — | No change to novelty-relevant content. The profile-depth framing, calibration inversion, and defect detection results are unchanged. The optimization connection remains undrawn. |
| **Soundness** | 2 | 3 | +1 | The conclusion rewrite and the profile attribution limitation are substantive improvements. The paper now makes only the claims the data supports, without the overselling that concerned me in Round 1. The structural quality / human-expert agreement distinction is now made explicit. Raising to 3. |
| **Significance** | 3 | 3 | — | Unchanged. The C8 optimization framing and token-efficiency analysis, which would have pushed this to 4, remain absent. The contributions are real and the release is valuable. |
| **Presentation** | 4 | 4 | — | Still the paper's strongest dimension. |

**Round 2 Average: 3.25** (up from 3.0)

---

## Recommendation

**Accept** (upgraded from Weak Accept).

The upgrade is driven primarily by the soundness improvement. The conclusion rewrite is the kind of revision that matters: the paper now describes itself accurately. The calibration inversion result, defect detection result, and Riven Standard validation are genuine contributions regardless of whether the automation framing I wanted is present. The limitations are now appropriately prominent rather than buried.

I am not withdrawing my Round 1 concerns — the token-count gap (MP1/W4), the missing information-theoretic redundancy characterization (W3), and the absence of a tractability analysis for automated profile optimization (W1) are real deficiencies that leave the paper less useful to researchers working on LLM program design than it could be. But these deficiencies do not undermine the paper's contribution to its stated research question, which is an empirical characterization of how profile depth affects creative evaluation quality. On that question, the paper delivers.

The revision has not strengthened the paper in the directions I most wanted. It has strengthened it in the direction that matters most for a publication record: it no longer claims more than it can support. That is the appropriate criterion for moving from Weak Accept to Accept.

My one remaining non-negotiable for camera-ready: report token counts for each condition alongside line counts. The C8 recommendation in the artifact release uses a line count as the primary format descriptor. This is insufficient for any reader who wants to replicate the context-length properties of the recommended configuration. This should take less than one hour to compute and add to Table 2 or an appendix.

---

*Round 2 review prepared for ICCC 2026. Reviewer conflict of interest: none declared.*
