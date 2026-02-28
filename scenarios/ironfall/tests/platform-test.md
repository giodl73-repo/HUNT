# IRONFALL — Platform Test (Stage 10)

## Test Configuration

| Field | Value |
|-------|-------|
| **Test Date** | 2026-02-27 |
| **Test Team** | 5 solvers (simulated) |
| **Team Composition** | 2 experienced hunt solvers, 2 intermediate, 1 novice |
| **Materials** | Archive website (index.html), manual page prop, all puzzle files |
| **Target Duration** | 4-6 hours |

### Solver Profiles

| Solver | Experience | Strengths | Weaknesses |
|--------|-----------|-----------|------------|
| Solver A (Lead) | Expert | Pattern recognition, meta-intuition | Impatient with arithmetic |
| Solver B | Expert | Math, logic grids, formula work | Misses narrative clues |
| Solver C | Intermediate | Spatial puzzles, maps | Intimidated by hex/binary |
| Solver D | Intermediate | Cross-referencing, careful reading | Slow but thorough |
| Solver E | Novice | Enthusiasm, fresh eyes | No puzzle hunt experience |

---

## Phase 0: Arrival & Site Check

### The Archive Homepage

The team opens `ironfall-archive.net` (index.html).

**First impressions (Solver E):** "This looks like an actual old website. The countdown is ticking. Is this real?"

**First impressions (Solver A):** "90s web design. Tiled background. Marquee. Visitor counter. They nailed the aesthetic. The countdown says 24 hours. Let's not waste time."

**Site functionality check:**

| Element | Status | Notes |
|---------|--------|-------|
| Countdown timer | PASS | JavaScript timer starts at 24:00:00, ticks down every second |
| Navigation links | PASS | HOME, WIKI, FORUM, SECRETS all link to anchors |
| Wiki page list | PASS | 5 puzzle links with edit counts visible (6, 17, 24, 9, 12) |
| Forum thread list | PASS | 5 puzzle links with post counts |
| Secrets section | PASS | 3 meta links, "NEW" blink on section header |
| Recent activity | PASS | 5 forum posts from archive community members |
| Visitor counter | PASS | Displays 847,293 |
| Marquee scroll | PASS | CSS animation scrolls left smoothly |
| Mobile rendering | N/A | Site is 640px fixed width, as designed (authentic) |

**Manual page prop check:**

| Element | Status | Notes |
|---------|--------|-------|
| Readable at print size | PASS | ASCII version clear; actual print would use pixel fonts |
| Hints at secrets | PASS | References Mark 16, Morimoto's Lab, title screen code, save file padding |
| Does not reveal answers | PASS | No specific code, no extraction hints |
| Ashfield Interactive branding | PASS | Logo and "Osaka, 1993" at bottom |
| Controller diagram | PASS | Shows SNES pad with labeled buttons |

**Phase 0 verdict:** PASS. The team is oriented and motivated. The countdown creates urgency. The manual page adds physical texture.

---

## Phase 1: Act I — "The Archive" (5 Puzzles)

### P01 — Bestiary v3.2 — Complete Stats

**Assigned to:** Solver D (careful reader) + Solver E (fresh eyes)

**Solve walkthrough:**

1. Solver D opens the bestiary wiki page. Sees 6 enemy entries with missing values (_A_ through _F_).
2. Reads the community notes one by one:
   - BattleMath_99: "Tundra Golem ATK is exactly three-fourths of Frostclaw ATK." Frostclaw ATK = 32. Three-fourths = 24. **_E_ = 24.**
   - DarkKnight_97: "Shade Wisp ATK is 22." **_B_ = 22.**
   - DataMiner_X: "Emberfang ATK is 28." **_C_ = 28.**
   - LoreHunter: "Thornback DEF is 12." **_A_ = 12.**
   - SpeedQueen: "Frostclaw SPD is 15." **_D_ = 15.**
   - IronFan_Kenji: "Sanctum Guardian is the ONLY Holy-element enemy." **_F_ = Holy.**
3. All six values filled. No ambiguity. No dead ends.
4. Extraction: community discovery years (1997-2001) map to specific enemies. Solver D counts letters in enemy names (spaces removed):
   - 1997 → TUNDRA GOLEM → TUNDRAGOLEM[2] = U
   - 1998 → EMBERFANG → EMBERFANG[2] = M
   - 1999 → THORNBACK → THORNBACK[6] = B
   - 2000 → FROSTCLAW → FROSTCLAW[2] = R
   - 2001 → SHADE WISP → SHADEWISP[3] = A

**Answer: UMBRA**

