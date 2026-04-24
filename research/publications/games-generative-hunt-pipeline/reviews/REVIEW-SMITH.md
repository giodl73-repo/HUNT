# Peer Review — FDG 2026

**Paper**: "An 11-Stage AI-Assisted Pipeline for Puzzle Hunt Creation: Empirical Characterization Across Five Domains"
**Reviewer**: Gillian Smith (Worcester Polytechnic Institute)
**Expertise**: Mixed-initiative procedural content generation, game design tools, human-AI collaboration

---

## Summary

This paper presents a structured, 11-stage pipeline for generating puzzle hunts using AI, evaluated across five scenarios and 52 puzzles. The pipeline's key contributions are: a stage-gated architecture with two review gates at Stages 3 and 7; a world-lock mechanism to prevent coherence drift; and a skill library of 13 CLAUDE.md prompts that the authors claim transfers across domains without per-scenario modification. The empirical evaluation reports a 90.4% first-pass pass rate and identifies Stage 8 (testing and integration) as the primary failure concentration point.

The paper addresses a legitimate and underexplored problem — generating coherent multi-artifact creative systems — and the engineering effort behind the pipeline is substantial. The gate design with cost-staging rationale is the paper's strongest contribution: the economic argument (fail cheaply at Stage 3, fail moderately at Stage 7, avoid late-stage loss) is well-reasoned and generalizable beyond puzzle hunts. The failure analysis is honest and specific, which is more than can be said for much AI-assisted creativity research. These are genuine merits.

However, the paper has significant gaps that substantially limit its contribution to the research community this venue represents. Most critically: it is written almost entirely from the system-builder's perspective, with almost no analysis of the human experience within the pipeline. For a paper that introduces an "autonomy gradient" as a conceptual contribution and makes claims about "human-AI collaboration," the near-total absence of the human's voice is a serious omission.

---

## Strengths

**S1. Gate design with cost-staging rationale.** The two-gate architecture is the paper's most immediately useful engineering contribution. The argument — that failure cost is proportional to downstream investment, so gates should be placed before major investment thresholds — is clearly stated, economically defensible, and generalizable. Section 3.2 ("Why These Two Gates") is the clearest writing in the paper. The empirical distribution (zero Stage 3 failures, three Stage 8 failures) is consistent with the gates functioning as intended, and the authors are appropriately careful not to overclaim from this result.

**S2. Honest failure analysis.** Table 4 (failure inventory) is a model of honest empirical reporting. The authors categorize each failure by type, name where it was caught versus where it originated, and explicitly call out process gaps (Stage 7 not cross-referencing META.md; the delivery checklist missing HTML artifacts). This kind of failure analysis is rare in AI creativity papers, which tend to report only successes. It significantly increases my confidence in the empirical results.

**S3. Failure concentration identifies actionable design recommendation.** The observation that answer-coordination failures (Ironfall P01, P02) could have been caught earlier if Stage 7 editorial review had loaded META.md is a concrete, specific, and immediately actionable finding. This is exactly the kind of gap analysis a pipeline evaluation should produce.

**S4. World lock as coherence mechanism.** The world lock is a clean, low-cost mechanism for a real problem in multi-artifact generation: drift across parallel or sequential generation runs. The authors correctly note that its value is demonstrated by a negative result (no coherence failures observed), and they are appropriately modest about what this proves. Committing a canonical document before parallel agent runs is sound practice with analogs in software engineering (interface contracts, API versioning) and in PCG constraint propagation.

---

## Weaknesses

**W1. The autonomy gradient analysis is conceptually underdeveloped — and its implications for designer agency are not addressed.**

Section 5.3 introduces the observation that "the pipeline's middle stages — authoring (Stage 6) and editorial review (Stage 7) — are where AI operates most autonomously, while the earliest and latest stages involve the greatest proportion of human judgment." This is described as a "counterintuitive finding," and the paper offers a structural explanation: the middle stages are highly constrained (locked world, coordinated answers, explicit rubric), so AI operates reliably there.

This is interesting, but the analysis stops at the observation level and never asks the question that matters for the community reading this paper: *What does this gradient mean for designer agency?* If the human is most involved at Stage 1 (scope) and Stage 11 (polish), and essentially uninvolved at Stages 4 through 10, then the human's creative role is confined to defining intent at the start and adjusting experience at the end. The substantive creative acts — what puzzles exist, what mechanisms they use, how the world is constructed, how extraction works — are almost entirely AI-generated. Is this augmentation of human creativity, or is it replacement of it?

