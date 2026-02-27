# Chess — World Data

Source material for Module M1 (The Professor).

---

## Piece Movements

| Piece | Symbol | Movement | Captures | Special rules |
|-------|--------|----------|----------|---------------|
| King | K | 1 square any direction | Same as movement | Cannot move into check; castling |
| Queen | Q | Any number of squares in any direction (rank, file, diagonal) | Same as movement | — |
| Rook | R | Any number of squares along rank or file | Same as movement | Castling with King |
| Bishop | B | Any number of squares diagonally | Same as movement | Stays on its starting color for entire game |
| Knight | N | L-shape: 2+1 squares (or 1+2) | Same as movement | Only piece that jumps over others |
| Pawn | — | 1 square forward (2 from starting rank) | 1 square diagonally forward | En passant; promotion on 8th rank |

### Special Moves

| Move | Conditions | Notation |
|------|-----------|----------|
| **Kingside castling** | King and h-Rook unmoved; squares f1/g1 empty and not attacked | O-O |
| **Queenside castling** | King and a-Rook unmoved; squares b1/c1/d1 empty; c1/d1 not attacked | O-O-O |
| **En passant** | Opponent's pawn advances 2 squares, landing beside your pawn; must capture immediately | exd6 e.p. (example) |
| **Promotion** | Pawn reaches 8th rank; must promote to Q, R, B, or N | e8=Q |

---

## Algebraic Notation

### Square Naming

