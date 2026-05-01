#!/usr/bin/env pwsh
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)

if ($env:JCODE_REMOTE_CARGO -eq "1") {
    & "$repoRoot\scripts\remote_build.ps1" @args
    exit $LASTEXITCODE
}

& cargo @args
exit $LASTEXITCODE
