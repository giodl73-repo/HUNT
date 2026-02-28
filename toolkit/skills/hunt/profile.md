# /hunt profile — Full Profile Rebuild from Confirmed Puzzle Data

Rebuild a reviewer profile completely from top to bottom, using confirmed puzzle credits and mechanics — not inference. Every section is rewritten. The output should read like the person, not a description of the person.

## Usage

```
/hunt profile <name>          — full rebuild of one profile
/hunt profile all             — rebuild all profiles
/hunt profile score           — score all profiles and list by quality
```

---

## What This Skill Does

Most profiles are written from the outside: "this person believes X, so they would ask Y." This skill works from the inside: gather every confirmed puzzle this person has made, read the mechanics and answers, extract the convictions that produced them — then write every section from that understanding.

The result is a complete profile with five sections:

**What Makes Them Exceptional** — The opening statement. Tells you why this person matters, what they've done that others haven't, and why you should care what they think. Direct, specific, no hedging. 2–4 short paragraphs.

**Identity** — Role, affiliation, defining achievement, and bio quote. One sentence each. No padding.

**Puzzle Hunt Credentials** — Every confirmed hunt credit, puzzle, and achievement. Tables where useful. Sourced from evidence files.

**Design Philosophy** — A single coherent voice, not a list. Their core conviction about what puzzles are for. No puzzle titles cited. No mechanics described. Just the extracted belief, stated directly. 3–5 short paragraphs.

**Review Lens** — Their actual checklist. Not a restatement of the philosophy. Specific criteria drawn from what they've actually built. 4–7 bullet points, each a real demand grounded in their body of work.

---

## Process

### Step 1 — Load the profile

Read `toolkit/profiles/<name>.md`.

Note what's already in Design Philosophy and Review Lens. These are the sections you will replace. Everything else (Identity, Credentials, puzzle tables, Sources) stays untouched.

### Step 2 — Load all confirmed puzzle data

Check these sources in order:

1. `evidence/ms-hunt-puzzles-detail.md` — full PH23 Economics + Placement Test mechanics
2. `evidence/ph23-all-rounds.md` — all PH23 rounds with author credits
3. `evidence/mit-mh-2009-puzzles.md` — MIT Mystery Hunt 2009 full puzzle data
4. `evidence/ph-scraped/puzzle-university.json` — scraped solution pages

From the scraped JSON, extract all puzzles credited to this person:
```
node -e "
const data = JSON.parse(require('fs').readFileSync('evidence/ph-scraped/puzzle-university.json'));
const sols = data.solutions || {};
const keys = Object.keys(sols).filter(k => (sols[k]||'').includes('<NAME>'));
keys.forEach(slug => {
  const text = sols[slug] || '';
  const answerMatch = text.match(/Answer:\s*([A-Z][A-Z '!?.]+)/);
  const lines = text.split('\n').filter(l => l.trim());
  const answerIdx = lines.findIndex(l => l.match(/^Answer:/));
  const mechanic = lines.slice(answerIdx+1, answerIdx+3).join(' ').substring(0, 200);
  console.log(slug + ' | ' + (answerMatch ? answerMatch[1].trim() : '?') + ' | ' + mechanic);
});
"
```

### Step 3 — Analyze the body of work

With the full puzzle list in front of you, look for:

**Patterns across mechanics** — What types of puzzles do they repeatedly make? Are they visual? Logic-based? Word-based? Do they combine constraints? Do they use images as primary data?

**Patterns across answers** — Do the answers do something beyond terminate the puzzle? Do they name an experience, complete an argument, or reframe what you were doing?

**The implied conviction** — If someone made all these puzzles, what must they believe about what puzzles are for? State it in one sentence. That sentence is the core of the Design Philosophy.

**What they would hate** — Based on what they've built, what would immediately fail their standard? That's the first item in the Review Lens.

### Step 4 — Write Design Philosophy

Rules:
- **One voice, one argument.** Not sections with headers. Flowing prose that builds.
- **No puzzle titles or mechanics cited by name.** Extract the principle, not the example.
- **No citations, no "she believes that..."** Write as if articulating their view directly.
- **Short.** 3–5 paragraphs. If it needs more, the argument isn't clear yet.
- **The conviction comes first.** Open with the thing they believe that most designers don't.

### Step 5 — Write Review Lens

Rules:
- **Checklist format** — bullet points, each a clear criterion.
- **Grounded in their actual work** — each criterion should come from something they've actually built. Ask: "what would they notice because they've done it themselves?"
- **Not a restatement of the philosophy** — the philosophy explains their worldview; the lens is what they check off on a puzzle.
- **4–7 items.** More than 7 means the lens isn't focused.
- **Direct.** Each bullet is a demand, not a suggestion.
- **The first item is the dealbreaker** — the thing that fails immediately if it's wrong.

---

## Quality Bar

A finished profile passes this test: read Design Philosophy to someone who knows this person. They should say "yes, that's exactly how they think." Read Review Lens to someone who has had their work reviewed by this person. They should say "yes, that's what they look at."

If the philosophy could describe any thoughtful puzzle designer — it's not specific enough.
If the review lens could be applied by anyone — it's not grounded enough.

---

## Example Output Pattern

**Design Philosophy** opens with a conviction sentence:
> "[Name] doesn't start with X. They start with Y."

Then builds the argument in 3–4 paragraphs showing how that conviction plays out.

**Review Lens** opens with the dealbreaker:
> - **[Core question that fails immediately if wrong]** — one sentence explanation.

Then 4–6 additional criteria that escalate the scrutiny.

---

## Reference Profiles

Use these as the quality standard:
- `toolkit/profiles/dana-young.md` — the template for this approach
- `toolkit/profiles/kenny-young.md` — once completed, the second reference

---

## Notes

- Do not invent puzzle credits. Only use what's in the evidence files or confirmed sources.
- Do not reference specific puzzle titles in the philosophy or lens sections.
- If evidence is thin for a profile, note what's missing and write from documented philosophy quotes instead.
- The "What Makes Them Exceptional" section at the top stays as-is — only Design Philosophy and Review Lens are rebuilt by this skill.
