# Synthesis — Round 1
**Paper:** Orientation Before Expertise: A Unified Tier Model of Creative Quality in AI Authoring and Reviewing
**Venue:** CHI 2026
**Panel avg:** 3.1/4 | **Gate:** PASS (avg 3.1 >= 2.5; no reviewer below 2.8 >= 2.0)

---

## Score Table

| Reviewer | Novelty | Rigor | Clarity | Significance | Overall |
|----------|---------|-------|---------|--------------|---------|
| Jordanous | 3 | 3 | 4 | 3 | **3.3** |
| Churchill | 3 | 3 | 4 | 4 | **3.5** |
| Lupfer | 3 | 3 | 3 | 3 | **3.0** |
| Gervás | 2 | 3 | 3 | 3 | **2.8** |
| Ikegami | 3 | 2 | 3 | 3 | **2.8** |
| **Panel avg** | **2.8** | **2.8** | **3.4** | **3.2** | **3.1** |

Gate: PASS. All reviewers above 2.0; panel average 3.1 clears 2.5 threshold.

---

## Consensus Strengths

1. **Ablation + reconstruction methodology.** All five reviewers accepted the leave-one-out and progressive-reconstruction design as the right empirical approach. The Phase 2 tier separation (T1 drop = 29.4; T4/T5 drops = 11–13; T3 drops = 4–8) and the Phase 3 threshold crossing at R3 are regarded as the paper's most credible findings (Jordanous, Gervás, Ikegami).

2. **Cross-role symmetry is the paper's strongest theoretical contribution.** Jordanous, Churchill, and Gervás all flag the structural analogy between L-T1 and C5 — both producing below-baseline performance — as the result that most distinguishes this paper from prior persona-prompting work. This is the finding the revision should protect and foreground.

3. **Non-monotonicity (C7 < C6; Tier 3 without Tier 1 < baseline) is an important practical result.** Lupfer and Ikegami both highlight this as the most directly actionable finding for CST designers: adding domain content degrades output when orientation is absent. This result is more surprising than the OLE ordering itself and deserves more prominence.

4. **OLE formula is clear, specific, and actionable.** Churchill, Lupfer, and Jordanous all note that the three-sentence prescription is unusually precise for a framework paper. Churchill calls it "immediately actionable" for CST designers. No reviewer disputes its clarity.

5. **Boundary condition discussion (§5.3) is appropriate.** All reviewers who addressed generalization (Churchill, Gervás, Ikegami) acknowledged that §5.3's distinction between receiver-defined and formally-defined quality domains is a meaningful and correctly placed constraint.

---

## P1 Items (Blocking — must be resolved before acceptance)

### P1-A: Universality claims are not validated beyond the testbed
**Raised by:** Churchill (P1.1), Gervás (P1.1)
**Current state:** Abstract, introduction, and conclusion assert that OLE is "a universal prompt design principle" and "cross-domain." Section 5.2 contains domain generalizations (marketing, product design, legal, training) but these are analogical extrapolations, not empirical findings. Section 7 correctly identifies cross-domain validation as future work but the preceding sections do not qualify the universality claim to match.
**Required fix:** Audit every universality assertion in the abstract, introduction, and conclusion. Replace unqualified universality claims with qualified predictions: "OLE is empirically validated in puzzle-hunt design and theoretically predicted to generalize to domains where quality is receiver-defined." Section 5.2's Table 4 should be relabeled as "predicted OLE structure" with an explicit note that cross-domain validation is deferred.
**Target sections:** Abstract (lines 26–34), §1 Introduction (lines 61–68), §5.2 Generalization Argument (throughout), §7 Conclusion (lines 33–47).

### P1-B: Evaluation rubrics are not reproduced
**Raised by:** Lupfer (P1.2), Gervás (P2.1), Jordanous (P2.2)
**Current state:** The 45-point authoring rubric (9 dimensions) and 30-point reviewing rubric (5 dimensions) are referenced but never defined. The quality threshold (35/45) is central to the tier-2 "load-bearing" designation but is not justified in this paper.
**Required fix:** Add a rubric appendix or supplementary table listing all rubric dimensions, their point values, and their weighting. Include a short paragraph justifying the 35/45 threshold (e.g., whether it was pre-registered, set by a panel of domain experts, or defined relative to a specific publication standard). If rubrics are published in the three prior papers, provide a clear cross-reference and include a summary table here.
**Target sections:** New Appendix or §4 (Evidence), §5.1 (Formalization).

