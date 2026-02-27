# World Data — Game Night

Five board games. Each file contains the rules, components, and verifiable facts needed to build one puzzle. Authors should use ONLY these files (plus general board game knowledge) as their source material.

---

## Data File Index

| File | Game | What it covers | Puzzle module |
|------|------|----------------|---------------|
| `chess.md` | Chess | Pieces, notation, openings, famous games, checkmate patterns | M1 |
| `settlers.md` | Settlers of Catan | Hex types, resources, building costs, dev cards, ports, robber | M2 |
| `risk.md` | Risk | Territories, continents, bonuses, combat dice, card sets | M3 |
| `pandemic.md` | Pandemic | Roles, infection, outbreaks, cures, epidemic cards | M4 |
| `codenames.md` | Codenames | Grid layout, spymaster rules, clue mechanics, card types | M5 |

---

## Usage Rules

1. **Authors see only their assigned game file + WORLD.md.** Module isolation means M1 author reads `chess.md` but not `pandemic.md`.
2. **Facts marked [VERIFY] are believed correct but should be double-checked before puzzle publication.** Do not build a puzzle's critical path on an unverified fact.
3. **These files are reference, not flavor text.** The puzzle should USE this data (Riven Standard), not merely decorate with it.
4. **No Deliberate Errors.** Every fact in these files is intended to be correct. If an author finds an error, flag it — do not use it as a puzzle element.

---

## Coverage Notes

Each file aims to provide enough data for 2-3 different puzzle approaches. The files are intentionally richer than any single puzzle needs — the author picks which slice of the data to use.

- **Chess**: deep on notation and openings (supports position-reconstruction and opening-identification puzzles)
- **Settlers**: deep on resource economics (supports building-cost and trading puzzles)
- **Risk**: deep on geography and combat math (supports territory-identification and probability puzzles)
- **Pandemic**: deep on role abilities and outbreak chains (supports deduction and chain-tracing puzzles)
- **Codenames**: deep on clue constraints (supports spymaster-simulation puzzles)
