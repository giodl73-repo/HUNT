# Stage 9 — LIVE TEST Transcript

**Hunt:** Wololo (Age of Empires)
**Test Date:** 2026-02-27
**Solver Team:** Captain, Speedster, Skeptic, Newbie (4-agent simulation)
**Simulated Clock:** T+0:00 through completion

---

## Phase 1: Opening (T+0:00 - T+4:30)

---

**CAPTAIN** (T+0:00): All right, team. We have five puzzles plus a meta. Let me scan the whole hunt before anyone touches anything.

*[Captain reads the intro — "The Monk" narrator, five Ages, each puzzle tied to a domain: Civilizations, Units, Technologies, Maps, Strategy.]*

**CAPTAIN** (T+0:30): OK. Here's the structure. Five feeder puzzles, each named after an Age of Empires age — Dark, Feudal, Castle, Imperial, Post-Imperial. The meta is a crossword where the five feeder answers slot in and we extract a final word from numbered squares. Every puzzle uses AoE2 game knowledge. Let me categorize them.

- **Puzzle I (Dark Age)** — Match 8 civilization bonuses to civs, extract letters. Knowledge-based, lookup-friendly. Good warm-up.
- **Puzzle II (Feudal Age)** — Tournament bracket, 8 units, counter logic. Needs game knowledge about what beats what. Has a trick matchup.
- **Puzzle III (Castle Age)** — Fill in missing technologies in upgrade chains. Straightforward if you know the tech tree.
- **Puzzle IV (Imperial Age)** — Plot resources on 5x5 grids, dots trace letters. Visual/spatial puzzle.
- **Puzzle V (Post-Imperial)** — Annotated game replay with 6 math questions. Number-to-letter extraction (A=1, Z=26). Heaviest puzzle — lots of lookups.

**SPEEDSTER** (T+1:00): I already skimmed them all. Puzzle III is four questions, four blanks — I can knock that out in two minutes. I'm grabbing it.

**CAPTAIN** (T+1:10): Hold on. Let me assign—

**SPEEDSTER** (T+1:12): Too late, already working.

**CAPTAIN** (T+1:15): Fine. Speedster takes Puzzle III. Skeptic, I want you on Puzzle II — the tournament bracket. It has a trick matchup and you'll spot it. Newbie, you're with me on Puzzle I. It's the most accessible — matching bonuses to civilizations.

**NEWBIE** (T+1:30): OK, so... what do I actually DO? I opened Puzzle I and it says "Eight civilizations stand before you. Each one carries a gift no other has. Name them." That sounds like a riddle. Am I supposed to guess?

**CAPTAIN** (T+1:45): Not a riddle. It's a lookup. Each "Bonus" describes a real civ bonus from Age of Empires 2. You match the description to the correct civilization. Then there's a circled letter position — that tells you which letter of the civ name to extract. We collect all 8 letters and find a word.

**NEWBIE** (T+2:00): Oh! OK. So like, Bonus A says "infantry moves 15% faster starting in Feudal Age." I need to figure out which civilization has that bonus?

**CAPTAIN** (T+2:05): Exactly. And we have the world data as our reference sheet. Let me pull up civs.md.

**SKEPTIC** (T+2:10): Before I start Puzzle II — Captain, I notice the intro says "The Monk" is our narrator. And the hunt is called "Wololo," which is the Monk's conversion sound. I suspect the meta answer is WOLOLO. Just flagging that so we have it in our back pocket.

**CAPTAIN** (T+2:20): Noted. Good catch. Let's not assume though — solve the feeders clean and let the meta confirm it.

**NEWBIE** (T+2:30): Wait, nobody said anything about Puzzle V. Who's doing that one?

**CAPTAIN** (T+2:35): I'll take V after we finish I together. It's the heaviest — six math questions based on game economy data. I need the reference sheets. Let's get moving.

---

## Phase 2: Solving (T+4:30 - T+38:00)

---

### Puzzle III — The Speedster (T+2:00 - T+7:30)

**SPEEDSTER** (T+2:00): Chain 1. "??? -> Bodkin Arrow -> Bracer." Ranged attack chain. The first tier is Fletching. Everyone knows that. FLETCHING. Circle letter 2. F-L... second letter is L.

**SPEEDSTER** (T+2:30): Chain 2. "Forging -> ??? -> Blast Furnace." Melee attack. Middle tier is Iron Casting. I-R-O-N... wait, do I ignore spaces? "Ignore spaces when counting." So IRONCASTING. Third letter is O.

**SPEEDSTER** (T+3:00): Chain 3. "Double-Bit Axe -> ??? -> Two-Man Saw." Lumber chain. Middle is Bow Saw. B-O-W-S-A-W. Second letter is O.

**SPEEDSTER** (T+3:20): Chain 4. "??? -> Architecture." Fortification chain. The prerequisite is Masonry. M-A-S-O-N-R-Y. First letter is M.

**SPEEDSTER** (T+3:30): Letters: L, O, O, M. The answer is **LOOM**!

*[Speedster grins.]*

**SPEEDSTER** (T+3:35): That's thematic too. Loom is the first tech you research in every game. Dark Age, at the Town Center.

