# Scope — Game Night

**Stage 1: SCOPE**

---

## Hunt Identity

| Field | Value |
|-------|-------|
| **Name** | Game Night |
| **Content library** | 5 classic board games: Chess, Settlers of Catan, Risk, Pandemic, Codenames |
| **Theme** | A game night where each game is a puzzle. You don't play them — you solve them. |
| **Audience** | Board game players (casual to serious) |
| **Scale** | 5 puzzles + 1 meta |
| **Authors** | 5 different AI personalities + 1 admin (meta) |
| **Narrator** | The Host — someone setting up game night. Dry, organized, slightly anxious about whether everyone will have fun. |
| **Format** | Printable Markdown |
| **Solve time** | 2-3 hours |

---

## Content Library: 5 Board Games

Each game becomes one puzzle. Each puzzle is a separate module assigned to a different author personality.

| Game | Module | Author | What makes it puzzle-worthy |
|------|--------|--------|---------------------------|
| **Chess** | M1 | The Professor | Deep positional logic, notation system, famous games |
| **Settlers of Catan** | M2 | The Speedrunner | Resource management, trading, hex placement |
| **Risk** | M3 | The Rebel | Territory control, probability, strategic alliances |
| **Pandemic** | M4 | The Collaborator | Cooperative mechanics, disease spread, role powers |
| **Codenames** | M5 | The Silent One | Word association, grid layout, the spymaster's constraint |

---

## Narrative: The Host

The Host is setting up for game night. Voice rules:
- Short sentences. Slightly flustered.
- Present tense. "The board is ready. The pieces are sorted. Mostly."
- No exclamation marks (too anxious for exclamation marks).
- References setup rituals: "Who wants to go first? Never mind, you're already solving."
- The Host addresses "you" but also worries about "the others."

---

## Module Workflow

```
Admin creates 5 briefs (one per game)
    ↓
Admin assigns each brief to an author personality
    ↓
5 authors work in parallel (each in their module directory)
    ↓
Authors submit puzzles → Admin reviews
    ↓
Admin resolves conflicts (The Rebel argues)
    ↓
Admin designs meta (after all 5 answers are locked)
    ↓
Integration → Live Test → Ship
```

---

## Answer Word Constraints

The meta mechanism is TBD (Stage 6 — after all 5 answers are locked). But the 5 answer words should:
1. Be board-game-related terms
2. Work in whatever meta mechanism the admin designs
3. Be independently chosen by each author (the realistic case — no coordination on answer words)

This tests Goal G5: can the admin build a meta from 5 uncoordinated answer words?

---

## Stage 1 Status: COMPLETE
