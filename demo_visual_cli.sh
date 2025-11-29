#!/bin/bash
# UMBRA.chat Visual Demo Script
# Shows off the beautiful new CLI interface

echo "🎨 UMBRA.chat Visual CLI Demo"
echo "=============================="
echo ""

# Build the project
echo "📦 Building UMBRA CLI..."
cargo build --release 2>&1 | grep -E "(Compiling umbra-cli|Finished)" || echo "Already built!"
echo ""

# Show the info command
echo "1️⃣  Project Info (Beautiful formatting)"
echo "   Command: ./target/release/umbra info"
echo "   Press Enter to see..."
read
./target/release/umbra info
echo ""

# Instructions for two-peer demo
echo "2️⃣  Two-Peer Chat Demo"
echo "   ====================="
echo ""
echo "   To see the full visual chat experience, you need two terminals:"
echo ""
echo "   📱 Terminal 1 (Alice):"
echo "   $ ./target/release/umbra start -u alice -p 9000"
echo ""
echo "   📱 Terminal 2 (Bob):"
echo "   First, copy Alice's peer address from Terminal 1, then:"
echo "   $ ./target/release/umbra start -u bob -c '<alice_address>'"
echo ""
echo "   ✨ Features to try:"
echo "   • Send messages (automatically encrypted)"
echo "   • Type /peers to see beautiful peer info"
echo "   • Type /help for formatted help menu"
echo "   • Type /clear to refresh the screen"
echo "   • Type /quit for graceful goodbye"
echo ""

echo "🎉 Visual Features:"
echo "   ✓ Color-coded messages and status"
echo "   ✓ Timestamps on all messages"
echo "   ✓ Unicode borders and emojis"
echo "   ✓ Real-time visual feedback"
echo "   ✓ Professional status displays"
echo ""

echo "📖 Documentation:"
echo "   • CLI_VISUAL_GUIDE.md - User guide with screenshots"
echo "   • CLI_ENHANCEMENT_SUMMARY.md - Technical details"
echo ""

echo "Ready to try? Copy the commands above! 🚀"