**CAPTAIN** (T+3:40): Already? Walk me through it.

**SPEEDSTER** (T+3:45): Fletching-Bodkin-Bracer, letter 2 of Fletching is L. Forging-Iron Casting-Blast Furnace, letter 3 of IronCasting (no spaces) is O. Double-Bit Axe-Bow Saw-Two-Man Saw, letter 2 of BowSaw is O. Masonry-Architecture, letter 1 of Masonry is M. L-O-O-M. Done.

**SKEPTIC** (T+4:00): *[From across the table.]* Wait. Chain 4 says "Both tiers grant buildings +10% HP and additional armor." Let me check — Masonry and Architecture. Masonry: buildings +10% HP, +1/+1 armor, +3 building armor. Architecture: buildings +10% HP, +1/+1 armor, +3 building armor. Yes, both grant +10% HP plus armor. Masonry is correct.

**SKEPTIC** (T+4:10): And Chain 2 — "ignore spaces when counting." So Iron Casting: I(1), R(2), O(3), N(4), C(5), A(6), S(7), T(8), I(9), N(10), G(11). Third letter is O. Confirmed.

**SPEEDSTER** (T+4:15): Told you. LOOM. I'm moving on.

**CAPTAIN** (T+4:20): Puzzle III solved. LOOM. Speedster, grab Puzzle IV — the visual one with the grids.

**SPEEDSTER** (T+4:25): On it.

---

### Puzzle I — Captain & Newbie (T+2:00 - T+14:00)

**CAPTAIN** (T+4:30): OK Newbie, let's work through these. I have the civ reference open. Bonus A: "infantry moves 15% faster, starting in Feudal Age."

**NEWBIE** (T+4:45): Umm... is there a list of all the civs and their bonuses?

**CAPTAIN** (T+4:50): Yes, right here in civs.md. Look at the Key Bonus column.

**NEWBIE** (T+5:00): *[Scanning.]* Oh! Celts: "Infantry move 15% faster (starting Feudal Age)." That matches exactly!

**CAPTAIN** (T+5:10): Good. CELTS. Five letters. Circle letter 1. C-E-L-T-S. First letter: **C**.

**NEWBIE** (T+5:20): OK I think I get how this works! Bonus B: "infantry attacks 33% faster, starting in Feudal Age." Let me look... Japanese! "Infantry attack 33% faster (starting Feudal Age)."

**CAPTAIN** (T+5:30): Right. JAPANESE. Eight letters. Circle letter 2. J-A-P-A-N-E-S-E. Second letter: **A**.

**NEWBIE** (T+5:40): This is fun! I can just match them. Bonus C: "Town Center and Dock work rate increases each Age: +10% Feudal, +15% Castle, +20% Imperial." That's... *[scanning]* ...Persians!

**CAPTAIN** (T+5:50): Correct. PERSIANS. Circle letter 4. P-E-R-S-I-A-N-S. Fourth letter: **S**.

**NEWBIE** (T+6:00): Bonus D: "foot archers except Skirmishers gain +1 range Castle and +1 Imperial." That's Britons! It says "+1 range Castle, +1 Imperial."

**CAPTAIN** (T+6:10): BRITONS. Seven letters. Circle letter 4. B-R-I-T-O-N-S. Fourth letter: **T**.

**NEWBIE** (T+6:30): Bonus E: "cavalry archers fire 25% faster." Mongols!

**CAPTAIN** (T+6:35): MONGOLS. Circle letter 6. M-O-N-G-O-L-S. Sixth letter: **L**.

**NEWBIE** (T+6:50): Wait, hold on. The puzzle says "Two of these bonuses concern infantry. Read them closely." Bonuses A and B are both infantry bonuses — Celts move faster, Japanese attack faster. Is that a warning about something?

**CAPTAIN** (T+7:00): Good observation. I think it's just making sure solvers don't mix them up. "Moves faster" versus "attacks faster" — if you're not reading carefully, you might swap Celts and Japanese. It's a built-in hint for careful reading.

**NEWBIE** (T+7:10): Oh, that's actually really helpful. I almost did mix them up at first.

**CAPTAIN** (T+7:20): Bonus F: "villagers carry +5 resources, military units created 11% faster."

**NEWBIE** (T+7:30): *[Scanning.]* Aztecs! "Villagers carry +5 resources; Military units created 11% faster." Exactly.

**CAPTAIN** (T+7:40): AZTECS. Six letters. Circle letter 4. A-Z-T-E-C-S. Fourth letter: **E**.

**NEWBIE** (T+7:55): Bonus G: "cavalry has +20% HP." That's... Franks! "Cavalry +20% HP."

**CAPTAIN** (T+8:00): FRANKS. Circle letter 1. F-R-A-N-K-S. First letter: **F**.

**NEWBIE** (T+8:10): And Bonus H: "trash units — Skirmishers, Pikemen, Light Cavalry — cost 25% less." Byzantines!

**CAPTAIN** (T+8:15): BYZANTINES. Ten letters. Circle letter 1. B-Y-Z-A-N-T-I-N-E-S. First letter: **B**.

**CAPTAIN** (T+8:30): So our eight circled letters in order A through H are: C, A, S, T, L, E, F, B.

