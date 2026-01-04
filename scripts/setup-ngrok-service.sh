#!/bin/bash
set -euo pipefail

# ngrok Service Setup Script

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
SERVICE_FILE="${PROJECT_DIR}/ngrok/ngrok.service"

echo "=== ngrok Service Setup ==="
echo ""

# Check if running as root
if [[ $EUID -ne 0 ]]; then
    echo "This script requires root privileges."
    echo "Run with: sudo $0"
    exit 1
fi

# Get the actual user
ACTUAL_USER="${SUDO_USER:-$USER}"
ACTUAL_HOME=$(getent passwd "$ACTUAL_USER" | cut -d: -f6)

# Step 1: Check ngrok is installed
echo "[1/5] Checking ngrok installation..."
if ! command -v ngrok &> /dev/null; then
    echo "ngrok is not installed. Installing..."
    curl -s https://ngrok-agent.s3.amazonaws.com/ngrok.asc | \
        tee /etc/apt/trusted.gpg.d/ngrok.asc >/dev/null
    echo "deb https://ngrok-agent.s3.amazonaws.com buster main" | \
        tee /etc/apt/sources.list.d/ngrok.list
    apt-get update && apt-get install -y ngrok
fi
NGROK_PATH=$(which ngrok)
echo "  ngrok found: $NGROK_PATH"
echo ""

# Step 2: Check ngrok authtoken
echo "[2/5] Checking ngrok configuration..."
NGROK_CONFIG="${ACTUAL_HOME}/.config/ngrok/ngrok.yml"
if [[ ! -f "$NGROK_CONFIG" ]]; then
    echo ""
    echo "ERROR: ngrok authtoken not configured."
    echo ""
    echo "Please run the following command as ${ACTUAL_USER}:"
    echo "  ngrok config add-authtoken <your-authtoken>"
    echo ""
    echo "Get your authtoken from: https://dashboard.ngrok.com/get-started/your-authtoken"
    exit 1
fi
echo "  Config found: $NGROK_CONFIG"
echo ""

# Step 3: Install service file
echo "[3/5] Installing systemd service..."
sed "s|/usr/bin/ngrok|${NGROK_PATH}|g; s|User=kucho|User=${ACTUAL_USER}|g; s|Group=kucho|Group=${ACTUAL_USER}|g" \
    "$SERVICE_FILE" > /etc/systemd/system/ngrok-mcp.service
echo "  Service file installed: /etc/systemd/system/ngrok-mcp.service"
echo ""

# Step 4: Reload and enable
echo "[4/5] Enabling service..."
systemctl daemon-reload
systemctl enable ngrok-mcp
echo ""

# Step 5: Start service
echo "[5/5] Starting service..."
systemctl start ngrok-mcp
sleep 3
echo ""

# Show status and URL
echo "=== Setup Complete ==="
echo ""
systemctl status ngrok-mcp --no-pager -l || true
echo ""

# Get the public URL
echo "Fetching ngrok URL..."
sleep 2
NGROK_URL=$(curl -s http://localhost:4040/api/tunnels 2>/dev/null | grep -o '"public_url":"[^"]*"' | head -1 | cut -d'"' -f4 || echo "")

if [[ -n "$NGROK_URL" ]]; then
    echo ""
    echo "=========================================="
    echo "ngrok URL: ${NGROK_URL}"
    echo ""
    echo "Claude Desktop custom connector URL:"
    echo "  ${NGROK_URL}/sse"
    echo "=========================================="
else
    echo ""
    echo "Could not fetch ngrok URL automatically."
    echo "Check manually: curl -s http://localhost:4040/api/tunnels | jq -r '.tunnels[0].public_url'"
fi
echo ""
echo "Commands:"
echo "  Status:  sudo systemctl status ngrok-mcp"
echo "  Logs:    sudo journalctl -u ngrok-mcp -f"
echo "  Restart: sudo systemctl restart ngrok-mcp"
echo "  Stop:    sudo systemctl stop ngrok-mcp"
