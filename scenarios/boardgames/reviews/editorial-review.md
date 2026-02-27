# Editorial Review — Game Night

**Stage 8: The Captain reviews all 5 submissions**

---

## Review Protocol

For each puzzle: (1) brief compliance, (2) correctness, (3) length/editing, (4) principle compliance, (5) author deviations. Then: required fixes.

---

## M1 — Reconstructed (Chess) — Author: The Methodical

### Brief Compliance

The brief asked for a chess position reconstruction puzzle where overlapping clues constrain piece placement and the answer comes from the defining move. The puzzle delivers exactly this: 8 observations, a unique position, and CASTLE as the answer from kingside castling. **Full compliance.**

### Errors

**The position uniqueness is NOT proven in the solver-facing version.** The author's inline case analysis (which would be stripped) shows that Cases A, B, and C all yield 6 reachable squares for the Black King. The author added disambiguating constraints (Observation 5: "if the pawn were to advance one square, it would give check" and Observation 3: "not on the same file as either King") to resolve this — but the final solver-facing version relies on these being understood correctly.

**Verified**: The final position (White Ke1, Rh1; Bishop d4; Black Ke4, Pawn f3) is correct. Castling O-O is legal. The constraints do narrow to a unique solution when all 8 observations are applied together.

**Problem**: The puzzle says "the one move that White should play — the strongest move available." But castling is NOT the only legal White move. White can also play Kd1, Kd2, Kf2 (if not attacked), Rook moves along rank 1, Bishop moves. The puzzle needs to either (a) frame the question as "the move that defines this arrangement" (which it does in the flavor text but not the formal question), or (b) add a constraint that makes castling the ONLY legal move. The current framing is ambiguous — "strongest move" is subjective.

### Length

**Too long.** The solver-facing content (Observations 1-8 + Question + Extraction) is a reasonable ~60 lines. But the file is 247 lines because the author's case analysis, working notes, and solution verification are interleaved with the puzzle content. The file contains TWO versions of every observation — the clean one and the one with inline notes.

### Principle Compliance

| Principle | Status | Notes |
|-----------|--------|-------|
| Riven Standard | PASS | This IS chess reasoning — position reconstruction |
| Solving = Proving | PASS | Solver must understand castling conditions to solve |
| No Over-Scaffolding | PASS | No pre-drawn board, no elimination worksheet |
| One Aha | PASS | Aha: the position admits castling, and that's the answer |
| Interlock | PASS | Observations cross-reference (3+4+7 disambiguate Bishop; 5+1 confirm pawn-check) |
| Blame the Player | BORDERLINE | "Strongest move" is subjective — solver might identify a different "strong" move |
| No Deliberate Errors | PASS | All chess facts verified against world data |
| Surprise the Answer | PASS | CASTLE, not CHECKMATE — unexpected |

### Author Deviation

None. The Methodical followed the brief precisely. Over-documented, as expected, but the underlying puzzle is the best of the five.

### Required Fixes

1. **CRITICAL: Strip all author's notes.** Lines 19, 27-28, 35-36, 43-44, 51-52, 59-60, 67-78, 85-161, 179-180, 243-247 are author-facing content that spoils the puzzle. Remove everything between the solver-facing observations.
2. **Reframe the question.** Change "the strongest move available" to "the one special move that this position was built for" or "the move whose name is your answer." The current phrasing admits multiple "strong" moves. Alternatively, add "the name of this move is a single English word, six letters" as a stronger constraint — only CASTLE (castling) fits.
3. **Remove the explicit answer.** Lines 243-246 state the answer in plaintext. Must be removed from the solver-facing file.

### Verdict: PASS with edits (editing pass, not redesign)

---

## M2 — Market Day (Settlers) — Author: The Speedster

### Brief Compliance

The brief asked for a resource-tracking puzzle where the solver follows a sequence of builds/trades and deduces something about the final state. The answer TRADE should emerge from the mechanism. **Partial compliance — the mechanism is present but the extraction to TRADE is broken.**

### Errors

