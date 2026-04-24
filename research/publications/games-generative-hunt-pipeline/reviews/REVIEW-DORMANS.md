# Peer Review: "An 11-Stage AI-Assisted Pipeline for Puzzle Hunt Creation"

**Reviewer:** Joris Dormans
**Venue:** Foundations of Digital Games (FDG) 2026
**Date:** 2026-02-28

---

## Summary

This paper presents an 11-stage structured pipeline for generating puzzle hunts using AI assistance, evaluated across five scenarios covering five thematic domains and three delivery formats. The core contribution is a process architecture: sequential stages with defined artifact handoffs, two explicit review gates (at Stage 3 and Stage 7), a world-lock mechanism to prevent coherence drift, and a library of 13 skill prompts that execute the pipeline without per-scenario code changes. The empirical evaluation reports a 90.4% first-pass pass rate across 52 puzzles, with failure concentrated at Stage 8 (post-gate integration) and zero failures at either gate stage. The paper is readable, clearly motivated, and the failure analysis is its strongest contribution. However, several structural and methodological weaknesses limit the rigor of the contribution as presented.

---

## Strengths

### 1. Failure analysis is the paper's most rigorous contribution

Table 3 (the failure inventory) and the surrounding analysis in Section 4.4 constitute the paper's most valuable section. The four-category failure taxonomy — mechanism failure, answer coordination failure, construction error, delivery gap — is not just descriptive; it is analytically grounded. Crucially, the paper maps each failure type to its earliest detectable stage, which is precisely the right question for a staged pipeline: not where a failure manifests, but where the system's design should have caught it. The observation that answer coordination failures (Ironfall P01, P02) are structurally undetectable at Stage 3 because meta design has not yet occurred, and detectable at Stage 7 only if the reviewer cross-references META.md (which they did not), is a lucid analysis of a real design gap. This is the kind of reasoning that makes pipeline design knowledge transferable, and it belongs in the methodological canon for AI-assisted creative systems.

### 2. The two-gate cost-staging argument is sound

The economic logic for placing review gates at Stages 3 and 7 rather than at other stages is clearly stated and internally consistent. The cost-of-failure argument — brief only at Stage 3, authoring effort at Stage 7, integration-plus-delivery at Stage 8 and beyond — is a structured design rationale that goes beyond "we tried it and it worked." This is the kind of formal reasoning about pipeline economics that the game design engineering literature needs more of. The empirical data (zero failures at both gates, three failures post-gate) is consistent with the design intent, though it raises the question of whether the gates are calibrated or merely lenient (see Weaknesses).

### 3. The world-lock mechanism is a clean structural solution

The world lock — a single committed document, treated as read-only by all downstream stages — is an elegant and appropriately minimal answer to coherence drift in multi-stage generation. The mechanism is simple enough to be replicated without the specific software stack, which is exactly the property a structural contribution should have. The negative result (zero coherence failures across all scenarios) is argued correctly: absence of failure is the intended outcome, not an uninformative result, because the failure class the lock prevents is well-defined.

### 4. Cross-domain invariance as transferability evidence

The empirical record of 142 commits containing zero per-scenario skill-file modifications is a concrete operationalization of the transferability claim. Observing this in a version-controlled artifact is more rigorous than a claim made in prose, and the method of using commit history as a verifiability mechanism is methodologically sound.

---

## Weaknesses

### 1. Artifact schemas are not formally specified — replication is underspecified

This is the paper's most significant structural weakness. Section 3 (System Description) and Table 1 identify inputs and outputs for each stage: SCOPE.md, ROUNDS.md, PUZZLE-POOL.md, ASSIGNMENT.md, META-DESIGN.md, and so on. But the paper never specifies what these documents contain. What fields are mandatory in ROUNDS.md? What scoring schema does PUZZLE-POOL.md use? Does ASSIGNMENT.md include the target answer word, the extraction mechanism, both, or neither? Without formal schemas — even expressed informally as field lists or example structures — these are artifact names, not artifact specifications.

The importance of this gap is not cosmetic. The pipeline's correctness claims depend on stage-to-stage handoffs working as described: Stage 4 consumes PUZZLE-POOL.md and produces per-puzzle ASSIGNMENT.md files; Stage 5 uses feeder answers; Stage 7 optionally cross-references META.md. If a practitioner implements ROUNDS.md in a different format — as a prose document rather than a structured table, for example — the Stage 3 pool generation may produce a pool that does not map correctly onto round slots, and the Stage 8 integration checklist may not verify answer coordination correctly. The paper claims the pipeline is "reproducible from a public git repository," and perhaps it is, but the paper as a document does not specify it. Readers who cannot or do not access the repository cannot replicate the pipeline from the paper alone. For a contribution framed as a "precise specification," this is a material shortfall.

