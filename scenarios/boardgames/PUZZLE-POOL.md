# Puzzle Pool — Game Night

**Stage 4: PUZZLE POOL**

2-3 candidates per game. Starred (★) = recommended pick per slot. No ranking — the panel does that.

---

## ♟ Chess (M1 — The Professor)

### C1. "Reconstructed" ★

**Pitch**: Reconstruct a chess position from a set of overlapping clues, then identify the only legal move — which spells the answer.

**Mechanism**: 8 clues describe constraints on piece placement ("a bishop controls the long diagonal," "the king is on a light square adjacent to the e-file," etc.). Each clue eliminates positions. The unique valid arrangement has exactly one legal move — the notation of that move, combined with other extracted letters, yields the answer word.

**World data used**: Piece movements, board geometry, algebraic notation.

**Aha**: The position is determined — there's only one arrangement satisfying all clues simultaneously (interlock between clues).

**Principles satisfied**: Riven Standard (this IS chess reasoning), Interlock (clues constrain each other), No Over-Scaffolding (no pre-drawn board provided), Blame the Player (all clues are fair).

---

### C2. "Opening Night"

**Pitch**: Identify 5 chess openings from their first 4 moves (given in scrambled order within each opening), then use the opening names to extract letters.

**Mechanism**: Each opening's 8 half-moves (4 White, 4 Black) are presented scrambled. The solver must reconstruct the correct move order, identify the opening, then extract indexed letters from the opening name.

**World data used**: Famous openings (first 4 moves), opening names, algebraic notation.

**Aha**: Recognizing which move can only be first (e.g., e4 must precede Nf3 in certain openings because of pawn structure).

**Principles satisfied**: Riven Standard (knowing openings IS chess knowledge), Reading Reward (must engage deeply with move logic).

---

### C3. "Checkmate in What"

**Pitch**: Match 6 checkmate positions to their named patterns, then extract from the pattern names.

**Mechanism**: Six board diagrams show final positions. The solver identifies each named checkmate pattern (Fool's Mate, Smothered Mate, Boden's Mate, etc.). Indexed letters from the pattern names spell the answer.

**World data used**: Checkmate patterns, piece movements, board geometry.

**Aha**: Recognizing the patterns — some are obvious, some require knowing the specific name (e.g., "Epaulette Mate" vs. generic queen checkmate).

**Principles satisfied**: Solving = Proving Understanding (must know the pattern names, not just see the checkmate).

---

## ⬡ Settlers of Catan (M2 — The Speedrunner)

### S1. "Market Day" ★

**Pitch**: Given a specific hand of resource cards and a sequence of trades/builds, determine what the player built last — which names the answer.

**Mechanism**: The solver starts with a known hand (e.g., "3 Brick, 2 Lumber, 1 Ore, 2 Grain, 3 Wool"). A sequence of 4-5 actions is described ("Built a road. Traded 3 Wool for 1 Ore at the port. Built a settlement. Drew a development card..."). After tracking resources through each action, the solver determines what the player could have built with their remaining cards. The name of that structure (or the VP card drawn) is the answer.

**World data used**: Building costs, trading ratios (port trades), resource cards, development card types.

**Aha**: The resource math is tight — only one build is possible with the remaining cards, and it requires tracking every transaction precisely.

**Principles satisfied**: Riven Standard (this IS Settlers resource management), Interlock (each transaction depends on the previous), No Computation Without Deduction (one step requires inferring a missing piece of information — which port the player used).

---

### S2. "Probability Isle"

**Pitch**: A board position is described. Determine which settlement produces the most expected resources per turn.

**Mechanism**: Three settlement positions are described, each bordering 3 hexes with known number tokens. The solver calculates expected resource production per turn using dice probabilities, then identifies the position with the highest expected value. The hex terrain types at that position spell the answer through their first letters.

**World data used**: Number token probabilities, hex terrain types, resource production rules.

**Aha**: The solver must realize that raw probability isn't enough — they need to consider which resources are most valuable given the current build goals (deduction layer on top of computation).

**Principles satisfied**: No Computation Without Deduction (adds strategic layer beyond pure math).

---

### S3. "The Longest Road"

**Pitch**: Given a partial road network, determine the minimum number of road segments needed to claim Longest Road — the count encodes the answer.

**Mechanism**: A hex grid shows roads from three players. The solver must trace each player's longest continuous path (remembering that opponent settlements break continuity), determine which player is closest to Longest Road (5+ segments), and calculate the exact roads needed. Combined with other extracted data, this yields the answer.

**World data used**: Longest Road rules, road continuity, settlement blocking.

**Aha**: One player's road looks longer but is broken by an opponent's settlement — the actual longest road belongs to someone unexpected.

