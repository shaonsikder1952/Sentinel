use crate::types::*;
use crate::task_manager::{TaskManager, ApprovalType as TaskApprovalType};
use serde::{Deserialize, Serialize};
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::mpsc;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "method")]
pub enum IpcRequest {
    CreateTask {
        task_name: String,
        task_source: TaskSource,
        workflow: Workflow,
        approval_flags: Option<ApprovalFlags>,
        scheduling: Option<Scheduling>,
        automation: Option<Automation>,
    },
    GetTask {
        task_id: String,
    },
    ApproveTask {
        task_id: String,
        approval_type: ApprovalType,
    },
    StartTask {
        task_id: String,
    },
    PauseTask {
        task_id: String,
    },
    ResumeTask {
        task_id: String,
    },
    CompleteTask {
        task_id: String,
    },
    FailTask {
        task_id: String,
        error: String,
    },
    GetAllTasks,
    GetPendingTasks,
    RegisterScheduledTask {
        task_id: String,
        scheduling: Scheduling,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IpcResponse {
    TaskCreated { task: Task },
    Task { task: Option<Task> },
    Tasks { tasks: Vec<Task> },
    Success,
    Error { message: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ApprovalType {
    PreApproval,
    PostApproval,
}

pub struct IpcLayer {
    task_manager: Arc<TaskManager>,
    request_tx: mpsc::UnboundedSender<IpcRequest>,
    response_rx: mpsc::UnboundedReceiver<IpcResponse>,
}

impl IpcLayer {
    pub fn new(task_manager: Arc<TaskManager>) -> (Self, mpsc::UnboundedReceiver<IpcRequest>, mpsc::UnboundedSender<IpcResponse>) {
        let (request_tx, request_rx) = mpsc::unbounded_channel();
        let (response_tx, response_rx) = mpsc::unbounded_channel();

        let layer = Self {
            task_manager,
            request_tx,
            response_rx,
        };

        (layer, request_rx, response_tx)
    }

    pub fn send_request(&self, request: IpcRequest) -> Result<()> {
        self.request_tx.send(request)?;
        Ok(())
    }

    pub async fn handle_request(&self, request: IpcRequest, response_tx: &mpsc::UnboundedSender<IpcResponse>) -> Result<()> {
        let response = match request {
            IpcRequest::CreateTask {
                task_name,
                task_source,
                workflow,
                approval_flags,
                scheduling,
                automation,
            } => {
                match self.task_manager.create_task(
                    task_name,
                    task_source,
                    workflow,
                    approval_flags,
                    scheduling,
                    automation,
                ) {
                    Ok(task) => IpcResponse::TaskCreated { task },
                    Err(e) => IpcResponse::Error { message: e.to_string() },
                }
            }
            IpcRequest::GetTask { task_id } => {
                let task = self.task_manager.get_task(&task_id);
                IpcResponse::Task { task }
            }
            IpcRequest::ApproveTask { task_id, approval_type } => {
                let task_approval_type = match approval_type {
                    ApprovalType::PreApproval => TaskApprovalType::PreApproval,
                    ApprovalType::PostApproval => TaskApprovalType::PostApproval,
                };
                match self.task_manager.approve_task(&task_id, task_approval_type) {
                    Ok(_) => IpcResponse::Success,
                    Err(e) => IpcResponse::Error { message: e.to_string() },
                }
            }
            IpcRequest::StartTask { task_id } => {
                match self.task_manager.start_task(&task_id) {
                    Ok(_) => IpcResponse::Success,
                    Err(e) => IpcResponse::Error { message: e.to_string() },
                }
            }
            IpcRequest::PauseTask { task_id } => {
                match self.task_manager.pause_task(&task_id) {
                    Ok(_) => IpcResponse::Success,
                    Err(e) => IpcResponse::Error { message: e.to_string() },
                }
            }
            IpcRequest::ResumeTask { task_id } => {
                match self.task_manager.resume_task(&task_id) {
                    Ok(_) => IpcResponse::Success,
                    Err(e) => IpcResponse::Error { message: e.to_string() },
                }
            }
            IpcRequest::CompleteTask { task_id } => {
                match self.task_manager.complete_task(&task_id) {
                    Ok(_) => IpcResponse::Success,
                    Err(e) => IpcResponse::Error { message: e.to_string() },
                }
            }
            IpcRequest::FailTask { task_id, error } => {
                match self.task_manager.fail_task(&task_id, error) {
                    Ok(_) => IpcResponse::Success,
                    Err(e) => IpcResponse::Error { message: e.to_string() },
                }
            }
            IpcRequest::GetAllTasks => {
                let tasks = self.task_manager.get_all_tasks();
                IpcResponse::Tasks { tasks }
            }
            IpcRequest::GetPendingTasks => {
                let tasks = self.task_manager.get_pending_tasks();
                IpcResponse::Tasks { tasks }
            }
            IpcRequest::RegisterScheduledTask { task_id: _, scheduling: _ } => {
                // This would be handled by the scheduler
                IpcResponse::Success
            }
        };

        response_tx.send(response)?;
        Ok(())
    }
}

