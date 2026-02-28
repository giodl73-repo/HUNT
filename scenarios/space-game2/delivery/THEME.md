# DEAD RECKONING v2 — Visual Design System

**Document type:** Complete visual specification for Phaser 4 game
**Audience:** Game developer (UI, typography, color, animation)
**Companion to:** `GAME-SCRIPT.md`, `site/ASSET-LIST.md`, `site/SCENE-ARCHITECTURE.md`

---

## 1. DIVISION COLOR PALETTE

### Primary Division Colors

Each of the ship's four divisions has a primary and accent color. Engineering uses the Orange division color (a sub-division of Operations in the fiction but visually distinct).

| Division | Deck | Primary | Accent | Hex (Primary) | Hex (Accent) |
|----------|------|---------|--------|---------------|-------------|
| RED — Command | 1 | Deep red | Bright red | `#8B0000` | `#CC2222` |
| YELLOW — Operations | 2 | Amber | Gold | `#DAA520` | `#FFCC44` |
| GREEN — Crew | 3 | Sea green | Bright green | `#2E8B57` | `#44CC77` |
| BLUE — Support | 4 | Deep navy | Medium blue | `#1E3A5F` | `#3366AA` |
| ORANGE — Engineering | 5 | Burnt orange | Bright orange | `#CC5500` | `#FF7722` |

### Dark Backgrounds (per deck)

Each deck has a dark background tint derived from its division color. These are used for console panel backgrounds, room lighting tints, and corridor atmospherics.

| Deck | Dark Background | Hex |
|------|----------------|-----|
| 1 — Command | Very dark red | `#1a0505` |
| 2 — Operations | Very dark amber | `#0d0a00` |
| 3 — Crew | Very dark green | `#000d04` |
| 4 — Support | Very dark navy | `#000508` |
| 5 — Engineering | Very dark orange | `#0d0500` |

### Neutral Palette

Used for text, borders, inactive elements, and UI chrome that is not division-colored.

| Role | Hex | Description |
|------|-----|-------------|
| Primary text | `#E0E0E0` | Light grey — all body text, log entries, readouts |
| Secondary text | `#999999` | Medium grey — labels, timestamps, secondary info |
| Disabled text | `#555555` | Dark grey — locked items, unavailable controls |
| Panel background | `#0A0A0F` | Near-black — base for all console panels |
| Panel border | `#222233` | Dark blue-grey — console bezel border |
| Highlight border | `#4488AA` | Teal — active element borders, focus indicators |
| White (emphasis) | `#FFFFFF` | Pure white — setpoint markers, critical labels |
| Black (true) | `#000000` | True black — used only for silence/void moments |

### System State Colors

Standard across all decks and consoles. Never overridden by division color.

| State | Hex | Usage |
|-------|-----|-------|
| ONLINE / Active / Green | `#00CC44` | System running, door open, correct answer |
| READY / Blue | `#2288FF` | Power available, can activate |
| STANDBY / Amber | `#DAA520` | Was running, instant wake |
| LOCKED / Red outline | `#CC2222` | Power available but restricted |
| OFFLINE / Grey | `#555555` | No power, dependency not met |
| FAULT / Red flash | `#FF0000` | Hardware failure (animated flash at 2 Hz) |

---

## 2. ROUND ACCENT COLORS

Each investigation round has a distinct accent color used for feeder values, progress indicators, and narrative transitions.

| Round | Accent Color | Hex | Usage |
|-------|-------------|-----|-------|
| 1 — TELEMETRY | Cyan | `#00CCFF` | R1 feeder chimes, R1 progress markers, R1-META highlight |
| 2 — SYSTEMS LOG | Amber | `#FFAA33` | R2 feeder chimes, R2 progress markers, R2-META highlight |
| 3 — CREW RECORD | Red | `#FF4444` | R3 feeder chimes, R3 progress markers, R3-META highlight |

### Simultaneous Appearance (Scene 31 only)

The only moment in the game when all three round accent colors appear at once: the Security Office console after all three round metas are solved. Three values, three colors, one line each:

