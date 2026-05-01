#!/usr/bin/env pwsh
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
Set-Location $repoRoot

$bin = if ($env:JCODE_AUTH_MATRIX_BIN) { $env:JCODE_AUTH_MATRIX_BIN } else { "" }
$outDir = if ($env:JCODE_AUTH_MATRIX_OUT) { $env:JCODE_AUTH_MATRIX_OUT } else { "$repoRoot/target/auth-test-reports" }
$prompt = if ($env:JCODE_AUTH_MATRIX_PROMPT) { $env:JCODE_AUTH_MATRIX_PROMPT } else { "Reply with exactly AUTH_TEST_OK and nothing else. Do not call tools." }
$providers = if ($env:JCODE_AUTH_MATRIX_PROVIDERS) { $env:JCODE_AUTH_MATRIX_PROVIDERS } else { "claude copilot openrouter deepseek zai alibaba-coding-plan openai-compatible" }
$mode = if ($env:JCODE_AUTH_MATRIX_MODE) { $env:JCODE_AUTH_MATRIX_MODE } else { "configured" }
$keepGoing = if ($env:JCODE_AUTH_MATRIX_KEEP_GOING) { $env:JCODE_AUTH_MATRIX_KEEP_GOING } else { "1" }
$perCommandTimeout = if ($env:JCODE_AUTH_MATRIX_TIMEOUT) { $env:JCODE_AUTH_MATRIX_TIMEOUT } else { "90" }

function Show-Usage {
    Write-Output @"
Usage: scripts/auth_regression_matrix.ps1 [options]

Runs jcode auth-test across the auth/provider matrix and writes one JSON report per provider.
By default it only tests providers that are configured enough for auth-test to run.

Options:
  --all                 Try every provider in the matrix, even if not configured
  --configured          Test only configured providers (default)
  --provider NAME       Test one provider. Can be repeated.
  --out DIR             Report directory (default: target/auth-test-reports)
  --bin PATH            jcode binary to run (default: cargo run --bin jcode --)
  --login               Run login before validation for each provider
  --no-smoke            Skip runtime model smoke
  --no-tool-smoke       Skip tool-enabled runtime smoke
  --fail-fast           Stop after the first failed provider
  --prompt TEXT         Custom smoke prompt
  --timeout SECONDS     Per auth-test command timeout (default: 90)
  -h, --help            Show this help

Environment equivalents:
  JCODE_AUTH_MATRIX_BIN=/path/to/jcode
  JCODE_AUTH_MATRIX_OUT=target/auth-test-reports
  JCODE_AUTH_MATRIX_PROVIDERS="claude deepseek zai"
  JCODE_AUTH_MATRIX_MODE=configured|all
  JCODE_AUTH_MATRIX_LOGIN=1
  JCODE_AUTH_MATRIX_NO_SMOKE=1
  JCODE_AUTH_MATRIX_NO_TOOL_SMOKE=1
  JCODE_AUTH_MATRIX_KEEP_GOING=0
  JCODE_AUTH_MATRIX_TIMEOUT=90

Examples:
  scripts/auth_regression_matrix.ps1 --configured --no-smoke
  scripts/auth_regression_matrix.ps1 --provider deepseek --provider zai
  JCODE_AUTH_MATRIX_BIN=target/selfdev/jcode scripts/auth_regression_matrix.ps1 --all
"@
}

$selected = @()
$extraArgs = @()

$i = 0
while ($i -lt $args.Count) {
    switch ($args[$i]) {
        "--all" {
            $mode = "all"
            $i++
        }
        "--configured" {
            $mode = "configured"
            $i++
        }
        "--provider" {
            if ($i + 1 -ge $args.Count) {
                Write-Error "error: --provider requires a value"
                exit 2
            }
            $selected += $args[$i + 1]
            $i += 2
        }
        "--out" {
            if ($i + 1 -ge $args.Count) {
                Write-Error "error: --out requires a value"
                exit 2
            }
            $outDir = $args[$i + 1]
            $i += 2
        }
        "--bin" {
            if ($i + 1 -ge $args.Count) {
                Write-Error "error: --bin requires a value"
                exit 2
            }
            $bin = $args[$i + 1]
            $i += 2
        }
        "--login" {
            $extraArgs += "--login"
            $i++
        }
        "--no-smoke" {
            $extraArgs += "--no-smoke"
            $i++
        }
        "--no-tool-smoke" {
            $extraArgs += "--no-tool-smoke"
            $i++
        }
        "--fail-fast" {
            $keepGoing = "0"
            $i++
        }
        "--prompt" {
            if ($i + 1 -ge $args.Count) {
                Write-Error "error: --prompt requires a value"
                exit 2
            }
            $prompt = $args[$i + 1]
            $i += 2
        }
        "--timeout" {
            if ($i + 1 -ge $args.Count) {
                Write-Error "error: --timeout requires a value"
                exit 2
            }
            $perCommandTimeout = $args[$i + 1]
            $i += 2
        }
        { $_ -in "-h", "--help" } {
            Show-Usage
            exit 0
        }
        default {
            Write-Error "error: unknown argument: $($args[$i])"
            Show-Usage
            exit 2
        }
    }
}