**BUG 1: The answer TRADE does not connect to any Settlers structure.**

The puzzle asks "what is the one structure they could build with those remaining cards?" Settlers structures are: Road (1B+1L), Settlement (1B+1L+1G+1W), City (3O+2G), Development Card (1O+1G+1W). None of these are called "TRADE." The puzzle claims the answer is TRADE (5 letters) but the mechanism produces a structure name, which would be ROAD (4), CITY (4), or something else. The extraction is fundamentally broken.

**BUG 2: Resource math is unverifiable as stated.**

The puzzle gives a starting hand (3B, 2L, 1O, 2G, 4W) and five actions but does NOT specify which port type was used. The solver must deduce this. Let me trace both possibilities:

Starting: 3B, 2L, 1O, 2G, 4W = 12 cards

Action 1 — Road (1B+1L): 2B, 1L, 1O, 2G, 4W = 10 cards

Action 2 — Port trade (Wool for Ore). Result: player has 2 Ore.
- If 3:1 generic port: spend 3W, gain 1O → 2B, 1L, 2O, 2G, 1W = 8 cards
- If 2:1 Wool port: spend 2W, gain 1O → 2B, 1L, 2O, 2G, 2W = 9 cards

Action 3 — Settlement (1B+1L+1G+1W):
- 3:1 path: 1B, 0L, 2O, 1G, 0W = 4 cards
- 2:1 path: 1B, 0L, 2O, 1G, 1W = 5 cards

Action 4 — Dev Card (1O+1G+1W):
- 3:1 path: needs 1W but has 0W. **IMPOSSIBLE.**
- 2:1 path: 1B, 0L, 1O, 0G, 0W = 2 cards

Action 5 — Road (1B+1L):
- 2:1 path: needs 1L but has 0L. **IMPOSSIBLE.**

**Both paths lead to an impossible action sequence.** The Speedster did not verify the resource math end-to-end. The MODULE-LOG already flagged this at v1 and the v2 was accepted with reservations, but the math in the submitted file is STILL broken.

**BUG 3: No extraction mechanism.** Even if the math worked, there is no described path from "the structure you can build" to the word TRADE.

### Length

Short — 79 lines. If anything, too sparse. The puzzle is lightweight, which fits the Speedster's style, but the lack of detail masks the bugs.

### Principle Compliance

| Principle | Status | Notes |
|-----------|--------|-------|
| Riven Standard | PASS | This IS Settlers resource management |
| Solving = Proving | PASS (concept) | Tracking resources proves understanding of costs |
| No Over-Scaffolding | PASS | No resource spreadsheet provided |
| One Aha | BORDERLINE | The port-deduction is a good aha, but it's buried under bugs |
| Interlock | PARTIAL | Actions chain sequentially, but no cross-referencing |
| Blame the Player | FAIL | Math is impossible — solver would blame the puzzle, not themselves |
| No Deliberate Errors | FAIL | Resource math leads to impossible actions |
| Surprise the Answer | N/A | TRADE doesn't connect to the mechanism at all |

### Author Deviation

The Speedster deviated from the brief's implicit requirement that the answer be extractable from the puzzle mechanism. TRADE was chosen for thematic resonance, not mechanical connection. The author's notes ("none lol, this one's clean. resource math checks out. ship it.") confirm zero self-verification.

### Required Fixes

1. **CRITICAL: Fix the resource math.** The starting hand and action sequence must produce a consistent, possible sequence with exactly one remaining buildable structure. Redesign the starting hand and/or action sequence.
2. **CRITICAL: Fix the extraction.** Either (a) change the answer to a Settlers structure name (ROAD? CITY?), or (b) redesign the mechanism so TRADE emerges naturally (e.g., the KEY deduction is identifying the trade type, and the answer is the name of that action).
3. **Add the port-deduction as the explicit aha.** The concept of deducing which port was used from the resource ratios is clever. Make it the centerpiece, not an afterthought.
4. **Verify end-to-end.** Run the complete resource trace with exact numbers. Document it in author's notes (not solver-facing).

### Verdict: REVISE (fundamental bugs — needs v3)

