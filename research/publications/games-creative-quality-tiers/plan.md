# Paper Plan: Orientation Before Expertise — A Unified Tier Model of Creative Quality in AI Authoring and Reviewing

## The Finding in One Sentence

A three-tier cognitive architecture — **orientation** (irreplaceable), **operational discipline** (load-bearing), **domain specificity** (amplifying) — governs quality in both AI creative authoring and reviewing roles, and domain expertise occupies only Tier 3 in both.

## Research Question

Is the structure of creative quality — what matters most, what matters second, what is merely additive — the same across authoring and reviewing roles? And if so, what does that tell us about how to design AI creative prompts?

## Core Argument

Prior work on AI creative systems treats domain expertise as the primary quality variable: you add a domain expert persona, you get better creative output or evaluation. Our three papers show the opposite structure. Across both authoring (Paper #6) and reviewing (Paper #1), the same three-tier hierarchy governs quality:

**Tier 1 — Irreplaceable: Orientation**
- Reviewing: Design Philosophy/Worldview. Remove it (C5 = lens without philosophy = 21.7/30) and quality drops *below the bare baseline* (C0 = 23.7/30).
- Authoring: Experience-first (T1). Remove it (L-T1) and the output loses its solver path entirely (12.3/45 vs. 41.7/45).
- The orientation is *not* domain knowledge — it is a stance toward the other person (solver, reader, user). "What does the person on the other end experience?"

**Tier 2 — Load-bearing: Lens**
- Reviewing: 3 non-redundant principles (Verify the Last Mile, Blame the Player, Snyder's Computer Test). These add unique signal beyond the full profile; the other 8 principles are redundant.
- Authoring: Elegance drive (T4) + Rigor/verification (T5). Removing either drops quality below the threshold; they are constraint gates.
- Operational discipline operationalizes the orientation into checkable tests and constraints.

**Tier 3 — Amplifying: Domain Specificity**
- Reviewing: Review Lens + Credentials/Authority. The lens adds defect-detection specificity; credentials calibrate conservatism.
- Authoring: Surprise (T2), Visual thinking (T3), Systematic thinking (T6) + domain world data.
- Domain expertise lives here. It is necessary but substitutable and never sufficient.

## The Practical Implication

Every AI creative system that leads with domain expertise (a "puzzle designer" persona, a "marketing expert" persona, a "legal analyst" persona) is starting at Tier 3. The correct order is:

1. **Inject orientation first**: "Think first about how the other person will experience this."
2. **Add operational discipline**: "Remove anything unnecessary. Verify every claim."
3. **Provide domain content**: world data, style guides, precedents as context — not as persona identity.

This is the **OLE formula** (Orientation → Lens → Expertise) for AI creative quality prompts.

## Target Venue

- **Primary**: CHI 2026 (ACM Conference on Human Factors in Computing Systems)
  - CHI values: practical guidance for AI systems, human-centered design, empirical studies of AI interaction
  - The OLE formula is directly actionable for practitioners
  - Deadline: September 2025 (already passed) → CHI 2027 or submit to CSCW 2026 (April deadline)
- **Secondary**: NeurIPS 2026 Creativity and Design Workshop (September)
- **Alternative**: AAAI 2027 (October)
- **Best fit given timing**: ICCC 2026 as a synthesis/position paper, or submit to ACM TOCHI (Transactions on Computer-Human Interaction) for the journal version

## Evidence Base

All empirical evidence is in hand — no new experiments needed.

### From Paper #1 (games-ai-expert-panel-creative) — Reviewing side
| Condition | Context | Score /30 |
|-----------|---------|-----------|
| C0 | Nothing | 23.7 |
| C4 | Philosophy only | **26.7** |
| C5 | Lens only (no philosophy) | **21.7** ← below bare baseline |
| C6 | Full profile | 25.3 |
| C3 | Full principles | 26.7 |
| C7 | Full profile + all principles | 22.3 ← worse than C6 |
| C8 | Full profile + 3 non-redundant | (best condition) |

Non-redundant principles: Verify the Last Mile, Blame the Player, Snyder's Computer Test
Redundant: Solving=Proving Understanding, No Over-Scaffolding, The Riven Standard (8 more)

### From Paper #6 (games-persona-authoring-traits) — Authoring side
| Condition | Context | Score /45 |
|-----------|---------|-----------|
| AA | Full artist | 41.7 |
| L-T1 | Minus Experience-first | 12.3 ← catastrophic |
| L-T4 | Minus Elegance | 30.0 ← below threshold |
| L-T5 | Minus Rigor | 29.0 ← below threshold |
| L-T3 | Minus Visual thinking | 37.3 ← stays above |
| R3 | T1+T4+T5 only | 35.7 ← minimum sufficient |

### From Paper #3 (games-profile-taxonomy-creative) — Designer comparison
Katz (operational) A6 > A5 (+1.6): Tier 2 lens transfers well to authoring
Young (perceptual) A6 > A5 (+3.7): Tier 1 orientation unlocks Tier 3 domain specificity
Snyder (construction) A6 ≈ A5 (-0.7): Tier 1 and Tier 2 already merged in his profile

### Persona study (from Paper #3 / Paper #6 motivating study)
"You are an artist" (40.3) > "You are a puzzle designer" (24.7) by 15.6 points
"Artist" activates Tier 1 orientation; "puzzle designer" activates Tier 3 domain labels

## Sections

### 00-abstract (~250 words)
Problem, approach, findings (the symmetric tier structure), OLE formula, contribution.

### 01-introduction (~1000 words)
Hook: domain expertise is Tier 3, not Tier 1. The field's implicit assumption. Our synthesis.
The OLE formula as the practical contribution. Why this matters for any AI creative system.

### 02-related-work (~1000 words)
AI creative systems and persona prompting. Design cognition hierarchy (Simon, Lawson).
Expert knowledge structure (Dreyfus, Chi). Experience design (Norman, Hassenzahl).
Principle-based design methods (Rams, WCAG, Strunk & White).

### 03-the-tier-model (~1500 words)
The empirical case for each tier. Tables showing reviewing and authoring side by side.
The orientation tier: why worldview/philosophy is irreplaceable in both roles.
The lens tier: why operational tests > domain principles.
The specificity tier: where domain expertise actually lives.
The OLE formula formalized.

### 04-evidence (~1500 words)
Per-tier evidence from all three papers. Cross-role symmetry analysis.
The persona study as the natural experiment (artist vs. puzzle designer).
The "artist outperforms domain expert" finding as predicted by the tier model.
Quantitative: score deltas per tier per role.

### 05-the-ode-formula (~800 words)
The practical formulation: three sentences that constitute a minimum sufficient creative prompt.
Generalization argument: does OLE apply beyond puzzle design?
Business domains: marketing, product, legal, training — predicted tier structure.
Limits: when domain expertise is Tier 1 (technical correctness domains).

### 06-discussion (~600 words)
Why AI systems get this wrong (trained on domain-labeled outputs).
The orientation as the hardest thing to train (it requires a theory of the other).
The symmetry: the same architecture that makes a good reviewer makes a good author.
Implications for prompt engineering, AI assistant design, creative AI teams.

### 07-conclusion (~400 words)
The three-tier model as a generalizable structure.
OLE as the practical takeaway.
Open questions: cross-domain validation, multi-role systems.

## Figures

1. **The Tier Model**: Two-column diagram (Reviewing | Authoring) × Three rows (Orientation / Lens / Specificity) — the paper's key conceptual figure
2. **Evidence Heatmap**: Score delta table for both roles × tier components — shows irreplaceable/load-bearing/amplifying empirically
3. **OLE Formula Diagram**: Three concentric circles or a layered pyramid: Orientation (innermost) → Lens → Expertise (outermost)
4. **The Artist vs. Expert comparison**: Bar chart showing the 15.6-point gap + annotation explaining which tier each persona activates

## Module Position

This is Paper #7 and the module capstone. It synthesizes:
- Paper #1: reviewer-side evidence
- Paper #3: authoring-side (profile depth) + designer comparison
- Paper #6: authoring-side (trait decomposition)

And makes the claim that transcends the puzzle hunt testbed: **the OLE formula is a universal structure for creative quality prompts**, derivable from the empirical evidence but applicable anywhere humans need to produce or evaluate creative work with AI assistance.

## Related Papers

| Paper | Role in #7 |
|-------|-----------|
| games-ai-expert-panel-creative | Provides Tier 1-2-3 evidence for reviewing |
| games-profile-taxonomy-creative | Provides designer comparison (operational vs. perceptual lenses) |
| games-persona-authoring-traits | Provides Tier 1-2-3 evidence for authoring; the OLE minimum sufficient set |
| games-human-ai-calibration | Future: does OLE close the human-AI calibration gap on subjective dimensions? |
