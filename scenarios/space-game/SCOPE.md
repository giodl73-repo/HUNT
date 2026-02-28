# DEAD RECKONING — Scope

## Hunt Identity

| Field | Value |
|-------|-------|
| **Name** | DEAD RECKONING |
| **Genre** | Sci-fi investigation / certification hunt |
| **Scale** | 15 puzzles + 3 round-metas + 1 final meta |
| **Audience** | Puzzle enthusiasts |
| **Format** | Digital — website or downloadable files |
| **Duration** | 4–6 hours |
| **Structure** | 3 rounds, sequential unlock |
| **Tone** | Trek/Firefly register. Professional but human. Wry, tired, competent. |

## The Frame

You are a new officer aboard a working starship. Today was supposed to be your certification exam — the real-world assessment that proves you can operate independently at your station.

Then the incident happened.

Six hours of ship logs are missing. Not corrupted — deleted. The senior officers who were on duty aren't talking. The CO has ordered a full investigation and, in a move that is either inspired or desperate, has assigned it to you. Solve the incident and you'll have proven something no simulation could: that you can operate under real conditions.

Clear all three evidence rounds. Earn your commission.

**Dead reckoning** (n): the process of estimating one's current position based on a previously known position, without access to current navigation data. You know where the ship was before. You know where it was after. Navigate the gap.

## The Incident

Six hours of logs are missing from the ship's records. Not a system failure — the deletion is targeted, precise, and covered with forged timestamps. Something happened during those six hours. The ship was changed by it. The crew was changed by it.

Players reconstruct the missing six hours from three evidence types, each forming one round.

## Round Structure

### Round 1 — TELEMETRY
*What was out there.*

External evidence: navigation records, sensor logs, comm band activity, tactical alerts, signal intercepts. What the ship was looking at during the gap, reconstructed from echoes and indirect records that weren't caught in the deletion sweep.

**Departments in play:** COMMS, NAV, TAC, SEC, SCIENCE
**Certification earned:** OPERATIONS

### Round 2 — SYSTEMS LOG
*What the ship recorded internally.*

The deletion couldn't touch everything. Power consumption records, EPS routing logs, computer processing allocations, environmental adjustments, ODN data pathways. The ship's body has a memory the crew didn't think to scrub.

**Departments in play:** POWER, COMPUTER, ENVIRO, PROPULSION, AUXILIARY
**Certification earned:** ENGINEERING

### Round 3 — CREW RECORD
*What happened to the people.*

Medical intake logs. Transporter usage records. Replicator requests during the gap window. Security badge swipes. Personnel assignment anomalies. The crew left traces the logs didn't capture — because people are harder to redact than timestamps.

**Departments in play:** MEDICAL, TRANSIT, SEC (personnel), DAMAGE CONTROL, REPLICATOR
**Certification earned:** COMMAND

### Final Meta — THE COMMISSION
*What actually happened.*

Each round-meta produces one piece of the picture:
- Round 1 reveals **WHAT** was encountered
- Round 2 reveals **HOW** the ship responded
- Round 3 reveals **WHO** made the decision to cover it up

The final meta combines all three to answer the only question that matters: **what happened during those six hours, and why was it hidden?**

## Certification Arc

| Milestone | How earned | Title granted |
|-----------|-----------|---------------|
| Round 1 complete | Solve all 5 feeders + round-meta | Operations Certified |
| Round 2 complete | Solve all 5 feeders + round-meta | Engineering Certified |
| Round 3 complete | Solve all 5 feeders + round-meta | Command Certified |
| Final meta | Reconstruct the incident | Commissioned Officer |

The certification framing is in-world. Players are receiving actual certification credits. The investigation was the exam.

## The Answer Architecture

| Element | Answers to |
|---------|-----------|
| Round 1 meta | WHAT — the nature of the contact |
| Round 2 meta | HOW — the ship's response |
| Round 3 meta | WHO — the person who ordered the cover-up |
| Final meta | THE INCIDENT — one phrase describing what was hidden and why |

## Tone Reference

**The ship:** Trek/Firefly. Real switches. Scuff marks on the bulkheads. The replicator gives you lukewarm coffee. Not utopia — a working vessel where the people are professional, occasionally wry, and genuinely invested in the ship and each other.

**Evidence voice:** Each puzzle IS a document type — sensor log, EPS routing report, medical intake form, transporter log. No puzzle instructions overlay. The solver reads the document as an officer would, using ship knowledge to extract the embedded puzzle.

**Player register:** You're new, but you're not stupid. The CO gave you this because they think you can handle it. Prove it.

## World Reference

All puzzle facts verified against:
- `world/WORLD.md` — ship systems overview, 16 departments, power architecture, console model
- `world/systems/departments.md` — per-department detail, cross-links, failure modes, puzzle hooks

No facts from memory. Every clue traceable to the world files.

## Answer Security

ROT13 encoded. Stored in `.claude/` project memory only. No plaintext answers in git-tracked files.
