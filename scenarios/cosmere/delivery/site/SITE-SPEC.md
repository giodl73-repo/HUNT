# THE SIXTEENTH SHARD — Site Architecture

## Overview

Static HTML/CSS/JS site. Deployable to GitHub Pages, Netlify, or any static host. No backend. Answer validation via SHA-256 hashes computed client-side. Aesthetic: a fan-archive countdown site — warm, worn, slightly obsessive. Not slick. Real. The kind of site someone built in their spare time because the Cosmere matters to them, and now the server is shutting down.

---

## Site Map

```
delivery/site/
├── index.html                   ← homepage: Hoid's opening, 3 round links, countdown
├── rounds/
│   ├── scadrial.html            ← Round 1: The Metal Arts (12 puzzle links)
│   ├── roshar.html              ← Round 2: Words of Radiance (12 puzzle links)
│   └── cosmere.html             ← Round 3: The Shattering (12 puzzle links)
├── puzzles/
│   ├── P01-burning-through.html
│   ├── P02-alloy-partners.html
│   ├── ...                      (36 puzzle pages)
│   └── P36-all-roads-lead-to-hoid.html
├── meta/
│   ├── the-well.html            ← Round 1 meta: THE WELL
│   ├── the-oaths.html           ← Round 2 meta: THE OATHS
│   ├── adonalsium.html          ← Round 3 meta: ADONALSIUM
│   └── the-seventeenth-shard.html  ← Super-meta (final page)
├── submit.html                  ← universal answer submission form
├── assets/
│   ├── theme.css                ← all site styling (3 round themes + shared)
│   ├── hunt.js                  ← answer validation, unlock logic, hash checking
│   ├── fonts/                   ← web fonts (Cinzel, Alegreya, monospace fallback)
│   └── images/
│       ├── logo.svg             ← THE SIXTEENTH SHARD wordmark
│       ├── bg-scadrial.jpg      ← mist/ash background
│       ├── bg-roshar.jpg        ← highstorm/stone background
│       └── bg-cosmere.jpg       ← void/silver background
├── data/
│   ├── puzzles.json             ← puzzle metadata, answer hashes, unlock state
│   ├── hashes.json              ← SHA-256 hashes of all 40 answers (ROT13 decoded first)
│   └── progress.json            ← team progress (local storage mirror)
└── admin/
    ├── admin.html               ← hunt control panel
    └── admin.js                 ← unlock management, progress reset
```

---

## Page Specifications

### Homepage (`index.html`)

