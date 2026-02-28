# R2-04 — SHIELD PROFILE
**Round:** SYSTEMS LOG
**Department:** TAC / SHIELD SYSTEMS
**Panel:** Shield Configuration Console — Bridge Tactical Station, Deck 1
**Classification:** RESTRICTED — SYSTEMS LOG RECONSTRUCTION

---

## 1. Panel Overview

**Narrative beat:** The ship took fire during the gap. The shield damage pattern was preserved in a backup sensor — specific layers absorbed specific amounts of damage. The pattern is not consistent with a reactive defense (where all layers would be balanced). It matches a pre-programmed defensive posture — one layer deliberately weakened, another strengthened. The solver adjusts three shield layer power sliders until the simulated damage distribution matches the recorded pattern exactly.

**Win condition:** The ShieldDisplay damage indicators on all three layers match the reference damage overlay from the gap log. The three PowerSliders show the exact power allocation that produced the recorded damage pattern.

**Answer value:** 65 (Layer 2 power percentage)

---

## 2. Widget Configuration

### Primary Display

**Widget:** `ShieldDisplay`
**Config:**
```
{
  layerCount: 3,
  shape: "hexagonal",
  layers: [
    { id: "L1", label: "LAYER 1 — OUTER",  position: "outermost" },
    { id: "L2", label: "LAYER 2 — MIDDLE", position: "middle" },
    { id: "L3", label: "LAYER 3 — INNER",  position: "innermost" }
  ],
  strengthColorScale: {
    low:  "#ff2222",
    mid:  "#ffcc00",
    high: "#2288ff"
  },
  damageOverlay: true,
  referencePattern: {
    L1: { damagePercent: 78, label: "HEAVY" },
    L2: { damagePercent: 42, label: "MODERATE" },
    L3: { damagePercent: 14, label: "LIGHT" }
  },
  referenceColor: "#FFFFFF44",
  matchTolerance: 2,
  matchIndicator: true,
  matchIndicatorColor: "#00FF00",
  impactSimulation: {
    enabled: true,
    attackPower: 200,
    attackType: "sustained-beam",
    duration: "6h equivalent"
  }
}
```

### Reference Damage Overlay

The ShieldDisplay shows a faint white overlay on each layer representing the recorded damage from the gap. Each layer has a damage indicator bar:

```
SHIELD DAMAGE LOG — RECOVERED FROM BACKUP SENSOR
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

  LAYER 1 (OUTER):   ████████████████████░░░░░  78% damage   HEAVY
  LAYER 2 (MIDDLE):  ██████████░░░░░░░░░░░░░░░  42% damage   MODERATE
  LAYER 3 (INNER):   ███░░░░░░░░░░░░░░░░░░░░░░  14% damage   LIGHT
```

### Input Controls

**Widget:** `PowerSlider` (x3)
```
[
  { id: "slider-L1", label: "LAYER 1 POWER",  unit: "%", min: 0, max: 100, step: 1,
    initialValue: 50, setpoint: null },
  { id: "slider-L2", label: "LAYER 2 POWER",  unit: "%", min: 0, max: 100, step: 1,
    initialValue: 50, setpoint: null },
  { id: "slider-L3", label: "LAYER 3 POWER",  unit: "%", min: 0, max: 100, step: 1,
    initialValue: 50, setpoint: null }
]
```

### Threat Gauge

**Widget:** `GaugeDisplay`
```
{
  id: "gauge-threat",
  label: "INCOMING THREAT POWER",
  unit: "MW equivalent",
  min: 0,
  max: 300,
  value: 200,
  zones: { green: [0, 100], yellow: [100, 200], red: [200, 300] },
  note: "Fixed — this gauge shows the total attack energy recorded during the gap. Not adjustable."
}
```

### Damage Simulation Logic

