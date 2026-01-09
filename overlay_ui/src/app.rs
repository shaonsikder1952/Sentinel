use eframe::egui;
use crate::planner_client::PlannerClient;

#[derive(serde::Deserialize, serde::Serialize)]
pub struct ChatMessage {
    pub role: String,
    pub content: String,
}

pub struct SentinelApp {
    messages: Vec<ChatMessage>,
    input_text: String,
    planner_client: PlannerClient,
    is_processing: bool,
    scroll_to_bottom: bool,
    animate_typing: bool,
    typing_dots: usize,
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
            scroll_to_bottom: true,
            animate_typing: false,
            typing_dots: 0,
        }
    }
}

impl eframe::App for SentinelApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Animation for typing indicator
        if self.animate_typing {
            ctx.request_repaint();
            self.typing_dots = (self.typing_dots + 1) % 4;
        }

        // Professional styling
        let mut style = (*ctx.style()).clone();
        
        // Modern font sizes with proper hierarchy
        style.text_styles = [
            (egui::TextStyle::Body, egui::FontId::proportional(15.0)),
            (egui::TextStyle::Button, egui::FontId::proportional(14.0)),
            (egui::TextStyle::Heading, egui::FontId::proportional(20.0)),
            (egui::TextStyle::Monospace, egui::FontId::monospace(14.0)),
            (egui::TextStyle::Small, egui::FontId::proportional(12.0)),
        ]
        .into();
        
        // Professional spacing following 8px grid
        style.spacing.item_spacing = egui::vec2(8.0, 8.0);
        style.spacing.window_margin = egui::Margin::same(0.0);
        style.spacing.button_padding = egui::vec2(12.0, 8.0);
        
        ctx.set_style(style);

        // Main panel with professional background
        egui::CentralPanel::default()
            .frame(
                egui::Frame::none()
                    .fill(egui::Color32::from_rgb(250, 250, 250))
            )
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    // Professional header with subtle shadow
                    egui::Frame::none()
                        .fill(egui::Color32::WHITE)
                        .inner_margin(egui::Margin::symmetric(20.0, 16.0))
                        .shadow(egui::epaint::Shadow {
                            offset: egui::vec2(0.0, 1.0),
                            blur: 4.0,
                            spread: 0.0,
                            color: egui::Color32::from_black_alpha(10),
                        })
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 12.0;
                                
                                // Professional logo/icon area
                                ui.label(
                                    egui::RichText::new("💬")
                                        .size(20.0)
                                );
                                
                                ui.label(
                                    egui::RichText::new("Sentinel AI")
                                        .size(16.0)
                                        .color(egui::Color32::from_rgb(30, 30, 30))
                                        .strong()
                                );
                                
                                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                                    // Professional settings icon
                                    if ui.add(
                                        egui::Button::new("⚙")
                                            .frame(false)
                                            .fill(egui::Color32::TRANSPARENT)
                                    ).on_hover_text("Settings").clicked() {
                                        // Settings action
                                    }
                                });
                            });
                        });

                    // Chat messages area with proper padding
                    egui::ScrollArea::vertical()
                        .auto_shrink([false, false])
                        .stick_to_bottom(self.scroll_to_bottom)
                        .show(ui, |ui| {
                            ui.add_space(20.0);
                            
                            for message in &self.messages {
                                self.render_message(ui, message);
                                ui.add_space(16.0);
                            }
                            
                            // Typing indicator with animation
                            if self.animate_typing {
                                self.render_typing_indicator(ui);
                                ui.add_space(16.0);
                            }
                            
                            ui.add_space(20.0);
                            
                            if self.scroll_to_bottom {
                                ui.scroll_to_cursor(Some(egui::Align::BOTTOM));
                                self.scroll_to_bottom = false;
                            }
                        });

                    // Professional input area with shadow
                    egui::Frame::none()
                        .fill(egui::Color32::WHITE)
                        .inner_margin(egui::Margin::symmetric(20.0, 16.0))
                        .shadow(egui::epaint::Shadow {
                            offset: egui::vec2(0.0, -1.0),
                            blur: 6.0,
                            spread: 0.0,
                            color: egui::Color32::from_black_alpha(12),
                        })
                        .show(ui, |ui| {
                            ui.horizontal(|ui| {
                                ui.spacing_mut().item_spacing.x = 12.0;
                                
                                // Professional text input with proper styling
                                let text_edit = egui::TextEdit::multiline(&mut self.input_text)
                                    .desired_width(f32::INFINITY)
                                    .desired_rows(1)
                                    .hint_text("Ask anything...")
                                    .frame(true)
                                    .margin(egui::vec2(12.0, 10.0));
                                
                                let response = ui.add(text_edit);
                                
                                // Handle Enter key
                                if response.lost_focus() && ui.input(|i| i.key_pressed(egui::Key::Enter)) {
                                    if !self.input_text.trim().is_empty() && !self.is_processing {
                                        self.send_message();
                                    }
                                }
                                
                                // Professional send button with hover effects
                                let send_button = egui::Button::new(
                                    egui::RichText::new("➤")
                                        .size(18.0)
                                        .color(egui::Color32::WHITE)
                                )
                                .fill(if self.input_text.trim().is_empty() || self.is_processing {
                                    egui::Color32::from_rgb(180, 180, 180)
                                } else {
                                    egui::Color32::from_rgb(59, 130, 246)
                                })
                                .min_size(egui::vec2(48.0, 48.0))
                                .rounding(24.0);
                                
                                if ui.add(send_button).clicked() && !self.input_text.trim().is_empty() && !self.is_processing {
                                    self.send_message();
                                }
                            });
                        });
                });
            });
    }
}

