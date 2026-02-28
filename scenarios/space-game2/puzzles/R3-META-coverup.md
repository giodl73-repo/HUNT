# R3-META — COVER-UP
**Round:** CREW RECORD (Meta)
**Department:** COMMAND / SECURITY
**Panel:** Authorization Chain Analysis Console — Security Office, Deck 1
**Classification:** RESTRICTED — INCIDENT RECONSTRUCTION

---

## 1. Panel Overview

The five CREW RECORD puzzles each identified a person or action in the cover-up chain. Patient 3 is Kwon. Clock position 7 is Chen. Element 4 is Reeves. Badge position 5 is Vasquez. Procedure register 12 confirms the CO bypass. Five data points. Five threads.

The meta traces the authorization chain. Who ordered the cover-up? Who executed it? How many links connect them?

The five feeder values are crew duty-roster indices (with R3-05's value of 12 reduced modulo 8 to fit the roster). Each points to a crew member involved in the gap. The solver traces the chain from the ordering officer to the executing officer. The chain length — the number of distinct authorization links — is the step size for the commission decoder.

**Win condition:** The authorization chain displayed and verified. Chain length = 2 (Vasquez ordered, Reeves executed). The solver enters 2 into the NumericInput.

**Answer value:** 2 (authorization chain length = step size for the CyclicGroupDisplay)

---

## 2. Widget Configuration

### Primary Display — Feeder Value Summary with Roster Mapping

**Widget:** Authorization Chain Display (custom panel)
**Config:**
```
{
  display: "authorization-chain",
  feederValues: [
    { puzzle: "R3-01", label: "CRISIS PATIENT",     value: 3,  roster: "KWON",    role: "Incapacitated" },
    { puzzle: "R3-02", label: "ACCESS CODE RESULT",  value: 7,  roster: "CHEN",    role: "Physical access" },
    { puzzle: "R3-03", label: "AUTH TOKEN ELEMENT",   value: 4,  roster: "REEVES",  role: "Authorization" },
    { puzzle: "R3-04", label: "FIRST FORCED ENTRY",   value: 5,  roster: "VASQUEZ", role: "Command override" },
    { puzzle: "R3-05", label: "PROCEDURE REGISTER",   value: 12, roster: "REEVES*", role: "Execution",
      note: "12 mod 8 = 4 = REEVES (same as R3-03)" }
  ],
  chainVisualization: true,
  chainArrows: [
    { from: "VASQUEZ", to: "REEVES", label: "AUTHORIZED" },
    { from: "REEVES", to: "(execution)", label: "EXECUTED" }
  ],
  chainLength: 2
}
```

### Action-Authorization Matrix

**Widget:** `CayleyTableDisplay` variant — reference table
**Config:**
```
ACTION-AUTHORIZATION MATRIX
━━━━━━━━━━━━━━━━━━━━━━━━━━━

  FEEDER    │ VALUE │ MOD 8 │ CREW MEMBER   │ ROLE IN CHAIN
  ──────────┼───────┼───────┼───────────────┼──────────────────────
  R3-01     │   3   │   3   │ KWON (pos 3)  │ Neutralized (victim)
  R3-02     │   7   │   7   │ CHEN (pos 7)  │ Physical access agent
  R3-03     │   4   │   4   │ REEVES (pos 4)│ Authorization provider
  R3-04     │   5   │   5   │ VASQUEZ (pos 5)│ Ordering officer (CO)
  R3-05     │  12   │   4   │ REEVES (pos 4)│ Procedure executor

  DISTINCT CREW IN CHAIN:
    VASQUEZ — ordered the cover-up (CO authority, forced doors)
    REEVES  — provided authorization token + executed bypass
    CHEN    — used pre-authorized code for physical access
    KWON    — incapacitated (not an agent, a casualty)

  AUTHORIZATION CHAIN:
    Link 1:  VASQUEZ authorized herself (self-authorization via CO privilege)
    Link 2:  VASQUEZ authorized REEVES to execute the emergency procedure

    VASQUEZ ──[authorized]──> REEVES
            (ordering officer)   (executing officer)

    Chain length: 2 links
```

### Answer Input

**Widget:** `NumericInput`
**Config:**
```
{
  id: "chainLengthEntry",
  label: "AUTHORIZATION CHAIN LENGTH",
  min: 1,
  max: 8,
  decimalPlaces: 0,
  unit: "links",
  confirmButton: true,
  confirmLabel: "SET STEP SIZE",
  successMessage: "STEP SIZE ACCEPTED. STEP = 2.",
  failureMessage: "CHAIN LENGTH DOES NOT MATCH AUTHORIZATION ANALYSIS."
}
```

### Panel Layout

```
┌─────────────────────────────────────────────────────────────────────┐
│  SECURITY — AUTHORIZATION CHAIN ANALYSIS                            │
│  Terminal: SEC-AUTH-CHAIN-01    Location: SECURITY OFFICE, DECK 1   │
│  Data Source: Round 3 Feeder Values + Ship Authorization Protocol  │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  CREW RECORD VALUES:                                                │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  R3-01  Patient ID:          3  → KWON      (neutralized)  │   │
│  │  R3-02  Clock position:      7  → CHEN      (access agent) │   │
│  │  R3-03  Auth token element:  4  → REEVES    (authorized)   │   │
│  │  R3-04  First forced entry:  5  → VASQUEZ   (CO override)  │   │
│  │  R3-05  Procedure register: 12  → 12 mod 8 = 4 → REEVES   │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  AUTHORIZATION CHAIN:                                               │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │                                                              │   │
│  │   ┌──────────┐  LINK 1   ┌──────────┐  LINK 2              │   │
│  │   │ VASQUEZ  │──────────>│ REEVES   │──────────> (executed) │   │
│  │   │  pos 5   │ authorized│  pos 4   │ performed             │   │
│  │   │    CO    │           │   ENG    │ 12-op bypass          │   │
│  │   └──────────┘           └──────────┘                       │   │
│  │                                                              │   │
│  │   Chain length: _____ links                                  │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  CHAIN LENGTH:  [ ___ ] links    [ SET STEP SIZE ]                 │
│                                                                     │
│  ---- REFERENCE CARD (see below) ----                              │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 3. Reference Card

```
AUTHORIZATION CHAIN ANALYSIS — OPERATOR REFERENCE
═══════════════════════════════════════════════════════════════════

  WHAT THIS PANEL DOES
    Your five Round 3 values each identify a crew member involved
    in the gap. This panel traces the authorization chain — who
    ordered the cover-up and who executed it.

  STEP 1: MAP VALUES TO CREW
    Each feeder value is a duty roster index (0-7).
    For values > 7, reduce modulo 8 (subtract 8 until 0-7).

    R3-01:  3  → position 3 → KWON
    R3-02:  7  → position 7 → CHEN
    R3-03:  4  → position 4 → REEVES
    R3-04:  5  → position 5 → VASQUEZ
    R3-05: 12  → 12 mod 8 = 4 → position 4 → REEVES

  STEP 2: IDENTIFY ROLES
    Not all crew members in the chain played the same role.
    - KWON (3): incapacitated — a casualty, not an agent
    - CHEN (7): used a pre-authorized code — an access agent
    - REEVES (4): provided authorization + executed procedure
    - VASQUEZ (5): forced restricted doors — command authority

    REEVES appears TWICE (from R3-03 and R3-05).
    This means Reeves both provided the authorization token
    and executed the emergency bypass.

  STEP 3: TRACE THE CHAIN
    The authorization chain runs from the ORDERING officer to
    the EXECUTING officer.

    Who had command authority? VASQUEZ (the CO — only the CO
    can force restricted doors via Badge 401).

    Who executed the classified procedure? REEVES (the Chief
    Engineer — one of three people who knew the 12-op bypass).

    The chain:
      VASQUEZ (ordered) ──> REEVES (executed)
      Link 1: Vasquez authorized herself (CO self-authorization)
      Link 2: Vasquez authorized Reeves to perform the bypass

    Chain length = 2

  WHAT TO ENTER
    Enter the chain length (number of authorization links).
    This sets the step size on the commission decoder.

═══════════════════════════════════════════════════════════════════
  "The authorization chain from the ordering officer to the
   executing officer has how many links? Enter this as the
   step size."
═══════════════════════════════════════════════════════════════════
```

---

## 4. Novice Solve Path

1. **Read the feeder values.** The panel shows the five R3 answers with their roster mappings:
   - 3 = KWON, 7 = CHEN, 4 = REEVES, 5 = VASQUEZ, 12 mod 8 = 4 = REEVES

2. **Note the repetition.** Reeves appears twice (from R3-03 and R3-05). The reference card explains: Reeves both authorized and executed.

3. **Identify roles.** The reference card categorizes each person:
   - KWON was incapacitated (not an agent)
   - CHEN had physical access (an operative)
   - REEVES provided authorization and executed (double role)
   - VASQUEZ forced doors with CO authority (the commander)

4. **Trace the chain.** The reference card asks: who ordered? Who executed?
   - Ordering officer: VASQUEZ (the only person with command authority to force doors)
   - Executing officer: REEVES (the only person who knew the 12-operation bypass)

5. **Count the links.**
   - Link 1: Vasquez authorized herself (CO privilege — self-authorization)
   - Link 2: Vasquez authorized Reeves to perform the bypass

6. **Chain length = 2.** Enter 2. Press SET STEP SIZE.

**Time estimate:** 3-5 minutes. Mostly reading and understanding the chain structure.

---

## 5. Expert Solve Path

1. Scan the five values: 3, 7, 4, 5, 12. Map to roster: Kwon, Chen, Reeves, Vasquez, Reeves.

2. Vasquez is the CO (forced doors). Reeves is the executor (12-op bypass = CO bypass, Reeves is Chief Engineer). Chain: Vasquez -> Reeves = 2 links.

3. Enter 2.

**Time estimate:** 30-60 seconds.

---

## 6. Data Values

### Feeder Value to Roster Mapping

| Puzzle | Value | Mod 8 | Roster Position | Officer | Role |
|--------|-------|-------|-----------------|---------|------|
| R3-01 | 3 | 3 | 3 | KWON | Incapacitated (victim of suppressant) |
| R3-02 | 7 | 7 | 7 | CHEN | Physical access agent |
| R3-03 | 4 | 4 | 4 | REEVES | Authorization token provider |
| R3-04 | 5 | 5 | 5 | VASQUEZ | Ordering officer (CO override) |
| R3-05 | 12 | 4 | 4 | REEVES | Procedure executor (12-op bypass) |

### Distinct Crew Members

| Officer | Appears In | Chain Role |
|---------|-----------|-----------|
| KWON | R3-01 | Casualty (neutralized to prevent detection) |
| CHEN | R3-02 | Operative (physical access to Sensor Bay) |
| REEVES | R3-03, R3-05 | Intermediary (authorization + execution) |
| VASQUEZ | R3-04 | Principal (ordered and personally oversaw) |

### Authorization Chain

```
VASQUEZ (pos 5, CO)
  │
  ├── Link 1: Self-authorized via CO privilege
  │           (Badge 401 forced 3 restricted doors)
  │
  └── Link 2: Authorized REEVES (pos 4, Chief Engineer)
              (Reeves' token composed the OVERRIDE permission)
              (Reeves executed the 12-operation bypass)

Chain length: 2
```

### Chain Length Derivation

The chain counts AUTHORIZATION LINKS, not people:
- Link 1: The CO authorizing the operation (self-authorization — Vasquez decided to do this)
- Link 2: The CO delegating execution to Reeves (Reeves could not have acted without Vasquez's order)

Two links connect the decision to the execution. Not three (that would count each person). Not one (that would mean self-execution, but Vasquez needed Reeves for the engineering bypass).

### 80% Rule Verification

With 4 of 5 feeder values, the solver can still identify the chain:
- Remove R3-01 (Kwon): Still have Chen (access), Reeves (auth), Vasquez (override), Reeves (execution). Chain = 2.
- Remove R3-02 (Chen): Still have Kwon (casualty), Reeves (auth), Vasquez (override), Reeves (execution). Chain = 2.
- Remove R3-03 (Reeves auth): Still have Kwon, Chen, Vasquez (override), Reeves (execution). Chain = 2.
- Remove R3-04 (Vasquez): Lose the direct badge evidence, but R3-05 note says "only CO can use bypass." Chain = 2.
- Remove R3-05 (Reeves execution): Still have R3-03 (Reeves auth) + R3-04 (Vasquez override). Chain = 2.

All 5 removal cases still yield chain length 2. **80% rule satisfied.**

### Answer

**Chain length: 2** --> Step size = 2 for the CyclicGroupDisplay

---

## 7. Narrative Revelation

```
SEC-AUTH-CHAIN-01 — AUTHORIZATION CHAIN RECONSTRUCTION
TIMESTAMP: ROUND 3 COMPLETE

CREW RECORD ANALYSIS:
  R3-01  Patient 3 = KWON        Neutralized. V-tach from suppressant.
  R3-02  Position 7 = CHEN       Entered Sensor Bay. Pre-authorized code.
  R3-03  Element 4  = REEVES     Provided OVERRIDE authorization token.
  R3-04  Position 5 = VASQUEZ    Forced 3 restricted doors. Badge 401.
  R3-05  Register 12 = REEVES    Executed 12-operation CO confirmation bypass.

AUTHORIZATION CHAIN:
  VASQUEZ (Commanding Officer)
    ├── Self-authorized: CO privilege invoked at 3 restricted doors
    └── Authorized REEVES: token provided for OVERRIDE composition
        └── REEVES executed: 12-operation deflector bypass

  Chain: VASQUEZ ──> REEVES
  Length: 2 links

CHAIN ANALYSIS:
  Vasquez did not act alone. She could not. The emergency deflector
  reconfiguration required engineering knowledge (the 12-operation
  bypass sequence) that only three people possessed: the CO, the XO,
  and the Chief Engineer. The XO was off-ship. Vasquez needed Reeves.

  But Vasquez was the ordering authority. Every action in the chain
  traces back to her. Kwon was neutralized by her order (atmospheric
  suppressant). Chen used a code distributed under her protocol.
  Reeves composed the override with a token from her authorization
  cache. And Vasquez herself walked the corridor, forced the doors,
  and touched the hardware.

  Two links. Two people who made it happen.
  Vasquez gave the order. Reeves executed it.

STEP SIZE: 2
```

---

## 8. Story Layer

By the time the solver reaches the R3-META, they already know. The five puzzles of Round 3 laid out every thread: Kwon incapacitated, Chen at the sensor bay, Reeves' token in the override, Vasquez's badge at three restricted doors, the 12-operation bypass fingerprint. The meta does not reveal new information. It confirms what the instruments have already shown.

The authorization chain has exactly two links. Vasquez ordered. Reeves executed. Chen and Kwon were secondary — Chen was an operative following orders, Kwon was a casualty of the operation. The chain that matters is the one between the person who decided to meet the Phase-Locked Echo in secret and the person who reconfigured the ship's systems to make it possible.

The step size of 2 feeds the CyclicGroupDisplay. Starting at position 3 (Kwon, the first to detect), stepping by 2 (the chain length from ordering officer to executor), the pointer advances: 3 + 2 = 5. Position 5 on the duty circle. VASQUEZ.

The authorization chain, when traced through the commission decoder, points directly at the person who built the chain. The instruments do not lie. The chain does not break.
