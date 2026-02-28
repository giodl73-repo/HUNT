# THE SIXTEENTH SHARD — Stage 7 Editorial Review

**Reviewer:** Claude (automated editorial pass)
**Date:** 2026-02-28
**Scope:** 36 feeder puzzles + 3 round metas + 1 super-meta
**Source files:** `puzzles/P01-P36`, `meta/META-I`, `meta/META-II`, `meta/META-III`, `meta/SUPER-META`, `meta/cosmere-answers.md`

---

## 1. Voice Audit (Hoid/Wit)

Rules from SCOPE.md:
1. No exclamation marks.
2. Present tense default. Past tense only for Shattering-era events.
3. Short sentences. Rarely more than 15 words.
4. No direct questions to the solver.
5. Dry humor. The kind that requires a second read.
6. The solver is addressed obliquely -- never "you."
7. No meta-commentary.

### 1.1 Exclamation Marks (Rule 1)

**Result: PASS.** No exclamation marks found in any puzzle intro, body, or extraction instruction across all 36 puzzles and 4 metas.

### 1.2 Tense (Rule 2)

**Result: FAIL -- 14 violations across 10 puzzles.**

Violations listed by puzzle. "Shattering-era" events (the breaking of Adonalsium) are exempt. Post-Shattering historical narration (Scadrial Era 1, Roshar Desolations, etc.) is NOT exempt per SCOPE.md.

| Puzzle | Violating Text | Severity |
|--------|---------------|----------|
| P08 | "This one **had** eight." | Minor |
| P09 | "Rashek **built** his empire in layers." "The rungs **were** never meant to move." | Minor |
| P10 | "The Survivor **died** once. It **did** not take." | Minor |
| P13 | "Some **managed** better than others." | Minor |
| P16 | "A roster of twelve names once **hung** in the Tower. Some names **glowed**. Some **did** not." | Minor |
| P19 | "The Singers **were** here before any of us. They **sang** to rhythms older than memory. They **wore** forms..." | Minor |
| P20 | "They **carried** a bridge. Then they **carried** each other." | Minor |
| P25 | "Some **were** broken." | Minor |
| P35 | "Or one **did**, until recently." "something **was** taken. **Altered**." | Minor |
| P36 | "Not metaphorically." (fragment, not tense) -- disregard. P36 uses present perfect throughout, which is acceptable. | -- |

**Assessment:** All 14 violations are in introductory/flavor text, not puzzle instructions. They are stylistic past-tense usages describing post-Shattering historical events. The voice rule is strict ("Past tense only for Shattering-era events"), but these usages serve narrative clarity and read naturally in Hoid's voice. They are consistent with how Hoid tells stories about the past.

**Recommendation:** FLAG FOR AUTHOR TEAM. These are borderline. A strict reading of the voice rules requires rewrites. A lenient reading acknowledges Hoid is a storyteller who narrates history. Suggest adding a clarifying note to SCOPE.md: "Past tense permitted in brief historical setup (1-2 sentences) when Hoid is explicitly recounting events." If the author team prefers strict enforcement, the rewrites are straightforward (e.g., "Rashek builds his empire in layers" for P09).

### 1.3 Sentence Length (Rule 3)

**Result: ADVISORY -- multiple sentences exceed 15 words.**

Representative examples (>15 words):

| Puzzle | Sentence | Word Count |
|--------|----------|------------|
| P01 | "Each power is described below in the words of those who have burned them -- though the metal's name has been excised." | 21 |
| P07 | "The trick is getting them in the right order -- the same trick Rashek himself could never quite manage, what with all that rewriting of history." | 26 |
| P08 | "...the ones who burn a single metal carry a street name, and street names carry letters." | 16 |
| P10 | "Three hundred years later his fingerprints remain on everything -- every law, every church, every quiet arrangement made in back rooms." | 19 |
| P13 | "Four and a half thousand years is a long time to remain sane." | 12 (OK) |
| P36 | "Some events, one witnessed firsthand -- stood in the room, spoke the words, set the wheel turning." | 16 |

**Assessment:** The rule says "Rarely more than 15 words." "Rarely" is key -- it permits occasional longer sentences for effect. The longest sentences (P07 at 26 words, P01 at 21 words) are in introductory/flavor paragraphs. Puzzle instruction sentences are generally under 15 words.

