#!/bin/bash
# Run UMBRA Desktop

cd /Users/abuhamzah/Dev/umbra-chat/apps/desktop

echo "🚀 Starting UMBRA Desktop..."
echo ""
echo "Window should open with:"
echo "  - Dark theme UI"
echo "  - Sidebar on left"
echo "  - Chat area on right"
echo "  - 'Connect' button in sidebar"
echo ""
echo "If blank, check browser console (Cmd+Option+I)"
echo ""

RUST_LOG=info cargo run
