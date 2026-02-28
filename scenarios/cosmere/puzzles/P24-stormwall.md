# P24 — Stormwall

**Elsecallers** · Module D · Round 2 (Roshar)

---

> Two storms. One comes from the east, as it always has. The other comes from the west, born of betrayal. For the first time in history, Roshar is struck from both directions. The stormwardens have gone quiet. Only the math remains.

---

## The Puzzle

Seven watchtowers stand along an east-west line across Roshar. Each tower bears a signal glyph — a single letter. When the first storm strikes a tower, its glyph blazes to life.

Two storms approach. Calculate which storm reaches each tower first. Sort the towers by when they light up. Read the glyphs.

---

### The Map

```
WEST ←——————————————————————————————————————————————→ EAST

  [E]     [E]     [T]     [S]     [P]       [M]               [T]
Shinovar  Azir  Purelake  Emul  Thaylenah  Alethkar     Shattered Plains
   0       2       4       6       8         10                14

                    (distance in storm-units)
```

---

### Storm Data

**The Highstorm** sweeps east to west.
- Strikes the Shattered Plains (position 14) at **Hour 0**
- Speed: **2 storm-units per hour**
- Arrival at any position: **(14 − position) ÷ 2** hours

**The Everstorm** sweeps west to east.
- Strikes Shinovar (position 0) at **Hour 1**
- Speed: **1 storm-unit per hour**
- Arrival at any position: **1 + position** hours

---

### Rules

1. For each tower, calculate both arrival times. The **earlier** storm lights the glyph.
2. If both arrive at the same hour, the **Highstorm** wins (it strikes harder).
3. Sort the seven towers by the hour their glyph lights — earliest first.
4. Ties broken: **east before west** (the Highstorm arrives from the east; its near-side towers report first).
5. Read the glyphs in that order.

---

## Worksheet

| Tower | Pos | Glyph | Highstorm | Everstorm | Which First | Hour Lit |
|-------|-----|-------|-----------|-----------|-------------|----------|
| Shinovar | 0 | E | (14−0)÷2 = ___ | 1+0 = ___ | | |
| Azir | 2 | E | (14−2)÷2 = ___ | 1+2 = ___ | | |
| Purelake | 4 | T | (14−4)÷2 = ___ | 1+4 = ___ | | |
| Emul | 6 | S | (14−6)÷2 = ___ | 1+6 = ___ | | |
| Thaylenah | 8 | P | (14−8)÷2 = ___ | 1+8 = ___ | | |
| Alethkar | 10 | M | (14−10)÷2 = ___ | 1+10 = ___ | | |
| Shattered Plains | 14 | T | (14−14)÷2 = ___ | 1+14 = ___ | | |

---

### Calculations (verify your work)

| Tower | Highstorm Hour | Everstorm Hour | First Storm | Hour Lit |
|-------|---------------|----------------|-------------|----------|
| Shinovar | | | | |
| Azir | | | | |
| Purelake | | | | |
| Emul | | | | |
| Thaylenah | | | | |
| Alethkar | | | | |
| Shattered Plains | | | | |

---

### Arrival Order (sort by Hour Lit, break ties east-first)

| Rank | Tower | Hour | Glyph |
|------|-------|------|-------|
| 1st | | | [_] |
| 2nd | | | [_] |
| 3rd | | | [_] |
| 4th | | | [_] |
| 5th | | | [_] |
| 6th | | | [_] |
| 7th | | | [_] |

Glyphs in order: [_] [_] [_] [_] [_] [_] [_]

---

**Your answer:** _______________

*The storms care nothing for the towers between them. But the towers care very much about the storms.*