**NEWBIE** (T+8:40): "Eight letters. Not all of them matter. Find the word." So... not all eight letters are part of the answer?

**CAPTAIN** (T+8:50): Right. The first six spell CASTLE. F and B are extras — decoys to confirm you got the right civs.

**NEWBIE** (T+8:55): CASTLE! Like the building you build in Castle Age!

**CAPTAIN** (T+9:00): Puzzle I solved. **CASTLE**. Nice work, Newbie — you drove most of that.

**NEWBIE** (T+9:05): *[Beaming.]* I actually solved a puzzle hunt puzzle!

---

### Puzzle II — The Skeptic (T+2:30 - T+16:00)

**SKEPTIC** (T+4:30): All right. Tournament bracket. Eight units, single elimination. Counter logic decides each match. Let me map out the unit types first.

*[Skeptic writes out a classification chart.]*

```
Onager       — Siege
Crossbowman  — Archer
Pikeman      — Anti-cavalry Infantry
Knight       — Cavalry
Cataphract   — Cavalry (unique)
Halberdier   — Anti-cavalry Infantry
Mangonel     — Siege
Champion     — Infantry
```

**SKEPTIC** (T+5:30): QF-1: Onager vs. Crossbowman. Siege vs. Archer. Counter logic says siege destroys archers. Winner: **Onager**.

**SKEPTIC** (T+5:45): QF-2: Pikeman vs. Knight. Anti-cavalry infantry vs. cavalry. "Anti-cavalry infantry destroys cavalry." Winner: **Pikeman**.

**SKEPTIC** (T+6:00): QF-3: Cataphract vs. Halberdier. Now HERE is where it gets interesting. The puzzle says "One matchup breaks the rule you expect. A certain cavalry unit does not fear the spear." The Cataphract is Byzantine cavalry that specifically resists anti-cavalry bonus damage. Halberdier is anti-cavalry infantry. By NORMAL rules, Halberdier should win. But the Cataphract's trample damage and anti-bonus resistance mean it beats Halberdiers. Winner: **Cataphract**.

**SKEPTIC** (T+6:30): I want to double-check this. The puzzle says "Its armor was designed to resist the very bonus that should destroy it." The Cataphract's key trait is literally "Resists anti-cavalry bonus damage." This is unambiguous. Good.

**SKEPTIC** (T+7:00): QF-4: Mangonel vs. Champion. Siege vs. infantry. Counter logic says siege destroys infantry. Winner: **Mangonel**.

**SKEPTIC** (T+7:30): Semifinals. SF-1: Onager vs. Pikeman. Siege vs. infantry. Siege destroys infantry. Winner: **Onager**.

**SKEPTIC** (T+8:00): SF-2: Cataphract vs. Mangonel. Cavalry vs. siege. Cavalry destroys siege. Winner: **Cataphract**.

**SKEPTIC** (T+8:30): Now the letter extraction. Each bracket slot has a number in square brackets telling me which letter to take from the winner's name.

- QF-1: Onager, [1st]. O-N-A-G-E-R. 1st = **O**
- QF-2: Pikeman, [7th]. P-I-K-E-M-A-N. 7th = **N**
- QF-3: Cataphract, [2nd]. C-A-T-A-P-H-R-A-C-T. 2nd = **A**
- QF-4: Mangonel, [4th]. M-A-N-G-O-N-E-L. 4th = **G**
- SF-1: Onager, [5th]. O-N-A-G-E-R. 5th = **E**
- SF-2: Cataphract, [7th]. C-A-T-A-P-H-R-A-C-T. 7th = **R**

**SKEPTIC** (T+9:00): Six letters: O, N, A, G, E, R. The answer is **ONAGER**.

**SKEPTIC** (T+9:10): That's self-referential — the Onager is one of the units in the bracket AND the answer. I want to check whether that's a coincidence or a problem.

*[Skeptic pauses to think.]*

**SKEPTIC** (T+9:30): It's fine. The extraction pulls letters from MULTIPLE winners (Onager, Pikeman, Cataphract, Mangonel), not just one. The fact that the answer happens to match one of the units is thematic, not circular. No solver would know the answer is ONAGER just because Onager appears in the bracket — there are 8 units in there.

**SKEPTIC** (T+9:45): One thing I want to flag though. The worksheet says "Read your six circled letters in order, QF-1 through SF-2." But the bracket also has a FINAL match (SF-1 winner vs SF-2 winner = CHAMPION). The worksheet doesn't include the final. That's slightly confusing — a solver might try to extract a letter from the final match winner too.

**CAPTAIN** (T+10:00): Is it clear enough from the worksheet?

**SKEPTIC** (T+10:10): The worksheet has exactly six rows — QF-1 through SF-2. It explicitly says "Read your six circled letters." So yes, it's clear IF you follow the worksheet. But a solver who works from the bracket diagram alone might include the final. I'd note this as a minor clarity issue.

**CAPTAIN** (T+10:15): Noted. Puzzle II solved. **ONAGER**.

---

### Puzzle IV — The Speedster (T+4:30 - T+20:00)

**SPEEDSTER** (T+4:30): Five maps, each with a 5x5 grid. Plot the resource coordinates, dots form letters. Let me just bang through these.

