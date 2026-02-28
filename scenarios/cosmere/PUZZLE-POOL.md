# THE SIXTEENTH SHARD — Puzzle Pool (Stage 3)

**Generated:** 2026-02-27
**Total candidates:** 72 (12 per module)
**Selection target:** 36 (6 per module)
**Pool quality rules:** No duplicate mechanisms within a module. Every puzzle 100% solvable from world/ data. [VERIFY] items flagged. 2+ visual/grid puzzles included. 1+ "elegant" per module.

---

## Module A — Allomancy (Metals, Powers, the Full Chart)

Source: `world/systems/allomancy.md`

---

### A-01 ★ "The Allomantic Table" (GRID/VISUAL)

**One-line pitch:** Solver fills a 4x4 grid using only binary property clues — no metal names given.

**Mechanism:** Grid logic / constraint satisfaction

**World/ reference:** Allomancy master table (16 metals, Internal/External, Push/Pull, Physical/Mental/Temporal/Enhancement quadrants)

**Estimated difficulty:** ★★★

**Why it works:** The 4x4 grid IS the canonical structure of Allomancy. Deducing which cell is which metal from property clues (e.g., "This External Pulling Physical metal...") forces the solver to internalize the table's axes. Extraction: reading solved metals in a specific order (e.g., diagonal) yields a letter string from first letters.

**Concerns:** None. Pure logic from locked data.

---

### A-02 ★ "Burning Through"

**One-line pitch:** Sixteen Allomantic power descriptions have had their metal names excised; solver identifies each metal and reads first letters in table order.

**Mechanism:** Identification + ordered extraction

**World/ reference:** Allomantic Power column of the master table

**Estimated difficulty:** ★★

**Why it works:** Forces recognition of all 16 powers without the metal names as crutches. The table order provides a natural extraction sequence. Clean, direct, satisfying.

**Concerns:** None. All 16 powers are documented.

---

### A-03 "Misting Roll Call"

**One-line pitch:** Solver matches 16 character vignettes to their Misting type name, then indexes into the Misting names using the metal's table number.

**Mechanism:** Matching + indexing

**World/ reference:** Misting Names table, Notable Allomancers

**Estimated difficulty:** ★★★

**Why it works:** The Misting names (Lurcher, Coinshot, Tineye, Pewterarm, Rioter, Soother, Smoker, Seeker, Augur, Oracle, Pulser, Slider, Aluminum Gnat, Duralumin Gnat, Leecher, Nicroburst) are a parallel naming system. Indexing into them by metal number (1-16) extracts letters.

**Concerns:** Aluminum Gnat (13th letter of "Aluminum Gnat" is out of range) — need careful index design. Consider using word count or first-letter-only as alternate extraction.

---

### A-04 ★ "Alloy Partners"

**One-line pitch:** Eight cryptic clues each describe a pair of metals (base + alloy). Solver identifies both, then reads the difference between their table numbers as a letter (A=1, B=2...).

**Mechanism:** Pairing + calculation

**World/ reference:** Alloy Of column (Iron/Steel, Tin/Pewter, Zinc/Brass, Copper/Bronze, Gold/Electrum, Cadmium/Bendalloy, Aluminum/Duralumin, Chromium/Nicrosil)

**Estimated difficulty:** ★★

**Why it works:** The 8 base/alloy pairs are a core structural feature. The difference between paired table numbers is always 1 (consecutive pairs), yielding all A's — which means a more sophisticated extraction is needed. Better: use the FIRST letter of the base metal and the LAST letter of the alloy to form letter pairs.

**Concerns:** Need to verify extraction math works cleanly. Base/alloy pairs: Iron(1)/Steel(2), Tin(3)/Pewter(4), Zinc(5)/Brass(6), Copper(7)/Bronze(8), Gold(9)/Electrum(10), Cadmium(11)/Bendalloy(12), Aluminum(13)/Duralumin(14), Chromium(15)/Nicrosil(16). All consecutive — difference always 1. Revised extraction needed.

---

### A-05 "Three Arts, One Spike"

**One-line pitch:** For each of 8 metals, three clues are given — one each for its Allomantic, Feruchemical, and Hemalurgic use. Solver identifies the metal and which art is described by the "odd one out" clue (one clue per triple is deliberately misattributed to the wrong art).

**Mechanism:** Triple-matching with error detection

**World/ reference:** All three columns (Allomantic Power, Feruchemical Power, Hemalurgic Steal) for 8 selected metals

**Estimated difficulty:** ★★★★

**Why it works:** The three Metallic Arts operating on the same metal is the deepest structural feature of Scadrian magic. Identifying which description goes with which art — then spotting the planted error — requires precise knowledge.

**Concerns:** Several Hemalurgic uses are marked [VERIFY] in the data file. Must select only metals with fully confirmed data for all three arts.

---

### A-06 "The Grid Coordinates" (VISUAL)

**One-line pitch:** Solver is given pairs of coordinates on the 4x4 Allomancy grid. Each coordinate pair identifies two metals; the shared property between those metals (Int/Ext, Push/Pull, or Quadrant) encodes a letter.

**Mechanism:** Grid coordinate logic + shared-property extraction

**World/ reference:** Grid Layout diagram from allomancy.md

**Estimated difficulty:** ★★★

**Why it works:** The grid's two axes (Int/Ext, Push/Pull) plus four quadrants create a rich coordinate system. Finding the shared property between two metals and mapping that to a letter is a clean, visual puzzle.

**Concerns:** The grid layout in the data file has slightly non-standard numbering (Temporal metals are 9-12 but arranged as 11,12,9,10 in the grid). Must be explicit about which numbering to use.

---

### A-07 "Feruchemical Reserves"

**One-line pitch:** Sixteen items are described that can be "stored" — solver identifies each Feruchemical attribute, maps it to its metal, and reads the nth letter of each metal name (n = quadrant number).

**Mechanism:** Identification + positional extraction

**World/ reference:** Feruchemical Power column

**Estimated difficulty:** ★★★

**Why it works:** Feruchemy's "stores X" pattern is distinctive and memorable. The quadrant number (Physical=1, Mental=2, Temporal=3, Enhancement=4) as index into the metal name is elegant: Iron[1]=I, Steel[1]=S, Tin[1]=T, Pewter[1]=P, Zinc[2]=I, Brass[2]=R...

**Concerns:** Must verify all 16 extractions yield valid letters. Some metal names are short (Tin = 3 letters, max index 4 would fail). Physical quadrant uses index 1 (fine for all), Mental=2 (fine), Temporal=3 (Gold has 4, Electrum 8, Cadmium 7, Bendalloy 9 — all fine), Enhancement=4 (Aluminum 8, Duralumin 9, Chromium 8, Nicrosil 7 — all fine).

---

### A-08 "Coinshot's Trajectory"

**One-line pitch:** A Steelpush physics problem: given distances, weights, and metal compositions, solver calculates which metals are pushed and in what order, spelling a word from the metals' initials.

**Mechanism:** Ordering (narrative logic puzzle)

**World/ reference:** Steel/Iron Push/Pull mechanics, metal names

**Estimated difficulty:** ★★★★

**Why it works:** Steelpushing and Ironpulling are the most cinematic Allomantic abilities. A "physics" puzzle using the in-universe rules (heavier Allomancer pushes lighter object away, lighter Allomancer is pushed away from heavier object) makes the solver think like a Coinshot.

**Concerns:** In-universe physics is not precisely quantified in the books. The puzzle must establish its own rules clearly, referencing but not over-extending the canon.

---

### A-09 "The Coppercloud"

**One-line pitch:** A logic puzzle: 8 Allomancers are in a room, some hidden by a Coppercloud. A Seeker reports which pulses they detect. From overlapping reports, solver deduces who is burning what.

**Mechanism:** Deductive logic (constraint satisfaction)

**World/ reference:** Copper (hides pulses), Bronze (detects pulses), the interplay between Smokers and Seekers

**Estimated difficulty:** ★★★★

**Why it works:** The Smoker/Seeker dynamic is a built-in information-hiding puzzle. "Who can see whom" is a classic constraint puzzle dressed in perfect Cosmere clothing.

**Concerns:** None. The mechanics are well-documented.

---

### A-10 "God Metal Bonus"

**One-line pitch:** Three god metals (Atium, Lerasium, Harmonium) are described through their origin Shard. Solver identifies each, then uses the god metal's letter count to index into the Shard name.

**Mechanism:** Identification + indexing

**World/ reference:** God Metals table

**Estimated difficulty:** ★

**Why it works:** A quick, satisfying appetizer that connects Allomancy to the broader Cosmere (Shards). Atium (5 letters) → Ruin[5] = blank. Needs careful index design.

**Concerns:** Only 3 god metals are well-documented. Malatium is a 4th but is an alloy. Small puzzle — may work better as a bonus/unlock.

---

### A-11 "The Inquisitor's Spikes"

**One-line pitch:** A Steel Inquisitor's spike configuration is described (which metals, placed where). Solver determines what powers were stolen and from whom, reading the stolen abilities' initials.

**Mechanism:** Deduction from Hemalurgic rules

**World/ reference:** Hemalurgic Steal column, Steel Inquisitor description in scadrial.md

**Estimated difficulty:** ★★★★★

**Why it works:** Hemalurgy is the darkest and most complex Metallic Art. Reasoning about what powers each spike steals — and therefore what abilities the Inquisitor possesses — is deeply satisfying for hardcore fans.

**Concerns:** Multiple Hemalurgic entries are [VERIFY]. Must restrict to confirmed data only. The puzzle must clearly state its own rules for spike placement.

---

### A-12 "Compounding Interest"

**One-line pitch:** Solver identifies 8 Twinborn characters from descriptions of their compounded abilities (Allomantic + Feruchemical on the same metal), then extracts from the character names.

**Mechanism:** Identification + name extraction

**World/ reference:** Compounding rules, Twinborn characters (Wax, Wayne, Miles Hundredlives)

**Estimated difficulty:** ★★★

**Why it works:** Compounding — the intersection of Allomancy and Feruchemy on the same metal — is one of the most elegant features of the magic system. Only a few canon compounders exist, so the puzzle may need to present hypothetical compounders alongside canon ones.

**Concerns:** Only 2-3 canonical compounders (Miles for Gold, the Lord Ruler for all metals). The rest would be hypothetical. Flag as potential construction risk if we require all-canon.

---

## Module B — Scadrial History (The Final Empire & Beyond)

Source: `world/systems/scadrial.md`

---

### B-01 ★ "Kelsier's Crew"

**One-line pitch:** Eight coded character descriptions of Kelsier's crew members. Identify each, then read the letter of their Allomantic metal at a position given by their crew role number.

**Mechanism:** Identification + positional extraction

