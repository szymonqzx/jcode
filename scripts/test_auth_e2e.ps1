#!/usr/bin/env pwsh
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$provider = if ($env:JCODE_PROVIDER) { $env:JCODE_PROVIDER } else { "auto" }
$prompt = if ($env:JCODE_AUTH_TEST_PROMPT) { $env:JCODE_AUTH_TEST_PROMPT } else { "Reply with exactly AUTH_TEST_OK and nothing else. Do not call tools." }

Write-Output "=== Auth E2E Test ==="
Write-Output "Provider: ${provider}"

$cargoArgs = @("auth-test", "--prompt", $prompt)

if ($provider -ne "auto") {
    $cargoArgs = @("--provider", $provider) + $cargoArgs
} else {
    $cargoArgs += "--all-configured"
}

if ($env:JCODE_AUTH_TEST_LOGIN -eq "1") {
    $cargoArgs += "--login"
}

if ($env:JCODE_AUTH_TEST_NO_SMOKE -eq "1") {
    $cargoArgs += "--no-smoke"
}

if ($env:JCODE_AUTH_TEST_JSON -eq "1") {
    $cargoArgs += "--json"
}

Push-Location $repoRoot
try {
    & cargo run --bin jcode -- @cargoArgs
} finally {
    Pop-Location
}

Write-Output ""
Write-Output "=== Auth E2E OK ==="
