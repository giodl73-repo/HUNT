# R2-META — SYSTEMS RECONSTRUCTION
**Round:** SYSTEMS LOG
**Department:** ENGINEERING (ALL SECTIONS)
**Instrument:** Compiled Systems Summary Panel — Round Assessment Terminal
**Classification:** RESTRICTED — SYSTEMS LOG RECONSTRUCTION

---

## SYSTEMS RECONSTRUCTION — COMPILED ASSESSMENT TERMINAL

```
RECONSTRUCTION SUMMARY: GAP WINDOW [+00:00 → +06:00]
TERMINAL ID:             SYS-RECON-COMPILED
LOCATION:                MAIN ENGINEERING — DAMAGE CONTROL, DECK 5
ANALYST AUTHORITY:       SENIOR ENGINEER / RECONSTRUCTION BOARD
PURPOSE:                 Collate all internal system log findings from
                         the gap window. Identify which system states
                         were recorded. Flag anomalies for review board.
```

---

## COMPILED FINDINGS — SYSTEM STATE EVENTS RECORDED

> Summary of findings from five engineering department log reconstructions. Each entry shows the system analyzed, the log source, and the system state identified in the records.

```
┌──────────────────────────────────────────────────────────────────────────────┐
│  SYSTEMS LOG RECONSTRUCTION — FINDINGS SUMMARY                               │
├──────────┬──────────────────────┬──────────────────────────┬─────────────────┤
│ LOG ITEM │ DEPARTMENT           │ SYSTEM / FINDING         │ STATE RECORDED  │
├──────────┼──────────────────────┼──────────────────────────┼─────────────────┤
│ ITEM-01  │ POWER                │ EPS Distribution Board   │ ●               │
│          │                      │ PCU in fault recovery    │ [ ___________ ] │
│          │                      │ Phantom loads on 3 OFFLINE│                │
│          │                      │ channels: 160 MW total   │                 │
├──────────┼──────────────────────┼──────────────────────────┼─────────────────┤
│ ITEM-02  │ COMPUTER             │ ODN Junction Board       │ ○               │
│          │                      │ Toggle sequence isolated  │ [ ___________ ] │
│          │                      │ 2 nodes from primary      │                 │
│          │                      │ computer data path        │                 │
├──────────┼──────────────────────┼──────────────────────────┼─────────────────┤
│ ITEM-03  │ ENVIRO               │ Chemistry Panel          │ ◌               │
│          │                      │ Automation paused on     │ [ ___________ ] │
│          │                      │ Decks 3 & 4; atmosphere  │                 │
│          │                      │ drifted without correction│                 │
├──────────┼──────────────────────┼──────────────────────────┼─────────────────┤
│ ITEM-04  │ PROPULSION           │ Impulse Strip Chart      │ ◉               │
│          │                      │ Drive executed 2 complete│ [ ___________ ] │
│          │                      │ maneuver sequences while  │                 │
│          │                      │ official log shows static │                 │
├──────────┼──────────────────────┼──────────────────────────┼─────────────────┤
│ ITEM-05  │ AUXILIARY            │ Master Diagnostics Feed  │ ◇               │
│          │                      │ Combat systems configured │ [ ___________ ] │
│          │                      │ for immediate activation: │                 │
│          │                      │ Shields, Transporter,     │                 │
│          │                      │ Weapons-FC               │                 │
└──────────┴──────────────────────┴──────────────────────────┴─────────────────┘
```

---

## COMPLETE SYSTEM STATE REFERENCE

> Standard six-state reference. Mounted on all engineering panels. Source: Engineering Operations Manual, Section 1 — System Architecture.

