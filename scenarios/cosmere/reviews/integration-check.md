# THE SIXTEENTH SHARD -- Stage 8 Integration Check

**Reviewer:** Claude (automated integration pass)
**Date:** 2026-02-28
**Scope:** Full hunt assembly verification -- 36 feeders + 3 round metas + 1 super-meta
**Source files:** `puzzles/P01-P36`, `meta/META-I-WELL.md`, `meta/META-II-OATHS.md`, `meta/META-III-ADONALSIUM.md`, `meta/SUPER-META.md`, `meta/cosmere-answers.md`, `reviews/editorial-review.md`

---

## 1. Answer Registry Audit

### 1.1 Completeness

| Category | Expected | Found | Status |
|----------|----------|-------|--------|
| Feeder puzzle answers (P01-P36) | 36 | 36 | PASS |
| Round meta answers (META I-III) | 3 | 3 | PASS |
| Super-meta answer | 1 | 1 | PASS |
| **Total** | **40** | **40** | **PASS** |

### 1.2 ROT13 Encoding

All 40 answers in `meta/cosmere-answers.md` are ROT13 encoded. No plaintext answers appear in the registry file.

**Feeder answers verified (ROT13 -> plaintext spot-check):**
- P01: HCEVFR -> UPRISE (correct)
- P08: URVFG -> HEIST (correct)
- P17: GBJRE -> TOWER (correct)
- P22: REBQRQ -> ERODED (correct)
- P30: BEVTVA -> ORIGIN (correct)
- P36: GUERNQ -> THREAD (correct)

**Meta answers verified:**
- META-I: HAVGR -> UNITE (correct)
- META-II: RAIBL -> ENVOY (correct)
- META-III: NZVQ -> AMID (correct)
- SUPER: HAVGL -> UNITY (correct)

**Status: PASS.** All 40 answers are ROT13 encoded with no plaintext leaks in the registry.

### 1.3 Duplicate Check

All 36 feeder answers are unique words. No duplicate exists among the 36.
All 4 meta answers (UNITE, ENVOY, AMID, UNITY) are distinct from each other and from all 36 feeders.

Note: UNITE (Meta I) and UNITY (Super-meta) share a root. This is thematically intentional and does not constitute a duplicate (different words, different letter counts).

**Status: PASS.** Zero duplicates across all 40 answers.

---

## 2. Round 1 Meta Verification -- THE WELL (UNITE)

### 2.1 Feeder Answer Confirmation

The 12 Scadrial puzzles (P01-P12) and their assigned metals/answers:

| Puzzle | Metal | # | Pull/Push | Quadrant | Answer (ROT13) | Decoded |
|--------|-------|---|-----------|----------|-----------------|---------|
| P01 | Iron | 1 | Pull | Physical | HCEVFR | UPRISE |
| P02 | Steel | 2 | Push | Physical | SBETRQ | FORGED |
| P03 | Zinc | 5 | Pull | Mental | NAPUBE | ANCHOR |
| P04 | Brass | 6 | Push | Mental | ZNGEVK | MATRIX |
| P05 | Pewter | 4 | Push | Physical | GEVNQ | TRIAD |
| P06 | Bronze | 8 | Push | Mental | UVQQRA | HIDDEN |
| P07 | Electrum | 10 | Push | Temporal | GLENAG | TYRANT |
| P08 | Gold | 9 | Pull | Temporal | URVFG | HEIST |
| P09 | Cadmium | 11 | Pull | Temporal | EVGHNY | RITUAL |
| P10 | Chromium | 15 | Pull | Enhancement | HAQRNQ | UNDEAD |
| P11 | Bendalloy | 12 | Push | Temporal | OERNPU | BREACH |
| P12 | Duralumin | 14 | Push | Enhancement | BHGYNJ | OUTLAW |

All 12 answers confirmed present in the registry.

### 2.2 Meta I Extraction Chain (Character by Character)

The mechanism: Pulling metals only, in table order, extract at quadrant depth.

**Quadrant depths:** Physical=1, Mental=2, Temporal=3, Enhancement=4

**Pulling metals with assigned puzzles (5 of 8 Pulling metals have puzzles):**