```
N = 8         (cyan background glow — #00CCFF at 15% opacity)
START = 3     (amber background glow — #FFAA33 at 15% opacity)
STEP = 2      (red background glow — #FF4444 at 15% opacity)
```

Centered on the console. Accompanied by three ascending tones. Subtle background glow, not garish. This is the "all evidence collected" moment.

---

## 3. TYPOGRAPHY

### Primary Monospace — Ship Terminal Font

**Purpose:** All log entries, terminal text, data displays, instrument readouts, commission documents, reference cards, puzzle console text. This is the voice of the ship.

**Recommended:** JetBrains Mono
**Alternates:** IBM Plex Mono, Source Code Pro
**Fallback:** `monospace` (system)

**Weight variants:**
| Weight | Value | Usage |
|--------|-------|-------|
| Light | 300 | Ambient status text, secondary readouts, timestamps |
| Regular | 400 | Body text, log entries, reference cards, puzzle instructions |
| Bold | 700 | Status headers, console labels, puzzle titles, answer readouts |

**Size scale:**
| Size | Usage |
|------|-------|
| 10px | Dense log entries, scrolling text, small status displays |
| 12px | Reference card body text, data tables, readout values |
| 14px | Console panel labels, room status text |
| 16px | Puzzle titles, section headers within panels |
| 20px | Room names in the navigation UI |
| 24px | Deck titles, round headers |
| 32px | VASQUEZ reveal name (Scene 32) |

**Line height:** 1.4 for body text, 1.2 for labels and headers.

**Character spacing:** Standard. Do not add letter-spacing. The monospace grid is the rhythm.

### Display Font — Room Titles and Briefing Headers

**Purpose:** Section headers, room titles, deck names, commission briefing headers, "DEAD RECKONING" title screen. Warmer than the terminal font. Functional, not decorative.

**Recommended:** Space Grotesk
**Alternates:** Inter Display, Exo 2
**Fallback:** `sans-serif` (system)

**Weight variants:**
| Weight | Value | Usage |
|--------|-------|-------|
| Medium | 500 | Room titles, deck labels, navigation labels |
| Bold | 700 | Section headers, briefing titles, commission headers |
| Black | 900 | "DEAD RECKONING" title screen, "VASQUEZ" reveal (backup if monospace does not convey sufficient weight) |

**Size scale:**
| Size | Usage |
|------|-------|
| 18px | Deck labels in minimap sidebar |
| 24px | Room titles in navigation header |
| 36px | Briefing headers, commission order titles |
| 48px | "DEAD RECKONING" title screen |

### Font Loading

Load both font families via Google Fonts or self-hosted WOFF2. Preload the Regular (400) and Bold (700) weights of the monospace font — they appear on the first screen. Display font can load asynchronously (first use is after the opening sequence).

---

## 4. CONSOLE PANEL DESIGN

### Panel Structure

Every puzzle console follows the same visual hierarchy:

```
+----------------------------------------------------------+
|  HEADER BAR (division color accent)                      |
|  [DEPT] -- [CONSOLE NAME]                                |
|  Terminal: [ID]    Location: [ROOM], DECK [N]            |
+----------------------------------------------------------+
|                                                          |
|  PRIMARY DISPLAY AREA                                    |
|  (widget: SineWaveDisplay, RadarSweepDisplay, etc.)     |
|  Background: #0A0A0F                                     |
|  Border: 1px #222233                                     |
|  Corner radius: 4px                                      |
|                                                          |
+----------------------------------------------------------+
|                                                          |
|  CONTROL AREA                                            |
|  (steppers, dials, sliders, buttons)                    |
|  Layout: horizontal row, evenly spaced                   |
|                                                          |
+----------------------------------------------------------+
|                                                          |
|  REFERENCE CARD AREA                                     |
|  Background: #0D0D14 (slightly lighter than panel)       |
|  Border: 1px dashed #333344                              |
|  Font: Primary monospace, Regular 400, 12px              |
|                                                          |
+----------------------------------------------------------+
```

### Header Bar

