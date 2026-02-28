# DEAD RECKONING — v2

**Complete redesign.** The original hunt used interference type classification as its puzzle mechanism — solvers named types. This version throws that out. The mechanism is now **operating equipment**. The solver achieves physical states. The physics is real. The instruments are real. The conclusion is demonstrated, not typed.

---

## Design Philosophy

### Controls-based, not social

This is a controls-based puzzle game. Story unfolds through instrument data, not dialogue or crew interaction. There are no conversations, no NPCs to question, no social deduction. The player is alone with the panels. The intrigue reveals itself through what the instruments *show* — a medical scanner registering cortisol spike at the exact moment of contact, a power reroute to an unauthorized compartment, a badge swipe that couldn't have happened if the person was where they claimed. The player pieces together the conspiracy through physical operation of equipment. The instruments don't lie. Someone wishes they did.

**Story beats are embedded in the data.** Not in flavor text, not in cutscenes. If there's narrative, the panel shows it.

### The Riven Standard, fully applied

The puzzle IS what the instrument does. Not a crossword overlaid on a COMMS console. The COMMS console IS the puzzle. A solver who understands carrier acquisition can solve R1-01. A solver who understands Lissajous figures can solve R1-02. The knowledge required is operator knowledge, not puzzle-hunt knowledge.

### No typed word answers

The answer to a puzzle is not a word you submit. It is a state you achieve:

- Phase lock acquired at 2.618 GHz
- Lissajous figure stabilizes to a tilted ellipse
- Correlation peak appears at τ = +0.34 ms
- FFT bin 7 exceeds noise floor by 23 dB
- Squelch breaks at threshold –94 dBm

The delivery site accepts the **control state** as the answer — a frequency, a time offset, a bin number, a dB value, a dial position. Something the instrument shows when the solver has done the work correctly.

### Real physics drives everything

Every instrument behaves according to real physics. The solver can reason from first principles. The world/ files document the physics, not a fictional taxonomy.

- Phase-locked loops lock when VCO frequency matches carrier
- Lissajous figures are stable when frequency ratio is rational
- Correlation peaks when signals are temporally aligned
- FFT reveals hidden periodic components
- Harmonics appear at integer multiples of fundamental

No invented rules. No fictional interference taxonomy. Real signal processing.

### The instruments

Six instruments, one per puzzle in Round 1. Each is a real piece of signal analysis equipment, described with enough physical fidelity that a solver can work it:

1. **Spectrum Analyzer** — center frequency, span, RBW, noise floor, peak hold
2. **Oscilloscope (X-Y mode / Lissajous)** — CH1/CH2 input, frequency ratio, phase difference
3. **Phase-Locked Loop Controller** — VCO frequency, loop bandwidth, lock indicator, phase error display
4. **Correlator** — reference signal select, lag sweep, correlation peak, τ display
5. **Vector Signal Analyzer** — IQ display, constellation diagram, modulation type, symbol rate
6. **Null Steering Array** — beam angle, null depth, interference direction, signal enhancement

---

## The Hunt

**Name:** DEAD RECKONING

**The hook:** You come on shift to find your captain has been relieved of command. Official reason: navigational incident. Six hours of ship logs are gone — deleted. A sealed order is moving through the chain that will wipe the backup instrument logs in 72 hours. The only record left. You have 72 hours to reconstruct what happened before the truth disappears. Not as an investigation. As a matter of survival — because whatever the ship encountered during those six hours is still out there.

**The story the instruments tell:**
- Round 1 (TELEMETRY): The signals from outside are too deliberate. Someone was waiting for you.
- Round 2 (SYSTEMS LOG): The ship's response was calm, coordinated, and *prepared*. Someone on this ship knew the encounter was coming.
- Round 3 (CREW RECORD): The cover-up was ordered before the encounter ended. Whoever deleted the logs didn't need to review them. They already knew what was in them.

**The revelation:** The encounter was a rendezvous. Not an attack. A meeting. The senior officer who ordered the cover-up is still on this ship, still in command, still waiting for the thing they met to come back.

