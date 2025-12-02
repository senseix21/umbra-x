#!/bin/bash
# Complete connection test with all logging

echo "═══════════════════════════════════════════════════════════"
echo "  UMBRA Desktop - Connection Deep Debug"
echo "═══════════════════════════════════════════════════════════"
echo ""
echo "New logging added:"
echo "  ✅ Dialing events"
echo "  ✅ Peer added to Kademlia routing table"
echo "  ✅ All swarm events logged"
echo "  ✅ Connection errors detailed"
echo ""
echo "═══════════════════════════════════════════════════════════"
echo ""

cd /Users/abuhamzah/Dev/umbra-chat/apps/desktop

echo "Starting with RUST_LOG=info (filtered output)..."
echo ""
echo "EXPECTED SEQUENCE:"
echo "  1. 🚀 Node started with PeerID: ..."
echo "  2. 📡 Subscribing to discovery topic: umbra-global"
echo "  3. ⚠️  DHT bootstrap skipped (OK)"
echo "  4. Listening on /ip4/..."
echo ""
echo "WHEN YOU CONNECT:"
echo "  5. 🔌 Attempting to connect to: ..."
echo "  6. 📡 Dialing peer..."
echo "  7. Adding peer ... to routing table"
echo "  8. Dial request sent for ..."
echo "  9. 📞 Dialing peer: ..."
echo "  10. ✓ Connected to ..."
echo ""
echo "IF CONNECTION FAILS, YOU'LL SEE:"
echo "  ❌ Outgoing connection error to ... : [reason]"
echo ""
echo "════════════════════════════════════════════════════════════"
echo "Starting app now..."
echo ""

RUST_LOG=info cargo run
