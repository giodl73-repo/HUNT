# Peer Review: "Generating Coherent Multi-Artifact Creative Systems: An Empirical Characterization of an 11-Stage AI-Assisted Puzzle Hunt Pipeline"

**Reviewer:** Hartmut Koenitz, Södertörn University
**Venue:** Foundations of Digital Games (FDG) 2026
**Review Type:** Standard program committee review

---

## Summary

This paper presents an 11-stage AI-assisted generative pipeline for producing puzzle hunts — competitive solving events consisting of thematically linked puzzles feeding into a coordinating meta-puzzle. The authors argue that puzzle hunts represent a "multi-artifact creative system" requiring coordinated decisions across scope, world-building, authorship, structure, and delivery, and that existing single-shot or agent-loop AI generation approaches fail to maintain coherence across this scope. Their solution is a stage-gated pipeline in which each stage produces version-controlled artifacts consumed by subsequent stages, two of which function as formal quality gates evaluated by AI expert panels.

The paper evaluates the pipeline across five scenarios spanning game-knowledge, noir fiction, science fiction, music, and investigation domains, reporting a 90.4% first-pass editorial pass rate across 52 puzzles. The key structural findings are: failures concentrate at Stage 8 (testing and integration), not at the review gates; the world-lock mechanism appears to prevent canon drift across parallel authoring sessions; and the same skill library transferred across all five domains without per-scenario modification.

---

## Strengths

**1. Genuine cross-domain breadth as empirical evidence.**

The most impressive result in this paper is not the pass rate figure itself but the demonstrated transferability of the pipeline across five thematically distinct scenarios — a real-time strategy game, noir detective fiction, science-fiction role-playing, music, and print puzzles — without modification to the underlying skill library. For researchers in AI-assisted creative systems, this is meaningful. It suggests that the pipeline's structural logic (staged constraint propagation, artifact-driven handoffs, world-lock coherence) is separable from domain content in a way that prior single-artifact AI generation systems have not demonstrated. The claim is well-evidenced: the paper correctly notes that cross-scenario invariance is observable directly from the commit history, and this kind of commit-timestamp methodology — unusual in games research but appropriate here — is a credible form of process traceability.

**2. The world-lock mechanism is a real contribution.**

The conceptual move of treating the Stage 2 world document as a hard read-only constraint — rather than a soft preference or a memory structure subject to revision — is the paper's most interesting design decision. The authors frame it correctly as a coherence mechanism rather than a content mechanism. The distinction between "soft preference" coherence (as in generative agent memory systems) and "hard lock" coherence (as in this pipeline) is not merely procedural; it reflects a substantive position on what coherence means in a multi-artifact creative system. The reported negative result — no post-lock canon violations detected in Stage 7 editorial review across all five scenarios, compared to multiple violations per scenario in unlocked pilot runs — is the right kind of evidence for this claim, even if the comparison condition is not fully controlled.

**3. The failure taxonomy is honest and useful.**

The paper does not paper over its failures. The failure inventory (Table 4) distinguishes mechanism failure, answer coordination failure, construction error, and delivery gap as mechanistically distinct categories, and the authors are direct about which failure types the pipeline's own gates did not catch. The observation that Stage 7 editorial review missed both Ironfall answer-coordination failures because it did not cross-reference META.md is an honest and specific diagnosis — the kind that practitioners can act on — rather than a vague acknowledgment that "more work is needed." The recommendation that follows (Stage 7 should explicitly load and verify answer consistency against META.md) is a concrete design improvement, not a hedge.

**4. The autonomy gradient analysis is the paper's most interesting theoretical contribution.**

Section 3.3 and the discussion of why the middle stages are most AI-autonomous (because constraint density is highest there) while the boundary stages require the most human judgment inverts the intuitive assumption that AI must be supervised most closely at the point of creation. This is a genuinely interesting observation about the relationship between task structure and AI autonomy that extends beyond the puzzle hunt domain. The authors are perhaps too brief here — this point deserves more development — but it is the kind of claim that makes the paper worth reading for researchers beyond the procedural content generation community.

---

## Weaknesses

**1. The paper has no player perspective. This is a serious omission for an FDG submission.**

