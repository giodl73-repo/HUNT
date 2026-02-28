# P10 — Anomaly in the Code

*Author: The Lurker*

---

> Eight battle records. Datamined from the ROM by DataMiner_X. The ATK, DEF, and Level values are verified from the bestiary and save data. The damage column was calculated by a fan-made tool. The tool has errors. Find them.

---

## ironfall-archive.net/datamine/battle-records — Extracted Combat Data

DataMiner_X (2003): "I pulled these eight records from the ROM's battle test data. The attacker and defender stats are correct — I verified them against the bestiary hex tables. But the damage values were calculated by IronFan_Kenji's spreadsheet, and I think some of them are wrong. Check every record against the damage formula. If the calculated damage does not match, note the difference. The differences are not random."

---

### Battle Records

All battles are physical attacks at Level 50.

```
╔═════╤══════╤══════╤═══════╤══════════╗
║  #  │  ATK │  DEF │ Level │  Damage  ║
╠═════╪══════╪══════╪═══════╪══════════╣
║  1  │   65 │   12 │   50  │    236   ║
║  2  │   55 │   35 │   50  │    158   ║
║  3  │   50 │   22 │   50  │    156   ║
║  4  │   42 │   48 │   50  │     77   ║
║  5  │   40 │   20 │   50  │    120   ║
║  6  │   60 │   10 │   50  │    232   ║
║  7  │   48 │   42 │   50  │    117   ║
║  8  │   65 │   22 │   50  │    240   ║
╚═════╧══════╧══════╧═══════╧══════════╝
```

---

## Your Task

1. Determine the physical damage formula from the correct records.
2. Apply the formula to all eight records.
3. Identify which records have incorrect damage values.
4. For each incorrect record, calculate the difference between the shown damage and the correct damage.
5. Convert each difference to a letter (1=A, 2=B, ... 26=Z).
6. Read the letters in record order.

---

## Solution

### Step 1: Derive the formula

The physical damage formula (from the battle system reference, or derivable from the correct records):

```
Damage = (ATK × 2 - DEF) × (1 + Level/50)
```

At Level 50: multiplier = (1 + 50/50) = 2.0.

So: Damage = (ATK × 2 - DEF) × 2

### Step 2: Verify all records

| # | ATK | DEF | (ATK×2 − DEF) | ×2 | Correct Damage | Shown | Match? | Diff |
|---|-----|-----|---------------|-----|---------------|-------|--------|------|
| 1 | 65 | 12 | 118 | 236 | 236 | 236 | Yes | — |
| 2 | 55 | 35 | 75 | 150 | 150 | 158 | **No** | 8 |
| 3 | 50 | 22 | 78 | 156 | 156 | 156 | Yes | — |
| 4 | 42 | 48 | 36 | 72 | 72 | 77 | **No** | 5 |
| 5 | 40 | 20 | 60 | 120 | 120 | 120 | Yes | — |
| 6 | 60 | 10 | 110 | 220 | 220 | 232 | **No** | 12 |
| 7 | 48 | 42 | 54 | 108 | 108 | 117 | **No** | 9 |
| 8 | 65 | 22 | 108 | 216 | 216 | 240 | **No** | 24 |

Records 1, 3, and 5 are correct. Records 2, 4, 6, 7, and 8 have errors.

### Step 3: Convert differences

| Record | Diff | A1Z26 |
|--------|------|-------|
| 2 | 8 | H |
| 4 | 5 | E |
| 6 | 12 | L |
| 7 | 9 | I |
| 8 | 24 | X |

### Step 4: Read in record order

H, E, L, I, X.

**Answer: HELIX** (ROT13: URYVK)

### Extraction Verification

Answer: HELIX. Puzzle slot: Act II, position 5. Super-meta extracts position 5: HELIX[5] = X. X in ROT13 = K. Matches the extraction table.

---

*The battle system reference is in the archive. The formula does not lie. The errors do.*
