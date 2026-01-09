# Sentinel - AI Desktop Overlay Assistant

An intelligent desktop overlay application with AI chat, task management, and automation capabilities.

## Quick Start

### 1. Setup Environment
```bash
# Copy environment template
cp .env.example .env

# Edit .env and add your Groq API key
# Get free API key from: https://console.groq.com
```

### 2. Start Backend (Python)
```bash
cd planner
pip install -r requirements.txt
python api_server.py
```

### 3. Build Frontend (Rust) - NEEDS FIXES
```bash
cd overlay_ui
cargo build  # Currently has compilation errors
```

## Project Structure

```
Sentinel/
├── planner/              # Python backend with AI
│   ├── api_server.py
│   └── requirements.txt
├── overlay_ui/           # Rust GUI overlay
│   ├── Cargo.toml
│   └── src/
├── .env                  # Your API keys (not in git)
├── .env.example          # Template
└── FINAL_STATUS.md       # Detailed status report
```

## Features

- **AI Chat Interface**: Powered by Groq AI (Mixtral model)
- **Task Management**: Create, approve, and execute tasks
- **Desktop Overlay**: 80/20 split (workspace + control panel)
- **Automation**: Schedule and repeat tasks
- **Memory System**: Multi-layer task/project/system memory

## Current Status

✅ **Complete:**
- Python backend with Groq AI integration
- Environment configuration
- Rust project structure
- All dependencies specified

⚠️ **Needs Attention:**
- Rust compilation errors in overlay_ui/src/app.rs
- Errors around egui API usage (line ~401)

## Next Steps

1. Fix Rust compilation errors (see FINAL_STATUS.md)
2. Test Python backend independently
3. Build and test Rust frontend
4. Integration testing

## Documentation

- `FINAL_STATUS.md` - Complete status and architecture
- `IMPLEMENTATION_GUIDE.md` - Step-by-step guide
- `PROGRESS.md` - Development progress
- `COMPLETION_SUMMARY.md` - 12-part plan

## API Configuration

**Groq AI (Free Tier)**
- 14,400 requests/day
- No credit card required
- Signup: https://console.groq.com

