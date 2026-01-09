/**
 * Window Manager: Cross-platform overlay positioning and management
 */
use eframe::egui;
use eframe::Frame;

pub struct WindowManager {
    overlay_width_ratio: f32,
}

impl WindowManager {
    pub fn new() -> Self {
        Self {
            overlay_width_ratio: 0.2, // 20% of screen width
        }
    }

    pub fn position_overlay(&self, ctx: &egui::Context, _frame: &mut Frame) {
        // Window positioning is handled by eframe::NativeOptions in main.rs
        // This function is kept for future window management features
        let _screen_size = ctx.screen_rect().size();
    }

    pub fn update_overlay_position(&self, _ctx: &egui::Context, _frame: &mut Frame) {
        // Keep overlay always on top and positioned correctly
        // This is called every frame to ensure proper positioning
        // Window positioning is handled by eframe::NativeOptions
    }
}

#[cfg(target_os = "windows")]
mod windows {
    use winapi::um::winuser::{SetWindowPos, HWND_TOPMOST, SWP_NOMOVE, SWP_NOSIZE};

    pub fn set_always_on_top(hwnd: *mut std::ffi::c_void) {
        unsafe {
            SetWindowPos(
                hwnd as _,
                HWND_TOPMOST,
                0, 0, 0, 0,
                SWP_NOMOVE | SWP_NOSIZE,
            );
        }
    }
}

#[cfg(target_os = "macos")]
mod macos {
    use cocoa::appkit::{NSWindow, NSWindowCollectionBehavior};
    use cocoa::base::id;

    pub fn set_always_on_top(window: id) {
        unsafe {
            window.setCollectionBehavior_(
                NSWindowCollectionBehavior::NSWindowCollectionBehaviorCanJoinAllSpaces
                    | NSWindowCollectionBehavior::NSWindowCollectionBehaviorStationary
            );
        }
    }
}

#[cfg(target_os = "linux")]
mod linux {
    // X11/Wayland window management
    // Implementation would use x11 or wayland APIs
}

