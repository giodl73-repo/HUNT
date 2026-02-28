# FFT Analyzer — Instrument Reference

**Department:** COMMS / SCIENCE
**Location:** Bridge Science console [1], Comms station, Science Lab
**Instrument class:** Frequency-domain transform processor

---

## Controls

| Control | Range | Step | Default |
|---------|-------|------|---------|
| FFT Size (N) | 256 / 512 / 1024 / 2048 / 4096 | selector | 1024 |
| Sample Rate (fs) | 1 / 2 / 5 / 10 / 20 MHz | selector | 10 MHz |
| Window Function | Rectangular / Hanning / Blackman | selector | Hanning |
| Averaging | 1 / 4 / 16 / 64 sweeps | selector | 1 |
| Display Scale | Linear / Logarithmic (dB) | toggle | dB |

## Display

Horizontal axis: frequency, from 0 to fs/2 (Nyquist limit). Divided into N/2 bins.
Vertical axis: magnitude (linear or dB).

---

## Physics

### Frequency Resolution

The spacing between frequency bins is:

```
delta_f = fs / N
```

Each bin k represents the frequency:

```
f_k = k * delta_f = k * fs / N
```

Where k ranges from 0 to N/2 - 1.

Higher N = finer frequency resolution (more bins, narrower spacing).
Higher fs = wider frequency range (higher Nyquist limit) but coarser resolution for a given N.

### Window Functions

The window function shapes the time-domain data before transformation. It controls the trade-off between frequency resolution and spectral leakage.

| Window | Main lobe width | Side lobe level | Best for |
|--------|----------------|----------------|----------|
| **Rectangular** | Narrowest (1 bin) | Highest (-13 dB) | Best resolution, worst leakage. Use when two signals are closely spaced and similar amplitude. |
| **Hanning** | Medium (2 bins) | Medium (-32 dB) | General purpose. Good resolution with moderate leakage suppression. |
| **Blackman** | Widest (3 bins) | Lowest (-58 dB) | Best for weak signals near strong ones. Side lobes from the strong signal won't mask the weak one. |

### Carrier Identification

A carrier (coherent signal) appears as a narrow spike in one or a few bins, rising above the noise floor. The noise floor is relatively flat.

```
Carrier: one bin (or a few adjacent bins) >> noise floor
Noise: all bins at roughly the same level
```

The signal frequency is the bin number times the frequency resolution:

```
f_signal = k_peak * fs / N
```

### Averaging

Averaging multiple FFT sweeps reduces noise variance. A carrier (deterministic signal) remains at full amplitude. Noise (random) decreases by:

```
Noise reduction = 10 * log10(number_of_averages) dB
```

16 averages reduces noise by 12 dB, making weak carriers more visible.

---

## Operating Procedure: Hidden Signal Detection

1. Select appropriate sample rate for expected frequency range.
2. Set N = 1024 (default). Window = Hanning.
3. Take a single sweep. Look for any bin significantly above the flat noise floor.
4. If no obvious peak: increase averaging to 16 or 64. Deterministic signals will emerge from noise.
5. If peak found: read bin number k. Calculate frequency: f = k * fs / N.
6. If resolution is insufficient (peak spans many bins): increase N or decrease fs.
7. If a weak signal is masked by a strong signal's side lobes: switch to Blackman window.

---

## Bin/Frequency Quick Reference (fs = 10 MHz, N = 1024)

```
delta_f = 10,000,000 / 1024 = 9765.625 Hz (approx 9.77 kHz per bin)

Bin 0:     0 Hz (DC)
Bin 1:     9,766 Hz
Bin 10:    97,656 Hz
Bin 50:    488,281 Hz
Bin 100:   976,563 Hz
Bin 256:   2,500,000 Hz (2.5 MHz)
Bin 512:   5,000,000 Hz (5.0 MHz = Nyquist)
```

---

## Diagnostic Notes

- A signal that appears in the same bin(s) across multiple sweeps is a real carrier. A signal that wanders across bins is either drifting in frequency or is wideband noise.
- Harmonics from a fundamental at bin k will appear at bins 2k, 3k, 4k, etc.
- If a known carrier at frequency f does not appear at the expected bin, check the sample rate — aliasing can fold signals above Nyquist back into lower bins.
- Rectangular window with N = 4096 gives the best frequency resolution but takes longer to compute. Use only when resolving closely spaced signals.
