# Paper Plan: What Does a Persona Do? Cognitive Trait Decomposition of Authoring Identity in AI Creative Task Generation

## Motivating Finding

In a controlled persona study (games-profile-taxonomy-creative, authoring study Experiment 2), "You are an artist" outperformed "You are a puzzle designer" by 15.6 points on a /45 weighted rubric (40.3 vs. 24.7). More significantly: the artist was the only condition to escape mechanism gravity — it invented a structurally novel mechanism (damaged inscriptions with one letter missing per AoE2 item) while all six other persona conditions (null, puzzle designer, game designer, programmer, doctor, no job) produced letter-indexing or first-letter acrostic puzzles.

The artist persona activated something that domain-specific expertise did not. The question this paper asks: **what cognitive trait, specifically, did "You are an artist" activate?**

## Research Question

When a bare persona prompt improves AI creative output quality, which component of that persona identity is doing the work?

We decompose the "artist" persona into its constituent cognitive traits and test each in isolation, using the same puzzle authoring task as the motivating study.

## Core Claim

Authoring quality gains from persona prompting are attributable to specific cognitive traits, not to holistic professional identity. The artist advantage over the puzzle designer baseline comes primarily from **experience-first thinking** (designing toward what the solver will *feel*) rather than from creativity, visual thinking, or aesthetic sensibility per se. This claim is falsifiable by the trait ablation: if visual thinking or surprise instinct independently replicate the artist's mechanism novelty, experience-first thinking is not the unique driver.

## The 6 Cognitive Traits to Test

Each trait is operationalized as a single sentence added to the bare task prompt.

| Code | Trait | Prompt addition |
|------|-------|----------------|
| T1 | Experience-first | "You always think first about how the other person will experience what you make." |
| T2 | Surprise instinct | "You care deeply about surprising people — you want them to pause and think 'wait, really?'" |
| T3 | Visual thinking | "You think in images and spatial relationships before you think in words." |
| T4 | Elegance drive | "You remove anything that isn't necessary — you value spare, clean form above all." |
| T5 | Rigor/verification | "You check your work carefully before calling it done — you never assume it works." |
| T6 | Systematic thinking | "You approach problems by breaking them into a logical structure before building anything." |

Plus: full artist persona as control ("You are an artist."), puzzle designer baseline (A0), null baseline (AX).

## Experiment Design

### Phase 1: Trait isolation (one at a time)
- 6 agents (T1–T6), each with bare task + one trait sentence
- Same task: AoE2 feeder puzzle, answer TOWER
- Evaluate with C8 panel, /45 rubric

### Phase 2: Trait leave-one-out
- 6 agents, each with full artist persona minus one trait description
- Artist persona = all 6 traits combined into a coherent prompt
- Evaluate with C8 panel, /45 rubric

### Phase 3: Minimal artist reconstruction
- Start from null; add traits one at a time until artist-level performance is reached
- Identifies the minimum sufficient trait set

## Key Comparisons

- **T1 vs. AA**: does experience-first alone replicate the artist's mechanism novelty?
- **T2 vs. AA**: does surprise instinct alone replicate it?
- **T3 vs. AA**: does visual thinking alone replicate it?
- **T4/T5/T6 vs. AA**: do structural/analytical traits replicate it?
- **Full artist minus T1**: does removing experience-first collapse the artist advantage?
- **A0 (puzzle designer) vs. T6 (systematic)**: are these effectively the same prompt?

## Predicted Results

1. T1 (experience-first) will produce the highest score among trait-isolation conditions, and will be the only condition likely to generate a novel mechanism
2. T4 (elegance) and T6 (systematic) will produce well-constructed but mechanically conventional puzzles — similar to A5/A6 in the motivating study
3. T2 (surprise) will produce unusual answer words or clue framings, but not novel mechanisms
4. T3 (visual) may replicate the damaged-inscription approach (visual gap = missing letter) — the key test of whether mechanism novelty is about visual thinking or experience-first
5. Artist minus T1 will show the largest leave-one-out drop

