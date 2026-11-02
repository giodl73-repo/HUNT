# HUNT planner corresponding software source

The browser's `source.zip` is a complete, buildable portable SOFTWARE subset at
the recorded Git commit. It contains every tracked file under
`tools/hunt-sim/core/` and `browser/planner/`, this scope document and HUNT's
original split `LICENSE`. It preserves repository-relative paths and both
standalone Cargo workspaces. It is deliberately not a full HUNT repository
archive: the native CLI and scenario/content trees include answer material and
are outside the browser's delivery/custody boundary.

The portable core is exactly the library used by the native CLI. The subset
does not include the native `tools/hunt-sim/Cargo.toml` or its `main.rs` and does
not require them to build the browser or its native control adapter. RALLY is
pinned at f64d8d4a5a04d8890ebcb9448ce28eb4f25861e7 through Cargo Git dependency
and committed locks. Other dependencies are pinned in the companion lock.
Source is not a vendored/offline package; Rust, Cargo registry/Git sources,
Node/Playwright and the pinned binding tool must be available.

Extract into any directory, including one with spaces; follow
`browser/planner/BUILD.md`. The build helper compiles the native control adapter
and the same WASM engine. Tests consume newly generated synthetic control cases.
Git-based release packaging and the native foundation/known-answer guards are
full-checkout maintainer tools; they are not required to build this subset.
Release qualification independently rebuilds this delivered subset outside Git,
compares exact native and WASM control outcomes and replays behavior checks.

Source bytes, ZIP SHA, commit, resolved dependency inventory, artifact inventory,
binding/compiler versions and Rust library notices are linked independently
from the app. This is corresponding source for the bounded planner, not the
generic HUNT toolkit distribution, Game Night adapter or solver-facing hunt.

HUNT's original license grants remain intact: source code, executable scripts,
tests, configuration and ordinary software documentation are MIT; original
non-software content is CC BY-NC 4.0 unless otherwise marked. No scenario assets
are added to this bundle. Third-party dependencies retain their own terms.
