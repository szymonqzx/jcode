#!/usr/bin/env bash
# Setup script for WSL2 development environment
# Installs Rust, toolchains, and build dependencies for jcode

set -euo pipefail

info() { printf '\033[1;34m%s\033[0m\n' "$*"; }
success() { printf '\033[1;32m%s\033[0m\n' "$*"; }
warn() { printf '\033[1;33m%s\033[0m\n' "$*"; }
err() { printf '\033[1;31merror: %s\033[0m\n' "$*" >&2; exit 1; }

echo "=== jcode WSL2 Development Environment Setup ==="
echo ""

# Check if running in WSL2
if [[ ! -f /proc/version ]] || ! grep -qi "microsoft" /proc/version; then
    warn "This script is designed for WSL2. It may work on other Linux systems, but is not tested."
    read -p "Continue anyway? (y/N) " -n 1 -r
    echo
    if [[ ! $REPLY =~ ^[Yy]$ ]]; then
        exit 0
    fi
fi

# Install Rust if not present
if ! command -v rustup >/dev/null 2>&1; then
    info "Installing Rust via rustup..."
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y --default-toolchain stable --profile default
    source "$HOME/.cargo/env"
    success "Rust installed successfully"
else
    info "Rust already installed"
fi

# Install nightly toolchain (required for dev builds with Cranelift)
if ! rustup toolchain list | grep -q "nightly"; then
    info "Installing nightly toolchain..."
    rustup toolchain install nightly
    rustup component add rust-src --toolchain nightly
    success "Nightly toolchain installed"
else
    info "Nightly toolchain already installed"
fi

# Install lld linker (for faster linking)
if command -v apt-get >/dev/null 2>&1; then
    if ! command -v lld >/dev/null 2>&1; then
        info "Installing lld linker..."
        sudo apt-get update
        sudo apt-get install -y lld
        success "lld linker installed"
    else
        info "lld linker already installed"
    fi
elif command -v dnf >/dev/null 2>&1; then
    if ! command -v lld >/dev/null 2>&1; then
        info "Installing lld linker..."
        sudo dnf install -y lld
        success "lld linker installed"
    else
        info "lld linker already installed"
    fi
else
    warn "Package manager not detected. Please install lld manually for faster linking."
fi

# Install sccache (optional, for compilation caching)
if ! command -v sccache >/dev/null 2>&1; then
    info "Installing sccache for compilation caching..."
    cargo install sccache
    success "sccache installed"
else
    info "sccache already installed"
fi

# Install common build dependencies
if command -v apt-get >/dev/null 2>&1; then
    info "Installing common build dependencies..."
    sudo apt-get update
    sudo apt-get install -y build-essential pkg-config libssl-dev
    success "Build dependencies installed"
elif command -v dnf >/dev/null 2>&1; then
    info "Installing common build dependencies..."
    sudo dnf install -y gcc pkg-config openssl-devel
    success "Build dependencies installed"
fi

# Configure cargo to use sccache if available
if command -v sccache >/dev/null 2>&1; then
    info "Configuring cargo to use sccache..."
    mkdir -p "$HOME/.cargo"
    if ! grep -q "RUSTC_WRAPPER" "$HOME/.cargo/env" 2>/dev/null; then
        echo 'export RUSTC_WRAPPER=sccache' >> "$HOME/.cargo/env"
        success "sccache configured in cargo environment"
    fi
fi

echo ""
success "=== WSL2 Development Environment Setup Complete ==="
echo ""
info "Next steps:"
echo "  1. Source your cargo environment: source \$HOME/.cargo/env"
echo "  2. Navigate to the jcode directory"
echo "  3. Run: ./scripts/install_dev.sh"
echo ""