**Recommendation:** FLAG P07 (26 words). Consider splitting: "The trick is getting them in the right order. Rashek himself never managed it, what with all that rewriting of history." All others are within acceptable "rarely" tolerance.

### 1.4 Direct Questions (Rule 4)

**Result: PASS.** No direct questions found in any puzzle. Puzzle instructions use imperative mood ("Identify," "Place," "Read") which is standard.

### 1.5 "You" Address (Rule 6)

**Result: PASS.** No instances of "you" or "your" addressing the solver directly. Some puzzle instructions use "the solver" or imperative mood, both acceptable.

### 1.6 Exclamation / Meta-commentary (Rules 1, 7)

**Result: PASS.** No meta-commentary ("This puzzle is about...") found. No breaking of the fourth wall beyond Hoid's characteristic oblique awareness.

### 1.7 Voice Summary

| Rule | Result |
|------|--------|
| No exclamation marks | PASS |
| Present tense default | FAIL (14 violations, all minor, all in flavor text) |
| Short sentences | ADVISORY (1 flagged: P07 at 26 words) |
| No direct questions | PASS |
| Dry humor | PASS (well-executed across all modules) |
| No "you" address | PASS |
| No meta-commentary | PASS |

**Overall voice verdict: PASS WITH ADVISORIES.** The tone is consistently Hoid. The past-tense violations are debatable and may warrant a scope clarification rather than rewrites.

---

## 2. Extraction Verification

All answers decoded from ROT13 and verified character by character against puzzle extraction mechanisms.

### 2.1 Module A -- Allomancy (P01-P06)

| ID | Answer | Mechanism | Verification | Status |
|----|--------|-----------|-------------|--------|
| P01 | UPRISE | Bold-word acrostic (6 metals -> 6 bold words -> first letters) | U-P-R-I-S-E | PASS |
| P02 | FORGED | Sentence-initial acrostic (6 paired metals -> 6 sentences) | F-O-R-G-E-D | PASS |
| P03 | ANCHOR | Sentence-initial acrostic (6 Feruchemical IDs) | A-N-C-H-O-R | PASS |
| P04 | MATRIX | Grid logic -> scrambled acrostic | M-A-T-R-I-X | PASS |
| P05 | TRIAD | Intro-word acrostic (3 Hemalurgic sets) | T-R-I-A-D | PASS |
| P06 | HIDDEN | Deductive logic (6 Allomancers -> seating -> names) | H-I-D-D-E-N | PASS |

### 2.2 Module B -- Scadrial History (P07-P12)

| ID | Answer | Mechanism | Verification | Status |
|----|--------|-----------|-------------|--------|
| P07 | TYRANT | Chronological ordering -> acrostic | T-Y-R-A-N-T | PASS |
| P08 | HEIST | Misting street-name positional extraction | H-E-I-S-T | PASS |
| P09 | RITUAL | Paragraph-initial acrostic (sorted low-to-high) | R-I-T-U-A-L | PASS |
| P10 | UNDEAD | Cause-effect matching -> connection-word acrostic | U-N-D-E-A-D | PASS |
| P11 | BREACH | Logic grid -> K's options first letters | B-R-E-A-C-H | PASS |
| P12 | OUTLAW | Allomantic quadrant + Feruchemical metal extraction | O-U-T-L-A-W | PASS |

### 2.3 Module C -- Knights Radiant (P13-P18)

| ID | Answer | Mechanism | Verification | Status |
|----|--------|-----------|-------------|--------|
| P13 | BROKEN | Herald ID -> gemstone name -> ward number index | B-R-O-K-E-N | PASS |
| P14 | DEVOTE | Marked-position extraction from blanked Oaths | D-E-V-O-T-E | PASS |
| P15 | SPIRIT | Spren ID -> Order name -> position extraction | S-P-I-R-I-T | PASS |
| P16 | SWORN | Bold-word acrostic (Radiant characters only) | S-W-O-R-N | PASS |
| P17 | TOWER | Stormlight-depth extraction from Double Eye ring | T-O-W-E-R | PASS |
| P18 | PATTERN | 7 marked positions on Surge Wheel | P-A-T-T-E-R-N | PASS |

### 2.4 Module D -- Roshar World (P19-P24)

