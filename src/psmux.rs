//! psmux integration for Windows terminal multiplexing
//!
//! This module provides optional integration with psmux (tmux for Windows),
//! allowing teammate agents to spawn in separate tmux panes when running
//! inside a psmux session.

use anyhow::Result;

/// Check if we're currently running inside a psmux session
#[cfg(feature = "psmux")]
pub fn is_inside_psmux_session() -> bool {
    std::env::var("PSMUX").is_ok()
}

/// Check if psmux is available on the system
#[cfg(feature = "psmux")]
pub fn psmux_available() -> bool {
    std::process::Command::new("psmux")
        .arg("--version")
        .output()
        .is_ok()
}

/// Spawn a command in a new psmux pane if inside a psmux session
#[cfg(feature = "psmux")]
pub fn spawn_in_psmux_pane(command: &str, args: &[&str]) -> Result<bool> {
    if !is_inside_psmux_session() {
        return Ok(false);
    }

    // Build the psmux command to spawn in a new pane
    // Command and args must be passed as a single string to split-window
    let full_command = if args.is_empty() {
        command.to_string()
    } else {
        format!("{} {}", command, args.join(" "))
    };

    let mut cmd = std::process::Command::new("psmux");
    cmd.arg("split-window");
    cmd.arg(&full_command);

    match cmd.status() {
        Ok(status) if status.success() => Ok(true),
        Ok(status) => {
            eprintln!("psmux split-window failed with exit code: {:?}", status.code());
            Ok(false)
        }
        Err(e) => {
            eprintln!("Failed to spawn in psmux pane: {}", e);
            Err(e.into())
        }
    }
}

/// Get the current psmux session name if available
#[cfg(feature = "psmux")]
pub fn get_psmux_session_name() -> Option<String> {
    if !is_inside_psmux_session() {
        return None;
    }

    std::env::var("PSMUX_SESSION").ok()
}

// Stub implementations when psmux feature is not enabled
#[cfg(not(feature = "psmux"))]
pub fn is_inside_psmux_session() -> bool {
    false
}

#[cfg(not(feature = "psmux"))]
pub fn psmux_available() -> bool {
    false
}

#[cfg(not(feature = "psmux"))]
pub fn spawn_in_psmux_pane(_command: &str, _args: &[&str]) -> Result<bool> {
    Ok(false)
}

#[cfg(not(feature = "psmux"))]
pub fn get_psmux_session_name() -> Option<String> {
    None
}
