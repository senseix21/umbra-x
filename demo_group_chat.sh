#!/bin/bash

# UMBRA Group Chat Demo
# Shows how to set up a 3-person group chat

set -e

echo "╔════════════════════════════════════════════════════════════════╗"
echo "║       UMBRA Group Chat Demo - 3 Person Setup                  ║"
echo "╚════════════════════════════════════════════════════════════════╝"
echo ""

# Build first if needed
if [ ! -f "./target/release/umbra" ]; then
    echo "[*] Building UMBRA..."
    cargo build --release
    echo ""
fi

echo "[*] Instructions for 3-person group chat:"
echo ""
echo "┌────────────────────────────────────────────────────────────────┐"
echo "│ Terminal 1 (Alice - Room Host)                                │"
echo "└────────────────────────────────────────────────────────────────┘"
echo ""
echo "  Run this command:"
echo "  \$ ./target/release/umbra start -u alice -t lounge -p 9000"
echo ""
echo "  Then COPY the Peer ID from the output (looks like 12D3Koo...)"
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "┌────────────────────────────────────────────────────────────────┐"
echo "│ Terminal 2 (Bob - Joins Alice)                                │"
echo "└────────────────────────────────────────────────────────────────┘"
echo ""
echo "  Replace ALICE_PEER_ID below with the ID from Terminal 1:"
echo ""
echo "  \$ ./target/release/umbra start -u bob -t lounge \\"
echo "      -c \"/ip4/127.0.0.1/udp/9000/quic-v1/p2p/ALICE_PEER_ID\""
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "┌────────────────────────────────────────────────────────────────┐"
echo "│ Terminal 3 (Charlie - Joins Alice)                            │"
echo "└────────────────────────────────────────────────────────────────┘"
echo ""
echo "  Same command as Bob, but with different username:"
echo ""
echo "  \$ ./target/release/umbra start -u charlie -t lounge \\"
echo "      -c \"/ip4/127.0.0.1/udp/9000/quic-v1/p2p/ALICE_PEER_ID\""
echo ""
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
echo ""
echo "[*] Key Points:"
echo "    - All three MUST use the same topic: -t lounge"
echo "    - Bob and Charlie connect to Alice's Peer ID"
echo "    - Everyone can now chat together!"
echo ""
echo "[*] Try these commands once connected:"
echo "    /peers  - See who's connected"
echo "    /help   - Show all commands"
echo "    /quit   - Leave the chat"
echo ""
echo "╔════════════════════════════════════════════════════════════════╗"
echo "║  Ready to start? Open 3 terminals and follow the steps above  ║"
echo "╚════════════════════════════════════════════════════════════════╝"
