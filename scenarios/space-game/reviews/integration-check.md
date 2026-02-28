# DEAD RECKONING — Integration Check

**Date:** 2026-02-27

---

## Checklist

- [x] All 18 puzzles: editorial review passed (APPROVE or APPROVE WITH NOTES)
- [x] All 2 blocking editorial fixes applied (R1-04 bridge sentence, R3-03 rule text)
- [x] Meta mechanisms: all 4 verified
- [x] 80% rule: verified for all 3 round-metas
- [x] Answer words: no duplicates across the hunt
- [x] Cross-round conflicts: none blocking (one flag — see below)
- [x] Difficulty curve: verified across rounds and within rounds
- [x] Narrator voice: terse technical instrument-panel format consistent throughout
- [x] World consistency: all facts traceable to world/ canon
- [x] Cross-puzzle consistency: CREW-7741-OMICRON thread intact; timestamps aligned
- [x] Answer security: no plaintext in git-tracked files; ANSWERS.md encoded
- [x] Unlock gates: correct (R1 → R2 via R1-META; R2 → R3 via R2-META; FINAL via R3-META)

---

## Meta Verification

### R1-META — PHASE-LOCKED PAIR

Feeder inputs received: DECOYS · SHATTERED CARRIER · HIERATIC TRIPLET · HARMONIC ECHO · ABSORPTION SHADOW

Reference: 6 interference types in world canon.
Five feeders account for types 1, 2, 3, 4, 6. Type 5 (Phase-Locked Pair) is absent from all records.
Mechanism: "name the missing type" — unambiguous, one solution.

**80% check:** With 4 of 5 feeders, the solver has 4 types accounted for, 2 missing. Phase-Locked Pair's definition ("two signals in perfect synchronization — indistinguishable from own signal") is uniquely distinctive. The solver can identify it from the world reference without the fifth feeder. ✓

**Answer: PHASE-LOCKED PAIR** — matches world canon type name exactly. ✓

---

### R2-META — LOCKOUT

Feeder inputs received: FAULT · OFFLINE · STANDBY · ONLINE · READY

Reference: 6 system states in world canon.
Five feeders account for states ◉, ◇, ◌, ○, ●. State ⊘ (LOCKED) is absent from all internal records.
Step 1: identify LOCKED as missing. Step 2: name the operational procedure for systems in LOCKED state → LOCKOUT.

**80% check:** With 4 of 5 feeders, two states are unaccounted for. LOCKED's definition ("power available but restricted, requires deliberate action to unlock") is the only state describing deliberate restriction — identifiable from world reference even without the fifth feeder. ✓

**Answer: LOCKOUT** — world-canonical operational procedure. ✓

---

### R3-META — SENIOR OFFICER

Feeder inputs received: STASIS · BUFFER · FORCED · LOCKED · RESTRICTED

Career tier access table (world canon):
| Action | Technician | Rotation | Diagnostician | Senior Officer |
|--------|-----------|----------|---------------|----------------|
| Emergency STASIS | ✗ | ✗ | ✗ | ✓ |
| BUFFER expansion | ✗ | ✗ | ✗ | ✓ |
| FORCED override (all gates) | ✗ | ✗ | ✓ (1 gate) | ✓ |
| LOCKED imposition | ✗ | ✗ | ✗ | ✓ |
| RESTRICTED auth bypass | ✗ | ✗ | ✗ | ✓ |

Only SENIOR OFFICER has ✓ on all five rows. Unambiguous.

**80% check:** With 4 of 5 feeder words, the solver checks 4 actions. All 4 require SENIOR OFFICER exclusively. The fifth action (whichever is missing) also requires Senior Officer — the pattern is conclusive with 4 of 5. ✓

**Answer: SENIOR OFFICER** — world-canonical career tier. ✓

---

### FINAL META — CALIBRATE

Round-meta inputs: PHASE-LOCKED PAIR · LOCKOUT · SENIOR OFFICER

Three clue paths from world definitions:

| Input | World connection | Points to |
|-------|-----------------|-----------|
| PHASE-LOCKED PAIR | Phase-locking a signal requires precise frequency adjustment of the receiving array | CALIBRATE |
| LOCKOUT | System lockout can only be initiated through CALIBRATE authority (the console command that imposes LOCKED state) | CALIBRATE |
| SENIOR OFFICER | CALIBRATE is the console meta-control exclusively unlocked at Senior Officer tier | CALIBRATE |

All three paths converge on a single world-canonical console command. Mechanism: find the common command.

**Answer: CALIBRATE** — world-canonical console meta-control. ✓