The paper does not engage with this question. The autonomy gradient is presented as an operational fact and a practical recommendation ("practitioners should expect to invest human time at Stage 1 and Stage 11, and should trust AI autonomy most heavily in the middle") without acknowledging that this recommendation effectively reduces the human designer to a brief-writer and quality approver. The FDG community has grappled seriously with when AI assistance becomes AI authorship; this paper sidesteps that conversation entirely.

More concretely: the Table 1 autonomy coding collapses a great deal of complexity. Stage 1 is coded AP (AI-primary, human confirms), but the scope document drives the entire hunt's identity — the domain, the audience, the delivery format, the thematic frame. How much of that document does the human author? Is the human editing a draft the AI generated, or articulating a vision the AI then transcribes? The distinction matters enormously for understanding where human creative agency actually lives, and the paper does not make it.

**W2. No voice from the people who ran the scenarios.**

The pipeline was executed across five scenarios. Who ran them? The paper refers to "the human" throughout — reviewing Stage 3 output, deciding on Stage 7 overrides, writing hints at Stage 11 — but never identifies or characterizes this person or persons. Were these the authors themselves? Were multiple people involved? Did different people run different scenarios?

This matters because the paper's practical claims rest on how the pipeline was experienced. Section 3.3 states that Stage 7 "are genuine decision points at which the human can redirect the hunt's identity before downstream work commits to a particular direction" — but in the same paragraph reports that "human override occurred in fewer than 5% of pool decisions; the panel's ranking was treated as authoritative in the large majority of cases." Taken together: the human *can* redirect, but in practice almost never does. Is this because the AI panel's recommendations are excellent and trustworthy? Or because the pipeline's momentum discourages overrides? Or because the human running the scenario deferred to the AI out of uncertainty? The paper cannot answer this question because it never asked the humans involved.

A mixed-initiative systems paper would treat the human experience as data, not as background. Did the designers feel they were making creative decisions, or executing the AI's decisions? Were there moments where the world lock prevented a design direction they wanted to pursue? Did the pipeline feel like a creative tool or an assembly line? These questions are not ornamental — they are the core evaluative questions for a human-AI collaboration system.

**W3. "Domain-transferable" is an overclaim for what the evidence supports.**

The paper claims that the pipeline is "domain-transferable" and treats this as a significant contribution. The evidence: the same skill library ran five scenarios spanning game knowledge, noir fiction, sci-fi RPG, music, and investigation. No per-scenario modifications to skill files were required.

This is real evidence for something — call it theme-transferable, or format-transferable. But the paper's own description of what varies per scenario (SCOPE.md, world/ data files, ROUNDS.md) makes clear that what transfers is the *production process*, not anything specific to the creative domain. All five scenarios are puzzle hunts: they share the same structural grammar (feeder puzzles, meta puzzle, answer words, extraction mechanisms), the same quality criteria (Clarity, Solvability, Elegance, etc.), the same delivery formats (website, physical, print-and-play), and the same goal (a coherent solver experience). The "domains" differ only in their thematic content and world canon.

Genuine domain transferability would mean the pipeline could generate a tabletop RPG adventure module, an interactive fiction narrative, or an educational escape room — creative systems with different structural grammars, different quality criteria, and different relationships between component artifacts. The conclusion gestures at this: "Cross-domain pipeline adaptation — to tabletop campaign generation, interactive fiction, or educational puzzle design — would test whether the specification-level transferability observed across hunt domains extends to structurally different creative artifact classes." This is an important open question; it should not be buried in future work. The paper's present framing implies the harder version of transferability has been demonstrated when it has not.

I do not object to the claim that the pipeline transfers across puzzle hunt themes. I object to calling theme-variation "domain" variation in a venue where domain carries technical weight.

**W4. The world lock's creative cost is not discussed.**

Section 5.2 presents the world lock purely as a coherence benefit: it prevents drift, costs nothing, and should always be used. But the lock is also a *constraint on creative iteration*, and the paper does not discuss this cost.