The implementation section (3.4) acknowledges that skills are markdown text files in ~/.claude/skills/, and the paper correctly notes these are "persistent markdown instruction files." But a markdown instruction file is neither a formal interface definition nor an executable program. The paper's use of "implemented" to describe this substrate should be qualified: the pipeline is specified in natural language prompts, not implemented in the engineering sense. This distinction matters for claims about replication and transferability.

### 2. Stage-count decomposition is presented as settled, but its optimality is not analyzed

The paper presents an 11-stage decomposition as though the number and boundaries of stages are determined by the task structure. They are not argued to be. Could a 7-stage pipeline — collapsing Scope + World Build, merging Assignment + Meta Design, collapsing Editorial + Integration — achieve similar quality properties? Could a 15-stage pipeline — splitting Authoring into constraint-setting, drafting, and self-review sub-stages — improve the Stage 8 pass rate? The paper neither asks nor answers this question.

This matters because the decomposition is the primary structural contribution. Without a principled account of what determines the right granularity, the 11-stage structure is an engineering artifact of how this particular team iterated on this particular system over two days, not a generalizable design principle. The "pipeline maturity growth" subsection (4.5) actually undermines the decomposition's claimed status: the pipeline grew from some unspecified smaller structure to 13 skills over 48 hours in response to failures discovered during execution. This is normal iterative development, but it means the stage count is empirically derived, not principled. The paper should be honest about this and frame the 11-stage structure as one candidate decomposition with observed properties, rather than as a specified architecture with justified stage boundaries.

A related question: what are the coupling rules? Why is meta design (Stage 5) ordered before authoring (Stage 6)? The paper gives a correct functional reason — the meta determines which feeder answers are required, so answer words must be established before puzzle content is written — but this is one ordering constraint, not a general theory of stage dependencies. A formal dependency analysis, even a simple directed acyclic graph of which artifacts block which stages, would make the architecture significantly more rigorous and replicable.

### 3. Domain knowledge transferability is conflated with pipeline transferability

Section 4.4 (Cross-Domain Transferability) argues that the pipeline transfers across domains because the skill library required no per-scenario modifications. This is true but conflates two distinct claims. The pipeline process transfers. The domain knowledge does not.

Stage 2 (World Build) is the locus of this distinction, and the paper largely elides it. For fictional domains (Ironfall, Grand Larceny, Wavelength, Dead Reckoning), Stage 2 requires the generation of a coherent fictional universe: geography, characters, factions, history, thematic vocabulary, tone. For real-world domains (Age of Empires), Stage 2 requires sourcing, verifying, and canonicalizing accurate factual data from an existing IP. These are qualitatively different tasks. In the fictional case, Stage 2 is a generative act; in the factual case, it is a curation and verification act. The /hunt world skill prompt presumably accommodates both, but the paper does not explain how, and it does not characterize the human effort required per scenario at this stage.

The transferability claim would be significantly stronger if the paper characterized the Stage 2 effort separately from the rest of the pipeline. How many human-minutes does Stage 2 require for a fictional domain versus a real-world domain? How does the quality of Stage 2 output affect downstream pass rates? The world-lock mechanism is only as coherent as the world document it locks. If Stage 2 produces a sparse or internally inconsistent world document, the lock protects downstream consistency against a weak anchor. The paper notes that Stage 2 is "41 minutes" in Scenario 1 (Age of Empires), but timing data for the fictional scenarios is unavailable due to overlapping session execution, which is precisely where the interesting comparison would be.

### 4. The empirical baseline is absent

The paper's 90.4% first-pass pass rate is presented as a quality result, but there is no comparison condition. What would the pass rate be with a single-stage prompt? With a pipeline that lacks the Stage 3 gate? With the Stage 7 gate removed? The paper asserts that "unconstrained generation of creative artifacts at this level of quality is not achievable without the stage structure the pipeline imposes" (Conclusion), but no data is presented to support this assertion. This is not a minor gap — it is the central empirical claim of the paper.

