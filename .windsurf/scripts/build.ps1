#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Build automation script for FastRamdisk project.

.DESCRIPTION
    This script provides automated build options including release builds,
    debug builds, with/without crando wrapper, and build verification.

.PARAMETER Release
    Build in release mode. Default: false

.PARAMETER Debug
    Build in debug mode. Default: true

.PARAMETER WithWrapper
    Build with crando wrapper enabled. Default: true

.PARAMETER Clean
    Clean build artifacts before building. Default: false

.PARAMETER Verify
    Run build verification tests after build. Default: false

.PARAMETER Target
    Target triple for cross-compilation (e.g., x86_64-pc-windows-msvc)
#>

param(
    [Parameter()]
    [switch]$Release,

    [Parameter()]
    [switch]$Debug,

    [Parameter()]
    [switch]$WithWrapper,

    [Parameter()]
    [switch]$Clean,

    [Parameter()]
    [switch]$Verify,

    [Parameter()]
    [string]$Target
)

$ErrorActionPreference = "Stop"

function Invoke-CleanBuild {
    Write-Host "Cleaning build artifacts..."
    cargo clean
}

function Invoke-Build {
    param(
        [switch]$Release,
        [switch]$WithWrapper,
        [string]$Target
    )

    $buildArgs = @("build")

    if ($Release) {
        $buildArgs += "--release"
    }

    if ($Target) {
        $buildArgs += "--target", $Target
    }

    if ($WithWrapper) {
        $env:RUSTC_WRAPPER = "crando"
    }

    Write-Host "Building with args: $($buildArgs -join ' ')"
    & cargo @buildArgs

    if ($LASTEXITCODE -ne 0) {
        throw "Build failed with exit code $LASTEXITCODE"
    }
}

function Invoke-BuildVerification {
    Write-Host "Running build verification..."

    # Check if binary exists
    $binaryPath = if ($Release) {
        "target/release/crando.exe"
    } else {
        "target/debug/crando.exe"
    }

    if (-not (Test-Path $binaryPath)) {
        throw "Build verification failed: Binary not found at $binaryPath"
    }

    # Run basic version check
    & $binaryPath --version
    if ($LASTEXITCODE -ne 0) {
        throw "Build verification failed: Version check failed"
    }

    Write-Host "✓ Build verification passed"
}

function Get-BuildInfo {
    Write-Host "=== Build Information ==="

    $rustcVersion = rustc --version
    $cargoVersion = cargo --version

    Write-Host "Rustc: $rustcVersion"
    Write-Host "Cargo: $cargoVersion"

    if ($env:RUSTC_WRAPPER) {
        Write-Host "Wrapper: $env:RUSTC_WRAPPER"
    }

    if ($Target) {
        Write-Host "Target: $Target"
    }

    Write-Host ""
}

# Main execution
Write-Host "=== FastRamdisk Build Automation ==="
Write-Host ""

Get-BuildInfo

if ($Clean) {
    Invoke-CleanBuild
}

if ($Debug) {
    Write-Host "Building debug..."
    Invoke-Build -WithWrapper:$WithWrapper -Target $Target
}

if ($Release) {
    Write-Host "Building release..."
    Invoke-Build -Release -WithWrapper:$WithWrapper -Target $Target
}

if ($Verify) {
    Invoke-BuildVerification
}

Write-Host ""
Write-Host "Build complete!"
