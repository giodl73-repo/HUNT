# Wololo — An Age of Empires Puzzle Hunt

**Scenario CLAUDE.md** — Claude reads this when working in this directory.

## Hunt Summary

| Field | Value |
|-------|-------|
| **Name** | Wololo |
| **Theme** | Age of Empires (AoE2 primarily, all versions welcome) |
| **Scale** | 5 puzzles + 1 meta |
| **Audience** | Solo AoE player, casual puzzler |
| **Narrator** | The Monk (short sentences, present tense, no exclamation marks) |
| **Solve time** | 2-3 hours |
| **Purpose** | Toolkit pipeline test — validating all 8 stages |

## Content Library

This hunt uses AoE game knowledge as its content library. There are no encyclopedia files — the solver's own AoE experience is the reference. For authoring/testing, Claude uses web knowledge of Age of Empires.

## Files

| File | Stage | Status |
|------|-------|--------|
| `SCOPE.md` | Stage 1 | ✅ Complete |
| `ROUNDS.md` | Stage 2 | ✅ Complete — 5 puzzles + 1 meta, roman numeral ages, WOLOLO |
| `PUZZLES.md` | Stage 4 | ✅ Complete — 5 puzzles assigned, testers assigned, WOLOLO meta |
| `HINTS.md` | Stage 8 | Template |
| `meta/META-DESIGN.md` | Stage 5 | Template |
| `puzzles/` | Stage 6 | Empty |
| `reviews/` | — | Empty |
| `tests/` | — | Empty |

## Toolkit Reference

- Principles: `../../toolkit/PRINCIPLES.md`
- Profiles: `../../toolkit/profiles/`
- Skills: `../../toolkit/skills/`
- Templates: `../../toolkit/templates/`

## Instructions for Claude

When authoring puzzles for this scenario:
- The "content library" is AoE game knowledge (civilizations, units, techs, maps, strategies)
- Apply all 18 design principles from the toolkit
- The Monk narrates — follow the voice rules
- Keep puzzles accessible to anyone who has played AoE2 for a few hundred hours
- The meta answer should be a memorable AoE reference
