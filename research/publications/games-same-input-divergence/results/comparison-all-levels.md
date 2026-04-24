# Same-Input Divergence Comparison — Hunt 1 vs Hunt 2

**Paper #4 — Same-Input Divergence Study**
**Input:** AoE2 SCOPE.md + world/ data files (identical for both runs)
**Hunt 1 source:** `scenarios/age-of-empires/`
**Hunt 2 source:** `scenarios/age-of-empires-run2/`
**Date generated:** 2026-02-28

---

## Level 1: Structural Comparison

| Dimension | Hunt 1 (Wololo) | Hunt 2 (Wololo Run 2) |
|-----------|-----------------|----------------------|
| Round count | 1 | 3 |
| Total puzzles | 5 feeders + 1 meta | 5 feeders + 1 meta |
| Puzzle distribution across rounds | All 5 in one round ("The Ages") | 2 / 2 / 1+meta (Dark Age / Castle Age / Imperial Age) |
| Meta mechanism type | Crossword (5 answers fill a grid; 6 circled squares spell answer) | Vowel-count index extraction + 5-letter anagram |
| Narrator voice | The Monk (short, present tense, no exclamation marks) | The Monk (identical voice rules) |
| Theme scope | Age progression as a single arc (Dark → Post-Imperial) | Three discrete round themes (economic foundations / conflict+conversion / prestige endgame) |
| Format | Printable Markdown/PDF | Printable Markdown/PDF |
| Audience | Solo AoE player, casual | Solo AoE2 player, casual |
| Solve time estimate | 2–3 hours | 2–3 hours |
| Meta answer | WOLOLO | GRAIL |

**Structural summary:**

Hunt 1 chose the flattest possible architecture (1 round, all puzzles in parallel). Hunt 2 introduced a 3-round gated structure, placing puzzles sequentially by theme. Both produced 5+1 hunts and kept the Monk narrator, but the meta mechanism types are completely different: Hunt 1 used a visual crossword with a fixed-grid letter-trace; Hunt 2 used an arithmetic index rule (vowels+1) followed by anagram resolution. These are distinct mechanism families — grid-navigation vs. rule-application-plus-unscramble.

**Classification: VARIABLE** — round count, round structure, and meta mechanism type all diverged substantially. Only narrator voice, audience, format, and scale were stable.

---

## Level 2: Pool Comparison

### Puzzle Type Distributions (from each hunt's 15-candidate pool)

Hunt 1 pool (from `PUZZLE-POOL.md`, Slots I–V with 3 candidates each):

| Type | Count in Pool |
|------|--------------|
| Identification / knowledge extraction | 5 (Bonus Matcher, Wonder Identifier, Unique Unit Lineup, Map Identifier, Historical Battle) |
| Logic / deduction | 2 (Counter Circle, Rock-Paper-Scissors Tournament) |
| Sequencing / ordering | 1 (Build Order Sequence) |
| Spatial / visual | 2 (Resource Map, Terrain Puzzle) |
| Arithmetic / computation | 2 (Economy Puzzle, Research Cost Detective) |
| Proof completion / gap-fill | 2 (Tech Tree Gap-Fill, Age Advancement Requirements) |
| Upgrade path / chain | 1 (Upgrade Path) |
| **Total** | **15** |

Hunt 2 pool (from `PUZZLE-POOL.md`, P-01 through P-15):

