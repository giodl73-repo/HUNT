# Blind Test Results — P5 (Between the Lines)

Testers: Thomas Snyder, Wei-Hwa Huang, Dan Katz
Date: Stage 6 authoring gate

---

## P5 — Between the Lines (Mike Selinker)

### Thomas Snyder
**Solve attempt:**

Song 1: "Another One Bites the Dust" — Queen (1980)
Removing spaces: ANOTHERONEBITESTHEDUST
Looking for hidden words crossing boundaries...
- ANOTHER+ONE: "HERONE" -> HERON! H-E-R-O-N. A real word (a wading bird). Spans ANOTHER/ONE. First letter: H.

Song 2: "Total Eclipse of the Heart" — Bonnie Tyler (1983)
TOTALECLIPSEOFTHEHEART
- TOTAL+ECLIPSE: "ALECL" -> ALE? A-L-E spanning totAL + Eclipse. First letter: A. Short but valid.
- Other candidates: TALEC? No. LECLI? No. ALE is the best fit.

Song 3: "Livin' on a Prayer" — Bon Jovi (1987)
LIVINONAPRAYER
- LIVIN+ON: "VINON" -> VINO? V-I-N-O spanning liVIN + On. First letter: V. VINO is informal for wine. Valid English word.

Song 4: "The Same Deep Water as You" — The Cure (1989)
THESAMEDEE PWATERASYO U
- WATER+AS: "ERAS" -> E-R-A-S. First letter: E. Clean crossing.
- Other candidates: "DEEP WATER" -> EEPWA? No. "SAME DEEP" -> MEDEE? No. ERAS is the clear winner.

Song 5: "Born in the U.S.A." — Bruce Springsteen (1984)
BORNINTHEUSA
- BORN+IN+THE: "NINT" -> NINTH? N-I-N-T-H spanning borN + IN + THe. N(end of BORN), I-N (IN), T-H (start of THE). First letter: N. Beautiful — spans three words.

**Answer:** H-A-V-E-N = HAVEN

**Time:** ~20 minutes

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | The example in the instructions is a bit muddled (the "REATHY... no" self-correction is confusing). Clean it up. But the concept is clear. |
| Solvability | 5 | All hidden words are findable with patience. HERON is the gateway (longest, most obvious). NINTH is the hardest (spans 3 words). |
| Elegance | 4 | The mechanism is clean. Some hidden words are short (ALE = 3 letters) which feels slight. But HERON (5) and NINTH (5) anchor the puzzle well. |
| Reading Reward | 4 | The solver engages with iconic song titles and sees them differently. "I never noticed HERON hiding in Another One." |
| Fun | 5 | This is the puzzle you tell friends about. "Song titles have secret words hiding in them." |
| Confirmation | 4 | HAVEN is a real word. Each hidden word can be independently confirmed. |
| **Total** | **26/30** | |

**Issues:**
- Minor: The "How to Find Hidden Words" example is self-correcting ("REATHY... no. Look more carefully."). Remove the false start. Just show VERY as the example.
- Minor: ALE and ERAS are both 3-4 letters. The puzzle would be stronger if all hidden words were 4+ letters.

---

### Wei-Hwa Huang
**Solve attempt:**

Systematically removed spaces from each title and scanned for English words crossing boundaries.

1. ANOTHERONEBITESTHEDUST -> HERON (anotHERONe). Confirmed.
2. TOTALECLIPSEOFTHEHEART -> ALE (totALEclipse). Also found LIPS (ecLIPSe) but that doesn't cross a boundary — it's within ECLIPSE. ALE crosses TOTAL/ECLIPSE. Confirmed.
3. LIVINONAPRAYER -> VINO (liVINOn). Confirmed. Also considered INN (livINNon? No, one N). VINO is correct.
4. THESAMEDEE PWATERASYO U -> ERAS (watERAS). Confirmed. Also checked RASA (wateRASyou? "RASY" — no). ERAS is clean.
5. BORNINTHEUSA -> NINTH (borNINTHe). Confirmed. Spans 3 words — impressive.

**Answer:** HAVEN

**Time:** ~15 minutes

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | The example section needs cleanup. The concept is clear from the title "Between the Lines" and Vee's intro. |
| Solvability | 5 | No ambiguity — each title has exactly one strong hidden word at the right starting letter. VINO might cause a brief pause (is it English?) but it's in dictionaries. |
| Elegance | 4 | Mixed word lengths (3-5 letters) feel slightly uneven. But the mechanism is sound. |
| Reading Reward | 4 | |
| Fun | 5 | Strong dinner-party factor. The hidden words are genuinely surprising. |
| Confirmation | 4 | |
| **Total** | **26/30** | |

**Issues:**
- Minor: VINO is informal/borrowed from Italian. Some solvers might not count it as an English word. It appears in Merriam-Webster as informal, so it's defensible. But a more standard English word would be cleaner.
- Minor: Example section self-correction is distracting.
- The puzzle could benefit from a hint that the hidden words are thematically related, but they aren't strongly related. The brief suggested radio/communication themes. The actual words (HERON, ALE, VINO, ERAS, NINTH) don't have an obvious thematic link. This means the solver must find each word independently — no cross-confirmation from theme.

---

### Dan Katz
**Solve attempt:**

Found HERON first (most obvious). Then NINTH (satisfying — spans three words). ALE, VINO, ERAS followed.

**Answer:** HAVEN

**Time:** ~12 minutes

| Dimension | Score | Notes |
|-----------|-------|-------|
| Clarity | 4 | |
| Solvability | 5 | |
| Elegance | 4 | |
| Reading Reward | 4 | |
| Fun | 5 | NINTH spanning three words is the star of this puzzle. |
| Confirmation | 4 | |
| **Total** | **26/30** | |

**Issues:** None blocking. The example cleanup is the only real fix needed.

### P5 Aggregate: 26.0/30 — PASS

**Action items:**
1. Fix the "How to Find Hidden Words" example — remove the false start, just show VERY in EVERY.
2. Accept VINO as a valid English word (in Merriam-Webster).
3. Accept lack of thematic connection between hidden words — the puzzle works mechanically without it.
