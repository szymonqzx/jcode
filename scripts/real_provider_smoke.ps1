#!/usr/bin/env pwsh
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$provider = if ($env:JCODE_PROVIDER) { $env:JCODE_PROVIDER } else { "auto" }
$prompt = if ($args) { $args[0] } else { "Use the bash tool to run 'pwd', then use the ls tool to list the current directory, then respond with DONE." }
$expect = if ($env:JCODE_TRACE_EXPECT) { $env:JCODE_TRACE_EXPECT } else { "DONE" }
$cargoExec = "$repoRoot\scripts\cargo_exec.ps1"

Write-Output "=== Real Provider Smoke ==="
Write-Output "Provider: ${provider}"

if ($env:JCODE_REAL_PROVIDER_TEST_API -ne "0") {
    if ($provider -eq "claude" -and $env:JCODE_USE_DIRECT_API -ne "1") {
        Write-Output ""
        Write-Output "Test 1: Claude CLI smoke (test_api)"
        if ($env:JCODE_REMOTE_CARGO -eq "1") {
            Push-Location $repoRoot
            try {
                & $cargoExec build --bin test_api
                & ./target/debug/test_api.exe
            } finally {
                Pop-Location
            }
        } else {
            Push-Location $repoRoot
            try {
                & cargo run --bin test_api
            } finally {
                Pop-Location
            }
        }
    } else {
        Write-Output ""
        Write-Output "Test 1: Skipping test_api (provider=${provider}, JCODE_USE_DIRECT_API=$($env:JCODE_USE_DIRECT_API))"
    }
}

Write-Output ""
Write-Output "Test 2: Tool harness (network tools enabled)"
if ($env:JCODE_REMOTE_CARGO -eq "1") {
    Push-Location $repoRoot
    try {
        & $cargoExec build --bin jcode-harness
        & ./target/debug/jcode-harness.exe -- --include-network
    } finally {
        Pop-Location
    }
} else {
    Push-Location $repoRoot
    try {
        & cargo run --bin jcode-harness -- --include-network
    } finally {
        Pop-Location
    }
}

Write-Output ""
Write-Output "Test 3: End-to-end trace"
$binaryPath = "$repoRoot/target/release/jcode.exe"
if (-not (Test-Path $binaryPath)) {
    Push-Location $repoRoot
    try {
        & $cargoExec build --release
    } finally {
        Pop-Location
    }
}

$workdir = New-TemporaryDirectory
try {
    $env:JCODE_HOME = $workdir
    $env:PATH = "$repoRoot/target/release;$env:PATH"
    $output = jcode run --no-update --trace --provider $provider $prompt 2>&1
    $status = $LASTEXITCODE

    Write-Output $output

    if ($status -ne 0) {
        Write-Error "Trace failed with exit code $status"
        exit $status
    }

    if ($expect -and $output -notmatch [regex]::Escape($expect)) {
        Write-Error "Trace output did not include expected marker: ${expect}"
        exit 1
    }
} finally {
    Remove-Item $workdir -Recurse -Force
}

Write-Output ""
Write-Output "=== Real provider smoke OK ==="
