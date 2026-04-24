# Revision Plan — Paper #4 Same-Input Divergence Study
**Round:** 1 → 2
**Date:** 2026-02-28
**Target venue:** ICCC 2026
**Panel decision:** Major Revision
**Panel average:** 2.98/4
**Source:** SYNTHESIS.md Round 1

> **Purpose**: This plan translates the Round 1 panel synthesis into specific, actionable revision tasks. P1 items are blocking — both must be addressed before resubmission. P2 items must be addressed or explicitly justified in the response-to-reviewers document. P3 items are optional polish.

---

## Summary

The panel found genuine strengths (novel paradigm, clean central finding, excellent clarity, practical significance) but consistent weakness in rigor: N=2 inferential overclaims pervade the body text after the limitations section acknowledges the problem, the paper lacks CC theoretical grounding for an ICCC submission, the panel score comparison is confounded by a rubric change, and the "domain semantic gravity" claim lacks operationalization. Neither P1 issue requires new experiments — both are addressable through targeted writing revisions estimated at 400–600 words of new or revised text plus a hedge-language audit throughout Sections 4–6.

## Panel

| # | Reviewer | Expertise | Novelty | Rigor | Clarity | Significance | Overall |
|---|----------|-----------|---------|-------|---------|--------------|---------|
| 1 | Anna Jordanous | CC evaluation, SPECS framework | 3/4 | 2/4 | 3/4 | 3/4 | 2.8/4 |
| 2 | Julian Togelius | PCG, quality-diversity, expressive range | 3/4 | 2/4 | 4/4 | 3/4 | 3.0/4 |
| 3 | Pablo Gervás | CC, narrative generation, design cognition | 3/4 | 2/4 | 3/4 | 3/4 | 2.8/4 |
| 4 | Michael Cook | Computational game design, ANGELINA | 3/4 | 2/4 | 4/4 | 3/4 | 3.0/4 |
| 5 | Mark Riedl | AI narrative, story generation, CC | 3/4 | 2/4 | 4/4 | 4/4 | 3.3/4 |

---

## P1: Blocking Issues

### P1.1: Add CC Theoretical Grounding
**Source**: Synthesis P1.1
**Identified by**: Jordanous (primary), Gervás (secondary)
**Target sections**: `sections/02-related-work.tex`, `sections/05-discussion.tex`, `sections/00-abstract.tex`, `sections/01-introduction.tex`

**Enhancement**:
- [ ] Add new subsection to `sections/02-related-work.tex` (after Section 2.4, before gap statement) titled "Computational Creativity Evaluation and the Status of Divergence." ~200–300 words. Choose one of two defensible positions and argue it consistently:
  - **Option A (recommended):** Characterising pipeline reliability structure is a *prerequisite* for CC evaluation in the SPECS sense — you cannot assess whether a generator produces novel outputs relative to its prior productions without knowing which production decisions are stable vs. variable. Same-input divergence grounds CC evaluation; it does not claim that divergence is creativity.
  - **Option B:** Same-input divergence operationalizes a CC property (e.g., non-redundancy across productions, or per-decision surprise capacity) and connect to SPECS (Jordanous 2012).
- [ ] Revise first sentence of `sections/00-abstract.tex` to add CC-relevant hook consistent with chosen position.
- [ ] Add one sentence to the contributions list in `sections/01-introduction.tex` explicitly scoping the CC relevance: "We situate this paradigm within the CC research agenda as a prerequisite for reliable CC evaluation, not as a direct claim about the creativity of individual pipeline outputs."
- [ ] Add one sentence or short paragraph in `sections/05-discussion.tex` (end of Section 5.1 or new subsection) addressing whether specification-driven convergence implies reduced creativity or reduced stochasticity — and arguing these are distinguishable.

---

### P1.2: Hedge Inferential Language Throughout Body Text
**Source**: Synthesis P1.2
**Identified by**: Jordanous, Cook, Togelius, Gervás
**Target sections**: `sections/04-results.tex`, `sections/05-discussion.tex`, `sections/06-conclusion.tex`

**Enhancement**:
- [ ] Audit `sections/04-results.tex` for distributional/categorical overclaims. Required hedges:
  - Stability classifications in Table 2 summary: add footnote "Stable/variable classifications reflect agreement across two independent runs and should be treated as case-study observations pending N≥10 replication."
  - Section 4.6 panel score language: "the panel is measuring a quality dimension that is invariant" → "the panel appears to measure a quality dimension that is robust to the specific design choices in these two runs"
- [ ] Audit `sections/05-discussion.tex`. Required hedges:
  - Section 5.2: "domain semantic gravity is a tendency" → "domain semantic gravity is an observed pattern in this case study, consistent with a tendency to cluster in domain-salient semantic regions; replication is required to establish it as a general property"
  - Section 5.3: "the panel is measuring a quality dimension that is invariant across the arbitrary choices" → "in both runs, the panel's quality signal appears robust to the specific generative choices made at Levels 3–5, suggesting a quality dimension that may be invariant to those choices"
  - Section 5.1: "run once and trust" recommendations → qualify as provisional guidance supported by two runs in a single domain, to be confirmed with N≥5 replication before operational deployment
