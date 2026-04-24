# Classification Master Table — All 25 Puzzles
**Compiled from:** Batch 1 (MIT Mystery Hunt 2023), Batch 2 (MIT Mystery Hunt 2023), Batch 3 (Huntinality III 2023 + Teammate Hunt 2021)
**Panel:** Dan Katz, Thomas Snyder, Dana Young (simulated)
**Compiled:** 2026-02-28

---

## Master Table

Sorted by Hunt Level (MIT/Elite first, then Standard Hunt, then Accessible Hunt), then alphabetically within each tier.

| # | Puzzle | Source | Hunt Level | Mechanism | Domain Load | Aha Clarity | Paper Suitability | Quality Signal | Ablation |
|---|--------|--------|-----------|-----------|-------------|-------------|------------------|---------------|----------|
| 1 | Apples Plus Bananas | MIT Mystery Hunt 2023 | MIT/Elite | Logic-Constraint + Knowledge (Hybrid) | 3 | 2 | Yes | 3 | Yes |
| 2 | Bridge Building | MIT Mystery Hunt 2023 | MIT/Elite | Logic-Constraint + Knowledge (Hybrid) | 3 | 3 | Yes | 3 | Yes |
| 3 | Information Relay | Huntinality III 2023 | MIT/Elite | Hybrid (Transform + Wordplay + Meta) | 2 | 3 | Yes | 3 | Yes |
| 4 | Much Ado About Nothing | MIT Mystery Hunt 2023 | MIT/Elite | Logic/Constraint | 3 | 1 | Partial | 2 | Conditional |
| 5 | Museum Rules | MIT Mystery Hunt 2023 | MIT/Elite | Knowledge extraction | 3 | 2 | No | 2 | Conditional |
| 6 | Natural Transformation | MIT Mystery Hunt 2023 | MIT/Elite | Logic-Constraint + Transformation (Hybrid) | 2 | 2 | Yes | 3 | Yes |
| 7 | Par Re-sing | Teammate Hunt 2021 | MIT/Elite | Hybrid (Wordplay + Transformation) | 2 | 2 | Yes | 3 | Yes |
| 8 | People Watching | MIT Mystery Hunt 2023 | MIT/Elite | Knowledge extraction | 3 | 2 | Yes | 3 | Yes |
| 9 | Scicabulary | MIT Mystery Hunt 2023 | MIT/Elite | Wordplay | 1 | 3 | Yes | 3 | Yes |
| 10 | Single Elimination | Teammate Hunt 2021 | MIT/Elite | Interactive-JS | 1 | 1 | No | 3 | No |
| 11 | Street Smarts | MIT Mystery Hunt 2023 | MIT/Elite | Physical-Location + Knowledge | 2 | 2 | No | 3 | No |
| 12 | A Trip to the Museum | MIT Mystery Hunt 2023 | MIT/Elite | Physical-Location + Logic + Transformation | 1 | 1 | Partial | 3 | No |
| 13 | World's Smallest Logic Puzzles | MIT Mystery Hunt 2023 | MIT/Elite | Logic/Constraint | 3 | 2 | Partial | 3 | Yes |
| 14 | You're Telling Me | MIT Mystery Hunt 2023 | MIT/Elite | Wordplay + Transformation | 1 | 3 | Yes | 3 | Yes |
| 15 | Brain Freeze | MIT Mystery Hunt 2023 | Standard Hunt | Hybrid | 2 | 2 | Partial | 2 | Yes |
| 16 | Characters | Huntinality III 2023 | Standard Hunt | Hybrid (Knowledge-extraction + Transform) | 2 | 3 | Yes | 3 | Yes |
| 17 | Dropypasta | MIT Mystery Hunt 2023 | Standard Hunt | Knowledge + Transformation (Hybrid) | 3 | 2 | Yes | 2 | Yes |
| 18 | Extrasensory | MIT Mystery Hunt 2023 | Standard Hunt | Wordplay + Knowledge | 2 | 2 | Partial | 2 | Conditional |
| 19 | Front and Center | Huntinality III 2023 | Standard Hunt | Wordplay | 1 | 3 | Yes | 3 | Yes |
| 20 | H2No | MIT Mystery Hunt 2023 | Standard Hunt | Knowledge extraction | 3 | 2 | Yes | 2 | Yes |
| 21 | Itinerary | Teammate Hunt 2021 | Standard Hunt | Knowledge-extraction | 3 | 2 | Yes | 2 | Conditional |
| 22 | One of the Puzzles of All Time | MIT Mystery Hunt 2023 | Standard Hunt | Hybrid | 2 | 2 | Yes | 2 | Yes |
| 23 | Tales from a Dating Show | Huntinality III 2023 | Standard Hunt | Hybrid (KE + JS + Transform) | 3 | 2 | Partial | 2 | Conditional |
| 24 | What's Next? | Huntinality III 2023 | Standard Hunt | Hybrid (Knowledge-extraction + Transform) | 3 | 2 | Partial | 3 | Yes |
| 25 | The Bakery | Huntinality III 2023 | Accessible Hunt | Knowledge-extraction | 3 | 2 | Yes | 2 | Conditional |

