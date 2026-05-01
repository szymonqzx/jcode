//! Windsurf authentication module
//!
//! This module handles credential discovery for Windsurf/Codeium integration.
//! It reads API keys from VSCode state database or ~/.codeium/config.json,
//! and discovers the local language server port from running Windsurf processes.
//!
//! Based on: https://github.com/rsvedant/opencode-windsurf-auth

use anyhow::{Context, Result};
use serde::Deserialize;
use std::path::PathBuf;

use rusqlite::Connection;

/// Windsurf credentials discovered from config and running process
#[derive(Debug, Clone)]
pub struct WindsurfCredentials {
    pub csrf_token: Option<String>,
    pub port: u16,
    pub api_key: Option<String>,
    pub version: String,
}

/// Get the VSCode state database path (platform-specific)
pub fn vscode_state_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Failed to determine home directory")?;

    #[cfg(target_os = "macos")]
    {
        Ok(home.join("Library/Application Support/Windsurf/User/globalStorage/state.vscdb"))
    }

    #[cfg(target_os = "linux")]
    {
        Ok(home.join(".config/Windsurf/User/globalStorage/state.vscdb"))
    }

    #[cfg(target_os = "windows")]
    {
        Ok(home.join("AppData/Roaming/Windsurf/User/globalStorage/state.vscdb"))
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        anyhow::bail!("Windsurf integration not supported on this platform");
    }
}

/// Get the legacy config file path (~/.codeium/config.json)
pub fn legacy_config_path() -> Result<PathBuf> {
    let home = dirs::home_dir().context("Failed to determine home directory")?;
    Ok(home.join(".codeium").join("config.json"))
}

/// Helper function to read a JSON value from the VSCode state database
/// Returns the JSON string value for the given key, or None if not found
fn read_state_db_value(state_path: &PathBuf, key: &str) -> Option<String> {
    if let Ok(conn) = Connection::open(state_path) {
        if let Ok(mut stmt) = conn.prepare("SELECT value FROM ItemTable WHERE key = ?1;") {
            if let Ok(mut rows) = stmt.query([key]) {
                if let Ok(Some(row)) = rows.next() {
                    if let Ok(value) = row.get::<_, String>(0) {
                        return Some(value);
                    }
                }
            }
        }
    }

    None
}

/// Get the language server process pattern for the current platform
fn language_server_pattern() -> &'static str {
    #[cfg(target_os = "macos")]
    {
        "language_server_macos"
    }

    #[cfg(target_os = "linux")]
    {
        "language_server_linux"
    }

    #[cfg(target_os = "windows")]
    {
        "language_server_windows"
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        "language_server"
    }
}

/// Get the language server process information
fn get_language_server_process() -> Result<String> {
    let pattern = language_server_pattern();

    #[cfg(target_os = "windows")]
    {
        let output = std::process::Command::new("wmic")
            .args(["process", "where", &format!("name like '%{}%'", pattern), "get", "CommandLine", "/format:list"])
            .output()
            .context("Failed to run wmic to find Windsurf language server")?;

        if !output.status.success() {
            anyhow::bail!("No Windsurf language server process found");
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }

    #[cfg(not(target_os = "windows"))]
    {
        let output = std::process::Command::new("sh")
            .args(["-c", &format!("ps aux | grep {}", pattern)])
            .output()
            .context("Failed to run ps to find Windsurf language server")?;

        if !output.status.success() {
            anyhow::bail!("No Windsurf language server process found");
        }

        Ok(String::from_utf8_lossy(&output.stdout).to_string())
    }
}

/// Extract CSRF token from running Windsurf language server process
pub fn get_csrf_token() -> Result<String> {
    let process_info = get_language_server_process()?;

    // Windsurf 1.9577+ uses --stdin_initial_metadata instead of --csrf_token
    // The CSRF token is passed via stdin during the initial handshake
    // For newer Windsurf versions, we need to read it from the VSCode state database
    // Try to extract --csrf_token from process arguments (older versions)
    let re = regex::Regex::new(r"--csrf_token\s+([a-f0-9-]+)")?;
    if let Some(captures) = re.captures(&process_info) {
        if let Some(token) = captures.get(1) {
            return Ok(token.as_str().to_string());
        }
    }

    // For newer Windsurf versions (1.9577+), read from VSCode state database
    let state_path = vscode_state_path()?;
    if state_path.exists() {
        if let Some(value) = read_state_db_value(&state_path, "windsurfAuthStatus") {
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&value) {
                if let Some(csrf) = parsed.get("csrfToken").and_then(|v| v.as_str()) {
                    return Ok(csrf.to_string());
                }
            }
        }
    }

    // Another fallback: try to read from ~/.codeium/config.json
    let legacy_path = legacy_config_path()?;
    if legacy_path.exists() {
        #[derive(Deserialize)]
        struct LegacyConfig {
            #[serde(rename = "csrfToken")]
            csrf_token: Option<String>,
        }

        if let Ok(content) = std::fs::read_to_string(&legacy_path) {
            if let Ok(config) = serde_json::from_str::<LegacyConfig>(&content) {
                if let Some(token) = config.csrf_token {
                    return Ok(token);
                }
            }
        }
    }

    anyhow::bail!(
        "CSRF token not found in Windsurf process or config. Tried: process arguments, VSCode state database ({}), and legacy config ({}). Is Windsurf running and logged in?",
        state_path.display(),
        legacy_path.display()
    );
}

