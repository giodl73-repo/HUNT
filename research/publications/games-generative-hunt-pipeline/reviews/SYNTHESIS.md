# SYNTHESIS — games-generative-hunt-pipeline
**Round:** 1 | **Reviewers:** Nelson, Smith, Cook, Koenitz, Dormans
**Average score:** 2.75/4 | **Gate:** PASS | **Consensus:** Major Revision

---

## Score Summary

| Reviewer | Novelty | Soundness | Significance | Presentation | Avg | Rec |
|---|---|---|---|---|---|---|
| Nelson (Adobe Research / Falmouth) | 3 | 2 | 3 | 3 | 2.75 | Weak Accept |
| Smith (WPI) | 3 | 2 | 3 | 3 | 2.75 | Revise & Resubmit |
| Cook (QMUL / ANGELINA) | 3 | 2 | 3 | 3 | 2.75 | Major Revision |
| Koenitz (Södertörn) | 3 | 2 | 3 | 3 | 2.75 | Revise & Resubmit |
| Dormans | 3 | 2 | 3 | 3 | 2.75 | Major Revision |
| **Average** | **3.0** | **2.0** | **3.0** | **3.0** | **2.75** | |

---

## Consensus Strengths

- **World-lock mechanism** (all 5): The Stage 2 canonical document committed as read-only before any generative stage is recognized as the paper's most practically replicable contribution. The negative-result evidence (zero post-lock canon violations vs. multiple per scenario in pilot runs) is the most convincing empirical result in the paper.
- **Gate design with cost-staging rationale** (all 5): The argument that gates should be placed before major investment thresholds — brief at Stage 3, authoring effort at Stage 7, integration cost at Stage 8 — is called "the clearest writing in the paper" (Smith) and "the kind of formal reasoning the field needs more of" (Dormans). Reviewers agree the empirical distribution (zero gate-stage failures, failures concentrated at Stage 8) is consistent with the design intent.
- **Failure taxonomy is honest and actionable** (all 5): Table 4 / Table 3 and the surrounding analysis are consistently cited as the paper's most valuable section. The identification that Stage 7 missed Ironfall answer-coordination failures because it did not cross-reference META.md is a concrete, specific, actionable diagnostic. Cook: "This is exactly the kind of gap analysis a pipeline evaluation should produce."
- **Cross-domain transferability is evidenced by observable artifact** (4/5 — Nelson, Smith, Cook, Dormans): The 142-commit record showing zero per-scenario skill-file modifications is recognized as a stronger form of evidence than author assertion. Reviewers note this is "directly observable, falsifiable, and specific to the strongest version of the claim" (Dormans).
- **Autonomy gradient analysis is genuinely interesting** (Koenitz, Smith, Cook): The observation that middle stages (constrained) are most AI-autonomous while boundary stages (open) require most human judgment inverts the intuitive assumption and is identified as a generative hypothesis for future research. However, all three note it is too compressed to fully support the weight placed on it.

---

## Consensus Concerns

- **Self-referential evaluation: 90.4% pass rate measures AI-panel approval, not solver quality** (all 5): Every reviewer flags that the pass rate is defined against a rubric constructed and deployed by the authors, evaluated by an AI panel whose calibration is the subject of an incomplete companion paper. No human play-testing. Cook notes the rubric includes "Fun" — a criterion that can only be validated by a human having fun, which has not been measured. The caveat in the Limitations section is considered insufficient; it must be applied consistently and prominently.
- **Maturation confound: aggregate pass rate conflates early and late pipeline behavior** (Nelson, Cook, Dormans): The pipeline was in active improvement during the study (Age of Empires: 83%, Dead Reckoning: 100%). The 90.4% aggregate blends a developing system with a mature one. A practitioner deploying this pipeline for the first time should expect 83% behavior, not 90.4%. The paper should either stratify by pipeline maturity or qualify the headline figure as a blended estimate.
- **No comparison baseline: "structure explains quality" is asserted, not demonstrated** (Smith, Cook, Dormans): The central causal claim — that stage structure is the quality lever — has no comparison condition. There is no unstructured baseline, no gate-ablation, and no characterization of Stage 3 rejection rate. Smith: "A reviewer cannot assess the effect of the pipeline structure without knowing what quality unstructured generation produces on the same task." Dormans adds that a high-recall, low-precision gate would also show zero gate-stage failures with no real filtration.
- **No player/solver perspective** (Smith, Koenitz, Cook): Three reviewers flag the absence of any solver-side evidence as a serious omission for FDG specifically. The paper evaluates entirely from the production perspective. Koenitz is most direct: deferring solver experience to a companion paper footnote "is structurally insufficient for an FDG submission." There is no investigation of solver enjoyment, aha-moment quality, meta satisfaction, or voice coherence as experienced by players.
- **"Domain-transferable" overclaims the evidence** (Smith, Cook, Dormans): All five scenarios are puzzle hunts sharing the same structural grammar, quality criteria, and delivery formats. Theme variation is not domain variation. Smith: "Genuine domain transferability would mean the pipeline could generate a tabletop RPG adventure module, an interactive fiction narrative, or an educational escape room." The conclusion's gesture toward this is an important open question that should not be buried in future work.
- **Framing imprecision: "generative pipeline" and "AI creativity" language** (Nelson, Cook): Nelson argues the PCG framing will read as category confusion — the system is an AI co-authorship workflow with human editorial review, not a procedural content generator in the technical sense. Cook makes the same point from the computational creativity side: the AI in Stage 6 operates from a fully specified brief, locked world, and committed answer word; calling this "AI creative authorship" misrepresents what the system does.