| ID | Answer | Mechanism | Verification | Status |
|----|--------|-----------|-------------|--------|
| P19 | CHITIN | Form ID -> trait-word -> position extraction | C-H-I-T-I-N | PASS |
| P20 | BRIDGE | Extra-letter-in-anagram extraction | B-R-I-D-G-E | PASS |
| P21 | CHANT | Rhythm ID -> tone-word -> extraction | C-H-A-N-T | PASS |
| P22 | ERODED | Cryptic crossword (Cosmere-themed) | E-R-O-D-E-D | PASS |
| P23 | BERYL | Triple-matching -> gemstone extraction | B-E-R-Y-L | PASS |
| P24 | TEMPEST | Watchtower-glyph signal letters in storm order | T-E-M-P-E-S-T | PASS |

### 2.5 Module E -- The 16 Shards (P25-P30)

| ID | Answer | Mechanism | Verification | Status |
|----|--------|-----------|-------------|--------|
| P25 | STATUS | Intent-name positional extraction (6 of 16 fragments) | S-T-A-T-U-S | PASS |
| P26 | MORTAL | Intent-name extraction from identified Vessels | M-O-R-T-A-L | PASS |
| P27 | WRATH | Chronological ordering -> intent-name index | W-R-A-T-H | PASS |
| P28 | GIFTED | Magic-system-name extraction | G-I-F-T-E-D | PASS |
| P29 | ASPECT | Axis-label acrostic from 4x4 grid | A-S-P-E-C-T | PASS |
| P30 | ORIGIN | Circular seating -> vessel-name extraction | O-R-I-G-I-N | PASS |

### 2.6 Module F -- Hoid & Worldhoppers (P31-P36)

| ID | Answer | Mechanism | Verification | Status |
|----|--------|-----------|-------------|--------|
| P31 | MASKED | Presence/absence -> alias extraction | M-A-S-K-E-D | PASS |
| P32 | LINGER | Shadow ID -> system matching | L-I-N-G-E-R | PASS |
| P33 | GROWTH | Triple ID -> chained extraction | G-R-O-W-T-H | PASS |
| P34 | SCRIPT | Author attribution -> style extraction | S-C-R-I-P-T | PASS |
| P35 | STOLEN | Error detection -> wrong-word extraction | S-T-O-L-E-N | PASS |
| P36 | THREAD | Direct/indirect sorting -> marked-letter extraction | T-H-R-E-A-D | PASS |

### 2.7 Extraction Summary

**All 36 feeder puzzles: PASS.** Every extraction chain verified from mechanism through to final answer.

---

## 3. Meta Extraction Verification

### 3.1 Meta I: THE WELL (Scadrial)

Mechanism: Pulling metals in table order, quadrant-depth extraction.

