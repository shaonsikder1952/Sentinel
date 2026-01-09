/**
 * Sentinel Native Desktop Overlay App
 * 
 * Right 20% overlay: Chat, task list, approvals, scheduling
 * Left 80% workspace: AI executes tasks autonomously
 */
use eframe::egui;
use sentinel_overlay::SentinelApp;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Sentinel AI Overlay")
            .with_always_on_top()
            .with_decorations(false) // No window decorations for overlay
            .with_transparent(true)  // Transparent background
            .with_resizable(true)
            .with_position(egui::Pos2::new(0.0, 0.0)), // Will be positioned to right 20%
            // .with_icon(icon), // Add icon if needed
        ..Default::default()
    };

    eframe::run_native(
        "Sentinel AI Overlay",
        options,
        Box::new(|_cc| {
            Box::new(SentinelApp::default())
        }),
    )
}

