#!/usr/bin/env pwsh
$ErrorActionPreference = "Stop"

# Keep files created by this helper private by default.
$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$userName = $env:USERNAME
$runtimeDir = if ($env:XDG_RUNTIME_DIR) { $env:XDG_RUNTIME_DIR } else { $env:TEMP }
$defaultHome = "$env:USERPROFILE/.jcode-refactor"
$defaultSocket = "$runtimeDir/jcode-refactor-$userName.sock"

$refHome = if ($env:JCODE_REF_HOME) { $env:JCODE_REF_HOME } else { $defaultHome }
$refSocket = if ($env:JCODE_REF_SOCKET) { $env:JCODE_REF_SOCKET } else { $defaultSocket }
$refProfile = if ($env:JCODE_REF_PROFILE) { $env:JCODE_REF_PROFILE } else { "debug" }

switch ($refProfile) {
    "debug" { $defaultBin = "$repoRoot/target/debug/jcode.exe" }
    "release" { $defaultBin = "$repoRoot/target/release/jcode.exe" }
    default {
        Write-Error "error: unsupported JCODE_REF_PROFILE: $refProfile (expected debug or release)"
        exit 1
    }
}

$refBin = if ($env:JCODE_REF_BIN) { $env:JCODE_REF_BIN } else { $defaultBin }

function Show-Usage {
    Write-Output @"
Usage:
  scripts/refactor_shadow.ps1 env
  scripts/refactor_shadow.ps1 build [--release]
  scripts/refactor_shadow.ps1 serve [-- <jcode serve args>]
  scripts/refactor_shadow.ps1 run [-- <jcode args>]
  scripts/refactor_shadow.ps1 connect [-- <jcode connect args>]
  scripts/refactor_shadow.ps1 check

What it does:
  - Runs jcode in an isolated refactor environment
  - Uses separate JCODE_HOME and JCODE_SOCKET
  - Refuses to run against ~/.jcode to protect live sessions

Environment overrides:
  JCODE_REF_HOME      Isolated home dir (default: ~/.jcode-refactor)
  JCODE_REF_SOCKET    Isolated socket path
  JCODE_REF_PROFILE   debug|release (default: debug)
  JCODE_REF_BIN       Explicit jcode binary path
"@
}

function Write-Error-Exit {
    param([string]$Message)
    Write-Error "error: $Message"
    exit 1
}

function Assert-SafePaths {
    if ([string]::IsNullOrEmpty($refHome)) {
        Write-Error-Exit "JCODE_REF_HOME resolved to empty path"
    }
    if ([string]::IsNullOrEmpty($refSocket)) {
        Write-Error-Exit "JCODE_REF_SOCKET resolved to empty path"
    }
    if (-not [System.IO.Path]::IsPathRooted($refHome)) {
        Write-Error-Exit "JCODE_REF_HOME must be an absolute path: $refHome"
    }
    if (-not [System.IO.Path]::IsPathRooted($refSocket)) {
        Write-Error-Exit "JCODE_REF_SOCKET must be an absolute path: $refSocket"
    }

    $prodHome = "$env:USERPROFILE/.jcode"
    if ($refHome -eq $prodHome) {
        Write-Error-Exit "refusing to run with production home ($prodHome); set JCODE_REF_HOME to an isolated path"
    }
}

function Ensure-RefHome {
    if (-not (Test-Path $refHome)) {
        $null = New-Item -Path $refHome -ItemType Directory -Force
    }
    # Best-effort hardening if dir already exists
    try {
        $acl = Get-Acl $refHome
        # Restrict to current user only
        $acl.SetAccessRuleProtection($true, $false)
        $accessRule = New-Object System.Security.AccessControl.FileSystemAccessRule(
            $env:USERNAME,
            "FullControl",
            "ContainerInherit,ObjectInherit",
            "None",
            "Allow"
        )
        $acl.SetAccessRule($accessRule)
        Set-Acl $refHome $acl
    } catch {
        # Ignore permission errors
    }
}