**Tone:** Tension beneath calm professionalism. You are a competent officer doing methodical work. Each panel you successfully read tightens the knot. The instruments don't lie. Someone wishes they did.

**Story delivery:** Through the data only. No dialogue, no NPC interaction, no social deduction. The medical scanner shows a cortisol spike at the moment of contact. The EPS log shows an unauthorized power reroute to a compartment that should be empty. The badge log shows a swipe that couldn't have happened. The player feels the intrigue because the instruments show it.

**Rounds:**

```
Round 1: TELEMETRY      — 6 puzzles, 1 per instrument
Round 2: SYSTEMS LOG    — 6 puzzles, ship-internal instruments
Round 3: CREW RECORD    — 5 puzzles, personnel/biological data
Meta:    THE COMMISSION — what happened during the gap
```

---

## Round 1 Puzzle Design — TELEMETRY

Each puzzle presents a complete instrument panel: all controls, all displays, all reference data. The solver manipulates the controls (in text: reasons through what each setting does) until a target condition is achieved. The condition, once achieved, reveals a number or value that feeds the meta.

### R1-01 — CARRIER ACQUISITION (Spectrum Analyzer)

**The instrument:** A spectrum analyzer with adjustable center frequency (1.0–5.0 GHz, 10 MHz steps), span (10/50/100/500 MHz), resolution bandwidth (1/10/100 kHz), and peak hold.

**The situation:** The analyzer was running during the gap. Its log shows 6 frequency sweeps captured automatically at 7-minute intervals. The center frequency drifted between sweeps — someone was tuning. Each sweep shows a different spectrum. One sweep shows a clean carrier embedded in what looks like noise. The others show only interference.

**The work:** The solver reasons through which sweep contains the real carrier (it has specific properties: narrow bandwidth, stable amplitude, EM-only footprint). Then determines what center frequency and span settings would have been required to capture it cleanly. Those settings ARE the answer.

**Physics the solver uses:**
- Real carriers have narrow bandwidth relative to noise
- Spurious signals appear at harmonics of the fundamental
- SNR improves as RBW narrows (for carriers) and worsens for noise
- A real carrier survives peak hold across multiple sweeps; noise does not

**The conclusion:** Solver dials to the correct center frequency and span. The display shows the carrier cleanly. The frequency readout IS the answer value.

---

### R1-02 — PHASE LOCK (Oscilloscope, X-Y mode)

**The instrument:** A dual-channel oscilloscope in X-Y mode. CH1 is the ship's own outbound signal (known, stable). CH2 is the incoming mystery signal. The display shows a Lissajous figure.

**The situation:** The Lissajous figure is unstable — a rotating, drifting spiral. The mystery signal's frequency is close to the outbound but not identical. The solver must determine the exact frequency ratio that would produce a stable figure, then identify what that ratio means.

**The work:**
- Stable Lissajous figures occur when f_CH2/f_CH1 = p/q (small integers)
- The figure's topology reveals the ratio: loops crossing the vertical axis = f_CH1, loops crossing horizontal = f_CH2
- The current drifting figure has a characteristic drift rate that reveals the frequency offset
- Solver calculates the exact f_CH2 at lock, compares to the known f_CH1

**Physics the solver uses:**
- Lissajous frequency ratios from figure shape
- Drift rate = beat frequency = |f1 - f2|
- Phase at lock is stable; phase difference encodes information

**The conclusion:** The frequency ratio at lock (e.g., 3:2) and the phase tilt at lock (e.g., 45°) together encode the answer value.

---

### R1-03 — CORRELATION PEAK (Correlator)

**The instrument:** A cross-correlator. Reference signal = ship's outbound pulse train (known timing). Input = recovered signal from the gap. Output = correlation function vs. lag τ.

**The situation:** The correlation output shows multiple peaks — the mystery signal is a composite. Some peaks are at positive τ (echoes), some at negative τ (precursors — physically impossible unless the signal was anticipated). One peak stands out by its height.