/// Get the language server gRPC port from running process
pub fn get_port() -> Result<u16> {
    let process_info = get_language_server_process()?;

    // Extract PID from ps output (second column)
    let pid_re = regex::Regex::new(r"^\s*\S+\s+(\d+)")?;
    let _pid = pid_re.captures(&process_info)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse::<u32>().ok());

    // Extract extension_server_port as a reference point
    let re = regex::Regex::new(r"--extension_server_port\s+(\d+)")?;
    let ext_port = re.captures(&process_info)
        .and_then(|c| c.get(1))
        .and_then(|m| m.as_str().parse::<u16>().ok());

    // Use lsof to find actual listening ports for this specific PID (Unix only)
    #[cfg(not(target_os = "windows"))]
    {
        let pid = pid_re.captures(&process_info)
            .and_then(|c| c.get(1))
            .and_then(|m| m.as_str().parse::<u32>().ok());

        if let Some(pid) = pid {
            if let Ok(lsof_output) = std::process::Command::new("lsof")
                .args(["-p", &pid.to_string(), "-i", "-P", "-n"])
                .output()
            {
                let stdout = String::from_utf8_lossy(&lsof_output.stdout);
                // Extract all listening ports
                let port_re = regex::Regex::new(r":(\d+)\s+\(LISTEN\)")?;
                let mut ports: Vec<u16> = port_re
                    .captures_iter(&stdout)
                    .filter_map(|c| c.get(1))
                    .filter_map(|m| m.as_str().parse::<u16>().ok())
                    .collect();

                if !ports.is_empty() {
                    // If we have extension_server_port, prefer the port closest to it (usually +3)
                    if let Some(ext) = ext_port {
                        ports.sort();
                        let candidate_ports: Vec<u16> = ports.iter().filter(|p| **p > ext).cloned().collect();
                        if !candidate_ports.is_empty() {
                            return Ok(candidate_ports[0]);
                        }
                    }
                    // Otherwise return the first listening port
                    return Ok(ports[0]);
                }
            }
        }
    }

    // Fallback: try common offsets (+3, +2, +4)
    if let Some(ext) = ext_port {
        crate::logging::warn(&format!(
            "Windsurf port detection using fallback offset (extension_server_port {} + 3 = {}). This may not be correct.",
            ext,
            ext + 3
        ));
        return Ok(ext + 3);
    }

    // Fallback to default port
    crate::logging::warn("Windsurf port detection using default port 42100. This may not be correct.");
    Ok(42100)
}

