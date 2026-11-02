# Build the HUNT planner

Use Rust 1.95.0 (`wasm32-unknown-unknown` target), wasm-bindgen CLI 0.2.127,
Python 3.11+ and Node 22 with the committed Playwright 1.61.1 lockfile.
From the repository root:

```powershell
python browser/planner/build-prototype.py --bindgen C:/path/to/wasm-bindgen.exe
cd browser/planner
npm ci
npx playwright install chromium
$env:HUNT_SITE_DIR='../../target/hunt-prototype'
npm run test:pages
```

The build uses committed Cargo lockfiles and the same pinned RALLY source
revision as the existing native CLI. `CARGO_TARGET_DIR` can point at a shared
external build cache; no TRACKER-relative paths are needed. Output is the owned
`target/hunt-prototype/HUNT` directory, not source control. WASM linear memory is
bounded at 64 MiB. The Python helper rebuilds both host and WASM and regenerates
native control outcomes. QA files in `.cache` are not public assets.

When building the delivered portable source outside Git, run the twelve behavior
checks with `npm run test:pages -- --grep-invert "packaged delivery|source/legal"`;
the two delivery checks require a full packaged site. This rebuilds the same core
and synthetic outcomes with no native CLI/scenario files. See the included
`docs/planner-source-scope.md` from the ZIP root. Cargo/Node dependencies are
pinned but are not vendored; network or populated caches are required.

In a clean full HUNT checkout, maintainers run `package-site.py` after the build,
then `verify-site.py --commit <HEAD> target/hunt-release/HUNT`, the full fourteen
browser cases with default HUNT_SITE_DIR, and `verify-source.py --site
target/hunt-release/HUNT --bindgen <absolute tool path>`. The release verifier
checks every deployed/source byte and the native-known-answer custody boundary.
The source replayer extracts the exact delivered subset outside Git in a path
with spaces, tests the native adapter, rebuilds host/WASM, compares both complete
outcome sets separately and replays twelve behavior cases. Matching binary hashes
across paths/platforms are not required or claimed.
