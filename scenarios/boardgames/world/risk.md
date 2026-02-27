# Risk — World Data

Source material for Module M3 (The Rebel).

---

## Territories by Continent

### North America (9 territories) — Bonus: +5 armies

| # | Territory | Borders |
|---|-----------|---------|
| 1 | Alaska | Kamchatka, Northwest Territory, Alberta |
| 2 | Northwest Territory | Alaska, Alberta, Ontario, Greenland |
| 3 | Greenland | Northwest Territory, Ontario, Quebec, Iceland |
| 4 | Alberta | Alaska, Northwest Territory, Ontario, Western United States |
| 5 | Ontario | Northwest Territory, Alberta, Quebec, Greenland, Western United States, Eastern United States |
| 6 | Quebec | Ontario, Greenland, Eastern United States |
| 7 | Western United States | Alberta, Ontario, Eastern United States, Central America |
| 8 | Eastern United States | Ontario, Quebec, Western United States, Central America |
| 9 | Central America | Western United States, Eastern United States, Venezuela |

### South America (4 territories) — Bonus: +2 armies

| # | Territory | Borders |
|---|-----------|---------|
| 1 | Venezuela | Central America, Brazil, Peru |
| 2 | Brazil | Venezuela, Peru, Argentina, North Africa |
| 3 | Peru | Venezuela, Brazil, Argentina |
| 4 | Argentina | Peru, Brazil |

### Europe (7 territories) — Bonus: +5 armies

| # | Territory | Borders |
|---|-----------|---------|
| 1 | Iceland | Greenland, Great Britain, Scandinavia |
| 2 | Great Britain | Iceland, Scandinavia, Western Europe, Northern Europe |
| 3 | Scandinavia | Iceland, Great Britain, Northern Europe, Ukraine |
| 4 | Northern Europe | Great Britain, Scandinavia, Western Europe, Southern Europe, Ukraine |
| 5 | Western Europe | Great Britain, Northern Europe, Southern Europe, North Africa |
| 6 | Southern Europe | Northern Europe, Western Europe, Ukraine, Middle East, Egypt, North Africa |
| 7 | Ukraine | Scandinavia, Northern Europe, Southern Europe, Ural, Afghanistan, Middle East |

### Africa (6 territories) — Bonus: +3 armies

| # | Territory | Borders |
|---|-----------|---------|
| 1 | North Africa | Brazil, Western Europe, Southern Europe, Egypt, East Africa, Congo |
| 2 | Egypt | North Africa, Southern Europe, Middle East, East Africa |
| 3 | East Africa | North Africa, Egypt, Congo, South Africa, Madagascar, Middle East |
| 4 | Congo | North Africa, East Africa, South Africa |
| 5 | South Africa | Congo, East Africa, Madagascar |
| 6 | Madagascar | East Africa, South Africa |

### Asia (12 territories) — Bonus: +7 armies

| # | Territory | Borders |
|---|-----------|---------|
| 1 | Ural | Ukraine, Afghanistan, Siberia, China |
| 2 | Afghanistan | Ukraine, Ural, Middle East, India, China |
| 3 | Middle East | Southern Europe, Ukraine, Egypt, East Africa, Afghanistan, India |
| 4 | India | Afghanistan, Middle East, China, Siam |
| 5 | Siberia | Ural, China, Mongolia, Irkutsk, Yakutsk |
| 6 | China | Ural, Afghanistan, India, Siberia, Mongolia, Siam |
| 7 | Mongolia | Siberia, China, Irkutsk, Japan, Kamchatka |
| 8 | Siam | India, China, Indonesia |
| 9 | Irkutsk | Siberia, Mongolia, Yakutsk, Kamchatka |
| 10 | Yakutsk | Siberia, Irkutsk, Kamchatka |
| 11 | Kamchatka | Yakutsk, Irkutsk, Mongolia, Japan, Alaska |
| 12 | Japan | Mongolia, Kamchatka |

### Australia (4 territories) — Bonus: +2 armies

| # | Territory | Borders |
|---|-----------|---------|
| 1 | Indonesia | Siam, New Guinea, Western Australia |
| 2 | New Guinea | Indonesia, Eastern Australia, Western Australia |
| 3 | Western Australia | Indonesia, New Guinea, Eastern Australia |
| 4 | Eastern Australia | New Guinea, Western Australia |

---

## Continent Summary

