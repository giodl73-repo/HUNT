# Allomancy — The 16 Metals

The Metallic Arts of Scadrial use 16 Allomantic metals arranged in a 4x4 grid. Each metal has an Allomantic power (burned), a Feruchemical power (stored/tapped), and a Hemalurgic use (stolen). The grid is organized by:

- **Quadrant:** Physical, Mental, Temporal, Enhancement
- **Axis 1:** Internal (self-targeting) vs External (other-targeting)
- **Axis 2:** Pushing vs Pulling

A "god metal" (atium, lerasium, harmonium) operates outside the standard table.

---

## The 16 Metals — Master Table

### Physical Quadrant

| # | Metal | Alloy Of | Int/Ext | Push/Pull | Allomantic Power | Feruchemical Power | Hemalurgic Steal |
|---|-------|----------|---------|-----------|------------------|--------------------|------------------|
| 1 | **Iron** | — | External | Pull | Ironpull — pulls on nearby metals (attracts metal toward the Allomancer) | Stores physical weight (becomes lighter while storing, heavier while tapping) | Steals Allomantic Physical powers (spike through heart) |
| 2 | **Steel** | Iron alloy | External | Push | Steelpush — pushes on nearby metals (repels metal away from Allomancer) | Stores physical speed (slower while storing, faster while tapping) | Steals Feruchemical Physical powers |
| 3 | **Tin** | — | Internal | Pull | Heightens all five senses dramatically | Stores one specific sense (diminished while storing, enhanced while tapping) | Steals Allomantic Mental powers |
| 4 | **Pewter** | Tin alloy | Internal | Push | Enhances physical strength, speed, balance, and endurance | Stores physical strength (weaker while storing, stronger while tapping) | Steals Feruchemical Mental powers |

### Mental Quadrant

| # | Metal | Alloy Of | Int/Ext | Push/Pull | Allomantic Power | Feruchemical Power | Hemalurgic Steal |
|---|-------|----------|---------|-----------|------------------|--------------------|------------------|
| 5 | **Zinc** | — | External | Pull | Riots (inflames) emotions in others | Stores mental speed (thinking slower while storing, faster while tapping) | Steals Allomantic Emotional powers [VERIFY — some sources say Human Emotional Fortitude] |
| 6 | **Brass** | Zinc alloy | External | Push | Soothes (dampens) emotions in others | Stores warmth (colder while storing, warmer while tapping) | Steals Feruchemical Cognitive powers [VERIFY] |
| 7 | **Copper** | — | Internal | Pull | Creates a coppercloud — hides Allomantic pulses from Seekers | Stores memories (forgets while storing, recalls perfectly while tapping) | Steals Allomantic Mental powers (specifically, one Mental Allomantic power) [VERIFY — overlap with Tin noted] |
| 8 | **Bronze** | Copper alloy | Internal | Push | Seeking — detects nearby Allomantic pulses through copperclouds (with enough power) | Stores wakefulness (drowsy while storing, alert while tapping) | Steals Allomantic Mental powers [VERIFY — need to distinguish from Copper/Tin] |

### Temporal Quadrant

| # | Metal | Alloy Of | Int/Ext | Push/Pull | Allomantic Power | Feruchemical Power | Hemalurgic Steal |
|---|-------|----------|---------|-----------|------------------|--------------------|------------------|
| 9 | **Gold** | — | Internal | Pull | Shows alternate past selves (who you would have been) | Stores health (sickly while storing, heals while tapping) | Steals Allomantic Hybrid powers [VERIFY] |
| 10 | **Electrum** | Gold alloy | Internal | Push | Shows your own immediate future (a few seconds of possible actions) | Stores determination / investiture [VERIFY — Electrum Feruchemy is not well-documented in published texts] | Steals Feruchemical Hybrid powers [VERIFY] |
| 11 | **Cadmium** | — | External | Pull | Creates a time-slowing bubble (time passes faster outside the bubble) | Stores breath (suffocates while storing, breathes perfectly in vacuum while tapping) | Steals Allomantic Temporal powers |
| 12 | **Bendalloy** | Cadmium alloy | External | Push | Creates a time-speeding bubble (time passes slower outside the bubble) | Stores energy/nutrition (hungry while storing, nourished while tapping) | Steals Feruchemical Temporal powers |

### Enhancement Quadrant

| # | Metal | Alloy Of | Int/Ext | Push/Pull | Allomantic Power | Feruchemical Power | Hemalurgic Steal |
|---|-------|----------|---------|-----------|------------------|--------------------|------------------|
| 13 | **Aluminum** | — | Internal | Pull | Wipes all of the Allomancer's own metal reserves instantly | Stores Identity (sense of self; losing Identity while storing allows unkeyed metalminds) | Steals all powers (destroys the spike and the stolen power; used as a "blank" or removal spike) [VERIFY] |
| 14 | **Duralumin** | Aluminum alloy | Internal | Push | Burns away all current metals in a single massive flare (enhances the effect of other burning metals enormously) | Stores Connection (spiritual bonds, language, relationships) | Steals Connection / Identity [VERIFY] |
| 15 | **Chromium** | — | External | Pull | Wipes all of another person's metal reserves on touch (Leeching) | Stores Fortune (luck) | Steals destiny [VERIFY — Hemalurgic Chromium poorly documented] |
| 16 | **Nicrosil** | Chromium alloy | External | Push | Causes another person's metals to flare in a single massive burst on touch (Nicroburst) | Stores Investiture itself | Steals Investiture [VERIFY] |

---