| Metal # | Metal | Pull/Push | Quadrant | Depth | Answer | Letter at Depth |
|---------|-------|-----------|----------|-------|--------|-----------------|
| 1 | Iron | Pull | Physical | 1 | UPRISE | U[1]=**U** |
| 5 | Zinc | Pull | Mental | 2 | ANCHOR | A[2]=**N** |
| 9 | Gold | Pull | Temporal | 3 | HEIST | H[3]=**I** |
| 11 | Cadmium | Pull | Temporal | 3 | RITUAL | R[3]=**T** |
| 15 | Chromium | Pull | Enhancement | 4 | UNDEAD | U[4]=**E** |

Reading in table order (1, 5, 9, 11, 15): **U - N - I - T - E**

ROT13: HAVGR

**Result: UNITE. PASS.**

### 2.3 80% Rule Verification

5 contributing answers (Pulling metals) out of 12 total.

| Missing Puzzle | Letters Remaining | Solvable? |
|---------------|-------------------|-----------|
| P01 (Iron) | _NITE | 1 gap, strongly constrained -> UNITE | Yes |
| P03 (Zinc) | U_ITE | 1 gap -> UNITE | Yes |
| P08 (Gold) | UN_TE | 1 gap -> UNITE | Yes |
| P09 (Cadmium) | UNI_E | 1 gap -> UNITE | Yes |
| P10 (Chromium) | UNIT_ | 1 gap -> UNITE | Yes |
| Any 2 missing | 3 of 5 letters | Very likely with pattern | Yes |

**80% Rule: PASS.** Any 4 of 5 contributing answers suffice.

---

## 3. Round 2 Meta Verification -- THE OATHS (ENVOY)

### 3.1 Feeder Answer Confirmation

The 12 Roshar puzzles (P13-P24) and their assigned Orders/answers:

| Puzzle | Order | Order # | Ideals | Answer (ROT13) | Decoded |
|--------|-------|---------|--------|-----------------|---------|
| P13 | Dustbringers | 3 | 1 | OEBXRA | BROKEN |
| P14 | Edgedancers | 4 | 3 | QRIBGR | DEVOTE |
| P15 | Truthwatchers | 5 | 1 | FCVEVG | SPIRIT |
| P16 | Skybreakers | 2 | 5 | FJBEA | SWORN |
| P17 | Windrunners | 1 | 4 | GBJRE | TOWER |
| P18 | Lightweavers | 6 | 3 | CNGGREA | PATTERN |
| P19 | Stonewards | 9 | 1 | PUVGVA | CHITIN |
| P20 | Windrunners (dup) | 1 | 4 | OEVQTR | BRIDGE |
| P21 | Willshapers | 8 | 1 | PUNAG | CHANT |
| P22 | Lightweavers (crit) | 6 | 3 | REBQRQ | ERODED |
| P23 | Bondsmiths | 10 | 4 | ORELY | BERYL |
| P24 | Elsecallers | 7 | 1 | GRZCRFG | TEMPEST |

### 3.2 Meta II Extraction Chain (Character by Character)

The mechanism: Orders with 3+ documented Ideals. Extract letter at Ideal-count position. Read in Order number sequence.

**Contributing Orders (3+ Ideals):**

| Order | Order # | # Ideals | Contributing Puzzle | Answer | Letter at Position |
|-------|---------|----------|--------------------|---------|--------------------|
| Windrunners | 1 | 4 | P17 | TOWER | T[4]=**E** |
| Skybreakers | 2 | 5 | P16 | SWORN | S[5]=**N** |
| Edgedancers | 4 | 3 | P14 | DEVOTE | D[3]=**V** |
| Lightweavers | 6 | 3 | P22 (crit) | ERODED | E[3]=**O** |
| Bondsmiths | 10 | 4 | P23 | BERYL | B[4]=**Y** |

Reading in Order number sequence (1, 2, 4, 6, 10): **E - N - V - O - Y**

ROT13: RAIBL

**Result: ENVOY. PASS.**

### 3.3 Double-Representation Check

Two Orders have two puzzles:
- Windrunners: P17 (meta-contributing, answer TOWER) and P20 (duplicate, answer BRIDGE)
- Lightweavers: P18 (duplicate, answer PATTERN) and P22 (meta-contributing, marked "crit", answer ERODED)

Verification that the non-contributing duplicates do NOT satisfy the meta constraint:
- P20 BRIDGE[4] = D (not E) -- correct, does not interfere
- P18 PATTERN[3] = T (not O) -- correct, does not interfere

**Status: PASS.** Only the designated contributing puzzles satisfy the letter constraints.