In practice, creative work is not linear. A designer authoring puzzles in Stage 6 may discover that the world Canon committed in Stage 2 forecloses an interesting puzzle direction — a mechanism that would work brilliantly if only a particular character existed, a location had a slightly different property, or a piece of technology worked differently. The world lock makes such discoveries expensive: revising the locked world document would require re-running world-dependent artifacts downstream. The paper acknowledges in Section 2 that "the constraints are not relaxed or negotiated during generation, but locked at explicit gate conditions" and that this "prioritizes coherence over flexibility, accepting that late-stage discoveries cannot propagate back to world-level decisions." But this acceptance is stated without analysis of when it is the wrong trade-off.

For a first hunt with a tight deadline, locking the world early is probably correct. For a hunt where the creative vision is still forming, or where the puzzle concepts are driving the world rather than the other way around, the lock may be premature and creatively costly. The paper needs to discuss the conditions under which the world lock helps versus constrains, and what a designer should do when they discover mid-pipeline that the locked world is wrong. A single canonical document committed to version control is hard to revision-manage when creative iteration is the goal.

**W5. The evaluation does not establish what the structured pipeline contributes over an unstructured baseline.**

The paper claims that "structure is the primary quality lever" and that "unconstrained generation of creative artifacts at this level of quality is not achievable without the stage structure the pipeline imposes." This may be true, but the paper does not demonstrate it. There is no comparison condition: no baseline of unstructured generation against which the 90.4% pass rate can be evaluated. The claim that the structure explains the quality is inferential, not empirical.

This is partly mitigated by the pilot-run comparison in Section 3.1 ("multiple coherence errors per scenario in early pilot runs conducted without the lock mechanism"), but this is mentioned only in passing and does not constitute a controlled evaluation. A reviewer cannot assess the effect of the pipeline structure without knowing what quality unstructured generation produces on the same task.

---

## Scores

- **Novelty**: 3/4. The multi-artifact coherence problem is genuinely underaddressed. The gate placement rationale and world lock mechanism are novel contributions. The autonomy gradient observation is interesting but underdeveloped. Deducted one point for the overclaimed transferability and the absence of the human in a paper nominally about human-AI collaboration.

- **Soundness**: 2/4. The empirical characterization is honest and the failure analysis is careful, but the central causal claim — that structure explains quality — is not supported by a comparison condition. The transferability claim exceeds what the evidence supports. The human-experience data is absent.

- **Significance**: 3/4. The gate-design rationale is sound and generalizable beyond puzzle hunts; it should be useful to practitioners building any structured generative pipeline. The open pipeline specification has real value. Deducted one point for the narrow scope of the "domain" variation, which limits the community this finding actually reaches.

- **Presentation**: 3/4. Well-organized and clearly written in most sections. The failure analysis table is exemplary. The autonomy gradient section is too brief given its importance. The limitations section is honest but the transferability limitation is understated relative to the claim made in the introduction.

**Overall score: 2.75 / 4**

---

## Recommendation

**Revise and Resubmit.**

The paper makes a real contribution — the gate design, the failure analysis, and the open pipeline specification — that deserves to be in the literature. But in its current form it is a system-builder's report, not a human-AI collaboration paper. The autonomy gradient section promises an analysis of where human judgment lives and what AI autonomy costs; it does not deliver that analysis. The "domain-transferable" claim overstates the evidence. The human experience of running the pipeline is entirely absent.

Required revisions:

1. Either substantially expand the autonomy gradient analysis to address designer agency (what creative decisions does the human actually make? where does human authorship live in a pipeline that AI executes?), or reframe the paper's contribution to focus on the engineering and pipeline specification contributions, dropping the human-AI framing where it is not supported by data.

2. Revise the transferability claim to accurately reflect what varies across scenarios (theme/world/scope) versus what would need to change for genuine domain transfer. Reserve "domain-transferable" for the open empirical question it currently is.

3. Add discussion of the world lock's creative cost: when does early lock help, when does it constrain, and what is the recommended practice when a designer discovers mid-pipeline that the locked world is wrong?

4. If any designer experience data is available (even informal notes, chat logs, or post-hoc reflection from the people who ran the scenarios), include it. If not, acknowledge explicitly that the human experience of the pipeline is unstudied and identify it as a primary limitation.

The paper is worth revising. The engineering is careful, the failure analysis is honest, and the gate design is genuinely useful. What is missing is the half of the story that makes this a human-AI collaboration contribution rather than a system report.
