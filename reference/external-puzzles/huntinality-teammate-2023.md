# Real Puzzle Batch 3 — Huntinality III + Teammate Hunt 2021

Fetched 2026-02-28. Sources: huntinality.com (Huntinality III / "Hunts Upon a Time") and 2021.teammatehunt.com.

---

## Characters
**Source:** Huntinality III ("Hunts Upon a Time"), 2023
**URL:** https://huntinality.com/puzzles/characters
**Answer:** UNHEXING
**Type:** Hexadecimal arithmetic + ASCII extraction — clues yield words made only of letters A–F, which are treated as hex numbers; arithmetic on those hex values produces ASCII character codes that spell the answer.

**Flavor text:** "You encounter some characters in the forest. But to get them to help, you'll need to find their base and then understand their code."

**Puzzle text:**

The puzzle presents eight equations. Each equation uses clues with letter-count indicators (parenthetical numbers) to resolve into words that happen to consist only of letters A–F — i.e., valid hexadecimal strings. The arithmetic is then performed in hexadecimal, and the numeric result is decoded as an ASCII character.

The eight equations (clues and letter counts given; operations are /, ×, −, with standard hex arithmetic):

1. You, according to a game title (4) / Undergrad degree type (2) − Cooling system (2)
2. Speed of light (1) × Singer Sheeran (2) − Highest-ranking card (3)
3. (Visage (4) − Small restaurant (4)) / Hawaiian lava type (2)
4. Taxi (3) / (US org that evaluates meds (3) − Wonderful, for short (3))
5. Payment (3) − Sound scale unit abbreviation (2) − Flow away (3)
6. Mi (1) × Gave sustenance to (3) − No longer alive (4)
7. Art movement after World War I (4) − Romantic partner (3) − Give up (4)
8. Worst grade (1) × Send to a secondary recipient (2) − Not good (3)

**Sample resolutions (from solution):**
- "You, according to a game title (4)" → BABA (from *Baba Is You*)
- "Cooling system (2)" → AC
- "Highest-ranking card (3)" → ACE
- "Small restaurant (4)" → CAFE

Each equation yields a decimal value in the range of printable ASCII; the eight results are 85, 78, 72, 69, 88, 73, 78, 71 → U N H E X I N G.

**Mechanism summary:** Solvers must (1) recognize that all clue answers contain only A–F, (2) perform hex arithmetic, (3) read ASCII. The title "Characters," flavor reference to "base," and "code" are the aha-moment hints.

---

## Tales from a Dating Show
**Source:** Huntinality III ("Hunts Upon a Time"), 2023
**URL:** https://huntinality.com/puzzles/tales_from_a_dating_show
**Answer:** GEORGE III
**Type:** Disney lyric identification + *Guess Who?* elimination — song lyrics identify characters; elimination criteria progressively narrow a 24-character board; eliminated characters' letters spell a clue phrase that leads to the answer.

**Flavor text:** "Our most promising characters are here to serenade you!"

**Puzzle text:**

Twelve "contestants" on a dating show each make a declaration of love. The first letter of each declaration spells PLAY GUESS WHO. Each declaration contains an italicized passage drawn from a Disney song lyric; identifying the song and its film links the contestant to a Disney character.

**The twelve declarations (by first letter, with italicized lyric excerpts):**

- **P** — "Pretty much since I can remember, I have *often dreamed of a lifelong partner*."
- **L** — "Let your mind open up to all the possibilities and *you'll love me at once*."
- **A** — "An extrovert at heart, *I wanna be where the people are* hanging out."
- **Y** — "Ya know *if I had a girlfriend, I'd have so many things to tell her*."
- **G** — "Going axe throwing is my *favorite first date*."
- **U** — "Understand this: when I find my person, *the birds will sing and wedding bells will ring*."
- **E** — "Even I think we could be the perfect couple — *all it takes is faith and trust*."
- **S** (first) — "So many people here are looking for love. *Oh, isn't this amazing?*"
- **S** (second) — "Sadly this dating show is *kinda short, but let's not be too hasty*."
- **W** — "*Why am I stuck in the same place I've always been* while my friends are getting married."
- **H** — "Honestly, *I'd never find love if I were truly to be myself*."
- **O** — "Once you date me, *you'll learn things you never knew you never knew*."