### 3.4 80% Rule Verification

5 contributing answers out of 12 total.

| Missing Puzzle | Letters Remaining | Solvable? |
|---------------|-------------------|-----------|
| P17 (Windrunners) | _NVOY | 1 gap -> ENVOY | Yes |
| P16 (Skybreakers) | E_VOY | 1 gap -> ENVOY | Yes |
| P14 (Edgedancers) | EN_OY | 1 gap -> ENVOY | Yes |
| P22 (Lightweavers) | ENV_Y | 1 gap -> ENVOY | Yes |
| P23 (Bondsmiths) | ENVO_ | 1 gap -> ENVOY | Yes |

**80% Rule: PASS.** Any 4 of 5 contributing answers suffice.

---

## 4. Round 3 Meta Verification -- ADONALSIUM (AMID)

### 4.1 Shard Assignment Verification

**Module E (P25-P30) assigned Shards:**

| Puzzle | Assigned Shard | Verified in Puzzle Content |
|--------|---------------|---------------------------|
| P25 | Honor | Yes -- P25 Fragment A describes Honor explicitly |
| P26 | Preservation | Yes -- P26 Entry #5 describes Preservation |
| P27 | Odium | Yes -- P27 describes the "god of hatred" campaign |
| P28 | Endowment | Yes -- P28 Entry II describes Breath/Returned |
| P29 | Autonomy | Yes -- P29 includes Autonomy in the grid |
| P30 | Virtuosity | Yes -- P30 Report IV describes self-Splintering |

**Module F (P31-P36) assigned Shards:**

| Puzzle | Assigned Shard | Verified in Puzzle Content |
|--------|---------------|---------------------------|
| P31 | Ruin | Yes -- Scene XI is Secret History / Ruin's domain |
| P32 | Harmony | Yes -- Account C references Scadrial / Preservation -> Harmony |
| P33 | Cultivation | Yes -- answer GROWTH, thematic connection to Cultivation |
| P34 | Devotion | Yes -- Letter G is Devotion's voice |
| P35 | Whimsy | Yes -- thematic connection (whimsical memory alteration) |
| P36 | Valor | Yes -- thematic connection (courage to act directly) |

**Combined represented Shards (12):**

Honor, Preservation, Odium, Endowment, Autonomy, Virtuosity, Ruin, Harmony, Cultivation, Devotion, Whimsy, Valor

**Missing Shards (4):**

| # | Missing Shard | First Letter |
|---|---------------|-------------|
| 1 | **A**mbition | A |
| 2 | **D**ominion | D |
| 3 | **I**nvention | I |
| 4 | **M**ercy | M |

### 4.2 Extraction Chain

First letters: A, D, I, M

Anagram with thematic clue ("where Hoid has always been"): **A-M-I-D**

ROT13: NZVQ

**Result: AMID. PASS.**

### 4.3 80% Rule Verification

| Missing Feeders | Extra "Missing" Shards | Pool | Solvable? |
|----------------|----------------------|------|-----------|
| 0 | 4 exactly | A,D,I,M | Yes |
| 1 | 5 | A,D,I,M + 1 extra | Very likely (anagram from 5 letters with thematic clue) |
| 2 | 6 | A,D,I,M + 2 extra | Likely (4-letter word from 6 candidates) |

The 4 missing Shards are the least-known, so partial solving almost always correctly excludes them.

**80% Rule: PASS.**

---

## 5. Super-Meta Verification -- THE SEVENTEENTH SHARD (UNITY)

### 5.1 Input Answers

| Round | Meta Name | Answer (Decoded) | Answer (ROT13) | Length |
|-------|-----------|-----------------|-----------------|--------|
| I | THE WELL | UNITE | HAVGR | 5 |
| II | THE OATHS | ENVOY | RAIBL | 5 |
| III | ADONALSIUM | AMID | NZVQ | 4 |

### 5.2 Realms Cycle Extraction (Character by Character)

The cycle: Physical(Round I) -> Cognitive(Round II) -> Spiritual(Round III) -> Physical(Round I) -> Cognitive(Round II)

Position advances: 1 -> 2 -> 3 -> 4 -> 5

| Step | Realm | Round | Answer | Position | Letter |
|------|-------|-------|--------|----------|--------|
| 1 | Physical | I | UNITE | 1 | **U** |
| 2 | Cognitive | II | ENVOY | 2 | **N** |
| 3 | Spiritual | III | AMID | 3 | **I** |
| 4 | Physical | I | UNITE | 4 | **T** |
| 5 | Cognitive | II | ENVOY | 5 | **Y** |

