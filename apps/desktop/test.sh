#!/bin/bash
# Test UMBRA Desktop App

set -e

echo "🧪 Testing UMBRA Desktop..."
echo

echo "✅ Step 1: Build check"
cd /Users/abuhamzah/Dev/umbra-chat/apps/desktop
cargo check --quiet

echo "✅ Step 2: Verify files exist"
test -f src/main.rs || { echo "❌ main.rs missing"; exit 1; }
test -f ui/public/index.html || { echo "❌ index.html missing"; exit 1; }
test -f ui/public/style.css || { echo "❌ style.css missing"; exit 1; }
test -f ui/public/app.js || { echo "❌ app.js missing"; exit 1; }
test -f tauri.conf.json || { echo "❌ tauri.conf.json missing"; exit 1; }

echo "✅ Step 3: Check structure"
grep -q "start_node" src/main.rs || { echo "❌ start_node command missing"; exit 1; }
grep -q "connect_peer" src/main.rs || { echo "❌ connect_peer command missing"; exit 1; }
grep -q "send_message" src/main.rs || { echo "❌ send_message command missing"; exit 1; }

echo "✅ Step 4: Check UI"
grep -q "UMBRA" ui/public/index.html || { echo "❌ Title missing"; exit 1; }
grep -q "sidebar" ui/public/style.css || { echo "❌ Styles missing"; exit 1; }
grep -q "invoke" ui/public/app.js || { echo "❌ Tauri API missing"; exit 1; }

echo
echo "✅ All checks passed!"
echo "📦 To run: cargo run"
echo "🚀 App structure is ready!"
