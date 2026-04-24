# Paper #3 Section Outline
## A Taxonomy of Evaluative Frameworks + Authoring Profile Depth Study

**Title (working):** "Profile Depth and the Reviewing-to-Authoring Transfer Problem: Evidence from AI Expert Personas in Puzzle Hunt Design"

**Venue:** ICCC 2026
**Target length:** 8–10 pages
**Rubric:** /45 weighted (Elegance ×2, Reading Reward ×2, Riven Standard ×1)

---

## Core Claims (in order of importance)

1. Authoring profile depth affects output quality symmetrically to reviewing profile depth — the gradient shape mirrors Paper #1's reviewer ablation
2. Whether a reviewing profile transfers to authoring depends on lens type: operational lenses (Katz) transfer or improve; perceptual lenses (Young) fail when the format can't provide the required material; construction lenses (Snyder) are already authoring-compatible
3. Domain expertise alone (A2) adds negligible quality over bare baseline (A0) — context without framework doesn't help
4. The 6 emergent evaluative frameworks (a-ha economy, load-bearing test, world-model consistency, social fabric, perceptual-shift, visual grammar) are the qualitative contribution of richer profiles

---

## 00 — Abstract (~250 words)

- Problem: prior work shows reviewer profile depth improves evaluation quality; does the same hold for authoring?
- Study: 7-condition authoring ablation (A0–A6) + 3-designer reviewing-to-authoring transfer study (Snyder/Katz/Young × A5/A6)
- Findings: symmetric gradient confirmed; transfer depends on lens type (operational vs perceptual); domain expertise without framework adds nothing
- Contribution: symmetric profile effect + lens-type transfer model + 6-framework taxonomy

---

## 01 — Introduction (~900 words)

