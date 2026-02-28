# IRONFALL — Visual Theme Guide

## The Aesthetic: 1997 Web, 2026 Nostalgia

The IRONFALL archive looks like a website built by a devoted fan community in the late 1990s and never redesigned. It is not a parody of 90s web design. It IS 90s web design, with all its limitations treated as features. The warmth comes from the obvious care behind the rough edges.

---

## Core Principles

### 1. Built by hand, not by framework

No CSS grid. No flexbox. No responsive breakpoints. Tables for layout. Inline styles where "necessary." Background colors set with `bgcolor`. The HTML is the design. This is a site where someone learned HTML from a "teach yourself in 24 hours" book and built something they were proud of.

### 2. The countdown is emotional, not technical

The 24-hour countdown timer is the narrative spine. It appears on every page. It ticks down. The solver knows that this world — these wiki pages, these forum posts, these thousands of hours of community work — is about to disappear. The urgency is not "solve faster." The urgency is "this mattered."

### 3. Pixel fonts and bold colors

The visual language is 16-bit console gaming translated to web. Pixel-art borders. Nintendo-era color palettes. The same contrast ratios that made SNES sprites readable on a CRT. Text is sharp. Colors are saturated. There is no gradient that was not drawn pixel by pixel.

### 4. The warmth of something about to be lost

This is the hardest part to get right. The archive is not dying because nobody cares. It is dying because the person who maintained it can no longer afford the hosting. The community tried to save it. They could not. The last posts are grateful, not angry. "Thank you for 29 years." The solver should feel the weight of that.

---

## Color Palette

### Primary (Dark Pages — Homepage, Forum, Secrets)

| Role | Color | Hex | Notes |
|------|-------|-----|-------|
| Background (tile) | Deep navy | `#0A0A2E` | Dark but not pure black — has depth |
| Text (body) | Terminal green | `#00FF00` | Classic CRT green. Used sparingly. |
| Text (body alt) | Light gray | `#C0C0C0` | For longer passages. Easier on the eyes. |
| Headers | Gold | `#FFCC00` | Achievement-badge gold. Warm. |
| Links | Classic blue | `#0066FF` | Underlined, always. |
| Visited links | Purple | `#660099` | The original web convention. |
| Borders | Silver/gray | `#C0C0C0` | Beveled (outset) for that 3D button look. |
| Accent | 90s orange | `#FF6600` | Warnings, countdown, important notices. |
| Error/urgency | Red | `#FF0000` | Shutdown warnings. Countdown when < 1 hour. |

### Secondary (Light Pages — Wiki)

| Role | Color | Hex | Notes |
|------|-------|-----|-------|
| Background | Parchment | `#F5F0E0` | Warm off-white. Wiki/reference feel. |
| Text | Dark gray | `#333333` | Standard readability. |
| Headers | Dark navy | `#0A0A2E` | Contrast against light background. |
| Table borders | Black | `#000000` | Solid 1px. Clean data display. |
| Code blocks | Light gray bg | `#E8E8E8` | Monospace content areas. |
| Stat highlights | Red | `#CC0000` | Missing values, unknowns. |

---

## Typography

### Font Stack

```css
/* Body text */
font-family: 'Courier New', Courier, monospace;
font-size: 14px;
line-height: 1.4;

/* Headers */
font-family: 'Arial Black', 'Impact', Arial, sans-serif;
font-size: 18px; /* H1: 24px, H2: 18px, H3: 16px */
text-transform: uppercase;
letter-spacing: 2px;

/* Pixel accent (where available) */
font-family: 'Press Start 2P', 'Courier New', monospace;
font-size: 12px;
```

### Rules
- No italic text. The 90s web did not do italics well.
- Bold for emphasis. Underline for links only.
- ALL CAPS for headers and navigation. Mixed case for body.
- Monospace for all data: stat blocks, hex dumps, save files, battle logs.

---

## Layout

### Page Width
640 pixels. Centered. No exceptions. The content lives in a single column inside a table with a fixed width.

