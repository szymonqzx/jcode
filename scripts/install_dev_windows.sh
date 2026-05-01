#!/usr/bin/env bash
# Install the current dev binary (Windows .exe) built from Linux/WSL2
# into the version store and point the launcher at dev.
#
# This script cross-compiles from Linux to Windows target, producing a .exe
# that can be run on Windows. Useful for development in WSL2 while deploying
# to Windows.
#
# Paths after install:
# - ~/.jcode/builds/versions/<hash>/jcode.exe (immutable)
# - ~/.jcode/builds/dev/jcode.exe -> .../versions/<hash>/jcode.exe
# - ~/.local/bin/jcode.exe -> ~/.jcode/builds/dev/jcode.exe (launcher)
#
# This uses nightly toolchain with Cranelift backend and nightly optimizations:
# -Zthreads=8, -Zshare-generics for maximum compilation speed
set -euo pipefail

# Use script directory as base (more reliable in WSL2)
repo_root="$(cd "$(dirname "$0")/.." && pwd)"

if [[ "$#" -gt 0 ]]; then
  echo "Usage: $0" >&2
  exit 1
fi

# Add Windows target if not present
if ! rustup target list --installed | grep -q "x86_64-pc-windows-msvc"; then
  echo "Adding Windows target..."
  rustup target add x86_64-pc-windows-msvc
fi

echo "Building with nightly toolchain and Cranelift (optimized for speed)..."
echo "This uses: -Zthreads=8, -Zshare-generics, Cranelift backend"
echo "Cross-compiling to Windows target (x86_64-pc-windows-msvc)"
echo ""

# Enable sccache if available for compilation caching
if command -v sccache >/dev/null 2>&1; then
  export RUSTC_WRAPPER=sccache
  echo "sccache enabled for compilation caching"
else
  echo "sccache not found, using standard cargo caching"
fi

# Linux-specific optimizations
if [[ "$(uname -s)" = "Linux" ]]; then
  echo "Applying Linux-specific optimizations (native CPU, lld linker)"
fi

# Cross-compile to Windows target
cargo +nightly build --target x86_64-pc-windows-msvc --manifest-path "$repo_root/Cargo.toml"
bin="$repo_root/target/x86_64-pc-windows-msvc/debug/jcode.exe"

if [[ ! -f "$bin" ]]; then
  echo "Windows dev binary not found: $bin" >&2
  exit 1
fi

hash=""
if command -v git >/dev/null 2>&1; then
  if git -C "$repo_root" rev-parse --git-dir >/dev/null 2>&1; then
    hash="$(git -C "$repo_root" rev-parse --short HEAD 2>/dev/null || true)"
    if [[ -n "${hash}" ]] && [[ -n "$(git -C "$repo_root" status --porcelain 2>/dev/null || true)" ]]; then
      hash="${hash}-dirty"
    fi
  fi
fi

if [[ -z "$hash" ]]; then
  hash="$(date +%Y%m%d%H%M%S)"
fi

# Install versioned binary into ~/.jcode/builds/versions/<hash>/
builds_dir="$HOME/.jcode/builds"
version_dir="$builds_dir/versions/$hash"
mkdir -p "$version_dir"
cp "$bin" "$version_dir/jcode.exe"

# Update dev symlink (copy for Windows compatibility)
dev_dir="$builds_dir/dev"
mkdir -p "$dev_dir"
cp "$version_dir/jcode.exe" "$dev_dir/jcode.exe"

# Update dev-version marker
printf '%s\n' "$hash" > "$builds_dir/dev-version"

# Update launcher path to dev channel
install_dir="${JCODE_INSTALL_DIR:-$HOME/.local/bin}"
mkdir -p "$install_dir"
cp "$dev_dir/jcode.exe" "$install_dir/jcode.exe"

echo "Installed: $version_dir/jcode.exe"
echo "Updated dev binary: $dev_dir/jcode.exe -> $version_dir/jcode.exe"
echo "Updated launcher: $install_dir/jcode.exe -> $dev_dir/jcode.exe"

if ! echo "$PATH" | tr ':' '\n' | grep -qx "$install_dir"; then
  echo ""
  echo "Tip: add $install_dir to PATH if needed."
fi

echo ""
echo "Note: This builds a Windows .exe from Linux/WSL2."
echo "The binary can be copied to Windows or accessed via WSL2 mount points."
