use crate::types::*;
use crate::verifier::Verifier;
use crate::task_manager::TaskManager;
use anyhow::Result;
use serde_json;
use sha2::{Sha256, Digest};
use std::time::Duration;
use tokio::time::sleep;

pub struct StepExecutor {
    verifier: Verifier,
    task_manager: Arc<TaskManager>,
}

impl StepExecutor {
    pub fn new(task_manager: Arc<TaskManager>) -> Self {
        Self {
            verifier: Verifier::new(),
            task_manager,
        }
    }

    pub async fn execute_step(
        &self,
        task_id: &str,
        step: &Step,
        browser_context: &dyn BrowserContext,
    ) -> Result<serde_json::Value> {
        let mut retry_count = 0;
        let max_retries = step.retry_config.max_retries;

        loop {
            match self.execute_step_internal(task_id, step, browser_context).await {
                Ok(result) => {
                    // Log successful execution
                    let dom_hash = self.compute_dom_hash(browser_context).await?;
                    let verification = self.verifier.verify_step(step, Some(&result), &dom_hash);

                    let log_entry = ExecutionLogEntry {
                        step_id: step.step_id.clone(),
                        timestamp: chrono::Utc::now(),
                        action: format!("{:?}", step.action),
                        dom_snapshot_hash: dom_hash,
                        extracted_data: Some(result.clone()),
                        verification_result: Some(verification.clone()),
                        retry_count,
                    };

                    self.task_manager.add_execution_log_entry(task_id, log_entry)?;

                    if !verification.passed {
                        if retry_count < max_retries {
                            retry_count += 1;
                            sleep(Duration::from_millis(step.retry_config.retry_delay_ms)).await;
                            continue;
                        } else {
                            return Err(anyhow::anyhow!(
                                "Step verification failed after {} retries",
                                max_retries
                            ));
                        }
                    }

                    return Ok(result);
                }
                Err(e) => {
                    if retry_count < max_retries {
                        retry_count += 1;
                        sleep(Duration::from_millis(step.retry_config.retry_delay_ms)).await;
                        continue;
                    } else {
                        return Err(e);
                    }
                }
            }
        }
    }

    async fn execute_step_internal(
        &self,
        task_id: &str,
        step: &Step,
        browser_context: &dyn BrowserContext,
    ) -> Result<serde_json::Value> {
        // Update current step
        self.task_manager.update_current_step(task_id, Some(step.step_id.clone()))?;

        let result = match step.action {
            Action::Navigate => {
                let url = step.parameters
                    .as_ref()
                    .and_then(|p| p.get("url"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Navigate action requires 'url' parameter"))?;
                browser_context.navigate(url).await?;
                serde_json::json!({ "url": url, "status": "navigated" })
            }
            Action::Click => {
                browser_context.click(&step.target).await?;
                serde_json::json!({ "target": step.target, "status": "clicked" })
            }
            Action::Type => {
                let text = step.parameters
                    .as_ref()
                    .and_then(|p| p.get("text"))
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| anyhow::anyhow!("Type action requires 'text' parameter"))?;
                browser_context.type_text(&step.target, text).await?;
                serde_json::json!({ "target": step.target, "text": text, "status": "typed" })
            }
            Action::Extract => {
                let data = browser_context.extract(&step.target, &step.expected_schema).await?;
                data
            }
            Action::Wait => {
                let duration_ms = step.parameters
                    .as_ref()
                    .and_then(|p| p.get("duration_ms"))
                    .and_then(|v| v.as_u64())
                    .unwrap_or(1000);
                sleep(Duration::from_millis(duration_ms)).await;
                serde_json::json!({ "duration_ms": duration_ms, "status": "waited" })
            }
            Action::Verify => {
                let data = browser_context.extract(&step.target, &step.expected_schema).await?;
                let dom_hash = self.compute_dom_hash(browser_context).await?;
                let verification = self.verifier.verify_step(step, Some(&data), &dom_hash);
                serde_json::json!({
                    "verification": verification.passed,
                    "checks": verification.checks
                })
            }
            Action::Submit => {
                browser_context.submit(&step.target).await?;
                serde_json::json!({ "target": step.target, "status": "submitted" })
            }
        };

        Ok(result)
    }

    async fn compute_dom_hash(&self, browser_context: &dyn BrowserContext) -> Result<String> {
        let dom_snapshot = browser_context.get_dom_snapshot().await?;
        let mut hasher = Sha256::new();
        hasher.update(dom_snapshot.as_bytes());
        let hash = format!("{:x}", hasher.finalize());
        Ok(hash)
    }
}

// Trait for browser context abstraction
#[async_trait::async_trait]
pub trait BrowserContext: Send + Sync {
    async fn navigate(&self, url: &str) -> Result<()>;
    async fn click(&self, selector: &str) -> Result<()>;
    async fn type_text(&self, selector: &str, text: &str) -> Result<()>;
    async fn extract(&self, selector: &str, schema: &Option<serde_json::Value>) -> Result<serde_json::Value>;
    async fn submit(&self, selector: &str) -> Result<()>;
    async fn get_dom_snapshot(&self) -> Result<String>;
}

use std::sync::Arc;

