/**
 * Main application state and UI rendering
 */
use eframe::egui;
use crate::sidebar::Sidebar;
use crate::task_list::TaskList;
use crate::chat::ChatPanel;
use crate::window_manager::WindowManager;
use sentinel_engine::{TaskManager, MemoryManager, Scheduler};
use std::sync::Arc;

pub struct SentinelApp {
    sidebar: Sidebar,
    task_list: TaskList,
    chat: ChatPanel,
    window_manager: WindowManager,
    
    // Engine components
    task_manager: Option<Arc<TaskManager>>,
    memory_manager: Option<Arc<MemoryManager>>,
    scheduler: Option<Arc<Scheduler>>,
    
    // State
    initialized: bool,
    planner_client: crate::planner_client::PlannerClient,
}

impl Default for SentinelApp {
    fn default() -> Self {
        Self {
            sidebar: Sidebar::default(),
            task_list: TaskList::default(),
            chat: ChatPanel::default(),
            window_manager: WindowManager::new(),
            task_manager: None,
            memory_manager: None,
            scheduler: None,
            initialized: false,
            planner_client: crate::planner_client::PlannerClient::new("http://localhost:8000".to_string()),
        }
    }
}

impl eframe::App for SentinelApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Initialize on first frame
        if !self.initialized {
            self.initialize(ctx, frame);
            self.initialized = true;
        }

        // Position overlay to right 20% of screen
        self.window_manager.update_overlay_position(ctx, frame);

        // Main UI
        egui::CentralPanel::default()
            .show(ctx, |ui| {
                ui.horizontal(|ui| {
                    // Left 80% - AI Workspace (can be hidden or semi-transparent)
                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(ui.available_width() * 0.8, ui.available_height()),
                        egui::Layout::left_to_right(egui::Align::Center),
                        |ui| {
                            self.render_ai_workspace(ui);
                        },
                    );

                    // Right 20% - Overlay Sidebar
                    ui.allocate_ui_with_layout(
                        egui::Vec2::new(ui.available_width() * 0.2, ui.available_height()),
                        egui::Layout::right_to_left(egui::Align::Min),
                        |ui| {
                            self.render_overlay(ui);
                        },
                    );
                });
            });
    }
}

impl SentinelApp {
    fn initialize(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        // Initialize engine components
        let memory_manager = Arc::new(
            MemoryManager::new("./storage").expect("Failed to create memory manager")
        );
        let task_manager = Arc::new(TaskManager::new(memory_manager.clone()));
        let scheduler = Arc::new(Scheduler::new(task_manager.clone()));

        self.memory_manager = Some(memory_manager);
        self.task_manager = Some(task_manager);
        self.scheduler = Some(scheduler);

//         // Start scheduler in background
//         let scheduler_clone = self.scheduler.clone().unwrap();
//         tokio::spawn(async move {
//             if let Err(e) = scheduler_clone.start_scheduler_loop().await {
//                 eprintln!("Scheduler error: {}", e);
//             }
//         });
// 
//         // Position window to right 20%
        self.window_manager.position_overlay(ctx, frame);
    }

    fn render_ai_workspace(&mut self, ui: &mut egui::Ui) {
        // AI workspace - can be hidden or show status
        ui.vertical_centered(|ui| {
            ui.heading("AI Workspace");
            ui.separator();
            ui.label("AI tasks execute here in the background");
            ui.label("This area can be hidden or made semi-transparent");
            
            if let Some(task_manager) = &self.task_manager {
                let tasks = task_manager.get_all_tasks();
                if !tasks.is_empty() {
                    ui.separator();
                    ui.label(format!("Active tasks: {}", tasks.len()));
                }
            }
        });
    }

    fn render_overlay(&mut self, ui: &mut egui::Ui) {
        // Right 20% overlay panel
        egui::Frame::none()
            .fill(egui::Color32::from_rgba_unmultiplied(30, 30, 30, 240)) // Semi-transparent dark
            .inner_margin(egui::Margin::same(10.0))
            .show(ui, |ui| {
                ui.vertical(|ui| {
                    // Header
                    ui.horizontal(|ui| {
                        ui.heading("🤖 Sentinel AI");
                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if ui.button("⚙️").clicked() {
                                // Settings
                            }
                        });
                    });
                    ui.separator();

                    // Chat panel
                    self.chat.ui(ui, &mut self.planner_client, self.task_manager.as_ref());

                    ui.separator();

                    // Task list
                    self.task_list.ui(ui, self.task_manager.as_ref(), self.scheduler.as_ref());

                    ui.separator();

                    // Sidebar controls
                    self.sidebar.ui(ui, self.task_manager.as_ref());
                });
            });
    }
}

