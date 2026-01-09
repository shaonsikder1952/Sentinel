use crate::types::*;
use chrono::Utc;
use dashmap::DashMap;
use std::sync::Arc;
use uuid::Uuid;
use anyhow::Result;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum TaskManagerError {
    #[error("Task not found: {0}")]
    TaskNotFound(String),
    #[error("Approval required: {0}")]
    ApprovalRequired(String),
    #[error("Invalid state transition: {0} -> {1}")]
    InvalidStateTransition(String, String),
    #[error("Task already in progress: {0}")]
    TaskInProgress(String),
}

pub struct TaskManager {
    tasks: Arc<DashMap<String, Task>>,
    memory_manager: Arc<MemoryManager>,
}

impl TaskManager {
    pub fn new(memory_manager: Arc<MemoryManager>) -> Self {
        Self {
            tasks: Arc::new(DashMap::new()),
            memory_manager,
        }
    }

    pub fn create_task(
        &self,
        task_name: String,
        task_source: TaskSource,
        workflow: Workflow,
        approval_flags: Option<ApprovalFlags>,
        scheduling: Option<Scheduling>,
        automation: Option<Automation>,
    ) -> Result<Task> {
        let task_id = Uuid::new_v4().to_string();
        let now = Utc::now();

        let approval = approval_flags.unwrap_or_else(|| {
            // Check project memory for default preferences
            let project_memory = self.memory_manager.get_project_memory("default");
            if let Some(project) = project_memory {
                ApprovalFlags {
                    pre_approval_required: project.automation_preferences.default_pre_approval,
                    pre_approval_granted: false,
                    pre_approval_timestamp: None,
                    post_approval_required: project.automation_preferences.default_post_approval,
                    post_approval_granted: false,
                    post_approval_timestamp: None,
                    auto_approved: false,
                }
            } else {
                ApprovalFlags::default()
            }
        });

        let automation = automation.unwrap_or_default();

        let task = Task {
            task_id: task_id.clone(),
            task_name,
            task_source,
            status: TaskStatus::Pending,
            approval_flags: approval.clone(),
            scheduling,
            automation,
            workflow,
            current_step: None,
            page_state: None,
            execution_log: Vec::new(),
            created_at: now,
            updated_at: now,
        };

        // Store in memory manager
        self.memory_manager.store_task_memory(&task)?;

        // Insert into active tasks
        self.tasks.insert(task_id.clone(), task.clone());

        Ok(task)
    }

    pub fn get_task(&self, task_id: &str) -> Option<Task> {
        self.tasks.get(task_id).map(|t| t.clone())
    }

    pub fn approve_task(&self, task_id: &str, approval_type: ApprovalType) -> Result<()> {
        let mut task = self.tasks.get_mut(task_id)
            .ok_or_else(|| TaskManagerError::TaskNotFound(task_id.to_string()))?;

        let now = Utc::now();

        match approval_type {
            ApprovalType::PreApproval => {
                if !task.approval_flags.pre_approval_required {
                    return Ok(()); // No approval needed
                }
                task.approval_flags.pre_approval_granted = true;
                task.approval_flags.pre_approval_timestamp = Some(now);
                if task.status == TaskStatus::Pending {
                    task.status = TaskStatus::Approved;
                }
            }
            ApprovalType::PostApproval => {
                if !task.approval_flags.post_approval_required {
                    return Ok(()); // No approval needed
                }
                task.approval_flags.post_approval_granted = true;
                task.approval_flags.post_approval_timestamp = Some(now);
                if task.status == TaskStatus::Completed {
                    // Task is finalized
                }
            }
        }

        task.updated_at = now;
        self.memory_manager.store_task_memory(&task)?;

        Ok(())
    }

    pub fn can_start_task(&self, task_id: &str) -> Result<bool> {
        let task = self.tasks.get(task_id)
            .ok_or_else(|| TaskManagerError::TaskNotFound(task_id.to_string()))?;

        // Check if task is in a valid state
        match task.status {
            TaskStatus::Pending | TaskStatus::Approved | TaskStatus::Paused => {},
            TaskStatus::InProgress => {
                return Err(TaskManagerError::TaskInProgress(task_id.to_string()).into());
            }
            _ => {
                return Err(TaskManagerError::InvalidStateTransition(
                    format!("{:?}", task.status),
                    "InProgress".to_string(),
                ).into());
            }
        }

        // Check approval requirements
        if task.approval_flags.pre_approval_required {
            if !task.approval_flags.pre_approval_granted && !task.approval_flags.auto_approved {
                return Ok(false);
            }
        }

        // Check if repetitive task can auto-run
        if task.automation.auto_run_enabled && task.automation.execution_count > 0 {
            return Ok(true);
        }

        Ok(task.approval_flags.pre_approval_granted || task.approval_flags.auto_approved)
    }