/// Read API key from VSCode state database (windsurfAuthStatus)
/// Falls back to legacy config file if database read fails
pub fn get_api_key() -> Result<String> {
    // Try VSCode state database first
    let state_path = vscode_state_path()?;
    if state_path.exists() {
        if let Some(value) = read_state_db_value(&state_path, "windsurfAuthStatus") {
            if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&value) {
                if let Some(api_key) = parsed.get("apiKey").and_then(|v| v.as_str()) {
                    return Ok(api_key.to_string());
                }
            }
        }
    }

    // Fallback to legacy config file
    let legacy_path = legacy_config_path()?;
    if legacy_path.exists() {
        let content = std::fs::read_to_string(&legacy_path)
            .context("Failed to read legacy Windsurf config")?;

        #[derive(Deserialize)]
        struct LegacyConfig {
            #[serde(rename = "apiKey")]
            api_key: Option<String>,
        }

        let config: LegacyConfig = serde_json::from_str(&content)
            .context("Failed to parse legacy Windsurf config")?;

        if let Some(api_key) = config.api_key {
            return Ok(api_key);
        }
    }

    anyhow::bail!("API key not found. Please login to Windsurf first.");
}

/// Get Windsurf version from process arguments
pub fn get_windsurf_version() -> Result<String> {
    let process_info = get_language_server_process()?;

    let re = regex::Regex::new(r"--windsurf_version\s+(\S+)")?;
    if let Some(captures) = re.captures(&process_info) {
        if let Some(version) = captures.get(1) {
            // Extract version number before + if present
            let version_str = version.as_str().split('+').next().unwrap_or(version.as_str());
            return Ok(version_str.to_string());
        }
    }

    // Default fallback version
    Ok("1.13.104".to_string())
}

/// Load all Windsurf credentials
///
/// CSRF token and API key are optional - some Windsurf versions or configurations
/// may not require them. Port and version are required for the provider to function.
pub fn load_credentials() -> Result<WindsurfCredentials> {
    Ok(WindsurfCredentials {
        csrf_token: get_csrf_token().ok(),
        port: get_port()?,
        api_key: get_api_key().ok(),
        version: get_windsurf_version()?,
    })
}

/// Check if Windsurf is running and accessible
pub fn is_windsurf_running() -> bool {
    get_port().is_ok()
}

/// Check if Windsurf is installed
pub fn is_windsurf_installed() -> bool {
    #[cfg(target_os = "macos")]
    {
        PathBuf::from("/Applications/Windsurf.app").exists()
    }

    #[cfg(target_os = "linux")]
    {
        PathBuf::from("/usr/share/windsurf").exists() ||
        dirs::home_dir().map(|h| h.join(".local/share/windsurf")).map(|p| p.exists()).unwrap_or(false)
    }

    #[cfg(target_os = "windows")]
    {
        PathBuf::from("C:\\Program Files\\Windsurf").exists() ||
        dirs::home_dir().map(|h| h.join("AppData\\Local\\Programs\\Windsurf")).map(|p| p.exists()).unwrap_or(false)
    }

    #[cfg(not(any(target_os = "macos", target_os = "windows", target_os = "linux")))]
    {
        false
    }
}

/// Check if Windsurf is configured and running
pub fn is_available() -> bool {
    is_windsurf_installed() && is_windsurf_running()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_credential_discovery() {
        println!("Testing Windsurf credential discovery...");

        match get_csrf_token() {
            Ok(token) => println!("CSRF token found: {}...", &token[..std::cmp::min(8, token.len())]),
            Err(e) => println!("CSRF token error: {}", e),
        }

        match get_port() {
            Ok(port) => println!("Port found: {}", port),
            Err(e) => println!("Port error: {}", e),
        }

        match get_api_key() {
            Ok(key) => println!("API key found: {}...", &key[..std::cmp::min(8, key.len())]),
            Err(e) => println!("API key error: {}", e),
        }

        match get_windsurf_version() {
            Ok(version) => println!("Version: {}", version),
            Err(e) => println!("Version error: {}", e),
        }

        match load_credentials() {
            Ok(creds) => {
                println!("Credentials loaded successfully!");
                if let Some(ref csrf) = creds.csrf_token {
                    println!("  CSRF: {}...", &csrf[..std::cmp::min(8, csrf.len())]);
                } else {
                    println!("  CSRF: (not found)");
                }
                println!("  Port: {}", creds.port);
                if let Some(ref key) = creds.api_key {
                    println!("  API Key: {}...", &key[..std::cmp::min(8, key.len())]);
                } else {
                    println!("  API Key: (not found)");
                }
                println!("  Version: {}", creds.version);
            }
            Err(e) => println!("Failed to load credentials: {}", e),
        }
    }
}