- Hook: the authoring/reviewing profile distinction. We know richer reviewer profiles produce better evaluations (Paper #1). Does the same hold when the profile guides creation rather than critique?
- The transfer question: when a reviewer profile is repurposed for authoring, does the expertise transfer? This depends on whether the reviewer's lens is generative (operational tests that invert to authoring constraints) or terminal (perceptual checks that require finished material to apply)
- Our approach: a 7-condition authoring ablation mirroring the Paper #1 reviewer ablation, extended with a 3-designer × 2-mode comparison
- Why puzzle hunt design: same testbed as Paper #1 — verifiable answers, documented expert community, domain data available
- Contributions:
  1. Symmetric profile effect: authoring quality gradient mirrors reviewing quality gradient
  2. Lens-type transfer model: operational vs perceptual vs construction-facing lenses
  3. The A2 inversion: domain expertise without framework hurts calibration
  4. 6-framework taxonomy as qualitative evidence of what richer profiles introduce

---

## 02 — Related Work (~900 words)

- AI reviewer profiles and profile depth effects (Paper #1; Berry et al.; Liu et al.)
- Persona prompting for creative generation (Park et al.; Dang et al.; Zhang et al.)
- The authoring/reviewing distinction in human creativity research (Weisberg; Csikszentmihalyi — creator vs. field)
- Puzzle hunt design as testbed (Paper #1 methodology section — inherit by citation)
- Key gap: no prior work studies whether reviewing expertise transfers to authoring, or identifies which properties of a profile determine its generativity

---

## 03 — Methodology (~1200 words)

### 3.1 Authoring Ablation Design
- 7 conditions (A0–A6), mirroring Paper #1's C0–C6 reviewer ablation
- Table: condition × context provided × parallel reviewer condition
- A0: bare ("you are a puzzle designer")
- A1: + task specification (domain, mechanism, answer, rubric)
- A2: + domain world data (AoE2 world files) — no design guidance
- A3: + all 20 design principles
- A4: + Snyder authoring philosophy (generative worldview)
- A5: + full Snyder authoring profile (construction-focused)
- A6: + Snyder C8 reviewing profile repurposed for authoring

### 3.2 Designer Comparison Design
- 3 designers (Snyder, Katz, Young) × 2 modes (A5 full authoring profile, A6 reviewer profile as author) = 6 conditions
- Authoring profiles: generative versions of each designer — how they start, signature moves, what they avoid
- Reviewing profiles: same C8 profiles used in Paper #1 evaluation

### 3.3 Profile Types: The Three Lens Categories
- **Construction-facing** (Snyder): lens questions are uniqueness and load-bearing tests — already generative, invert to authoring instructions directly
- **Operational** (Katz): lens questions are named testable operations (short-circuit check, Computer Test, backwards read) — invert cleanly to authoring constraints
- **Perceptual** (Young): lens questions require finished visual/experiential material (visual grammar consistency, layout hierarchy, visual resolution) — format-dependent, fail when format can't provide the required material

### 3.4 Puzzle Authoring Task
- Domain: AoE2 (world data locked)
- Mechanism: knowledge extraction (5 clues, first letter spells answer)
- Answer: TOWER (fixed across all conditions)
- Evaluation: C8 panel (Katz, Snyder, Young), /45 weighted rubric

### 3.5 Evaluation Rubric
- 7 dimensions: Clarity ×1, Solvability ×1, Elegance ×2, Reading Reward ×2, Fun ×1, Confirmation ×1, Riven Standard ×1
- Riven Standard: 1 = theme decorative, 5 = mechanism inseparable from domain
- Pass: panel average ≥ 33/45

---

## 04 — Results (~1500 words)

### 4.1 Authoring Ablation (A0–A6)
- Score table with gradient visualization
- Key findings:
  - A2 (domain data) ≈ A0 (bare): domain expertise without framework adds nothing
  - A4 (philosophy) ≈ A5 (full profile): worldview as powerful as construction checklist
  - A4/A5/A6 form clear top tier, all pass
  - Format ceiling: all A-series capped at Riven ≤ 3 (mechanism imported from hunt tradition, not native to AoE2)
- Mirror to Paper #1 reviewer ablation: figure showing parallel gradients

### 4.2 Designer Comparison (A5 vs A6 × 3 Designers)
- 3×2 score table
- Katz: A6 > A5 (+5.6 within session) — operational lens transfers and improves
- Snyder: A5 ≈ A6 (−0.7) — construction lens already authoring-compatible
- Young: A5 > A6 (−2.7) — perceptual lens fails in text-only format
- Qualitative analysis: which review questions transferred cleanly, which created friction
- The Katz finding: "Are the mechanisms varied enough?" and "Would I want to solve this?" translate directly to authoring constraints; Computer Test becomes item selection filter

### 4.3 The 6-Framework Taxonomy
- Brief exposition of 6 frameworks surfaced by richer profiles in Paper #1 C6 reviews
- Table: framework, originating profile, definition, example from review text
- Note: these frameworks appear in C6 reviews but not C0-C5 — qualitative evidence that profile depth introduces new evaluative language, not just more words
- Cross-reference to Paper #3 data: which frameworks appeared in A5/A6 reviewer notes?

---

## 05 — Discussion (~800 words)

### 5.1 The Symmetric Gradient
- Authoring profile depth produces the same non-monotonic gradient as reviewing profile depth
- The A2 inversion (domain data without framework ≤ bare) mirrors C2 < C1 in reviewer study
- Implication: the profile's framework matters more than its content — a worldview outperforms a data dump

### 5.2 The Lens-Type Transfer Model
- The key variable is not profile depth but lens type
- Operational lenses (Katz) are generative by construction — each test has an authoring-time analog
- Perceptual lenses (Young) are terminal — they require material that only exists after authoring is complete
- Construction lenses (Snyder) are already generative — the distinction between authoring and reviewing profiles collapses
- Implication for profile design: when building authoring profiles from reviewing profiles, preserve operational tests; replace perceptual checks with their generative precursors

### 5.3 The Format Ceiling and the Riven Standard
- All A-series puzzles cap at Riven ≤ 3 because the first-letter acrostic mechanism is format-imported
- The double-weighting of Elegance and Reading Reward in the /45 rubric rewards domain-native mechanisms
- P-series puzzles (one principle at a time) outscored the A-series on the weighted rubric despite less construction rigor — because some principles directly target domain-mechanism integration (Framing Is Structural, Reading Reward, Riven Standard itself)
- Implication: profile depth raises the floor; mechanism choice determines the ceiling

### 5.4 Limitations
- Single task, single domain, single answer word (TOWER)
- Format constrained to text acrostic — Young's lens would likely transfer in a visually-structured format
- A5/A6 profiles authored for this study, not validated against actual designer practice
- Calibration: evaluation sessions require same-session scoring for cross-condition comparisons

---

## 06 — Conclusion (~400 words)

- Restate: authoring profile depth produces symmetric quality effects to reviewing profile depth
- The lens-type transfer model is the novel contribution: operational > construction > perceptual for reviewing-to-authoring transfer
- Practical recommendation: for authoring profiles, use operational reviewing tests as the foundation; supplement with generative worldview; don't import perceptual checks into text-only formats
- The format ceiling problem motivates future work: what authoring profile unlocks mechanism novelty rather than clue quality within a fixed mechanism?
- That question — already partially answered by the artist persona finding — is the subject of companion paper (games-persona-authoring-traits)

---

## Figures

- **Figure 1**: A0–A6 authoring gradient vs C0–C6 reviewing gradient (parallel bar charts)
- **Figure 2**: 3×2 designer comparison heatmap (Snyder/Katz/Young × A5/A6, colored by score)
- **Figure 3**: 6-framework taxonomy diagram (frameworks, originating profiles, transfer domains)
- **Figure 4**: Lens-type diagram (operational/construction/perceptual × generativity axis)

## Tables

- **Table 1**: 7-condition authoring ablation (A0–A6 × scores × pass)
- **Table 2**: Designer comparison (3×2 with gap column and lens-type annotation)
- **Table 3**: 6-framework taxonomy reference (name, profile, definition, failure mode, example)

---

## Writing Order

1. Methodology (3.1–3.5) — most concrete, establishes the experiment
2. Results 4.1 (A0–A6) — core finding
3. Results 4.2 (designer comparison) — novel finding
4. Results 4.3 (taxonomy) — qualitative contribution
5. Discussion 5.1–5.3 — interpretation
6. Introduction + Abstract — write last
7. Conclusion

## Pre-Writing Data Needs

- [ ] Run A0–A6 + all 6 designer conditions in a SINGLE evaluation session (to fix calibration drift)
- [ ] Mine Paper #1 C6 review texts for 6-framework instances (for Section 4.3)
- [ ] Verify Katz/Young authoring profiles against their published work (for methodology credibility)
