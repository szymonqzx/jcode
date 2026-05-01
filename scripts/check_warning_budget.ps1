#!/usr/bin/env pwsh
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$baselineFile = "$repoRoot\scripts\warning_budget.txt"

function Show-Usage {
    Write-Output @"
Usage:
  scripts/check_warning_budget.ps1            # fail if warnings exceed baseline
  scripts/check_warning_budget.ps1 --update   # update baseline to current warning count

Notes:
  - Counts Rust compiler lines that begin with "warning:" from `cargo check -q`
  - Baseline is stored in scripts/warning_budget.txt
"@
}

if ($args -and ($args[0] -eq "-h" -or $args[0] -eq "--help")) {
    Show-Usage
    exit 0
}

if (-not (Test-Path $baselineFile)) {
    Write-Error "error: missing baseline file: $baselineFile"
    exit 1
}

$env:CARGO_TERM_COLOR = "never"
$current = & cargo check -q 2>&1 | Select-String "^warning:" | Measure-Object | Select-Object -ExpandProperty Count
if (-not $current) { $current = 0 }

$baseline = (Get-Content $baselineFile -Raw).Trim()

if ($args -and $args[0] -eq "--update") {
    $current | Out-File -FilePath $baselineFile -Encoding utf8
    Write-Output "Updated warning baseline: $baseline"
    Write-Output "New warning baseline: $current"
    exit 0
}

if ($baseline -notmatch "^\d+$") {
    Write-Error "error: invalid warning baseline in $baselineFile: '$baseline'"
    exit 1
}

if ($current -gt $baseline) {
    Write-Error "Warning budget exceeded: current=$current baseline=$baseline"
    Write-Error "Run scripts/check_warning_budget.ps1 --update only after intentional cleanup."
    exit 1
}

if ($current -lt $baseline) {
    Write-Output "Warning budget improved: current=$current baseline=$baseline"
    Write-Output "Consider running: scripts/check_warning_budget.ps1 --update"
} else {
    Write-Output "Warning budget OK: current=$current baseline=$baseline"
}