- Height: 48px
- Background: Division primary color at 20% opacity over `#0A0A0F`
- Left-aligned text: Department and console name in Display font, Bold 700, 16px
- Right-aligned text: Terminal ID and location in monospace, Light 300, 10px
- Bottom border: 1px solid division accent color at 40% opacity

### Console Bezel

- The entire panel is framed by a 9-patch bezel texture (`console-bezel.9patch`)
- Bezel: dark brushed metal with rounded corners
- Division-colored accent strip: 2px line along the top edge of the bezel, matching the deck's accent color
- Drop shadow: 0 4px 12px rgba(0,0,0,0.6)

### Reference Card Area

- Located at the bottom of every console panel
- Background: `#0D0D14` (slightly lighter than the console background)
- Border: 1px dashed `#333344`
- Text: Primary monospace, Regular 400, 12px, `#CCCCCC`
- Padding: 12px
- Always visible. Never collapsed. The reference card is part of the console — physically printed on the bezel.

---

## 5. WIDGET VISUAL STANDARDS

### Color Conventions

| Widget Element | Color | Notes |
|---------------|-------|-------|
| Primary signal / data | `#00FF88` (green) | The "active" signal color |
| Reference overlay | `#FFD700` (gold) | Target patterns, ideal positions |
| Error / fault | `#FF2222` (red) | Wrong state, alarm, fault |
| Neutral / grid | `#1A1A2A` | Background grids, axes |
| Selected / active | `#FFFFFF` (white) | Leading dots, selected items |
| Coolant / absorption | `#2196F3` (blue) | Coolant zones in R2-03 |
| Constructive | `#FFCC00` (yellow) | Constructive interference |
| Destructive | `#000022` (near-black) | Null zones in R1-03 |

### Animation Standards

| Element | Timing | Easing |
|---------|--------|--------|
| Gauge needle | 300ms | `Phaser.Math.Easing.Sine.InOut` |
| Waveform redraw | 200ms | Linear |
| LED state change | 150ms | `Phaser.Math.Easing.Cubic.Out` |
| Lock indicator flash | 500ms cycle | Square wave (on/off) |
| Radar sweep | 4 sec/revolution | Linear |
| Particle flow | 2.0 speed factor | Linear |
| Guard lift/close | 200ms | `Phaser.Math.Easing.Back.Out` |
| Switch flip | 100ms | `Phaser.Math.Easing.Cubic.Out` |
| Door slide open | 400ms | `Phaser.Math.Easing.Sine.InOut` |
| Turbolift transit | 1500ms per deck | `Phaser.Math.Easing.Quad.InOut` |

### Match Indicator

When a puzzle reaches its win condition, the match indicator follows this pattern:

1. Green glow appears around the display border (0.5s fade in)
2. A single ascending confirmation tone plays
3. The puzzle title text turns green
4. After 2 seconds, the narrative revelation log entry appears in the reference card area (scrolling text)
5. The feeder chime plays (round-colored — cyan/amber/red)

---

## 6. THE VASQUEZ REVEAL (Scene 32)

This is the most important visual moment in the game. Every design choice in this section is deliberate. Nothing is optional.

### Setup (Scene 31 — Briefing Room)

The CyclicGroupDisplay is rendered on the Briefing Room console. An 8-point circle. Labels visible: TORRES, NAKAMURA, OKAFOR, KWON, REEVES, VASQUEZ, PARK, CHEN.

Three inputs are displayed:

```
N = 8         [entered, confirmed]
START = 3     [entered, confirmed]
STEP = 2      [entered, confirmed]
```

When all three are confirmed, the EXECUTE button illuminates.

### The Pointer Animation

The player presses EXECUTE. The following sequence is precisely timed:

