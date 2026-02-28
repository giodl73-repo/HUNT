# R3-01 — TRIAGE
**Round:** CREW RECORD
**Department:** MEDICAL
**Instrument:** LifesignsDisplay — Multi-Channel ECG Monitor
**Classification:** RESTRICTED — CREW RECORD RECONSTRUCTION

---

## 1. Panel Overview

Three crew members were brought to sickbay during the gap. Their vital signs were recorded in the passive sensor buffer — the only medical data that survived the purge. One of these three was not merely stressed. One was in cardiac crisis. The crisis waveform does not lie: it says someone was hit hard enough to stop functioning at their station. The duty log says that person was asleep in their quarters. The duty log is wrong.

This is the first panel in Round 3. The tone is clinical. The anomaly is a quiet discrepancy — a waveform that does not match the story someone told.

---

## 2. Widget Configuration

### Primary Display

| Widget | Class | Configuration |
|--------|-------|---------------|
| ECG Monitor | `LifesignsDisplay` | 3 channels, simultaneous display. Each channel renders a morphing ECG waveform. Channel color: CH1 = cyan, CH2 = amber, CH3 = red. Grid background with 25mm/s time scale, 10mm/mV amplitude. Heart rate value displayed below each channel. |

### Input Controls

| Widget | Class | Configuration |
|--------|-------|---------------|
| Patient Selector | `RotaryDial` | 3 positions: Patient 1 / Patient 2 / Patient 3. Selects the active patient for slider adjustment. Inactive channels continue displaying at their logged rate. |
| Heart Rate Adjust | `LinkedSliderInput` | Range: 60-200 BPM, step: 1 BPM. Adjusts the selected patient's heart rate. The LifesignsDisplay morphs the waveform in real-time — not just faster/slower, but changing morphology as rate increases. |
| Heart Rate Readouts | `IndicatorPanel` | 3 LEDs. Each shows current BPM for its patient. Color follows waveform classification: green (sinus normal), yellow (sinus tachycardia), red (V-tach). |

### Panel Layout

```
┌─────────────────────────────────────────────────────────────────────┐
│  MEDICAL STATION — LIFESIGNS MONITOR                               │
│  Terminal: MED-BIOTERM-03-B       Location: SICKBAY, DECK 3       │
│  Data Source: Passive Sensor Buffer — GAP +01:22                   │
├─────────────────────────────────────────────────────────────────────┤
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  CH1  [Patient 1]  ───∿∿∿∿∿∿∿∿∿∿∿∿∿∿∿∿∿───  104 BPM    │   │
│  │  CH2  [Patient 2]  ───∿∿∿∿∿∿∿∿∿∿∿∿∿∿∿∿∿───  112 BPM    │   │
│  │  CH3  [Patient 3]  ───∿∿∿∿∿∿∿∿∿∿∿∿∿∿∿∿∿───  168 BPM    │   │
│  └─────────────────────────────────────────────────────────────┘   │
│                                                                     │
│  ┌──────────────┐  ┌──────────────────────────────┐                │
│  │ PATIENT SEL  │  │ HEART RATE         [● ● ●]  │                │
│  │   ◯ P1       │  │ ◄──────────────────────► BPM │                │
│  │   ◯ P2       │  │ 60                       200 │                │
│  │   ● P3       │  └──────────────────────────────┘                │
│  └──────────────┘                                                   │
│                                                                     │
│  ┌─────────────────────────────────────────────────────────────┐   │
│  │  REFERENCE CARD ──────────────────────────────────────────  │   │
│  │  (see Section 3)                                            │   │
│  └─────────────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────────────┘
```

---

## 3. Reference Card

Printed on the bezel placard, starboard side of the LifesignsDisplay. Standard sickbay reference material — all medical officers are expected to recognize these patterns.

```
ECG WAVEFORM CLASSIFICATION — QUICK REFERENCE
═══════════════════════════════════════════════════════════════════

  NORMAL SINUS RHYTHM (60–100 BPM)
    Shape:  Narrow QRS complex (< 120 ms duration)
            Visible P-wave before each QRS
            Regular rhythm, consistent R-R interval
    Meaning: Heart functioning normally.

  SINUS TACHYCARDIA (100–150 BPM)
    Shape:  Narrow QRS complex (< 120 ms) — same shape as normal
            P-waves still visible (may merge with preceding T-wave)
            Regular rhythm, faster rate
    Meaning: Elevated heart rate from physical exertion, pain,
             anxiety, or pharmacological stimulus. The heart is
             beating faster but the electrical pathway is normal.
             Common in acute stress. NOT a cardiac emergency.

  VENTRICULAR TACHYCARDIA / V-TACH (> 150 BPM)
    Shape:  WIDE QRS complex (> 120 ms) — broad, blobby peaks
            NO visible P-waves — atrial signal overwhelmed
            Regular rhythm, very fast rate
    Meaning: Abnormal electrical pathway originating in the
             ventricles. The ventricles are contracting without
             proper atrial coordination. Life-threatening if
             sustained. Requires immediate intervention.

  KEY DISTINCTION:
    Sinus tachycardia = fast but NARROW peaks, P-waves present
    V-tach = fast AND WIDE peaks, P-waves absent

    The QRS width is the diagnostic marker. If the peaks are
    WIDE (> 120 ms), it is V-tach regardless of rate.

═══════════════════════════════════════════════════════════════════
  NOTE: The LifesignsDisplay morphs waveform shape as rate
  changes. It does not merely speed up the trace. Watch the
  QRS width and P-wave presence — they change with rate.
═══════════════════════════════════════════════════════════════════
```

