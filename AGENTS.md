# Repository Guidelines

**Purpose:** This document provides operational guidelines for AI agents working on the jcode codebase. It is self-contained, comprehensive, and should be read before starting any work on this repository.

**Audience:** AI agents, contributors, and developers working on the jcode codebase.

**Last Updated:** 2026-05-03

---

## Quick Start

If you're new to this repository, start here:

1. Read this document in full
2. Explore the project structure in `docs/`
3. Run `cargo check` to verify the build
4. Review existing tests with `cargo test`
5. Check `.github/workflows/` for CI/CD patterns

---

## Table of Contents

- [Quick Start](#quick-start)
- [Development Workflow](#development-workflow)
- [Project Structure](#project-structure)
- [Testing Guidelines](#testing-guidelines)
- [Logs and Debugging](#logs-and-debugging)
- [Platform-Aware Scripts](#platform-aware-scripts)
- [Install Notes](#install-notes)
- [Common Patterns](#common-patterns)
- [Troubleshooting](#troubleshooting)
- [Getting Help](#getting-help)

---

## Development Workflow

### Git Commit Protocol [P0]

All agents MUST automatically git commit when finishing a task.

**Commit Guidelines:**
- Make small, focused commits after completing each feature, fix, or refactoring
- If git state is not clean or other agents are working in parallel, still commit your work
- Push all commits to remote when finishing a task or session
- Use conventional commit format when applicable (e.g., `feat: add new feature`, `fix: resolve bug`)
- Write commit messages that explain WHAT and WHY, not HOW

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
- Example: `cargo check --package jcode-core`

**Full Rebuild (when done):**
- When you are done making changes and other agents are not working in the codebase
- Build the full source to ensure no integration issues
- Verify all tests pass before considering the task complete
- Example: `cargo build --release && cargo test`

### Release Process

**Version Bumping:**
- Update version in `Cargo.toml` when making releases
- Look at all changes since the last release to determine bump type:
  - **Patch:** Bug fixes, minor improvements
  - **Minor:** New features, backward-compatible changes
  - **Major:** Breaking changes, significant rewrites

**Release Checklist:**
- [ ] All tests passing
- [ ] Documentation updated
- [ ] Version bumped appropriately
- [ ] Release notes prepared
- [ ] Tagged and pushed
- [ ] CI/CD pipelines verified

---

## Project Structure

### Directory Layout

```text
jcode/
├── .github/           # GitHub Actions workflows (CI/CD, automation)
├── crates/            # Workspace crates (modular Rust packages)
├── docs/              # Project documentation (architecture, guides)
├── scripts/           # Build and utility scripts (platform-aware)
├── src/               # Main source code (core application)
├── tests/             # Integration and E2E tests
├── Cargo.toml         # Workspace manifest (dependencies, members)
└── AGENTS.md          # This file (agent guidelines)
```

### Key Directories

**`crates/`** - Workspace crates
- Each crate is a separate Rust package with its own `Cargo.toml`
- Shared functionality organized by domain (e.g., `jcode-core`, `jcode-auth`)
- See workspace `Cargo.toml` for the complete member list
- Crates can depend on each other using relative paths

**`docs/`** - Project documentation
- Architecture documents and design specifications
- API documentation and usage guides
- Contributing guidelines and development workflows
- Review this directory before making architectural changes

**`scripts/`** - Build and utility scripts
- Platform-aware (`.sh` for Unix, `.ps1` for Windows)
- Installation, build, and maintenance scripts
- See `scripts/WINDOWS_SCRIPT_STATUS.md` for script equivalents

---

## Testing Guidelines

### Test Priorities

1. **Unit tests** - Fast, isolated tests for individual functions
   - Located in each crate's `src/` or `tests/` directory
   - Should run in < 1 second per test
   - Test pure functions and business logic

2. **Integration tests** - Tests that verify crate interactions
   - Located in crate `tests/` directories
   - Test module boundaries and interfaces
   - Should run in < 10 seconds per test

3. **E2E tests** - Full-system tests in `tests/e2e/`
   - Test complete workflows and user scenarios
   - May be slower but provide high confidence
   - Run before releases

### Test Execution

**During development:**
- Run targeted tests for the code you're modifying
  - Example: `cargo test --package jcode-core`
  - Example: `cargo test test_function_name`
- Use `cargo test <package>` to test specific crates
- Use `cargo test <test_name>` to run specific tests
- Use `cargo test --lib` for library-only tests

**Before completion:**
- Run the full test suite: `cargo test --all`
- Ensure all tests pass
- Check for test coverage gaps: `cargo llvm-cov` (if available)
- Run tests on multiple platforms if applicable

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
- Use `RUST_BACKTRACE=1` for stack traces: `RUST_BACKTRACE=1 cargo test`
- Review build logs in CI for reproducible issues
- Try `cargo clean` if you encounter strange build errors
- Check for dependency conflicts with `cargo tree`

**For runtime issues:**
- Enable debug logging if available
- Use the debug socket for live inspection
- Check log files in `~/.jcode/logs/` for error patterns
- Use `RUST_LOG=debug` environment variable for verbose output
- Isolate the issue with minimal test cases

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
- Migration notes between Unix and Windows

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

**Note:** Always run scripts from the repository root directory.

---

## Common Patterns

### Before Starting Work

1. Read this AGENTS.md file in full
2. Read the main project documentation in `docs/`
3. Ensure all tests pass: `cargo test --all`
4. Check the GitHub Actions workflows in `.github/workflows/` for CI/CD patterns
5. Review the `Cargo.toml` to understand workspace structure
6. Check for open issues or PRs that might be related to your work

### When Modifying Code

- Add descriptive code comments explaining the reason for changes
  - Focus on WHY, not WHAT (code shows what)
- Check project documentation for file dependencies
- Update ALL affected files together in a single commit when possible
- Follow existing style and patterns in the codebase
  - Use `rustfmt` for consistent formatting
  - Follow naming conventions (snake_case for functions, PascalCase for types)
- Write general-purpose solutions that are reusable
- Add tests for new functionality

### When Finishing Work

1. Ensure all tests pass: `cargo test --all`
2. Run `cargo check --all` to verify no compilation errors
3. Commit changes with descriptive message
4. Push commits to remote
5. Verify CI/CD pipeline passes
6. Update relevant documentation if needed

---

## Troubleshooting

### Common Build Issues

**Issue:** `cargo check` fails with dependency errors
- **Solution:** Run `cargo update` to update dependencies
- **Alternative:** Check `Cargo.lock` for version conflicts

**Issue:** Tests pass locally but fail in CI
- **Solution:** Check platform-specific differences (Windows vs Unix)
- **Solution:** Verify environment variables match CI configuration

**Issue:** Incremental build produces unexpected behavior
- **Solution:** Run `cargo clean` to clear cache
- **Solution:** Rebuild with `cargo build --release`

### Common Runtime Issues

**Issue:** Application crashes on startup
- **Solution:** Check logs in `~/.jcode/logs/`
- **Solution:** Enable debug logging with `RUST_LOG=debug`

**Issue:** Performance degradation after changes
- **Solution:** Profile with `cargo flamegraph` or similar tools
- **Solution:** Check for unnecessary allocations or clones

### Common Git Issues

**Issue:** Merge conflicts after pulling changes
- **Solution:** Use `git pull --rebase` to maintain clean history
- **Solution:** Resolve conflicts carefully, preserving both sides' intent

---

## Getting Help

### Documentation Resources

- **Project docs:** `docs/` directory
- **API docs:** Run `cargo doc --open` to generate and view API documentation
- **Rust docs:** https://doc.rust-lang.org/

### Community Resources

- **Issues:** Check GitHub Issues for known problems
- **Discussions:** Use GitHub Discussions for questions
- **Contributing:** See CONTRIBUTING.md for contribution guidelines

### Debugging Resources

- **Logs:** `~/.jcode/logs/` directory
- **Debug socket:** Runtime inspection tool
- **CI logs:** GitHub Actions workflow runs
