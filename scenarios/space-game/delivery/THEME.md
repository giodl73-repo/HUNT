# DEAD RECKONING — Site Theme

**Font:** Share Tech Mono, Courier New, monospace
**Background:** #0d1117
**Surface:** #161b22
**Border:** #1e2a38
**Text primary:** #c9d1d9
**Text secondary:** #8b949e
**Text bright:** #e6edf3
**Accent:** #4dd0e1 (cyan — links, active, buttons)
**Solved:** #3fb950 (green)
**Error / Locked:** #f85149 (red)
**Aesthetic:** Ship computer terminal — dark, functional, monospace throughout. No decorative imagery.

## Round Colors

| Round | Color Name | Hex |
|-------|-----------|-----|
| TELEMETRY (Round 1) | Amber | #ffd166 |
| SYSTEMS LOG (Round 2) | Orange | #ef8c4a |
| CREW RECORD (Round 3) | Blue | #58a6ff |
| THE COMMISSION (Final) | Purple | #b083f0 |

## Voice

Terse. Technical. Monospace throughout. No exclamation marks. Short declarative sentences.
All labels uppercase. Solve confirmations read as instrument acknowledgments, not celebration.

## File Structure

```
delivery/
├── THEME.md                     ← this file
└── site/
    ├── index.html               ← splash / mission brief
    ├── puzzles.html             ← puzzle list, all 19 cards
    ├── submit.html              ← answer submission form
    ├── standings.html           ← leaderboard
    ├── assets/
    │   ├── theme.css            ← complete site CSS
    │   └── hunt.js              ← answer checking, unlock logic, state
    ├── data/
    │   ├── puzzles.json         ← metadata for all 19 puzzles
    │   └── standings.json       ← team standings (placeholder)
    ├── puzzles/                 ← individual puzzle HTML files (Stage 9b)
    └── admin/
        └── admin.html           ← hunt control panel
```

## Stage 10 Note

All `answerHash` fields in `puzzles.json` are currently set to `"TODO"`.
At Stage 10, compute SHA-256 of each answer (uppercased, trimmed) and replace.
The `checkAnswer()` function in `hunt.js` uses `SHA256(input.toUpperCase().trim())`
via the Web Crypto API.
