# Sentinel AI Platform - Native Desktop Overlay

A **native desktop AI overlay app** that allows AI to work alongside humans on any application, with a **right 20% overlay** for controls and an **80% AI workspace** for autonomous task execution.

## Architecture

### Core Components

1. **Rust Engine** (`/engine`) - Deterministic execution engine
   - Task Manager: Task lifecycle, approval flows, scheduling
   - Step Executor: Atomic action execution via automation APIs
   - Verifier: Step-by-step validation
   - Memory Manager: Multi-layer memory (task/project/system)
   - Scheduler: Automated task triggering
   - IPC Layer: Communication interface

2. **Python Planner** (`/planner`) - Intelligence & NLP layer
   - Task Detector: Natural language parsing
   - Step Generator: Workflow generation from tasks
   - Planner API: JSON-RPC over WebSocket/HTTP

3. **Native Overlay UI** (`/overlay_ui`) - Rust egui desktop app
   - **Right 20% Overlay**: Chat, task list, approvals, scheduling
   - **Left 80% Workspace**: AI executes tasks autonomously
   - Window Manager: Cross-platform overlay positioning
   - Automation Adapter: Browser and desktop app automation

4. **Shared Schemas** (`/shared_schemas`) - Versioned contracts
   - Workflow schema
   - Task memory schema
   - Project memory schema
   - System memory schema

## Features

### ✅ Native Desktop Overlay
- **Always-on-top**: Right 20% overlay, resizable
- **Transparent background**: Doesn't interfere with user's work
- **Cross-platform**: Windows, macOS, Linux support

### ✅ Task Creation & Detection
- **Natural Language Chat**: Type commands like "Do weekly KPI report"
- **Auto-Detection**: AI monitors repetitive behavior and suggests tasks
- **Manual Creation**: Form-based task creation with scheduling options

### ✅ Approval Flow
- **Pre-Execution Approval**: Required before task starts (configurable)
- **Post-Execution Approval**: Required before task finalization
- **Auto-Approval**: For repetitive tasks after initial approval

### ✅ Scheduling & Automation
- **One-time Scheduling**: Run task at specific date/time
- **Recurring Tasks**: Daily, weekly, monthly with time specification
- **Repetitive Automation**: Auto-run after approval without pre-approval

### ✅ Deterministic Execution
- **Zero Hallucinations**: Contract-based step execution
- **Verification**: Schema, sanity checks, element presence
- **Retry Logic**: Configurable retries with delays
- **Full Logging**: Append-only, replayable workflows

### ✅ Multi-Layer Memory
- **Task Memory**: Short-term, per-task context
- **Project Memory**: Medium-term, recurring rules, workflow history
- **System Memory**: Long-term, app schemas, verified selectors, safety rules

## Setup

### Prerequisites
- Rust (latest stable)
- Python 3.9+
- Playwright browsers installed (for browser automation)

### Installation

1. **Rust Engine**
```bash
cd engine
cargo build --release
```

2. **Python Planner**
```bash
cd planner
python3 -m venv venv
source venv/bin/activate  # On Windows: venv\Scripts\activate
pip install -r requirements.txt
python planner_api.py
```

3. **Native Overlay UI**
```bash
cd overlay_ui
cargo build --release
cargo run --release
```

## Usage

### Starting the Platform

1. Start Python Planner API:
```bash
cd planner
python planner_api.py
```

2. Start Rust Engine (optional, can run as service):
```bash
cd engine
cargo run --release
```

3. Start Native Overlay:
```bash
cd overlay_ui
cargo run --release
```

The overlay will appear on the right 20% of your screen, always on top.

### Creating a Task via Chat

1. Type in the chat input: `"Do weekly KPI report"`
2. AI parses command and creates task
3. Review and approve in sidebar
4. Task executes in AI workspace (left 80%)

### Scheduling a Task

Type: `"Schedule monthly revenue report every Monday at 9:00 AM"`

The system will:
- Parse scheduling information
- Create task with recurrence
- Auto-trigger at specified times

### Repetitive Task Automation

1. Create and approve a task
2. Mark as repetitive
3. After first successful run, enable auto-run
4. Task runs automatically without pre-approval

## Workflow

```
Chat Command (Overlay)
    ↓
Planner (Python) - Detects task, generates workflow
    ↓
Task Manager (Rust) - Creates task, checks approval
    ↓
Pre-Approval Check (Overlay UI)
    ↓
AI Workspace (80% left) - Task execution
    ↓
Environment Mapping (DOM/UI tree)
    ↓
Step Execution (Automation APIs)
    ↓
Verification (Schema, sanity checks)
    ↓
Post-Execution Approval (Overlay UI)
    ↓
Task Completed / Scheduled for next run
```

## File Structure

```
/proto_ai_native_app
├─ engine/                # Rust core engine
│   ├─ src/
│   │   ├─ task_manager.rs
│   │   ├─ step_executor.rs
│   │   ├─ verifier.rs
│   │   ├─ memory_manager.rs
│   │   ├─ scheduler.rs
│   │   ├─ ipc.rs
│   │   └─ types.rs
│   └─ Cargo.toml
├─ planner/               # Python intelligence & NLP
│   ├─ task_detector.py
│   ├─ step_generator.py
│   ├─ planner_api.py
│   └─ requirements.txt
├─ overlay_ui/            # Native Rust overlay (egui)
│   ├─ src/
│   │   ├─ main.rs
│   │   ├─ app.rs
│   │   ├─ sidebar.rs
│   │   ├─ task_list.rs
│   │   ├─ chat.rs
│   │   ├─ window_manager.rs
│   │   ├─ planner_client.rs
│   │   └─ automation_adapter.rs
│   └─ Cargo.toml
├─ shared_schemas/        # Versioned JSON schemas
│   ├─ workflow_schema.json
│   ├─ task_memory_schema.json
│   ├─ project_memory_schema.json
│   └─ system_memory_schema.json
└─ logs/                  # Execution logs
```

## Cross-Platform Support

- **Windows**: Win32 APIs for window management
- **macOS**: AppKit/Cocoa for window management
- **Linux**: X11/Wayland for window management

## Future Extensions

- Desktop app automation (beyond browsers)
- Multiple workflow support
- API adapters for external integrations
- Advanced AI intelligence upgrades
- Cloud sync for memory layers
- Multi-monitor support

## License

MIT
