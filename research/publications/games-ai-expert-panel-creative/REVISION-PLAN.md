# Revision Plan — games-ai-expert-panel-creative

**Round:** 1 | **Average score:** 2.84/4 | **Recommendation:** Major Revision
**Reviewers:** Liang, Bernstein, Wei, Khattab, Amershi

---

## P1 — Blocking (must address)

- [x] **P1.1 Self-referential validation loop** — 5/5 reviewers
  The rubric, tier assignments, pass thresholds, and profiles were all constructed by the same team. The 24.5pp gap measures agreement with the authors' framework, not community standards. Fix: either add minimal external calibration data OR scope all quantitative claims precisely as "within the authors' evaluative framework, not as community ground truth." The latter is sufficient for ICCC without running a new study.
  **Target:** sections/01-introduction.tex, 03-methodology.tex, 04-evaluation.tex, 06-conclusion.tex

- [x] **P1.2 "Feedback quality" claim not grounded** — 4/5 reviewers
  Two sub-issues: (a) "emergent" frameworks may be retrieved from training data, not from the profile; (b) "actionability" claim is not empirically supported. Fix: (a) hedge "emerged from the persona" to "appeared in v2 reviews but not in any explicit profile input" — careful not to claim model causation; (b) reframe "actionable" as "specific and repair-pointing" rather than claiming practitioner outcomes.
  **Target:** sections/01-introduction.tex, 05-discussion.tex, 06-conclusion.tex

- [x] **P1.3 Incomplete experiments as findings** — 4/5 reviewers
  Experiments 2–4 are described as "in progress" in the methodology but treated as established in the abstract and conclusion. Fix: either complete the experiments OR consistently mark those findings as "preliminary" / "pending full data." The rubric validation (Exp 4) is recomputed from existing data so it IS complete — clarify this.
  **Target:** sections/00-abstract.tex, 04-evaluation.tex, 06-conclusion.tex

---

## P2 — Important (should address)

- [x] **P2.1 Variance / replication** — 3/5 reviewers (Wei, Liang, Khattab)
  Single-run results at temperature 1.0. C3 ≈ C4 (26.7 each) could flip in a second run. Add: report score ranges from the 3-reviewer panels as a proxy for variance; acknowledge single-run limitation explicitly.
  **Target:** sections/03-methodology.tex, 04-evaluation.tex

- [x] **P2.2 Training-data confound** — 3/5 reviewers (Wei, Liang, Khattab)
  When the model applies "load-bearing test" attributed to Snyder, it may be retrieving his vocabulary from training data. Fix: add a paragraph noting this confound and the fictional-persona control experiment as future work that would disentangle it.
  **Target:** sections/05-discussion.tex

- [x] **P2.3 Calibration inversion warning placement** — 3/5 reviewers (Bernstein, Amershi, Liang)
  The most practically important finding is buried in discussion. Move a 2-sentence "practitioner warning" to the conclusion's practitioner recommendation section.
  **Target:** sections/06-conclusion.tex

- [x] **P2.4 C3 vs. C6 recommendation gap** — 2/5 reviewers (Amershi, Khattab)
  C3 (full principles, no profiles) has 0 wrong verdicts; C6 has 2. Paper recommends C6 without explaining why. Add: one sentence explaining that C6 produces richer qualitative feedback (named frameworks, actionable revisions) that C3 cannot produce even with better binary accuracy.
  **Target:** sections/05-discussion.tex

- [x] **P2.5 Framework emergence coding reliability** — 2/5 reviewers (Wei, Bernstein)
  How were the 6 emergent frameworks identified? Need: brief methodology for framework detection (was it manual? automated keyword matching? both?) and acknowledgment that author coding introduces bias.
  **Target:** sections/03-methodology.tex

- [x] **P2.6 Profile construction methodology** — 2/5 reviewers (Bernstein, Amershi)
  Profiles built from authors' reading of published work, not self-descriptions. Add: explicit acknowledgment in limitations that profiles synthesize documented positions, not direct expert input, and this limits simulation fidelity claims.
  **Target:** sections/05-discussion.tex

---

## P3 — Nice to Have

- [ ] **P3.1 CoT framing** (Wei) — The v2 review lens functions like chain-of-thought: externalizing the reasoning chain. Frame this explicitly to connect to a well-studied mechanism.

- [ ] **P3.2 Prompt optimization framing** (Khattab) — Profiles are tunable LLM program parameters. Frame C8 recommendation as "manual prompt compression toward maximum tier separation per token."

- [ ] **P3.3 Profile governance** (Amershi) — Add a paragraph on how to add/update profiles and what disclosures prevent confusion with named individuals' actual views.

- [ ] **P3.4 Token/line count** (Khattab) — Report profile token counts, not just line counts, for the C8 efficiency claim.

- [ ] **P3.5 Riven Standard variance** (Amershi) — Acknowledge that RS scores were author-inferred from review texts; a proper inter-rater reliability study is future work.

---

## Summary Assessment

The paper has 3 clear contributions that survive the critique: (1) the 8-condition profile ablation with its non-monotonic gradient finding, (2) the calibration inversion (C1 inverts the tier hierarchy — the most practically important and most novel finding), and (3) the Riven Standard as a scored dimension with demonstrably better tier separation. The P1 items all reduce to the same fix: **scope the quantitative claims more precisely**. The authors don't need new experiments — they need one paragraph in the methodology clearly stating that the evaluation framework is author-constructed and not calibrated against external ground truth, and correspondingly hedged language in the abstract and conclusion. This is a 2-3 hour revision, not a 3-month study.

**Path to acceptance:** Address P1.1 (claim scoping) + P1.3 (experiment status consistency). The others strengthen the paper but don't block acceptance at a creativity/HCI venue like ICCC.
