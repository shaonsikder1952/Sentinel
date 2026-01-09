use eframe::egui;
use crate::planner_client::PlannerClient;

pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

pub struct SentinelApp {
    messages: Vec<ChatMessage>,
    input_text: String,
    planner_client: PlannerClient,
    is_processing: bool,
}

impl Default for SentinelApp {
    fn default() -> Self {
        Self {
            messages: vec![
                ChatMessage {
                    role: "assistant".to_string(),
                    content: "Hello! I'm Sentinel AI. How can I help you today?".to_string(),
                },
            ],
            input_text: String::new(),
            planner_client: PlannerClient::new("http://localhost:8000".to_string()),
            is_processing: false,
        }
    }
}

impl SentinelApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self::default()
    }
}

impl eframe::App for SentinelApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Modern dark theme like Cursor/Comet
        let mut style = (*ctx.style()).clone();
        style.visuals.window_fill = egui::Color32::from_rgb(23, 23, 23);
        style.visuals.panel_fill = egui::Color32::from_rgb(23, 23, 23);
        style.visuals.extreme_bg_color = egui::Color32::from_rgb(18, 18, 18);
        style.visuals.faint_bg_color = egui::Color32::from_rgb(30, 30, 30);
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        ctx.set_style(style);

        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(egui::Color32::from_rgb(23, 23, 23)))
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Header
                    self.render_header(ui);
                    
                    ui.add_space(4.0);
                    ui.separator();
                    ui.add_space(8.0);

                    // Chat messages area
                    self.render_messages(ui);

                    // Input area at bottom
                    self.render_input(ui);
                });
            });
    }
}

impl SentinelApp {
    fn render_header(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add_space(16.0);
            
            // Title with icon
            ui.label(
                egui::RichText::new("💬 Sentinel")
                    .size(18.0)
                    .color(egui::Color32::from_rgb(200, 200, 200))
                    .strong(),
            );

            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(16.0);
                
                // Settings button
                if ui
                    .add(
                        egui::Button::new(
                            egui::RichText::new("⚙")
                                .size(16.0)
                                .color(egui::Color32::from_rgb(150, 150, 150)),
                        )
                        .frame(false),
                    )
                    .on_hover_text("Settings")
                    .clicked()
                {
                    // Settings menu
                }
            });
        });
    }

    fn render_messages(&mut self, ui: &mut egui::Ui) {
        let available_height = ui.available_height() - 120.0; // Reserve space for input

        egui::ScrollArea::vertical()
            .auto_shrink([false; 2])
            .max_height(available_height)
            .stick_to_bottom(true)
            .show(ui, |ui| {
                ui.add_space(8.0);

                for msg in &self.messages {
                    self.render_message(ui, msg);
                }

                if self.is_processing {
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.add_space(16.0);
                        ui.spinner();
                        ui.label(
                            egui::RichText::new("Thinking...")
                                .color(egui::Color32::from_rgb(150, 150, 150))
                                .size(14.0),
                        );
                    });
                }

                ui.add_space(8.0);
            });
    }

    fn render_message(&self, ui: &mut egui::Ui, msg: &ChatMessage) {
        ui.horizontal(|ui| {
            ui.add_space(16.0);

            ui.vertical(|ui| {
                // Role label
                let (role_text, role_color) = if msg.role == "user" {
                    ("You", egui::Color32::from_rgb(100, 150, 255))
                } else {
                    ("Sentinel", egui::Color32::from_rgb(100, 200, 100))
                };

                ui.label(
                    egui::RichText::new(role_text)
                        .size(13.0)
                        .color(role_color)
                        .strong(),
                );

                ui.add_space(4.0);

                // Message content
                ui.label(
                    egui::RichText::new(&msg.content)
                        .size(14.0)
                        .color(egui::Color32::from_rgb(220, 220, 220)),
                );

                ui.add_space(12.0);
            });

            ui.add_space(16.0);
        });
    }

    fn render_input(&mut self, ui: &mut egui::Ui) {
        ui.add_space(8.0);
        ui.separator();
        ui.add_space(8.0);

        ui.horizontal(|ui| {
            ui.add_space(16.0);

            ui.vertical(|ui| {
                // Input box
                let response = ui.add(
                    egui::TextEdit::multiline(&mut self.input_text)
                        .desired_width(ui.available_width())
                        .desired_rows(3)
                        .hint_text("Ask anything...")
                        .frame(true),
                );

                ui.add_space(8.0);

                // Send button
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let send_button = ui.add_enabled(
                            !self.input_text.trim().is_empty() && !self.is_processing,
                            egui::Button::new(
                                egui::RichText::new("Send")
                                    .size(14.0)
                                    .color(egui::Color32::WHITE),
                            )
                            .fill(egui::Color32::from_rgb(70, 130, 255)),
                        );

                        // Handle send
                        let should_send = send_button.clicked()
                            || (response.has_focus()
                                && ui.input(|i| i.key_pressed(egui::Key::Enter))
                                && !ui.input(|i| i.modifiers.shift));

                        if should_send && !self.input_text.trim().is_empty() {
                            let user_message = self.input_text.trim().to_string();
                            self.messages.push(ChatMessage {
                                role: "user".to_string(),
                                content: user_message.clone(),
                            });
                            self.input_text.clear();
                            self.is_processing = true;

                            // Simulate response (you can integrate with planner_client here)
                            self.messages.push(ChatMessage {
                                role: "assistant".to_string(),
                                content: format!("I received your message: '{}'. I'm working on a response!", user_message),
                            });
                            self.is_processing = false;
                        }
                    });
                });
            });

            ui.add_space(16.0);
        });

        ui.add_space(16.0);
    }
}
