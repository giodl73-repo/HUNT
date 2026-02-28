# THE GRAND LARCENY -- Stage 10: Platform Test

**Date:** 2026-02-27
**Tester:** Simulated (2-solver team)
**Format:** Dossier physical test + live solve simulation + site check

---

## 1. Dossier Physical Test

### 1.1 Cover Sheet (PR02)

**Does it set up the fiction convincingly?**

YES. The cover sheet establishes the assignment cleanly in Det. Callahan's voice -- terse, official, slightly weary. The "CITY POLICE DEPARTMENT / Investigative Division" header in the double-ruled box is immediately institutional. The case fields (case number, date, classification, location) read like a real police briefing. The assignment box lists all four exhibits by label (A-D) and dimension (SUSPECT, LOCATION, TIME, OBJECT), giving the team a clear roadmap without over-explaining. The closing line -- "The trail is going cold." -- lands because everything before it is dry.

The HTML version (`cover-sheet.html`) renders correctly: header box, field rows, assignment box with bullets, signature block. The optional CONFIDENTIAL watermark is present and unobtrusive (rgba 0.06 opacity). Print margins at 0.75in are appropriate.

**Issues:**
- None. The cover sheet is strong.

---

### 1.2 Puzzle Documents -- Period Authenticity

**P1 -- Police Report (Exhibit A):** Reads convincingly as a 1947 incident report. The header box has the correct institutional weight. Suspect blocks are structured with Statement/Corroborating/Contradicting/Unaccounted fields -- realistic police format. The Critical Deduction section (clue box with left border) guides without solving. The HTML extraction markers (underlined words with boxed letters) are visually clear.

ISSUE (P1-HTML-01): In the HTML file (`P1-police-report.html`), the extraction words #2 and #3 have incomplete underlining. Word #2 "annual" is marked as `ann<span class="extract-word">u<span class="extract-letter">U</span>al</span>`, meaning only "uUal" gets underlined -- the "ann" prefix is outside the span. Same for word #3 "persons" -- `per<span class="extract-word">so<span class="extract-letter">S</span>ns</span>` -- only "soSns" is underlined. The full word should be underlined in each case to match the extraction instructions ("seven underlined words"). This is a cosmetic but potentially confusing rendering bug.

**P2 -- Hotel Receipt (Exhibit B):** Reads as a period-appropriate hotel billing statement. The four folios are laid out in accounting format with timestamps, service categories, items, and right-aligned dollar amounts. The $0.01 beer (Kessler, 9:01 PM) is the intentional anomaly -- absurdly cheap, flagged in the design notes as the biggest "tell." The Accounts Department Notes section provides a natural nudge toward the theft-window charges.

ISSUE (P2-HTML-MISSING): The print-ready HTML file `P2-hotel-receipt.html` has not yet been built. The layout spec exists (`layout-P2-hotel-receipt.md`) and the source markdown is complete, but the HTML rendering is outstanding from Stage 9.

**P3 -- Floor Plan (Exhibit C):** The ASCII floor plans in the source markdown convey the spatial layout effectively. Four levels (3F, 2F, 1F, B) arranged top-to-bottom, matching the building's structure and the thief's descending route. The connecting door between Gallery (200) and Linen Closet (206) is shown with the thin-line symbol and explicitly noted below the 2F diagram -- discoverable but not obvious. Reference markers (circled Unicode numbers) are placed at key locations.

ISSUE (P3-HTML-MISSING): The print-ready HTML file `P3-floor-plan.html` has not yet been built. The layout spec recommends converting the ASCII art to clean vector graphics. The ASCII rendering in Courier New would work as a minimum viable version but is not beautiful.

ISSUE (P3-MARKER-01): There are two markers with the value 9 -- one at the Gallery (200) center and one at the 2F-HALL junction. The solver needs to distinguish between them. The route log asks for exactly 4 stops (303, 200, 206, B02), which constrains the path and prevents taking the hallway marker. However, the duplicate number could still cause momentary confusion. The reference key table lists both, which helps. Acceptable as designed, but worth noting.