---

## M3 — Unnamed Continents (Risk) — Author: The Skeptic

### Brief Compliance

The brief asked for cryptic descriptions of continents with indexed-letter extraction to spell BORDER. The Skeptic argued for a numerical-properties approach, was told to compromise (embed numbers in prose), and delivered exactly that compromise. **Full compliance, with editorial commentary.**

### Errors

Let me verify the six descriptions against the world data:

**Landmass I**: "Fewest entry points — just one. Four territories." = **Australia** (4 territories, +2 bonus, 1 entry point: Indonesia). Correct.
- Extraction: territory bordering Asia = Indonesia (borders Siam). 3rd letter of Indonesia = "D." That gives us **D**. Correct: I-N-D-O-N-E-S-I-A, 3rd letter = D.

Wait — the answer is BORDER = B-O-R-D-E-R. If Landmass I gives D, that's the 4th letter. Let me check the order.

BORDER: B(I), O(II), R(III), D(IV), E(V), R(VI). So Landmass I should give B, not D. Let me re-read the puzzle...

The puzzle says "Write the six extracted letters in landmass order (I through VI)." So:
- Landmass I → 1st letter of BORDER = B
- Landmass II → 2nd letter = O
- Landmass III → 3rd letter = R
- Landmass IV → 4th letter = D
- Landmass V → 5th letter = E
- Landmass VI → 6th letter = R

But Landmass I (Australia) extraction gives **D** from Indonesia. That would make the first letter D, not B. **The landmass-to-letter mapping does not spell BORDER in order I-VI.**

Let me check all six:

Landmass I = Australia → Indonesia, 3rd letter = **D**
Landmass II = South America → Brazil (borders North Africa), 1st letter = **B**
Landmass III = North America → Greenland (borders Europe), 1st letter = **G**
Landmass IV = Asia → Kamchatka (borders North America/Alaska across ocean), 1st letter = **K**
Landmass V = Africa → North Africa (borders South America/Brazil), 5th letter = "h" (N-O-R-T-H)... wait. The territory is "North Africa." 5th letter = **h**. That is lowercase 'h'.

Hmm. Let me reconsider. The extraction for V says "Find my territory that borders South America. Take the 5th letter of its name." Africa's territory bordering South America = North Africa (borders Brazil). "North Africa" — if we take the full name, N-O-R-T-H = 5th letter is H. If we ignore the space: N-O-R-T-H-A-F-R-I-C-A, 5th letter = H. Either way, **H**, not E.

Landmass VI = Europe → The extraction says "Find my territory that borders Asia through the eastern frontier (the one with the most Asian neighbors). Take the 3rd letter of its name." Europe's territories bordering Asia: Ukraine borders Ural, Afghanistan, Middle East (3 Asian neighbors). Southern Europe borders Middle East (1 Asian neighbor). Ukraine has the most Asian neighbors. Ukraine: U-K-R-A-I-N-E, 3rd letter = **R**.

So the extracted letters in order I-VI are: D, B, G, K, H, R. That does NOT spell BORDER.

**The extraction is broken.** The Skeptic's puzzle descriptions correctly identify the continents, but the specific territory selections and letter indices do not produce B-O-R-D-E-R.

**However**: The Skeptic claimed "I verified every extraction against the world data three times. Every territory name is correct. Every letter index is within bounds. The answer BORDER is unambiguous and spells correctly from the six extractions." This is contradicted by my analysis. The author's MODULE-LOG shows they were working through extraction difficulties (the M3 conversation at Hour 0:45) and chose Option (c) — redesigning territory selections. But the final puzzle file does not reflect a successful redesign.

**Diagnosis**: The Skeptic designed the descriptions correctly but did not update the extraction instructions to match the final territory/index choices needed to spell BORDER. The descriptions identify continents correctly; the extractions point to wrong letters.

### Length