The failure analysis does provide partial evidence: the distribution of failures (zero at gates, three at Stage 8) is argued to be consistent with gates functioning correctly. But consistent-with is not the same as demonstrated-by. A pipeline whose gates are set too loosely would also show zero gate-stage failures while offering no actual filtration. The Stage 3 recall calibration note (the gate is "calibrated to have high recall rather than high precision") raises this concern directly: a gate with very high recall and very low precision would let nearly everything through and still show zero Stage 3 failures. Without knowing the rejection rate at Stage 3 — how many pool candidates were filtered — the gate's contribution to the 90.4% figure cannot be isolated.

The Dead Reckoning scenario (19 puzzles, 45 candidates, 100% pass rate) provides the closest thing to a natural experiment: a large pool allowed stronger candidate selection, and the pass rate was higher. But this is a single data point, and it confounds pool size with scenario order (Dead Reckoning ran last, after pipeline maturation). A controlled ablation — running the pipeline with Stage 3 gate disabled, or varying the gate threshold — would convert the empirical claim from plausible to demonstrated.

### 5. The skill library as "implementation" is understated in its epistemological status

The paper frames the skill library as the pipeline's implementation. This claim requires more careful treatment. A CLAUDE.md skill file is a natural language instruction document. It specifies desired behavior but does not enforce it: a different LLM, a different context window, or a different session state could interpret the same instruction differently and produce different output. The pipeline's "invariance" across scenarios is behavioral consistency under a specific AI system (Claude Code, as named in the implementation section), not formal invariance under a specification.

This has implications for the replication claim. The paper states the pipeline is "reproducible from a public git repository with no proprietary dependencies." But the git repository contains natural language prompts, and the behavior of those prompts is determined by the underlying model, which is not fixed in the repository and which will change as the model is updated. A practitioner who clones the repository in 2028 and runs the skill library against a different Claude version may observe different gate pass rates, different failure distributions, and different world-build quality. The pipeline specification is open; the behavioral contract is model-specific. These are not the same thing, and the paper should be explicit about the distinction.

---

## Scores (1 = reject, 2 = major revision, 3 = minor revision, 4 = accept)

| Criterion | Score | Rationale |
|-----------|-------|-----------|
| **Novelty** | 3 | The multi-artifact creative pipeline with structured artifact handoffs and pre-hoc review gates is a genuine advance on prior single-artifact generation and agent-loop approaches. The specific framing of failure costs by stage is novel. Incremental relative to general pipeline architectures but novel in application. |
| **Soundness** | 2 | The failure analysis is sound. The main empirical claim (90.4% pass rate demonstrates pipeline value) lacks a comparison condition and cannot be isolated from gate calibration effects. The transferability claim conflates pipeline and domain-knowledge transfer. The replication claim is model-dependent in ways the paper does not acknowledge. |
| **Significance** | 3 | Cross-scenario empirical characterization of a multi-artifact generative pipeline is a meaningful methodological contribution to the AI and game design communities. The failure taxonomy (Table 3) and stage-level failure mapping are directly useful to practitioners. Significance is contingent on replication claim being appropriately qualified. |
| **Presentation** | 3 | Clear writing, well-organized stages, useful tables. The system description (Section 3) would benefit substantially from artifact schema examples. The pipeline diagram is a placeholder. The conflation of "implemented" with "specified in natural language" creates presentational ambiguity that runs through the paper. |

---

## Recommendation

**Major Revision.**

The paper makes a real contribution — the failure analysis and the cost-staging logic for gate placement are the kind of rigorous process knowledge the game design methodology field needs — but the empirical claims exceed the evidence as presented. The minimum requirements for acceptance are:

1. Artifact schemas (field-level specification of key markdown documents, particularly ROUNDS.md, PUZZLE-POOL.md, and ASSIGNMENT.md) either in the paper or as a clearly documented appendix/supplement.
2. A comparison condition or principled argument for why a comparison condition is unavailable. At minimum, the Stage 3 rejection rate across all scenarios should be reported so readers can assess whether the gate filtered candidates or merely ratified them.
3. Explicit acknowledgment that the replication claim is model-dependent and that behavioral invariance across scenarios is not the same as formal specification-level invariance.
4. Separate characterization of Stage 2 human effort for fictional versus factual domains, with implications for the domain transferability claim.

The decomposition-optimality question (Weakness 2) and the baseline absence (Weakness 4) are the two concerns I would most want to see addressed. The paper is worth revising and publishing; it is not ready to accept as written.

---

*Review prepared for FDG 2026 Program Committee.*