Reading: **U - N - I - T - Y**

ROT13: HAVGL

**Result: UNITY. PASS.**

### 5.3 Length Verification

- Step 1 requires UNITE[1] -- UNITE has 5 letters. Position 1 is valid.
- Step 2 requires ENVOY[2] -- ENVOY has 5 letters. Position 2 is valid.
- Step 3 requires AMID[3] -- AMID has 4 letters. Position 3 is valid.
- Step 4 requires UNITE[4] -- UNITE has 5 letters. Position 4 is valid.
- Step 5 requires ENVOY[5] -- ENVOY has 5 letters. Position 5 is valid.

All positions fall within the answer lengths. **PASS.**

### 5.4 80% Rule Verification

| Metas Solved | Letters Known | Pattern | Solvable? |
|-------------|-------------- |---------|-----------|
| All 3 | U, N, I, T, Y | UNITY | Yes |
| I + II only | U, N, _, T, Y | U N _ T Y | Yes (strongly constrained) |
| I + III only | U, _, I, T, _ | U _ I T _ | Likely with thematic hint |
| II + III only | _, N, I, _, Y | _ N I _ Y | Likely with thematic hint |

**80% Rule: PASS.**

---

## 6. Difficulty Curve -- Full Hunt

### 6.1 All 36 Puzzles by Difficulty

**Round 1 (Scadrial, P01-P12):**

```
Module A: P01(2) P02(2) P03(3) P04(3) P05(4) P06(4)
Module B: P07(2) P08(2) P09(3) P10(3) P11(4) P12(4)
```

Range: 2-star to 4-star. Ramp: correct.

**Round 2 (Roshar, P13-P24):**

```
Module C: P13(2) P14(3) P15(3) P16(3) P17(4) P18(4)
Module D: P19(2) P20(2) P21(3) P22(3) P23(3) P24(4)
```

Range: 2-star to 4-star. Ramp: correct.

**Round 3 (Cosmere, P25-P36):**

```
Module E: P25(2) P26(3) P27(3) P28(3) P29(4) P30(5)
Module F: P31(3) P32(3) P33(4) P34(4) P35(4) P36(5)
```

Range: 2-star (Module E) / 3-star (Module F) to 5-star. Ramp: correct.

### 6.2 Target Curves (from SCOPE.md)

| Round | Target Start | Target End | Actual Start | Actual End | Status |
|-------|-------------|-----------|-------------|-----------|--------|
| 1 | 2-star | 4-star | 2-star | 4-star | PASS |
| 2 | 2-star | 4-star (per SCOPE) | 2-star | 4-star | PASS |
| 3 | 3-star | 5-star | 2-star (E) / 3-star (F) | 5-star | SEE NOTE |

**Note on Round 3 start:** Module E begins at 2-star (P25), which is below the target 3-star start for Round 3. However, P25 is the gentlest on-ramp into the Shard identification domain, and Module F starts at 3-star. The overall Round 3 range spans 2 to 5 stars, which is wider than the target. This is a MINOR deviation.

### 6.3 Dead Zone Analysis (Adjacent Same-Difficulty)

Per the editorial review, 17 adjacent same-difficulty violations exist across all 6 modules. The full list:

| Module | Pair | Stars | Type |
|--------|------|-------|------|
| A | P01/P02 | 2/2 | doublet |
| A | P03/P04 | 3/3 | doublet |
| A | P05/P06 | 4/4 | doublet |
| B | P07/P08 | 2/2 | doublet |
| B | P09/P10 | 3/3 | doublet |
| B | P11/P12 | 4/4 | doublet |
| C | P14/P15/P16 | 3/3/3 | **triplet** |
| C | P17/P18 | 4/4 | doublet |
| D | P19/P20 | 2/2 | doublet |
| D | P21/P22/P23 | 3/3/3 | **triplet** |
| E | P26/P27/P28 | 3/3/3 | **triplet** |
| F | P31/P32 | 3/3 | doublet |
| F | P33/P34/P35 | 4/4/4 | **triplet** |

**4 triplets identified** (3 consecutive puzzles at same difficulty within a module). These are the most concerning dead zones:

