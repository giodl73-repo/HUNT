# DEAD RECKONING — Meta Design

**Answer registry:** `ANSWERS.md` (ROT13 encoded). Do not put answers in this file.

---

## The Questions

The hunt reconstructs six missing hours through three evidence rounds.

| Round | Question |
|-------|---------|
| TELEMETRY | What was encountered? |
| SYSTEMS LOG | How did the ship respond? |
| CREW RECORD | Who gave the order? |
| FINAL | What was done — and what does that reveal? |

---

## R1-META — Telemetry Reconstruction

### Mechanism

The SPACEGAMIVERSE world defines exactly **six interference types**:

| # | Type |
|---|------|
| 1 | Decoys |
| 2 | Shattered Carrier |
| 3 | Hieratic Triplet |
| 4 | Harmonic Echo |
| 5 | Phase-Locked Pair |
| 6 | Absorption Shadow |

Five feeders (R1-01 through R1-05) each name one type present in the telemetry data during the deleted window. One type is conspicuously absent from all records.

**The meta panel presents:**
- The five identified interference types (the feeder answers)
- The complete six-type reference from the world
- The question: *"Five interference types were identified in the deleted telemetry. One was not observed as background noise. It was the signal itself. Name it."*

The missing type is the answer. It is defined in the world as "two signals locked together in perfect synchronization." This was not interference. This was contact: the unknown entity had phase-locked onto the ship's own transmissions, mirroring them so precisely the comm log could not distinguish it from the ship's own signal.

### 80% Rule

With 4 of 5 feeders solved, the solver has 4 interference types named. Two remain unaccounted for from the six-type reference. The definition of the answer type — *two signals in perfect synchronization* — is distinctive enough to identify from the world reference even without the fifth feeder. **80% rule satisfied.**

---

## R2-META — Systems Reconstruction

### Mechanism

The SPACEGAMIVERSE world defines exactly **six system states**:

| # | State | Symbol | Definition |
|---|-------|--------|------------|
| 1 | ONLINE | ◉ | Running (ACTIVE or IDLE) |
| 2 | READY | ◇ | Power available, can be activated |
| 3 | STANDBY | ◌ | Was running, put to sleep, instant wake |
| 4 | LOCKED | ⊘ | Power available but restricted; requires deliberate action to unlock |
| 5 | OFFLINE | ○ | No power, upstream dependency not met |
| 6 | FAULT | ● | Hardware failure, needs repair |

Five feeders (R2-01 through R2-05) each name one system state observed in the internal logs during the deleted window. One state is absent from every log entry — not because it didn't occur, but because it was scrubbed.

**The meta panel presents:**
- The five observed system states (the feeder answers)
- The complete six-state reference from the world (with symbols and definitions)
- Two questions:
  1. *"One system state does not appear anywhere in the internal records. Name it."* → the missing state
  2. *"What is the operational procedure when critical systems are placed in this state?"* → the answer

The two-step structure is the puzzle. The missing state is defined as "power available but restricted, requires deliberate action to unlock." The operational name for systems held in that state is the round-meta answer. The internal logs were put through this procedure — not by destroying the systems, but by restricting their records while keeping power available.

### 80% Rule

With 4 of 5 feeders, two states are unaccounted for. The solver must identify the missing state from its definition: "power available but restricted, requires deliberate action to unlock." That definition distinguishes it from every other state. **80% rule satisfied.**

---

## R3-META — Crew Reconstruction

### Mechanism

The five feeder words describe five actions taken during the cover-up:
- **STASIS** (R3-01): someone was placed in emergency stasis
- **BUFFER** (R3-02): the transporter pattern buffer was artificially expanded
- **FORCED** (R3-03): security gates were overridden by command authority
- **LOCKED** (R3-04): critical systems were placed in LOCKED state without standard repair logging
- **RESTRICTED** (R3-05): Tier 3 Restricted replicator items were authorized without dual sign-off

**The meta panel presents** the career tier access table from the world, showing which actions each tier can perform:

