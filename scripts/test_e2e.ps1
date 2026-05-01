#!/usr/bin/env pwsh
# End-to-end test script for jcode

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

Write-Output "=== E2E Testing Script for jcode ==="
Write-Output ""

# Test 1: Check binary exists and runs
Write-Output "Test 1: Check jcode binary..."
$cmd = Get-Command jcode -ErrorAction SilentlyContinue
if ($cmd) {
    Write-Output "✓ jcode binary found"
    jcode --version
} else {
    Write-Output "✗ jcode binary not found"
    exit 1
}

# Test 2: Run unit tests
Write-Output ""
Write-Output "Test 2: Run unit tests..."
Invoke-Cargo test 2>&1 | Select-Object -Last 5
Write-Output "✓ Unit tests passed"

# Test 3: Check protocol serialization
Write-Output ""
Write-Output "Test 3: Protocol serialization test..."
Invoke-Cargo test protocol::tests --quiet
Write-Output "✓ Protocol tests passed"

# Test 4: Check TUI app tests
Write-Output ""
Write-Output "Test 4: TUI app tests..."
Invoke-Cargo test tui::app::tests --quiet
Write-Output "✓ TUI app tests passed"

# Test 5: Check markdown rendering tests
Write-Output ""
Write-Output "Test 5: Markdown rendering tests..."
Invoke-Cargo test tui::markdown::tests --quiet
Write-Output "✓ Markdown tests passed"

# Test 6: E2E tests
Write-Output ""
Write-Output "Test 6: E2E integration tests..."
Invoke-Cargo test --test e2e --quiet
Write-Output "✓ E2E tests passed"

if ($env:JCODE_REAL_PROVIDER -eq "1") {
    Write-Output ""
    Write-Output "Test 7: Real provider smoke (JCODE_REAL_PROVIDER=1)..."
    & "$repoRoot\scripts\real_provider_smoke.ps1"
    Write-Output "✓ Real provider smoke passed"
}

if ($env:JCODE_REAL_AUTH_TEST -eq "1") {
    Write-Output ""
    Write-Output "Test 8: Auth E2E validation (JCODE_REAL_AUTH_TEST=1)..."
    & "$repoRoot\scripts\test_auth_e2e.ps1"
    Write-Output "✓ Auth E2E validation passed"
}

Write-Output ""
Write-Output "=== All tests passed! ==="
Write-Output ""
Write-Output "To test interactively:"
Write-Output "  jcode        # Start TUI mode"
Write-Output "  jcode server # Start server mode"
Write-Output "  jcode client # Connect to server"
