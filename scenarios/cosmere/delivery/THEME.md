# THE SIXTEENTH SHARD — Visual Design System

## Design Philosophy

A fan archive that has been running for too long. Someone built this site because they cared, and the care shows in the typography and the attention to atmosphere, not in polish or branding. The site looks like it was made by one person over many years. Warm where it should be warm. Cold where it should be cold. The kind of site that makes you slow down and read.

Not a corporate puzzle platform. Not a game launcher. An archive. A collection. A record of something someone spent a very long time assembling, maintained by a narrator who is drier than he lets on and older than anyone suspects.

---

## Shared Design Tokens

### Typography

| Use | Font | Fallback | Weight | Notes |
|-----|------|----------|--------|-------|
| Wordmark, round titles | Cinzel | Georgia, serif | 600 | Small caps. Letter-spacing +0.15em. Roman-era fantasy. |
| Body text, Hoid's voice | Alegreya | Palatino, serif | 400, 400i | Humanist serif. Warm, readable. 16px/1.6 base. |
| Puzzle content, tables | Alegreya | Palatino, serif | 400 | Same as body, 15px. |
| Extraction worksheets | JetBrains Mono | Consolas, monospace | 400 | Answer blanks, code blocks, the countdown. 14px. |
| Answer input fields | JetBrains Mono | Consolas, monospace | 400 | Uppercase transform. Letter-spacing +0.3em. |

### Shared Colors

| Token | Hex | Use |
|-------|-----|-----|
| `--bg-deep` | #0a0a0f | Homepage background, deepest black |
| `--text-primary` | #e0dcd4 | Default body text across all rounds |
| `--text-muted` | #8a8478 | Secondary text, captions, breadcrumbs |
| `--text-solved` | #4a9e6a | Solved puzzle status |
| `--text-locked` | #555050 | Locked meta status |
| `--border-subtle` | #2a2a30 | Table borders, card borders, horizontal rules |
| `--link-visited` | #7a6a8a | Visited puzzle links |
| `--error-flash` | #8b3030 | Wrong answer flash (subtle, not aggressive) |
| `--success-flash` | #3a7a4a | Correct answer flash |

### Spacing

```css
:root {
  --space-xs: 0.25rem;    /* 4px */
  --space-sm: 0.5rem;     /* 8px */
  --space-md: 1rem;       /* 16px */
  --space-lg: 2rem;       /* 32px */
  --space-xl: 4rem;       /* 64px */
  --space-xxl: 8rem;      /* 128px — vertical breathing room on homepage */
  --content-max: 48rem;   /* 768px — readable line length */
  --page-max: 64rem;      /* 1024px — max page width */
}
```

### Borders and Decoration

- No rounded corners. Sharp edges throughout. The Cosmere is not soft.
- Horizontal rules: 1px solid `--border-subtle`. The only ornament.
- No drop shadows. No gradients on text. No glow effects except stormlight.
- Card borders: 1px solid, color varies by round.
- Tables: 1px borders, no zebra striping. Headers get a slightly heavier border-bottom.

---

## ROUND 1: SCADRIAL — "The Metal Arts"

The metals are warm. Allomancy burns. The color palette comes from forge-fire, cooling steel, and the ash that covers everything.

### Colors

| Token | Hex | Use |
|-------|-----|-----|
| `--scadrial-bg` | #1a1a1f | Page background — dark charcoal with a slight brown warmth |
| `--scadrial-accent` | #d4a843 | Links, titles, metal references — amber gold |
| `--scadrial-accent-dim` | #9a7830 | Hover states, secondary accents |
| `--scadrial-text` | #b8b0a8 | Body text — ash grey with warmth |
| `--scadrial-mist` | rgba(180, 175, 168, 0.03) | Mist overlay color |
| `--scadrial-border` | #3a352e | Borders — warm dark |
| `--scadrial-table-header` | #2a2520 | Table header row |

### Atmosphere

**Mist overlay:** A CSS pseudo-element on the body creates a subtle, drifting mist effect. Two semi-transparent radial gradients that move on a 30-second animation loop. Barely visible. The effect should be felt more than seen.

```css
.round-scadrial::after {
  content: '';
  position: fixed;
  inset: 0;
  pointer-events: none;
  background:
    radial-gradient(ellipse 60% 40% at 20% 30%, var(--scadrial-mist), transparent),
    radial-gradient(ellipse 50% 50% at 70% 60%, var(--scadrial-mist), transparent);
  animation: mist-drift 30s ease-in-out infinite alternate;
  z-index: 1;
}
```

**Ash:** The background image (`bg-scadrial.jpg`) is a dark, barely-visible texture suggesting ash fallout. Tile-ready. Overlaid at 5% opacity. Most of the time, the page just looks dark. Sometimes, if you look, you notice the ash.

### Typography Overrides

- Round title: Cinzel, `--scadrial-accent`, 2.5rem
- Puzzle titles: Cinzel, `--scadrial-accent`, 1.5rem
- Metal badges: JetBrains Mono, `--scadrial-accent-dim`, 0.75rem, uppercase, border 1px solid