impl SentinelApp {
    fn render_message(&self, ui: &mut egui::Ui, message: &ChatMessage) {
        let is_user = message.role == "user";
        
        ui.horizontal(|ui| {
            if is_user {
                ui.add_space(ui.available_width() * 0.15);
            }
            
            ui.vertical(|ui| {
                ui.set_max_width(ui.available_width() * 0.75);
                
                // Professional message bubble
                egui::Frame::none()
                    .fill(if is_user {
                        egui::Color32::from_rgb(59, 130, 246) // Modern blue for user
                    } else {
                        egui::Color32::WHITE
                    })
                    .inner_margin(egui::Margin::symmetric(16.0, 12.0))
                    .rounding(16.0)
                    .shadow(egui::epaint::Shadow {
                        offset: egui::vec2(0.0, 1.0),
                        blur: 3.0,
                        spread: 0.0,
                        color: egui::Color32::from_black_alpha(8),
                    })
                    .show(ui, |ui| {
                        ui.label(
                            egui::RichText::new(&message.content)
                                .color(if is_user {
                                    egui::Color32::WHITE
                                } else {
                                    egui::Color32::from_rgb(30, 30, 30)
                                })
                                .size(15.0)
                        );
                    });
            });
            
            if !is_user {
                ui.add_space(ui.available_width());
            }
        });
    }
    
    fn render_typing_indicator(&self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_max_width(ui.available_width() * 0.75);
                
                egui::Frame::none()
                    .fill(egui::Color32::WHITE)
                    .inner_margin(egui::Margin::symmetric(16.0, 12.0))
                    .rounding(16.0)
                    .shadow(egui::epaint::Shadow {
                        offset: egui::vec2(0.0, 1.0),
                        blur: 3.0,
                        spread: 0.0,
                        color: egui::Color32::from_black_alpha(8),
                    })
                    .show(ui, |ui| {
                        ui.horizontal(|ui| {
                            ui.spacing_mut().item_spacing.x = 4.0;
                            for i in 0..3 {
                                let alpha = if i < self.typing_dots { 255 } else { 100 };
                                ui.label(
                                    egui::RichText::new("●")
                                        .color(egui::Color32::from_gray(alpha))
                                        .size(12.0)
                                );
                            }
                        });
                    });
            });
        });
    }

    fn send_message(&mut self) {
        let user_message = self.input_text.trim().to_string();
        self.input_text.clear();
        
        self.messages.push(ChatMessage {
            role: "user".to_string(),
            content: user_message.clone(),
        });
        
        self.is_processing = true;
        self.animate_typing = true;
        self.scroll_to_bottom = true;
        
        let client = self.planner_client.clone();
        let messages = self.messages.clone();
        
        std::thread::spawn(move || {
            if let Ok(response) = client.send_chat_message(&messages) {
                // Response will be handled in main thread
            }
        });
    }
}