## God Metals (Outside the Standard Table)

| Metal | Origin | Allomantic Power | Feruchemical Power | Hemalurgic Use |
|-------|--------|------------------|--------------------|----------------|
| **Atium** | Ruin's body (condensed in the Pits of Hathsin) | Reveals an opponent's movements a few seconds before they happen (combat precognition) | Stores age (younger while tapping, ages while storing) | Steals any power; the "universal" Hemalurgic spike. Extremely effective. |
| **Lerasium** | Preservation's body | Transforms a person into a Mistborn (grants full Allomantic ability) | Unknown / not documented in published works | Unknown / not documented |
| **Harmonium (Ettmetal)** | Harmony's body (Sazed) | Explosive when in contact with water. Not burned in conventional sense. | Not documented | Not documented |
| **Malatium** | Atium-gold alloy ("the Eleventh Metal") | Shows alternate past selves of another person (external version of Gold) | Not documented | Not documented |

---

## Grid Layout (Puzzle-Useful)

The 4x4 grid can be laid out as follows for encoding:

```
              PULLING              PUSHING
           ┌────────────┬────────────┐
EXTERNAL   │  1. Iron    │  2. Steel   │  PHYSICAL
INTERNAL   │  3. Tin     │  4. Pewter  │
           ├────────────┼────────────┤
EXTERNAL   │  5. Zinc    │  6. Brass   │  MENTAL
INTERNAL   │  7. Copper  │  8. Bronze  │
           ├────────────┼────────────┤
EXTERNAL   │ 11. Cadmium │ 12. Bendalloy│  TEMPORAL
INTERNAL   │  9. Gold    │ 10. Electrum│
           ├────────────┼────────────┤
EXTERNAL   │ 15. Chromium│ 16. Nicrosil│  ENHANCEMENT
INTERNAL   │ 13. Aluminum│ 14. Duralumin│
           └────────────┴────────────┘
```

Note: The numbering above follows the conventional Allomancy table order. The grid positions are the canonical arrangement from the Ars Arcanum.

---

## Notable Allomancers

### Mistborn (can burn all 16 metals)

| Character | Era | Notes |
|-----------|-----|-------|
| **Vin** | Era 1 | Protagonist of the original Mistborn trilogy. Street urchin turned Mistborn. |
| **Kelsier** | Era 1 | The Survivor of Hathsin. Led the rebellion against the Lord Ruler. |
| **The Lord Ruler (Rashek)** | Era 1 | Both a full Mistborn and a full Feruchemist — a Fullborn. Ruled for 1000 years. |
| **Elend Venture** | Era 1 | Became Mistborn by burning lerasium (Well of Ascension). |
| **Spook** | Era 1 | Originally a Misting (tin only), became Mistborn through Hemalurgy/Harmony's intervention at end of Era 1. |

### Mistings (burn one metal only)

| Character | Metal | Misting Name | Era | Notes |
|-----------|-------|-------------|-----|-------|
| **Waxillium Ladrian (Wax)** | Steel | Coinshot | Era 2 | Also a Feruchemist (iron) — a Twinborn. |
| **Wayne** | Bendalloy | Slider | Era 2 | Also a Feruchemist (gold/health) — a Twinborn. |
| **Marasi Colms** | Cadmium | Pulser | Era 2 | Slows time in a bubble. |
| **Ham** | Pewter | Pewterarm (Thug) | Era 1 | Member of Kelsier's crew. |
| **Breeze** | Brass | Soother | Era 1 | Member of Kelsier's crew. |
| **Clubs** | Copper | Smoker | Era 1 | Member of Kelsier's crew. Hid the crew's Allomantic pulses. |
| **OreSeur / TenSoon** | N/A | N/A | Era 1 | Kandra, not Allomancers — but relevant to metallic arts through Hemalurgy (kandra use Blessings — Hemalurgic spikes). |
| **Marsh** | Bronze (originally) | Seeker | Era 1 | Became a Steel Inquisitor through Hemalurgy. |

### Misting Type Names

| Metal | Misting Name |
|-------|-------------|
| Iron | Lurcher |
| Steel | Coinshot |
| Tin | Tineye |
| Pewter | Pewterarm (Thug) |
| Zinc | Rioter |
| Brass | Soother |
| Copper | Smoker |
| Bronze | Seeker |
| Gold | Augur |
| Electrum | Oracle |
| Cadmium | Pulser |
| Bendalloy | Slider |
| Aluminum | Aluminum Gnat (no useful power for Misting — wipes own reserves) |
| Duralumin | Duralumin Gnat (no useful power without other metals burning) |
| Chromium | Leecher |
| Nicrosil | Nicroburst |

---

## Allomantic Symbols

Each metal has a unique Allomantic symbol used in the Steel Alphabet. These symbols appear on the Allomancy poster and in the Ars Arcanum. The symbols could serve as a visual encoding layer for puzzles.

---

## Puzzle Affordances

- **16 metals = 16 letters** — direct A1Z26-style encoding (Iron=A, Steel=B, ..., Nicrosil=P — or any mapping)
- **4x4 grid** — row/column coordinates encode letter pairs
- **Paired metals** — each base metal has an alloy partner (Iron/Steel, Tin/Pewter, etc.) — 8 pairs
- **Three arts on one metal** — Allomancy, Feruchemy, and Hemalurgy each do something different with the same metal
- **Internal vs External, Push vs Pull** — binary properties for encoding
- **Misting names** — a second set of 16 unique names mapped to metals
- **God metals** — bonus entries outside the grid for special extraction
