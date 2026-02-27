# Live Test — Game Night

**Stage 10: 4-agent abbreviated live test**

Agents: Captain (admin/moderator), Newbie (first-timer), Speedster (fast solver), Skeptic (edge-case finder).

Format: Abbreviated transcript. Focus on the four test questions: Newbie comprehension, Speedster shortcuts, Skeptic edge cases, meta functionality.

---

## Setup

The Captain distributes the five puzzles simultaneously. No solve order required. The Host's opening text is read aloud:

> *The board is out. The pieces are sorted. Mostly. Five games, five puzzles. You don't play them tonight. Tonight you solve them. The snacks are on the counter. The timer is running. Nobody asked for a timer but it felt right.*

---

## Opening Phase (Minutes 0-5)

**Newbie**: "OK so we have five puzzles? Do we each take one, or do we all work on the same one? ... And what does 'solve' mean here? Like, play the game? Or there's a question to answer?"

**Captain**: "Each puzzle has a question or task. You solve it, and the answer is a word. All five answer words feed into a final puzzle — the meta. Let's start with whichever one looks most approachable."

**Speedster**: *[already scanning all five]* "Codenames is the easiest. It's just 'find the connecting word.' I can see ENCODE, SECRET, LOCK — that's cryptography. The answer is CIPHER. Done. Next."

**Captain**: "Speedster, slow down. Did you check the assassin?"

**Speedster**: "BOMB? Yeah, CIPHER doesn't relate to BOMB. Well, maybe the Enigma bombe machine... but that's a stretch. It's clean. CIPHER."

**Skeptic**: "Actually, hold on. The bombe WAS a cipher-cracking device. The word CIPHER is etymologically related to the entire codebreaking apparatus, which includes physical devices colloquially called 'bombs.' I want to verify that the puzzle designer considered this."

**Newbie**: "Wait, what are we talking about? I haven't even figured out what the grid is. Where's the question?"

**Captain**: "Newbie, look at puzzle 5 — Spymaster. There's a 5x5 word grid and a key card. You're the spymaster. Find a one-word clue connecting exactly 3 Blue words. That clue is your answer."

**Newbie**: "Oh. Like the actual game Codenames? I've played that. OK, so I need to find three Blue words that share a theme... and not the assassin word BOMB?"

**Captain**: "Exactly."

**Newbie**: "OK. ENCODE, SECRET, LOCK... those are all about hiding things. And the word would be... CODE? CIPHER? ... CIPHER. Six letters. Is that right?"

**Captain**: "That's right. You just solved your first puzzle."

**Newbie**: "That was... not as hard as I expected? The Codenames format really helped — I already knew what a spymaster does."

---

### Finding 1: The Newbie understood the Host's voice and the Codenames puzzle immediately.

The familiar game format (Codenames) served as a natural on-ramp. The Newbie's board-game knowledge translated directly into puzzle-solving capability. This validates the Riven Standard — the puzzle IS what the game does.

### Finding 2: The Speedster solved Codenames in under 30 seconds.

CIPHER is the "obvious" connection for anyone who sees ENCODE + SECRET + LOCK. The Speedster found it immediately through pattern matching, without reading the rules or checking the key card carefully. This is expected for a difficulty-2 puzzle.

### Finding 3: The Skeptic's BOMB/bombe concern is valid but not blocking.

The historical connection between CIPHER and bombe machines is real but obscure. In Codenames terms, an operative hearing "CIPHER, 3" would not guess BOMB. The connection is safe for gameplay purposes. The Skeptic files the note but does not object to the answer.

---

## Settlers Puzzle (Minutes 5-15)

**Speedster**: "OK, Settlers next. Starting hand, five actions, track resources. I can do this in my head..."

*[2 minutes later]*

**Speedster**: "Something's wrong. After the road and port trade, I don't have enough Wool for the settlement AND the dev card. Let me try the 2:1 port... nope, then I'm short on Lumber for the second road. This puzzle is broken."

**Skeptic**: "Let me check independently."

*[3 minutes of resource tracking]*

