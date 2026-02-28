# IRONFALL Archive — Site Specification

## Overview

**Domain (fictional):** ironfall-archive.net
**Purpose:** Primary puzzle delivery vehicle. The solver experiences the hunt as a fan archive about to shut down.
**Aesthetic:** Late-90s web — Geocities-era. Tiled backgrounds, table layouts, bold system fonts, visitor counters, marquee scrolls, and the particular warmth of a community site built by hand.

---

## Site Structure

```
ironfall-archive.net/
├── index.html               ← Homepage. Countdown timer. Navigation.
├── wiki/                    ← Puzzle delivery (Act I)
│   ├── bestiary             → P01
│   ├── crafting             → P02
│   ├── saves                → P03
│   ├── worldmap             → P04
│   └── battle               → P05
├── forum/                   ← Puzzle delivery (Act II) + narrative flavor
│   ├── sightings            → P06
│   ├── endgame-build        → P07
│   ├── 100-completion       → P08
│   ├── speedrun             → P09
│   └── anomaly              → P10
├── secrets/                 ← Meta delivery
│   ├── archive-index        → META-I (Top Edited Pages)
│   ├── game-secrets         → META-II (Secrets Screen)
│   └── true-ending          → SUPER-META (DataMiner_X's post)
└── about.html               ← Archive history, credits, shutdown notice
```

---

## Page Types

### Wiki Pages (Act I Puzzles)

**Look:** White background panel on a dark tiled page. Monospace stat blocks. Revision history in the page footer. "Last edited by [username] on [date]" timestamps. Edit count visible (feeds META-I).

**Structure per page:**
1. Page title ("Bestiary v3.2 — Complete Stats")
2. Revision bar: "Last edited by DarkKnight_97 on March 14, 1998 | Edits: 24"
3. Puzzle content (data tables, community notes, working space)
4. Category tags at bottom ("Filed under: Bestiary, Community Data")
5. Archive watermark: "IRONFALL-ARCHIVE.NET — Est. 1997"

**Edit counts (feed META-I extraction):**
| Page | Puzzle | Edits |
|------|--------|-------|
| Bestiary | P01 | 6 |
| Crafting | P02 | 17 |
| Saves | P03 | 24 |
| World Map | P04 | 9 |
| Battle | P05 | 12 |

### Forum Threads (Act II Puzzles)

**Look:** Classic phpBB/vBulletin forum aesthetic. Thread title, post author with avatar placeholder, post timestamp, signature lines. Dark background, light text panels.

**Structure per thread:**
1. Thread title ("Enemy Sightings — Unconfirmed")
2. Series of posts, each with:
   - Username + join date + post count
   - Timestamp
   - Post body (puzzle content)
   - Signature (flavor text)
3. Thread footer: "You must be a registered member to reply."

### Meta Pages (Secrets Section)

**Look:** Restricted section. Dark background. Monospace terminal-style text. No revision history — these pages feel discovered, not curated.

**META-I ("Archive Index"):** The top-edited-pages table. Looks administrative. The edit counts ARE the extraction mechanism.

**META-II ("Game Secrets"):** A mock game screen rendered in ASCII art. Five secret names with discovery markers (highlighted letters). Looks like datamined output.

**SUPER-META ("True Ending"):** DataMiner_X's final post. Terminal-style frame. Countdown at 00:00:01. The instructions for the 12-button code extraction.

---

## Navigation

**Header bar (every page):**
```
[ HOME ] [ WIKI ] [ FORUM ] [ SECRETS ] [ ABOUT ]
```

**Footer (every page):**
```
IRONFALL-ARCHIVE.NET — Online since 1997 — Shutting down [COUNTDOWN]
Visitors: 847,293 — Thank you for 29 years.
```

---

## Countdown Timer

**Location:** Homepage (large), footer (small on every page).
**Display:** `SHUTTING DOWN IN 23:59:47` (hours:minutes:seconds).
**Behavior:** Cosmetic only — counts down from 24:00:00 using JavaScript. Does not actually shut anything down. The urgency is narrative, not functional.
**End state:** When timer reaches 00:00:00, the text changes to: `THE ARCHIVE IS CLOSED. THANK YOU FOR EVERYTHING.`

---

## Answer Submission

**Mechanism:** Each puzzle page has an "ANSWER SUBMISSION" box at the bottom — a text input styled as a 90s web form. Styled with beveled borders and a gray submit button.

**Behavior options (implementation-dependent):**
1. **Static (print/offline):** The input box is non-functional. Answers are checked against the answer sheet.
2. **JavaScript (online):** The input checks against a hashed answer. Correct answer reveals a confirmation message ("ARCHIVE ENTRY ACCEPTED") and unlocks the next section.

**Visual cues on correct submission:** The page's "status" light changes from red to green. A small "VERIFIED" stamp appears next to the page title.

---

## Asset Requirements

### Backgrounds
- **Homepage:** Dark tiled background (pixel pattern, 16x16 tile). Deep navy or dark purple.
- **Wiki pages:** Lighter tiled background. Neutral gray or parchment.
- **Forum pages:** Dark charcoal with subtle pixel pattern.
- **Secrets pages:** Black with faint green scanlines (terminal aesthetic).

### Fonts
- Primary: System monospace (`Courier New`, `monospace`)
- Headers: Bold system sans-serif (`Arial Black`, `Impact`, `sans-serif`)
- Decorative: Pixel font (web-safe fallback to monospace)

### Colors
- Text: `#00FF00` (terminal green) on dark backgrounds, `#333333` on light
- Links: `#0066FF` (classic blue), visited: `#660099`
- Borders: Beveled gray (`#C0C0C0` / `#808080`)
- Accent: `#FF6600` (90s orange), `#FFCC00` (gold for achievements)

### Images (descriptions for asset creation)
- IRONFALL logo: Pixel-art title with iron/fire gradient
- Controller diagram: SNES pad layout (ASCII art, no image needed)
- Achievement badges: 16x16 pixel squares, gold-filled when earned

---

## Responsive Behavior

None. The site is 640px wide, centered. This is 1997. Fixed-width tables. If your screen is too small, you scroll horizontally. That is the authentic experience.

---

## Accessibility Notes

Despite the retro aesthetic, ensure:
- All ASCII art has alt-text descriptions in comments
- Color is not the only indicator (text labels accompany status lights)
- Tab order follows logical reading order
- Screen readers can parse the table structures