```
The attack (200 MW equivalent) is distributed across the three shield layers.
Damage to each layer depends on its power allocation:

  Layer damage formula:
    damage_fraction(L) = (1 - power(L)/100) * weight(L)
    where weight distributes the attack according to shield geometry:
      L1 (outer): weight = 0.50 (faces the attack first, absorbs the most)
      L2 (middle): weight = 0.30 (absorbs what passes L1)
      L3 (inner):  weight = 0.20 (absorbs residual)

  Effective damage percent:
    damage%(L) = attack_power * (1 - power(L)/100) * weight(L) / layer_capacity(L) * 100

  Simplified model (what the solver sees):
    At power P%, the layer absorbs (100 - P)% of incoming energy assigned to it.
    Higher power = less damage. Lower power = more damage.

  Calibration (these produce the reference pattern):
    L1 at 30% power → 78% damage  (weak outer layer takes heavy beating)
    L2 at 65% power → 42% damage  (moderate middle layer, moderate damage)
    L3 at 90% power → 14% damage  (strong inner layer, light damage)

  The ShieldDisplay animates damage indicators in real time as sliders move.
  When all three damage indicators match the reference overlay (within ±2%),
  the match indicators turn green and the panel signals success.
```

---

## 3. Reference Card

```
SHIELD CONFIGURATION CONSOLE — OPERATOR REFERENCE
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

WHAT YOU SEE:
  A three-layer hexagonal shield display. Each layer has:
  - A color showing current power level (red = low, blue = high)
  - A damage indicator bar showing simulated damage
  - A faint white overlay showing the REFERENCE damage from the gap log

  The reference pattern is fixed — it shows what actually happened.
  Your simulated pattern changes as you adjust the power sliders.

THE PROBLEM:
  During the gap, the shields were set to a specific power allocation.
  That allocation produced the damage pattern in the reference overlay.
  You must find the power settings that reproduce the pattern exactly.

HOW SHIELD DAMAGE WORKS:
  - A STRONGER layer (higher power %) absorbs more energy and takes
    LESS damage. The shield deflects the attack.
  - A WEAKER layer (lower power %) absorbs less energy and takes
    MORE damage. The attack passes through.
  - The outer layer faces the attack first and bears the heaviest load.
  - The inner layer is protected by the outer layers and takes the least.

WHAT TO DO:
  1. Read the reference pattern:
     Layer 1: HEAVY damage (78%) → Layer 1 was WEAK during the gap.
     Layer 2: MODERATE damage (42%) → Layer 2 was MODERATE.
     Layer 3: LIGHT damage (14%) → Layer 3 was STRONG.

  2. Adjust the three PowerSliders:
     - Set Layer 1 LOW (it took heavy damage, so it was set weak)
     - Set Layer 3 HIGH (it took light damage, so it was set strong)
     - Set Layer 2 somewhere in between

  3. Watch the damage indicators. When they match the reference
     overlay on all three layers, the match indicators turn green.

  4. Fine-tune. The match must be within ±2% on all layers.

SUCCESS STATE:
  All three match indicators green. Read the Layer 2 slider value.
  That percentage is your answer.
```

---

## 4. Novice Solve Path

1. **Read the reference pattern:** Layer 1 heavy (78%), Layer 2 moderate (42%), Layer 3 light (14%). The novice understands: more damage = weaker layer.

2. **Set Layer 1 low:** Move slider to ~20%. Check damage indicator. Layer 1 damage shows ~85%. Too high — the layer is too weak. Move up to 30%. Damage shows ~78%. Close to the reference. Keep it.

3. **Set Layer 3 high:** Move slider to ~95%. Layer 3 damage shows ~8%. Too low — the layer is too strong. Move down to 90%. Damage shows ~14%. Matches reference.

4. **Adjust Layer 2:** Start at 50%. Damage shows ~55%. Too much damage. Move up to 70%. Damage shows ~35%. Too little. Move down to 60%. Damage shows ~48%. Still too much. Try 65%. Damage shows ~42%. Matches reference.

5. **Confirmation:** All three match indicators turn green. The solver reads Layer 2: 65%.

6. **Answer:** 65.

---

## 5. Expert Solve Path

1. Read the reference: L1 = 78% damage (heavy), L2 = 42% (moderate), L3 = 14% (light). Inverse relationship: high damage = low power.

2. The damage model is approximately linear in (100 - P), scaled by layer weight. The expert estimates:
   - L3 at 14% damage with weight 0.20 → power must be high, try 85-90%.
   - L1 at 78% damage with weight 0.50 → power must be low, try 25-35%.
   - L2 at 42% damage with weight 0.30 → power somewhere around 60-70%.

