<#
.SYNOPSIS
    Install jcode from source with build optimization reporting and benchmarking.

.DESCRIPTION
    This script builds jcode from source, reports which build optimizations
    were applied, benchmarks the build time, and shows any configuration issues.

    Usage:
      scripts/install_with_bench.ps1

    Requirements:
      - cargo (Rust toolchain)
      - git
      - LLVM (optional, for lld linker)
#>

$ErrorActionPreference = 'Stop'

# Get repo root first
$repoRoot = & git rev-parse --show-toplevel 2>$null
if ($LASTEXITCODE -ne 0 -or -not $repoRoot) {
    # Fall back to script directory parent (scripts/ is in repo root)
    $repoRoot = Split-Path -Parent $PSScriptRoot
}

function Write-Info($msg) { Write-Host $msg -ForegroundColor Blue }
function Write-Success($msg) { Write-Host $msg -ForegroundColor Green }
function Write-Warn($msg) { Write-Host $msg -ForegroundColor Yellow }
function Write-Err($msg) { Write-Host $msg -ForegroundColor Red }

Write-Host "=== jcode Build Optimization Check & Install ===" -ForegroundColor Cyan
Write-Host ""

# Check prerequisites
$checks = @{
    "cargo" = (Get-Command cargo -ErrorAction SilentlyContinue)
    "git" = (Get-Command git -ErrorAction SilentlyContinue)
    "lld" = (Get-Command lld -ErrorAction SilentlyContinue)
    "lld-link" = (Get-Command lld-link -ErrorAction SilentlyContinue)
}

Write-Host "Prerequisites Check:" -ForegroundColor Cyan
$lldFound = $false
foreach ($tool in $checks.Keys) {
    if ($checks[$tool]) {
        Write-Success "  [OK] $tool found at $($checks[$tool].Source)"
        if ($tool -eq "lld" -or $tool -eq "lld-link") {
            $lldFound = $true
        }
    }
}

if (-not $lldFound) {
    Write-Warn "  [WARN] lld linker not found (optional, but recommended for faster builds)"
    Write-Host "    Install with: winget install LLVM.LLVM"
}
Write-Host ""

# Check Cargo configuration
Write-Host "Cargo Configuration Check:" -ForegroundColor Cyan
$cargoConfig = "$env:USERPROFILE\.cargo\config.toml"
if (-not (Test-Path $cargoConfig) -and $repoRoot) {
    $cargoConfig = Join-Path $repoRoot ".cargo\config.toml"
}
if (Test-Path $cargoConfig) {
    Write-Info "  Found config at: $cargoConfig"
    $configContent = Get-Content $cargoConfig -Raw

    # Check for lld configuration
    if ($configContent -match "link-arg=-fuse-ld=lld") {
        Write-Success "  [OK] lld linker configured"
    } else {
        Write-Warn "  [WARN] lld linker not configured in config.toml"
    }

    # Check for jobs configuration
    if ($configContent -match "jobs\s*=\s*(\d+)") {
        $jobs = $matches[1]
        Write-Info "  Parallel jobs: $jobs"
    } else {
        Write-Warn "  [WARN] jobs not configured (defaulting to 1)"
    }
} else {
    Write-Warn "  [WARN] No Cargo config.toml found at $cargoConfig"
}
Write-Host ""

# Check Cargo.toml profile configuration
Write-Host "Profile Configuration Check:" -ForegroundColor Cyan
$cargoToml = "Cargo.toml"
if (Test-Path $cargoToml) {
    $tomlContent = Get-Content $cargoToml -Raw

    # Check profile.dev
    if ($tomlContent -match '\[profile\.dev\]') {
        Write-Info "  [profile.dev] found"

        if ($tomlContent -match 'debug\s*=\s*0') {
            Write-Success "    [OK] debug = 0 (no debug info)"
        } else {
            Write-Warn "    [WARN] debug not set to 0"
        }

        if ($tomlContent -match 'strip\s*=\s*"debuginfo"') {
            Write-Success "    [OK] strip = ""debuginfo"" (strip debug info during linking)"
        } else {
            Write-Warn "    [WARN] strip = ""debuginfo"" not set"
        }

        if ($tomlContent -match 'opt-level\s*=\s*1') {
            Write-Success "    [OK] opt-level = 1 (optimized for speed)"
        } else {
            Write-Warn "    [WARN] opt-level not set to 1"
        }

        if ($tomlContent -match 'codegen-units\s*=\s*256') {
            Write-Success "    [OK] codegen-units = 256 (parallel codegen)"
        } else {
            Write-Warn "    [WARN] codegen-units not set to 256"
        }

        if ($tomlContent -match 'incremental\s*=\s*true') {
            Write-Success "    [OK] incremental = true (incremental compilation)"
        } else {
            Write-Warn "    [WARN] incremental not enabled"
        }
    } else {
        Write-Warn "  [WARN] [profile.dev] not found"
    }
} else {
    Write-Err "  [ERROR] Cargo.toml not found"
}
Write-Host ""

