#!/usr/bin/env pwsh
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$prompt = if ($args) { $args[0] } else { "Use the bash tool to run 'pwd', then use the ls tool to list the current directory, then respond with DONE." }
$provider = if ($env:JCODE_PROVIDER) { $env:JCODE_PROVIDER } else { "auto" }
$cargoExec = "$repoRoot\scripts\cargo_exec.ps1"

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
    jcode run --no-update --trace --provider $provider $prompt
} finally {
    Remove-Item $workdir -Recurse -Force
}
