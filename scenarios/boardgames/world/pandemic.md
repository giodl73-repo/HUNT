# Pandemic — World Data

Source material for Module M4 (The Collaborator).

---

## Overview

Cooperative game. 2-4 players work together against 4 diseases spreading across the world. Win by curing all 4 diseases. Lose if outbreaks reach 8, a disease cube supply runs out, or the player draw pile is exhausted.

---

## The 4 Diseases

| Color | Cube supply | Regions (thematic) |
|-------|-------------|-------------------|
| Blue | 24 cubes | North America, Europe |
| Yellow | 24 cubes | South America, Africa |
| Black | 24 cubes | Middle East, South/Central Asia |
| Red | 24 cubes | East Asia, Australasia |

Note: Diseases have no official names in the base game — only colors. Some editions include named diseases.

---

## Roles and Special Abilities

7 roles in the base game (each player draws one randomly):

| Role | Card color | Special ability |
|------|-----------|-----------------|
| **Medic** | Orange | **Treat Disease**: Remove ALL cubes of one color (not just 1). If disease is cured, automatically removes cubes from any city entered — no action needed. |
| **Scientist** | White | **Discover a Cure**: Needs only 4 cards of the same color (instead of 5) at a research station. |
| **Researcher** | Brown | **Share Knowledge**: Can give any City card to another player in the same city (not just the card matching the city). |
| **Operations Expert** | Light green | **Build**: Can build a research station in current city without discarding the matching City card. Once per turn, can move from a research station to any city by discarding any City card. |
| **Dispatcher** | Pink/magenta | **Move others**: Can move any player's pawn as if it were their own (using that player's cards for direct/charter flights). Can move any pawn to any city containing another pawn. |
| **Quarantine Specialist** | Dark green | **Prevent**: Prevents both placement AND outbreak of disease cubes in the city they occupy AND all cities connected to it. |
| **Contingency Planner** | Teal/cyan | **Recycle**: Can pick up a discarded Event card and store it (doesn't count toward hand limit). Play it as normal, then remove it from the game. |

---

## Actions

Each player gets **4 actions per turn**. Actions:

### Movement Actions

| Action | Cost | Effect |
|--------|------|--------|
| **Drive/Ferry** | 1 action | Move to an adjacent (connected) city |
| **Direct Flight** | 1 action + discard matching City card | Fly to the city named on the discarded card |
| **Charter Flight** | 1 action + discard current city's card | Fly to ANY city |
| **Shuttle Flight** | 1 action | Fly between two research stations |

### Other Actions

| Action | Cost | Effect |
|--------|------|--------|
| **Build Research Station** | 1 action + discard current city's card | Place research station in current city (max 6 on board) |
| **Treat Disease** | 1 action | Remove 1 disease cube from current city (or all if cured — see Medic) |
| **Share Knowledge** | 1 action | Give or take the City card matching your current city (both players must be there) |
| **Discover a Cure** | 1 action + discard 5 cards of same color | Cure that disease (must be at a research station). 4 cards for Scientist. |

---

## Infection Phase

After each player's turn:

1. **Draw infection cards** equal to the current infection rate
2. Each infection card names a city — place 1 disease cube of that city's color there
3. If a city already has 3 cubes of that color → **Outbreak**

### Infection Rate Track

| Position on track | Infection cards drawn per turn |
|-------------------|-------------------------------|
| 1-3 (starting) | 2 |
| 4-5 | 3 |
| 6-7 | 4 |

The infection rate marker advances each time an Epidemic card is drawn.

---

## Outbreaks

When a 4th cube of the same color would be placed in a city:

1. **No 4th cube placed** in that city
2. Place 1 cube of that color in EVERY adjacent city
3. If any adjacent city ALSO reaches 4 cubes → **chain outbreak**
4. Each city can only outbreak once per chain (prevents infinite loops)
5. Move outbreak marker up by 1 for each outbreak in the chain
6. **8 outbreaks = game over (loss)**

### Chain Outbreak Example

```
Suppose: City A has 3 blue cubes. City B (adjacent to A) has 3 blue cubes.
         City C (adjacent to B but not A) has 2 blue cubes.

Event: Infection card draws City A.
  → City A would get 4th blue cube → OUTBREAK
  → 1 blue cube added to each neighbor of A, including City B
  → City B now would get 4th blue cube → CHAIN OUTBREAK
  → 1 blue cube added to each neighbor of B, including City C
  → City C now has 3 blue cubes (no outbreak — under limit)
  → City A does NOT get another cube from B's outbreak (already outbroke this chain)
  → Outbreak marker moves from 0 → 2 (two outbreaks in one chain)
```

---

## Epidemic Cards

Mixed into the player draw deck. When drawn:

1. **Increase**: Advance infection rate marker by 1
2. **Infect**: Draw the BOTTOM card of the infection deck — place 3 cubes in that city (may cause outbreak)
3. **Intensify**: Shuffle the infection DISCARD pile and place it ON TOP of the infection deck

### Why Intensify Matters

After an epidemic, cities that were ALREADY infected are now on top of the deck — they will be drawn again soon. This creates a "pressure cooker" effect where previously infected cities keep getting reinfected.

### Difficulty Settings

| Difficulty | Epidemic cards in deck |
|------------|----------------------|
| Introductory | 4 |
| Standard | 5 |
| Heroic | 6 |

---

## Cure vs Eradicate

| State | Condition | Effect |
|-------|-----------|--------|
| **Uncured** | Default | Must treat cubes manually (1 per action) |
| **Cured** | 5 same-color cards discarded at research station | Medic auto-removes. Treat Disease removes all cubes (not just 1) from that color. Disease still spreads. |
| **Eradicated** | Cured AND all cubes of that color removed from board | No new cubes of that color are ever placed. Infection cards of that color are ignored. |

---

## Win and Loss Conditions

### Win
- Discover cures for ALL 4 diseases (does not require eradication)

### Loss (any one of three)
1. **8th outbreak** occurs
2. **Cube supply runs out** — need to place a cube but none of that color remain in supply
3. **Player draw pile exhausted** — cannot draw 2 player cards at end of turn

---

## Player Cards

The player draw deck contains:
- **City cards** (48) — one per city on the board, color-coded to disease
- **Event cards** (5 in base game) — special one-time actions, can be played at any time (not just your turn)
- **Epidemic cards** (4-6) — shuffled evenly into the deck

### Event Cards (Base Game)

| Event | Effect |
|-------|--------|
| **One Quiet Night** | Skip the next infection phase |
| **Forecast** | Look at the top 6 infection cards; rearrange them in any order |
| **Government Grant** | Build a research station in any city (no card discard needed) |
| **Airlift** | Move any pawn to any city |
| **Resilient Population** | Remove 1 card from the infection discard pile (permanently) |

---

## Board Geography

48 cities connected by routes. Starting research station: Atlanta (CDC headquarters).

### Cities by Disease Color

**Blue (12 cities)**: San Francisco, Chicago, Montreal, New York, Washington, Atlanta, London, Madrid, Paris, Essen, St. Petersburg, Milan

**Yellow (12 cities)**: Los Angeles, Miami, Mexico City, Bogota, Lima, Santiago, Buenos Aires, Sao Paulo, Lagos, Kinshasa, Johannesburg, Khartoum

**Black (12 cities)**: Algiers, Cairo, Istanbul, Moscow, Baghdad, Riyadh, Tehran, Karachi, Mumbai, Delhi, Chennai, Kolkata

**Red (12 cities)**: Beijing, Seoul, Tokyo, Shanghai, Taipei, Hong Kong, Bangkok, Ho Chi Minh City, Manila, Jakarta, Osaka, Sydney

[VERIFY: exact city list — above is from memory of the standard 2013 edition]

---

## Hand Limit

- Each player: maximum **7 cards** in hand
- Must discard immediately when exceeding 7 (after drawing)
- Event cards count toward hand limit (unless held by Contingency Planner's special ability)