**SPEEDSTER** (T+5:00): Map 1 — Arabia. Resources at:
```
(1,1) (1,2) (1,3) (1,4) (1,5) — entire top row
(2,3) (3,3) (4,3) (5,3)       — column 3 from row 2-5
```

```
Row 1: X X X X X
Row 2: . . X . .
Row 3: . . X . .
Row 4: . . X . .
Row 5: . . X . .
```

That's a T. Letter 1 = **T**.

**SPEEDSTER** (T+6:00): Map 2 — Arena. Resources at:
```
(1,2) (1,3) (1,4)
(2,1) (2,5)
(3,1) (3,5)
(4,1) (4,5)
(5,2) (5,3) (5,4)
```

```
Row 1: . X X X .
Row 2: X . . . X
Row 3: X . . . X
Row 4: X . . . X
Row 5: . X X X .
```

That's an O. Letter 2 = **O**.

**SPEEDSTER** (T+7:00): Map 3 — Gold Rush. Resources at:
```
(1,1) (1,5)
(2,1) (2,5)
(3,1) (3,3) (3,5)
(4,2) (4,4)
```

```
Row 1: X . . . X
Row 2: X . . . X
Row 3: X . X . X
Row 4: . X . X .
Row 5: . . . . .
```

That's... a W? Top heavy, two diagonal legs meeting at the bottom-center... Yeah. W. Letter 3 = **W**.

**SPEEDSTER** (T+8:00): Map 4 — Mediterranean. Resources at:
```
(1,1) (1,2) (1,3)
(2,1)
(3,1) (3,2)
(4,1)
(5,1) (5,2) (5,3)
```

```
Row 1: X X X . .
Row 2: X . . . .
Row 3: X X . . .
Row 4: X . . . .
Row 5: X X X . .
```

That's an E. Three horizontal bars with a vertical spine on the left. Letter 4 = **E**.

**SPEEDSTER** (T+9:00): Map 5 — Baltic. Resources at:
```
(1,1) (1,2)
(2,1) (2,3)
(3,1) (3,2)
(4,1) (4,3)
(5,1) (5,4)
```

```
Row 1: X X . . .
Row 2: X . X . .
Row 3: X X . . .
Row 4: X . X . .
Row 5: X . . X .
```

Vertical line on the left, bump at top-right, diagonal leg... That's an R. Letter 5 = **R**.

**SPEEDSTER** (T+10:00): T-O-W-E-R. The answer is **TOWER**!

**SKEPTIC** (T+10:30): Hold on. Let me verify Map 3. You said it's a W?

```
Row 1: X . . . X
Row 2: X . . . X
Row 3: X . X . X
Row 4: . X . X .
Row 5: . . . . .
```

**SKEPTIC** (T+10:45): That's a W only if you read it upside-down from normal letter orientation. Typically a W has the "feet" at the bottom and the open part at the top. Here the convergence point is at the bottom (rows 3-4) and the two vertical strokes are at the top. If you flip it mentally — yeah, I can see W. But this is top-down, with the V-shapes pointing downward. Some solvers might see an M instead.

**SPEEDSTER** (T+11:00): It's a W. The two legs angle inward at the bottom. An M would have the legs angling outward at the top.

**SKEPTIC** (T+11:10): Actually, I think you're right. The empty row 5 at the bottom means the "points" of the W are at rows 3-4, and the "tops" of the W are at row 1. That tracks. But I want to flag this — the letter-recognition step is the most subjective part of any visual puzzle. Different people might see different letters.

**CAPTAIN** (T+11:20): Does TOWER make sense as an AoE answer?

**SPEEDSTER** (T+11:25): Watch Towers, Guard Towers, Keeps, Bombard Towers — towers are everywhere in AoE. It fits.

**CAPTAIN** (T+11:30): Then we go with TOWER. Good. Puzzle IV solved. Speedster, you now have two solves. Nice.

---

### Puzzle V — The Captain (T+9:30 - T+28:00)

**CAPTAIN** (T+9:30): Puzzle V. Six "moments" from a replay. Each answer is a number 1-26, converted to a letter. This is the most calculation-heavy puzzle. I need economy.md, units.md, and techs.md.

**CAPTAIN** (T+10:00): Moment 1 — "Blue has 400 wood, 720 gold. Spends everything training ONE unit type from the Archery Range. Both hit zero."

So I need: n * wood_cost = 400, n * gold_cost = 720, for some Archery Range unit.

Archery Range units that cost wood and gold:
- Archer: 25W 45G. 400/25 = 16, 720/45 = 16. Both = 16! That works.
- Crossbowman: 25W 45G. Same cost as Archer. Also 16.
- Cavalry Archer: 40W 60G. 400/40 = 10, 720/60 = 12. Mismatch. No.
- Hand Cannoneer: 45F 50G. Costs food and gold, not wood. Doesn't use wood. No.
- Skirmisher: 25F 35W. Costs food and wood, not gold. No.

**CAPTAIN** (T+11:00): The puzzle says "Castle Age" and "Archery Range" — Crossbowman is the Castle Age upgrade of the Archer, same cost. Either way: **16** units. Answer 1 = 16 = **P**.