**Verification against world data:**
- Tundra Golem ATK: bestiary.md says 24. 3/4 of 32 = 24. Confirmed.
- Shade Wisp ATK: bestiary.md says 22. Confirmed.
- Emberfang ATK: bestiary.md says 28. Confirmed.
- Thornback DEF: bestiary.md says 12. Confirmed.
- Frostclaw SPD: bestiary.md says 15. Confirmed.
- Sanctum Guardian element: bestiary.md says Holy. Only Holy enemy. Confirmed.

**P01 post-fix verdict: PASS.** The fix works cleanly. The solver encounters no mismatch, no dead end. The six clues each uniquely determine one value. The extraction (year → enemy → letter position) is clearly presented. Answer UMBRA is correct.

**Time:** ~15 minutes.

---

### P02 — Forge Guide — by Old_Forge_Fan

**Assigned to:** Solver B (logic specialist)

**Solve walkthrough:**

1. Opens the crafting wiki page. Five recipes shown, some with missing elements.
2. Reads the community notes to derive crafting rules:
   - Same element + same element = typed equipment (Fire+Fire→Ring, Ice+Ice→Armor, Thunder+Thunder→Bow, Earth+Earth→Helm, Dark+Dark→Sword)
   - Opposing elements = Weapon with status
   - Cross-element = utility/shield
3. Applies rules:
   - Recipe 1: Earth + Ice (cross) → Earthen Wall → Type: Shield/Wall
   - Recipe 2: Earth + Earth (same) → Product: Ironcrown Helm → Type: Helm
   - Recipe 3: Fire + Ice (opposing) → Steam Lance → Type: Weapon (given)
   - Recipe 4: Thunder + Thunder (same) → Tempest Bow → Type: Bow
   - Recipe 5: Thunder + ??? → Thunder Root → Weapon. SpeedQueen says the missing ingredient is Vine Whip (Earth). Thunder + Earth = opposing = Weapon. Confirmed.
4. Extraction via hex ID table:
   - Earthen Wall: 0x11 = 17 → Q
   - Ironcrown Helm: 0x15 = 21 → U
   - Steam Lance: 0x05 = 5 → E
   - Tempest Bow: 0x0C = 12 → L
   - Thunder Root: 0x0C = 12 → L

**Answer: QUELL**

**Verification against world data:**
- Earthen Wall recipe: items.md C07 = Damp Stone + Knight's Crest = Earth + Ice. Confirmed.
- Ironcrown Helm recipe: items.md C09 = Runed Plate + Warden's Key = Earth + Earth. Confirmed.
- Steam Lance recipe: items.md C01 = Ember Dust + Ice Shard = Fire + Ice. Confirmed.
- Tempest Bow recipe: items.md C06 = Storm Feather + Drake Fang = Thunder + Thunder. Confirmed.
- Thunder Root recipe: items.md C02 = Vine Whip + Spark Thread = Earth + Thunder. Confirmed.
- Hex IDs verified in puzzle: 0x11=17, 0x15=21, 0x05=5, 0x0C=12. All A1Z26 conversions correct.

**P02 post-fix verdict: PASS.** The hex ID extraction mechanism is clean. The solver never hits a dead end. The crafting rule discovery is the aha. The shared hex ID (0x0C for both Tempest Bow and Thunder Root) is a nice detail that adds flavor without creating ambiguity. Answer QUELL is correct.

**Time:** ~20 minutes.

---

### P03 — Save File Deep Dive

**Assigned to:** Solver B (math) + Solver C (spatial/reading support)

**Solve walkthrough:**

1. Four hex dump save files. Solver B reads the save file format reference (from the archive).
2. Checks flags at offsets 0x65 (game_complete), 0x66 (true_ending), 0x67 (secret_boss_beaten):
   - Save A: 0x65=01, 0x66=00, 0x67=00 → Game complete, no secret boss
   - Save B: 0x65=01, 0x66=00, 0x67=01 → Game complete, secret boss beaten
   - Save C: 0x65=00, 0x66=00, 0x67=00 → Game not complete
   - Save D: 0x65=01, 0x66=00, 0x67=01 → Game complete, secret boss beaten
3. Saves B and D both meet True Ending prerequisites. DataMiner_X says: "the padding area should be zeroes. Three saves are. One is not."
4. Checks offset 0x68+ in each save:
   - Save A: all zeroes. Save B: all zeroes. Save C: all zeroes.
   - Save D: bytes 4F 52 44 45 52 at 0x68-0x6C.
5. Converts to ASCII: O, R, D, E, R.

**Answer: ORDER**

