# IRONFALL Save File Format

Source of truth for the save data structure. Save files are displayed on the game's save/load screen and are a key puzzle surface for the archive — dataminers have decoded the format.

---

## Save File Display (In-Game)

The save screen shows 3 slots:

```
╔══════════════════════════════════════════╗
║  SAVE FILE 1                             ║
║  Rynn  Lv.38  ⏱ 24:17:33               ║
║  Location: Iron Citadel - 5F             ║
║  Party: Rynn, Elara, Kael, Maren         ║
║  Marks: ████████████░░░░  (12/16)        ║
║  Gil: 48,203                              ║
╚══════════════════════════════════════════╝
```

---

## Raw Save Data Format (Hex)

Dataminers (led by forum user DataMiner_X) decoded the save format in 1998. Each save file is 256 bytes.

| Offset | Size | Field | Format | Example |
|--------|------|-------|--------|---------|
| 0x00 | 4 | Magic number | ASCII | "IRON" |
| 0x04 | 1 | Save slot | uint8 | 0x01 = Slot 1 |
| 0x05 | 16 | Player name | ASCII padded | "Rynn\0\0\0\0..." |
| 0x15 | 1 | Level | uint8 | 0x26 = 38 |
| 0x16 | 2 | HP (current) | uint16 LE | 0x0258 = 600 |
| 0x18 | 2 | HP (max) | uint16 LE | 0x0320 = 800 |
| 0x1A | 2 | MP (current) | uint16 LE | 0x0064 = 100 |
| 0x1C | 2 | MP (max) | uint16 LE | 0x00C8 = 200 |
| 0x1E | 1 | ATK | uint8 | 0x37 = 55 |
| 0x1F | 1 | DEF | uint8 | 0x22 = 34 |
| 0x20 | 1 | MAG | uint8 | 0x23 = 35 |
| 0x21 | 1 | SPD | uint8 | 0x1C = 28 |
| 0x22 | 2 | Region ID | uint16 LE | 0x0004 = Iron Citadel |
| 0x24 | 2 | Sub-area ID | uint16 LE | 0x0005 = Floor 5 |
| 0x26 | 4 | Playtime (seconds) | uint32 LE | 0x00015A15 = 88597s = 24:36:37 |
| 0x2A | 4 | Gil | uint32 LE | 0x0000BC4B = 48,203 |
| 0x2E | 2 | Achievement flags | uint16 bitmask | 0x0FFF = marks 1-12 earned |
| 0x30 | 4 | Party members | 4× uint8 | 0x01020304 = Rynn,Elara,Kael,Maren |
| 0x34 | 3 | Bestiary flags | 3× uint8 (24 bits) | 0xFFFF03 = enemies 1-18 seen |
| 0x37 | 20 | Inventory | 20× uint8 (item IDs) | 0x01,0x04,0x07... |
| 0x4B | 8 | Special moves | 8× uint8 (1=learned) | 0x0101010101010101 |
| 0x53 | 12 | Lore tablets | 12× uint8 (1=found) | 0x010101010101... |
| 0x5F | 6 | Secret areas | 6× uint8 (1=found) | 0x010101010101 |
| 0x65 | 1 | Game complete flag | uint8 | 0x01 = beaten final boss |
| 0x66 | 1 | True Ending flag | uint8 | 0x00 = not triggered |
| 0x67 | 1 | Secret boss flag | uint8 | 0x01 = defeated Morimoto's Shadow |
| 0x68-0xFF | padding/checksum | ... | ... | ... |

---

## Region IDs

| ID | Region |
|----|--------|
| 0x0001 | Ashveil Forest |
| 0x0002 | Glacial Reach |
| 0x0003 | Stormspire Peaks |
| 0x0004 | Iron Citadel |
| 0x0005 | Sunken Vale |
| 0x0006 | Ember Wastes |

---

## Achievement Bitmask

The achievement flags at offset 0x2E are a 16-bit field. Each bit corresponds to a Mark of Mastery:

```
Bit 0  (0x0001) = Mark 01: Seedling
Bit 1  (0x0002) = Mark 02: Trailblazer
Bit 2  (0x0004) = Mark 03: Ashen Victor
Bit 3  (0x0008) = Mark 04: Relentless
Bit 4  (0x0010) = Mark 05: Tempest Born
Bit 5  (0x0020) = Mark 06: Ironclad
Bit 6  (0x0040) = Mark 07: Artisan
Bit 7  (0x0080) = Mark 08: Collector
Bit 8  (0x0100) = Mark 09: Night Walker
Bit 9  (0x0200) = Mark 10: Ghost Runner
Bit 10 (0x0400) = Mark 11: Scholar
Bit 11 (0x0800) = Mark 12: Loyal Heart
Bit 12 (0x1000) = Mark 13: Oathbreaker
Bit 13 (0x2000) = Mark 14: Unseen
Bit 14 (0x4000) = Mark 15: Legacy
Bit 15 (0x8000) = Mark 16: Transcendent
```

A save with Marks 1-12 has bitmask `0x0FFF` = `0000 1111 1111 1111`.

---

## Forum Save File Analysis Posts

**DataMiner_X** posted the definitive save format breakdown on March 22, 1998. It became the most-referenced page on the archive.

**SpeedQueen** used save editing to test title-screen inputs. Her post (June 8, 2001) confirmed: "Offset 0x66 flips from 0x00 to 0x01 when you enter the correct code. The game checks for the code ONLY when 0x65=0x01 (game complete) AND 0x67=0x01 (secret boss beaten). This narrows the requirements."

---

## Puzzle Affordances

1. **Pattern recognition**: Show several save files (hex dumps) and ask the solver to identify which save meets the True Ending requirements.
2. **Binary/hex decoding**: Convert hex values to game stats, identify anomalies.
3. **Bitmask reading**: Read achievement bitmask to determine which Marks a save file has.
4. **Cross-referencing**: Compare save file inventory field against items.md to identify which items a player has.
