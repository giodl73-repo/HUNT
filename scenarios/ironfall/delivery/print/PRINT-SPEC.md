# IRONFALL Manual Page — Print Specification

## Overview

One page from the fictional IRONFALL game manual (1993, SNES). This is a physical prop or high-quality printable that solvers receive alongside the website experience. The page hints at the existence of secrets without revealing them.

---

## Dimensions

| Property | Value |
|----------|-------|
| **Page size** | 5.5" x 8.5" (half-letter, standard SNES manual size) |
| **Bleed** | 0.125" on all sides |
| **Safe area** | 0.375" margins inside trim |
| **Orientation** | Portrait |
| **Print** | Full color, both sides (front only for this asset) |

---

## Layout

### Border Frame

A pixel-art border surrounds the entire page. The border is a repeating pattern of iron-colored (dark gray, `#404040`) interlocking tiles with corner flourishes. The pattern is 8px-scale pixel art, suggesting 16-bit-era craftsmanship. Inside the pixel border, a 1pt solid black rule defines the content area.

### Page Number

Bottom center, outside the border frame: `— 42 —` in small serif font. Page 42 is deep enough in the manual to feel like an "advanced" section.

### Column Layout

Single column. The page is too narrow for two columns at this size. Content flows top to bottom:

```
+---------------------------------------+
|  ╔═══ PIXEL BORDER ═══════════════╗   |
|  ║                                 ║   |
|  ║  SECTION HEADER                 ║   |
|  ║  ─────────────────────          ║   |
|  ║                                 ║   |
|  ║  Body text paragraph 1         ║   |
|  ║                                 ║   |
|  ║  ┌─────────────────────┐       ║   |
|  ║  │  SIDEBAR / CALLOUT  │       ║   |
|  ║  │  (pixel-art frame)  │       ║   |
|  ║  └─────────────────────┘       ║   |
|  ║                                 ║   |
|  ║  Body text paragraph 2         ║   |
|  ║                                 ║   |
|  ║  ┌─────────────────────┐       ║   |
|  ║  │  CONTROLLER DIAGRAM │       ║   |
|  ║  │  (pixel art)        │       ║   |
|  ║  └─────────────────────┘       ║   |
|  ║                                 ║   |
|  ║  Footer note                    ║   |
|  ║                                 ║   |
|  ╚═════════════════════════════════╝   |
|              — 42 —                    |
+---------------------------------------+
```

---

## Typography (Print)

| Element | Font | Size | Style |
|---------|------|------|-------|
| Section header | Pixel font (Press Start 2P or similar) | 14pt | ALL CAPS, centered |
| Body text | Palatino / Georgia / serif | 9pt | Regular, justified |
| Callout text | Courier New / monospace | 8pt | Regular |
| Tips/hints | Body font | 8pt | Italic, indented |
| Page number | Body font | 7pt | Centered |

---

## Color (Print)

| Element | Color | CMYK |
|---------|-------|------|
| Border | Iron gray | C:0 M:0 Y:0 K:75 |
| Header text | Dark red | C:0 M:100 Y:100 K:30 |
| Body text | Black | C:0 M:0 Y:0 K:100 |
| Callout background | Warm cream | C:0 M:3 Y:10 K:5 |
| Callout border | Iron gray | C:0 M:0 Y:0 K:75 |
| Accent (tips) | Dark blue | C:100 M:80 Y:0 K:20 |

---

## Content Section

**Chapter:** ADVANCED TECHNIQUES
**Page title:** The Archive & Community Discoveries

---

## Tone

Written in the voice of a 1993 game manual -- formal but friendly, as if a technical writer at Ashfield Interactive is guiding a 12-year-old through the game's deeper systems. Slightly stilted English (translated from Japanese). Short sentences. Occasional second-person address ("You may find..."). No spoilers for the True Ending, but deliberate hints that something exists beyond the standard ending.

---

## Asset Requirements

1. **Pixel-art border:** Iron-colored tile pattern. Needs to be created as a repeating PNG at 300 DPI or as vector art.
2. **Controller diagram:** SNES controller with labeled buttons. Pixel-art style. Can be created as ASCII art for the markdown version, replaced with actual pixel art for print.
3. **Ashfield Interactive logo:** Small, bottom of page. "ASHFIELD INTERACTIVE" in a rectangular border. Pixel font.

---

## Production Notes

- Print on heavy stock (80lb+ cover or 100lb text) for a manual feel.
- Consider aging the paper slightly (cream/off-white stock, or light tea-stain effect on white).
- If producing multiples, a saddle-stitch booklet of 4-8 pages would be ideal, but the single page works as a standalone prop.
- The page should feel like it was pulled from a used game manual bought at a garage sale -- slightly worn, well-read, real.
