# Site Spec -- WAVELENGTH (WVLG 92.7 FM)

## Overview

The website is the WVLG radio station's final broadcast page. Unlike Grand Larceny (answer-submission only), this site IS part of the experience. It establishes the fiction, hosts all 6 puzzles, and accepts the meta answer. Teams open the site, hear Vee's voice in the text, and work through the sets.

The physical playlist prop is handed out separately. The site and the prop work together.

---

## Pages

### 1. Homepage (`index.html`)

**Purpose:** Establish the fiction. This is the WVLG station page on the night of the final broadcast. It should feel like tuning in to a real radio station's website, circa 2003, updated one last time.

**Content:**
- "WVLG 92.7 FM" -- station call letters (Courier New, bold, large)
- ON AIR indicator -- small red pill/dot, subtly pulsing CSS animation
- "THE LONG WAVE -- Final Broadcast" -- show title (Georgia, italic)
- Station tagline: "Independent radio since 1983. Signing off tonight."
- Divider (VU meter motif -- thin segmented bar)
- Vee's opening line (Georgia italic): "Forty years. Six sets. One last playlist. You know how this works -- I play, you listen. But tonight, listen closer."
- Navigation to 6 sets:
  ```
  SET 1: Side A
  SET 2: Notation
  SET 3: Deep Cuts
  SET 4: Chart Toppers
  SET 5: Between the Lines
  SET 6: Name That Band
  ```
- Below the sets: "When you have decoded all six sets, use the playlist card."
- Link to answer submission: [SUBMIT THE FINAL MESSAGE]
- Footer: "WVLG 92.7 FM / The Long Wave / Est. 1983 / Signing off 2023"

**Design:**
- Background: #1a1210 (booth dark)
- Text: #c9b88a (faded cream)
- Accent: #c43c2d (dial red, for ON AIR indicator only)
- Station name: #e8c36a (warm amber)
- Max-width: 640px, centered
- The page should feel like it loads slowly, deliberately. No animations except the ON AIR pulse.

---

### 2. Puzzle Pages (`set1.html` through `set6.html`)

**Purpose:** Each page presents one puzzle (one "set" from Vee's broadcast).

**Content per page:**
- Station header: "WVLG 92.7 -- THE LONG WAVE"
- "SET [N]: [Title]"
- Vee's intro line (Georgia italic, faded cream)
- Horizontal rule
- Full puzzle content (from `puzzles/P[N]-*.md`, solver-facing sections only -- strip author's working notes)
- Workspace table/blanks for solver use (if solving on-screen; optional since teams may print or use paper)
- "Your answer (N letters): _ _ _ _ _ _"
- Navigation: [< Previous Set] [All Sets] [Next Set >]
- Footer

**Design:** Same dark background. Each puzzle page is a single scrollable document. No sidebar. No distractions. The puzzle IS the page.

**Important:** Author's working notes MUST NOT appear on these pages. Strip everything below the "---" that precedes "## Author's Working Notes."

---

### 3. Submit Page (`submit.html`)

**Purpose:** Meta answer submission. Teams enter the 6-letter word they extracted from the diagonal.

**Content:**
- Station header
- "THE FINAL MESSAGE"
- Vee's meta prompt (Georgia italic): "The answer was always in the music. What did it spell?"
- Form:
  - **Team name / Listener ID:** text input
  - **The message:** text input, 6 characters, auto-uppercase
  - [TRANSMIT] button
- On correct: reveal Vee's closing line and a sign-off message
- On incorrect: "That's not it. The signal is still there. Listen again."

**Validation (client-side, static):**
- Hash the meta answer (SHA-256 of uppercase string) and store the hash in JS
- Compare the solver's input against the hash
- On match: display the sign-off panel

**Sign-off panel (shown on correct answer):**
```
SIGNAL RECEIVED.

"The signal was always there. You just had to listen."

                                        -- Vee
                            WVLG 92.7 FM
                         The Long Wave
                      1983 - 2023


  [ON AIR light fades to off]
```

The ON AIR indicator animates from red to dark (CSS transition, 3 seconds). The page goes slightly dimmer. The broadcast is over.

---

## Design System

| Element | Value |
|---------|-------|
| Background | #1a1210 |
| Primary text | #c9b88a |
| Station name | #e8c36a |
| Muted text | #6b6156 |
| Accent (ON AIR) | #c43c2d |
| Font (body/puzzles) | Courier New, monospace |
| Font (Vee's voice, headers) | Georgia, serif |
| Max content width | 640px |
| Button style | Outlined, amber border, hover fills amber with dark text |
| Link style | Faded cream, no underline, underline on hover |

---

## Technical Requirements

- Static HTML/CSS/JS. No framework.
- Hostable on any static platform (GitHub Pages, Netlify, local file).
- Mobile-responsive (teams will use phones and laptops).
- No cookies, no tracking.
- Answer validation via SHA-256 hash comparison (no plaintext in source).
- Puzzle pages strip all author notes -- solver-facing content only.

---

## What the Site Does

- Establishes the WVLG fiction (the final broadcast)
- Presents all 6 puzzles as "sets" in the broadcast
- Provides answer submission for the meta
- Delivers the emotional payoff (Vee's sign-off) on correct answer

## What the Site Does NOT Do

- No audio. Everything is text. The radio is in the reader's head.
- No timer. The broadcast "ends" when they solve the meta.
- No hint system (hints handled in person by staff).
- No leaderboard.
- No registration.

---

## File Index

| File | Status | Description |
|------|--------|-------------|
| `index.html` | BUILT | Station homepage and set navigation |
| `set1.html` - `set6.html` | TO BUILD | Individual puzzle pages |
| `submit.html` | TO BUILD | Meta answer submission |
