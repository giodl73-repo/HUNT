# Review: Profile Depth and the Reviewing-to-Authoring Transfer Problem: Evidence from AI Expert Personas in Puzzle Hunt Design
**Reviewer:** James Lester
**Expertise:** AI in games, narrative intelligence, game-based learning, interactive narrative systems
**Venue:** ICCC 2026

## Scores (1-4)
- Novelty: 3/4
- Rigor: 3/4
- Clarity: 3/4
- Significance: 2/4
- **Overall: 2.75/4**

## Overall Assessment

The paper reports a clean and carefully designed empirical study with a valid and interesting main finding. My concern is domain generalizability: puzzle hunt design is a highly constrained, rule-governed creative domain with a single verifiable extraction answer per puzzle, an established expert community with published design philosophies, and a closed domain corpus. These properties make it an excellent testbed for controlled ablation studies but raise the question of whether findings transfer to other game design contexts where correctness is not so cleanly defined, expert personas are not so readily grounded in published practice, and mechanisms are not so precisely enumerable. The paper's claims are scoped appropriately in the limitations section, but the significance assessment depends on whether the lens-type transfer model and symmetric-gradient finding travel to other domains.

## Overall Assessment

A solid contribution to AI-assisted game content generation. The methodology is unusually careful, the format ceiling finding is a genuine insight, and the practical guidance for profile construction is valuable for practitioners. The main limitation is that the generalizability question — which I consider central to the paper's significance claim — is left largely to future work.

## Strengths

- The experimental design is the strongest aspect of the paper. The fixed answer word, fixed mechanism, locked domain corpus, and same-session calibration protocol together make cross-condition comparisons more credible than most studies in this area.
- The Riven Standard dimension and the format ceiling finding are the paper's most practically useful contributions for game content generation. The insight that mechanism choice determines the ceiling while profile depth determines the floor is directly applicable to any AI-assisted game design pipeline.
- The lens-type taxonomy (construction/operational/perceptual) is a useful vocabulary for practitioners thinking about how to design AI authoring profiles for different game design roles — a level designer's reviewing profile will look different from a narrative designer's, and the taxonomy gives a way to think about which properties will transfer.
- The A2 domain-inversion finding has direct practical implications: supplying domain data without a design framework to an AI authoring system is a common and expensive mistake, and the paper provides clean evidence that framework is the operative variable.
- The Young A6 result — the highest-scoring puzzle in the corpus — illustrates that perceptual instincts, when properly adapted to the available format, can produce the most domain-integrated outputs. This is a useful result for game design AI specifically.

## Major Concerns (P1 — blocking)

**P1.1: The testbed's specific properties may not generalize to most game design contexts.**
Puzzle hunt design is unusually well-suited to this study: single correct extraction answer (correctness is verifiable), published designer philosophies (the three experts' profiles are grounded in real practice), and a closed domain corpus (world data is lockable). Most game content generation contexts lack one or more of these properties. A quest in an RPG does not have a single verifiable correct answer; a level in a platformer does not have a published designer philosophy that can be extracted and repurposed; an NPC dialogue tree does not have a closed domain corpus. The paper should address which of its findings are likely to generalize to game content generation contexts that lack these properties, and which depend on them. Currently this is addressed only implicitly in the limitations section.

**P1.2: The AoE2 domain is well-chosen for control but is an unusual creative production context.**
AoE2 is a strategy game with a large, structured, and well-documented domain. Its civilization roster, unit statistics, and technology tree form exactly the kind of closed enumerable world data that supports first-letter acrostic extraction. Most game content generation tasks do not involve enumerable world data of this type. The paper's practical guidance for profile construction (Section 5.5, "The Lens-Type Transfer Model") will be most useful to practitioners working with similarly structured domains. A brief discussion of what "world data" looks like in less structured domains — open-world games, procedurally generated settings, character-driven narratives — would help readers calibrate the practical applicability.

**P1.3: The rubric's game design relevance is underspecified.**
The 45-point rubric is well-described, but its relationship to game design quality criteria more broadly is not established. Clarity, Solvability, Elegance, Reading Reward, Fun, Confirmation, and Riven Standard are puzzle hunt criteria. Fun is the closest to a general game quality criterion, but it is single-weighted and the least operationally defined. A game-focused venue reviewer would want to know how these dimensions map onto game design quality frameworks (playability, engagement, narrative coherence, replayability). Even a brief note on which dimensions are puzzle-hunt-specific and which might generalize would help.

## Minor Issues (P2 — important but not blocking)

**P2.1: The "same three reviewers authored and evaluated" problem.** The paper notes that Katz, Snyder, and Young served as both the evaluation panel (C8 tier) and the authoring personas (A5/A6 conditions). Even with trimmed credentials and injected principles, this creates a potential circularity: the evaluation criteria are constructed by the same experts whose authoring profiles are being evaluated. A brief discussion of whether this introduces systematic bias — and whether any conditions are more affected than others — would strengthen the methodology discussion.

**P2.2: The "companion persona study" is referenced for the artist-persona finding but not described.** Section 5.3 references a result from a companion paper where a bare "artist" persona produced a higher Riven Standard score than any A-series puzzle by generating a domain-native mechanism. This is presented as important evidence for the format ceiling argument, but since the paper relies on this result to motivate the ceiling finding, readers need at least a brief description of the artist persona and its mechanism. Citing an unreleased companion paper for a load-bearing result is a methodological issue.

**P2.3: The pass threshold (33/45, 73%) is not motivated.** Why does this threshold define "passing"? Is it derived from human benchmark data, from practitioner consensus, or is it constructed for this study? This threshold matters because no A-series condition passes, and this non-passing result is used to characterize the format ceiling. The ceiling claim would be strengthened if the pass threshold had an independent derivation.

**P2.4: The practical guidance in Section 5.2 (adapting perceptual profiles for authoring) is the most practically useful part of the paper but is too brief.** The recommendation — identify what the perceptual check is ultimately verifying and construct a generative instruction that can be satisfied during creation — is correct but needs one worked example to be actionable for practitioners. Young's narrative-grammar substitution could be the worked example.

## Recommendations (P3 — suggested improvements)

**P3.1:** Add a paragraph in Section 5 (or extend the limitations section) that explicitly discusses which findings are likely to generalize to non-puzzle game design contexts and which depend on the specific properties of the puzzle hunt testbed (verifiable answers, closed corpus, published designer philosophies). This would substantially increase the paper's practical significance for ICCC's AI-in-games readership.

**P3.2:** Extend the practical guidance section to include one concrete worked example of converting a perceptual reviewing profile into an authoring-compatible generative precursor, using Young's narrative-grammar substitution as the template. This would make the paper immediately usable by practitioners.

**P3.3:** Provide a brief description of the artist persona result (referenced in Section 5.3) either in the main text or in a footnote, rather than relying solely on the unreleased companion paper citation. The format ceiling argument currently depends on this result as evidence.

## Recommendation
Minor Revision