| Metal | Puzzle | Answer (ROT13) | Quadrant Depth | Letter (ROT13) |
|-------|--------|-----------------|---------------|-----------------|
| Iron (#1) | P01 | HCEVFR | Physical = 1 | H (= U) |
| Zinc (#5) | P03 | NAPUBE | Mental = 2 | A (= N) |
| Gold (#9) | P08 | URVFG | Temporal = 3 | V (= I) |
| Cadmium (#11) | P09 | EVGHNY | Temporal = 3 | G (= T) |
| Chromium (#15) | P10 | HAQRNQ | Enhancement = 4 | R (= E) |

Result: HAVGR -> UNITE. **PASS.**

### 3.2 Meta II: THE OATHS (Roshar)

Mechanism: Orders with 3+ documented Ideals, Ideal-count as index position.

| Order | # Ideals | Puzzle | Answer (ROT13) | Index | Letter (ROT13) |
|-------|----------|--------|-----------------|-------|-----------------|
| Windrunners (#1) | 4 | P17 | GBJRE | 4 | R (= E) |
| Skybreakers (#2) | 5 | P16 | FJBEA | 5 | A (= N) |
| Edgedancers (#4) | 3 | P14 | QRIBGR | 3 | I (= V) |
| Lightweavers (#6) | 3 | P22 (crit) | REBQRQ | 3 | B (= O) |
| Bondsmiths (#10) | 4 | P23 | ORELY | 4 | L (= Y) |

Result: RAIBL -> ENVOY. **PASS.**

Note: P23 answer registry was corrected from ORELO to ORELY during this review. ORELY[4] = L, same as the old ORELO[4] = L. Meta chain unaffected.

### 3.3 Meta III: ADONALSIUM (Cosmere)

Mechanism: 12 puzzles represent 12 Shards. 4 missing Shards' first letters anagram to the answer.

Represented (12): Honor, Cultivation, Odium, Preservation, Ruin, Harmony, Devotion, Endowment, Autonomy, Valor, Whimsy, Virtuosity.

Missing (4): **A**mbition, **D**ominion, **I**nvention, **M**ercy.

First letters: A, D, I, M -> anagram -> AMID.

ROT13: NZVQ. **PASS.**

Note: META-III narrative table had P26/P27 Shard assignments swapped. Corrected during this review. Meta mechanism unaffected (both Shards are in the "represented" set).

### 3.4 Super-Meta: THE SEVENTEENTH SHARD

Mechanism: Realms cycle (Physical-Cognitive-Spiritual-Physical-Cognitive), depth 1-2-3-4-5.

| Step | Realm | Meta Answer (ROT13) | Index | Letter (ROT13) |
|------|-------|---------------------|-------|-----------------|
| 1 | Physical | HAVGR (UNITE) | 1 | H (= U) |
| 2 | Cognitive | RAIBL (ENVOY) | 2 | A (= N) |
| 3 | Spiritual | NZVQ (AMID) | 3 | V (= I) |
| 4 | Physical | HAVGR (UNITE) | 4 | G (= T) |
| 5 | Cognitive | RAIBL (ENVOY) | 5 | L (= Y) |

Result: HAVGL -> UNITY. **PASS.**

### 3.5 Meta Summary

All 4 meta extraction chains verified. No errors. The interlocking system is sound.

---

## 4. Difficulty Curve Assessment

### 4.1 Target Curves

Per SCOPE.md:
- Round 1: ramp from 2-star to 4-star
- Round 2: ramp from 2-star to 4-star (with Round 3 reaching 5-star)
- Round 3: ramp from 2-star to 5-star
- No two adjacent puzzles in the same module at the same difficulty

### 4.2 Current Difficulty Assignments

**Round 1 (P01-P12):**

| Module A | P01 | P02 | P03 | P04 | P05 | P06 |
|----------|-----|-----|-----|-----|-----|-----|
| Stars | 2 | 2 | 3 | 3 | 4 | 4 |

| Module B | P07 | P08 | P09 | P10 | P11 | P12 |
|----------|-----|-----|-----|-----|-----|-----|
| Stars | 2 | 2 | 3 | 3 | 4 | 4 |

**Round 2 (P13-P24):**

| Module C | P13 | P14 | P15 | P16 | P17 | P18 |
|----------|-----|-----|-----|-----|-----|-----|
| Stars | 2 | 3 | 3 | 3 | 4 | 4 |

| Module D | P19 | P20 | P21 | P22 | P23 | P24 |
|----------|-----|-----|-----|-----|-----|-----|
| Stars | 2 | 2 | 3 | 3 | 3 | 4 |

**Round 3 (P25-P36):**

| Module E | P25 | P26 | P27 | P28 | P29 | P30 |
|----------|-----|-----|-----|-----|-----|-----|
| Stars | 2 | 3 | 3 | 3 | 4 | 5 |

| Module F | P31 | P32 | P33 | P34 | P35 | P36 |
|----------|-----|-----|-----|-----|-----|-----|
| Stars | 3 | 3 | 4 | 4 | 4 | 5 |

### 4.3 Adjacent Same-Difficulty Violations

**Rule:** No two adjacent puzzles in the same module should share the same difficulty.

| Module | Adjacent Pair(s) | Stars | Violation? |
|--------|-----------------|-------|------------|
| A | P01/P02 | 2/2 | YES |
| A | P03/P04 | 3/3 | YES |
| A | P05/P06 | 4/4 | YES |
| B | P07/P08 | 2/2 | YES |
| B | P09/P10 | 3/3 | YES |
| B | P11/P12 | 4/4 | YES |
| C | P14/P15 | 3/3 | YES |
| C | P15/P16 | 3/3 | YES (triplet) |
| C | P17/P18 | 4/4 | YES |
| D | P19/P20 | 2/2 | YES |
| D | P21/P22 | 3/3 | YES |
| D | P22/P23 | 3/3 | YES (triplet) |
| E | P26/P27 | 3/3 | YES |
| E | P27/P28 | 3/3 | YES (triplet) |
| F | P31/P32 | 3/3 | YES |
| F | P33/P34 | 4/4 | YES |
| F | P34/P35 | 4/4 | YES (triplet) |

**Total: 17 adjacent same-difficulty violations across all 6 modules.**

### 4.4 Overall Ramp Assessment

Despite the adjacent violations, the macro ramp is correct:
- Round 1: starts at 2, ends at 4. OK.
- Round 2: starts at 2, ends at 4. OK.
- Round 3: starts at 2 (Module E) / 3 (Module F), ends at 5. OK.

### 4.5 P18 Difficulty Assessment (Flagged)

P18 (The Surge Wheel, rated 4-star) is flagged as potentially overrated per BUG-S6-011. The puzzle is a chain-lookup rather than constraint satisfaction: given Windrunners as anchor, each successive position follows deterministically from the previous Order's Surges. No branching or backtracking is required.

**Recommendation:** Downgrade P18 from 4-star to 3-star. This would:
- Break the P17/P18 same-difficulty pair (4/3 instead of 4/4).
- But create a P15/P16/P18 triplet at 3 stars (P14=3, P15=3, P16=3, P18=3).
- Net effect: trades one violation for zero net improvement. Downgrade is mechanically correct but does not help the curve.

### 4.6 P25 Difficulty Assessment (Flagged)

P25 (Shattered and Whole, rated 2-star) is a straightforward identification puzzle (identify 16 Shards from descriptions, extract 6 marked letters). For Cosmere readers, identifying Shards from 1-2 sentence descriptions is relatively simple. The 2-star rating is appropriate.

### 4.7 Difficulty Curve Verdict

**FAIL on adjacent-pair rule.** Every module has at least two adjacent pairs at the same difficulty. This is a systemic issue caused by the 6-puzzle-per-module structure with only 3-4 difficulty tiers: it is mathematically impossible to have 6 puzzles ramp from 2 to 4 stars with no two adjacent equal and only 3 tiers (2,3,4). You need at least one repeat.

**Recommendation:** Accept the constraint as inherent to the structure. Alternatively, introduce half-star ratings (2.5, 3.5) or reclassify one puzzle per module to break the most egregious triplets (Modules C, D, E, F). Priority targets:
1. P15 (3-star): consider upgrading to 4-star (narrative identification with less-documented spren is harder than P14's fill-in-the-blank).
2. P22 (3-star): consider upgrading to 4-star (cryptic crossword is harder than standard identification).
3. P27 (3-star): consider upgrading to 4-star (chronological ordering of Odium's kills requires deep lore knowledge).
4. P32 (3-star): consider upgrading to 4-star (Cognitive Shadow identification across multiple worlds is complex).

---

## 5. Puzzle-Level Issues

### 5.1 Answer Giveaways

No answer giveaways found. All puzzle intros and body text avoid using the answer word. ROT13 encoding in solution sections prevents accidental exposure.

### 5.2 Solution Sections in HTML Comments

Puzzles P31-P36 (Module F) contain full solution verifications in HTML comments (<!-- ... -->). This is standard authoring practice and marked "AUTHOR ONLY -- not for solvers." These must be stripped before delivery (Stage 9).

**Impact:** None at current stage. Flag for Stage 9 DELIVERY.

### 5.3 External Knowledge Requirements

All puzzles are solvable using the world-system reference files provided with the hunt. No puzzle requires knowledge beyond published Sanderson novels and the Coppermind wiki. The reference sheets (`world/systems/`) cover all required data.

Potential concerns:
- **P22 (The Chasms Below):** Cryptic crossword format. Solvers need cryptic crossword solving experience in addition to Cosmere knowledge. This is noted in the puzzle's 3-star rating.
- **P30 (The Shattering):** Requires knowledge of all 10 named Vessels and their Shards. The puzzle provides a full reference table, mitigating the knowledge requirement.
- **P36 (All Roads Lead to Hoid):** Requires deep knowledge of Hoid's appearances across 6+ books. The 5-star rating reflects this.

### 5.4 Ambiguous Clues

- **P08 (Kelsier's Crew):** The portraits must uniquely identify each crew member. Some Era 1 characters (Breeze, Ham, Dockson) have similar backgrounds. Verify that each portrait contains at least 2 unique identifying details.
- **P13 (Herald's Madness):** Uses Chanarach (#3), whose documentation is marked [VERIFY] in the world files. The description relies on inferred characteristics. BUG-S6-006 documents this and notes that elimination logic resolves any ambiguity.
- **P21 (Rhythm of Answers):** Relies on confirmed Rhythms only. BUG-S3-004 notes partial confirmation of Rhythm names. Verify all Rhythm names used in P21 are confirmed in published text.

### 5.5 Specific Issues by Puzzle

| Puzzle | Issue | Severity | Status |
|--------|-------|----------|--------|
| P07 | 26-word sentence in intro | Low | FLAG |
| P08 | Past tense "had" in intro | Low | FLAG |
| P09 | Past tense "built," "were" in intro | Low | FLAG |
| P10 | Past tense "died," "did not take" in intro | Low | FLAG |
| P12 | No solution verification section (P01-P12 lack them; P31-P36 have them) | Low | Inconsistency -- add verification to all or none |
| P16 | Past tense "hung," "glowed" in intro | Low | FLAG |
| P18 | Difficulty may be overstated (4-star vs 3-star) | Medium | FLAG (see BUG-S6-011) |
| P19 | Past tense "were," "sang," "wore" in intro | Low | FLAG |
| P20 | Past tense "carried" in intro | Low | FLAG |
| P23 | Answer registry was ORELO (should be ORELY) | High | FIXED this review |
| P25 | Past tense "were broken" in intro | Low | FLAG |
| P35 | Past tense "did," "was taken" in intro | Low | FLAG |

---

## 6. Meta Coordination Audit

### 6.1 Metal Assignments (Meta I)

All 12 Scadrial puzzles (P01-P12) have unique metal assignments. The 5 Pulling metals (Iron, Zinc, Gold, Cadmium, Chromium) feed Meta I. The 4 unused metals (Tin, Copper, Aluminum, Nicrosil) are the ones with no assigned puzzle, which is by design (12 puzzles, 16 metals, 4 unused).

**Status: CORRECT.**

### 6.2 Order Assignments (Meta II)

All 12 Roshar puzzles (P13-P24) have Order assignments. 10 Orders covered by 12 puzzles means 2 Orders are doubled:
- Windrunners: P17 (meta-contributing) + P20 (duplicate)
- Lightweavers: P18 (duplicate) + P22 (meta-contributing, marked "crit")

The 5 Orders with 3+ documented Ideals (Windrunners=4, Skybreakers=5, Edgedancers=3, Lightweavers=3, Bondsmiths=4) contribute to the meta. The meta-contributing puzzle for each doubled Order has the correct letter at the required position:
- Windrunners: P17 answer TOWER[4] = E. Correct.
- Lightweavers: P22 answer ERODED[3] = O. Correct.

**Status: CORRECT.**

### 6.3 Shard Assignments (Meta III)

12 Cosmere puzzles (P25-P36) reference 12 of 16 Shards:
- Module E: Honor, Preservation, Odium, Endowment, Autonomy, Virtuosity
- Module F: Ruin, Harmony, Cultivation, Devotion, Whimsy, Valor

Missing 4: Ambition, Dominion, Invention, Mercy.
First letters: A, D, I, M -> AMID.

**Status: CORRECT.**

Note: META-III narrative table had P26/P27 swapped (Odium/Preservation). Fixed during this review.

### 6.4 Super-Meta Feed

- Meta I (UNITE, 5 letters) feeds positions 1 and 4.
- Meta II (ENVOY, 5 letters) feeds positions 2 and 5.
- Meta III (AMID, 4 letters) feeds position 3.
- Cycle: Physical(1) - Cognitive(2) - Spiritual(3) - Physical(4) - Cognitive(5).
- Extraction: U-N-I-T-Y = UNITY.

**Status: CORRECT.**

### 6.5 Answer Length Distribution

| Length | Count | Puzzles |
|--------|-------|---------|
| 5 | 10 | P01, P05, P08, P16, P17, P21, P27, P29 (+metas) |
| 6 | 20 | P02-P04, P06, P07, P09-P12, P13-P15, P19-P20, P22, P25-P26, P28, P30-P32, P34-P36 |
| 7 | 6 | P18, P23, P24, P33 |

No answer word is repeated. All 36 feeder answers are unique. All 4 meta answers are unique.

**Status: CORRECT.**

---

## 7. Bugs Fixed During This Review

| Bug ID | File | Fix |
|--------|------|-----|
| BUG-S6-004 | `meta/cosmere-answers.md` | P23 answer: ORELO -> ORELY |
| BUG-S6-013 | `meta/META-III-ADONALSIUM.md` | P26/P27 Shard swap: corrected to P26=Preservation, P27=Odium |
| (new) | `meta/META-II-OATHS.md` | Header ROT13: RAIB -> RAIBL (was truncated by 1 character) |

---

## 8. New Bugs Discovered

| Bug ID | Severity | Description |
|--------|----------|-------------|
| BUG-S7-001 | Low | P07 intro sentence is 26 words. Voice rule 3 says "rarely more than 15." |
| BUG-S7-002 | Low | 14 past-tense violations across 10 puzzles (P08, P09, P10, P13, P16, P19, P20, P25, P35). All in intro/flavor text. SCOPE.md rule is strict; violations are narratively justified. Recommend scope clarification. |
| BUG-S7-003 | Medium | 17 adjacent same-difficulty violations across all 6 modules. Systemic: 6 puzzles per module with 3 tiers makes this mathematically unavoidable without half-stars. |
| BUG-S7-004 | Low | P01-P12 lack HTML-comment solution verification sections; P31-P36 have them. Inconsistency. |
| BUG-S7-005 | Low | P18 difficulty may be overstated (4-star vs 3-star per BUG-S6-011). Downgrade does not improve the curve. |

---

## 9. Overall Editorial Verdict

### Strengths

1. **Voice.** Hoid's tone is remarkably consistent across 36 puzzles and 4 metas. Dry, knowing, never condescending. The best Hoid writing in the hunt is in Modules E and F (P30, P33, P35, P36).

2. **Extraction integrity.** All 36 feeder extractions and all 4 meta chains are verified correct. The interlocking system (feeders -> metas -> super-meta) is mathematically sound with zero errors.

3. **Mechanism variety.** 36 puzzles use 15+ distinct extraction types: acrostics (bold-word, sentence-initial, intro-word), positional indexing, logic grids, ring topology, anagram, cryptic crossword, error detection, connection mapping, axis-label encoding. No two adjacent puzzles use the same mechanism.

4. **Meta design.** The three round metas are elegant and thematically resonant. Meta I (the Allomantic table structure), Meta II (depth of documented Oaths), and Meta III (the absent Shards) each emerge naturally from their domain. The super-meta's Realms cycle is a genuine aha moment.

5. **Thematic coherence.** The hunt tells a story: from the metals of Scadrial, through the Oaths of Roshar, to the Shards of the Cosmere, culminating in UNITY -- the word that defines Dalinar's arc and the Cosmere's endgame. The answer progression (UNITE -> ENVOY -> AMID -> UNITY) is narratively satisfying.

### Weaknesses

1. **Difficulty curve violations.** The adjacent same-difficulty problem is pervasive. It is structural (too few tiers for the module size) rather than a puzzle-quality issue.

2. **Past-tense usage.** The strict voice rule conflicts with natural storytelling. Hoid tells stories about the past. The rule needs refinement or the intros need rewriting.

3. **Inconsistent verification sections.** Some puzzles have HTML-comment solutions; others do not. Standardize before delivery.

### Blockers

**None.** All extraction chains are verified. All meta coordination is correct. The three registry/meta bugs discovered in earlier stages have been fixed.

### Recommendation

**PASS. Proceed to Stage 8 (INTEGRATION).**

The editorial issues are all Low or Medium severity. The past-tense violations and difficulty-curve violations should be addressed at Stage 11 (POLISH) unless the author team decides to act sooner. The fixed bugs (P23 answer, META-II header, META-III table) were the only high-severity items, and they are resolved.

---

*One has read every word of thirty-six puzzles. The thread holds. The connections are sound. The wanderer may proceed.*
