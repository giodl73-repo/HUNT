# THE GRAND LARCENY — Structure

## Architecture

```
THE GRAND LARCENY — 1 round, 4 puzzles + 1 meta

Round 1: THE CASE FILE
├── P1: The Police Report     (logic/grid)       → answer word
├── P2: The Hotel Receipt     (arithmetic/pattern) → answer word
├── P3: The Floor Plan        (spatial/path)      → answer word
└── P4: The Witness Statement (text/hidden message) → answer word
         │
         ▼
    META: The Method          (combination)       → final answer
```

## Single Round — "The Case File"

All four puzzles are available from the start. Teams can solve in any order. Each puzzle is a physical document in the evidence dossier. There is no unlock gating — the dossier is the full hunt.

## Puzzle Count

| Category | Count |
|----------|-------|
| Feeder puzzles | 4 |
| Meta puzzles | 1 |
| **Total** | **5** |

## Difficulty Curve

No difficulty ramp — all four puzzles are mid-range difficulty. The variety is in puzzle TYPE, not difficulty level. Estimated solve times:

| Puzzle | Type | Estimated time | Difficulty |
|--------|------|---------------|------------|
| P1 | Logic grid | 20-30 min | Medium |
| P2 | Arithmetic pattern | 15-25 min | Medium |
| P3 | Spatial path | 20-30 min | Medium-Hard |
| P4 | Hidden text | 15-25 min | Medium |
| META | Combination | 10-20 min (after all 4 solved) | Medium |

**Total estimated time:** 80-130 minutes for a team of 2-4.

## Numbering System

Evidence numbering: P1, P2, P3, P4. The evidence log in the dossier uses "EXHIBIT A, B, C, D" as the in-universe numbering. The meta is "CASE SUMMARY."

## The 80% Rule

With 4 feeders, 80% = 3 answers. The meta should be solvable with 3 of 4 answers and a reasonable guess for the 4th. This is tight — design the meta so that 3 answers plus the structure strongly constrain the 4th.

## Meta Architecture

The four feeder answers are:
- P1 → a word (representing the suspect dimension)
- P2 → a word (representing the location dimension)
- P3 → a word (representing the time dimension)
- P4 → a word (representing the object dimension)

These four words, when combined with the hotel's narrative, reveal THE METHOD — how the painting was physically stolen. The meta mechanism is defined in `meta/META-DESIGN.md`.

## Delivery

| Medium | Role | Content |
|--------|------|---------|
| Physical dossier | PRIMARY | All 4 puzzles as printed documents, cover sheet, evidence log |
| Website | SECONDARY | Answer submission only. No puzzle content. |
| Props | SUPPLEMENT | Hotel key card (P3), calling card (META flavor) |

## World Canon Lock

Before authoring begins, all world/ systems must be locked:
- `world/systems/layout.md` — hotel floor plan and room codes
- `world/systems/characters.md` — suspect alibis and staff details
- `world/systems/timeline.md` — gala schedule and theft window
- `world/systems/services.md` — room charges, receipts, phone logs
