#!/usr/bin/env pwsh
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$cargoExec = "$repoRoot/scripts/cargo_exec.ps1"

function Invoke-Cargo {
    param([string[]]$CargoArgs)
    Push-Location $repoRoot
    try {
        & $cargoExec @CargoArgs
    } finally {
        Pop-Location
    }
}

Write-Output "=== Phase 1 Refactor Verification ==="

Write-Output "[1/7] Isolated environment sanity"
& "$repoRoot/scripts/refactor_shadow.ps1" check

Write-Output "[2/7] Build (debug)"
& "$repoRoot\scripts\refactor_shadow.ps1" build

Write-Output "[3/7] Compile + budgets"
Invoke-Cargo check -q
& "$repoRoot/scripts/check_warning_budget.ps1"
python3 "$repoRoot/scripts/check_code_size_budget.py"

Write-Output "[4/7] Security preflight"
& "$repoRoot\scripts\security_preflight.ps1"

Write-Output "[5/7] Full tests"
Invoke-Cargo test -q

Write-Output "[6/7] E2E tests"
Invoke-Cargo test --test e2e -q

Write-Output "[7/7] All-targets/all-features lint"
Invoke-Cargo check --all-targets --all-features
Invoke-Cargo clippy --all-targets --all-features -- -D warnings

Write-Output "=== Phase 1 verification passed ==="
