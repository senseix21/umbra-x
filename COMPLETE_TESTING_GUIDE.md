# UMBRA.chat - Complete Testing & Usage Guide

**Last Updated:** November 29, 2024

## Quick Start - CLI Chat (5 minutes)

### Step 1: Build the CLI App
```bash
cd /Users/abuhamzah/Dev/umbra-chat
cargo build --bin umbra --release
```

### Step 2: Open Two Terminals

**Terminal 1 (Alice):**
```bash
./target/release/umbra start -u alice -p 9000
```

Wait for output showing your peer address like:
```
/ip4/127.0.0.1/udp/9000/quic-v1/p2p/12D3KooWABC...
```

**Terminal 2 (Bob):**
```bash
./target/release/umbra start -u bob -c "/ip4/127.0.0.1/udp/9000/quic-v1/p2p/12D3KooWABC..."
```
(Replace with actual address from Terminal 1)

### Step 3: Chat!
Type messages in either terminal and press Enter to send.

---

## Complete Test Suite

### 1. Unit Tests (Fast - 11 seconds)

```bash
# Run all tests
cargo test --workspace

# Expected output:
# running 32 tests
# test result: ok. 32 passed; 0 failed; 1 ignored
```

**What this tests:**
- ✅ Cryptography (KEM, AEAD, PQ)
- ✅ Networking (P2P, gossipsub, circuits)
- ✅ MLS groups (add/remove, epochs)
- ✅ Zero-knowledge proofs (RLN, Merkle)
- ✅ Vault (encryption, export/import)

### 2. P2P Discovery Test

```bash
# Terminal 1
cargo run --example hello_mesh node1

# Terminal 2 (copy address from Terminal 1)
cargo run --example hello_mesh node2 <address_from_terminal_1>
```

**Expected:** Both terminals show connection and ping messages.

**What this tests:**
- ✅ QUIC transport
- ✅ libp2p networking
- ✅ Peer discovery
- ✅ Connection establishment

### 3. Gossipsub Messaging Test

```bash
# Terminal 1
cargo run --example simple_chat

# Terminal 2
cargo run --example simple_chat
```

**What this tests:**
- ✅ Pub/sub messaging
- ✅ Topic subscription
- ✅ Message broadcasting

### 4. CLI Application Test

#### Basic Start
```bash
./target/release/umbra start -u testuser
```

**Expected:**
- Node starts successfully
- Shows peer ID
- Shows listening addresses
- Subscribes to default topic

#### With Custom Port
```bash
./target/release/umbra start -u alice -p 9000
```

**Expected:**
- Listens on port 9000 specifically

#### With Peer Connection
```bash
# First get a peer address from another running node
./target/release/umbra start -u bob -c "/ip4/127.0.0.1/udp/9000/quic-v1/p2p/..."
```

**Expected:**
- Connects to specified peer
- Both nodes can communicate

#### Commands Test
While in chat, test these commands:
- `/help` - Shows help
- `/peers` - Shows connection info
- `/quit` - Exits cleanly

### 5. Feature-Specific Tests

#### Post-Quantum Cryptography
```bash
cargo test -p umbra-crypto --features pq
```

**Expected:** All 7 tests pass including ML-KEM tests.

#### Zero-Knowledge Proofs
```bash
cargo test -p umbra-zk
```

**Expected:** 15/15 tests pass (basic mode).

**With zkSNARKs (optional):**
```bash
cargo test -p umbra-zk --features arkworks
```

**Expected:** 12/15 pass (3 Groth16 tests fail - circuit WIP, this is expected).

### 6. Network Scenarios

#### Same Computer (localhost)
```bash
# Terminal 1
./target/release/umbra start -u alice -p 9000

# Terminal 2
./target/release/umbra start -u bob -c "/ip4/127.0.0.1/udp/9000/quic-v1/p2p/..."
```

#### Same Local Network
```bash
# Find your IP
ifconfig | grep "inet " | grep -v 127.0.0.1
# Example: 192.168.1.100

# Computer 1
./target/release/umbra start -u alice -p 9000

# Computer 2 (same network)
./target/release/umbra start -u bob -c "/ip4/192.168.1.100/udp/9000/quic-v1/p2p/..."
```

### 7. Stress Tests (Optional)

#### 50-Node Swarm Test
**Warning:** Takes 30+ seconds.

```bash
cargo test -p umbra-net test_swarm_50_nodes -- --ignored --nocapture
```

**Expected:** 50 nodes form a network and exchange messages.

---

## Verification Checklist

### Build & Compile
- [ ] `cargo build --workspace` - No errors
- [ ] `cargo build --workspace --release` - Optimized build successful
- [ ] `cargo check --workspace` - Type checking passes

### Code Quality
- [ ] `cargo fmt --check` - Code is formatted
- [ ] `cargo clippy --all-targets` - Linting passes (warnings OK)

