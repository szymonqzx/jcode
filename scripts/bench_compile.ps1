#!/usr/bin/env pwsh
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
Set-Location $repoRoot

function Show-Usage {
    Write-Output @"
Usage:
  scripts/bench_compile.ps1 <target> [options] [-- <extra cargo args>]

Targets:
  check            Run cargo check --quiet
  build            Run cargo build --quiet
  release-jcode    Run scripts/dev_cargo.ps1 build --release -p jcode --bin jcode --quiet
  selfdev-jcode    Run scripts/dev_cargo.ps1 build --profile selfdev -p jcode --bin jcode --quiet

Options:
  --cold                 Run cargo clean before timing the first run
  --touch <path>         Touch a source file before each timed run to simulate an edit
  --edit <path>          Toggle a harmless text edit before each run (restored afterward)
  --runs <n>             Number of timed runs to execute (default: 1)
  --json                 Print per-run + summary data as JSON
  -h, --help             Show this help

Examples:
  scripts/bench_compile.ps1 check
  scripts/bench_compile.ps1 check --runs 3 --touch src/server.rs
  scripts/bench_compile.ps1 check --runs 3 --edit src/server.rs
  scripts/bench_compile.ps1 build -- --package jcode --bin test_api
  scripts/bench_compile.ps1 release-jcode --json
  scripts/bench_compile.ps1 selfdev-jcode --json
"@
}

if ($args -and ($args[0] -eq "-h" -or $args[0] -eq "--help")) {
    Show-Usage
    exit 0
}

$target = if ($args) { $args[0] } else { "" }
$remainingArgs = if ($args.Count -gt 1) { $args[1..($args.Count - 1)] } else { @() }

if ([string]::IsNullOrEmpty($target)) {
    Show-Usage
    exit 1
}

$cold = $false
$touchPath = ""
$editPath = ""
$runs = 1
$jsonOutput = $false
$extraArgs = @()

$i = 0
while ($i -lt $remainingArgs.Count) {
    switch ($remainingArgs[$i]) {
        "--cold" {
            $cold = $true
            $i++
        }
        "--touch" {
            if ($i + 1 -ge $remainingArgs.Count) {
                Write-Error "error: --touch requires a path"
                exit 1
            }
            $touchPath = $remainingArgs[$i + 1]
            $i += 2
        }
        "--edit" {
            if ($i + 1 -ge $remainingArgs.Count) {
                Write-Error "error: --edit requires a path"
                exit 1
            }
            $editPath = $remainingArgs[$i + 1]
            $i += 2
        }
        "--runs" {
            if ($i + 1 -ge $remainingArgs.Count) {
                Write-Error "error: --runs requires a positive integer"
                exit 1
            }
            $runs = $remainingArgs[$i + 1]
            $i += 2
        }
        "--json" {
            $jsonOutput = $true
            $i++
        }
        "--" {
            $i++
            $extraArgs = $remainingArgs[$i..($remainingArgs.Count - 1)]
            break
        }
        { $_ -in "-h", "--help" } {
            Show-Usage
            exit 0
        }
        default {
            Write-Error "error: unknown argument: $($remainingArgs[$i])"
            exit 1
        }
    }
}

if ($runs -notmatch "^[1-9][0-9]*$") {
    Write-Error "error: --runs must be a positive integer (got $runs)"
    exit 1
}

if ($touchPath -and $editPath) {
    Write-Error "error: --touch and --edit are mutually exclusive"
    exit 1
}

switch ($target) {
    "check" {
        $cmd = @("cargo", "check", "--quiet")
    }
    "build" {
        $cmd = @("cargo", "build", "--quiet")
    }
    "release-jcode" {
        $cmd = @("$repoRoot/scripts/dev_cargo.ps1", "build", "--release", "-p", "jcode", "--bin", "jcode", "--quiet")
    }
    "selfdev-jcode" {
        $cmd = @("$repoRoot\scripts\dev_cargo.ps1", "build", "--profile", "selfdev", "-p", "jcode", "--bin", "jcode", "--quiet")
    }
    default {
        Write-Error "error: unsupported target: $target"
        Show-Usage
        exit 1
    }
}