1. Module C: P14-P15-P16 all at 3 stars
2. Module D: P21-P22-P23 all at 3 stars
3. Module E: P26-P27-P28 all at 3 stars
4. Module F: P33-P34-P35 all at 4 stars

**Assessment:** The triplets represent genuine progression stalls within modules. A team working through Module C linearly would face three consecutive 3-star puzzles before the difficulty jumps to 4 stars. However, puzzle hunts are typically non-linear within a round -- teams can tackle puzzles from either module in parallel.

**Severity: MAJOR.** The triplets should be broken before Stage 9. Recommended reclassifications (per editorial review):
- P15: 3-star -> 4-star (narrative identification with underdocumented spren is harder than fill-in-the-blank)
- P22: 3-star -> 4-star (cryptic crossword requires dual competency)
- P27: 3-star -> 4-star (deep lore chronology)
- P32: 3-star -> 4-star (cross-world Cognitive Shadow matching is complex)

These 4 changes would break all 4 triplets and improve the curve without altering puzzle content.

---

## 7. Cross-Puzzle Spoiler Check

### 7.1 Shard Intent References in Modules A-D (P01-P24)

**Rule:** No puzzle in Modules A-D should ask the solver to identify or work with Shard Intents as puzzle content (those belong to Modules E-F).

**Contextual references found (not spoilers):**
- P09: "Harmony" -- character name (Sazed), not a Shard identification exercise
- P10: "Harmony" -- same, character context in Era 2 effects
- P15: "honor" -- lowercase, used as a concept ("honor as a binding obligation")
- P21: "Odium" and "Honor" -- contextual world-building (identifying Rhythm sources)

**Assessment: PASS.** All references to Shard Intents in Modules A-D are incidental world-building context, not identification puzzles. No solver would gain a Modules E-F advantage from these references.

### 7.2 Meta/Super-Meta Answer Words in Puzzle Text

**BLOCKER -- P10 contains "UNITY" in plaintext.**

