use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::sync::OnceLock;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DetectTaskResponse {
    pub success: bool,
    pub task: Option<TaskInfo>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TaskInfo {
    pub task_name: String,
    pub task_source: String,
    pub scheduling: Option<String>,
    pub automation: AutomationInfo,
    pub workflow: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct SchedulingInfo {
    pub schedule_type: String,
    pub next_run: String,
    pub recurrence: Option<String>,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AutomationInfo {
    pub is_repetitive: bool,
    pub auto_run_enabled: bool,
}

pub struct PlannerClient {
    base_url: String,
    client: OnceLock<reqwest::Client>,
}

impl PlannerClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: OnceLock::new(),
        }
    }
    
    fn get_client(&self) -> &reqwest::Client {
        self.client.get_or_init(|| reqwest::Client::new())
    }

    pub async fn detect_task_from_chat(
        &self,
        command: &str,
    ) -> Result<DetectTaskResponse> {
        let url = format!("{}/detect-task", self.base_url);
        let response = self.get_client()
            .post(&url)
            .json(&serde_json::json!({
                "user_input": command
            }))
            .send()
            .await?;

        let result: DetectTaskResponse = response.json().await?;
        Ok(result)
    }
}
