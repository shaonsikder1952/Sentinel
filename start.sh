#!/bin/bash

# Sentinel AI Platform - Native Desktop Overlay - Quick Start Script

echo "🚀 Starting Sentinel AI Platform (Native Desktop Overlay)..."

# Check prerequisites
command -v cargo >/dev/null 2>&1 || { echo "❌ Rust/Cargo not found. Please install Rust."; exit 1; }
command -v python3 >/dev/null 2>&1 || { echo "❌ Python 3 not found. Please install Python 3.9+."; exit 1; }

# Start Python Planner API
echo "📋 Starting Python Planner API..."
cd planner
if [ ! -d "venv" ]; then
    python3 -m venv venv
fi
source venv/bin/activate
pip install -r requirements.txt > /dev/null 2>&1
python planner_api.py &
PLANNER_PID=$!
cd ..

# Wait for planner to start
sleep 3

# Start Rust Engine (optional, can run as service)
echo "⚙️  Starting Rust Engine (optional)..."
cd engine
if [ ! -f "target/release/sentinel-engine" ]; then
    echo "   Building Rust engine (this may take a while)..."
    cargo build --release
fi
# Uncomment to run engine as separate service:
# ./target/release/sentinel-engine &
# ENGINE_PID=$!
cd ..

# Start Native Overlay UI
echo "🌐 Starting Native Overlay UI..."
cd overlay_ui
if [ ! -f "target/release/sentinel-overlay" ]; then
    echo "   Building overlay UI (this may take a while)..."
    cargo build --release
fi
echo "   Launching overlay..."
./target/release/sentinel-overlay &
OVERLAY_PID=$!
cd ..

echo ""
echo "✅ Sentinel AI Platform is running!"
echo "   - Planner API: http://localhost:8000"
echo "   - Native Overlay: Should appear on right 20% of screen"
echo ""
echo "Press Ctrl+C to stop all services..."

# Wait for user interrupt
trap "kill $PLANNER_PID $OVERLAY_PID 2>/dev/null; exit" INT
wait

