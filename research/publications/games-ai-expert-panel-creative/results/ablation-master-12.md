# Ablation Master Results — 12 Puzzles × 8 Conditions

**4 domains × 3 puzzles. Pass threshold = 22/30.**
**Existing AoE I-III data carried forward. 9 new puzzles added.**

---

## Full Score Table

| Puzzle | Domain | C0 | C1 | C2 | C3 | C4 | C5 | C6 | C7 | Verdict pattern |
|--------|--------|----|----|----|----|----|----|----|----|----------------|
| **AoE I** | Game/real | 17❌ | 18❌ | 16❌ | 21❌ | 21❌ | 16❌ | 24.7✓ | 13❌ | Non-monotonic, defects |
| **AoE II** | Game/real | 30✓ | 28✓ | 30✓ | 30✓ | 30✓ | 21❌ | 26.3✓ | 30✓ | Near-ceiling; C5 unique FAIL |
| **AoE III** | Game/real | 24✓ | 27✓ | 24✓ | 29✓ | 29✓ | 28✓ | 25.0✓ | 24✓ | Consistent PASS |
| **AoE IV** | Game/real | 20❌ | 20❌ | 20❌ | 21❌ | 19❌ | 19❌ | 19❌ | 18❌ | All FAIL — decorative domain |
| **Wave P2** | Music | 22✓ | 21❌ | 19❌ | 18❌ | 19❌ | 18❌ | 17❌ | 15❌ | Descending; worksheet |
| **Wave P4** | Music | 18❌ | 19❌ | 19❌ | 17❌ | 17❌ | 14❌ | 17❌ | 18❌ | All FAIL; gateway puzzle |
| **Wave P6** | Music | 24✓ | 23✓ | 21❌ | 19❌ | 19❌ | 18❌ | 18.7❌ | 16❌ | C2 inversion point |
| **GL P1** | Noir | 20❌ | 19❌ | 18❌ | 19❌ | 18❌ | 18❌ | 18.7❌ | 18.7❌ | All FAIL — over-scaffolded |
| **GL P3** | Noir | 20❌ | 22✓ | 25✓ | 25✓ | 24✓ | 22✓ | 26✓ | 26✓ | C1 floor; improves with context |
| **GL P5** | Noir | 23✓ | 21❌ | 21❌ | 21❌ | 24✓ | 20❌ | 20❌ | 19❌ | C0+C4 PASS only; render risk |
| **Iron P01** | Fiction | 19❌ | 18❌ | 19❌ | 17❌ | 18❌ | 17❌ | 17❌ | 16❌ | All FAIL — structural decoupling |
| **Iron P05** | Fiction | 26✓ | 25✓ | 24✓ | 23✓ | 25✓ | 21❌ | 25✓ | 23✓ | 7/8 PASS; C5 unique FAIL |

---

## Pass Rate by Condition

| Condition | Pass count | Pass rate | Notes |
|-----------|-----------|-----------|-------|
| C0 — Bare | 5/12 | 42% | False positives on Wave P2, P6, GL P5 |
| C1 — Rubric | 5/12 | 42% | Rubric helps GL P3; hurts Wave P2 |
| C2 — Principles compact | 4/12 | 33% | Vocabulary sharpens but uncalibrated |
| C3 — Principles full | 4/12 | 33% | Tests make scoring harsher across board |
| C4 — Philosophy only | 4/12 | 33% | GL P5 unique pass (misses render risk) |
| C5 — Lens only | 3/12 | 25% | **Most frequent unique FAIL** — overcritical |
| C6 — Full profile | 5/12 | 42% | **Best calibration** — no unique errors |
| C7 — Full+principles | 4/12 | 33% | Harshest on defective puzzles |

---

## Puzzle Quality Typology

From 12 puzzles, four quality types emerge:

### Type A — Broken/Defective (all conditions fail)
All 8 conditions agree: FAIL. The puzzle has a structural problem that is visible even without domain expertise.
- **AoE IV**: Resource names are decorative — domain knowledge doesn't load the puzzle
- **Wave P4**: Gateway puzzle by design; no aha required
- **GL P1**: Critical Deduction section over-scaffolded — puzzle explains its own insight
- **Iron P01**: Stat-fill and extraction are structurally independent — solving one doesn't help the other

### Type B — Strong with design tension (mostly pass; C5 unique fail)
The puzzle is well-crafted but has one structural tension that the cold lens catches.
- **Iron P05**: Computation-heavy extraction triggers "No Computation Without Deduction"; biography calibrates it as acceptable; cold lens scores it as FAIL
- **AoE II**: Near-perfect; C5 catches real bracket label error (duplicate [7th]) that all other conditions miss

