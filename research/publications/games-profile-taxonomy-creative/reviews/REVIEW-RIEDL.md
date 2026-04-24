# Review: Profile Depth and the Reviewing-to-Authoring Transfer Problem: Evidence from AI Expert Personas in Puzzle Hunt Design
**Reviewer:** Mark Riedl
**Expertise:** AI narrative intelligence, computational creativity, story generation, persona-based generation, evaluation of AI creative systems
**Venue:** ICCC 2026

## Scores (1-4)
- Novelty: 3/4
- Rigor: 3/4
- Clarity: 4/4
- Significance: 3/4
- **Overall: 3.25/4**

## Overall Assessment

This is a well-executed empirical study with a clear hypothesis, a careful design, and genuinely interesting results. The A5-vs-A6 finding — that reviewing profiles transfer to authoring in a way that depends on lens type — is the most distinctive contribution and is the kind of result that will influence how practitioners construct AI authoring systems. My primary concern is that the paper treats the model (the LLM generating puzzles) as a black box and never asks what is mechanistically happening when profile depth affects output quality. The result that "framework matters more than content" is an important practical finding but cries out for a computational explanation that the paper does not attempt.

## Overall Assessment

The paper makes a real contribution: the lens-type transfer model gives practitioners a principled tool for deciding whether to invest in building authoring-specific profiles or whether reviewing profiles can be repurposed. The empirical grounding is solid. The primary gap is the absence of any mechanistic account of why profiles produce the gradient they do — what, computationally, a "framework" does that raw "content" does not.

## Strengths

- The symmetric gradient finding is the clearest and most replicable result: the same non-monotonic shape appears in both authoring and reviewing directions, with the same structural inflection points. This is a genuine contribution that establishes the framework/content distinction as an organizing principle for profile design.
- The A5 vs. A6 comparison design is elegant: using the same three experts in both authoring and reviewing roles, with the same rubric, allows direct attribution of score differences to profile type rather than expertise level.
- The Young A6 finding is the paper's most interesting result from a generation perspective. Blocking visual grammar checks forced a structural substitution that produced the highest score in the corpus. This is exactly the kind of constraint-induced creative pressure that generation system designers care about.
- The format ceiling result and the Riven Standard dimension are practically important for the field: the insight that mechanism origin (imported vs. native) determines the quality ceiling, and that no profile depth can substitute for mechanism novelty, is a finding with direct implications for AI content generation system design.
- The writing is clear and the experimental description is detailed enough to permit replication. The table structure and condition labeling are exemplary.

## Major Concerns (P1 — blocking)

**P1.1: No mechanistic account of why framework outperforms content.**
The paper's central empirical finding — that profile framework (design principles, generative worldview) contributes more to output quality than domain content (world data) — is never explained at a computational level. What does the LLM do differently when given a design philosophy versus a civilization roster? The paper's explanation is cognitive (frameworks activate generative orientation; content alone doesn't provide a generative frame) but offers nothing about the actual computational process. At minimum, the paper should acknowledge this gap and position it as a research question. More ambitiously: even a qualitative analysis of what the A2 and A3 outputs look like at the sentence level, or what prompting constructs appear in higher-scoring outputs that are absent in lower-scoring ones, would begin to answer the mechanistic question. Without any such analysis, the paper risks contributing a prompting finding rather than a creativity-systems finding.

**P1.2: The A5-to-A6 finding requires a stronger alternative-explanations analysis.**
The paper attributes the Katz gain (+1.6), Snyder neutrality (-0.7), and Young gain (+3.7) to lens type, and specifically to the operational/construction/perceptual distinction. But alternative explanations are not systematically ruled out. For Katz: his operational vocabulary may simply be more verbally specific and action-directive than the other two profiles, and specificity of language (not lens type per se) may be the causal variable. For Young: the A6 mode may have induced a narrative substitution for an independent reason — the profile's evaluative questions may have been more easily misread as generative instructions than the other two — rather than because of a "forced substitution" mechanism. The paper's lens-type explanation is plausible but not the only plausible account. A brief alternative-explanations paragraph would strengthen the theoretical claim.

**P1.3: The companion paper dependency is a structural problem.**
Table 3 and its discussion reference the six-framework taxonomy as "full empirical evidence provided in the companion taxonomy paper." The companion paper on the taxonomy (cited as \cite{paper3taxonomy}) appears to be the source of the framework definitions, the frequency analysis, and the cross-condition presence/absence comparison that the current paper uses to contextualize its qualitative findings. Depending on an unreleased companion paper for the foundational evidence of a key qualitative result is not acceptable at camera-ready stage. Either the necessary evidence should be reproduced in this paper (possibly as an appendix) or the taxonomy section should be rewritten to present only what can be independently supported here.

## Minor Issues (P2 — important but not blocking)

**P2.1: The "worldview approximately equals full profile" finding (A4 ≈ A5) is treated as confirming a prior claim but is actually the most important authoring-system design implication in the paper.** If a generative worldview alone (where does the designer start? what does the finish line look like?) captures most of the quality value of a full construction profile, the practical implication is that authoring profile construction should focus on worldview elicitation rather than checklist enumeration. This has significant implications for how AI authoring systems should be designed. The paper notes this in passing but does not foreground it as a design principle.

**P2.2: The paper does not discuss what the A0 output actually looks like.** A0 (bare persona: "You are a puzzle designer. Design a puzzle.") produces the lowest-scoring puzzle (21.7/45). What does this puzzle look like? Is it a coherent puzzle? A structureless list? Understanding the floor condition would help readers calibrate the range being measured. Even one or two sentences describing the A0 output's character would help.

**P2.3: The evaluation panel's per-puzzle score variance is unreported.** Table 1 shows per-reviewer scores (DK, TS, DY columns), which is good. But the variance across reviewers for each condition is not analyzed. For the A5-to-A6 comparison — where the total gap is 0.7 points for Snyder — knowing whether this reflects consistent low disagreement across all three reviewers or high variance (two reviewers agree, one dissents) would affect how much weight to place on the finding.

**P2.4: "Calibration drift" is introduced as a concern (Section 3.5) but not discussed as a potential residual confound.** The paper uses same-session scoring to eliminate calibration drift, which is methodologically sound. But reading all 13 puzzles before scoring raises an order effect: the relative positions of puzzles in the reading sequence may affect how reviewers calibrate their expectations. This is not the same as calibration drift but is a related validity threat.

## Recommendations (P3 — suggested improvements)

**P3.1:** Add a section — even a short one — that attempts a qualitative or quantitative analysis of what profile elements produce the A2-to-A3 quality jump. What appears in A3 outputs that does not appear in A2 outputs? If the answer is "the clues become constructions rather than database rows," showing a specific clue comparison would ground the "framework activates generative orientation" claim empirically.

**P3.2:** Foreground the A4 ≈ A5 finding as the paper's primary design implication, not just a confirmatory result. A single sentence in the conclusion — "when building AI authoring profiles, invest in worldview elicitation; checklist elaboration is insurance, not generative resource" — would give practitioners an immediately actionable takeaway.

**P3.3:** Either reproduce the essential framework taxonomy evidence (instance counts, cross-condition frequency data) from the companion paper in an appendix, or rewrite Section 4.3 to present only what is directly evidenced in this study. The taxonomy section as written relies too heavily on work that readers cannot access.

## Recommendation
Minor Revision