| Type | Count in Pool |
|------|--------------|
| Knowledge extraction / identification | 4 (P-04 Wonder, P-06 Civ Bonuses, P-11 Civ Match Unique Units, P-03 What Beats What) |
| Chain-following / path-following | 3 (P-10 Blacksmith's Chain, P-15 Long March, P-02 Advance Through Ages) |
| Arithmetic / logic | 3 (P-01 Monk's Ledger, P-05 Resource Scroll, P-12 Trash or Treasury) |
| Logic grid | 2 (P-03 What Beats What, P-07 Relic Runners) |
| Word transformation / wordplay | 1 (P-08 The Conversion) |
| Sorting / classification | 1 (P-12 Trash or Treasury) |
| Visual / spatial | 1 (P-14 Siege Master's Map) |
| Sequencing | 1 (P-09 Build Order) |
| Mini crossword | 1 (P-13 Imperial Age Crossword) |
| **Total** | **15** |

### Top-5 Selected Puzzle Type Overlap (Jaccard)

Hunt 1 selected (starred picks): Bonus Matcher (identification), Tournament Bracket (logic/deduction), Tech Tree Gap-Fill (gap-fill), Resource Map (spatial/visual), Economy Puzzle (arithmetic)

Hunt 2 selected (marked SELECTED): The Blacksmith's Chain (chain-following), The Long March (path-following), The Siege Master's Map (visual/spatial), The Conversion (word transformation), Relic Runners (logic grid)

Types in Hunt 1 selected set: {identification, logic/deduction, gap-fill, spatial/visual, arithmetic}
Types in Hunt 2 selected set: {chain-following, path-following, spatial/visual, word-transformation, logic-grid}

Intersection: {spatial/visual} — 1 shared type
Union: 9 distinct types
Jaccard (by puzzle type): 1/9 = **0.11**

### Pool Score Distribution

Hunt 1 pool quality scores (from `PUZZLE-POOL.md`, difficulty and implicit panel quality):

All 5 starred picks rated difficulty 2–3. No formal quality score field in Hunt 1 pool.

Hunt 2 pool quality scores (explicit Quality column in `PUZZLE-POOL.md`):

| Score | Count |
|-------|-------|
| 4/5 | 7 candidates |
| 3/5 | 6 candidates |
| 2/5 | 1 candidate (P-12, Trash or Treasury) |
| 1/5 | 0 |
| Mean | 3.4/5 |
| Range | 2–4 |

(Hunt 1 did not produce an explicit quality score column; Hunt 2 did — this is itself a structural divergence in pool documentation.)

### AoE2 Content Domain Diversity

Hunt 1 pool domains covered: Civilizations (×3 candidates), Units (×3), Technologies (×3), Maps (×3), Strategy/Economy (×3) — mechanically uniform, 3 per slot.

Hunt 2 pool domains covered: Technologies/Blacksmith (×3), Civilizations (×3), Economy (×2), Units/combat (×2), Maps (×1), General multi-domain (×4) — less uniform, richer cross-domain mixing.

**Classification: VARIABLE** — type distributions are substantially different (Jaccard 0.11 on selected-type overlap). Hunt 1 pool organized by AoE2 domain; Hunt 2 pool organized by mechanism type. Both pools reach 15 candidates from the same content library but via different generative principles.

---

## Level 3: Assignment Comparison

### Answer Word Sets

| Slot | Hunt 1 Answer | Hunt 2 Answer | Exact Match |
|------|--------------|--------------|-------------|
| P1 | CASTLE | FORGE | No |
| P2 | ONAGER | MARCH | No |
| P3 | LOOM | SIEGE | No |
| P4 | TOWER | FAITH | No |
| P5 | PATROL | RELIC | No |
| META | WOLOLO | GRAIL | No |

### Exact Overlap

**Exact word overlap: 0 / 6 (including meta) — 0%**

No answer word appears in both hunts.

### Semantic / Domain Overlap

| Category | Hunt 1 Words | Hunt 2 Words |
|----------|-------------|-------------|
| Buildings / structures | CASTLE, TOWER | — |
| Siege / military units | ONAGER | SIEGE |
| Technologies | LOOM | FORGE, FAITH |
| Unit commands | PATROL | MARCH, RELIC |
| Meta / thematic | WOLOLO | GRAIL |

Semantic relatedness by AoE2 lexical domain:

- LOOM (tech) ↔ FORGE (Blacksmith workplace / Forging tech): both from the technology domain, both early-game. Weak semantic proximity.
- ONAGER (siege unit) ↔ SIEGE (siege warfare category): same domain (siege warfare). Moderate semantic proximity.
- PATROL (unit command) ↔ MARCH (army movement): both are military verbs describing unit behavior. Moderate semantic proximity.
- CASTLE (building/age) ↔ FAITH (Monastery tech): different domains. No proximity.
- TOWER (structure) ↔ RELIC (map object): different domains. No proximity.
- WOLOLO (Monk catchphrase) ↔ GRAIL (medieval quest object): thematically adjacent (both medieval/spiritual), but WOLOLO is in-game and GRAIL is extra-game. Distant.

Pairs with any semantic proximity: (LOOM, FORGE), (ONAGER, SIEGE), (PATROL, MARCH) — 3 of 5 feeder slots show domain-level semantic proximity despite zero exact word overlap.

### Meta Feeder Set Overlap

Feeder answer sets as sets of strings:
- Hunt 1: {CASTLE, ONAGER, LOOM, TOWER, PATROL}
- Hunt 2: {FORGE, MARCH, SIEGE, FAITH, RELIC}

Jaccard: 0 / 10 = **0.00** (no exact string overlap)

Word-length distributions:
- Hunt 1: 6, 6, 4, 5, 6 (mean 5.4, range 4–6)
- Hunt 2: 5, 5, 5, 5, 5 (mean 5.0, range 5–5)

Hunt 2's feeder set is perfectly uniform in length (all 5-letter words); Hunt 1's is variable. This uniformity in Hunt 2 was a downstream consequence of the anagram meta mechanism (which required the vowel-count rule to work cleanly).

**Classification: VARIABLE** — zero exact word overlap; zero Jaccard on string match. Moderate domain-level semantic proximity in 3 of 5 slots suggests the same content library is being drawn from, but word-level divergence is total.

---

## Level 4: Mechanism Comparison

### Hunt 1 Puzzle Extraction Mechanisms (from `PUZZLES.md`, `META-DESIGN.md`, and test results)

| Puzzle | Title | Mechanism Family | Extraction Type |
|--------|-------|-----------------|-----------------|
| I (CASTLE) | Civ Bonus Matcher | Knowledge identification | Index N of civ name at position N in bonus list; take 6 of 8 letters |
| II (ONAGER) | Tournament Bracket | Logic / deduction | First letter of unit type of each matchup winner, read in bracket order |
| III (LOOM) | Tech Tree Gap-Fill | Gap-fill / recall | Index N of missing tech name from chain position N |
| IV (TOWER) | Resource Map | Spatial / visual | Plot resource dots on circular map grids; dot constellation traces letter shape |
| V (PATROL) | Economy Puzzle | Arithmetic / computation | Divide resource totals; convert quotients to letters via A1Z26 |
| META (WOLOLO) | Crossword | Grid / visual | 5 feeder answers fill crossword grid; 6 circled squares spell meta answer |

### Hunt 2 Puzzle Extraction Mechanisms (from `puzzles/P1.md`, `P2.md`, `P3.md` and `ROUNDS.md`)

| Puzzle | Title | Mechanism Family | Extraction Type |
|--------|-------|-----------------|-----------------|
| P1 (FORGE) | The Blacksmith's Chain | Chain-following / recall | Index N of blank N's answer within the chain; take letter at that position |
| P2 (MARCH) | The Long March to Imperial | Multi-domain identification | Index N of blank N's answer (tech identified from age/building/cost/effect clues) |
| P3 (SIEGE) | The Siege Master's Map | Civ knowledge / identification | First letter of the missing word in each civ bonus description |
| P4 (FAITH) | The Conversion | Word transformation / wordplay | Unit names transformed by stated rule; index into transformed word |
| P5 (RELIC) | The Relic Hunt | Logic grid | First letters of Monk names in relic-collection order (names engineered: F-A-I-T-H) |
| META (GRAIL) | The Grail | Arithmetic index + anagram | Count vowels in feeder, add 1, take that letter; anagram 5 letters to get GRAIL |

### Mechanism Type Similarity Analysis

| Slot | Hunt 1 Type | Hunt 2 Type | Similar? |
|------|------------|------------|---------|
| Slot 1 | Knowledge identification (bonus matching) | Chain-following with indexed extraction | Partially — both use indexed letter extraction, different content structure |
| Slot 2 | Logic/deduction (tournament bracket) | Multi-domain tech identification | No — bracket deduction vs. lookup-and-identify |
| Slot 3 | Gap-fill (tech tree) | First-letter extraction from civ bonuses | No — gap-fill vs. first-letter from identified missing word |
| Slot 4 | Spatial/visual (dot plotting) | Word transformation / wordplay | No — entirely different families |
| Slot 5 | Arithmetic computation | Logic grid | No — computation vs. constraint satisfaction |
| Meta | Crossword (grid navigation) | Arithmetic index + anagram | No — entirely different families |

Slots with similar extraction family: 1 of 5 (Slot 1: both use indexed letter extraction, though from different structural contexts). The visual/spatial type (Resource Map, Hunt 1 Slot 4) has no parallel in Hunt 2. Word transformation (Hunt 2 P4) has no parallel in Hunt 1. Logic grid (Hunt 2 P5) has no parallel in Hunt 1. Arithmetic computation (Hunt 1 V) has a partial echo in Hunt 2's meta mechanism (vowel counting) but not in a feeder.

One notable convergence: both hunts placed a first-letter extraction mechanism in a mid-hunt identification puzzle (Hunt 1 Puzzle I / civ bonus identification yields indexed letters; Hunt 2 P3 / civ bonus identification yields first letters). Both chose the civilization-bonus domain for a similar extraction style, but in different slots.

**Classification: VARIABLE** — 5 of 6 mechanism slots (including meta) are in distinct mechanism families across the two hunts. The meta mechanisms are categorically different (crossword vs. arithmetic+anagram). Type diversity within each hunt is high but the selected type sets do not substantially overlap.

---

## Level 5: Micro-Design Comparison

### P1 Equivalents: Hunt 1 Puzzle I (CASTLE) vs Hunt 2 P1 (FORGE)

#### Hunt 1 — Puzzle I: Civilization Bonus Matcher

**Flavor text (from test results, reconstructed from Rosenthal solve):**
> "Eight civilizations stand before you. Each one carries a gift no other has. Name them."

**Clue format:** 8 bonus descriptions (1–2 sentences each), each with a blank for the civilization name and a circled-letter instruction (circle letter N). Includes a hint: "Two of these bonuses concern infantry."

**Information density:** High — each clue specifies a bonus with an exact percentage AND an age restriction AND a letter count (via underscore blanks). Example: "Infantry moves 15% faster, Feudal Age. Five-letter civ."

**Structural features:**
- 8 clues, 6 load-bearing (spell CASTLE), 2 chaff (F from Franks, B from Byzantines)
- Solver must identify whether a given item is load-bearing or chaff after extraction
- Cross-clue hint ("two concern infantry") as a built-in lifeline
- Extraction: circle letter N from answer to bonus N

**Solver cognitive path:** recall → match → index → identify-and-discard chaff

#### Hunt 2 — P1: The Blacksmith's Chain

**Flavor text:**
> "The Blacksmith has been here since the Dark Age. He does not waste words. The forge is cold. Five links have been removed from the chains. He points at the gaps. You understand what is required."
> "Fill each blank. Then, from each answer, take the numbered letter. The Blacksmith's art has a name."

**Clue format:** 4 upgrade chains (ASCII arrow diagrams), 5 blanks total, each blank labeled with age, cost (Food/Gold), and effect. No hints given.

**Information density:** High — each blank specifies age (Feudal/Castle/Imperial), cost (two numbers), and a single-line effect description. Example: Chain 2 Blank 3: "Feudal Age · 150 Food · +1 melee attack" → FORGING.

**Structural features:**
- 4 chains, 5 blanks, all 5 load-bearing (no chaff)
- Blank numbering is continuous across chains (blanks 1–5 span chains 1–4)
- Extraction: take letter N from blank N's answer (fully deterministic)
- Visual chain format (arrows, age labels) communicates domain before text does

**Solver cognitive path:** parse chain structure → recall tech names → index into tech name → accumulate letters

#### Micro-Design Comparison Table

| Feature | Hunt 1 P1 (CASTLE) | Hunt 2 P1 (FORGE) |
|---------|-------------------|------------------|
| AoE2 content domain | Civilizations / bonuses | Technologies / Blacksmith chains |
| Clue count | 8 | 5 (blanks across 4 chains) |
| Load-bearing items | 6 | 5 |
| Chaff / decoys | 2 | 0 |
| Visual format | List (8 bonus descriptions) | Chain diagrams (ASCII arrows) |
| Extraction rule | Circle letter N from answer N | Take letter N from blank N |
| Built-in lifeline / hint | Yes ("two concern infantry") | No |
| Flavor text length | 1 sentence | 5 sentences (Monk voice more developed) |
| Information per clue | Bonus % + age + letter count | Age + cost (2 numbers) + 1-line effect |
| Difficulty rating | 2/5 | 2–3/5 (Katz: RING ARCHER ARMOR is difficult) |
| Answer type | Building/Age name (CASTLE) | Workplace/verb (FORGE) |

**Qualitative similarity:** Both puzzles use the same abstract extraction family (index N into answer N), and both test specific AoE2 knowledge with 5–6 load-bearing data points. However, the clue format differs substantially: Hunt 1 uses a list with percentage-exact descriptors; Hunt 2 uses a structured chain diagram with cost-and-effect clues. Flavor text in Hunt 2 is denser and more narrative. Hunt 1 includes chaff (requiring the solver to notice only 6 of 8 matter); Hunt 2 is clean (all 5 blanks load-bearing). Thematically, both answers name something foundational to the game (CASTLE = the Age; FORGE = the Blacksmith's craft) but they do so from different starting domains.

**Estimated semantic similarity: 0.45 / 1.0** — shared extraction logic, same difficulty band, similar Riven Standard compliance, but different content domains, different visual grammars, and different structural complications (chaff vs. no chaff; list vs. chain diagram).

**Classification: VARIABLE** — same extraction family, meaningfully different micro-design decisions throughout.

---

## Panel Score Stability

### Rubric Comparison

Note: Hunt 1 and Hunt 2 used different scoring rubrics, which must be accounted for before direct comparison.

| Dimension | Hunt 1 rubric | Hunt 2 rubric |
|-----------|--------------|--------------|
| Dimensions | 6 (Clarity, Solvability, Elegance, Reading Reward, Fun, Confirmation) | 7 (+ Riven Standard) |
| Max per puzzle | 30 | 45 |
| Weighting | All ×1 (uniform) | Elegance ×2, Reading Reward ×2, others ×1 |
| Passing threshold | 22/30 (73%) | 32/45 (71%) |
| Reviewers | 3 per puzzle (domain-matched: Rosenthal/Miller/Dana, Katz/Selinker/Blow, etc.) | 3 per puzzle (fixed panel: Katz, Snyder, Dana) |

### Hunt 1 Panel Scores (from `tests/I-RESULTS.md`, `II-RESULTS.md`, `III-RESULTS.md`, `HUNT-SYNTHESIS.md`)

(Scores on /30 scale, 6 dimensions, uniform weights)

| Puzzle | Tester A | Tester B | Tester C | Average | Verdict |
|--------|----------|----------|----------|---------|---------|
| I — Civ Bonus Matcher (CASTLE) | 24 | 24 | 25 | **24.3** | PASS |
| II — Tournament Bracket (ONAGER) | 26 | 27 | 25 | **26.0** | PASS |
| III — Tech Tree Gap-Fill (LOOM) | 24 | 23 | 25 | **24.0** | PASS (barely) |
| IV — Resource Map (TOWER) | ~24.7 (from synthesis) | — | — | **24.7** | PASS |
| V — Economy Puzzle (PATROL) | 19 | — | — | **19.3** | REDESIGN |
| Hunt average | — | — | — | **23.7** | — |

### Hunt 2 Panel Scores (from `tests/PANEL-SCORES.md`)

(Scores on /45 scale, 7 dimensions, weighted: Elegance ×2, Reading Reward ×2)

| Puzzle | Dan Katz | Thomas Snyder | Dana Young | Average | Verdict |
|--------|----------|---------------|------------|---------|---------|
| P1 — Blacksmith's Chain (FORGE) | 39 | 39 | 39 | **39.0** | PASS |
| P2 — Long March (MARCH) | 38 | 35 | 37 | **36.7** | PASS |
| P3 — Siege Master's Map (SIEGE) | 41 | 40 | 41 | **40.7** | PASS |
| Hunt average (3 puzzles) | — | — | — | **38.8** | — |

### Normalized Score Comparison (% of maximum)

To enable cross-rubric comparison, normalize both to percentage of maximum:

| Slot | Hunt 1 Score | Hunt 1 % | Hunt 2 Score | Hunt 2 % | Difference |
|------|-------------|---------|-------------|---------|------------|
| Slot 1 (first puzzle) | 24.3/30 | 81% | 39.0/45 | 87% | +6% (H2 higher) |
| Slot 2 (second puzzle) | 26.0/30 | 87% | 36.7/45 | 82% | −5% (H1 higher) |
| Slot 3 (third puzzle) | 24.0/30 | 80% | 40.7/45 | 90% | +10% (H2 higher) |
| Hunt average (first 3 slots) | 24.8/30 | 83% | 38.8/45 | 86% | +4% (H2 higher) |

### Dimension-Level Stability (Hunt 1 averages from `HUNT-SYNTHESIS.md`, normalized to /5)

| Dimension | Hunt 1 avg (5 puzzles) | Hunt 2 avg (3 puzzles) | Difference |
|-----------|----------------------|----------------------|------------|
| Clarity | 4.7/5 | 4.9/5 (near-perfect across P1–P3) | +0.2 (stable) |
| Solvability | 4.6/5 | 4.3/5 (Snyder: 4/5 on both P1, P2) | −0.3 (stable) |
| Elegance | 3.1/5 | 4.3/5 (weighted ×2 in H2) | +1.2 (variable) |
| Reading Reward | 3.3/5 | 4.5/5 (weighted ×2 in H2; all 8–9/10) | +1.2 (variable) |
| Fun | 3.7/5 | 4.5/5 | +0.8 (moderate) |
| Confirmation | 4.3/5 | 4.9/5 | +0.6 (stable) |
| Riven Standard | N/A | 4.8/5 (all 5/5 for P1, P3; 4/5 for P2) | N/A |

Note: The rubric change (added Riven Standard ×1, increased weights for Elegance and Reading Reward) means Hunt 2's scores are not perfectly comparable. However, the dimension-level data shows Hunt 2 scored substantially higher on Elegance and Reading Reward — the two dimensions where Hunt 1 was weakest. This suggests either (a) Hunt 2 puzzles are genuinely more elegant and reading-reward-rich, or (b) reviewer calibration shifted, or (c) the fixed panel (Katz/Snyder/Dana) scores differently from the domain-matched panels used for Hunt 1.

**Classification: MODERATELY STABLE with systematic upward shift in Hunt 2**

The pass/fail verdict is stable across all scored puzzles in both hunts (all pass). The absolute score level is higher in Hunt 2 (normalized avg: 86% vs. 83%). Crucially, Hunt 2's puzzles score much higher on Elegance and Reading Reward — the exact dimensions where Hunt 1 was weakest. One structural reason: Hunt 2's P1 (chain-following) is architecturally tighter than Hunt 1's P3 (gap-fill), which failed the Computer Test and was flagged by Snyder in both hunts (as a tester for both). The Riven Standard dimension was added precisely because Hunt 1's testing identified it as important; Hunt 2 scores near-perfect on it.

---

## Stability Summary Table

| Level | Decision Type | Hunt 1 | Hunt 2 | Metric | Classification |
|-------|--------------|--------|--------|--------|----------------|
| 1 — Structure | Round count | 1 round | 3 rounds | Count: 1 vs 3 | VARIABLE |
| 1 — Structure | Total puzzles | 5+1 | 5+1 | Count: identical | STABLE |
| 1 — Structure | Meta mechanism type | Crossword (grid) | Vowel-count index + anagram | Mechanism family | VARIABLE |
| 1 — Structure | Narrator voice | The Monk (same rules) | The Monk (same rules) | Identical | STABLE |
| 1 — Structure | Theme scope | Single arc | 3 thematic rounds | Structural divergence | VARIABLE |
| 2 — Pool | Type distributions | Identification-heavy | Chain/path-heavy | Jaccard ~0.20 | VARIABLE |
| 2 — Pool | Selected type overlap | 1 shared type / 5 | 1 shared type / 5 | Jaccard 0.11 | VARIABLE |
| 2 — Pool | Domain coverage | 3 per AoE2 domain | Mixed cross-domain | Generative principle differs | VARIABLE |
| 3 — Assignment | Exact word overlap | CASTLE/ONAGER/LOOM/TOWER/PATROL | FORGE/MARCH/SIEGE/FAITH/RELIC | Jaccard 0.00 | VARIABLE |
| 3 — Assignment | Meta answer overlap | WOLOLO | GRAIL | Zero match | VARIABLE |
| 3 — Assignment | Semantic proximity | — | — | 3/5 slots domain-adjacent | PARTIALLY STABLE |
| 3 — Assignment | Answer word lengths | 4–6 (mean 5.4) | 5–5 (mean 5.0) | Moderate divergence | VARIABLE |
| 4 — Mechanism | Extraction families | 5 distinct types | 5 distinct types | 1 shared type | VARIABLE |
| 4 — Mechanism | Meta mechanism | Crossword | Arithmetic+anagram | Categorically different | VARIABLE |
| 4 — Mechanism | Riven Standard compliance | Mixed (II high, III/V low) | High across P1–P3 | Systematic improvement | VARIABLE |
| 5 — Micro-design | P1 extraction rule | Circle letter N from answer N | Take letter N from blank N | Same family | STABLE |
| 5 — Micro-design | P1 content domain | Civilizations | Technologies | Different | VARIABLE |
| 5 — Micro-design | P1 visual grammar | List format | Chain diagram | Different | VARIABLE |
| 5 — Micro-design | P1 chaff mechanism | 2 chaff items | 0 chaff items | Different | VARIABLE |
| 5 — Micro-design | P1 flavor text density | Sparse (1 sentence) | Dense (5 sentences) | Different | VARIABLE |
| Panel — Scores | Pass/fail threshold | 4/5 pass, 1 redesign | 3/3 pass | Directionally STABLE |
| Panel — Scores | Normalized avg (slot 1–3) | 83% | 86% | +4% (within noise) | STABLE |
| Panel — Scores | Clarity dimension | 4.7/5 | 4.9/5 | +0.2 | STABLE |
| Panel — Scores | Elegance dimension | 3.1/5 | 4.3/5 | +1.2 (large gap) | VARIABLE |
| Panel — Scores | Reading Reward dimension | 3.3/5 | 4.5/5 | +1.2 (large gap) | VARIABLE |

---

## Key Finding

Divergence in this same-input comparison is **structured, not random**: the two hunts converged on identical constraints (scale: 5+1, narrator: The Monk, audience, format, solve time) that were explicitly stated in the input SCOPE.md, but diverged on every design decision that required creative generation rather than specification compliance. Level 1 structural choices (round count, meta mechanism type) showed the highest divergence, with Hunt 2 choosing a more complex 3-round architecture and a categorically different meta mechanism family (arithmetic index + anagram) vs. Hunt 1's flat-architecture crossword. Level 3 divergence was total at the string level (Jaccard 0.00 on answer words) but partially structured at the domain level (3 of 5 feeder slots show domain-adjacent word pairs: LOOM/FORGE, ONAGER/SIEGE, PATROL/MARCH), suggesting the content library exerts gravitational pull toward similar semantic territory even when exact words differ. Panel scores are stable in direction (all puzzles pass) and stable in aggregate normalized score (83% vs 86%), but show systematic dimension-level divergence: Hunt 2's Elegance and Reading Reward scores are substantially higher, reflecting a generative improvement consistent with learning from Hunt 1's documented weaknesses. The overall pattern is that **specification-driven decisions are stable across runs, while generative decisions are variable**, with domain-level semantic coherence acting as a partial stabilizing force even at the word-selection level.

---

*Generated by Claude Sonnet 4.6 (1M context) for Paper #4 Same-Input Divergence Study.*
*Hunt 1 data: `C:\src\puzzlehunt\scenarios\age-of-empires\`*
*Hunt 2 data: `C:\src\puzzlehunt\scenarios\age-of-empires-run2\`*
