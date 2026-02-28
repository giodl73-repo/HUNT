# DEAD RECKONING v2 — Asset List

**Document type:** Complete asset requirements for Phaser 4 build
**Audience:** Game developer (sourcing, creation, integration)
**Companion to:** `SCENE-ARCHITECTURE.md`, `THEME.md`

---

## AUDIO

### Ambient Tracks (5 deck loops + 4 special locations)

Each ambient track is a seamless loop, 60-120 seconds. Crossfade on room transitions (500ms).

| ID | Location | Description | Key Characteristics |
|----|----------|-------------|---------------------|
| `amb-deck1-bridge` | Deck 1 — Bridge, Ready Room | Low reactor hum, distant ventilation, periodic console chimes | Quiet, professional. Minimal. The sound of a ship at rest. |
| `amb-deck2-ops` | Deck 2 — All Operations rooms | Comm static, signal processing tones, data flow sounds | Busier than bridge. Active data environment. Signal chirps. |
| `amb-deck3-crew` | Deck 3 — All Crew spaces | Softer hum, occasional intercom, life support cycling | Residential, calm. Lower energy. Human spaces. |
| `amb-deck4-support` | Deck 4 — Science Lab, Security, Cargo | Security beeps, lab equipment hum, cargo bay echoes | Functional, institutional. Harder surfaces, more reverb. |
| `amb-deck5-eng` | Deck 5 — All Engineering spaces | Loud reactor hum, plasma conduit flow, heavy equipment | Industrial, alive. The loudest ambient. Engine pulse. |
| `amb-turbolift` | Turbolift interior | Mechanical movement hum, acceleration/deceleration | Transit only. 5-second loop with speed ramp. |
| `amb-briefing` | Briefing Room (1-02-02) | Near-silence, occasional console hum | Formal, tense. The quietest location in the game. |
| `amb-observation` | Observation Lounge (1-03-01) | Deep space ambient, hull creaks | Vast, empty. The sound of space pressing against the hull. |
| `amb-sickbay` | Sickbay (3-01-01) | Clinical environment, life support, three ECG rhythms | Sterile. ECG beeps are diegetic, not ambient. Separate layer. |

---

### The Echo Motif

