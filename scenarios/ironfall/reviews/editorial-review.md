# IRONFALL — Editorial Review (Stage 7)

## Voice Consistency

### Narrator Voice Rules
- Past tense for forum memories. Present tense for the countdown and solver instructions.
- No exclamation marks.
- No questions in narrator voice (forum user quotes may contain questions).
- Short sentences.
- The solver is "you."

### Review Results

| Puzzle | Exclamation marks | Narrator questions | Tense consistency | Short sentences | "You" for solver | Verdict |
|--------|------------------|-------------------|-------------------|----------------|-----------------|---------|
| P01 | None | None | Past for memories, present for instructions | Yes | Yes (implicit) | PASS |
| P02 | None | None | Past for memories | Yes | Yes (implicit) | PASS |
| P03 | None | None | Past for forum history, present for task | Yes | Yes (implicit) | PASS |
| P04 | None | None | Past for narrative, present for task | Yes | Yes (implicit) | PASS |
| P05 | None | None | Past for BattleMath_99's research | Yes | Yes (implicit) | PASS |
| P06 | None | None | Past for sighting reports | Yes | Yes (implicit) | PASS |
| P07 | None | None (fixed: questions converted to commands) | Past for forum history | Yes | Yes (implicit) | PASS |
| P08 | None | None | Past for screenshot event, present for task | Yes | Yes (implicit) | PASS |
| P09 | None | None (fixed: question converted to statement) | Past for speedrun log | Yes | Yes (implicit) | PASS |
| P10 | None | None | Past for datamining, present for task | Yes | Yes (implicit) | PASS |

### Voice Notes

All 10 puzzles use a consistent archive-narrator voice. Forum user quotes are clearly attributed and use their own voice (forum casual, first person, past tense narrative). The narrator never speaks in first person.

The Garek riddles in P04 use a distinct poetic voice, which is appropriate — Garek is a character who speaks in riddles, not the archive narrator.

The Speedster's P03 and P04 are slightly less polished in narrative framing than The Methodical's P01/P02, consistent with the personality. The Lurker's P09 and P10 are the most terse, also consistent.

---

## Extraction Verification (Character-by-Character)

### P01: USHER (ROT13: HFURE)
- TUNDRA[2]=U, SHADE[1]=S, DREADKNIGHT[10]=H, VOLTSPIDER[9]=E, FROSTCLAW[2]=R
- Slot I-1, extract position 1: U. ROT13(U) = H. Matches meta table row 1.

### P02: DRESS (ROT13: QERFF) — **MISMATCH**
- P02 produces DRESS. PUZZLES.md requires QUELL (ROT13: DHRYY).
- Slot I-2, extract position 2: QUELL[2]=U. ROT13(U)=H. Meta requires H.
- DRESS[2]=R. ROT13(R)=E. This does NOT match.
- **ACTION REQUIRED:** P02 must be revised to produce QUELL.

### P03: ORDER (ROT13: BEQRE)
- ASCII bytes 0x4F,0x52,0x44,0x45,0x52 = O,R,D,E,R
- Slot I-3, extract position 3: ORDER[3]=D. ROT13(D)=Q. Matches meta table row 3.

### P04: SHADE (ROT13: FUNQR)
- STORMSPIREPEAKS[1]=S, ASHVEILFOREST[3]=H, EMBERWASTES[7]=A, IRONCITADEL[9]=D, SUNKENVALE[5]=E
- Slot I-4, extract position 4: SHADE[4]=D. ROT13(D)=Q. Matches meta table row 4.

### P05: ANVIL (ROT13: NAIVY)
- DEF values: 1=A, 14=N, 22=V, 9=I, 12=L
- Slot I-5, extract position 5: ANVIL[5]=L. ROT13(L)=Y. Matches meta table row 5.

### P06: LOTUS (ROT13: YBGHF)
- Bestiary numbers: 12=L, 15=O, 20=T, 21=U, 19=S
- Slot II-1, extract position 1: LOTUS[1]=L. ROT13(L)=Y. Matches meta table row 7.

