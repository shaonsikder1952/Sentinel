# Running Sentinel Locally - Complete Guide

## Prerequisites

### Required Software
1. **Python 3.8+** - For backend API server
2. **Rust & Cargo** - For frontend GUI application
3. **Git** - To clone the repository

### Install Prerequisites

**On macOS:**
```bash
# Install Homebrew if not already installed
/bin/bash -c "$(curl -fsSL https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh)"

# Install Python
brew install python

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**On Windows:**
```powershell
# Install Python from python.org or using winget
winget install Python.Python.3.12

# Install Rust from rustup.rs
# Download and run: https://win.rustup.rs/x86_64
```

**On Linux (Ubuntu/Debian):**
```bash
# Install Python
sudo apt update
sudo apt install python3 python3-pip python3-venv

# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env

# Install build dependencies
sudo apt install build-essential pkg-config libssl-dev
```

---

## Step 1: Clone/Download the Repository

### If using GitHub:
```bash
git clone https://github.com/YOUR_USERNAME/Sentinel.git
cd Sentinel
```

### If downloading from Codespaces:
```bash
# Download the entire Sentinel folder to your local machine
# Then navigate to it:
cd /path/to/Sentinel
```

---

## Step 2: Setup Environment Variables

### Get Free Groq API Key
1. Visit **https://console.groq.com**
2. Sign up (free, no credit card required)
3. Click "API Keys" in left sidebar
4. Create new API key
5. Copy the key

### Configure .env file
```bash
# Copy the example file
cp .env.example .env

# Edit .env and add your API key
# On macOS/Linux:
nano .env

# On Windows:
notepad .env
```

**Add this to .env:**
```
GROQ_API_KEY=gsk_your_actual_api_key_here
API_HOST=127.0.0.1
API_PORT=8000
```

Save and close the file.

---

## Step 3: Run Python Backend

### Terminal 1 - Start API Server

```bash
# Navigate to planner directory
cd planner

# Create virtual environment (recommended)
python3 -m venv venv

# Activate virtual environment
# On macOS/Linux:
source venv/bin/activate

# On Windows:
venv\\Scripts\\activate

# Install dependencies
pip install -r requirements.txt

# Run the server
python api_server.py
```

**Expected output:**
```
INFO:     Started server process
INFO:     Uvicorn running on http://127.0.0.1:8000
```

**Keep this terminal running!**

---

## Step 4: Build & Run Rust Frontend (AFTER FIXING ERRORS)

### Terminal 2 - Build Frontend

**⚠️ IMPORTANT: The Rust code currently has compilation errors!**

You need to fix the errors in `overlay_ui/src/app.rs` first.

```bash
# Open a NEW terminal window
cd overlay_ui

# Check for compilation errors
cargo check

# After fixing errors, build the project
cargo build --release

# Run the application
cargo run --release
```

---

## Step 5: Verify Everything Works

### Test Backend API (in Terminal 3)
```bash
# Test if API is responding
curl http://localhost:8000/

# Expected response:
{"message":"Sentinel Planner API","status":"running"}

# Test AI chat endpoint
curl -X POST http://localhost:8000/chat \\
  -H "Content-Type: application/json" \\
  -d '{"message":"Hello"}'
```

### Frontend Window
- A GUI window should open with the overlay interface
- Right 20% panel: Chat and controls
- Left 80% area: Workspace (currently transparent/minimal)

---

## Troubleshooting

### Backend Issues

**Error: "ModuleNotFoundError"**
```bash
# Make sure you're in the virtual environment
source venv/bin/activate  # or venv\\Scripts\\activate on Windows
pip install -r requirements.txt
```

**Error: "Address already in use"**
```bash
# Kill process using port 8000
# On macOS/Linux:
lsof -ti:8000 | xargs kill -9

# On Windows:
netstat -ano | findstr :8000
taskkill /PID <PID_NUMBER> /F
```

**Error: "Invalid API key"**
- Check your .env file has correct GROQ_API_KEY
- Make sure .env is in the root Sentinel/ directory
- Verify API key at https://console.groq.com

### Frontend Issues

**Error: "cargo: command not found"**
```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source $HOME/.cargo/env
```

**Compilation Errors in app.rs**
- The current code has errors that need fixing
- Check FINAL_STATUS.md for list of error codes
- You'll need to debug the Rust code or wait for fixes

**Error: "failed to compile"**
```bash
# On Linux, install dependencies:
sudo apt install build-essential pkg-config libssl-dev

# On macOS, install Xcode command line tools:
xcode-select --install
```

---

## Quick Start Summary

```bash
# Terminal 1 - Backend
cd Sentinel/planner
python3 -m venv venv
source venv/bin/activate  # or venv\\Scripts\\activate on Windows
pip install -r requirements.txt
python api_server.py

# Terminal 2 - Frontend (after fixing Rust errors)
cd Sentinel/overlay_ui
cargo run --release
```

---

## Project Structure on Local Machine

```
Sentinel/
├── .env                    # Your API keys (DO NOT COMMIT)
├── .env.example           # Template
├── planner/               # Python backend
│   ├── venv/             # Virtual environment (created by you)
│   ├── api_server.py
│   └── requirements.txt
└── overlay_ui/            # Rust frontend
    ├── Cargo.toml
    ├── target/           # Build artifacts (created by cargo)
    └── src/
```

---

## Next Steps After Setup

1. **Fix Rust compilation errors** - See FINAL_STATUS.md
2. **Test backend independently** - Use curl or Postman
3. **Integrate frontend with backend** - Once Rust compiles
4. **Customize and extend** - Add your own features

---

## Getting Help

- **Rust errors**: Check `cargo check` output
- **Python errors**: Check terminal output with traceback
- **API errors**: Check backend logs in Terminal 1
- **Documentation**: See FINAL_STATUS.md and README.md

