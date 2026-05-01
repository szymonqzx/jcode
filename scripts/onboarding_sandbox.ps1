#!/usr/bin/env pwsh
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$command = if ($args) { $args[0] } else { "help" }
$remainingArgs = if ($args.Count -gt 1) { $args[1..($args.Count - 1)] } else { @() }

$sandboxName = if ($env:JCODE_ONBOARDING_SANDBOX) { $env:JCODE_ONBOARDING_SANDBOX } else { "default" }
$sandboxRootDefault = "$repoRoot/.tmp/onboarding/$sandboxName"
$sandboxRoot = if ($env:JCODE_ONBOARDING_DIR) { $env:JCODE_ONBOARDING_DIR } else { $sandboxRootDefault }
$jcodeHome = "$sandboxRoot/home"
$runtimeDir = "$sandboxRoot/runtime"
$mobileSocket = "$runtimeDir/jcode-mobile-sim.sock"

function Ensure-Dirs {
    $null = New-Item -Path $jcodeHome -ItemType Directory -Force -ErrorAction SilentlyContinue
    $null = New-Item -Path $runtimeDir -ItemType Directory -Force -ErrorAction SilentlyContinue
}

function Invoke-InSandbox {
    param([scriptblock]$ScriptBlock)
    Ensure-Dirs
    Push-Location $repoRoot
    try {
        $env:JCODE_HOME = $jcodeHome
        $env:JCODE_RUNTIME_DIR = $runtimeDir
        & $ScriptBlock
    } finally {
        Pop-Location
        Remove-Item Env:\JCODE_HOME -ErrorAction SilentlyContinue
        Remove-Item Env:\JCODE_RUNTIME_DIR -ErrorAction SilentlyContinue
    }
}

function Show-Usage {
    Write-Output @"
Usage: $PSCommandPath <command> [args...]

Commands:
  env                    Print the sandbox environment exports
  status                 Show sandbox paths and current contents
  reset                  Delete the sandbox entirely
  shell                  Open a clean shell with sandbox env vars set
  jcode [args...]        Run jcode inside the sandbox
  auth-status            Run 'jcode auth status' inside the sandbox
  fresh [args...]        Reset sandbox, then launch jcode with args
  login <provider> ...   Run 'jcode --provider <provider> login ...' in sandbox
  mobile-start [scenario]
                         Start jcode-mobile-sim in background (default: onboarding)
  mobile-serve [scenario]
                         Run jcode-mobile-sim in foreground (default: onboarding)
  mobile-status          Show mobile simulator status
  mobile-state           Show full mobile simulator state
  mobile-reset           Reset the mobile simulator back to its initial scenario
  mobile-log             Show mobile simulator transition log
  help                   Show this help

Environment overrides:
  JCODE_ONBOARDING_SANDBOX   Sandbox name (default: default)
  JCODE_ONBOARDING_DIR       Explicit sandbox directory

Examples:
  $PSCommandPath fresh
  $PSCommandPath login openai
  $PSCommandPath auth-status
  $PSCommandPath mobile-start onboarding
  $PSCommandPath mobile-status
"@
}

function Show-Env {
    Ensure-Dirs
    Write-Output "JCODE_HOME=$jcodeHome"
    Write-Output "JCODE_RUNTIME_DIR=$runtimeDir"
}

function Show-Status {
    Ensure-Dirs
    Write-Output "Sandbox name: $sandboxName"
    Write-Output "Sandbox root: $sandboxRoot"
    Write-Output "JCODE_HOME:   $jcodeHome"
    Write-Output "RUNTIME_DIR:  $runtimeDir"
    Write-Output ""

    if (Test-Path $jcodeHome) {
        Write-Output "Home contents:"
        Get-ChildItem -Path $jcodeHome -Recurse -Depth 3 | Select-Object -ExpandProperty FullName | ForEach-Object {
            $_.Replace($sandboxRoot, ".")
        } | Sort-Object
    }
    Write-Output ""

    if (Test-Path $mobileSocket) {
        Write-Output "Mobile simulator socket: $mobileSocket"
    } else {
        Write-Output "Mobile simulator socket: not running"
    }
}

function Reset-Sandbox {
    Remove-Item $sandboxRoot -Recurse -Force -ErrorAction SilentlyContinue
    Write-Output "Removed onboarding sandbox: $sandboxRoot"
}

function Open-Shell {
    Ensure-Dirs
    Write-Output "Opening sandbox shell"
    Write-Output "  JCODE_HOME=$jcodeHome"
    Write-Output "  JCODE_RUNTIME_DIR=$runtimeDir"
    $env:JCODE_HOME = $jcodeHome
    $env:JCODE_RUNTIME_DIR = $runtimeDir
    pwsh -NoProfile -NoExit
}

function Invoke-Jcode {
    $binaryPath = "$repoRoot/target/debug/jcode.exe"
    if (Test-Path $binaryPath) {
        Invoke-InSandbox { & $binaryPath @remainingArgs }
    } else {
        Invoke-InSandbox { & cargo run --bin jcode -- @remainingArgs }
    }
}

function Invoke-MobileSim {
    $binaryPath = "$repoRoot/target/debug/jcode-mobile-sim.exe"
    if (Test-Path $binaryPath) {
        Invoke-InSandbox { & $binaryPath @remainingArgs }
    } else {
        Invoke-InSandbox { & cargo run -p jcode-mobile-sim -- @remainingArgs }
    }
}

function Get-ScenarioArg {
    if ($remainingArgs) {
        return $remainingArgs[0]
    }
    return "onboarding"
}

switch ($command) {
    "env" {
        Show-Env
    }
    "status" {
        Show-Status
    }
    "reset" {
        Reset-Sandbox
    }
    "shell" {
        Open-Shell
    }
    "jcode" {
        Invoke-Jcode
    }
    "auth-status" {
        $remainingArgs = @("auth", "status")
        Invoke-Jcode
    }
    "fresh" {
        Reset-Sandbox
        Invoke-Jcode
    }
    "login" {
        if (-not $remainingArgs) {
            Write-Error "login requires a provider, for example: $PSCommandPath login openai"
            exit 1
        }
        $provider = $remainingArgs[0]
        $remainingArgs = $remainingArgs[1..($remainingArgs.Count - 1)]
        $remainingArgs = @("--provider", $provider, "login") + $remainingArgs
        Invoke-Jcode
    }
    "mobile-start" {
        $scenario = Get-ScenarioArg
        $remainingArgs = @("start", "--scenario", $scenario)
        Invoke-MobileSim
    }
    "mobile-serve" {
        $scenario = Get-ScenarioArg
        $remainingArgs = @("serve", "--scenario", $scenario)
        Invoke-MobileSim
    }
    "mobile-status" {
        $remainingArgs = @("status")
        Invoke-MobileSim
    }
    "mobile-state" {
        $remainingArgs = @("state")
        Invoke-MobileSim
    }
    "mobile-reset" {
        $remainingArgs = @("reset")
        Invoke-MobileSim
    }
    "mobile-log" {
        $remainingArgs = @("log")
        Invoke-MobileSim
    }
    { $_ -in "help", "-h", "--help" } {
        Show-Usage
    }
    default {
        Write-Error "Unknown command: $command"
        Write-Error ""
        Show-Usage
        exit 1
    }
}
