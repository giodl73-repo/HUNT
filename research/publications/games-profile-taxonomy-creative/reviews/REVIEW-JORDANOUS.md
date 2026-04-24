# Review: Profile Depth and the Reviewing-to-Authoring Transfer Problem: Evidence from AI Expert Personas in Puzzle Hunt Design
**Reviewer:** Anna Jordanous
**Expertise:** Computational creativity evaluation (SPECS framework), AI creative system assessment, operationalizing creative quality
**Venue:** ICCC 2026

## Scores (1-4)
- Novelty: 3/4
- Rigor: 2/4
- Clarity: 3/4
- Significance: 3/4
- **Overall: 2.75/4**

## Overall Assessment

This paper addresses a genuinely interesting question — whether the profile-depth gradient observed for AI evaluation transfers to AI authoring — and the symmetric-gradient finding is a real contribution. However, the central evaluation instrument (the 45-point rubric) lacks sufficient psychometric grounding to support the precision of the claims being made: score differences as small as 0.6 and 1.3 points are used to establish theoretical conclusions without any demonstration that such differences exceed inter-rater variability or are reproducible. The lens-type taxonomy is the most original contribution but remains post-hoc and unvalidated.

## Strengths

- The research question is clearly motivated and the parallel to Paper I is well-constructed; the symmetric gradient finding provides genuine theoretical interest.
- The A2 domain-inversion effect is a sharply formulated empirical result and the narrative explanation (database rows vs. constructions) is illuminating.
- The six-framework qualitative taxonomy in Section 4.3 introduces genuinely new evaluative vocabulary, even if its connection to the main quantitative argument needs tightening.
- The limitations section is admirably candid about the format ceiling and the scope of the conclusions.
- The lens-type transfer model (construction/operational/perceptual) is a meaningful theoretical contribution if it holds up to scrutiny.

## Major Concerns (P1 — blocking)

**P1.1: The rubric lacks psychometric validation; claimed precision is unsupported.**
The paper makes quantitative claims on differences of 0.6 points (A1 vs. A2), 1.3 points (A4 vs. A5), and 0.7 points (Snyder A5 vs. A6). The rubric has a three-reviewer panel and a weighted 45-point scale, but no inter-rater reliability statistics are reported (no ICC, Krippendorff's alpha, or even raw per-reviewer disagreement data). Without knowing how much raters agree on individual items — particularly the double-weighted Elegance and Reading Reward dimensions, which are the most subjective — the reader cannot know whether a 0.6-point difference is signal or noise. The SPECS framework and broader creativity evaluation literature are clear that score differences require reliability grounding before theoretical weight can be placed on them. The paper reports per-reviewer scores for some conditions (e.g., Table 1 shows DK/TS/DY columns) but never analyzes reviewer disagreement as a validity check.

**P1.2: "Quality" is not operationalized at the construct level.**
The paper claims that profile depth affects "quality" but quality here is defined entirely procedurally as the rubric score. There is no argument that the rubric is a valid measurement of creative quality as distinct from technical compliance with puzzle hunt conventions. Specifically, the double-weighting of Elegance and Reading Reward, and the inclusion of Riven Standard as a single-weight dimension, are design choices that substantially shape the gradient results — particularly the format ceiling finding. The paper acknowledges that the format ceiling appears because the rubric penalizes non-native mechanisms, but this means the rubric is sensitive to a property (mechanism origin) that is not central to the paper's main claims about profile depth. A validity argument for the rubric is needed: why are these dimensions the right ones, why are these weights appropriate, and how does the rubric connect to any external criterion of puzzle quality?

**P1.3: The lens-type taxonomy is post-hoc and relies on N=1 per lens type.**
The construction/operational/perceptual trichotomy is derived from three designers (one per lens type). The paper presents this as a taxonomy, but with one exemplar per category the typology cannot be validated — it describes three individuals, not three categories. Katz could be both operational and construction-oriented on different aspects of his reviewing practice. Young's result is attributed to her perceptual lens being "format-dependent," but an alternative explanation is that the A6 mode simply produced a better mechanism by accident (or by the specific properties of the AoE2 domain that favor narrative scenarios). No evidence is presented that the lens-type label is doing explanatory work beyond describing the three individual results post-hoc.

## Minor Issues (P2 — important but not blocking)

**P2.1: The evaluation panel is also the authoring panel.** The same three reviewers (Katz, Snyder, Young) who authored puzzles under A5/A6 also constituted the C8 evaluation panel that scored all 13 puzzles. Even with trimmed credentials and injected principles, reviewers evaluating their own authored puzzles introduces a systematic bias that is not discussed. Young scoring her own A5 and A6 puzzles on Reading Reward and Elegance is particularly problematic for those dimensions.

**P2.2: Statistical framing is absent throughout.** The paper presents score differences as evidence of theoretical effects without any statistical framing — no confidence intervals, no effect-size measures, no mention of whether the study is adequately powered to detect the differences being claimed. Even a simple bootstrap confidence interval on the panel averages would substantially strengthen the empirical claims.

**P2.3: The qualitative taxonomy (Section 4.3) sits uneasily in this paper.** The six frameworks are interesting but their connection to the authoring ablation results is indirect — they are presented as evaluative language from Paper I's C6 reviews and then gesturally linked to the authoring study. The section would benefit from either fuller integration (show how each framework appears or is absent in the A-series outputs) or deferral to the companion taxonomy paper.

**P2.4: The pass threshold (33/45) is introduced but never defended.** Why is 73% of maximum the pass threshold? If this threshold is defined relative to human puzzle hunt conventions, a citation is needed. If it is constructed for this study, the construction logic should be explained.

## Recommendations (P3 — suggested improvements)

**P3.1:** Add a table or appendix with per-reviewer per-condition scores and an inter-rater reliability measure. This would allow readers to assess the precision of the reported differences independently.

**P3.2:** Frame the lens-type taxonomy explicitly as a provisional model requiring multi-designer validation rather than a confirmed taxonomy. Acknowledging that N=1 per category is a preliminary finding would increase scientific credibility without diminishing the contribution.

**P3.3:** Add a paragraph in Section 5 connecting the symmetric-gradient finding to the SPECS framework or other computational creativity evaluation frameworks — ICCC readers will expect this connection and the paper's results speak directly to the question of how structured assessment contexts affect AI creative output quality.

## Recommendation
Major Revision