- [ ] Audit `sections/06-conclusion.tex`. Required hedges:
  - All practical recommendations: "run once and trust for explicitly specified decisions" → "in this pipeline and domain, explicitly specified decisions showed stable behaviour across both runs, suggesting a 'run once' strategy may be appropriate for such decisions"
  - "run multiple times and select for unspecified ones" → "unspecified decisions diverged substantially in both runs, consistent with a 'run and select' strategy for such decisions"

---

## P2: Important Issues

### P2.1: Clarify Mechanism of Specification-Drives-Convergence
**Source**: Synthesis P2.1
**Identified by**: Riedl (P1.1), Gervás (P1.1), Cook (implicit)
**Target section**: `sections/05-discussion.tex`

**Enhancement**:
- [ ] Add mechanism-distinction paragraph to Section 5.1 (Specification Drives Convergence). Distinguish three candidate mechanisms:
  - (a) **Specification-as-hard-filter**: outputs are checked against spec constraints and rejected if non-compliant (relevant for formally checkable properties: puzzle count, format)
  - (b) **Specification-as-distributional-prior**: spec conditioning narrows the model's conditional distribution at generation time (relevant for stylistic properties: narrator voice)
  - (c) **Specification-as-attention-anchor**: mentioned items are salient in the model's context and consequently stable; unmentioned items are left unconstrained
- [ ] Provide a brief argument for which mechanism is most consistent with the evidence. Note: narrator voice being stable alongside scale suggests (a) alone is insufficient; (b) or (c) must be operative. Acknowledge that distinguishing these mechanisms requires N=10 replication with varied specification formats (e.g., one run with full spec, one with spec randomized in order, one with spec items withheld incrementally).

---

### P2.2: Acknowledge Rubric Change as Confound in Panel Score Comparison
**Source**: Synthesis P2.2
**Identified by**: Jordanous (P2.2), Riedl (P2.3), Cook (P2.2)
**Target sections**: `sections/03-methodology.tex`, `sections/05-discussion.tex`, `sections/00-abstract.tex`

**Enhancement**:
- [ ] Add paragraph to Section 3.4 (Panel Score Comparison methodology) explicitly stating: (1) Hunt 1 = 6 dimensions, 30 pts, uniform weight; Hunt 2 = 7 dimensions, 45 pts, Elegance and Reading Reward double-weighted; (2) this rubric change is a confound for dimension-level comparisons; (3) the aggregate normalized comparison is more robust but is still affected by the reviewer-panel change (domain-matched Hunt 1 vs. fixed Katz/Snyder/Dana Hunt 2).
- [ ] Revise Section 5.3 to hedge the Elegance/Reading Reward uplift: change "consistent with a genuine mechanism-level improvement" to "consistent with, but not conclusively attributable to, a genuine mechanism-level improvement; the double-weighting of these dimensions in Hunt 2's rubric and the change in reviewer panel are confounds that cannot be disentangled from this case study alone."
- [ ] Revise `sections/00-abstract.tex`: "systematic +1.2-point uplift in Elegance and Reading Reward in the second run" → "observed +1.2-point uplift in Elegance and Reading Reward in the second run, partially confounded by a rubric weighting change."

---

### P2.3: Operationalize Domain Semantic Gravity and Add Null Model
**Source**: Synthesis P2.3
**Identified by**: Togelius (P2.3), Jordanous (P2.1), Gervás (P2.1)
**Target sections**: `sections/03-methodology.tex`, `sections/04-results.tex`

**Enhancement**:
- [ ] Add operationalization of "domain-adjacent" to Section 3.3 (Level 3 methodology). At minimum: "Two answer words from different runs are classified as domain-adjacent if both words belong to the same semantic category in the AoE2 world data taxonomy (units, technologies, maps, economy, unit commands, buildings/structures, civilization-specific bonuses)." Apply consistently to all five feeder slot pairs and confirm that (LOOM/FORGE), (ONAGER/SIEGE), (PATROL/MARCH) meet this criterion, and (CASTLE/FAITH) and (TOWER/RELIC) do not.
- [ ] Add null model argument to Section 4.3 (Level 3 results). Informal estimate procedure:
  1. Count total AoE2-relevant gameplay terms in world data files (estimate from civs.md, units.md, techs.md, maps.md, economy.md)
  2. Count terms per semantic category
  3. Under independent uniform sampling for each of 5 slots, compute expected number of same-category slot pairs
  4. Compare expected value to observed 3/5
  5. If 3/5 exceeds the null-model expected value at a non-trivial margin, the gravity claim is supported; otherwise, reframe as a description.

---

### P2.4: Compute and Report KL Divergence for Level 2
**Source**: Synthesis P2.4
**Identified by**: Togelius (P2.2), Cook (P2.2)
**Target section**: `sections/04-results.tex`

