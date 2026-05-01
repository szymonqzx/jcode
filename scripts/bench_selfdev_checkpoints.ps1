#!/usr/bin/env pwsh
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
Set-Location $repoRoot

function Show-Usage {
    Write-Output @"
Usage:
  scripts/bench_selfdev_checkpoints.ps1 [options]

Runs the standard compile checkpoints for the self-dev loop using scripts/bench_compile.ps1.

Options:
  --touch <path>   Source file to touch for warm edit-loop runs (default: src/server.rs)
  --runs <n>       Number of warm runs per checkpoint (default: 3)
  --skip-cold      Skip cold checkpoints and only run warm edit-loop measurements
  --json           Print a single JSON object with all checkpoint summaries
  -h, --help       Show this help

Checkpoints:
  cold_check           cargo check after cargo clean
  warm_check_edit      touched-file cargo check loop
  cold_selfdev_build   selfdev jcode build after cargo clean
  warm_selfdev_edit    touched-file selfdev jcode build loop
"@
}

$runs = 3
$touchPath = "src/server.rs"
$jsonOutput = $false
$skipCold = $false

$i = 0
while ($i -lt $args.Count) {
    switch ($args[$i]) {
        "--touch" {
            if ($i + 1 -ge $args.Count) {
                Write-Error "error: --touch requires a path"
                exit 1
            }
            $touchPath = $args[$i + 1]
            $i += 2
        }
        "--runs" {
            if ($i + 1 -ge $args.Count) {
                Write-Error "error: --runs requires a positive integer"
                exit 1
            }
            $runs = $args[$i + 1]
            $i += 2
        }
        "--json" {
            $jsonOutput = $true
            $i++
        }
        "--skip-cold" {
            $skipCold = $true
            $i++
        }
        { $_ -in "-h", "--help" } {
            Show-Usage
            exit 0
        }
        default {
            Write-Error "error: unknown argument: $($args[$i])"
            exit 1
        }
    }
}

if ($runs -notmatch "^[1-9][0-9]*$") {
    Write-Error "error: --runs must be a positive integer (got $runs)"
    exit 1
}

if (-not (Test-Path $touchPath)) {
    Write-Error "error: touch path does not exist: $touchPath"
    exit 1
}

function Invoke-Bench {
    param([string]$Name, [string[]]$BenchArgs)

    $stdoutFile = New-TemporaryFile
    $stderrFile = New-TemporaryFile

    try {
        $proc = Start-Process -FilePath "$repoRoot\scripts\bench_compile.ps1" -ArgumentList $BenchArgs + "--json" -RedirectStandardOutput $stdoutFile -RedirectStandardError $stderrFile -PassThru -NoNewWindow -Wait

        if ($proc.ExitCode -eq 0) {
            $payload = Get-Content $stdoutFile -Raw | ConvertFrom-Json
            $payload | Add-Member -NotePropertyName "checkpoint" -NotePropertyValue $Name
            $payload | Add-Member -NotePropertyName "ok" -NotePropertyValue $true
            $payload | ConvertTo-Json -Compress
        } else {
            $stderr = Get-Content $stderrFile -Raw
            @{
                checkpoint = $Name
                ok = $false
                exit_code = $proc.ExitCode
                error = $stderr.Trim()
            } | ConvertTo-Json -Compress
        }
    } finally {
        Remove-Item $stdoutFile -Force -ErrorAction SilentlyContinue
        Remove-Item $stderrFile -Force -ErrorAction SilentlyContinue
    }
}

$coldCheckJson = if ($skipCold) {
    @{ checkpoint = "cold_check"; ok = $null; skipped = $true } | ConvertTo-Json -Compress
} else {
    Invoke-Bench "cold_check" @("check", "--cold")
}

$coldSelfdevJson = if ($skipCold) {
    @{ checkpoint = "cold_selfdev_build"; ok = $null; skipped = $true } | ConvertTo-Json -Compress
} else {
    Invoke-Bench "cold_selfdev_build" @("selfdev-jcode", "--cold")
}

$warmCheckJson = Invoke-Bench "warm_check_edit" @("check", "--runs", "$runs", "--touch", $touchPath)
$warmSelfdevJson = Invoke-Bench "warm_selfdev_edit" @("selfdev-jcode", "--runs", "$runs", "--touch", $touchPath)

# Generate summary
$summary = python3 -c @"
import json
import sys

touch_path = '$touchPath'
runs = $runs
skip_cold = $skipCold.ToString().ToLower()
cold_check = json.loads('$coldCheckJson')
warm_check = json.loads('$warmCheckJson')
cold_selfdev = json.loads('$coldSelfdevJson')
warm_selfdev = json.loads('$warmSelfdevJson')

skip = cold_check.get('skipped', False) and cold_selfdev.get('skipped', False)

summary = {
    "touch_path": touch_path,
    "warm_runs": runs,
    "skip_cold": skip,
    "checkpoints": {
        "cold_check": cold_check,
        "warm_check_edit": warm_check,
        "cold_selfdev_build": cold_selfdev,
        "warm_selfdev_edit": warm_selfdev,
    },
    "failed_checkpoints": [
        name for name, payload in {
            "cold_check": cold_check,
            "warm_check_edit": warm_check,
            "cold_selfdev_build": cold_selfdev,
            "warm_selfdev_edit": warm_selfdev,
        }.items()
        if payload.get("ok") is False
    ],
}
print(json.dumps(summary))
"@

if ($jsonOutput) {
    Write-Output $summary
} else {
    python3 -c @"
import json
import sys
summary = json.loads('$summary')
print("selfdev compile checkpoints")
print(f"  touch_path: {summary['touch_path']}")
print(f"  warm_runs:  {summary['warm_runs']}")
print(f"  skip_cold:  {summary['skip_cold']}")
for name, payload in summary["checkpoints"].items():
    if payload.get("skipped"):
        print(f"  {name}: SKIPPED")
    elif payload.get("ok", False):
        print(
            f"  {name}: min={payload['min_seconds']:.3f}s "
            f"median={payload['median_seconds']:.3f}s avg={payload['avg_seconds']:.3f}s "
            f"max={payload['max_seconds']:.3f}s"
        )
    else:
        print(
            f"  {name}: FAILED exit={payload.get('exit_code')} error={payload.get('error', '')[:160]}"
        )
if summary["failed_checkpoints"]:
    print(f"  failed_checkpoints: {', '.join(summary['failed_checkpoints'])}")
"@
}
