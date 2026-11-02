# Six-feeder forecast and custody

This is HUNT's existing WAVELENGTH heuristic over six synthetic feeder positions,
compiled from the same `hunt-sim-core` library used by its native CLI. The browser
accepts numeric difficulty/answer-length/team assumptions and existing native
variant names. It accepts no answer words or puzzle content.

P5/P6 alone receive clue relief. Solve minutes are sorted then chunked by team
parallelism, not optimally scheduled. Five feeder answers are assumed ready for
the meta; there is no actual extraction or solve. Hint thresholds count pressure
and do not decrease minutes. The time target is 180 minutes. A finding-free run
is not a hunt passing a playtest or a stage gate.

The native run ID includes variant name, affecting the RNG stream. A variant
comparison combines parameter changes and different sampled noise; it does not
isolate a causal treatment effect. Repeating the exact request/build is testable;
cross-target outcomes must be measured before any identity claim.

Raw RALLY metric classification is retained: no strictly improved metrics means
`regressed`, including a complete tie. The UI explains that rule and shows all
summary values, deltas, findings and individual runs without recommending a hunt
for release. Blind solver/editorial reviews remain separate.

The browser package excludes native scenario data, Game Night answers, scenario
assets, research, examples, reference and hidden-layer material. Its source
download will be the complete buildable portable core and browser companion,
with pinned RALLY Git dependency and lockfiles, rather than a full repository
archive. HUNT's split MIT software / CC BY-NC content grants remain intact.
An explicit file allowlist and native-known-answer scan must qualify delivery.

This prototype is not the generic HUNT toolkit's clean-install acceptance or a
solver-facing scenario release. Its actual Pages/source packaging and final
qualification are pending.
