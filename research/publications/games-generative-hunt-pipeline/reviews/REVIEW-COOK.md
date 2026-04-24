# Peer Review — FDG 2026

**Paper:** A Structured Generative Pipeline for Puzzle Hunt Creation: Empirical Characterization Across Five Domains

**Reviewer:** Michael Cook (Queen Mary University of London / ANGELINA project)

**Submitted:** February 2026

---

## Summary

This paper presents an 11-stage pipeline for generating puzzle hunts — multi-artifact creative systems consisting of thematically unified puzzles, a meta-puzzle, and a finished delivery package — using large language model-driven skill prompts. The core contribution is a pipeline specification with two formal review gates (Stage 3: pool ranking; Stage 7: editorial review) implemented as a 13-skill CLAUDE.md library. The system is empirically evaluated across five hunt scenarios spanning five content domains and three delivery formats, yielding 52 puzzles with a reported 90.4% first-pass pass rate. Key findings are that (1) failures concentrate at Stage 8 (integration) rather than at the review gates, (2) the pipeline transfers across domains without per-scenario code changes, and (3) a world-lock mechanism at Stage 2 effectively suppresses coherence drift across multi-stage generation.

The paper addresses a genuine gap: the coordination problem in multi-artifact creative generation is real, understudied, and meaningfully harder than single-artifact generation. The choice of puzzle hunts as a testbed is well-motivated — the domain provides verifiable answers, explicit practitioner quality standards, and a rich community of documented expertise. The pipeline specification itself is a concrete, reusable engineering contribution.

---

## Strengths

**1. The domain choice is excellent.**

Puzzle hunts are a near-ideal testbed for structured generative pipelines. Unlike most creative domains, they offer binary verifiability (an extraction chain either works or it does not), documented practitioner quality standards (the Riven Standard, craftsmanship criteria), and a constraint structure — feeder answers must compose with a meta-puzzle — that creates global correctness conditions spanning the full artifact set. The paper exploits all of these properties. The community-calibrated rubric, the world-lock mechanism, and the gate placement all make sense given the domain's structure. Researchers attempting to generalize this work to other domains will immediately recognize what they are giving up by leaving puzzle hunts: exactly this kind of structural grounding.

**2. The failure analysis is the paper's most valuable section.**

The failure taxonomy — mechanism failure, answer coordination failure, construction error, delivery gap — and the observation that Stage 7 missed the two Ironfall coordination failures because editorial review did not cross-reference META.md is genuinely useful. It turns a limitation into a specific, actionable design recommendation, and it demonstrates the kind of empirical honesty the field needs more of. The recommendation to add META.md cross-referencing to Stage 7 is the paper's best concrete contribution to the practitioner literature.

**3. The reactive growth of the skill library is worth more attention than it receives.**

The pipeline grew from 0 to 13 skills over two days, driven entirely by deficiencies discovered during scenario execution. The paper acknowledges this but frames it as a limitation to be disclosed rather than a finding to be analyzed. In the computational creativity literature, the pattern by which a design space reveals its own structure through iterative failure is precisely how design knowledge is built — this is the core of Schon's reflective practice, applied to AI pipeline design. The 13 skills that emerged are, by construction, a sufficient basis for multi-artifact hunt generation. That number, and the inflection points at which new skills became necessary, tells us something about the problem structure. The paper currently leaves this implicit.

**4. The cross-domain transferability claim is well-evidenced at the specification level.**

The commit history evidence — 142 commits, no per-scenario modifications to any skill file — is a clean empirical anchor for the transferability claim. This is the right kind of evidence: it is directly observable, falsifiable, and specific to the strongest version of the claim (specification-level transferability without prompt re-engineering). The paper is careful to distinguish this from implementation-level transferability, which is a distinction that matters.

---

## Weaknesses

**1. The paper conflates AI execution with AI creativity, and the framing should be corrected.**

