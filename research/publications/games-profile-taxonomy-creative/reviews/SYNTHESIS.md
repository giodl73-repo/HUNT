# Review Synthesis — Round 1
**Paper:** Profile Depth and the Reviewing-to-Authoring Transfer Problem: Evidence from AI Expert Personas in Puzzle Hunt Design
**Date:** 2026-02-28
**Reviewers:** Jordanous, Veale, Gervás, Lester, Riedl

---

## Score Summary

| Reviewer | Novelty | Rigor | Clarity | Significance | Overall |
|----------|---------|-------|---------|--------------|---------|
| Jordanous | 3/4 | 2/4 | 3/4 | 3/4 | 2.75/4 |
| Veale | 3/4 | 2/4 | 3/4 | 2/4 | 2.50/4 |
| Gervás | 3/4 | 3/4 | 4/4 | 3/4 | 3.25/4 |
| Lester | 3/4 | 3/4 | 3/4 | 2/4 | 2.75/4 |
| Riedl | 3/4 | 3/4 | 4/4 | 3/4 | 3.25/4 |
| **Average** | **3.0/4** | **2.6/4** | **3.4/4** | **2.6/4** | **2.90/4** |

**Panel average: 2.90/4**

---

## Consensus Strengths

All or most reviewers agreed on the following:

1. **The symmetric gradient finding is a genuine empirical contribution.** All five reviewers acknowledged that the mirror between the authoring and reviewing ablation gradients is real, well-controlled, and interesting. The non-monotonic shape (early plateau, principle-introduction jump, high-conditions plateau) is the paper's most credible quantitative result.

2. **The A2 domain-inversion result is clearly formulated and practically important.** Three reviewers (Jordanous, Gervás, Lester) specifically noted the A2 finding as the sharpest and most interpretable individual result: domain data without a design framework adds negligible authoring quality over bare task specification.

3. **The experimental design is unusually clean for this area.** Four reviewers (Gervás, Lester, Riedl, Jordanous) praised the fixed answer word, fixed mechanism, locked domain corpus, and same-session calibration protocol. The ablation logic is sound.

4. **The Young A6 finding is the most striking and theoretically interesting result.** All five reviewers noted the Young A6 perceptual-lens forced-substitution result as the paper's most distinctive finding, though they differ on how adequately it is explained.

5. **The lens-type taxonomy (construction/operational/perceptual) is a valuable vocabulary contribution.** Three reviewers (Gervás, Lester, Riedl) found the taxonomy useful even while noting its N=1-per-category limitation.

6. **The format ceiling / Riven Standard finding is practically important.** Lester and Riedl specifically identified the insight that mechanism choice determines the ceiling while profile depth determines the floor as a contribution with direct implications for AI content generation system design.

7. **Clarity is high.** Three reviewers scored Clarity at 3-4/4. The condition labeling, table structure, and ablation description are praised across reviews.

---

## P1 — Blocking Issues (must address)

### P1.1: Rubric lacks psychometric validation; claimed precision is unsupported
**Source:** Jordanous (P1.1), Riedl (P2.3)
**Core problem:** Score differences of 0.6 (A1 vs. A2), 1.3 (A4 vs. A5), and 0.7 (Snyder A5 vs. A6) are used to establish theoretical conclusions without any inter-rater reliability statistics. No ICC, Krippendorff's alpha, or per-reviewer disagreement analysis is reported. The per-reviewer scores are visible in Table 1 but the variance is never analyzed.
**Required fix:** Report inter-rater reliability across the rubric dimensions (at minimum, per-dimension raw disagreement ranges; ideally ICC or Krippendorff's alpha). Frame small score differences (< 1.0 point) explicitly as within-uncertainty observations rather than theoretical conclusions. Add at minimum a confidence interval estimate or reliability note to the key comparisons (A1 vs. A2, A4 vs. A5, Snyder A5 vs. A6).
**Target section:** `sections/03-methodology.tex` (rubric subsection) and `sections/04-results.tex`

### P1.2: "Quality" is not operationalized at the construct level; rubric validity is not argued
**Source:** Jordanous (P1.2), Veale (P1.1)
**Core problem:** The paper claims profile depth affects "quality" but quality is defined entirely as rubric score. There is no argument that the rubric is a valid measurement of creative quality independent of puzzle hunt craft conventions. The double-weighting of Elegance and Reading Reward, and the Riven Standard dimension, are design choices that substantially shape the format-ceiling result — but the validity of these weightings is never argued. At ICCC, quality claims require connection to creativity-theoretic frameworks.
**Required fix:** Add a rubric validity paragraph in Section 3.4 that (a) explains why the double-weighted dimensions are the most creativity-relevant, (b) connects at least two dimensions to established creativity criteria (novelty, surprise, domain integration), and (c) acknowledges the rubric's puzzle-hunt specificity and what that implies for generalization claims. Optionally, add a paragraph in Section 5 connecting the gradient findings to a computational creativity framework (SPECS, Wiggins' P-creativity, or similar).
**Target section:** `sections/03-methodology.tex` (rubric subsection), `sections/05-discussion.tex`

