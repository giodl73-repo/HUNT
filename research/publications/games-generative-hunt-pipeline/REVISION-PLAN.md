# Revision Plan — games-generative-hunt-pipeline

**Round:** 1 | **Average score:** 2.75/4 | **Gate:** PASS | **Consensus:** Major Revision
**Reviewers:** Nelson, Smith, Cook, Koenitz, Dormans

---

## P1 — Blocking (must address)

- [ ] **P1.1 Self-referential pass rate** — 5/5 reviewers
  The 90.4% pass rate measures AI-panel approval against an author-constructed rubric. It is not a community quality standard. Fix: add one sentence in §4.2 and the abstract clarifying "first-pass pass rate against the system's AI panel rubric." Same pattern as companion paper fix.
  **Target:** sections/00-abstract.tex, sections/04-empirical-characterization.tex

- [ ] **P1.2 Maturation confound** — 3/5 reviewers (Nelson, Cook, Dormans)
  90.4% blends an evolving pipeline (83% Scenario 1, 100% Scenario 5). A first-time deployer should expect early-run behavior, not mature-run behavior. Fix: add one sentence in §4.2 and §5.4 limitations noting that the aggregate spans a maturing system; deployers should expect lower first-run rates.
  **Target:** sections/04-empirical-characterization.tex, sections/05-discussion.tex

---

## P2 — Important (should address)

- [ ] **P2.1 No comparison baseline** — 3/5 reviewers (Smith, Cook, Dormans)
  The claim that structure explains quality is inferential without a baseline. Fix: add an explicit limitation statement: "We do not report a gate-disabled baseline condition; the causal attribution of pass rate to pipeline structure is inferential."
  **Target:** sections/05-discussion.tex

- [ ] **P2.2 Framing precision** — 2/5 reviewers (Nelson, Cook)
  "Generative pipeline" and "AI creative authorship" both overstate. Fix: change "generative" to "AI-assisted" in title and key uses; add one sentence clarifying that Stage 6 operates under dense human constraints (brief, world lock, answer word) — this is AI execution under specification, not open-ended creativity.
  **Target:** main.tex (title), sections/01-introduction.tex, sections/03-system-description.tex

- [ ] **P2.3 Player/solver perspective gap** — 3/5 reviewers (Smith, Koenitz, Cook)
  The paper reports production metrics but no player experience data. Fix: add to limitations: "All quality metrics are derived from AI panel review; correlation with solver enjoyment, engagement, and perceived difficulty is studied in companion work \cite{dellaLibera2025calibration}. The pipeline optimizes for panel-pass quality, which is a proxy for but not identical to player experience."
  **Target:** sections/05-discussion.tex

- [ ] **P2.4 Domain transferability scope** — 3/5 reviewers (Smith, Cook, Dormans)
  Five puzzle hunts are five themes, not five domains. Fix: change "cross-domain" to "cross-theme" or "cross-setting" throughout; add one sentence noting that structural domain transferability (to tabletop RPGs, interactive fiction) is an open question.
  **Target:** sections/01-introduction.tex, sections/04-empirical-characterization.tex, sections/06-conclusion.tex

- [ ] **P2.5 Timing caveat consistency** — 2/5 reviewers (Nelson, Dormans)
  "3h17m" appears in Abstract and Introduction unqualified despite a Limitations caveat. Fix: add "(elapsed, not active human effort)" or similar qualifier at first use in abstract and introduction.
  **Target:** sections/00-abstract.tex, sections/01-introduction.tex

---

## P3 — Nice to Have

- [ ] **P3.1 Autonomy gradient expanded** — 2/5 reviewers (Smith, Koenitz): one paragraph on what the gradient means for designer agency
- [ ] **P3.2 Scale failure modes** — 1/5 reviewers (Cook): name what would fail first at 50+ puzzles
- [ ] **P3.3 Artifact schemas** — 1/5 reviewer (Dormans): one table showing field-level specification of key artifacts
- [ ] **P3.4 Stage 3 rejection rate** — 1/5 reviewer (Dormans): report what % of pool candidates were rejected at Stage 3
- [ ] **P3.5 World lock creative cost** — 1/5 reviewer (Smith): one sentence acknowledging the creative cost of locking

---

## Summary

The paper's three genuine contributions — gate cost-staging, world lock mechanism, and failure taxonomy — survive scrutiny across all reviewers. The P1 and P2 fixes are all 1-3 sentences each; no new experiments required. The pattern is identical to Paper 1: scope quantitative claims within their actual evidence base, fix overclaims, add consistent caveats. ~90 minutes of revision work.

**Path to acceptance:** P1.1 + P1.2 + P2.3-P2.5 are the minimum set. P2.1-P2.2 strengthen the paper. P3s are optional.