---

## P1 — Blocking

### P1.1 Self-referential pass rate must be explicitly scoped in abstract and body

The 90.4% figure appears in the Abstract, Introduction, and Contributions as an unqualified quality claim. Reviewers across all five reviews agree this is the paper's most misleading presentation choice. The figure measures AI-panel approval rate against an AI-constructed rubric — not puzzle quality as human solvers would experience it. Apply the Limitations caveat consistently: every instance of the 90.4% claim in body text, abstract, and contributions must be qualified as "first-pass AI-panel approval rate." Add a dedicated paragraph that explicitly separates what is measured (AI panel pass rate) from what is implied (solver-facing quality), and makes the dependency on the companion calibration paper explicit.

**Target sections:** Abstract, Section 1 (Introduction), Section 3.4 (Empirical Results header), Section 5.4 (Limitations)

### P1.2 Maturation confound invalidates 90.4% as a steady-state figure

The aggregate pass rate is calculated across a pipeline that changed materially between its first and last runs. This is acknowledged in Section 4.5 but not acted upon. Either stratify the pass-rate analysis into early scenarios (Age of Empires, Dead Reckoning small run) versus mature-pipeline scenarios, and present bracketed figures; or restate the 90.4% headline explicitly as a blended estimate from a pipeline in active development, not a stable system. A practitioner deciding whether to adopt this pipeline needs to know to expect ~83% on first deployment, not 90.4%.

**Target sections:** Section 3.4 / Section 4.5 / Section 6 (Conclusion)

---

## P2 — Important

### P2.1 Comparison baseline absent — causal claim requires either evidence or demotion

The paper's conclusion states that "unconstrained generation at this level of quality is not achievable without the stage structure the pipeline imposes." No evidence supports this. Add one of: (a) the Stage 3 rejection rate across all scenarios (what fraction of pool candidates the gate filtered out), which would at minimum establish that the gate did non-trivial work; (b) a characterization of pilot failure modes more detailed than "multiple coherence errors per scenario"; or (c) a reframed claim — that the pipeline is consistent with the gate design intent, without asserting it is the sufficient cause of quality. Option (a) or (b) is strongly preferred.

**Target sections:** Section 4 (gate analysis), Section 6 (Conclusion)

### P2.2 Framing terminology must match system type

Nelson and Cook independently reach the same diagnosis: the paper's "generative pipeline" / "AI creative system" / "AI-authored" language will read as category confusion or overclaim to the relevant communities. Replace with accurate terminology — "AI-assisted editorial pipeline," "structured LLM orchestration workflow," or "human-directed AI-assisted production pipeline." If the authors wish to argue for an extended definition of PCG that encompasses prompted LLM workflows, that argument must be made explicitly rather than imported by citation. Framing should match what the autonomy coding already shows: the human sets scope, approves meta design, and performs final shaping; the AI executes under dense specification.

**Target sections:** Title, Abstract, Section 1, Section 2 (Related Work), Section 5.3 (Autonomy Gradient)

### P2.3 Solver/player perspective must be acknowledged as a substantive gap

FDG is a venue concerned with game experience, not merely game production. Upgrade the companion-paper citation from a footnote-level acknowledgment to a substantive discussion in Section 5 or 6 addressing: what it means to evaluate a pipeline that generates hunts without any human solver data; which rubric criteria (specifically Fun, Reading Reward, voice consistency) cannot be validated by AI panel review alone; and what the Stage 10 AI solver simulation reports actually showed, even qualitatively. If any informal solver data is available — even one play-through with notes — include it. If none is available, state explicitly that the human solver experience is the primary open validation question.

**Target sections:** Section 5.4 (Limitations), Section 6 (Discussion)

### P2.4 "Domain-transferable" claim must be accurately scoped

