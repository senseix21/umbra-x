#!/bin/bash
# UMBRA Chat - Decryption Fix Verification Script

set -e

echo "╔═══════════════════════════════════════════════════════════════════╗"
echo "║         UMBRA Chat - Decryption Fix Verification                  ║"
echo "╚═══════════════════════════════════════════════════════════════════╝"
echo ""

# Colors
GREEN='\033[0;32m'
RED='\033[0;31m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Step 1: Run unit tests
echo "📋 Step 1: Running unit tests..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if cargo test -p umbra-crypto --lib chat_crypto -- --nocapture; then
    echo -e "${GREEN}✓ Unit tests PASSED${NC}"
else
    echo -e "${RED}✗ Unit tests FAILED${NC}"
    exit 1
fi
echo ""

# Step 2: Build release binary
echo "🔨 Step 2: Building release binary..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if cargo build --release --bin umbra; then
    echo -e "${GREEN}✓ Build SUCCESSFUL${NC}"
else
    echo -e "${RED}✗ Build FAILED${NC}"
    exit 1
fi
echo ""

# Step 3: Verify binary exists
echo "🔍 Step 3: Verifying binary..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
if [ -f "./target/release/umbra" ]; then
    BINARY_SIZE=$(du -h ./target/release/umbra | cut -f1)
    echo -e "${GREEN}✓ Binary exists: ./target/release/umbra (${BINARY_SIZE})${NC}"
else
    echo -e "${RED}✗ Binary not found${NC}"
    exit 1
fi
echo ""

# Step 4: Code inspection
echo "🔬 Step 4: Code inspection..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"

# Check if from_key exists
if grep -q "pub fn from_key" crates/umbra-crypto/src/chat_crypto.rs; then
    echo -e "${GREEN}✓ ChatCrypto::from_key() implemented${NC}"
else
    echo -e "${RED}✗ ChatCrypto::from_key() NOT found${NC}"
    exit 1
fi

# Check if derive_topic_key exists
if grep -q "fn derive_topic_key" apps/cli/src/chat.rs; then
    echo -e "${GREEN}✓ derive_topic_key() implemented${NC}"
else
    echo -e "${RED}✗ derive_topic_key() NOT found${NC}"
    exit 1
fi

# Check if sha2 dependency added
if grep -q 'sha2 = "0.10"' apps/cli/Cargo.toml; then
    echo -e "${GREEN}✓ sha2 dependency added${NC}"
else
    echo -e "${RED}✗ sha2 dependency NOT found${NC}"
    exit 1
fi

# Check if new test exists
if grep -q "test_from_key_shared_encryption" crates/umbra-crypto/src/chat_crypto.rs; then
    echo -e "${GREEN}✓ Shared key encryption test added${NC}"
else
    echo -e "${RED}✗ Shared key test NOT found${NC}"
    exit 1
fi
echo ""

# Step 5: Documentation check
echo "📚 Step 5: Documentation check..."
echo "━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━"
DOCS=("DECRYPTION_ANALYSIS.md" "DECRYPTION_FIX_COMPLETE.md")
for doc in "${DOCS[@]}"; do
    if [ -f "$doc" ]; then
        echo -e "${GREEN}✓ $doc exists${NC}"
    else
        echo -e "${YELLOW}⚠ $doc not found (optional)${NC}"
    fi
done
echo ""

# Summary
echo "╔═══════════════════════════════════════════════════════════════════╗"
echo "║                       VERIFICATION COMPLETE                        ║"
echo "╚═══════════════════════════════════════════════════════════════════╝"
echo ""
echo -e "${GREEN}✅ All checks passed!${NC}"
echo ""
echo "Next steps:"
echo "1. Run two peers in separate terminals:"
echo "   Terminal 1: ./target/release/umbra start -u alice -p 9000"
echo "   Terminal 2: ./target/release/umbra start -u bob -c \"/ip4/127.0.0.1/udp/9000/quic-v1/p2p/<ALICE_PEER_ID>\""
echo ""
echo "2. Type messages and verify:"
echo "   - No 'Decryption failed' errors"
echo "   - Messages appear with [peer_id] prefix"
echo "   - Both peers can see each other's messages"
echo ""
echo -e "${YELLOW}⚠ Security Note:${NC}"
echo "   This fix uses topic-based key derivation for development only."
echo "   Production deployment requires:"
echo "   - Phase 2: Hybrid KEM (Week 11)"
echo "   - Phase 3: MLS Groups (Week 13)"
echo ""
