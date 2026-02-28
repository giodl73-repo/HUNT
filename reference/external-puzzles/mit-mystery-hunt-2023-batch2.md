# Real Puzzles — Batch 2
**Source:** MIT Mystery Hunt 2023 (Interesting Things Museum)
**Fetched:** 2026-02-28

---

## Bridge Building
**Source:** MIT Mystery Hunt 2023
**URL:** https://puzzles.mit.edu/2023/interestingthings.museum/puzzles/bridge-building
**Answer:** PROTEINPOWDER
**Type:** Hashiwokakero (bridge-building logic puzzle) with chemistry extraction — solve the island-bridge grid, then read the resulting node-and-connection pattern as a molecular structure (polymer backbone) to identify the answer

---

**Flavor text / instructions:**

The puzzle presents a Hashiwokakero (Hashi) logic grid — a puzzle genre in which numbered islands must be connected by bridges such that each island has exactly as many bridges as its number and all islands form a single connected group. The title "Bridge Building" directly clues the genre.

The grid nodes carry (+) and (−) superscripts representing ionic charges. After solving the bridge-building logic puzzle, the resulting pattern is read as a chemical polymer backbone: each node represents an atom (hydrogen ignored), and the charges disambiguate the identity of each component. The backbone structure, traced through the solved grid, spells out or identifies a specific polymer — ultimately yielding the answer PROTEINPOWDER.

**Canned hints (decoded):**

- "The genre of this logic puzzle is clued by the title. What kind of logic is it that asks you to build bridges?"
- "The 'backbone' from flavortext relates to numbers not exceeding 4 in the logic puzzle. What else has maximum 4 connections with (+) and (−) superscripts?"
- "The (+) and (−) superscripts represent charges. What has charges and maximum 4 connections?"
- "Each 'node' represents an atom (ignoring hydrogen). Think about bond types needed for the polymer backbone, then determine the extraction mechanism. Charges disambiguate components."

---

## Natural Transformation
**Source:** MIT Mystery Hunt 2023
**URL:** https://puzzles.mit.edu/2023/interestingthings.museum/puzzles/natural-transformation
**Answer:** TANGRAM
**Type:** Logical deduction puzzle — reverse-engineer transformation rules from a set of example diagrams (arrows between images/word-circles), then apply those rules to subsequent diagrams to extract the answer

---

**Flavor text / instructions:**

"Each arrow represents a transformation of the words. The first group of diagrams will help you determine what each arrow means."

The puzzle presents six interconnected diagrams. Each diagram contains images and word-circles connected by labeled arrows. The first group of diagrams serves as a key: by analyzing the relationships between the connected items, solvers must determine what rule each arrow type encodes.

Arrow types include: a "derivative" arrow (using another meaning of the word derivative), a "co-arrow," an "animal collective noun arrow," and an "element of / type of arrow." Once all arrow meanings are determined from the key diagrams, solvers apply the rules to the remaining diagrams. The extraction step involves a distinction between "gram" and "g" and a mathematical expression that sounds like a word.

**Canned hints (decoded):**

- Focus on the first group of diagrams to determine arrow definitions.
- Try solving some of the later diagrams first to deduce arrow meanings through constraint propagation.
- One arrow uses "another meaning of the word derivative."
- The extraction references a distinction between "gram" versus "g" and a "mathematical expression" that sounds like a word.

---

## Extrasensory
**Source:** MIT Mystery Hunt 2023
**URL:** https://puzzles.mit.edu/2023/interestingthings.museum/puzzles/extrasensory
**Answer:** DEODORANT
**Type:** Crossword with letter extraction — fill the themed crossword, then identify and extract letters that don't belong in order to read the final answer

---

**Flavor text / instructions:**

The puzzle is a 20-clue crossword. Clues are presented in a reordered sequence (not 1–20 top to bottom); the grid numbering order given is: 19, 7, 11, 4, 17, 6, 1, 10, 9, 2, 8, 18, 15, 20, 16, 12, 3, 5, 14, 13.

**Clues (numbered as presented in the puzzle):**

