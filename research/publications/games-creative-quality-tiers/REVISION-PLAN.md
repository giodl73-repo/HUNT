# Revision Plan — Round 1
**Paper:** Orientation Before Expertise: A Unified Tier Model of Creative Quality in AI Authoring and Reviewing
**Panel result:** PASS (avg 3.1/4) | **Status:** Major revisions required

---

## Overview

Four P1 (blocking) issues and five P2 (important) issues were raised. All P1 items are revisions of framing, scoping, and transparency — none require new experiments. The revision is achievable within a standard CHI revision window.

Total estimated addition: ~600–900 words of new text plus one rubric table (appendix).

---

## P1 Revisions (Required)

### R1 — Qualify universality claims to match evidence scope
**Raised by:** Churchill (P1.1), Gervás (P1.1)
**Priority:** High — affects abstract, introduction, and conclusion
**What to do:**

1. **Abstract, lines 26–34:** Replace "universal prompt design principle applicable wherever quality is defined by receiver experience" with "a prompt design principle empirically validated in puzzle-hunt design and theoretically predicted to generalize to domains where quality is receiver-defined; cross-domain validation is future work."

2. **Introduction, lines 61–68:** Revise the OLE description from "This is a universal prescription derivable from, but not limited to, puzzle-hunt design" to "This prescription is derived from and empirically grounded in puzzle-hunt design; its generalization to other domains follows from the theoretical account of receiver-defined quality and is specified as a falsifiable prediction in §5."

3. **Section 5.2 (Generalization Argument):** Add a heading note or first paragraph that explicitly labels the business-domain examples in Table 4 as "predicted OLE structure" — not validated structure. The table caption should read "Predicted OLE tier structure..." (currently reads "Predicted" — verify this is in the caption and not contradicted by surrounding text). Add one sentence: "These predictions have not been empirically validated; they constitute falsifiable targets for cross-domain replication studies."

4. **Conclusion, lines 33–47:** Add one qualifying sentence before the prediction list: "These predictions extend beyond the validated testbed; each represents a falsifiable target for future work, not a current finding."

**Effort:** Medium — targeted search-and-replace plus 2–3 new sentences. No new content.

---

### R2 — Add rubric definitions and threshold justification
**Raised by:** Lupfer (P1.2), Gervás (P2.1), Jordanous (P2.2)
**Priority:** High — gatekeeping for tier-designation validity
**What to do:**

1. **Create a supplementary table or appendix (Appendix A: Evaluation Rubrics)** listing:
   - Authoring rubric: all 9 dimensions, their point range (0–5 each), and their descriptions. State the source paper (paper6) and whether the rubric was pre-registered or post-hoc.
   - Reviewing rubric: all 5 dimensions, their point range (0–6 each), and their descriptions. State the source paper (paper1).

2. **Add a "Threshold Justification" paragraph** in §5.1 (Formalization) or §4 (Evidence), immediately after the first mention of the 35-point threshold: "The 35/45 threshold for 'publishable puzzle quality' was defined [prior to / independently of] this study as [definition source or rationale]. The threshold identifies the minimum score at which a puzzle is judged acceptable for inclusion in a public puzzle-hunt event by a panel of [N] experienced constructors. Tier 2's 'load-bearing' designation is specific to this threshold: at a lower threshold (e.g., 30/45), the Tier 1+Tier 2 minimum sufficient set claim would require re-evaluation."

3. **Add a cross-reference** in §4.1 and §4.2 to the appendix: "(see Appendix A for rubric dimensions and threshold derivation)."

**Effort:** Medium — one table, two paragraphs. Requires checking the source papers for rubric details.

---

### R3 — Acknowledge computational mechanism as unestablished
**Raised by:** Ikegami (P1.1), Gervás (P1.2)
**Priority:** High — affects theoretical integrity of the paper's main claim
**What to do:**