### P07: ORBIT (ROT13: BEOVG)
- Marked letters: M[O]SS CRAWLER=O, D[R]EAD KNIGHT=R, [B]LIZZARD WRAITH=B, SHADE W[I]SP=I, PERMAFROSKNIGHT with [T]=T
- Slot II-2, extract position 2: ORBIT[2]=R. ROT13(R)=E. Matches meta table row 8.

### P08: EMBER (ROT13: RZORE)
- Fake Mark 3rd letters: GREENHORN[3]=E, HAMMERED[3]=M, UMBERLIGHT[3]=B, STEADFAST[3]=E, SURVEYOR[3]=R
- Slot II-3, extract position 3: EMBER[3]=B. ROT13(B)=O. Matches meta table row 9.

### P09: GLEAM (ROT13: TYRNZ)
- Segment times: 7=G, 12=L, 5=E, 1=A, 13=M
- Slot II-4, extract position 4: GLEAM[4]=A. ROT13(A)=N. Matches meta table row 10.

### P10: HELIX (ROT13: URYVK)
- Damage differences: 8=H, 5=E, 12=L, 9=I, 24=X
- Slot II-5, extract position 5: HELIX[5]=X. ROT13(X)=K. Matches meta table row 11.

---

## Blind Test Scores

| ID | Deductive Rigor | Reading Reward | Aha Moment | Elegance | Fairness | Extraction | Total | Verdict |
|----|----------------|---------------|------------|----------|---------|------------|-------|---------|
| P01 | -- | -- | -- | -- | -- | -- | -- | Pre-existing (not tested this round) |
| P02 | -- | -- | -- | -- | -- | -- | -- | Pre-existing (answer mismatch) |
| P03 | 4 | 4 | 5 | 4 | 4 | 5 | 26/30 | PASS |
| P04 | 4 | 4 | 4 | 4 | 4 | 4 | 24/30 | PASS |
| P05 | 5 | 5 | 4 | 4 | 5 | 5 | 28/30 | PASS |
| P06 | 5 | 5 | 3 | 4 | 5 | 4 | 26/30 | PASS |
| P07 | 4 | 5 | 4 | 3 | 4 | 4 | 24/30 | PASS |
| P08 | 4 | 5 | 4 | 4 | 5 | 5 | 27/30 | PASS |
| P09 | 5 | 4 | 4 | 4 | 4 | 5 | 26/30 | PASS |
| P10 | 5 | 5 | 4 | 5 | 5 | 5 | 29/30 | PASS |

All P03-P10 score >= 22/30 with no dimension below 3.

---

## Issues Requiring Resolution

### Critical

1. **P02 answer mismatch.** The authored P02 produces DRESS. The meta requires QUELL. P02 must be revised before integration. This blocks Stage 8.

### Advisory

2. **P04 elegance.** Lowest-scoring dimension across all puzzles (tied with P07). The numbers in Garek's riddles (1, 3, 7, 9, 5) are thematically grounded but require the solver to trust that in-game references contain the extraction position. Revised clues with DataMiner_X's explicit instruction ("count that many letters into the place name, no spaces") mitigates this.

3. **P07 extraction is presentational.** The marked letters in DataMiner_X's farming chart are given rather than deduced. The solver earns the extraction by correctly determining the 5 enemies (through crafting chain logic), then reads the chart as confirmation. This is acceptable for a Social-authored puzzle but lacks the deductive elegance of P05 or P10.

4. **P06 aha.** Scores 3/5 on Aha Moment. The puzzle is primarily an identification exercise — satisfying but not revelatory. The bestiary-number-to-A1Z26 extraction adds a layer but is stated rather than discovered.

---

## Recommendation

Mark Stage 7 EDITORIAL as Done with one blocker: P02 revision needed. Proceed to Stage 8 INTEGRATION for P03-P10. P01 and P02 integration is conditional on P02 fix.
