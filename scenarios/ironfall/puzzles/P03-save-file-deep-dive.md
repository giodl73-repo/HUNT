# P03 — Save File Deep Dive

*Author: The Speedster*

---

> The archive preserves everything. Four save files were traded on the forum between 1998 and 2003. DataMiner_X decoded them all. One save holds a secret. The rest hold nothing. Find the message Morimoto left in the empty space.

---

## ironfall-archive.net/datamine/saves — Archived Save Data

Four save files recovered from the forum's save-trading thread. Each is shown as a raw hex dump — 128 bytes, 16 bytes per row. Use the save file format reference from the archive.

DataMiner_X (2003): "I dumped all four saves. The format is documented. But the padding area at the end of every save should be zeroes. Three of them are. One is not. Morimoto wrote something in the empty space."

---

### Save File A — "DarkKnight_97"

```
OFFSET  00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F
------  -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
0x0000  49 52 4F 4E 01 52 79 6E 6E 00 00 00 00 00 00 00
0x0010  00 00 00 00 00 2A 02 D0 03 84 00 96 00 DC 3A 26
0x0020  28 19 04 00 07 00 37 EE 01 00 F3 9A 00 00 FF 3C
0x0030  01 02 03 04 FF FF 03 01 04 06 07 08 09 0C 0D 0E
0x0040  10 11 12 13 00 00 00 00 00 00 00 01 01 01 01 01
0x0050  01 01 01 01 01 01 01 01 01 01 00 01 01 01 00 00
0x0060  00 00 00 00 00 01 00 00 00 00 00 00 00 00 00 00
0x0070  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
```

---

### Save File B — "SpeedQueen"

```
OFFSET  00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F
------  -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
0x0000  49 52 4F 4E 02 52 79 6E 6E 00 00 00 00 00 00 00
0x0010  00 00 00 00 00 32 03 E7 03 E7 01 2C 01 2C 48 32
0x0020  37 23 04 00 0B 00 3F A5 02 00 9F 86 01 00 FF 7F
0x0030  01 02 03 04 FF FF FF 01 02 03 04 05 06 07 08 09
0x0040  0A 0B 0C 0D 0E 0F 10 11 12 13 14 01 01 01 01 01
0x0050  01 01 01 01 01 01 01 01 01 01 01 01 01 01 01 01
0x0060  01 01 01 01 01 01 00 01 00 00 00 00 00 00 00 00
0x0070  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
```

---

### Save File C — "LoreHunter"

```
OFFSET  00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F
------  -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
0x0000  49 52 4F 4E 01 52 79 6E 6E 00 00 00 00 00 00 00
0x0010  00 00 00 00 00 26 02 58 03 20 00 B4 00 FA 34 22
0x0020  2D 1C 03 00 05 00 AE 72 01 00 E0 87 00 00 FF 0C
0x0030  01 02 03 00 FF FF 03 01 02 03 05 07 08 0A 0C 0D
0x0040  0E 0F 12 00 00 00 00 00 00 00 00 01 01 01 01 01
0x0050  01 01 01 01 01 01 01 00 00 00 00 00 01 01 00 00
0x0060  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
0x0070  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
```

---

### Save File D — "IronFan_Kenji"

```
OFFSET  00 01 02 03 04 05 06 07 08 09 0A 0B 0C 0D 0E 0F
------  -- -- -- -- -- -- -- -- -- -- -- -- -- -- -- --
0x0000  49 52 4F 4E 03 52 79 6E 6E 00 00 00 00 00 00 00
0x0010  00 00 00 00 00 2D 03 52 03 B6 00 C8 01 18 41 2C
0x0020  30 1E 06 00 05 00 37 EE 01 00 20 16 01 00 FF 6D
0x0030  01 02 03 04 FF FF FF 01 02 03 04 05 06 07 08 09
0x0040  0A 0B 0C 0D 0E 0F 10 11 12 13 14 01 01 01 01 01
0x0050  01 01 01 01 01 01 01 01 01 01 01 01 01 01 01 01
0x0060  01 01 01 01 01 01 00 01 4F 52 44 45 52 00 00 00
0x0070  00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00
```

---

## Your Task

Three steps.

1. Decode each save's key flags using the archive's format reference. The fields you need are at fixed offsets.
2. Check the padding area (offset 0x68 onward) in each save. Three saves have all zeroes there. One does not.
3. Read the non-zero padding bytes as ASCII.

---

## Solution

### Step 1: Decode the flags

From the save file format reference:
- Offset 0x65: game_complete (0x01 = yes)
- Offset 0x66: true_ending (0x00 = not triggered)
- Offset 0x67: secret_boss_beaten (0x01 = yes)

Row 0x0060 in each save shows offsets 0x60-0x6F. Position 5 in the row = offset 0x65, position 6 = 0x66, position 7 = 0x67.

| Save | 0x65 (game) | 0x66 (ending) | 0x67 (boss) | Notes |
|------|------------|--------------|------------|-------|
| A | 01 | 00 | 00 | Game complete, no secret boss |
| B | 01 | 00 | 01 | Game complete, secret boss beaten |
| C | 00 | 00 | 00 | Game not complete |
| D | 01 | 00 | 01 | Game complete, secret boss beaten |

Both Save B and Save D meet the True Ending prerequisites.

### Step 2: Check the padding

Offset 0x68 onward:

| Save | Bytes at 0x68+ | Padding |
|------|---------------|---------|
| A | `00 00 00 00 00 00 00 00` | All zeroes |
| B | `00 00 00 00 00 00 00 00` | All zeroes |
| C | `00 00 00 00 00 00 00 00` | All zeroes |
| D | `4F 52 44 45 52 00 00 00` | Non-zero bytes present |

Only Save D has data in the padding.

### Step 3: Read as ASCII

Bytes at offsets 0x68-0x6C in Save D:

| Offset | Hex | ASCII |
|--------|-----|-------|
| 0x68 | 4F | O |
| 0x69 | 52 | R |
| 0x6A | 44 | D |
| 0x6B | 45 | E |
| 0x6C | 52 | R |

**Answer: ORDER** (ROT13: BEQRE)

### Extraction Verification

Answer: ORDER. Puzzle slot: Act I, position 3. Super-meta extracts the 3rd letter: ORDER[3] = D. D in ROT13 = Q. Matches the extraction table.

---

*The save file format reference is in the archive. The padding tells the truth.*
