# Condition P17 — Snyder's Computer Test

## Puzzle

Each clue below identifies a specific item from Age of Empires II: Definitive Edition
(a civilization, unit, technology, or map). Identify each item, then take its first
letter. The five first letters, read in order, spell the five-letter answer.

---

**1.**
Every technology at the Monastery either expands what Monks can target, increases
how far they reach, makes them faster, gives them more HP, or hardens your units
against enemy conversion. One technology in the full Monastery list does none of
these things for a player who always sends a single Monk to convert. Its benefit
vanishes entirely the moment you work your Monks one at a time. Name it.

**2.**
This Imperial Age siege unit's most distinctive capability does not target enemy
units, does not damage enemy buildings, and does not belong to any combat category.
It acts against the map itself — removing an obstacle that blocks movement but is
owned by no player and cannot be attacked by any other military unit. Name it.

**3.**
This civilization's team bonus speeds up a building, but that building produces units
of a class that the civilization's own unique unit does not belong to. Cross-reference
the team bonus, the building it affects, and the unit class that building trains —
then name the civilization's unique unit, the one whose class is the mismatch.

**4.**
In the counter triangle, cavalry is countered by a specific infantry sub-class.
That infantry sub-class is itself countered by archers. Archers are countered by
a dedicated anti-archer unit. Name the upgraded form of that anti-archer unit —
the version that first becomes available in Castle Age.

**5.**
Every Monastery technology affects either Monks directly or your units' resistance
to conversion. This is the only one whose effect would be completely useless in a
game where your opponent trained units exclusively and never constructed or deployed
anything that is not a unit. Name it.

---

Take the first letter of each answer in order (1 through 5).

**Answer:** _ _ _ _ _

---

## Solution Key

T: Theocracy
O: Onager
W: Woad Raider
E: Elite Skirmisher
R: Redemption

Answer: TOWER

---

## Principle Application

Each clue withholds the answer's name and defining keyword, requiring the solver to
complete a chain of inference: clue 1 uses exhaustive elimination across all nine
Monastery technologies to isolate the only one with a conditionally null effect;
clue 2 requires reasoning from "removes an obstacle owned by no player" to trees to
Onager without the words trees, forest, or fell appearing anywhere; clue 3 requires
matching a team bonus to a building to a unit class and then identifying the civ
whose unique unit is in a different class; clue 4 traverses two hops of the counter
triangle before arriving at the anti-archer unit and selects its Castle Age upgrade
tier; clue 5 requires reasoning about which conversion target category (buildings and
siege) is absent when an opponent trains only units, isolating Redemption as the one
tech whose target class never appears under that condition. A ten-line database lookup
script fails every clue because no clue contains a string that directly indexes the
answer in the world data.