**P4 -- Witness Statement (Exhibit D):** Reads naturally as a police interview transcript. Delacroix's voice is nervous, earnest, and full of pauses. The acrostic is invisible on casual reading -- the sentences flow as natural speech. The paragraph breaks and [pause] stage directions feel authentic. The follow-up exchange with Det. Callahan about seeing a shadow on the staircase provides good cross-puzzle interlock with P1 (Fontaine).

ISSUE (P4-HTML-MISSING): The print-ready HTML file `P4-witness-statement.html` has not yet been built. The layout spec exists and is detailed.

---

### 1.3 Hotel Key Card (PR08)

**Physically producible?** YES. 3.5x2 inches on 110lb+ white cardstock. The HTML (`hotel-key-cards.html`) produces a 3x4 grid of 12 fronts per letter-size sheet. Instructions for double-sided printing (flip short edge) and cutting are clear.

**Clear for printer?** YES. The front has ROOM 303 / ROOF ACCESS in Courier New 16pt bold, hotel name in Georgia, pointing hand symbol (U+261E). The back has the "north stairwell on the third floor" text. Both sides are fully specified in the HTML.

**Issue:** None. The key card spec is complete and producible.

---

### 1.4 Calling Card (PR09)

**Physically producible?** YES. 3x2 inches on cream/light grey 110lb+ cardstock. The slightly smaller size distinguishes it from the hotel key card. The HTML (`calling-cards.html`) produces 12 fronts per sheet in a 3-column grid.

**Clear for printer?** YES. Front: diamond symbols (U+25C6), motto in Georgia italic, numbers "3 . 1 . 2 . 6" in Courier New 14pt bold. Back: EVIDENCE TAG with item/found/logged/case fields. All specified in HTML.

**Issue:** None. The calling card spec is complete and producible.

---

### 1.5 Distribution Plan (DISTRIBUTION.md)

**Clear enough for a volunteer?** YES. The assembly instructions are step-by-step (8 steps), clearly ordered, and include specific details:
- Which documents are stapled (P1, P2, P4) and which are loose (P3, evidence log)
- What goes on top (cover sheet on cardstock) and bottom (evidence log, then loose cards)
- Cards described by color/material (white key card, cream calling card) and content (ROOM 303, 3-1-2-6)

The quantities table, day-of timeline, and packing checklist are practical and complete. The staff script for distribution is concise. Emergency supplies are specified.

**Issue:** None. The distribution plan is production-ready.

---

## 2. Live Solve Simulation

### Setup

**Solver A:** Experienced puzzler. Has done 5+ puzzle hunts. Familiar with extraction mechanisms, acrostics, A1Z26 encoding.
**Solver B:** First-time puzzler. Smart, engaged, but has never seen a puzzle hunt before.

---

### 2.1 Opening the Dossier

Solver B opens the manila folder. The dossier label reads "CASE FILE No. 47-1011-GL / Classification: GRAND LARCENY / STATUS: OPEN." Both solvers note the CONFIDENTIAL warning. Solver B: "This feels real."

Cover sheet is on top. They read it together.

**Does the team understand the setup from the cover sheet alone?** YES. The summary is clear: painting stolen, cut from frame, 9:15-9:25 PM, four suspects. The assignment lists all four exhibits with their dimensions (SUSPECT, LOCATION, TIME, OBJECT) and the goal (THE METHOD). The instruction to use the evidence log and calling card for the final answer is explicit.

Solver A notes the loose hotel key card and calling card at the bottom of the folder. "Don't lose these -- they're props for the puzzles."

They decide to split up: Solver A takes P1 (Police Report) and P3 (Floor Plan). Solver B takes P2 (Hotel Receipt) and P4 (Witness Statement).

---

### 2.2 P1 -- The Police Report

**Solver A reads P1.**

**Minutes 0-5:** Reads the summary and four suspect blocks. Notes the unaccounted windows: Ashworth 30 min, Fontaine (placed on wrong floor), Kessler 66 min, Delacroix 32 min. "Everyone has a gap."

**Minutes 5-10:** Reads Physical Evidence section. Notes the service elevator at 9:20 PM (unlogged), the key card log table, and the 10-minute theft window. The Critical Deduction section's five clues are read carefully.