### Puzzle Card Treatment

```
┌─────────────────────────────────┐
│ P01 ★★                Iron (#1) │  ← Cinzel title, amber, metal badge right-aligned
│ Burning Through                 │
├─────────────────────────────────┤
│ Sixteen metals burn in a        │  ← Alegreya, ash-grey
│ Mistborn's stomach...           │
├─────────────────────────────────┤
│ ○ OPEN                          │  ← status indicator
└─────────────────────────────────┘
```

Border: 1px solid `--scadrial-border`. No fill. On hover: border shifts to `--scadrial-accent`.

---

## ROUND 2: ROSHAR — "Words of Radiance"

Storms. Stormlight. Stone. The palette comes from the highstorm's passage: deep blue-black skies, cyan lightning, and the grey stone of Urithiru. The light comes in cracks.

### Colors

| Token | Hex | Use |
|-------|-----|-----|
| `--roshar-bg` | #0d1520 | Page background — deep blue-black, the sky before a storm |
| `--roshar-accent` | #00e5ff | Stormlight cyan — links, titles, the glow |
| `--roshar-accent-dim` | #007a8a | Hover states, secondary |
| `--roshar-text` | #c0c8d0 | Body text — cool grey with blue tint |
| `--roshar-stone` | #2a3040 | Borders, table backgrounds — stone color |
| `--roshar-glow` | rgba(0, 229, 255, 0.08) | Subtle stormlight glow on active elements |
| `--roshar-border` | #253040 | Borders — cool dark |
| `--roshar-table-header` | #1a2535 | Table header row |

### Atmosphere

**Stormlight glow:** Interactive elements (links, buttons, active puzzle cards) have a subtle box-shadow glow in stormlight cyan. The glow pulses on hover — a 2-second CSS animation. Not bright. Just enough to suggest light leaking through cracks.

```css
.round-roshar a:hover,
.round-roshar .puzzle-card:hover {
  box-shadow: 0 0 12px var(--roshar-glow), 0 0 24px var(--roshar-glow);
  transition: box-shadow 0.3s ease;
}
```

**Stone texture:** The background uses a CSS pattern suggesting worked stone. Geometric, angular — not natural rock, but cut stone. Think Urithiru's carved walls. Implemented as a repeating SVG pattern at very low opacity.

**Geometric patterns:** Thin-line geometric borders around section headers, echoing the symmetry glyphs of Alethi culture. Purely decorative. Implemented as CSS border-image with a custom SVG tile.

### Typography Overrides

- Round title: Cinzel, `--roshar-accent`, 2.5rem
- Puzzle titles: Cinzel, `--roshar-accent`, 1.5rem
- Order badges: JetBrains Mono, `--roshar-accent-dim`, 0.75rem, uppercase

### Puzzle Card Treatment

Same structure as Scadrial, but:
- Border: 1px solid `--roshar-border`
- On hover: stormlight glow (box-shadow)
- Badge: Order name instead of metal

---

## ROUND 3: THE COSMERE — "The Shattering"

The void between worlds. Almost nothing. The color palette is near-black and silver. The text is sparse. The page feels vast and mostly empty, like looking out at the space between Shardworlds. Minimalist. What is not there matters more than what is.

### Colors

| Token | Hex | Use |
|-------|-----|-----|
| `--cosmere-bg` | #050508 | Page background — the closest to true black. The void. |
| `--cosmere-accent` | #c0c0c0 | Silver — links, titles, the only color |
| `--cosmere-accent-dim` | #808080 | Hover states, secondary |
| `--cosmere-text` | #a0a0a0 | Body text — neutral grey, no warmth, no coolness |
| `--cosmere-void` | #0a0a0f | Card backgrounds — slightly lighter than void |
| `--cosmere-border` | #1a1a20 | Borders — barely visible |
| `--cosmere-table-header` | #0f0f14 | Table header row |

### Atmosphere

**Emptiness:** Maximum whitespace. Puzzle cards are widely spaced. Text has extra line-height (1.8). The page should feel like there is more space than content. The void is the design.

**No effects.** No mist. No glow. No texture. No animation. The Cosmere pages are still. If the Scadrial pages burn and the Roshar pages crackle, the Cosmere pages hold their breath.

**Silver text on near-black.** The contrast is softer than the other rounds. The text asks you to lean in. To pay attention. To notice what is not written.

### Typography Overrides

- Round title: Cinzel, `--cosmere-accent`, 2.5rem, letter-spacing +0.25em (wider than other rounds)
- Puzzle titles: Cinzel, `--cosmere-accent`, 1.5rem
- Shard badges: JetBrains Mono, `--cosmere-accent-dim`, 0.75rem

### Puzzle Card Treatment

Same structure, but:
- Border: 1px solid `--cosmere-border` (barely visible)
- On hover: border shifts to `--cosmere-accent` (silver appears from the void)
- No effects. No glow. No animation. Stillness.

