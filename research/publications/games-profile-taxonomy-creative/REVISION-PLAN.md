# Revision Plan — Round 1
**Paper**: Profile Depth and the Reviewing-to-Authoring Transfer Problem
**Round**: 1 → 2
**Date**: 2026-02-28
**Source**: SYNTHESIS.md (reviews/SYNTHESIS.md) — panel of 5 AI expert personas

> **Purpose**: This plan is based on AI-generated quality assessment feedback. Use it to strengthen your work, not as reviewer responses. These are suggestions for improvement, not requirements.

---

## Summary

The panel found the empirical foundation solid — the symmetric gradient and A2 domain-inversion results are well-controlled and credible — but identified five blocking issues centered on statistical grounding, rubric validity, the post-hoc character of the lens taxonomy, companion-paper dependencies, and underdeveloped CC theoretical positioning. No new experiments are required. All P1 items are addressable through text additions and restructuring.

## Simulated Feedback Panel

| # | AI Persona | Based On | Score | Assessment |
|---|------------|----------|-------|------------|
| 1 | Anna Jordanous | SPECS framework, CC evaluation | 2.75/4 | Major Revision — rubric lacks psychometric grounding |
| 2 | Tony Veale | Computational creativity, generativity | 2.50/4 | Major Revision — CC positioning underdeveloped |
| 3 | Pablo Gervás | Cognitive models, narrative generation | 3.25/4 | Minor Revision — cognitive framework not operationalized |
| 4 | James Lester | AI in games, narrative intelligence | 2.75/4 | Minor Revision — generalizability underspecified |
| 5 | Mark Riedl | Persona-based generation, CC evaluation | 3.25/4 | Minor Revision — companion paper dependency blocking |
| **Panel** | | | **2.90/4** | **Major Revision** |

---

## P1: Critical Enhancements (High Impact)

