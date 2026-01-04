#!/bin/bash
set -euo pipefail

# Academic Paper Interpreter MCP Server - Service Update Script

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
SERVICE_NAME="academic-paper-interpreter-mcp"

echo "=== Academic Paper Interpreter MCP Server Update ==="
echo ""

# Check if running as root
if [[ $EUID -ne 0 ]]; then
    echo "This script requires root privileges."
    echo "Run with: sudo $0"
    exit 1
fi

# Get the actual user (not root)
ACTUAL_USER="${SUDO_USER:-$USER}"
ACTUAL_HOME=$(getent passwd "$ACTUAL_USER" | cut -d: -f6)

# Find cargo binary
CARGO_BIN="${ACTUAL_HOME}/.cargo/bin/cargo"
if [[ ! -x "$CARGO_BIN" ]]; then
    CARGO_BIN=$(which cargo 2>/dev/null || true)
fi
if [[ -z "$CARGO_BIN" || ! -x "$CARGO_BIN" ]]; then
    echo "Error: cargo not found. Please install Rust first."
    exit 1
fi
echo "Using cargo: $CARGO_BIN"
echo ""

cd "$PROJECT_DIR"

# Step 1: Pull latest changes (if git repo)
if [[ -d ".git" ]]; then
    echo "[1/5] Pulling latest changes..."
    sudo -u "$ACTUAL_USER" git pull || echo "  (skipped - not a git repo or no remote)"
else
    echo "[1/5] Skipping git pull (not a git repo)"
fi
echo ""

# Step 2: Build release binary
echo "[2/5] Building release binary..."
sudo -u "$ACTUAL_USER" "$CARGO_BIN" build --release
echo "  Binary: ${PROJECT_DIR}/target/release/${SERVICE_NAME}"
echo ""

# Step 3: Update service file
echo "[3/5] Updating service file..."
cp "${PROJECT_DIR}/${SERVICE_NAME}.service" "/etc/systemd/system/${SERVICE_NAME}.service"

# Enable EnvironmentFile if exists
if [[ -f "/etc/${SERVICE_NAME}/env" ]]; then
    sed -i 's|# EnvironmentFile=/etc/academic-paper-interpreter-mcp/env|EnvironmentFile=/etc/academic-paper-interpreter-mcp/env|' \
        "/etc/systemd/system/${SERVICE_NAME}.service"
    echo "  EnvironmentFile enabled"
fi
echo ""

# Step 4: Reload systemd
echo "[4/5] Reloading systemd daemon..."
systemctl daemon-reload
echo ""

# Step 5: Restart service
echo "[5/5] Restarting service..."
systemctl restart "$SERVICE_NAME"
sleep 1

# Show status
echo ""
echo "=== Update Complete ==="
echo ""
systemctl status "$SERVICE_NAME" --no-pager -l || true
echo ""
echo "View logs: sudo journalctl -u ${SERVICE_NAME} -f"
