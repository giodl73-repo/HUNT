# THE GRAND LARCENY — Puzzle Registry

## Puzzle Assignments

| ID | Title | Type | Document form | Round | Author | Status |
|----|-------|------|---------------|-------|--------|--------|
| P1 | The Police Report | Logic/grid | Police incident report | 1 | The Methodical | unassigned |
| P2 | The Hotel Receipt | Arithmetic/pattern | Itemized hotel bill | 1 | The Social | unassigned |
| P3 | The Floor Plan | Spatial/path | Architectural floor plan | 1 | The Methodical | unassigned |
| P4 | The Witness Statement | Text/acrostic | Typed witness transcript | 1 | The Social | unassigned |
| META | The Method | Combination | Evidence log / case summary | 1 | Admin | unassigned |

## Coordinated Answer Words

Answer words are coordinated to support the meta. Each word represents one dimension of the crime:

| Puzzle | Dimension | Answer word (ROT13) | Length | Verified for meta |
|--------|-----------|--------------------|---------|----|
| P1 | Who | FHFCRPG (ROT13) | 7 | Yes |
| P2 | Where | YBPNGVBA (ROT13) | 8 | Yes |
| P3 | When | GVZR (ROT13) | 4 | Yes |
| P4 | What | BOWRPG (ROT13) | 6 | Yes |
| META | How | FYVG (ROT13) | 4 | Yes |

---

## Puzzle Briefs

### P1 — The Police Report

**Type:** Logic grid with alibi cross-referencing
**Document form:** Typed police incident report (Case No. 47-1011-GL)
**Answer:** 7 letters (ROT13: FHFCRPG)
**Author:** The Methodical

**Mechanism:**
The police report presents four suspects with their stated alibis in a structured format. Each alibi is contradicted by one piece of physical evidence or witness testimony. The solver must:
1. Read each suspect's alibi
2. Cross-reference against witness statements and physical evidence cited in the report
3. Map each suspect's actual (vs. claimed) location during the theft window (9:15-9:25 PM)
4. Determine which suspect's movements are CONSISTENT with being at the gallery during the window

**Extraction:**
The police report includes a "Suspect Summary" table with seven fields per suspect (Name, Room, Occupation, Alibi Location, Witness, Evidence, Status). For each suspect cleared by the logic, their "Status" row contains one letter in a box. The four boxed letters, plus three more from the report header, spell the answer.

Specifically: the report has blanks labeled "PRIMARY [_]" through the body. As the solver eliminates suspects or confirms facts, they fill in letters from a cipher embedded in the report structure. The seven filled letters spell the answer word.

**Interlock:** Uses timeline data from world/systems/timeline.md. Receipt data from P2 confirms timing. Key card log is relevant.

**Aha:** The moment the solver realizes that one suspect's alibi, despite being contradicted, is still consistent with innocence — while another suspect's "solid" alibi actually has a hidden gap.

**World references:** characters.md (alibis), timeline.md (theft window), services.md (key card log, phone log)

---

### P2 — The Hotel Receipt

**Type:** Arithmetic pattern with selective extraction
**Document form:** Hotel billing statement (4 guest folios on one page)
**Answer:** 8 letters (ROT13: YBPNGVBA)
**Author:** The Social

**Mechanism:**
Four itemized guest receipts are presented in hotel accounting format. Each receipt is arithmetically correct — totals add up. The solver notices that certain charges across the four receipts have cents values that, when converted via A1Z26 (01=A, 02=B, ... 26=Z), spell letters.

The aha: not ALL charges encode letters — only the charges timestamped during specific windows. The solver must figure out which time windows matter. The clue: only charges from guests who were NOT where they claimed to be encode letters. Cross-referencing with the timeline reveals which charges occurred during "gap" periods in each suspect's alibi.

**Extraction:**
Ashworth's gap charges (9:15-9:45): cents = 12, 15 → L, O
Kessler's gap charges (8:22-9:30): cents = 03, 01 → C, A
Fontaine's gap charges (around 9:10): cents = 20 → T
Delacroix's gap charges (9:00-9:30): cents = 09, 15, 14 → I, O, N

Read in suspect order (P1 determines the order — Ashworth, Kessler, Fontaine, Delacroix by room number): L-O-C-A-T-I-O-N.

**Design note:** The gap charges are additional items not on the original receipts in services.md. During authoring, the author adds specific "mystery charges" timestamped during each suspect's gap period. These charges look normal (drinks, phone calls) but their cents values encode the letters.

