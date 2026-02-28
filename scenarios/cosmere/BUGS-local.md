# BUGS-local — THE SIXTEENTH SHARD

Local bug tracker. Do NOT write to `../../BUGS.md`.

---

## Stage 3 Bugs

### BUG-S3-001: Allomantic symbol visual assets missing
**Severity:** Medium
**Discovered:** Stage 3 (Pool candidate B-10 Broadsheet Ciphers)
**Description:** The Allomantic symbols (Steel Alphabet) are referenced in `allomancy.md` as "a visual encoding layer for puzzles" but no visual representations are included in the data files. Any puzzle using these symbols (B-10, potential grid overlays) would need external assets.
**Impact:** B-10 was cut from the recommended set partly for this reason.
**Mitigation:** If B-10 is resurrected at Stage 4, create or source symbol assets and add to `world/assets/`.

### BUG-S3-002: Hemalurgic steal data has multiple [VERIFY] flags
**Severity:** Medium
**Discovered:** Stage 3 (Pool candidate A-05 Three Arts One Spike, A-11 Inquisitor's Spikes)
**Description:** The Hemalurgic Steal column in `allomancy.md` has [VERIFY] flags on 7 of 16 metals (Zinc, Brass, Copper, Bronze, Gold, Electrum, Chromium, Nicrosil, Aluminum, Duralumin). This limits any puzzle that requires the full 16-metal Hemalurgic data.
**Impact:** A-05 is in the recommended set but must be restricted to the Physical quadrant (confirmed data). A-11 was cut.
**Mitigation:** Verify all Hemalurgic data against Coppermind before Stage 6 authoring.

### BUG-S3-003: Fused types poorly documented
**Severity:** Low
**Discovered:** Stage 3 (Pool candidates D-06 The Unmade, D-08 Voidbringers)
**Description:** The Fused table in `roshar.md` has [VERIFY] on 6 of 8 named Fused types. Only Shanay-im (Gravitation) and Nex-im (Division) are confirmed. The Unmade are even less documented — only mentioned in passing.
**Impact:** D-06 and D-08 were both cut from the recommended set.
**Mitigation:** If Fused/Unmade puzzles are desired, extend `roshar.md` with verified data before Stage 6.

### BUG-S3-004: Singer Rhythm names partially unconfirmed
**Severity:** Medium
**Discovered:** Stage 3 (Pool candidate D-01 Rhythm of Answers)
**Description:** The Rhythms section of `roshar.md` names Peace, Awe, and Anxiety but notes others with [VERIFY]. The title Rhythm of War (a combined-Light rhythm) is confirmed. D-01 is in the recommended set.
**Impact:** D-01 must be restricted to confirmed Rhythms or the data file must be extended.
**Mitigation:** Verify full Rhythm list against Coppermind. Known from published text: Rhythm of Peace, Rhythm of Awe, Rhythm of Anxiety, Rhythm of Appreciation, Rhythm of Resolve, Rhythm of the Lost, Rhythm of Joy, Rhythm of Destruction, Rhythm of Command, Rhythm of War. Confirm before authoring.

### BUG-S3-005: E-01 / Meta III mechanism overlap
**Severity:** Low
**Discovered:** Stage 3 (Pool candidate E-01 The Sixteen Intents vs Round Meta III ADONALSIUM)
**Description:** E-01 (word search with 4 missing Shards) uses the same "identify missing Shards" mechanism as Round Meta III (12 answers reference 12 Shards, 4 missing Shards spell the meta answer). Risk of redundancy.
**Impact:** Both are in the plan. Must differentiate at Stage 4/5.
**Mitigation:** E-01 should use a different extraction from the missing Shards than the meta (e.g., E-01 uses first letters, meta uses full names or a different property). Or replace E-01 with E-01's alternate (E-10 Adonalsium's Name).

### BUG-S3-006: Toolkit gap — no template for ring-topology puzzles
**Severity:** Low
**Discovered:** Stage 3 (Pool candidates C-01, C-05 — both use the Double Eye ring)
**Description:** The toolkit's `templates/` directory likely has no template for ring/circular constraint puzzles. C-01 and C-05 both require circular placement logic.
**Impact:** Author must construct from scratch at Stage 6.
**Mitigation:** Consider adding a ring-topology puzzle template to the toolkit.

---

## Stage 6 Bugs (Module D Authoring — Team Delta)

