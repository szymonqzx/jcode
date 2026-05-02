#!/usr/bin/env pwsh
<#
.SYNOPSIS
    Dependency management script for FastRamdisk project.

.DESCRIPTION
    This script checks, installs, and updates project dependencies including
    Rust tools, WinFsp, and other required components.

.PARAMETER Check
    Check if all dependencies are installed. Default: true

.PARAMETER Install
    Install missing dependencies. Default: false

.PARAMETER Update
    Update all dependencies. Default: false

.PARAMETER Rust
    Include Rust toolchain in operations. Default: true

.PARAMETER WinFsp
    Include WinFsp in operations. Default: true

.PARAMETER MemFs
    Include WinFsp-MemFs-Extended in operations. Default: true
#>

param(
    [Parameter()]
    [switch]$Check,

    [Parameter()]
    [switch]$Install,

    [Parameter()]
    [switch]$Update,

    [Parameter()]
    [switch]$Rust,

    [Parameter()]
    [switch]$WinFsp,

    [Parameter()]
    [switch]$MemFs
)

$ErrorActionPreference = "Stop"

function Test-RustToolchain {
    Write-Host "Checking Rust toolchain..."

    $rustc = Get-Command rustc -ErrorAction SilentlyContinue
    $cargo = Get-Command cargo -ErrorAction SilentlyContinue

    if ($rustc -and $cargo) {
        Write-Host "✓ Rust toolchain installed"
        Write-Host "  rustc: $(rustc --version)"
        Write-Host "  cargo: $(cargo --version)"
        return $true
    } else {
        Write-Host "✗ Rust toolchain not found"
        return $false
    }
}

function Test-WinFsp {
    Write-Host "Checking WinFsp..."

    $winfspPath = "${env:ProgramFiles}\WinFsp"
    if (Test-Path $winfspPath) {
        Write-Host "✓ WinFsp installed at $winfspPath"
        return $true
    } else {
        Write-Host "✗ WinFsp not found"
        return $false
    }
}

function Test-MemFs {
    Write-Host "Checking WinFsp-MemFs-Extended (memefs.exe)..."

    $memefs = Get-Command memefs -ErrorAction SilentlyContinue
    if ($memefs) {
        Write-Host "✓ memefs.exe found at $($memefs.Source)"
        return $true
    } else {
        Write-Host "✗ memefs.exe not found in PATH"
        return $false
    }
}

function Test-CargoTools {
    Write-Host "Checking cargo tools..."

    $tools = @{
        "cargo-tarpaulin" = "cargo-tarpaulin"
        "grcov" = "grcov"
    }

    $allInstalled = $true
    foreach ($tool in $tools.GetEnumerator()) {
        $installed = Get-Command $tool.Value -ErrorAction SilentlyContinue
        if ($installed) {
            Write-Host "✓ $($tool.Key) installed"
        } else {
            Write-Host "✗ $($tool.Key) not found"
            $allInstalled = $false
        }
    }

    return $allInstalled
}

function Install-RustToolchain {
    Write-Host "Installing Rust toolchain..."
    Write-Host "Please visit https://rustup.rs/ to install Rust"
    Write-Host "Or run: winget install Rustlang.Rustup"
}

function Install-WinFsp {
    Write-Host "Installing WinFsp..."
    Write-Host "Please download from: https://github.com/winfsp/winfsp/releases"
    Write-Host "Or run: winget install winfsp"
}

function Install-MemFs {
    Write-Host "Installing WinFsp-MemFs-Extended..."
    Write-Host "Please download from: https://github.com/Ceiridge/WinFsp-MemFs-Extended/releases"
    Write-Host "Add memefs.exe to your PATH after installation"
}

function Install-CargoTools {
    Write-Host "Installing cargo tools..."

    Write-Host "Installing cargo-tarpaulin..."
    cargo install cargo-tarpaulin

    Write-Host "Installing grcov..."
    cargo install grcov
}

function Update-CargoTools {
    Write-Host "Updating cargo tools..."

    Write-Host "Updating cargo-tarpaulin..."
    cargo install cargo-tarpaulin --force

    Write-Host "Updating grcov..."
    cargo install grcov --force
}

function Update-RustToolchain {
    Write-Host "Updating Rust toolchain..."
    rustup update
}

# Main execution
Write-Host "=== FastRamdisk Dependency Management ==="
Write-Host ""

$missingDeps = @()

if ($Rust) {
    if (-not (Test-RustToolchain)) {
        $missingDeps += "Rust toolchain"
    }
}

if ($WinFsp) {
    if (-not (Test-WinFsp)) {
        $missingDeps += "WinFsp"
    }
}

if ($MemFs) {
    if (-not (Test-MemFs)) {
        $missingDeps += "WinFsp-MemFs-Extended"
    }
}

Test-CargoTools

if ($Install -and $missingDeps.Count -gt 0) {
    Write-Host ""
    Write-Host "Installing missing dependencies..."

    if ($missingDeps -contains "Rust toolchain") {
        Install-RustToolchain
    }

    if ($missingDeps -contains "WinFsp") {
        Install-WinFsp
    }

    if ($missingDeps -contains "WinFsp-MemFs-Extended") {
        Install-MemFs
    }

    Install-CargoTools
}

if ($Update) {
    Write-Host ""
    Write-Host "Updating dependencies..."

    if ($Rust) {
        Update-RustToolchain
    }

    Update-CargoTools
}

Write-Host ""
if ($missingDeps.Count -eq 0) {
    Write-Host "✓ All dependencies are installed!"
} else {
    Write-Host "Missing dependencies: $($missingDeps -join ', ')"
    Write-Host "Run with -Install to install missing dependencies."
}
