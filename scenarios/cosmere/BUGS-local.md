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
