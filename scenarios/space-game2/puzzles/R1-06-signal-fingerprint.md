# R1-06 — Signal Fingerprint

## Panel Overview

| Field | Value |
|-------|-------|
| **Puzzle ID** | R1-06 |
| **Title** | Signal Fingerprint |
| **Round** | 1 — TELEMETRY |
| **Department** | COMMS |
| **Instrument** | Vector signal analyzer (IQ constellation + waveform) |
| **Answer** | 384 kbps |

**Narrative beat:** The contact was transmitting structured data — not a simple beacon tone. The raw IQ recording from the gap survived in the signal buffer. The solver must determine the modulation scheme and symbol rate of the transmission to calculate the data rate. The data rate reveals how much information was being exchanged — and 384 kbps is far too high for a navigation ping or distress call. This was a conversation.

**Win condition:** The RotaryDial is set to 8-PSK, the NumericStepper reads 128 kSym/s. The CommSignalDisplay shows a clean 8-point constellation with EVM < 15% and the data rate readout shows 384 kbps.

---

## Widget Configuration

### Display

```
Widget: CommSignalDisplay
Config:
  - mode: CONSTELLATION (IQ diagram, primary display)
  - size: 400 x 400 px
  - backgroundColor: #000011
  - gridColor: #111133
  - iAxisLabel: "I (In-Phase)"
  - qAxisLabel: "Q (Quadrature)"
  - symbolDotColor: #00AAFF (cyan-blue for received symbols)
  - symbolDotRadius: 3
  - symbolCount: 200 (number of received symbol dots shown)
  - idealOverlay: true (shows ideal constellation positions when
    modulation type is selected)
  - idealOverlayColor: #FFCC00 (gold circles at ideal positions)
  - idealOverlayRadius: 8 (target circles)
  - signalData:
    - trueModulation: 8-PSK
    - trueSymbolRate: 128 kSym/s
    - phaseOffset: 22 degrees (constellation rotated from standard)
    - noiseLevel: EVM ~12% (moderate scatter around ideal positions)
  - evmReadout: top-right corner, shows "EVM: XX.X%"
    - When correct modulation selected: ~12% (clusters around ideal)
    - When wrong modulation selected: > 40% (clusters don't fit)
  - waveformOverlay: small inset (100 x 80 px, bottom-right)
    - Shows symbol transitions
    - Clean transitions when symbol rate is correct
    - Blurred/smeared when symbol rate is wrong
  - dataRateReadout: bottom-center, large font
    - Shows "DATA RATE: --- kbps" until both modulation and symbol
      rate are correct
    - Shows "DATA RATE: 384 kbps" when both are correct
    - Calculated as: symbolRate * bitsPerSymbol
```

### Controls

```
Widget: RotaryDial
ID: modulationType
Config:
  - label: "MODULATION TYPE"
  - positions: 4
  - positionLabels: ["BPSK", "QPSK", "8-PSK", "16-QAM"]
  - positionValues: ["BPSK", "QPSK", "8PSK", "16QAM"]
  - default: position 0 (BPSK)
  - knobColor: #667788
  - pointerColor: #FFFFFF
  - behavior: selecting a modulation type overlays the ideal
    constellation points for that scheme. The CommSignalDisplay
    redraws with the ideal overlay.
```

```
Widget: NumericStepper
ID: symbolRate
Config:
  - label: "SYMBOL RATE"
  - min: 1 kSym/s
  - max: 1000 kSym/s
  - step: 1 kSym/s
  - default: 100 kSym/s
  - decimalPlaces: 0
  - unit: "kSym/s"
```

### Layout

