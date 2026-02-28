# Evidence Correlation Log -- Meta Worksheet

**Prop ID:** PR07
**Type:** document (printed page)
**Material:** White letter paper, single page
**Print:** Single-sided, black ink
**Position in dossier:** Last printed page (bottom of stack, before loose props)
**Print-ready file:** `delivery/print/puzzles/evidence-log.html`

---

## Purpose

The evidence log serves two functions:

1. **Answer collection:** Teams write in the four exhibit answers (SUSPECT, LOCATION, TIME, OBJECT) as they solve each puzzle. This gives them a central place to track progress.

2. **Meta extraction:** The log instructs teams to use the calling card (PR09) to extract one letter from each answer word. The four extracted letters spell the final answer: THE METHOD.

The evidence log does NOT reveal the extraction positions. It says "take the indicated letter" and points to the calling card. The calling card has the numbers 3, 1, 2, 6.

---

## Content

```
+================================================================+
|                    CITY POLICE DEPARTMENT                        |
|                    EVIDENCE CORRELATION LOG                      |
|                                                                  |
|  Case No.: 47-1011-GL                                           |
|  Investigating Officer: _____________________________ (your team)|
+================================================================+


FINDINGS

Exhibit A -- The Police Report
    The primary SUSPECT is:  ___ ___ ___ ___ ___ ___ ___

Exhibit B -- The Hotel Receipt
    The critical LOCATION was:  ___ ___ ___ ___ ___ ___ ___ ___

Exhibit C -- The Floor Plan
    The decisive TIME was:  ___ ___ ___ ___

Exhibit D -- The Witness Statement
    The key OBJECT was:  ___ ___ ___ ___ ___ ___


CASE CORRELATION

A calling card was recovered at the scene (see physical evidence
in your dossier). The card bears four numbers: a sequence the thief
left as a signature.

Using the calling card, take the indicated letter from each finding:

    From EXHIBIT A, take letter #___: ___
    From EXHIBIT B, take letter #___: ___
    From EXHIBIT C, take letter #___: ___
    From EXHIBIT D, take letter #___: ___


THE METHOD
+---+---+---+---+
|   |   |   |   |
+---+---+---+---+
(4 letters)

How was the painting stolen?

Submit your answer at the answer submission site.
Case closed.
```

---

## Layout Notes

- **Header:** Same institutional double-ruled box as cover sheet and P1 (Georgia serif, centered)
- **Findings section:** Each exhibit gets a labeled field with letter blanks (underscored, evenly spaced). Blank count matches answer word length:
  - Exhibit A: 7 blanks (for ROT13: FHFCRPG)
  - Exhibit B: 8 blanks (for ROT13: YBPNGVBA)
  - Exhibit C: 4 blanks (for ROT13: GVZR)
  - Exhibit D: 6 blanks (for ROT13: BOWRPG)
- **Case Correlation section:** Light background box (#fafaf5), single border. Four extraction rows with blanks for both the number and the resulting letter.
- **Final answer:** Centered, 2px solid border box. Four 36x36px answer cells in a row. "THE METHOD" header, "(4 letters)" label below.
- **Footer:** "Submit your answer..." centered, 10pt, separated by thin rule.

---

## How the Meta Works (design reference, NOT printed)

The calling card shows: **3 -- 1 -- 2 -- 6**

These are the letter positions to extract from each exhibit answer:

| Exhibit | Answer (ROT13) | Position | Letter |
|---------|----------------|----------|--------|
| A (P1) | FHFCRPG | 3 | F (ROT13) |
| B (P2) | YBPNGVBA | 1 | Y (ROT13) |
| C (P3) | GVZR | 2 | V (ROT13) |
| D (P4) | BOWRPG | 6 | G (ROT13) |

Result (ROT13): F-Y-V-G (4 letters)

---

## 80% Rule

With 3 of 4 answers filled in plus the extraction structure, the missing letter is strongly constrained. See `meta/META-DESIGN.md` for full analysis.

---

## Canon Verification

| Element | Source | Verified |
|---------|--------|----------|
| Case No. 47-1011-GL | Consistent across all documents | Yes |
| Exhibit A-D labeling | ROUNDS.md numbering system, cover sheet | Yes |
| Dimension names (SUSPECT, LOCATION, TIME, OBJECT) | PUZZLES.md coordinated answers | Yes |
| Answer word lengths (7, 8, 4, 6) | PUZZLES.md | Yes |
| Final answer 4 letters | meta/META-DESIGN.md | Yes |
| Calling card reference | PR09 (calling-card.md) | Yes |
