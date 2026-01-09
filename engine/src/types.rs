use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Task {
    pub task_id: String,
    pub task_name: String,
    pub task_source: TaskSource,
    pub status: TaskStatus,
    pub approval_flags: ApprovalFlags,
    pub scheduling: Option<Scheduling>,
    pub automation: Automation,
    pub workflow: Workflow,
    pub current_step: Option<String>,
    pub page_state: Option<PageState>,
    pub execution_log: Vec<ExecutionLogEntry>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskSource {
    UserManual,
    UserChat,
    AiAutoDetected,
    AiSuggested,
    Scheduled,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Pending,
    Approved,
    InProgress,
    Paused,
    Completed,
    Failed,
    Cancelled,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApprovalFlags {
    pub pre_approval_required: bool,
    pub pre_approval_granted: bool,
    pub pre_approval_timestamp: Option<DateTime<Utc>>,
    pub post_approval_required: bool,
    pub post_approval_granted: bool,
    pub post_approval_timestamp: Option<DateTime<Utc>>,
    pub auto_approved: bool,
}

impl Default for ApprovalFlags {
    fn default() -> Self {
        Self {
            pre_approval_required: true,
            pre_approval_granted: false,
            pre_approval_timestamp: None,
            post_approval_required: true,
            post_approval_granted: false,
            post_approval_timestamp: None,
            auto_approved: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Scheduling {
    pub schedule_type: ScheduleType,
    pub next_run: DateTime<Utc>,
    pub recurrence: Option<Recurrence>,
    pub enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ScheduleType {
    Once,
    Recurring,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recurrence {
    pub frequency: Frequency,
    pub interval: Option<u32>,
    pub days_of_week: Option<Vec<u8>>,
    pub time: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Frequency {
    Daily,
    Weekly,
    Monthly,
    Custom,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Automation {
    pub is_repetitive: bool,
    pub auto_run_enabled: bool,
    pub execution_count: u32,
}

impl Default for Automation {
    fn default() -> Self {
        Self {
            is_repetitive: false,
            auto_run_enabled: false,
            execution_count: 0,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Workflow {
    pub workflow_id: String,
    pub steps: Vec<Step>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Step {
    pub step_id: String,
    pub action: Action,
    pub target: String,
    pub parameters: Option<HashMap<String, serde_json::Value>>,
    pub expected_schema: Option<serde_json::Value>,
    pub verification: Vec<VerificationType>,
    pub retry_config: RetryConfig,
    pub requires_approval: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Action {
    Navigate,
    Click,
    Type,
    Extract,
    Wait,
    Verify,
    Submit,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum VerificationType {
    Schema,
    SanityCheck,
    ElementPresence,
    NumericRange,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RetryConfig {
    pub max_retries: u32,
    pub retry_delay_ms: u64,
}

impl Default for RetryConfig {
    fn default() -> Self {
        Self {
            max_retries: 2,
            retry_delay_ms: 1000,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageState {
    pub url: String,
    pub initial_state_hash: String,
    pub elements_seen: Vec<ElementInfo>,
    pub elements_relevant: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ElementInfo {
    pub selector: String,
    pub semantic_type: String,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExecutionLogEntry {
    pub step_id: String,
    pub timestamp: DateTime<Utc>,
    pub action: String,
    pub dom_snapshot_hash: String,
    pub extracted_data: Option<serde_json::Value>,
    pub verification_result: Option<VerificationResult>,
    pub retry_count: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationResult {
    pub passed: bool,
    pub checks: Vec<CheckResult>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CheckResult {
    pub check_type: String,
    pub passed: bool,
    pub message: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectMemory {
    pub project_id: String,
    pub project_name: String,
    pub recurring_rules: Vec<RecurringRule>,
    pub workflow_history: Vec<WorkflowHistoryEntry>,
    pub automation_preferences: AutomationPreferences,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecurringRule {
    pub rule_id: String,
    pub pattern: String,
    pub auto_create_task: bool,
    pub suggest_task: bool,
    pub workflow_template: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorkflowHistoryEntry {
    pub task_id: String,
    pub executed_at: DateTime<Utc>,
    pub success: bool,
    pub duration_ms: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AutomationPreferences {
    pub default_pre_approval: bool,
    pub default_post_approval: bool,
    pub auto_approve_repetitive_after: u32,
}

impl Default for AutomationPreferences {
    fn default() -> Self {
        Self {
            default_pre_approval: true,
            default_post_approval: true,
            auto_approve_repetitive_after: 3,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMemory {
    pub app_schemas: HashMap<String, AppSchema>,
    pub safety_rules: Vec<SafetyRule>,
    pub workflow_templates: Vec<Workflow>,
    pub version: String,
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSchema {
    pub app_name: String,
    pub domain: String,
    pub verified_selectors: Vec<VerifiedSelector>,
    pub ui_patterns: Vec<UIPattern>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerifiedSelector {
    pub selector: String,
    pub semantic_type: String,
    pub verified_at: DateTime<Utc>,
    pub success_rate: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UIPattern {
    pub pattern_name: String,
    pub description: String,
    pub selectors: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyRule {
    pub rule_id: String,
    pub rule_type: SafetyRuleType,
    pub condition: serde_json::Value,
    pub action: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum SafetyRuleType {
    ApprovalRequired,
    VerificationRequired,
    RateLimit,
    DomainRestriction,
}

