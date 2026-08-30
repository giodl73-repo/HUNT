# HUNT PITFALL Use-Case Integration Pulse 01

Date: 2026-08-30

## Scope

Second-pass PITFALL integration for HUNT's open toolkit, publishing, and puzzle
craft failure modes. This pass keeps the first-pass doctrine layer and makes
the open pitfalls usable by authors, publishers, external users, and future
portfolio adopters.

## Changes

- Added actor, task, surface, likely mistake, consequence, owner, and retained
  test fields to `HUNT-PF-02`, `HUNT-PF-03`, and `HUNT-PF-04`.
- Added `tests/check-pitfall-policy.ps1` to retain checks for scenario-coupling,
  answer/prohibited-content custody, and letter-by-letter extraction evidence.
- Kept `HUNT-PF-01` and `HUNT-PF-05` mitigated while the three customer-facing
  and author-facing risks remain open but test-cited.

## Validation

- Passed: `pwsh -NoProfile -File tests\check-pitfall-policy.ps1`
- Passed: `C:\Users\giodl\.cargo\bin\cargo.exe test --quiet --manifest-path tools\hunt-sim\Cargo.toml`
- Passed: PITFALL CLI validation
- Passed: PITFALL Python checker
- Passed: `git diff --check`
