//! JCode Workflow Types
//!
//! Consolidated workflow/task/background types for the jcode system.
//! This crate contains types for tasks, ambient mode, and batch processing.

use jcode_base_types::ToolCall;
use serde::{Deserialize, Serialize};

// =============================================================================
// Task Types (from jcode-task-types)
// =============================================================================

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum GoalScope {
    Global,
    #[default]
    Project,
}

impl GoalScope {
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "global" => Some(Self::Global),
            "project" => Some(Self::Project),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Global => "global",
            Self::Project => "project",
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Default)]
#[serde(rename_all = "snake_case")]
pub enum GoalStatus {
    Draft,
    #[default]
    Active,
    Paused,
    Blocked,
    Completed,
    Archived,
    Abandoned,
}

impl GoalStatus {
    pub fn parse(value: &str) -> Option<Self> {
        match value.trim().to_ascii_lowercase().as_str() {
            "draft" => Some(Self::Draft),
            "active" => Some(Self::Active),
            "paused" => Some(Self::Paused),
            "blocked" => Some(Self::Blocked),
            "completed" => Some(Self::Completed),
            "archived" => Some(Self::Archived),
            "abandoned" => Some(Self::Abandoned),
            _ => None,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Draft => "draft",
            Self::Active => "active",
            Self::Paused => "paused",
            Self::Blocked => "blocked",
            Self::Completed => "completed",
            Self::Archived => "archived",
            Self::Abandoned => "abandoned",
        }
    }

    pub fn sort_rank(self) -> u8 {
        match self {
            Self::Active => 0,
            Self::Blocked => 1,
            Self::Draft => 2,
            Self::Paused => 3,
            Self::Completed => 4,
            Self::Archived => 5,
            Self::Abandoned => 6,
        }
    }

    pub fn is_resumable(self) -> bool {
        matches!(self, Self::Active | Self::Blocked | Self::Draft)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct GoalStep {
    pub id: String,
    pub content: String,
    #[serde(default = "default_pending_status")]
    pub status: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub struct GoalMilestone {
    pub id: String,
    pub title: String,
    #[serde(default = "default_pending_status")]
    pub status: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub steps: Vec<GoalStep>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct GoalUpdate {
    pub at: chrono::DateTime<chrono::Utc>,
    pub summary: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Goal {
    pub id: String,
    pub title: String,
    #[serde(default)]
    pub scope: GoalScope,
    #[serde(default)]
    pub status: GoalStatus,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub why: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub success_criteria: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub milestones: Vec<GoalMilestone>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub next_steps: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blockers: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub current_milestone_id: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub progress_percent: Option<u8>,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub updates: Vec<GoalUpdate>,
}

impl Goal {
    pub fn new(title: &str, scope: GoalScope) -> Self {
        let now = chrono::Utc::now();
        let trimmed = title.trim();
        Self {
            id: sanitize_goal_id(trimmed),
            title: trimmed.to_string(),
            scope,
            status: GoalStatus::Active,
            description: String::new(),
            why: String::new(),
            success_criteria: Vec::new(),
            milestones: Vec::new(),
            next_steps: Vec::new(),
            blockers: Vec::new(),
            current_milestone_id: None,
            progress_percent: None,
            created_at: now,
            updated_at: now,
            updates: Vec::new(),
        }
    }

    pub fn current_milestone(&self) -> Option<&GoalMilestone> {
        let current_id = self.current_milestone_id.as_deref()?;
        self.milestones.iter().find(|m| m.id == current_id)
    }
}

pub fn sanitize_goal_id(id: &str) -> String {
    let slug = slugify(id);
    if slug.is_empty() {
        "goal".to_string()
    } else {
        slug
    }
}

fn slugify(input: &str) -> String {
    let mut slug = String::new();
    let mut prev_dash = false;
    for ch in input.chars() {
        let lower = ch.to_ascii_lowercase();
        if lower.is_ascii_alphanumeric() {
            slug.push(lower);
            prev_dash = false;
        } else if !prev_dash {
            slug.push('-');
            prev_dash = true;
        }
    }
    slug.trim_matches('-').to_string()
}

fn default_pending_status() -> String {
    "pending".to_string()
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TodoItem {
    pub content: String,
    pub status: String,
    pub priority: String,
    pub id: String,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub blocked_by: Vec<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub assigned_to: Option<String>,
}

use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct PersistedCatchupState {
    #[serde(default)]
    pub seen_at_ms_by_session: HashMap<String, i64>,
}

#[derive(Debug, Clone)]
pub struct CatchupBrief {
    pub reason: String,
    pub tags: Vec<String>,
    pub last_user_prompt: Option<String>,
    pub activity_steps: Vec<String>,
    pub files_touched: Vec<String>,
    pub tool_counts: Vec<(String, usize)>,
    pub validation_notes: Vec<String>,
    pub latest_agent_response: Option<String>,
    pub needs_from_user: String,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}

// =============================================================================
// Ambient Types (from jcode-ambient-types)
// =============================================================================

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum UsageSource {
    User,
    Ambient,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UsageRecord {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub source: UsageSource,
    pub tokens_input: u32,
    pub tokens_output: u32,
    pub provider: String,
}

impl UsageRecord {
    pub fn total_tokens(&self) -> u64 {
        self.tokens_input as u64 + self.tokens_output as u64
    }
}

#[derive(Debug, Clone)]
pub struct RateLimitInfo {
    pub limit_tokens: Option<u64>,
    pub remaining_tokens: Option<u64>,
    pub limit_requests: Option<u64>,
    pub remaining_requests: Option<u64>,
    pub reset_at: Option<chrono::DateTime<chrono::Utc>>,
}

// =============================================================================
// Batch Types (from jcode-batch-types)
// =============================================================================

/// Progress update from a running batch tool call
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum BatchSubcallState {
    Running,
    Succeeded,
    Failed,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BatchSubcallProgress {
    pub index: usize,
    pub tool_call: ToolCall,
    pub state: BatchSubcallState,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct BatchProgress {
    pub session_id: String,
    /// Parent tool_call_id of the batch call
    pub tool_call_id: String,
    /// Total number of sub-calls in this batch
    pub total: usize,
    /// Number of sub-calls that have completed (success or error)
    pub completed: usize,
    /// Name of the sub-call that just completed
    pub last_completed: Option<String>,
    /// Sub-calls that are currently still running
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub running: Vec<ToolCall>,
    /// Ordered per-subcall progress state for richer UI rendering
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub subcalls: Vec<BatchSubcallProgress>,
}