**Aesthetic:** Near-black background (#0a0a0f). A single column of text, centered, with generous vertical rhythm. The feel of a letter from someone who has been waiting. A counter in the corner ticking toward a date that has already passed.

**Content:**

1. **Site header:** THE SIXTEENTH SHARD — rendered in Cinzel, silver (#c0c0c0), small caps. Below it, in smaller Alegreya italic: "A Cosmere Puzzle Archive."

2. **Countdown banner:** "This archive closes on [date]. The puzzles remain." Displayed in monospace, amber (#d4a843), with a CSS animation that makes it pulse slowly. The date is in the past. The archive is already supposed to be gone. It is not.

3. **Hoid's opening monologue:** Dry, present tense, oblique. The voice of someone who has waited a very long time and is not surprised that a visitor has arrived.

   > One has maintained this archive for longer than most civilizations endure. The puzzles were placed across three worlds by someone with too much time and a very particular sense of humor. They encode a message. The message has a single word at its center.
   >
   > Scadrial's metals burn with names that have letters in useful positions. Roshar's knights swear oaths with measurable depth. The Cosmere's Shards number sixteen, but the count is never quite right.
   >
   > A careful Worldhopper might start with the metals. They have always been the most honest.
   >
   > The archive contains thirty-six puzzles, three round revelations, and one final question that has taken an exceptionally long time to phrase correctly. The answer is a single word. One already knows it. The trick is proving that one knows it.

4. **Three round links:** Displayed as three cards in a horizontal row (stacking vertically on mobile). Each card has:
   - Round number and name
   - World name
   - Puzzle count ("12 puzzles")
   - A one-line Hoid quote
   - Visual treatment matching the round's aesthetic (see THEME.md)

   | Round | Card Title | Hoid Quote |
   |-------|-----------|------------|
   | 1 | SCADRIAL: The Metal Arts | "The metals have names. The names have letters." |
   | 2 | ROSHAR: Words of Radiance | "The depth of an oath is not measured in volume." |
   | 3 | THE COSMERE: The Shattering | "Sixteen pieces. Four of them have something to say." |

5. **Footer:** "Maintained by a fool who refused a Shard and has been walking ever since." No copyright. No attribution. The site does not explain who made it.

---

### Round Pages (`rounds/scadrial.html`, `rounds/roshar.html`, `rounds/cosmere.html`)

Each round page displays its 12 puzzle links plus the round meta (locked until all 12 feeders solved).

**Common layout:**

```
[Round header — name, world, Hoid quote]
[Module A/C/E — 6 puzzle cards]
[Module B/D/F — 6 puzzle cards]
[Round Meta — locked/unlocked]
```

**Puzzle cards** show:
- Puzzle ID and title
- Difficulty stars
- Status indicator: LOCKED / OPEN / SOLVED
- One-line teaser from the puzzle's flavor text

**Unlock model:** All 12 puzzles in a round are open from the start. Round meta unlocks after all 12 feeder answers are submitted correctly. Super-meta unlocks after all 3 round metas are solved.

#### Round 1: SCADRIAL

**Background:** Dark charcoal (#1a1a1f) with a subtle mist-overlay animation (CSS gradient that drifts slowly). Ash-grey text (#b8b0a8). Amber/gold accents (#d4a843) for links and titles. The metals are warm.

**Header:** "SCADRIAL: The Metal Arts" in Cinzel, amber. Below: "Twelve puzzles. Sixteen metals. The table has always been the key."

**Module A (Allomancy):** P01-P06 listed with metal assignments visible as subtle badges.
**Module B (Scadrial History):** P07-P12 listed.
**Round Meta:** THE WELL — locked card reads "The Pulling metals are generous. Solve all twelve to hear what they have been saying."

#### Round 2: ROSHAR

**Background:** Deep blue-black (#0d1520) with stormlight cyan (#00e5ff) accents. Stone-texture border (CSS pattern). The feel of Urithiru's corridors — angular, ancient, luminous in the cracks.

**Header:** "ROSHAR: Words of Radiance" in Cinzel, stormlight cyan. Below: "Twelve puzzles. Ten Orders. The Oaths go deeper than most suspect."

**Module C (Knights Radiant):** P13-P18 listed with Order names as badges.
**Module D (Roshar World):** P19-P24 listed.

**Knowledge Guide:** A brief note below the round header, in Hoid's voice: "Round 2 draws heavily from The Stormlight Archive. If one has not read it, one would be wise to pair with a teammate who has. The reference sheets carry the facts. The stories carry the rest."

**Round Meta:** THE OATHS — locked card reads "Only those who have sworn deeply enough can speak here. Solve all twelve."

#### Round 3: THE COSMERE

**Background:** Near-black (#050508). Silver (#c0c0c0) text. Minimal decoration. The void between worlds. Sparse.

**Header:** "THE COSMERE: The Shattering" in Cinzel, silver. Below: "Twelve puzzles. Sixteen Shards. Four of them remain silent."

**Module E (The 16 Shards):** P25-P30 listed with Shard associations as badges.
**Module F (Hoid & Worldhoppers):** P31-P36 listed.
**Round Meta:** ADONALSIUM — locked card reads "Sixteen Intents. Twelve represented. The silence of the absent four is the message."

---

### Puzzle Pages (`puzzles/P01-burning-through.html` through `puzzles/P36-all-roads-lead-to-hoid.html`)

Each puzzle page contains:

1. **Breadcrumb navigation:** THE SIXTEENTH SHARD > SCADRIAL > P01: Burning Through
2. **Puzzle header:** Title, difficulty stars, module badge, round badge
3. **Flavor text:** Hoid's intro paragraph from the puzzle file
4. **Puzzle content:** The full puzzle (tables, grids, clues, text) from `puzzles/P##-name.md`, converted to HTML
5. **Extraction note:** The puzzle's extraction instructions
6. **Answer submission form:**
   - Text input field (uppercase, stripped to alpha only)
   - Submit button
   - SHA-256 validation: on submit, the input is uppercased, hashed, and compared to the stored hash in `data/hashes.json`
   - Correct: green flash, "Confirmed." stored in localStorage, puzzle marked SOLVED
   - Incorrect: brief red flash, no message, no feedback on closeness
   - ROT13 note: answers stored as ROT13 in source files; hashes computed from the PLAINTEXT (decoded) answer
7. **Closing flavor:** Hoid's closing line from the puzzle file (shown only after solve)
8. **Footer navigation:** Previous puzzle / Round page / Next puzzle

**Visual treatment:** Each puzzle page inherits the round's color scheme and typography. Tables and grids are styled with CSS matching the round aesthetic.

---

### Meta Pages (`meta/the-well.html`, `meta/the-oaths.html`, `meta/adonalsium.html`)

Each meta page is unlocked when all 12 feeder puzzles in the round are solved. Content drawn from the `meta/META-*.md` files.

**Layout:**
1. **Meta title and Hoid's framing quote** (from the "Presentation" section of each meta file)
2. **The extraction workspace:**
   - For THE WELL: The 4x4 Allomancy grid, pre-labeled. 12 answer input fields. Quadrant-depth extraction instructions.
   - For THE OATHS: The Double Eye ring diagram. 10 Order positions. Ideal-count table. Oath depth extraction instructions.
   - For ADONALSIUM: The full 16-Shard list. 12 identification fields. Missing-Shard anagram instructions.
3. **Meta answer input:** Single text field with SHA-256 validation
4. **Solve confirmation:** Hoid's closing quote (from the "When solved" line of each meta file)

---

### Super-Meta Page (`meta/the-seventeenth-shard.html`)

Unlocks after all 3 round metas are solved. The final page.

**Layout:**
1. Hoid's opening: "Sixteen pieces. Sixteen names. Sixteen keepers. And one fool who refused a piece and has spent eternity looking for something else entirely."
2. Three input boxes (pre-filled with round meta answers if solved)
3. The Realms cycle extraction diagram: three rows, five columns, with Physical/Cognitive/Spiritual cycle highlighted
4. The prompt: "What did the fool seek?"
5. Final answer input (5 letters, SHA-256 validated)
6. On solve: Hoid's closing statement from SUPER-META.md
7. A final line, shown only once, after a 3-second delay: "The archive is closed. One thanks the visitor for the company."

---

## Answer Submission System

### SHA-256 Hash Validation

All answer checking happens client-side. No backend needed.

**Process:**
1. Solver types an answer
2. JavaScript uppercases it, strips non-alpha characters
3. Computes SHA-256 hash of the normalized string
4. Compares against the stored hash in `data/hashes.json`
5. Match = correct. Stored in localStorage. Puzzle status updated.

**Hash generation (for `data/hashes.json`):**

All 40 answers (36 feeders + 3 round metas + 1 super-meta) are stored as ROT13 in `meta/cosmere-answers.md`. To generate hashes:
1. Decode each ROT13 answer to plaintext
2. Uppercase the plaintext
3. Compute SHA-256
4. Store as `{ "P01": "<hash>", "P02": "<hash>", ... }`

This ensures no plaintext answers appear anywhere in the deployed site files.

**data/hashes.json structure:**

```json
{
  "P01": "sha256_of_UPRISE",
  "P02": "sha256_of_FORGED",
  "...": "...",
  "P36": "sha256_of_THREAD",
  "META-I": "sha256_of_UNITE",
  "META-II": "sha256_of_ENVOY",
  "META-III": "sha256_of_AMID",
  "SUPER": "sha256_of_UNITY"
}
```

### Progress Tracking

Solver progress stored in `localStorage` under key `sixteenth-shard-progress`:

```json
{
  "solved": ["P01", "P03", "P17"],
  "meta_solved": [],
  "super_solved": false,
  "last_updated": "2026-03-15T14:22:00Z"
}
```

Round meta pages check `localStorage` for all 12 feeder solves before revealing content. Super-meta checks for all 3 round meta solves.

---

## Unlock Logic

```
ALL OPEN:     P01-P12 (Round 1), P13-P24 (Round 2), P25-P36 (Round 3)
ROUND-GATED:  META-I unlocks after P01-P12 all solved
              META-II unlocks after P13-P24 all solved
              META-III unlocks after P25-P36 all solved
FINAL-GATED:  SUPER unlocks after META-I + META-II + META-III all solved
```

All 36 feeder puzzles are available from the start. The solver chooses their path. The three rounds can be worked in any order. Only the metas and super-meta are gated.

---

## Technical Notes

### Fonts

- **Cinzel** (Google Fonts): headers, titles, the wordmark. A Roman-inspired serif. Sanderson-era fantasy typography — clean, old-world, not Gothic.
- **Alegreya** (Google Fonts): body text, Hoid's voice, puzzle content. A humanist serif with warmth. Readable at 11pt+.
- **JetBrains Mono** (or system monospace): extraction worksheets, answer blanks, the countdown timer. Functional.

### Responsive Design

- Desktop: 3-column round card layout, wide puzzle content area
- Tablet: 2-column round cards, full-width puzzle content
- Mobile (375px+): single column, stacked cards, full-width everything
- All puzzle tables scroll horizontally on small screens

### Performance

- No JavaScript frameworks. Vanilla JS only.
- CSS custom properties for round-specific theming (switch via body class)
- All images optimized, SVG preferred
- Total site weight target: under 2MB excluding images

### Accessibility

- Semantic HTML (header, nav, main, article, footer)
- ARIA labels on interactive elements
- Color contrast: WCAG AA minimum on all text
- Keyboard navigation for all forms
- Answer input: autocomplete off, spellcheck off

---

## Deployment

### Recommended: Netlify (drag-and-drop)

1. Build the `delivery/site/` directory
2. Drag onto Netlify deploy area
3. Custom domain: sixteenthshard.net (or similar)
4. HTTPS automatic

### Alternative: GitHub Pages

```bash
cd delivery/site/
git init
git add .
git commit -m "THE SIXTEENTH SHARD"
git remote add origin https://github.com/[user]/sixteenth-shard-site
git push -u origin main
```

### Local (for testing or LAN events)

```bash
cd delivery/site/
python -m http.server 8080
```

---

## Pre-Launch Checklist

```
SITE TEST — THE SIXTEENTH SHARD

Navigation
  [ ] Homepage loads, Hoid's monologue renders correctly
  [ ] All 3 round pages load
  [ ] All 36 puzzle pages load with correct content
  [ ] All 4 meta pages exist (3 round + 1 super)
  [ ] Breadcrumb navigation works on every page

Answer Submission
  [ ] Correct answer → green flash, stored in localStorage
  [ ] Incorrect answer → red flash, no info leak
  [ ] Answers are case-insensitive
  [ ] Answers strip spaces and punctuation

Unlock Logic
  [ ] All 36 feeders are accessible from the start
  [ ] META-I is locked until P01-P12 all solved
  [ ] META-II is locked until P13-P24 all solved
  [ ] META-III is locked until P25-P36 all solved
  [ ] Super-meta locked until all 3 round metas solved
  [ ] Solving a round meta unlocks the super-meta check

Content
  [ ] No plaintext answers anywhere in deployed HTML/JS/JSON
  [ ] All 40 SHA-256 hashes are correct
  [ ] Hoid's voice is consistent — no exclamation marks, no "you" address
  [ ] All puzzle tables/grids render correctly
  [ ] HTML-comment verification sections stripped from P31-P36

Mobile
  [ ] Site readable at 375px width
  [ ] Answer submission works on mobile keyboards
  [ ] Puzzle tables scroll horizontally

Aesthetic
  [ ] Scadrial pages: mist/ash/amber
  [ ] Roshar pages: storm/cyan/stone
  [ ] Cosmere pages: void/silver/minimal
  [ ] Homepage: countdown banner, Hoid's monologue, three round cards
  [ ] Fonts load correctly (Cinzel, Alegreya, monospace)
```