**Skeptic**: "Confirmed. Both port interpretations lead to impossible builds. The resource math doesn't work. I'm filing this as a puzzle error."

**Newbie**: "I was trying to follow along but I got confused at the port trade. How do I know which port they used?"

**Captain**: "That's supposed to be the puzzle — you deduce the port type from the resource counts. But the math is broken, so nobody can solve it. This one needs to go back for revision."

**Speedster**: "Called it. Can we skip it and move on?"

**Captain**: "Yes. We'll mark M2 as BLOCKED and come back if the author fixes it."

---

### Finding 4: Both Speedster and Skeptic independently confirmed M2's resource math bug.

The Speedster found it in 2 minutes through mental arithmetic. The Skeptic confirmed methodically in 3 minutes. Neither could solve the puzzle. The Newbie couldn't even reach the point of failure — they got lost at the port trade step. This puzzle fails all three solver profiles.

### Finding 5: The Speedster wanted to skip M2 immediately, validating the profile.

"Can we skip it and move on?" is textbook Speedster behavior. In a real hunt with the 80% Rule, this is the correct strategy — don't waste time on a broken puzzle.

---

## Risk Puzzle (Minutes 15-30)

**Newbie**: "OK, Risk. I've played this. Six descriptions, six continents. This is like a matching game?"

**Captain**: "Yes. Each description fits one continent. Identify all six, then extract letters."

**Newbie**: "'The fortress. Fewest entry points — just one. Four territories.' ... Is that Australia? Because Australia is the easy one to hold in Risk."

**Captain**: "Keep going."

**Newbie**: "'The crossroads. More entry points than any other. Twelve provinces.' That's Asia. Has to be."

**Skeptic**: "Let me verify the numbers. Australia: 4 territories, 1 entry point (Indonesia→Siam). Correct. Asia: 12 territories, 5 entry points. Correct."

*[Newbie identifies all six continents in 8 minutes]*

**Newbie**: "OK, now the extraction. Landmass I is Australia. Find the territory bordering Asia — that's Indonesia. Third letter of Indonesia... I-N-D... D."

**Skeptic**: "Wait. If Landmass I gives D, and the answer is BORDER (B-O-R-D-E-R), then D is the fourth letter. But we're supposed to read letters in order I through VI. That means the first letter should be B, not D."

**Captain**: "Let me check all six extractions..."

*[5 minutes of verification]*

**Captain**: "The Skeptic is right. The extraction instructions don't produce BORDER. The continent identification is perfect — every description maps correctly. But the territory names and letter indices are wrong. This is the same extraction bug we flagged in the editorial review."

**Skeptic**: "So the PUZZLE works but the EXTRACTION is broken. The author verified the puzzle three times but apparently didn't verify the extraction even once."

**Newbie**: "I identified all six continents though. Does that count for partial credit?"

**Captain**: "In a real hunt, no. But for testing purposes, the continent-matching mechanism is validated. The extraction is the only broken part."

---

### Finding 6: The Newbie solved the continent identification faster than expected.

Board game knowledge directly translated to puzzle-solving ability. The Risk descriptions use language that casual players recognize ("the fortress," "the crossroads"). The Skeptic's numerical properties are embedded but the prose is doing the real work for accessible solvers.

### Finding 7: The Skeptic caught the extraction bug immediately.

The moment the first extracted letter (D) didn't match the first letter of BORDER (B), the Skeptic flagged it. This is exactly what the Skeptic profile is designed to catch — a puzzle that FEELS solvable but produces a wrong answer.

### Finding 8: The Skeptic's preferred numerical approach would not have had this extraction bug.

The extraction failure is in the "prose wrapper" layer, not the structural identification layer. The Skeptic's original proposal (pure numerical identification) would have required different extraction mechanics entirely. The Captain's compromise (embed numbers in prose) worked for the identification step but the author fumbled the extraction step. This partially validates the Skeptic's objection.

---

## Chess Puzzle (Minutes 30-55)