### P1.3: The lens-type taxonomy is post-hoc with N=1 per category; it cannot be presented as a validated taxonomy
**Source:** Jordanous (P1.3), Veale (P1.3), Gervás (P1.1)
**Core problem:** Construction/operational/perceptual are labels applied to three individual designers, one per category. The taxonomy cannot be validated with N=1 per type. Additionally, the classification procedure — how would a reader classify a new designer's profile without observing their A5-to-A6 gap? — is not specified. Three reviewers independently flagged this as a circular or post-hoc issue.
**Required fix:** (1) Reframe the taxonomy explicitly as a provisional three-category model derived from N=3 exemplars, requiring multi-designer validation. Change all instances of "taxonomy" to "proposed taxonomy" or "preliminary classification" where the N=1 limitation applies. (2) Add a profile-content classification procedure in Section 3.3: describe how to assign lens type from profile content (e.g., tagging profile statements as prescriptive/procedural vs. evaluative/declarative) rather than from observed transfer outcomes. (3) Move the "taxonomy" language from the abstract and conclusion to the discussion, positioned appropriately as a model rather than a finding.
**Target section:** `sections/00-abstract.tex`, `sections/03-methodology.tex`, `sections/05-discussion.tex`, `sections/06-conclusion.tex`

### P1.4: The companion paper dependency in Section 4.3 is a structural problem
**Source:** Riedl (P1.3), Lester (P2.2)
**Core problem:** Table 3 and Section 4.3 reference the six-framework taxonomy and state that "full empirical evidence — review text instances, frequency analysis, cross-condition comparison — is provided in the companion taxonomy paper." Two reviewers flagged this: the current paper cannot rely on an unreleased companion paper for foundational evidence of a key qualitative result. The six frameworks are used to contextualize the lens-type patterns and the format ceiling, but the evidence supporting their claim to be new (absent from C0-C5 reviews) is deferred.
**Required fix:** Either (a) reproduce the essential evidence in this paper — specifically, the cross-condition presence/absence data showing that the six frameworks do not appear in C0-C5 reviews — as a condensed table or appendix, or (b) rewrite Section 4.3 to present the six frameworks only as interpretive vocabulary introduced in this study, not as empirical findings established elsewhere. Option (a) is strongly preferred. The six exemplar quotes are compelling; the issue is the absence of the frequency/absence data supporting the "none appear in C0-C5" claim.
**Target section:** `sections/04-results.tex` (Section 4.3)

