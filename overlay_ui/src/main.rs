use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    // Get screen size
    let screen_width = 1920.0; // Will be auto-detected
    let screen_height = 1080.0;

    // Calculate 20% width panel on right edge
    let panel_width = screen_width * 0.2;
    let x_position = screen_width - panel_width;

    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_title("Sentinel AI")
            .with_inner_size([panel_width, screen_height])
            .with_position([x_position, 0.0])
            .with_always_on_top()
            .with_resizable(true)
            .with_decorations(true)
            .with_transparent(false),
        ..Default::default()
    };

    eframe::run_native(
        "Sentinel AI",
        options,
        Box::new(|cc| Box::new(sentinel_overlay::app::SentinelApp::new(cc))),
    )
}
