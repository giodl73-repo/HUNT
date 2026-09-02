$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent $PSScriptRoot

function Read-RepoFile {
    param([string]$Path)
    $fullPath = Join-Path $repoRoot $Path
    if (-not (Test-Path -LiteralPath $fullPath)) {
        throw "Missing required file: $Path"
    }
    Get-Content -LiteralPath $fullPath -Raw
}

function Assert-Contains {
    param(
        [string]$Content,
        [string]$Needle,
        [string]$Label
    )
    if (-not $Content.Contains($Needle)) {
        throw "$Label missing expected text: $Needle"
    }
}

$pitfalls = Read-RepoFile ".pitfall/hunt-pitfalls.md"
$readme = Read-RepoFile "README.md"
$bugs = Read-RepoFile "BUGS.md"
$publish = Read-RepoFile "toolkit/skills/hunt/publish.md"
$author = Read-RepoFile "toolkit/skills/puzzle/author.md"
$principles = Read-RepoFile "toolkit/PRINCIPLES.md"
$gettingStarted = Read-RepoFile "toolkit/GETTING-STARTED.md"
$roles = Read-RepoFile ".roles/ROLE.md"
$boundaries = Read-RepoFile "docs/pitfall-boundaries.v1.json"

foreach ($id in @("HUNT-PF-02", "HUNT-PF-03", "HUNT-PF-04")) {
    Assert-Contains $pitfalls "## ${id}:" "$id section"
    foreach ($field in @(
        "**Actor:**",
        "**Task:**",
        "**Surface:**",
        "**Likely mistake:**",
        "**Consequence:**",
        "**Owner:**",
        "**Test:** ``pwsh -NoProfile -File tests\check-pitfall-policy.ps1``"
    )) {
        Assert-Contains $pitfalls $field "$id use-case fields"
    }
}

# HUNT-PF-02: scenario-specific success cannot be advertised as a generic toolkit contract.
Assert-Contains $readme "they are not a supported clean-install" "README packaging boundary"
Assert-Contains $readme "not currently a distributable portfolio dependency" "README reuse boundary"
Assert-Contains $readme "clean-install acceptance test ship" "README acceptance gate"
Assert-Contains $bugs "Path references need to be relative to the scenario" "BUGS scenario path coupling"
Assert-Contains $bugs "ASK the user to choose their own encoding" "BUGS answer protocol choice"
Assert-Contains $gettingStarted "/puzzle-plan" "getting-started command surface"
Assert-Contains $roles "Generic toolkit release" "roles generic toolkit PITFALL gate"
Assert-Contains $boundaries "scenario-success-is-not-generic-toolkit" "boundary generic toolkit rule"
Assert-Contains $boundaries "clean-install acceptance tests" "boundary clean-install evidence"

# HUNT-PF-03: answer, spoiler, and prohibited-content custody must block publish.
Assert-Contains $readme "Plaintext answers must never appear in git-tracked files" "README answer custody"
Assert-Contains $bugs "Plaintext answers must NEVER appear in git-tracked files" "BUGS plaintext answer blocker"
Assert-Contains $bugs "never lyric text" "BUGS lyric prohibition"
Assert-Contains $publish "Strips all working pipeline files and answer content" "publish scrub purpose"
Assert-Contains $publish "grep the output directory for known answer words" "publish answer grep"
Assert-Contains $author "NEVER create a plaintext answer key file in the repo" "author answer custody"
Assert-Contains $roles "Publish and answer custody" "roles publish custody PITFALL gate"
Assert-Contains $boundaries "answer-and-prohibited-content-custody" "boundary answer custody rule"
Assert-Contains $boundaries "prohibited lyric text" "boundary lyric prohibition"

# HUNT-PF-04: mechanism quality is not enough without letter-by-letter extraction.
Assert-Contains $bugs "Answer words must be validated against world data before briefs are issued" "BUGS brief feasibility"
Assert-Contains $author "verify extraction character by character" "author extraction check"
Assert-Contains $author "the extraction is broken" "author extraction failure rule"
Assert-Contains $principles "Verify the Last Mile" "principles last-mile rule"
Assert-Contains $principles "letter by letter" "principles letter-by-letter evidence"
Assert-Contains $roles "Brief and extraction feasibility" "roles extraction feasibility PITFALL gate"
Assert-Contains $boundaries "mechanism-quality-is-not-extraction-proof" "boundary extraction proof rule"
Assert-Contains $boundaries "verified character by character" "boundary character-level extraction"

Write-Output "HUNT PITFALL policy checks passed"