**Principles satisfied**: Blame the Player (the settlement break is visible but easy to miss).

---

## 🎲 Risk (M3 — The Rebel)

### R1. "Unnamed Continents" ★

**Pitch**: Six continents are described by their properties — territory count, bonus, number of entry points, and adjacency clues — without being named. Identify each, then extract from territory names.

**Mechanism**: Six cryptic descriptions, each fitting exactly one continent. ("I have the fewest entry points of any continent. My bonus matches another continent with more than twice my territories.") Once identified, the solver finds a specific territory in each continent matching an additional clue ("my territory that borders two continents") and extracts indexed letters from those territory names.

**World data used**: Continent bonuses, territory counts, border territories, cross-continent connections.

**Aha**: The descriptions interlock — identifying one continent constrains which others are possible, because the descriptions reference each other.

**Principles satisfied**: Interlock (descriptions cross-reference), Riven Standard (this IS Risk strategic geography), Blame the Player (all data is in the territory tables).

---

### R2. "Dice Odds"

**Pitch**: A combat scenario is described. The solver must calculate whether the attacker or defender is favored, and the margin determines which territory is the answer.

**Mechanism**: Three battles are described with specific army counts. For each, the solver determines the favored side and the approximate probability. The probabilities, rounded and indexed into a territory list, spell the answer.

**World data used**: Combat rules, dice probabilities, territory names.

**Aha**: One battle is a trap — it looks attacker-favored but the army distribution means the defender should win.

**Principles satisfied**: No Computation Without Deduction (the trap scenario requires strategic insight, not just calculation).

---

### R3. "Supply Lines"

**Pitch**: Given a set of controlled territories, determine the minimum number of fortification moves needed to create an unbroken chain from Alaska to South Africa.

**Mechanism**: A player's territory list is given. The solver must find the path through connected territories (consulting the adjacency data), identify the gaps, and determine which uncontrolled territories need to be captured to complete the chain. The initials of the gap territories spell the answer.

**World data used**: Territory adjacency, cross-continent connections, fortification rules.

**Aha**: The obvious land route doesn't work — the solver must use the Alaska-Kamchatka ocean connection to find the shortest chain.

**Principles satisfied**: Surprise the Answer (the path goes through Asia, not through Europe/Africa as expected).

---

## 🦠 Pandemic (M4 — The Collaborator)

### P1. "Patient Zero" ★

**Pitch**: Given the current disease state of 8 cities (cube counts), the infection discard pile, and the knowledge that exactly 2 epidemics have occurred, trace the infection history backward to determine which city was infected first.

**Mechanism**: The solver works backward from the current board state. The infection discard pile order + the Intensify mechanic (reshuffle and place on top) creates layers. By analyzing which cities have 3 cubes (outbreak candidates), which have been re-infected (post-Intensify), and the epidemic rule (bottom card = 3 cubes), the solver deduces the original infection order. The first city infected — Patient Zero — is the answer.

**World data used**: Infection rules, epidemic mechanics (especially Intensify), outbreak chains, city list by disease color.

**Aha**: The Intensify mechanic means the discard pile has a stratigraphy — recent infections sit on top, creating recoverable layers.