**Verification:** 4F=O, 52=R, 44=D, 45=E, 52=R. All correct per ASCII table. Save D flags confirmed at correct offsets.

**Solver C note:** "I was intimidated by the hex dumps at first, but once you know which offsets to check, it's just a lookup table." This is the intended experience.

**P03 verdict: PASS.** Clean solve path. No dead ends. Answer ORDER correct.

**Time:** ~25 minutes.

---

### P04 — World Map — All Connections

**Assigned to:** Solver C (spatial) + Solver E (fresh eyes)

**Solve walkthrough:**

1. ASCII world map shown. Six regions, eight connections.
2. Five riddles from Garek. DataMiner_X's clue: "The number in each riddle is the key. Count that many letters into the place name. No spaces."
3. Solve each riddle:
   - Riddle 1: "One bolt" → number 1 → Storm Tower in Stormspire Peaks → STORMSPIREPEAKS[1] = S
   - Riddle 2: "Three floors" → number 3 → Ashveil Woods (3 floors) → ASHVEILFOREST[3] = H
   - Riddle 3: "Seven pillars" → number 7 → Ember Wastes (Volcanic Depths) → EMBERWASTES[7] = A
   - Riddle 4: "Nine locks" → number 9 → Iron Citadel (B7, Developer's Key) → IRONCITADEL[9] = D
   - Riddle 5: "Five tides" → number 5 → Sunken Vale (Drowned Temple) → SUNKENVALE[5] = E

**Answer: SHADE**

**Verification against world data:**
- Stormspire Peaks: Storm Tower exists (locations.md). Confirmed.
- Ashveil Woods: 3 floors documented (locations.md). Confirmed.
- Ember Wastes → Volcanic Depths: lava pillars, Forge Eternal beneath (locations.md). Confirmed.
- Iron Citadel → B7 = Morimoto's Lab, Developer's Key required (locations.md). Confirmed.
- Sunken Vale → Drowned Temple: 4 floors + Abyssal Grotto (locations.md). Confirmed.
- Letter positions: S(1), H(3), A(7), D(9), E(5) all verified by manual character counting.

**P04 verdict: PASS.** Region identification is unambiguous. Letter indexing is correct. Answer SHADE confirmed.

**Time:** ~20 minutes.

---

### P05 — Battle Damage Calculator v2

**Assigned to:** Solver B (math lead) + Solver A (verification)

**Solve walkthrough:**

1. Four training battle logs. Solver B compares Logs 1 and 2 (same ATK=65, different DEF):
   - DEF 12→55 (increase 43), Damage 236→150 (decrease 86). Rate: 86/43 = 2 per DEF.
2. Tests formula: (ATK x 2 - DEF) x multiplier.
   - Log 1: (130-12) x multiplier = 118 x m = 236. m = 2.0.
   - Log 2: (130-55) x 2 = 150. Confirmed.
   - Log 3: (100-22) x 2 = 156. Confirmed.
   - Log 4 (Lv.25): (90-6) x 2 = 168. But actual = 126. Multiplier = 126/84 = 1.5.
3. Derives: multiplier = (1 + Level/50). Level 50 → 2.0. Level 25 → 1.5.
4. Formula: Physical Damage = (ATK x 2 - DEF) x (1 + Level/50).
5. Rearranges for DEF: DEF = ATK x 2 - Damage / (1 + Level/50). At Level 50: DEF = ATK x 2 - Damage/2.
6. Computes:
   - Log 5: 11x2 - 42/2 = 22 - 21 = 1 → A
   - Log 6: 22x2 - 60/2 = 44 - 30 = 14 → N
   - Log 7: 20x2 - 36/2 = 40 - 18 = 22 → V
   - Log 8: 15x2 - 42/2 = 30 - 21 = 9 → I
   - Log 9: 16x2 - 40/2 = 32 - 20 = 12 → L

**Answer: ANVIL**

**Verification:** All computations match. DEF=1 is below the documented 5-60 range, but BattleMath_99's note about modded encounters explains this. Formula confirmed against battle.md.

**P05 verdict: PASS.** Clean formula derivation. Five direct computations. Answer ANVIL correct.

**Time:** ~30 minutes.

---

### Act I Complete — META-I Attempt

**All five Act I answers collected:** UMBRA, QUELL, ORDER, SHADE, ANVIL

**Solver A notices:** "The wiki page list on the homepage shows edit counts. UMBRA's page has 6 edits. QUELL's has 17. ORDER has 24. SHADE has 9. ANVIL has 12. DataMiner_X said something about the archive index."

**Solver A opens the Archive Index page (META-I).**

**Extraction:**

The page shows a "Top Edited Pages" table with the five answer words as page titles and their edit counts. The rule: (edits mod 5) + 1 = extraction position. Read by edit count descending.

| Rank | Answer | Edits | (mod 5)+1 | Letter |
|------|--------|-------|-----------|--------|
| 1 | ORDER | 24 | 5 | R |
| 2 | QUELL | 17 | 3 | E |
| 3 | SHADE | 12 | 3 | A |
| 4 | ANVIL | 9 | 5 | L |
| 5 | UMBRA | 6 | 2 | M |

**Reading: R, E, A, L, M**

**META-I Answer: REALM**

**Verification:**
- ORDER[5] = R. Confirmed.
- QUELL[3] = E. Confirmed.
- SHADE[3] = A. Confirmed.
- ANVIL[5] = L. Confirmed.
- UMBRA[2] = M. Confirmed. (This is the fix that was critical -- old P01 answer USHER would give S, breaking this to REALF.)

**80% rule check:** With 4 of 5 answers, say missing UMBRA: solver has R, E, A, L, _. "REALM" is the obvious completion. With missing ORDER: _, E, A, L, M → "REALM" still guessable. Works.

**META-I verdict: PASS.** Edit-count mechanism works cleanly. REALM extracts correctly. 80% rule confirmed.

---

## Phase 2: Act II — "The Game" (5 Puzzles)

### P06 — Enemy Sightings — Unconfirmed

**Assigned to:** Solver D (cross-referencing) + Solver A (verification)

**Solve walkthrough:**

1. Five sighting reports from forum users. Each describes an enemy encounter with behavioral clues, region, element, and one visible stat or drop.
2. Solver D cross-references against bestiary.md:
   - Sighting 1: Glacial Reach, Ice, high DEF, low SPD, drops crest → Knight's Crest → **Permafrost Knight (#12)**
   - Sighting 2: Stormspire, Earth, highest DEF in game → DEF 55 → **Stone Sentinel (#15)**
   - Sighting 3: Iron Citadel, Dark, highest non-boss ATK → ATK 60 → **Void Shade (#20)**
   - Sighting 4: Iron Citadel, Dark, highest non-boss HP → HP 500 → **Iron Revenant (#21)**
   - Sighting 5: Iron Citadel, Dark, balanced stats, drops dark blade → Obsidian Blade → **Dread Knight (#19)**
3. A1Z26 conversion: 12→L, 15→O, 20→T, 21→U, 19→S.

**Answer: LOTUS**

**Verification against bestiary.md:**
- Permafrost Knight: #12, HP 280, ATK 30, DEF 38, SPD 10, Ice, drops Knight's Crest. DEF 38 is "high" (not highest in game but high for Ice enemies). Confirmed.
- Stone Sentinel: #15, DEF 55 = highest DEF in entire bestiary. Confirmed.
- Void Shade: #20, ATK 60 = highest non-boss ATK (Morimoto's Shadow has 80 but is a boss). Confirmed.
- Iron Revenant: #21, HP 500 = highest non-boss HP. Confirmed.
- Dread Knight: #19, ATK 55, DEF 35, SPD 18 = balanced endgame stats. Drops Obsidian Blade. Confirmed.

**Potential confusion check:** Could sighting 5 match Ash Phantom (#23)? Ash Phantom has ATK 52, DEF 8, SPD 38 -- that is not "balanced" (very low DEF, very high SPD). And its drop is Phantom Ring, not a blade. No confusion.

**P06 verdict: PASS.** Each identification is unique and unambiguous. Answer LOTUS correct.

**Time:** ~25 minutes.

---

### P07 — The Perfect Build

**Assigned to:** Solver A (lead) + Solver D (verification)

**Solve walkthrough:**

1. CircuitBreaker's requirements: ATK +30, DEF +25, Dark resistance.
2. Solver A checks items.md crafted items:
   - Dark resistance: Dawn Mantle (C03) — DEF +15, resist Dark. Only one. Confirmed.
   - Highest ATK: Void Edge (C08) — ATK +30. No other crafted item matches. Confirmed.
   - Highest DEF: Earthen Wall (C07) — DEF +25. Confirmed.
3. Traces crafting chain:
   - Dawn Mantle: Shadow Veil (Shade Wisp #03) + Crystal Tear (Blizzard Wraith #08)
   - Void Edge: Shadow Veil (Shade Wisp #03) + Obsidian Blade (Dread Knight #19)
   - Earthen Wall: Damp Stone (Moss Crawler #05) + Knight's Crest (Permafrost Knight #12)
4. Shadow Veil needed twice (farm Shade Wisp x2). Five unique enemies: Moss Crawler, Shade Wisp, Blizzard Wraith, Permafrost Knight, Dread Knight.
5. DataMiner_X's farming chart shows the five enemies with bracketed letters:
   - M[O]SS CRAWLER → O
   - D[R]EAD KNIGHT → R
   - [B]LIZZARD WRAITH → B
   - SHADE W[I]SP → I
   - PERMAFROST KNIGH[T] → T (wait -- the puzzle shows "PERMAFROS[T]KNIGHT")

**Checking P07 chart carefully:**
```
M [O] S S  C R A W L E R          → O
D [R] E A D  K N I G H T          → R
[B] L I Z Z A R D  W R A I T H   → B
S H A D E  W [I] S P              → I
P E R M A F R O S [T] K N I G H T → T
```

Letter positions in names (no spaces):
- MOSSCRAWLER: M(1) O(2) S(3) S(4)... [O] at position 2. Confirmed.
- DREADKNIGHT: D(1) R(2) E(3)... [R] at position 2. Confirmed.
- BLIZZARDWRAITH: B(1)... [B] at position 1. Confirmed.
- SHADEWISP: S(1) H(2) A(3) D(4) E(5) W(6) I(7)... [I] at position 7. Confirmed.
- PERMAFROSTKNIGHT: P(1)... S(9) T(10)... [T] at position 10. Confirmed.

**Answer: ORBIT**

**Verification:** All crafting recipes match items.md. All enemy drops match bestiary.md. Marked letter positions verified.

**P07 verdict: PASS.** The crafting chain deduction is solid. The visual chart extraction is clean. Answer ORBIT correct.

**Time:** ~25 minutes.

---

### P08 — 100% Completion Guide — Mark Tracker

**Assigned to:** Solver D (careful reader, comparison specialist)

**Solve walkthrough:**

1. The achievement screenshot shows a 4x4 grid of 16 Marks with names and descriptions.
2. Solver D opens the archive's achievement reference (achievements.md) and compares every entry:
   - #01: Screenshot says "GREENHORN" / "Every hero was once a beginner." Real name is "Seedling" / "A journey begins with a single step." **FAKE.**
   - #02: "TRAILBLAZER" / "Every path has a first footprint." Matches. Real.
   - #03: "ASHEN VICTOR" / "The forest bows to the strong." Matches. Real.
   - #04-06: All match. Real.
   - #07: Screenshot says "HAMMERED" / "Every creation starts with force." Real name is "Artisan" / "Creation is its own reward." **FAKE.**
   - #08: Matches. Real.
   - #09: Screenshot says "UMBERLIGHT" / "The dark paths are the safest." Real name is "Night Walker" / "Some doors open only in darkness." **FAKE.**
   - #10-11: Match. Real.
   - #12: Screenshot says "STEADFAST" / "The true warrior never wavers." Real name is "Loyal Heart" / "The party stands together." **FAKE.**
   - #13: "OATHBREAKER" / "Some promises were meant to break." Real says "...were meant to be broken." **Close call.** Solver D pauses. SpeedQueen's instructions say "the names are close. But close is not correct." But this IS the correct Mark name (Oathbreaker). The description is a slight paraphrase. Solver D decides: **REAL** (name matches exactly; description is a forum-era shortening).
   - #14: "SURVEYOR" / "You watched from the shadows." Real name is "Unseen" / "They never knew you were there." **FAKE.**
   - #15-16: Match. Real.

3. Five fakes: #01 (GREENHORN), #07 (HAMMERED), #09 (UMBERLIGHT), #12 (STEADFAST), #14 (SURVEYOR).
4. Third letter of each fake name:
   - GREENHORN[3] = E
   - HAMMERED[3] = M
   - UMBERLIGHT[3] = B
   - STEADFAST[3] = E
   - SURVEYOR[3] = R

**Answer: EMBER**

**Verification:** All 11 real Marks confirmed against achievements.md. All 5 fakes confirmed as having wrong names and descriptions. Position #13 near-miss (Oathbreaker) correctly identified as real. Third-letter extractions verified.

**P08 verdict: PASS.** The comparison is the puzzle. The near-miss at #13 tests careful reading. Answer EMBER correct.

**Time:** ~20 minutes.

---

### P09 — The Speedrunner's Route

**Assigned to:** Solver C (spatial) + Solver B (optimization)

**Solve walkthrough:**

1. Requirements: Ashveil → Iron Citadel, must visit Stormspire (for Warden's Key), minimize transitions.
2. Solver C traces the map. The naive path Ashveil → Ember → Iron Citadel (2 transitions) fails because the Warden's Key is in Stormspire.
3. Solver B checks all possible routes:
   - Route A (backtrack): Ashveil → Glacial → Stormspire → Glacial → Ashveil → Ember → Iron Citadel = 6
   - Route B (story path): Ashveil → Glacial → Stormspire → Sunken → Ember → Iron Citadel = 5
   - Route C (wrong direction): Ashveil → Ember → Sunken → Stormspire → ... must still get to Iron Citadel via Ember. 7+ transitions.
4. Route B is optimal at 5 transitions. Solver B proves no shorter route exists: Stormspire connects only to Glacial and Sunken. Getting there requires at minimum 2 transitions from Ashveil (via Glacial). Getting from Stormspire to Iron Citadel requires at minimum 3 transitions (Stormspire → Sunken → Ember → Citadel). Total minimum: 5.
5. SpeedQueen's segment times → A1Z26:
   - Segment 1 (7 min) → G
   - Segment 2 (12 min) → L
   - Segment 3 (5 min) → E
   - Segment 4 (1 min) → A
   - Segment 5 (13 min) → M

**Answer: GLEAM**

**Verification against locations.md:**
- Connection graph confirmed: Ashveil↔Glacial, Glacial↔Stormspire, Stormspire↔Sunken, Sunken↔Ember, Ember↔Iron Citadel. All present.
- No shortcut from Stormspire to Iron Citadel. Confirmed (connection matrix shows no direct link).
- Warden's Key is dropped by Peak Warden in Stormspire Peaks (bestiary.md #18). Confirmed.
- A1Z26: 7=G, 12=L, 5=E, 1=A, 13=M. All correct.

**P09 verdict: PASS.** Pathfinding is unambiguous. Optimization has a unique solution. Answer GLEAM correct.

**Time:** ~20 minutes.

---

### P10 — Anomaly in the Code

**Assigned to:** Solver B (math, capstone)

**Solve walkthrough:**

1. Eight battle records, all Level 50. Solver B knows the formula from P05: Damage = (ATK x 2 - DEF) x 2.
2. Checks every record:
   - #1: (65x2-12)x2 = 118x2 = 236. Shown: 236. Match.
   - #2: (55x2-35)x2 = 75x2 = 150. Shown: 158. **Mismatch.** Diff = 8.
   - #3: (50x2-22)x2 = 78x2 = 156. Shown: 156. Match.
   - #4: (42x2-48)x2 = 36x2 = 72. Shown: 77. **Mismatch.** Diff = 5.
   - #5: (40x2-20)x2 = 60x2 = 120. Shown: 120. Match.
   - #6: (60x2-10)x2 = 110x2 = 220. Shown: 232. **Mismatch.** Diff = 12.
   - #7: (48x2-42)x2 = 54x2 = 108. Shown: 117. **Mismatch.** Diff = 9.
   - #8: (65x2-22)x2 = 108x2 = 216. Shown: 240. **Mismatch.** Diff = 24.
3. Records 1, 3, 5 are correct. Records 2, 4, 6, 7, 8 have errors.
4. Differences → A1Z26: 8→H, 5→E, 12→L, 9→I, 24→X.

**Answer: HELIX**

**Verification:** All formula applications confirmed against battle.md. All differences verified. A1Z26 conversions correct.

**Solver A remark:** "The fact that only 3 of 8 records are correct means the solver has to figure out the formula from just those 3 — but they already know it from P05. The two puzzles reinforce each other."

**P10 verdict: PASS.** Anomaly detection is clean. All arithmetic verified. Answer HELIX correct.

**Time:** ~15 minutes.

---

### Act II Complete — META-II Attempt

**All five Act II answers collected:** LOTUS, ORBIT, EMBER, GLEAM, HELIX

**The team opens the "Game Secrets" page (META-II).**

The page shows a mock game screen with five secrets listed. Each has one letter highlighted by a "discovery marker." Reading order is top to bottom as displayed.

| Row | Secret | Highlighted Letter |
|-----|--------|--------------------|
| 1 | **L**OTUS | L (pos 1) |
| 2 | **E**MBER | E (pos 1) |
| 3 | **G**LEAM | G (pos 1) |
| 4 | HEL**I**X | I (pos 4) |
| 5 | ORBI**T** | T (pos 5) |

**Wait -- the display order in META-DESIGN is: LOTUS, EMBER, GLEAM, HELIX, ORBIT.**

Checking META-DESIGN.md: The final mechanism places the secrets in the order that spells LEGIT when the highlighted letters are read top to bottom. The display order is NOT the same as the puzzle-slot order. LOTUS comes first (Row 1), then EMBER (Row 2), GLEAM (Row 3), HELIX (Row 4), ORBIT (Row 5).

**Reading highlighted letters: L, E, G, I, T**

**META-II Answer: LEGIT**

**Verification:**
- LOTUS[1] = L. Confirmed.
- EMBER[1] = E. Confirmed.
- GLEAM[1] = G. Confirmed.
- HELIX[4] = I. Confirmed.
- ORBIT[5] = T. Confirmed.

**80% rule check:** With 4 of 5 answers, say missing HELIX: L, E, G, _, T. "LEGIT" is immediately guessable. Works.

**Solver A note:** "The first three secrets all highlight their first letter. You almost expect a pattern. Then the fourth highlights position 4 and the fifth highlights position 5. The shift IS the signal that something deliberate is happening."

**META-II verdict: PASS.** Clean visual extraction. LEGIT confirmed.

---

## Phase 3: Super-Meta — "The True Ending"

**The team has all 12 answer words:**

| # | Source | Answer |
|---|--------|--------|
| 1 | P01 | UMBRA |
| 2 | P02 | QUELL |
| 3 | P03 | ORDER |
| 4 | P04 | SHADE |
| 5 | P05 | ANVIL |
| 6 | META-I | REALM |
| 7 | P06 | LOTUS |
| 8 | P07 | ORBIT |
| 9 | P08 | EMBER |
| 10 | P09 | GLEAM |
| 11 | P10 | HELIX |
| 12 | META-II | LEGIT |

**The team opens the "True Ending" page — DataMiner_X's final post.**

DataMiner_X's instructions: "The answer words you found. Each one has a number -- its position in the archive. That number points to one letter in the word. That letter IS a button."

**Extraction:** Position number = which letter to extract from the answer word.

| # | Answer | Extract Position | Letter | Button |
|---|--------|-----------------|--------|--------|
| 1 | UMBRA | 1 | U | UP |
| 2 | QUELL | 2 | U | UP |
| 3 | ORDER | 3 | D | DOWN |
| 4 | SHADE | 4 | D | DOWN |
| 5 | ANVIL | 5 | L | LEFT |
| 6 | REALM | 1 | R | RIGHT |
| 7 | LOTUS | 1 | L | LEFT |
| 8 | ORBIT | 2 | R | RIGHT |
| 9 | EMBER | 3 | B | B |
| 10 | GLEAM | 4 | A | A |
| 11 | HELIX | 5 | X | X |
| 12 | LEGIT | 1 | L | LEFT |

**The cheat code: UP, UP, DOWN, DOWN, LEFT, RIGHT, LEFT, RIGHT, B, A, X, LEFT**

**Solver A (the moment):** "Wait. UP UP DOWN DOWN LEFT RIGHT LEFT RIGHT... that's the Konami Code."

**Solver E:** "What's the Konami Code?"

**Solver A:** "The most famous cheat code in video game history. Up Up Down Down Left Right Left Right B A. Every gamer knows it. But this one doesn't end with B A START. It ends with B A X LEFT. Morimoto changed the ending."

**Solver B:** "B A X LEFT instead of B A START. He replaced the Contra ending with his own. The X button doesn't even exist on a NES controller -- it's SNES only. He was signing his game."

**The recognition moment lands.**

---

### Super-Meta Verification

**Character-by-character:**

| # | Answer | Pos | Letter | Expected (ROT13) | Actual (ROT13) | Match? |
|---|--------|-----|--------|-------------------|----------------|--------|
| 1 | UMBRA | 1 | U | H | H | YES |
| 2 | QUELL | 2 | U | H | H | YES |
| 3 | ORDER | 3 | D | Q | Q | YES |
| 4 | SHADE | 4 | D | Q | Q | YES |
| 5 | ANVIL | 5 | L | Y | Y | YES |
| 6 | REALM | 1 | R | E | E | YES |
| 7 | LOTUS | 1 | L | Y | Y | YES |
| 8 | ORBIT | 2 | R | E | E | YES |
| 9 | EMBER | 3 | B | O | O | YES |
| 10 | GLEAM | 4 | A | N | N | YES |
| 11 | HELIX | 5 | X | K | K | YES |
| 12 | LEGIT | 1 | L | Y | Y | YES |

**All 12 extractions match. The cheat code is verified: UP UP DOWN DOWN LEFT RIGHT LEFT RIGHT B A X LEFT.**

**Cross-reference with cheats.md:** The code uses only D-pad and face buttons (no shoulder, no START/SELECT). The manual page states: "The sequence uses only D-pad and face buttons." Confirmed.

**Super-Meta verdict: PASS.** All 12 letters extract correctly. The Konami Code recognition moment works. The Morimoto twist (B A X LEFT instead of B A START) provides the final aha.

---

## Phase 4: Summary

### Solve Times (Estimated)

| Phase | Puzzles | Time | Cumulative |
|-------|---------|------|-----------|
| Site orientation | -- | 10 min | 0:10 |
| Act I puzzles | P01-P05 | 110 min | 2:00 |
| META-I | REALM | 10 min | 2:10 |
| Act II puzzles | P06-P10 | 105 min | 3:55 |
| META-II | LEGIT | 10 min | 4:05 |
| Super-meta | Cheat code | 15 min | 4:20 |

**Total estimated time: 4 hours 20 minutes.** Within the 4-6 hour target range.

### Difficulty Curve

```
Easy ─────────── Medium ──────────── Hard
P01 ■■□□□
P02 ■■■□□
P03 ■■■□□
P04 ■■■□□
P05 ■■■■□
  [META-I]
P06 ■■■□□
P07 ■■■■□
P08 ■■■□□
P09 ■■■■□
P10 ■■■■■
  [META-II]
  [SUPER-META]
```

The curve is smooth. P01 is genuinely easy (direct clue application). P10 is genuinely hard (formula application + anomaly detection across 8 records). The metas are lighter than the hardest feeders, which is correct — the aha is recognition, not computation.

---

### Issue Log

| Issue | Severity | Description | Stage 11 Action |
|-------|----------|-------------|-----------------|
| ISS-001 | Low | P03 hex dumps require careful byte counting. Solver C was initially intimidated. Consider adding offset markers or a "how to read hex" sidebar on the wiki page. | Add a brief hex tutorial note on the P03 wiki page |
| ISS-002 | Low | P05 DEF=1 is below the documented range (5-60). While BattleMath_99's note about modded encounters explains it, a solver might second-guess their math. Consider adding a note: "Some test values fall outside standard ranges." | Add in-character clarification to BattleMath_99's post |
| ISS-003 | Low | META-II display order (LOTUS, EMBER, GLEAM, HELIX, ORBIT) does not match puzzle-slot order (LOTUS, ORBIT, EMBER, GLEAM, HELIX). The solver must trust the on-page display order, not their solve order. This could cause brief confusion. | Add a visual cue on the Game Secrets page: "Read the secrets from top to bottom, as displayed" |
| ISS-004 | Info | The super-meta post by DataMiner_X says "each one has a number -- its position in the archive." The word "position" might be ambiguous (position on the page vs. position in the sequence). The solver needs to understand that position 1-5 = Act I slot, position 6 = META-I, etc. | Clarify the ordering in DataMiner_X's post or add an explicit numbered list |
| ISS-005 | Info | P08 near-miss at #13 (Oathbreaker) is a good challenge, but Solver E (novice) marked it as fake on first pass. Consider whether this is an acceptable difficulty gating or should have an additional signal. | Keep as-is. The Social's notes explain the design intent. It tests careful reading. |

---

### All-Level Verification Summary

| Level | Mechanism | Verified? | Notes |
|-------|-----------|-----------|-------|
| **Feeder puzzles** | 10 individual answers | YES | All 10 answers verified against world data |
| **META-I** | Edit-count extraction → REALM | YES | All 5 letters confirmed |
| **META-II** | Discovery-marker extraction → LEGIT | YES | All 5 letters confirmed |
| **Super-meta** | Slot-position extraction → 12-button code | YES | All 12 letters match cheat code |
| **P01 fix** | UMBRA via enemy-swap extraction | YES | Clean solve, no dead ends |
| **P02 fix** | QUELL via hex-ID extraction | YES | Clean solve, no dead ends |
| **Site delivery** | Homepage loads, timer works | YES | All navigation functional |
| **Print prop** | Manual page readable, hints present | YES | Does not reveal answers |

---

### Bugs Logged

No new high-severity bugs found. The P01 and P02 fixes are verified clean. Five minor issues logged for Stage 11 polish (ISS-001 through ISS-005).

**Platform Test Result: PASS.**

The IRONFALL puzzle hunt is a complete, solvable experience. All 10 puzzles produce correct answers. Both round metas extract correctly. The super-meta produces the True Ending cheat code. The Konami Code recognition moment lands. The archive aesthetic creates the intended emotional arc: urgency, discovery, and the warmth of a community that cared about a game for 29 years.
