# Condition A5 — Full Authoring Profile (Snyder)

## Puzzle

**First Letters**

Each clue below identifies one unit or technology from Age of Empires II.
Identify it. Take the first letter of each answer, in order.

1. The only infantry unit in the game that attacks from range — it throws its weapon.
2. The Imperial Age siege engine that can destroy trees to carve new corridors through forests.
3. The unit with the highest hit points of any in the game: 600 at elite tier, with trample damage, but extremely slow.
4. The anti-archer unit that costs no gold to train, and whose Castle Age form is obtained by researching an upgrade rather than queuing a new unit.
5. The Monastery technology that extends Monks' conversion ability to buildings and siege weapons.

---

## Solution Key

Clue 1: **Throwing Axeman** → first letter: **T**
Verified: "Throwing Axeman | Franks | Infantry (ranged) | Castle | Ranged infantry — throws axes" (units.md, Notable Unique Units table); "Throwing Axeman | Franks | Infantry | Castle | Ranged infantry — throws axes past walls" (civs.md, Unique Unit Quick Reference)

Clue 2: **Onager** → first letter: **O**
Verified: "Siege Onagers can cut through trees to create new paths" (maps.md, Black Forest description); "Onager | Imperial | 160W 135G | Grouped units, trees | Cavalry, Bombard Cannons" (units.md, Siege table — the Strong vs. column lists "trees" uniquely among all siege units)

Clue 3: **War Elephant** → first letter: **W**
Verified: "War Elephant | Persians | Cavalry (elephant) | Castle | Highest HP of any unit (600); trample damage; very slow" (civs.md, Unique Unit Quick Reference); "War Elephant | Persians | Cavalry | 200F 75G | 450 HP (elite: 600); trample; extremely slow" (units.md, Notable Unique Units table)

Clue 4: **Elite Skirmisher** → first letter: **E**
Verified: "Elite Skirmisher | Skirm → | Castle | 25F 35W | Archers | Knights, Infantry, Siege" (units.md, Archers table — cost is 25F 35W, zero gold; upgrade is Castle Age; it is listed as an upgrade of the Skirmisher, not a separately trained unit); "Skirmishers (archers) hard-counter other archers" (units.md, Counter Triangle section)

Clue 5: **Redemption** → first letter: **R**
Verified: "Redemption | 475G | Monks can convert buildings and siege weapons" (techs.md, Monastery Technologies table)

**Answer: TOWER ✓**
(T from **T**hrowing Axeman, O from **O**nager, W from **W**ar Elephant, E from **E**lite Skirmisher, R from **R**edemption)

---

## Construction Log

**Step 1 — Identify the target deduction per clue (profile sequence step 1):**
Wrote the answer key before writing a single clue word. For each of the five letters in TOWER, scanned all four world data files for items whose name begins with that letter. Selected the item per letter whose defining characteristic is stated explicitly in the source data and is not shared by any other item. The five selected items: Throwing Axeman (only ranged infantry in the game), Onager (only siege unit listing "trees" as a target), War Elephant (explicitly stated "Highest HP of any unit"), Elite Skirmisher (no-gold anti-archer whose Castle Age form is a researched upgrade), Redemption (the exact tech effect is unique among all Monastery technologies).

**Step 2 — Verify uniqueness (profile sequence step 2):**

- T — Throwing Axeman: Other T-items from the data are Trebuchet (anti-building siege weapon, not infantry), Treadmill Crane (economic tech, not a unit), Two-Handed Swordsman (melee infantry, does not throw), Theocracy (Monastery tech), Todai-ji (a wonder building). None satisfies "infantry unit that attacks from range by throwing its weapon." Unique. ✓
- O — Onager: The only O-item present in all four data files. No competing item. Additionally, its "trees" entry in the Strong vs. column is unique across all siege entries: Battering Ram → buildings; Mangonel → "Grouped archers, infantry"; Scorpion → "Grouped infantry"; Trebuchet → "Buildings, static defenses"; Bombard Cannon → "Buildings, Siege, Trebuchets." Only Onager lists trees. ✓
- W — War Elephant: The other W-item is Woad Raider (Celts unique infantry, speed 1.03). The clue's constraints — highest HP record of 600, trample damage, extremely slow — match only War Elephant. Woad Raider's defining trait is speed, not HP. No conflict. ✓
- E — Elite Skirmisher: The only E-item in all four data files. Additionally, the three-part constraint (anti-archer, zero gold cost, Castle Age researched upgrade) is exclusive: the base Skirmisher is trained directly from Feudal Age; the Elite is the Castle Age research upgrade of the Skirmisher line. No other unit satisfies all three simultaneously. ✓
- R — Redemption: The other R-item is Ring Archer Armor (Imperial Blacksmith tech affecting archer defense). The clue references Monks converting buildings and siege weapons — the exact stated effect of Redemption. Ring Archer Armor has no connection to Monks or conversion. No conflict. ✓

