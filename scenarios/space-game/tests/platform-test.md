# DEAD RECKONING — Platform Test

**Stage 10 — Platform Test**
**Date:** 2026-02-27

---

## Delivery Structure Verification

```
delivery/
├── THEME.md                          ✓
└── site/
    ├── index.html                    ✓
    ├── puzzles.html                  ✓
    ├── submit.html                   ✓
    ├── standings.html                ✓
    ├── assets/
    │   ├── theme.css                 ✓
    │   └── hunt.js                   ✓
    ├── data/
    │   ├── puzzles.json              ✓  (19 answer hashes populated)
    │   └── standings.json            ✓  (empty, ready for teams)
    ├── admin/
    │   └── admin.html                ✓
    └── puzzles/                      ✓  (19 HTML files)
        ├── R1-01-carrier-wave.html
        ├── R1-02-dead-reckoning.html
        ├── R1-03-contact-report.html
        ├── R1-04-gate-log.html
        ├── R1-05-anomaly-station.html
        ├── R1-META-telemetry-reconstruction.html
        ├── R2-01-load-shedding.html
        ├── R2-02-pathway-trace.html
        ├── R2-03-atmospheric-incident.html
        ├── R2-04-burn-pattern.html
        ├── R2-05-partial-recovery.html
        ├── R2-META-systems-reconstruction.html
        ├── R3-01-vital-signs.html
        ├── R3-02-pattern-integrity.html
        ├── R3-03-access-record.html
        ├── R3-04-repair-queue.html
        ├── R3-05-unauthorized-request.html
        ├── R3-META-crew-reconstruction.html
        └── FINAL-the-commission.html
```

**Total files:** 28 ✓

---

## Answer Hash Verification

All 19 SHA-256 hashes computed from `SHA256(ANSWER.toUpperCase().trim())` and written to `data/puzzles.json`.

| ID | Answer | Hash (first 16 chars) |
|----|--------|-----------------------|
| R1-01 | DECOYS | 2db886d451a4bc33... |
| R1-02 | SHATTERED CARRIER | 92a74110743c10a8... |
| R1-03 | HIERATIC TRIPLET | 2f2ad5138ce7d1ce... |
| R1-04 | HARMONIC ECHO | 87fb16a7945872c7... |
| R1-05 | ABSORPTION SHADOW | 1c8f340d9a6aa5c8... |
| R1-META | PHASE-LOCKED PAIR | 037b22468590c498... |
| R2-01 | FAULT | 7d9f5428a7bbd02d... |
| R2-02 | OFFLINE | 5c3ce8d496e2626b... |
| R2-03 | STANDBY | fefeeeacf4e0c691... |
| R2-04 | ONLINE | ac7e24813998d31d... |
| R2-05 | READY | c2e3ac47f4a32546... |
| R2-META | LOCKOUT | 71929340 3fceabc9... |
| R3-01 | STASIS | 0b6a45ce5d885c9e... |
| R3-02 | BUFFER | 196451e64b086165... |
| R3-03 | FORCED | 6722800d3675fb74... |
| R3-04 | LOCKED | bfc160483cb0e2c3... |
| R3-05 | RESTRICTED | d377c53e17e39a50... |
| R3-META | SENIOR OFFICER | 0b3322e4094fa3ba... |
| FINAL | CALIBRATE | ed15028e4c3ca285... |

---

## Pre-Launch Checklist

### Navigation
- [ ] index.html loads — mission briefing visible, round cards render
- [ ] puzzles.html loads — Round 1 feeders show READY, all others LOCKED
- [ ] submit.html loads — puzzle dropdown populated (19 puzzles)
- [ ] standings.html loads — empty state message (no teams yet)
- [ ] admin/admin.html — password prompt appears; password "deadreckoning" grants access

### Answer Submission (test locally with a browser)
- [ ] Submit "DECOYS" on R1-01 → shows correct, marks solved, updates card to SOLVED
- [ ] Submit "WRONG" on R1-01 → shows incorrect, no state change
- [ ] Submit lowercase "decoys" on R1-01 → treats as correct (JS normalizes to uppercase)
- [ ] Submit "FAULT" on R2-01 while R2 is locked → puzzle is not accessible (locked)
- [ ] Solve 4 of 5 Round 1 feeders → R1-META unlocks
- [ ] Solve R1-META → Round 2 feeders unlock
- [ ] Solve 4 of 5 Round 2 feeders → R2-META unlocks
- [ ] Solve R2-META → Round 3 feeders unlock
- [ ] Solve R3-META → FINAL unlocks
- [ ] Submit "CALIBRATE" on FINAL → commission overlay appears

### Content
- [ ] All puzzle pages load without broken HTML
- [ ] Tables render with correct alignment
- [ ] ASCII/pre blocks preserve character spacing
- [ ] Classification stamp visible on each puzzle page
- [ ] Answer input placeholder matches answer length
- [ ] Round badges show correct colors (amber/orange/blue/purple)

### State Persistence
- [ ] Solve a puzzle, refresh page → still shows solved
- [ ] Clear localStorage → all puzzles reset to initial state

### Mobile
- [ ] index.html readable at 375px width
- [ ] Puzzle pages readable (tables may scroll horizontally — acceptable)
- [ ] Answer input and submit button accessible on mobile

### Admin
- [ ] Wrong password blocked
- [ ] Correct password ("deadreckoning") grants access
- [ ] Solve state grid shows all 19 puzzles
- [ ] Manual solve override works
- [ ] Standings JSON download works

---

## Deployment Options

### Option A — Local (for in-person play on a local network)
```bash
cd puzzlehunt/scenarios/space-game/delivery/site/
# Use any static file server:
npx serve .
# or: python3 -m http.server 8080
# All devices on local WiFi → http://[your-ip]:8080
```

### Option B — GitHub Pages (free, permanent URL)
```bash
cd puzzlehunt/scenarios/space-game/delivery/site/
git init
git add .
git commit -m "DEAD RECKONING — hunt site"
git remote add origin https://github.com/[user]/dead-reckoning
git push -u origin main
# Enable Pages in repo Settings → Source: main branch, / (root)
```

### Option C — Netlify (drag-and-drop, instant)
1. Go to netlify.com → "Add new site" → "Deploy manually"
2. Drag the `delivery/site/` folder onto the deploy area
3. Site is live at `https://[random-name].netlify.app`
4. Set a custom domain if desired

---

## Known Limitations

1. **Client-side answer checking** — SHA-256 hashes are in puzzles.json (public). A determined solver could inspect the file and reverse-engineer answers. For this hunt's audience (husband + close group) this is acceptable.

2. **No server-side state** — each player's progress is stored in their own browser localStorage. Admin must manually update standings.json. Consider a shared Google Sheet for real-time standings.

3. **No automatic puzzle unlocking between players** — if players work together on different devices, they each need to submit answers on their own device to unlock rounds. A shared session link or admin override (in admin panel) can work around this.

4. **Hyphens in answers** — PHASE-LOCKED PAIR contains a hyphen. The answer input should accept it. Test that `checkAnswer` normalizes correctly (currently does `.toUpperCase().trim()` — the hyphen is preserved, which matches the hash computed from "PHASE-LOCKED PAIR"). ✓

---

## Stage 10 Status

| Item | Status |
|------|--------|
| Answer hashes computed and loaded | ✓ Done |
| Delivery structure complete | ✓ Done |
| Pre-launch checklist | Pending manual test |
| Live test with volunteer solver | Pending |

**Platform test is ready to run. Open `delivery/site/index.html` in a browser to begin.**
