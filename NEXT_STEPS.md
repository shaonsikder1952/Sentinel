# Next Steps - Getting Sentinel Running

## Step 1: Build Everything ✅ (You're Here)

Make sure everything compiles:

```bash
cd overlay_ui
cargo build --release
```

If there are any remaining errors, fix them first.

## Step 2: Start the Python Planner API

Open **Terminal 1**:

```bash
cd planner
python3 -m venv venv
source venv/bin/activate
pip install -r requirements.txt
python planner_api.py
```

You should see:
```
INFO:     Uvicorn running on http://0.0.0.0:8000
```

**Keep this terminal open!**

## Step 3: Run the Native Overlay

Open **Terminal 2**:

```bash
cd overlay_ui
cargo run --release
```

The overlay window should appear on the right 20% of your screen.

## Step 4: Test Basic Functionality

### Test 1: Create a Task via Chat
1. In the overlay, type in the chat input:
   ```
   Do weekly KPI report
   ```
2. Press Enter or click "Send"
3. You should see:
   - Your message in chat
   - AI response: "✅ Task created: Weekly KPI Report"
   - Task appears in task list

### Test 2: Approve Task
1. Click on the task in the task list
2. Click "✅ Approve" button
3. Task status should change

### Test 3: Schedule a Task
Type in chat:
```
Schedule monthly revenue report every Monday at 9:00 AM
```

## Step 5: What to Implement Next

### High Priority (Core Functionality)

1. **Fix Workflow Conversion** (if tasks don't execute)
   - The workflow JSON from planner needs proper conversion
   - Check `overlay_ui/src/chat.rs` workflow parsing

2. **Implement Step Executor Integration**
   - Connect the Rust engine's step executor to actual automation
   - Implement the `AutomationTarget` trait for browser automation

3. **Add Browser Automation**
   - Install and configure Playwright
   - Implement `BrowserAutomation` in `automation_adapter.rs`

4. **Fix Window Positioning**
   - The overlay should appear on right 20% automatically
   - May need to adjust `main.rs` window options

### Medium Priority (UX Improvements)

5. **Task Execution UI**
   - Show real-time step progress
   - Display execution logs
   - Show extracted data

6. **Better Error Handling**
   - Show user-friendly error messages
   - Handle planner API connection failures
   - Handle task creation failures

7. **Task Editing**
   - Allow users to modify tasks before approval
   - Edit scheduling options
   - Edit workflow steps

### Low Priority (Nice to Have)

8. **Settings/Configuration**
   - Overlay position preferences
   - Default approval settings
   - Theme customization

9. **Task History**
   - View past task executions
   - Replay tasks
   - Export logs

10. **Notifications**
    - System notifications for task completion
    - Task failure alerts

## Step 6: Debugging Tips

### If Overlay Doesn't Appear
- Check terminal for errors
- Try running with `RUST_BACKTRACE=1 cargo run --release`
- Check if window is off-screen (try moving/resizing)

### If Planner API Fails
- Check if port 8000 is available
- Verify Python dependencies are installed
- Check planner terminal for error messages

### If Tasks Don't Create
- Check browser console/terminal for errors
- Verify planner API is responding: `curl http://localhost:8000/api/v1/detect-task -X POST -H "Content-Type: application/json" -d '{"command":"test"}'`
- Check workflow conversion in chat.rs

## Step 7: Development Workflow

For active development:

**Terminal 1** - Planner (with auto-reload):
```bash
cd planner
source venv/bin/activate
uvicorn planner_api:app --reload
```

**Terminal 2** - Overlay (with watch):
```bash
cd overlay_ui
cargo watch -x "run --release"
```

## Current Status

✅ **Completed:**
- Project structure
- Rust engine (task manager, scheduler, memory)
- Python planner (NLP, workflow generation)
- Native overlay UI (chat, task list, approvals)
- Basic schemas and types

🚧 **In Progress:**
- Workflow execution
- Browser automation integration
- Window positioning

📋 **Todo:**
- Full end-to-end task execution
- Error handling and recovery
- UI polish and UX improvements

## Quick Commands Reference

```bash
# Build overlay
cd overlay_ui && cargo build --release

# Run overlay
cd overlay_ui && cargo run --release

# Start planner
cd planner && source venv/bin/activate && python planner_api.py

# Check planner health
curl http://localhost:8000/docs  # Should show FastAPI docs
```

## Need Help?

- Check `README.md` for architecture overview
- See `ARCHITECTURE.md` for system design
- Review `NATIVE_OVERLAY.md` for overlay-specific details
- Check `QUICK_START.md` for setup instructions