| Time | Event | Duration |
|------|-------|----------|
| 0.0s | Clock mechanism wind-up sound (`sfx-the-reveal-wind`) | 1.0s |
| 1.0s | Pointer appears at position 3 (KWON). A white highlight ring pulses once around KWON's label. | 0.5s |
| 1.5s | Pointer begins moving clockwise. Slow, deliberate. One tick sound (`sfx-the-reveal-tick`) per position crossed. The pointer passes position 4 (REEVES). Brief echo of chain-link sound (`sfx-chain-link`) as it passes REEVES. | 2.0s |
| 3.5s | Pointer arrives at position 5. It decelerates and stops precisely on VASQUEZ. | 0.5s |
| 4.0s | **THE HOLD BEGINS.** | |

### The Hold (5 seconds of absolute stillness)

This is not a cutscene. This is a held state. Nothing moves. Nothing animates. Nothing flickers.

**Background:** Fades from the console dark background to `#1a0505` (very dark command red). The fade takes 500ms and completes before the hold begins. During the hold, the background is static.

**The Circle:** Remains visible but dimmed to 30% opacity. Position 5 is highlighted with a white ring at full opacity. All other positions are ghosted. The circle is secondary — it frames the name but does not compete with it.

**The Name:** VASQUEZ appears centered on screen.

| Property | Value |
|----------|-------|
| Font | Primary monospace (JetBrains Mono) |
| Weight | Bold 700 |
| Size | 32px |
| Color | `#FFFFFF` (pure white) |
| Position | Horizontally centered, vertically centered (offset slightly above true center to account for the circle below) |
| Letter spacing | Standard (no extra spacing) |
| Animation | None. The text appears instantly at the start of the hold. No fade-in. No typing effect. No glow. Static. |

**Audio:** Complete silence. The `sfx-reveal-silence` asset plays — which is 5 seconds of literal silence. All ambient audio is muted for the duration. No reactor hum. No ventilation. Nothing.

**The Echo Motif:** Plays at 100% volume at the END of the hold (after the 5 seconds). Full, clear, unmistakable. The motif that has been building since Scene 10 reaches its penultimate statement. Two metallic impacts precede it (from the R3-META solve). Then the motif itself.

### After the Hold

The background remains at `#1a0505`. The name VASQUEZ remains on screen. The echo motif sustains and resolves. Then:

- Scene 33 begins: the reconstruction scrolling text. The name fades to 60% opacity and drifts upward as the scrolling text appears below it. The text uses primary monospace, Regular 400, 14px, `#CCCCCC`.
- The echo motif plays its 6th iteration: harmonized, resolved, at rest. The motif was the contact's signature all along.

### What NOT to Do

- Do NOT add a typing effect to the name. It appears instantly.
- Do NOT add a glow, pulse, or breathing animation. The name sits.
- Do NOT play any sound during the 5-second hold. Silence is the effect.
- Do NOT show the name in the display font. It must be monospace — it is data, not decoration. It is a readout, not a title card.
- Do NOT add a subtitle like "Captain" or "Commanding Officer." The name alone.
- Do NOT animate the background during the hold. The red is static after the initial fade.
- Do NOT show the pointer arriving in slow motion. The pointer moves at a deliberate but not dramatic pace. The drama is in the stillness after it stops.

---

## 7. EPILOGUE SCREENS (Scene 34-35)

### Scene 34: The Choice

**Location:** Ready Room (1-02-01)
**Background:** `#0A0A0F` (standard console dark)
**Layout:** Commission document format — centered text block, monospace.

The choice text is displayed as a commission document:

```
Font: Primary monospace, Regular 400, 14px
Color: #E0E0E0
Line spacing: 1.6 (wider than standard — this text needs to breathe)
Max width: 600px, centered
```

Two buttons at the bottom:

| Button | Label | Color | Hover |
|--------|-------|-------|-------|
| SUBMIT | SUBMIT FINDINGS | Amber `#DAA520` face, `#FFCC44` glow | Brightens to `#FFD700` |
| CLOSE | CLOSE COMMISSION FILE | Grey `#555555` face, `#777777` glow | Brightens to `#999999` |

Button size: 240px wide, 48px tall. Rounded corners: 4px. Gap between buttons: 40px. Centered horizontally.

**Audio:** Ship ambient only (the quietest ambient — observation lounge level). No music. No motifs. The choice is made in near-silence.