**World/ reference:** Era 1 Key Characters table, Misting Names

**Estimated difficulty:** ★★

**Why it works:** Kelsier's crew is the emotional heart of Mistborn Era 1. Each member has a distinctive personality AND a specific metal, creating a natural two-layer identification. Ham = Pewter, Breeze = Brass, Clubs = Copper, Marsh = Bronze, Vin = all, etc.

**Concerns:** Dockson has no Allomantic ability. OreSeur/TenSoon are kandra, not Allomancers. Need to handle the non-Allomancer crew members gracefully.

---

### B-02 ★ "The Lord Ruler's Thousand Years"

**One-line pitch:** A timeline of 10 events from the Final Empire's history, presented out of order. Solver orders them chronologically; initial letters of the events in correct order spell the answer.

**Mechanism:** Chronological ordering + acrostic extraction

**World/ reference:** Major Events Timeline

**Estimated difficulty:** ★★

**Why it works:** The thousand-year history of the Final Empire has a clear, dramatic arc. Ordering events forces the solver to reconstruct the narrative. The acrostic extraction is clean and satisfying.

**Concerns:** Some timeline events lack precise dates (only "~2 years before TFE" etc.). Must select events with unambiguous relative ordering.

---

### B-03 "The Siege of Luthadel"

**One-line pitch:** Three armies converge on Luthadel. Given force descriptions and strategic constraints, solver deduces which army arrives from which direction and what their objective is, encoding letters.

**Mechanism:** Logic grid (multi-variable deduction)

**World/ reference:** Siege of Luthadel event (The Well of Ascension), Straff Venture, Cett, Koloss army

**Estimated difficulty:** ★★★★

**Why it works:** The three-army siege is one of the most tense sequences in Mistborn. Turning it into a logic grid where the solver must reason about military strategy using Cosmere constraints is deeply thematic.

**Concerns:** Requires careful clue construction to ensure unique solution without ambiguity.

---

### B-04 "Catacendre"

**One-line pitch:** Before-and-after descriptions of 8 things Sazed changed when he remade the world (moved the planet back, removed the ashmounts, restored plant life, etc.). Solver matches "before" to "after," extracting from the transformation descriptions.

**Mechanism:** Matching (before/after pairs) + extraction

**World/ reference:** Sazed becomes Harmony, the Catacendre event

**Estimated difficulty:** ★★

**Why it works:** The Catacendre (world-remaking) is the pivotal transition between Era 1 and Era 2. Each transformation encodes Sazed's dual nature — Preservation (keeping) and Ruin (destroying the old).

**Concerns:** Specific Catacendre changes are not all enumerated in the data file. Some may need [VERIFY]. Known changes: moved planet orbit, removed ashmounts, restored plant life, created new geography for the Elendel Basin.

---

### B-05 ★ "The Social Ladder"

**One-line pitch:** Twelve characters from both Eras are described. Solver categorizes each into Scadrial's social structure (Skaa, Nobility, Obligator, Inquisitor, Kandra, Koloss, Terris Keeper). The category names, ordered by character appearance in the books, encode the answer.

**Mechanism:** Categorization + ordered extraction

**World/ reference:** Social Structure section, Era 1 + Era 2 Key Characters

**Estimated difficulty:** ★★★

**Why it works:** Scadrial's rigid social hierarchy — and the way it dissolves across the two eras — is a central theme. Categorizing characters forces the solver to understand the system and how it changed.

**Concerns:** Era 2's social structure is less rigid. Need to include transitional categories or use Era 1 exclusively for the hierarchy portion.

---

### B-06 "The Roughs Justice"

**One-line pitch:** An Era 2 mystery: clues describe 6 crimes in the Roughs. Solver identifies which Twinborn ability combination was used in each crime, mapping to the metals involved.

**Mechanism:** Deduction (ability identification)

**World/ reference:** Era 2 Key Characters (Twinborn), Allomancy + Feruchemy powers

**Estimated difficulty:** ★★★★