if ($extraArgs) {
    $cmd += $extraArgs
}

if ($touchPath -and -not (Test-Path $touchPath)) {
    Write-Error "error: touch path does not exist: $touchPath"
    exit 1
}

if ($editPath -and -not (Test-Path $editPath -PathType Leaf)) {
    Write-Error "error: edit path must be an existing file: $editPath"
    exit 1
}

$editBackup = ""
$cleanupScript = {
    if ($editBackup -and $editPath -and (Test-Path $editBackup)) {
        Copy-Item $editBackup $editPath -Force
        Remove-Item $editBackup -Force
    }
}

if ($editPath) {
    $editBackup = New-TemporaryFile
    Copy-Item $editPath $editBackup -Force
}

try {
    if ($cold) {
        Write-Error "bench_compile: running cargo clean"
        cargo clean
    }

    Write-Error "bench_compile: target=$target cold=$cold runs=$runs"
    Write-Error "bench_compile: touch=$(if ($touchPath) { $touchPath } else { '<none>' })"
    Write-Error "bench_compile: edit=$(if ($editPath) { $editPath } else { '<none>' })"
    Write-Error "bench_compile: command=$($cmd -join ' ')"

    $runTimes = @()

    function Invoke-RunOnce {
        param([int]$RunIndex)

        if ($touchPath) {
            Write-Error "bench_compile: touching $touchPath (run $RunIndex/$runs)"
            (Get-Item $touchPath).LastWriteTime = Get-Date
        } elseif ($editPath) {
            Write-Error "bench_compile: editing $editPath (run $RunIndex/$runs)"
            $backupContent = Get-Content $editBackup -Raw -AsByteStream
            if ($RunIndex % 2 -eq 1) {
                $newBytes = $backupContent + 10
                [System.IO.File]::WriteAllBytes($editPath, $newBytes)  # Add newline
            } else {
                [System.IO.File]::WriteAllBytes($editPath, $backupContent)
            }
        }

        $startNs = python3 -c "import time; print(int(time.perf_counter_ns()))"

        & $cmd[0] @($cmd[1..($cmd.Count - 1)])

        $endNs = python3 -c "import time; print(int(time.perf_counter_ns()))"
        $elapsedNs = $endNs - $startNs
        $elapsedSecs = python3 -c "import sys; print(f'{int(sys.argv[1]) / 1_000_000_000:.3f}')" $elapsedNs

        $runTimes += [double]$elapsedSecs

        if (-not $jsonOutput) {
            Write-Error "bench_compile: run $RunIndex/$runs real ${elapsedSecs}s"
        }
    }

    for ($i = 1; $i -le $runs; $i++) {
        Invoke-RunOnce $i
    }

    # Generate summary JSON using Python
    $summaryJson = python3 -c @"
import json
import statistics
import sys

target = '$target'
cold = $cold.ToString().ToLower()
touch = '$touchPath' if '$touchPath' else None
edit = '$editPath' if '$editPath' else None
runs = $runs
command = '$($cmd -join ' ')'
times = [$($runTimes -join ',')]

summary = {
    "target": target,
    "cold": cold == 'true',
    "touch": touch,
    "edit": edit,
    "runs": runs,
    "command": command,
    "times_seconds": times,
    "min_seconds": min(times),
    "max_seconds": max(times),
    "avg_seconds": sum(times) / len(times),
    "median_seconds": statistics.median(times),
}
print(json.dumps(summary))
"@

    if ($jsonOutput) {
        Write-Output $summaryJson
    } else {
        python3 -c @"
import json
import sys
summary = json.loads('$summaryJson')
print(
    "bench_compile: summary "
    "min={:.3f}s ".format(summary['min_seconds'])
    "median={:.3f}s ".format(summary['median_seconds'])
    "avg={:.3f}s ".format(summary['avg_seconds'])
    "max={:.3f}s".format(summary['max_seconds'])
)
"@
    }
} finally {
    & $cleanupScript
}
