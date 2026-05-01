#!/usr/bin/env bash
# Install the current dev binary into the version store,
# update the dev channel symlink, and point the launcher at dev.
#
# Paths after install:
# - ~/.jcode/builds/versions/<hash>/jcode (immutable)
# - ~/.jcode/builds/dev/jcode -> .../versions/<hash>/jcode
# - ~/.local/bin/jcode -> ~/.jcode/builds/dev/jcode (launcher)
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

echo "Building with nightly toolchain and Cranelift (optimized for speed)..."
echo "This uses: -Zthreads=8, -Zshare-generics, Cranelift backend"
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

cargo +nightly build --manifest-path "$repo_root/Cargo.toml"
bin="$repo_root/target/debug/jcode"

if [[ ! -x "$bin" ]]; then
  echo "Dev binary not found: $bin" >&2
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
install -m 755 "$bin" "$version_dir/jcode"

# Update dev symlink
dev_dir="$builds_dir/dev"
mkdir -p "$dev_dir"
ln -sfn "$version_dir/jcode" "$dev_dir/jcode"

# Update dev-version marker
printf '%s\n' "$hash" > "$builds_dir/dev-version"

# Update launcher path to dev channel
install_dir="${JCODE_INSTALL_DIR:-$HOME/.local/bin}"
mkdir -p "$install_dir"
ln -sfn "$dev_dir/jcode" "$install_dir/jcode"

echo "Installed: $version_dir/jcode"
echo "Updated dev symlink: $dev_dir/jcode -> $version_dir/jcode"
echo "Updated launcher symlink: $install_dir/jcode -> $dev_dir/jcode"

if ! echo "$PATH" | tr ':' '\n' | grep -qx "$install_dir"; then
  echo ""
  echo "Tip: add $install_dir to PATH if needed."
fi
