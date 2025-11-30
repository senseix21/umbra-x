#!/bin/bash
# Quick handshake integration test

set -e

echo "🔐 Testing Quantum-Resistant Handshake Protocol"
echo "================================================"
echo

echo "1️⃣  Running crypto handshake tests..."
cargo test --package umbra-crypto --lib handshake::tests --quiet
echo "   ✅ Crypto handshake tests passed"
echo

echo "2️⃣  Running network handshake tests..."
cargo test --package umbra-net --lib handshake::tests --quiet
echo "   ✅ Network handshake tests passed"
echo

echo "3️⃣  Running wire protocol tests..."
cargo test --package umbra-wire --lib handshake::tests --quiet
echo "   ✅ Wire protocol tests passed"
echo

echo "4️⃣  Running message encryption tests..."
cargo test --package umbra-net --lib message::tests --quiet
echo "   ✅ Message encryption tests passed"
echo

echo "================================================"
echo "✅ All handshake tests passed!"
echo
echo "Handshake Protocol Status:"
echo "  - ML-KEM-768 (Kyber): ✅ Working"
echo "  - X25519 ECDH: ✅ Working"
echo "  - Ed25519 Signatures: ✅ Working"
echo "  - Session Keys: ✅ Derived and stored"
echo "  - ChaCha20Poly1305: ✅ Encrypting messages"
echo
echo "🎉 Quantum-resistant handshake protocol is operational!"