**NEWBIE** (T+11:30): Can I help with Puzzle V?

**CAPTAIN** (T+11:35): Sure. Read me Moment 2.

**NEWBIE** (T+11:40): "Blue is thinking about Imperial Age. Standard requirement is two Castle Age buildings. But there's an alternative — a single building type that satisfies the prerequisite on its own. How many of that building does the shortcut require?"

**CAPTAIN** (T+11:50): The techs reference says: "Imperial Age requires 2 Castle Age buildings (Siege Workshop, Monastery, University) OR 1 Castle." The shortcut is a Castle. You need just 1 Castle to satisfy the Imperial Age prerequisite. Answer 2 = **1** = **A**.

**NEWBIE** (T+12:00): Oh cool! So building a Castle counts as both buildings?

**CAPTAIN** (T+12:05): Exactly. It's a real game shortcut — expensive (650 stone) but saves you building two separate structures.

**CAPTAIN** (T+12:30): Moment 3 — "One Archery Range, nonstop, 9 minutes, Crossbowmen. How many?"

9 minutes = 540 seconds. Crossbowman train time from economy.md: 27 seconds.
540 / 27 = **20**. Answer 3 = 20 = **T**.

**NEWBIE** (T+12:45): The puzzle says "The Crossbowman does not train at the same speed as the Archer it upgraded from." Is that a hint?

**CAPTAIN** (T+12:50): Yes. Archers take 35 seconds. Crossbowmen take 27 seconds. If you mistakenly use the Archer's 35-second train time, you'd get 540/35 = 15.4, which isn't clean. The hint steers you to look up the correct Crossbowman time.

**CAPTAIN** (T+13:30): Moment 4 — "One Stable, nonstop, 9 minutes, Knights."

540 seconds. Knight train time: 30 seconds.
540 / 30 = **18**. Answer 4 = 18 = **R**.

**CAPTAIN** (T+14:00): Moment 5 — "Cavalier upgrade gold cost divided by Militia per-unit gold cost."

Cavalier upgrade from techs.md: 300F 300G. Gold cost = 300.
Militia per-unit cost from economy.md: 60F 20G. Gold cost = 20.
300 / 20 = **15**. Answer 5 = 15 = **O**.

**CAPTAIN** (T+14:30): Moment 6 — "Monk's conversion range after Block Printing."

From techs.md: Monks have base conversion range 9. Block Printing gives +3 range.
9 + 3 = **12**. Answer 6 = 12 = **L**.

**CAPTAIN** (T+15:00): Extraction: 16, 1, 20, 18, 15, 12. Letters: P, A, T, R, O, L. The answer is **PATROL**.

**SPEEDSTER** (T+15:10): Patrol! That's the unit command. You select units and patrol between waypoints.

---

### Cross-checking pause (T+15:00 - T+18:00)

**SKEPTIC** (T+15:30): Before we go to the meta, I want to verify a few things in Puzzle V.

