# DEAD RECKONING v2 — World Canon Lock

**Locked:** 2026-02-28
**Status:** LOCKED — no world changes after this point without updating this log

All puzzle facts must be verified against the files listed below. If a fact isn't in these files, it doesn't exist in this hunt's canon.

---

## Locked Files

### v1 Fiction (copied from ../space-game/)

| File | What It Contains | Locked |
|------|-----------------|--------|
| `world/WORLD.md` | Ship systems overview, 16 departments, power architecture (3 tiers, dependency tree), system states (6), comm bands (8), interference taxonomy (6 types), console model (3 meta-controls), career progression (4 tiers), ship scaling (6 classes), division colors | Yes |
| `world/systems/departments.md` | Per-department detail for all 16 departments — key systems, dependencies, cross-links, failure modes, puzzle hooks | Yes |
| `world/systems/data-tables.md` | ODN topology (15 nodes, data flow, components, scaling, security), room/console layout (console types, room bindings, access depth, shuttlepod), replicator security flags (6 categories, 5 auth tiers, skill progression, hard limits), EPS power consumption reference | Yes |

### v2 Physics Layer (new for this hunt)

| File | What It Contains | Locked |
|------|-----------------|--------|
| `world/systems/instruments/spectrum-analyzer.md` | Center frequency, span, RBW, noise floor, SNR improvement formula, peak hold behavior, harmonic identification, carrier acquisition procedure | Yes |
| `world/systems/instruments/oscilloscope.md` | X-Y mode, Lissajous figures, frequency ratio from tangencies, phase angle from Y-intercept, drift rate = beat frequency, stable figure reference table (1:1 through 5:3) | Yes |
| `world/systems/instruments/correlator.md` | Cross-correlation R_xy(tau), positive tau = echo, negative tau = precursor, distance d = c*tau/2, multiple peaks = multipath/composite, peak characteristics | Yes |
| `world/systems/instruments/phased-array.md` | Null steering, beam angle, null angle = interference bearing, maximum SNR = target bearing, multiple interference sources, SNR calculation | Yes |
| `world/systems/instruments/fft-analyzer.md` | N-point FFT, frequency resolution delta_f = fs/N, bin-frequency mapping f_k = k*delta_f, window functions (rectangular/Hanning/Blackman), averaging for noise reduction, carrier identification | Yes |
| `world/systems/instruments/vector-signal-analyzer.md` | IQ constellation, modulation identification (BPSK/QPSK/8-PSK/16-QAM), phase offset = constellation rotation, EVM, data rate = symbol_rate * bits_per_symbol | Yes |

---

## What Is Canon

### v1 Fiction (unchanged)

- 4 divisions: RED (Command), YELLOW (Operations), ORANGE (Engineering), BLUE (Science)
- 16 departments as listed in WORLD.md
- Power: Battery -> Fusion -> M/AM. Electrical (kW) + EPS (MW). PCU splits fusion.
- System States (6): ONLINE, READY, STANDBY, LOCKED, OFFLINE, FAULT
- Comm Bands (8): 4 short-range, 4 subspace
- Interference Taxonomy (6): Decoys, Shattered Carrier, Hieratic Triplet, Harmonic Echo, Phase-Locked Pair, Absorption Shadow
- Console Meta-Controls (3): AUTO, DIAGNOSTIC, CALIBRATE
- Career Progression (4): Technician -> Rotation -> Diagnostician -> Senior Officer
- ODN: 15 computer nodes, firewalled Science and Holographic computers, Linkage Node for Replicator/Transporter
- Replicator: 6 security categories (OPEN, BIO, HAZ, POISON, SEC, UNK), 5 auth tiers
- EPS Power Reference: Impulse 60-180 MW, Shields 95 MW, Env Fields 25 MW, Transporter 40 MW, Replicator 35 MW, Holodeck 55 MW

### v2 Physics Layer (new)

- **Spectrum Analyzer:** SNR_gain = 10*log10(span/RBW). Carrier power independent of RBW. Noise power scales with RBW. Peak hold separates carrier from noise.
- **Oscilloscope (Lissajous):** Frequency ratio = vertical tangencies / horizontal tangencies. Phase = arcsin(2*Y0/Y_max). Drift rate = beat frequency = |f1-f2|.
- **Correlator:** R_xy(tau) = cross-correlation. tau > 0 = echo. tau < 0 = precursor (anomalous). Distance d = c*tau/2. c = 3e8 m/s.
- **Phased Array:** Null at interference source maximizes target SNR. Optimal null angle = interference bearing. Beam angle at max signal = target bearing.
- **FFT Analyzer:** delta_f = fs/N. f_k = k*fs/N. Rectangular = best resolution, worst leakage. Hanning = general. Blackman = weak-near-strong. Averaging reduces noise by 10*log10(n_avg) dB.
- **Vector Signal Analyzer:** BPSK=2pt/1bit, QPSK=4pt/2bit, 8-PSK=8pt/3bit, 16-QAM=16pt/4bit. Data rate = symbol_rate * bits/symbol. Phase offset = constellation rotation. EVM = scatter quality.

---

## Canon Lock Log

| Date | What changed | Why |
|------|-------------|-----|
| 2026-02-28 | Initial lock | v1 fiction copied, v2 physics layer created, Stage 2 complete |

---

## Lock Protocol

If a puzzle author needs a fact that isn't in these files:
1. Check all world files first — it may be there under a different term
2. If genuinely missing, flag it here before authoring
3. A new fact can be added ONLY if it doesn't contradict existing canon
4. All additions must be logged in the Canon Lock Log above
5. Once a puzzle is authored against a fact, that fact is immutable
