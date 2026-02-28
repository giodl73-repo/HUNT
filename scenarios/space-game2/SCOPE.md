# DEAD RECKONING — Scope

## Identity

**Name:** DEAD RECKONING
**Genre:** Hard science fiction — starship incident reconstruction via instrument operation
**Tone:** Technical. Terse. The instruments speak. You listen.
**Scale:** 17 puzzles (6 + 6 + 5 feeders) + 3 round metas + 1 final meta
**Audience:** Puzzle enthusiasts who enjoy simulation and technical puzzles; no prior physics or signal-processing knowledge required, but physics literacy rewarded
**Format:** Interactive HTML instrument panels (knobs, dials, sliders that work) + detailed text fallback for non-interactive solvers
**Duration:** 8-12 hours (full team), 15-20 hours (solo)

---

## The Frame

A starship. Six hours of missing telemetry. Someone deleted the main log. You are a new officer assigned to the ship's inquiry commission. Your job: reconstruct what happened using the surviving instrument data. The instruments were still recording — the main log was the only thing wiped. The data is there. You have to read it.

You are not told what happened. You are not told what to look for. The instruments present evidence. You operate them until you understand what each one recorded. The answers are not words you type — they are physical states you achieve: a frequency you tune to, a bearing you determine, a correlation delay you measure, a data rate you calculate.

---

## The Mechanism

**Real instrument operation.** Each puzzle presents a complete instrument panel with controls and displays. The solver manipulates the controls (adjusts parameters, selects settings, interprets readouts) until a target physical condition is achieved. The condition, once achieved, reveals a numeric value that feeds the round meta.

This is the Riven Standard fully applied. The puzzle IS what the instrument does. A solver who understands how a spectrum analyzer works can solve R1-01. A solver who understands Lissajous figures can solve R1-02. The knowledge required is operator knowledge, not puzzle-hunt knowledge.

**No invented physics.** Every instrument behaves according to real physics. Phase-locked loops lock when VCO frequency matches carrier. Lissajous figures stabilize when the frequency ratio is rational. Correlation peaks when signals are temporally aligned. FFT reveals hidden periodic components. The world/ files document real signal processing — not a fictional taxonomy.

---

## Answer Format

Answers are numeric values — not words.

- Frequencies in GHz (e.g., 2.618)
- Bearings in degrees (e.g., 247)
- Time delays in milliseconds (e.g., 0.34)
- Data rates in kbps (e.g., 128)
- Bin numbers (e.g., 7)
- Phase angles in degrees (e.g., 45)
- Power levels in dBm (e.g., -94)

The delivery site accepts the control state as the answer. When the solver has configured the instrument correctly, the display shows the answer value. The meta architecture combines these numeric values to reconstruct the incident.

---

## Rounds

```
Round 1: TELEMETRY      — 6 puzzles  (external signals, 6 instruments)
Round 2: SYSTEMS LOG    — 6 puzzles  (internal ship instruments)
Round 3: CREW RECORD    — 5 puzzles  (personnel/biological data)

Round 1 Meta: CONTACT       — what was out there?
Round 2 Meta: RESPONSE      — what did the ship do?
Round 3 Meta: COVER-UP      — who ordered the deletion?
Final Meta:   THE COMMISSION — the full incident reconstruction
```

**Round 1 — TELEMETRY:** Six signal-processing instruments, each a real piece of equipment. The solver operates each one to characterize a different aspect of the mystery signal detected during the gap. Answer values are physical measurements that together describe a specific contact.

**Round 2 — SYSTEMS LOG:** Six ship-internal instruments. Power grid logs, navigation data, computer systems, propulsion telemetry. Each puzzle uses the same mechanism — operate an instrument readout, achieve a target state — but now the instruments are shipboard systems rather than signal analysis tools.

**Round 3 — CREW RECORD:** Five puzzles using personnel and biological data. Medical biometrics, access logs, duty rosters, comm records. These instruments read people, not signals. The mechanism is the same: operate the readout until the evidence resolves.

---

## Meta Architecture

**Round 1 Meta (CONTACT):** The 6 answer values from Round 1 are physical measurements (frequency, bearing, data rate, phase, delay, bin number) that together characterize a specific contact. Given these measurements, what is the contact? The measurements triangulate to a specific object type.

**Round 2 Meta (RESPONSE):** The 6 answer values from Round 2 are ship system states (power draw, course heading, engine output, etc.) that together describe the ship's response. Given these states, what orders were given?

**Round 3 Meta (COVER-UP):** The 5 answer values from Round 3 are personnel data points (biometric readings, access timestamps, comm durations) that together identify who ordered the cover-up.

**Final Meta (THE COMMISSION):** Combines all three round-meta conclusions — what the ship encountered, how it responded, and who decided to delete the record — to reconstruct the full incident.

---

## Delivery

**Primary:** Interactive HTML pages. Each instrument is rendered as a realistic control panel:
- Knobs, sliders, frequency dials implemented in HTML/CSS/JS
- Display area shows instrument output for current control settings
- Solver literally turns the knobs
- When target state is achieved, the display changes and the answer value appears

**Fallback:** Detailed text documents describing the instrument and its readings at multiple control settings. Solver reasons through which settings achieve the target state. The answer is what the display would show at those settings.

---

## Narrator Voice

The instruments speak. Terse. No flavor text. Evidence only.

- No exclamation marks.
- No questions.
- Short sentences. Rarely more than 10 words.
- Data, then silence.
- Readout labels. Status lines. Log entries.
- The solver is "you" only in commission briefings. In instrument panels, there is no "you" — only the display.

Example instrument voice:
> CENTER FREQ: 2.400 GHz
> SPAN: 100 MHz
> RBW: 10 kHz
> SWEEP 3 OF 6
> PEAK HOLD: ON
> CARRIER DETECTED: —

Example commission briefing voice:
> The main telemetry log was deleted at 0347 UTC. Backup instruments continued recording. You have been assigned to reconstruct the gap. Begin with the spectrum analyzer.

---

## Physics Systems

Six core physics domains, each grounding one Round 1 instrument and potentially referenced by later rounds:

1. **Spectrum Analyzer** — center frequency, span, RBW, noise floor, peak hold, carrier identification
2. **Oscilloscope (Lissajous)** — X-Y mode, frequency ratios, phase determination, drift rate
3. **Correlator** — cross-correlation, lag sweep, time delay, distance calculation
4. **Phased Array (Null Steering)** — beam angle, null placement, interference rejection, bearing determination
5. **FFT Analyzer** — frequency resolution, window functions, bin identification, hidden signal detection
6. **Vector Signal Analyzer** — IQ constellation, modulation identification, symbol rate, data rate

All physics is real. No invented rules. Reference data locked in `world/systems/`.

---

## Review Gate

This scope defines a hunt that:
- Uses real physics as the puzzle mechanism (Riven Standard — the puzzle IS the instrument)
- Requires operator knowledge, not puzzle-hunt knowledge (Solving = Proving Understanding)
- Returns numeric answers, not word answers (novel meta architecture)
- Supports two delivery modes (interactive HTML and text fallback)
- Can be solved without prior signal-processing training (all reference data provided)
- Has three escalating rounds: external signals, internal systems, personnel data
- Reconstructs a coherent narrative through physical evidence