**Minutes 10-15:** Works through the logic. Clue 2 is the key breakthrough: Kessler entered Room 204 at 9:28 PM. If he committed the theft (gallery ~9:20, elevator DOWN to basement), he could not have returned to his room by 9:28 -- the thief exited via the basement alley, but Kessler went to his room. "Kessler's the one who DIDN'T do it -- his key card proves he went back to his room, not out the alley." The critical deduction section explicitly states this reasoning.

**Minutes 15-20:** Searches for the seven underlined words with boxed letters. Finds: Saturday (S), annual (U), persons (S), approximately (P), Evening (E), mechanical (C), question (T). Writes: S-U-S-P-E-C-T.

**Sticking point:** Minor. Solver A initially reads too quickly through the dense text and misses word #6 "mechanical" on the second page. Re-reads Physical Evidence section and finds it. The extraction markers in the printed version are visually clear (underline + box), so scanning for them works.

**Answer: SUSPECT** (7 letters). Confirmed correct.

**Time: ~20 minutes.** Difficulty: Medium.

---

### 2.3 P2 -- The Hotel Receipt

**Solver B reads P2.**

**Minutes 0-5:** Reads through all four folios. Notes the prices, timestamps. "There are a lot of numbers here." Checks arithmetic on one folio -- totals add up. Reads the Accounts Department Notes: "charges timestamped between 9:00 PM and 9:30 PM may be of particular interest."

**Minutes 5-10:** Reads the Investigator's Analysis section. This guides strongly: "examine the cents values of suspicious charges." The section explicitly lists the gap-period charges for each folio and says "Convert cents to letters (A=01, B=02...)." This is very direct guidance.

**Sticking point:** Solver B doesn't immediately know A1Z26. Asks Solver A: "What's A=01, B=02?" Solver A: "Just the alphabet numbered. A is 1, B is 2, Z is 26." Solver B nods and proceeds.

**Minutes 10-18:** Converts cents values:
- Ashworth: 12=L, 15=O
- Fontaine: 03=C
- Kessler: 01=A, 20=T, 09=I
- Delacroix: 15=O, 14=N

Reads in folio order (202, 203, 204, 205): L-O-C-A-T-I-O-N.

**Sticking point:** Solver B notices the $0.01 beer and laughs: "A one-cent beer? In 1947?" This is the designed "tell" -- it works as intended. The $0.03 garment pressing is also suspiciously cheap but less obviously wrong.

ISSUE (P2-TOTAL-MISMATCH): The base receipts in `services.md` have different totals than the puzzle versions in `P2-hotel-receipt.md`. For example, Ashworth's base receipt in services.md totals $30.45, but the puzzle version (with added gap charges of $0.12 and $0.15) totals $30.72. This is by design -- the puzzle adds "mystery charges" during gap periods. However, the puzzle's Dining & Drinks subtotal ($18.62) needs verification: $8.50+$1.50+$2.00+$1.50+$1.00+$0.35+$0.12+$0.15+$1.75+$1.75 = $18.62. Correct. Folio total: $18.62+$12.00+$0.10 = $30.72. Correct.

Kessler verification: $8.50+$1.75+$1.75+$3.15+$0.35+$0.01+$0.20+$0.09 = $15.80. Folio total: $15.80+$12.00+$0.25+$0.10 = $28.15. Correct.

Fontaine verification: $8.50+$2.00+$2.00+$0.03+$1.50 = $14.03. Folio total: $14.03+$12.00+$0.80 = $26.83. Correct.

Delacroix verification: $8.50+$0.15+$0.35+$0.15+$0.14 = $9.29. Folio total: $9.29+$12.00+$0.40+$0.00 = $21.69. Correct.

All arithmetic checks out.

**Answer: LOCATION** (8 letters). Confirmed correct.

**Time: ~18 minutes.** Difficulty: Medium. The Investigator's Analysis section is quite guided, which helps Solver B.

---

### 2.4 P3 -- The Floor Plan

**Solver A takes P3 after finishing P1.**

**Minutes 0-5:** Studies the four floor diagrams. Notes the hotel key card prop: ROOM 303, ROOF ACCESS. Locates Room 303 on the 3F plan -- it has marker (20). Back of key card says "north stairwell on the third floor" -- this connects to the fire escape on the north face.

