# Build the HUNT planner prototype

Use Rust 1.95.0 (`wasm32-unknown-unknown` target), wasm-bindgen CLI 0.2.127,
Python 3.11+ and Node 22 with the committed Playwright 1.61.1 lockfile.
From the repository root:

```powershell
python browser/planner/build-prototype.py --bindgen C:/path/to/wasm-bindgen.exe
cd browser/planner
npm ci
npx playwright install chromium
npm run test:pages
```

The build uses committed Cargo lockfiles and the same pinned RALLY source
revision as the existing native CLI. `CARGO_TARGET_DIR` can point at a shared
external build cache; no TRACKER-relative paths are needed. Output is the owned
`target/hunt-prototype/HUNT` directory, not source control. WASM linear memory is
bounded at 64 MiB. The Python helper rebuilds both host and WASM and regenerates
native control outcomes. QA files in `.cache` are not public assets.

Full portable-source packaging, replay, dependency notices and default-branch
Pages artifact qualification remain a subsequent release step.
