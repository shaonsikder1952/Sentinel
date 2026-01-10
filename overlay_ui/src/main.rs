use eframe::egui;
use std::sync::{Arc, Mutex};

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([400.0, 800.0])
            .with_position([1520.0, 0.0]) // Right side of 1920px screen
            .with_always_on_top()
            .with_decorations(true)
            .with_resizable(true)
            .with_transparent(false),
        ..Default::default()
    };

    eframe::run_native(
        "Sentinel AI Overlay",
        options,
        Box::new(|_cc| Box::new(SentinelApp::default())),
    )
}

struct SentinelApp {
    chat_input: String,
    messages: Vec<ChatMessage>,
    tasks: Vec<Task>,
}

#[derive(Clone)]
struct ChatMessage {
    user: bool,
    text: String,
}

#[derive(Clone)]
struct Task {
    description: String,
    status: TaskStatus,
}

#[derive(Clone, PartialEq)]
enum TaskStatus {
    Pending,
    Approved,
    Running,
    Complete,
}

impl Default for SentinelApp {
    fn default() -> Self {
        Self {
            chat_input: String::new(),
            messages: vec![
                ChatMessage {
                    user: false,
                    text: "Hello! I'm your Sentinel AI assistant. How can I help?".to_string(),
                }
            ],
            tasks: Vec::new(),
        }
    }
}

impl eframe::App for SentinelApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("\u{1f6e1}\ufe0f Sentinel AI");
            ui.separator();

            // Chat Section
            ui.group(|ui| {
                ui.heading("\u{1f4ac} Chat");
                
                egui::ScrollArea::vertical()
                    .max_height(300.0)
                    .show(ui, |ui| {
                        for msg in &self.messages {
                            let color = if msg.user {
                                egui::Color32::from_rgb(100, 150, 255)
                            } else {
                                egui::Color32::from_rgb(200, 200, 200)
                            };
                            
                            ui.horizontal(|ui| {
                                ui.label(if msg.user { "You:" } else { "AI:" });
                                ui.colored_label(color, &msg.text);
                            });
                            ui.add_space(5.0);
                        }
                    });
                
                ui.separator();
                
                ui.horizontal(|ui| {
                    ui.text_edit_singleline(&mut self.chat_input);
                    if ui.button("Send").clicked() && !self.chat_input.is_empty() {
                        let user_msg = self.chat_input.clone();
                        self.messages.push(ChatMessage {
                            user: true,
                            text: user_msg.clone(),
                        });
                        
                        // Simple echo response for now
                        self.messages.push(ChatMessage {
                            user: false,
                            text: format!("I received: {}", user_msg),
                        });
                        
                        self.chat_input.clear();
                    }
                });
            });

            ui.add_space(10.0);

            // Tasks Section
            ui.group(|ui| {
                ui.heading("\u{2705} Tasks");
                
                if self.tasks.is_empty() {
                    ui.label("No tasks yet");
                } else {
                    for task in &self.tasks {
                        ui.horizontal(|ui| {
                            let status_text = match task.status {
                                TaskStatus::Pending => "\u{23f3}",
                                TaskStatus::Approved => "\u{2705}",
                                TaskStatus::Running => "\u{25b6}\ufe0f",
                                TaskStatus::Complete => "\u{2714}\ufe0f",
                            };
                            ui.label(status_text);
                            ui.label(&task.description);
                        });
                    }
                }
                
                if ui.button("\u{2795} Add Test Task").clicked() {
                    self.tasks.push(Task {
                        description: "Example task".to_string(),
                        status: TaskStatus::Pending,
                    });
                }
            });

            ui.add_space(10.0);

            // Status Section
            ui.group(|ui| {
                ui.heading("\u{1f4ca} Status");
                ui.label(format!("Messages: {}", self.messages.len()));
                ui.label(format!("Tasks: {}", self.tasks.len()));
                ui.label("API: Disconnected (demo mode)");
            });
        });
    }
}
