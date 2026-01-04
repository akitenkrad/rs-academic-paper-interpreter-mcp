#!/bin/bash
set -euo pipefail

# Academic Paper Interpreter MCP Server - Service Installation Script

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
SERVICE_NAME="academic-paper-interpreter-mcp"
SERVICE_FILE="${PROJECT_DIR}/${SERVICE_NAME}.service"
ENV_FILE="${PROJECT_DIR}/${SERVICE_NAME}.env"
ENV_DIR="/etc/${SERVICE_NAME}"

echo "=== Academic Paper Interpreter MCP Server Installation ==="
echo ""

# Check if running as root for system installation
if [[ $EUID -ne 0 ]]; then
    echo "This script requires root privileges for system service installation."
    echo "Run with: sudo $0"
    exit 1
fi

# Build release binary
echo "[1/5] Building release binary..."
cd "$PROJECT_DIR"
sudo -u "${SUDO_USER:-$USER}" cargo build --release
echo "Binary built: ${PROJECT_DIR}/target/release/${SERVICE_NAME}"
echo ""

# Create environment directory
echo "[2/5] Setting up environment configuration..."
mkdir -p "$ENV_DIR"
if [[ ! -f "${ENV_DIR}/env" ]]; then
    cp "$ENV_FILE" "${ENV_DIR}/env"
    chmod 600 "${ENV_DIR}/env"
    echo "Environment file created: ${ENV_DIR}/env"
    echo "  -> Edit this file to add your API keys"
else
    echo "Environment file already exists: ${ENV_DIR}/env (skipped)"
fi
echo ""

# Update service file to use environment file
echo "[3/5] Installing systemd service..."
sed 's|# EnvironmentFile=/etc/academic-paper-interpreter-mcp/env|EnvironmentFile=/etc/academic-paper-interpreter-mcp/env|' \
    "$SERVICE_FILE" > "/etc/systemd/system/${SERVICE_NAME}.service"
echo "Service file installed: /etc/systemd/system/${SERVICE_NAME}.service"
echo ""

# Reload systemd
echo "[4/5] Reloading systemd daemon..."
systemctl daemon-reload
echo ""

# Enable and start service
echo "[5/5] Enabling service..."
systemctl enable "$SERVICE_NAME"
echo ""

echo "=== Installation Complete ==="
echo ""
echo "Next steps:"
echo "  1. Edit environment file with your API keys:"
echo "     sudo vim ${ENV_DIR}/env"
echo ""
echo "  2. Start the service:"
echo "     sudo systemctl start ${SERVICE_NAME}"
echo ""
echo "  3. Check status:"
echo "     sudo systemctl status ${SERVICE_NAME}"
echo ""
echo "  4. View logs:"
echo "     sudo journalctl -u ${SERVICE_NAME} -f"
echo ""
echo "Server will be available at: http://localhost:18080"