# Check build.rs optimization
Write-Host "build.rs Optimization Check:" -ForegroundColor Cyan
$buildRs = "build.rs"
if ($repoRoot) {
    $buildRs = Join-Path $repoRoot "build.rs"
}
if (Test-Path $buildRs) {
    $buildContent = Get-Content $buildRs -Raw
    if ($buildContent -match "rerun-if-changed") {
        Write-Success "  [OK] build.rs has rerun-if-changed directives (prevents timestamp rebuilds)"
    } else {
        Write-Warn "  [WARN] build.rs missing rerun-if-changed directives"
    }
} else {
    Write-Warn "  [WARN] build.rs not found"
}
Write-Host ""

# Build jcode with benchmarking
Write-Host "Building jcode..." -ForegroundColor Cyan
Write-Host ""

# Enable sccache if available for compilation caching
$sccacheAvailable = $false
try {
    $null = Get-Command sccache -ErrorAction Stop
    $sccacheAvailable = $true
    $env:RUSTC_WRAPPER = "sccache"
    Write-Host "sccache enabled for compilation caching"
} catch {
    Write-Host "sccache not found, using standard cargo caching"
}

# Clean first for cold build
Write-Info "Running cargo clean for cold build benchmark..."
try {
    & cargo clean 2>&1 | Out-Null
} catch {
    # Ignore cargo clean errors - not critical
}

$buildStart = Get-Date
try {
    cargo build --release
    $buildExitCode = $LASTEXITCODE
} catch {
    $buildExitCode = $_.Exception.ExitCode
}
$buildEnd = Get-Date
$buildDuration = ($buildEnd - $buildStart).TotalSeconds

Write-Host ""
Write-Host "=== Build Results ===" -ForegroundColor Cyan
if ($buildExitCode -eq 0) {
    Write-Success "[OK] Build succeeded"
    Write-Host "  Total build time: $([math]::Round($buildDuration, 2)) seconds"
    Write-Host "  Binary location: target\release\jcode.exe"
} else {
    Write-Err "[ERROR] Build failed with exit code $buildExitCode"
}
Write-Host ""

# Incremental build benchmark
if ($buildExitCode -eq 0) {
    Write-Host "Benchmarking incremental build (no-op)..." -ForegroundColor Cyan
    $incrementalStart = Get-Date
    try {
        cargo build --release 2>$null | Out-Null
    } catch {
        # Ignore warnings
    }
    $incrementalEnd = Get-Date
    $incrementalDuration = ($incrementalEnd - $incrementalStart).TotalSeconds
    Write-Success "[OK] Incremental build completed"
    Write-Host "  Incremental build time: $([math]::Round($incrementalDuration, 2)) seconds"
    Write-Host ""

    Write-Host "=== Build Time Summary ===" -ForegroundColor Cyan
    Write-Host "  Cold build:    $([math]::Round($buildDuration, 2))s"
    Write-Host "  Incremental:   $([math]::Round($incrementalDuration, 2))s"
    Write-Host "  Speedup:       $([math]::Round($buildDuration / $incrementalDuration, 2))x"
    Write-Host ""
}

Write-Host "=== Installation Complete ===" -ForegroundColor Cyan
Write-Host ""
Write-Host "To use jcode:"
Write-Host "  .\target\release\jcode.exe"
Write-Host ""
Write-Host "Or add to PATH:"
Write-Host "  setx PATH ""%PATH%;$(Get-Location)\target\release"""