**Interlock:** Requires understanding the alibi gaps from P1/timeline to know which charges to extract. The time windows are the key.

**Aha:** Realizing that "gap period" charges are the significant ones. The arithmetic is a red herring — the cents digits are the message.

**World references:** services.md (receipt data, price tables), timeline.md (alibi gaps)

---

### P3 — The Floor Plan

**Type:** Spatial path tracing with physical prop
**Document form:** Architectural floor plan (4 levels, drafting style)
**Answer:** 4 letters (ROT13: GVZR)
**Author:** The Methodical

**Mechanism:**
A detailed floor plan of The Grand Hotel showing all four levels. The solver uses evidence from the police report (the unlogged elevator use at 9:20 PM, the fire escape, the cut canvas, the gallery-to-linen-closet connecting door) plus the hotel key card prop (which shows Room 303 — Roof Access) to trace the thief's route through the building.

The route: Roof Access (303) → service elevator down to 2F → Sargent Gallery (200) → through connecting door to Linen Closet (206) → service elevator down to Basement → Storage (B04) → Staff Corridor → Laundry (B02) → alley exit.

**Extraction:**
The floor plan has small numbered markers at each room entrance (like architectural reference points). When the solver traces the path and reads the markers in order, they get four numbers. Converting via A1Z26: the four numbers spell the answer word.

Marker at 303 entrance: 20 (T)
Marker at 200 entrance: 9 (I)
Marker at 206 → B04 path: 13 (M)
Marker at B02 exit: 5 (E)

Answer: T-I-M-E.

**Physical prop:** The hotel key card has "ROOM 303" and a small diagram showing the roof access location. This tells the solver where the thief entered. Without the key card, the starting point is ambiguous.

**Aha:** Discovering the unmarked connecting door between the Sargent Gallery (200) and the Linen Closet (206). It is shown on the floor plan as a thin line (a door) but not labeled as a passage. The solver must notice it and realize it is the thief's escape route from the gallery.

**World references:** layout.md (all room positions, connections, paths), timeline.md (elevator log)

---

### P4 — The Witness Statement

**Type:** Acrostic hidden in testimony
**Document form:** Typed interview transcript
**Answer:** 6 letters (ROT13: BOWRPG)
**Author:** The Social

**Mechanism:**
A typed transcript of the police interview with Miss Colette Delacroix. The text reads as a natural, nervous account of her evening — her observations of the other guests, what she saw in the gallery, her admiration for the painting. The voice is halting, with corrections, repetitions, and verbal tics ("I mean," "you understand," "well").

Hidden in the text: the first letter of each sentence in the main body of the transcript spells a message. The message is: "ODD BITS JOTTED: OBJECT" — where the last six letters spell the answer word. (The acrostic reads: O-D-D-B-I-T-S-J-O-T-T-E-D-O-B-J-E-C-T, with the colon/space represented by short sentences.)

Alternatively, a cleaner extraction: the transcript has 24 sentences. The first letters of sentences 19-24 spell OBJECT.

**Extraction:**
Read the first letter of each sentence in the transcript. The last six letters spell the answer word.

**Interlock:** The witness statement confirms details about other suspects (she saw Fontaine near the gallery, she noticed Kessler leaving the telephone room early). These observations support the logic in P1.

**Aha:** Realizing the transcript has a hidden structure. The first-letter pattern becomes visible once the solver starts reading the initials — they shift from seeming random to clearly spelling words.

**World references:** characters.md (Delacroix's perspective, other suspects' movements), timeline.md (her evening schedule)

---

## Meta Brief

See `meta/META-DESIGN.md` for full meta design.

**Quick summary:** The four answer words (7+8+4+6 = 25 letters) combine to answer "How was the painting stolen?" The four dimensions (who, where, when, what) plus the hotel narrative point to the method. The meta answer is 6 letters (ROT13: ZRGUBQ).

---

## Tester Assignments

| Puzzle | Tester 1 | Tester 2 | Tester 3 |
|--------|----------|----------|----------|
| P1 | Lucas Pope | Wei-Hwa Huang | Dan Katz |
| P2 | Jonathan Blow | Thomas Snyder | Mark Gottlieb |
| P3 | Rand Miller | Kenny Young | Peter Sarrett |
| P4 | Mike Selinker | Alex Rosenthal | Dana Young |
| META | Dan Katz | Wei-Hwa Huang | Mike Selinker |