### P1.1: Add inter-rater reliability statistics and hedge small score differences
**Source**: Jordanous P1.1, Riedl P2.3
**Identified by**: Jordanous, Riedl
**Enhancement**:
- [ ] Compute and report per-dimension inter-rater reliability for the 13-puzzle evaluation (at minimum: per-dimension average pairwise disagreement; ideally Krippendorff's alpha or ICC)
- [ ] Add a "Range (min–max)" column to Tables 1 and 2 showing the spread across the three reviewers for each condition
- [ ] In Section 4.1, explicitly frame score differences below 1.0 point (A1 vs. A2 = 0.6; Snyder A5 vs. A6 = 0.7) as "within reviewer variance" — retain but hedge the language (e.g., "the near-identical scores suggest..." rather than "A2 demonstrates an inversion")
- [ ] Add a reliability note to Section 3.4 or 3.5 acknowledging the small-N evaluation design (N=3 reviewers, N=13 puzzles) and what level of precision is warranted for reported panel averages

**Target sections**: `sections/03-methodology.tex` (rubric/evaluation subsection); `sections/04-results.tex` (Tables 1 and 2, discussion text)

**Verify**: Every score difference below 1.0 point that carries theoretical weight is accompanied by range data or explicit uncertainty hedging. Tables include range columns.

---

### P1.2: Add rubric validity argument and connect quality claims to CC frameworks
**Source**: Jordanous P1.2, Veale P1.1
**Identified by**: Jordanous, Veale
**Enhancement**:
- [ ] Add a "Rubric Validity" paragraph in Section 3.4 that: (a) explains the rationale for double-weighting Elegance and Reading Reward (these dimensions are most sensitive to mechanism-domain integration, established in Paper I calibration); (b) connects at least two dimensions to established creativity criteria (Reading Reward → experiential novelty / P-creativity; Riven Standard → domain-specificity of creative contribution); (c) acknowledges that "quality" in this paper means craft quality within puzzle hunt conventions, not creative quality in a broad CC-theoretic sense
- [ ] Add 2-3 sentences in Section 5.1 connecting the symmetric gradient finding to one CC framework — minimum: frame the framework/content distinction as a structural property of how constrained generation systems engage open-ended tasks, positioning it relative to Boden's exploratory creativity (searching within a conceptual space vs. restructuring it)

**Target sections**: `sections/03-methodology.tex` (add rubric validity paragraph); `sections/05-discussion.tex` (add CC connection in Section 5.1)

**Verify**: Section 3.4 contains an explicit validity argument for the rubric structure with at least two CC-relevant mappings. Section 5.1 cites at least one named CC framework.

---

### P1.3: Reframe lens-type taxonomy as provisional; add profile-content classification procedure
**Source**: Jordanous P1.3, Veale P1.3, Gervás P1.1
**Identified by**: Jordanous, Veale, Gervás (three independent flags)
**Enhancement**:
- [ ] Global: replace unqualified "taxonomy" with "proposed taxonomy" or "preliminary three-category model" wherever the lens-type classification is presented as validated. Key locations: abstract (paragraph 3), Section 3.3 heading and body, Section 4.2 discussion, Section 5.2, conclusion paragraph 2
- [ ] Add a profile-content classification procedure to Section 3.3 that allows assigning lens type from profile content rather than from observed transfer outcomes. The procedure should: (1) tag profile statements along two axes — direction (applicable to finished artifact only vs. applicable during construction) and content type (declarative quality standard vs. procedural construction instruction); (2) map the three designers' profiles to the taxonomy using this tagging before results are known
- [ ] Add a sentence in Section 3.3 or Section 5.2 explicitly acknowledging that each category is represented by one exemplar designer and that the model requires validation with additional designers

**Target sections**: `sections/00-abstract.tex`; `sections/03-methodology.tex`; `sections/04-results.tex` (Section 4.2); `sections/05-discussion.tex` (Section 5.2); `sections/06-conclusion.tex`

**Verify**: No instance of "taxonomy" or "model" in the paper presents the three-category classification as confirmed. Section 3.3 contains a profile-content classification procedure that does not require observing transfer outcomes.

---

### P1.4: Resolve companion paper dependency in Section 4.3
**Source**: Riedl P1.3, Lester P2.2
**Identified by**: Riedl, Lester
**Enhancement**:
- [ ] Option A (preferred): Add a condensed cross-condition frequency table to Section 4.3 or as an appendix. Minimum data: for each of the six frameworks, indicate whether it appeared in C0–C5 reviews vs. C6 reviews (presence/absence or count). The six exemplar quotes are already present; what is missing is the cross-condition comparison that supports the "none appear in C0–C5" claim in Table 3's caption
- [ ] Option B (acceptable fallback): Rewrite Section 4.3 to position the six frameworks as interpretive vocabulary first introduced and illustrated in this study, with the companion paper cited for full frequency evidence. Change Table 3 caption from "None appear in C0–C5 reviews" to "First systematically documented in C6-depth reviews (full frequency analysis in companion paper)" — this is a weaker claim but one the paper can support independently

**Target sections**: `sections/04-results.tex` (Section 4.3 and Table 3 caption)

**Verify**: Section 4.3 makes no presence/absence or frequency claims that cannot be independently evaluated by a reader without access to the companion taxonomy paper.

---

### P1.5: Add computational creativity theoretical positioning
**Source**: Veale P1.1, P1.2; Gervás P1.1, P1.3; Jordanous P3.3
**Identified by**: Veale, Gervás, Jordanous (three independent flags)
**Enhancement**:
- [ ] Add a Section 5.5 (or extend Section 5.1) titled "Implications for Computational Creativity" with 3-4 paragraphs:
  1. Connect the symmetric gradient to CC theory — frame the framework/content distinction as a structural property of generative profiles: framework activates exploratory search within a conceptual space (Boden), while raw content without framework does not change the search space
  2. Address explicitly whether quality claims are about creativity or craft: the rubric measures craft quality within puzzle hunt conventions; the significance for CC is that profile structure (framework > content, worldview > checklist) likely governs generative quality across constrained creative tasks
  3. Connect the Young A6 forced-substitution result to CC constraint literature — cite Boden on transformational creativity (format constraint forced a reformulation that restructured the design space) and/or Stokes on creative constraints as constitutive of creative breakthroughs
  4. Optionally: connect the lens-type model to the generativity question in CC — what makes a representation generative vs. terminal? — as a structural contribution to CC system design
- [ ] Ensure that by the end of the discussion section, the paper has answered the implicit ICCC question: "what does this study add to our understanding of AI creative cognition?"

**Target sections**: `sections/05-discussion.tex` (new Section 5.5 or extended 5.1)

**Verify**: The discussion section contains at least 3 paragraphs using CC-theoretic vocabulary, naming specific CC frameworks, and positioning the paper within the ICCC theoretical conversation rather than the AI/NLP prompting literature alone.

---

## P2: Substantial Improvements (Medium Impact)

### P2.1: Acknowledge same-panel evaluation bias
**Source**: Jordanous P2.1, Lester P2.1
**Identified by**: Jordanous, Lester
**Enhancement**:
- [ ] Add a paragraph to the evaluation subsection of Section 3 acknowledging that Katz, Snyder, and Young served as both AI authoring personas and the C8 evaluation panel
- [ ] Discuss directional bias: do designers score their own authored puzzles systematically higher or lower on Elegance and Reading Reward? If any protocol steps mitigated this (blind scoring, randomized order), describe them
- [ ] If no mitigation was in place, note this as a threat to internal validity for conditions where reviewers scored their own authored outputs

**Target section**: `sections/03-methodology.tex`

**Verify**: The methodology section explicitly acknowledges the same-panel overlap and assesses its directional implications.

---

### P2.2: Describe the artist persona result inline
**Source**: Lester P2.2, Riedl P3.3
**Identified by**: Lester, Riedl
**Enhancement**:
- [ ] In Section 5.3 (format ceiling), add 3-5 sentences describing the artist persona: (1) what the persona was ("you are an artist" — a minimal two-word role declaration), (2) what mechanism it generated (damaged inscriptions with one missing letter per item corresponding to a domain-specific property of each AoE2 item), and (3) what rubric score it achieved, particularly the Riven Standard score that exceeded any A-series puzzle
- [ ] The companion paper citation should remain; this addition ensures the ceiling argument can be evaluated without accessing the companion paper

**Target section**: `sections/05-discussion.tex` (Section 5.3)

**Verify**: A reader without the companion paper can evaluate the format ceiling claim using evidence in this paper.

---

### P2.3: Motivate the pass threshold (33/45)
**Source**: Jordanous P2.4, Lester P2.3
**Identified by**: Jordanous, Lester
**Enhancement**:
- [ ] Add 1-2 sentences in Section 3.5 (or the rubric subsection) explaining how the 33/45 pass threshold was established: from Paper I calibration data, from practitioner consensus, or from study-specific construction logic
- [ ] If the threshold derives from Paper I, add a back-citation to that paper's calibration section

**Target section**: `sections/03-methodology.tex`

**Verify**: The pass threshold has a stated derivation that a reader can evaluate.

---

### P2.4: Connect knowing-that / knowing-how distinction to the lens taxonomy
**Source**: Gervás P1.2, Veale P2.3
**Identified by**: Gervás, Veale
**Enhancement**:
- [ ] Add a paragraph in Section 5.2 mapping the three lens types onto Polanyi's knowing-that / knowing-how distinction introduced in Section 2.3:
  - Construction lenses: reviewing and authoring profiles share the same procedural content (knowing-how) — the tests are already procedural in both directions
  - Operational lenses: each named evaluative check (knowing-that) inverts into an authoring-time procedural constraint (knowing-how) because the test is logically reversible
  - Perceptual lenses: purely declarative (knowing-that) — they describe desired states of finished artifacts and have no procedural analog when the format cannot supply the required medium
- [ ] This mapping completes the connection between Section 2.3's theoretical framework and the empirical lens-type results

**Target section**: `sections/05-discussion.tex` (Section 5.2)

**Verify**: Section 5.2 contains explicit reference to the knowing-that/knowing-how distinction from Section 2.3, with each lens type mapped to one or both sides of the distinction.

---

### P2.5: Add description of Figure 1 in the text
**Source**: Gervás P2.3
**Identified by**: Gervás
**Enhancement**:
- [ ] At the end of the symmetric-gradient paragraph in Section 4.1, add 1-2 sentences describing what Figure 1 shows: the superimposed authoring (A0–A6) and reviewing (C0–C6) gradient curves on a normalized quality scale, with the structural inflection points visible (early plateau, principle-introduction jump, high-conditions plateau)
- [ ] Ensure the figure caption is self-contained and matches the text description

**Target section**: `sections/04-results.tex` (end of Section 4.1)

**Verify**: Figure 1 is described in the text and its caption is self-contained.

---

### P2.6: Add score ranges to tables and hedge all sub-1.0-point differences
**Source**: Jordanous P2.2, Riedl P2.3
**Identified by**: Jordanous, Riedl
**Enhancement** (partially covered by P1.1 — coordinate):
- [ ] Add parenthetical min–max ranges in the text for the three key small differences: A1 vs. A2 (0.6 points), Snyder A5 vs. A6 (0.7 points), A4 vs. A5 (1.3 points)
- [ ] Where the text currently presents sub-1.0-point differences as confirmed effects, soften to "near-identical," "within reviewer variance," or "suggests rather than confirms"
- [ ] Do not remove the theoretical interpretation — keep the narrative explanation for each difference — but ensure the framing does not overstate the statistical confidence

**Target section**: `sections/04-results.tex`

**Verify**: All score differences below 1.0 point are accompanied by range data or hedging language. No sub-1-point difference is presented as a confirmed directional effect without qualification.

---

## P3: Refinements (Optional Polish)

### P3.1: Add exemplar clue comparison between A2 and A3 outputs
**Source**: Riedl P3.1
- [ ] Add a 2-3 row inline table or side-by-side excerpt in Section 4.1 showing representative clues from the A2 puzzle (Stone and Steel) alongside the parallel A3 clues, demonstrating "database rows vs. constructions" at the text level

**Target section**: `sections/04-results.tex`

---

### P3.2: Foreground the worldview ≈ full profile finding as a design principle
**Source**: Riedl P2.1
- [ ] Add a sentence in the conclusion explicitly framing the A4 ≈ A5 result as a design principle: when building AI authoring profiles, invest in worldview elicitation (where does the designer begin? what constitutes a finished artifact?) rather than checklist elaboration; the checklist contributes verification, not generation

**Target section**: `sections/06-conclusion.tex`

---

### P3.3: Add worked example for converting a perceptual reviewing profile to an authoring profile
**Source**: Lester P3.2
- [ ] Extend the practical guidance in Section 5.2 with a 3-step worked example using Young's narrative-grammar substitution: (1) original perceptual check, (2) underlying generative intent, (3) format-compatible authoring instruction. Make the conversion procedure actionable for practitioners

**Target section**: `sections/05-discussion.tex`

---

### P3.4: Add a profile statement classification note
**Source**: Gervás P3.3
- [ ] Add a footnote in Section 3.3 describing a simple tagging procedure for profile statements: prescriptive/procedural ("do X during construction") vs. evaluative/declarative ("assess whether X is true of the finished artifact"). This supports the lens-type claims with content evidence rather than inference from output quality alone

**Target section**: `sections/03-methodology.tex`

---

### P3.5: Map the six evaluative frameworks to CC theory
**Source**: Veale P2.2
- [ ] In Section 4.3, add brief parenthetical or footnote annotations connecting the six frameworks to CC concepts: A-ha economy (flow / experiential creativity); Load-bearing test (structural constraint in creative systems); World-model consistency (narrative coherence); Social fabric (collaborative creativity); Perceptual-shift (exploratory P-creativity); Visual grammar (form-meaning integration)

**Target section**: `sections/04-results.tex` (Section 4.3)

---

## Revision Timeline

| Day | Focus | Deliverable |
|-----|-------|-------------|
| 1–2 | P1.1 + P2.6 — Reliability statistics, range data, hedging | Tables 1 and 2 with ranges; reliability note in Section 3.4 |
| 3 | P1.2 — Rubric validity paragraph | Updated Section 3.4 |
| 4 | P1.3 — Taxonomy reframing + classification procedure | Updated Section 3.3; global search-replace |
| 5 | P1.4 — Companion paper dependency resolution | Updated Section 4.3 + Table 3 caption |
| 6–7 | P1.5 — CC positioning section | New Section 5.5 (or extended 5.1) |
| 8 | P2.1–P2.4 — Minor methodology and discussion fixes | Updated Sections 3 and 5.2 |
| 9 | P2.5–P2.6 + P3 items | Final polish |
| 10 | Full rebuild, self-review, check against SYNTHESIS.md | Camera-ready draft |

---

## Quality Checkpoints

- [ ] P1.1: Every sub-1.0-point difference carries range data or explicit hedging
- [ ] P1.2: Section 3.4 has rubric validity argument; Section 5 cites a CC framework
- [ ] P1.3: "Taxonomy" is consistently qualified as provisional; Section 3.3 has a classification procedure
- [ ] P1.4: Section 4.3 makes no companion-paper-only claims without independent support
- [ ] P1.5: Discussion section has a CC positioning section with named frameworks
- [ ] P2.1: Methodology section acknowledges same-panel evaluation overlap
- [ ] P2.2: Artist persona result is described inline in Section 5.3
- [ ] P2.3: Pass threshold derivation is stated in Section 3
- [ ] P2.4: Section 5.2 maps lens types to knowing-that / knowing-how
- [ ] P2.5: Figure 1 is described in the text
- [ ] Paper builds without LaTeX errors after all edits
- [ ] Claims are consistently supported by evidence present in this paper

---

*Based on AI-generated panel reviews. Adapt to your research goals and constraints.*