```
╔═════════════════════════════════════════════════════════════════════════════╗
║  SYSTEM STATES — COMPLETE REFERENCE                                         ║
╠══════════╦══════════════════════════════════════════════════════════════════╣
║  SYMBOL  ║  STATE       DEFINITION                                          ║
╠══════════╬══════════════════════════════════════════════════════════════════╣
║    ◉     ║  ONLINE    — System running. Actively doing work (ACTIVE) or    ║
║          ║              monitoring at idle (IDLE). Power draw confirmed.    ║
╠══════════╬══════════════════════════════════════════════════════════════════╣
║    ◇     ║  READY     — Power available. System can be activated on        ║
║          ║              command. Awaiting activation order.                 ║
╠══════════╬══════════════════════════════════════════════════════════════════╣
║    ◌     ║  STANDBY   — Was running. Put to sleep. Setpoints and state     ║
║          ║              retained. Instant wake on RESUME command.           ║
╠══════════╬══════════════════════════════════════════════════════════════════╣
║    ⊘     ║  LOCKED    — Power available but restricted. Requires           ║
║          ║              deliberate action to unlock before system can       ║
║          ║              be activated. Activation blocked until unlocked.    ║
╠══════════╬══════════════════════════════════════════════════════════════════╣
║    ○     ║  OFFLINE   — No power. Upstream power dependency not met.       ║
║          ║              System cannot be activated without restoring        ║
║          ║              power upstream.                                     ║
╠══════════╬══════════════════════════════════════════════════════════════════╣
║    ●     ║  FAULT     — Hardware failure. System needs physical repair.    ║
║          ║              May operate unexpectedly or not at all.             ║
╚══════════╩══════════════════════════════════════════════════════════════════╝
```

---

## ENGINEERING OPERATIONS PROTOCOL — LOCKED STATE

> Excerpt from Engineering Operations Manual, Section 3 — System Control Procedures. Posted in all engineering sections.

```
SECTION 3.4 — LOCKED SYSTEM PROCEDURE

  When a system is placed in LOCKED (⊘) state:

    — The system has power available at its input.
    — The system CANNOT be activated without deliberate unlock action.
    — Activation commands are ignored until LOCKED state is cleared.
    — LOCKED state is used when a system must be available for rapid
      activation but must not activate accidentally or without authorization.

  Common uses of LOCKED state:
    — Weapons during docking operations (power available, firing blocked)
    — Emergency systems during routine operations
    — Any system placed under restricted access by command authority

  ┌─────────────────────────────────────────────────────────────────┐
  │  OPERATIONAL PROCEDURE FOR LOCKED SYSTEMS:                      │
  │                                                                 │
  │  When the Duty Officer declares a system in LOCKED (⊘) state,  │
  │  all engineering personnel must execute:                        │
  │                                                                 │
  │  PROCEDURE:   L _ _ _ O _ T                                    │
  │  (Full procedure name on file — Duty Officer will specify)      │
  │                                                                 │
  │  This procedure covers: access verification, override sequence, │
  │  two-person authorization, and system state log annotation.     │
  └─────────────────────────────────────────────────────────────────┘

  The LOCKED procedure name is drawn from the state itself:
  the standard term for the operational response to a LOCKED system.
```

---

## RECONSTRUCTION BOARD — ASSESSMENT QUESTIONS

```
ANALYST FIELD — FINAL ASSESSMENT

  QUESTION 1:
  ─────────────────────────────────────────────────────────────────────
  The five system log items above record five distinct system states.
  List the five states recorded in the gap window:

    Item-01: ● [ ___________ ]
    Item-02: ○ [ ___________ ]
    Item-03: ◌ [ ___________ ]
    Item-04: ◉ [ ___________ ]
    Item-05: ◇ [ ___________ ]

  ─────────────────────────────────────────────────────────────────────
  QUESTION 2:
  ─────────────────────────────────────────────────────────────────────
  The complete system state reference lists SIX states.

  Cross-reference your five findings against the six-state reference.

  Which system state does NOT appear anywhere in the internal records?

    Symbol: [ __ ]   State: [ ___________ ]

  ─────────────────────────────────────────────────────────────────────
  QUESTION 3:
  ─────────────────────────────────────────────────────────────────────
  Refer to the Engineering Operations Manual excerpt above.

  What is the operational procedure executed when critical systems
  are placed in the state identified in Question 2?

    PROCEDURE: [ ___________ ]

  ─────────────────────────────────────────────────────────────────────
  LOG THIS ANSWER TO THE RECONSTRUCTION BOARD.
```