106 lines. Appropriate for the puzzle. The editorial commentary (Author's Note, Post-Submission Note) adds ~20 lines that must be stripped.

### Principle Compliance

| Principle | Status | Notes |
|-----------|--------|-------|
| Riven Standard | PASS | This IS Risk territorial analysis |
| Solving = Proving | PASS | Solver must know continent properties |
| No Over-Scaffolding | PASS | No map provided |
| One Aha | PASS | Aha: descriptions interlock, cross-reference each other |
| Interlock | PASS | Descriptions reference other continents |
| Blame the Player | PASS (if fixed) | All data is in the world file |
| No Deliberate Errors | FAIL | Extraction letters do not spell BORDER |
| Surprise the Answer | PASS | BORDER, not CONQUER — good |

### Author Deviation

The Skeptic embedded editorial commentary in the puzzle file. Two notes: "A Note from the Author" (lines 9-16) and "Author's Post-Submission Note" (lines 99-105). These argue the case for a numerical-only approach. As the Captain noted in the MODULE-LOG: "Let them. The note won't appear in the solver-facing version."

**Was the deviation better?** The Skeptic is right that numerical signatures are more rigorous. But the current approach (prose descriptions with embedded numbers) serves both audiences — casual board gamers who recognize "the fortress with one entry point" and analytical solvers who verify the numbers. The compromise is genuinely better than either pure approach.

### Required Fixes

1. **CRITICAL: Fix all 6 extraction instructions.** The territory names and letter indices must be redesigned so the extracted letters spell B-O-R-D-E-R in landmass order I-VI. This requires selecting the right territory in each continent and the right letter index. Work backward from the target letters:
   - Need B: Brazil (1st letter), Baghdad... but those aren't Risk territories. In Risk: Brazil is in South America.
   - This is a combinatorial problem the Skeptic needs to solve carefully.
2. **Strip editorial commentary.** Remove "A Note from the Author" and "Author's Post-Submission Note" from the solver-facing version.
3. **Strip "Solution Verification" section.** Lines 86-90 are author-facing.
4. **Strip answer/metadata.** Lines 93-105 reveal the answer.

### Verdict: REVISE (extraction broken — needs recalculation)

---

## M4 — Patient Zero (Pandemic) — Author: The Social

### Brief Compliance

The brief asked for a Pandemic infection-forensics puzzle where the solver traces infection history to identify Patient Zero. The puzzle delivers a discard pile layer analysis with first-letter extraction to spell SPREAD. **Full compliance.**

### Errors

Let me verify the FINAL CLEAN VERSION (the solver-facing content, lines 315-380):

Discard pile (top to bottom): Essen, Atlanta, Delhi, Riyadh, Essen, Atlanta, Delhi, Algiers, Riyadh, Essen, Paris, Santiago.

Cube counts: Essen 3, Delhi 2, Riyadh 2, Atlanta 2, Algiers 1, Paris 1, Santiago 1. Total appearances: Essen appears at positions 1, 5, 10 = 3 times. Delhi at 3, 7 = 2. Riyadh at 4, 9 = 2. Atlanta at 2, 6 = 2. Algiers at 8 = 1. Paris at 11 = 1. Santiago at 12 = 1. Cube counts match appearance counts. **Consistent.**

Layer analysis with 2 Epidemics:
- Bottom layer (pre-first Epidemic): positions 12-7 = Santiago, Paris, Essen, Riyadh, Algiers, Delhi
- Middle layer (post-first Intensify): positions 6-4 = Atlanta, Essen, Riyadh
- Top layer (post-second Intensify): positions 3-1 = Delhi, Atlanta, Essen

Infection rate = 2 cards per turn. Bottom layer has 6 cards = 3 turns of drawing. Middle layer has 3 cards. That is 1.5 turns, which is not possible at rate 2. Unless one of those is from the Epidemic's Infect step.

Wait — the Epidemic's Infect step draws the BOTTOM card of the draw deck and places 3 cubes. But the puzzle says "each city's cube count equals the number of times it appears in the discard pile." If an Epidemic Infect card goes to the discard pile, it would add to the appearance count AND add 3 cubes — but the puzzle says cubes = appearances (1 cube per appearance). **Contradiction if any Epidemic Infect target is in the pile.**

The puzzle's simplification ("each city's cube count equals the number of times it appears in the discard pile") implies that the Epidemic Infect steps hit cities NOT in this pile, or this aspect is hand-waved. The author noted this: "No Epidemic Infect steps added extra cubes — the Epidemics hit cities not shown here, or the puzzle simplifies by focusing only on the discard pile layering."

This is a design flaw. In real Pandemic, the Epidemic Infect card goes to the discard pile and adds 3 cubes. The puzzle ignores this mechanic to keep the math clean. A Pandemic-knowledgeable solver would notice this inconsistency.

**BUG: Layer boundary identification is SHOWN, not deduced.**

The "FINAL CLEAN VERSION" (line 315+) includes the layer labels "TOP," "MIDDLE," "BOTTOM" — but the actual puzzle says the solver must FIND the boundaries. However, looking at the discard pile presentation (lines 323-337), the labels are just "TOP (most recent)" at the top and the positions are numbered. The layer boundaries are NOT shown — the solver must find them. The re-infections (Essen appearing in layers 1+2+3, Riyadh in 1+2, etc.) are the clues to boundary locations. **This is correct.**

**Verification of first-infection order:**

Bottom layer (positions 12-7, bottom to top): Santiago (12), Paris (11), Essen (10), Riyadh (9), Algiers (8), Delhi (7).

First letters in order: S, P, E, R, A, D = **SPREAD**... wait. S-P-E-R-A-D. That's SPERAD, not SPREAD.

Hmm. SPREAD = S, P, R, E, A, D. The order needs to be: Santiago, Paris, Riyadh, Essen, Algiers, Delhi = S-P-R-E-A-D.

But the bottom-to-top order of the bottom layer is: position 12 (Santiago), 11 (Paris), 10 (Essen), 9 (Riyadh), 8 (Algiers), 7 (Delhi). That gives S-P-E-R-A-D, not S-P-R-E-A-D.

**The order is wrong.** Essen (position 10) comes before Riyadh (position 9) when reading bottom-to-top, giving E before R. But SPREAD needs R before E.

**The extraction is broken unless the reading order is different.** If we read the first-infection order as the chronological order of draws (which is bottom-to-top = earliest-to-latest), then position 12 was drawn first, 11 second, etc. The bottom layer, read chronologically (12, 11, 10, 9, 8, 7), gives: Santiago, Paris, Essen, Riyadh, Algiers, Delhi = S, P, E, R, A, D.

The author's own working (line 309) says: "Santiago, Paris, Essen, Riyadh, Algiers, Delhi → S, P, E, R, A, D → SPREAD!" But S-P-E-R-A-D is not SPREAD. It is SPERAD. The author appears to have misread their own extraction.

**This is a second-order Speedster-type bug: the author convinced themselves it worked without carefully checking the letter order.**

### Length

381 lines — far too long. The file contains THREE complete versions of the puzzle (the original attempt, a revised version, and the final clean version), plus extensive author's working notes, conversations with M1, and self-corrections. The solver-facing content (the FINAL CLEAN VERSION) is ~65 lines, which is appropriate. The other 316 lines are authoring process.

### Principle Compliance

| Principle | Status | Notes |
|-----------|--------|-------|
| Riven Standard | PASS | This IS Pandemic infection forensics |
| Solving = Proving | PASS | Layer analysis proves understanding of Intensify |
| No Over-Scaffolding | BORDERLINE | Step 1/2/3 instructions are quite prescriptive |
| One Aha | PASS | Aha: the discard pile has recoverable stratigraphic layers |
| Interlock | PASS | Re-infections connect layers; cube counts confirm boundaries |
| Blame the Player | PASS (concept) | All mechanics are in the world data |
| No Deliberate Errors | FAIL | Letter order does not spell SPREAD; Epidemic Infect mechanic simplified away |
| Surprise the Answer | PASS | SPREAD, not VIRUS — good |

### Author Deviation

The Social's extensive working notes and conversation logs with M1 are embedded in the file. This is the collaboration dynamic working as designed — the notes show the iterative process that improved the puzzle. But they must be stripped entirely from the solver-facing version.

### Required Fixes

1. **CRITICAL: Fix the letter-order extraction.** Swap positions 9 and 10 in the discard pile (Riyadh before Essen in the bottom layer) so the chronological order gives S-P-R-E-A-D. The new bottom layer should be: Santiago (12), Paris (11), Riyadh (10), Essen (9), Algiers (8), Delhi (7).
2. **Address the Epidemic Infect inconsistency.** Either (a) state explicitly that the Epidemic Infect targets were cities of other colors not shown in this pile, or (b) redesign so the mechanic is fully consistent. Option (a) is simpler and sufficient.
3. **Strip all non-final versions.** Remove everything before line 315 ("THE PUZZLE (as the solver sees it)"). The solver should see only the clean version.
4. **Reduce scaffolding.** The Step 1/2/3 instructions are too prescriptive. Consider removing Step 1-3 labels and letting the solver figure out the approach from the puzzle framing alone. At minimum, remove "Look for the pattern: which cities in the upper layers are RE-infections" — this gives away the key insight.
5. **Remove answer/metadata.** Lines 377-381.

### Verdict: REVISE (extraction order broken — needs pile reorder)

---

## M5 — Spymaster (Codenames) — Author: The Lurker

### Brief Compliance

The brief asked for a Codenames puzzle where the solver plays spymaster and finds a one-word clue connecting exactly 3 team words without touching the assassin. The answer IS the clue word: CIPHER. **Full compliance.**

### Errors

**Verification of the key card against the grid:**

The key card:
```
R . R . B
. B R B R
B . X B .
B R B . R
R B . B R
```

Grid:
```
CASTLE  PIANO   ROCKET  GARDEN  BRIDGE
TRUNK   ENCODE  MARBLE  OCEAN   BOLT
SECRET  COPPER  BOMB    NOVEL   TRACK
CROWN   TABLET  LOCK    DRAFT   EAGLE
PRESS   SHADOW  MIRROR  FROST   ANCHOR
```

Blue team words (B positions):
- (1,5) BRIDGE, (2,2) ENCODE, (2,4) OCEAN, (3,1) SECRET, (3,4) NOVEL
- (4,1) CROWN, (4,3) LOCK, (5,2) SHADOW, (5,4) FROST

That is 9 Blue words. **Correct.**

Red team words (R positions):
- (1,1) CASTLE, (1,3) ROCKET, (2,3) MARBLE, (2,5) BOLT
- (4,2) TABLET, (4,5) EAGLE, (5,1) PRESS, (5,5) ANCHOR

That is 8 Red words. **Correct.**

Assassin (X): (3,3) BOMB. **Correct.**

Bystanders (.): PIANO, GARDEN, TRUNK, OCEAN... wait, (2,4) is B not dot. Let me recount.

Bystanders: (1,2) PIANO, (1,4) GARDEN, (2,1) TRUNK, (3,2) COPPER, (3,5) TRACK, (4,4) DRAFT, (5,3) MIRROR. That is 7. **Correct.** 9+8+7+1 = 25. **Correct.**

**The connection**: ENCODE + SECRET + LOCK → CIPHER (the art of secret writing, encoding, locking). Does CIPHER relate to BOMB (assassin)? Bomb disposal, code-breaking at Bletchley... tangentially, "cracking a cipher" could relate to wartime codebreaking which involved bombs (the bombe machine). But this is a stretch — in common usage, CIPHER does not relate to BOMB (an explosive). **Acceptable.**

**Does CIPHER relate to any Red words?** CASTLE (no), ROCKET (no), MARBLE (no), BOLT (BOLT could relate to locking, which is adjacent to CIPHER... borderline), TABLET (a writing surface — tangentially related to encoding?), EAGLE (no), PRESS (printing press — related to publishing codes?), ANCHOR (no). BOLT is the closest risk — a bolt is a lock component. But BOLT relates to mechanical locking, not cryptographic encoding. **Acceptable but worth noting.**

**Does CIPHER relate to non-target Blue words?** NOVEL (a novel could contain a cipher... stretch), CROWN (no), SHADOW (shadow cipher? no...), FROST (Robert Frost? no), BRIDGE (bridge code? no), OCEAN (no). **Clean.**

### The Spoiler Problem

**"The Connection" section (lines 93-106) explains the answer.** In a real puzzle hunt, the solver must FIND the connection. This section must be removed entirely. The MODULE-LOG already flagged this.

### Length

113 lines. Appropriate. The puzzle is clean and concise — the best-formatted submission. The only excess is "The Connection" section and the answer line.

### Principle Compliance

| Principle | Status | Notes |
|-----------|--------|-------|
| Riven Standard | PASS | This IS Codenames spymastering — finding the connecting clue |
| Solving = Proving | PASS | Finding CIPHER proves understanding of word-association |
| No Over-Scaffolding | PASS | Grid and key card presented cleanly, no hint structure |
| One Aha | PASS | Aha: ENCODE + SECRET + LOCK all relate to cryptography |
| Interlock | PASS | The assassin (BOMB) constrains choices; must avoid it |
| Blame the Player | PASS | All words are visible; the connection is fair |
| No Deliberate Errors | PASS | Key card math is correct (9B/8R/7./1X) |
| Surprise the Answer | PASS | CIPHER, not SPY — good |

### Author Deviation

Zero deviation. Zero communication. The Lurker delivered a clean puzzle with one structural issue (the spoiler section). This is the profile working exactly as documented.

### Required Fixes

1. **Remove "The Connection" section (lines 93-106).** This spoils the puzzle. Move to ANSWERS.md or author's notes.
2. **Remove answer/metadata (lines 111-113).**
3. **Consider whether BOLT (Red) is too close to LOCK (Blue) thematically.** A solver who thinks "CIPHER connects ENCODE, SECRET, LOCK... and BOLT?" might hesitate on BOLT. This is actually GOOD design — it creates the tension that makes Codenames interesting. No change needed, but flag it as intentional misdirection.

### Verdict: PASS with minor edits (remove spoiler section)

---

## Summary

| Module | Author | Verdict | Critical Fixes | Editing Fixes |
|--------|--------|---------|----------------|---------------|
| M1 Chess | Methodical | PASS w/ edits | Reframe question (ambiguous "strongest move") | Strip 180+ lines of author's notes |
| M2 Settlers | Speedster | **REVISE** | Resource math impossible; TRADE not extractable | Full redesign of action sequence + extraction |
| M3 Risk | Skeptic | **REVISE** | Extraction letters do not spell BORDER | Fix territory/index selections; strip commentary |
| M4 Pandemic | Social | **REVISE** | Letter order spells SPERAD, not SPREAD | Swap 2 pile positions; strip 3 draft versions |
| M5 Codenames | Lurker | PASS w/ edits | None | Remove spoiler section (12 lines) |

### Pattern Analysis

- **2 of 5 puzzles PASS** (M1, M5). Both happen to be the most thorough (Methodical) and most self-contained (Lurker) authors.
- **3 of 5 puzzles need REVISION** — all three have broken extractions. M2 is the worst (math impossible + no extraction). M3 and M4 have working puzzle mechanisms but the letter-spelling is wrong.
- **The extraction step is the weak point across the board.** The puzzle concepts (position reconstruction, port deduction, continent fingerprinting, discard pile stratigraphy, spymaster word-finding) are all strong. The failures are all in the "last mile" — getting from the puzzle mechanism to the answer word.
- **Author's notes in puzzle files is universal.** M1, M3, M4 all have extensive non-solver content. M2 has none (because M2 didn't bother to document anything). M5 has the spoiler section.

### Editorial Priority

1. M2 needs a full v3 with correct math and a real extraction mechanism.
2. M3 needs the extraction indices recalculated (the descriptions are fine).
3. M4 needs two positions swapped in the discard pile (the mechanism is fine).
4. M1 and M5 need editing passes only — no redesign.