**Stage 2 — The board:**
An interactive 4×6 grid shows 24 Disney characters arranged as in *Guess Who?*. Each character appears twice on the board — once marked with a microphone icon (the lyric singer / protagonist) and once with a devil symbol (a villain from the same film). Solvers match the pairs.

**Stage 3 — Elimination (in reading order):**
Apply each elimination criterion to remove characters; each eliminated character contributes a letter. The criteria and eliminated counts:

| Criterion | Eliminated |
|-----------|-----------|
| Reddish hair | ×3 |
| Magical powers | ×7 |
| Hat wearers | ×3 |
| Questionable character (i.e., villains) | ×4 |
| Forest singers surrounded by animals | ×4 |
| Anyone with a pulse (i.e., living characters) | ×3 |

The sequence of letters extracted from eliminated characters reads: **ANSWER IS KING FROM HAMILTON**

**Extraction:** The enumeration shown is "6 2 4 4 8 → 6 3." "King from Hamilton" = King George III (6 letters, 3 letters). Final answer: **GEORGE III**.

---

## Information Relay
**Source:** Huntinality III ("Hunts Upon a Time"), 2023
**URL:** https://huntinality.com/puzzles/information_relay
**Answer:** FINISH LINE
**Type:** Telephone game / unreliable narrator — five bears each give clues for a target word in different broken ways (homophones, word associations, game references, technically-correct-but-useless, or completely unrelated); solvers deduce target words, identify which bear contributed each pair, and the extraction meta-step requires applying one bear's method to another's clues.

**Flavor text / setup:**

The puzzle is framed as a game called "Exquisite 'Froot'" (a portmanteau of Exquisite Corpse and the Fruit of the Loom bears). The five Bear family members take turns adding two-word clues to a trivia question, but each only sees the previous word written.

**The five bears and their clue styles:**

| Bear | Method |
|------|--------|
| **Papa Bear** | Technically correct but deliberately unhelpful (e.g., "hard, stuff" for METAL) |
| **Mama Bear** | One word that precedes the target and one that follows it (sandwich clues) |
| **Child Bear** | Video game references |
| **Grandma Bear** | Clues for a homophone of the target (she mishears the word) |
| **Grandpa Bear** | Completely unrelated clues (he misunderstood the game) — his clues are used for extraction |

**Six target words and their clue pairs (by bear):**

1. **KNIGHT** (6 letters)
   - Grandma: "dark, time" (clues for NIGHT)
   - Mama: "white, errant"
   - Child: "horsey, L" (chess piece moves in an L-shape)

2. **METAL** (5 letters)
   - Mama: "hair, detector"
   - Papa: "hard, stuff"
   - Grandma: "honor, prize" (clues for MEDAL)

3. **THRONE** (6 letters)
   - Mama: "iron, room"
   - Child: "nuclear, roguelike" (game references)
   - Grandpa: "per, scenes" (extraction clues)

4. **PAPER** (5 letters)
   - Grandpa: "ba, roar" (extraction clues)
   - Grandma: "spend, each" (clues for PAYER)
   - Mama: "wall, tiger"

5. **RAISE** (5 letters)
   - Papa: "more, money"
   - Child: "FF, revival" (Final Fantasy reference)
   - Grandpa: "Q, weegee" (extraction clues)

6. **SANDWICH** (8 letters)
   - Child: "earthbound, lucky" (game references)
   - Grandpa: "seeks, for" (extraction clues)
   - Mama: "knuckle, between"

**Extraction step:**