## Target Venue

- **Primary**: ICCC 2026 (Computational Creativity — cognitive model of what personas activate)
- **Secondary**: CHI 2026 (Human-Computer Interaction — persona prompting for creative tasks)
- Page limit: 8-10 pages
- Deadline: March–April 2026

## Sections

### 00-abstract (~250 words)
- Motivating finding (artist > puzzle designer by 15.6 points, mechanism novelty)
- Research question (what trait did "artist" activate?)
- Approach (trait decomposition — isolation + leave-one-out)
- Key finding (T1 experience-first is the primary driver)
- Contribution (cognitive trait model of persona prompting for creative generation)

### 01-introduction (~900 words)
- The persona prompting puzzle: why does professional identity affect creative output?
- Prior work assumes holistic persona identity; no study decomposes traits
- The artist finding as an anomaly that requires explanation
- Our approach: trait isolation as causal identification

### 02-related-work (~1000 words)
- Persona prompting in LLMs (Park et al., Zhang et al., Dang et al.)
- Creative cognition and domain expertise (Csikszentmihalyi, Weisberg)
- Experience-first design theory (Norman, Hassenzahl — experience as primary)
- AI creative generation: prior work on prompt design for creative tasks
- Key gap: no prior work decomposes professional identity prompts into cognitive traits

### 03-methodology (~1200 words)
- Trait taxonomy: how the 6 traits were identified (from artist-persona literature; cognitive psychology of artistic practice)
- Prompt construction: how each trait is operationalized as a single sentence
- Full artist persona construction: all 6 traits in coherent prose
- Experiment phases 1–3
- Evaluation: same C8 panel and /45 rubric as motivating study
- Connection to motivating study: full methodological alignment

### 04-results (~1500 words)
- Phase 1 (isolation): T1 and T2 scores, mechanism novelty analysis
- Phase 2 (leave-one-out): which removal hurts most
- Phase 3 (reconstruction): minimum sufficient trait set
- Mechanism novelty analysis: does any single trait replicate the damaged-inscription approach, or does artist-level novelty require a trait combination?

### 05-discussion (~800 words)
- Experience-first as the operative trait: implications for persona design
- The puzzle designer paradox: why domain-specific expertise can constrain mechanism novelty
- Implications for prompt engineering: practitioners should inject experiential traits, not professional labels
- Limitations: single task, single domain, single model

### 06-conclusion (~400 words)
- Restate: experience-first thinking is the operative component of creative persona advantage
- The practical recommendation: add "think first about how the other person will experience this" to any creative generation prompt
- Future work: cross-domain replication, multi-trait interaction effects

## Dependencies

- games-profile-taxonomy-creative: provides motivating data (persona study results), world data files, C8 reviewer profiles, evaluation infrastructure
- All puzzle files from motivating study available for comparison
- Same PRINCIPLES.md, world-aoe2/, profiles-c8/ can be copied or linked

## Files to Create

```
games-persona-authoring-traits/
├── plan.md                    ← THIS FILE
├── _panel.yaml
├── HANDOFF.md                 ← for next session
├── profiles-c8/               ← copy from taxonomy paper
├── world-aoe2/                ← copy from taxonomy paper
├── results/
│   ├── phase1-isolation/      ← T1-T6 puzzles
│   ├── phase2-leaveoneout/    ← artist minus one trait each
│   ├── phase3-reconstruction/ ← cumulative builds
│   └── evaluation/            ← C8 scores
└── sections/                  ← LaTeX paper
```

## Related Papers

- `games-profile-taxonomy-creative`: the motivating study — provides all baseline data
- `games-ai-expert-panel-creative`: Paper #1 — reviewer-side profile depth ablation
- `games-human-ai-calibration`: would validate whether human artists show the same trait pattern