### BUG-S6-001: P19 Singer Forms — no 'H' or 'I' in form names
**Severity:** Medium
**Discovered:** Stage 6 (authoring P19)
**Description:** The answer CHITIN requires letters C, H, I, T, I, N. However, no Singer form name (Dullform, Mateform, Workform, Warform, Nimbleform, Scholarform, Stormform, Envoyform, Slaveform) contains the letter 'I' except Nimbleform, and only Scholarform contains 'H'. Two I's are needed but only one form provides I. Indexing directly into form names cannot produce CHITIN.
**Impact:** The brief's stated extraction ("take the Nth letter of the form name") is infeasible for the assigned answer.
**Resolution:** P19 uses a trait-word extraction instead — each form description includes a bolded keyword, and the index extracts from the keyword rather than the form name. This preserves the identification + indexing mechanic while enabling the correct answer.

### BUG-S6-002: P20 Bridge Four — no 'B' in any listed member name
**Severity:** Medium
**Discovered:** Stage 6 (authoring P20)
**Description:** The answer BRIDGE requires the letter B. None of the 9 Bridge Four members listed in roshar.md (Kaladin, Rock, Teft, Sigzil, Skar, Drehy, Rlain, Lopen, Moash) contain the letter B in their name. Direct indexing into member names cannot produce B.
**Impact:** The brief's stated extraction ("take the specified letter from the unscrambled name") cannot produce the answer.
**Resolution:** P20 uses an extra-letter-in-anagram mechanic. Six of nine scrambled names contain one stowaway letter. The six extra letters spell BRIDGE. This preserves the anagram + extraction theme while enabling the correct answer.

### BUG-S6-003: P21 Rhythms — no 'H' in any confirmed Rhythm name
**Severity:** Medium
**Discovered:** Stage 6 (authoring P21)
**Description:** The answer CHANT requires the letter H. No confirmed Singer Rhythm name (Peace, Awe, Anxiety, Appreciation, Resolve, Joy, Destruction, Command, War) contains H. Indexing into Rhythm names cannot produce H.
**Impact:** The brief's stated extraction ("take a specified letter from the Rhythm name") is infeasible.
**Resolution:** P21 uses tone-word extraction — each Rhythm description includes a bolded emotional descriptor, and the index extracts from that word. This preserves the identification + extraction mechanic.

### BUG-S6-004: P23 answer registry typo — ORELO should be ORELY
**Severity:** High
**Discovered:** Stage 6 (authoring P23)
**Description:** The answer registry in `meta/cosmere-answers.md` lists P23's ROT13 answer as ORELO. Decoded, this gives BERYB (not a word). The intended answer is BERYL (a gemstone family), whose ROT13 encoding is ORELY. The last two letters are transposed: ORELO vs ORELY.
**Impact:** The Meta II verification chain uses ORELO[4]=L, which equals ORELY[4]=L, so the meta answer is unaffected. However, the registered answer is technically wrong.
**Mitigation:** Update `meta/cosmere-answers.md` line for P23 from ORELO to ORELY. Verify no downstream dependencies on the specific string ORELO.

