# Condition AP — Programmer

## Puzzle

**ARRAY ACCESS**
*A feeder puzzle for the Age of Empires II hunt*

Each clue below identifies a unit or building from Age of Empires II: Definitive Edition.
Once you have the correct name, extract one letter from it using the given 0-based index.
Read the five extracted letters top to bottom.

```
Notation: name[index]  means "the character at position index, counting from 0"
Example:  "CASTLE"[2]  →  S
```

---

```
┌─────┬───────────────────────────────────────────────────────────────────────┬───────┬────────┐
│ Row │ Clue                                                                  │ Index │ Letter │
├─────┼───────────────────────────────────────────────────────────────────────┼───────┼────────┤
│  1  │ The only siege unit in the game that must pack and unpack to switch   │   0   │   ?    │
│     │ between mobile and firing modes. Imperial Age. Trained at the Castle. │       │        │
├─────┼───────────────────────────────────────────────────────────────────────┼───────┼────────┤
│  2  │ Costs exactly 100 gold and nothing else. Trained at the Monastery.    │   1   │   ?    │
│     │ Can convert enemy units to your side.                                 │       │        │
├─────┼───────────────────────────────────────────────────────────────────────┼───────┼────────┤
│  3  │ Your civilization's command center. You begin every standard game     │   2   │   ?    │
│     │ with one. The only building where Loom can be researched.             │       │        │
│     │ Villagers are produced here.                                          │       │        │
├─────┼───────────────────────────────────────────────────────────────────────┼───────┼────────┤
│  4  │ The building that produces Mangonels, Scorpions, and Battering Rams.  │   2   │   ?    │
│     │ It must be constructed before any Castle Age ground siege can be      │       │        │
│     │ fielded.                                                              │       │        │
├─────┼───────────────────────────────────────────────────────────────────────┼───────┼────────┤
│  5  │ Archers, Skirmishers, Cavalry Archers, and Hand Cannoneers are all    │   1   │   ?    │
│     │ trained here. Becomes available in the Feudal Age.                    │       │        │
└─────┴───────────────────────────────────────────────────────────────────────┴───────┴────────┘
```

Read your five extracted letters, rows 1 through 5, to form the answer.

---

## Solution Key

**Row 1 — index 0**
Clue: pack/unpack siege unit, Imperial Age, trained at the Castle
Answer: **TREBUCHET**
Extraction: `"TREBUCHET"[0]` = **T**
(T-R-E-B-U-C-H-E-T; position 0 is the first character)

**Row 2 — index 1**
Clue: 100 gold, Monastery, converts enemy units
Answer: **MONK**
Extraction: `"MONK"[1]` = **O**
(M-O-N-K; position 1 is the second character)

**Row 3 — index 2**
Clue: starting command center, only source of Loom, trains Villagers
Answer: **TOWN CENTER**
Extraction: `"TOWN CENTER"[2]` = **W**
(T-O-W-N-...; position 2 is the third character)

**Row 4 — index 2**
Clue: produces Mangonels, Scorpions, Battering Rams; prerequisite for Castle Age ground siege
Answer: **SIEGE WORKSHOP**
Extraction: `"SIEGE WORKSHOP"[2]` = **E**
(S-I-E-G-E-...; position 2 is the third character)

**Row 5 — index 1**
Clue: trains Archers, Skirmishers, Cavalry Archers, Hand Cannoneers; Feudal Age
Answer: **ARCHERY RANGE**
Extraction: `"ARCHERY RANGE"[1]` = **R**
(A-R-C-H-E-R-Y-...; position 1 is the second character)

**Extracted letters in order: T · O · W · E · R**

Answer: **TOWER**
