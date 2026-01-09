/**
 * Sidebar component: Approval controls, scheduling, task management
 */
use eframe::egui;
use sentinel_engine::TaskManager;
use std::sync::Arc;

pub struct Sidebar {
    selected_task_id: Option<String>,
}

impl Default for Sidebar {
    fn default() -> Self {
        Self {
            selected_task_id: None,
        }
    }
}

impl Sidebar {
    pub fn ui(&mut self, ui: &mut egui::Ui, task_manager: Option<&Arc<TaskManager>>) {
        ui.heading("Task Controls");

        if let Some(task_manager) = task_manager {
            if let Some(task_id) = &self.selected_task_id {
                if let Some(task) = task_manager.get_task(task_id) {
                    self.render_task_controls(ui, &task, task_manager);
                }
            } else {
                ui.label("Select a task to view controls");
            }
        }
    }

    fn render_task_controls(
        &mut self,
        ui: &mut egui::Ui,
        task: &sentinel_engine::types::Task,
        task_manager: &Arc<TaskManager>,
    ) {
        ui.group(|ui| {
            ui.label(format!("Task: {}", task.task_name));
            ui.label(format!("Status: {:?}", task.status));
            ui.separator();

            // Approval controls
            match task.status {
                sentinel_engine::types::TaskStatus::Pending | sentinel_engine::types::TaskStatus::Approved => {
                    if task.approval_flags.pre_approval_required && !task.approval_flags.pre_approval_granted {
                        ui.label("⚠️ Pre-approval required");
                        ui.horizontal(|ui| {
                            if ui.button("✅ Approve").clicked() {
                                if let Err(e) = task_manager.approve_task(
                                    &task.task_id,
                                    sentinel_engine::task_manager::ApprovalType::PreApproval,
                                ) {
                                    eprintln!("Failed to approve task: {}", e);
                                }
                            }
                            if ui.button("✏️ Edit").clicked() {
                                // TODO: Open edit dialog
                            }
                            if ui.button("❌ Cancel").clicked() {
                                // TODO: Cancel task
                            }
                        });
                    } else {
                        if ui.button("▶️ Start Task").clicked() {
                            if let Err(e) = task_manager.start_task(&task.task_id) {
                                ui.label(format!("Error: {}", e));
                            }
                        }
                    }
                }
                sentinel_engine::types::TaskStatus::InProgress => {
                    ui.horizontal(|ui| {
                        if ui.button("⏸️ Pause").clicked() {
                            if let Err(e) = task_manager.pause_task(&task.task_id) {
                                ui.label(format!("Error: {}", e));
                            }
                        }
                        if ui.button("⏹️ Stop").clicked() {
                            // TODO: Stop task
                        }
                    });
                }
                sentinel_engine::types::TaskStatus::Paused => {
                    if ui.button("▶️ Resume").clicked() {
                        if let Err(e) = task_manager.resume_task(&task.task_id) {
                            ui.label(format!("Error: {}", e));
                        }
                    }
                }
                sentinel_engine::types::TaskStatus::Completed => {
                    if task.approval_flags.post_approval_required && !task.approval_flags.post_approval_granted {
                        ui.label("✅ Post-approval required");
                        ui.horizontal(|ui| {
                            if ui.button("✅ Accept").clicked() {
                                if let Err(e) = task_manager.approve_task(
                                    &task.task_id,
                                    sentinel_engine::task_manager::ApprovalType::PostApproval,
                                ) {
                                    ui.label(format!("Error: {}", e));
                                }
                            }
                            if ui.button("✏️ Request Changes").clicked() {
                                // TODO: Request changes
                            }
                        });
                    } else {
                        ui.label("✅ Task completed");
                    }
                }
                _ => {}
            }

            // Scheduling info
            if let Some(scheduling) = &task.scheduling {
                ui.separator();
                ui.label("📅 Scheduled");
                ui.label(format!("Next run: {}", scheduling.next_run));
                if let Some(recurrence) = &scheduling.recurrence {
                    ui.label(format!("Frequency: {:?}", recurrence.frequency));
                }
            }

            // Automation info
            if task.automation.is_repetitive {
                ui.separator();
                ui.label("🔄 Repetitive Task");
                ui.label(format!("Executions: {}", task.automation.execution_count));
                // Note: Cannot mutate task directly, would need to update via task_manager
                ui.label(format!("Auto-run: {}", if task.automation.auto_run_enabled { "Enabled" } else { "Disabled" }));
            }
        });
    }
}