A recurring 3-note musical phrase built on a 3:1 harmonic relationship (matching the contact's signal ratio from R1-02). Two tones: a base note and a note at 3x the frequency. The motif is the contact's signature in musical form.

| Occurrence | Scene | Volume | Quality | Notes |
|------------|-------|--------|---------|-------|
| 1st | Scene 10 — R1-META solve | 15% | Distant, faint, single iteration | First hint something is out there. Barely noticed. |
| 2nd | Scene 19 — THE TURN (R2-06) | 25% | Closer, clearer, slight sustain | The ship was waiting. The contact was expected. |
| 3rd | Scene 23 — Act 2 narrative (turbolift) | 40% | Present, recognizable, two iterations | "Someone on this ship knew." |
| 4th | Scene 30 — R3-META solve | 60% | Close, clear, with overtones | The chain is traced. Two metallic impacts precede it. |
| 5th | Scene 32 — THE REVEAL (pointer lands) | 100% | Full volume, clear, unmistakable. Not distant. Present. | The naming moment. Complete silence before and after. |
| 6th | Scene 33 — Reconstruction scrolling | 100% | Full, harmonized, resolved, at rest | The motif was the contact's signature all along. Resolution. |
| 7th | Scene 35 — Aftermath (viewport) | 10% | Faint, external, from viewport direction | Still out there. Very far. The final note. |

**Production notes:** As the motif recurs, it gains harmonic richness — additional overtones, longer sustain, warmer timbre. Iterations 1-3 are sparse (single instrument). Iterations 4-5 add depth. Iteration 6 is harmonized (resolved chord). Iteration 7 strips back to a single distant tone.

---

### Cutscene Audio

| ID | Scene | Description | Duration |
|----|-------|-------------|----------|
| `sfx-the-turn-silence` | Scene 19 (R2-06 solve) | 3 seconds of complete silence after "WAITING." before echo motif | 3 sec |
| `sfx-the-drop-clunks` | Scene 27 (R3-04 solve) | Three FORCED door clunks replayed in rapid succession, then sustained low tone, then silence | 5 sec total |
| `sfx-the-reveal-wind` | Scene 32 (Final meta) | Clock mechanism winding up sound before pointer begins moving | 1 sec |
| `sfx-the-reveal-tick` | Scene 32 (Final meta) | Slow resonant tick accompanying pointer movement around the circle | 2 sec |
| `sfx-chain-link` | Scene 32 (Final meta) | Brief echo of metallic chain link sound as pointer passes REEVES | 0.5 sec |
| `sfx-reveal-silence` | Scene 32 (Final meta) | Complete silence after VASQUEZ name appears on screen | 5 sec |
| `sfx-submit-chime` | Scene 35 (Epilogue — SUBMIT) | Distant comm transmission chime (findings going out to Sector Command) | 2 sec |
| `sfx-close-seal` | Scene 35 (Epilogue — CLOSE) | Sharp, final file seal click (the commission is sealed) | 1 sec |
| `sfx-kwon-heartbeat` | Scene 21 (R2-META) | Heartbeat monitor sound, brief, then flatline | 3 sec |

---

### UI Sounds

| ID | Trigger | Description |
|----|---------|-------------|
| `ui-panel-activate` | Player enters a room with a puzzle console | 3-second power-up: fan spin, display flicker, steady hum |
| `ui-puzzle-solve` | Correct answer submitted | Three ascending confirmation tones |
| `ui-wrong-answer` | Incorrect answer submitted | Low error buzz, brief |
| `ui-turbolift-doors` | Turbolift opens/closes | Hydraulic door mechanism, smooth |
| `ui-door-unlock` | Room transitions from LOCKED to OPEN | Lock disengage click + door slide |
| `ui-door-locked` | Player clicks a LOCKED door | Denied buzz + lock icon flash |
| `ui-feeder-chime` | Each feeder puzzle solved | Single ascending chime |
| `ui-threshold-tone` | Round threshold reached (4/6, 4/6, 3/5) | Different, deeper tone than feeder chime |
| `ui-commission-update` | Phase transition (round complete) | Formal commission update chime |
| `ui-console-hum` | Steady background on any active console | Constant low hum when a puzzle panel is open |
| `ui-stepper-click` | NumericStepper increment/decrement | Mechanical click, brief |
| `ui-dial-turn` | RotaryDial position change | Knob detent click with smooth rotation |
| `ui-toggle-flip` | ToggleSwitch state change | Switch snap sound |
| `ui-slider-drag` | PowerSlider / LinkedSliderInput movement | Smooth analog slide |
| `ui-throttle-detent` | ThrottleLever snaps to position | Heavy mechanical detent snap |
| `ui-guard-lift` | BatSwitch safety guard lifted | Mechanical click-clack |
| `ui-guard-close` | BatSwitch safety guard closed | Snap |
| `ui-bat-flip` | BatSwitch toggled | Heavy toggle sound |
| `ui-alarm-pulse` | MasterAlarm active | 3 Hz pulsing alarm tone |
| `ui-alarm-ack` | MasterAlarm acknowledged | Alarm silence + reset click |
| `ui-lock-on` | TargetingReticleDisplay lock achieved | Targeting lock confirmation tone |
| `ui-lock-lost` | TargetingReticleDisplay lock lost | Warning warble |
| `ui-radar-ping` | RadarSweepDisplay rotation | Single ping per revolution |
| `ui-probe-green` | CircuitTopologyDisplay probe — current present | High electrical test tone |
| `ui-probe-red` | CircuitTopologyDisplay probe — no current | Dead silence (absence of tone) |
| `ui-badge-granted` | Badge swipe GRANTED (R3-04) | Clean beep |
| `ui-badge-denied` | Badge swipe DENIED (R3-04) | Harsh buzz |
| `ui-badge-forced` | Badge swipe FORCED (R3-04) | Heavy metallic door clunk |
| `ui-register-tick` | Procedure register increment (R3-05) | Audible counter tick |
| `ui-scroll-text` | ScrollingTextDisplay advancing | Subtle paper-on-screen scroll |
| `ui-auth-chime` | "AUTHORIZED" confirmation | Brief authentication chime |

---

## VISUAL

### Ship Corridor Textures (5 deck color schemes)

Each deck has a distinct color palette applied to walls, floors, door frames, and console trim. The textures convey the division identity.

| Deck | Division | Primary Color | Accent Color | Floor | Wall Texture |
|------|----------|--------------|-------------|-------|-------------|
| 1 | RED — Command | `#8B0000` deep red | `#CC2222` bright red | Dark grey non-slip, red strip lighting | Brushed dark metal, red accent panels. Professional. |
| 2 | YELLOW — Operations | `#DAA520` amber | `#FFCC44` gold | Dark grey, amber strip lighting | Warm grey panels, amber comm displays. Active. |
| 3 | GREEN — Crew | `#2E8B57` sea green | `#44CC77` bright green | Lighter grey, green strip lighting | Softer wall material, warmer tones. Residential. |
| 4 | BLUE — Support | `#1E3A5F` deep navy | `#3366AA` medium blue | Dark grey, blue strip lighting | Institutional. Lab-grade surfaces. Cool. |
| 5 | ORANGE — Engineering | `#CC5500` burnt orange | `#FF7722` bright orange | Grated metal, orange strip lighting | Industrial. Exposed conduits. Functional. |

### Door States (4 visual states)

| State | Frame Color | Indicator | Texture Notes |
|-------|-------------|-----------|---------------|
| OPEN | Green (`#00CC44`) | Green light above frame, no icon | Door panel slides open, corridor visible |
| LOCKED | Red (`#CC2222`) | Red light, lock icon overlay | Door panel closed, tooltip on hover shows unlock condition |
| SECURED | Amber (`#DAA520`) | Amber light, keypad icon | Door panel closed, security keypad visible on frame |
| FORCED | Amber (`#DAA520`) + damage | Amber light, damage texture | Door panel ajar, scorch marks on frame, forced-entry damage. Flavor only. |

### Turbolift Interior

Simple functional box. Deck selection button panel on one wall (5 buttons, labeled by deck name and color). Small status display panel above buttons for narrative text. Overhead lighting. No windows. Mechanical, utilitarian.

### The VASQUEZ Reveal (Scene 32 — The Most Important Visual Moment)

**Full specification in `THEME.md`**. Summary: when the pointer lands on position 5, the entire screen transitions to a full-screen hold. Background fades to deep command red (`#1a0505`). The name VASQUEZ appears centered in primary monospace font, large, white. The circle remains visible but dimmed, with position 5 highlighted. The hold lasts exactly 5 seconds with zero animation during the hold. No flicker. No pulse. Static. The name sits.

### Round Accent Colors — Simultaneous Appearance

The only moment in the game when all three round accent colors appear simultaneously is Scene 31 (Round 3 complete / final meta unlock). The display shows:

```
N = 8         (cyan — Round 1 accent)
START = 3     (amber — Round 2 accent)
STEP = 2      (red — Round 3 accent)
```

Three values, three colors, one line each. Centered on the Security Office console. Accompanied by three ascending tones. This is the "all evidence collected" moment. The colors are:
- Round 1 (TELEMETRY): `#00CCFF` cyan
- Round 2 (SYSTEMS LOG): `#FFAA33` amber
- Round 3 (CREW RECORD): `#FF4444` red

These appear as background glow behind each value line. Not garish. Subtle. The first and only time the three investigation threads visually converge.

---

## FONTS

### Primary Monospace — Ship Terminal Font

**Purpose:** All log entries, terminal text, data displays, instrument readouts, commission documents, reference cards, puzzle console text.

**Characteristics:** Fixed-width. Clean. No serifs. Slightly condensed. Must render cleanly at small sizes (10px for log entries) and large sizes (32px for VASQUEZ reveal). High contrast white on dark backgrounds.

**Recommended:** JetBrains Mono, IBM Plex Mono, or Source Code Pro. Fallback: system monospace.

**Weight variants needed:**
- Regular (400) — body text, log entries, reference cards
- Bold (700) — status headers, console labels, puzzle titles
- Light (300) — ambient status text, secondary readouts

### Display Font — Room Titles and Briefing Headers

**Purpose:** Section headers, room titles, deck names, commission briefing headers, "DEAD RECKONING" title screen.

**Characteristics:** Slightly warmer than the terminal font. Still geometric and clean but with subtle humanist touches. Not decorative. Functional warmth — the difference between a terminal and a briefing room.

**Recommended:** Space Grotesk, Inter Display, or Exo 2. Fallback: system sans-serif.

**Weight variants needed:**
- Medium (500) — room titles, deck labels
- Bold (700) — section headers, briefing titles
- Black (900) — "DEAD RECKONING" title, "VASQUEZ" reveal backup

---

## SPRITE SHEETS / UI ELEMENTS

### Console Widget Sprites

All widgets are implemented in `ConsoleWidgets.ts` and `InteractiveControls.ts` as Phaser GameObjects. They render programmatically, not from sprite sheets. However, the following decorative elements need art assets:

| Asset | Description | Size | Notes |
|-------|-------------|------|-------|
| `console-bezel.9patch` | Console frame around each puzzle panel | 9-patch scalable | Dark metal with rounded corners, division-colored accent strip |
| `reference-card-bg.png` | Reference card panel background | 500x300 px | Slightly lighter than console background, paper-like texture |
| `commission-seal.png` | Commission Order #2247 seal/watermark | 128x128 px | Faded official stamp for commission documents |
| `deck-map-bg-{1-5}.png` | Top-down deck map background per deck | 800x600 px | Simplified architectural blueprint style in deck division color |
| `minimap-strip.png` | 5-deck minimap sidebar | 40x200 px | 5 colored strips (RED/YELLOW/GREEN/BLUE/ORANGE) with player dot |

### Status Icons

| Asset | Description | Size |
|-------|-------------|------|
| `icon-lock.png` | Lock icon for LOCKED doors/consoles | 24x24 px |
| `icon-keypad.png` | Keypad icon for SECURED doors | 24x24 px |
| `icon-solved.png` | Checkmark for solved puzzles in progress tracker | 16x16 px |
| `icon-unsolved.png` | Empty box for unsolved puzzles | 16x16 px |
| `icon-hint.png` | Hint button icon | 32x32 px |

---

## DATA FILES

### Puzzle Configuration JSONs

Each puzzle needs a config JSON that the Puzzle Container reads at runtime:

| File | Puzzle | Contents |
|------|--------|----------|
| `puzzle-R1-01.json` | Carrier Isolation | SineWaveDisplay config, signal components, filter params |
| `puzzle-R1-02.json` | Phase Lock | LissajousDisplay config, frequencies, lock tolerance |
| `puzzle-R1-03.json` | Null Zone | PhaseInterferenceDisplay config, source positions, bearings |
| `puzzle-R1-04.json` | Orbit Classification | ConicSectionDisplay config, observed positions, eccentricity |
| `puzzle-R1-05.json` | Contact Lock | RadarSweepDisplay config, 8 contact positions and behaviors |
| `puzzle-R1-06.json` | Signal Fingerprint | CommSignalDisplay config, constellation data, EVM table |
| `puzzle-R2-01.json` | Power Path | ConduitFlowDisplay config, source maps, junction logic |
| `puzzle-R2-02.json` | Data Breach | NetworkGridDisplay config, 15 nodes, rogue route |
| `puzzle-R2-03.json` | Heat Source | HeatMapDisplay config, initial temperatures, source params |
| `puzzle-R2-04.json` | Shield Profile | ShieldDisplay config, damage model, reference pattern |
| `puzzle-R2-05.json` | Fault Trace | CircuitTopologyDisplay config, 20-node tree, failed component |
| `puzzle-R2-06.json` | Reactor State | GaugeDisplay (x4) config, throttle detents, setpoints |
| `puzzle-R3-01.json` | Triage | LifesignsDisplay config, 3 patient waveform params |
| `puzzle-R3-02.json` | Access Code | ModularClockDisplay config, rotation sequence |
| `puzzle-R3-03.json` | Permission Chain | CayleyTableDisplay config, Z_8 table, base/target |
| `puzzle-R3-04.json` | Badge Sequence | IndicatorPanel config, 18 swipe entries |
| `puzzle-R3-05.json` | Emergency Sequence | BatSwitch (x4) config, correct sequence, register logic |
| `puzzle-R1-META.json` | Contact | Contact Signature Reference table, class codes |
| `puzzle-R2-META.json` | Response | Station Command Log table, parity logic |
| `puzzle-R3-META.json` | Cover-Up | Authorization chain data, action-auth matrix |
| `puzzle-FINAL.json` | The Commission | CyclicGroupDisplay config, 8 labels, animation params |

### Answer Key (embedded, not a separate file)

Answer validation is compiled into the game binary per the `ANSWER_KEY` constant in `GAME-SCRIPT.md` Section 4. Not shipped as a readable file.