Revise to accurately distinguish what transfers (the production process, across puzzle hunt themes) from what does not (structural grammar, quality criteria, and delivery format are constant across all five scenarios). The conclusion's gesture toward genuine domain transfer — to tabletop RPGs, interactive fiction, educational escape rooms — should be promoted from a future-work sentence to an explicit open question with a named hypothesis: that specification-level transferability observed across hunt themes may or may not extend to structurally different creative artifact classes. Reserve "domain-transferable" for that harder claim.

**Target sections:** Section 4.4 (Cross-Domain Transferability), Section 6 (Conclusion)

### P2.5 Artifact schemas must be specified at field level

Dormans identifies this as the paper's most significant structural weakness for replication. The pipeline's stage-to-stage handoffs depend on specific document structures (ROUNDS.md, PUZZLE-POOL.md, ASSIGNMENT.md), but the paper never specifies what fields these documents contain. Add at minimum an appendix or supplemental table listing mandatory fields for each key artifact document. The paper currently claims "reproducible from a public git repository" — but the paper-as-document must be independently sufficient for replication to constitute a specification. Clarify also that the skill library is implemented in natural language prompts, not formally enforced code, and that behavioral replication is model-specific.

**Target sections:** Section 3 (System Description), Appendix or Supplement

---

## P3 — Nice to Have

### P3.1 Expand autonomy gradient analysis

Koenitz, Smith, and Cook all identify Section 3.3 / 5.3 as the paper's most interesting theoretical contribution and note it is too brief. The claim that constraint density, not creative complexity, drives AI autonomy is a generative hypothesis for future research. Either develop it — specifically address the question of what human creative agency looks like when the AI is most autonomous at Stage 6, and whether that constitutes augmentation or displacement — or explicitly signal it as a hypothesis awaiting future study. Smith's question is the sharpest: "Is this augmentation of human creativity, or is it replacement of it?"

**Target sections:** Section 5.3 (Autonomy Gradient)

### P3.2 Analyze scaling failure modes rather than noting absence of large-scale testing

Cook proposes four distinct scaling hypotheses: (a) meta-puzzle complexity grows super-linearly with feeder count; (b) world-lock coherence degrades as the world document exceeds context-window capacity; (c) Stage 7 editorial panel loses inter-rater consistency at 200+ reviews; (d) Stage 11 human polish becomes the binding constraint. Naming these as testable predictions turns a limitation disclosure into a research agenda.

**Target sections:** Section 5.4 (Limitations / Future Work)

### P3.3 Stage 2 effort should be characterized separately for fictional vs. factual domains

Dormans observes that Stage 2 is qualitatively different for real-world IP (Age of Empires: curation and verification of factual data) versus fictional domains (Ironfall, Grand Larceny: generative world creation). The transferability claim is stronger if the paper characterizes this difference explicitly and reports human effort at Stage 2 for both domain types. The 41-minute timestamp for Scenario 1 is the only data point; the fictional scenarios are unavailable due to overlapping execution — acknowledge this gap and note its implication for per-stage effort estimates.

**Target sections:** Section 4.4 (Cross-Domain Transferability)

### P3.4 World lock's creative cost should be acknowledged

Smith raises a genuine design tension: the lock is also a constraint on creative iteration. A designer who discovers in Stage 6 that the locked world forecloses an interesting puzzle direction has no low-cost path to revision. Add a brief discussion of conditions under which early locking is the right trade-off versus when it may be premature, and what the recommended practice is when mid-pipeline discovery reveals the locked world is wrong.

**Target sections:** Section 5.2 (World Lock discussion) or Section 6

### P3.5 Report timing consistently as elapsed wall-clock, not implied human-effort

Nelson flags that the 3h17m figure appears in the Abstract and Introduction as an unqualified throughput claim, while the Limitations section correctly notes that commit timestamps capture wall-clock intervals, not active human-minutes. Apply the caveat consistently throughout. Reframe as "elapsed session time for Scenario 1" everywhere it appears, and note that multi-session parallel execution of Scenarios 3–5 makes production-time estimates ambiguous.

**Target sections:** Abstract, Section 1, Section 4.1 (Timing), Section 5.4 (Limitations)

---

## Summary Assessment

This is a genuine engineering contribution — the world-lock mechanism, cost-staged gate design, and failure taxonomy are all novel and immediately useful to practitioners — supported by an honest empirical study that is unusually forthright about its own failure modes. Reviewers are unanimous that the core findings are worth publishing. The path to acceptance requires two categories of correction: (1) precision of claims — the 90.4% pass rate must be consistently presented as an AI-panel approval rate with acknowledged maturation confound, the causal claim about stage structure must be qualified without a comparison baseline, and the "domain-transferable" framing must be scoped to what the evidence actually demonstrates; (2) venue fit for FDG — the absence of any solver-side evidence must be upgraded from a footnote acknowledgment to a substantive discussion of what it means to evaluate a puzzle hunt pipeline without human player data. None of the required revisions demand new experiments; all are matters of accurate framing and honest scoping of existing results.