**Step 3 — Design the entry point (profile sequence step 3):**
Clue 1 (Throwing Axeman) is placed first because it is the most immediately constrained for an AoE2 player. "Ranged infantry that throws its weapon" has no synonym in the game — there is no other unit class or unit instance that satisfies the description. An AoE2 player familiar with the Franks unique unit arrives at the answer before finishing the clue sentence. This is the designed entry point.

**Step 4 — Trace the solve path (profile sequence step 4):**
All five clues are fully self-contained. No clue depends on the answer to another clue. Each can be solved independently, in any order, without backtracking. The puzzle has a single forward solve path. ✓

**Step 5 — Strip decorative elements (profile sequence step 5):**
Each phrase in each clue was tested: does removing it make the clue ambiguous? "It throws its weapon" in clue 1 — removing it leaves "the only infantry unit in the game that attacks from range," which is already sufficient; it was retained because it provides the mechanism (throwing) as a confirming detail that eliminates any edge-case interpretation of "ranged infantry." "Extremely slow" in clue 3 — removing it leaves the HP and trample constraints, which already uniquely identify War Elephant; it was retained because it eliminates Cataphract (cavalry with trample and high but non-record HP) as a possible distractor. "Carve new corridors through forests" in clue 2 restates the tree-destruction mechanic in spatial terms — it is the constraint, not a wrapper. Nothing decorative survives.

**Step 6 — Verify the extraction (profile sequence step 6):**
T: **T**hrowing Axeman — first character T ✓
O: **O**nager — first character O ✓
W: **W**ar Elephant — first character W ✓
E: **E**lite Skirmisher — first character E ✓
R: **R**edemption — first character R ✓
Spelled in order: T-O-W-E-R = TOWER ✓

---

## Instinct Avoidance Check

**1. Decorative elements** — Every phrase in every clue directly constrains the answer. Tested by asking: if this phrase were removed, would the clue still uniquely identify the same item? Phrases retained where removal would create ambiguity or leave a distractor uneliminated; phrases removed where they added nothing to constraint. Final count of non-constraining phrases: zero. ✓

**2. Multiple valid extraction paths** — The puzzle has exactly one mechanism: first letter of each identified item. The header states this explicitly. No alternate reading (last letter, letter count, positional encoding) is available or implied. ✓

**3. Noise-based difficulty** — All five clue facts are iconic, well-known game properties visible to a casual-to-intermediate AoE2 player: Throwing Axeman's range, Onager's tree-cutting, War Elephant's HP dominance, the Skirmisher's anti-archer role and cost structure, Redemption's Monk expansion. Difficulty arises from the extraction mechanism (first letters spell a word), not from obscure or expert-tier trivia. ✓

**4. Incomplete verification** — Every extraction letter was traced character by character. Every clue was cross-checked against an exact quoted passage from the world data files, documented inline in the Solution Key above. Verification is complete. ✓

**5. Ambiguous clue phrasing** — Each clue was tested against all other items in the data files, both those beginning with the same letter and those sharing the described characteristic. Results: clue 1 — no other infantry unit is ranged; clue 2 — no other siege unit targets trees; clue 3 — no other unit holds the highest HP record; clue 4 — no other unit is simultaneously no-gold, anti-archer, and a Castle Age researched upgrade; clue 5 — no other tech grants Monks conversion of buildings and siege. Zero ambiguous phrasings. ✓

**6. Thematic decoration separate from mechanism** — The title "First Letters" names the extraction mechanism directly. The header instructions ("Identify it. Take the first letter of each answer, in order.") tell the solver exactly how the puzzle is solved. There is no AoE2 narrative wrapper, no fictional narrator, no thematic frame that is disconnected from the mechanism. The puzzle is its mechanism. ✓