### BUG-S6-006: P13 Herald's Madness — no 'B' in well-documented Herald gemstones
**Severity:** Medium
**Discovered:** Stage 6 (authoring P13, Team Gamma)
**Description:** The answer BROKEN requires 'B'. Among the 6 best-documented Heralds (Jezrien, Nale, Shalash, Kalak, Talenel, Ishar), their gemstones (Sapphire, Smokestone, Garnet, Amethyst, Topaz, Heliodor) contain no 'B'. Only Ruby (Chanarach/Dustbringers) has B. This required including Chanarach (#3, with [VERIFY] status) and substituting Shalash for Kalak to obtain N from Garnet.
**Impact:** The brief's stated extraction ("Herald number ordering") was replaced with presentation-order extraction using explicit ward numbers. The Herald set was expanded to include one less-documented Herald.
**Resolution:** P13 uses 6 Heralds (Chanarach, Jezrien, Talenel, Nale, Ishar, Shalash) with explicit ward numbers (3,7,2,4,2,4) that index into gemstone names. Extraction verified: Ruby[3]=B, Sapphire[7]=R, Topaz[2]=O, Smokestone[4]=K, Heliodor[2]=E, Garnet[4]=N = BROKEN.

### BUG-S6-007: P14 Ideals and Oaths — no 'V'-initial word in confirmed Radiant Oaths
**Severity:** High
**Discovered:** Stage 6 (authoring P14, Team Gamma)
**Description:** The answer DEVOTE requires 'V' via acrostic. No confirmed Radiant Oath (First Ideal, Windrunner 2nd-4th, Edgedancer 2nd-3rd, Bondsmith 2nd-3rd, Lightweaver Truths, Skybreaker 2nd-5th) contains any word starting with V. A pure first-letter acrostic cannot produce DEVOTE.
**Impact:** The brief's stated extraction ("first letters of missing words") is infeasible for V.
**Resolution:** P14 uses a marked-position extraction instead — each blanked word has one position marked with [_], and the letter at that position is extracted. The word "divide" (Bondsmith 2nd Ideal) provides V at position 3. This preserves the fill-in-the-blank + identification mechanic while enabling DEVOTE. Ordering rule: Ideal number first, then Order number within same Ideal.

### BUG-S6-008: P15 Spren Bonds — Ashspren behavior underdocumented
**Severity:** Low
**Discovered:** Stage 6 (authoring P15, Team Gamma)
**Description:** Ashspren (Dustbringers) have minimal behavioral documentation in knights-radiant.md or published works. The P15 vignette for Ashspren relies on inferring fire/destruction/control themes from the Order's nature rather than confirmed spren behavior.
**Impact:** The Ashspren vignette may not be uniquely identifiable by all solvers. However, the other 5 spren types in P15 are well-documented, and elimination logic can resolve any ambiguity.
**Mitigation:** The description emphasizes fire/ember imagery and the destruction/restraint duality, which strongly implies the Dustbringer-associated spren. No other spren type matches this profile.

### BUG-S6-009: P16 Radiant Roster — no 'W' in any Radiant character first name
**Severity:** High
**Discovered:** Stage 6 (authoring P16, Team Gamma)
**Description:** The answer SWORN requires 'W'. No confirmed Radiant character (Kaladin, Szeth, Shallan, Jasnah, Dalinar, Navani, Lift, Renarin, Venli, Malata, Zu) has 'W' in their first name. The brief's stated extraction ("Order number indexes into character name") cannot produce W.
**Impact:** The extraction mechanism was redesigned.
**Resolution:** P16 uses a bold-word acrostic — each character description contains one bolded word, and the first letters of the 5 Radiant characters' bold words (sorted by Order number) spell SWORN: Surgeon's(S), White(W), Overlooked(O), Refracted(R), New(N). Non-Radiants' bold words are discarded.

### BUG-S6-010: P17 Double Eye — ring topology conflicts with TOWER extraction
**Severity:** High
**Discovered:** Stage 6 (authoring P17, Team Gamma)
**Description:** The answer TOWER requires 'O'. Only Stonewards[3] and Bondsmiths[2] contain 'O' among Order names. The brief's stated extraction ("Surge pairing position -> letter 1 or 2") creates conflicts: Windrunners and Bondsmiths both need Adhesion for their respective letters (W and O), but Adhesion can only confirm one of them. Similar conflicts arise for T/Stonewards via Tension.
**Impact:** The Surge-pairing extraction is infeasible for TOWER.
**Resolution:** P17 uses a Stormlight-depth extraction — 5 of 10 positions are marked as "infused" with depth values (1 or 2) indicating which letter of the Order name to read. Starting from the Soldier's seat (Stonewards) clockwise: Stonewards[2]=T, Bondsmiths[2]=O, Windrunners[1]=W, Edgedancers[1]=E, Truthwatchers[2]=R = TOWER.

### BUG-S6-011: P18 Surge Wheel — ring placement is mechanical chain-deduction
**Severity:** Low
**Discovered:** Stage 6 (authoring P18, Team Gamma)
**Description:** With Windrunners fixed and all 10 Surge names labeled on the wheel, the remaining 9 positions are determined by simple chaining (each position follows uniquely from its neighbor's Surges). No branching, backtracking, or logical deduction is needed beyond lookup.
**Impact:** The 4-star difficulty rating may be generous. The puzzle functions more as a knowledge test (do you know each Order's two Surges?) than a constraint satisfaction problem.
**Mitigation:** P18 is positioned as the "reconstruction" complement to P17. The pair covers the Double Eye system comprehensively. The extraction (reading 7 marked positions to spell PATTERN) adds a second step. The puzzle may warrant downgrade to 3 stars at editorial review.

### BUG-S6-012: Toolkit gap confirmed — no ring-topology puzzle template
**Severity:** Low
**Discovered:** Stage 6 (authoring P17 and P18, Team Gamma)
**Description:** Confirms BUG-S3-006. Both P17 and P18 required custom ring-topology designs with no template support. The extraction design for both puzzles required extensive iteration due to letter-availability constraints in Order/Gemstone/Surge names.
**Mitigation:** Recommend adding a ring-topology template to `toolkit/templates/` with guidance on checking letter availability in cyclic constraint puzzles before committing to answer words.

### BUG-S6-005: P24 Stormwall — roshar.md location list insufficient for TEMPEST
**Severity:** Low
**Discovered:** Stage 6 (authoring P24)
**Description:** The brief specifies that location initials in storm-arrival order spell the answer TEMPEST. roshar.md's Key Locations table (Urithiru, Shattered Plains, Kholinar, Shinovar, Kharbranth, Thaylen City, Alethkar, Azir) lacks locations starting with E, M, or P. The answer requires two T's, two E's, one M, one P, and one S — impossible from the table alone.
**Impact:** The brief's extraction ("each location's initial letter, in storm-arrival order") requires locations not in roshar.md.
**Resolution:** P24 uses a watchtower-glyph mechanic instead. Each location bears an assigned signal letter. The storm arrival order determines the reading sequence of these letters. This preserves the spatial/directional logic core while decoupling the extraction from location initials. Locations used (Shinovar, Azir, Purelake, Emul, Thaylenah, Alethkar, Shattered Plains) are all canonical Rosharan locations from published novels, though Purelake, Emul, and Thaylenah are not in the roshar.md table.

---

## Stage 6 Bugs (Module E Authoring — Team Epsilon)

### BUG-S6-013: META-III narrative table has P26/P27 Shard assignments swapped
**Severity:** Low
**Discovered:** Stage 6 (post-authoring verification, Team Epsilon)
**Description:** The narrative table in `meta/META-III-ADONALSIUM.md` (lines 98-100) lists P26 as referencing Odium ("The Shard's campaign of destruction") and P27 as referencing Preservation ("The Shard's magic system (Allomancy) and sacrifice"). The authoritative sources — `PUZZLES.md` and the Shard Assignment Map in `meta/cosmere-answers.md` — list P26=Preservation and P27=Odium. The descriptions in META-III are swapped.
**Impact:** Descriptive inconsistency only. The meta mechanism (which Shards are represented vs. missing) is unaffected because both Preservation and Odium are in the "represented" set either way. The 4 missing Shards (Ambition, Dominion, Invention, Mercy) and the meta answer AMID are unchanged.
**Mitigation:** Swap lines 99-100 in `meta/META-III-ADONALSIUM.md` so P26 references Preservation and P27 references Odium. Verify no downstream dependencies.

### BUG-S6-014: P25 brief mechanism mismatch — count-based indexing infeasible
**Severity:** Medium
**Discovered:** Stage 6 (authoring P25, Team Epsilon)
**Description:** The PUZZLES.md brief for P25 specifies "Categorization + count-based indexing" — the solver counts Shards per status category, and that count indexes into the category name. No combination of status-category names and Shard counts produces STATUS. The 4 status categories (Intact, Splintered, Transformed, Unknown) have counts 7, 5, 2, 2 respectively, yielding letters I/n, p/l, a/r, n/k — none spelling STATUS.
**Impact:** The brief's stated extraction mechanism was replaced.
**Resolution:** P25 uses direct positional extraction from identified Shard Intent names. Six of 16 fragment descriptions carry a bracketed extraction position. The solver identifies each fragment's Shard, then reads the indicated letter from the Intent name: PRESERVATION[4]=S, AUTONOMY[3]=T, VALOR[2]=A, DEVOTION[5]=T, RUIN[2]=U, VIRTUOSITY[7]=S = STATUS.

### BUG-S6-015: P26 brief mechanism mismatch — "extraction" underspecified
**Severity:** Low
**Discovered:** Stage 6 (authoring P26, Team Epsilon)
**Description:** The PUZZLES.md brief for P26 specifies "Identification + extraction" without specifying what is extracted from or how. The answer MORTAL requires letters not available from Vessel names alone (no Vessel name contains the letter T at a useful position). Extraction from Intent names was needed.
**Resolution:** P26 extracts from Intent names (not Vessel names): HARMONY[4]=M, HONOR[2]=O, PRESERVATION[2]=R, DEVOTION[5]=T, AMBITION[1]=A, CULTIVATION[3]=L = MORTAL. Six of 11 Vessel descriptions carry extraction markers.

### BUG-S6-016: P28 — no letter 'G' in any of the 16 Shard Intent names
**Severity:** Medium
**Discovered:** Stage 6 (authoring P28, Team Epsilon)
**Description:** The answer GIFTED requires the letter G. None of the 16 Shard Intent names (Honor, Cultivation, Odium, Preservation, Ruin, Devotion, Dominion, Endowment, Autonomy, Ambition, Mercy, Valor, Whimsy, Invention, Virtuosity, Harmony) contain the letter G at any position. Extraction from Intent names cannot produce G.
**Impact:** The brief's implied extraction (from Shard-level data) is infeasible for the first letter.
**Resolution:** P28 extracts from magic system names instead: AWAKENING[9]=G, SURGEBINDING[7]=I, FERUCHEMY[1]=F, SANDMASTERY[8]=T, FORGERY[5]=E, VOIDBINDING[4]=D = GIFTED. This aligns with the puzzle's theme (Investiture types) better than Shard names would.

### BUG-S6-017: P29 — extraction from Intent names infeasible for ASPECT
**Severity:** Medium
**Discovered:** Stage 6 (authoring P29, Team Epsilon)
**Description:** The answer ASPECT requires both P and S at specific positions. Only PRESERVATION contains P among the 16 Intents, and it cannot serve as two different extraction sources. Direct letter extraction from grid-placed Intent names cannot produce ASPECT without duplication conflicts.
**Impact:** The brief's implied extraction (from placed Shard names) was replaced.
**Resolution:** P29 encodes the answer in the axis labels instead. Column initials A-S-P-E (Affirming, Strike, Principled, Expressive) plus Row A initial C (Conquered) and Row B initial T (Thriving) spell ASPECT. The solver must deduce the axis category names from the grid structure.

### BUG-S6-018: P30 — circular arrangement uniqueness required anchoring and disambiguation clues
**Severity:** Low
**Discovered:** Stage 6 (authoring P30, Team Epsilon)
**Description:** A 16-seat circular arrangement with 10 placed Vessels and 14 constraint clues initially had 2 valid solutions (Koravellium Avast and Bavadin could swap seats 10 and 11). Additionally, without an anchoring clue, all solutions had 16 rotational equivalents.
**Resolution:** Added clue 12 ("Aona holds seat 1") as anchor and clue 13 ("Koravellium Avast holds an even-numbered seat") to force unique solution. Final arrangement verified: 1-Aona, 2-Skai, 4-Rayse, 6-Ati, 7-Leras, 9-Edgli, 10-Koravellium Avast, 11-Bavadin, 13-Ulas Dal, 15-Tanavast. Seats 3,5,8,12,14,16 are unknown Vessels.

---

## Stage 7 Bugs (Editorial Review)

### BUG-S7-001: P07 intro sentence is 26 words
**Severity:** Low
**Discovered:** Stage 7 (editorial voice audit)
**Description:** P07's intro contains "The trick is getting them in the right order -- the same trick Rashek himself could never quite manage, what with all that rewriting of history." at 26 words. Voice rule 3 says "Rarely more than 15 words."
**Impact:** Single sentence. The humor lands, but it exceeds the guideline substantially.
**Mitigation:** Split into two sentences: "The trick is getting them in the right order. Rashek himself never managed it, what with all that rewriting of history."

### BUG-S7-002: 14 past-tense violations across 10 puzzles
**Severity:** Low
**Discovered:** Stage 7 (editorial voice audit)
**Description:** SCOPE.md rule 2 says "Present tense default. Past tense only for Shattering-era events." Fourteen instances of past tense in non-Shattering-era contexts were found in puzzle intro/flavor text: P08 ("had"), P09 ("built," "were"), P10 ("died," "did not take"), P13 ("managed"), P16 ("hung," "glowed," "did"), P19 ("were," "sang," "wore"), P20 ("carried"), P25 ("were broken"), P35 ("did," "was taken").
**Impact:** All violations are in introductory flavor text, not puzzle instructions. They read naturally in Hoid's storytelling voice. The voice rule may be overly strict for historical narration.
**Mitigation:** Either (a) rewrite all intros to present tense (straightforward but may lose flavor), or (b) amend SCOPE.md rule 2: "Past tense permitted in brief historical setup (1-2 sentences) when Hoid is explicitly recounting events." Recommend option (b).

### BUG-S7-003: 17 adjacent same-difficulty violations across all 6 modules
**Severity:** Medium
**Discovered:** Stage 7 (editorial difficulty curve check)
**Description:** The difficulty curve rule ("No two adjacent puzzles in the same module at the same difficulty") is violated 17 times. Every module has at least 2 adjacent pairs at the same difficulty. This is structurally caused by 6 puzzles per module with only 3 difficulty tiers (2,3,4 stars in Rounds 1-2; 2/3,3,4,5 in Round 3).
**Impact:** Mathematically unavoidable without half-star ratings. The macro ramp (start low, end high) is correct in every module.
**Mitigation:** Accept as structural limitation, or introduce half-star ratings. Priority targets for reclassification: P15 (3->4), P22 (3->4), P27 (3->4), P32 (3->4).

### BUG-S7-004: Inconsistent solution verification sections
**Severity:** Low
**Discovered:** Stage 7 (editorial consistency check)
**Description:** P31-P36 (Module F) contain HTML-comment solution verification sections ("AUTHOR ONLY -- not for solvers"). P01-P30 lack these sections. Inconsistency across modules.
**Impact:** No solver impact (HTML comments are invisible). Creates inconsistency for author teams.
**Mitigation:** Either add verification sections to all 36 puzzles at Stage 11 (POLISH), or strip them from P31-P36 at Stage 9 (DELIVERY). Recommend adding to all for author reference, then stripping at delivery.

### BUG-S7-005: P18 difficulty potentially overstated
**Severity:** Low
**Discovered:** Stage 7 (editorial difficulty assessment, confirming BUG-S6-011)
**Description:** P18 (The Surge Wheel, rated 4-star) is a mechanical chain-lookup with no branching or backtracking. Given Windrunners as anchor, each position follows uniquely. The puzzle functions as a knowledge test, not constraint satisfaction. A 3-star rating may be more accurate.
**Impact:** Downgrading to 3-star does not improve the difficulty curve (creates P15/P16/P18 triplet at 3 stars). The overrating is minor.
**Mitigation:** Leave at 4 stars unless the author team wants to differentiate it from P17. Note: the extraction step (7 marked positions spelling PATTERN) adds modest complexity beyond the placement.

---

## Stage 8 Bugs (Integration Check)

### BUG-INT-001: P10 word bank contains "UNITY" (super-meta answer)
**Severity:** BLOCKER
**Discovered:** Stage 8 (integration cross-puzzle spoiler check)
**Description:** P10 (The Survivor's Legacy) includes a word bank of 8 connection words. One of them is UNITY -- the super-meta answer. A solver who encounters UNITY in P10 (Round 1, early in the hunt) and later reaches the super-meta may recall it, drastically reducing the final extraction's difficulty.
**Impact:** Direct exposure of the super-meta answer in a feeder puzzle. Compromises the hunt's final aha moment.
**Fix:** Replace UNITY in P10's word bank with UNIFICATION (or UNION). The connection between C7 (Kelsier's rebellion -> conditions for Sazed) and E-e (Harmony) can be named UNIFICATION. The U-initial is preserved for the acrostic extraction. Update the extraction verification accordingly.

### BUG-INT-002: SUPER-META.md has "RAIB" instead of "RAIBL" (truncated)
**Severity:** MAJOR
**Discovered:** Stage 8 (integration meta verification)
**Description:** SUPER-META.md lines 28 and 59 list the Round II meta answer as "RAIB" (4 characters). The correct ROT13 encoding of ENVOY is "RAIBL" (5 characters). The verification block then references RAIB[5]=L, which is invalid for a 4-character string. This was partially flagged at Stage 7 (the META-II-OATHS.md header was corrected from RAIB to RAIBL), but the SUPER-META.md references were not updated.
**Impact:** Documentation inconsistency. If SUPER-META.md is used during Stage 9 delivery build, the truncated answer could cause confusion or errors.
**Fix:** Replace "RAIB" with "RAIBL" on lines 28 and 59 of `meta/SUPER-META.md`.

### BUG-INT-003: 4 difficulty triplets across Modules C, D, E, F
**Severity:** MAJOR
**Discovered:** Stage 8 (integration difficulty curve analysis)
**Description:** Four modules contain 3 consecutive puzzles at the same difficulty rating:
- Module C: P14/P15/P16 all at 3 stars
- Module D: P21/P22/P23 all at 3 stars
- Module E: P26/P27/P28 all at 3 stars
- Module F: P33/P34/P35 all at 4 stars
These triplets represent genuine progression stalls within modules.
**Impact:** Teams working linearly through a module face 3 consecutive puzzles with no difficulty variation, reducing the sense of progression.
**Fix:** Reclassify 4 puzzles (one per triplet) to break each run:
- P15: 3-star -> 4-star (underdocumented spren identification is harder than stated)
- P22: 3-star -> 4-star (cryptic crossword requires dual competency)
- P27: 3-star -> 4-star (deep lore chronology of Splintering events)
- P32: 3-star -> 4-star (cross-world Cognitive Shadow matching)
These changes break all 4 triplets without altering puzzle content.

---

## Stage 10 Bugs (Platform Test)

### BUG-PT-001: P30 solve time dominates Round 3 (90 min)
**Severity:** MAJOR
**Discovered:** Stage 10 (live solve simulation)
**Description:** P30 (The Shattering, 5-star) consumes one solver for 90 minutes -- the longest single feeder solve in the hunt by a wide margin. The 16-seat circular arrangement with 14 constraint clues is grueling. While thematically justified for a 5-star showpiece, it creates a bottleneck: one solver (the Methodical) is locked on P30 for 1.5 hours while the team works around them.
**Impact:** P30 absorbs disproportionate team resources in Round 3. Teams without a strong logic solver may stall entirely.
**Mitigation:** Add an optional hint at Stage 11 that provides the anchoring step (Clue 12: Aona at seat 1) explicitly, reducing cold-start friction without trivializing the puzzle.

### BUG-PT-002: Round 2 pacing runs 15-20% heavy
**Severity:** MAJOR
**Discovered:** Stage 10 (live solve simulation timing)
**Description:** Round 2 (Roshar) takes 4-5 hours of a team's ~11.5 hour total, representing ~38% of solve time. Rounds 1 and 3 take ~30% and ~32% respectively. The Knights Radiant system (10 Orders x Surges x Heralds x Gemstones) has more moving parts than the Allomancy table, creating longer lookup times for identification puzzles.
**Impact:** Slight pacing imbalance. Teams may feel Round 2 drags relative to the other rounds.
**Mitigation:** Add a 1-page "Roshar Quick Reference" card to the print companion -- a condensed cheat sheet mapping each Order to its 2 Surges, Herald, and Gemstone on a single page. This reduces lookup time without altering puzzle difficulty.

### BUG-PT-003: Newbie engagement in Round 2 is LOW
**Severity:** MAJOR
**Discovered:** Stage 10 (live solve simulation -- Newbie profile)
**Description:** A solver who has read only Mistborn (not Stormlight Archive) can meaningfully contribute to only 3 of 12 Round 2 puzzles: P19 (identification with reference sheet), P20 (anagram mechanics are domain-neutral), and P24 (pure math). For 9 of 12 puzzles, the Newbie is either idle or serving as a transcription assistant.
**Impact:** A team member with partial Cosmere knowledge may disengage during Round 2 (the longest round).
**Mitigation:** This is inherent to the hunt's domain specificity. The print companion's reference sheets partially address it. No structural change recommended -- the Newbie profile is an edge case, and the hunt's target audience is "Cosmere readers who have read at least Mistborn Era 1 and The Stormlight Archive" per SCOPE.md.

### BUG-PT-004: P33 "NAHELBOND" vs "NAHEL BOND" term ambiguity
**Severity:** MINOR
**Discovered:** Stage 10 (live solve simulation)
**Description:** P33's worksheet shows 9 blanks for the VII term, implying a single word. The canonical Cosmere term is usually written "Nahel bond" (two words). A solver who writes "NAHEL BOND" (with a space) will miscount letter positions.
**Impact:** Potential extraction error for an otherwise correctly identified term.
**Fix:** Add a note to P33's extraction instructions: "All terms are written as a single word for extraction purposes. Spaces are removed."

### BUG-PT-005: P36 Event X (Taravangian) direct/indirect classification debatable
**Severity:** MINOR
**Discovered:** Stage 10 (live solve simulation -- team debate)
**Description:** Event X describes Taravangian-as-Odium altering Hoid's memories. The correct classification is INDIRECT (Hoid is passive/acted upon). The simulated team debated this for 15 minutes. The clue language uses passive voice for Hoid ("memories are altered in a conversation"), distinguishing it from direct events where Hoid is the agent, but reasonable solvers may interpret "Hoid was present" as sufficient for a direct classification.
**Impact:** Incorrect classification of Event X would add a noise letter (Y) and remove a signal letter, breaking the extraction. However, the team resolved the debate correctly using close reading.
**Mitigation:** The debate is a feature of the puzzle -- it rewards careful parsing of the direct/indirect distinction. Consider adding a Stage 11 hint: "Direct means the wanderer chose to act. Passive presence is not direct."

### BUG-PT-006: SUPER-META.md "RAIB" truncation still unfixed
**Severity:** MINOR (documentation only)
**Discovered:** Stage 10 (reconfirmed from BUG-INT-002)
**Description:** SUPER-META.md lines 28 and 59 still show "RAIB" instead of "RAIBL". The editorial review (Stage 7) fixed this in META-II-OATHS.md but not in SUPER-META.md. The integration check (Stage 8) flagged it as MAJOR. It has not been fixed.
**Impact:** Documentation inconsistency only -- no solver sees this file. But if used during delivery build, could cause confusion.
**Fix:** Replace "RAIB" with "RAIBL" on lines 28 and 59 of `meta/SUPER-META.md`.

### BUG-PT-007: cosmere-answers.md verification chain has old typo (ORELO vs ORELY)
**Severity:** MINOR (documentation only)
**Discovered:** Stage 10 (formal verification)
**Description:** `meta/cosmere-answers.md` line 116 shows `Bondsmiths(10, 4 Ideals): ORELO[4] = L`. The correct ROT13 for BERYL is ORELY, not ORELO. The answer is correct (both ORELO[4] and ORELY[4] = L), but the verification text has the old transposition.
**Fix:** Update line 116 to `Bondsmiths(10, 4 Ideals): ORELY[4] = L`.

### BUG-PT-008: P18 difficulty confirmed overstated (4-star should be 3-star)
**Severity:** MINOR
**Discovered:** Stage 10 (live solve simulation, confirming BUG-S6-011 and BUG-S7-005)
**Description:** The Specialist chain-solves P18 in 10 minutes with no branching or backtracking. The 4-star rating is too high for what is effectively a lookup chain. The simulation confirms the editorial review's assessment.
**Fix:** Downgrade P18 from 4-star to 3-star at Stage 11. Accept the resulting triplet (P15/P16/P18 at 3 stars) as a structural limitation.

---

## Stage 11 Bugs (Polish)

### BUG-S11-001: No HINTS.md existed
**Severity:** Medium
**Discovered:** Stage 11 (polish pass)
**Description:** The hunt had no hint system. HINTS.md was missing entirely.
**Fix:** Created `HINTS.md` with 3-tier hints for all 36 puzzles + 4 metas. P30 Tier 2 hint specifically addresses the anchor step to reduce cold-start friction.
**Status:** FIXED.

### BUG-S11-002: P19 and P23 Round 2 aha weakness
**Severity:** MAJOR (PT-002 pacing)
**Discovered:** Stage 11 (response to PT-002 Round 2 pacing)
**Description:** P19 (Singer Forms) and P23 (Fabrial Workshop) had the weakest thematic "aha" moments in Round 2, contributing to slower engagement.
**Fix:** Added one sentence of Rosharan flavor to each:
- P19: rhythm/form connection sentence linking each form to its sound
- P23: spanreed/ruby example showing the Essence-gemstone-effect logic
**Status:** FIXED.

### BUG-S11-003: Round 2 Newbie engagement not acknowledged
**Severity:** MAJOR (PT-003)
**Discovered:** Stage 11 (response to PT-003)
**Description:** Round 2 requires Stormlight Archive knowledge, and the hunt had no acknowledgment of this for partial-knowledge teams.
**Fix:** Added a "Knowledge Guide" line to the Round 2 page spec in SITE-SPEC.md, in Hoid's voice.
**Status:** FIXED.

### BUG-S11-004: All 14 past-tense violations resolved
**Severity:** Low (BUG-S7-002)
**Discovered:** Stage 7, deferred to Stage 11
**Description:** 14 past-tense violations in 10 puzzle intros. Changed to present tense in all cases.
**Files modified:** P08, P09, P10, P13, P16, P19, P20, P25, P35.
**Status:** FIXED.

### BUG-S11-005: 4 difficulty triplets broken
**Severity:** MAJOR (BUG-INT-003)
**Discovered:** Stage 8, deferred to Stage 11
**Description:** Reclassified P15, P22, P27, P32 from 3-star to 4-star.
**Files modified:** P15, P22, P27, P32 puzzle files + CLAUDE.md registry.
**Status:** FIXED.

### BUG-S11-006: cosmere-answers.md ORELO typo
**Severity:** MINOR (BUG-PT-007)
**Discovered:** Stage 10
**Description:** Verification chain line 116 had ORELO instead of ORELY.
**Fix:** Updated to ORELY.
**Status:** FIXED.

### BUG-S11-007: P33 NAHELBOND term ambiguity
**Severity:** MINOR (BUG-PT-004)
**Description:** Solvers could write "NAHEL BOND" with a space and miscount.
**Fix:** Added note: "All terms are written as a single word for extraction purposes. Spaces are removed."
**Status:** FIXED.

### BUG-S11-008: P36 direct/indirect distinction
**Severity:** MINOR (BUG-PT-005)
**Description:** The direct/indirect classification needed a concrete example.
**Fix:** Added a canonical example after the definition paragraph: storytelling is direct; someone else's victory is indirect.
**Status:** FIXED.

### BUG-S11-009: P07 26-word sentence
**Severity:** Low (BUG-S7-001)
**Description:** P07 intro had a 26-word sentence violating voice rule 3.
**Fix:** Split into two sentences. Now 13 + 12 words.
**Status:** FIXED.

### BUG-S11-010: SUPER-META.md RAIB truncation
**Severity:** MAJOR (BUG-INT-002 / BUG-PT-006)
**Description:** Checked during Stage 11. Both lines 28 and 59 of SUPER-META.md already show RAIBL. The fix was applied at some point between Stage 8 and Stage 10.
**Status:** ALREADY FIXED (verified).
