# Review: Orientation Before Expertise: A Unified Tier Model of Creative Quality in AI Authoring and Reviewing
**Reviewer:** Nic Lupfer | **Expertise:** Creativity support tools, creative AI systems, HCI | **Round:** 1

## Scores (1-4)
Novelty: 3 | Rigor: 3 | Clarity: 3 | Significance: 3 | **Overall: 3.0**

## Overall Assessment
The paper makes a concrete, positive contribution to understanding how AI creative systems should be structured and prompted, and the OLE formula is a useful design principle for creativity support tool builders. The core finding — that orientation is categorically irreplaceable while expertise is categorically amplifying — is well-supported by the ablation evidence. What the paper does not do is connect the tier model to system design implications: it tells CST designers what a good prompt looks like, but not what a good system looks like when OLE is the organizing principle. That gap limits the paper's contribution to a prompt-level prescription that stops short of system design guidance.

## Strengths
1. **The ablation evidence directly addresses a real CST design assumption.** Many creativity support tools are designed around domain scaffolding (templates, genre conventions, persona assignment). The finding that domain scaffolding without orientation degrades below baseline is directly relevant to these systems and motivates a design intervention.
2. **The non-monotonicity result is the most CST-relevant finding.** The C7 result (full profile + all principles = 22.3, worse than full profile alone = 25.3) directly demonstrates that more domain content does not monotonically improve creative AI output. This has immediate implications for systems that offer users the ability to layer on domain knowledge, expertise libraries, or style guides without first establishing receiver orientation.
3. **Four falsifiable predictions in Section 4 are exactly right for this community.** Stating testable predictions at the outset and confirming them across both studies gives the paper a credibility structure that CST research often lacks.
4. **The "what OLE is for" framing in §5.1 is clear.** The distinction between providing expertise as context versus as persona identity is actionable for system designers deciding how to structure knowledge inputs.

## P1 — Blocking Issues

1. **The paper does not address how OLE changes system design, only prompt design.** The OLE formula is a prompt-construction protocol, and the paper treats the system boundary as the prompt. But CST tools have system architectures — they include memory, context windows, user interaction models, knowledge bases, retrieval mechanisms, and output scaffolds. The paper's implication is that a CST system should inject the orientation sentence before all other context, but it does not address: (a) how OLE interacts with retrieval-augmented generation, where domain context is injected automatically and may precede any orientation signal; (b) how OLE applies to multi-turn interactions, where orientation must be maintained or re-activated across turns; (c) whether OLE changes the optimal placement of user-provided domain knowledge (before or after the model's orientation activation). Without addressing at least one of these, the paper's contribution to "how we should build creativity support systems" is limited to "structure your initial system prompt this way," which is a practitioner note rather than a system design contribution.

2. **The evaluation rubrics are never defined in this paper.** The authoring rubric (45 points, nine dimensions) and reviewing rubric (30 points, five dimensions) are the foundation of every quantitative claim, but neither is presented here. The paper references three prior papers (paper1, paper3, paper6) but cannot assume CHI reviewers have read them. Without at least a summary rubric table, the reader cannot assess whether the quality threshold (35/45) is principled or arbitrary, whether the dimensional weightings favor the tier model, or whether alternative rubric structures would change the tier assignments. This is a minimum-bar transparency requirement.

## P2 — Important Issues

1. **The paper does not discuss failure modes of OLE.** Under what conditions would a practitioner apply OLE correctly and still get poor output? The boundary condition discussion (§5.3: formal logic, mathematical proof) covers cases where OLE does not apply, but not cases where OLE is applied and fails. Understanding the failure modes of the formula is essential for system designers who need to know when to add additional mechanisms beyond OLE.

2. **The multi-agent direction in §7 (Conclusion) is underdeveloped relative to its importance.** The prediction that OLE authoring agents and OLE reviewing agents in a feedback loop produce compounding quality improvements is the paper's most important system design implication, but it is presented as a single paragraph of future work. Given that the paper's evidence base already includes both authoring and reviewing, a preliminary two-agent experiment would substantially strengthen the paper's contribution and is feasible within the existing testbed.

## Recommendation
Accept with revisions. The empirical contribution is sound. The paper needs either (a) a section addressing OLE's system-level design implications beyond prompt structure, or (b) a clear scoping statement that the contribution is at the prompt level and that system-level implications are future work. Rubrics must be included or referenced in accessible form.
