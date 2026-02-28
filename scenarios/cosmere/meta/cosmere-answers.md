# THE SIXTEENTH SHARD — Answer Registry (ROT13 Encoded)

**WARNING: All answers are ROT13 encoded. Do NOT decode in any git-tracked file.**
**NOTE: This file should be moved to .claude/cosmere-answers.md for production. Stored here temporarily.**

**Last updated:** 2026-02-27
**Stage:** 4+5 complete

---

## Feeder Puzzle Answers (36)

### Module A — Allomancy (P01-P06)

| ID | Title | Metal | Answer (ROT13) |
|----|-------|-------|-----------------|
| P01 | Burning Through | Iron (#1) | HCEVFR |
| P02 | Alloy Partners | Steel (#2) | SBETRQ |
| P03 | Feruchemical Reserves | Zinc (#5) | NAPUBE |
| P04 | The Allomantic Table | Brass (#6) | ZNGEVK |
| P05 | Three Arts, One Spike | Pewter (#4) | GEVNQ |
| P06 | The Coppercloud | Bronze (#8) | UVQQRA |

### Module B — Scadrial History (P07-P12)

| ID | Title | Metal | Answer (ROT13) |
|----|-------|-------|-----------------|
| P07 | The Lord Ruler's 1000 Years | Electrum (#10) | GLENAG |
| P08 | Kelsier's Crew | Gold (#9) | URVFG |
| P09 | The Social Ladder | Cadmium (#11) | EVGHNY |
| P10 | The Survivor's Legacy | Chromium (#15) | HAQRNQ |
| P11 | The Siege of Luthadel | Bendalloy (#12) | OERNPU |
| P12 | The Roughs Justice | Duralumin (#14) | BHGYNJ |

### Module C — Knights Radiant (P13-P18)

| ID | Title | Order | Answer (ROT13) |
|----|-------|-------|-----------------|
| P13 | Herald's Madness | Dustbringers | OEBXRA |
| P14 | Ideals and Oaths | Edgedancers | QRIBGR |
| P15 | Spren Bonds | Truthwatchers | FCVEVG |
| P16 | Radiant Roster | Skybreakers | FJBEA |
| P17 | The Double Eye | Windrunners | GBJRE |
| P18 | The Surge Wheel | Lightweavers | CNGGREA |

### Module D — Roshar World (P19-P24)

| ID | Title | Order | Answer (ROT13) |
|----|-------|-------|-----------------|
| P19 | Singer Forms | Stonewards | PUVGVA |
| P20 | Bridge Four | Windrunners (dup) | OEVQTR |
| P21 | Rhythm of Answers | Willshapers | PUNAG |
| P22 | The Chasms Below | Lightweavers (crit) | REBQRQ |
| P23 | Fabrial Workshop | Bondsmiths | ORELY |
| P24 | Stormwall | Elsecallers | GRZCRFG |

### Module E — The 16 Shards (P25-P30)

| ID | Title | Shard | Answer (ROT13) |
|----|-------|-------|-----------------|
| P25 | Shattered and Whole | Honor | FGNGHF |
| P26 | Vessel Roll Call | Preservation | ZBEGNY |
| P27 | Odium's Kill List | Odium | JENGU |
| P28 | Investiture Types | Endowment | TVSGRQ |
| P29 | The 4x4 Shard Grid | Autonomy | NFCRPG |
| P30 | The Shattering | Virtuosity | BEVTVA |

### Module F — Hoid & Worldhoppers (P31-P36)

| ID | Title | Shard | Answer (ROT13) |
|----|-------|-------|-----------------|
| P31 | The Wanderer's Aliases | Ruin | ZNFXRQ |
| P32 | Cognitive Shadows | Harmony | YVATRE |
| P33 | Collected Powers | Cultivation | TEBJGU |
| P34 | Letters to a Dragon | Devotion | FPEVCG |
| P35 | The Memory Thief | Whimsy | FGBYRA |
| P36 | All Roads Lead to Hoid | Valor | GUERNQ |

---

## Round Meta Answers (3)

| Meta | Name | Answer (ROT13) | Letters |
|------|------|-----------------|---------|
| META-I | THE WELL | HAVGR | 5 |
| META-II | THE OATHS | RAIBL | 5 |
| META-III | ADONALSIUM | NZVQ | 4 |

---

## Super-Meta Answer

| Meta | Name | Answer (ROT13) | Letters |
|------|------|-----------------|---------|
| SUPER | THE SEVENTEENTH SHARD | HAVGL | 5 |

---

## Verification Chain

### Meta I: THE WELL
Pulling metals in table order, quadrant-depth extraction:
- Iron(1) Physical depth=1: HCEVFR[1] = H
- Zinc(5) Mental depth=2: NAPUBE[2] = A
- Gold(9) Temporal depth=3: URVFG[3] = V
- Cadmium(11) Temporal depth=3: EVGHNY[3] = G
- Chromium(15) Enhancement depth=4: HAQRNQ[4] = R
Result: H-A-V-G-R = HAVGR ✓

### Meta II: THE OATHS
Orders with 3+ Ideals, extract at Ideal-count depth:
- Windrunners(1, 4 Ideals): GBJRE[4] = R
- Skybreakers(2, 5 Ideals): FJBEA[5] = A
- Edgedancers(4, 3 Ideals): QRIBGR[3] = I
- Lightweavers(6, 3 Ideals): REBQRQ[3] = B
- Bondsmiths(10, 4 Ideals): ORELO[4] = L
Result: R-A-I-B-L = RAIBL ✓

### Meta III: ADONALSIUM
Missing Shards: Ambition(A), Dominion(D), Invention(I), Mercy(M)
First letters anagrammed: A,D,I,M → NZVQ ✓

### Super-Meta: THE SEVENTEENTH SHARD
Realms cycle (Phys-Cog-Spir-Phys-Cog), depth (1-2-3-4-5):
- HAVGR[1] = H
- RAIBL[2] = A
- NZVQ[3] = V
- HAVGR[4] = G
- RAIBL[5] = L
Result: H-A-V-G-L = HAVGL ✓

---

## Metal Assignment Map (Scadrial, Meta I)

| # | Metal | Puzzle | Pull/Push | Quadrant | Extract Pos | Used? |
|---|-------|--------|-----------|----------|-------------|-------|
| 1 | Iron | P01 | Pull | Physical | 1 | META |
| 2 | Steel | P02 | Push | Physical | 1 | yes |
| 3 | Tin | -- | Pull | Physical | -- | UNUSED |
| 4 | Pewter | P05 | Push | Physical | 1 | yes |
| 5 | Zinc | P03 | Pull | Mental | 2 | META |
| 6 | Brass | P04 | Push | Mental | 2 | yes |
| 7 | Copper | -- | Pull | Mental | -- | UNUSED |
| 8 | Bronze | P06 | Push | Mental | 2 | yes |
| 9 | Gold | P08 | Pull | Temporal | 3 | META |
| 10 | Electrum | P07 | Push | Temporal | 3 | yes |
| 11 | Cadmium | P09 | Pull | Temporal | 3 | META |
| 12 | Bendalloy | P11 | Push | Temporal | 3 | yes |
| 13 | Aluminum | -- | Pull | Enhancement | -- | UNUSED |
| 14 | Duralumin | P12 | Push | Enhancement | 4 | yes |
| 15 | Chromium | P10 | Pull | Enhancement | 4 | META |
| 16 | Nicrosil | -- | Push | Enhancement | -- | UNUSED |

## Order Assignment Map (Roshar, Meta II)

| # | Order | Puzzle(s) | Ideals | Contributes? | Pos | Letter (ROT13) |
|---|-------|-----------|--------|-------------|-----|----------------|
| 1 | Windrunners | P17, P20(dup) | 4 | META (P17) | 4 | R |
| 2 | Skybreakers | P16 | 5 | META | 5 | A |
| 3 | Dustbringers | P13 | 1 | no | -- | -- |
| 4 | Edgedancers | P14 | 3 | META | 3 | I |
| 5 | Truthwatchers | P15 | 1 | no | -- | -- |
| 6 | Lightweavers | P18, P22(crit) | 3 | META (P22) | 3 | B |
| 7 | Elsecallers | P24 | 1 | no | -- | -- |
| 8 | Willshapers | P21 | 1 | no | -- | -- |
| 9 | Stonewards | P19 | 1 | no | -- | -- |
| 10 | Bondsmiths | P23 | 4 | META | 4 | L |

## Shard Assignment Map (Cosmere, Meta III)

| # | Shard | Puzzle | Represented? |
|---|-------|--------|-------------|
| 1 | Honor | P25 | yes |
| 2 | Cultivation | P33 | yes |
| 3 | Odium | P27 | yes |
| 4 | Preservation | P26 | yes |
| 5 | Ruin | P31 | yes |
| 6 | Harmony | P32 | yes |
| 7 | Devotion | P34 | yes |
| 8 | Dominion | -- | MISSING |
| 9 | Endowment | P28 | yes |
| 10 | Autonomy | P29 | yes |
| 11 | Ambition | -- | MISSING |
| 12 | Mercy | -- | MISSING |
| 13 | Valor | P36 | yes |
| 14 | Whimsy | P35 | yes |
| 15 | Invention | -- | MISSING |
| 16 | Virtuosity | P30 | yes |
