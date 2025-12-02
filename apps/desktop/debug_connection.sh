#!/bin/bash
# Debug connection issues with detailed logging

echo "🔍 UMBRA Connection Debugger"
echo ""
echo "This will:"
echo "  1. Show all console output"
echo "  2. Log connection attempts"
echo "  3. Show DHT bootstrap"
echo "  4. Display peer discovery"
echo ""

cd /Users/abuhamzah/Dev/umbra-chat/apps/desktop

echo "Starting with RUST_LOG=debug for maximum verbosity..."
echo ""
echo "Watch for these messages:"
echo "  🚀 Node started with PeerID: ..."
echo "  📡 Subscribing to discovery topic: ..."
echo "  🌐 Bootstrapping DHT..."
echo "  🔌 Attempting to connect to: ..."
echo "  ✓ Connected to ..."
echo ""

RUST_LOG=debug cargo run 2>&1 | grep --line-buffered -E "(Node started|Subscribing|Bootstrapping|Attempting|Connected|Dialing|error|ERROR)"
