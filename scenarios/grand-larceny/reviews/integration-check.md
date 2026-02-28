# THE GRAND LARCENY — Integration Check

Date: 2026-02-27

---

## Checklist

- [x] All 4 puzzles: editorial review passed (APPROVE or APPROVE WITH NOTES)
- [x] Meta: extraction verified (SLIT from positions 3,1,2,6 across four answer words)
- [x] Meta: 80% rule verified (any 3 of 4 answers → SLIT is strongly constrained)
- [x] Answer words: no duplicates, no unintended overlaps
  - SUSPECT (7), LOCATION (8), TIME (4), OBJECT (6) — all unique, no shared substrings that could confuse
- [x] Difficulty curve: all four puzzles are medium difficulty, variety is in TYPE not difficulty
  - P1 (logic): 20-30 min
  - P2 (arithmetic): 15-25 min
  - P3 (spatial): 20-30 min
  - P4 (text): 15-25 min
- [x] Narrator voice: each document uses its document type voice consistently
  - P1: police bureaucratic
  - P2: hotel accounting
  - P3: architectural notation
  - P4: witness transcript
- [x] Cover sheet voice: terse police briefing
- [x] World consistency: all facts cross-referenced against world/ canon
  - Room numbers: consistent across P1, P3, and world/systems/layout.md
  - Character names: consistent across P1, P4, and world/systems/characters.md
  - Timeline: consistent across P1, P2, P4, and world/systems/timeline.md
  - Prices: consistent across P2 and world/systems/services.md
- [x] No deliberate errors anywhere
- [x] Physical elements: all props defined (dossier, hotel key, calling card)
- [x] Answer security: no plaintext answers in git-tracked files

## Cross-Puzzle Verification

### Information flow between puzzles

| Source puzzle | Information | Used by |
|--------------|-------------|---------|
| P1 | Alibi gap windows for each suspect | P2 (which charges to extract) |
| P1 | Service elevator heard at 9:20 | P3 (confirms elevator in route) |
| P1 | Fire escape accessible | P3 (confirms entry point) |
| P1 | Kessler elimination logic | META (validates suspect dimension) |
| P3 | Gallery→linen closet door | P1 (explains how thief escaped gallery) |
| P4 | Shadow near 3F stairs | P1 (corroborates Fontaine on 3F) |
| P4 | Service elevator sound heard | P3 (corroborates elevator use) |
| P4 | "Canvas inside a wooden object" | META (thematic bridge to method) |

No circular dependencies. P1 and P3 have bidirectional information sharing (each provides context for the other), but neither REQUIRES the other to solve. Each puzzle is independently solvable.

### Answer Word Compatibility

Meta extraction: P1[3]=S, P2[1]=L, P3[2]=I, P4[6]=T → SLIT

Verified that:
- Position 3 of SUSPECT = S (S-U-**S**-P-E-C-T) -- yes, this is the second S
- Position 1 of LOCATION = L (**L**-O-C-A-T-I-O-N) -- yes
- Position 2 of TIME = I (T-**I**-M-E) -- yes
- Position 6 of OBJECT = T (O-B-J-E-C-**T**) -- yes

The calling card must show: **3 — 1 — 2 — 6**

These numbers correspond to exhibit labels:
- Exhibit A (P1): take letter 3
- Exhibit B (P2): take letter 1
- Exhibit C (P3): take letter 2
- Exhibit D (P4): take letter 6

### Evidence Log Design (Meta Physical Page)

The evidence log is the last page of the dossier. It contains:
1. A summary of the case (narrative flavor)
2. Four fill-in fields for the four exhibit answers
3. The extraction instruction: "Using the calling card, take the indicated letter from each answer"
4. A final answer blank: THE METHOD = _ _ _ _

The calling card (separate physical prop) has four symbols corresponding to the four exhibits, each with a number. This is the meta's key — without the calling card, the solver would need to guess which positions to extract.

## Outstanding Items for Delivery Build

1. Floor plan (P3) needs vector/graphic treatment for print — ASCII art insufficient
2. P1 extraction markers need clear visual treatment (bold, underline, or color boxes)
3. P2 analysis section — soften the A1Z26 hint per editorial notes
4. P4 analysis section — lighten the sentence enumeration per editorial notes
5. Calling card design needs the four exhibit numbers (3, 1, 2, 6) integrated into a visual design

## Verdict

Integration check PASSED. All puzzles are internally consistent, cross-referenced, and the meta extraction is verified. Ready for delivery build (Stage 9).