### P1.5: ICCC positioning — the paper's relationship to creativity theory is underdeveloped
**Source:** Veale (P1.1, P1.2), Gervás (P1.3), Jordanous (P3.3)
**Core problem:** Three reviewers independently flagged that the paper's relationship to computational creativity is underdeveloped for an ICCC submission. Veale noted that the paper reads as a prompt-engineering study submitted to a creativity conference; Gervás noted that the cognitive model introduced in Section 2.3 is not operationalized in the lens taxonomy; Jordanous requested connection to SPECS or other CC evaluation frameworks. Whether any output is creative in any CC-theoretic sense is never asked.
**Required fix:** Add a 2-4 paragraph Section 5.5 (or extend Section 5.1) that: (a) connects the symmetric-gradient finding to at least one CC evaluation framework (SPECS or Wiggins' P-creativity), framing the profile-depth gradient as a structural property of how constrained generation systems engage complex tasks; (b) explicitly addresses whether the lens-type model makes claims about creativity or about craft quality, and why the distinction matters at ICCC; (c) connects the Young A6 forced-substitution result to literature on creative constraint and reformulation (Stokes, Boden on transformational creativity, or Ward on path-of-least-resistance in creative cognition). This does not require claiming the outputs are creative — it requires positioning the findings within the ICCC theoretical conversation.
**Target section:** `sections/05-discussion.tex`

---

## P2 — Important Issues (should address)

### P2.1: Same-panel evaluation bias — reviewers evaluated their own authored puzzles
**Source:** Jordanous (P2.1), Lester (P2.1)
**Issue:** Katz, Snyder, and Young served as both the AI authoring personas (A5/A6 conditions) and the C8 evaluation panel. Even with trimmed credentials and injected principles, reviewers scoring their own authored puzzles — particularly on subjective dimensions like Reading Reward and Elegance — is a systematic bias that is not acknowledged. Young scoring her own A5 and A6 puzzles is particularly problematic.
**Enhancement:** Add a paragraph in Section 3 acknowledging this limitation explicitly and discussing whether it creates a directional bias (e.g., do designers score their own authored puzzles higher or lower?). If the evaluation protocol included any blind conditions or ordering controls that mitigate this, describe them. If not, note it as a threat to internal validity that future studies should address with independent raters.
**Target section:** `sections/03-methodology.tex`

### P2.2: The "artist persona" companion study result is used as load-bearing evidence but is not described
**Source:** Lester (P2.2), Riedl (P3.3)
**Issue:** Section 5.3 (format ceiling) uses the artist persona result — a bare two-word persona that produced a higher Riven Standard score than any A-series puzzle — as key evidence for the format ceiling finding. This result comes from an unreleased companion paper. Two reviewers flagged that relying on an unreleased result for a load-bearing argument is not acceptable.
**Enhancement:** Add a 3-5 sentence description of the artist persona, the mechanism it generated (damaged inscriptions with one missing letter per item), and the rubric score it achieved, either in the main text or a footnote. The companion citation can remain, but the minimum evidence needed to evaluate the claim should be present in this paper.
**Target section:** `sections/05-discussion.tex` (Section 5.3)

### P2.3: The pass threshold (33/45, 73%) is unmotivated
**Source:** Jordanous (P2.4), Lester (P2.3)
**Issue:** The pass threshold is introduced in Section 3.5 but never justified. No A-series condition passes this threshold, and the format ceiling argument depends on the threshold as a reference point. The threshold's derivation — from practitioner convention, from human benchmark data, or from this study's calibration data — is not stated.
**Enhancement:** Add one or two sentences in Section 3.5 explaining how the 33/45 threshold was established. If it derives from Paper I calibration data, state this. If it is a practitioner standard, cite the source. If it is constructed for this study, explain the construction logic.
**Target section:** `sections/03-methodology.tex`

### P2.4: The cognitive framework (knowing-that / knowing-how) introduced in Section 2.3 is not operationalized in the results
**Source:** Gervás (P1.2), Veale (P2.3)
**Issue:** Section 2.3 grounds the authoring/reviewing distinction in Polanyi's knowing-that / knowing-how distinction and Weisberg's account of expert procedural knowledge. The lens taxonomy is not connected to this framework in the discussion. The tacit-knowledge argument — that declarative quality standards cannot be fully converted to procedural generation guidance — is introduced and then abandoned.
**Enhancement:** Add a paragraph in Section 5.2 that maps the three lens types onto the knowing-that / knowing-how distinction: operational lenses carry procedural (knowing-how) content that inverts directly to authoring constraints; construction lenses are already procedural by nature; perceptual lenses are purely declarative (knowing-that) and cannot be applied before the artifact exists. This mapping would unify the related work and the theoretical contribution.
**Target section:** `sections/05-discussion.tex`

### P2.5: Figure 1 (gradient comparison) is referenced but not described in the text
**Source:** Gervás (P2.3)
**Issue:** Section 4.1 references "Figure 1" as evidence for the symmetric-gradient claim without describing what the reader should observe. The figure exists (referenced in the paper) but no caption description or text guidance is provided in the manuscript.
**Enhancement:** Add 1-2 sentences at the end of the relevant Section 4.1 paragraph describing what Figure 1 shows (the superimposed authoring and reviewing gradient curves, the structural inflection points, and the scale comparison). Ensure the figure caption is self-contained.
**Target section:** `sections/04-results.tex`

### P2.6: Statistical framing is absent throughout
**Source:** Jordanous (P2.2), Riedl (P2.3)
**Issue:** Score differences are reported without any statistical framing — no confidence intervals, no effect-size measures, no mention of whether the study is adequately powered. The three-reviewer panel is too small for formal inferential statistics, but at minimum bootstrap confidence intervals on the panel averages, or reporting score ranges alongside means, would strengthen the empirical claims.
**Enhancement:** Add a methodology note (Section 3.5 or the rubric subsection) acknowledging the small-N evaluation design and what this implies for the precision of reported differences. Report score ranges (min-max across reviewers) alongside panel averages in Tables 1 and 2. Frame differences below 1.0 point explicitly as "within normal reviewer variance" or similar.
**Target section:** `sections/03-methodology.tex`, `sections/04-results.tex`

---

## P3 — Suggestions (nice to have)

### P3.1: Add an exemplar output comparison between A2 and A3 conditions
**Source:** Riedl (P3.1)
**Suggestion:** Add a small table or inline excerpt showing one or two clues from the A2 puzzle (Stone and Steel) alongside the parallel clues from A3, demonstrating what "database rows vs. constructions" looks like at the text level. This would ground the framework/content distinction empirically rather than rhetorically.
**Target section:** `sections/04-results.tex`

### P3.2: Foreground the A4 ≈ A5 worldview finding as an authoring system design principle
**Source:** Riedl (P2.1), Gervás (indirectly)
**Suggestion:** Add a sentence in the conclusion explicitly framing the worldview-approximately-equals-full-profile finding as a design principle: when building AI authoring profiles, invest in worldview elicitation (where does the designer start? what is the finish condition?) rather than checklist elaboration. The checklist is insurance; the worldview is the generative resource.
**Target section:** `sections/06-conclusion.tex`

### P3.3: Extend Section 5.2 with a worked example of perceptual-profile conversion
**Source:** Lester (P3.2)
**Suggestion:** The practical guidance for converting perceptual reviewing profiles into authoring-compatible generative precursors is the paper's most immediately useful practitioner contribution but is too brief to be actionable. Add a short worked example using Young's narrative-grammar substitution as the template: (1) the original perceptual check ("does the visual hierarchy communicate structure?"), (2) the underlying generative intent ("make the solver feel inside the domain"), (3) the format-specific authoring instruction ("write in second-person present tense with the solver as an active agent in a coherent domain scenario").
**Target section:** `sections/05-discussion.tex`

### P3.4: Add a methodological note on profile statement classification (procedural vs. evaluative)
**Source:** Gervás (P3.3)
**Suggestion:** Even informally, a note describing how one might tag profile statements as prescriptive/procedural (do X during construction) versus evaluative/declarative (assess whether X is true of the finished artifact) would provide evidence for the cognitive claims about lens type that currently rest on inference from output quality alone. This could be a footnote or a two-paragraph appendix.
**Target section:** `sections/03-methodology.tex`

### P3.5: Map the six evaluative frameworks to CC theory (P3 only)
**Source:** Veale (P2.2)
**Suggestion:** If space permits, add brief notes in Section 4.3 connecting the six frameworks to established CC concepts (e.g., perceptual-shift to Wiggins' exploratory creativity; social fabric to collaborative creativity models; a-ha economy to Csikszentmihalyi's flow theory). This would increase the ICCC relevance of what is already an interesting qualitative contribution.
**Target section:** `sections/04-results.tex`

---

## Panel Recommendation
**Major Revision** — The empirical foundation and central findings are solid, and the paper makes a genuine contribution to understanding how profile structure governs AI creative output quality. The blocking issues (P1.1-P1.5) are substantive but addressable without new experiments: they require added statistical grounding, repositioning of the lens taxonomy as provisional, resolution of the companion-paper dependency, and clearer ICCC theoretical positioning. The paper should not require new data collection to address any P1 issue. A strong revision round has a clear path to acceptance.
