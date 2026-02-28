# P08 — 100% Completion Guide — Mark Tracker

*Author: The Social*

---

> The completion screenshot was posted by IronFan_Kenji in 2002. The community celebrated — first confirmed 100%. But DataMiner_X looked closer. Five of the sixteen badges were wrong. Not earned or unearned. Wrong. The names did not match the data. Someone had tampered with the image.

---

## ironfall-archive.net/guides/100-completion — "Achievement Screenshot"

IronFan_Kenji (2002): "After 200 hours, I finally have all sixteen Marks of Mastery. Here is my save's achievement screen. Every badge is gold."

DataMiner_X (2002): "Five of these are not real Marks. Compare them to the official list. The names are close. But close is not correct."

SpeedQueen (2002): "Find the five fakes. Take the third letter of each fake name. Read them in grid order. If that is a coincidence, I am done with this game."

---

## The Screenshot

```
┌──────────────┬──────────────┬──────────────┬──────────────┐
│ ★ 01         │ ★ 02         │ ★ 03         │ ★ 04         │
│ GREENHORN    │ TRAILBLAZER  │ ASHEN VICTOR │ RELENTLESS   │
│ "Every hero  │ "Every path  │ "The forest  │ "Cold cannot  │
│  was once a  │  has a first │  bows to the │  stop the     │
│  beginner."  │  footprint." │  strong."    │  determined." │
├──────────────┼──────────────┼──────────────┼──────────────┤
│ ★ 05         │ ★ 06         │ ★ 07         │ ★ 08         │
│ TEMPEST BORN │ IRONCLAD     │ HAMMERED     │ COLLECTOR    │
│ "You are the │ "The citadel │ "Every       │ "Every piece │
│  storm now." │  falls."     │  creation    │  matters."   │
│              │              │  starts with │              │
│              │              │  force."     │              │
├──────────────┼──────────────┼──────────────┼──────────────┤
│ ★ 09         │ ★ 10         │ ★ 11         │ ★ 12         │
│ UMBERLIGHT   │ GHOST RUNNER │ SCHOLAR      │ STEADFAST    │
│ "The dark    │ "Speed is    │ "The old     │ "The true    │
│  paths are   │  truth."     │  words still │  warrior     │
│  the safest."│              │  have power."│  never       │
│              │              │              │  wavers."    │
├──────────────┼──────────────┼──────────────┼──────────────┤
│ ★ 13         │ ★ 14         │ ★ 15         │ ★ 16         │
│ OATHBREAKER  │ SURVEYOR     │ LEGACY       │ TRANSCENDENT │
│ "Some        │ "You watched │ "The         │ "You found   │
│  promises    │  from the    │  developer   │  the way."   │
│  were meant  │  shadows."   │  left        │              │
│  to break."  │              │  something   │              │
│              │              │  behind."    │              │
└──────────────┴──────────────┴──────────────┴──────────────┘
```

---

## Your Task

Compare the screenshot above to the official Marks of Mastery list from the archive (achievements reference). Five of the sixteen entries have been altered — the name is different, the description is wrong, or both.

1. Identify the five fakes.
2. For each fake, take the **3rd letter** of the fake name.
3. Read those letters in grid-position order (lowest number first).

---

## Solution

### Comparison against official Marks

| Pos | Screenshot name | Screenshot description | Real name | Real description | Verdict |
|-----|----------------|----------------------|-----------|-----------------|---------|
| 01 | GREENHORN | "Every hero was once a beginner." | Seedling | "A journey begins with a single step." | **FAKE** — wrong name AND wrong description |
| 02 | TRAILBLAZER | "Every path has a first footprint." | Trailblazer | "Every path has a first footprint." | Real |
| 03 | ASHEN VICTOR | "The forest bows to the strong." | Ashen Victor | "The forest bows to the strong." | Real |
| 04 | RELENTLESS | "Cold cannot stop the determined." | Relentless | "Cold cannot stop the determined." | Real |
| 05 | TEMPEST BORN | "You are the storm now." | Tempest Born | "You are the storm now." | Real |
| 06 | IRONCLAD | "The citadel falls." | Ironclad | "The citadel falls." | Real |
| 07 | HAMMERED | "Every creation starts with force." | Artisan | "Creation is its own reward." | **FAKE** — wrong name AND wrong description |
| 08 | COLLECTOR | "Every piece matters." | Collector | "Every piece matters." | Real |
| 09 | UMBERLIGHT | "The dark paths are the safest." | Night Walker | "Some doors open only in darkness." | **FAKE** — wrong name AND wrong description |
| 10 | GHOST RUNNER | "Speed is truth." | Ghost Runner | "Speed is truth." | Real |
| 11 | SCHOLAR | "The old words still have power." | Scholar | "The old words still have power." | Real |
| 12 | STEADFAST | "The true warrior never wavers." | Loyal Heart | "The party stands together." | **FAKE** — wrong name AND wrong description |
| 13 | OATHBREAKER | "Some promises were meant to break." | Oathbreaker | "Some promises were meant to be broken." | Real (description slightly shortened but recognizable) |
| 14 | SURVEYOR | "You watched from the shadows." | Unseen | "They never knew you were there." | **FAKE** — wrong name AND wrong description |
| 15 | LEGACY | "The developer left something behind." | Legacy | "The developer left something behind." | Real |
| 16 | TRANSCENDENT | "You found the way." | Transcendent | "You found the way." | Real |

### The five fakes

| Grid position | Fake name | 3rd letter |
|--------------|-----------|------------|
| 01 | GREENHORN | E |
| 07 | HAMMERED | M |
| 09 | UMBERLIGHT | B |
| 12 | STEADFAST | E |
| 14 | SURVEYOR | R |

### Read in grid order

01→E, 07→M, 09→B, 12→E, 14→R.

**Answer: EMBER** (ROT13: RZORE)

### Extraction Verification

Answer: EMBER. Puzzle slot: Act II, position 3. Super-meta extracts position 3: EMBER[3] = B. B in ROT13 = O. Matches the extraction table.

---

### Social's Notes

The fun part is the comparison. Sixteen badges, eleven real, five fake. The fakes are close enough to require checking every entry against the reference. Some fakes are thematically similar to the real Marks (GREENHORN vs SEEDLING, STEADFAST vs LOYAL HEART). Others are deliberately off (SURVEYOR vs UNSEEN).

The 3rd-letter extraction is stated upfront. No hidden aha needed for the extraction — the aha IS finding the fakes. One aha. Clean.

Position 13 (Oathbreaker) is a near-miss: the description says "were meant to break" instead of "were meant to be broken." This is intentionally kept as REAL (it is the correct Mark, just with a tiny forum-era paraphrase) to force careful comparison rather than snap judgments. Solvers who mark it as fake will get the wrong answer.

---

*The Marks of Mastery reference is in the archive. The official names do not change.*
