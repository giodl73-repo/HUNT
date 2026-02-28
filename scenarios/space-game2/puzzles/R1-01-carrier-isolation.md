# R1-01 — Carrier Isolation

## Panel Overview

| Field | Value |
|-------|-------|
| **Puzzle ID** | R1-01 |
| **Title** | Carrier Isolation |
| **Round** | 1 — TELEMETRY |
| **Department** | COMMS |
| **Instrument** | Bandpass filter + waveform display |
| **Answer** | 2.340 GHz |

**Narrative beat:** The raw signal buffer from the gap still contains the combined incoming waveform. Three overlapping signal components are present: the target carrier, a broadband noise component, and a harmonic artifact. Someone at this console isolated the carrier during the gap, but the filter log was deleted. The solver redoes that work.

**Win condition:** The filtered waveform aligns exactly with the target carrier overlay. The NumericStepper reads 2.340 GHz.

---

## Widget Configuration

### Display

```
Widget: SineWaveDisplay
Config:
  - mode: COMBINED (raw composite of 3 signal components)
  - component_1: carrier at 2.340 GHz, amplitude 0.8, phase 0
  - component_2: broadband noise, center 1.750 GHz, bandwidth 600 MHz, amplitude 0.4
  - component_3: harmonic artifact at 4.680 GHz (2x carrier), amplitude 0.3
  - targetOverlay: true
  - targetFrequency: 2.340 GHz (hidden value — overlay appears as a faint reference wave)
  - targetColor: #FFD700 (gold)
  - signalColor: #00FF88 (green)
  - gridColor: #1A3A1A
  - scrollSpeed: 2
  - showMatchIndicator: true (glows when filtered signal aligns with target)
  - matchTolerance: 0.001 GHz (within 1 MHz triggers visual alignment)
```

### Controls

```
Widget: NumericStepper
ID: filterCenterFrequency
Config:
  - label: "FILTER CENTER"
  - min: 1.000 GHz
  - max: 5.000 GHz
  - step: (driven by RotaryDial)
  - default: 3.000 GHz
  - decimalPlaces: 3
  - unit: "GHz"
```

```
Widget: RotaryDial
ID: stepSizeSelector
Config:
  - label: "STEP SIZE"
  - positions: 3
  - positionLabels: ["100 MHz", "10 MHz", "1 MHz"]
  - positionValues: [0.100, 0.010, 0.001]
  - default: position 0 (100 MHz)
  - knobColor: #667788
  - pointerColor: #FFFFFF
```

```
Widget: LCARSButton
ID: applyFilter
Config:
  - label: "APPLY FILTER"
  - mode: momentary
  - faceColor: #CC6600
  - glowColor: #FF9933
  - action: redraws SineWaveDisplay showing only signal content within
            a narrow bandpass (50 MHz wide) centered on the NumericStepper value;
            all signal content outside this passband is suppressed
```

### Layout

```
+--------------------------------------------------+
|  [ COMMS — SIGNAL ISOLATION ]                    |
|                                                  |
|  +--------------------------------------------+ |
|  |                                            | |
|  |         SineWaveDisplay                    | |
|  |    (green: filtered signal)                | |
|  |    (gold: target carrier overlay)          | |
|  |                                            | |
|  +--------------------------------------------+ |
|                                                  |
|  STEP SIZE         FILTER CENTER                 |
|  ( o )            [< ] 3.000 GHz [> ]           |
|  100/10/1 MHz                                    |
|                                                  |
|             [ APPLY FILTER ]                     |
|                                                  |
|  ---- REFERENCE CARD (see below) ----           |
+--------------------------------------------------+
```

---

## Reference Card (Panel Legend)

Printed on the bezel beneath the display. Readable at all times.