1. A popular course at MIT is an introduction to these
2. One will knock you out
3. Paradigm that looks to address cross-cutting concerns
4. Area of achievement for JFK, Gandhi, Joan of Arc, and Bonnie and Clyde
5. Key part of a crane or elevator
6. Houdini, for example
7. Tract that makes you a torus
8. Larger body led by the Nasi
9. Like an experience you might have after consuming some fungi
10. It had five movies and several series even though there can only be one
11. Object of ridicule
12. It's represented by the left side of a certain compass
13. Nice miss
14. It's often called the language of the universe
15. What we are doing to privilege
16. It might be called Course 9-5 at MIT
17. Male who looks off-duty, but is not
18. Reward for being the "first loser"
19. Type of Western
20. It's often overshadowed by Christmas

---

## A Trip to the Museum
**Source:** MIT Mystery Hunt 2023
**URL:** https://puzzles.mit.edu/2023/interestingthings.museum/puzzles/a-trip-to-the-museum
**Answer:** REPLICA
**Type:** Multi-part physical/museum interaction puzzle — five sub-puzzles tied to specific MIT Museum exhibits (Tracing Threads, Gene Cultures, Essential MIT), each yielding a one-word answer; the five answers form a final clue phrase

---

**Flavor text / instructions:**

"Once you have your tickets, go to the MIT Museum! Each of the subparts of this puzzle are related to an exhibit on the second floor. Part 4 is in Essential MIT, parts 2, 3, and 5 are in Gene Cultures, and part 1 is in Tracing Threads."

The puzzle requires physical presence at the MIT Museum. Five sub-puzzles are embedded in or reference real museum exhibits:

- **Part 1 (Tracing Threads):** Involves the loom exhibit; solvers must "turn the loom" to find missing information.
- **Part 2 (Gene Cultures):** Involves finding a specific leaf from the Blue Flower biotechnology exhibit, matching an outline, then solving a logic puzzle on triangulated cells. Shaded cells form letters.
- **Part 3 (Gene Cultures):** References The Last Supper exhibit. Solvers count additions to the depicted meal scene.
- **Part 4 (Essential MIT):** Involves an animation of a starshade unfolding. Solvers track where labeled letters end up after the unfolding process.
- **Part 5 (Gene Cultures):** A single exhibit presents 21 "exhibits" described in ways resembling names of other things. Each of 21 clues provides a partial ordering on three "exhibits"; solvers reconstruct the full sequence, then trace the path taken through each "gallery" to extract letters.

Each sub-puzzle yields a one-word answer. The five answers together form a final clue phrase whose answer is REPLICA.

**Canned hints (decoded):**

- "Do you notice anything missing from the given images? You should turn the loom in Tracing Threads."
- "You will need to find a leaf from Biotechnology from the Blue Flower in Gene Cultures which looks like the given outline. This leaf has been triangulated into cells which you need to do the logic puzzle."
- "The shaded cells should look like a couple of letters."
- "The Last Supper is a piece about a meal. In Gene Cultures, you will need to find an exhibit about a meal being served. Make sure to count the additions to The Last Supper!"
- "In Essential MIT, you will see an animation of a starshade unfolding. Where would the letters on the puzzle animation end up after following this process?"
- "This subpart utilizes a single exhibit in Gene Cultures... Each of the 21 'exhibits' is described in a way that seems similar to the name of something else. Each of the clues gives a partial ordering on three of the 'exhibits,' which you will need to use to order the entire sequence."
- "Trace out the path taken for each 'gallery' to get letters of the answer to this subpart!"
- "Each subpuzzle yields a one-word answer. Together, these 5 answers form a final cluephrase."

---

## Street Smarts
**Source:** MIT Mystery Hunt 2023
**URL:** https://puzzles.mit.edu/2023/interestingthings.museum/puzzles/street-smarts
**Answer:** ROADTOHELL
**Type:** Physical location-based scavenger hunt — find 29 named plaques/benches/trees on Boston's Commonwealth Avenue Mall matching the clues, use street-block indexing to extract letters

---

**Flavor text / instructions:**

"You decide to track down the Museum's anonymous donors out on the streets. Once you find all of the donors, you can make your way back."

"This puzzle requires physical presence. You may need to move leaves or snow."

