# Modules — Author Assignments

A module is a unit of work: one author, one deliverable, one review cycle. Modules are how work gets divided when multiple authors (humans or AI sessions) contribute to the same hunt.

---

## Module Registry

| Module ID | Scope | Scenario | Round | Assigned to | Status | Deliverable | Review |
|-----------|-------|----------|-------|-------------|--------|-------------|--------|
| | | | | | unassigned | | |

### Scope types

| Scope | What the author delivers |
|-------|------------------------|
| `single_puzzle` | One complete puzzle (authored + tested + passing) |
| `round` | All puzzles in one round (authored + tested) |
| `meta` | The meta puzzle for a round or the super-meta |
| `physical_build` | A physical puzzle component (cipher wheel, punch card, etc.) |
| `narrative` | Narrator voice, intros, closing text |
| `testing_pass` | Test all puzzles in a round (no authoring, just testing) |

---

## Author Registry

| Author ID | Name | Type | Sessions | Modules completed |
|-----------|------|------|----------|-------------------|
| | | human / ai | | |

### Author types

- **human**: A person using Claude Code to author puzzles
- **ai**: A Claude session that authored puzzles autonomously

---

## Module Workflow

```
1. Create module in MODULES.md (assign scope, scenario, round)
2. Assign to author
3. Author works (using /puzzle-author, /puzzle-test)
4. Author marks module "review_pending"
5. Run /puzzle-review on the deliverable
6. If pass → "complete"
7. If revise → "revision_needed" → author fixes → back to step 4
8. When all modules for a round are complete → integration
```

---

## Module Assignment Rules

- One author per module (no shared ownership)
- An author can have multiple modules (but only 1 in_progress at a time)
- Modules for the same round should be assigned to different authors (diversity of perspective)
- Meta modules should be assigned AFTER all feeder modules are complete
- Narrative modules can run in parallel with puzzle modules

---

## Honor — NATO Phonetic Callsigns

After completing a significant module, the author claims a NATO phonetic callsign. 26 callsigns for 26 sessions. See `_vocab/honor.json` for the full list with meanings.

| # | Callsign | Claimed by | Module | Date |
|---|----------|-----------|--------|------|
| 1 | **Alpha** | Claude Opus 4.6 | Toolkit foundation — schemas, skills, principles, 12 profiles, AoE scenario, bug fixes, NATO honor system | Feb 27 2026 |
| 2 | Bravo | | | |
| 3 | Charlie | | | |
| 4 | Delta | | | |
| 5 | Echo | | | |
| 6 | Foxtrot | | | |
| 7 | Golf | | | |
| 8 | Hotel | | | |
| 9 | India | | | |
| 10 | Juliet | | | |
| 11 | Kilo | | | |
| 12 | Lima | | | |
| 13 | Mike | | | |
| 14 | November | | | |
| 15 | Oscar | | | |
| 16 | Papa | | | |
| 17 | Quebec | | | |
| 18 | Romeo | | | |
| 19 | Sierra | | | |
| 20 | Tango | | | |
| 21 | Uniform | | | |
| 22 | Victor | | | |
| 23 | Whiskey | | | |
| 24 | X-ray | | | |
| 25 | Yankee | | | |
| 26 | Zulu | | | |
