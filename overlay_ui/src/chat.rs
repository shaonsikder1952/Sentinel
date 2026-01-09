/**
 * Chat panel: Natural language command input and task creation
 */
use eframe::egui;
use sentinel_engine::TaskManager;
use std::sync::Arc;
use crate::planner_client::PlannerClient;

pub struct ChatPanel {
    input_buffer: String,
    messages: Vec<ChatMessage>,
}

#[derive(Clone)]
struct ChatMessage {
    role: String,
    content: String,
    timestamp: chrono::DateTime<chrono::Utc>,
}

impl Default for ChatPanel {
    fn default() -> Self {
        Self {
            input_buffer: String::new(),
            messages: Vec::new(),
        }
    }
}

impl ChatPanel {
    pub fn ui(
        &mut self,
        ui: &mut egui::Ui,
        planner_client: &mut PlannerClient,
        task_manager: Option<&Arc<TaskManager>>,
    ) {
        ui.heading("💬 Chat");

        // Messages area
        egui::ScrollArea::vertical()
            .max_height(200.0)
            .show(ui, |ui| {
                for msg in &self.messages {
                    ui.horizontal(|ui| {
                        ui.label(format!("[{}] {}", msg.role, msg.content));
                    });
                }
            });

        ui.separator();

        // Input area
        ui.horizontal(|ui| {
            let input = egui::TextEdit::singleline(&mut self.input_buffer)
                .hint_text("Type a command... (e.g., 'Do weekly KPI report')")
                .desired_width(ui.available_width() - 60.0);

            let response = ui.add(input);

            // Handle Enter key
            if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                self.handle_command(planner_client, task_manager);
            }

            if ui.button("Send").clicked() {
                self.handle_command(planner_client, task_manager);
            }
        });
    }

    fn handle_command(
        &mut self,
        planner_client: &mut PlannerClient,
        task_manager: Option<&Arc<TaskManager>>,
    ) {
        if self.input_buffer.trim().is_empty() {
            return;
        }

        let command = self.input_buffer.clone();
        self.input_buffer.clear();

        // Add user message
        self.messages.push(ChatMessage {
            role: "You".to_string(),
            content: command.clone(),
            timestamp: chrono::Utc::now(),
        });

        // Process command
        let command_clone = command.clone();
        let task_manager_clone = task_manager.cloned();
        
        if let Some(task_manager) = task_manager_clone {
            let rt = tokio::runtime::Runtime::new().unwrap();
            rt.block_on(async {
                match planner_client.detect_task_from_chat(&command_clone, None).await {
                    Ok(detected) => {
                        if detected.success {
                            if let Some(task_info) = detected.task {
                                // Create task - need to convert workflow from JSON
                                use sentinel_engine::types::*;
                                let workflow = Workflow {
                                    workflow_id: task_info.workflow.get("workflow_id")
                                    .and_then(|v| v.as_str())
                                    .unwrap_or("default")
                                    .to_string(),
                                    steps: task_info.workflow.get("steps")
                                        .and_then(|v: &serde_json::Value| v.as_array())
                                        .map(|arr: &Vec<serde_json::Value>| {
                                            arr.iter().filter_map(|s: &serde_json::Value| {
                                            // Convert JSON step to Step struct
                                            // This is a simplified conversion
                                            Some(Step {
                                                step_id: s.get("step_id")?.as_str()?.to_string(),
                                                action: match s.get("action")?.as_str()? {
                                                    "navigate" => Action::Navigate,
                                                    "click" => Action::Click,
                                                    "type" => Action::Type,
                                                    "extract" => Action::Extract,
                                                    "wait" => Action::Wait,
                                                    "verify" => Action::Verify,
                                                    "submit" => Action::Submit,
                                                    _ => return None,
                                                },
                                                target: s.get("target")?.as_str()?.to_string(),
                                                parameters: s.get("parameters").and_then(|p: &serde_json::Value| {
                                                    if p.is_object() {
                                                        p.as_object().map(|obj| {
                                                            obj.iter().map(|(k, v)| (k.clone(), v.clone())).collect::<std::collections::HashMap<String, serde_json::Value>>()
                                                        })
                                                    } else {
                                                        None
                                                    }
                                                }),
                                                expected_schema: s.get("expected_schema").cloned(),
                                                verification: s.get("verification")
                                                    .and_then(|v: &serde_json::Value| v.as_array())
                                                    .map(|arr: &Vec<serde_json::Value>| {
                                                        arr.iter().filter_map(|v: &serde_json::Value| {
                                                            match v.as_str()? {
                                                                "schema" => Some(VerificationType::Schema),
                                                                "sanity_check" => Some(VerificationType::SanityCheck),
                                                                "element_presence" => Some(VerificationType::ElementPresence),
                                                                "numeric_range" => Some(VerificationType::NumericRange),
                                                                _ => None,
                                                            }
                                                        }).collect()
                                                    })
                                                    .unwrap_or_default(),
                                                retry_config: s.get("retry_config")
                                                    .and_then(|r: &serde_json::Value| {
                                                        Some(RetryConfig {
                                                            max_retries: r.get("max_retries")?.as_u64()? as u32,
                                                            retry_delay_ms: r.get("retry_delay_ms")?.as_u64()?,
                                                        })
                                                    })
                                                    .unwrap_or_default(),
                                                requires_approval: s.get("requires_approval")
                                                    .and_then(|v| v.as_bool())
                                                    .unwrap_or(false),
                                            })
                                        }).collect()
                                    })
                                    .unwrap_or_default(),
                            };
                            
                                let task = task_manager.create_task(
                                    task_info.task_name.clone(),
                                    sentinel_engine::types::TaskSource::UserChat,
                                    workflow,
                                    None,
                                    task_info.scheduling.map(|s| {
                                    sentinel_engine::types::Scheduling {
                                        schedule_type: match s.schedule_type.as_str() {
                                            "once" => sentinel_engine::types::ScheduleType::Once,
                                            "recurring" => sentinel_engine::types::ScheduleType::Recurring,
                                            _ => sentinel_engine::types::ScheduleType::Once,
                                        },
                                        next_run: chrono::DateTime::parse_from_rfc3339(&s.next_run)
                                            .unwrap()
                                            .with_timezone(&chrono::Utc),
                                        recurrence: s.recurrence.map(|r| {
                                            sentinel_engine::types::Recurrence {
                                                frequency: match r.frequency.as_str() {
                                                    "daily" => sentinel_engine::types::Frequency::Daily,
                                                    "weekly" => sentinel_engine::types::Frequency::Weekly,
                                                    "monthly" => sentinel_engine::types::Frequency::Monthly,
                                                    _ => sentinel_engine::types::Frequency::Custom,
                                                },
                                                interval: r.interval,
                                                days_of_week: r.days_of_week,
                                                time: r.time,
                                            }
                                        }),
                                        enabled: s.enabled,
                                    }
                                }),
                                    Some(sentinel_engine::types::Automation {
                                        is_repetitive: task_info.automation.is_repetitive,
                                        auto_run_enabled: task_info.automation.auto_run_enabled,
                                        execution_count: 0,
                                    }),
                                );

                                match task {
                                    Ok(task) => {
                                        self.messages.push(ChatMessage {
                                            role: "AI".to_string(),
                                            content: format!("✅ Task created: {}", task.task_name),
                                            timestamp: chrono::Utc::now(),
                                        });
                                    }
                                    Err(e) => {
                                        self.messages.push(ChatMessage {
                                            role: "AI".to_string(),
                                            content: format!("❌ Error: {}", e),
                                            timestamp: chrono::Utc::now(),
                                        });
                                    }
                                }
                            } else {
                                self.messages.push(ChatMessage {
                                    role: "AI".to_string(),
                                    content: "❌ No task information received".to_string(),
                                    timestamp: chrono::Utc::now(),
                                });
                            }
                        } else {
                            self.messages.push(ChatMessage {
                                role: "AI".to_string(),
                                content: format!("❌ {}", detected.error.unwrap_or_default()),
                                timestamp: chrono::Utc::now(),
                            });
                        }
                    }
                    Err(e) => {
                        self.messages.push(ChatMessage {
                            role: "AI".to_string(),
                            content: format!("❌ Error: {}", e),
                            timestamp: chrono::Utc::now(),
                        });
                    }
                }
            });
        }
    }
}