### Scene 35: The Aftermath

**Location:** Observation Lounge (1-03-01)
**Background:** Deep space viewport — stars visible through a wide window. The room is dark. Only the viewport provides light.

**Viewport visual:** A static star field. No nebulae, no dramatic space features. Stars only. Small, sharp, white points on black. The emptiness is the point.

#### SUBMIT Variant

```
Font: Primary monospace, Regular 400, 14px
Color: #E0E0E0

Header: "COMMISSION ORDER #2247 -- FILED"
  Font: Display font, Bold 700, 24px
  Color: #DAA520 (amber)

Body text fades in over 3 seconds, line by line (500ms per line).
After the body text completes, the DEAD RECKONING title appears:
  Font: Display font, Black 900, 36px
  Color: #FFFFFF
  Position: Centered, below the body text
  Fade-in: 2 seconds

Subtitle: "Incident reconstruction complete."
  Font: Primary monospace, Light 300, 12px
  Color: #999999
  Fade-in: 1 second after title

Audio:
  - sfx-submit-chime: distant comm transmission chime (2s)
  - Then silence (3s)
  - Then echo motif iteration 7: faint, from viewport direction, 10% volume
  - Viewport ambient continues indefinitely
```

#### CLOSE Variant

```
Font: Primary monospace, Regular 400, 14px
Color: #E0E0E0

Header: "COMMISSION ORDER #2247 -- SEALED"
  Font: Display font, Bold 700, 24px
  Color: #555555 (grey — sealed, not active)

Body text fades in over 3 seconds, line by line (500ms per line).
After the body text completes, the DEAD RECKONING title appears:
  Font: Display font, Black 900, 36px
  Color: #FFFFFF
  Position: Centered, below the body text
  Fade-in: 2 seconds

Subtitle: "Incident reconstruction complete."
  Font: Primary monospace, Light 300, 12px
  Color: #999999
  Fade-in: 1 second after title

Audio:
  - sfx-close-seal: sharp file seal click (1s)
  - Then silence (4s)
  - Then echo motif iteration 7: faint, from viewport direction, 10% volume
  - Viewport ambient continues indefinitely
```

### Final Echo Motif (Iteration 7)

In both variants, the echo motif plays one last time. It is:

- 10% volume (barely audible)
- Single distant tone (stripped back from the harmonized iteration 6)
- Panned slightly to the right (toward the viewport)
- No reverb tail — it ends cleanly
- The implication: the contact is still out there. Very far. Still transmitting at the harmonic.

The game holds on the viewport indefinitely after the motif. No timeout. The player exits when they are ready. A credits overlay appears after 30 seconds of inactivity (translucent, does not cover the viewport).

---

## 8. NAVIGATION UI

### Minimap Sidebar

A persistent sidebar on the left edge of the screen showing the 5-deck structure:

```
Width: 40px
Height: 200px (40px per deck)
Position: Left edge, vertically centered

+------+
| RED  |  Deck 1 — Command
+------+
| YLW  |  Deck 2 — Operations
+------+
| GRN  |  Deck 3 — Crew
+------+
| BLU  |  Deck 4 — Support
+------+
| ORG  |  Deck 5 — Engineering
+------+

Each strip: division primary color at 60% opacity
Current deck: 100% opacity with white border
Player dot: 6px white circle on the current deck strip
```

### Room Navigation Header

A horizontal bar at the top of the screen showing the current room.

```
Height: 36px
Background: Division dark background color
Left: Deck name (Display font, Medium 500, 14px, division accent color)
Center: Room name (Display font, Bold 700, 18px, #E0E0E0)
Right: Puzzle progress (e.g., "3/6 SOLVED" in monospace, Regular 400, 12px)
Bottom border: 1px solid division accent color at 30%
```

### Turbolift Interface

When the player enters the turbolift, the screen transitions to the turbolift interior:

