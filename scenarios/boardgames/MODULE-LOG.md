# Module Log — Game Night

**Multi-author dynamics record for Stages 5-7**

This log captures the human side of the authoring process: who asked for help, who argued, who went silent, and how the Captain (admin) managed it all.

---

## Simulator follow-up

After the live scenario, `tools/hunt-sim` replayed Game Night as a seeded
multi-author simulation. The comparison run found that the original serial admin
workflow is the schedule bottleneck: baseline integration exceeded the target
window, while `parallel-review` moved most runs back inside it. The strongest
variant was `ship-room`, which combines:

1. parallel review lanes for submitted modules,
2. explicit editorial relief for Speedster/Social-style cleanup,
3. lightweight meta briefing before final answer lock,
4. admin standups to close Lurker-style visibility gaps.

Recommendation for future Game Night-style hunts: keep module isolation, but do
not keep review serial. The Captain should run a ship-room cadence once the
first module lands.

---

## Timeline

### Hour 0:00 — Briefs Issued

The Captain distributed five briefs simultaneously. Each author received their brief, the world/ data file for their game, the principles document, and the answer word constraints.

**Responses within 5 minutes:**

| Author | Response | Captain's reaction |
|--------|----------|--------------------|
| **The Methodical** (M1) | "Received. Reading now. Will have questions." | Expected. Noted. |
| **The Speedster** (M2) | "got it lol" | ...concerning but typical. |
| **The Skeptic** (M3) | "I have concerns about the No Computation Without Deduction principle as applied to my puzzle." | Here we go. |
| **The Social** (M4) | "Thanks! Quick question — who has the chess module? I think there might be overlap in our approaches." | Connected M4 to M1. Good instinct. |
| **The Lurker** (M5) | *(no response)* | File access timestamp confirmed they opened it. That's all we've got. |

---

### Hour 0:05 — The Methodical's Questions

M1 sent four clarifying questions before touching a single chess piece:

1. "The brief says 'the notation of that move, combined with other extracted letters, yields the answer word.' Does 'combined with' mean concatenated, or is there an additional transformation?"
2. "When you say 'overlapping clues,' do you mean clues that share piece constraints, or clues that reference each other explicitly?"
3. "Is there a target word count for the puzzle page? The brief says nothing about length."
4. "The answer word constraint says 5-8 letters. Does the mechanism need to produce the answer through extraction, or can the answer be directly named by the puzzle element (e.g., the name of the move)?"

**Captain's responses:**
1. "Author's choice. Make it clean."
2. "Either or both. The point is interlock."
3. "No hard limit. Write what the puzzle needs."
4. "Either approach is fine. Just make sure the solver EARNS the answer."

M1 acknowledged and went silent. For two hours.

---

### Hour 0:08 — The Skeptic's Argument

M3 opened a channel to the Captain:

> "I need to push back on the brief. It says 'cryptic descriptions' to identify continents. That's a reading comprehension exercise dressed up as a puzzle. The Riven Standard says 'the puzzle IS what the section does.' Risk is a game of territorial STRATEGY — troop counts, borders, force projection. The way you REALLY identify continents in Risk is by their structural properties: territory count, bonus, entry points, adjacency graph topology.
>
> The principles also say 'No Computation Without Deduction.' But structural fingerprinting IS deduction. I'm looking at a continent's numerical signature and deducing its identity. That's MORE rigorous than riddle-style clues.
>
> I want to write the puzzle using numerical properties. Can I?"

**Captain's response:**

> "I hear you. You're not wrong about the structural approach — it IS more rigorous. But the brief asks for descriptions that a casual board gamer can engage with. Not everyone memorizes territory counts.
>
> Compromise: Write the descriptions the way the brief asks (accessible, cryptic, thematic). But embed the numerical properties AS PART OF each description. The numbers are the backbone; the prose is the surface. Both audiences get served.
>
> If your final puzzle is good and the extraction works, I don't care how you get there."

**M3's response:**

> "Fine. But I'm putting a note in the puzzle file explaining my position."

**Captain's note to self:** *Let them. The note won't appear in the solver-facing version. Letting the Skeptic vent in the file is cheaper than fighting about it. Pick your battles.*

---