---

## 4. Novice Solve Path

A solver who has never read an ECG can solve this panel from the reference card alone.

1. **Read the reference card.** It defines three waveform types: Normal Sinus, Sinus Tachycardia, and V-tach. The key distinction is QRS width (narrow vs wide) and presence/absence of P-waves.

2. **Observe the three channels.** All three patients show elevated heart rates. The display renders three simultaneous ECG traces.

3. **Select Patient 1** (RotaryDial to P1). Observe the waveform at 104 BPM. The peaks are narrow and sharp. Small bumps (P-waves) are visible before each peak. Compare to the reference card: narrow QRS + visible P-waves = sinus tachycardia. Stressed, not in crisis.

4. **Select Patient 2** (RotaryDial to P2). Observe at 112 BPM. Peaks are narrow, slightly wider than Patient 1 but still clearly under 120 ms. P-waves visible. Sinus tachycardia. Stressed, not in crisis.

5. **Select Patient 3** (RotaryDial to P3). Observe at 168 BPM. The peaks are visibly different — broad, rounded, blobby. No small P-wave bumps before the peaks. Compare to the reference card: wide QRS + no P-waves = V-tach. Cardiac crisis.

6. **Confirm by adjusting.** Use the LinkedSliderInput to slowly decrease Patient 3's rate. Even at lower rates, the QRS remains wide — this is not just a fast sinus rhythm. The morphology is fundamentally different. Increase Patient 1 or 2 toward 168 BPM — their peaks get faster but remain narrow. The shape difference is unmistakable.

7. **Identify Patient 3** as the crisis patient. Patient ID = **3**.

---

## 5. Expert Solve Path

A solver who knows what V-tach looks like does not need the reference card.

1. Glance at the three ECG channels. Patient 3's trace is immediately recognizable: monomorphic V-tach at 168 BPM. Wide QRS, no P-waves, regular RR intervals.

2. Patients 1 and 2 show sinus tachycardia — narrow QRS, visible P-waves, rates 104 and 112. Unremarkable stress response.

3. Patient 3 = V-tach. Patient ID = **3**. Done.

Time for expert: under 30 seconds.

---

## 6. Data Values

### Patient Vital Data

| Patient | Logged Heart Rate | QRS Width | P-waves | Classification |
|---------|------------------|-----------|---------|----------------|
| Patient 1 | 104 BPM | 88 ms (narrow) | Present | Sinus Tachycardia |
| Patient 2 | 112 BPM | 96 ms (narrow) | Present | Sinus Tachycardia |
| Patient 3 | 168 BPM | 148 ms (wide) | Absent | Ventricular Tachycardia |

### Waveform Morph Parameters

| Rate Range | QRS Duration | P-wave Visibility | Classification |
|------------|-------------|-------------------|----------------|
| 60–100 BPM | 80–100 ms | Clear, distinct | Normal Sinus |
| 100–150 BPM | 80–110 ms | Present, may merge with T-wave | Sinus Tachycardia |
| >150 BPM (sinus) | 80–115 ms | Difficult to see, still present | Sinus Tachycardia (extreme) |
| >150 BPM (V-tach) | 130–180 ms | Absent | Ventricular Tachycardia |

### Answer

**Patient ID: 3**

The IndicatorPanel LED for Patient 3 glows red when classified as V-tach.

---

## 7. Narrative Revelation

```
MED-BIOTERM-03-B — PASSIVE BUFFER RECONSTRUCTION
TIMESTAMP: GAP +01:22

PATIENT 3 — CREW-ID 8837-SIGMA — LT. KWON, J.
STATION: COMPUTER    ROSTER POSITION: 3    SHIFT: OFF-DUTY (LOGGED)

ECG CLASSIFICATION: VENTRICULAR TACHYCARDIA — 168 BPM
QRS DURATION: 148 ms    P-WAVE: ABSENT    RHYTHM: REGULAR

DUTY LOG ENTRY (PURGED — RECOVERED FROM BACKUP):
  "Lt. Kwon: off-shift, quarters, sleeping."

PASSIVE BUFFER ENTRY (HARDWARE — NOT PURGED):
  Emergency intake. Unscheduled. Sickbay triage at 11:22.
  Cardiac monitor placed. V-tach confirmed. Intervention required.

DISCREPANCY NOTED.
```

---

## 8. Story Layer

The anomaly is quiet but clear. Lt. Kwon's duty log says off-shift, sleeping in quarters. The medical passive buffer says emergency cardiac intake in sickbay at GAP +01:22. A person asleep in their quarters does not present to sickbay in ventricular tachycardia.

Kwon was not asleep. Kwon was at the COMPUTER station — roster position 3 — when something happened. The V-tach episode is consistent with acute pharmacological stress: the atmospheric suppressant that Vasquez ordered dispersed through environmental systems caused an adverse cardiac reaction in Kwon. Kwon collapsed at the COMPUTER console. The station went unmonitored. The COMPUTER alert — the first system to flag anomalous data during the gap — went unacknowledged because its operator was in sickbay, unable to speak.

Someone edited the duty log to say Kwon was asleep. The passive sensor buffer was not edited because it writes to hardened memory outside the purge chain. The instruments do not agree with the story. The instruments are correct.

Patient 3 = Kwon. Roster position 3. The first thread of the cover-up, pulled loose.