A puzzle hunt is not a collection of artifacts — it is an experience structured by pacing, difficulty arc, narrative voice, and the emotional shape of the solving journey. The paper's framing is entirely production-oriented: pass rates, stage timing, failure distribution. There is no investigation of what it is like to solve one of these hunts, no evidence that the generated hunts produce satisfying solver experiences, and no framework for evaluating that dimension.

The closest the paper comes is the acknowledgment in the Limitations section that "whether AI panel verdict at Stage 7 correlates with solver enjoyment, puzzle fairness, or aha-moment quality is studied in a companion paper." But this deferral is structurally insufficient for an FDG submission. Foundations of Digital Games is a venue concerned with game experience, not merely game production. A paper that claims to generate puzzle hunts — a form whose value is almost entirely in the experience of playing them — but provides no evidence about that experience has a fundamental gap that cannot be resolved by citing a companion paper in a footnote.

The reviewer recognizes that user studies are expensive and that this pipeline is novel. But even a limited qualitative account — one or two solvers working through a generated hunt, describing their experience — would substantially change the evidential picture. The absence of any such account is not a limitation to acknowledge; it is a scope decision that shapes what claims the paper can legitimately make.

**2. Voice consistency — the closest criterion to player-experience quality — is treated as a checkbox.**

Stage 7 editorial review includes a "voice consistency audit" as one of three additional checks beyond the Stage 3 rubric. The paper describes this check in a single clause: "does the puzzle match the hunt's established narrative voice." This is a genuinely important quality dimension for player experience, and it is the one criterion in the pipeline that directly addresses whether the generated hunt feels tonally coherent to a solver rather than merely mechanically correct.

What does it mean for an AI-generated puzzle to have a consistent narrative voice? How is that voice established — is it encoded in the world-lock document, in the scope document, or in the reviewer profiles? What kinds of voice inconsistency did the editorial review detect in practice, and how were they corrected? Was voice consistency a factor in any of the five reported failures?

None of these questions is addressed. The criterion appears in a subordinate clause and is never returned to. For a paper submitted to FDG, this is a significant analytical gap: voice consistency is the one criterion that directly concerns the cultural and aesthetic coherence of the generated experience, and it receives the least analytical attention of any rubric dimension.

**3. The meta-puzzle's narrative function is unexamined.**

The meta-puzzle occupies a structurally unique position in a hunt. It is not merely the final puzzle; it is the culminating experience that retroactively gives meaning to all the feeder puzzles. A well-designed meta creates what the puzzle hunt community calls a "satisfying click" — a moment when the solver realizes that the feeder answers, which seemed arbitrary during solving, suddenly reveal a hidden structure. This is a narrative experience in the technical sense: it depends on retrospective reinterpretation, on the coherent relationship between parts and whole, on the appropriateness of the final revelation to the thematic frame.

The paper treats meta-puzzle design (Stage 5) and meta-puzzle coordination (Stage 8 failure analysis) as purely structural problems: does the extraction mechanism work, do the feeder answers match the required slots, is the chain verifiable. These are necessary conditions, but they are not sufficient conditions for a satisfying meta. The paper offers no account of whether the generated metas — WOLOLO, SLIT, Konami Code, FADING, CALIBRATE — feel like satisfying conclusions to their respective hunts, whether the extraction mechanisms are thematically appropriate to their domains, or whether the retrospective coherence that defines the meta experience was present in solver feedback.

This is not a peripheral concern. In the puzzle hunt community, meta quality is the primary driver of hunt reputation. A technically correct meta that feels arbitrary or thematically disconnected is a poor meta. The paper has no framework for evaluating this dimension.

**4. The autonomy gradient section contains an unexplored claim about Stage 11 that deserves much more attention.**

The paper states, in a single sentence, that Stage 11 (Polish) is where "the hunt designer's creative voice is most directly expressed: the final pass shapes the solver's experience in ways that are not captured by the quality rubric." This is a striking admission. It acknowledges that the rubric — the paper's primary quality instrument — does not capture the dimension of creative authorship most relevant to the solver's experience. The designer's voice is acknowledged to matter, but only in a stage that the paper codes as human-primary and does not analyze.

If the most experientially important quality dimension (designer voice) and the most structurally interesting puzzle type (the meta) are both outside the rubric's scope, what exactly is the rubric measuring? The paper would benefit from an explicit account of what the rubric captures and what it does not, and from a more honest assessment of where the gap between "passes the rubric" and "produces a good experience" lies.

**5. The comparison baseline is incompletely specified.**