Indexed letters from the six words (via Grandpa's clue positions) spell: **HOW NANA HEARS GRAMPS**

This instruction tells solvers to apply Grandma's method (interpret as homophones) to Grandpa's clues:
- Grandpa's clue pairs, parsed as homophones: "Swedish bordering person's bar, or queue for example" → FINNISH LINE → homophone: **FINISH LINE**

---

## The Bakery
**Source:** Huntinality III ("Hunts Upon a Time"), 2023
**URL:** https://huntinality.com/puzzles/the_bakery
**Answer:** LOAF-BEARING
**Type:** "Baker's dozen" set-completion — clues each yield an answer that belongs to a well-known set; each set has one member not clued directly (the "extra" / "baker's extra"); indexed letters from those extra members spell the answer.

**Flavor text:** "The baker always gives a little extra."

**Puzzle text:**

Twenty-four clues are provided. Each clue's answer belongs to a familiar group or set. The set has one more member than the puzzle clues — the uncued extra member is the one to index into.

**The 24 clues:**

1. German for "to flash"
2. Dejected
3. Rented
4. Astronomical object sometimes said to foretell doom
5. Civil War losers
6. Hockey's Sidney
7. Venus' son
8. Ballerina, for one
9. Usain Bolt, for example
10. Cannibal party
11. It's sometimes called The Hexagon
12. Car juice
13. Convertible to cash, in economics
14. Home of the Maya and the Aztecs
15. Inventor of an equilibrium in game theory
16. Cutting edge
17. Ancient
18. One who struts
19. Battle of the Alamo's loser
20. Well-founded
21. Land of the Fighting Bull
22. Frames of a movie
23. Third-most populous country
24. Jenny Ryan

**Known set groupings and their extra members (from solution):**

| Set | Clued Members (examples) | Extra Member |
|-----|--------------------------|--------------|
| Crosby, Stills & Nash | Crosby, Stills, Nash | **Young** |
| Santa's Reindeer | Dasher, Dancer, Prancer, Vixen, Comet, Cupid, Donner, Blitzen | **Rudolph** |
| "Something Old, New, Borrowed, Blue" | Old, New, Borrowed, Blue | **Sixpence** |
| Six Flags Over Texas (historical flags) | Spain, France, Mexico, Texas, US, Confederacy | **Rio Grande Republic** |
| States of Matter | Solid, Liquid, Gas | **Plasma** |

**Extraction:** The extra members are indexed by the numbers provided at the end of the puzzle (7, 3, 5, 4, 11, 8, 1, 9, 6, 2, 10 — selecting specific letters) to spell **LOAF-BEARING**.

---

## What's Next?
**Source:** Huntinality III ("Hunts Upon a Time"), 2023
**URL:** https://huntinality.com/puzzles/whats_next
**Answer:** AFGHANISTAN
**Type:** "We Didn't Start the Fire" song sequence + indexing — images depict events from the Billy Joel song; solvers find the *next* item in the song, use provided numbers to index into it, and extracted letters spell a phrase that is itself a lyric, and the answer is the word that follows it.

**Flavor text / framing:** "You hear small parts of a song, and you wonder what comes next after each."

**Puzzle text:**

Ten images are presented, each paired with two numbers in the format A/B. The images depict historical events or people referenced in Billy Joel's "We Didn't Start the Fire." For each image:

1. Identify the event/person from the song
2. Find the *next* item in the song's lyric sequence
3. B = the letter count of that next item (confirmation)
4. A = the index into that next item's name to extract a letter

**The ten images (as described; images not reproduced here):**

- A map silhouette with a marked location — paired 1/7
- A movie poster (man on horseback with cast)
- Two decorative rings on grass
- A professional headshot (man in business attire)
- A silhouette of a horse holding a pole with city skyline
- A cartoon character in fantasy adventure clothing
- A map silhouette with a star marking
- A historical photograph of an Asian man
- Another map silhouette
- A mushroom cloud photograph

**Example (from solution):**
- Image shows Budapest → the next item in the song is ALABAMA (7 letters) → index 1 = **A**

**Extracted letters (in puzzle order, then reordered by song position):** RUSSIANS IN

The phrase "RUSSIANS IN" appears in the song. The next word in "We Didn't Start the Fire" is **AFGHANISTAN**.

**Note:** The final step — knowing that the extracted phrase is itself a lyric and the answer is what follows — is the second aha after the song identification step.

---

## Front and Center
**Source:** Huntinality III ("Hunts Upon a Time"), 2023 — from the "Witch's Hut" round
**URL:** https://huntinality.com/puzzles/front_and_center
**Answer:** RADAR
**Type:** Palindrome headline reconstruction — five newspaper clippings from "SEMI TIMES" have blacked-out headlines with enumerated word lengths; each headline is a palindrome; the center letter of each palindrome is extracted to spell the answer (which is itself a palindrome).

**Flavor text / framing:** Five clippings from the *SEMI TIMES* newspaper, each with a blackout headline and a subtitle describing the story.

**The five clippings:**

**Clipping 1**
Headline word lengths: 1, 5, 5, 5, 4, 5, 2, 4
Subtitle: "Investigation of Kennedy Space Center finds no evidence of any Christmastime satanic presence"
Reconstructed headline: **A SANTA DEVIL NEVER EVEN LIVED AT NASA**
Center letter: **R**

**Clipping 2**
Headline word lengths: 4, 4, 5, 4, 4
Subtitle: "Unsurprisingly, rodents feature in analytics of 2022 Mystery Hunt prologue"
Reconstructed headline: **STAR RATS STATS STAR RATS**
Center letter: **A**

**Clipping 3**
Headline word lengths: 5, 4, 2, 4, 2, 2, 6
Subtitle: "Paradise damaged due to errant throw of Cuban export, situation described as 'extremely awful'"
Reconstructed headline: **CIGAR TOSS IN EDEN IS SO TRAGIC**
Center letter: **D**

**Clipping 4**
Headline word lengths: 3, 2, 1, 6, 3, 1, 3
Subtitle: "Witness to purported sighting of fruit-mammal hybrid still in disbelief, asks question"
Reconstructed headline: **WAS IT A BANANA BAT I SAW?**
Center letter: **A**

**Clipping 5**
Headline word lengths: 2, 3, 3, 2, 5, 4
Subtitle: "Bird continues reign of terror after devouring child's Tootsie pop, consumes iron wiggler colleague"
Reconstructed headline: **MR. OWL ATE MR. METAL WORM**
Center letter: **R**

**Extraction:** Center letters R-A-D-A-R → **RADAR** (itself a palindrome, reinforcing the theme).

**Key aha moments:**
- "SEMI TIMES" is a palindrome
- The newspaper's masthead crease bisects the T in SEMI TIMES, hinting at "front and center"
- All headlines are palindromes
- The center letter of each is extracted

---

## Itinerary
**Source:** Teammate Hunt 2021
**URL:** https://2021.teammatehunt.com/puzzles/itinerary
**Answer:** WILLIAMSBURG
**Type:** All-October-1st historical events + chronological letter extraction — each clue describes an event that happened on October 1st in different years; solvers identify the entity (person/place/organization) involved; tickets mark a specific letter in each entity's name; reading those letters in chronological order spells the answer.

**Flavor text:** "It's finally the date of the carnival! There's so much we can do today, so let's make sure we do things in order."

**Erratum (issued October 1, 2021):** "The first two rows were previously swapped. The version currently on the page is correct."

**Puzzle text:**

Twelve historical events, all occurring on October 1st of various years. Solvers identify the specific entity (person, place, or organization) referenced. Ticket stubs indicate the letter position to extract from each entity's name. Events must be sorted chronologically to read the answer.

**Complete event list with solutions:**

| Year | Event clue | Entity | Letter extracted |
|------|-----------|--------|-----------------|
| 959 | Mourn the death of an English king | Eadwig the All-Fair | **W** |
| 1800 | Sign away Louisiana in a secret agreement | Third Treaty of San Ildefonso | **I** |
| 1829 | Found a colonial educational institution | South African College | **L** |
| 1861 | Publish a recipe collection | Mrs Beeton's Book of Household Management | **L** |
| 1890 | Establish a nature reserve | Yosemite National Park | **I** |
| 1891 | Open an American educational institution | Stanford University | **A** |
| 1908 | Start selling an affordable car | Ford Model T | **M** |
| 1958 | Begin operations of a federal independent agency | NASA | **S** |
| 1964 | Launch the Free Speech Movement in a city | Berkeley | **B** |
| 1968 | Nationalize a radio corporation | British Guiana Broadcasting Service | **U** |
| 1971 | Open a magical resort | Walt Disney World | **R** |
| 1988 | Award gold to a Golden Slam champion | Steffi Graf | **G** |

**Answer:** W-I-L-L-I-A-M-S-B-U-R-G = **WILLIAMSBURG**

---

## Single Elimination
**Source:** Teammate Hunt 2021
**URL:** https://2021.teammatehunt.com/puzzles/single-elimination
**Answer:** PLATINUM
**Type:** Interactive meta-mechanical puzzle (entangled with "Quick Response") — solvers submit words to probe a hidden 26×26 binary matrix encoding letter-vs-letter matchup outcomes; the matrix plus a QR mask from a companion puzzle yields the final answer.

**Note:** This puzzle is heavily interactive and JavaScript-dependent. The core puzzle content cannot be fully reproduced as static text. The mechanism is described below.

**Puzzle framing:**
A single-elimination tournament bracket where letters of the alphabet compete against each other. Each ordered pair of letters (A vs B, etc.) has a fixed winner. This defines a hidden 26×26 binary grid (rows = letter A, columns = letter B; pixel = which wins).

**Mechanism:**
1. Solvers submit English words to the puzzle interface
2. The system runs a single-elimination bracket on the letters of that word, with two variants:
   - **leftbye**: In odd rounds, the leftmost letter gets a bye (advances without competing)
   - **rightbye**: In odd rounds, the rightmost letter gets a bye
3. The word length strategy: start with 2-letter words (directly reveals one cell), then words of length 2^n + 1 to progressively reconstruct the full matrix
4. Over many submissions, the full 26×26 binary image is recovered

**Entanglement with "Quick Response":**
The companion puzzle "Quick Response" provides:
- A QR code mask pattern
- Coordinates ("corrupted bits") from 8 QR code layers

The mask is applied to the recovered Single Elimination grid. Reading the binary values at the specified coordinates, in the order given by Quick Response's bitmask layers, spells: **PLATINUM**

**Note:** This is among the most technically complex puzzle types encountered — effectively a constraint-satisfaction problem over letter-pair relationships, solved through strategic combinatorial probing.

---

## Par Re-sing
**Source:** Teammate Hunt 2021
**URL:** https://2021.teammatehunt.com/puzzles/par-re-sing
**Answer:** BLACKADDER'S CHRISTMAS CAROL
**Type:** Syllable rearrangement + second-order syllable rearrangement — nonsense compound phrases are clued; rearranging their syllables yields real words matching right-side category clues; indexed letters spell a clue phrase; that phrase's syllables are themselves rearranged to give the final answer.

**Puzzle text:**

The puzzle title "Par Re-sing" is itself a syllable rearrangement of "Reparsing" (or "Parsing Re-"), hinting at the mechanism.

**Instructions (paraphrased):** Each left-side clue describes a nonsense phrase. Rearrange the syllables of those words to get a real word that matches the right-side category clue.

**Sample clue pairs (27 total):**

| Nonsense phrase (left clue description) | Category / right clue | Answer word |
|----------------------------------------|----------------------|-------------|
| "Sand Per Amp" | Symbol | AMPERSAND |
| "If everything was one sea creature, you might remark it is (3 1 4)" | — | ALL A CRAB → ALL A CLAM → [phrase clue] |
| "If you made a hot beverage out of an insect conflict, you would have an (3 3 3)" | — | ANT WAR TEA → [phrase] |
| "If you presented a TV award to a kind of public transit, it would be a (3 4)" | — | BUS EMMY → EMBASSY → Ministry |
| "Crete Con" | Construction Material | CONCRETE |
| "Dull Can Stick" | Light source | CANDLESTICK |
| — | Bridge | (5 letters) |
| — | Teacher | (5 letters) |
| — | Sweetener | (8 letters) |
| — | Cannon | (7 letters) |

**Right-side category point values (the secondary grid):**

| Category | Points |
|----------|--------|
| Symbol | 1 |
| Spice | 4 |
| Humanity | 10 |
| Ministry | 3 |
| Authoritative | 8 |
| Officer | 8 |
| Construction Material | 1 |
| Light source | 11 |
| Bridge | 5 |
| Teacher | 5 |
| Learn | 1 |
| Sweetener | 8 |
| Alter | 4 |
| Fruit | 4 |
| Grain | 1 |
| Restaurant | 1 |
| Hairy | 3 |
| Showcase | 2 |
| Flavoring | 1 |
| Guarantee | 3 |
| Identifier | 3 |
| Touchdown | 1 |
| Cannon | 7 |
| Doubt | 5 |
| Makeup | 2 |
| Definite | 3 |
| Sulfur | 9 |

**Extraction:**
- Answers are ordered alphabetically
- A number and letter are extracted from each row (the point values index into the answer words)
- This yields the intermediate phrase: **ADDBLACKERS MUSSKRIS ROLECARE** (approximately — scrambled syllables)
- Those syllables rearrange into: **BLACKADDER'S CHRISTMAS CAROL**

**Final answer:** BLACKADDER'S CHRISTMAS CAROL

---

*End of Batch 3. 9 puzzles total: 6 from Huntinality III (huntinality.com), 3 from Teammate Hunt 2021 (2021.teammatehunt.com).*