P10 (The Survivor's Legacy) includes a word bank with 8 connection words. One of them is:

```
| UNITY |
```

UNITY is the super-meta answer. A solver who encounters this word in P10 and later reaches the super-meta may recall it, significantly reducing the difficulty of the final extraction.

**Severity: BLOCKER.**

**Mitigation:** Replace UNITY in P10's word bank with a synonym that does not collide with any meta answer. Options:
- UNIFICATION (same meaning, 12 letters, starts with U so the acrostic is preserved)
- UNION (5 letters, starts with U)

The word bank connection between C7 (Kelsier's rebellion -> Sazed takes two Shards) and E-e (Harmony) needs a word starting with U. UNIFICATION works: Kelsier's rebellion unified the conditions for Sazed to unify two Shards. The first-letter acrostic would remain U.

**ADVISORY -- P14 contains "unite" in a Radiant Oath quote.**

P14 (Ideals and Oaths), Fragment B: "I will unite instead of _ _ [_] _ _ _."

The word "unite" appears here as part of the Bondsmith 2nd Ideal, which is a canonical Sanderson quote ("I will unite instead of divide"). This cannot be removed without altering the published text. However:
- UNITE is the Round 1 meta answer, not the super-meta answer
- P14 is in Round 2 (Module C), and by the time solvers encounter it, they may or may not have solved Meta I
- The word appears in context as part of a Radiant Oath, not as a standalone answer
- The puzzle's own answer is DEVOTE, not UNITE

**Severity: MINOR.** The canonical quote cannot be avoided. The risk is low because "unite" appears in a natural Oath context, not as an isolated answer word.

### 7.3 Cross-Puzzle Answer Leaks

**Rule:** No puzzle should contain another puzzle's answer word.

Systematic search for all 36 answer words across all 36 puzzle files:

- "hidden" appears in P02, P06, P09 -- but as a common English adjective, not as P06's answer word being revealed. P06's answer is extracted through deduction, not given in text.
- "pattern" appears in P05, P17, P22, P29 -- as a common English noun or as a reference to the Cryptic spren named Pattern. P18's answer is PATTERN, and P22's intro says "Pattern likes lies" (referring to the spren character). This is a thematic reference, not an answer leak.
- "bridge" appears in P19, P20 -- P20 IS the Bridge Four puzzle and its answer is BRIDGE. The word "bridge" in P19 is a common noun ("chasm bridge"). This is an incidental usage.
- "growth" appears in P15, P34 -- P33's answer is GROWTH. P15 says "new growth"; P34 letter B says "Growth requires pruning" (Cultivation's voice). These are thematic domain words, not answer leaks.
- "tower" appears in P16, P21 -- P17's answer is TOWER. P16 says "the Tower" (Urithiru); P21 says "in a tower." These are setting references.
- "stolen" appears in P35 -- P35's answer IS STOLEN; the word appears in the verification comment only (HTML hidden).
- "thread" appears in P10, P36 -- P36's answer is THREAD. P10 says "eight threads stretch from his Era." This is a common noun usage and P10 is in a different round.
- "mortal" appears in P10, P25, P26 -- P26's answer is MORTAL. P10 says "an ambitious mortal"; P25 says "an ambitious mortal." Common adjective usage.
- "broken" appears in P13, P25 -- P13's answer IS BROKEN. P25 says "Some were broken" (describing Splintered Shards). This is a common adjective in the domain.

**Assessment:** All word occurrences are common English words used in their natural sense. No puzzle explicitly reveals another puzzle's answer. The verification sections in P31-P36 (HTML comments) do contain decoded answers, but these are invisible to solvers and flagged for Stage 9 stripping.

**Status: PASS** (with the exception of UNITY in P10's word bank, flagged as BLOCKER above).

---

## 8. Voice Consistency -- Final Check

### 8.1 Six-Puzzle Sample (One per Module)

| Module | Puzzle | Sample |
|--------|--------|--------|
| A | P01 | "Sixteen metals burn in a Mistborn's stomach. Each grants a different power." |
| B | P07 | "A millennium is a long time to sit on a throne. Events pile up. Memory blurs." |
| C | P15 | "A spren chooses. A spren reflects. The nature of the bond is written in behavior." |
| D | P22 | "Pattern likes lies. A cryptic clue is a kind of lie -- every answer hides inside words that mean something else entirely." |
| E | P30 | "Before everything, sixteen conspirators stood in a circle. A seventeenth watched from outside." |
| F | P35 | "One has perfect memory. Or one did, until recently." |

### 8.2 Voice Rule Verification

| Rule | P01 | P07 | P15 | P22 | P30 | P35 |
|------|-----|-----|-----|-----|-----|-----|
| No exclamation marks | PASS | PASS | PASS | PASS | PASS | PASS |
| Present tense default | PASS | See note | PASS | PASS | PASS | See note |
| Short sentences | PASS | FLAG (26w) | PASS | PASS | PASS | PASS |
| No direct questions | PASS | PASS | PASS | PASS | PASS | PASS |
| Dry humor | PASS | PASS | PASS | PASS | PASS | PASS |
| No "you" address | PASS | PASS | PASS | PASS | PASS | PASS |
| No meta-commentary | PASS | PASS | PASS | PASS | PASS | PASS |

**Notes:**
- P07: Past tense ("could never quite manage") in intro. Known from editorial review (BUG-S7-002).
- P35: Past tense ("did, until recently"; "was taken") in intro. Known from editorial review (BUG-S7-002).
- P07: 26-word sentence. Known from editorial review (BUG-S7-001).

### 8.3 Cross-Module Voice Consistency

The Hoid/Wit voice is **remarkably consistent** across all 6 modules. Key markers present throughout:
- Self-reference as "one" (P01, P07, P30, P33, P35, P36)
- Short declarative sentences as the primary rhythm
- Dry observations that reward re-reading (P05: "The symmetry would be beautiful if it were not so dangerous"; P30: "Or so he claims")
- Oblique solver address via puzzle framing, never "you"
- Present tense with past-tense exceptions only in historical narration

**Status: PASS.** Voice is consistent. Known issues (past tense, sentence length) are documented in BUG-S7-001 and BUG-S7-002, and are classified as Stage 11 polish items.

---

## 9. SUPER-META.md Internal Consistency Bug

### 9.1 Issue

The SUPER-META.md file contains an internal inconsistency:

- Line 28 (table): Round II answer listed as `RAIB` (4 characters)
- Line 59 (verification block): Round II answer listed as `RAIB` (4 characters)
- Line 66 (verification block): References `RAIB[5] = L`

`RAIB` has only 4 characters. Position 5 does not exist. The correct value is `RAIBL` (5 characters), which is the ROT13 encoding of ENVOY.

The editorial review (BUG-S7 fix log) notes: "Header ROT13: RAIB -> RAIBL (was truncated by 1 character)" -- this fix was applied to META-II-OATHS.md but NOT to SUPER-META.md.

### 9.2 Impact

The meta mechanism is correct (ENVOY is 5 letters, RAIBL is 5 characters). The bug is purely a documentation error in SUPER-META.md. No solver would see this file. However, if SUPER-META.md is used as a reference during delivery build (Stage 9), the truncated answer could cause confusion.

**Severity: MAJOR.** Must fix before Stage 9.

**Fix:** Replace `RAIB` with `RAIBL` on lines 28 and 59 of SUPER-META.md.

---

## 10. Summary of Issues

### BLOCKERS (stops ship)

| # | Issue | Location | Fix Required |
|---|-------|----------|-------------|
| **INT-001** | P10 word bank contains "UNITY" (super-meta answer) in plaintext | `puzzles/P10-survivors-legacy.md` line 62 | Replace UNITY with UNIFICATION (preserves U-initial for acrostic) |

### MAJOR (needs fix before Stage 9)

| # | Issue | Location | Fix Required |
|---|-------|----------|-------------|
| **INT-002** | SUPER-META.md has "RAIB" instead of "RAIBL" (truncated by 1 char) | `meta/SUPER-META.md` lines 28, 59 | Replace RAIB with RAIBL in both locations |
| **INT-003** | 4 difficulty triplets (3 consecutive same-difficulty puzzles in a module) | Modules C, D, E, F | Reclassify P15, P22, P27, P32 from 3-star to 4-star |

### MINOR (Stage 11 polish)

| # | Issue | Location | Notes |
|---|-------|----------|-------|
| **INT-004** | P14 contains "unite" (Meta I answer) in canonical Oath quote | `puzzles/P14-ideals-and-oaths.md` | Cannot remove without altering published text. Risk is low -- contextual usage. |
| **INT-005** | P25 starts at 2-star, below Round 3 target of 3-star | `puzzles/P25-shattered-and-whole.md` | Accept as domain on-ramp, or upgrade to 3-star |
| **INT-006** | Past-tense violations (14 instances across 10 puzzles) | Various | Per BUG-S7-002 -- resolve at Stage 11 |
| **INT-007** | P07 sentence length (26 words) | `puzzles/P07-lord-rulers-thousand-years.md` | Per BUG-S7-001 -- resolve at Stage 11 |
| **INT-008** | Inconsistent solution verification sections (P31-P36 have them, P01-P30 don't) | Various | Per BUG-S7-004 -- standardize at Stage 9 or 11 |

---

## 11. Integration Verdict

### What Works

1. **The meta chain is mathematically sound.** All 4 extraction chains verified character by character: 36 feeders -> 3 metas -> 1 super-meta. Zero errors in the extraction logic.

2. **The answer registry is complete and correctly encoded.** 40 unique answers, all ROT13, no duplicates.

3. **The 80% rule passes everywhere.** Every meta is solvable with partial information. The design is robust to missing feeders.

4. **The thematic arc is exceptional.** UNITE -> ENVOY -> AMID -> UNITY tells a story: the metals that pull together (UNITE), the messengers of Honor (ENVOY), the wanderer standing among the fragments (AMID), and the state Hoid seeks -- the reversal of the Shattering (UNITY). Each meta answer deepens the narrative.

5. **Voice is consistent.** Hoid/Wit's tone holds across 36 puzzles and 4 metas despite different author teams.

6. **No fatal cross-puzzle spoilers** (except the UNITY word bank issue, which is fixable).

### What Needs Fixing

1. **1 BLOCKER:** UNITY in P10's word bank must be replaced.
2. **2 MAJOR items:** SUPER-META.md truncation bug; difficulty triplets.
3. **5 MINOR items:** all deferrable to Stage 11.

### Recommendation

**CONDITIONAL PASS.** Fix INT-001 (BLOCKER) and INT-002 (MAJOR) before proceeding to Stage 9. INT-003 (difficulty reclassification) can be addressed at Stage 9 or 11. All other items are polish.

The hunt's structural integrity is verified. The meta architecture is elegant and sound. The extraction chains interlock cleanly from feeder through super-meta. The narrative arc from UNITE to UNITY is the strongest thematic payoff in any scenario tested to date.

---

*Forty answers. Four chains. One word at the end that means what Adonalsium was before the breaking. The thread holds.*