1. **Add a paragraph to §6.2** (after the current "theory of the other" paragraph), titled or introduced as "Computational account (speculative)":

   "The computational mechanism by which the orientation sentence produces this effect is not established by the current experiments. Three candidate mechanisms are consistent with the data: (a) distribution shift — the orientation sentence shifts the model's output distribution toward receiver-experience-relevant content before domain conventions organize the generation; (b) attractor competition — the orientation sentence introduces a competing constraint that prevents the domain-content attractor basin from dominating; (c) saliency weighting — the orientation sentence increases the attention weight assigned to receiver-perspective features in the domain content that follows. The current experimental design cannot distinguish among these mechanisms, as it varies prompt content but not the computational process by which content is integrated. An attention-visualization experiment or activation-steering intervention would be required to adjudicate. We adopt interpretation (a) as the working hypothesis because it is consistent with the phase-3 reconstruction curve, but we flag this as speculative."

2. **Add one sentence to §3.1** (Tier 1 definition), after the current mechanism description: "The computational basis for this mechanism is a matter of current uncertainty; see §6.2 for discussion."

**Effort:** Low-medium — one new paragraph in §6.2, one sentence in §3.1.

---

### R4 — Reframe CC-theory connections as structural analogues, not derivations
**Raised by:** Jordanous (P1.1)
**Priority:** Medium-high — affects the theoretical contribution claim
**What to do:**

1. **Revise §2.2 opening** to state clearly that the tier model was derived empirically and then found to be structurally analogous to prior theoretical accounts: "The tier model was identified empirically before these analogues were drawn; we present the analogues not as sources from which the model was derived but as independent theoretical convergences that support its plausibility."

2. **Revise the Csikszentmihalyi paragraph** in §2.2: Replace "Our tier model maps onto this tripartite structure" with "Our tier model is structurally analogous to this tripartite structure. The analogy is post-hoc: the tier model was derived from ablation data, not from Csikszentmihalyi's framework. The convergence is nonetheless theoretically informative because it suggests the tier structure reflects a general architecture of creative quality that has been independently identified in both empirical AI studies and humanistic creativity research."

3. **Revise §3 (Tier Model introduction)** to remove any language implying the tiers were derived from prior theory: "identified empirically across authoring and reviewing roles and formalized here as a generalizable architecture" is correct; ensure no earlier sentence implies the structure was deduced from theory.

**Effort:** Low — targeted revisions in §2.2 and §3. No new content.

---

## P2 Revisions (Recommended)