**Speedster**: "Chess. Eight observations. Reconstruct the position. I'm gonna skim these fast... OK, King e1 (never moved, starting square), Rook h1 (never moved, same rank). These are castling conditions. The answer is CASTLE."

**Captain**: "Did you actually solve the puzzle?"

**Speedster**: "I mean, I can see where it's going. The observations are setting up a castling scenario. King and Rook unmoved, f1/g1 empty and unattacked — that's literally the castling checklist. The answer is CASTLE."

**Captain**: "But what about the Black pieces? The whole puzzle is figuring out WHERE everything goes."

**Speedster**: "Does it matter? The answer is obviously CASTLE. The position reconstruction is just... proof of work."

**Skeptic**: "It matters. Let me work through it properly."

*[15 minutes of careful case analysis]*

**Skeptic**: "King e1, Rook h1, Bishop d4, Black King e4, Black Pawn f3. I verified this against all eight observations. The position is unique. Castling is legal. But I have a concern: the puzzle says 'the strongest move available.' Other legal moves exist — Kd1, Kd2, Bd3, etc. Castling may be the strongest positionally, but 'strongest' is subjective. In a real puzzle hunt, I'd submit CASTLE based on the six-letter constraint, but I'd file an errata about the ambiguous phrasing."

**Newbie**: "I got lost at Observation 3. 'Three squares from the corner along this diagonal' — I don't know how to count diagonal squares. Is a1 square zero or square one?"

**Captain**: "Good question. The puzzle should clarify: a1 is the corner (square 0 or square 1?). The author means 3 steps from the corner: a1→b2→c3→d4. So d4 is 3 steps from a1."

**Newbie**: "OK, with that explained, I can follow the rest... sort of. Observation 8 with the mobility count is really hard. I had to draw the board and count squares one by one."

---

### Finding 9: The Speedster identified the answer from the first 2 observations without solving the puzzle.

This is a shortcut vulnerability. A Speedster who recognizes the castling setup from "King unmoved + Rook unmoved + f1/g1 empty" can skip the entire position reconstruction. The answer CASTLE is guessable from the chess context + 6-letter constraint alone. This doesn't necessarily break the puzzle (the Speedster still needs the answer for the meta), but it means the depth of the puzzle is wasted on fast solvers.

**Mitigation**: The puzzle is difficulty 3. It's designed to be deeply satisfying for solvers who do the full reconstruction, and the Speedster shortcut doesn't harm anyone. This is acceptable.

### Finding 10: The Skeptic's "strongest move" objection is valid.