---

## Dimension Distributions

### Hunt Level
- MIT/Elite: 14 puzzles (56%)
- Standard Hunt: 10 puzzles (40%)
- Accessible Hunt: 1 puzzle (4%)

### Domain Load
- Load 1 (Not required): 5 puzzles
- Load 2 (Helpful): 8 puzzles
- Load 3 (Essential): 12 puzzles

### Aha Clarity
- Aha 1 (None / procedural): 3 puzzles
- Aha 2 (Present but sequential): 16 puzzles
- Aha 3 (Single clean aha): 6 puzzles

### Paper Suitability
- Yes (fully text-reproducible): 15 puzzles
- Partial (some image/interactive dependency): 6 puzzles
- No (format-dependent or physical): 4 puzzles

### Quality Signal
- Quality 1: 0 puzzles
- Quality 2 (Standard): 10 puzzles
- Quality 3 (Exceptional): 15 puzzles

### Ablation Use
- Yes: 15 puzzles
- Conditional: 7 puzzles
- No: 3 puzzles (A Trip to the Museum, Street Smarts, Single Elimination)

---

## Top 12 Ablation Recommendations

**Selection criteria:** Text-reproducible (Paper Suitability = Yes), varied mechanisms, diverse hunt levels, strong quality signals (Quality 3 preferred), clear aha structure.

### Tier 1 — Include Without Reservation (8 puzzles)

These 8 represent the strongest candidates: all have Paper Suitability = Yes, Quality Signal = 3, and Ablation = Yes. Together they cover all mechanism types and span MIT/Elite and Standard Hunt tiers.

| Rank | Puzzle | Hunt Level | Mechanism | Domain | Aha | Rationale |
|------|--------|-----------|-----------|--------|-----|-----------|
| 1 | **Scicabulary** | MIT/Elite | Wordplay | 1 | 3 | Cleanest single-aha example in the corpus. Zero domain load, pure mechanism insight. Ideal ablation anchor. |
| 2 | **Bridge Building** | MIT/Elite | Logic + Knowledge (Hybrid) | 3 | 3 | Dual-domain hybrid with a single reframing aha. The Hashi-to-molecule transformation is the clearest example of integrated (vs. juxtaposed) hybrid mechanism. |
| 3 | **You're Telling Me** | MIT/Elite | Wordplay + Transformation | 1 | 3 | Meme-format-as-mechanism. Self-teaching, low domain load, 25+ entries sustain a single insight across length. Tests AI generation in comedic register. |
| 4 | **Information Relay** | MIT/Elite | Transform + Wordplay + Meta | 2 | 3 | Five distinct transformation systems composing into a meta-transformation. The intermediate phrase "HOW NANA HEARS GRAMPS" is the most elegant self-descriptive instruction in the corpus. |
| 5 | **Natural Transformation** | MIT/Elite | Logic + Transformation (Hybrid) | 2 | 2 | Rule-induction mechanism: solver must reconstruct the grammar before applying it. Rare mechanism type, well-constructed, text-reproducible. |
| 6 | **People Watching** | MIT/Elite | Knowledge extraction | 3 | 2 | Domain Load 3 with exceptional thematic integration. Contrasts with H2No (also Load 3, Standard Hunt, less thematic). Best example of "domain-heavy but world-coherent." |
| 7 | **Front and Center** | Standard Hunt | Wordplay | 1 | 3 | Exemplary single-mechanism wordplay with closed thematic loop. No domain, Aha 3, Quality 3. Clean control-condition puzzle for mechanism purity. |
| 8 | **Characters** | Standard Hunt | Hybrid (KE + Transform) | 2 | 3 | Three-layer chained transformation (clue → hex-valid word → hex arithmetic → ASCII) with self-naming answer (UNHEXING). Model of mechanism-chain design at Standard Hunt tier. |

### Tier 2 — Include for Specific Contrasts (4 additional to round out to 12)

These 4 fill mechanism gaps and provide useful negative/contrast cases. All have Paper Suitability = Yes.

