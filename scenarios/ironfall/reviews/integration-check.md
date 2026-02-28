# IRONFALL — Integration Check (Stage 8)

## Super-Meta Extraction Verification

The super-meta extracts one letter from each of the 12 answer words. The extraction position equals the puzzle's slot number within its act (or position 1 for meta answers). The 12 extracted letters form the True Ending cheat code.

### Canonical Answers (from META-DESIGN.md)

| # | Source | Answer (ROT13) | Answer (decoded) | Slot | Pos | Letter | ROT13 | Button |
|---|--------|----------------|-----------------|------|-----|--------|-------|--------|
| 1 | P01 | HZOEN | UMBRA | I-1 | 1 | U | H | UP |
| 2 | P02 | DHRYY | QUELL | I-2 | 2 | U | H | UP |
| 3 | P03 | BEQRE | ORDER | I-3 | 3 | D | Q | DOWN |
| 4 | P04 | FUNQR | SHADE | I-4 | 4 | D | Q | DOWN |
| 5 | P05 | NAIVY | ANVIL | I-5 | 5 | L | Y | LEFT |
| 6 | META-I | ERNYZ | REALM | -- | 1 | R | E | RIGHT |
| 7 | P06 | YBGHF | LOTUS | II-1 | 1 | L | Y | LEFT |
| 8 | P07 | BEOVG | ORBIT | II-2 | 2 | R | E | RIGHT |
| 9 | P08 | RZORE | EMBER | II-3 | 3 | B | O | B |
| 10 | P09 | TYRNZ | GLEAM | II-4 | 4 | A | N | A |
| 11 | P10 | URYVK | HELIX | II-5 | 5 | X | K | X |
| 12 | META-II | YRTVG | LEGIT | -- | 1 | L | Y | LEFT |

**Cheat code:** UP, UP, DOWN, DOWN, LEFT, RIGHT, LEFT, RIGHT, B, A, X, LEFT

First 8 = Konami Code prefix. Last 4 = Morimoto's twist.

---

## META-I Verification

Mechanism: Five Act I answers in a grid. Edit count determines extraction position: (edits mod 5) + 1. Read by edit count descending.

| Rank | Answer | Edits | (mod 5)+1 | Position | Letter | ROT13 |
|------|--------|-------|-----------|----------|--------|-------|
| 1 | ORDER | 24 | 5 | 5 | R | E |
| 2 | QUELL | 17 | 3 | 3 | E | R |
| 3 | SHADE | 12 | 3 | 3 | A | N |
| 4 | ANVIL | 9 | 5 | 5 | L | Y |
| 5 | UMBRA | 6 | 2 | 2 | M | Z |

Reading: E, R, N, Y, Z → decode ROT13 → R, E, A, L, M → **REALM**. Confirmed.

80% rule: with 4 of 5 answers, solver has 4 letters of REALM plus one blank. The word is guessable.

---

## META-II Verification

Mechanism: Five Act II answers displayed on "Game Secrets" screen. Each has a discovery marker highlighting one letter. Read highlighted letters top to bottom.

| Row | Answer | Highlighted position | Letter |
|-----|--------|---------------------|--------|
| 1 | LOTUS | 1 | L |
| 2 | EMBER | 1 | E |
| 3 | GLEAM | 1 | G |
| 4 | HELIX | 4 | I |
| 5 | ORBIT | 5 | T |

Reading: L, E, G, I, T → **LEGIT**. Confirmed.

80% rule: 4 of 5 letters of LEGIT is guessable.

---

## Puzzle-by-Puzzle Verification

### P01 — Bestiary v3.2

| Check | Result | Notes |
|-------|--------|-------|
| Answer matches META-DESIGN | **FAIL** | Authored: USHER (HFURE). META-DESIGN requires: UMBRA (HZOEN). PUZZLES.md says HFURE, creating a PUZZLES.md-vs-META-DESIGN conflict. |
| Super-meta extraction | OK (coincidence) | Both USHER[1] and UMBRA[1] = U → ROT13 H → UP. Works either way. |
| META-I extraction | **FAIL** | USHER[2]=S (ROT13=F). Needs UMBRA[2]=M (ROT13=Z). META-I would spell REALF instead of REALM. |
| All facts from world/ | OK | Bestiary stats verified |

**Status: FAIL — answer must be UMBRA (per META-DESIGN). Both PUZZLES.md and the authored P01 need revision.**

### P02 — Forge Guide

| Check | Result | Notes |
|-------|--------|-------|
| Answer matches META-DESIGN | **FAIL** | Authored: DRESS (QERFF). META-DESIGN requires: QUELL (DHRYY). |
| Super-meta extraction | **FAIL** | DRESS[2]=R (ROT13=E). Needs QUELL[2]=U (ROT13=H → UP). |
| META-I extraction | **FAIL** | Same issue |
| All facts from world/ | OK | Crafting rules verified |

**Status: FAIL — answer must be QUELL, not DRESS.**

### P03 — Save File Deep Dive

| Check | Result | Notes |
|-------|--------|-------|
| Answer matches META-DESIGN | OK | ORDER = BEQRE |
| Super-meta extraction | OK | ORDER[3]=D → ROT13 Q → DOWN |
| Hex data internally consistent | OK | Key offsets verified: 0x65, 0x66, 0x67, 0x68+ |
| Save format matches savefile.md | OK | Magic number, field offsets correct |
| ASCII conversion correct | OK | 4F=O, 52=R, 44=D, 45=E, 52=R |

