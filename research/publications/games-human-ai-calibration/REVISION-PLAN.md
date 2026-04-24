# Revision Plan — games-human-ai-calibration

**Round:** 1 | **Average score:** 2.80/4 | **Gate:** PASS | **Consensus:** Major Revision
**Reviewers:** Shneiderman, Bernstein, Kamar, Amershi, Heer

---

## P1 — Blocking (must address)

- [x] **P1.1 Figures must be produced** — 5/5 reviewers (Heer primary, drops presentation to 2)
  All five key figures are described but absent. CHI requires them.
  Fix: produce the five figures and add `\includegraphics` calls + captions to results sections.
  **Target:** figures/ directory, sections/04-results.tex

- [x] **P1.2 Reframe corrective factors as bias corrections** — 3/5 reviewers (Bernstein, Shneiderman, Heer)
  Fun slope CI [0.09, 1.27] and LOO-CV R²=0.26 render per-puzzle slope correction unreliable at N=10.
  Fix: replace "practitioner-ready corrective scaling factors" with mean-shift bias corrections throughout.
  New framing: "AI Fun scores are biased low by ~0.83 points on average; lower the threshold or add ~0.83 to Fun scores for threshold comparison. Per-puzzle linear correction requires larger N."
  **Target:** sections/04-results.tex (§4.5), sections/05-discussion.tex (§5.3), sections/06-conclusion.tex

---

## P2 — Important (should address)

- [x] **P2.1 Calibration lifecycle** — 2/5 reviewers (Kamar, Amershi)
  Paper treats calibration as static; deployed systems need a maintenance protocol.
  Fix: add ~150 words in §5.3: calibration triggers (model update, profile change), data needed for recalibration, validity window. Reference v1→v2 drift as motivation.
  **Target:** sections/05-discussion.tex

- [x] **P2.2 Pass threshold: probabilistic, not point** — 2/5 reviewers (Heer, Kamar)
  "20–21/30" is a point estimate without uncertainty.
  Fix: replace with decision-theoretic framing — at threshold 20/30, expected false-negative rate drops from X% to Y%. Add 2 sentences or small table.
  **Target:** sections/04-results.tex (§4.5), sections/05-discussion.tex (§5.3)

- [x] **P2.3 IRB/ethics documentation** — 4/5 reviewers
  CHI requires ethical review documentation for human participant studies.
  Fix: add one paragraph in §3: "This study was conducted in accordance with [institution] IRB protocol; all participants provided informed consent prior to scoring."
  **Target:** sections/03-methodology.tex

- [x] **P2.4 "Generalizable principle" → "Generalizable hypothesis"** — 1/5 reviewers (Shneiderman)
  Single-domain study cannot establish a "general principle."
  Fix: add qualifier throughout; add explicit sentence noting multi-domain replication is needed.
  **Target:** sections/05-discussion.tex (§5.4), sections/06-conclusion.tex

- [x] **P2.5 Range-selection bias in limitations** — 1/5 reviewers (Bernstein)
  Puzzle selection to span the score range inflates r.
  Fix: add one sentence to §5.4 Limitations.
  **Target:** sections/05-discussion.tex

---

## P3 — Nice to Have

- [x] **P3.1 Per-reviewer AI-human agreement** (Amershi): one paragraph on per-reviewer vs. panel-mean AI-human agreement
- [x] **P3.2 Disagreement protocol** (Kamar, Shneiderman): add decision rule for specific AI-human disagreements
- [x] **P3.3 Analysis scripts** (Amershi, Heer): cite in results or note release with paper

---

## Summary

P1 items: figures (labor-intensive but essential for CHI) + corrective factor reframing (edit across 3 sections, no new analysis). P2 items: all 1–2 paragraph additions, no new experiments. Estimated revision effort: figures ~2h, P1.2 reframing ~1h, P2 items ~1h total.