function Ensure-SocketParent {
    $socketParent = Split-Path -Parent $refSocket
    if (-not (Test-Path $socketParent)) {
        $null = New-Item -Path $socketParent -ItemType Directory -Force
        try {
            $acl = Get-Acl $socketParent
            $acl.SetAccessRuleProtection($true, $false)
            $accessRule = New-Object System.Security.AccessControl.FileSystemAccessRule(
                $env:USERNAME,
                "FullControl",
                "ContainerInherit,ObjectInherit",
                "None",
                "Allow"
            )
            $acl.SetAccessRule($accessRule)
            Set-Acl $socketParent $acl
        } catch {
            # Ignore permission errors
        }
    }
}

function Ensure-Binary {
    if (-not (Test-Path $refBin)) {
        Write-Error-Exit "jcode binary not found: $refBin (run 'scripts/refactor_shadow.ps1 build')"
    }
}

function Remove-StaleSocket {
    $debugSocket = $refSocket -replace '\.sock$', '-debug.sock'
    foreach ($path in @($refSocket, $debugSocket)) {
        if (Test-Path $path) {
            if (Test-Path $path -PathType Leaf) {
                Remove-Item $path -Force
            } else {
                Write-Error-Exit "refusing to remove non-socket path: $path"
            }
        }
    }
}

function Invoke-Isolated {
    param([string[]]$ScriptArgs)
    $env:JCODE_HOME = $refHome
    $env:JCODE_SOCKET = $refSocket
    & @ScriptArgs
}

function Invoke-Env {
    Write-Output @"
JCODE_REF_HOME=$refHome
JCODE_REF_SOCKET=$refSocket
JCODE_REF_PROFILE=$refProfile
JCODE_REF_BIN=$refBin

# One-off command example:
JCODE_HOME=$refHome JCODE_SOCKET=$refSocket $refBin --version
"@
}

function Invoke-Build {
    $profileFlag = ""
    if ($args -and $args[0] -eq "--release") {
        $profileFlag = "--release"
    } elseif ($args -and $args[0] -ne "") {
        Write-Error "error: unknown build argument: $($args[0])"
        exit 1
    }

    Push-Location $repoRoot
    try {
        & "$repoRoot\scripts\dev_cargo.ps1" build $profileFlag
    } finally {
        Pop-Location
    }
}

function Invoke-Check {
    Assert-SafePaths
    Ensure-RefHome
    Ensure-SocketParent

    Write-Output "Refactor home:    $refHome"
    Write-Output "Refactor socket:  $refSocket"
    Write-Output "Refactor binary:  $refBin"

    if (Test-Path $refSocket) {
        Write-Output "Socket status:    present (server likely running)"
    } else {
        Write-Output "Socket status:    not present"
    }
}

function Invoke-Serve {
    Assert-SafePaths
    Ensure-RefHome
    Ensure-SocketParent
    Ensure-Binary
    Remove-StaleSocket

    Invoke-Isolated $refBin serve @args
}

function Invoke-Run {
    Assert-SafePaths
    Ensure-RefHome
    Ensure-SocketParent
    Ensure-Binary

    Invoke-Isolated $refBin @args
}

function Invoke-Connect {
    Assert-SafePaths
    Ensure-RefHome
    Ensure-SocketParent
    Ensure-Binary

    Invoke-Isolated $refBin connect @args
}

$cmd = if ($args) { $args[0] } else { "help" }
$remainingArgs = if ($args.Count -gt 1) { $args[1..($args.Count - 1)] } else { @() }

# Handle "--" separator
if ($remainingArgs -and $remainingArgs[0] -eq "--") {
    $remainingArgs = $remainingArgs[1..($remainingArgs.Count - 1)]
}

switch ($cmd) {
    "env" {
        Invoke-Env
    }
    "build" {
        Invoke-Build @remainingArgs
    }
    "serve" {
        Invoke-Serve @remainingArgs
    }
    "run" {
        Invoke-Run @remainingArgs
    }
    "connect" {
        Invoke-Connect @remainingArgs
    }
    "check" {
        Invoke-Check
    }
    { $_ -in "help", "-h", "--help" } {
        Show-Usage
    }
    default {
        Write-Error-Exit "unknown command: $cmd (use --help)"
    }
}
