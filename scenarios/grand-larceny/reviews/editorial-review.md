# THE GRAND LARCENY — Editorial Review

Reviewer: Admin
Date: 2026-02-27

---

## P1 — The Police Report — The Methodical

**Brief compliance:** MATCH. Logic grid using alibi contradictions as specified.
**Factual accuracy:** 0 errors. All times, room numbers, and character details match world/ canon.
**Principles check:**
- Riven Standard: PASS — this IS police investigation work (cross-referencing alibis, evaluating evidence)
- No Over-Scaffolding: PASS — the worksheet provides the grid but the deduction is the solver's
- No Computation Without Deduction: PASS — multiple deduction steps (alibi gaps, route analysis, elimination)
- Interlock: PASS — Clues 1-5 interlock (Kessler's elimination depends on understanding the escape route)
- Snyder's Computer Test: PASS — the key deduction (Kessler's return to his room contradicts the thief's escape route) requires inference, not just pattern matching
**Length:** OK. Dense but appropriate for a police report.
**Voice:** PASS. Bureaucratic, terse, formal. Reads like an actual incident report.
**Extraction:** VERIFIED. Seven underlined words with boxed letters spell the answer. However, the specific words need to be marked clearly in the print layout — in markdown, "underlined words with one letter in a box" is hard to render. The print version must use bold, underline, or color to distinguish the extraction letters.

**Verdict:** APPROVE WITH NOTES

**Required fixes:**
1. The extraction mechanism (finding underlined words) needs clearer visual treatment in the print layout. In the markdown source, add explicit markers or formatting notes.

**Editor notes:**
Strong puzzle. The critical deduction (Kessler went to his room at 9:28, so he could not have exited via the basement alley) is elegant and fair. The solver feels like a real detective. The Methodical delivered exactly what the brief asked for.

---

## P2 — The Hotel Receipt — The Social

**Brief compliance:** MATCH. Cents-value extraction from gap-period charges as specified.
**Factual accuracy:** 0 errors. All prices match services.md rate tables. Phone charge math is correct ($0.45 x 7 = $3.15). All subtotals verified:
- Folio 202: $18.62 + $12.00 + $0.10 = $30.72 CORRECT
- Folio 203: $14.03 + $12.00 + $0.80 = $26.83 CORRECT
- Folio 204: $15.80 + $12.00 + $0.25 + $0.10 = $28.15 CORRECT
- Folio 205: $9.29 + $12.00 + $0.40 + $0.00 = $21.69 CORRECT
**Principles check:**
- Riven Standard: PASS — analyzing financial documents for anomalies IS forensic accounting
- No Over-Scaffolding: BORDERLINE — the Investigator's Analysis section is fairly guided. It tells the solver to look at gap-period charges and convert cents to letters. Consider trimming the explicit A1Z26 instruction.
- No Computation Without Deduction: PASS — the deduction is "which charges matter?" The arithmetic is just conversion.
- Interlock: PASS — requires understanding alibi gaps from P1/timeline
**Length:** OK.
**Voice:** PASS. Accounting format, columns and totals. Reads like a real hotel bill.
**Extraction:** VERIFIED. L(12)-O(15)-C(03)-A(01)-T(20)-I(09)-O(15)-N(14) = LOCATION.

**Verdict:** APPROVE WITH NOTES

**Required fixes:**
1. Consider removing or softening the explicit A1Z26 conversion hint in the Investigator's Analysis. The connection "cents → letter position" should be the aha, not handed to the solver. Keep the hint that gap-period charges are important, but let the solver discover the number-to-letter conversion.

**Editor notes:**
The cents values are conspicuously round ($0.01, $0.03, $0.09, $0.12, etc.) which is a good subtle signal. The beer at $0.01 is especially suspicious — no beer costs one cent. Good design choice. The interlock with P1 (knowing the alibi gaps) is organic and well-motivated.

---

## P3 — The Floor Plan — The Methodical

**Brief compliance:** MATCH. Spatial path tracing with reference markers, physical prop (hotel key card).
**Factual accuracy:** 0 errors. All room positions, connections, and elevator shaft locations match layout.md.
**Principles check:**
- Riven Standard: PASS — tracing a route through a building IS spatial investigation
- No Over-Scaffolding: PASS — the floor plan provides the map, solver must determine the route
- Interlock: PASS — requires evidence from P1 (service elevator sound, fire escape) and the physical prop (key card)
- Aha: The connecting door between Gallery (200) and Linen Closet (206) — shown on the plan but not labeled as a passage. VERIFIED — this is a genuine discovery moment.
**Length:** OK. Floor plans are appropriately detailed.
**Voice:** PASS. Architectural drafting notation. Clean lines and labels.
**Extraction:** VERIFIED. Route: 303(marker 20=T) → 200(marker 9=I) → 206(marker 13=M) → B02(marker 5=E) = TIME.

**Verdict:** APPROVE

**Editor notes:**
Strongest puzzle in the set. The floor plan reads exactly like an architectural survey. The connecting door between 200 and 206 is visible but unlabeled — the solver must notice the thin line and realize it's a door. The key card prop (ROOM 303) is a clean physical-to-puzzle integration. The Methodical excelled here — thorough, precise, exactly what the brief needed.

Note on the floor plan rendering: the ASCII art works for the markdown version, but the print version should be a proper architectural diagram. The /hunt print step will need to convert this to a clean vector or hand-drawn style graphic. Flag for delivery build.

---

## P4 — The Witness Statement — The Social

**Brief compliance:** MATCH. Acrostic hidden in witness testimony.
**Factual accuracy:** 0 errors. Character details match characters.md. Timeline references match timeline.md.
**Principles check:**
- Riven Standard: PASS — reading witness statements for hidden meaning IS detective work
- No Over-Scaffolding: BORDERLINE — the analysis section enumerates all 32 sentences with blanks. This guides the solver heavily. Consider: should the solver discover the acrostic themselves, or is it presented as a task?
- Voice: EXCEPTIONAL. The witness voice is natural, nervous, halting. The acrostic constraint is invisible on first read.
**Length:** OK. Natural length for a witness statement.
**Extraction:** VERIFIED character by character:
  I-S-A-W-T-H-E-C-A-N-V-A-S-I-N-S-I-D-E-A-W-O-O-D-E-N-O-B-J-E-C-T
  "I SAW THE CANVAS INSIDE A WOODEN OBJECT"
  Last 6 letters: OBJECT. CORRECT.

**Verdict:** APPROVE WITH NOTES

**Required fixes:**
1. Same as P2 — consider whether the analysis section should enumerate all sentences or just hint that the first letters contain a message. The enumeration makes the puzzle mechanical (just reading first letters) rather than deductive (discovering the acrostic pattern). A lighter hint: "Miss Delacroix chose her words very carefully. Perhaps too carefully. Read the statement again — slowly, from the beginning."

**Editor notes:**
The hidden message "I SAW THE CANVAS INSIDE A WOODEN OBJECT" is a superb narrative payoff — Delacroix is telling the detective exactly what she saw, hidden in plain text. The Social captured her voice perfectly: nervous, verbose, full of pauses and corrections. The acrostic is genuinely hard to spot on a casual read, which is exactly right.

The answer OBJECT has a nice double meaning: the "wooden object" in the message (presumably a furniture piece or crate), and OBJECT as the abstract dimension (what was used in the theft).

---

## META — The Method

**Brief compliance:** MATCH. Calling card extraction.
**Extraction:** VERIFIED.
- P1[3] = S (S-U-**S**-P-E-C-T)
- P2[1] = L (**L**-O-C-A-T-I-O-N)
- P3[2] = I (T-**I**-M-E)
- P4[6] = T (O-B-J-E-C-**T**)
- SLIT. CORRECT.
**80% Rule:** VERIFIED. Any 3 of 4 letters + context strongly constrains SLIT:
- _LIT → SLIT obvious
- S_IT → SLIT, SPIT, SKIT → SLIT fits crime
- SL_T → SLIT, SLOT → SLIT fits
- SLI_ → SLIT, SLIM, SLID → SLIT fits
**Narrative payoff:** "The canvas was slit from its frame." The method clicks all evidence into place.

**Verdict:** APPROVE

---

## Summary

| Puzzle | Verdict | Key strength | Fix needed |
|--------|---------|-------------|------------|
| P1 | APPROVE WITH NOTES | Deduction quality, police voice | Print layout for extraction markers |
| P2 | APPROVE WITH NOTES | Financial document authenticity | Soften A1Z26 hint |
| P3 | APPROVE | Physical prop integration, spatial aha | Floor plan needs vector art for print |
| P4 | APPROVE WITH NOTES | Acrostic invisibility, voice quality | Lighten analysis section scaffold |
| META | APPROVE | Clean extraction, strong 80% rule | None |

All puzzles pass editorial review and are ready for blind testing.