**Status: PASS**

### P04 — World Map — All Connections

| Check | Result | Notes |
|-------|--------|-------|
| Answer matches META-DESIGN | OK | SHADE = FUNQR |
| Super-meta extraction | OK | SHADE[4]=D → ROT13 Q → DOWN |
| Region identification correct | OK | All 5 clues point to correct regions |
| Name indexing correct | OK | STORMSPIREPEAKS[1]=S, ASHVEILFOREST[3]=H, EMBERWASTES[7]=A, IRONCITADEL[9]=D, SUNKENVALE[5]=E |
| Map topology matches locations.md | OK | All connections verified |

**Status: PASS**

### P05 — Battle Damage Calculator v2

| Check | Result | Notes |
|-------|--------|-------|
| Answer matches META-DESIGN | OK | ANVIL = NAIVY |
| Super-meta extraction | OK | ANVIL[5]=L → ROT13 Y → LEFT |
| Formula derivation from training data | OK | All 4 training logs verified |
| Missing DEF computations | OK | 1(A), 14(N), 22(V), 9(I), 12(L) all verified |
| A1Z26 conversion | OK | All within 1-26 range |

**Status: PASS**

### P06 — Enemy Sightings — Unconfirmed

| Check | Result | Notes |
|-------|--------|-------|
| Answer matches META-DESIGN | OK | LOTUS = YBGHF |
| Super-meta extraction | OK | LOTUS[1]=L → ROT13 Y → LEFT |
| Enemy identification unique | OK | Each sighting has exactly one matching enemy |
| Bestiary stats correct | OK | All HP, ATK, DEF, SPD, drops verified against bestiary.md |
| A1Z26 conversion | OK | 12=L, 15=O, 20=T, 21=U, 19=S |

**Status: PASS**

### P07 — The Perfect Build

| Check | Result | Notes |
|-------|--------|-------|
| Answer matches META-DESIGN | OK | ORBIT = BEOVG |
| Super-meta extraction | OK | ORBIT[2]=R → ROT13 E → RIGHT |
| Optimal loadout correct | OK | Dawn Mantle (only Dark resist), Void Edge (ATK+30), Earthen Wall (DEF+25) |
| Crafting chain correct | OK | All recipes verified against items.md |
| Enemy-to-drop mapping correct | OK | All 5 enemies verified against bestiary.md |
| Marked letters correct | OK | O in MOSS CRAWLER, R in DREAD KNIGHT, B in BLIZZARD WRAITH, I in SHADE WISP, T in PERMAFROST KNIGHT |

**Status: PASS**

### P08 — 100% Completion Guide

| Check | Result | Notes |
|-------|--------|-------|
| Answer matches META-DESIGN | OK | EMBER = RZORE |
| Super-meta extraction | OK | EMBER[3]=B → ROT13 O → B button |
| 11 real Marks accurate | OK | Names and descriptions match achievements.md |
| 5 fake Marks detectable | OK | All fakes have wrong name and/or description |
| Position 13 near-miss correct | OK | Oathbreaker is REAL (slight paraphrase is intentional) |
| 3rd letter extraction | OK | E(GREENHORN), M(HAMMERED), B(UMBERLIGHT), E(STEADFAST), R(SURVEYOR) |

**Status: PASS**

### P09 — The Speedrunner's Route

| Check | Result | Notes |
|-------|--------|-------|
| Answer matches META-DESIGN | OK | GLEAM = TYRNZ |
| Super-meta extraction | OK | GLEAM[4]=A → ROT13 N → A button |
| Optimal route verified | OK | Ashveil→Glacial→Stormspire→Sunken→Ember→Iron Citadel = 5 transitions |
| No shorter route exists | OK | Stormspire required, minimum distance verified |
| Segment times A1Z26 | OK | 7=G, 12=L, 5=E, 1=A, 13=M |

**Status: PASS**

### P10 — Anomaly in the Code

| Check | Result | Notes |
|-------|--------|-------|
| Answer matches META-DESIGN | OK | HELIX = URYVK |
| Super-meta extraction | OK | HELIX[5]=X → ROT13 K → X button |
| Correct records verified | OK | Records 1, 3, 5 all match formula |
| Anomalous records verified | OK | Records 2, 4, 6, 7, 8 have specific errors |
| Differences correct | OK | 8(H), 5(E), 12(L), 9(I), 24(X) |
| A1Z26 conversion | OK | All within 1-26 range |

**Status: PASS**

---

## Summary

| Puzzle | Integration Status |
|--------|-------------------|
| P01 | **FAIL** — answer should be UMBRA, not USHER |
| P02 | **FAIL** — answer should be QUELL, not DRESS |
| P03 | PASS |
| P04 | PASS |
| P05 | PASS |
| P06 | PASS |
| P07 | PASS |
| P08 | PASS |
| P09 | PASS |
| P10 | PASS |
| META-I | PASS (design verified, depends on P01/P02 fix) |
| META-II | PASS (design verified) |
| SUPER-META | PASS (design verified, depends on P01/P02 fix) |

**Blockers:** P01 and P02 must be revised to produce UMBRA and QUELL respectively. These are pre-existing issues from Stage 6 (The Methodical's authored puzzles). Until fixed, META-I and the super-meta cannot be fully validated end-to-end.

**P03-P10:** All 8 puzzles authored in this session pass integration. All answers verified character-by-character. All meta extractions confirmed. All world data references checked against locked system files.