The paper claims that the pipeline addresses the gap of "coherence collapse" in unconstrained multi-artifact AI generation, and references pilot runs without the world-lock mechanism as comparison evidence. But the pilot runs are described only in passing: "multiple coherence errors per scenario in early pilot runs conducted without the lock mechanism." This is a weak comparison condition. The paper would benefit from a more systematic characterization of what unconstrained generation looks like for a puzzle hunt task — either through a controlled experiment or through a more detailed description of the pilot failure modes. Without this, the claim that stage structure is "the intervention, not the individual prompts" is asserted rather than demonstrated.

---

## Detailed Comments

- Section 3.3 (AI Roles by Stage) is the paper's most theoretically interesting section and is too brief. The claim that "constraint density, not creativity, is what drives AI autonomy" is a generative hypothesis for future research that the paper does not fully develop. The authors should either develop it or signal more explicitly that it is a hypothesis for future work.

- The rubric's six dimensions (Clarity, Solvability, Elegance, Reading Reward, Fun, Confirmation) are mentioned throughout but never examined for their experiential implications. "Fun" in particular is a contested and multidimensional criterion in game studies. What does "Fun" mean in the context of a puzzle hunt, how is it operationalized in the AI panel review, and what is the evidence that the panel's "Fun" verdict correlates with solver enjoyment? A brief discussion would strengthen the paper's credibility for an FDG audience.

- The pipeline correctly notes that Stage 11 (hint writing, edge case detection) requires human calibration for "solver populations." This is one of two moments where the paper acknowledges that solvers exist and have experience. The other is the solver simulation at Stage 10. Neither is analyzed. What do the solver simulation reports say? Do they reveal anything about the experiential arc of the generated hunts?

- The paper refers to five "fully-tested" scenarios but "tested" means AI solver simulation (Stage 10), not human play-testing. The choice of terminology is slightly misleading for an FDG audience, where "tested" typically implies human participants.

- The discussion section's five design recommendations are practical and well-grounded in the empirical findings. They are the most immediately actionable part of the paper and may be its most cited section by practitioners.

---

## Scores

| Criterion | Score (1–4) | Notes |
|-----------|-------------|-------|
| Novelty | 3 | The pipeline architecture and world-lock mechanism are genuine contributions; cross-domain empirical characterization of structured multi-artifact generation is new |
| Soundness | 2 | The production-side methodology is sound; the absence of any player-experience evaluation leaves the paper's central claim — that this pipeline generates good puzzle hunts — undersubstantiated |
| Significance | 3 | Real practical significance for AI-assisted creative production; theoretical significance is present but underdeveloped; the player-experience gap limits significance for an FDG audience specifically |
| Presentation | 3 | Clear and well-organized; the tables and failure taxonomy are effective; the autonomy gradient section is too compressed relative to its importance |

---

## Recommendation

**Revise and Resubmit.**

This paper makes a genuine contribution to AI-assisted creative system generation and demonstrates cross-domain pipeline transferability in a way that is empirically credible. It is a strong paper for a production-systems venue.

For FDG specifically, however, it has a fundamental framing problem: it evaluates the pipeline entirely from the designer/producer perspective and provides no evidence about the player/solver experience. The paper's own rubric includes "Fun" as a criterion. Its own Stage 11 acknowledges that designer voice shapes solver experience in ways the rubric does not capture. Its own companion-paper citation acknowledges that AI panel verdict may not correlate with solver enjoyment. The paper knows this gap exists; it has not addressed it.

The minimum revision needed for FDG is an explicit and honest reckoning with this limitation, upgraded from a brief footnote to a substantive discussion of what it means to evaluate a puzzle hunt pipeline without any human solver data. The ideal revision would include at least a pilot player-experience study, even a small one, or a qualitative analysis of the meta-puzzle experiences generated across the five scenarios. Without this, the paper's claims about generating coherent hunts rest entirely on AI-panel verdicts whose validity for the relevant quality dimensions is undemonstrated.

The world-lock mechanism, the autonomy gradient analysis, and the cross-domain transferability result are worth publishing. The question is whether this venue is the right one without the player-side evidence. If the authors cannot acquire that evidence before the revision deadline, FDG-X or a venue more focused on AI systems than on game experience would be a more appropriate target.

---

*Review submitted in good faith. The reviewer has no conflicts of interest with the authors or institutions.*
