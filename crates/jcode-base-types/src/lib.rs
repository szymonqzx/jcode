//! JCode Base Types
//!
//! Consolidated foundational types for the jcode system.
//! This crate contains types that only depend on serde.

// =============================================================================
// Message Types (from jcode-message-types)
// =============================================================================

#[derive(Debug, Clone, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct ToolCall {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub input: serde_json::Value,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub intent: Option<String>,
}

impl ToolCall {
    pub fn intent_from_input(input: &serde_json::Value) -> Option<String> {
        input
            .get("intent")
            .and_then(|value| value.as_str())
            .map(str::trim)
            .filter(|intent| !intent.is_empty())
            .map(ToString::to_string)
    }

    pub fn refresh_intent_from_input(&mut self) {
        self.intent = Self::intent_from_input(&self.input);
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct InputShellResult {
    pub command: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub cwd: Option<String>,
    pub output: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub exit_code: Option<i32>,
    pub duration_ms: u64,
    #[serde(default)]
    pub truncated: bool,
    #[serde(default)]
    pub failed_to_start: bool,
}

// =============================================================================
// Auth Types (from jcode-auth-types)
// =============================================================================

/// State of a single auth credential
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum AuthState {
    /// Credential is available and valid
    Available,
    /// Partial configuration exists (or OAuth may be expired)
    Expired,
    /// Credential is not configured
    #[default]
    NotConfigured,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthCredentialSource {
    #[default]
    None,
    EnvironmentVariable,
    AppConfigFile,
    JcodeManagedFile,
    TrustedExternalFile,
    TrustedExternalAppState,
    LocalCliSession,
    AzureDefaultCredential,
    Mixed,
}

impl AuthCredentialSource {
    pub fn label(self) -> &'static str {
        match self {
            Self::None => "none",
            Self::EnvironmentVariable => "environment variable",
            Self::AppConfigFile => "app config file",
            Self::JcodeManagedFile => "jcode-managed file",
            Self::TrustedExternalFile => "trusted external file",
            Self::TrustedExternalAppState => "trusted external app state",
            Self::LocalCliSession => "local CLI session",
            Self::AzureDefaultCredential => "Azure DefaultAzureCredential",
            Self::Mixed => "mixed",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthExpiryConfidence {
    #[default]
    Unknown,
    Exact,
    PresenceOnly,
    ConfigurationOnly,
    NotApplicable,
}

impl AuthExpiryConfidence {
    pub fn label(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Exact => "exact timestamp",
            Self::PresenceOnly => "presence only",
            Self::ConfigurationOnly => "configuration only",
            Self::NotApplicable => "not applicable",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthRefreshSupport {
    #[default]
    Unknown,
    Automatic,
    Conditional,
    ManualRelogin,
    ExternalManaged,
    NotApplicable,
}

impl AuthRefreshSupport {
    pub fn label(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::Automatic => "automatic",
            Self::Conditional => "conditional",
            Self::ManualRelogin => "manual re-login",
            Self::ExternalManaged => "external/manual",
            Self::NotApplicable => "not applicable",
        }
    }
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AuthValidationMethod {
    #[default]
    Unknown,
    PresenceCheck,
    TimestampCheck,
    ConfigurationCheck,
    TrustedImportScan,
    CommandProbe,
    CompositeProbe,
}

impl AuthValidationMethod {
    pub fn label(self) -> &'static str {
        match self {
            Self::Unknown => "unknown",
            Self::PresenceCheck => "presence check",
            Self::TimestampCheck => "timestamp check",
            Self::ConfigurationCheck => "configuration check",
            Self::TrustedImportScan => "trusted import scan",
            Self::CommandProbe => "command probe",
            Self::CompositeProbe => "composite probe",
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct ProviderValidationRecord {
    pub checked_at_ms: i64,
    pub success: bool,
    pub provider_smoke_ok: Option<bool>,
    pub tool_smoke_ok: Option<bool>,
    pub summary: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub struct ProviderRefreshRecord {
    pub last_attempt_ms: i64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_success_ms: Option<i64>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub last_error: Option<String>,
}

// =============================================================================
// Config Types (from jcode-config-types) - partial, includes common types
// =============================================================================

/// Compaction mode
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Default)]
#[serde(rename_all = "lowercase")]
pub enum CompactionMode {
    /// Compact when context hits a fixed threshold (default)
    #[default]
    Reactive,
    /// Compact early based on predicted token growth rate
    Proactive,
    /// Compact based on semantic topic shifts and relevance scoring
    Semantic,
}

impl CompactionMode {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Reactive => "reactive",
            Self::Proactive => "proactive",
            Self::Semantic => "semantic",
        }
    }

    pub fn parse(input: &str) -> Option<Self> {
        match input.trim().to_ascii_lowercase().as_str() {
            "reactive" => Some(Self::Reactive),
            "proactive" => Some(Self::Proactive),
            "semantic" => Some(Self::Semantic),
            _ => None,
        }
    }
}

/// Session picker Enter action: "new-terminal" (default) or "current-terminal".
/// Ctrl+Enter performs the alternate action.
#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "kebab-case")]
pub enum SessionPickerResumeAction {
    #[default]
    NewTerminal,
    CurrentTerminal,
}

impl SessionPickerResumeAction {
    pub fn alternate(self) -> Self {
        match self {
            Self::NewTerminal => Self::CurrentTerminal,
            Self::CurrentTerminal => Self::NewTerminal,
        }
    }
}

/// How to display file diffs from edit/write tools.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum DiffDisplayMode {
    /// Don't show diffs at all.
    Off,
    /// Show diffs inline in the chat (default).
    #[default]
    Inline,
    /// Show the full inline diff in the chat without preview truncation.
    #[serde(
        rename = "full-inline",
        alias = "full_inline",
        alias = "fullinline",
        alias = "inline-full",
        alias = "inline_full",
        alias = "inlinefull",
        alias = "full"
    )]
    FullInline,
    /// Show diffs in a dedicated pinned pane.
    Pinned,
    /// Show full file with diff highlights in side panel, synced to scroll position.
    File,
}

impl DiffDisplayMode {
    pub fn is_inline(&self) -> bool {
        matches!(self, Self::Inline | Self::FullInline)
    }

    pub fn is_full_inline(&self) -> bool {
        matches!(self, Self::FullInline)
    }

    pub fn is_pinned(&self) -> bool {
        matches!(self, Self::Pinned)
    }

    pub fn is_file(&self) -> bool {
        matches!(self, Self::File)
    }

    pub fn has_side_pane(&self) -> bool {
        matches!(self, Self::Pinned | Self::File)
    }
}

// =============================================================================
// Gateway Types (from jcode-gateway-types)
// =============================================================================

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PairedDevice {
    pub id: String,
    pub name: String,
    pub token_hash: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub apns_token: Option<String>,
    pub paired_at: String,
    pub last_seen: String,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PairingCode {
    pub code: String,
    pub created_at: String,
    pub expires_at: String,
}

// =============================================================================
// Background Types (from jcode-background-types)
// =============================================================================

/// Status of a background task.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum BackgroundTaskStatus {
    Running,
    Completed,
    Superseded,
    Failed,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BackgroundTaskProgressKind {
    Determinate,
    Indeterminate,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum BackgroundTaskProgressSource {
    Reported,
    ParsedOutput,
    Heuristic,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct BackgroundTaskProgress {
    pub kind: BackgroundTaskProgressKind,
    pub percent: Option<f32>,
    pub message: Option<String>,
    pub current: Option<u64>,
    pub total: Option<u64>,
    pub unit: Option<String>,
    pub eta_seconds: Option<u64>,
    pub updated_at: String,
    pub source: BackgroundTaskProgressSource,
}

impl BackgroundTaskProgress {
    pub fn normalize(mut self) -> Self {
        if let (Some(current), Some(total)) = (self.current, self.total)
            && total > 0
            && self.percent.is_none()
        {
            let computed = (current as f64 / total as f64) * 100.0;
            self.percent = Some(((computed * 100.0).round() / 100.0) as f32);
        }

        self.percent = self
            .percent
            .map(|percent| ((percent.clamp(0.0, 100.0) * 100.0).round()) / 100.0);

        if matches!(self.kind, BackgroundTaskProgressKind::Indeterminate)
            && (self.percent.is_some()
                || matches!((self.current, self.total), (_, Some(total)) if total > 0))
        {
            self.kind = BackgroundTaskProgressKind::Determinate;
        }

        self
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq)]
pub struct BackgroundTaskProgressEvent {
    pub task_id: String,
    pub tool_name: String,
    pub display_name: Option<String>,
    pub session_id: String,
    pub progress: BackgroundTaskProgress,
}

// =============================================================================
// Side Panel Types (from jcode-side-panel-types)
// =============================================================================

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum SidePanelPageFormat {
    #[default]
    Markdown,
}

impl SidePanelPageFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Markdown => "markdown",
        }
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum SidePanelPageSource {
    #[default]
    Managed,
    LinkedFile,
    Ephemeral,
}

impl SidePanelPageSource {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Managed => "managed",
            Self::LinkedFile => "linked_file",
            Self::Ephemeral => "ephemeral",
        }
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct PersistedSidePanelState {
    #[serde(default)]
    pub focused_page_id: Option<String>,
    #[serde(default)]
    pub pages: Vec<PersistedSidePanelPage>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct PersistedSidePanelPage {
    pub id: String,
    pub title: String,
    pub file_path: String,
    #[serde(default)]
    pub format: SidePanelPageFormat,
    #[serde(default)]
    pub source: SidePanelPageSource,
    pub updated_at_ms: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Default)]
pub struct SidePanelPage {
    pub id: String,
    pub title: String,
    pub file_path: String,
    #[serde(default)]
    pub format: SidePanelPageFormat,
    #[serde(default)]
    pub source: SidePanelPageSource,
    #[serde(default)]
    pub content: String,
    #[serde(default)]
    pub updated_at_ms: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, PartialEq, Eq, Default)]
pub struct SidePanelSnapshot {
    #[serde(default)]
    pub focused_page_id: Option<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub pages: Vec<SidePanelPage>,
}

impl SidePanelSnapshot {
    pub fn has_pages(&self) -> bool {
        !self.pages.is_empty()
    }

    pub fn focused_page(&self) -> Option<&SidePanelPage> {
        let focused_id = self.focused_page_id.as_deref()?;
        self.pages.iter().find(|page| page.id == focused_id)
    }
}

pub fn snapshot_is_empty(snapshot: &SidePanelSnapshot) -> bool {
    !snapshot.has_pages()
}

// =============================================================================
// Usage Types (from jcode-usage-types)
// =============================================================================

#[derive(Debug, Clone, Default)]
pub struct ProviderUsage {
    pub provider_name: String,
    pub limits: Vec<UsageLimit>,
    pub extra_info: Vec<(String, String)>,
    pub hard_limit_reached: bool,
    pub error: Option<String>,
}

#[derive(Debug, Clone)]
pub struct UsageLimit {
    pub name: String,
    pub usage_percent: f32,
    pub resets_at: Option<String>,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct CopilotUsageTracker {
    pub today: DayUsage,
    pub month: MonthUsage,
    pub all_time: AllTimeUsage,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct DayUsage {
    pub date: String,
    pub requests: u64,
    pub premium_requests: u64,
    pub input_tokens: u64,
    pub output_tokens: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct MonthUsage {
    pub month: String,
    pub requests: u64,
    pub premium_requests: u64,
    pub input_tokens: u64,
    pub output_tokens: u64,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize, Default)]
pub struct AllTimeUsage {
    pub requests: u64,
    pub premium_requests: u64,
    pub input_tokens: u64,
    pub output_tokens: u64,
}
