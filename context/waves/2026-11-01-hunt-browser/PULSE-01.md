# Pulse 01 — shared model and real worker prototype

The existing feeder engine, summary, comparison and six variant constants now
live in standalone `tools/hunt-sim/core`, shared by both CLI and WASM. Twelve
remaining native functions and four extracted functions are token-exact except
explicit names/data-parameter bindings and formatting. Protected policy/grants
remain exact. RALLY stays at f64d8d4; its existing lock revision is now explicit
in both manifests, without upgrading the dependency. Game Night remains native.

Seven existing tests pass; 48 before/after native CLI outcomes are byte-exact.
Four adapter tests and 46 exact repeated host/WASM controls pass, plus four actual
browser cases. Raw private native scenario outputs stay outside committed/uploaded
evidence. The 242,313-byte WASM has a 64 MiB limit. Nine prototype assets pass an
explicit allowlist and five native-known-answer checks. No scenario/answer content
is packaged. Mobile/desktop captures were inspected; full report values remain
exact while the summary display rounds to one decimal.

The UI accepts own synthetic numeric assumptions, keeps original opened bytes
separate from current/LF/request/report, fences stale results and has recovery
mechanisms. Only initial behavior is qualified: expanded actual deadline/stale/
crash/resource coverage, source/legal packaging, source replay and hosted/default/
live custody remain open. This is not a Pages release or toolkit/hunt approval.

Native semantics stay visible: variants alter RNG stream through run ID, hints
count pressure without reducing solve time, sorted chunking is not optimal
scheduling, five-of-six meta readiness is assumed, and the raw RALLY classifier
calls ties regressed. No causal intervention, real playtest or readiness claim.

The initial visibility/build/helper/encoding/guard mistakes are retained in
`portability.json`; they were corrected without changing native formulas or
weakening assertions. No TRACKER pointer has advanced.
