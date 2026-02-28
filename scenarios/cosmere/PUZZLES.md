# THE SIXTEENTH SHARD — Puzzle Briefs (Stage 4)

**Total puzzles:** 36 (6 per module)
**Answer encoding:** ROT13 (see `meta/cosmere-answers.md` for answer registry)
**All answers coordinated with 4 metas before authoring begins (Principle #19)**

---

## ROUND 1: SCADRIAL — "The Metal Arts"

### Module A — Allomancy (P01-P06, Team Alpha)

---

#### P01 — "Burning Through"

| Field | Value |
|-------|-------|
| **Pool ID** | A-02 |
| **Module** | A (Allomancy) |
| **Round** | 1 (Scadrial) |
| **Difficulty** | ★★ |
| **Mechanism** | Identification + ordered extraction |
| **Metal Assignment** | Iron (#1, Physical, External, Pull) |
| **Answer (ROT13)** | HCEVFR |
| **Author Team** | Team Alpha |

**Description:** Sixteen Allomantic power descriptions are presented with their metal names excised. Each description is taken verbatim from the allomancy.md power descriptions (e.g., "Pulls on nearby metals, attracting metal toward the Allomancer" = Iron; "Heightens all five senses dramatically" = Tin). The solver identifies each of the 16 metals from its power description alone.

**World/ Reference:** `world/systems/allomancy.md` — Allomantic Power column of the Master Table (all 16 metals)

**Extraction:** The solver identifies all 16 metals and orders them by table number (1-16). The answer is extracted by reading specific letters from the metal names at positions given by a secondary clue (e.g., "the quadrant number of each metal indexes into its name: Physical=1, Mental=2, Temporal=3, Enhancement=4"). The extracted letters from this indexing spell the answer.

**Interlock Notes:** This is the first puzzle in the hunt and the gentlest on-ramp. It forces the solver to internalize the 16-metal table, which is essential for the Round 1 meta.

**Meta Role:** Iron is a Pulling metal (Physical quadrant). The answer's 1st letter feeds the Meta I extraction at position 1.

---

#### P02 — "Alloy Partners"

| Field | Value |
|-------|-------|
| **Pool ID** | A-04 |
| **Module** | A (Allomancy) |
| **Round** | 1 (Scadrial) |
| **Difficulty** | ★★ |
| **Mechanism** | Pairing + extraction |
| **Metal Assignment** | Steel (#2, Physical, External, Push) |
| **Answer (ROT13)** | SBETRQ |
| **Author Team** | Team Alpha |

**Description:** Eight cryptic clues each describe a pair of metals (base + alloy). The solver identifies both metals in each pair (Iron/Steel, Tin/Pewter, Zinc/Brass, Copper/Bronze, Gold/Electrum, Cadmium/Bendalloy, Aluminum/Duralumin, Chromium/Nicrosil). Each clue references both the base metal's Allomantic power and the alloy's Feruchemical power to make the pairing non-trivial.

**World/ Reference:** `world/systems/allomancy.md` — Alloy Of column, Allomantic Power + Feruchemical Power columns

**Extraction:** For each identified pair, the solver takes the first letter of the base metal name and the last letter of the alloy name. These 16 letters (8 pairs x 2 letters) are filtered and combined to spell the answer.

**Interlock Notes:** None. Independent of other puzzles.

**Meta Role:** Steel is a Pushing metal. Its extracted letter does NOT feed the meta answer directly but is present in the extraction table as a decoy.

---

#### P03 — "Feruchemical Reserves"

| Field | Value |
|-------|-------|
| **Pool ID** | A-07 |
| **Module** | A (Allomancy) |
| **Round** | 1 (Scadrial) |
| **Difficulty** | ★★★ |
| **Mechanism** | Identification + positional extraction |
| **Metal Assignment** | Zinc (#5, Mental, External, Pull) |
| **Answer (ROT13)** | NAPUBE |
| **Author Team** | Team Alpha |

**Description:** Sixteen items are described that can be "stored" in a metalmind — each is a Feruchemical attribute (weight, speed, senses, strength, mental speed, warmth, memories, wakefulness, health, determination, breath, energy, Identity, Connection, Fortune, Investiture). The solver identifies each attribute, maps it to its metal, and applies the quadrant-based extraction (Physical=1, Mental=2, Temporal=3, Enhancement=4) to read a letter from each metal name.

**World/ Reference:** `world/systems/allomancy.md` — Feruchemical Power column

**Extraction:** For each identified attribute-to-metal mapping, take the Nth letter of the metal name where N = the metal's quadrant number. The 16 extracted letters contain the answer embedded within them (the solver uses a secondary filter to isolate the answer letters).

**Interlock Notes:** Shares the quadrant-index extraction concept with the Round 1 meta (intentional — this puzzle teaches the mechanic).

**Meta Role:** Zinc is a Pulling metal (Mental quadrant). The answer's 2nd letter feeds the Meta I extraction at position 2.

---

#### P04 — "The Allomantic Table"

| Field | Value |
|-------|-------|
| **Pool ID** | A-01 |
| **Module** | A (Allomancy) |
| **Round** | 1 (Scadrial) |
| **Difficulty** | ★★★ |
| **Mechanism** | Grid logic / constraint satisfaction |
| **Metal Assignment** | Brass (#6, Mental, External, Push) |
| **Answer (ROT13)** | ZNGEVK |
| **Author Team** | Team Alpha |

**Description:** The solver is presented with an empty 4x4 grid labeled only with the axes (Internal/External rows, Push/Pull columns, and the four quadrant labels: Physical, Mental, Temporal, Enhancement). Sixteen binary property clues are given (e.g., "This External Pulling Physical metal..." or "This Internal Pushing Enhancement metal..."). Each clue uniquely identifies a cell. The solver fills the grid with the 16 metal names.

**World/ Reference:** `world/systems/allomancy.md` — Grid Layout diagram, full Master Table

**Extraction:** Once the grid is complete, the solver reads the diagonal (top-left to bottom-right) or follows a path indicated by a secondary clue, extracting letters from the metal names at specific positions to spell the answer.

**Interlock Notes:** This is the visual centerpiece of Module A. The completed grid serves as a reference tool for the Round 1 meta.

**Meta Role:** Brass is a Pushing metal. Decoy in the meta extraction.

---

#### P05 — "Three Arts, One Spike"

| Field | Value |
|-------|-------|
| **Pool ID** | A-05 |
| **Module** | A (Allomancy) |
| **Round** | 1 (Scadrial) |
| **Difficulty** | ★★★★ |
| **Mechanism** | Triple-matching with error detection |
| **Metal Assignment** | Pewter (#4, Physical, Internal, Push) |
| **Answer (ROT13)** | GEVNQ |
| **Author Team** | Team Alpha |

**Description:** For each of 8 selected metals (restricted to Physical and Mental quadrants where all three Metallic Arts are well-documented), three clues are given — one each for its Allomantic, Feruchemical, and Hemalurgic use. One clue per triple is deliberately misattributed (e.g., a Feruchemical description placed in the Allomancy slot). The solver identifies the metal, assigns each clue to the correct art, and finds the "odd one out" — the misattributed clue.

**World/ Reference:** `world/systems/allomancy.md` — All three columns (Allomantic Power, Feruchemical Power, Hemalurgic Steal) for Physical + Mental quadrant metals

**Extraction:** The misattributed art (A for Allomancy, F for Feruchemy, H for Hemalurgy) in each triple, combined with the metal number, encodes the answer. Specifically: the initial of the wrong art at each step, read in metal order, spells the answer.

**Interlock Notes:** Uses only Physical + Mental quadrant metals to avoid [VERIFY] flags on Temporal and Enhancement Hemalurgic entries.

**Meta Role:** Pewter is a Pushing metal. Decoy in the meta extraction.

---

#### P06 — "The Coppercloud"

| Field | Value |
|-------|-------|
| **Pool ID** | A-09 |
| **Module** | A (Allomancy) |
| **Round** | 1 (Scadrial) |
| **Difficulty** | ★★★★ |
| **Mechanism** | Deductive logic (constraint satisfaction) |
| **Metal Assignment** | Bronze (#8, Mental, Internal, Push) |
| **Answer (ROT13)** | UVQQRA |
| **Author Team** | Team Alpha |

**Description:** Eight Allomancers are in a room. Some are hidden by a Coppercloud (Smoker). A Seeker (Bronze Misting) reports which Allomantic pulses they detect from various positions. From overlapping reports and knowledge of which metals are affected by Copperclouds, the solver deduces who is burning what metal.

**World/ Reference:** `world/systems/allomancy.md` — Copper ("hides Allomantic pulses"), Bronze ("detects Allomantic pulses"), the Smoker/Seeker dynamic

**Extraction:** Once all 8 Allomancers' metals are identified, the answer is extracted from the first letters of their metals (or Misting names) in a specified order.

**Interlock Notes:** The Smoker/Seeker mechanic is unique to this puzzle. No other Module A puzzle uses information-hiding logic.

**Meta Role:** Bronze is a Pushing metal. Decoy in the meta extraction.

---

### Module B — Scadrial History (P07-P12, Team Beta)

---

#### P07 — "The Lord Ruler's Thousand Years"

| Field | Value |
|-------|-------|
| **Pool ID** | B-02 |
| **Module** | B (Scadrial History) |
| **Round** | 1 (Scadrial) |
| **Difficulty** | ★★ |
| **Mechanism** | Chronological ordering + acrostic extraction |
| **Metal Assignment** | Electrum (#10, Temporal, Internal, Push) |
| **Answer (ROT13)** | GLENAG |
| **Author Team** | Team Beta |

**Description:** Ten events from the Final Empire's history are presented out of chronological order. Each event is described in a single sentence. The solver orders them from earliest to latest. Reading the first letter of each event description in the correct order spells a phrase that contains the answer.

**World/ Reference:** `world/systems/scadrial.md` — Major Events Timeline

**Extraction:** Acrostic — first letters of correctly ordered event descriptions. The 10 first letters spell a phrase; the answer is a 6-letter word embedded within or formed by those letters.

**Interlock Notes:** Events span all three Era 1 books. Does not overlap with Module A's focus on metals.

**Meta Role:** Electrum is a Pushing metal. Decoy in the meta extraction.

---

#### P08 — "Kelsier's Crew"

| Field | Value |
|-------|-------|
| **Pool ID** | B-01 |
| **Module** | B (Scadrial History) |
| **Round** | 1 (Scadrial) |
| **Difficulty** | ★★ |
| **Mechanism** | Identification + positional extraction |
| **Metal Assignment** | Gold (#9, Temporal, Internal, Pull) |
| **Answer (ROT13)** | URVFG |
| **Author Team** | Team Beta |

**Description:** Eight coded character descriptions of Kelsier's crew members. Each description references the character's personality, role in the heist, and Allomantic ability (or lack thereof for Dockson). The solver identifies each crew member: Vin, Kelsier, Ham, Breeze, Clubs, Marsh, Dockson, Spook.

**World/ Reference:** `world/systems/scadrial.md` — Era 1 Key Characters table

**Extraction:** For each identified crew member who is an Allomancer, the solver looks up their metal and uses the crew member's position in the crew roster (alphabetical or role-based) to index into the metal name. Non-Allomancer crew members (Dockson) contribute via a different rule (index into their character name instead). The extracted letters spell the answer.

**Interlock Notes:** Kelsier's crew is the emotional on-ramp. This puzzle anchors Module B in the characters solvers love.

**Meta Role:** Gold is a Pulling metal (Temporal quadrant). The answer's 3rd letter feeds the Meta I extraction at position 3.

---

#### P09 — "The Social Ladder"

| Field | Value |
|-------|-------|
| **Pool ID** | B-05 |
| **Module** | B (Scadrial History) |
| **Round** | 1 (Scadrial) |
| **Difficulty** | ★★★ |
| **Mechanism** | Categorization + ordered extraction |
| **Metal Assignment** | Cadmium (#11, Temporal, External, Pull) |
| **Answer (ROT13)** | EVGHNY |
| **Author Team** | Team Beta |

**Description:** Twelve characters from both Eras are described. The solver categorizes each into Scadrial's social structure: Skaa, Nobility, Obligator, Steel Inquisitor, Kandra, Koloss, Terris Keeper. Each category has a specific hierarchical rank (from lowest to highest status in the Final Empire). The category names, ordered by first appearance in the books, encode the answer.

**World/ Reference:** `world/systems/scadrial.md` — Social Structure section, Era 1 + Era 2 Key Characters

**Extraction:** Each character's social category maps to a letter via the category's rank number (Skaa=1, Koloss=2, ..., Lord Ruler=7) or first letter. The characters, ordered by their book appearance, produce a letter sequence that spells the answer.

**Interlock Notes:** Bridges Era 1 and Era 2 — characters from both eras force the solver to understand how social structures evolved.

**Meta Role:** Cadmium is a Pulling metal (Temporal quadrant). The answer's 3rd letter feeds the Meta I extraction at position 4 (the "T" in UNITE).

---

#### P10 — "The Survivor's Legacy"

| Field | Value |
|-------|-------|
| **Pool ID** | B-11 |
| **Module** | B (Scadrial History) |
| **Round** | 1 (Scadrial) |
| **Difficulty** | ★★★ |
| **Mechanism** | Cross-era matching (cause/effect) |
| **Metal Assignment** | Chromium (#15, Enhancement, External, Pull) |
| **Answer (ROT13)** | HAQRNQ |
| **Author Team** | Team Beta |

**Description:** Eight cause-and-effect pairs spanning Era 1 and Era 2. Each "cause" is an action taken by Kelsier (or a direct consequence of his actions) in Era 1. Each "effect" is a condition or event in Era 2 that results from that cause. The solver matches cause to effect across the 300-year gap.

**World/ Reference:** `world/systems/scadrial.md` — Era 1 Key Characters (Kelsier), Era 2 Key Characters, Major Events Timeline

**Extraction:** Each matched pair has a "connection point" — the specific concept that links cause to effect (e.g., "the Survivor religion" links Kelsier's martyrdom to Era 2 religious movements). The first letters of the 8 connection points, ordered by chronological era-1 cause date, spell a phrase containing the answer.

**Interlock Notes:** Requires knowledge of both Eras. The cross-era connections are the puzzle's core insight.

**Meta Role:** Chromium is a Pulling metal (Enhancement quadrant). The answer's 4th letter feeds the Meta I extraction at position 5 (the "E" in UNITE).

---

#### P11 — "The Siege of Luthadel"

| Field | Value |
|-------|-------|
| **Pool ID** | B-03 |
| **Module** | B (Scadrial History) |
| **Round** | 1 (Scadrial) |
| **Difficulty** | ★★★★ |
| **Mechanism** | Logic grid (multi-variable deduction) |
| **Metal Assignment** | Bendalloy (#12, Temporal, External, Push) |
| **Answer (ROT13)** | OERNPU |
| **Author Team** | Team Beta |

**Description:** Three armies converge on Luthadel (Straff Venture's, Cett's, the Koloss). Given descriptions of each army's composition, direction of approach, strategic objective, and the key Allomancers on each side, the solver deduces a multi-variable logic grid: which army arrives from which direction, which attacks first, and what their ultimate objective is.

**World/ Reference:** `world/systems/scadrial.md` — Siege of Luthadel event, Straff Venture, Cett, Koloss army descriptions

**Extraction:** The solved logic grid reveals a sequence. The answer is extracted from the grid's solution pattern — e.g., the first letters of the army commanders in the order they arrive.

**Interlock Notes:** The most complex Module B puzzle. Requires understanding of the military situation from The Well of Ascension.

**Meta Role:** Bendalloy is a Pushing metal. Decoy in the meta extraction.

---

#### P12 — "The Roughs Justice"

| Field | Value |
|-------|-------|
| **Pool ID** | B-06 |
| **Module** | B (Scadrial History) |
| **Round** | 1 (Scadrial) |
| **Difficulty** | ★★★★ |
| **Mechanism** | Deduction (Twinborn ability identification) |
| **Metal Assignment** | Duralumin (#14, Enhancement, Internal, Push) |
| **Answer (ROT13)** | BHGYNJ |
| **Author Team** | Team Beta |

**Description:** Six crimes in the Roughs are described. Each crime was committed using a specific Twinborn ability combination (one Allomantic metal + one Feruchemical metal). The solver identifies which metals were used based on the crime's characteristics (e.g., a thief who can slow time in a bubble while storing health to survive a fight = Bendalloy Allomancy + Gold Feruchemy = Wayne's combination).

**World/ Reference:** `world/systems/scadrial.md` — Era 2 Key Characters (Twinborn), `world/systems/allomancy.md` — Allomantic + Feruchemical powers

**Extraction:** For each crime, the solver identifies two metals (Allomantic + Feruchemical). The Allomantic metal's table number minus the Feruchemical metal's table number (or vice versa) gives a number that encodes a letter (A=1, B=2, etc.). The 6 letters spell the answer.

**Interlock Notes:** This is the only puzzle in the hunt that uses Twinborn combinations, making it Module B's unique contribution to the hunt's mechanical diversity.

**Meta Role:** Duralumin is a Pushing metal. Decoy in the meta extraction.

---

## ROUND 2: ROSHAR — "Words of Radiance"

### Module C — Knights Radiant (P13-P18, Team Gamma)

---

#### P13 — "Herald's Madness"

| Field | Value |
|-------|-------|
| **Pool ID** | C-06 |
| **Module** | C (Knights Radiant) |
| **Round** | 2 (Roshar) |
| **Difficulty** | ★★ |
| **Mechanism** | Identification (behavioral) + mapping |
| **Order Assignment** | Dustbringers (#3) |
| **Answer (ROT13)** | OEBXRA |
| **Author Team** | Team Gamma |

**Description:** Descriptions of 6 Heralds based on their current mental state and behavior: Jezrien (dead, killed by Moash), Nale (leads Skybreakers, obsessed with law), Shalash (destroys art depicting herself), Taln (endured millennia of torture, returned broken), Ishar (claims to be God, dangerous), Kalak (fearful, broken). The solver identifies each Herald, then maps each to their associated Radiant Order.

**World/ Reference:** `world/systems/roshar.md` — The 10 Heralds table (Current Fate column); `world/systems/knights-radiant.md` — Herald-Order associations

**Extraction:** Each identified Herald maps to an Order, which maps to a Gemstone. The gemstone names, ordered by Herald number, yield letters at specific positions to spell the answer.

**Interlock Notes:** Uses only the 6 best-documented Herald statuses to avoid [VERIFY] issues. Introduces the Herald-Order-Gemstone mapping that Module C builds upon.

**Meta Role:** Dustbringers have only 1 documented Ideal. Non-contributing in the Meta II extraction.

---

#### P14 — "Ideals and Oaths"

| Field | Value |
|-------|-------|
| **Pool ID** | C-02 |
| **Module** | C (Knights Radiant) |
| **Round** | 2 (Roshar) |
| **Difficulty** | ★★★ |
| **Mechanism** | Fill-in-the-blank + identification + acrostic |
| **Order Assignment** | Edgedancers (#4) |
| **Answer (ROT13)** | QRIBGR |
| **Author Team** | Team Gamma |

**Description:** Fragments of Radiant Oaths from various orders are presented, each missing key words. The solver identifies which Order spoke each Oath and fills in the missing words. Restricted to confirmed Oaths: the First Ideal (universal), Windrunner 2nd and 3rd, Edgedancer 2nd and 3rd, Bondsmith 2nd and 3rd, and Lightweaver Truths. The missing words' first letters spell the answer.

**World/ Reference:** `world/systems/knights-radiant.md` — The Ideals (Immortal Words) section

**Extraction:** Acrostic from the first letters of the missing words, ordered by Ideal number within each Order.

**Interlock Notes:** The most emotionally resonant puzzle in Module C. The First Ideal ("Life before death. Strength before weakness. Journey before destination.") is central.

**Meta Role:** Edgedancers have 3 documented Ideals. The answer's 3rd letter feeds Meta II extraction (the "V" in ENVOY).

---

#### P15 — "Spren Bonds"

| Field | Value |
|-------|-------|
| **Pool ID** | C-04 |
| **Module** | C (Knights Radiant) |
| **Round** | 2 (Roshar) |
| **Difficulty** | ★★★ |
| **Mechanism** | Identification (narrative) + mapping |
| **Order Assignment** | Truthwatchers (#5) |
| **Answer (ROT13)** | FCVEVG |
| **Author Team** | Team Gamma |

**Description:** Ten character vignettes describe a Radiant and their bonded spren's behavior. Each vignette highlights the spren's distinctive personality: Honorspren are rigid and honorable, Cryptics speak in patterns and lies, Cultivationspren are nurturing, Inkspren are logical, etc. The solver identifies the spren type from behavior, then maps to the Radiant Order.

**World/ Reference:** `world/systems/knights-radiant.md` — Spren Type column, Notable Members

**Extraction:** Each identified spren type's first letter, combined with the Order number as an index, extracts letters from the spren type names to spell the answer.

**Interlock Notes:** Restricted to spren types with documented behavioral characteristics (Honorspren, Cryptics, Cultivationspren, Inkspren, plus generic descriptions for less-documented types).

**Meta Role:** Truthwatchers have only 1 documented Ideal. Non-contributing in the Meta II extraction.

---

#### P16 — "Radiant Roster"

| Field | Value |
|-------|-------|
| **Pool ID** | C-07 |
| **Module** | C (Knights Radiant) |
| **Round** | 2 (Roshar) |
| **Difficulty** | ★★★ |
| **Mechanism** | Character identification + conditional indexing |
| **Order Assignment** | Skybreakers (#2) |
| **Answer (ROT13)** | FJBEA |
| **Author Team** | Team Gamma |

**Description:** Twelve characters from the Stormlight Archive are described without names. The solver identifies each and determines their Radiant Order (if any): Kaladin (Windrunner), Shallan (Lightweaver), Jasnah (Elsecaller), Dalinar (Bondsmith), Navani (Bondsmith), Szeth (Skybreaker), Lift (Edgedancer), Renarin (Truthwatcher), Venli (Willshaper), plus non-Radiants. The Order number indexes into the character name.

**World/ Reference:** `world/systems/roshar.md` — Key Characters; `world/systems/knights-radiant.md` — Notable Members

**Extraction:** For each identified Radiant, use the Order number (1-10) to take the Nth letter of the character's name. Non-Radiants contribute differently (e.g., position 0 or a fixed character). The extracted letters spell the answer.

**Interlock Notes:** Szeth's complex history (Honorblade Windrunner abilities initially, then Skybreaker) is resolved by specifying "current Order as of Wind and Truth."

**Meta Role:** Skybreakers have 5 documented Ideals. The answer's 5th letter feeds Meta II extraction (the "N" in ENVOY).

---

#### P17 — "The Double Eye"

| Field | Value |
|-------|-------|
| **Pool ID** | C-01 |
| **Module** | C (Knights Radiant) |
| **Round** | 2 (Roshar) |
| **Difficulty** | ★★★★ |
| **Mechanism** | Ring topology logic + multi-attribute confirmation |
| **Order Assignment** | Windrunners (#1) |
| **Answer (ROT13)** | GBJRE |
| **Author Team** | Team Gamma |

**Description:** The 10 Orders are arranged in the Double Eye pattern (decagon). The solver receives 10 clues, each pointing to a Surge. Since each Surge is shared by two adjacent Orders, the solver must determine which ORDER each clue targets using secondary confirmation (Herald name, Gemstone color, Spren type). Letters extracted from the confirmed Order names spell the answer.

**World/ Reference:** `world/systems/knights-radiant.md` — Surge Sharing Pattern (Double Eye), Master Table (all columns)

**Extraction:** For each confirmed Order, take the letter at the position corresponding to its Surge pairing (1st shared Surge → letter 1, 2nd → letter 2). The extracted letters, read in ring order, spell the answer.

**Interlock Notes:** This is the visual centerpiece of the Roshar round. The Double Eye diagram should be beautifully rendered.

**Meta Role:** Windrunners have 4 documented Ideals. The answer's 4th letter feeds Meta II extraction (the "E" in ENVOY).

---

#### P18 — "The Surge Wheel"

| Field | Value |
|-------|-------|
| **Pool ID** | C-05 |
| **Module** | C (Knights Radiant) |
| **Round** | 2 (Roshar) |
| **Difficulty** | ★★★★ |
| **Mechanism** | Circular constraint placement (visual logic) |
| **Order Assignment** | Lightweavers (#6, non-critical duplicate) |
| **Answer (ROT13)** | CNGGREA |
| **Author Team** | Team Gamma |

**Description:** A circular diagram shows 10 positions. Numbered arrows between adjacent positions indicate the Surge they share (Adhesion=1 through Tension=10). The solver must place the 10 Orders in the correct positions, with one position fixed ("Windrunners are at the top"). The ring topology constrains placement tightly.

**World/ Reference:** `world/systems/knights-radiant.md` — Surge Sharing Pattern, all 10 Surges with their sharing Orders

**Extraction:** Once placed, the solver reads the 7th letter of each Order name (or a letter at a position derived from the Surge number shared with the clockwise neighbor), traversing the ring clockwise. The 10 extracted letters contain the 7-letter answer.

**Interlock Notes:** This puzzle and P17 (The Double Eye) are complementary — P17 identifies individual Orders from Surge clues, while P18 reconstructs the entire ring from Surge-sharing constraints. Together they fully cover the Double Eye system.

**Meta Role:** Lightweavers are doubled (also P22). This puzzle's answer does NOT contribute to Meta II; the critical extraction comes from P22.

---

### Module D — Roshar World (P19-P24, Team Delta)

---

#### P19 — "Singer Forms"

| Field | Value |
|-------|-------|
| **Pool ID** | D-04 |
| **Module** | D (Roshar World) |
| **Round** | 2 (Roshar) |
| **Difficulty** | ★★ |
| **Mechanism** | Identification + indexing |
| **Order Assignment** | Stonewards (#9) |
| **Answer (ROT13)** | PUVGVA |
| **Author Team** | Team Delta |

**Description:** Eight descriptions of Singer physical appearances and abilities. The solver identifies the form: Warform (carapace armor), Mateform (reproduction), Workform (labor), Nimbleform (agile), Scholarform (intellectual), Stormform (Everstorm summoning), Dullform (docile), Envoyform (diplomatic). The form name and a given index number produce the answer.

**World/ Reference:** `world/systems/roshar.md` — Singer Forms table

**Extraction:** For each identified form, take the Nth letter of the form name (where N is provided as a secondary clue per form). The 8 extracted letters, in presentation order, contain the 6-letter answer.

**Interlock Notes:** The Singer Forms table provides physical descriptions sufficient for identification. Only form descriptions (not spren types, which are [VERIFY]) are used.

**Meta Role:** Stonewards have only 1 documented Ideal. Non-contributing in Meta II.

---

#### P20 — "Bridge Four"

| Field | Value |
|-------|-------|
| **Pool ID** | D-12 |
| **Module** | D (Roshar World) |
| **Round** | 2 (Roshar) |
| **Difficulty** | ★★ |
| **Mechanism** | Anagram + indexing |
| **Order Assignment** | Windrunners (#1, non-critical duplicate) |
| **Answer (ROT13)** | OEVQTR |
| **Author Team** | Team Delta |

**Description:** The members of Bridge Four are listed with scrambled names. The solver unscrambles each name (Kaladin, Rock, Teft, Sigzil, Skar, Drehy, Rlain, Lopen, Moash). Each unscrambled name has an index provided (e.g., "take the 3rd letter"). The indexed letters spell the answer.

**World/ Reference:** `world/systems/roshar.md` — Kaladin Stormblessed's story, Bridge Four references

**Extraction:** Anagram each name, then take the specified letter from each. Read in the order given to spell the answer.

**Interlock Notes:** Uses alphabetical rank among Bridge Four members for indexing (not chapter-of-first-appearance, which is not in the data file).

**Meta Role:** Windrunners are doubled (also P17). This puzzle does NOT contribute to Meta II.

---

#### P21 — "Rhythm of Answers"

| Field | Value |
|-------|-------|
| **Pool ID** | D-01 |
| **Module** | D (Roshar World) |
| **Round** | 2 (Roshar) |
| **Difficulty** | ★★★ |
| **Mechanism** | Identification + name extraction |
| **Order Assignment** | Willshapers (#8) |
| **Answer (ROT13)** | PUNAG |
| **Author Team** | Team Delta |

**Description:** Eight Rhythms of the Singers are described by their emotional quality. Each description captures the mood and tempo of a Rhythm (Peace = calm and steady, Awe = slow and reverent, Anxiety = rapid and uneven). The solver identifies each Rhythm name and extracts letters from the names.

**World/ Reference:** `world/systems/roshar.md` — Rhythms section

**Extraction:** For each identified Rhythm, take a specified letter from the Rhythm name. Read in order to spell the answer. Restricted to confirmed Rhythms: Peace, Awe, Anxiety, plus Rhythms derivable from the "Rhythm of War" title and Odium's Rhythms (Destruction, Command).

**Interlock Notes:** This is the only puzzle in the hunt that uses Singer Rhythms as primary content.

**Meta Role:** Willshapers have only 1 documented Ideal. Non-contributing in Meta II.

---

#### P22 — "The Chasms Below"

| Field | Value |
|-------|-------|
| **Pool ID** | D-07 |
| **Module** | D (Roshar World) |
| **Round** | 2 (Roshar) |
| **Difficulty** | ★★★ |
| **Mechanism** | Cryptic crossword (Cosmere-themed) |
| **Order Assignment** | Lightweavers (#6, meta-critical) |
| **Answer (ROT13)** | REBQRQ |
| **Author Team** | Team Delta |

**Description:** A cryptic crossword where each clue uses a Rosharan concept as the wordplay element. Across answers are Rosharan fauna/flora (chasmfiend, rockbud, cremling, whitespine). Down answers are Rosharan locations (Urithiru, Kholinar, Shinovar). Each clue works both as traditional cryptic wordplay AND as a Cosmere knowledge check.

**World/ Reference:** `world/systems/roshar.md` — flora/fauna references, Key Locations

**Extraction:** The completed crossword grid has highlighted cells. Reading the highlighted cells in order spells the answer.

**Interlock Notes:** The only crossword in the hunt. Requires careful cryptic clue construction ensuring each clue has a valid cryptic parse.

**Meta Role:** Lightweavers have 3 documented Ideals. The answer's 3rd letter feeds Meta II extraction (the "O" in ENVOY). This is the critical Lightweaver puzzle.

---

#### P23 — "Fabrial Workshop"

| Field | Value |
|-------|-------|
| **Pool ID** | D-03 |
| **Module** | D (Roshar World) |
| **Round** | 2 (Roshar) |
| **Difficulty** | ★★★ |
| **Mechanism** | Triple-matching (effect / type / gemstone) |
| **Order Assignment** | Bondsmiths (#10) |
| **Answer (ROT13)** | ORELO |
| **Author Team** | Team Delta |

**Description:** The solver is given descriptions of 8 magical effects and must determine which fabrial type produces each, what gemstone is used, and what spren is trapped. Each fabrial type (Heating, Spanreed, Soulcaster, etc.) uses a specific gemstone, which maps to a Radiant Order via the Polestones table.

**World/ Reference:** `world/systems/roshar.md` — Fabrial Types table, Gemstones and Stormlight section; `world/systems/knights-radiant.md` — Polestones table

**Extraction:** Each correctly matched gemstone name contributes a letter. The gemstone names, ordered by fabrial type, yield the answer through positional extraction.

**Interlock Notes:** Only confirmed fabrial-gemstone pairings are used (heating/ruby, Spanreed mechanics). [VERIFY] items avoided.

**Meta Role:** Bondsmiths have 4 documented Ideals. The answer's 4th letter feeds Meta II extraction (the "Y" in ENVOY).

---

#### P24 — "Stormwall"

| Field | Value |
|-------|-------|
| **Pool ID** | D-02 |
| **Module** | D (Roshar World) |
| **Round** | 2 (Roshar) |
| **Difficulty** | ★★★★ |
| **Mechanism** | Spatial/directional logic |
| **Order Assignment** | Elsecallers (#7) |
| **Answer (ROT13)** | GRZCRFG |
| **Author Team** | Team Delta |

**Description:** Highstorms go east-to-west, the Everstorm goes west-to-east. Given 8 locations on a simplified Rosharan map and storm timing descriptions, the solver determines which storm hit each location first and in what order. The map provides relative east/west positions: Shinovar (far west), Azir (west), Alethkar (central-east), Shattered Plains (east), with other locations between.

**World/ Reference:** `world/systems/roshar.md` — Highstorms and the Everstorm sections, Key Locations

**Extraction:** The order in which locations are first hit by either storm type encodes the answer. Each location's initial letter, in storm-arrival order, spells the answer.

**Interlock Notes:** Provides a simplified map (not demanding precise geographic knowledge beyond relative east/west positioning). The dual-storm directionality is the core puzzle mechanic.

**Meta Role:** Elsecallers have only 1 documented Ideal. Non-contributing in Meta II.

---

## ROUND 3: THE COSMERE — "The Shattering"

### Module E — The 16 Shards (P25-P30, Team Epsilon)

---

#### P25 — "Shattered and Whole"

| Field | Value |
|-------|-------|
| **Pool ID** | E-03 |
| **Module** | E (The 16 Shards) |
| **Round** | 3 (Cosmere) |
| **Difficulty** | ★★ |
| **Mechanism** | Categorization + count-based indexing |
| **Shard Assignment** | Honor |
| **Answer (ROT13)** | FGNGHF |
| **Author Team** | Team Epsilon |

**Description:** The solver sorts the 16 Shards by status: Splintered (Honor, Devotion, Dominion, Ambition, Virtuosity = 5), Held by Original Vessel (Cultivation, Endowment, Autonomy = 3), Held by New Vessel (Odium/Taravangian = 1), Combined (Preservation + Ruin → Harmony = 2 original → 1 combined), Uncertain (Mercy, Valor, Whimsy, Invention = 4).

**World/ Reference:** `world/systems/shards.md` — Shard Statuses Summary table

**Extraction:** The count of Shards in each category serves as an index into the category name: Splintered(5)→"Splintered"[5]="t", Held-Original(3)→"Original"[3]="i", etc. Or: the count per category encodes a letter via A1Z26. The 5 category counts (5, 3, 1, 2, 4) map to letters that spell the answer.

**Interlock Notes:** This puzzle establishes the Shard status vocabulary used throughout Round 3. It is the gentlest on-ramp for the Cosmere round.

**Meta Role:** Primarily references Honor (the most prominent Splintered Shard). Feeds Meta III by marking Honor as "represented."

---

#### P26 — "Vessel Roll Call"

| Field | Value |
|-------|-------|
| **Pool ID** | E-02 |
| **Module** | E (The 16 Shards) |
| **Round** | 3 (Cosmere) |
| **Difficulty** | ★★★ |
| **Mechanism** | Identification + extraction |
| **Shard Assignment** | Preservation |
| **Answer (ROT13)** | ZBEGNY |
| **Author Team** | Team Epsilon |

**Description:** Descriptions of 10-11 known original Vessels without naming their Shards. Each description references the Vessel's personality, fate, and home world. The solver identifies which Shard each Vessel held: Tanavast → Honor, Koravellium Avast → Cultivation, Rayse → Odium, Leras → Preservation, Ati → Ruin, Aona → Devotion, Skai → Dominion, Edgli → Endowment, Bavadin → Autonomy, Ulas Dal → Ambition, Sazed → Harmony.

**World/ Reference:** `world/systems/shards.md` — Original Vessel column

**Extraction:** For each matched Vessel-Shard pair, the solver takes a letter from the Intent name at a position determined by the Vessel name's length. The extracted letters spell the answer.

**Interlock Notes:** Only uses confirmed Vessel names (10-11 of 16). The 4-5 Shards with unknown Vessels are excluded from the identification set but noted as "unmatched."

**Meta Role:** Primarily references Preservation (Leras). Feeds Meta III.

---

#### P27 — "Odium's Kill List"

| Field | Value |
|-------|-------|
| **Pool ID** | E-04 |
| **Module** | E (The 16 Shards) |
| **Round** | 3 (Cosmere) |
| **Difficulty** | ★★★ |
| **Mechanism** | Identification + chronological ordering |
| **Shard Assignment** | Odium |
| **Answer (ROT13)** | JENGU |
| **Author Team** | Team Epsilon |

**Description:** The Shards that Odium Splintered are presented in cryptic clue form. The solver identifies each Splintered Shard (Ambition, Devotion, Dominion, Honor; plus Virtuosity which self-Splintered) and orders them by Odium's chronological campaign: Ambition first (attacked near Threnody), Devotion and Dominion together (Splintered on Sel), then Honor (Splintered on Roshar).

**World/ Reference:** `world/systems/shards.md` — Known Combatants section, Shard Interactions

**Extraction:** The Splintered Shards, in kill order, have their Intent name first letters read: A, D, D, H (plus V for Virtuosity's self-Splintering). A secondary extraction from the Intent names at specific positions spells the answer.

**Interlock Notes:** Virtuosity's self-Splintering is distinguished from Odium's kills. The puzzle must make this distinction clear.

**Meta Role:** Primarily references Odium. Feeds Meta III.

---

#### P28 — "Investiture Types"

| Field | Value |
|-------|-------|
| **Pool ID** | E-08 |
| **Module** | E (The 16 Shards) |
| **Round** | 3 (Cosmere) |
| **Difficulty** | ★★★ |
| **Mechanism** | Identification (magic system) + chained mapping |
| **Shard Assignment** | Endowment |
| **Answer (ROT13)** | TVSGRQ |
| **Author Team** | Team Epsilon |

**Description:** Descriptions of 10-12 magic systems without naming them: Allomancy (burn metals for power), Surgebinding (bond spren, use Stormlight), AonDor (draw Aons for effects), Awakening (use Breath to animate objects), Sand Mastery (white sand turns black), Hemalurgy (steal powers via metal spikes), Feruchemy (store attributes in metalminds), and others. The solver identifies each system and maps it to its source Shard.

**World/ Reference:** `world/systems/shards.md` — Investiture / Magic System column; cross-references to allomancy.md, knights-radiant.md

**Extraction:** Each identified magic system → source Shard → take a specific letter from the Intent name (position determined by the number of words in the magic system's name). The extracted letters spell the answer.

**Interlock Notes:** This is the grand unifying puzzle of Module E — it connects every magic system to its cosmic source.

**Meta Role:** Primarily references Endowment (BioChromatic Breath / Awakening). Feeds Meta III.

---

#### P29 — "The 4x4 Shard Grid"

| Field | Value |
|-------|-------|
| **Pool ID** | E-07 |
| **Module** | E (The 16 Shards) |
| **Round** | 3 (Cosmere) |
| **Difficulty** | ★★★★ |
| **Mechanism** | Grid logic (axis deduction + placement) |
| **Shard Assignment** | Autonomy |
| **Answer (ROT13)** | NFCRPG |
| **Author Team** | Team Epsilon |

**Description:** The 16 Shards must be arranged in a 4x4 grid. The solver must DISCOVER the axes (they are not provided). Clues indicate which Shards share a row and which share a column. Possible axes: Intent type (destructive/creative/neutral/personal), current status (Splintered/Held/Combined/Uncertain), or world association. The solver deduces the grid structure and places all 16 Shards.

**World/ Reference:** `world/systems/shards.md` — All Shard attributes

**Extraction:** Once placed, the grid reveals the answer through a specific reading path (diagonal, spiral, or highlighted cells).

**Interlock Notes:** Parallels the Allomancy table grid (P04) — both are 4x4 grids where the solver must deduce the organizational logic. The resonance is intentional.

**Meta Role:** Primarily references Autonomy (the most independently-placed Shard — its isolation helps anchor the grid). Feeds Meta III.

---

#### P30 — "The Shattering"

| Field | Value |
|-------|-------|
| **Pool ID** | E-11 |
| **Module** | E (The 16 Shards) |
| **Round** | 3 (Cosmere) |
| **Difficulty** | ★★★★★ |
| **Mechanism** | Circular logic (seating arrangement) |
| **Shard Assignment** | Virtuosity |
| **Answer (ROT13)** | BEVTVA |
| **Author Team** | Team Epsilon |

**Description:** 16 people stand in a circle (the 16 who took up Shards). Hoid is the 17th, standing outside. From clues about who stood next to whom and who took which piece (based on confirmed Vessel-Shard pairings, world associations, and relationship data), the solver deduces the full circular arrangement.

**World/ Reference:** `world/systems/shards.md` — The Shattering section, all Vessel-Shard pairings, Shard Interactions

**Extraction:** The circular arrangement, read from a specific starting position (Hoid's perspective), yields the answer through letter extraction from Vessel or Intent names.

**Interlock Notes:** This is the hardest puzzle in the hunt (★★★★★). The circular arrangement is puzzle-constructed, not canon-claimed. The puzzle must be explicit that the arrangement is hypothetical. Uses only confirmed Vessel names (others are labeled "Unknown Vessel #N").

**Meta Role:** Primarily references Virtuosity (the Shard that chose to self-Splinter — the most dramatic individual Shard story after Odium's campaign). Feeds Meta III.

---

### Module F — Hoid & Worldhoppers (P31-P36, Team Zeta)

---

#### P31 — "The Wanderer's Aliases"

| Field | Value |
|-------|-------|
| **Pool ID** | F-01 |
| **Module** | F (Hoid & Worldhoppers) |
| **Round** | 3 (Cosmere) |
| **Difficulty** | ★★★ |
| **Mechanism** | Identification (presence/absence) + alias extraction |
| **Shard Assignment** | Ruin |
| **Answer (ROT13)** | ZNFXRQ |
| **Author Team** | Team Zeta |

**Description:** Twelve book excerpts from across the Cosmere, each containing a character cameo. Not all are Hoid — some are other Worldhoppers or local characters. The solver identifies which excerpts feature Hoid and what alias he used. Confirmed appearances: Elantris (Dust/Hoid), Final Empire (informant), Hero of Ages (beggar), Secret History (Hoid), Warbreaker (storyteller), Stormlight books (Wit), Tress (narrator), Yumi (narrator), Emperor's Soul (Fool).

**World/ Reference:** `world/systems/hoid.md` — Appearances by Book, Known Names and Aliases

**Extraction:** For confirmed Hoid appearances, take the first letter of his alias in that book. The aliases' first letters, ordered by publication date of the book, spell the answer.

**Interlock Notes:** This is Module F's cornerstone and the puzzle most readers will be excited to attempt. Hoid's cameos are a beloved Cosmere tradition.

**Meta Role:** Connected to Ruin's domain (Hoid's encounter in Secret History near the Well). Feeds Meta III by marking Ruin as "represented."

---

#### P32 — "Cognitive Shadows"

| Field | Value |
|-------|-------|
| **Pool ID** | F-04 |
| **Module** | F (Hoid & Worldhoppers) |
| **Round** | 3 (Cosmere) |
| **Difficulty** | ★★★ |
| **Mechanism** | Identification + system matching |
| **Shard Assignment** | Harmony |
| **Answer (ROT13)** | YVATRE |
| **Author Team** | Team Zeta |

**Description:** Characters from across the Cosmere who persist after death as Cognitive Shadows are described: Kelsier (Preservation/Harmony's domain, Scadrial), the Heralds (Honor's Oathpact, Roshar), the Returned (Endowment's divine Breaths, Nalthis), the Fused (Odium's power, Roshar). The solver identifies each and determines which magic system/Shard sustains them.

**World/ Reference:** `world/systems/hoid.md` (cross-reference); `world/systems/scadrial.md` — Kelsier as Cognitive Shadow; `world/systems/roshar.md` — Heralds, Fused; `world/systems/shards.md` — Endowment (Returned)

**Extraction:** Each identified Cognitive Shadow's sustaining Shard name contributes a letter. The Shard names, ordered by the "age" of each Shadow (oldest to youngest), yield the answer.

**Interlock Notes:** This is the cross-world synthesis puzzle — the solver must recognize that different worlds' afterlife phenomena are all the same underlying Cosmere mechanism.

**Meta Role:** Connected to Harmony (Kelsier exists as a Cognitive Shadow in Harmony's domain). Feeds Meta III.

---

#### P33 — "Collected Powers"

| Field | Value |
|-------|-------|
| **Pool ID** | F-02 |
| **Module** | F (Hoid & Worldhoppers) |
| **Round** | 3 (Cosmere) |
| **Difficulty** | ★★★★ |
| **Mechanism** | Triple identification (ability / world / Shard) |
| **Shard Assignment** | Cultivation |
| **Answer (ROT13)** | TEBJGU |
| **Author Team** | Team Zeta |

**Description:** Hoid's abilities from multiple magic systems are described without naming the system or world. The solver identifies each ability (Yolish Lightweaving, Allomancy via lerasium, BioChromatic Breath/Awakening, Surgebinding via Cryptic bond, Fortune/Worldhopping), maps it to its source world (Yolen, Scadrial, Nalthis, Roshar, unknown), and then to the Shard that powers it (pre-Shattering, Preservation, Endowment, Honor/Cultivation, unknown).

**World/ Reference:** `world/systems/hoid.md` — Hoid's Collected Abilities table

**Extraction:** The triple identification (ability → world → Shard) produces a chain. From each Shard's Intent name, take a letter at a position determined by the order Hoid acquired the ability (1st acquired → letter 1, 2nd → letter 2, etc.). The extracted letters spell the answer.

**Interlock Notes:** The ultimate cross-system puzzle. Each ability is from a different Cosmere subsystem, requiring the broadest possible knowledge.

**Meta Role:** Connected to Cultivation (Hoid's growth/collection mirrors Cultivation's Intent). Feeds Meta III.

---

#### P34 — "Letters to a Dragon"

| Field | Value |
|-------|-------|
| **Pool ID** | F-06 |
| **Module** | F (Hoid & Worldhoppers) |
| **Round** | 3 (Cosmere) |
| **Difficulty** | ★★★★ |
| **Mechanism** | Attribution (author identification from text style) |
| **Shard Assignment** | Devotion |
| **Answer (ROT13)** | FPEVCG |
| **Author Team** | Team Zeta |

**Description:** Paraphrased excerpts from the Stormlight Archive epigraph letters. Each excerpt has the sender and recipient removed. The solver identifies who wrote each letter (Hoid, Frost, Endowment, Autonomy/Bavadin, other Shard Vessels) and to whom. Each Shard's writing voice reflects their Intent: Endowment is dismissive ("I have given and shall give no more"), Autonomy is threatening ("Do not come to my worlds").

**World/ Reference:** `world/systems/hoid.md` — Hoid's Correspondence section

**Extraction:** Each identified sender's Shard Intent name (or "Hoid" / "Frost" for non-Shard senders) contributes letters. The first letter of each sender, in the order the letters appear in the Stormlight books (WoK Part 2, WoR Part 4, OB, RoW), spell the answer.

**Interlock Notes:** Uses paraphrased excerpts (not direct quotes) to avoid copyright issues. The solver is directed to the specific Stormlight book parts for reference.

**Meta Role:** Connected to Devotion (the letters are acts of devotion/dedication, and Sel's Devotion is thematically linked to communication across distance). Feeds Meta III.

---

#### P35 — "The Memory Thief"

| Field | Value |
|-------|-------|
| **Pool ID** | F-11 |
| **Module** | F (Hoid & Worldhoppers) |
| **Round** | 3 (Cosmere) |
| **Difficulty** | ★★★★ |
| **Mechanism** | Error detection (true/false from reference data) |
| **Shard Assignment** | Whimsy |
| **Answer (ROT13)** | FGBYRA |
| **Author Team** | Team Zeta |

**Description:** Eight of Hoid's "memories" are presented. Each is a factual claim about a Cosmere event that Hoid witnessed. Some are accurate (matching the world/ data files); others have been "tampered with" by Taravangian-as-Odium (containing a subtle factual error). The solver identifies which memories are real and which are altered by cross-referencing against the world/ files.

**World/ Reference:** All world/ files (cross-reference against hoid.md, shards.md, scadrial.md, roshar.md)

**Extraction:** The altered memories contain errors. The specific element that was changed in each altered memory (e.g., a wrong metal name, wrong Shard, wrong character) gives a letter. The error-letters, in presentation order, spell the answer.

**Interlock Notes:** This puzzle requires the broadest fact-checking across all world/ files. Each "altered" fact must be detectable from the provided data (not from memory of the books).

**Meta Role:** Connected to Whimsy (the capricious nature of altered memories — truth and falsehood intermingled). Feeds Meta III.

---

#### P36 — "All Roads Lead to Hoid"

| Field | Value |
|-------|-------|
| **Pool ID** | F-12 |
| **Module** | F (Hoid & Worldhoppers) |
| **Round** | 3 (Cosmere) |
| **Difficulty** | ★★★★★ |
| **Mechanism** | Connection mapping (multi-source cross-reference) |
| **Shard Assignment** | Valor |
| **Answer (ROT13)** | GUERNQ |
| **Author Team** | Team Zeta |

**Description:** Twelve clues each reference events from different Cosmere books. Each event has a connection to Hoid (he was present, he caused it, he benefits from it, or he is affected by it). The solver traces all 12 connections back to Hoid and categorizes the connection type (Present, Caused, Benefited, Affected).

**World/ Reference:** `world/systems/hoid.md` — all sections, cross-referenced with all other world files

**Extraction:** The connection types (P, C, B, A) form a sequence. A secondary encoding (the specific book in which each connection occurs) provides numbers that index into the connection types or event descriptions. The final letters spell the answer.

**Interlock Notes:** This is the hardest puzzle in the hunt. It is the capstone of Module F and the final puzzle before the metas. Every clue requires cross-referencing at least two world/ files. Each connection must be traceable to a specific entry.

**Meta Role:** Connected to Valor (Hoid's courage in pursuing his quest across millennia — valor is what defines his persistence). Feeds Meta III by marking Valor as "represented."

---

## Summary Tables

### Difficulty Distribution

| Difficulty | Count | Puzzles |
|-----------|-------|---------|
| ★★ | 8 | P01, P02, P07, P08, P13, P19, P20, P25 |
| ★★★ | 14 | P03, P04, P09, P10, P14, P15, P16, P21, P22, P23, P26, P27, P28, P31, P32 |
| ★★★★ | 11 | P05, P06, P11, P12, P17, P18, P24, P29, P33, P34, P35 |
| ★★★★★ | 3 | P30, P36 |

### Mechanism Diversity (No Duplicates Within Module)

| Module | Mechanisms |
|--------|-----------|
| A | Identification+ordering, Pairing+extraction, ID+positional, Grid logic, Triple-matching, Deductive logic |
| B | Chronological ordering, ID+positional, Categorization, Cross-era matching, Logic grid, Twinborn deduction |
| C | Herald behavioral ID, Fill-in-blank+acrostic, Spren narrative ID, Character ID+indexing, Ring topology, Circular constraint |
| D | Form ID+indexing, Anagram+indexing, Rhythm ID, Cryptic crossword, Triple-matching, Spatial logic |
| E | Categorization+counting, Vessel ID, Chronological ordering, Magic-system ID, Grid axis deduction, Circular logic |
| F | Alias ID, Cognitive Shadow ID, Triple ID chain, Author attribution, Error detection, Connection mapping |

### Module-to-Round Mapping

| Round | Modules | Puzzles | Meta |
|-------|---------|---------|------|
| 1 (Scadrial) | A + B | P01-P12 | META-I: THE WELL |
| 2 (Roshar) | C + D | P13-P24 | META-II: THE OATHS |
| 3 (Cosmere) | E + F | P25-P36 | META-III: ADONALSIUM |
| Super | All | All metas | THE SEVENTEENTH SHARD |