Files: a-h (left to right from White's perspective)
Ranks: 1-8 (bottom to top from White's perspective)

```
8 | r  n  b  q  k  b  n  r    ← Black's back rank
7 | p  p  p  p  p  p  p  p    ← Black's pawns
6 | .  .  .  .  .  .  .  .
5 | .  .  .  .  .  .  .  .
4 | .  .  .  .  .  .  .  .
3 | .  .  .  .  .  .  .  .
2 | P  P  P  P  P  P  P  P    ← White's pawns
1 | R  N  B  Q  K  B  N  R    ← White's back rank
    a  b  c  d  e  f  g  h
```

### Notation Symbols

| Symbol | Meaning |
|--------|---------|
| K, Q, R, B, N | Piece moved (no letter for pawn) |
| x | Capture |
| + | Check |
| # | Checkmate |
| O-O | Kingside castle |
| O-O-O | Queenside castle |
| = | Promotion (e.g., e8=Q) |
| ! | Good move |
| !! | Brilliant move |
| ? | Mistake |
| ?? | Blunder |

### Example Moves

| Notation | Meaning |
|----------|---------|
| e4 | Pawn to e4 |
| Nf3 | Knight to f3 |
| Bxc6 | Bishop captures on c6 |
| Qd1+ | Queen to d1, giving check |
| Rfe1 | Rook from f-file to e1 (disambiguation) |
| exd5 | Pawn on e-file captures on d5 |

---

## Famous Openings — First 4 Moves

| Opening | White 1 | Black 1 | White 2 | Black 2 | White 3 | Black 3 | White 4 | Black 4 | ECO |
|---------|---------|---------|---------|---------|---------|---------|---------|---------|-----|
| **Italian Game** | e4 | e5 | Nf3 | Nc6 | Bc4 | Bc5 | c3 | Nf6 | C54 |
| **Ruy Lopez** | e4 | e5 | Nf3 | Nc6 | Bb5 | a6 | Ba4 | Nf6 | C80 |
| **Sicilian Defense (Najdorf)** | e4 | c5 | Nf3 | d6 | d4 | cxd4 | Nxd4 | Nf6 | B90 |
| **Queen's Gambit Declined** | d4 | d5 | c4 | e6 | Nc3 | Nf6 | Bg5 | Be7 | D35 |
| **King's Indian Defense** | d4 | Nf6 | c4 | g6 | Nc3 | Bg7 | e4 | d6 | E70 |
| **French Defense** | e4 | e6 | d4 | d5 | Nc3 | Nf6 | Bg5 | Be7 | C13 |
| **Caro-Kann Defense** | e4 | c6 | d4 | d5 | Nc3 | dxe4 | Nxe4 | Bf5 | B18 |
| **Scotch Game** | e4 | e5 | Nf3 | Nc6 | d4 | exd4 | Nxd4 | Nf6 | C45 |
| **Pirc Defense** | e4 | d6 | d4 | Nf6 | Nc3 | g6 | f4 | Bg7 | B09 |
| **English Opening** | c4 | e5 | Nc3 | Nf6 | Nf3 | Nc6 | g3 | d5 | A28 |

---

## Famous Games

| Game | Year | Players | Famous for |
|------|------|---------|------------|
| **The Immortal Game** | 1851 | Anderssen vs. Kieseritzky | Sacrificed both rooks, a bishop, and the queen — still won |
| **The Evergreen Game** | 1852 | Anderssen vs. Dufresne | Brilliant queen sacrifice leads to unstoppable mate |
| **The Opera Game** | 1858 | Morphy vs. Duke of Brunswick & Count Isouard | Elegant miniature — development and open lines |
| **Game of the Century** | 1956 | Fischer (age 13) vs. Byrne | Queen sacrifice at move 17 (Qb6!!) [VERIFY: exact move number] |
| **Kasparov vs. Deep Blue Game 6** | 1997 | Kasparov vs. Deep Blue | Machine beats world champion — Kasparov resigned after 19 moves |
| **Kasparov vs. Topalov** | 1999 | Kasparov vs. Topalov | "Kasparov's Immortal" — Rook sacrifice leads to king hunt across the board |

---

## Checkmate Patterns

| Pattern | Description | Pieces involved |
|---------|-------------|-----------------|
| **Scholar's Mate** | 4-move checkmate: 1.e4 e5 2.Bc4 Nc6 3.Qh5 Nf6 4.Qxf7# | Queen + Bishop |
| **Fool's Mate** | 2-move checkmate (fastest possible): 1.f3 e5 2.g4 Qh4# | Queen only (Black wins) |
| **Back Rank Mate** | Rook or Queen delivers mate on 1st/8th rank; King trapped by own pawns | Rook or Queen |
| **Smothered Mate** | Knight delivers checkmate; King surrounded by own pieces | Knight |
| **Anastasia's Mate** | Knight + Rook; King on h-file, Knight controls escape square | Knight + Rook |
| **Arabian Mate** | Rook + Knight; Rook on 8th rank, Knight covers escape | Rook + Knight |
| **Epaulette Mate** | Queen checkmates; King flanked by own pieces on both sides | Queen |
| **Boden's Mate** | Two bishops deliver mate on criss-crossing diagonals | Two Bishops |
| **Legall's Mate** | Knight + Bishop(s); arises from a queen sacrifice trap | Knight + Bishop(s) |
| **Damiano's Mate** | Queen + Pawn; Queen on h7 (or h2), pawn supports | Queen + Pawn |

---

## Board Geometry

| Fact | Value |
|------|-------|
| Total squares | 64 (8x8) |
| Light squares | 32 |
| Dark squares | 32 |
| a1 square color | Dark (by convention) |
| h1 square color | Light |
| Maximum squares a Queen can reach from center (d4) | 27 |
| Maximum squares a Knight can reach from center | 8 |
| Total possible first moves (White) | 20 (16 pawn moves + 4 knight moves) |
| Shannon number (game-tree complexity) | ~10^120 [VERIFY: commonly cited approximation] |

---

## Piece Values (Standard)

| Piece | Value (pawns) |
|-------|---------------|
| Pawn | 1 |
| Knight | 3 |
| Bishop | 3 (slightly > Knight in open positions) |
| Rook | 5 |
| Queen | 9 |
| King | -- (infinite / game-ending) |
