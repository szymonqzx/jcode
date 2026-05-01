#!/usr/bin/env pwsh
$ErrorActionPreference = "Stop"

$repoRoot = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)
Set-Location $repoRoot

function Log {
    param([string[]]$Message)
    Write-Output "dev_cargo: $Message"
}

$selectedLinkerMode = "not-configured"
$selectedLinkerDesc = ""
$sccacheStatus = "disabled"
$selfdevLowMemoryStatus = "disabled"

function Append-Rustflags {
    param([string]$NewFlag)
    $envKey = "CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS"
    $currentValue = [Environment]::GetEnvironmentVariable($envKey)
    if ([string]::IsNullOrEmpty($currentValue)) {
        [Environment]::SetEnvironmentVariable($envKey, $NewFlag)
    } else {
        [Environment]::SetEnvironmentVariable($envKey, "$currentValue $NewFlag")
    }
}

function Maybe-Enable-Sccache {
    if (-not [string]::IsNullOrEmpty($env:RUSTC_WRAPPER)) {
        $script:sccacheStatus = "external:$($env:RUSTC_WRAPPER)"
        Log "keeping existing RUSTC_WRAPPER=$($env:RUSTC_WRAPPER)"
        return
    }
    if (Get-Command sccache -ErrorAction SilentlyContinue) {
        $null = sccache --start-server 2>$null
        $env:RUSTC_WRAPPER = "sccache"
        $script:sccacheStatus = "enabled"
        Log "using sccache"
    } else {
        $script:sccacheStatus = "not-found"
        Log "sccache not found; using direct rustc"
    }
}

function Uses-Selfdev-Profile {
    param([string[]]$Args)
    $expectProfileName = $false
    foreach ($arg in $Args) {
        if ($expectProfileName) {
            if ($arg -eq "selfdev") {
                return $true
            }
            $expectProfileName = $false
            continue
        }

        switch -Regex ($arg) {
            "^--profile=selfdev$" {
                return $true
            }
            "^--profile$" {
                $expectProfileName = $true
            }
        }
    }
    return $false
}

function Get-MemInfo-KiB {
    param([string]$Key)
    # Windows equivalent: Get memory info via WMI/CIM
    $mem = Get-CimInstance Win32_ComputerSystem
    $os = Get-CimInstance Win32_OperatingSystem
    switch ($Key) {
        "MemTotal" {
            return [math]::Floor($mem.TotalPhysicalMemory / 1KB)
        }
        "SwapTotal" {
            return [math]::Floor($os.SizeStoredInPagingFiles / 1KB)
        }
        default {
            return $null
        }
    }
}

function Selfdev-LowMemory-Default-Needed {
    if ($IsLinux) {
        # Linux logic from original script
        if (-not (Test-Path /proc/meminfo)) {
            return $false
        }
        if (-not (Get-Command pgrep -ErrorAction SilentlyContinue)) {
            return $false
        }
        $null = pgrep -x earlyoom 2>$null
        if ($LASTEXITCODE -ne 0) {
            return $false
        }
        $memTotalKiB = Meminfo-KiB "MemTotal"
        $swapTotalKiB = Meminfo-KiB "SwapTotal"
        if ([string]::IsNullOrEmpty($memTotalKiB) -or [string]::IsNullOrEmpty($swapTotalKiB)) {
            return $false
        }
        return ($swapTotalKiB -eq 0 -and $memTotalKiB -lt 24576 * 1024)
    } elseif ($IsWindows) {
        # Windows: check if low memory (no pagefile and < 24GB RAM)
        $memTotalKiB = Get-MemInfo-KiB "MemTotal"
        $swapTotalKiB = Get-MemInfo-KiB "SwapTotal"
        if ([string]::IsNullOrEmpty($memTotalKiB) -or [string]::IsNullOrEmpty($swapTotalKiB)) {
            return $false
        }
        return ($swapTotalKiB -eq 0 -and $memTotalKiB -lt 24576 * 1024)
    }
    return $false
}

function Maybe-Configure-LowMemory-Selfdev {
    param([string[]]$Args)
    if (-not (Uses-Selfdev-Profile $Args)) {
        $script:selfdevLowMemoryStatus = "not-selfdev"
        return
    }

    $mode = if ($env:JCODE_SELFDEV_LOW_MEMORY) { $env:JCODE_SELFDEV_LOW_MEMORY } else { "auto" }
    switch ($mode) {
        { $_ -in "1", "true", "yes", "on", "force" } {
            # Continue to enable
        }
        { $_ -in "0", "false", "no", "off", "never" } {
            $script:selfdevLowMemoryStatus = "disabled-by-env"
            return
        }
        "auto" {
            if (-not (Selfdev-LowMemory-Default-Needed)) {
                $script:selfdevLowMemoryStatus = "auto-not-needed"
                return
            }
        }
        "" {
            if (-not (Selfdev-LowMemory-Default-Needed)) {
                $script:selfdevLowMemoryStatus = "auto-not-needed"
                return
            }
        }
        default {
            Write-Error "error: unsupported JCODE_SELFDEV_LOW_MEMORY=$mode (expected auto|on|off)"
            exit 1
        }
    }

    $env:CARGO_INCREMENTAL = if ($env:CARGO_INCREMENTAL) { $env:CARGO_INCREMENTAL } else { "0" }
    $env:CARGO_PROFILE_SELFDEV_INCREMENTAL = if ($env:CARGO_PROFILE_SELFDEV_INCREMENTAL) { $env:CARGO_PROFILE_SELFDEV_INCREMENTAL } else { "false" }
    $env:CARGO_PROFILE_SELFDEV_CODEGEN_UNITS = if ($env:CARGO_PROFILE_SELFDEV_CODEGEN_UNITS) { $env:CARGO_PROFILE_SELFDEV_CODEGEN_UNITS } else { "16" }
    $script:selfdevLowMemoryStatus = "enabled:incremental=$($env:CARGO_PROFILE_SELFDEV_INCREMENTAL),codegen-units=$($env:CARGO_PROFILE_SELFDEV_CODEGEN_UNITS)"
    Log "using low-memory selfdev overrides ($($script:selfdevLowMemoryStatus.Substring(8)))"
}

