# Site Spec -- THE GRAND LARCENY

## Overview

The website is answer submission only. No puzzle content lives on the site. Teams solve entirely from the physical dossier. The site exists for one purpose: accepting and validating the final answer.

---

## Pages

### 1. Landing Page (`index.html`)

**Purpose:** Atmosphere and entry point. Links to the answer submission form.

**Content:**
- "THE GRAND LARCENY" -- title (Georgia serif, 1.8rem, letter-spacing 4px)
- "CASE No. 47-1011-GL" -- subtitle (dark red, 0.85rem)
- Divider line
- Tagline (Georgia italic):
  "A painting was stolen from The Grand Hotel.
   Four suspects. Four pieces of evidence.
   The trail is going cold."
- Divider line
- Instruction: "All evidence is in your dossier. When you have determined THE METHOD, submit below."
- [SUBMIT ANSWER] button -- links to `submit.html`
- Footer: "City Police Department -- Investigative Division / Confidential"

**Design:**
- Background: #0d0d0d (near-black)
- Text: #c9c2a5 (warm parchment/brass tone)
- Accent: #8B0000 (dark red for case number)
- Max-width: 600px, centered
- Minimal -- just atmosphere and a button
- Button: outlined (border matches text color), uppercase, letter-spacing

**Status:** BUILT (`delivery/site/index.html`)

---

### 2. Submit Page (`submit.html`)

**Purpose:** Answer submission form. Teams enter their case number (team identifier) and their answer.

**Content:**
- Same visual style as landing page
- "SUBMIT YOUR FINDINGS" -- header
- Form fields:
  - **Case ID / Team Name:** text input (this identifies the team)
  - **THE METHOD:** text input, uppercase, 4 characters max (auto-uppercase)
  - [SUBMIT] button
- On submit: validate that THE METHOD field is not empty, display confirmation or rejection

**Behavior:**

Option A -- Static (no backend):
- On submit, compare the entered answer against a hardcoded hash
- If correct: display "CASE CLOSED. The canvas was [answer] from its frame. Well done, detective."
- If incorrect: display "That is not the method. Review your evidence." Allow retry.
- No server required. All validation in client-side JavaScript.
- The answer hash is stored as a SHA-256 of the uppercase answer string, so the plaintext answer is not in the source code.

Option B -- Server-backed (if running a live event):
- Form POSTs to a simple backend (Google Form, Airtable, or custom)
- Backend logs: team name, answer, timestamp
- Response: correct/incorrect
- Allows the organizer to monitor submissions in real time

**Recommended:** Option A for simplicity. Option B if tracking team progress matters.

---

### 3. Confirmation / Case Closed (inline or separate page)

Shown after correct answer submission:

```
CASE CLOSED

The canvas was [ANSWER] from its frame.

The thief entered through the roof, descended via the service
elevator, cut the painting from the frame in the Sargent Gallery,
escaped through the connecting door to the linen closet, and
exited through the basement to the alley.

Ten minutes. One blade. Gone.

                    Det. R. Callahan
                    Case 47-1011-GL -- CLOSED
```

**Design:** Same dark background. Text fades in. The narrative payoff. No further interaction needed.

NOTE: This confirmation reveals the narrative resolution but NOT the specific answer word in plaintext committed to git. The answer display should be injected dynamically from the validation logic, not hardcoded in the HTML source.

---

## Design System

| Element | Value |
|---------|-------|
| Background | #0d0d0d |
| Primary text | #c9c2a5 |
| Secondary text | #a09880 |
| Accent (case number, errors) | #8B0000 |
| Muted text (footer) | #555555 |
| Font (body) | Courier New, monospace |
| Font (headers, tagline) | Georgia, serif |
| Max content width | 600px |
| Button style | Outlined, no fill, text-color border, hover inverts to filled |

The site aesthetic is "dark office at night" -- the noir counterpart to the bright institutional documents in the dossier. The dossier is daytime bureaucracy; the site is late-night detective work.

---

## Technical Requirements

- Static HTML/CSS/JS. No framework needed.
- Hosted on any static host (GitHub Pages, Netlify, Vercel, S3, or local file).
- Mobile-responsive (teams may use phones to submit).
- No cookies, no tracking beyond answer submission.
- Answer validation does not reveal the answer in page source (use hash comparison).

---

## File Index

| File | Status | Description |
|------|--------|-------------|
| `index.html` | BUILT | Landing page with atmosphere and submit button |
| `submit.html` | TO BUILD | Answer submission form |
| `style.css` | Optional | Shared styles (currently inline in index.html) |

---

## What the Site Does NOT Do

- No puzzle content. No clue text. No images of documents.
- No hint system (hints are handled in-person by staff).
- No leaderboard (optional future addition).
- No team registration (teams are identified by name at check-in).
- No timer (the 90-120 minute target is managed by event staff, not the site).
