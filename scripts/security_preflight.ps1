#!/usr/bin/env pwsh
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
$strict = $false

function Show-Usage {
    Write-Output @"
Usage:
  scripts/security_preflight.ps1 [--strict]

Checks:
  1) Secret-pattern scan in tracked source/docs/scripts
  2) World-writable file check under scripts/
  3) Rust dependency advisory scan via cargo-audit (when available)

Options:
  --strict   Fail if cargo-audit is not installed
"@
}

function Write-Error-Exit {
    param([string]$Message)
    Write-Error "error: $Message"
    exit 1
}

for ($i = 0; $i -lt $args.Count; $i++) {
    switch ($args[$i]) {
        "--strict" {
            $strict = $true
        }
        { $_ -in "-h", "--help" } {
            Show-Usage
            exit 0
        }
        default {
            Write-Error-Exit "unknown option: $($args[$i])"
        }
    }
}

Set-Location $repoRoot

Write-Output "=== Security Preflight ==="

Write-Output "[1/3] Scanning for likely secrets"
$secretRegex = '(AKIA[0-9A-Z]{16}|ASIA[0-9A-Z]{16}|gh[pousr]_[A-Za-z0-9]{36,}|xox[baprs]-[A-Za-z0-9-]{10,}|-----BEGIN (RSA|OPENSSH|EC|DSA|PGP) PRIVATE KEY-----|AIza[0-9A-Za-z_-]{35})'

$trackedFiles = git ls-files
if ($trackedFiles) {
    $excludePatterns = @('*.snap', '*.png', '*.jpg', '*.jpeg', '*.gif', '*.svg', '*.pdf', '*.woff', '*.woff2', '*.ttf', 'Cargo.lock')
    $scanFiles = $trackedFiles | Where-Object {
        $file = $_
        -not ($excludePatterns | Where-Object { $file -like $_ })
    }
    
    if ($scanFiles) {
        $scanResults = $scanFiles | Select-String -Pattern $secretRegex
        if ($scanResults) {
            $scanResults | ForEach-Object { Write-Output $_.ToString() }
            Write-Error-Exit "potential secret material detected"
        }
    }
}

Write-Output "[2/3] Checking script permissions"
# On Windows, check if scripts folder has overly permissive ACLs
$scriptsPath = "$repoRoot/scripts"
if (Test-Path $scriptsPath) {
    $acl = Get-Acl $scriptsPath
    # Check for "Everyone" or "Anonymous Logon" with write access
    foreach ($access in $acl.Access) {
        if ($access.IdentityReference.Value -in "Everyone", "Anonymous Logon", "BUILTIN\Users") {
            if ($access.FileSystemRights -match "Write") {
                Write-Error-Exit "world-writable permissions detected under scripts/"
            }
        }
    }
}

Write-Output "[3/3] Dependency advisories (cargo-audit)"
$cargoAuditFound = $false
try {
    $null = cargo audit --version 2>&1
    $cargoAuditFound = $true
} catch {
    # Try cargo-audit command
    if (Get-Command cargo-audit -ErrorAction SilentlyContinue) {
        $cargoAuditFound = $true
    }
}

if ($cargoAuditFound) {
    & cargo audit
} else {
    if ($strict) {
        Write-Error-Exit "cargo-audit is not installed (install with: cargo install cargo-audit --locked)"
    }
    Write-Output "warning: cargo-audit not installed; skipping advisory check"
}

Write-Output "=== Security preflight passed ==="
