# jcode

[![Latest Release](https://img.shields.io/github/v/release/szymonqzx/jcode?style=flat-square)](https://github.com/szymonqzx/jcode/releases)
[![License](https://img.shields.io/github/license/szymonqzx/jcode?style=flat-square)](LICENSE)
[![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20macOS%20%7C%20Windows-blue?style=flat-square)](https://github.com/szymonqzx/jcode/releases)
[![Commit Activity](https://img.shields.io/github/commit-activity/m/szymonqzx/jcode?style=flat-square)](https://github.com/szymonqzx/jcode/commits/master)
[![GitHub Stars](https://img.shields.io/github/stars/szymonqzx/jcode?style=flat-square)](https://github.com/szymonqzx/jcode/stargazers)

The next generation coding agent harness to raise the skill ceiling. Built for multi-session workflows, infinite customizability, and performance.

> **Note:** This fork is primarily developed and tested on Windows. Linux compatibility is not actively tested but should work due to the cross-platform Rust foundation.

---

## Installation

```bash
# macOS & Linux
curl -fsSL https://raw.githubusercontent.com/szymonqzx/jcode/master/scripts/install.sh | bash

# Windows (PowerShell)
irm https://raw.githubusercontent.com/szymonqzx/jcode/master/scripts/install.ps1 | iex

# macOS via Homebrew
brew tap szymonqzx/jcode
brew install jcode
```

For detailed installation options, see [Detailed Installation](#detailed-installation).

---

## Performance

jcode is optimized for performance and resource efficiency, especially important for multi-session workflows:

- **RAM usage**: ~27.8 MB baseline (local embedding off), ~167 MB with embeddings
- **Startup time**: ~14 ms to first frame, ~49 ms to first input
- **Memory scaling**: ~10 MB per additional session
- Significantly more efficient than alternatives like Claude Code, Cursor Agent, and GitHub Copilot CLI

---

## Memory (Agent memory)

jcode embeds each turn/response as a semantic vector and queries a memory graph to efficiently find related entries via cosine similarity. This creates a human-like memory system that automatically recalls relevant information without explicit memory tool calls. Memories are extracted periodically and consolidated automatically via ambient mode.

---

## UI Features

- **Side panels** for auxiliary information with real-time updates and diff viewing
- **Mermaid diagrams** rendered inline (1800x faster rendering via custom library)
- **Info widgets** that use negative space without displacing content
- **1000+ fps rendering** for flicker-free experience
- **Custom scrollback** with advanced navigation
- **Alignment switching** between left and center modes (Alt+C hotkey)

**TUI enhancements in this fork:**
- Raised stream stall timeout from 2 to 5 minutes
- Added `/help-cmds` slash command for quick reference
- Enhanced TUI helpers, state UI, and launch system

---

## Swarm

Spawn multiple agents in the same repo for native collaboration. The server automatically manages conflicts when agents edit files others have read. Agents can message each other (DM, broadcast, or repo-specific) and can autonomously spawn their own swarms for parallel task execution.

---

## OAuth and Providers

jcode supports subscription-backed OAuth flows and many provider integrations. Use models you already pay for or fall back to direct API providers.

**Built-in login flows:** Claude, OpenAI, Gemini, GitHub Copilot, Azure OpenAI, Alibaba Cloud Coding Plan, Fireworks, MiniMax, LM Studio, Ollama, custom OpenAI-compatible endpoints

**New providers in this fork:**
- OpenCode Go (344 lines) - included in the original, though for some reason did not work for me.
- Windsurf (330 lines provider + 398 lines auth)

**Other providers:** OpenRouter, OpenCode, zai/kimi, 302ai, baseten, cortecs, deepseek, firmware, huggingface, moonshotai, nebius, scaleway, stackit, groq, mistral, perplexity, togetherai, deepinfra, fireworks, minimax, xai, lmstudio, ollama, chutes, cerebras, cursor, antigravity, google

Supports multi-account switching, headless/SSH sessions (`--no-browser`), scriptable auth flows, and MCP server configuration.

---

## Self-Dev Mode

Tell your jcode agent to enter self-dev mode to modify its own source code. jcode is optimized to iterate on itself with infrastructure for editing, building, testing, and reloading the binary across sessions. Requires a frontier model (GPT 5.5 or latest) for reliable results.

---

## Additional Features

**Infrastructure improvements in this fork:**
- Enhanced platform detection (160 lines)
- Stdin detection module (301 lines)
- Improved AnthropicProvider and Copilot error handling
- Comprehensive auth, platform, and provider tests
- Clippy compliance fixes

**Other features:**
- Claude cache cold warnings to avoid token waste
- Firefox Agent Bridge setup for browser automation
- Agent grep with file structure information
- Interleaved input sending (shift+enter for queue mode)
- Session resume from other harnesses (Claude Code, Codex, OpenCode, pi)
- Semantic skill loading with automatic injection

---

## Quick Start

```bash
# Launch the TUI
jcode

# Run a single command non-interactively
jcode run "say hello"

# Resume a previous session
jcode --resume fox

# Run as a persistent server, then attach clients
jcode serve
jcode connect

# Voice input
jcode dictate
```

Supports interactive TUI, non-interactive runs, server/client workflows, and dictation.

---

## Browser Automation

Built-in `browser` tool for Firefox via Firefox Agent Bridge. Actions include: status, setup, open, snapshot, get_content, interactables, click, type, fill_form, select, wait, screenshot, eval, scroll, upload, press.

```bash
jcode browser status
jcode browser setup
```

---

## Further Reading

- [OpenAI Apps SDK, MCP, Apps UI, and Assistant Review](docs/OPENAI_APPS_MCP_ASSISTANT_REVIEW.md) (new)
- [Ambient Mode / OpenClaw](docs/AMBIENT_MODE.md)
- [Browser Provider Protocol](docs/BROWSER_PROVIDER_PROTOCOL.md)
- [Memory Architecture](docs/MEMORY_ARCHITECTURE.md)
- [Swarm Architecture](docs/SWARM_ARCHITECTURE.md)
- [Server Architecture](docs/SERVER_ARCHITECTURE.md)
- [Windows Notes](docs/WINDOWS.md) (updated)
- [CLAUDE.md](CLAUDE.md) (new) - Repo guide for Claude Code sessions

---

## Detailed Installation

### Quick Install

```bash
# macOS & Linux
curl -fsSL https://raw.githubusercontent.com/szymonqzx/jcode/master/scripts/install.sh | bash

# Windows (PowerShell)
irm https://raw.githubusercontent.com/szymonqzx/jcode/master/scripts/install.ps1 | iex

# macOS via Homebrew
brew tap szymonqzx/jcode
brew install jcode
```

### From Source

```bash
git clone https://github.com/szymonqzx/jcode.git
cd jcode
cargo build --release
scripts/install_release.sh
```

### Platform Support

| Platform | Status |
|---|---|
| **Windows** x86_64 | **Primary development platform, fully supported** |
| **Linux** x86_64 / aarch64 | Should work (untested in this fork) |
| **macOS** Apple Silicon & Intel | Should work (untested in this fork) |

**Windows-specific improvements in this fork:**
- Console window suppression across bash tool spawns and server-side processes
- PowerShell install script with automatic executable cleanup
- Fixed auto-spawn timeout and detached server handoff
- Enhanced Windows path handling and JSON escaping
- Stabilized Windows test suite
- Named pipe support via interior mutability