```
COMMS — SIGNAL ISOLATION REFERENCE

WHAT THE DISPLAY SHOWS
  The green waveform is the filtered incoming signal.
  The gold waveform is the target carrier reference.
  When both waveforms match in frequency and shape,
  the carrier has been isolated.

CONTROLS
  STEP SIZE (rotary dial): Sets how much the filter
    moves per click. 100 MHz for coarse sweep,
    10 MHz for approach, 1 MHz for lock.
  FILTER CENTER (stepper): The center frequency of
    the bandpass filter. Click arrows to adjust.
  APPLY FILTER (button): Redraws the display with
    the current filter setting applied.

HOW TO ISOLATE A CARRIER
  1. Start with STEP SIZE on 100 MHz. Click through
     the frequency range. Watch the green waveform.
  2. When the green waveform begins to look like a
     clean sine wave (not jagged noise), you are near
     the carrier. Switch STEP SIZE to 10 MHz.
  3. Narrow further. When the green waveform nearly
     matches the gold overlay, switch to 1 MHz.
  4. At the exact carrier frequency, the green wave
     locks onto the gold reference. Both waveforms
     align perfectly.

KEY PHYSICS FACT
  A carrier is a single-frequency signal. A bandpass
  filter passes only frequencies near its center.
  When the filter center matches the carrier, the
  carrier passes through cleanly. Noise and
  interference at other frequencies are rejected.
  The closer the filter center to the carrier, the
  cleaner the output.
```

---

## Novice Solve Path

1. Read the reference card. Understand: the green wave is filtered output, the gold wave is the target. The goal is to make them match.
2. RotaryDial is on 100 MHz. Click the NumericStepper right arrow. Each click moves the filter center by 100 MHz. Press APPLY FILTER after each change.
3. At 1.000 GHz: the green waveform is jagged noise. No match.
4. At 2.000 GHz: the green waveform starts to show a sinusoidal shape emerging from noise. Getting closer.
5. At 2.300 GHz: the green waveform is clearly sinusoidal, similar in frequency to the gold overlay but not aligned. Switch RotaryDial to 10 MHz.
6. At 2.340 GHz: very close match. Switch RotaryDial to 1 MHz for fine adjustment.
7. At 2.340 GHz (already there from the 10 MHz step): the green waveform locks onto the gold overlay. Both waves align in frequency, amplitude, and phase. The match indicator glows.
8. Read the NumericStepper: 2.340 GHz. This is the carrier frequency.

**Time estimate:** 3-5 minutes. Approximately 25-30 stepper clicks total.

---

## Expert Solve Path

1. Recognize: this is a bandpass filter tuning exercise. The target overlay is the carrier. Need to find its frequency.
2. Start at 100 MHz steps. S-band is 2-4 GHz. Jump to 2.000 GHz in 10 clicks.
3. Waveform cleaning up. Switch to 10 MHz. Three clicks to 2.300. One more to 2.340. Waveform locks.
4. Confirm at 1 MHz if desired — already at the answer.
5. Read: 2.340 GHz. Done.

**Time estimate:** 30-60 seconds.

---

## Data Values

| Parameter | Value | Source/Justification |
|-----------|-------|---------------------|
| Carrier frequency | 2.340 GHz | META-DESIGN.md R1-01 answer |
| Carrier amplitude | 0.8 (normalized) | Dominant component in the composite |
| Noise center | 1.750 GHz | Placed in L/S-band transition to be clearly distinct from carrier |
| Noise bandwidth | 600 MHz | Wide enough to overlap carrier frequency, creating the filtering challenge |
| Harmonic artifact frequency | 4.680 GHz | Exactly 2x carrier (second harmonic), consistent with spectrum analyzer physics |
| Harmonic amplitude | 0.3 | Harmonics are 8-9 dB below fundamental (within 10-30 dB range per instrument reference) |
| Filter passband width | 50 MHz | Narrow enough to reject noise and harmonic when centered correctly |
| Starting frequency | 3.000 GHz | Mid-range default, forces solver to search in both directions |
| Frequency range | 1.000-5.000 GHz | Matches spectrum analyzer instrument reference |

---

## Narrative Revelation

```
[COMMS] GAP+00:23:11 — Carrier isolated at 2.340 GHz. S-band.
Source: unregistered. Signal structure: coherent, narrow-bandwidth.
Classification: DELIBERATE TRANSMISSION.
```

---

## Story Layer

The carrier at 2.340 GHz is in the S-band (2-4 GHz) — the same band used by military transponders and phase-locked echoes per the Contact Signature Reference Table. The frequency is offset from any standard communication channel center. Someone was transmitting on a non-standard frequency — deliberately avoiding the ship's routine monitoring bands. The harmonic at 4.680 GHz confirms a clean, engineered transmitter, not a natural emission. The broadband noise component is consistent with deliberate jamming — an attempt to mask the carrier in the background. The original operator at this console knew exactly where to tune because they knew the carrier was there. The solver has just proven that the signal was not accidental.