Solvers must physically visit Commonwealth Avenue Mall in Boston. The Mall runs from Arlington Street through Hereford Street, divided into blocks by cross streets. Each of the 29 clues identifies a specific named plaque, bench, or labeled tree along the Mall. The street block location (Arlington, Berkeley, Clarendon, Dartmouth, Exeter, Fairfield, Gloucester, Hereford) and position index within that block are used to extract one letter per clue, spelling the answer.

**Clues (29 total, organized by block):**

Block 1:
1. He has a large statue and shares both his first and last names with a character recently portrayed by Seth Carr (1/2)
2. Botanical name on a botanical specimen (1/3)
3. Botanical name on a non-botanical specimen (3/3)
4. Honored by one Lee, but the enemy of another (1/2)
5. Name on a labeled tree which is directly across from a labeled bench (1/2)

Block 2:
6. His head is not actually marble (2/2)
7. Name on a plaque containing the name of a Taylor Swift song (3/3)
8. Shares both first and last names with a famous mathematician (2/2)
9. Someone nicknamed "O'B" (3/1)

Block 3:
10. Flanked by an American woman and an Irish woman (3/3)
11. Modern parlance: plaque says five words "YOLO" (2/2)
12. Name directly on a bench under her reverend (3/3)
13. Name on tree next to tree with quote about trees (2/2)

Block 4:
14. A noted white abolitionist (3/3)
15. Someone sponsored by his family fund (2/2)
16. Someone with two benches bearing their name (3/3)
17. Woman sharing name with head on copper hated by copperheads (1/3)

Block 5:
18. Judge with two daughters (3/3)
19. Neighborhood champion and defender (3/3)
20. Small grandchildren next to bench in person's honor (4/4)
21. They had advice for readers and writers (3/2)

Block 6:
22. A gentleman scientist (3/3)
23. Nicknamed "Buzz" (3/3)
24. Quoted on plaque dedicated to Margaret (2/1)
25. The only one who is standing (1/2)

Block 7:
26. Born near San Juan (1/3)
27. Earlier born of two exactly 71 years old (2/1)
28. He and his mother remembered close together (2/3)
29. Later born of two who saw exactly 71 years (2/2)

---

## Dropypasta
**Source:** MIT Mystery Hunt 2023
**URL:** https://puzzles.mit.edu/2023/interestingthings.museum/puzzles/dropypasta
**Answer:** VINDICATIONISLAND
**Type:** Dropquote puzzle with character-matching extraction — fill in missing letters from quotes (all from the same genre), then match two characters per stage using stage-specific win conditions to read off extracted letters

---

**Flavor text / instructions:**

"Each row of the dropquote has a single space with a letter missing from the top. All the dropquotes come from the same genre."

The puzzle presents a series of dropquote rows. In a standard dropquote, letters fall from cells above a grid into the grid below to form a quotation. Here, each row has exactly one letter missing from the top; that missing letter is the one to be extracted.

All quotations come from the Super Smash Bros. universe (the shared genre). After filling in the dropquotes and collecting the missing letters, solvers must match two characters per stage. Each named Smash Bros. stage has its own win condition determining which character "wins" that stage and thus contributes the extracted letter to the answer.

**Stage win conditions:**

- **Battlefield:** The character whose corresponding letter from the dropquote comes first alphabetically
- **Dream Land:** The character whose original video game came out earlier
- **Final Destination:** The higher-tier character according to the current tier list
- **Fountain of Dreams:** The character who takes less inhale damage from Kirby
- **Pokemon Stadium:** The character closer to a Pokémon on the character selection screen
- **Yoshi's Story:** The heavier character by weight

---

## You're Telling Me
**Source:** MIT Mystery Hunt 2023
**URL:** https://puzzles.mit.edu/2023/interestingthings.museum/puzzles/youre-telling-me
**Answer:** CARBONSINK
**Type:** "Shrimp Fried Rice" meme compound-word reinterpretation — each clue is a rhetorical question reading a compound word or phrase in a non-standard way; solvers identify the intended compound, then extract letters to reach the environmental-themed answer

---

**Flavor text / instructions:**

The puzzle is based on the "Shrimp Fried Rice" meme format, in which a compound word or phrase is read in a deliberately non-standard way (e.g., "shrimp fried rice" parsed as "shrimp that has been fried with rice" versus "fried rice with shrimp"). Each of the 25 rhetorical-question clues presents one such misreading.