Moment 1: The puzzle says "Castle Age" and "Archery Range." In Castle Age you have access to Crossbowmen (if you've researched the upgrade), Cavalry Archers, and still Archers/Skirmishers. But the puzzle says "several Archery Range units cost wood and gold." Let me list them:
- Archer: 25W 45G
- Crossbowman: 25W 45G
- Cavalry Archer: 40W 60G

Only those three cost wood AND gold. Skirmishers cost food+wood. Hand Cannoneers cost food+gold and aren't Castle Age anyway.

400/25 = 16, 720/45 = 16 for Archer or Crossbowman. Both same cost, same answer.
400/40 = 10, 720/60 = 12 for Cavalry Archer. Doesn't work.

So 16 is the unique answer. Good.

**SKEPTIC** (T+16:00): Moment 5. The puzzle says "the Cavalier upgrade's gold cost." Let me confirm: Cavalier upgrade is Knight -> Cavalier. From techs.md: "Cavalier | Imperial | 300F 300G." Gold cost = 300.

And "the Militia's per-unit gold cost" — Militia costs 60F 20G per unit. Gold = 20.

300/20 = 15. Clean division. Good.

**SKEPTIC** (T+16:30): Moment 6. Base Monk conversion range. The techs file says "Monks have a base range of 9 for conversion. Block Printing raises it to 12." The puzzle says Block Printing "extends conversion reach" and asks for the range after. 9 + 3 = 12. Confirmed.

**SKEPTIC** (T+17:00): I have one concern. Moment 2 says "how many of that building does the shortcut require?" The answer is 1 Castle. But could someone interpret this differently — like "how many of any building" rather than "how many Castles"? The question asks "how many of THAT building" — singular building type. You need 1 Castle. It's unambiguous.

**SKEPTIC** (T+17:30): Actually, I want to double-check something else. The puzzle says Blue "spends every last resource" and "both wood and gold hit zero — no remainder." This implies the unit cost must divide BOTH stockpiles evenly with the same quantity. For Crossbowman (25W, 45G): 400/25 = 16 and 720/45 = 16. Both exactly 16. For Cavalry Archer (40W, 60G): 400/40 = 10 and 720/60 = 12. The quantities differ — you can't train the same number and zero out both. So the answer is forced to be 16. Clean puzzle design.

**CAPTAIN** (T+17:45): Good verification. All five feeders solved. Let's go to the meta.

---

## Phase 3: Meta Attempt (T+18:00 - T+28:00)

---

**CAPTAIN** (T+18:00): All right. We have all five feeder answers:

| Puzzle | Answer | Length |
|--------|--------|--------|
| I (Dark Age) | CASTLE | 6 |
| II (Feudal Age) | ONAGER | 6 |
| III (Castle Age) | LOOM | 4 |
| IV (Imperial Age) | TOWER | 5 |
| V (Post-Imperial) | PATROL | 6 |

The meta is a crossword. We have a grid with blank squares, numbered circles [1] through [6], and clue labels telling us where each answer goes.

**CAPTAIN** (T+18:30): Let me read the meta page. The Monk says: *"You have advanced through all five ages. One sound remains. You know what it is."*

Instructions: "Fill each answer into its slot. Read the circled squares in order."

There are down clues and across clues:
- Down 1: "Textile technology — first research in every game (4)" = LOOM at column 0
- Down 2: "Fortification built in the third Age (6)" = CASTLE at column 2
- Down 3: "Defensive structure that shoots arrows (5)" = TOWER at column 4
- Across: "Siege weapon — hurls stones over walls (6)" = ONAGER at row 2
- Across: "Unit command — march continuously along a path (6)" = PATROL at row 4

**NEWBIE** (T+19:00): So we just... put the words in the grid?

**CAPTAIN** (T+19:05): Yes. Let me fill it in.

```
     c0  c1  c2  c3  c4  c5

r0    L
r1   [4]O          C
r2   [2]O   N      A    G    E    R      << ONAGER across
r3    M             S         T
r4    P    A        T    R   [6]O  [5]L   << PATROL across
r5              [3]L        [1]W
r6               E              E
r7                              R
```

**CAPTAIN** (T+19:30): Now read the numbered circles [1] through [6]:

- [1] at (r5, c4) = **W** (from TOWER, position 2: T-O-**W**-E-R)
- [2] at (r2, c0) = **O** (from LOOM position 2 / ONAGER position 0: both **O**)
- [3] at (r5, c2) = **L** (from CASTLE position 4: C-A-S-T-**L**-E)
- [4] at (r1, c0) = **O** (from LOOM position 1: L-**O**-O-M)
- [5] at (r4, c5) = **L** (from PATROL position 5: P-A-T-R-O-**L**)
- [6] at (r4, c4) = **O** (from TOWER position 1 / PATROL position 4: both **O**)

**CAPTAIN** (T+20:00): [1]-[2]-[3]-[4]-[5]-[6]: **W-O-L-O-L-O**.

**NEWBIE** (T+20:10): WOLOLO!!! That's the sound the Monk makes when he converts something!

**SPEEDSTER** (T+20:15): Called it. Well, Skeptic called it at the beginning.

**SKEPTIC** (T+20:20): I said it was PROBABLY Wololo. Now it's confirmed.

**CAPTAIN** (T+20:25): Let me verify the crossings are consistent:
- (c0, r2): LOOM pos 2 = O, ONAGER pos 0 = O. Match.
- (c2, r2): CASTLE pos 1 = A, ONAGER pos 2 = A. Match.
- (c2, r4): CASTLE pos 3 = T, PATROL pos 2 = T. Match.
- (c4, r4): TOWER pos 1 = O, PATROL pos 4 = O. Match.

All four crossings check out. The grid is clean.

---

### 80% Rule Test (T+20:30 - T+24:00)

**CAPTAIN** (T+20:30): Before we celebrate, let me test whether the meta is solvable with only 4 of 5 answers. This is part of the test protocol.

**SKEPTIC** (T+21:00): I'll run the scenarios.

**Scenario 1 — Missing CASTLE:**
We have LOOM, ONAGER, TOWER, PATROL. CASTLE's slot is column 2, rows 1-6. From crossings, we know position 1 = A (from ONAGER) and position 3 = T (from PATROL). So the word is _A_T_E (6 letters). In AoE vocabulary, CASTLE is the only fit. **Recoverable.**

**Scenario 2 — Missing ONAGER:**
CASTLE fills in A at (c2,r2). LOOM fills in O at (c0,r2). So ONAGER's slot has O_A___ (positions 0 and 2 known). A 6-letter siege unit starting with O, with A at position 2 — ONAGER is the only candidate. **Recoverable.**

**Scenario 3 — Missing LOOM:**
ONAGER fills in O at (c0,r2). So LOOM's slot has __O_ (position 2 = O). A 4-letter AoE technology with O at position 2. LOOM fits. Could any other 4-letter tech fit _?O?_? Possible alternatives... LOOM is the most iconic Dark Age tech. With AoE context, a solver would get this. **Likely recoverable.**

**Scenario 4 — Missing TOWER:**
PATROL fills in O at (c4,r4). So TOWER's slot has _O___ (position 1 = O). A 5-letter AoE structure starting with _O___. TOWER is strongly constrained. **Recoverable.**

**Scenario 5 — Missing PATROL:**
CASTLE fills in T at (c2,r4). TOWER fills in O at (c4,r4). So PATROL's slot has __T_O_ (positions 2 and 4 known). A 6-letter AoE command — PATROL fits uniquely. **Recoverable.**

**SKEPTIC** (T+23:30): All five scenarios pass. The meta is solvable with any 4 of 5 answers. The 80% rule holds.

**CAPTAIN** (T+24:00): Good. The meta is robust.

---

## Phase 4: Completion (T+24:00 - T+28:00)

---

**CAPTAIN** (T+24:00): The final message from the Monk: *"You have advanced through all five ages. One sound remains. You know what it is."*

And our answer: **WOLOLO**.

The Monk has been narrating the entire hunt. Every puzzle was a lesson — civilizations, units, technologies, maps, strategy. And the final answer is the Monk's signature move: conversion. The whole hunt was a conversion ritual.

**NEWBIE** (T+24:30): That... is really cool? Like, the narrator was trying to convert US the whole time? Into AoE players?

**SPEEDSTER** (T+24:45): Into puzzle hunters, more like. The monk converts villagers. The hunt converts newbies.

**NEWBIE** (T+25:00): I genuinely want to do another one of these. This was way more fun than I expected. The first puzzle — matching the civ bonuses — I could actually do that. I contributed!

**SKEPTIC** (T+25:15): I have my notes for the post-hunt report. Overall this is a clean hunt. My specific issues:

1. **Puzzle II bracket ambiguity (minor):** The bracket diagram shows a FINAL match, but the worksheet only extracts from QF-1 through SF-2 (6 matches). The FINAL isn't used for extraction. A solver who works from the bracket diagram rather than the worksheet might be confused about whether to include the final. Recommendation: either remove the FINAL from the bracket diagram or add a note saying "The final match is not used for extraction."

2. **Puzzle IV letter ambiguity on Map 3 (minor):** The W formed by Gold Rush dots could be misread as an M by some solvers, depending on their mental orientation. The empty row 5 helps disambiguate, but it's the weakest letter recognition of the five maps. Recommendation: consider adding one more resource point at row 5 to make the W's bottom vertex more explicit.

3. **Puzzle V Moment 1 — Archer vs. Crossbowman ambiguity (cosmetic):** Both Archer and Crossbowman cost 25W 45G. The puzzle says Blue is in Castle Age, which is where Crossbowmen become available. But even if the solver thinks "Archer" instead of "Crossbowman," the answer (16) is the same because the costs are identical. This is not a bug — it's actually elegant design. The answer is forced regardless of which unit you pick.

4. **Puzzle I — the "not all of them matter" instruction:** This works, but might frustrate a solver who gets 6 correct and 2 wrong. If their wrong answers happen to produce letters that look like a word, they could submit a false positive. Low risk given that the two decoy positions (G and H) produce F and B, which don't extend CASTLE into anything plausible.

5. **Puzzle V Moment 2 — "how many of that building":** Clean as written. The phrase "that building" refers back to "a single building type that satisfies the prerequisite on its own" — which is a Castle. Answer: 1. No ambiguity.

**CAPTAIN** (T+26:00): Any hard bugs? Anything broken?

**SKEPTIC** (T+26:10): No hard bugs. Every puzzle has a unique, verifiable answer. The counter logic in Puzzle II is sound — the Cataphract exception is well-clued. The math in Puzzle V checks out to clean integers in every moment. The visual letters in Puzzle IV are recognizable. The tech chains in Puzzle III are standard AoE knowledge. The civ bonuses in Puzzle I are accurate per the reference data.

**CAPTAIN** (T+26:30): Verdict?

**SKEPTIC** (T+26:35): Ship it. With the minor notes above as polish items, not blockers.

**SPEEDSTER** (T+26:40): I cleared three puzzles. My personal record is five in one hunt, but these were satisfying. Puzzle III was a speed-solve dream — four clean lookups. Puzzle IV was fun for the visual pattern recognition.

**NEWBIE** (T+27:00): I want to say — the on-ramp worked. Puzzle I was perfect for my first puzzle hunt puzzle. Matching bonuses to civs from a reference sheet? I could do that. The extraction step (picking circled letters) was explained clearly enough that I understood what to do. And the "two of these bonuses concern infantry" hint saved me from making an error. I would have mixed up Celts and Japanese without it.

**CAPTAIN** (T+27:30): Hunt complete. Five feeders, one meta, answer is WOLOLO. Good hunt, team.

---

## Measurements

| Metric | Result |
|--------|--------|
| **Total simulated solve time** | ~28 minutes (T+0:00 to T+28:00) |
| **Puzzles solved in order** | III (Speedster, T+3:30) -> I (Captain+Newbie, T+9:00) -> IV (Speedster, T+10:00) -> II (Skeptic, T+9:00) -> V (Captain+Newbie, T+15:00) -> META (T+20:00) |
| **Which puzzle caused most stuckness** | None caused extended stuckness. Puzzle V took the longest (6 minutes of calculation) due to 6 separate lookups. Puzzle IV had the most ambiguity concern (Map 3 = W). |
| **Did the Newbie participate meaningfully?** | Yes. Newbie drove 6 of 8 civ-bonus matches in Puzzle I, asked a perceptive question about the infantry hint, and assisted with Moment 2 of Puzzle V. First-timer experience was positive. |
| **Did the Speedster get anything wrong?** | No. All three of Speedster's solves (III, IV, and meta fill assist) were correct. The Speedster's speed-solving style was well-matched to the puzzle difficulty. |
| **Did the Skeptic find any real issues?** | Yes — 2 minor clarity issues (bracket FINAL ambiguity in II, Map 3 W/M ambiguity in IV) and 3 cosmetic observations. No hard bugs or broken clues. |
| **Was the meta solvable with 4 of 5 answers?** | Yes. All 5 missing-word scenarios were tested. Each missing word is uniquely recoverable from crossing constraints + AoE vocabulary. 80% rule PASSES. |
| **Was the hunt FUN? (Captain's vibe)** | Yes. Good parallel work distribution — all 4 agents stayed busy. Difficulty curve was smooth (III easiest -> I accessible -> II/IV moderate -> V hardest). The WOLOLO reveal was satisfying. Thematic coherence (Monk narrator -> conversion -> WOLOLO) landed well. |
| **Would the team do another hunt?** | Yes. Newbie explicitly asked for more. Captain noted the hunt supports team play well. Skeptic's only complaint was polish, not structure. Speedster got enough fast-solve dopamine to stay engaged. |

---

## Findings

### Design Issues Exposed

**ISSUE 1 — Bracket FINAL not used in extraction (Puzzle II)**
- **Severity:** Minor (clarity)
- **Description:** The bracket diagram shows a FINAL match (SF-1 winner vs SF-2 winner) but the worksheet only extracts letters from QF-1 through SF-2. Solvers working from the diagram may be confused.
- **Recommendation:** Add a note to the bracket diagram: *"Extract letters from the six highlighted matches only. The final is for your satisfaction."* Alternatively, remove the FINAL from the diagram or add it as a 7th extraction that produces a confirmation letter.

**ISSUE 2 — Map 3 (Gold Rush) letter ambiguity (Puzzle IV)**
- **Severity:** Minor (visual)
- **Description:** The 9-dot pattern for "W" on Map 3 could be misread as "M" by solvers who orient the grid differently. Row 5 is empty, which helps disambiguate, but the letter is still the weakest of the five.
- **Recommendation:** Either add a resource at (5,1) and (5,5) to form explicit W "feet" at the bottom, or add a note/flavor text hint that reinforces top-to-bottom reading order.

**ISSUE 3 — Puzzle difficulty too uniform for Speedster testing**
- **Severity:** Observation (not a bug)
- **Description:** The Speedster solved 3 puzzles correctly with zero errors. The hunt did not produce any wrong submissions. This means the difficulty calibration is slightly easy for experienced puzzle hunters — Puzzle III especially (sub-2-minute solve). For a themed casual hunt, this is likely fine. For competitive use, consider making one feeder significantly harder.
- **Recommendation:** No change needed for the target audience (AoE fans, not MIT Mystery Hunt veterans). If scaling difficulty, Puzzle V is the natural place — add a 7th Moment or make one calculation require multi-step reasoning.

**ISSUE 4 — Puzzle V Moment 1 unit ambiguity is actually elegant**
- **Severity:** Non-issue (positive finding)
- **Description:** Both Archer and Crossbowman cost 25W 45G. The puzzle's answer (16) is the same regardless of which unit the solver identifies. This is not a bug — it's an elegant design choice that prevents a common error from blocking progress. Worth preserving.

**ISSUE 5 — No puzzle explicitly teaches extraction to the Newbie**
- **Severity:** Minor (on-ramp)
- **Description:** Puzzle I is the most Newbie-accessible, but the extraction step ("circle the Nth letter, collect them, find the word") is explained in the puzzle text without a worked example. The Newbie needed the Captain to explain what "Circle letter 1" meant. Consider adding a brief worked example in the hunt intro or on Puzzle I itself: *"Example: If the answer were FRANKS and the instruction said 'Circle letter 3,' you would mark the letter A."*
- **Recommendation:** Add a one-line worked example to the Puzzle I instructions OR to the hunt's opening page.

**ISSUE 6 — Meta clue numbering is non-standard**
- **Severity:** Cosmetic
- **Description:** The meta grid labels down words as "1 down, 2 down, 3 down" and across words as "6 across, 4 across." The numbered squares [1]-[6] in the grid are EXTRACTION markers, not clue numbers — but they visually overlap with the down clue numbering. "1 down" and "[1]" refer to different things (the clue vs. the extraction square). This could confuse solvers.
- **Recommendation:** Use a different visual language for extraction markers — e.g., circled letters (A)-(F) instead of [1]-[6], or use star symbols instead of numbers.

---

## Summary

The Wololo hunt is a clean, thematically coherent 5-puzzle hunt with a satisfying meta. All puzzles produce unique answers, the meta is robust under the 80% rule, and the difficulty curve supports both newcomers and experienced solvers. The WOLOLO reveal provides genuine thematic closure. Six findings were logged — zero hard bugs, two minor clarity issues, and four observations/polish items. The hunt is ready for human playtesting with the minor fixes noted above.
