<#
.SYNOPSIS
    Install the locally-built jcode dev build into the version store
    and point the launcher at the dev channel.

.DESCRIPTION
    Builds with `cargo +nightly build` using the optimized dev profile
    (Cranelift backend, -Zthreads=8, -Zshare-generics), copies the resulting
    jcode.exe into ~\.jcode\builds\versions\<hash>\jcode.exe,
    refreshes the dev channel files, writes the matching version markers,
    and points ~\.jcode\bin\jcode.exe at the dev channel.

    This is for development builds with maximum compilation speed optimizations.

.PARAMETER InstallDir
    Override the launcher install directory. Default:
    $env:USERPROFILE\.jcode\bin

.EXAMPLE
    pwsh scripts/install_dev.ps1
#>
[CmdletBinding()]
param(
    [string]$InstallDir
)

$ErrorActionPreference = 'Stop'

$repoRoot = $null
try {
    $repoRoot = git rev-parse --show-toplevel 2>$null
} catch {
    # Ignore git errors
}
if ($LASTEXITCODE -ne 0 -or -not $repoRoot) {
    # Fall back to script directory parent (scripts/ is in repo root)
    $repoRoot = Split-Path -Parent $PSScriptRoot
}

Write-Host "Building with nightly toolchain and Cranelift (optimized for speed)..."
Write-Host "This uses: -Zthreads=8, -Zshare-generics"
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

& cargo +nightly build --manifest-path (Join-Path $repoRoot 'Cargo.toml')
if ($LASTEXITCODE -ne 0) {
    Write-Error "cargo build failed with exit code $LASTEXITCODE"
    exit $LASTEXITCODE
}

$binSource = Join-Path $repoRoot "target/debug/jcode.exe"
if (-not (Test-Path -LiteralPath $binSource -PathType Leaf)) {
    Write-Error "Dev binary not found: $binSource"
    exit 1
}

# Resolve a short hash for the version directory
$hash = ''
try {
    $sha = & git -C $repoRoot rev-parse --short HEAD 2>$null
    if ($LASTEXITCODE -eq 0 -and $sha) {
        $hash = $sha.Trim()
        $dirty = & git -C $repoRoot status --porcelain 2>$null
        if ($LASTEXITCODE -eq 0 -and $dirty) {
            $hash = "$hash-dirty"
        }
    }
} catch {
    $hash = ''
}
if (-not $hash) {
    $hash = (Get-Date).ToString('yyyyMMddHHmmss')
}

$jcodeHome = if ($env:JCODE_HOME) { $env:JCODE_HOME } else { Join-Path ([Environment]::GetFolderPath('UserProfile')) '.jcode' }
$buildsDir = Join-Path $jcodeHome 'builds'
$versionDir = Join-Path $buildsDir "versions/$hash"
$null = New-Item -ItemType Directory -Force -Path $versionDir

# replace_executable_atomic-equivalent: rename any in-use .exe aside, then copy.
function Update-JcodeExe {
    param(
        [Parameter(Mandatory)] [string]$Source,
        [Parameter(Mandatory)] [string]$Destination
    )
    if (Test-Path -LiteralPath $Destination -PathType Leaf) {
        $stem = [System.IO.Path]::GetFileName($Destination)
        $parent = [System.IO.Path]::GetDirectoryName($Destination)
        $nanos = [DateTimeOffset]::UtcNow.ToUnixTimeMilliseconds()
        $aside = Join-Path $parent ".$stem.$PID-$nanos.old"
        try {
            Move-Item -LiteralPath $Destination -Destination $aside -Force -ErrorAction Stop
        } catch {
            # If rename fails the destination probably is not actually in use;
            # try a direct copy/overwrite as the fallback.
        }
    }
    Copy-Item -LiteralPath $Source -Destination $Destination -Force
}

$versionedExe = Join-Path $versionDir 'jcode.exe'
Update-JcodeExe -Source $binSource -Destination $versionedExe

# Update dev channel
$devDir = Join-Path $buildsDir 'dev'
$null = New-Item -ItemType Directory -Force -Path $devDir
Update-JcodeExe -Source $versionedExe -Destination (Join-Path $devDir 'jcode.exe')
Set-Content -LiteralPath (Join-Path $buildsDir 'dev-version') -Value $hash -NoNewline:$false

if (-not $InstallDir) {
    $InstallDir = if ($env:JCODE_INSTALL_DIR) { $env:JCODE_INSTALL_DIR } else { Join-Path $jcodeHome 'bin' }
}
$null = New-Item -ItemType Directory -Force -Path $InstallDir
$launcher = Join-Path $InstallDir 'jcode.exe'
Update-JcodeExe -Source (Join-Path $devDir 'jcode.exe') -Destination $launcher

# Best-effort sweep of stale .old sidecars
foreach ($dir in @($devDir, $versionDir, $InstallDir)) {
    Get-ChildItem -LiteralPath $dir -Filter '*.old' -Force -ErrorAction SilentlyContinue |
        Where-Object { $_.Name.StartsWith('.') } |
        ForEach-Object { Remove-Item -LiteralPath $_.FullName -Force -ErrorAction SilentlyContinue }
}

Write-Host ""
Write-Host "Installed:        $versionedExe"
Write-Host "Dev channel:      $(Join-Path $devDir 'jcode.exe')"
Write-Host "Launcher:         $launcher"

$pathDirs = ($env:PATH -split [System.IO.Path]::PathSeparator) | Where-Object { $_ }
if ($pathDirs -notcontains $InstallDir) {
    Write-Host ""
    Write-Host "Tip: add $InstallDir to PATH (User scope) so 'jcode' resolves in new shells."
    Write-Host "     Example: setx PATH ""%PATH%;$InstallDir"""
}
