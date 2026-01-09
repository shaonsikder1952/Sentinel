# Native Desktop Overlay - Implementation Guide

## Overview

The Sentinel AI Platform has been transformed into a **native desktop overlay app** using Rust and egui. The overlay provides:

- **Right 20%**: Always-on-top sidebar with chat, task list, and approval controls
- **Left 80%**: AI workspace for autonomous task execution
- **Cross-platform**: Windows, macOS, Linux support

## Architecture Transformation

### From Browser Extension to Native Overlay

**Before (Electron)**:
- Electron app with browser extension
- Browser tabs for AI workspace
- HTML/CSS/JS renderer

**After (Native Rust)**:
- Rust egui native app
- Always-on-top overlay window
- Direct integration with Rust engine
- Cross-platform window management

## Key Components

### 1. Overlay UI (`/overlay_ui`)

**Main App** (`app.rs`):
- Manages application state
- Renders left 80% AI workspace
- Renders right 20% overlay sidebar
- Initializes engine components

**Sidebar** (`sidebar.rs`):
- Approval controls (pre/post execution)
- Task status display
- Scheduling information
- Automation settings

**Task List** (`task_list.rs`):
- Displays all tasks
- Status indicators
- Task selection
- Scrollable list

**Chat Panel** (`chat.rs`):
- Natural language command input
- Message history
- Task creation from commands
- Integration with planner API

**Window Manager** (`window_manager.rs`):
- Cross-platform overlay positioning
- Always-on-top window management
- Right 20% screen positioning
- Resizable overlay

**Planner Client** (`planner_client.rs`):
- HTTP client for Python planner API
- Task detection from chat
- Workflow generation

**Automation Adapter** (`automation_adapter.rs`):
- Browser automation (Playwright)
- Desktop app automation (OS APIs)
- Unified `AutomationTarget` trait

## Building and Running

### Prerequisites

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Python 3.9+
python3 --version

# Install Playwright (for browser automation)
# This is handled by the automation adapter
```

### Build Steps

1. **Build Rust Engine**:
```bash
cd engine
cargo build --release
```

2. **Start Python Planner**:
```bash
cd planner
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
python planner_api.py
```

3. **Build and Run Overlay**:
```bash
cd overlay_ui
cargo build --release
cargo run --release
```

## Window Management

### Positioning

The overlay is positioned on the right 20% of the screen:

```rust
let screen_size = ctx.screen_rect().size();
let overlay_width = screen_size.x * 0.2; // 20%
let position = egui::Pos2::new(
    screen_size.x - overlay_width, // Right side
    0.0,
);
```

### Always-on-Top

Cross-platform implementation:

- **Windows**: `SetWindowPos` with `HWND_TOPMOST`
- **macOS**: `NSWindowCollectionBehavior`
- **Linux**: X11/Wayland window manager protocols

## UI Layout

```
┌─────────────────────────────────────────┐
│                                         │
│  Left 80% - AI Workspace                │
│  (Hidden or semi-transparent)          │
│                                         │
│  AI tasks execute here                  │
│                                         │
│                                         │
├─────────────────────────────────────────┤
│  Right 20% - Overlay Sidebar            │
│  ┌───────────────────────────────────┐ │
│  │ 🤖 Sentinel AI          [⚙️]     │ │
│  ├───────────────────────────────────┤ │
│  │ 💬 Chat                            │ │
│  │ [Command input...]        [Send]  │ │
│  ├───────────────────────────────────┤ │
│  │ Tasks                              │ │
│  │ ⏳ Weekly KPI Report               │ │
│  │ ✅ Monthly Revenue                 │ │
│  ├───────────────────────────────────┤ │
│  │ Task Controls                      │ │
│  │ [✅ Approve] [✏️ Edit] [❌ Cancel]│ │
│  └───────────────────────────────────┘ │
└─────────────────────────────────────────┘
```

## Integration Points

### Engine Integration

The overlay directly uses the Rust engine:

```rust
use sentinel_engine::{TaskManager, MemoryManager, Scheduler};

let memory_manager = Arc::new(MemoryManager::new("./storage")?);
let task_manager = Arc::new(TaskManager::new(memory_manager.clone()));
```

### Planner Integration

HTTP client communicates with Python planner:

```rust
let planner_client = PlannerClient::new("http://localhost:8000".to_string());
let detected = planner_client.detect_task_from_chat(command, None).await?;
```

### Automation Integration

Unified automation interface:

```rust
let browser = BrowserAutomation::new();
let desktop = DesktopAutomation::new();

// Both implement AutomationTarget trait
browser.click(selector).await?;
desktop.click(selector).await?;
```

## Future Enhancements

1. **Desktop App Automation**: Full OS automation support
2. **Multi-Monitor**: Overlay positioning across displays
3. **Themes**: Customizable overlay appearance
4. **Keyboard Shortcuts**: Quick task creation/approval
5. **Notifications**: System notifications for task completion
6. **Minimization**: Collapsible overlay to icon

## Troubleshooting

### Overlay Not Appearing

- Check window manager permissions (always-on-top)
- Verify screen resolution detection
- Check platform-specific window APIs

### Planner Connection Issues

- Ensure Python planner is running on port 8000
- Check firewall settings
- Verify HTTP client configuration

### Engine Initialization Errors

- Check storage directory permissions
- Verify Rust engine is built correctly
- Check memory manager initialization

## Performance

- **Lightweight**: egui is efficient, minimal resource usage
- **Native**: No browser overhead
- **Fast**: Direct Rust engine integration
- **Responsive**: 60 FPS UI updates

## Security

- **Local Storage**: All data stored locally
- **No Cloud**: No external data transmission (unless configured)
- **Approval Gates**: Human approval required for critical tasks
- **Audit Trail**: Full logging for compliance