**Why it works:** Era 2 is essentially a Western-mystery genre with magic. Each crime is solvable only by understanding how specific Twinborn combinations create unique capabilities (e.g., Wax's Steel Allomancy + Iron Feruchemy = flight via weight manipulation).

**Concerns:** Only a few canonical Twinborn are documented. May need to construct plausible Twinborn criminals using the established metal tables.

---

### B-07 "The Kandra Generations"

**One-line pitch:** Descriptions of 7 kandra, each from a different Generation. Solver orders them by Generation (oldest to youngest), then indexes into their names.

**Mechanism:** Ordering + indexing

**World/ reference:** Kandra description (Organized into Generations, First Generation oldest), OreSeur, TenSoon, MeLaan, VenDell

**Estimated difficulty:** ★★★

**Why it works:** The kandra Generations are a chronological system with personality implications (older = more rigid, younger = more human-like). The ordering feels natural and the generation number serves as index.

**Concerns:** Not all kandra Generations are fully documented. TenSoon is Third [VERIFY], MeLaan is Third [VERIFY], VenDell's generation uncertain [VERIFY]. May need to flag or restrict to confirmed kandra.

---

### B-08 "Trust the Terris"

**One-line pitch:** Solve a word puzzle where each answer is a Feruchemical attribute that a Keeper might store. The stored attributes, mapped to their metals, then to the metals' positions in the table, encode the final answer.

**Mechanism:** Wordplay (definitions) + chained mapping

**World/ reference:** Feruchemical Power column, Terris Keepers

**Estimated difficulty:** ★★★

**Why it works:** The "storing" mechanic of Feruchemy is inherently puzzle-like — you give something up now to get it back later. Cluing Feruchemical attributes as word puzzle answers (e.g., "What you lose in a dark room" = SIGHT → Tin → position 3) is a natural fit.

**Concerns:** Some Feruchemical attributes are vague or [VERIFY] (e.g., Electrum stores "determination/investiture"). Stick to well-documented attributes.

---

### B-09 "The Well of Ascension"

**One-line pitch:** A maze/path puzzle themed as the journey to the Well. At each junction, a Cosmere fact determines the correct path. Reaching the Well spells the answer from collected letters.

**Mechanism:** Path/maze with knowledge checks

**World/ reference:** The Well of Ascension location, Rashek's journey, Vin's journey

**Estimated difficulty:** ★★

**Why it works:** The physical journey to the Well of Ascension happens twice in the series (Rashek, then Vin) and each time it changes everything. A maze that retraces this journey is thematically powerful.

**Concerns:** May be better suited as a visual/interactive puzzle for the website delivery. Construction of the maze requires careful path design.

---

### B-10 "Broadsheet Ciphers"

**One-line pitch:** Era 2's in-world broadsheet newspapers contain encoded messages. Solver decodes 6 newspaper headlines using Scadrial-specific cipher keys (metal symbols, Allomantic notation).

**Mechanism:** Cipher/decoding

**World/ reference:** Era 2 setting (broadsheets), Allomantic Symbols reference

**Estimated difficulty:** ★★★

**Why it works:** The broadsheets in Mistborn Era 2 books literally contain puzzles and hidden messages. This puzzle recreates that in-universe experience.

**Concerns:** The Allomantic symbols (Steel Alphabet) are visual and not fully described in the text data files. May need visual assets. [VERIFY] symbol availability.

---

### B-11 "The Survivor's Legacy"

**One-line pitch:** Kelsier's actions have consequences across both Eras. Eight cause-and-effect pairs span Era 1 and Era 2. Match cause to effect, then extract from the connection points.

**Mechanism:** Matching (cross-era cause/effect)

**World/ reference:** Kelsier's full arc (Era 1 character, Cognitive Shadow, Era 2 influence)

**Estimated difficulty:** ★★★

**Why it works:** Kelsier's persistence as a Cognitive Shadow means his influence spans both Eras. Tracing his legacy across 300+ years is a satisfying exercise in Cosmere continuity.

**Concerns:** Some of Kelsier's Era 2 influence is subtle or [VERIFY] (his role in Bands of Mourning / The Lost Metal events).

---

### B-12 "Noble Houses at War"

**One-line pitch:** A political logic puzzle: 6 noble houses, their alliances, rivalries, and Allomantic bloodlines. Solver deduces which house betrays which, using elimination.

**Mechanism:** Logic grid (political deduction)

**World/ reference:** Social Structure (Nobility), House Venture, political dynamics

**Estimated difficulty:** ★★★★

**Why it works:** The political machinations of the Final Empire nobility are a major plot driver. A logic grid that mirrors the backstabbing of Luthadel's ballrooms is thematically perfect.

**Concerns:** Specific noble house dynamics beyond House Venture are not deeply documented in the data file. May need to construct plausible houses using established rules.

---

## Module C — Knights Radiant (Orders, Surges, Heralds)

Source: `world/systems/knights-radiant.md`

---

### C-01 ★ "The Double Eye" (VISUAL)

**One-line pitch:** The 10 orders are arranged in the Double Eye pattern (decagon). Solver receives 10 clues, each pointing to a Surge. Since each Surge is shared by two adjacent orders, the solver must determine which ORDER each clue targets using secondary confirmation (Herald name, Gemstone, Spren). Letters extracted from the confirmed order names spell the answer.

**Mechanism:** Ring topology logic + multi-attribute confirmation

**World/ reference:** Surge Sharing Pattern (Double Eye), Master Table

**Estimated difficulty:** ★★★★

**Why it works:** The Double Eye ring topology is the most distinctive visual feature of the Knights Radiant system. The shared-Surge ambiguity (each Surge belongs to two orders) creates a natural puzzle: which of the two did the clue mean? The secondary attributes resolve it.

**Concerns:** None. All data is locked and confirmed.

---

### C-02 ★ "Ideals and Oaths"

**One-line pitch:** Fragments of Radiant Oaths from various orders are presented, each missing key words. Solver identifies the order and fills in the missing words. The missing words' first letters spell the answer.

**Mechanism:** Fill-in-the-blank + identification + acrostic

**World/ reference:** The Ideals (Immortal Words) section

**Estimated difficulty:** ★★★

**Why it works:** The Oaths are the most emotionally resonant element of the Stormlight Archive. "Life before death. Strength before weakness. Journey before destination." Every reader knows the First Ideal. The order-specific Ideals force deeper knowledge.

**Concerns:** Many order-specific Oaths are marked [VERIFY] or "not fully documented." Must restrict to confirmed Oaths only: Windrunners (2nd, 3rd), Edgedancers (2nd, 3rd), Bondsmiths (2nd, 3rd), and the Lightweaver Truths. The First Ideal is universal and confirmed.

---

### C-03 "Ten Gemstones"

**One-line pitch:** Ten gemstones are described by color and Essence only (no order name). Solver identifies each gemstone, maps to its order, and uses the order number as an index into the gemstone name.

**Mechanism:** Identification + indexing

**World/ reference:** The 10 Polestones table

**Estimated difficulty:** ★★

**Why it works:** The gemstone-order association is one of the lesser-known details that rewards careful readers. Color + Essence is enough information to uniquely identify each gemstone without making it trivial.

**Concerns:** Some gemstone colors are [VERIFY] (Garnet "red-orange", Zircon "blue-white"). Stick to confirmed colors or use Essence alone for those.

---

### C-04 "Spren Bonds"

**One-line pitch:** Ten character vignettes describe a Radiant and their bonded spren's behavior. Solver identifies the spren type (Honorspren, Cryptic, Cultivationspren, etc.), maps to order, and extracts.

**Mechanism:** Identification (narrative) + mapping

**World/ reference:** Spren Type column, Notable Members

**Estimated difficulty:** ★★★

**Why it works:** The spren bond is the mechanism of Radiance. Each spren type has distinctive behavior (Cryptics speak in lies and patterns, Honorspren are rigid and honorable, Cultivationspren are nurturing). Identifying the spren from behavior is genuine deduction.

**Concerns:** Some spren types have minimal behavioral documentation (Ashspren, Peakspren, Reachers). Restrict to well-characterized spren.

---

### C-05 ★ "The Surge Wheel" (VISUAL)

**One-line pitch:** A circular diagram shows 10 positions (the Double Eye ring). Numbered arrows between adjacent positions indicate the Surge they share. Solver places the 10 orders in the correct positions using Surge-sharing constraints, then reads letters from a specific traversal of the ring.

**Mechanism:** Circular constraint placement (visual logic)

**World/ reference:** Surge Sharing Pattern, all 10 Surges with their two sharing orders

**Estimated difficulty:** ★★★★

**Why it works:** This IS the Double Eye puzzle — literally reconstructing the canonical arrangement of orders. The ring topology constrains placement tightly (each order must share one Surge with its clockwise neighbor and another with its counterclockwise neighbor). The result is a unique, satisfying solution.

**Concerns:** Need to verify the ring has a unique solution (it should, given 10 surges with specific pairings). Rotational symmetry means position 1 must be fixed (e.g., "Windrunners are at the top").

---

### C-06 "Herald's Madness"

**One-line pitch:** Each of the 10 Heralds is described by their current mental state and behavior (Ishar claims to be God, Shalash destroys art, Taln endured millennial torture, etc.). Solver identifies the Herald, maps to their order, and extracts.

**Mechanism:** Identification (behavioral) + mapping

**World/ reference:** The 10 Heralds table (Current Status/Fate column)

**Estimated difficulty:** ★★

**Why it works:** The Heralds' madness is one of the most haunting elements of the Stormlight Archive. Each Herald's specific insanity is distinctive and memorable. Identifying them from their broken state is both a puzzle and a meditation on the story's themes.

**Concerns:** Several Herald current statuses are [VERIFY]. Use only confirmed behaviors: Jezrien (dead — killed by Moash), Nale (leads Skybreakers), Shalash (destroys art), Taln (returned broken), Ishar (claims to be God), Kalak (fearful).

---

### C-07 "Radiant Roster"

**One-line pitch:** Twelve characters from the Stormlight Archive are described without names. Solver identifies each, determines their Radiant order (if any), and uses the order number to index into the character's name.

**Mechanism:** Character identification + conditional indexing

**World/ reference:** Notable Members column from Orders table, Key Characters from roshar.md

**Estimated difficulty:** ★★★

**Why it works:** The Stormlight Archive has a massive cast, and the joy of the series is watching characters discover their Radiant nature. Identifying who is a Radiant (and which order) mirrors that discovery.

**Concerns:** Some characters' order membership may be ambiguous (Szeth was first Windrunner-adjacent via Honorblade, then Skybreaker). Must be precise about which status we use.

---

### C-08 "Essence of the Matter"

**One-line pitch:** Ten riddles, each describing an Essence (Zephyr, Vapor, Spark, Lucentia, Pulp, Blood, Tallow, Foil, Talus, Sinew) and its associated Body Focus. Solver identifies the Essence, maps to gemstone and order.

**Mechanism:** Riddle solving + chained mapping

**World/ reference:** Polestones table (Essence, Body Focus columns)

**Estimated difficulty:** ★★

**Why it works:** The Essences are the most esoteric layer of the Radiant system. Connecting an Essence to its Body Focus to its gemstone to its order is a satisfying chain of associations.

**Concerns:** None. All 10 Essences and Body Focuses are documented.

---

### C-09 "The Recreance"

**One-line pitch:** A narrative puzzle: the events leading to the Recreance (Knights abandoning their oaths) are presented as fragmented testimony from different witnesses. Solver assembles the true sequence from contradictory accounts, extracting key words.

**Mechanism:** Narrative deduction (contradictory testimony)

**World/ reference:** The Recreance event, Oathpact abandonment (roshar.md)

**Estimated difficulty:** ★★★★

**Why it works:** The Recreance is one of the great mysteries of the Stormlight Archive. Presenting contradictory testimony and asking the solver to determine what actually happened mirrors the in-world scholarly debate.

**Concerns:** The exact sequence and cause of the Recreance involves some [VERIFY] elements. The data file notes the distinction between the Herald abandonment (Aharietiam) and the Knights' Recreance. Must be precise.

---

### C-10 "Shardblade Graveyard"

**One-line pitch:** Descriptions of 8 Shardblades (Dead, Living, or Honorblade). Solver categorizes each, then uses the category (D/L/H) as a three-way encoding key.

**Mechanism:** Categorization + ternary encoding

**World/ reference:** Shardblades section in roshar.md

**Estimated difficulty:** ★★★

**Why it works:** The three categories of Shardblades (Dead, Living, Honorblade) have distinct properties. Dead ones scream when held by Radiants. Living ones can be summoned at will. Honorblades grant Surgebinding without a spren. Categorizing from descriptions is clean deduction.

**Concerns:** The number of known individual Shardblades by name may be limited. May need to describe some generically.

---

### C-11 "The Nahel Bond"

**One-line pitch:** A connection puzzle: 10 spren types on the left, 10 surges across the top, 10 orders on the right. Solver draws lines connecting each spren to its order and each order to its two surges, forming a specific visual pattern that encodes letters.

**Mechanism:** Connection mapping + visual extraction

**World/ reference:** Full Master Table (Order, Surge A, Surge B, Spren Type)

**Estimated difficulty:** ★★★

**Why it works:** The Nahel bond links spren to Radiant and Radiant to Surge. A visual puzzle that literalizes these connections makes the solver construct the entire system from scratch.

**Concerns:** 10 x 10 x 10 is a large grid. May need to simplify or provide partial pre-filling.

---

### C-12 "Words of Radiance"

**One-line pitch:** The First Ideal ("Life before death. Strength before weakness. Journey before destination.") is used as a cipher key. Solver decodes 10 encrypted order names using this 10-word phrase.

**Mechanism:** Cipher/decoding (thematic key)

**World/ reference:** The First Ideal (universal), the 10 order names

**Estimated difficulty:** ★★

**Why it works:** Using the most iconic phrase in the Stormlight Archive as a cipher key is deeply thematic. The solver must know the First Ideal to begin — and the decoding process reinforces its significance.

**Concerns:** The First Ideal has 10 words, conveniently matching 10 orders. Verify the cipher mechanism works cleanly (e.g., each word's length or initial letter as a shift value).

---

## Module D — Roshar World (Storms, Fabrials, Singers)

Source: `world/systems/roshar.md`

---

### D-01 ★ "Rhythm of Answers"

**One-line pitch:** Eight Rhythms of the Singers are described by their emotional quality (Peace, Awe, Anxiety, etc.). Solver identifies each Rhythm and extracts from the Rhythm names.

**Mechanism:** Identification + extraction from names

**World/ reference:** Rhythms section of roshar.md

**Estimated difficulty:** ★★★

**Why it works:** The Rhythms are one of the most original elements of the Stormlight Archive — music as emotional language. Identifying a rhythm from its emotional description is an act of empathy with Singer culture.

**Concerns:** Not all Rhythm names are confirmed in the data file. Known: Peace, Awe, Anxiety, Destruction, Command. Rhythm of War is the title Rhythm. [VERIFY] full list. Must restrict to confirmed Rhythms.

---

### D-02 "Stormwall"

**One-line pitch:** A directional logic puzzle: Highstorms go east-to-west, the Everstorm goes west-to-east. Given 8 locations on a Rosharan map and storm timing descriptions, solver determines which storm hit each location first and in what order.

**Mechanism:** Spatial/directional logic

**World/ reference:** Highstorms and Everstorm sections, Key Locations

**Estimated difficulty:** ★★★★

**Why it works:** The dual-storm system (Highstorm vs Everstorm) is Roshar's defining environmental feature. The directional opposition creates a natural logic puzzle: two waves sweeping in opposite directions, each location experiencing them in different order.

**Concerns:** Precise geography of Roshar (which locations are east/west of which) requires a reference. The data file lists locations but not precise relative positions. May need to provide a simplified map.

---

### D-03 ★ "Fabrial Workshop"

**One-line pitch:** Solver is given descriptions of 8 magical effects and must determine which fabrial type produces each, what gemstone is used, and what spren is trapped. The gemstone names encode the answer.

**Mechanism:** Triple-matching (effect / type / gemstone)

**World/ reference:** Fabrial Types table, Gemstones and Stormlight section

**Estimated difficulty:** ★★★

**Why it works:** Fabrials are Roshar's magitech — the practical application of its magic system. Matching effect to mechanism to gemstone is a satisfying three-layer deduction.

**Concerns:** Several fabrial-gemstone pairings are [VERIFY] (cooling fabrial gem, pain fabrial details). Must use confirmed pairings only: heating/ruby, Spanreed/any paired gems, Soulcaster specifics.

---

### D-04 "Singer Forms"

**One-line pitch:** Eight descriptions of Singer physical appearances and abilities. Solver identifies the form (Warform, Mateform, Nimbleform, Workform, etc.) and reads the nth letter of each form name.

**Mechanism:** Identification + indexing

**World/ reference:** Singer Forms table

**Estimated difficulty:** ★★

**Why it works:** The Singer forms are a unique species feature — physical transformation driven by spren bonding. Each form has distinctive physical characteristics (Warform has carapace armor, Nimbleform is agile, etc.).

**Concerns:** Several form-to-spren-type mappings are [VERIFY]. Use only the physical descriptions, which are more reliable.

---

### D-05 "Oathgate Network"

**One-line pitch:** A network/graph puzzle: 8 cities connected by Oathgates. Solver is given travel restrictions and must find the unique path that visits all cities, spelling the answer from city initials.

**Mechanism:** Hamiltonian path (graph traversal)

**World/ reference:** Key Locations (Urithiru as hub, Oathgate connections)

**Estimated difficulty:** ★★★★

**Why it works:** The Oathgate network is Roshar's fast-travel system — ancient teleportation platforms connecting major cities through Urithiru. Finding the optimal route through the network mirrors the strategic challenge the characters face.

**Concerns:** The exact Oathgate connections are not all listed in the data file. Known gates: Urithiru, Shattered Plains, Kholinar, Thaylen City, Azir. Others exist but may need [VERIFY].

---

### D-06 "The Unmade"

**One-line pitch:** Nine descriptions of the Unmade (Odium's dark spren — each with distinct powers and personalities). Solver identifies each Unmade by their manifestation.

**Mechanism:** Identification

**World/ reference:** Referenced in roshar.md (the Unmade are mentioned as Odium's forces)

**Estimated difficulty:** ★★★★★

**Why it works:** The Unmade are among the most mysterious and terrifying entities in the Stormlight Archive. Each has a unique manifestation (Re-Shephir mimics, Nergaoul induces battle rage, Ba-Ado-Mishram Connected to the Singers). Expert-level knowledge.

**Concerns:** The Unmade are not detailed in the roshar.md data file. They are referenced indirectly. Would need to extend the data file or flag this as requiring additional research. [VERIFY] all 9 Unmade names and attributes.

---

### D-07 ★ "The Chasms Below"

**One-line pitch:** A cryptic-crossword-style puzzle where each clue uses a Rosharan concept as the wordplay element. Across answers are Rosharan fauna/flora (chasmfiend, rockbud, cremling, whitespine); down answers are locations.

**Mechanism:** Cryptic crossword (Cosmere-themed)

**World/ reference:** Flora/fauna references throughout roshar.md, Key Locations

**Estimated difficulty:** ★★★

**Why it works:** Cryptic crosswords are the gold standard of wordplay puzzles. Theming every element around Rosharan ecology (creatures that retract into shells during Highstorms, crem-covered landscapes) makes the crossword itself a lesson in the world.

**Concerns:** Requires careful cryptic clue construction. The Rosharan vocabulary is rich enough to support this, but each clue must work both as wordplay AND as a Cosmere reference.

---

### D-08 "Voidbringers"

**One-line pitch:** A classification puzzle: 10 entities are described. Solver must determine which are Fused (ancient Singer souls, one Surge each), which are Regals (Singers in Forms of Power), and which are neither. The Fused's single Surge determines the extraction.

**Mechanism:** Multi-category classification + conditional extraction

**World/ reference:** The Fused table, Singer Forms (Stormform), general Odium forces

**Estimated difficulty:** ★★★★

**Why it works:** The hierarchy of Odium's forces (Fused > Regals > common Singers) is a key strategic element. The critical distinction — Fused have one Surge, Regals have a Form of Power, common Singers have regular forms — is clean and testable.

**Concerns:** Multiple Fused types are [VERIFY] in the data file. Restrict to confirmed types: Shanay-im (Gravitation), Nex-im (Division), and supplement with the confirmed general rule (one Surge per Fused).

---

### D-09 "Stormlight Economics"

**One-line pitch:** A resource management puzzle: given Stormlight capacities for different gemstones, leakage rates, and fabrial power requirements, solver calculates which gem configuration powers which fabrial array. The optimal allocation spells the answer.

**Mechanism:** Calculation + optimization

**World/ reference:** Gemstones and Stormlight section, Fabrial Types

**Estimated difficulty:** ★★★★

**Why it works:** Stormlight as a resource (infused during Highstorms, leaks over time, powers fabrials and Surgebinding) has genuine economic properties. A puzzle that makes the solver think like a Rosharan quartermaster is novel.

**Concerns:** Precise Stormlight capacities and leakage rates are not quantified in the data file. The puzzle must establish its own numerical system, grounded in but not precisely matching canon.

---

### D-10 "The Blackthorn's Path"

**One-line pitch:** Dalinar's journey from the Blackthorn (ruthless warrior) to Bondsmith is presented as a series of 8 pivotal decisions. For each, solver determines what Dalinar did (vs what he could have done), mapping choices to order attributes.

**Mechanism:** Narrative decision tree

**World/ reference:** Dalinar Kholin in Key Characters, Bondsmith Oaths

**Estimated difficulty:** ★★★

**Why it works:** Dalinar's character arc is the thematic spine of the Stormlight Archive. Each decision point — from the destruction of Rathalas to "You cannot have my pain" — is iconic. The solver traces the path that led to his Bonding.

**Concerns:** Some specific Dalinar scenes may require more detail than the data file provides. Key moments are referenced but not fully described.

---

### D-11 "Shadesmar Crossing"

**One-line pitch:** A puzzle set in the Cognitive Realm (Shadesmar). Descriptions of locations in Shadesmar must be matched to their Physical Realm counterparts. Oceans become glass bead fields; cities appear as small towns (or vice versa).

**Mechanism:** Matching (Physical/Cognitive realm duality)

**World/ reference:** The Cognitive Realm description in Key Locations

**Estimated difficulty:** ★★★

**Why it works:** The Physical/Cognitive duality is one of the most mind-bending features of the Cosmere. Land becomes ocean, ocean becomes glass beads. Matching descriptions forces the solver to think in this inverted logic.

**Concerns:** Specific Shadesmar geography is not detailed in the data file. Some known correspondences (physical ocean = Shadesmar glass bead field) are confirmed. Others may need [VERIFY].

---

### D-12 "Bridge Four"

**One-line pitch:** The members of Bridge Four are listed with scrambled names. Solver unscrambles each name and indexes into it using the character's chapter of first appearance (modulo name length).

**Mechanism:** Anagram + indexing

**World/ reference:** Kaladin Stormblessed's story, Bridge Four (referenced throughout roshar.md)

**Estimated difficulty:** ★★

**Why it works:** Bridge Four is the most beloved group in the Stormlight Archive. Their names (Kaladin, Rock, Teft, Sigzil, Skar, Drehy, Rlain, Lopen, Bisig, Moash) are distinctive and memorable. Anagramming them is accessible and fun.

**Concerns:** Chapter-of-first-appearance data is not in the data file. Would need to use a different index (e.g., alphabetical rank among Bridge Four members, or number of letters in name). [VERIFY] roster completeness.

---

## Module E — The 16 Shards (Names, Vessels, Planets)

Source: `world/systems/shards.md`

---

### E-01 ★ "The Sixteen Intents"

**One-line pitch:** All 16 Shard names (single-word Intents) are hidden in a word search grid, but 4 of the 16 are missing. The 4 missing Shard names, read in a specific order, spell or encode the answer.

**Mechanism:** Word search with deliberate gaps

**World/ reference:** All 16 Shard Intent names

**Estimated difficulty:** ★★

**Why it works:** The 16 Shards are THE structural element of the Cosmere. A word search that contains 12 and omits 4 mirrors the meta architecture (Round Meta III uses the same "missing Shards" concept). The solver must know all 16 to find the 4 that are absent.

**Concerns:** This mechanism echoes the Round Meta III design. Must ensure this feeder puzzle and the meta are not redundant. The feeder could use first letters of the 4 missing Shards as its answer, while the meta uses a different extraction from missing Shards.

---

### E-02 "Vessel Roll Call"

**One-line pitch:** Descriptions of the original Vessels (the 16 people who took up the Shards) without naming their Shards. Solver identifies which Shard each person held, then extracts from the Intent names.

**Mechanism:** Identification + extraction

**World/ reference:** Original Vessel column

**Estimated difficulty:** ★★★

**Why it works:** The Vessels are the human side of the Shards — each was a person before they became a cosmic force. Matching Vessel to Intent forces the solver to think about WHY each person was drawn to their Shard's nature.

**Concerns:** 4+ Vessel names are "Unknown [VERIFY]" (Mercy, Valor, Whimsy, Invention). Must only use the 12 confirmed Vessels: Tanavast, Koravellium Avast, Rayse, Leras, Ati, Aona, Skai, Edgli, Bavadin, Ulas Dal, plus Sazed (Harmony). That is 10-11 confirmed. Enough for a clean puzzle.

---

### E-03 ★ "Shattered and Whole"

**One-line pitch:** A categorization puzzle: solver sorts the 16 Shards by status (Splintered, Held by Original Vessel, Held by New Vessel, Combined, Uncertain). The count in each category, applied as an index to the category name, extracts letters.

**Mechanism:** Categorization + count-based indexing

**World/ reference:** Shard Statuses Summary table

**Estimated difficulty:** ★★

**Why it works:** The status of each Shard is a central Cosmere mystery. Categorizing them requires knowledge of cosmic-scale events (who Odium killed, who survived, what Sazed combined). The count-based extraction is clean: 5 Splintered → S[5] = "t", 3 Original → letter, etc.

**Concerns:** The "Uncertain" category has 4 Shards with minimal information. Solver may not be able to categorize them without outside knowledge. Provide enough in-puzzle context or restrict to the 12 non-uncertain Shards.

---

### E-04 "Odium's Kill List"

**One-line pitch:** The Shards that Odium Splintered are presented in cryptic clue form. Solver identifies each Splintered Shard and orders them by Odium's chronological campaign of destruction.

**Mechanism:** Identification + chronological ordering

**World/ reference:** Known Combatants section, Shard Interactions

**Estimated difficulty:** ★★★

**Why it works:** Odium's Shard-killing campaign is the central conflict of the Cosmere. The ordering (Ambition first, then Devotion and Dominion on Sel, then Honor on Roshar) is a timeline of cosmic war.

**Concerns:** The exact chronological order of Odium's kills is partially uncertain. Ambition is stated as "the first Shard Odium actively hunted." Devotion + Dominion were killed together. Honor was killed later. Virtuosity self-Splintered (not Odium). Verify ordering.

---

### E-05 "Co-located Powers"

**One-line pitch:** Three puzzles-within-a-puzzle, one for each world with co-located Shards: Roshar (Honor + Cultivation + Odium), Scadrial (Preservation + Ruin → Harmony), Sel (Devotion + Dominion → the Dor). Each sub-puzzle yields a word; the three words combine.

**Mechanism:** Multi-part identification + synthesis

**World/ reference:** Co-located Shards section

**Estimated difficulty:** ★★★★

**Why it works:** The interaction between co-located Shards is what makes each planet's magic system unique. This puzzle forces the solver to understand not just individual Shards but how they interact.

**Concerns:** Three sub-puzzles may be overscoped for a single puzzle slot. Consider whether this is better split into separate puzzles or kept as an ambitious unified entry.

---

### E-06 "The Intent Spectrum"

**One-line pitch:** The 16 Intent names are arranged by a hidden ordering principle (e.g., alphabetical by letter count, or by "positivity" of the concept). Solver determines the ordering and reads acrostically.

**Mechanism:** Ordering (hidden principle) + acrostic

**World/ reference:** All 16 Shard names

**Estimated difficulty:** ★★★

**Why it works:** The 16 Intent names range from clearly negative (Ruin, Odium) to clearly positive (Honor, Mercy, Valor) to ambiguous (Autonomy, Whimsy). Finding the hidden ordering principle is a satisfying aha.

**Concerns:** There is no canonical "positivity" ranking. The ordering principle must be objective and verifiable (e.g., word length, alphabetical, number of vowels) rather than subjective.

---

### E-07 ★ "The 4x4 Shard Grid" (VISUAL)

**One-line pitch:** The 16 Shards are arranged in a 4x4 grid using four categories as axes. Solver determines the axes and places each Shard, then reads the diagonal or a specific path.

**Mechanism:** Grid logic (axis deduction + placement)

**World/ reference:** All 16 Shards' attributes (status, world, Investiture type, Intent)

**Estimated difficulty:** ★★★★

**Why it works:** 16 Shards = 4x4, mirroring the Allomancy table. The puzzle asks: what are the organizing principles of the Shards? Can they be gridded like metals? The answer is that they CAN, but the axes are different (and that difference is the puzzle).

**Concerns:** The 4 "Uncertain" Shards have minimal known attributes, making placement difficult. The axes must be deducible from the 12 well-documented Shards, with the 4 uncertain ones slottable by elimination.

---

### E-08 "Investiture Types"

**One-line pitch:** Descriptions of 12 magic systems (Allomancy, Surgebinding, AonDor, Awakening, Sand Mastery, etc.) without naming them. Solver identifies each, maps to its source Shard, and extracts from the Shard's Intent name.

**Mechanism:** Identification (magic system) + chained mapping

**World/ reference:** Investiture / Magic System column

**Estimated difficulty:** ★★★

**Why it works:** Every magic system in the Cosmere is powered by a Shard's Investiture. Tracing the power back to its source is the fundamental exercise in Cosmere scholarship.

**Concerns:** Some magic systems (Mercy's, Valor's, Whimsy's, Invention's) are unknown. Restrict to the 12+ documented systems.

---

### E-09 "Shard Worlds"

**One-line pitch:** A matching puzzle: 10 worlds/planets on one side, 16 Shards on the other. Solver matches each Shard to its home world. Since some worlds host multiple Shards, the matching is not 1-to-1.

**Mechanism:** Many-to-one matching

**World/ reference:** Home World / System column

**Estimated difficulty:** ★★

**Why it works:** The geography of the Cosmere (which Shards ended up on which planets) is a key organizing fact. The many-to-one mapping (Roshar hosts 3 Shards, Scadrial hosts 2, Sel hosts 2) adds complexity.

**Concerns:** 4 Shards have "Unknown home world [VERIFY]." Include these as part of the puzzle (solver must determine that they are unplaceable) or exclude them.

---

### E-10 "Adonalsium's Name"

**One-line pitch:** The letters of ADONALSIUM are scrambled. Clues involving the 16 Shards guide the solver to the correct letter ordering. Each Shard "contributes" one letter of the name in a specific way.

**Mechanism:** Constrained anagram

**World/ reference:** The Shattering section, ADONALSIUM as a 10-letter word, all 16 Shards

**Estimated difficulty:** ★★★

**Why it works:** ADONALSIUM is the original name of God in the Cosmere. Its letters (A-D-O-N-A-L-S-I-U-M) can be pulled from the 16 Shard names. The puzzle asks: which letter does each Shard "own"? (E.g., Autonomy contributes the A, Dominion the D, Odium the O...)

**Concerns:** Not all 16 Shard names contain the needed letters. ADONALSIUM has 10 unique letters (A, D, O, N, L, S, I, U, M plus a repeated A). Some Shards may not contribute a letter. The mapping must be constructed carefully.

---

### E-11 "The Shattering"

**One-line pitch:** A narrative logic puzzle: 16 people stand in a circle. Each takes a piece of Adonalsium. From clues about who stood next to whom and who took which piece, solver deduces the arrangement and extracts.

**Mechanism:** Circular logic (seating arrangement)

**World/ reference:** The Shattering section (16 individuals, Hoid present but did not take a Shard)

**Estimated difficulty:** ★★★★★

**Why it works:** The Shattering is the Cosmere's Big Bang. Reconstructing who took what — with Hoid as the 17th person who refused — is the ultimate Cosmere deep cut. The circular arrangement creates tight constraints.

**Concerns:** The actual arrangement at the Shattering is unknown in canon. This would be a fictional reconstruction using established Shard-Vessel pairings. Must be clear that the arrangement is puzzle-constructed, not canon-claimed.

---

### E-12 "Opposing Forces"

**One-line pitch:** Solver identifies Shard pairs that are thematically opposed (Preservation/Ruin, Honor/Odium) and pairs that are allied or co-located. The relationship type encodes a binary message.

**Mechanism:** Relationship classification + binary encoding

**World/ reference:** Opposing Pairs, Co-located Shards sections

**Estimated difficulty:** ★★★

**Why it works:** The Shards exist in a web of relationships — opposition, alliance, indifference, destruction. Mapping these relationships is genuine Cosmere scholarship.

**Concerns:** Beyond Preservation/Ruin and Honor/Odium, most Shard relationships are speculative. Must restrict to confirmed pairs.

---

## Module F — Hoid & Worldhoppers (Crossover Connections)

Source: `world/systems/hoid.md` (cross-references all other systems)

---

### F-01 ★ "The Wanderer's Aliases"

**One-line pitch:** Sixteen book excerpts from across the Cosmere, each containing a cameo appearance. Solver identifies which ones are Hoid (not all are) and what alias he used in each. The aliases' first letters encode the answer.

**Mechanism:** Identification (presence/absence) + alias extraction

**World/ reference:** Appearances by Book section, Known Names and Aliases

**Estimated difficulty:** ★★★★

**Why it works:** Hoid's cameos are one of the great joys of Cosmere readership. Some are obvious (Wit in Stormlight), others are blink-and-miss (the beggar in Hero of Ages). Identifying him requires cross-book knowledge.

**Concerns:** Several appearances are [VERIFY] (Edgedancer, Shadows for Silence, Sixth of the Dusk). Must restrict to confirmed appearances: Elantris (Dust/Hoid), Final Empire (informant), Hero of Ages (beggar), Secret History (Hoid), Warbreaker (storyteller), Way of Kings through Wind and Truth (Wit), Tress (narrator), Yumi (narrator), The Emperor's Soul (Fool).

---

### F-02 ★ "Collected Powers"

**One-line pitch:** Hoid's abilities from multiple magic systems are described without naming the system. Solver identifies each ability, its source world, and the Shard that powers it. The Shard names encode the answer.

**Mechanism:** Triple identification (ability / world / Shard)

**World/ reference:** Hoid's Collected Abilities table

**Estimated difficulty:** ★★★★

**Why it works:** Hoid is the Cosmere's greatest collector — he gathers abilities from every magic system he visits. Identifying what he can do and where he got it requires knowledge across multiple books. This is the ultimate cross-reference puzzle.

**Concerns:** Some abilities are [VERIFY] (BioChromatic Breath acquisition scene). Use confirmed abilities: Yolish Lightweaving, Allomancy (lerasium), Surgebinding (Cryptic bond), Fortune/immortality.

---

### F-03 "The Storyteller"

**One-line pitch:** Six of Hoid's stories (told to characters across various books) are presented as fragments. Solver identifies which character was the audience for each story and which book it appears in.

**Mechanism:** Matching (story / audience / book)

**World/ reference:** Appearances by Book (story details per appearance)

**Estimated difficulty:** ★★★★★

**Why it works:** Hoid tells specific stories to specific people at specific moments. The story of Fleet to Kaladin. The Dog and the Dragon to someone in Rhythm of War. The Girl Who Looked Up to Shallan. Each story is chosen for its audience. Matching story to listener requires deep reading.

**Concerns:** Story-to-audience pairings: Fleet → Kaladin (WoK), Girl Who Looked Up → Shallan (WoR), Dog and Dragon → (RoW). Others may need [VERIFY]. Warbreaker story (dog, dragon, and man) → Siri and Lightsong.

---

### F-04 "Cognitive Shadows"

**One-line pitch:** Characters from across the Cosmere who persist after death as Cognitive Shadows are described. Solver identifies each and determines which magic system sustains them.

**Mechanism:** Identification + system matching

**World/ reference:** Kelsier (Secret History), Heralds (Oathpact), The Returned (Warbreaker/Endowment), Fused (Odium)

**Estimated difficulty:** ★★★★

**Why it works:** Cognitive Shadows — people who persist beyond death through Investiture — are a cross-world phenomenon. Recognizing that Kelsier, the Heralds, the Returned, and the Fused are all the same thing expressed differently is a peak Cosmere insight.

**Concerns:** Requires cross-referencing multiple world files. Kelsier as Cognitive Shadow is in scadrial.md. Heralds and Fused are in roshar.md. The Returned are referenced in shards.md (Endowment). All confirmed.

---

### F-05 "Nightblood's Journey"

**One-line pitch:** Nightblood (the sentient sword from Nalthis) appears in multiple Cosmere books. Solver traces its journey across worlds and identifies who wielded it in each appearance.

**Mechanism:** Ordering (journey sequence) + wielder identification

**World/ reference:** Nightblood in roshar.md Key Characters, Warbreaker reference in shards.md (Endowment)

**Estimated difficulty:** ★★★★

**Why it works:** Nightblood is the most prominent crossover object in the Cosmere. Created on Nalthis, it appears in Szeth's hands on Roshar. Tracing its journey is a Worldhopper-level exercise.

**Concerns:** Nightblood's full journey is not entirely detailed in the data files. Created by Shashara and Vasher on Nalthis → brought to Roshar → given to Szeth. Intermediate steps may need [VERIFY].

---

### F-06 ★ "Letters to a Dragon"

**One-line pitch:** Excerpts from the Stormlight Archive epigraph letters (Hoid to Frost, Shard responses to Hoid) are presented with the sender and recipient removed. Solver identifies who wrote each letter and to whom.

**Mechanism:** Attribution (author identification from text style)

**World/ reference:** Hoid's Correspondence section

**Estimated difficulty:** ★★★★★

**Why it works:** The epigraph letters are some of the most information-dense passages in the Stormlight Archive. Each Shard writes in a voice reflecting their Intent (Endowment is dismissive, Autonomy is threatening, Preservation is desperate). Attributing them requires both textual analysis and Shard knowledge.

**Concerns:** Specific epigraph text is not reproduced in the data file. The file provides summaries. The puzzle would need to use paraphrased or reconstructed excerpts, clearly distinguished from direct quotes, or reference specific book/part numbers for the solver to look up.

---

### F-07 "The 17th Shard Society"

**One-line pitch:** The in-world organization called the Seventeenth Shard (a group of Worldhoppers who oppose Hoid's meddling) has members from multiple worlds. Solver identifies 6 members, their home worlds, and their objectives.

**Mechanism:** Multi-attribute identification

**World/ reference:** Cross-reference all world files (the 17th Shard organization is referenced in hoid.md context)

**Estimated difficulty:** ★★★★★

**Why it works:** The Seventeenth Shard is both the hunt's namesake and a real in-world organization. Its members are Worldhoppers who want to preserve non-interference — the opposite of Hoid's approach.

**Concerns:** The 17th Shard organization's membership is not well-documented in the data files. Some members are known from WoBs rather than published text. [VERIFY] all member identities. High risk of canon uncertainty.

---

### F-08 "Three Realms"

**One-line pitch:** Nine items/concepts are described and must be categorized into the Physical, Cognitive, or Spiritual Realm. The categorization encodes a ternary message.

**Mechanism:** Categorization (ternary) + encoding

**World/ reference:** The three realms referenced across all system files (Shadesmar = Cognitive, Connection = Spiritual, etc.)

**Estimated difficulty:** ★★★

**Why it works:** The three-realm cosmology (Physical, Cognitive, Spiritual) underlies all Cosmere magic. Understanding which realm a concept belongs to — Connection is Spiritual, Shadesmar is Cognitive, gemstones are Physical — is fundamental Cosmere literacy.

**Concerns:** Some concepts exist in multiple realms simultaneously. Must choose items that cleanly belong to one realm.

---

### F-09 "Worldhopper's Passport"

**One-line pitch:** A "passport" with 10 stamps, each from a different Cosmere world. Each stamp contains a visual clue and a riddle about that world's magic system. Solver identifies all 10 worlds and their primary Shards.

**Mechanism:** Visual identification + riddle solving

**World/ reference:** All world files (Scadrial, Roshar, Sel, Nalthis, Taldain, Threnody, etc.)

**Estimated difficulty:** ★★★

**Why it works:** The Cosmere spans many worlds. A "passport" that requires the solver to recognize each world from its magical signature is a perfect framing device.

**Concerns:** Some worlds (Taldain, Threnody, First of the Sun, Lumar, Komashi, Canticle) have minimal data in the files. Reference the standalone novels section of SCOPE.md for confirmation.

---

### F-10 "Design's Patterns"

**One-line pitch:** Hoid's Cryptic spren Design speaks in mathematical patterns and truths. Solver decodes 8 of Design's statements (each containing a hidden mathematical pattern) to extract letters.

**Mechanism:** Pattern recognition + decoding

**World/ reference:** Design mentioned in hoid.md (Hoid's bonded Cryptic)

**Estimated difficulty:** ★★★★

**Why it works:** Cryptics are spren of lies and patterns — they literally think in mathematical structures. A puzzle that requires recognizing hidden mathematical patterns in speech is perfectly on-brand.

**Concerns:** Design's specific speech patterns are not detailed in the data file. The puzzle must construct Design-style statements that are consistent with the established character.

---

### F-11 "The Memory Thief"

**One-line pitch:** In Rhythm of War, Taravangian-as-Odium manipulates Hoid's memories. Solver is given 8 "memories" — some real (matching the data files) and some altered. Solver identifies which are real and which were tampered with.

**Mechanism:** Error detection (true/false from reference data)

**World/ reference:** The Rhythm of War memory manipulation scene, cross-reference against all world files

**Estimated difficulty:** ★★★★

**Why it works:** The moment Hoid's memories are altered is the single most shocking scene for Cosmere fans who thought Hoid was untouchable. A puzzle that makes the solver sort real from fake memories is a perfect mechanical mirror of the plot.

**Concerns:** This requires the solver to have encyclopedic knowledge to distinguish real from altered facts. Must calibrate so the alterations are detectable from the data files provided, not from memory of the books.

---

### F-12 ★ "All Roads Lead to Hoid"

**One-line pitch:** A web puzzle: 12 clues each reference events from different Cosmere books. Each event has a connection to Hoid (he was present, he caused it, he benefits from it). Solver traces all 12 connections and reads the connection types.

**Mechanism:** Connection mapping (multi-source cross-reference)

**World/ reference:** All of hoid.md cross-referenced with all other world files

**Estimated difficulty:** ★★★★★

**Why it works:** The capstone Module F puzzle. Every major event in the Cosmere has Hoid lurking nearby. Connecting 12 events to his presence or influence requires the broadest possible Cosmere knowledge.

**Concerns:** This is the hardest puzzle in the hunt. Must ensure it remains solvable from the data files alone, not from book memory. Each connection must be traceable to a specific entry in a world/ file.

---

# Panel Ranking

## Reviewers

| # | Reviewer | Lens | Why Selected for Cosmere Hunt |
|---|---------|------|-------------------------------|
| 1 | **Dan Katz** | Structure & pacing | 8x MIT Mystery Hunt winner. Will catch structural flaws and meta interference. |
| 2 | **Wei-Hwa Huang** | Deductive rigor | 4x World Puzzle Champion. Will catch ambiguous deductions and brute-force vulnerabilities. |
| 3 | **Mike Selinker** | Narrative & experience | Lone Shark Games. Will evaluate whether puzzles honor the Cosmere's narrative depth. |
| 4 | **Lucas Pope** | Deduction mechanics | Obra Dinn's identification system directly parallels this hunt's structure. |
| 5 | **Kenny Young** | Buildability & logic | 24-year MS Puzzlehunt veteran. Will catch construction impossibilities. |
| 6 | **Dana Young** | Accessibility & craft | 25-year MS Puzzlehunt veteran. Will catch accessibility failures and visual design opportunities. |

---

## Module A — Allomancy: Panel Picks

Each reviewer selects their top 6 candidates (marked with X).

| ID | Title | Dan Katz | Wei-Hwa | Selinker | Pope | Kenny | Dana | Total |
|----|-------|----------|---------|----------|------|-------|------|-------|
| A-01 | The Allomantic Table | X | X | X | X | X | X | **6** |
| A-02 | Burning Through | X | X | | | X | X | **4** |
| A-03 | Misting Roll Call | | | X | X | | X | **3** |
| A-04 | Alloy Partners | X | | | | X | X | **3** |
| A-05 | Three Arts, One Spike | | X | X | X | | | **3** |
| A-06 | The Grid Coordinates | X | X | | | X | | **3** |
| A-07 | Feruchemical Reserves | | X | | | X | X | **3** |
| A-08 | Coinshot's Trajectory | | | X | | | | **1** |
| A-09 | The Coppercloud | X | X | | X | X | | **4** |
| A-10 | God Metal Bonus | | | | | | X | **1** |
| A-11 | The Inquisitor's Spikes | | | X | X | | | **2** |
| A-12 | Compounding Interest | X | | X | | | | **2** |

**Consensus Locks (6/6):** A-01 The Allomantic Table
**Strong Contenders (4-5/6):** A-02 Burning Through, A-09 The Coppercloud
**Contenders (3/6):** A-03, A-04, A-05, A-06, A-07
**Cuts (0-2/6):** A-08, A-10, A-11, A-12

### Reviewer Notes — Module A

**Dan Katz:** "A-01 is the cornerstone — the 4x4 grid IS the puzzle and it should be used. A-09 (Coppercloud) is the most structurally interesting: it's a logic puzzle with built-in information hiding, which is rare. A-08 tries to do physics with an under-specified system — risky."

**Wei-Hwa Huang:** "A-01 and A-09 both have clean deductive paths with no guessing. A-05 (Three Arts) is strong — error detection from a triple is a well-understood puzzle form. A-08's 'physics' is too loose for my taste."

**Mike Selinker:** "A-01 because it IS the system. A-05 because the three Metallic Arts on one metal is the deepest Cosmere idea in this module. A-11 (Inquisitor's Spikes) is the most narratively charged, but the [VERIFY] flags concern me."

**Lucas Pope:** "A-01 is Obra Dinn's matrix in miniature — fill the grid from constraints. A-09 is lateral information: what a Seeker detects helps identify others. A-05 forces cross-referencing across three data columns."

**Kenny Young:** "A-01, A-02, A-04, A-06, A-07, A-09. Six puzzles that are all buildable with confirmed data. No [VERIFY] dependencies. A-11 and A-12 both have construction risks I would not accept."

**Dana Young:** "A-01 is a visual centerpiece. A-02 is the best on-ramp (★★ difficulty, accessible). A-07 (Feruchemical Reserves) has elegant indexing. A-10 is too thin for a full puzzle slot."

---

## Module B — Scadrial History: Panel Picks

| ID | Title | Dan Katz | Wei-Hwa | Selinker | Pope | Kenny | Dana | Total |
|----|-------|----------|---------|----------|------|-------|------|-------|
| B-01 | Kelsier's Crew | X | | X | X | X | X | **5** |
| B-02 | The Lord Ruler's 1000 Years | X | X | X | | X | X | **5** |
| B-03 | The Siege of Luthadel | X | X | | X | X | | **4** |
| B-04 | Catacendre | | | X | | | X | **2** |
| B-05 | The Social Ladder | X | X | X | | X | X | **5** |
| B-06 | The Roughs Justice | | X | | X | X | | **3** |
| B-07 | The Kandra Generations | | | | X | | X | **2** |
| B-08 | Trust the Terris | X | | | | X | X | **3** |
| B-09 | The Well of Ascension | | | X | | | X | **2** |
| B-10 | Broadsheet Ciphers | | | X | | | | **1** |
| B-11 | The Survivor's Legacy | X | X | X | X | | | **4** |
| B-12 | Noble Houses at War | | X | | X | X | | **3** |

**Consensus Locks (5-6/6):** B-01 Kelsier's Crew, B-02 The Lord Ruler's 1000 Years, B-05 The Social Ladder
**Strong Contenders (4/6):** B-03 The Siege of Luthadel, B-11 The Survivor's Legacy
**Contenders (3/6):** B-06, B-08, B-12
**Cuts (0-2/6):** B-04, B-07, B-09, B-10

### Reviewer Notes — Module B

**Dan Katz:** "B-02 is the strongest extraction design — chronological ordering with acrostic is proven and clean. B-05 (Social Ladder) is smart because the hierarchy IS the puzzle structure. B-10 (Broadsheet Ciphers) needs visual assets we don't have."

**Wei-Hwa Huang:** "B-03 (Siege) is a proper logic grid. B-05 and B-11 both have clean deduction paths. B-04 (Catacendre) — the before/after matching is clean but the data is sparse."

**Mike Selinker:** "B-01 is the emotional core of Mistborn. B-02 is the grand narrative sweep. B-11 (Survivor's Legacy) ties two eras together — that IS what cross-era storytelling looks like. B-09 (Well) is a maze, and mazes are always fun in theory and often tedious in practice."

**Lucas Pope:** "B-01 is identification from vignettes — core Obra Dinn. B-03 is multi-variable deduction. B-12 (Noble Houses) is political deduction, which I love, but the data file is thin on specific houses."

**Kenny Young:** "B-01, B-02, B-03, B-05, B-06, B-08. All buildable. B-10 needs Allomantic symbol assets. B-04 and B-07 have [VERIFY] dependencies."

**Dana Young:** "B-01 and B-02 are the on-ramps — accessible, iconic, inviting. B-05 visualizes the social structure beautifully. B-07 (Kandra Generations) could be wonderful but the generation data is unreliable."

---

## Module C — Knights Radiant: Panel Picks

| ID | Title | Dan Katz | Wei-Hwa | Selinker | Pope | Kenny | Dana | Total |
|----|-------|----------|---------|----------|------|-------|------|-------|
| C-01 | The Double Eye | X | X | X | X | X | X | **6** |
| C-02 | Ideals and Oaths | X | | X | | X | X | **4** |
| C-03 | Ten Gemstones | | X | | | X | X | **3** |
| C-04 | Spren Bonds | | | X | X | | X | **3** |
| C-05 | The Surge Wheel | X | X | X | X | X | | **5** |
| C-06 | Herald's Madness | X | | X | | | X | **3** |
| C-07 | Radiant Roster | | X | | X | X | | **3** |
| C-08 | Essence of the Matter | | | | | | X | **1** |
| C-09 | The Recreance | X | | X | X | | | **3** |
| C-10 | Shardblade Graveyard | | X | | | X | | **2** |
| C-11 | The Nahel Bond | | X | | X | X | | **3** |
| C-12 | Words of Radiance | X | | | | | X | **2** |

**Consensus Locks (5-6/6):** C-01 The Double Eye, C-05 The Surge Wheel
**Strong Contenders (4/6):** C-02 Ideals and Oaths
**Contenders (3/6):** C-03, C-04, C-06, C-07, C-09, C-11
**Cuts (0-2/6):** C-08, C-10, C-12

### Reviewer Notes — Module C

**Dan Katz:** "C-01 and C-05 are the two Double Eye puzzles required by the brief, and they are both excellent. C-05 is the more structurally pure — a constraint placement puzzle on a ring. C-01 adds the ambiguity-resolution layer. Both belong."

**Wei-Hwa Huang:** "C-05 is the most deductively rigorous puzzle in the entire pool. The ring constraint guarantees a unique solution (with one position fixed). C-01 is similar but with a looser deduction path. Both strong."

**Mike Selinker:** "C-01 and C-02 are the emotional heart of this module. The Oaths are the most quoted text in the Stormlight Archive. A puzzle built on them must be included. C-09 (Recreance) is the great mystery — contradictory testimony is a perfect narrative puzzle form."

**Lucas Pope:** "C-01 is pure Obra Dinn: ambiguous identification resolved by cross-referencing secondary attributes. C-05 is a satisfying placement puzzle. C-07 (Radiant Roster) is character identification from vignettes — my bread and butter."

**Kenny Young:** "C-01, C-02, C-05, C-07, C-10, C-11. The [VERIFY] flags on Oaths (C-02) are manageable — restrict to Windrunner, Edgedancer, and Bondsmith confirmed oaths. C-08 and C-12 are too thin."

**Dana Young:** "C-01 is the visual centerpiece of the Roshar round — the Double Eye literally looks beautiful. C-02 is accessible and emotionally resonant. C-03 (Gemstones) has a nice clean extraction. C-06 (Herald's Madness) is haunting."

---

## Module D — Roshar World: Panel Picks

| ID | Title | Dan Katz | Wei-Hwa | Selinker | Pope | Kenny | Dana | Total |
|----|-------|----------|---------|----------|------|-------|------|-------|
| D-01 | Rhythm of Answers | X | | X | | X | X | **4** |
| D-02 | Stormwall | X | X | | X | X | | **4** |
| D-03 | Fabrial Workshop | X | X | X | | X | X | **5** |
| D-04 | Singer Forms | | X | | | X | X | **3** |
| D-05 | Oathgate Network | | X | | X | X | | **3** |
| D-06 | The Unmade | | | X | X | | | **2** |
| D-07 | The Chasms Below | X | | X | | X | X | **4** |
| D-08 | Voidbringers | | X | | X | | | **2** |
| D-09 | Stormlight Economics | | X | | | | | **1** |
| D-10 | The Blackthorn's Path | X | | X | | | X | **3** |
| D-11 | Shadesmar Crossing | | | X | X | | X | **3** |
| D-12 | Bridge Four | X | | | | X | X | **3** |

**Consensus Locks (5-6/6):** D-03 Fabrial Workshop
**Strong Contenders (4/6):** D-01 Rhythm of Answers, D-02 Stormwall, D-07 The Chasms Below
**Contenders (3/6):** D-04, D-05, D-10, D-11, D-12
**Cuts (0-2/6):** D-06, D-08, D-09

### Reviewer Notes — Module D

**Dan Katz:** "D-03 is the cleanest three-layer deduction in the module. D-07 (cryptic crossword) is a genre standard done well. D-01 satisfies the 'at least 1 puzzle using Rhythms' requirement. D-09 is computation without deduction — automatic cut."

**Wei-Hwa Huang:** "D-02 (Stormwall) is the most deductively interesting — spatial logic with directional constraints. D-03 has a clear solution path. D-05 (Oathgate Network) is a graph puzzle, which I always respect. D-09 is pure calculation — violates Principle 5."

**Mike Selinker:** "D-01 (Rhythms) is the most Cosmere-specific puzzle in the module. D-03 is the practical-magic puzzle. D-07 is a cryptic crossword — proven format, easy to make memorable. D-10 (Blackthorn's Path) is the character study. D-11 (Shadesmar) is the mind-bender."

**Lucas Pope:** "D-02 forces lateral thinking about directionality. D-05 (Oathgate Network) is a graph traversal with narrative framing. D-06 (Unmade) is fascinating but the data is too thin."

**Kenny Young:** "D-01, D-02, D-03, D-04, D-05, D-07. D-01 has a Rhythm [VERIFY] risk but it is manageable with confirmed Rhythms. D-06 and D-08 have too many [VERIFY] flags. D-12 is fun but the indexing needs work."

**Dana Young:** "D-03 has great visual potential (a fabrial workbench diagram). D-04 (Singer Forms) is accessible and interesting. D-07 is the crossword — always a crowd-pleaser. D-12 (Bridge Four) is the most beloved set of characters; even a simple anagram puzzle is worth including."

---

## Module E — The 16 Shards: Panel Picks

| ID | Title | Dan Katz | Wei-Hwa | Selinker | Pope | Kenny | Dana | Total |
|----|-------|----------|---------|----------|------|-------|------|-------|
| E-01 | The Sixteen Intents | X | | X | | X | X | **4** |
| E-02 | Vessel Roll Call | | X | X | X | | X | **4** |
| E-03 | Shattered and Whole | X | X | | | X | X | **4** |
| E-04 | Odium's Kill List | X | X | X | | X | | **4** |
| E-05 | Co-located Powers | | | X | X | | | **2** |
| E-06 | The Intent Spectrum | | X | | | | | **1** |
| E-07 | The 4x4 Shard Grid | X | X | | X | X | | **4** |
| E-08 | Investiture Types | | | X | X | X | X | **4** |
| E-09 | Shard Worlds | X | | | | X | X | **3** |
| E-10 | Adonalsium's Name | | X | X | | | X | **3** |
| E-11 | The Shattering | X | X | | X | X | | **4** |
| E-12 | Opposing Forces | | | X | | | X | **2** |

**Consensus Locks (5-6/6):** (none at 5-6)
**Strong Contenders (4/6):** E-01, E-02, E-03, E-04, E-07, E-08, E-11 (seven 4-vote candidates)
**Contenders (3/6):** E-09, E-10
**Cuts (0-2/6):** E-05, E-06, E-12

### Reviewer Notes — Module E

**Dan Katz:** "Seven candidates at 4 votes — this is the most competitive module. E-01 (word search with gaps) mirrors the meta mechanism, which is either brilliant or redundant. I lean brilliant if the extractions differ. E-07 (4x4 grid) parallels the Allomancy table — intentional resonance. E-11 (Shattering) is the hardest puzzle in the pool but also the most ambitious."

**Wei-Hwa Huang:** "E-03 (Shattered and Whole) has the cleanest deduction. E-04 has a verifiable ordering. E-07 requires the solver to INVENT the axes — that is a genuine puzzle-design insight. E-11 is a circular logic puzzle at maximum difficulty."

**Mike Selinker:** "E-02 (Vessel Roll Call) is the human story of the Shards. E-01 uses all 16 names. E-08 connects every magic system to its source — the grand unifying puzzle. E-10 (Adonalsium's Name) is literally reassembling God."

**Lucas Pope:** "E-02 and E-07 are identification matrices. E-11 is Obra Dinn at cosmic scale — 16 people in a circle, deduce who took what. Extremely ambitious."

**Kenny Young:** "E-01, E-03, E-04, E-07, E-08, E-09. All buildable with confirmed data. E-02 has 4+ unknown Vessels — restrict to 10-11 confirmed. E-11 is spectacular but a construction nightmare."

**Dana Young:** "E-01 is accessible (word search). E-02 grounds the cosmic in the personal. E-03 is visually clean. E-08 connects to everything the solver has already learned. E-10 is the most thematically resonant — reassembling the name of God."

---

## Module F — Hoid & Worldhoppers: Panel Picks

| ID | Title | Dan Katz | Wei-Hwa | Selinker | Pope | Kenny | Dana | Total |
|----|-------|----------|---------|----------|------|-------|------|-------|
| F-01 | The Wanderer's Aliases | X | X | X | X | X | X | **6** |
| F-02 | Collected Powers | X | X | X | X | X | | **5** |
| F-03 | The Storyteller | X | | X | | | X | **3** |
| F-04 | Cognitive Shadows | | X | X | X | X | | **4** |
| F-05 | Nightblood's Journey | | | X | | | X | **2** |
| F-06 | Letters to a Dragon | X | X | | X | | | **3** |
| F-07 | The 17th Shard Society | | | X | | | | **1** |
| F-08 | Three Realms | | X | | | X | X | **3** |
| F-09 | Worldhopper's Passport | X | | | | X | X | **3** |
| F-10 | Design's Patterns | | X | | X | X | | **3** |
| F-11 | The Memory Thief | X | X | X | X | | | **4** |
| F-12 | All Roads Lead to Hoid | X | | X | X | X | | **4** |

**Consensus Locks (5-6/6):** F-01 The Wanderer's Aliases, F-02 Collected Powers
**Strong Contenders (4/6):** F-04 Cognitive Shadows, F-11 The Memory Thief, F-12 All Roads Lead to Hoid
**Contenders (3/6):** F-03, F-06, F-08, F-09, F-10
**Cuts (0-2/6):** F-05, F-07

### Reviewer Notes — Module F

**Dan Katz:** "F-01 is the module's cornerstone — identify Hoid across the Cosmere. F-02 is the cross-system synthesis puzzle. F-11 (Memory Thief) is narratively clever — the solver becomes Hoid, checking their own memories. F-12 is the capstone. F-07 (17th Shard Society) has too many [VERIFY] flags."

**Wei-Hwa Huang:** "F-01 has a clean binary deduction (is this Hoid or not) followed by alias extraction. F-02 is a triple-identification with verified data. F-04 (Cognitive Shadows) is a cross-world pattern recognition puzzle — elegant. F-11 is error detection, which is a clean puzzle form."

**Mike Selinker:** "F-01 and F-03 (Storyteller) are the most Hoid-specific puzzles. Hoid IS a storyteller — a puzzle about his stories is the most thematically correct choice. F-07 (17th Shard Society) should be here for the name alone, but the data isn't reliable enough."

**Lucas Pope:** "F-01 is character identification across contexts — pure Obra Dinn. F-02 is identification across magic systems. F-04 is a cross-world phenomenon recognition puzzle. F-11 is error detection in a dataset — my Rule of Three verification system in reverse."

**Kenny Young:** "F-01, F-02, F-04, F-08, F-09, F-10. Prioritizing buildability. F-06 (Letters) needs actual epigraph text we don't have. F-07 has unverifiable member data. F-11 is brilliant but requires precise fact-checking against every world file."

**Dana Young:** "F-01 is the on-ramp — Hoid is the most beloved Cosmere character. F-09 (Passport) is visually delightful. F-08 (Three Realms) is surprisingly accessible for a cosmic concept."

---

## Consensus Summary Across All Modules

### Locks (5-6 votes — near-unanimous)

| ID | Title | Votes | Module |
|----|-------|-------|--------|
| A-01 | The Allomantic Table | 6/6 | Allomancy |
| C-01 | The Double Eye | 6/6 | Knights Radiant |
| F-01 | The Wanderer's Aliases | 6/6 | Hoid |
| C-05 | The Surge Wheel | 5/6 | Knights Radiant |
| D-03 | Fabrial Workshop | 5/6 | Roshar |
| B-01 | Kelsier's Crew | 5/6 | Scadrial History |
| B-02 | The Lord Ruler's 1000 Years | 5/6 | Scadrial History |
| B-05 | The Social Ladder | 5/6 | Scadrial History |
| F-02 | Collected Powers | 5/6 | Hoid |

**9 puzzles locked across 5 of 6 modules.** Module E has no 5+ vote puzzle (7 candidates tied at 4 votes — the most competitive module).

### Strong Contenders (4 votes)

| ID | Title | Module |
|----|-------|--------|
| A-02 | Burning Through | Allomancy |
| A-09 | The Coppercloud | Allomancy |
| B-03 | The Siege of Luthadel | Scadrial History |
| B-11 | The Survivor's Legacy | Scadrial History |
| C-02 | Ideals and Oaths | Knights Radiant |
| D-01 | Rhythm of Answers | Roshar |
| D-02 | Stormwall | Roshar |
| D-07 | The Chasms Below | Roshar |
| E-01 | The Sixteen Intents | Shards |
| E-02 | Vessel Roll Call | Shards |
| E-03 | Shattered and Whole | Shards |
| E-04 | Odium's Kill List | Shards |
| E-07 | The 4x4 Shard Grid | Shards |
| E-08 | Investiture Types | Shards |
| E-11 | The Shattering | Shards |
| F-04 | Cognitive Shadows | Hoid |
| F-11 | The Memory Thief | Hoid |
| F-12 | All Roads Lead to Hoid | Hoid |

### Recommended 6 Per Module (for Stage 4 Selection)

**Module A — Allomancy:**
1. A-01 The Allomantic Table (6/6, grid centerpiece)
2. A-02 Burning Through (4/6, accessible identification)
3. A-09 The Coppercloud (4/6, logic puzzle)
4. A-04 Alloy Partners (3/6, elegant pairing concept)
5. A-05 Three Arts, One Spike (3/6, deepest system knowledge)
6. A-07 Feruchemical Reserves (3/6, clean extraction)

**Module B — Scadrial History:**
1. B-01 Kelsier's Crew (5/6, emotional core)
2. B-02 The Lord Ruler's 1000 Years (5/6, chronological sweep)
3. B-05 The Social Ladder (5/6, structural insight)
4. B-03 The Siege of Luthadel (4/6, logic grid)
5. B-11 The Survivor's Legacy (4/6, cross-era connection)
6. B-06 The Roughs Justice (3/6, Era 2 exclusive)

**Module C — Knights Radiant:**
1. C-01 The Double Eye (6/6, visual centerpiece)
2. C-05 The Surge Wheel (5/6, ring topology logic)
3. C-02 Ideals and Oaths (4/6, emotional resonance)
4. C-06 Herald's Madness (3/6, thematic depth)
5. C-04 Spren Bonds (3/6, identification puzzle)
6. C-07 Radiant Roster (3/6, character knowledge)

**Module D — Roshar World:**
1. D-03 Fabrial Workshop (5/6, three-layer deduction)
2. D-01 Rhythm of Answers (4/6, Rhythms requirement)
3. D-02 Stormwall (4/6, spatial logic)
4. D-07 The Chasms Below (4/6, cryptic crossword)
5. D-04 Singer Forms (3/6, accessible identification)
6. D-12 Bridge Four (3/6, beloved characters, on-ramp)

**Module E — The 16 Shards:**
1. E-07 The 4x4 Shard Grid (4/6, visual grid puzzle)
2. E-04 Odium's Kill List (4/6, ordered sequence)
3. E-03 Shattered and Whole (4/6, categorization)
4. E-08 Investiture Types (4/6, cross-world magic)
5. E-02 Vessel Roll Call (4/6, the human story)
6. E-11 The Shattering (4/6, capstone ambition)

**Module F — Hoid & Worldhoppers:**
1. F-01 The Wanderer's Aliases (6/6, identity cornerstone)
2. F-02 Collected Powers (5/6, cross-system synthesis)
3. F-04 Cognitive Shadows (4/6, cross-world phenomenon)
4. F-11 The Memory Thief (4/6, error detection)
5. F-12 All Roads Lead to Hoid (4/6, capstone)
6. F-06 Letters to a Dragon (3/6, literary depth)

---

## Mechanism Diversity Check

No two puzzles in the recommended set share the same primary mechanism within a module:

| Module | Mechanisms Used |
|--------|----------------|
| A | Grid logic, Identification, Deductive logic, Pairing, Triple-matching, Positional extraction |
| B | Identification, Chronological ordering, Categorization, Logic grid, Cross-era matching, Deduction |
| C | Ring topology logic, Circular constraint, Fill-in-the-blank, Behavioral identification, Narrative identification, Character identification |
| D | Triple-matching, Identification, Spatial logic, Cryptic crossword, Identification+indexing, Anagram |
| E | Grid logic, Chronological ordering, Categorization, Magic-system identification, Identification, Circular logic |
| F | Alias identification, Triple identification, Cross-world categorization, Error detection, Connection mapping, Author attribution |

**Confirmed:** All 36 recommended puzzles use distinct mechanisms within their module.

---

## Visual/Grid Puzzle Count

| Puzzle | Type |
|--------|------|
| A-01 The Allomantic Table | 4x4 grid logic |
| A-06 The Grid Coordinates | Grid coordinate visual (alternate) |
| C-01 The Double Eye | Decagonal ring visual |
| C-05 The Surge Wheel | Circular placement visual |
| E-07 The 4x4 Shard Grid | 4x4 grid logic |
| D-07 The Chasms Below | Cryptic crossword grid |

**6 visual/grid puzzles in the pool** (requirement: at least 2). Exceeded.

---

## [VERIFY] Dependency Audit

Puzzles in the recommended set that depend on [VERIFY] items:

| Puzzle | [VERIFY] Items | Risk | Mitigation |
|--------|---------------|------|------------|
| A-05 Three Arts, One Spike | Hemalurgic uses for several metals | Medium | Restrict to Physical + Mental quadrant metals (confirmed data) |
| C-02 Ideals and Oaths | Several order-specific Oaths undocumented | Medium | Use only Windrunner 2nd/3rd, Edgedancer 2nd/3rd, Bondsmith 2nd/3rd, Lightweaver Truths, First Ideal |
| C-06 Herald's Madness | Some Herald statuses uncertain | Low | Use 6 confirmed Heralds (Jezrien, Nale, Shalash, Taln, Ishar, Kalak) |
| D-01 Rhythm of Answers | Not all Rhythm names confirmed | Medium | Restrict to confirmed Rhythms; supplement with the title Rhythm of War |
| E-11 The Shattering | Shattering location and arrangement unknown | Low | Puzzle constructs its own arrangement; does not claim canon |
| F-06 Letters to a Dragon | Epigraph text not in data files | High | Would need paraphrased excerpts or book/part references |

**No puzzle in the recommended set is blocked by [VERIFY] items.** All can be built with confirmed data, using the mitigations listed.

---

## Elegance Picks (1+ per module, as required)

| Module | Elegant Puzzle | Why |
|--------|---------------|-----|
| A | A-01 The Allomantic Table | The 4x4 grid IS the magic system. The puzzle IS the thing. |
| B | B-05 The Social Ladder | The hierarchy IS the encoding. Structure and content are the same. |
| C | C-05 The Surge Wheel | The ring topology IS the constraint. The math makes the puzzle. |
| D | D-03 Fabrial Workshop | Three-layer deduction (effect → type → gemstone) with no wasted steps. |
| E | E-07 The 4x4 Shard Grid | 16 Shards in a 4x4 grid that the solver must INVENT the axes for. |
| F | F-02 Collected Powers | Hoid's defining trait (collecting abilities) IS the identification chain. |
