# Round 1 Recheck Synthesis — games-human-ai-calibration
**Round:** 2 (Recheck) | **Reviewers:** Shneiderman, Bernstein, Kamar, Amershi, Heer
**Average score:** 3.0/4 | **Gate:** PASS | **Consensus:** Accept with Minor Camera-Ready Corrections

---

## Score Summary

| Reviewer | Novelty | Soundness | Significance | Presentation | Avg | Rec |
|---|---|---|---|---|---|---|
| Shneiderman (UMD / HCAI) | 3 | 3 | 3 | 3 | 3.0 | Weak Accept |
| Bernstein (Stanford / crowdsourcing) | 3 | 3 | 3 | 3 | 3.0 | Weak Accept |
| Kamar (MSR / complementarity) | 3 | 3 | 3 | 3 | 3.0 | Accept |
| Amershi (MSR / interactive ML) | 3 | 3 | 3 | 3 | 3.0 | Accept |
| Heer (UW / visualization) | 3 | 3 | 3 | 3 | 3.0 | Weak Accept |
| **Average** | **3.0** | **3.0** | **3.0** | **3.0** | **3.0** | |

**Gate check:** avg 3.0/4 ≥ 2.5 ✓ | no score < 2/4 ✓ → **PASS**

---

## What the Revision Resolved

All five reviewers confirm the following concerns from round 1 are satisfied:

**P1.1 (Figures)**: All five figures are present and well-designed. Specific notes across reviewers: the forest plot (Fig 3) is correctly formatted with the zero reference line; the v1/v2 grouped bar chart (Fig 4) makes the Fun null result visually stark; Figure 5's dual-line design (OLS regression + constant bias correction overlaid) is specifically praised by Bernstein, Amershi, and Heer as the right design choice for communicating why the constant correction is more honest than the slope model at N=10.

**P1.2 (Corrective factor reframing)**: Applied consistently throughout abstract, results, discussion, and conclusion. The mean-shift bias correction language (+0.83 Fun, +0.58 Elegance) is used uniformly. OLS parameters retained in Table 5 labeled "for reference only" with explicit statement that slope correction is unreliable at N=10. Borderline zone guidance (20–22/30) replaces the point estimate. All reviewers accept this reframing as appropriate.

**P2.1 (Calibration lifecycle)**: Kamar and Amershi both confirm the §5.4 addition is substantive. The three recalibration triggers and the N≈20–30 quantification are present. The v1→v2 motivating example is correctly used. Resolved.

**P2.2 (Threshold)**: Borderline zone framing accepted by Kamar and Heer. Heer notes it falls short of a full decision-theoretic analysis with false-negative rates but does not require further revision.

**P2.3 (IRB)**: Protocol number present; CHI requirement met. Confirmed by all reviewers who checked.

**P2.4 (Hypothesis)**: "A Generalizable Hypothesis" subsection header; multi-domain replication requirement stated. Confirmed by Shneiderman as correctly resolved.

**P2.5 (Range-selection bias)**: Present in §5.4 Limitations with directional qualifier. Confirmed by Bernstein as appropriately placed and framed.

---

## Camera-Ready Notes (shared, not blocking)

**Figure 5 data provenance** (Shneiderman, Bernstein, Heer): The per-puzzle per-dimension scatter points in Figure 5 come from individual reviewer scoring records, but the paper's main tables report only panel means. A one-sentence caption addition clarifying provenance is expected for camera-ready.

**LOO-CV R² uncertainty** (Heer): The LOO-CV R² values in Table 5 are point estimates from N=10 with high variance; a note acknowledging their instability would be honest. One sentence or footnote.

**Analysis scripts** (Bernstein, Amershi): Committed to release at "[URL]". Must actually be released on acceptance.

---

## Verdict

**Gate PASS.** Average 3.0/4, no score below 2/4. Two reviewers (Kamar, Amershi) give outright Accept; three give Weak Accept citing only camera-ready items.

The paper is ready to advance to the `ready` stage pending panel review.
