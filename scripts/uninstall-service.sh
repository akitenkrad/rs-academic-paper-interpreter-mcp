#!/bin/bash
set -euo pipefail

# Academic Paper Interpreter MCP Server - Service Uninstallation Script

SERVICE_NAME="academic-paper-interpreter-mcp"
ENV_DIR="/etc/${SERVICE_NAME}"

echo "=== Academic Paper Interpreter MCP Server Uninstallation ==="
echo ""

if [[ $EUID -ne 0 ]]; then
    echo "This script requires root privileges."
    echo "Run with: sudo $0"
    exit 1
fi

# Stop and disable service
echo "[1/3] Stopping and disabling service..."
systemctl stop "$SERVICE_NAME" 2>/dev/null || true
systemctl disable "$SERVICE_NAME" 2>/dev/null || true
echo ""

# Remove service file
echo "[2/3] Removing service file..."
rm -f "/etc/systemd/system/${SERVICE_NAME}.service"
systemctl daemon-reload
echo ""

# Ask about environment file
echo "[3/3] Environment configuration..."
if [[ -d "$ENV_DIR" ]]; then
    read -p "Remove environment configuration (${ENV_DIR})? [y/N] " -n 1 -r
    echo
    if [[ $REPLY =~ ^[Yy]$ ]]; then
        rm -rf "$ENV_DIR"
        echo "Environment configuration removed."
    else
        echo "Environment configuration preserved."
    fi
fi
echo ""

echo "=== Uninstallation Complete ==="