The paper's language throughout — "AI-generated puzzles," "AI-authored content," "AI creative system" — implies a form of AI creative authorship that the system does not actually instantiate. Stage 6, the paper's most autonomous stage, operates from a fully specified brief (Stage 4), a locked world (Stage 2), and a committed answer word (Stage 5). The AI agent's task is to write a puzzle about domain-entity X that produces answer-word Y and is consistent with world document Z. This is sophisticated, constrained generation. It is not creative authorship in any sense that the computational creativity community would recognize as such.

The distinction matters for where this paper sits in the literature. If the claim is "AI can reliably execute well-specified creative tasks at quality," that is a clean, defensible contribution. If the claim is "AI is a creative agent in puzzle hunt design," that requires a significantly different argument — one about the AI's role in setting the goals, selecting the constraints, and evaluating its own outputs against internally generated criteria. The pipeline, by design, has the human setting scope (Stage 1), approving meta design (Stage 5), overriding panel verdicts, and performing the final creative shaping (Stage 11). The AI executes against those human decisions. Calling this "AI creative system generation" overstates the AI's creative role and understates the human's.

I would ask the authors to sharpen the framing: what is AI doing here that constitutes creative work, specifically? If the answer is "none of it — this is AI-assisted human creative production," then say that clearly and own the contribution on those terms. It is still a significant contribution. If the answer is "the AI is making creative decisions at Stage 3 and Stage 7 through panel evaluation," then that is worth arguing for explicitly, with a theory of what makes panel evaluation creative rather than computational.

**2. Quality is defined entirely within the pipeline's own rubric, with no external validation.**

The 90.4% first-pass pass rate is defined against the AI expert panel rubric — a rubric the pipeline itself deployed and whose calibration is the subject of a not-yet-complete companion paper. The paper acknowledges this limitation in Section 5.4, but the acknowledgment undersells the problem. The companion calibration paper is cited as in preparation; without it, the central quality claim of this paper — that the pipeline produces high-quality puzzles — rests on a quality criterion that has not been externally validated.

Do human players actually enjoy these hunts? Are the puzzles fair? Do solvers experience the "aha moment" that the practitioner literature identifies as the hallmark of quality puzzle design? The paper cannot answer these questions. The Stage 10 solver simulation is AI-simulated, not human-tested. The Stage 7 editorial panel is AI-staffed. The rubric criteria (Clarity, Solvability, Elegance, Reading Reward, Fun, Confirmation) include "Fun" — a criterion that can only be validated by a human solver actually having fun, which has not been measured.

I do not think this is a fatal weakness, but the paper should be more forthcoming about what the 90.4% figure actually measures: AI panel approval rate against an AI-constructed rubric, not solver quality. The authors should add a paragraph that explicitly separates these two things and notes that the former is reported here and the latter awaits the companion paper.

**3. The scale ceiling is acknowledged but not analyzed.**

Section 5.4 acknowledges that the 19-puzzle maximum is untested at MIT Mystery Hunt scale (200+ puzzles). The limitation note is accurate but brief: "Pipeline behavior at that scale... is an open empirical question." This is insufficient. The interesting question is not just "does the pipeline scale?" but "why might it not scale, and which specific mechanism would fail first?"

Several distinct hypotheses are available: (a) meta-puzzle complexity grows super-linearly with feeder count, because the answer-coordination constraint space grows combinatorially; (b) world-lock coherence degrades as the world document grows large relative to the context window, causing late-stage generation to "forget" early canonical decisions; (c) the Stage 7 editorial panel loses consistency across 200+ individual puzzle reviews because reviewer profiles do not agree on quality at the margin; (d) the human bottleneck at Stage 11 becomes the binding constraint, not AI throughput. Each of these hypotheses has different implications for pipeline design and different mitigation strategies.

A paper about pipeline design should have a theory of its own scaling failure modes. The current treatment — "untested, future work" — leaves the most interesting question entirely open when it could at least be posed precisely.