The phrasing is ambiguous. The six-letter constraint narrows it effectively (CASTLE is the only chess move name that's a 6-letter English word — CHECK is 5, GAMBIT is 6 but not a move). But the puzzle should not rely on letter count to disambiguate meaning. Fix the phrasing.

### Finding 11: The Newbie struggled with chess notation and diagonal counting.

Observation 3's "three squares from the corner along this diagonal" assumes familiarity with chess geometry. The Newbie needed help. This is expected for a difficulty-3 puzzle, and the puzzle is NOT the on-ramp puzzle (Codenames serves that role). Acceptable.

---

## Pandemic Puzzle (Minutes 55-80)

**Speedster**: "Pandemic. Discard pile, layer analysis. I see Essen three times — that's a re-infection marker. The bottom of the pile is the oldest layer. Let me find the boundaries..."

*[5 minutes]*

**Speedster**: "Bottom layer: Santiago, Paris, Essen, Riyadh, Algiers, Delhi. All new cities. Middle layer: Atlanta, Essen, Riyadh. Essen and Riyadh are re-infections from the bottom layer. Top layer: Delhi, Atlanta, Essen. More re-infections. First infection order: Santiago, Paris, Essen, Riyadh, Algiers, Delhi, Atlanta."

**Newbie**: "How do you know where the layers start and end?"

**Speedster**: "Because of Intensify. When an Epidemic happens, the discard pile gets reshuffled on top of the draw deck. So cities from earlier layers reappear in later layers. When you see a city that already appeared lower in the pile, you know a boundary happened between them."

**Newbie**: "Oh. That's actually really clever. The pile has layers like... geology?"

**Speedster**: "Exactly. OK, first letters: S-P-E-R-A-D... wait. That's SPERAD. Not SPREAD."

**Skeptic**: "I got the same thing. The bottom-to-top order of the bottom layer has Essen at position 10 and Riyadh at position 9. Since 10 came before 9 (position 10 is older = drawn earlier), Essen was infected before Riyadh. E before R. That gives S-P-E-R-A-D, not S-P-R-E-A-D."

**Captain**: "The extraction order is wrong. The puzzle needs Riyadh before Essen in the bottom layer."

**Skeptic**: "Also, I have a mechanical concern. The Epidemic Infect step draws the BOTTOM card and places 3 cubes. But the puzzle says 'each city's cube count equals appearances.' If an Epidemic Infect target is in this pile, it would have 3 extra cubes from the Infect step. The puzzle quietly assumes the Infect targets were OTHER cities not shown. That's technically valid but the solver has to make an unstated assumption."

**Newbie**: "I understood the layer concept though. That was the aha for me. Even if the letters are wrong, the archaeology metaphor really worked."

---

### Finding 12: The "discard pile as stratigraphy" aha landed with all three solvers.

The Newbie independently called it "geology" — the same metaphor the Social used. The Speedster grasped it immediately. The Skeptic verified it mechanically. This is the strongest aha in the hunt.

### Finding 13: Both Speedster and Skeptic independently found the SPERAD bug.

The extraction order is objectively wrong. Neither solver could produce SPREAD. Both identified the specific cause (Essen before Riyadh in the bottom layer). The fix is a simple position swap.

### Finding 14: The Skeptic flagged the Epidemic Infect hand-wave.

This is a real mechanical inconsistency. A Pandemic expert (the Skeptic's role) would notice that the cube-count = appearance-count simplification doesn't hold if any Epidemic Infect target is in the pile. The puzzle needs an explicit statement: "The two Epidemic Infect targets were cities of different colors not shown in this pile."

---

## Meta (Minutes 80-95)

The team has 3 confirmed answers (CASTLE, BORDER, CIPHER) and 2 blocked answers (TRADE — puzzle broken, SPREAD — extraction wrong). For testing, the Captain supplies all 5 answers.

**Captain**: "The meta is called 'The Sixth Game.' Five questions, each about one board game. The answer to each question is a number. Use that number as an index into the corresponding feeder answer."

**Newbie**: "So Question 1 is about Pandemic: 'How many research stations at the start?' One. And the Pandemic answer was SPREAD. First letter of SPREAD is S."

**Captain**: "Right."

**Speedster**: *[already working ahead]* "Codenames assassin = 1, CIPHER[1] = C. Settlers ore for city = 3, TRADE[3] = A. Risk Africa entry points = 3, BORDER[3] = R. Chess piece types = 6, CASTLE[6] = E. S-C-A-R-E. SCARE."

**Skeptic**: "Wait. 'How many types of chess piece?' The standard answer is 6 (King, Queen, Rook, Bishop, Knight, Pawn). But some people count the Pawn separately from 'pieces' — in some traditions, pawns are not 'pieces,' they're 'pawns.' If the answer is 5, CASTLE[5] = L, and the meta answer is... SCARL? Not a word. So 6 is correct by elimination."

**Captain**: "Good catch. The question should say 'types of chess piece, including pawns' to close that loophole."

**Newbie**: "SCARE. The Host was scared the whole time. That's... actually kind of sweet? The anxious host, worried nobody was having fun, and the answer to the whole hunt is that it was all a SCARE."

**Speedster**: "The meta is clean. Five trivial game-knowledge questions, indexed extraction, short answer. Under 5 minutes."

**Skeptic**: "I have one more concern. The 80% rule. If I'm missing TRADE (M2 was broken), my meta answer is S_ARE. That could be SCARE, SHARE, SNARE, SPARE, STARE. Five candidates. I'd guess SCARE from the Host's personality, but it's not unique without the A from TRADE."

**Captain**: "Noted. The 80% rule is partially satisfied. Three of five missing-letter cases resolve uniquely. Two are ambiguous. For a 5-puzzle hunt, that's acceptable — the meta is intentionally tight."

---

### Finding 15: The meta works mechanically.

All five indexed extractions produce the correct letters. The game-trivia questions are fair and answerable from the world data. SCARE is a valid, thematic meta answer.

### Finding 16: The Newbie understood the meta extraction on first try.

The index-into-word mechanic is simple enough for a first-timer. The game-trivia questions leverage the same board-game knowledge the feeders required. No new skills needed.

### Finding 17: The Skeptic found a real edge case in the chess question.

"Types of chess piece" is ambiguous if you distinguish pieces from pawns. The question needs tightening. This is a P2 fix, not a P1 — the six-letter word constraint eliminates the wrong interpretation.

### Finding 18: The 80% rule is borderline.

With one answer missing, the meta is solvable in 3 of 5 cases (missing S, R, or E leaves a unique word). Missing A or C creates ambiguity. For a 5-puzzle hunt, this is the expected tradeoff — the meta is tight by design, and the Host's personality provides thematic disambiguation.

### Finding 19: The Speedster solved the meta in under 2 minutes.

All five questions are game trivia with well-known answers. There is no puzzle in the meta — only extraction. The meta functions as a capstone, not a challenge. This is appropriate for a hunt targeting difficulty 2-3.

---

## Full Hunt Timing

| Puzzle | Time | Solver(s) | Notes |
|--------|------|-----------|-------|
| M5 Codenames | 3 min | Speedster (30s), Newbie (2.5 min) | Fastest. Natural on-ramp. |
| M2 Settlers | 5 min (abandoned) | Speedster + Skeptic | Broken. Identified bug, moved on. |
| M3 Risk | 15 min | Newbie (identification), Skeptic (extraction bug found) | Identification: solved. Extraction: broken. |
| M1 Chess | 25 min | Skeptic (full solve), Speedster (shortcut) | Strongest puzzle. Full solve takes time. |
| M4 Pandemic | 20 min | All three | Best aha. Extraction order wrong. |
| META | 5 min | Speedster (2 min), Newbie (5 min) | Clean. Works. |

**Total: ~73 minutes** (excluding blocked M2). Target was 2-3 hours. The abbreviated solve format (experienced solver team, no wrong turns) naturally runs faster. A casual solver team would take longer.

---

## Summary of Findings

| # | Finding | Severity | Affects |
|---|---------|----------|---------|
| 1 | Newbie understood Host voice and Codenames on-ramp immediately | Positive | G6, G8 |
| 2 | Speedster solved Codenames in 30 seconds (expected) | Neutral | G4 |
| 3 | Skeptic's BOMB/bombe concern is valid but non-blocking | Note | M5 |
| 4 | M2 resource math confirmed broken by 2 solvers | Critical | M2 |
| 5 | Speedster skip behavior validates profile | Neutral | G4 |
| 6 | Newbie solved Risk identification faster than expected | Positive | G8 |
| 7 | Skeptic caught M3 extraction bug immediately | Critical | M3 |
| 8 | Skeptic's numerical approach partially validated | Note | G3 |
| 9 | Speedster shortcutted Chess from 2 observations | Note | M1 |
| 10 | "Strongest move" phrasing is ambiguous | Moderate | M1 |
| 11 | Newbie struggled with chess diagonal counting | Expected | M1 |
| 12 | Pandemic stratigraphy aha landed with all solvers | Positive | M4 |
| 13 | SPERAD bug confirmed by 2 solvers | Critical | M4 |
| 14 | Epidemic Infect hand-wave flagged | Moderate | M4 |
| 15 | Meta works mechanically | Positive | G5 |
| 16 | Newbie understood meta extraction | Positive | G8 |
| 17 | Skeptic found chess piece/pawn ambiguity | Moderate | META |
| 18 | 80% rule borderline for this meta | Note | META |
| 19 | Speedster solved meta in 2 minutes | Neutral | META |