```
+--------------------------------------------------+
|  [ COMMS — SIGNAL ANALYSIS ]                     |
|                                                  |
|  +--------------------------------------------+ |
|  |                              EVM: 45.2%    | |
|  |                                            | |
|  |      CommSignalDisplay                     | |
|  |      (cyan dots: received symbols)         | |
|  |      (gold circles: ideal positions)       | |
|  |                                            | |
|  |      Q                                     | |
|  |      |       *  *                          | |
|  |      |    *        *         +--------+   | |
|  |      |  *    (center)   *    | wave-  |   | |
|  |   ---+----*----------*----   | form   |   | |
|  |      |  *            *       +--------+   | |
|  |      |    *        *                      | |
|  |      |       *  *                         | |
|  |      +---------- I                        | |
|  |                                            | |
|  |           DATA RATE: --- kbps             | |
|  +--------------------------------------------+ |
|                                                  |
|  MODULATION TYPE           SYMBOL RATE           |
|  ( o )                    [< ] 100 kSym/s [> ]   |
|  BPSK/QPSK/8PSK/16QAM                           |
|                                                  |
|  ---- REFERENCE CARD (see below) ----           |
+--------------------------------------------------+
```

---

## Reference Card (Panel Legend)

```
COMMS — SIGNAL ANALYSIS REFERENCE

WHAT THE DISPLAY SHOWS
  The IQ constellation diagram plots each received
  data symbol as a dot. The pattern of dots reveals
  the modulation type used by the transmitter.

  Gold circles mark the IDEAL symbol positions for
  the currently selected modulation type. When the
  correct type is selected, the cyan dots cluster
  tightly around the gold circles.

MODULATION TYPES — HOW TO IDENTIFY
  Count the distinct clusters of cyan dots:
    2 clusters on a line    = BPSK  (1 bit/symbol)
    4 clusters in a square  = QPSK  (2 bits/symbol)
    8 clusters in a ring    = 8-PSK (3 bits/symbol)
    16 clusters in a grid   = 16-QAM (4 bits/symbol)

  If the clusters are ROTATED from standard
  positions, the signal has a phase offset. This
  does not change the cluster count.

EVM (ERROR VECTOR MAGNITUDE)
  Shown in the top-right corner. Measures how well
  the received dots match the ideal positions.
    < 15%   Good match — correct modulation type
    > 30%   Poor match — wrong modulation type

  Try each modulation type. The one with the
  lowest EVM is correct.

SYMBOL RATE
  Adjust the stepper. When the correct rate is set,
  the waveform inset (bottom-right) shows clean,
  sharp symbol transitions. When incorrect, the
  transitions are blurred.

DATA RATE FORMULA
  Data rate = symbol rate x bits per symbol
  The readout calculates this automatically when
  both modulation type and symbol rate are correct.

HOW TO SOLVE
  1. Count dot clusters. Select the matching
     modulation type on the rotary dial.
  2. Check EVM — it should drop below 15%.
  3. Adjust SYMBOL RATE until the waveform inset
     shows clean transitions.
  4. Read the DATA RATE from the bottom readout.

KEY PHYSICS FACT
  Digital signals encode data by placing symbols at
  specific points in the IQ plane. More points =
  more bits per symbol = higher data rate for the
  same symbol rate. The constellation geometry is
  the signal's fingerprint.
```

---

## Novice Solve Path

1. Read the reference card. Understand: count clusters to identify modulation, then find the symbol rate, then read the data rate.
2. The display shows ~200 cyan dots. Look at the pattern. They form a ring of clusters. Count: there are 8 distinct cluster regions arranged in a circle. (The clusters are rotated 22 degrees from standard 8-PSK positions due to phase offset, but there are clearly 8 of them.)
3. The reference card says 8 clusters in a ring = 8-PSK (3 bits/symbol). Turn the RotaryDial to 8-PSK.
4. Gold circles appear at the 8 ideal 8-PSK positions. The cyan dots cluster around them — not perfectly (due to the 22-degree rotation and noise), but clearly grouped near 8 points. EVM drops to ~12%. Much better than the >40% shown with BPSK.
5. Optionally test other settings: BPSK shows 2 gold circles — dots don't cluster around them (EVM > 40%). QPSK shows 4 circles — some dots are near them but many aren't (EVM ~35%). 16-QAM shows a 4x4 grid — dots don't fit the grid pattern at all (EVM > 50%). 8-PSK is the clear winner.
6. Now adjust symbol rate. Default is 100 kSym/s. The waveform inset looks slightly blurred. Click upward: 110, 120, 125, 128. At 128, the waveform transitions sharpen noticeably. Click to 129, 130 — transitions blur again. Back to 128.
7. The data rate readout updates: "DATA RATE: 384 kbps" (128 x 3 = 384).
8. Read: 384 kbps.