**4. The maturity confound is not adequately addressed.**

Section 4.5 acknowledges that the Age of Empires scenario (first run, 83% first-pass) and Dead Reckoning (last run, 100%) bookend a maturation arc, but then correctly notes that "separating these effects is a direction for future controlled evaluation." The problem is that the aggregate 90.4% pass rate is calculated across this maturation arc, conflating early-pipeline behavior with mature-pipeline behavior. A reader trying to set expectations for deploying this pipeline for the first time will not know whether to expect 83% (what the first run achieved) or 100% (what the mature pipeline achieved).

The paper should either present the aggregate figure with an explicit caveat about the maturation confound, or stratify the analysis by pipeline maturity (runs 1-2 vs. runs 3-5). The current presentation of 90.4% as the headline quality claim implies a steady-state figure that the data do not support.

---

## Scores

| Criterion | Score (1--4) | Justification |
|-----------|-------------|---------------|
| **Novelty** | 3 | Multi-artifact creative pipeline with formal gates is a genuine contribution; the specific combination of world-lock, parallel authoring, and AI panel evaluation has not been reported in this form. Deducted one point because the creativity framing overstates novelty beyond what the system actually demonstrates. |
| **Soundness** | 2 | The core pipeline engineering is sound and the failure analysis is honest. The quality criterion conflation (AI panel approval rate as proxy for solver quality) is a significant methodological gap. The maturation confound in the aggregate pass rate weakens the primary empirical claim. |
| **Significance** | 3 | The pipeline specification and the domain transferability evidence are genuinely useful to the field. The failure taxonomy — specifically the Stage 8 META.md coordination gap — is an actionable finding. Deducted one point because significance for solver-facing quality claims requires the companion calibration paper that is not yet available. |
| **Presentation** | 3 | The paper is clearly written, well-organized, and the stage table and failure table are both useful. The autonomy gradient discussion is the clearest articulation of the AI/human division of labor. Deducted one point for the creativity/authorship language that recurs throughout and misrepresents what the system does. |

---

## Recommendation

**Major Revision.**

The paper has a legitimate contribution and the empirical work is real. My two primary requests for revision are:

First, revise the authorship and creativity framing throughout to accurately represent what the AI is doing. The current language ("AI-generated," "AI creative system," "AI authorship") is not wrong in the narrow sense but it is misleading in the context of a computational creativity venue. The system is a human-directed AI-assisted production pipeline. The AI executes under dense human-specified constraints. Name it as such, and claim the contribution on those grounds — which are stronger, not weaker, because they are honest.

Second, add a paragraph to Section 5.4 that distinguishes the quality criterion the paper measures (AI panel pass rate against an AI-constructed rubric) from the quality criterion the paper implicitly claims (puzzles that human solvers would enjoy and find fair). Make the dependency on the companion calibration paper explicit, and scope the paper's claims accordingly.

Two additional requests that would substantially strengthen the paper if feasible:

Third, add a scaling failure analysis to Section 5.4. Name the specific mechanisms by which the pipeline would be expected to degrade at 50, 100, and 200+ puzzles, and propose testable predictions. This would turn a limitation disclosure into a research agenda.

Fourth, either stratify the pass rate analysis by pipeline maturity (early vs. late runs) or acknowledge explicitly that the 90.4% aggregate figure reflects a pipeline in active development, not a stable system. A practitioner deploying this pipeline for the first time should be told to expect Age of Empires behavior (83%, high Stage 8 failure rate) rather than Dead Reckoning behavior (100%), until they have run the pipeline enough times to identify and fix their own first-generation bugs.

The paper is worth publishing. The puzzle hunt domain is a legitimate and underused testbed for multi-artifact generative systems, the engineering contribution is solid, and the failure analysis demonstrates the kind of empirical honesty that computational creativity research needs. The revisions I am requesting are about accuracy of framing and precision of claims — not about the work itself.
