# Sentinel AI Platform - Architecture Overview (Native Desktop)

## System Architecture

```
┌─────────────────────────────────────────────────────────────┐
│              Native Desktop Overlay (egui)                  │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Left 80% - AI Workspace (hidden/semi-transparent) │   │
│  │  AI executes tasks autonomously                     │   │
│  └─────────────────────────────────────────────────────┘   │
│  ┌─────────────────────────────────────────────────────┐   │
│  │  Right 20% - Overlay Sidebar                        │   │
│  │  ┌──────────┐  ┌──────────┐  ┌──────────┐          │   │
│  │  │   Chat   │  │   Task   │  │ Approval│          │   │
│  │  │          │  │   List   │  │ Controls│          │   │
│  │  └──────────┘  └──────────┘  └──────────┘          │   │
│  └─────────────────────────────────────────────────────┘   │
└─────────────────────────────────────────────────────────────┘
                          │
                          │ IPC / HTTP
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                    Rust Engine (Core)                        │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │    Task      │  │    Step      │  │  Scheduler   │     │
│  │   Manager    │  │  Executor    │  │              │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │   Verifier   │  │   Memory     │  │  IPC Layer   │     │
│  │              │  │   Manager    │  │              │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
└─────────────────────────────────────────────────────────────┘
                          │
                          │ JSON-RPC / HTTP
                          ▼
┌─────────────────────────────────────────────────────────────┐
│                 Python Planner (Intelligence)                │
│  ┌──────────────┐  ┌──────────────┐  ┌──────────────┐     │
│  │    Task      │  │     Step     │  │   Planner   │     │
│  │  Detector    │  │  Generator   │  │     API     │     │
│  │   (NLP)      │  │              │  │             │     │
│  └──────────────┘  └──────────────┘  └──────────────┘     │
└─────────────────────────────────────────────────────────────┘
                          │
                          │ Automation APIs
                          ▼
┌─────────────────────────────────────────────────────────────┐
│              Automation Adapter                               │
│  ┌──────────────┐  ┌──────────────┐                        │
│  │   Browser    │  │   Desktop    │                        │
│  │ (Playwright) │  │   Apps       │                        │
│  │              │  │ (OS APIs)    │                        │
│  └──────────────┘  └──────────────┘                        │
└─────────────────────────────────────────────────────────────┘
```

## Key Design Decisions

### 1. Native Desktop Overlay
- **egui Framework**: Lightweight, Rust-native, cross-platform
- **Always-on-top**: Right 20% overlay, doesn't interfere
- **Transparent**: Semi-transparent background, resizable
- **Cross-platform**: Win32/AppKit/X11 window management

### 2. Separation of Concerns
- **80% AI Workspace**: Hidden or semi-transparent, AI executes autonomously
- **20% Overlay**: User controls, approvals, monitoring
- **No Interference**: AI workspace doesn't block user's work

### 3. Automation Abstraction
- **Browser Automation**: Playwright for web apps
- **Desktop Automation**: OS-specific APIs (UI Automation, Accessibility)
- **Unified Interface**: `AutomationTarget` trait for both

### 4. Deterministic Execution
- **Contract-based steps**: Each step has expected schema
- **Verification at every step**: Schema, sanity, element presence
- **Retry logic**: Configurable max retries with delays
- **Append-only logging**: Full audit trail, replayable

### 5. Multi-Layer Memory
- **Task Memory**: Per-task context, execution logs
- **Project Memory**: Recurring patterns, automation preferences
- **System Memory**: Cross-project knowledge, verified selectors

## Data Flow

### Task Creation Flow

1. **User Input** → Chat in overlay (right 20%)
2. **Planner** → Parses command, generates workflow
3. **Task Manager** → Creates task with approval flags
4. **Overlay UI** → Shows task preview with [Approve] [Edit] [Cancel]
5. **Approval** → User approves → Task status: `approved`
6. **Execution** → AI workspace (left 80%) → Steps executed
7. **Verification** → Each step verified
8. **Post-Approval** → User reviews output in overlay → Final approval

### Scheduling Flow

1. **Task Created** → With scheduling info
2. **Scheduler** → Registers task, calculates next_run
3. **Time Trigger** → Scheduler checks every minute
4. **Auto-Start** → If repetitive & auto-approved → Start
5. **Manual Start** → If needs approval → Show in overlay
6. **Recurrence** → Calculate next_run → Re-register

## Component Responsibilities

### Native Overlay UI (Rust/egui)
- **App**: Main application state and rendering
- **Sidebar**: Approval controls, task management
- **Task List**: Display all tasks with status
- **Chat**: Natural language command input
- **Window Manager**: Cross-platform overlay positioning
- **Planner Client**: HTTP client for planner API
- **Automation Adapter**: Browser and desktop app automation

### Rust Engine
- **Task Manager**: Lifecycle, approval, state transitions
- **Step Executor**: Atomic actions via automation trait
- **Verifier**: Schema, sanity, element presence checks
- **Memory Manager**: Multi-layer storage, versioning
- **Scheduler**: Time-based triggering, recurrence calculation
- **IPC Layer**: JSON-RPC communication interface

### Python Planner
- **Task Detector**: NLP parsing, behavior pattern detection
- **Step Generator**: Workflow generation from task descriptions
- **Planner API**: HTTP/WebSocket JSON-RPC server

## Cross-Platform Window Management

### Windows
- **Win32 APIs**: `SetWindowPos`, `HWND_TOPMOST`
- Always-on-top window positioning

### macOS
- **AppKit/Cocoa**: `NSWindowCollectionBehavior`
- Window positioning and always-on-top

### Linux
- **X11/Wayland**: Window manager protocols
- Always-on-top support via window manager

## Security & Safety

1. **Approval Gates**: Pre and post-execution approval
2. **Safety Rules**: System-level rules (domain restrictions, rate limits)
3. **Verification**: Every step verified before proceeding
4. **Human Override**: Always can pause/stop/resume via overlay
5. **Audit Trail**: Full logging for compliance

## Performance Considerations

1. **Lightweight Overlay**: egui is efficient, minimal resource usage
2. **Parallel Execution**: Multiple tasks can run simultaneously
3. **Memory Caching**: In-memory + disk persistence
4. **Efficient Scheduling**: Minute-based checks, not per-second
5. **Lazy Loading**: Project/system memory loaded on demand

## Future Extensions

1. **Desktop App Automation**: Full OS automation support
2. **API Adapters**: REST/GraphQL for external systems
3. **Multi-Workflow**: Parallel workflows per task
4. **Advanced AI**: GPT-4 integration for better planning
5. **Cloud Sync**: Memory layers synced across devices
6. **Multi-Monitor**: Overlay positioning across multiple displays