3. Set L1 = 30, L2 = 65, L3 = 90. Check: all three match indicators go green.

4. Answer: 65. Three slider adjustments, done.

---

## 6. Data Values

### Shield Layer Configuration (Answer State)

| Layer | Power % | Damage % (Reference) | Damage Label | Shield Color |
|-------|---------|---------------------|-------------|-------------|
| Layer 1 (Outer) | 30% | 78% | HEAVY | Red |
| Layer 2 (Middle) | 65% | 42% | MODERATE | Yellow |
| Layer 3 (Inner) | 90% | 14% | LIGHT | Blue |

### Damage Model Calibration Table

Selected power values and resulting damage percentages for each layer:

**Layer 1 (Outer, weight 0.50):**

| Power % | Damage % |
|---------|---------|
| 0 | 100 |
| 20 | 85 |
| **30** | **78** |
| 50 | 57 |
| 70 | 36 |
| 100 | 0 |

**Layer 2 (Middle, weight 0.30):**

| Power % | Damage % |
|---------|---------|
| 0 | 100 |
| 30 | 70 |
| 50 | 55 |
| **65** | **42** |
| 80 | 28 |
| 100 | 0 |

**Layer 3 (Inner, weight 0.20):**

| Power % | Damage % |
|---------|---------|
| 0 | 100 |
| 50 | 43 |
| 70 | 28 |
| 80 | 21 |
| **90** | **14** |
| 100 | 0 |

### Near-Miss Values (Layer 2 sensitivity)

| L2 Power | L2 Damage | Match? |
|----------|----------|--------|
| 63% | 45% | No (+3 from reference) |
| 64% | 43% | No (+1, within tolerance but L1/L3 may drift) |
| **65%** | **42%** | **YES** |
| 66% | 41% | No (-1, within tolerance but exact match at 65) |
| 67% | 39% | No (-3 from reference) |

The match tolerance is ±2%, so values 63-67 are technically within range. However, the calibration is designed so that 65% is the only integer value where ALL THREE layers simultaneously match within tolerance. At 64% or 66% for L2, the required L1 and L3 values shift to non-integer positions, making 65% the natural resting point.

### Attack Parameters

| Parameter | Value |
|-----------|-------|
| Attack type | Sustained beam (not torpedo burst) |
| Total energy | 200 MW equivalent over gap duration |
| Distribution | Geometric: 50% L1, 30% L2, 20% L3 |
| Duration | 6 hours (consistent with gap window) |

---

## 7. Narrative Revelation

On achieving the match (all three indicators green), the following log entry appears on the ShieldDisplay status line:

```
SHIELD CONFIGURATION LOG — GAP +00:03
  Defensive posture: PRESET ECHO-7
  Layer allocation: 30 / 65 / 90
  Authorization: Standing order. Not reactive.
  Preset loaded 4 minutes before contact detected on sensors.
```

---

## 8. Story Layer

**The anomaly:** The shield configuration was not a reactive defense. It was a pre-programmed preset — ECHO-7 — loaded four minutes before the contact appeared on sensors. A reactive defense would have all layers balanced (e.g., 80/80/80 or configured based on the observed attack vector). ECHO-7 is an asymmetric posture: Layer 1 deliberately weakened, Layer 3 strengthened. This configuration prioritizes protecting the aft section (where the contact was approaching from bearing 213 degrees, per R1-03) while accepting damage on the forward-facing outer layer.

**What this means:** Someone knew the attack vector before the contact was detected. The preset was loaded by standing order — an automatic configuration, not a manual response by the tactical officer on duty. Standing orders are set by the commanding officer. The CO pre-programmed a defensive posture for a specific threat geometry before anyone else on the bridge knew the threat existed.

**Connection to conspiracy:** The 4-minute lead time is critical. At GAP +00:03, the shields were configured. At GAP +00:07, the contact first appeared on sensors (cross-reference with R1-05 Contact Lock). In those four minutes, the CO knew what was coming and from where. This is not preparedness — this is foreknowledge. Combined with the sensor suppression (R2-02) that began at GAP +00:14 and the EPS reroute (R2-01) at GAP +00:22, the timeline shows a sequence of deliberate preparations that preceded the "official" detection of the contact.