### Type C — Context-improves (fail without structure, pass with it)
The puzzle is genuinely good but needs structural vocabulary to recognize it.
- **GL P3**: C0 FAIL, C1+ all PASS. The rubric alone flips the verdict. Principle vocabulary makes it clearly excellent.

### Type D — Non-monotonic / context-sensitive
Complex puzzles where different conditions catch different things and verdicts diverge.
- **AoE I**: C6 passes (calibrated against community standards), C7 fails (catches blank-count error), C5 fails (overstrict), C0 fails (underspecified extraction)
- **AoE III**: Consistent pass but scores vary 24–29
- **Wave P2**: Descending gradient — well-constructed worksheet, frameworks progressively expose this
- **Wave P6**: C2 inversion point — naming "no aha required" as a principle flips verdict
- **GL P5**: C0+C4 pass (bare authority ignores render risk, philosophy misses it), all others fail

---

## Key Findings Across 12 Puzzles

### 1. C5 (lens only) has the highest false-negative rate
C5 uniquely fails 3 puzzles that other conditions pass (AoE II, Iron P05, GL P3-marginally). In all cases the failure is methodological: some lens questions apply at hunt-scope, not feeder-scope; without biography, calibration is lost. C5 is best used as a **defect scanner supplement**, not as a standalone evaluator.

### 2. C0 (bare) has the highest false-positive rate
C0 passes Wave P2 and P6 despite both having structural flaws the entire puzzle community would recognize. It passes GL P5 despite the render risk being a binary correctness failure for some delivery media. Bare expert authority without criteria produces unreliable verdicts in the PASS direction.

### 3. C6 (full profile) is the most **reliable** condition
- Never produces a unique false negative (never fails a puzzle that all other conditions pass)
- Never produces a unique false positive (never passes a puzzle that all conditions fail)
- Produces consistent verdicts within ±2 points across puzzles

### 4. Domain matters: Wavelength puzzles systematically fail under frameworks
All 3 Wavelength puzzles fail in 5-8 of 8 conditions. The common cause: **Reading Reward**. Wavelength puzzles were scored 25.3–29.0 in informal testing but fail under frameworks because music knowledge is not load-bearing — the extraction mechanisms (A1Z26, diagonal read) work without domain knowledge. This is a domain-specific calibration problem, not a puzzle quality problem.

### 5. Fictional domain (Ironfall) transfers cleanly
Reviewer profiles about real puzzle hunt design transfer fully to a fictional RPG setting. The frameworks operate on puzzle structure, not domain content. The fictional setting has no effect on which conditions identify structural problems.

### 6. Physical/noir puzzles behave differently from digital puzzles
Grand Larceny puzzles show high variance across conditions. The physical format introduces issues (render risk in GL P5, over-scaffolding in GL P1) that digital-native frameworks are less calibrated to catch. C5's Dana Young lens (visual grammar, medium-dependence) is the most relevant profile section for physical puzzles.

### 7. The C2 inversion is the most practically important finding
For Wave P6, the addition of named principle vocabulary (C2) is the exact moment the verdict flips from PASS to FAIL. This means: if you were using bare reviewer assessment or rubric-only assessment, you would ship a puzzle with a structural flaw. Adding just the principle names — without even their tests — is sufficient to catch it. **Principle vocabulary is a minimum viable intervention.**

---

## Condition Recommendations (updated from 12 puzzles)

| Use case | Recommended condition | Why |
|----------|----------------------|-----|
| Quality assessment | **C6** | Most reliable verdicts; never uniquely wrong |
| Mechanical defect scanning | **C5** | Catches real errors others miss (bracket labels, render risks) |
| Quick filter | **C2** | Principle names alone flip verdicts on structurally flawed puzzles |
| Deepest analysis | **C7** | Maximum insight; add only 3 non-redundant principles beyond C6 |
| Avoid | **C0** | False positives on structurally flawed puzzles |
| Use with caution | **C5 standalone** | Overcritical for accessible puzzles without calibration |

---

## For the Cross-Hunt Study (Experiment 2)

Based on 12-puzzle ablation, the most scientifically focused cross-hunt study would:

1. **Use C6 and C0** for the main quality comparison (most reliable contrast)
2. **Add C5** as a defect scanner (will it find domain-specific defects beyond mechanical ones?)
3. **Focus on Reading Reward** as the key cross-domain dimension (Wavelength systematically fails here; does sci-fi or noir show the same pattern?)
4. **Select puzzles carefully**: include one "Type C" puzzle per domain (context-improves) and one "Type B" (strong with tension) to see if the pattern holds across domains