    pub fn start_task(&self, task_id: &str) -> Result<()> {
        if !self.can_start_task(task_id)? {
            return Err(TaskManagerError::ApprovalRequired(task_id.to_string()).into());
        }

        let mut task = self.tasks.get_mut(task_id)
            .ok_or_else(|| TaskManagerError::TaskNotFound(task_id.to_string()))?;

        task.status = TaskStatus::InProgress;
        task.updated_at = Utc::now();
        self.memory_manager.store_task_memory(&task)?;

        Ok(())
    }

    pub fn pause_task(&self, task_id: &str) -> Result<()> {
        let mut task = self.tasks.get_mut(task_id)
            .ok_or_else(|| TaskManagerError::TaskNotFound(task_id.to_string()))?;

        if task.status != TaskStatus::InProgress {
            return Err(TaskManagerError::InvalidStateTransition(
                format!("{:?}", task.status),
                "Paused".to_string(),
            ).into());
        }

        task.status = TaskStatus::Paused;
        task.updated_at = Utc::now();
        self.memory_manager.store_task_memory(&task)?;

        Ok(())
    }

    pub fn resume_task(&self, task_id: &str) -> Result<()> {
        let mut task = self.tasks.get_mut(task_id)
            .ok_or_else(|| TaskManagerError::TaskNotFound(task_id.to_string()))?;

        if task.status != TaskStatus::Paused {
            return Err(TaskManagerError::InvalidStateTransition(
                format!("{:?}", task.status),
                "InProgress".to_string(),
            ).into());
        }

        task.status = TaskStatus::InProgress;
        task.updated_at = Utc::now();
        self.memory_manager.store_task_memory(&task)?;

        Ok(())
    }

    pub fn complete_task(&self, task_id: &str) -> Result<()> {
        let mut task = self.tasks.get_mut(task_id)
            .ok_or_else(|| TaskManagerError::TaskNotFound(task_id.to_string()))?;

        task.status = TaskStatus::Completed;
        task.updated_at = Utc::now();
        task.automation.execution_count += 1;

        // Update project memory with workflow history
        self.memory_manager.record_workflow_history(
            "default",
            &task.task_id,
            true,
            0, // Duration would be calculated
        )?;

        self.memory_manager.store_task_memory(&task)?;

        Ok(())
    }

    pub fn fail_task(&self, task_id: &str, error: String) -> Result<()> {
        let mut task = self.tasks.get_mut(task_id)
            .ok_or_else(|| TaskManagerError::TaskNotFound(task_id.to_string()))?;

        task.status = TaskStatus::Failed;
        task.updated_at = Utc::now();

        // Log error in execution log
        task.execution_log.push(ExecutionLogEntry {
            step_id: "error".to_string(),
            timestamp: Utc::now(),
            action: "error".to_string(),
            dom_snapshot_hash: String::new(),
            extracted_data: Some(serde_json::json!({ "error": error })),
            verification_result: None,
            retry_count: 0,
        });

        self.memory_manager.store_task_memory(&task)?;

        Ok(())
    }

    pub fn update_current_step(&self, task_id: &str, step_id: Option<String>) -> Result<()> {
        let mut task = self.tasks.get_mut(task_id)
            .ok_or_else(|| TaskManagerError::TaskNotFound(task_id.to_string()))?;

        task.current_step = step_id;
        task.updated_at = Utc::now();
        self.memory_manager.store_task_memory(&task)?;

        Ok(())
    }

    pub fn add_execution_log_entry(&self, task_id: &str, entry: ExecutionLogEntry) -> Result<()> {
        let mut task = self.tasks.get_mut(task_id)
            .ok_or_else(|| TaskManagerError::TaskNotFound(task_id.to_string()))?;

        task.execution_log.push(entry);
        task.updated_at = Utc::now();
        self.memory_manager.store_task_memory(&task)?;

        Ok(())
    }

    pub fn get_all_tasks(&self) -> Vec<Task> {
        self.tasks.iter().map(|t| t.clone()).collect()
    }

    pub fn get_pending_tasks(&self) -> Vec<Task> {
        self.tasks.iter()
            .filter(|t| matches!(t.status, TaskStatus::Pending | TaskStatus::Approved))
            .map(|t| t.clone())
            .collect()
    }
}

#[derive(Debug, Clone)]
pub enum ApprovalType {
    PreApproval,
    PostApproval,
}

use crate::memory_manager::MemoryManager;