### Structure
```
+--[ 640px ]--------------------------------------------+
|  HEADER BAR: [ HOME ] [ WIKI ] [ FORUM ] [ SECRETS ]  |
+--------------------------------------------------------+
|  PAGE TITLE (large, gold, centered)                    |
+--------------------------------------------------------+
|                                                        |
|  CONTENT AREA                                          |
|  (tables, text, ASCII art, data blocks)                |
|                                                        |
+--------------------------------------------------------+
|  ANSWER SUBMISSION (text input + submit button)        |
+--------------------------------------------------------+
|  FOOTER: countdown + visitor counter                   |
+--------------------------------------------------------+
```

### Borders
- **Outer frame:** 3px beveled border (`border-style: outset`) in silver/gray.
- **Inner panels:** 1px solid borders. Double-line (`border-style: double`) for important sections.
- **Data tables:** Solid 1px black borders with `cellpadding="4"` and `cellspacing="1"`.

---

## Decorative Elements

### Tiled Backgrounds
- Homepage: 16x16 dark pixel tile (subtle diagonal pattern in navy/black)
- Wiki: 8x8 light texture tile (paper grain)
- Forum: 16x16 charcoal weave
- Secrets: No tile — solid black with CSS scanline overlay

### Horizontal Rules
```html
<hr noshade size="2" color="#C0C0C0">
```
Where visual flair is needed: a pixel-art divider line (image or CSS).

### Marquee (Homepage only)
```html
<marquee behavior="scroll" direction="left" scrollamount="2">
  IRONFALL-ARCHIVE.NET — 29 YEARS OF COMMUNITY — SHUTTING DOWN IN [TIMER]
</marquee>
```
One marquee. On the homepage. It scrolls slowly. This is not ironic. This is how it was.

### Under Construction GIF (About page only)
A small animated "UNDER CONSTRUCTION" badge. Because the about page was never finished. Because the archive is shutting down before it could be.

### Visitor Counter
```
VISITORS: 847,293
```
Displayed as a 7-segment LED counter graphic (or styled with monospace + dark background). The number is fixed. It does not increment. But it represents 29 years of people who cared about this game.

---

## Interactive Elements

### Answer Submission Box
```
+----------------------------------------------+
|  ARCHIVE VERIFICATION                        |
|  Enter your answer: [_________________]      |
|  [ SUBMIT ]                                  |
|                                              |
|  Status: AWAITING INPUT                      |
+----------------------------------------------+
```
- Input field: monospace, uppercase forced, 20 chars max.
- Submit button: beveled gray, `cursor: pointer`.
- Status line: "AWAITING INPUT" (red) / "VERIFIED" (green) / "INCORRECT — TRY AGAIN" (red flash).

### Countdown Timer
```
SHUTTING DOWN IN 23:59:47
```
- Font: large monospace, `#FF6600` (orange).
- Updates every second via JavaScript.
- At < 1 hour: color changes to `#FF0000` (red).
- At 00:00:00: "THE ARCHIVE IS CLOSED. THANK YOU FOR EVERYTHING." in white.

---

## Emotional Design Notes

**What the solver should feel on first load:**

1. Recognition — "This looks like a real old website."
2. Curiosity — "What is IRONFALL? What is this archive?"
3. Urgency — "24 hours. The countdown is ticking."
4. Warmth — "Someone built this by hand. Someone cared."

**What the solver should feel at the end:**

1. The Konami Code recognition — "Wait. I know this sequence."
2. The twist — "But the last four buttons are different. Morimoto changed it."
3. The farewell — "Thank you for playing. -- K.M."
4. The loss — the countdown has reached zero. The archive is gone. But the code was found in time.

---

## Reference: 90s Web Sites for Tone

- Space Jam (1996) — the canonical example
- GeoCities-era personal pages — tiled backgrounds, visitor counters
- GameFAQs (early era) — monospace text, community-contributed guides
- phpBB forums — the forum post aesthetic
- Angelfire/Tripod hosting — the "under construction" energy

The IRONFALL archive is closer to GameFAQs than to Space Jam. It is a reference site, not a promotional one. The design serves the data. But the data serves the community. And the community is saying goodbye.
