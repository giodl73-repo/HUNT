# Codenames — World Data

Source material for Module M5 (The Silent One).

---

## Grid Layout

- **5 x 5 grid** of word cards = 25 words
- Words are common English nouns (single words, not phrases)
- Cards laid out face-up — all words visible to everyone

```
┌────────┬────────┬────────┬────────┬────────┐
│ WORD 1 │ WORD 2 │ WORD 3 │ WORD 4 │ WORD 5 │
├────────┼────────┼────────┼────────┼────────┤
│ WORD 6 │ WORD 7 │ WORD 8 │ WORD 9 │ WORD10 │
├────────┼────────┼────────┼────────┼────────┤
│ WORD11 │ WORD12 │ WORD13 │ WORD14 │ WORD15 │
├────────┼────────┼────────┼────────┼────────┤
│ WORD16 │ WORD17 │ WORD18 │ WORD19 │ WORD20 │
├────────┼────────┼────────┼────────┼────────┤
│ WORD21 │ WORD22 │ WORD23 │ WORD24 │ WORD25 │
└────────┴────────┴────────┴────────┴────────┘
```

---

## Card Types

| Type | Color | Count (starting team) | Count (other team) |
|------|-------|----------------------|-------------------|
| **Starting team's agents** | Blue or Red | 9 | 8 |
| **Other team's agents** | Red or Blue | 8 | 9 |
| **Innocent bystanders** | Beige/tan | 7 | 7 |
| **Assassin** | Black | 1 | 1 |
| **Total** | — | **25** | **25** |

- The team that goes **first** has **9 agents** to find (one more than the other team's 8)
- This compensates for the first-move advantage

---

## Roles

### Spymaster (1 per team)

- Sees the **key card** — a 5x5 grid showing which words are which color
- Gives **one clue per turn**: a single word + a number
- The word relates to one or more of their team's words on the grid
- The number tells how many grid words the clue relates to
- **Cannot** point, gesture, or give any non-verbal hints
- Must keep a poker face regardless of operatives' guesses

### Operative(s) (rest of the team)

- See only the 25 words — no key card
- Hear the spymaster's clue (word + number)
- Discuss among themselves and point to words to guess
- Touch a word card to lock in a guess

---

## Clue Rules

### The Clue Format

```
CLUE: [one word], [number]
```

Example: "OCEAN, 3" — the spymaster thinks 3 of their team's words relate to "ocean."

### Legal Clues

| Rule | Detail |
|------|--------|
| Must be **one word** | No phrases, no hyphenated words (house rules vary) |
| Must be **in English** | No foreign words (unless commonly used in English) |
| Must relate to **meaning** | Not spelling, letter patterns, position on table, or rhyming |
| Number must be ≥ 0 | 0 and "unlimited" are legal (see special numbers below) |
| Cannot be any word **on the grid** | Even if it's covered; partial matches also banned |
| Cannot be a **different form** of a grid word | If SHIP is on grid, cannot say SHIPPING or SHIPPED |

### Special Numbers

| Number | Meaning |
|--------|---------|
| **0** | "None of our words relate to this clue — but I want to give you a hint to avoid something." Operatives still get 1 guess. |
| **Unlimited (∞)** | "This clue relates to some of our words." Operatives may guess until they get one wrong. Used to sweep up previous clue leftovers. |

---

## Guessing Rules

After hearing the clue:

1. Operatives **must make at least 1 guess** (unless they choose to pass — house rule varies)
2. Maximum guesses = **number + 1** (e.g., clue "OCEAN, 3" → up to 4 guesses)
3. After each guess, the card is revealed:
   - **Own team's agent** → card covered with team color, may continue guessing
   - **Innocent bystander** → card covered with beige, turn ends
   - **Other team's agent** → card covered with other color, turn ends (you helped them)
   - **Assassin** → **your team instantly loses the game**
4. Operatives may choose to **stop guessing** before using all their allowed guesses

---

## Turn Structure

```
Team A's turn:
  1. Spymaster gives clue (word + number)
  2. Operatives discuss
  3. Operatives guess one word at a time
  4. Each guess is resolved immediately
  5. Turn ends when:
     - Wrong guess (bystander or opponent's agent)
     - Assassin hit (game over)
     - All allowed guesses used
     - Operatives choose to stop

Team B's turn:
  (same structure)

Repeat until one team wins or assassin ends the game.
```

---

## Win Conditions

| Condition | Result |
|-----------|--------|
| All of your team's agents identified | **Your team wins** |
| Opponent guesses all their agents (including on your turn) | **Opponent wins** |
| Your operative touches the Assassin | **Your team loses immediately** |

---

## The Key Card

- A 5x5 grid with colored squares matching the word grid
- Shows which positions are: Blue, Red, Beige (bystander), Black (assassin)
- Only spymasters see this card
- A colored border indicates which team goes first
- The key card is symmetric — can be read from either side of the table (spymasters sit across from each other in 2-spymaster variants)

```
Example Key Card (Blue goes first):
┌───┬───┬───┬───┬───┐
│ B │ R │ . │ B │ B │     B = Blue team agent
├───┼───┼───┼───┼───┤     R = Red team agent
│ . │ B │ R │ . │ R │     . = Innocent bystander
├───┼───┼───┼───┼───┤     X = Assassin
│ R │ B │ X │ R │ B │
├───┼───┼───┼───┼───┤
│ R │ . │ B │ . │ R │
├───┼───┼───┼───┼───┤
│ B │ R │ . │ B │ R │
└───┴───┴───┴───┴───┘
Blue: 9  Red: 8  Bystander: 7  Assassin: 1
```

---

## Spymaster Strategy (for puzzle design)

What makes spymastering hard:

| Challenge | Detail |
|-----------|--------|
| **Connecting disparate words** | Your 9 words may have nothing in common — finding creative links is the core skill |
| **Avoiding the assassin** | If your clue accidentally points to the assassin, your team loses |
| **Avoiding opponent's words** | A clue that links 3 of yours but also 1 of theirs is risky |
| **Balancing breadth vs precision** | "WATER, 4" links many words but is vague; "FJORD, 1" is precise but slow |
| **Reading your team** | Will they interpret your clue the way you intended? |

### What Makes a Great Clue

A great clue:
1. Links **multiple** team words (high number = efficient)
2. Links **zero** opponent words, bystanders, or the assassin
3. Is **unambiguous** to the operatives — the intended connection is clear
4. Is **surprising** — the operatives feel clever when they see the connection

---

## Component Counts

| Component | Count |
|-----------|-------|
| Word cards (double-sided) | 200 cards = 400 words |
| Key cards | 40 (double-sided = 80 configurations) |
| Agent cards (Blue) | 8 + 1 double agent = 9 |
| Agent cards (Red) | 8 |
| Innocent bystander cards | 7 |
| Assassin card | 1 |
| Rulebook | 1 |

---

## Codenames Variants (for reference)

| Variant | Change |
|---------|--------|
| **Codenames Duet** | 2-player cooperative version; both players are spymasters for each other |
| **Codenames Pictures** | Grid of images instead of words |
| **Codenames Deep Undercover** | Adult words |
| **Codenames XXL** | Larger cards (same game) |