---

## Meta and Super-Meta Pages

### Round Meta Pages

Each round meta page uses its round's visual theme but with increased gravity:
- Larger title (3rem)
- More vertical space
- The extraction workspace is centered and given generous padding
- Answer input field is larger (1.5rem font, centered)

### Super-Meta: THE SEVENTEENTH SHARD

Uses the Cosmere (void/silver) theme as base, but with two additions:

1. **All three round accents appear:** The Realms cycle diagram uses amber for Physical, cyan for Cognitive, and silver for Spiritual. This is the only page where all three color worlds collide.

2. **The final reveal:** On correct answer submission, the background shifts from void-black to a very slow fade toward deep gold (#1a1508). The shift takes 10 seconds. The site is warming. Something has changed. Hoid's closing statement appears in amber Alegreya italic. The archive is no longer dark. It remembers what wholeness felt like.

---

## Print Theme Override

When generating print materials (`delivery/print/`), the visual system shifts:

- **Background:** white (for ink economy and readability)
- **Text:** black (#1a1a1a)
- **Accents:** dark versions of each round's accent color (amber -> #8a6020, cyan -> #006070, silver -> #404040)
- **Fonts:** same families, but body at 11pt, headers at 14pt
- **Tables:** thin black borders, no shading
- **Page size:** Letter (8.5x11in), margins 0.75in all sides
- **B&W safe:** all designs must work in greyscale. No information encoded solely in color.

---

## CSS Custom Properties (Full Reference)

```css
:root {
  /* Shared */
  --bg-deep: #0a0a0f;
  --text-primary: #e0dcd4;
  --text-muted: #8a8478;
  --text-solved: #4a9e6a;
  --text-locked: #555050;
  --border-subtle: #2a2a30;
  --link-visited: #7a6a8a;
  --error-flash: #8b3030;
  --success-flash: #3a7a4a;

  /* Scadrial */
  --scadrial-bg: #1a1a1f;
  --scadrial-accent: #d4a843;
  --scadrial-accent-dim: #9a7830;
  --scadrial-text: #b8b0a8;
  --scadrial-mist: rgba(180, 175, 168, 0.03);
  --scadrial-border: #3a352e;
  --scadrial-table-header: #2a2520;

  /* Roshar */
  --roshar-bg: #0d1520;
  --roshar-accent: #00e5ff;
  --roshar-accent-dim: #007a8a;
  --roshar-text: #c0c8d0;
  --roshar-stone: #2a3040;
  --roshar-glow: rgba(0, 229, 255, 0.08);
  --roshar-border: #253040;
  --roshar-table-header: #1a2535;

  /* Cosmere */
  --cosmere-bg: #050508;
  --cosmere-accent: #c0c0c0;
  --cosmere-accent-dim: #808080;
  --cosmere-text: #a0a0a0;
  --cosmere-void: #0a0a0f;
  --cosmere-border: #1a1a20;
  --cosmere-table-header: #0f0f14;

  /* Spacing */
  --space-xs: 0.25rem;
  --space-sm: 0.5rem;
  --space-md: 1rem;
  --space-lg: 2rem;
  --space-xl: 4rem;
  --space-xxl: 8rem;
  --content-max: 48rem;
  --page-max: 64rem;

  /* Typography */
  --font-display: 'Cinzel', Georgia, serif;
  --font-body: 'Alegreya', Palatino, serif;
  --font-mono: 'JetBrains Mono', Consolas, monospace;
  --font-size-base: 1rem;       /* 16px */
  --font-size-sm: 0.875rem;     /* 14px */
  --font-size-lg: 1.25rem;      /* 20px */
  --font-size-xl: 1.5rem;       /* 24px */
  --font-size-xxl: 2.5rem;      /* 40px */
  --line-height-base: 1.6;
  --line-height-tight: 1.3;
}
```

---

## Asset Manifest

| File | Type | Size Target | Notes |
|------|------|-------------|-------|
| `logo.svg` | SVG | <10KB | THE SIXTEENTH SHARD wordmark. Cinzel-based. Silver on transparent. |
| `bg-scadrial.jpg` | JPEG | <100KB | Ash/mist texture. Tile-ready. Dark. |
| `bg-roshar.jpg` | JPEG | <100KB | Stone texture. Angular. Dark blue-grey. |
| `bg-cosmere.jpg` | JPEG | <50KB | Near-black noise texture. Almost invisible. |
| `theme.css` | CSS | <15KB | All styling. No preprocessor needed. |
| `hunt.js` | JS | <10KB | Vanilla JS. Hash validation, localStorage, unlock logic. |
| Cinzel | WOFF2 | ~30KB | Google Fonts (self-hosted for reliability) |
| Alegreya | WOFF2 | ~40KB | Google Fonts (regular + italic) |
| JetBrains Mono | WOFF2 | ~30KB | Google Fonts (regular only) |

**Total site weight (excluding puzzle-specific images): ~300KB.** Well under the 2MB target.
