# Repository Guidelines

## Development Workflow

- **Automatic Git Commits [P0]** - All agents MUST automatically git commit when finishing a task (see .windsurf/rules/windsurfrules.md for detailed protocol)
- Make small, focused commits after completing each feature, fix, or refactoring
- If the git state is not clean, or there are other agents working in the codebase in parallel, do your best to still commit your work.
- **Push when done** - Push all commits to remote when finishing a task or session
- **Use fast iteration by default** - Prefer `cargo check`, targeted tests, and dev builds while iterating
- **Rebuild when done** - When you are done making changes, and other agents are not working on the codebase, build the source.
- **Bump version for releases** - Update version in `Cargo.toml` when making releases. When cutting a new release, look at all the changes that happened since the last release and determine what the version bump should be ie patch or minor, etc.

## Logs

- Logs are written to `~/.jcode/logs/` (daily files like `jcode-YYYY-MM-DD.log`).

## Debug Socket

- Use the debug socket for runtime level debugging.

## Platform-Aware Scripts

- The repository has PowerShell (`.ps1`) equivalents for most shell scripts (`.sh`)
- On Windows, scripts automatically use `.ps1` versions; on Unix/Linux/macOS, they use `.sh` versions
- The `src/platform.rs` module provides `script_extension()` and `platform_script_path()` helpers for dynamic script selection
- When referencing scripts in code, use `crate::platform::platform_script_path("scripts/foo.sh")` to get the platform-appropriate path
- See `scripts/WINDOWS_SCRIPT_STATUS.md` for a complete list of script equivalents and platform-specific scripts

## Install Notes

- `~/.local/bin/jcode` is the launcher symlink used from `PATH`.
- `~/.jcode/builds/current/jcode` is the active local/source-build channel; self-dev builds and `scripts/install_release.sh` point the launcher here.
- `~/.jcode/builds/stable/jcode` is the stable release channel; `scripts/install.sh` installs this and points the launcher here.
- `~/.jcode/builds/versions/<version>/jcode` stores immutable binaries.
- `~/.jcode/builds/canary/jcode` still exists for canary/testing flows, but it is not the primary self-dev install path.
- On Windows, the equivalents are `%LOCALAPPDATA%\\jcode\\bin\\jcode.exe` for the launcher, `%LOCALAPPDATA%\\jcode\\builds\\stable\\jcode.exe` for stable, and `%LOCALAPPDATA%\\jcode\\builds\\versions\\<version>\\jcode.exe` for immutable installs; `scripts/install.ps1` currently installs the stable channel.
- Ensure `~/.local/bin` is **before** `~/.cargo/bin` in `PATH`.
