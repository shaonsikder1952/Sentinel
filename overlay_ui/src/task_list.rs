/**
 * Task List component: Displays all tasks with status and actions
 */
use eframe::egui;
use sentinel_engine::TaskManager;
use std::sync::Arc;

pub struct TaskList {
    selected_task_id: Option<String>,
}

impl Default for TaskList {
    fn default() -> Self {
        Self {
            selected_task_id: None,
        }
    }
}

impl TaskList {
    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        task_manager: Option<&Arc<TaskManager>>,
        _scheduler: Option<&Arc<sentinel_engine::Scheduler>>,
    ) {
        ui.heading("Tasks");

        if let Some(task_manager) = task_manager {
            let tasks = task_manager.get_all_tasks();

            if tasks.is_empty() {
                ui.label("No tasks yet. Create one using the chat above.");
                return;
            }

            egui::ScrollArea::vertical()
                .max_height(300.0)
                .show(ui, |ui| {
                    for task in tasks {
                        let is_selected = self.selected_task_id.as_ref()
                            .map(|id| id == &task.task_id)
                            .unwrap_or(false);

                        let response = ui.selectable_label(is_selected, &task.task_name);
                        if response.clicked() {
                            self.selected_task_id = Some(task.task_id.clone());
                        }

                        // Show status badge
                        ui.horizontal(|ui| {
                            ui.label(self.status_icon(&task.status));
                            ui.label(format!("{:?}", task.status));
                            
                            if let Some(_scheduling) = &task.scheduling {
                                ui.label("📅");
                            }
                            if task.automation.is_repetitive {
                                ui.label("🔄");
                            }
                        });

                        ui.separator();
                    }
                });
        }
    }

    fn status_icon(&self, status: &sentinel_engine::types::TaskStatus) -> &'static str {
        match status {
            sentinel_engine::types::TaskStatus::Pending => "⏳",
            sentinel_engine::types::TaskStatus::Approved => "✅",
            sentinel_engine::types::TaskStatus::InProgress => "🔄",
            sentinel_engine::types::TaskStatus::Paused => "⏸️",
            sentinel_engine::types::TaskStatus::Completed => "✅",
            sentinel_engine::types::TaskStatus::Failed => "❌",
            sentinel_engine::types::TaskStatus::Cancelled => "🚫",
        }
    }
}