### R5 — Specify deployment model (system prompt vs. user prompt)
**Raised by:** Churchill (P1.2), Lupfer (P1.1)
**Priority:** Medium
**What to do:** Add one paragraph to §5.1 or §6.3 specifying that OLE is primarily a protocol for system-prompt designers (the practitioner writing the AI system's initial instructions) rather than for end users constructing individual queries. Note that end-user applications require additional interface scaffolding to support orientation-first prompt construction, which is identified as a CST design direction in §7.

**Effort:** Low — one paragraph.

---

### R6 — Report measurement uncertainty for key comparisons
**Raised by:** Gervás (P2.2), Ikegami (P1.2)
**Priority:** Medium
**What to do:** For the three most consequential comparisons — C5 vs. C0 (21.7 vs. 23.7), L-T1 vs. AA (12.3 vs. 41.7), and T4-alone vs. A0 (26.0 vs. 24.0) — add a note on measurement confidence. If multiple evaluations per condition were run, report mean and standard deviation. If single-run, acknowledge this as a limitation and note that the L-T1 catastrophic failure result is large enough (29.4-point drop) that it is robust to reasonable noise levels, while the C5 near-baseline result (2.0-point drop) is smaller and should be treated as directional evidence rather than a decisive finding.

**Effort:** Low-medium — data retrieval and a short paragraph in §4 or limitations.

---

### R7 — Foreground non-monotonicity in abstract and introduction
**Raised by:** Lupfer (S3), Ikegami (S1)
**Priority:** Medium
**What to do:** Add one sentence to the abstract and one to the introduction stating the non-monotonicity finding explicitly: "Adding domain expertise without a prior orientation sentence actively degrades creative output below the no-context baseline in both roles." This sentence belongs in the abstract's second paragraph (after the tier summary) and in the introduction's fourth paragraph (alongside the L-T1 and C5 results).

**Effort:** Very low — two sentences.

---

### R8 — Address rubric-tier circularity in limitations
**Raised by:** Ikegami (P1.2), Gervás (P2.1)
**Priority:** Low-medium
**What to do:** Add one paragraph to §6.3 (Limitations, third bullet): "A fourth limitation concerns the relationship between the evaluation rubric and the tier structure. If the rubric was designed to capture solver-experience quality — as puzzle-hunt rubrics typically are — then prompts that activate experience-first orientation will score higher because the rubric rewards the same criterion that orientation activates. The tier structure would then partly reflect the rubric's internal priorities rather than an independent property of the AI system's behavior. We address this by noting that [the rubric was defined independently of the study / the tier structure replicates across two rubrics with different dimensional structures], but we acknowledge that external-criterion rubric validation across domains is required to fully distinguish rubric artifact from genuine system architecture."

**Effort:** Low — one paragraph.

---

### R9 — Expand multi-agent future work direction
**Raised by:** Lupfer (P2.2)
**Priority:** Low (optional for this revision)
**What to do:** If a preliminary two-agent experiment is feasible before the camera-ready deadline, include it. If not, expand the §7 multi-agent paragraph to include a concrete experimental design: "A minimal test of the multi-agent prediction would pair an OLE-structured authoring agent (orientation sentence + discipline gates + domain context) with an OLE-structured reviewing agent (philosophy component + three non-redundant principles + review lens) and compare the collaborative output quality to a domain-labeled pair (puzzle-designer author + puzzle-expert reviewer). The OLE pair predicts higher output quality and, specifically, a higher frequency of structural novelty (mechanism escape from the acrostic-indexing attractor family) than the domain-labeled pair. This is a straightforward extension of the current testbed and does not require new rubric development."

**Effort:** Low — one expanded paragraph in §7.

---

## Revision Checklist

| Item | Priority | Section | Effort | Status |
|------|----------|---------|--------|--------|
| R1: Qualify universality claims | P1 | Abstract, §1, §5.2, §7 | Medium | Pending |
| R2: Add rubric table + threshold justification | P1 | Appendix A, §4–5 | Medium | Pending |
| R3: Acknowledge computational mechanism | P1 | §6.2, §3.1 | Low-medium | Pending |
| R4: Reframe CC connections as post-hoc analogues | P1 | §2.2, §3 | Low | Pending |
| R5: Specify deployment model | P2 | §5.1 or §6 | Low | Pending |
| R6: Report measurement uncertainty | P2 | §4, §6.3 | Low-medium | Pending |
| R7: Foreground non-monotonicity | P2 | Abstract, §1 | Very low | Pending |
| R8: Address rubric circularity in limitations | P2 | §6.3 | Low | Pending |
| R9: Expand multi-agent future work | P2 optional | §7 | Low | Pending |

---

## Author Response Guidance

The panel response should address the following in order of reviewer concern:

**For Jordanous:** Confirm that the CC connections are being reframed as post-hoc analogues (R4) and that rubrics will be included (R2). Note that the paper does not claim to be a CC-theory paper — it is an empirical HCI/AI paper with CC-theoretical convergences.

**For Churchill:** Confirm that universality claims are being qualified (R1) and that the deployment model will be specified (R5). Acknowledge that cross-domain validation is future work and that OLE's iterative-process implications are a CST design direction identified for follow-on work.

**For Lupfer:** Confirm that rubrics will be included (R2), that the non-monotonicity result will be foregrounded (R7), and that the deployment model will be specified (R5). Note that system-level OLE implications (RAG, multi-turn) are acknowledged as beyond the current paper's scope.

**For Gervás:** This is the most skeptical review. Address directly: (a) the universality qualification (R1) — the generalization is a prediction, not a finding; (b) the rubric inclusion (R2); (c) the C5 vs. C0 measurement uncertainty (R6) — acknowledge the 2.0-point difference is small in absolute terms and characterize it as directional; (d) the computational mechanism (R3) — provide a speculative account. The key concession is R1: Gervás is correct that the generalization is not validated. Granting this clearly will resolve most of his concern.

**For Ikegami:** Provide the computational mechanism account (R3). Acknowledge the rubric circularity question (R8). The non-monotonicity framing aligns well with Ikegami's complex-systems interpretation — note in the response that the C7 < C6 result can be read as evidence of non-linear component coupling, which is consistent with attractor competition (the mechanism candidate from R3). This is not in the paper text but can be noted in the response to signal engagement with his perspective.