**Enhancement**:
- [ ] Align the two pool type taxonomies across Hunt 1 and Hunt 2 (some categories appear in one pool but not the other; merge or split as needed for a comparable joint taxonomy).
- [ ] Compute KL(H1 || H2) and KL(H2 || H1) over the aligned type distributions using the distribution data from `results/comparison-all-levels.md`.
- [ ] Report KL divergence value in Section 4.2 alongside Jaccard 0.11. If KL > 0.5 nats (the threshold from Table 1), state this explicitly and confirm the VARIABLE classification is supported by both metrics.

---

### P2.5: Address Causal Propagation of Variability Across Pipeline Stages
**Source**: Synthesis P2.5
**Identified by**: Cook (P1.2)
**Target section**: `sections/05-discussion.tex`

**Enhancement**:
- [ ] Add a paragraph to Section 5.1 or create new Section 5.5 noting that the five comparison levels are causally ordered, not parallel. Key content:
  - Level 3 (answer words) causally constrains Levels 4 and 5. Hunt 2's uniform 5-letter feeder words were shaped by the vowel-count meta mechanism requirement; the mechanism design and the word selection co-evolved.
  - Variability at upstream stages propagates to downstream stages as correlated variability, not independent additional variability.
  - Practical implication: specifying upstream decisions (particularly answer words or meta answer) has a *multiplicative* stabilizing effect because it constrains the downstream design space available to later pipeline stages. Front-loading specification toward causally upstream decisions is a stronger lever than front-loading toward downstream ones.

---

## P3: Optional Polish

- [ ] **P3.1** — Elevate pool organisation principle finding. Add named observation ("generative strategy divergence") for the domain-by-slot vs. mechanism-by-type divergence in `sections/04-results.tex` Section 4.2 and/or `sections/05-discussion.tex`.

- [ ] **P3.2** — Reframe Contribution 2 in `sections/01-introduction.tex` as a world-claim: "We establish that complete lexical divergence (Jaccard 0.00) and complete pass/fail convergence can coexist in a multi-stage AI creative pipeline."

- [ ] **P3.3** — Add 2–3 sentences specifying the N=10 replication protocol in `sections/06-conclusion.tex`: same SCOPE.md, same toolkit, automated Level 4 mechanism coding using Table 2 taxonomy, automated Level 3 domain-adjacency analysis using world-data category structure.

- [ ] **P3.4** — Consider title revision to foreground central finding: "Specification Drives Convergence: A Same-Input Divergence Study of AI Puzzle Hunt Generation."

- [ ] **P3.5** — In `sections/06-conclusion.tex`, either add one motivating sentence for the evaluative-language stability future direction or cut it.

---

## Revision Timeline

| Day | Focus | Deliverable |
|-----|-------|-------------|
| 1 | P1.1 — CC grounding section | New Section 2.5 draft |
| 2 | P1.1 — Abstract + Intro + Discussion touches | 3 targeted edits complete |
| 3 | P1.2 — Hedge audit Section 4 | All overclaims in Results hedged |
| 4 | P1.2 — Hedge audit Sections 5–6 | All overclaims in Discussion + Conclusion hedged |
| 5 | P2.1 — Mechanism distinction paragraph | New paragraph in Section 5.1 |
| 5 | P2.2 — Rubric confound acknowledgment | Section 3.4 paragraph + Abstract edit |
| 6 | P2.3 — Domain adjacency operationalization + null model | Additions to Sections 3.3 and 4.3 |
| 6 | P2.4 — KL divergence computation + report | KL value added to Section 4.2 |
| 7 | P2.5 — Causal propagation paragraph | New paragraph in Section 5.1 or 5.5 |
| 8 | P3 items (optional) | Polish revisions |
| 9 | Rebuild + co-author review | PDF review pass |
| 10 | Response-to-reviewers document | Submission-ready package |

---

## Sign-Off Checklist

Before resubmitting:

- [ ] CC grounding section added to Related Work (~200–300 words)
- [ ] All distributional/categorical overclaims hedged in Sections 4, 5, 6
- [ ] Table 2 footnote added (case-study qualification)
- [ ] Mechanism distinction paragraph added to Discussion Section 5.1
- [ ] Rubric confound acknowledged in Section 3.4
- [ ] Section 5.3 Elegance/Reading Reward uplift hedged
- [ ] Abstract revised to hedge "+1.2 uplift" claim
- [ ] Domain-adjacency operationalization added to Section 3.3
- [ ] Null model argument added to Section 4.3
- [ ] KL divergence computed and reported in Section 4.2
- [ ] Causal propagation paragraph added to Discussion
- [ ] Response-to-reviewers document prepared (with diff summary table)
- [ ] Co-author sign-off
- [ ] Final word/page count within ICCC limits

---

*Plan generated from Round 1 panel review synthesis. Adapt to research goals and constraints.*