| Rank | Puzzle | Hunt Level | Mechanism | Domain | Aha | Rationale |
|------|--------|-----------|-----------|--------|-----|-----------|
| 9 | **Par Re-sing** | MIT/Elite | Wordplay + Transformation | 2 | 2 | Self-referential mechanism applied twice (output of first pass is input to second). Tests recognition of second-order structural sophistication. |
| 10 | **Apples Plus Bananas** | MIT/Elite | Logic + Knowledge (Hybrid) | 3 | 2 | Multi-layer hybrid: emoji lookup + primality constraint + variety-name extraction. Contrasts with Bridge Building as a case where the primality step may be computation rather than deduction. |
| 11 | **H2No** | Standard Hunt | Knowledge extraction | 3 | 2 | Clean Knowledge Load 3 at Standard Hunt level. The "differences" flavor text double-reading is Quality 2 construction. Contrasts with People Watching (same Load, different tier and thematic integration). |
| 12 | **Dropypasta** | Standard Hunt | Knowledge + Transformation (Hybrid) | 3 | 2 | Two-phase domain-stratified extraction with a visible mechanism seam (topical rather than mechanical integration). Useful negative example: tests whether panels detect "juxtaposed" vs. "integrated" hybrids. |

### Final Top 12 Ablation Set

| # | Puzzle | Hunt Level | Mechanism | Domain | Aha | Quality |
|---|--------|-----------|-----------|--------|-----|---------|
| 1 | Scicabulary | MIT/Elite | Wordplay | 1 | 3 | 3 |
| 2 | Bridge Building | MIT/Elite | Logic + Knowledge | 3 | 3 | 3 |
| 3 | You're Telling Me | MIT/Elite | Wordplay + Transform | 1 | 3 | 3 |
| 4 | Information Relay | MIT/Elite | Transform + WP + Meta | 2 | 3 | 3 |
| 5 | Natural Transformation | MIT/Elite | Logic + Transform | 2 | 2 | 3 |
| 6 | People Watching | MIT/Elite | Knowledge extraction | 3 | 2 | 3 |
| 7 | Par Re-sing | MIT/Elite | Wordplay + Transform | 2 | 2 | 3 |
| 8 | Apples Plus Bananas | MIT/Elite | Logic + Knowledge | 3 | 2 | 3 |
| 9 | Front and Center | Standard Hunt | Wordplay | 1 | 3 | 3 |
| 10 | Characters | Standard Hunt | KE + Transform | 2 | 3 | 3 |
| 11 | H2No | Standard Hunt | Knowledge extraction | 3 | 2 | 2 |
| 12 | Dropypasta | Standard Hunt | KE + Transform | 3 | 2 | 2 |

### Coverage Analysis of the Top 12

**Hunt Level:** 8 MIT/Elite + 4 Standard Hunt. No Accessible Hunt representative (The Bakery is the only one and is Conditional; can substitute if accessible-hunt tier coverage is needed).

**Mechanism diversity:**
- Pure Wordplay: Scicabulary, Front and Center (2)
- Wordplay + Transformation: You're Telling Me, Par Re-sing (2)
- Logic + Knowledge (Hybrid): Bridge Building, Apples Plus Bananas (2)
- Logic + Transformation: Natural Transformation (1)
- Knowledge extraction: People Watching, H2No (2)
- Knowledge + Transformation: Characters, Dropypasta (2)
- Meta-composition: Information Relay (1)

**Domain Load spread:** Three at Load 1, three at Load 2, six at Load 3. Covers full range.

**Aha distribution:** Four puzzles at Aha 3, eight at Aha 2. No Aha 1 in final set — the three Aha 1 puzzles (Much Ado, Single Elimination, A Trip to the Museum) all fail Paper Suitability or are Conditional. If a procedural/no-aha example is needed, substitute Much Ado About Nothing (Conditional, Partial paper suitability, but usable if described carefully).

**Positive/negative contrast:** Dropypasta serves as the planned negative example (mechanism seam). Much Ado About Nothing is available as a second negative example if a "high complexity, low aha" condition is needed.

---

## Excluded Puzzles — Rationale

| Puzzle | Reason Excluded |
|--------|----------------|
| Museum Rules | Paper Suitability = No (interactive flip-book required) |
| Much Ado About Nothing | Aha 1 (procedural), Partial paper suitability; no aha moment |
| World's Smallest Logic Puzzles | Paper Suitability = Partial (grid-dependent); otherwise strong |
| Brain Freeze | Paper Suitability = Partial (image-based) |
| A Trip to the Museum | Paper Suitability = Partial; Ablation = No (physical location) |
| Street Smarts | Paper Suitability = No; Ablation = No (physical location) |
| Single Elimination | Paper Suitability = No; Ablation = No (JS-interactive) |
| Extrasensory | Conditional; Paper Partial; Quality 2; weak positive example |
| One of the Puzzles of All Time | Quality 2, culturally aging; redundant with better hybrids |
| Tales from a Dating Show | Conditional; JS dependency; domain stacking creates noise |
| What's Next? | Paper Partial (image-dependent); otherwise strong — substitute if needed |
| Itinerary | Conditional; launch erratum; knowledge obscurity confound |
| The Bakery | Accessible Hunt only; Conditional; low novelty |