**Narrative payoff:** The Senior Officer used CALIBRATE authority to establish contact with the phase-locked entity and then CALIBRATE the records to hide it. The hunt title (DEAD RECKONING) resolves: you navigated without data and arrived at the word that names what was done. The answer IS the commission.

---

## Answer Word Audit

### No duplicates within any round

| Round 1 | Round 2 | Round 3 | Round-metas | Final |
|---------|---------|---------|-------------|-------|
| DECOYS | FAULT | STASIS | PHASE-LOCKED PAIR | CALIBRATE |
| SHATTERED CARRIER | OFFLINE | BUFFER | LOCKOUT | |
| HIERATIC TRIPLET | STANDBY | FORCED | SENIOR OFFICER | |
| HARMONIC ECHO | ONLINE | LOCKED | | |
| ABSORPTION SHADOW | READY | RESTRICTED | | |

No word appears twice. ✓

### Cross-round flag (non-blocking)

**LOCKED** appears as a Round 3 feeder answer (R3-04) AND as the missing system state underlying the Round 2 meta answer (LOCKOUT). These are in separate contexts and rounds — a solver who has completed Round 2 will have already resolved the LOCKED→LOCKOUT connection and will not confuse it with the Round 3 answer. No mechanical conflict. Flagged for solver testing observation.

---

## Difficulty Curve

### Within rounds (each round escalates)

| Round | Puzzle | Difficulty | Est. time |
|-------|--------|-----------|-----------|
| TELEMETRY | R1-01 Carrier Wave | Medium | 20-25 min |
| | R1-02 Dead Reckoning | Medium | 20-25 min |
| | R1-03 Contact Report | Medium-Hard | 25-30 min |
| | R1-04 Gate Log | Medium | 20-25 min |
| | R1-05 Anomaly Station | Hard | 30-35 min |
| | R1-META | Medium | 10-15 min |
| SYSTEMS LOG | R2-01 Load Shedding | Medium | 20-25 min |
| | R2-02 Pathway Trace | Medium | 20-25 min |
| | R2-03 Atmospheric Incident | Medium | 20-25 min |
| | R2-04 Burn Pattern | Medium-Hard | 25-30 min |
| | R2-05 Partial Recovery | Medium | 20-25 min |
| | R2-META | Medium | 10-15 min |
| CREW RECORD | R3-01 Vital Signs | Medium | 20-25 min |
| | R3-02 Pattern Integrity | Medium | 20-25 min |
| | R3-03 Access Record | Medium | 20-25 min |
| | R3-04 Repair Queue | Medium-Hard | 25-30 min |
| | R3-05 Unauthorized Request | Hard | 30-35 min |
| | R3-META | Medium | 10-15 min |
| FINAL | The Commission | Medium-Hard | 15-20 min |