| Continent | Territories | Bonus | Bonus per territory | Border territories (entry points) |
|-----------|------------|-------|--------------------|---------------------------------|
| North America | 9 | +5 | 0.56 | 3 (Alaska, Greenland, Central America) |
| South America | 4 | +2 | 0.50 | 2 (Venezuela, Brazil) |
| Europe | 7 | +5 | 0.71 | 4 (Iceland, Western Europe, Southern Europe, Ukraine) |
| Africa | 6 | +3 | 0.50 | 3 (North Africa, Egypt, East Africa) |
| Asia | 12 | +7 | 0.58 | 5 (Ural, Afghanistan, Middle East, Siam, Kamchatka) |
| Australia | 4 | +2 | 0.50 | 1 (Indonesia) |
| **Total** | **42** | — | — | — |

---

## Combat Rules

### Attacking

- Attacker must have 2+ armies in attacking territory (at least 1 must stay behind)
- Attacker rolls 1, 2, or 3 red dice (max = armies committed, up to 3)
- Must attack from an adjacent territory

### Defending

- Defender rolls 1 or 2 white dice (max = armies in defending territory, up to 2)

### Resolution

1. Sort attacker's dice highest to lowest
2. Sort defender's dice highest to lowest
3. Compare highest vs highest, second-highest vs second-highest
4. **Ties go to the defender**
5. Each comparison: losing side removes 1 army

### Combat Probability (3 attackers vs 2 defenders)

| Outcome | Probability |
|---------|-------------|
| Attacker loses 2 | 29.3% |
| Each loses 1 | 33.6% |
| Defender loses 2 | 37.2% |

[VERIFY: exact percentages — commonly cited from probability analysis]

### Key Probability Facts

- Attacker with 3 dice vs defender with 2 dice: attacker has slight edge per round
- Attacker with 3 dice vs defender with 1 die: attacker wins ~66% of comparisons [VERIFY]
- Large armies attacking small garrisons: attacker overwhelmingly favored over multiple rounds
- Defender's advantage: ties go to defender; fewer dice needed

---

## Card Sets and Reinforcements

### Territory Cards

- 42 territory cards (one per territory) + 2 wild cards = 44 total
- Each territory card shows: territory name + one of three symbols (Infantry, Cavalry, Artillery)
- Earn 1 card per turn IF you conquered at least 1 territory that turn

### Card Sets (trade in for armies)

| Set | Composition | Armies received (progressive) |
|-----|------------|-------------------------------|
| 3 Infantry | ♟♟♟ | 1st set: 4, 2nd: 6, 3rd: 8, 4th: 10, 5th: 12, 6th: 15, then +5 each |
| 3 Cavalry | ♞♞♞ | Same progressive scale |
| 3 Artillery | ♜♜♜ | Same progressive scale |
| 1 of each | ♟♞♜ | Same progressive scale |
| Any 2 + Wild | any + any + Wild | Same progressive scale |

[VERIFY: exact progressive army values — varies by edition. Above is the classic (pre-2008) scale.]

### Reinforcement (start of turn)

| Source | Armies received |
|--------|----------------|
| Territory count | max(3, territories / 3) rounded down |
| Continent bonus | See continent table above |
| Card set | Progressive scale (see above) |

---

## Fortification

- At end of turn, you may move armies from one territory to ONE adjacent territory you control
- Classic rules: one move only
- Some editions allow chain movement through connected territories [VERIFY: edition-dependent]

---

## Game Setup

| Player count | Starting armies per player |
|-------------|--------------------------|
| 2 | 40 (variant — not official) |
| 3 | 35 |
| 4 | 30 |
| 5 | 25 |
| 6 | 20 |

---

## Cross-Continent Connections (oceanic borders)

These territory pairs are adjacent despite being on different continents:

| From | To | Continents |
|------|----|-----------|
| Alaska | Kamchatka | North America ↔ Asia |
| Greenland | Iceland | North America ↔ Europe |
| Central America | Venezuela | North America ↔ South America |
| Brazil | North Africa | South America ↔ Africa |
| Western Europe | North Africa | Europe ↔ Africa |
| Southern Europe | North Africa | Europe ↔ Africa |
| Southern Europe | Egypt | Europe ↔ Africa |
| Southern Europe | Middle East | Europe ↔ Asia |
| Ukraine | Ural | Europe ↔ Asia |
| Ukraine | Afghanistan | Europe ↔ Asia |
| Ukraine | Middle East | Europe ↔ Asia |
| Egypt | Middle East | Africa ↔ Asia |
| East Africa | Middle East | Africa ↔ Asia |
| Siam | Indonesia | Asia ↔ Australia |
