# Vector Signal Analyzer — Instrument Reference

**Department:** COMMS
**Location:** Bridge OPS console [1], Comms station
**Instrument class:** IQ constellation analyzer

---

## Controls

| Control | Range | Step | Default |
|---------|-------|------|---------|
| Center Frequency | 1.000 - 5.000 GHz | 10 MHz | 2.400 GHz |
| Demodulation BW | 1 / 5 / 10 / 50 / 100 MHz | selector | 10 MHz |
| Symbol Rate | 1 kSym/s - 10 MSym/s | manual entry | AUTO |
| Modulation Type | AUTO / BPSK / QPSK / 8-PSK / 16-QAM | selector | AUTO |
| Phase Correction | 0 - 359 degrees | 1 degree | 0 |
| Display | Constellation / Eye Diagram / Spectrum | selector | Constellation |

## Display

**Constellation diagram:** I (in-phase) on horizontal axis, Q (quadrature) on vertical axis. Each received symbol is plotted as a point. Ideal symbols cluster at specific positions; real symbols scatter around ideal positions.

---

## Physics

### IQ Constellation Fundamentals

Digital modulation encodes data by varying the amplitude and/or phase of a carrier. Each symbol occupies a position in the IQ plane defined by its in-phase (I) and quadrature (Q) components.

### Modulation Identification

| Modulation | Points | Arrangement | Bits/Symbol |
|-----------|--------|-------------|-------------|
| **BPSK** | 2 | On I-axis: (+1, 0) and (-1, 0) | 1 |
| **QPSK** | 4 | At 45-degree angles: (+/-1, +/-1), normalized | 2 |
| **8-PSK** | 8 | Equally spaced on a circle (0, 45, 90, ... 315 degrees) | 3 |
| **16-QAM** | 16 | 4 x 4 rectangular grid | 4 |

### Phase Offset

A phase offset rotates the entire constellation. If the constellation appears rotated from its expected orientation:

```
Measured rotation (degrees) = phase offset
```

Apply Phase Correction = -(measured offset) to derotate.

- QPSK nominal positions: 45, 135, 225, 315 degrees. A 30-degree offset shifts them to 75, 165, 255, 345.
- 8-PSK nominal positions: 0, 45, 90, 135, 180, 225, 270, 315. Any rotation is detectable.

### Error Vector Magnitude (EVM)

EVM measures constellation quality — how far received symbols scatter from their ideal positions.

```
EVM = RMS(error vectors) / RMS(ideal positions) * 100%
```

| EVM | Quality |
|-----|---------|
| < 5% | Excellent. Clean signal. |
| 5-15% | Good. Minor distortion. |
| 15-30% | Degraded. Noise or interference present. |
| > 30% | Poor. Symbol errors likely. |

### Data Rate

```
Data rate = symbol_rate * bits_per_symbol
```

| Modulation | Bits/Symbol | Symbol Rate 1 MSym/s | Symbol Rate 5 MSym/s |
|-----------|-------------|---------------------|---------------------|
| BPSK | 1 | 1 Mbps | 5 Mbps |
| QPSK | 2 | 2 Mbps | 10 Mbps |
| 8-PSK | 3 | 3 Mbps | 15 Mbps |
| 16-QAM | 4 | 4 Mbps | 20 Mbps |

### Symbol Rate Determination

If symbol rate is unknown, the eye diagram can reveal it. The eye opening width corresponds to the symbol period:

```
symbol_rate = 1 / symbol_period
```

Alternatively, the constellation cluster count identifies modulation type, and the occupied bandwidth approximates symbol rate:

```
symbol_rate (approx) = occupied_bandwidth / (1 + roll-off_factor)
```

Typical roll-off factor: 0.25 to 0.5.

---

## Operating Procedure: Signal Identification

1. Set center frequency to the signal of interest.
2. Set Modulation Type = AUTO. The analyzer attempts to identify modulation from constellation geometry.
3. If AUTO fails (constellation too scattered), try each type manually. The correct type will show tightest clustering.
4. Apply Phase Correction to derotate the constellation to standard orientation.
5. Read EVM. If > 30%, the signal may be too degraded for reliable identification.
6. Determine symbol rate from eye diagram or bandwidth measurement.
7. Calculate data rate = symbol_rate * bits_per_symbol.

---

## Diagnostic Notes

- BPSK: two clusters on a line. If they're not on the I-axis, apply phase correction.
- QPSK: four clusters in a square. Rotation moves the square. Two overlapping pairs = excessive noise or wrong modulation type guess.
- 8-PSK: eight clusters on a ring. Missing clusters may indicate a lower-order modulation.
- 16-QAM: 4x4 grid. Corners have highest amplitude. If the grid is compressed or rhombus-shaped, there is IQ imbalance.
- A constellation that appears as a ring (not clusters) is likely an unsynchronized signal — the symbol rate setting is wrong.