function Configure-Linux-Linker {
    $requestedMode = if ($env:JCODE_FAST_LINKER) { $env:JCODE_FAST_LINKER } else { "auto" }
    $mode = $requestedMode

    switch ($mode) {
        "auto" {
            if ((Get-Command ld.lld -ErrorAction SilentlyContinue) -and (Get-Command clang -ErrorAction SilentlyContinue)) {
                $mode = "lld"
            } elseif ((Get-Command mold -ErrorAction SilentlyContinue) -and (Get-Command clang -ErrorAction SilentlyContinue)) {
                $mode = "mold"
            } else {
                $mode = "system"
            }
        }
        { $_ -in "lld", "mold", "system" } {
            # Valid mode
        }
        default {
            Write-Error "error: unsupported JCODE_FAST_LINKER=$mode (expected auto|lld|mold|system)"
            exit 1
        }
    }

    $script:selectedLinkerMode = $mode
    $envKey = "CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER"
    $currentValue = [Environment]::GetEnvironmentVariable($envKey)
    if ([string]::IsNullOrEmpty($currentValue)) {
        [Environment]::SetEnvironmentVariable($envKey, "clang")
    }

    switch ($mode) {
        "lld" {
            Append-Rustflags "-C link-arg=-fuse-ld=lld"
            $script:selectedLinkerDesc = "clang + lld"
            Log "using clang + lld"
        }
        "mold" {
            Append-Rustflags "-C link-arg=-fuse-ld=mold"
            $script:selectedLinkerDesc = "clang + mold"
            Log "using clang + mold"
        }
        "system" {
            $script:selectedLinkerDesc = "system linker settings"
            if ($requestedMode -eq "auto") {
                Log "no supported fast linker detected; using system linker settings"
            } else {
                Log "using system linker settings"
            }
        }
    }
}

function Configure-Windows-Linker {
    # Windows linker configuration
    $requestedMode = if ($env:JCODE_FAST_LINKER) { $env:JCODE_FAST_LINKER } else { "auto" }
    $mode = $requestedMode

    switch ($mode) {
        "auto" {
            # On Windows, use the default MSVC linker (link.exe)
            # Could potentially use lld-link if available
            if (Get-Command lld-link -ErrorAction SilentlyContinue) {
                $mode = "lld"
            } else {
                $mode = "msvc"
            }
        }
        "lld" {
            # Use lld-link
        }
        "msvc" {
            # Use MSVC link.exe
        }
        default {
            Write-Error "error: unsupported JCODE_FAST_LINKER=$mode on Windows (expected auto|lld|msvc)"
            exit 1
        }
    }

    $script:selectedLinkerMode = $mode
    $envKey = "CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER"

    switch ($mode) {
        "lld" {
            $currentValue = [Environment]::GetEnvironmentVariable($envKey)
            if ([string]::IsNullOrEmpty($currentValue)) {
                [Environment]::SetEnvironmentVariable($envKey, "lld-link")
            }
            $script:selectedLinkerDesc = "lld-link"
            Log "using lld-link"
        }
        "msvc" {
            $script:selectedLinkerDesc = "MSVC linker (link.exe)"
            if ($requestedMode -eq "auto") {
                Log "using default MSVC linker"
            } else {
                Log "using MSVC linker"
            }
        }
    }
}

function Print-Setup {
    Write-Output @"
repo_root=$repoRoot
os=$($PSVersionTable.Platform)
arch=$($env:PROCESSOR_ARCHITECTURE)
sccacheStatus=$sccacheStatus
selfdevLowMemoryStatus=$selfdevLowMemoryStatus
rustc_wrapper=$(if ($env:RUSTC_WRAPPER) { $env:RUSTC_WRAPPER } else { '<unset>' })
linkerMode=$selectedLinkerMode
linkerDesc=$(if ($selectedLinkerDesc) { $selectedLinkerDesc } else { '<none>' })
linker=$(if ($env:CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER) { $env:CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_LINKER } elseif ($env:CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER) { $env:CARGO_TARGET_X86_64_PC_WINDOWS_MSVC_LINKER } else { '<unset>' })
rustflags=$(if ($env:CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS) { $env:CARGO_TARGET_X86_64_UNKNOWN_LINUX_GNU_RUSTFLAGS } else { '<unset>' })
"@
}

# Main execution
$argsPassed = $args

Maybe-Configure-LowMemory-Selfdev $argsPassed
Maybe-Enable-Sccache

if ($IsLinux -and $env:PROCESSOR_ARCHITECTURE -eq "AMD64") {
    Configure-Linux-Linker
} elseif ($IsWindows) {
    Configure-Windows-Linker
}

if ($argsPassed -and $argsPassed[0] -eq "--print-setup") {
    Print-Setup
    exit 0
}

& cargo @argsPassed
exit $LASTEXITCODE