**Principles satisfied**: Riven Standard (this IS how Pandemic's infection deck works), Solving = Proving Understanding (must genuinely understand the epidemic shuffle), One Aha (the layered discard pile), Dinner Party Test ("I forensically reconstructed a disease outbreak").

---

### P2. "Role Call"

**Pitch**: Given a game state and the actions taken in one turn (but not the role), determine which role the player must have been to make those actions legal.

**Mechanism**: A 4-action turn is described. Some actions are only legal for specific roles (e.g., treating all cubes = Medic, curing with 4 cards = Scientist, giving a non-matching city card = Researcher). The solver must check each action against each role's abilities to find the unique role that makes the entire turn legal. The role name (or an indexed letter from it) is part of the answer.

**World data used**: Role abilities, action rules, cure mechanics, share knowledge rules.

**Aha**: Two of the four actions seem legal for any role — it's the combination of all four that narrows it to exactly one.

**Principles satisfied**: Interlock (must check all 4 actions together), Blame the Player (the constraining action is subtle but fair).

---

### P3. "Outbreak Chain"

**Pitch**: Given a map snippet with cube counts and a single infection card draw, determine how many outbreaks occur and which cities are affected.

**Mechanism**: A cluster of 6 connected cities is shown with their current cube counts (several at 3). Drawing one infection card triggers a chain outbreak. The solver must trace the chain correctly (remembering: each city outbreaks only once per chain, cubes spread to ALL neighbors). The number of outbreaks and the affected cities encode the answer.

**World data used**: Outbreak rules, chain outbreak mechanics, city connections.

**Aha**: The chain loops back but the "once per chain" rule prevents infinite cascading — solvers who forget this rule get the wrong count.

**Principles satisfied**: Blame the Player (the once-per-chain rule is in the world data), No Over-Scaffolding (no step-by-step chain-tracing worksheet).

---

## 🔍 Codenames (M5 — The Silent One)

### N1. "Spymaster" ★

**Pitch**: You ARE the spymaster. Given a 5x5 grid of words and a key card, find the single best one-word clue that connects exactly 3 of your team's words while avoiding the assassin.

**Mechanism**: A full 25-word grid is presented with the key card. The solver must find a one-word clue that links exactly 3 of their 9 team words (marked on the key) while being completely unambiguous — the 3 connected words must be clearly the best matches, and the clue must NOT relate to the assassin word. The clue word itself IS the answer.

**World data used**: Clue rules (one word, relates to meaning), grid layout, key card, assassin avoidance, team word identification.

**Aha**: The clue that connects the right 3 words — the moment the solver sees the unexpected thematic link between seemingly unrelated words.

**Principles satisfied**: Riven Standard (this IS what spymastering feels like), Dinner Party Test ("I was the Codenames spymaster and the answer was my clue"), Surprise the Answer (the connecting word is non-obvious), One Aha (finding the link).

---

### N2. "Operative"

**Pitch**: You're the operative. Given a sequence of 3 clues (word + number) from your spymaster, deduce which 9 words on the grid are yours.

**Mechanism**: A 25-word grid is shown. Three clues are given (e.g., "OCEAN, 3", "SHARP, 2", "VINTAGE, 4"). The solver must figure out which grid words each clue points to, identify all 9 team words, then extract from the non-team words (bystanders + opponents) using a pattern.

**World data used**: Clue interpretation rules, number = count, grid layout.

**Aha**: One of the clues has an unexpected interpretation — "SHARP" doesn't mean pointy objects, it means musical sharps (connecting PIANO, NOTE, KEY).

**Principles satisfied**: Solving = Proving Understanding (must think like an operative), Surprise the Answer (the misdirecting clue).

---

### N3. "Assassin's Grid"

**Pitch**: Five partial key cards are shown, each missing one cell. The solver determines where the assassin is on each card, then the assassin positions across all 5 cards encode the answer.

**Mechanism**: Each key card has 24 of 25 cells filled (9 blue, 8 red, 7 bystander, 1 assassin — but one cell is blank). The solver uses the count constraints (exactly 9/8/7/1) to determine the missing cell's type. On at least 3 of the 5 cards, the missing cell must be the assassin. The grid positions of the assassins spell a word when read as coordinates.

**World data used**: Card type counts (9/8/7/1), key card structure.

**Aha**: The count constraint uniquely determines the missing cell's type — it's arithmetic, but the extraction through grid coordinates is the puzzle layer.

**Principles satisfied**: Interlock (must count across all cards), No Computation Without Deduction (must figure out the coordinate-to-letter extraction).

---

## Pool Summary

| Slot | Candidates | Starred pick |
|------|-----------|--------------|
| ♟ Chess | C1 Reconstructed, C2 Opening Night, C3 Checkmate in What | ★ C1 |
| ⬡ Settlers | S1 Market Day, S2 Probability Isle, S3 The Longest Road | ★ S1 |
| 🎲 Risk | R1 Unnamed Continents, R2 Dice Odds, R3 Supply Lines | ★ R1 |
| 🦠 Pandemic | P1 Patient Zero, P2 Role Call, P3 Outbreak Chain | ★ P1 |
| 🔍 Codenames | N1 Spymaster, N2 Operative, N3 Assassin's Grid | ★ N1 |

**Total candidates: 15** (3 per game).

---

## Principle Compliance Notes

| Principle | How the starred picks address it |
|-----------|--------------------------------|
| Riven Standard | Each puzzle IS its game — chess reasoning, resource tracking, geography, epidemiology, word linking |
| Solving = Proving Understanding | Solvers demonstrate real game knowledge to reach the answer |
| Dinner Party Test | P1 and N1 are strong candidates ("I traced a disease outbreak backward" / "I was the spymaster") |
| No Over-Scaffolding | None of the starred picks include step-by-step worksheets |
| One Aha | Each has a single clear insight moment |
| Interlock | C1, S1, R1, P1 all have clues that constrain each other |
| No Computation Without Deduction | S1 and R1 add deduction layers on top of any math |
| Blame the Player | All clues are fair and verifiable against world data |
| No Deliberate Errors | All game facts are sourced from world/ data files |
