use eframe::egui;
use crate::chat::ChatPanel;
use crate::planner_client::PlannerClient;

pub struct SentinelApp {
    chat_panel: ChatPanel,
    planner_client: PlannerClient,
}

impl SentinelApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        Self {
            chat_panel: ChatPanel::new(),
            planner_client: PlannerClient::new("http://localhost:8000".to_string()),
        }
    }
}

impl eframe::App for SentinelApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Clean, minimal styling like Cursor/Comet
        let mut style = (*ctx.style()).clone();
        style.visuals.window_fill = egui::Color32::from_rgb(24, 24, 27); // Dark background
        style.visuals.panel_fill = egui::Color32::from_rgb(24, 24, 27);
        ctx.set_style(style);

        egui::CentralPanel::default()
            .frame(egui::Frame::none()
                .fill(egui::Color32::from_rgb(24, 24, 27)))
            .show(ctx, |ui| {
                // Header with minimal design
                ui.horizontal(|ui| {
                    ui.add_space(12.0);
                    ui.heading(egui::RichText::new("🤖 Sentinel AI")
                        .size(16.0)
                        .color(egui::Color32::from_rgb(255, 255, 255)));
                    
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        ui.add_space(12.0);
                        // Settings button
                        if ui.button("⚙").clicked() {
                            // Settings menu
                        }
                    });
                });
                
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(12.0);

                // Chat area - main content
                egui::ScrollArea::vertical()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| {
                        self.chat_panel.show(ui, &self.planner_client);
                    });
            });
    }
}
