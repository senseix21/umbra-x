#!/bin/bash
# Diagnose UMBRA Desktop issues

echo "🔍 UMBRA Desktop Diagnostics"
echo ""

cd /Users/abuhamzah/Dev/umbra-chat/apps/desktop

echo "1. Checking files..."
test -f ui/public/index.html && echo "✅ index.html exists" || echo "❌ index.html missing"
test -f ui/public/style.css && echo "✅ style.css exists" || echo "❌ style.css missing"
test -f ui/public/app.js && echo "✅ app.js exists" || echo "❌ app.js missing"
test -f tauri.conf.json && echo "✅ tauri.conf.json exists" || echo "❌ tauri.conf.json missing"
echo ""

echo "2. Checking Tauri config..."
grep -q '"frontendDist": "ui/public"' tauri.conf.json && echo "✅ frontendDist correct" || echo "❌ frontendDist wrong"
echo ""

echo "3. Checking Cargo.toml..."
grep -q 'protocol-asset' Cargo.toml && echo "✅ protocol-asset feature enabled" || echo "❌ protocol-asset feature missing"
echo ""

echo "4. Build check..."
cargo check --quiet 2>&1 && echo "✅ Build passes" || echo "❌ Build fails"
echo ""

echo "5. File sizes..."
ls -lh ui/public/*.{html,css,js} 2>/dev/null

echo ""
echo "If all checks pass, run: ./run.sh"
echo "If window is blank, open DevTools: Cmd+Opt+I"