**Time estimate:** 3-5 minutes. Modulation identification is 30 seconds (visual cluster counting). Symbol rate tuning is ~28 clicks from 100 to 128.

---

## Expert Solve Path

1. Glance at the constellation: 8 clusters in a ring, rotated. Immediately recognize 8-PSK. Set RotaryDial to 8-PSK. EVM confirms: ~12%.
2. 8-PSK = 3 bits/symbol. Need symbol rate.
3. Start at 100 kSym/s. The waveform is slightly soft. Click up by larger jumps — 110, 120, 128. Sharpens at 128. Verify: 129 is worse. Lock at 128.
4. 128 x 3 = 384 kbps. Data rate readout confirms. Done.

**Time estimate:** 30-60 seconds.

---

## Data Values

| Parameter | Value | Source/Justification |
|-----------|-------|---------------------|
| Modulation type | 8-PSK | Per PUZZLES.md brief. 8 constellation points, 3 bits/symbol. |
| Symbol rate | 128 kSym/s | Chosen so that 128 x 3 = 384 kbps (META-DESIGN.md R1-06 answer) |
| Data rate | 384 kbps | META-DESIGN.md R1-06 answer |
| Bits per symbol | 3 | 8-PSK: log2(8) = 3 |
| Phase offset | 22 degrees | Constellation rotated from standard orientation. Does not affect cluster count. Adds visual complexity for novice. |
| EVM at correct modulation | ~12% | "Good" quality per VSA instrument reference (5-15% range) |
| EVM at BPSK | > 40% | Poor — clearly wrong |
| EVM at QPSK | ~35% | Poor — some coincidental overlap but mostly wrong |
| EVM at 16-QAM | > 50% | Very poor — grid pattern does not fit ring arrangement |
| Visible symbol dots | 200 | Enough to show clear clustering pattern |
| Default symbol rate | 100 kSym/s | Close enough that solver reaches 128 quickly |
| Symbol rate range | 1-1000 kSym/s | Wide range per instrument reference |

### EVM Response Table

| Modulation setting | EVM (%) | Visual assessment |
|-------------------|---------|-------------------|
| BPSK | 47 | Dots scattered everywhere relative to 2 ideal points |
| QPSK | 35 | Some dots near 4 of the 8 clusters; rest orphaned |
| **8-PSK** | **12** | **All dots cluster tightly around 8 ideal positions** |
| 16-QAM | 53 | Ring pattern does not fit rectangular grid at all |

---

## Narrative Revelation

```
[COMMS] GAP+02:04:33 — Signal demodulated. Modulation: 8-PSK.
Symbol rate: 128 kSym/s. Data rate: 384 kbps. Content: structured,
encrypted, bidirectional. Classification: ACTIVE DATA EXCHANGE.
```

---

## Story Layer

384 kbps using 8-PSK modulation. This is not a beacon (beacons use BPSK at < 1 kbps). This is not a distress signal (typically QPSK at 10-50 kbps). 8-PSK at 128 kilosymbols per second is a moderately sophisticated scheme — it sacrifices some noise margin for higher throughput, which means the transmitter was confident in the link quality. The data rate of 384 kbps sustained over the gap's duration could have transferred approximately 100 megabytes of data. The signal was bidirectional (the constellation shows both transmitted and received symbols intermixed), which means the ship was not just receiving — it was transmitting back. A two-way encrypted data exchange at 384 kbps, using a modulation scheme chosen for throughput over robustness. Whatever conversation happened during the gap, it was long, detailed, and deliberate. Someone on this ship had a lot to say to whatever was at bearing 213.
