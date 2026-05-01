# PowerShell Script Equivalents Status

This document tracks which `.sh` scripts have PowerShell equivalents and which are Linux/macOS-specific.

## Scripts with PowerShell Equivalents

✅ **Created PowerShell equivalents:**
- `dev_cargo.ps1` - Build environment configuration
- `cargo_exec.ps1` - Cargo execution wrapper
- `check_warning_budget.ps1` - Warning budget checker
- `check_startup_budget.ps1` - Startup budget checker
- `test_fast.ps1` - Fast test runner
- `test_e2e.ps1` - End-to-end test runner
- `security_preflight.ps1` - Security preflight checks
- `agent_trace.ps1` - Agent tracing
- `debug_socket_test.ps1` - Debug socket testing (Unix sockets - WSL only)
- `refactor_shadow.ps1` - Refactor shadow environment
- `refactor_phase1_verify.ps1` - Refactor phase 1 verification
- `test_auth_e2e.ps1` - Auth E2E testing
- `bench_compile.ps1` - Compile benchmarking
- `bench_selfdev_checkpoints.ps1` - Selfdev checkpoint benchmarks
- `real_provider_smoke.ps1` - Real provider smoke tests
- `onboarding_sandbox.ps1` - Onboarding sandbox management
- `auth_regression_matrix.ps1` - Auth regression matrix testing

✅ **Already had PowerShell equivalents:**
- `install.ps1` - Installation
- `install_dev.ps1` - Development installation
- `install_release.ps1` - Release installation
- `check_powershell_syntax.ps1` - PowerShell syntax checker (no .sh equivalent)
- `invoke_cargo_with_timeout.ps1` - Cargo with timeout (no .sh equivalent)

## Linux/macOS-Specific Scripts (No Windows Equivalent Needed)

❌ **Docker/Linux-specific:**
- `build_linux_compat.sh` - Builds Linux x86_64 in Docker for older glibc baseline

❌ **Package Management (Linux/macOS):**
- `update_packages.sh` - Updates Homebrew tap and AUR packages

❌ **Release/Publishing:**
- `quick-release.sh` - Builds Linux/macOS releases using Docker and osxcross

❌ **Unix Sockets/ProcFS:**
- `benchmark_tools.sh` - Uses Unix sockets and /proc filesystem
- `stress_test_40.sh` - Uses /proc, lsof, Unix sockets for stress testing

❌ **macOS-Specific:**
- `mobile_simulator_tester.sh` - iOS simulator testing
- `mobile_simulator_smoke.sh` - iOS simulator smoke tests
- `screenshot_watcher.sh` - macOS screenshot tools
- `capture_screenshot.sh` - macOS screenshot capture
- `capture_demo.sh` - macOS demo capture
- `auto_screenshot.sh` - macOS automated screenshots
- `record_demo.sh` - macOS demo recording
- `replay_recording.sh` - macOS demo replay

❌ **Remote/SSH:**
- `remote_build.sh` - SSH/rsync-based remote cargo builds (would need WinRM adaptation)

❌ **Platform-Specific Tools:**
- `run_terminal_bench_harbor.sh` - Harbor-specific benchmarking

## Summary

- **Total .sh scripts:** 35
- **With .ps1 equivalents:** 17 (created) + 4 (existing) = 21
- **Linux/macOS-specific (no .ps1 needed):** 14
- **Coverage:** 60% of scripts have Windows equivalents; the remaining 40% are platform-specific tools that don't apply to Windows
