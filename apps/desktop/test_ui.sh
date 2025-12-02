#!/bin/bash
# Quick test for UMBRA Desktop

echo "🚀 UMBRA Desktop Quick Test"
echo ""
echo "What you should see:"
echo "  ✅ Window opens with dark theme"
echo "  ✅ Sidebar shows 'My Peer ID: 12D3Koo...'"
echo "  ✅ Click the Peer ID to copy it"
echo "  ✅ Click 'Connect' to connect to another peer"
echo ""
echo "Starting app..."
echo ""

cd /Users/abuhamzah/Dev/umbra-chat/apps/desktop
cargo run