**The work:**
- Real echoes have positive τ (arrive after the outbound)
- Negative τ peaks indicate the signal was transmitted BEFORE the ship's outbound — it was not a response
- The solver identifies which peaks are physically plausible and which are not
- The τ value of the dominant real peak encodes the distance to the signal source (τ = 2d/c)

**Physics:** τ = 2d/c, c = 3×10⁸ m/s, distance encodes the answer.

---

### R1-04 — NULLING (Null Steering Array)

**The instrument:** A phased array antenna with adjustable beam angle (0–360°, 1° resolution), null depth control, and signal strength meter.

**The situation:** Three signals are arriving simultaneously. The ship's array log shows 8 steering attempts during the gap — someone was trying to null out the interference to isolate a specific signal. The log shows beam angle, null depth setting, and the resulting SNR improvement for each attempt.

**The work:**
- Nulling an interference source maximizes SNR for the desired signal
- The optimal null angle points directly at the interference source
- The solver determines the interference source direction from the 8 steering attempts
- The target signal's bearing (the non-nulled direction that produced maximum SNR) is the answer

**The conclusion:** Target signal bearing in degrees = the answer value.

---

### R1-05 — FFT (Fast Fourier Transform)

**The instrument:** An FFT analyzer. 1024-point FFT, configurable window function (rectangular, Hanning, Blackman), configurable sample rate (1–20 MHz). Input: a raw time-domain signal recorded during the gap.

**The situation:** The raw signal looks like noise. But the FFT reveals hidden periodic structure — a signal encoded in the frequency domain. The solver must configure the FFT correctly (window function, sample rate) to see the structure clearly, then read what the frequency bins spell.

**The work:**
- Window function selection affects spectral leakage — Hanning is good for sinusoids, rectangular for pulses
- Sample rate determines frequency resolution (Δf = fs/N)
- The hidden signal is at a specific bin number
- Bin number × frequency resolution = signal frequency → encodes the answer

**The conclusion:** Correct FFT configuration reveals the hidden carrier frequency.

---

### R1-06 — VECTOR ANALYSIS (Vector Signal Analyzer)

**The instrument:** A vector signal analyzer showing an IQ constellation diagram. The incoming signal's constellation is distorted — points are scattered, rotated, compressed. The solver must determine the modulation type and symbol rate from the constellation.

**The situation:** The mystery signal used a specific digital modulation scheme (BPSK, QPSK, 8-PSK, 16-QAM). The constellation, once properly derotated and scaled, reveals the modulation type. The symbol rate, combined with the modulation type, reveals the data rate — and the data rate is the answer.

**Physics:** Constellation geometry identifies modulation type. Scattering indicates EVM. Rotation indicates phase offset. Symbol rate × bits/symbol = data rate.

---

## Meta Architecture

**Round 1 meta:** The 6 answer values (frequencies, bearing, data rate, etc.) are physical measurements that together describe a specific contact. The meta asks: given these measurements, what is the contact? The measurements triangulate to a specific object type (a vessel class, a beacon type, a natural phenomenon).

**Round 2 meta:** Ship-internal instruments show the ship's response during the gap. The meta reconstructs what orders were given.

**Round 3 meta:** Personnel records show who was on duty, who accessed what systems, and what happened to them. The meta identifies who ordered the cover-up.

**Final meta:** Combines all three to reconstruct the incident — what the ship encountered, how it responded, and who decided to delete the record.

---

## Delivery

The puzzles are HTML pages. Each instrument is rendered as a realistic panel:
- Knobs, sliders, frequency dials implemented as HTML/CSS/JS
- Display area shows the instrument output for the current control settings
- The solver can literally turn the knobs
- When they reach the target state, the display changes and the answer value appears

Alternatively (simpler for v1): puzzles are detailed text documents describing the instrument and its current readings at multiple control settings. The solver reasons through which settings achieve the target state without needing interactive controls. The answer is what the display would show at those settings.

---

## What This Tests

- Whether the toolkit can support instruments-as-puzzles (no word extraction)
- Whether real physics as puzzle mechanics holds up under blind testing
- Whether the delivery UX (interactive controls vs. text description) affects solvability
- Whether non-word answers (frequencies, bearings, data rates) work with the meta architecture
