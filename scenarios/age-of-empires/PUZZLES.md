# Puzzles — Master Registry

Every puzzle in the hunt. Filled out during Stage 4 (Assignment), maintained through authoring and testing. This is the single source of truth for puzzle status.

---

## How to Use

1. **Stage 4** — fill in the assignment table (ID, round, section, type, answer, testers)
2. **Stage 6** — update status as each puzzle is authored and tested
3. **Stage 7** — verify all puzzles pass before integration
4. Run `/puzzle-status` to generate a live dashboard from this file

---

## Hunt Identity

| Field | Value |
|-------|-------|
| **Hunt name** | |
| **Content library** | (path to your content) |
| **Rounds** | (number) |
| **Puzzles per round** | |
| **Total puzzles** | |
| **Metas** | (number — one per round? plus super-meta?) |
| **Format** | (book / website / live event / hybrid) |
| **Audience** | (solo / team of N / public) |
| **Estimated solve time** | |

---

## Puzzle Registry

### Status codes

| Code | Meaning |
|------|---------|
| `—` | Not started |
| `BRIEF` | Brief written (Stage 3/4) |
| `AUTHORED` | Full puzzle written |
| `TESTING` | Sent to testers |
| `PASS` | Passed testing (≥22/30, all dims ≥4) |
| `REVISE` | Failed testing, revision needed |
| `REVISED` | Revised, awaiting retest |
| `SHIP` | Final, ready for integration |

### The Registry

<!-- Copy rows as needed. One row per puzzle. -->

| ID | Round | Section | Puzzle type | Answer (encoded) | Difficulty | Status | Score | Tester 1 | Tester 2 | Tester 3 | Notes |
|----|-------|---------|-------------|-----------------|------------|--------|-------|----------|----------|----------|-------|
| | | | | | | — | | | | | |
| | | | | | | — | | | | | |
| | | | | | | — | | | | | |
| | | | | | | — | | | | | |
| | | | | | | — | | | | | |
| | | | | | | — | | | | | |
| | | | | | | — | | | | | |
| | | | | | | — | | | | | |

### Column guide

| Column | What to fill in |
|--------|----------------|
| **ID** | Unique identifier (number, element symbol, code — your choice) |
| **Round** | Which round this puzzle belongs to (1, 2, 3... or "meta") |
| **Section** | Which content section it references |
| **Puzzle type** | Mechanism (cipher, crossword, logic grid, identification, calculation, etc.) |
| **Answer (encoded)** | The answer word, encoded per your answer security protocol. NEVER plaintext. |
| **Difficulty** | 1-5 scale (1=warm-up, 5=expert) |
| **Status** | See status codes above |
| **Score** | Test score (N/30) — filled after testing |
| **Tester 1-3** | Which reviewer personas test this puzzle (from profiles/) |
| **Notes** | Issues, revision history, special considerations |

---

## Meta Registry

| Meta ID | Round(s) | Feeds from | Mechanism | Answer (encoded) | Status | Score |
|---------|----------|-----------|-----------|-----------------|--------|-------|
| | | | | | — | |
| | | | | | — | |
| | | | | | — | |

### Super-meta (if applicable)

| Field | Value |
|-------|-------|
| **Feeds from** | (which round metas) |
| **Mechanism** | |
| **Answer (encoded)** | |
| **Status** | |

---

## Tester Assignments

Pre-assign 3 testers per puzzle based on puzzle type. Use this table or create your own.

| Puzzle type | Recommended testers |
|------------|-------------------|
| Cipher / code-breaking | Huang, Blow, Snyder |
| Logic grid / deduction | Huang, Katz, Pope |
| Crossword / word puzzle | Katz, Dana, Rosenthal |
| Calculation / diagram | Kenny, Snyder, Sarrett |
| Identification / matching | Pope, Miller, Rosenthal |
| Visual / physical | Sarrett, Miller, Blow |
| Narrative / timeline | Selinker, Katz, Rosenthal |
| Cross-section synthesis | Blow, Selinker, Katz |

Customize based on your hunt. Spread load evenly (8-13 tests per reviewer max).

---

## Progress Summary

*Updated by `/puzzle-status` or manually.*

| Metric | Count |
|--------|-------|
| Total puzzles | |
| Authored | |
| Tested | |
| Passed | |
| Need revision | |
| Ship-ready | |
| Metas designed | |
| Metas tested | |

---

## Revision Log

Track what changed and why. Learn from test failures.

| Puzzle ID | Version | Score | Verdict | Key issue | Fix applied | New score |
|-----------|---------|-------|---------|-----------|-------------|-----------|
| | v1 | | | | | |
| | v2 | | | | | |

---

## Answer Encoding Reference

Answers in this file are encoded, not plaintext. **Plaintext answers must never appear in git-tracked files.**

Choose your encoding during `/puzzle-plan` Stage 1. Options: ROT13, Base64, custom cipher, or store answers only in `.claude/` project memory.

Document your chosen encoding key in `.claude/` project memory (gitignored), NOT here.