```
Background: #0A0A0F
Layout: Centered control panel

Five deck buttons in a vertical column:
  Each button: 200px wide, 40px tall
  Label: "DECK 1 — COMMAND" etc.
  Color: Division primary color
  Border: 2px solid division accent color
  Locked decks: Greyed out (#333333), lock icon overlay
  Current deck: White border highlight

Status display above buttons:
  "TURBOLIFT — SELECT DESTINATION"
  Font: Primary monospace, Bold 700, 14px

Transit animation:
  Button presses, then 1.5s per deck traveled
  A simple vertical motion indicator
  Ambient audio crossfades between deck ambients
```

---

## 9. PROGRESS TRACKING

### Puzzle Progress Display

Visible when the player opens the commission briefing (accessible from any turbolift or the Ready Room console).

```
COMMISSION ORDER #2247 — PROGRESS
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━

ROUND 1: TELEMETRY                    [####--] 4/6
  R1-01 Carrier Isolation         [x] SOLVED
  R1-02 Phase Lock                [x] SOLVED
  R1-03 Null Zone                 [x] SOLVED
  R1-04 Orbit Classification      [x] SOLVED
  R1-05 Contact Lock              [ ] AVAILABLE
  R1-06 Signal Fingerprint        [ ] AVAILABLE
  R1-META Contact                 [!] LOCKED (need 4/6)

ROUND 2: SYSTEMS LOG                  [------] 0/6
  (Locked until Round 1 meta solved)

ROUND 3: CREW RECORD                  [------] 0/5
  (Locked until Round 2 meta solved)

FINAL: THE COMMISSION                 [------] LOCKED
  (Locked until all three round metas solved)
```

**Visual:**
- `[x]` = green checkmark icon (`icon-solved.png`)
- `[ ]` = empty box icon (`icon-unsolved.png`)
- `[!]` = amber lock icon (`icon-lock.png`)
- Progress bars: filled segments use round accent color, empty segments use `#333333`
- Threshold marker: a small white tick on the progress bar at the meta unlock point (4/6, 4/6, 3/5)

---

## 10. COUNTDOWN TIMER

The commission countdown appears in the top-right corner of every screen after Scene 2 (the commission briefing).

```
Format: "ORDER #2247 EXPIRES: HH:MM:SS"
Font: Primary monospace, Light 300, 10px
Color: #999999 (normal) / #FF4444 (under 12 hours) / #FF0000 pulsing (under 1 hour)
Position: Top-right corner, 8px margin
Background: none (overlaid on the scene)
```

The countdown is cosmetic — it does not actually end the game. It creates narrative urgency without mechanical pressure. If the countdown reaches zero, it displays "EXPIRED" in red but gameplay continues normally. The sealed order to purge backup logs is the in-fiction deadline. The solver can take as long as they need.

---

## 11. LOADING AND TITLE

### Title Screen

```
Background: #000000 (true black)
Stars: Subtle static star field (same as epilogue viewport)

Title: "DEAD RECKONING"
  Font: Display font, Black 900, 48px
  Color: #FFFFFF
  Position: Centered, upper third
  Letter spacing: 2px

Subtitle: "Incident Reconstruction Commission"
  Font: Primary monospace, Regular 400, 14px
  Color: #999999
  Position: Below title, 16px gap

Version: "Commission Order #2247"
  Font: Primary monospace, Light 300, 10px
  Color: #555555
  Position: Below subtitle, 24px gap

Start button: "BEGIN COMMISSION"
  Font: Display font, Bold 700, 18px
  Color: #DAA520 (amber)
  Border: 1px solid #DAA520
  Hover: fill to #DAA520 at 20% opacity
  Position: Centered, lower third

Audio: Deep space ambient. No music. Just the void.
After 5 seconds of idle, the echo motif plays once
at 5% volume — barely perceptible.
```

### Loading Screen

```
Background: #0A0A0F
Text: "INITIALIZING SHIP SYSTEMS..."
  Font: Primary monospace, Regular 400, 14px
  Color: #999999
Progress: Simple horizontal bar
  Width: 300px, Height: 4px
  Background: #222233
  Fill: #00CC44 (green), left to right
  No percentage text — just the bar
```
