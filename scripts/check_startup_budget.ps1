#!/usr/bin/env pwsh
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)

if ($args -and ($args[0] -eq "-h" -or $args[0] -eq "--help")) {
    Write-Output "Usage: scripts/check_startup_budget.ps1 [binary_path]"
    Write-Output "Checks startup budget of jcode binary using bench_startup.py"
    exit 0
}

$binary = if ($args) { $args[0] } else { "$repoRoot/target/release/jcode.exe" }

if (-not (Test-Path $binary)) {
    Write-Error "Binary not found: $binary"
    Write-Error "Build it first with: cargo build --release"
    exit 1
}

& python3 "$repoRoot\scripts\bench_startup.py" $binary --check --runs 3
exit $LASTEXITCODE