| Action | Technician | Rotation | Diagnostician | Senior Officer |
|--------|-----------|----------|---------------|----------------|
| Emergency STASIS authorization | ✗ | ✗ | ✗ | ✓ |
| Pattern BUFFER expansion (non-standard) | ✗ | ✗ | ✗ | ✓ |
| FORCED gate override (all gates) | ✗ | ✗ | ✓ (1 gate) | ✓ (all gates) |
| LOCKED state imposition without DC log | ✗ | ✗ | ✗ | ✓ |
| RESTRICTED tier single-authorization bypass | ✗ | ✗ | ✗ | ✓ |

**The question:** *"The following five actions were performed during the cover-up. Identify the minimum career tier with authorization to perform all five."*

Only one tier has authorization for all five. A Diagnostician can force a single gate but cannot impose LOCKED state, authorize emergency stasis, or bypass Restricted tier requirements unilaterally. The answer is the tier that can do everything the cover-up required.

### 80% Rule

With 4 of 5 feeder words, the solver checks 4 actions against the access table. Four actions all pointing exclusively to the same tier is already conclusive — the fifth action (whichever is missing) also requires the same tier, but the pattern is established with 4. **80% rule satisfied.**

---

## FINAL META — The Commission

### Mechanism

The three round-meta answers each contain a clue path that leads to the same console command. The final meta panel presents three framed clue statements drawn from the world:

```
TELEMETRY FINDING:
  "Achieving a _______ signal requires precise adjustment of the
   receiving array's frequency parameters to match the source."

  → The blank is the contact type identified in Round 1.
  → The action required to achieve it names the answer.

SYSTEMS FINDING:
  "Placing critical systems in LOCKED state — a ___________ —
   can only be initiated through the highest console authority."

  → The blank is the operational procedure identified in Round 2.
  → The authority required to impose it names the answer.

CREW FINDING:
  "One console command is exclusively unlocked at the [Round 3 answer] tier.
   It is defined in the world as: 'tune/adjust.'
   It is the final meta-control in the career progression."

  → The tier is identified in Round 3.
  → The command they alone possess names the answer.
```

All three clue paths lead to the same console command: the highest meta-control in the world, available only to the career tier identified in Round 3, defined as "tune/adjust."

**The question:** *"Three facts about the deleted six hours. Each points to the same console command. Name it."*

### Narrative Payoff

The hunt is called DEAD RECKONING. The solver navigated without data, estimated position from what was known before and after the gap, and arrived at a single word — the highest console command in the world, the one that proves mastery of the ship's systems.

The Senior Officer used this command to calibrate the ship's response to the phase-locked contact, and then used it again to lock out the records. The incident is named by the action that made it possible and then hid it.

The certification arc closes: you've proven you understand the ship well enough to identify what only a Senior Officer can do — and you've caught one misusing it. The answer IS the commission.

### 80% Rule — Final Meta

All three round-meta answers are required to reach the final meta. There is no 80% path; this is standard for final metas. The 80% rule applies within each round (handled above). The final is the synthesis layer.

---

## Verification Checklist

- [x] All 15 feeder answers are single world-vocabulary words or two-word world phrases
- [x] R1 feeders = exactly 5 of 6 interference types (missing sixth = round-meta answer)
- [x] R2 feeders = exactly 5 of 6 system states (missing sixth → operational procedure = answer)
- [x] R3 feeders = 5 actions that collectively profile one career tier only
- [x] R3-META access table uses only world-canonical career tier data
- [x] Final meta three clue paths use only world-canonical definitions
- [x] All three round-meta mechanisms are distinct (missing-from-set ×2, table-identification ×1)
- [x] Final meta answer is a world-canonical console command
- [x] Final meta answer definition ("tune/adjust") is in world documentation
- [x] No answers in this file — full registry in `ANSWERS.md`
- [x] 80% rule verified for all three round-metas
- [ ] Final meta tested with a volunteer solver (Stage 10)