### P1-C: Computational meaning of "orientation" is unspecified
**Raised by:** Ikegami (P1.1), Gervás (P1.2)
**Current state:** "Orientation" is used as a first-class theoretical construct throughout the paper, but its computational interpretation is not given. Section 6.2 describes orientation as "activating the capacity that is most latent," which is a psychological metaphor. Ikegami notes that three different computational mechanisms are consistent with the data (distribution shift, attractor competition, prompt-length effects) and the paper does not distinguish among them.
**Required fix:** Add a paragraph in §6.2 (or a new subsection) that (a) acknowledges the computational mechanism is not established by these experiments, (b) proposes at least one candidate mechanism (e.g., the orientation sentence shifts the model's output distribution by providing a receiver-experience framing prior to domain-content processing), and (c) specifies what experimental design would distinguish among candidate mechanisms. This does not require additional experiments — it requires intellectual honesty about what the data do and do not establish.
**Target sections:** §6.2 (The Orientation as the Hardest Thing to Train), §3.1 (Tier 1 definition).

### P1-D: CC-theory grounding is post-hoc vocabulary, not derivation
**Raised by:** Jordanous (P1.1)
**Current state:** The paper maps the tier model onto Csikszentmihalyi's tripartite structure (domain/field/worldview) and claims convergence with Hassenzahl, Dreyfus, and Norman. These are legitimate connections but the mapping is retrospective — the tier model was derived from ablation data and then mapped to theory. No prediction in the paper is derived from CC theory and then confirmed by data; the causal arrow runs the wrong way.
**Required fix:** Either (a) reframe the CC connections as "structural analogues" rather than "grounding" — the tier model converges with prior theory but is not derived from it — or (b) identify one prediction that follows from CC theory (e.g., from Csikszentmihalyi: a system given worldview without domain knowledge should outperform a system given domain knowledge without worldview) and show that the data confirm it. Option (a) requires only reframing language in §2 and §3; option (b) requires identifying whether any existing condition set tests a CC-theoretic prediction.
**Target sections:** §2.2 (Expertise Structure and Creative Cognition), §3 (Tier Model introduction), §1 Introduction (lines 28–33).

---

## P2 Items (Important — should be addressed; non-blocking)

### P2-A: Deployment model (system prompt vs. user prompt) unspecified
**Raised by:** Churchill (P1.2), Lupfer (P1.1)
**Required fix:** Add a paragraph in §5.1 or §6 specifying whether OLE is a protocol for system-prompt designers, end-user practitioners, or both. Address the interaction design implication: if OLE is for end users, what interface scaffolding supports orientation-first prompt construction? If it is for system designers, how should it interact with RAG or other context-injection mechanisms? This is a scoping clarification, not a research gap; it can be addressed in one paragraph.

### P2-B: Measurement uncertainty not reported
**Raised by:** Gervás (P2.2), Ikegami (implicitly via P1.2)
**Required fix:** Report inter-rater reliability, confidence intervals, or replication counts for key comparisons — especially the C5 vs. C0 difference (21.7 vs. 23.7, a 2.0-point difference on a 30-point scale) and the T3-only below-baseline results. These are the tier model's signature results; their validity depends on the measurement being reliable. If single-rater scoring was used, acknowledge this as a limitation and note what magnitude of effect would survive noise.

### P2-C: Non-monotonicity result under-foregrounded
**Raised by:** Lupfer (S3), Ikegami (S1)
**Required fix:** The C7 < C6 and Tier 3 without Tier 1 < baseline results are the tier model's most surprising and practically important findings — more surprising than the OLE ordering itself. They are currently reported in §4 but not foregrounded in the abstract or introduction. Add one sentence to the abstract and one sentence to the introduction that names the non-monotonicity finding explicitly: "adding domain expertise without orientation actively degrades creative output below the no-context baseline."

### P2-D: Multi-agent OLE preliminary experiment
**Raised by:** Lupfer (P2.2)
**Required fix:** The conclusion's multi-agent direction is the paper's most important system-level implication but is presented as one paragraph of future work. If a preliminary two-agent experiment (OLE-structured authoring agent + OLE-structured reviewing agent vs. domain-labeled pair) is feasible within the existing testbed, adding it would substantially strengthen the paper's CHI contribution. If it is not feasible before submission, the future-work section should expand this direction with a specific experimental design.

### P2-E: Rubric–tier circularity
**Raised by:** Ikegami (P1.2), Gervás (P2.1)
**Required fix:** Acknowledge in §6.3 (Limitations) that the tier structure could partly reflect the evaluation rubric's internal weighting rather than an independent property of the creative system. The strongest response is to note that the rubric was defined independently of the study design (if true) and that the reconstruction curve's threshold crossing is consistent with the rubric's publisher-facing standards. If the rubric was defined partly from study data, acknowledge this as a limitation.

---

## Recommendation

**Panel recommendation: Accept with major revisions.**

The paper's core empirical contribution (ablation + reconstruction across two creative roles; cross-role symmetry; non-monotonicity) is well-supported and novel enough for CHI 2026. The OLE formula is the clearest and most actionable practical output in this research space.

Four P1 items must be resolved. P1-A (universality qualification) and P1-B (rubric transparency) are revisions of framing and presentation — they require no new experiments. P1-C (computational mechanism) and P1-D (CC-theory grounding) require one to two paragraphs of honest reframing each. None of the P1 items require additional empirical work.

P2 items A (deployment model) and C (non-monotonicity prominence) are small targeted additions. P2-B (measurement uncertainty) is a transparency requirement. P2-D (multi-agent experiment) is desirable but optional. P2-E (rubric circularity) belongs in limitations.

A revision addressing P1-A through P1-D and P2-A through P2-C would produce a paper whose empirical claims match its theoretical scope and whose practical implications are clearly targeted at the CHI community's design practice concerns.
