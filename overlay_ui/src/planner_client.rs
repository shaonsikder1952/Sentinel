/**
 * Planner Client: Communicates with Python planner API
 */
use serde::{Deserialize, Serialize};
use anyhow::Result;

#[derive(Debug, Serialize, Deserialize)]
pub struct DetectTaskResponse {
    pub success: bool,
    pub task: Option<TaskInfo>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TaskInfo {
    pub task_name: String,
    pub task_source: String,
    pub scheduling: Option<SchedulingInfo>,
    pub automation: AutomationInfo,
    pub workflow: serde_json::Value,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SchedulingInfo {
    pub schedule_type: String,
    pub next_run: String,
    pub recurrence: Option<RecurrenceInfo>,
    pub enabled: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RecurrenceInfo {
    pub frequency: String,
    pub interval: Option<u32>,
    pub days_of_week: Option<Vec<u8>>,
    pub time: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AutomationInfo {
    pub is_repetitive: bool,
    pub auto_run_enabled: bool,
}

pub struct PlannerClient {
    base_url: String,
    client: std::sync::OnceLock<reqwest::Client>,
}

impl PlannerClient {
    pub fn new(base_url: String) -> Self {
        Self {
            base_url,
            client: std::sync::OnceLock::new(),
        }
    }
    
    fn get_client(&self) -> &reqwest::Client {
        self.client.get_or_init(|| reqwest::Client::new())
    }

    pub async fn detect_task_from_chat(
        &self,
        command: &str,
        context: Option<serde_json::Value>,
    ) -> Result<DetectTaskResponse> {
        let url = format!("{}/api/v1/detect-task", self.base_url);
        
        let response = self.get_client()
            .post(&url)
            .json(&serde_json::json!({
                "command": command,
                "context": context,
            }))
            .send()
            .await?;

        let result: DetectTaskResponse = response.json().await?;
        Ok(result)
    }

    pub async fn generate_workflow(
        &self,
        task_name: &str,
        task_description: Option<&str>,
        context: Option<serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let url = format!("{}/api/v1/generate-workflow", self.base_url);
        
        let response = self.get_client()
            .post(&url)
            .json(&serde_json::json!({
                "task_name": task_name,
                "task_description": task_description,
                "context": context,
            }))
            .send()
            .await?;

        let result: serde_json::Value = response.json().await?;
        Ok(result["workflow"].clone())
    }
}