### Hour 0:10 — The Social Reaches Out

M4 messaged M1 directly:

> "Hey — I'm working on Pandemic, Patient Zero. Your chess puzzle has a forensic reconstruction element too, right? Working backward from a final state to figure out what happened?
>
> I'm thinking about the Pandemic discard pile as having LAYERS from the Intensify shuffle. Each Epidemic creates a stratum boundary. The solver reads the layers like an archaeologist reads dirt.
>
> Does that resonate with how you're thinking about the chess position? The observations as constraints that eliminate possibilities?"

**M1's response (20 minutes later):**

> "Yes. Each of my observations eliminates impossible positions. The interlock comes from observations that reference each other — you can't resolve Observation 7 without first narrowing Observations 3 and 4. The solve path is: read all observations, identify the dependencies, solve the most constrained variable first, propagate.
>
> Your layer idea is the same structure. The bottom of the discard pile is the most constrained (it's the oldest, least reshuffled). Start there. Each Intensify creates a known boundary. The re-infections (cities appearing in multiple layers) are your interlock — they confirm the layer boundaries.
>
> One thing to watch: make sure the cube counts are CONSISTENT with the discard pile. If a city appears 3 times in the pile, it should have 3 cubes. If it has fewer, you need to explain why (outbreak? cure? medic?). Keep it simple — let the cube count equal the appearance count. That's clean."

**M4:** "That's exactly what I needed. Thank you."

**Captain's note:** *This is the kind of cross-pollination I want to see. The Social's instinct to reach out produced a better puzzle. M1's advice about consistency will save M4 from a design error I've seen three times before.*

---

### Hour 0:12 — The Speedster Ships v1

M2 dropped a puzzle file into the module directory.

> "done. market day. resources track clean. answer is TRADE. ship it."

**Working time: 12 minutes.**

Captain opened the file. Read it. Found three issues:

**Issue 1: Ambiguous port trade.** Action 2 says "trades Wool for Ore at a port" and "After this trade, the player has 2 Ore." The player started with 1 Ore. So they gained 1 Ore from the trade. But the puzzle doesn't specify which port — the solver is supposed to deduce it. The solver can figure out it's a 3:1 generic port (trading 3 Wool for 1 Ore) because the player started with 4 Wool and a 2:1 Wool port would only cost 2 Wool. But wait — after a 3:1 trade, the player has 1 Wool left. After a 2:1 trade, 2 Wool left. Both are consistent with subsequent actions depending on whether later actions need Wool. Let me check...

Actually, the Speedster didn't fully verify this. The solver needs to track whether 1 or 2 remaining Wool changes the final answer. If the answer is "what can you build with the leftovers," the Wool count matters.

**Issue 2: Building cost for Settlement may be wrong.** The puzzle describes building a Settlement in Action 3. Settlement costs 1 Brick + 1 Lumber + 1 Grain + 1 Wool. After Action 1 (Road: -1 Brick, -1 Lumber) and Action 2 (port trade: -3 Wool, +1 Ore OR -2 Wool, +1 Ore), does the player have enough? Starting: 3B, 2L, 1O, 2G, 4W.

After Road: 2B, 1L, 1O, 2G, 4W.
After 3:1 port trade: 2B, 1L, 2O, 2G, 1W.
After Settlement (1B+1L+1G+1W): 1B, 0L, 2O, 1G, 0W.
After Dev Card (1O+1G+1W): needs 1 Wool but has 0. **CANNOT BUY DEV CARD.**

That's a bug. The Speedster didn't track the Wool through.

If it's a 2:1 port trade: 2B, 1L, 2O, 2G, 2W after trade.
After Settlement: 1B, 0L, 2O, 1G, 1W.
After Dev Card (1O+1G+1W): 1B, 0L, 1O, 0G, 0W.
After 2nd Road (1B+1L): needs 1 Lumber but has 0. **CANNOT BUILD 2ND ROAD.**

Also a bug.

**Issue 3: The answer.** The puzzle asks "what is the one structure they could build with remaining cards?" and says the answer is TRADE (5 letters). But TRADE is not a structure in Settlers. The structures are: Road, Settlement, City, Development Card. The puzzle concept says the NAME of the structure is the answer. TRADE isn't one of them. The Speedster chose TRADE as the answer word for thematic reasons but didn't connect it to the mechanism.

**Captain's action:** Filed three issues. Sent feedback to M2:

> "Three bugs. (1) The port trade is ambiguous in a way that breaks subsequent actions — both 3:1 and 2:1 lead to impossible builds later. Re-run the resource math from scratch. (2) The Wool runs out before the Dev Card purchase if 3:1 port, and Lumber runs out before the 2nd Road if 2:1 port. Your action sequence is impossible. (3) TRADE is not a Settlers structure name. How does the solver extract TRADE from the mechanism?
>
> Fix the resource math. Fix the extraction. Keep the clever idea — the deduction-of-port-type is good."

**M2's response:**

> "wait what. let me check... ok the wool thing is a problem. i'll fix it. give me 10 min."

**Captain's note:** *This is exactly what the Speedster profile predicts. Ships fast, has clever ideas, doesn't verify the math. The port-deduction concept is genuinely good. The execution needs a second pass. If M2 doesn't fix it in 20 minutes, I'll hand it to M1 for cleanup.*

---

### Hour 0:15 — Checking on The Lurker

Captain sent a status check to all authors.

| Author | Status requested | Response |
|--------|-----------------|----------|
| M1 | "How's the chess puzzle coming?" | "Working through case analysis. Have 7 of 8 observations nailed down. The disambiguation is tricky — multiple positions satisfy the first 7 constraints. Working on Observation 8." |
| M2 | "Fix those bugs yet?" | "working on it" |
| M3 | "How's Risk coming?" | "Writing. The numerical approach works. Each continent has a unique signature across (territory count, bonus, entry points). I've verified all six are uniquely identifiable. Writing the prose wrapper now." |
| M4 | "Pandemic status?" | "Making good progress! Talked to M1 about the reconstruction approach — super helpful. Building the discard pile now. Making sure cube counts match appearance counts like they suggested." |
| M5 | "Codenames status?" | *(no response)* |

**Captain, 5 minutes later:** "M5, acknowledge please."

**M5:** *(no response)*

**Captain's note:** *The Lurker. File access timestamps show they're actively editing puzzle.md. They're working. They just won't tell me. This is the profile working exactly as documented. I'll check the file directly in 30 minutes.*

---

### Hour 0:45 — The Skeptic's Extraction Dilemma

M3 messaged the Captain:

> "I've identified the six descriptions. Each maps to exactly one continent — verified against the world data three times. Now I need to extract letters to spell BORDER. The brief says 'indexed letters from territory names.' I've been trying to find a clean extraction path and it's finicky.
>
> Australia = Landmass I. The territory bordering Asia is Indonesia. 3rd letter of Indonesia is 'D.' That's my D.
>
> South America = Landmass II. The territory connecting across ocean is Brazil (borders North Africa). 1st letter = B. That's my B.
>
> North America = Landmass III. Territory bordering Europe is Greenland. 1st letter = G. I need O, not G.
>
> This is the problem. The territory names don't cooperate with the word BORDER unless I cherry-pick awkward extraction indices. Do I:
> (a) Change my answer word to something the territory names support?
> (b) Use less natural extraction rules (e.g., 4th letter of Greenland = E)?
> (c) Redesign which territories I extract from?
>
> Option (c) seems best but I want your take."

**Captain's response:**

> "Option (c). The extraction rule per landmass can vary — 'take the 3rd letter' for one, 'take the 1st letter' for another. As long as each rule is clearly stated and the solver doesn't have to guess which letter to take, it's clean. Design the territory selection and index to spell BORDER."

**M3:** "On it."

---

### Hour 1:10 — The Social's Redesign

M4 hit a wall:

> "OK so the discard pile layer analysis works perfectly. I can show the three layers. The solver identifies re-infections to find the boundaries. That's the aha and it's clean.
>
> But my extraction is a mess. I need the first letters of cities (in order of first infection) to spell SPREAD. That means I need cities starting with S, P, R, E, A, D. The Pandemic city list has cities starting with all of those letters: Santiago, Paris, Riyadh, Essen, Atlanta/Algiers, Delhi.
>
> The problem is making the discard pile CONSISTENT — the layers have to be mechanically correct according to Pandemic's infection rules, AND the first-infection order has to spell the word. I've gone through three complete redesigns.
>
> I talked to M1 again. They said: 'Work backward from the answer. Fix the word first, then build the pile around it.' That's what I'm doing now."

**Captain's note:** *M4 is collaborating effectively. The Social's instinct to share progress and get help is working as designed. The puzzle concept is strong — the issue is mechanical consistency, which is exactly the kind of thing M1's careful approach catches.*

---

### Hour 1:38 — The Social Delivers

M4 submitted puzzle.md. The file contains extensive author's notes showing the iteration process — three redesigns, conversations with M1, running math corrections. The final version is clean: the discard pile has consistent layers, cube counts match appearance counts, and the first-infection order spells SPREAD.

**Captain's assessment:** The puzzle is good but the file is messy. The solver-facing content is buried under author's working notes. Needs editing to separate the puzzle from the process. But the mechanism and answer are solid.

---

### Hour 1:45 — The Lurker Delivers

Captain checked M5's module directory. puzzle.md exists. It was last modified 3 minutes ago. No notes.md. No communication. No acknowledgment of any status requests.

Opened the file. Read it.

The puzzle is... actually good. The grid is well-designed: ENCODE, SECRET, and LOCK all genuinely connect to the concept of CIPHER. The assassin (BOMB) is cleanly separated — no accidental overlap with cryptography. The key card distribution works. The Blue team words include several that DON'T connect to CIPHER (BRIDGE, OCEAN, CROWN, etc.), creating noise that the solver must filter.

**One concern:** The puzzle includes a section titled "The Connection" that explains the answer. In a real puzzle hunt, the solver would NOT be shown this — they'd have to find the connection themselves. The section needs to be removed from the solver-facing version and moved to an answer key or author notes.

**Captain's action:** Would normally send feedback, but... it's the Lurker. Sending a message that says "good puzzle, one edit needed" would get no response. Filed the note. Will edit during final assembly.

---

### Hour 2:00 — The Speedster's v2

M2 resubmitted after the Captain's bug report. The response:

> "fixed! resource math works now. checked it twice. the port deduction still works — solver has to figure out it's a 3:1 from the ratio. answer is TRADE. extraction is from the port trade step."

**Captain's review of v2:** The resource math is... still not fully explained. The puzzle describes 5 actions but the path from "what the player built last" to the answer TRADE is unclear. The puzzle says "the name of the thing they can build — that is your answer" but the buildable structure would be something like "Settlement" or "City," not "TRADE." The extraction mechanism connecting the structure to the word TRADE is missing.

**Captain's decision:** The puzzle concept is salvageable. The port-deduction idea is clever. But the extraction needs a complete rethink. This is the Speedster profile in action: great idea, sloppy execution, resistant to revision. The Captain will either:
(a) Send another round of feedback and wait for v3, or
(b) Hand the file to M1 for cleanup.

For now, the puzzle is ACCEPTED WITH RESERVATIONS. The core mechanism works. The extraction and answer integration need work in the editing pass.

---

### Hour 2:14 — The Methodical Delivers

M1 submitted puzzle.md. It is, as predicted, enormous. The file contains:
- The puzzle as the solver would see it (8 observations + question)
- Extensive author's notes inline, showing every case analysis, every disambiguation step, every verification
- A complete worked solution
- Cross-references to the world data for every chess fact
- A confidence statement

**Captain's assessment:** The puzzle is correct. The position is unique. The move (castling) is the intended answer. The answer word CASTLE works. The mechanism satisfies every principle.

**The problem:** The file is 200+ lines and the author's notes are mixed in with the solver-facing content. The solver would see the author's case analysis, which completely spoils the puzzle. This needs aggressive editing to separate puzzle content from author's working.

But the underlying puzzle is solid. Best submission of the five.

---

### Hour 2:14 — All Five Submitted

| Module | Author | Status | Quality | Issues |
|--------|--------|--------|---------|--------|
| M1 Chess | Methodical | Submitted | Excellent (needs editing for length) | Author's notes mixed with puzzle. 200+ lines. Solver would see worked solution. |
| M2 Settlers | Speedster | Submitted (v2) | Concept good, execution flawed | Resource math may still have bugs. Extraction to TRADE unclear. Needs v3 or editing. |
| M3 Risk | Skeptic | Submitted | Good (with editorial commentary) | Author's dissenting opinion embedded. Extraction works. Puzzle is sound despite arguments. |
| M4 Pandemic | Social | Submitted | Good (needs cleanup) | Three redesigns visible in file. Final version is clean. Working notes need removal. |
| M5 Codenames | Lurker | Submitted | Good (one structural issue) | "The Connection" section spoils the puzzle. Must be removed. No author communication. |

---

## Answer Words Collected

| Module | Word | Status |
|--------|------|--------|
| M1 | CASTLE | Locked |
| M2 | TRADE | Locked |
| M3 | BORDER | Locked |
| M4 | SPREAD | Locked |
| M5 | CIPHER | Locked |

All 5 answer words are in. The meta (M6) can now be designed.

**Meta input**: CASTLE, TRADE, BORDER, SPREAD, CIPHER

**Captain's challenge**: Build a meta from 5 uncoordinated words. This is Goal G5.

---

## Multi-Author Dynamics Summary

### What Worked

1. **The Social's cross-pollination**: M4 reaching out to M1 produced a genuinely better puzzle. The "constraint elimination" insight from chess improved the Pandemic puzzle's structure. This validates the Social profile as a collaboration catalyst.

2. **The Skeptic's pushback was productive**: M3's argument about numerical vs. prose descriptions led to a compromise (embed numbers in prose) that made the puzzle more rigorous WITHOUT sacrificing accessibility. The Skeptic found a real tension in the brief and the resolution improved the output.

3. **The Methodical's thoroughness**: M1's case analysis caught every edge case in the chess position. The puzzle is provably unique. No other author achieved this level of verification.

4. **The Lurker delivered**: Zero communication, zero visibility, but the puzzle landed on time and it works. The Captain had to accept having no control over this module.

### What Didn't Work

1. **The Speedster's bugs**: M2 shipped with three errors in 12 minutes. The port-trade deduction concept is clever, but the resource math wasn't verified. The answer word TRADE doesn't connect to the mechanism cleanly. This module needs the most editing work.

2. **The Lurker's communication gap**: The Captain has zero insight into M5's process. If the puzzle had been broken, the Captain wouldn't have known until checking the file. In a real team, this creates an unmanageable blind spot.

3. **Author's notes in puzzle files**: M1, M3, and M4 all included working notes, commentary, or iteration history in puzzle.md. These spoil the puzzle for solvers. Every file needs an editing pass to separate author-facing content from solver-facing content.

4. **No coordination on answer words**: By design (Goal G5), the answer words are uncoordinated. The resulting set (CASTLE, TRADE, BORDER, SPREAD, CIPHER) is workable but not trivially elegant for meta construction. The Captain will need to be creative.

### Personality Conflicts

| Conflict | Resolution |
|----------|-----------|
| Skeptic vs. Brief | Compromise: embed numbers in prose. Let Skeptic add editorial note. |
| Speedster vs. Bug reports | Two rounds of revision. Captain accepted v2 with reservations. |
| Lurker vs. Status checks | Captain gave up after two unanswered messages. Checked file directly. |
| Social's dependency on Methodical | Net positive — but if M1 had been stuck, M4 would have been blocked. |

### Captain's Retrospective

> "Five authors, five personalities, five puzzles. Three delivered clean on first pass (M1, M3, M5). One needed collaboration to work (M4). One needed multiple revisions and still isn't fully clean (M2).
>
> The biggest risk was the Lurker — I had zero visibility. The biggest time sink was the Speedster — bugs in v1, bugs in v2, may need v3. The most valuable interaction was Social-to-Methodical — their conversation made both puzzles better.
>
> The Skeptic is right about the numerical approach, by the way. I just can't tell them that."

---

## Next Steps

1. **Editing pass**: Strip author notes from all puzzle files. Create solver-facing versions.
2. **M2 cleanup**: Fix the extraction mechanism. Verify resource math end-to-end.
3. **Meta design (M6)**: The Captain has the five words. Time to build the capstone.
4. **Testing**: Run each puzzle through the solver panel per the test framework.
