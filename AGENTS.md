# Repository Guidelines

**Purpose:** This document provides operational guidelines for AI agents working on the jcode codebase. It is self-contained and should be read before starting any work on this repository.

**Last Updated:** 2026-05-03

---

## Table of Contents

- [Development Workflow](#development-workflow)
- [Project Structure](#project-structure)
- [Testing Guidelines](#testing-guidelines)
- [Logs and Debugging](#logs-and-debugging)
- [Platform-Aware Scripts](#platform-aware-scripts)
- [Install Notes](#install-notes)
- [Common Patterns](#common-patterns)

---

## Development Workflow

### Git Commit Protocol [P0]

All agents MUST automatically git commit when finishing a task.

**Commit Guidelines:**
- Make small, focused commits after completing each feature, fix, or refactoring
- If git state is not clean or other agents are working in parallel, still commit your work
- Push all commits to remote when finishing a task or session
- Use conventional commit format when applicable

**Commit Triggers:**
- After completing each feature, fix, or refactoring
- After passing all relevant tests
- After any substantial code change (>30 lines OR 3+ files modified)
- Small changes (≤30 lines and <3 files) should not be committed unless completing a task

### Build Strategy

**Fast Iteration (during development):**
- Prefer `cargo check`, targeted tests, and dev builds while iterating
- Use incremental builds to speed up development
- Focus on the specific code path you're modifying

**Full Rebuild (when done):**
- When you are done making changes and other agents are not working in the codebase
- Build the full source to ensure no integration issues
- Verify all tests pass before considering the task complete

### Release Process

**Version Bumping:**
- Update version in `Cargo.toml` when making releases
- Look at all changes since the last release to determine bump type:
  - **Patch:** Bug fixes, minor improvements
  - **Minor:** New features, backward-compatible changes
  - **Major:** Breaking changes, significant rewrites

**Release Checklist:**
- All tests passing
- Documentation updated
- Version bumped appropriately
- Release notes prepared
- Tagged and pushed

---

## Project Structure

### Directory Layout

```
jcode/
├── .github/           # GitHub Actions workflows
├── crates/            # Workspace crates
├── docs/              # Project documentation
├── scripts/           # Build and utility scripts
├── src/               # Main source code
├── tests/             # Integration and E2E tests
├── Cargo.toml         # Workspace manifest
└── AGENTS.md          # This file
```

### Key Directories

**`crates/`** - Workspace crates
- Each crate is a separate Rust package
- Shared functionality organized by domain
- See `Cargo.toml` for workspace members

**`scripts/`** - Build and utility scripts
- Platform-aware (`.sh` for Unix, `.ps1` for Windows)
- See `scripts/WINDOWS_SCRIPT_STATUS.md` for equivalents

---

## Testing Guidelines

### Test Priorities

1. **Unit tests** - Fast, isolated tests for individual functions
2. **Integration tests** - Tests that verify crate interactions
3. **E2E tests** - Full-system tests in `tests/e2e/`

### Test Execution

**During development:**
- Run targeted tests for the code you're modifying
- Use `cargo test <package>` to test specific crates
- Use `cargo test <test_name>` to run specific tests

**Before completion:**
- Run the full test suite
- Ensure all tests pass
- Check for test coverage gaps

### Test Patterns

Use the AAA pattern (Arrange-Act-Assert) for all tests:
```rust
#[test]
fn test_example() {
    // Arrange
    let input = create_test_input();

    // Act
    let result = process(input);

    // Assert
    assert_eq!(result, expected);
}
```

---

## Logs and Debugging

### Log Location

Logs are written to `~/.jcode/logs/` with daily files named `jcode-YYYY-MM-DD.log`.

### Debug Socket

Use the debug socket for runtime-level debugging. This provides real-time access to the running process state.

### Debugging Tips

**For build issues:**
- Check `cargo check` output for compilation errors
- Use `RUST_BACKTRACE=1` for stack traces
- Review build logs in CI for reproducible issues

**For runtime issues:**
- Enable debug logging if available
- Use the debug socket for live inspection
- Check log files for error patterns

---

## Platform-Aware Scripts

### Script Equivalents

The repository maintains PowerShell (`.ps1`) equivalents for most shell scripts (`.sh`):

- **Windows:** Automatically uses `.ps1` versions
- **Unix/Linux/macOS:** Automatically uses `.sh` versions

### Dynamic Script Selection

The `src/platform.rs` module provides helpers for platform-appropriate script paths:

```rust
use crate::platform::platform_script_path;

// Returns the platform-appropriate script path
let script_path = platform_script_path("scripts/foo.sh");
// On Windows: scripts/foo.ps1
// On Unix: scripts/foo.sh
```

### Script Reference

See `scripts/WINDOWS_SCRIPT_STATUS.md` for:
- Complete list of script equivalents
- Platform-specific scripts
- Script availability by platform

---

## Install Notes

### Unix/Linux/macOS

**Installation Paths:**
- `~/.local/bin/jcode` - Launcher symlink (in PATH)
- `~/.jcode/builds/current/jcode` - Active local/source-build channel
- `~/.jcode/builds/stable/jcode` - Stable release channel
- `~/.jcode/builds/versions/<version>/jcode` - Immutable versioned binaries
- `~/.jcode/builds/canary/jcode` - Canary/testing channel (legacy)

**PATH Configuration:**
Ensure `~/.local/bin` is **before** `~/.cargo/bin` in your PATH.

### Windows

**Installation Paths:**
- `%LOCALAPPDATA%\jcode\bin\jcode.exe` - Launcher
- `%LOCALAPPDATA%\jcode\builds\stable\jcode.exe` - Stable release
- `%LOCALAPPDATA%\jcode\builds\versions\<version>\jcode.exe` - Versioned installs

**Install Script:**
`scripts/install.ps1` currently installs the stable channel.

### Installation Scripts

- `scripts/install.sh` / `scripts/install.ps1` - Install stable release
- `scripts/install_release.sh` - Install from local source build
- `scripts/uninstall.sh` / `scripts/uninstall.ps1` - Remove installation

---

## Common Patterns

### Before Starting Work

1. Read this AGENTS.md file
2. Read the main project documentation in `docs/`
3. Ensure all tests pass before making changes
4. Check the GitHub Actions workflows in `.github/workflows/` for CI/CD patterns

### When Modifying Code

- Add descriptive code comments explaining the reason for changes
- Check project documentation for file dependencies
- Update ALL affected files together
- Follow existing style and patterns in the codebase
- Write general-purpose solutions

### When Finishing Work

1. Ensure all tests pass
2. Commit changes with descriptive message
3. Push commits to remote