**Minutes 5-10:** Reads the Investigator's Task. "A hotel key card was found on the fire escape landing." Now the starting point is clear: the thief entered via 303 (Roof Access). The task asks to trace the route: entry, path to gallery, escape from gallery, exit.

Traces route:
1. Stop 1: 303 (Roof Access) -- marker 20. Thief entered here.
2. Stop 2: 200 (Sargent Gallery) -- marker 9. Got to gallery via service elevator (3F-ELEV to 2F-ELEV) then through 2F hall... or did the thief go directly?

**Sticking point:** How did the thief get from 303 to 200? The service elevator goes 3F to 2F. But the solver needs to trace the route on the map. 303 is on 3F near the elevator. Take elevator to 2F. Then walk to gallery. The marker at the 2F-HALL junction is also (9) -- same as the gallery. Solver A initially wonders: is the hallway marker part of the route? But the route log asks for exactly 4 stops, and the task frames them as (entry, path to gallery, escape, exit), so the hallway is transit, not a stop. The gallery itself (200, marker 9) is Stop 2.

3. Stop 3: Escape from gallery. Solver A looks for how to leave Room 200 without the hallway. Notices the thin line between 200 and 206. Reads the note: "The thin line between rooms 200 and 206 indicates a connecting door." The Linen Closet (206) has marker (13). Stop 3: 206, marker 13.

4. Stop 4: Exit. From 206, take service elevator down to basement. Then through staff corridor to laundry (B02), which has marker (5) near the alley exit. Stop 4: B02, marker 5.

**Minutes 10-15:** Converts markers to letters:
- 20 = T
- 9 = I
- 13 = M
- 5 = E

Spells: T-I-M-E.

**Sticking point (resolved):** Solver A briefly considers whether the route goes 303 -> 200 -> 206 -> B04 -> B02, which would be 5 stops. But the route log has exactly 4 blanks, and the task frames 4 questions (entry, path, escape, exit). B04 (Storage) has no marker. The 4 markers on the path are at 303, 200, 206, B02 -- exactly 4. Clean.

**Answer: TIME** (4 letters). Confirmed correct.

**Time: ~15 minutes.** Difficulty: Medium-Hard. The connecting door discovery is the aha moment.

---

### 2.5 P4 -- The Witness Statement

**Solver B takes P4.**

**Minutes 0-5:** Reads the interview transcript. Delacroix's nervous voice is engaging. Solver B notes the details that confirm things from other puzzles: she saw Ashworth in the gallery, she heard the elevator, she saw a shadow near the third-floor stairs (Fontaine).

**Minutes 5-8:** Reads the Investigator's Analysis section. "Take the first letter of each sentence in her main statement." Solver B starts going through the sentences.

**Minutes 8-18:** Writes down first letters:
I-S-A-W-T-H-E-C-A-N-V-A-S-I-N-S-I-D-E-A-W-O-O-D-E-N-O-B-J-E-C-T

Reads: "I SAW THE CANVAS INSIDE A WOODEN OBJECT"

Last 6 letters: O-B-J-E-C-T.

**Sticking point:** Solver B miscounts sentences initially -- groups two sentences as one because of a comma splice or em-dash. Goes back and recounts. The analysis section helpfully lists all 32 sentences with their opening fragments, which serves as a cross-check. After re-reading, gets the correct count.

ISSUE (P4-SENTENCE-COUNT): The analysis section lists 32 sentences, and the text contains exactly 32 sentences in the main statement (from "I came to the gala..." through "The thought of anything being wrong..."). The final paragraph ("I went to my room and lay down...") is outside the main statement boundary as defined by the analysis section. This boundary is clear in the instructions but might briefly confuse a solver who includes the final paragraph. The instruction says "beginning with 'I came to the gala...' and ending with '...paying closer attention'" -- which correctly scopes the 32 sentences. HOWEVER, the actual ending sentence is "The thought of anything being wrong did not cross my mind" (sentence 32), not "paying closer attention." The analysis section says "ending with '...paying closer attention'" but the testimony text doesn't end with that phrase -- it ends with "[pause] I wish I had been paying closer attention" AFTER the main 32-sentence block. This is actually fine: the analysis section's ending reference points to Delacroix's final line, which comes AFTER the extractable block. The 32 enumerated sentences in the analysis section are definitive. Minor potential for confusion but the enumeration resolves it.

