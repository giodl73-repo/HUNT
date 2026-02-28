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
