#!/usr/bin/env pwsh
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$cargoExec = "$repoRoot\scripts\cargo_exec.ps1"

function Invoke-Cargo {
    param([string[]]$CargoArgs)
    Push-Location $repoRoot
    try {
        & $cargoExec @CargoArgs
    } finally {
        Pop-Location
    }
}

Write-Output "=== Fast test loop (lib + bins) ==="
Invoke-Cargo test --lib --bins @args

Write-Output ""
$binaryPath = "$repoRoot/target/release/jcode.exe"
if (Test-Path $binaryPath) {
    Write-Output "=== Startup regression check (release binary) ==="
    & "$repoRoot\scripts\check_startup_budget.ps1" $binaryPath
    Write-Output ""
} else {
    Write-Output "Skipping startup regression check: build release first with cargo build --release"
    Write-Output ""
}

Write-Output "For full coverage, run: scripts/test_e2e.ps1"