**Answer: OBJECT** (6 letters). Confirmed correct.

**Time: ~18 minutes.** Difficulty: Medium. The guided analysis section makes the acrostic discoverable even for Solver B.

---

### 2.6 Meta -- The Evidence Log

Both solvers reconvene with all four answers.

**Minutes 0-3:** Fill in the evidence log:
- Exhibit A (SUSPECT): S-U-S-P-E-C-T
- Exhibit B (LOCATION): L-O-C-A-T-I-O-N
- Exhibit C (TIME): T-I-M-E
- Exhibit D (OBJECT): O-B-J-E-C-T

**Minutes 3-7:** Read the Case Correlation section. "A calling card was recovered at the scene... The card bears four numbers."

Pick up the calling card. Front shows: 3 . 1 . 2 . 6

The evidence log says: "Using the calling card, take the indicated letter from each finding."

Solver A: "Four numbers, four exhibits. The first number (3) goes with Exhibit A. Take letter 3 from SUSPECT."

Extraction:
- From EXHIBIT A, take letter #3: S-U-**S** = S
- From EXHIBIT B, take letter #1: **L** = L
- From EXHIBIT C, take letter #2: T-**I** = I
- From EXHIBIT D, take letter #6: O-B-J-E-C-**T** = T

**THE METHOD: S-L-I-T**

Solver A: "SLIT. The canvas was slit from its frame."

**Is the meta answer satisfying?** YES. "SLIT" is a sharp, specific, visceral answer. It connects directly to the physical evidence described in P1 ("the canvas was cut cleanly from the frame with a sharp instrument, likely a razor or utility knife"). The solver now has the full picture: the thief entered through the roof, descended via the service elevator, slit the canvas from the frame in the Sargent Gallery, escaped through the connecting door to the linen closet, took the elevator to the basement, and exited through the alley. Ten minutes. One blade. Gone.

The calling card's motto -- "A gentleman never leaves without a signature" -- now reads as the thief's taunt: the calling card IS the signature, and the numbers on it are the key to revealing the method.

**Time: ~7 minutes** after all four feeders solved.

---

## 3. Meta Verification (Character-by-Character)

### Feeder Answers

| Puzzle | Answer | Verified |
|--------|--------|----------|
| P1 | SUSPECT (S-U-S-P-E-C-T) | Extraction words 1-7 in order: S, U, S, P, E, C, T. Correct. |
| P2 | LOCATION (L-O-C-A-T-I-O-N) | Cents A1Z26: 12=L, 15=O, 03=C, 01=A, 20=T, 09=I, 15=O, 14=N. Correct. |
| P3 | TIME (T-I-M-E) | Markers A1Z26: 20=T, 9=I, 13=M, 5=E. Correct. |
| P4 | OBJECT (O-B-J-E-C-T) | Acrostic last 6: ...O-B-J-E-C-T. Correct. |

### Meta Extraction

Calling card numbers: 3, 1, 2, 6

| Exhibit | Answer | Position | Letter |
|---------|--------|----------|--------|
| A (P1) | S-U-S-P-E-C-T | 3 | S |
| B (P2) | L-O-C-A-T-I-O-N | 1 | L |
| C (P3) | T-I-M-E | 2 | I |
| D (P4) | O-B-J-E-C-T | 6 | T |

**Result: S-L-I-T** = SLIT. VERIFIED CORRECT.

### 80% Rule Check

With 3 of 4 answers:
- Missing P1: _-L-I-T = ?LIT --> SLIT strongly implied by crime narrative
- Missing P2: S-_-I-T = S?IT --> SLIT, SPIT, SKIT -- SLIT fits the cutting
- Missing P3: S-L-_-T = SL?T --> SLIT, SLOT, SLUT -- SLIT fits
- Missing P4: S-L-I-_ = SLI? --> SLIM, SLID, SLIT, SLIP -- SLIT fits the crime

All cases: SLIT is the most thematically justified completion. 80% rule satisfied.

---

## 4. Answer Submission Site Check

### Landing Page (`index.html`)

**Built?** YES.

