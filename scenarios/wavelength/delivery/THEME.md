# WAVELENGTH -- Visual Theme

## Aesthetic

Late-night independent radio. Analog warmth. The last broadcast before the signal goes dark. Everything feels slightly faded, like a photograph left on a dashboard. Warm tones, low contrast, the soft amber glow of a desk lamp in a sound booth.

## Color Palette

| Swatch | Hex | Usage |
|--------|-----|-------|
| Booth Dark | #1a1210 | Page background. The sound booth at midnight. |
| Warm Amber | #e8c36a | Primary text. The ON AIR light reflected on surfaces. |
| Faded Cream | #c9b88a | Body text. Aged paper and felt-tip pen. |
| Dial Red | #c43c2d | Accent. VU meter red, the ON AIR bulb. Used sparingly. |
| Signal Green | #4a9e5c | Secondary accent. Tuning dial, EQ bars. One or two touches. |
| Dust Grey | #6b6156 | Muted text. Timestamps, footers, faded labels. |
| Deep Brown | #2a211a | Card backgrounds, inset panels. |
| Static White | #f0ebe3 | High-contrast moments. Headings on dark backgrounds. |

## Typography

### Primary: "Courier New" (monospace)
The playlist card font. Felt-tip pen on index cards. Used for Vee's handwritten notes, puzzle content, song lists, and all body text. Evokes typewritten radio station logs.

| Context | Size | Weight |
|---------|------|--------|
| Body text (site) | 14-16px | Regular |
| Puzzle content | 13-15px | Regular |
| Station call letters | 18-24px | Bold |
| Timestamps / metadata | 11-12px | Regular |

### Secondary: Georgia (serif)
Used for Vee's spoken lines (the one-sentence intros) and for the station name in display contexts. Warm, literary, slightly old-fashioned.

| Context | Size | Weight |
|---------|------|--------|
| Station name display | 28-36px | Bold |
| Vee's voice lines | 16-18px | Italic |
| Section headers | 20-24px | Regular |

### Tertiary: system sans-serif
Used only for UI elements (buttons, form labels). Should be invisible -- never the focal font.

## Texture and Feel

- **No sharp edges.** Corners are softened. Borders are thin or absent.
- **Low contrast.** Text never pops; it glows. The eye adjusts like entering a dim room.
- **Coffee stains.** Not literal. The spirit of them. Things are slightly worn.
- **VU meter motif.** The segmented bars of a signal level meter can appear as a design element (section dividers, decorative rule).
- **The ON AIR sign.** A small red dot or pill-shaped indicator. Appears once on the homepage. Suggests the broadcast is live.

## Physical Prop Style

- **Paper:** Warm-toned (cream or light tan). Not bright white. Suggests age and handling.
- **Ink:** Dark brown or near-black. Not pure #000000.
- **Handwriting font:** Use a handwriting-style web font (e.g., Caveat, Patrick Hand, or Shadows Into Light) for the playlist prop, OR print in Courier New italic to suggest Vee's felt-tip scrawl.
- **Stains and wear:** Optional coffee ring watermark (very faint, 3-5% opacity) for print prop. Do not overdo it.

## Design Principles

1. **Less is more.** Every radio station that lasted 40 years had one desk lamp and one microphone. The design should feel the same way.
2. **Warmth, not nostalgia.** This is not a retro pastiche. It is a real place that existed. The aesthetic comes from sincerity, not imitation.
3. **The signal fades.** Toward the bottom of pages, text can become slightly more muted. Opacity decreasing. The broadcast is ending.
4. **Silence is part of the design.** Generous whitespace. Pauses between elements. Vee never rushed.

## CSS Variables

```css
:root {
  --booth-dark: #1a1210;
  --warm-amber: #e8c36a;
  --faded-cream: #c9b88a;
  --dial-red: #c43c2d;
  --signal-green: #4a9e5c;
  --dust-grey: #6b6156;
  --deep-brown: #2a211a;
  --static-white: #f0ebe3;
  --font-mono: 'Courier New', Courier, monospace;
  --font-serif: Georgia, 'Times New Roman', serif;
}
```