**The 25 clue questions (reproduced in order):**

1. Can a two-dimensional space even contain a gas?
2. Are you telling me you want me to make a timepiece panic?
3. You'd need billions of insects to make something tall enough to show up on a map.
4. WWE seems dangerous enough without any weapons.
5. I mean, unless you're billed by the hour, wouldn't you always want a legal ordeal to be short?
6. Why do you need me to cut some twigs?
7. Is that like a blunt weapon for beating Canada or France up?
8. What's the point of frightening a child?
9. Do you need me to give proper attribution to a piece of paper?
10. Are you making a picture of a flat surface?
11. Are those what you use to drink aqueous humor?
12. How's an insect supposed to get a job now?
13. You mean to say that this firearm shoots appendages?
14. You need some milk taken out, boss?
15. Throwing cutlery around seems too dangerous.
16. Is that where you go into the woods and shoot a crossword?
17. Isn't scaring a reptile animal abuse?
18. I'm pretty sure stones are made of elements too dense for fusion.
19. I didn't think that footwear could move by itself.
20. Are you asking me to record the dirt until something comes out of it?
21. You're telling me there's a scam happening around Wimbledon?
22. Do you need to find out how fast a display is?
23. Do you need me to give instructions to a stop?
24. Is it even possible to wash the lack of something?
25. Are you cleaning a mechanical device?
26. Who would employ a liquid?

**Clue grid structure (26 entries with enumeration and part of speech):**

| Entry | Enumeration | Part of Speech |
|-------|-------------|----------------|
| 1 | (7 5) | verb, present progressive |
| 1 | (3 9) | verb |
| 1 | (3 3) | verb |
| 1 | (4 3) | verb |
| 1 | (4 4) | verb |
| 1 | (3 4) | noun |
| 1 | (6 6) | noun |
| 1 | (6 4) | verb |
| 2 | (4 4) | noun |
| 3 | (4 6) | verb |
| 4 | (7 7) | verb, present progressive |
| 4 | (6 7) | noun |
| 4 | (4 5) | verb |
| 5 | (3 7) | noun |
| 5 | (5 4) | noun |
| 6 | (5 7) | verb |
| 6 | (4 3) | noun |
| 7 | (5 5) | independent clause |
| 7 | (5 4) | verb |
| 8 | (7 4) | noun |
| 8 | (3 5) | verb |
| 8 | (3 5) | noun |
| 10 | (6 4) | noun |
| 11 | (6 5) | verb |
| 11 | (7 5) | noun |

Answers are sorted alphabetically by solution. The final answer, CARBONSINK, is environment-themed: consider what coal is made of (carbon) and what a faucet is part of (sink).

---

## Apples Plus Bananas
**Source:** MIT Mystery Hunt 2023
**URL:** https://puzzles.mit.edu/2023/interestingthings.museum/puzzles/apples-plus-bananas
**Answer:** HARVESTFESTIVAL
**Type:** Cryptarithmetic / emoji substitution puzzle — assign numeric PLU (Price Look-Up) codes to produce emoji so that sums satisfy a primality constraint, then extract letters from produce variety names based on frequency counts

---

**Flavor text / instructions:**

"You've picked a variety of produce. Please scan your items and add them to a bag. Unfortunately, your total cannot be reduced."

"The puzzle is missing enumerations (7 8) at the bottom of the page." [erratum]

The puzzle presents two identical lists of 18 emoji sequences, each featuring various produce items:

🍋 🍌 🥦 🍑 🍆 🍇 🍎 🥑 🥭 🥝 🎃 🍅 🍊 🫐 🍌 🫘

Each emoji represents a produce item with a real-world PLU (Price Look-Up) code — the standardized 4- or 5-digit barcode numbers used in grocery stores. Solvers must assign the correct PLU codes to the produce emoji such that the sum of the two lists satisfies the constraint that "your total cannot be reduced" (i.e., is a prime number).

Once the PLU codes are correctly assigned, letters are extracted from the variety names of the produce based on positional or frequency counts. The 14 blank answer spaces at the bottom of the page (enumeration 7 8) accommodate HARVEST FESTIVAL.