**Noir aesthetic consistent?** YES. Background #0d0d0d (near-black), text #c9c2a5 (warm parchment), case number in #8B0000 (dark red). Courier New body, Georgia headers. The tagline in Georgia italic captures the hook. The outlined submit button with hover-invert is clean.

**Content check:**
- Title: "THE GRAND LARCENY" -- correct
- Case number: 47-1011-GL -- matches all documents
- Tagline mirrors cover sheet phrasing -- consistent
- "All evidence is in your dossier" -- correct framing (no puzzle content on site)
- Submit button links to `submit.html` -- correct

**Issue:** None. Landing page is solid.

### Submit Page (`submit.html`)

**Built?** NO. The submit page has not yet been created. The SITE-SPEC.md defines it in detail:
- Same visual style as landing
- Form fields: Case ID/Team Name + THE METHOD (4 chars, auto-uppercase)
- Client-side validation against SHA-256 hash (Option A) or backend POST (Option B)
- Correct answer displays narrative payoff: "The canvas was [SLIT] from its frame..."
- Incorrect answer allows retry

ISSUE (SITE-SUBMIT-MISSING): `submit.html` needs to be built before the hunt can run. The spec is complete; the implementation is outstanding.

---

## 5. Summary of Issues

### Blockers (must fix before event)

| ID | Issue | Severity | File |
|----|-------|----------|------|
| P2-HTML-MISSING | P2 print-ready HTML not built | Blocker | `delivery/print/puzzles/P2-hotel-receipt.html` |
| P3-HTML-MISSING | P3 print-ready HTML not built | Blocker | `delivery/print/puzzles/P3-floor-plan.html` |
| P4-HTML-MISSING | P4 print-ready HTML not built | Blocker | `delivery/print/puzzles/P4-witness-statement.html` |
| SITE-SUBMIT-MISSING | submit.html not built | Blocker | `delivery/site/submit.html` |

### Should Fix (Stage 11 Polish)

| ID | Issue | Severity | File |
|----|-------|----------|------|
| P1-HTML-01 | Extraction words #2 and #3 have partial underlining (only suffix underlined, not full word) | Medium | `delivery/print/puzzles/P1-police-report.html` |
| P4-SENTENCE-COUNT | Analysis section says "ending with '...paying closer attention'" but that phrase is outside the 32-sentence block; minor scope confusion possible | Low | `puzzles/P4-witness-statement.md` |
| P3-MARKER-01 | Two markers share value (9) at Gallery and 2F-Hall; 4-blank route log constrains correctly but could momentarily confuse | Low | `puzzles/P3-floor-plan.md` |

### Notes (no action needed)

| ID | Note |
|----|------|
| TIMELINE-KESSLER | Kessler's detailed movement table in timeline.md says "9:30 | Room 204" but the key card log table in the same file (and P1) says 9:28 PM. The 9:28 figure is canonical per the police report and key card log; the "9:30" in the prose is a rounded approximation. Not a puzzle-breaking issue but could be aligned. |
| P2-ARITHMETIC | All folio subtotals and totals verified correct. No arithmetic errors. |
| META-COMPLETE | Meta extraction verified character-by-character. SLIT is correct and thematically satisfying. |

---

## 6. Overall Assessment

**The hunt works.** The four puzzles are solvable from their documents alone. The meta extraction is clean and verifiable. The physical dossier format is immersive. The noir tone is consistent across all documents. The difficulty spread (all mid-range) is appropriate for mixed-experience teams.

**Estimated solve time (2 experienced solvers):** 60-80 minutes.
**Estimated solve time (2 mixed solvers, as simulated):** 80-100 minutes.
**Within target range (90-120 min)?** Yes, at the faster end. Slower teams or larger groups with coordination overhead will fill the window.

**Strongest element:** The meta. The calling card as a physical extraction key is elegant, and SLIT as the answer is visceral and narratively satisfying.

**Weakest element:** P3 relies on ASCII art floor plans that need proper graphic treatment for print. The ASCII version works but is visually the least immersive document in the dossier.

**Stage 11 priorities:**
1. Build remaining HTML files (P2, P3, P4)
2. Build submit.html with answer validation
3. Fix P1 extraction word underlining
4. Consider improving P3 floor plan graphics
5. Align Kessler's timeline entry (9:30 vs 9:28)
6. Clarify P4 analysis section ending reference