**Across rounds:** Round 3 is the hardest (R3-05 is the hunt's hardest individual puzzle). Round 1 introduces the control-panel idiom. Round 2 requires deeper systems knowledge. Round 3 requires the most cross-referencing. Appropriate escalation. ✓

**Total estimated time (team of 3-4):** 4.5–6 hours. Consistent with SCOPE.md target of 4–6 hours. ✓

---

## Voice Consistency

All 18 puzzles use instrument-panel format:
- No puzzle instructions or explanatory text directed at solvers
- Each puzzle IS its instrument (spectrum display, badge log, strip chart, etc.)
- Bezel legends are in-world reference material, not puzzle hints
- Analyst fields are in-character forms, not solver worksheets

Consistent throughout. ✓

---

## Cross-Puzzle Consistency

### CREW-7741-OMICRON thread (Round 3)

| Puzzle | Role | Timestamp | Detail |
|--------|------|-----------|--------|
| R3-01 | Subject of stasis scan | 11:22:07 | Post-stasis bio-signature confirmed |
| R3-05 | Single approver of 4 SEC items | 14:51–16:09 | Defense weapon, encrypted comms, surveillance array, restraints |
| R3-02 | Pattern acquired for transport | 15:09:11 | Returning to ship from external coordinates |
| R3-META | Named as recurring subject | — | All three appearances cited |

Narrative coherence: subject was put in stasis → revived → used Senior Officer credentials to acquire restricted equipment → transported to external rendezvous with UNKNOWN-ENTITY → returned to ship. Timestamps do not conflict. ✓

### Logging systems (R3-04 ↔ Rounds 1 and 2)

R3-04 shows four logging/record-keeping systems placed in LOCKED state early in the gap window (T-02 ≈ 13:24). These are: COMMS-LOG-ARCHIVE, SENSOR-RECORD-BUFFER, NAV-TELEMETRY-STORE, TRANSIT-PATTERN-LOG. This explains why the telemetry (Round 1) and systems logs (Round 2) have gaps — the record-keeping systems were locked before most of the gap's events occurred. Narrative coherence confirmed. ✓

### Timestamps across rounds

- Deleted window: 11:00–17:00 (used consistently in all puzzles that reference specific times)
- R3-04 T-01 snapshot ≈ 12:12 / T-02 ≈ 13:24 — logging systems locked early
- R3-05 replicator requests: 14:51–16:09 — after logging locked
- R3-02 transport: 15:07–15:09 — after logging locked
- R3-03 impossible swipes: 14:32, 14:45 — after logging locked
- All events consistent with cover-up sequence: lock records first, then act. ✓

---

## World Consistency

| Fact | Puzzle(s) | Source | Status |
|------|----------|--------|--------|
| 6 interference types | R1-01–R1-05, R1-META | world/WORLD.md | ✓ |
| 8 comm bands | R1-01, R1-04 | world/WORLD.md | ✓ |
| 6 system states + symbols | R2-01–R2-05, R2-META, R3-04 | world/WORLD.md | ✓ |
| EPS consumption figures | R2-01, R2-05 | world/WORLD.md | ✓ |
| Electrical vs EPS grid assignments | R2-05 | world/WORLD.md | ✓ |
| 15 ODN nodes (names + order) | R2-02 | world/systems/data-tables.md | ✓ |
| Atmospheric nominal values | R2-03 | world/WORLD.md | ✓ |
| Impulse drive MW profile | R2-04 | world/WORLD.md | ✓ |
| Deck layout (5 decks, room ID format) | R3-03, R3-META | world/systems/data-tables.md | ✓ |
| Replicator security flags (6 categories) | R2-03, R3-05, R3-META | world/systems/data-tables.md | ✓ |
| Career tier progression + CALIBRATE | R3-META, FINAL META | world/WORLD.md | ✓ |
| Console meta-controls (AUTO/DIAGNOSTIC/CALIBRATE) | FINAL META | world/WORLD.md | ✓ |

**One item to add to world data tables before delivery build:**
- Holodeck EPS draw (55 MW) — established in R2-01 puzzle data but not in world/systems/data-tables.md. Add to Table 1 (ODN) or create a system power consumption table.

---

## Unlock Gate Verification

| Gate | Condition | Unlocks |
|------|-----------|---------|
| R1-META | 4 of 5 Round 1 feeders solved | Round 2 |
| R2-META | 4 of 5 Round 2 feeders solved | Round 3 |
| R3-META | 4 of 5 Round 3 feeders solved | Final meta |
| FINAL | All 3 round-metas solved | Answer submission |

Sequential unlock structure is correct. No circular dependencies. Within each round, all 5 feeders are available simultaneously. ✓

---

## Answer Security

- No plaintext answers in any git-tracked file ✓
- ANSWERS.md at scenario root — ROT13 encoded ✓
- META-DESIGN.md contains no answer values — mechanism descriptions only ✓
- PUZZLES.md answer fields read "(encoded)" — answer notes removed ✓

---

## Outstanding Items for Delivery Build (Stage 9)

### Required before platform test
1. Add holodeck EPS draw (55 MW) to world/systems/data-tables.md
2. R2-02: Consider removing pre-filled path worksheet (optional difficulty adjustment)
3. R2-04: Clarify sequence-count sub-question in analyst field (optional)
4. R2-META: Consider reducing LOCKOUT procedure scaffolding (optional)

### Delivery format decisions needed
- Digital delivery: website or downloadable files? (Per SCOPE.md: "digital — website or files")
- Answer submission: form-based with validation, or honor system?
- Hint system: timed hints? on-demand? none? (See toolkit/HINTS.md)
- Certification display: how does "COMMISSIONED OFFICER" get shown to solvers after final meta?

### Puzzles that will need visual/graphic treatment
- R1-03: Polar grid diagram — ASCII sufficient for testing, may need SVG for delivery
- R1-05: Six-band false-color spectral analyzer — ASCII sufficient for testing
- R2-02: ODN topology diagram — ASCII node-and-junction grid, may need cleaner rendering
- R2-04: Strip chart recorder — ASCII waveform works well as-is, thematically appropriate

---

## Verdict

**Integration check PASSED.**

All 18 puzzles are internally consistent. Cross-puzzle references are verified. All 4 meta mechanisms confirmed. Difficulty curve is appropriate for puzzle enthusiasts. Answer security is clean. The hunt is ready for delivery build (Stage 9) and platform test (Stage 10).

The narrative arc is complete: players reconstruct six missing hours via TELEMETRY (what was out there) → SYSTEMS LOG (how the ship recorded it) → CREW RECORD (what happened to people) → THE COMMISSION (what was done and why). The final answer CALIBRATE is earned, world-grounded, and thematically satisfying.
