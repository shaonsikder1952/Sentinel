# Quick Start Guide - Running Sentinel AI Platform

## Prerequisites Check

Before running, ensure you have:

1. **Rust** (for engine and overlay)
   ```bash
   # Install Rust if not installed:
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   source $HOME/.cargo/env
   ```

2. **Python 3.9+** (for planner)
   ```bash
   python3 --version  # Should show 3.9 or higher
   ```

3. **Git** (if cloning from repo)

## Step-by-Step Setup

### Step 1: Setup Python Planner

```bash
cd planner

# Create virtual environment
python3 -m venv venv

# Activate virtual environment
# On macOS/Linux:
source venv/bin/activate
# On Windows:
# venv\Scripts\activate

# Install dependencies
pip install -r requirements.txt

# Start the planner API (runs on port 8000)
python planner_api.py
```

**Keep this terminal open!** The planner API must be running.

### Step 2: Build Rust Engine (Optional - can run as service)

Open a **new terminal**:

```bash
cd engine

# Build the engine
cargo build --release

# (Optional) Run engine as separate service
# ./target/release/sentinel-engine
```

### Step 3: Build and Run Native Overlay

Open a **new terminal**:

```bash
cd overlay_ui

# Build the overlay (first time will take a few minutes)
cargo build --release

# Run the overlay
cargo run --release
```

## What You Should See

1. **Planner API Terminal**: 
   - Should show: `INFO:     Uvicorn running on http://0.0.0.0:8000`
   - API is ready to accept requests

2. **Overlay Window**:
   - A window should appear on the **right 20% of your screen**
   - Always-on-top overlay with:
     - **Chat panel** at the top
     - **Task list** in the middle
     - **Task controls** at the bottom

## Testing the Application

### Test 1: Create a Task via Chat

1. In the overlay, type in the chat input:
   ```
   Do weekly KPI report
   ```

2. Press Enter or click "Send"

3. You should see:
   - Your message in the chat
   - AI response: "✅ Task created: Weekly KPI Report"
   - Task appears in the task list

### Test 2: Approve and Start Task

1. Click on the task in the task list
2. Click "✅ Approve" button
3. Click "▶️ Start Task" button
4. Task status should change to "InProgress"

### Test 3: Schedule a Task

Type in chat:
```
Schedule monthly revenue report every Monday at 9:00 AM
```

The system will:
- Parse the scheduling information
- Create a task with recurrence
- Show scheduling info in task controls

## Troubleshooting

### Issue: "cargo: command not found"

**Solution**: Install Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

### Issue: Overlay doesn't appear

**Check**:
1. Is the planner API running? (http://localhost:8000)
2. Check terminal for error messages
3. Try running with `RUST_BACKTRACE=1 cargo run --release` for debug info

### Issue: "Connection refused" errors

**Solution**: Make sure Python planner is running first:
```bash
cd planner
source venv/bin/activate
python planner_api.py
```

### Issue: Build errors in Rust

**Common fixes**:
```bash
# Update Rust
rustup update

# Clean and rebuild
cargo clean
cargo build --release
```

### Issue: Python import errors

**Solution**: Reinstall dependencies:
```bash
cd planner
source venv/bin/activate
pip install --upgrade -r requirements.txt
```

## Using the Quick Start Script

Alternatively, use the provided script:

```bash
./start.sh
```

This will:
1. Start Python planner
2. Build Rust engine (if needed)
3. Build and launch overlay

Press `Ctrl+C` to stop all services.

## Development Mode

For development with hot-reload:

```bash
# Terminal 1: Planner
cd planner
source venv/bin/activate
uvicorn planner_api:app --reload

# Terminal 2: Overlay (with auto-rebuild)
cd overlay_ui
cargo watch -x "run --release"
```

## Next Steps

1. **Explore the UI**: Try different chat commands
2. **Check Documentation**: See `README.md` and `ARCHITECTURE.md`
3. **Customize**: Modify overlay appearance in `overlay_ui/src/app.rs`
4. **Extend**: Add new automation adapters in `overlay_ui/src/automation_adapter.rs`

## Project Structure

```
Sentinel/
├── engine/          # Rust core engine
├── planner/        # Python planner API
├── overlay_ui/      # Native Rust overlay
├── shared_schemas/  # JSON schemas
└── logs/           # Execution logs
```

## Need Help?

- Check `README.md` for detailed documentation
- See `ARCHITECTURE.md` for system design
- Review `NATIVE_OVERLAY.md` for overlay-specific details

