#!/bin/bash
# Show the current ngrok public URL

NGROK_URL=$(curl -s http://localhost:4040/api/tunnels 2>/dev/null | grep -o '"public_url":"[^"]*"' | head -1 | cut -d'"' -f4)

if [[ -n "$NGROK_URL" ]]; then
    echo "ngrok URL: ${NGROK_URL}"
    echo ""
    echo "Claude Desktop custom connector URL:"
    echo "  ${NGROK_URL}/sse"
else
    echo "ngrok is not running or URL not available."
    echo ""
    echo "Start ngrok service:"
    echo "  sudo systemctl start ngrok-mcp"
fi