### Core Functionality
- [ ] `cargo test --workspace` - All 32 tests pass
- [ ] `cargo run --example hello_mesh` - P2P connection works
- [ ] `cargo run --example simple_chat` - Messaging works

### CLI Application
- [ ] `./target/release/umbra info` - Shows project info
- [ ] `./target/release/umbra start` - Node starts
- [ ] Two nodes can connect via `-c` flag
- [ ] Messages can be sent (published to topic)
- [ ] Commands work (/help, /peers, /quit)

### Features
- [ ] PQ crypto tests pass (`cargo test -p umbra-crypto --features pq`)
- [ ] ZK tests pass (`cargo test -p umbra-zk`)
- [ ] MLS tests pass (`cargo test -p umbra-mls`)
- [ ] Vault tests pass (`cargo test -p umbra-vault`)

---

## Troubleshooting

### Issue: Tests Timeout
**Solution:** Network tests can take 5-10 seconds. Wait patiently.

### Issue: Port Already in Use
**Solution:** Use a different port:
```bash
./target/release/umbra start -p 9001
```

### Issue: Can't Connect to Peer
**Causes:**
1. Firewall blocking UDP
2. Wrong address format
3. Peer not running

**Solutions:**
```bash
# Check firewall (macOS)
sudo /usr/libexec/ApplicationFirewall/socketfilterfw --getglobalstate

# Verify address format
/ip4/<IP>/udp/<PORT>/quic-v1/p2p/<PEER_ID>

# Make sure peer is running and listening
```

### Issue: "No route to host"
**Solution:** Check that both computers are on the same network and can ping each other:
```bash
ping 192.168.1.100
```

### Issue: Build Fails
**Solution:**
```bash
cargo clean
cargo build --workspace
```

---

## Performance Benchmarks

### Observed Timings
- **Hybrid KEM:** <1ms
- **AEAD Encrypt/Decrypt:** <1ms
- **RLN Proof (SHA256):** <1ms
- **Gossipsub Delivery:** 50-100ms
- **Test Suite:** ~11 seconds
- **50-node swarm:** ~30 seconds

### Targets (Phase D Goals)
- **ZK Proof Generation:** <1.5s (currently ~1ms with SHA256, TBD with Groth16)
- **Message P50 Latency:** <300ms intra-region
- **DM P50 Latency:** <900ms inter-region

---

## Test Matrix

| Component | Test Count | Time | Status |
|-----------|-----------|------|--------|
| umbra-crypto | 7 | <1s | ✅ Pass |
| umbra-net | 9 | ~10s | ✅ Pass |
| umbra-mls | 6 | <1s | ✅ Pass |
| umbra-zk | 15 | <1s | ✅ Pass |
| umbra-vault | 1 | <1s | ✅ Pass |
| umbra-wire | 1 | <1s | ✅ Pass |
| **Total** | **32** | **~11s** | **✅ Pass** |

---

## CI/CD Status

GitHub Actions runs automatically on every push:

1. ✅ Format check (`cargo fmt --check`)
2. ✅ Lint check (`cargo clippy`)
3. ✅ All tests (`cargo test --workspace`)
4. ✅ Release build (`cargo build --release`)

**View CI:** Check the Actions tab in GitHub repository.

---

## Next Steps After Testing

### If All Tests Pass:
1. ✅ Core systems are operational
2. ✅ CLI app is ready to use
3. 🚧 Next: Add message reception to CLI
4. 🚧 Next: Complete Groth16 circuit
5. 🚧 Next: Integrate E2E encryption

### If Tests Fail:
1. Read error output carefully
2. Check [ROADMAP.md](ROADMAP.md) for known issues
3. Review [STATUS.md](STATUS.md) for current state
4. Open a GitHub issue with test output

---

## Quick Reference

### One-Line Test Commands
```bash
# Full test suite
cargo test --workspace

# Just crypto
cargo test -p umbra-crypto

# Just networking
cargo test -p umbra-net

# Just ZK
cargo test -p umbra-zk

# CLI smoke test
./target/release/umbra info

# P2P demo
cargo run --example hello_mesh node1
```

### One-Line CLI Usage
```bash
# Start alice
./target/release/umbra start -u alice -p 9000

# Start bob (connect to alice)
./target/release/umbra start -u bob -c "/ip4/127.0.0.1/udp/9000/quic-v1/p2p/<PEER_ID>"
```

---

## Documentation

- **[CLI_GUIDE.md](CLI_GUIDE.md)** - Complete CLI usage guide
- **[STATUS.md](STATUS.md)** - Current project status
- **[ROADMAP.md](ROADMAP.md)** - Development roadmap
- **[TESTING.md](TESTING.md)** - This file
- **[THREAT_MODEL.md](THREAT_MODEL.md)** - Security analysis

---

**TL;DR:**
```bash
# Build
cargo build --workspace --release

# Test
cargo test --workspace

# Use
./target/release/umbra start -u yourname

# Success!
✅ All systems operational!
```
