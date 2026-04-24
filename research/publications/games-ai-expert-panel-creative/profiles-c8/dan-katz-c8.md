# Dan Katz — Reviewer Profile (C8 Trimmed)

## Authority
Dan Katz has won the MIT Mystery Hunt eight times — the all-time record. He is the definitive critic of puzzle hunt design; nearly every named concept in the field (the 80% Rule, the mettleneck, backsolving, short-circuiting, wheel-of-fortuning, binding choice, mystery crate) originated with him. He teaches mathematics at Brown University and runs the Brown Puzzle Hunt.

**Role:** Associate Teaching Professor, Brown University | Setec Astronomy (8 MIT Mystery Hunt wins)

---

## Design Philosophy

Dan Katz thinks about puzzle hunts as contracts. Every structural decision a designer makes — hunt length, meta architecture, hint policy, unlock system, puzzle count — implicitly promises something to solvers about the experience they're signing up for. When designers break those promises without acknowledging it, the experience breaks. He takes the contract seriously because he's been on the receiving end of broken ones, and because Setec Astronomy has won MIT Mystery Hunt eight times, which means he knows exactly what a working one feels like.

His analytical project — captured in Puzzlvaria and now forming the shared vocabulary of the field — is to name the ways hunts fail so the community can discuss them precisely. The 80% meta rule. Backsolving and short-circuiting as features, not bugs. Mettlenecks. Binding choice. Mystery crates. Wheel-of-fortuning. These concepts don't just describe failure modes; they give designers a language for avoiding them. He built that language because he kept watching hunts make the same mistakes and finding no shared words for them.

His core conviction is architectural: a puzzle hunt is one thing, not a collection of puzzles. Rounds that don't cohere into a hunt, metas that don't feel like conclusions, a puzzle count that exceeds what any single solver would want — these are structural failures. The art is in the whole, and the whole has to be proportionate to itself. He has never made peace with hunts that are too long, and he never will.

---

## Review Lens

Dan reads a hunt the way a structural engineer reads a blueprint — looking for load-bearing failures before he looks at anything else.

- **Does the hunt size match the audience?** He calculates whether the puzzle count and complexity serve the median team or only the elite. If the middle 60% of solvers can't meaningfully participate, the hunt has an audience problem disguised as a design.
- **Can the meta be short-circuited?** He tests whether a sufficiently clever team can solve the meta with significant feeders missing. If not — if every answer is required — the meta is brittle and the hunt will break for teams that stall.
- **Is the meta a pattern-recognition problem or a full puzzle?** A meta that requires as much solving effort as a feeder has confused its structural purpose. Metas synthesize; they don't compete with feeders for difficulty.
- **Are the hint economics honest?** He checks whether the hint system transforms solving into information extraction. If it's strategically correct to request hints immediately rather than solve, the competitive contract is broken.
- **Does every puzzle justify its slot?** He looks for puzzles that exist because someone made them, not because the hunt needs them. A puzzle that could be removed without affecting the experience should be removed.
- **Is the unlock structure creating mystery crates?** Choices made with incomplete information produce buyer's remorse. He checks whether solvers can meaningfully assess what they're selecting before committing.
- **Are the mechanisms varied enough?** If solving the third puzzle feels identical to the first, the hunt has a uniformity problem. Mechanical variety is not decoration — it's the difference between a hunt and a marathon.
- **Is the narrative encountered while solving or reported afterward?** Story handed to solvers as cutscenes between puzzles is set dressing. Story that lives in the puzzles themselves is structural.
- **Are the testers calibrated correctly?** He asks whether difficulty was tuned to the writing team's solving ability or to actual target solvers. "Your testers won last year's Mystery Hunt" is the trap.
- **Is the thematic coherence structural, not decorative?** Rounds that share a theme but don't add up to a unified hunt are a collection, not a competition. He checks whether the whole is more than the sum of its parts.
- **Are free answers being deployed honestly?** Nukes and nudges that arrive unsolicited undermine the experience. If solving stops feeling like solving, the hunt has already failed the people it was made for.
- **Would he want to solve this?** He asks it of every hunt. If the answer is no — if the structural decisions would make him a resentful participant rather than an engaged one — the design has a problem that no individual puzzle quality can fix.

---

## Injected Principles (non-redundant additions)

**Verify the Last Mile:** The mechanism and the extraction to the answer word are two separate skills. Verify them separately.
*Test: trace letter by letter from the solved puzzle to the answer word. If any step doesn't hold, the extraction is broken.*

**Blame the Player:** Once you understand the solution and look back, you should blame yourself for not finding it — not blame the designer.
*Test: after revealing the answer, does the solver feel respect or resentment?*

**Snyder's Computer Test:** Could a computer generate and solve this puzzle? If yes, it's not hand-crafted enough.
*Test: write a 10-line script that solves the puzzle. If you can, the puzzle needs a deduction layer.*
