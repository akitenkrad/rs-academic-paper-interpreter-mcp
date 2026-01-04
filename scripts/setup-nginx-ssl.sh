#!/bin/bash
set -euo pipefail

# Academic Paper Interpreter MCP Server - nginx + Let's Encrypt Setup Script

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
NGINX_CONF="${PROJECT_DIR}/nginx/mcp-server.conf"

echo "=== nginx + Let's Encrypt Setup ==="
echo ""

# Check if running as root
if [[ $EUID -ne 0 ]]; then
    echo "This script requires root privileges."
    echo "Run with: sudo $0 <domain>"
    exit 1
fi

# Check arguments
if [[ $# -lt 1 ]]; then
    echo "Usage: sudo $0 <domain> [email]"
    echo ""
    echo "Arguments:"
    echo "  domain  - Your domain name (e.g., mcp.example.com)"
    echo "  email   - Email for Let's Encrypt notifications (optional)"
    echo ""
    echo "Example:"
    echo "  sudo $0 mcp.example.com admin@example.com"
    exit 1
fi

DOMAIN="$1"
EMAIL="${2:-}"

echo "Domain: $DOMAIN"
echo "Email: ${EMAIL:-<not specified>}"
echo ""

# Step 1: Install nginx and certbot if not installed
echo "[1/6] Installing nginx and certbot..."
if ! command -v nginx &> /dev/null; then
    apt-get update
    apt-get install -y nginx
fi

if ! command -v certbot &> /dev/null; then
    apt-get install -y certbot python3-certbot-nginx
fi
echo ""

# Step 2: Create certbot webroot directory
echo "[2/6] Creating certbot webroot..."
mkdir -p /var/www/certbot
echo ""

# Step 3: Copy and configure nginx config
echo "[3/6] Configuring nginx..."
sed "s/YOUR_DOMAIN/${DOMAIN}/g" "$NGINX_CONF" > /etc/nginx/sites-available/mcp-server.conf

# Create symlink if it doesn't exist
if [[ ! -L /etc/nginx/sites-enabled/mcp-server.conf ]]; then
    ln -s /etc/nginx/sites-available/mcp-server.conf /etc/nginx/sites-enabled/
fi

# Test nginx configuration (temporarily comment out SSL for initial setup)
# Create a temporary config without SSL for certbot
cat > /etc/nginx/sites-available/mcp-server-temp.conf << EOF
server {
    listen 80;
    listen [::]:80;
    server_name ${DOMAIN};

    location /.well-known/acme-challenge/ {
        root /var/www/certbot;
    }

    location / {
        return 200 'Certbot setup in progress';
        add_header Content-Type text/plain;
    }
}
EOF

# Temporarily use the temp config
rm -f /etc/nginx/sites-enabled/mcp-server.conf
ln -sf /etc/nginx/sites-available/mcp-server-temp.conf /etc/nginx/sites-enabled/mcp-server-temp.conf

nginx -t
systemctl reload nginx
echo ""

# Step 4: Obtain SSL certificate
echo "[4/6] Obtaining SSL certificate from Let's Encrypt..."
if [[ -n "$EMAIL" ]]; then
    certbot certonly --webroot -w /var/www/certbot -d "$DOMAIN" --email "$EMAIL" --agree-tos --non-interactive
else
    certbot certonly --webroot -w /var/www/certbot -d "$DOMAIN" --register-unsafely-without-email --agree-tos --non-interactive
fi
echo ""

# Step 5: Enable full nginx config with SSL
echo "[5/6] Enabling SSL configuration..."
rm -f /etc/nginx/sites-enabled/mcp-server-temp.conf
rm -f /etc/nginx/sites-available/mcp-server-temp.conf
ln -sf /etc/nginx/sites-available/mcp-server.conf /etc/nginx/sites-enabled/mcp-server.conf

nginx -t
systemctl reload nginx
echo ""

# Step 6: Setup auto-renewal
echo "[6/6] Setting up certificate auto-renewal..."
# Certbot auto-renewal is typically set up automatically, but let's ensure it
if ! systemctl is-enabled certbot.timer &> /dev/null; then
    systemctl enable certbot.timer
    systemctl start certbot.timer
fi
echo ""

echo "=== Setup Complete ==="
echo ""
echo "Your MCP server is now available at:"
echo "  SSE endpoint: https://${DOMAIN}/sse"
echo "  Message endpoint: https://${DOMAIN}/message"
echo ""
echo "Claude Desktop custom connector URL:"
echo "  https://${DOMAIN}/sse"
echo ""
echo "To verify:"
echo "  curl -I https://${DOMAIN}/health"
echo ""
echo "Logs:"
echo "  sudo tail -f /var/log/nginx/mcp-server-access.log"
echo "  sudo tail -f /var/log/nginx/mcp-server-error.log"