if ($env:JCODE_AUTH_MATRIX_LOGIN -eq "1") {
    $extraArgs += "--login"
}
if ($env:JCODE_AUTH_MATRIX_NO_SMOKE -eq "1") {
    $extraArgs += "--no-smoke"
}
if ($env:JCODE_AUTH_MATRIX_NO_TOOL_SMOKE -eq "1") {
    $extraArgs += "--no-tool-smoke"
}

if ($selected.Count -eq 0) {
    $selected = $providers -split ' '
}

$null = New-Item -Path $outDir -ItemType Directory -Force -ErrorAction SilentlyContinue

function Invoke-Jcode {
    param([string[]]$JcodeArgs)
    if ($bin) {
        timeout /t $perCommandTimeout $bin @JcodeArgs
    } else {
        timeout /t $perCommandTimeout cargo run --quiet --bin jcode -- @JcodeArgs
    }
}

$configuredJson = "$outDir/configured-providers.json"
if ($mode -eq "configured") {
    Write-Output "Discovering configured providers..."
    Remove-Item $configuredJson -Force -ErrorAction SilentlyContinue
    $discoveryOut = "$outDir/discovery.out"
    $discoveryErr = "$outDir/discovery.err"
    $proc = Start-Process -FilePath "cargo" -ArgumentList "run", "--quiet", "--bin", "jcode", "--", "auth-test", "--all-configured", "--no-smoke", "--no-tool-smoke", "--json", "--output", $configuredJson -RedirectStandardOutput $discoveryOut -RedirectStandardError $discoveryErr -PassThru -NoNewWindow -Wait
    if ($proc.ExitCode -ne 0) {
        if (Test-Path $configuredJson -and (Get-Item $configuredJson).Length -gt 0) {
            Write-Error "note: configured-provider discovery reported non-ready providers; continuing with per-provider classification"
        } else {
            Get-Content $discoveryErr -ErrorAction SilentlyContinue | Write-Error
            Write-Error "warning: configured-provider discovery failed; continuing with explicit matrix and skipping only obvious unconfigured failures"
        }
    }
}

$failed = @()
$passed = @()
$skipped = @()
$blocked = @()

function Test-UnconfiguredFailure {
    param([string]$LogPath)
    if (-not (Test-Path $LogPath)) { return $false }
    $content = Get-Content $LogPath -Raw
    $content -match 'not configured|missing|no credentials|not found in environment|requires.*token|requires.*api key'
}

function Test-ExternalAccountBlockedFailure {
    param([string]$LogPath)
    if (-not (Test-Path $LogPath)) { return $false }
    $content = Get-Content $LogPath -Raw
    $content -match 'feature_flag_blocked|can_signup_for_limited|Contact Support|not entitled|not eligible|subscription required|quota exceeded|rate limit'
}

Write-Output "Auth regression matrix"
Write-Output "Mode: $mode"
Write-Output "Reports: $outDir"
Write-Output "Providers: $($selected -join ' ')"
Write-Output "Timeout: ${perCommandTimeout}s per command"
Write-Output ""

foreach ($provider in $selected) {
    $report = "$outDir/${provider}.json"
    $log = "$outDir/${provider}.log"
    $testArgs = @("auth-test", "--provider", $provider, "--prompt", $prompt, "--json", "--output", $report) + $extraArgs

    Write-Output "=== auth-test: $provider ==="
    $allArgs = @("run", "--quiet", "--bin", "jcode", "--") + $testArgs
    $proc = Start-Process -FilePath "cargo" -ArgumentList $allArgs -RedirectStandardOutput $log -RedirectStandardError $log -PassThru -NoNewWindow -Wait
    $status = $proc.ExitCode

    if ($status -eq 0) {
        $passed += $provider
        Write-Output "PASS $provider"
    } else {
        if ($mode -eq "configured" -and (Test-UnconfiguredFailure $log)) {
            $skipped += $provider
            Write-Output "SKIP $provider (not configured, see $log)"
        } elseif ($mode -eq "configured" -and (Test-ExternalAccountBlockedFailure $log)) {
            $blocked += $provider
            Write-Output "BLOCKED $provider (upstream account/entitlement unavailable, see $log)"
        } else {
            $failed += $provider
            Write-Output "FAIL $provider (exit $status, see $log)"
            if ($keepGoing -ne "1") {
                break
            }
        }
    }
    Write-Output ""
}

$summary = "$outDir/summary.txt"
$summaryContent = @"
passed: $(if ($passed) { $passed -join ' ' } else { '<none>' })
skipped: $(if ($skipped) { $skipped -join ' ' } else { '<none>' })
blocked: $(if ($blocked) { $blocked -join ' ' } else { '<none>' })
failed: $(if ($failed) { $failed -join ' ' } else { '<none>' })
"@
$summaryContent | Tee-Object -FilePath $summary

if ($failed.Count -gt 0) {
    exit 1
}
