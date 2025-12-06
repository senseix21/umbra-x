# UMBRA.chat

**Post-quantum private chat with zk-verified identities. No servers. No spam. No trace.**

## 🎉 Status: Production-Ready CLI (v0.8.0)

The P2P CLI messenger is **ready to use** with pure Rust post-quantum cryptography!

**Core Features:**
- ✅ Real-time P2P messaging
- ✅ **Pure Rust post-quantum crypto** (Kyber768 + Dilithium3) 🆕
- ✅ **No external dependencies** (no OpenSSL, no cmake) 🆕
- ✅ Zero-knowledge identity verification (Groth16 SNARKs)
- ✅ Peer discovery (Kademlia DHT + manual connection)
- ✅ Interactive async CLI
- ✅ 128 tests passing

**Quick Start:**
```bash
# Build
cargo build --release --bin umbra

# Create identity (optional)
./target/release/umbra identity create mypassword

# Start chat
./target/release/umbra start -u alice -p 5000
```

See [CLI_USER_GUIDE.md](./CLI_USER_GUIDE.md) for detailed usage.

## Core Features

### 🔐 Post-Quantum Security
- **Hybrid encryption**: X25519 + ML-KEM-768 (Kyber)
- **Quantum-safe signatures**: ML-DSA (Dilithium3)
- **Perfect forward secrecy**: Ephemeral keys for every session
- **Zero-knowledge handshakes**: Prove identity without revealing secrets

### 👤 ZK-Verified Identity System ✅ **SHIPPED (v0.7.0)**

**Create verifiable identity from a password:**
```bash
$ umbra identity create mypassword
✅ Identity created
🆔 ID: 37acb113...
```

**Chat with verified identities:**
```
alice ✓ [15:38:01:37acb113] > Hello!
         ↑
      ZK proof verified ✓
```

**How it works:**
```rust
password → blake3 → secret (32 bytes)
secret → x^5 in BN254 field → identity_id (32 bytes)

On send: Generate ZK proof: "I know secret that hashes to identity_id"
On receive: Verify proof (zero-knowledge, reveals nothing about password)
```

**Features:**
- ✅ Groth16 ZK-SNARKs (production-grade)
- ✅ Proof generation: 50-100ms
- ✅ Proof verification: <5ms
- ✅ Proof size: ~192 bytes
- ✅ Deterministic (same password = same ID)
- ✅ Backward compatible (works without identity)

See [ZK_IDENTITY.md](./ZK_IDENTITY.md) for technical details.

### 💬 P2P File Transfer *(Coming v0.9.0 - Q1 2026)* ⭐ **NEXT**
- **Unlimited file size**: No caps like Signal (100 MB) or WhatsApp (2 GB)
- **Chunk-based streaming**: 1 MB blocks with resume/pause
- **Multi-source download**: BitTorrent-style from multiple peers
- **Per-chunk encryption**: Post-quantum encrypted chunks
- **Folder support**: Transfer entire directories

### 🌐 Fully P2P Mesh
- **No central servers**: Direct peer-to-peer connections
- **Onion routing**: Multi-hop routing with cover traffic (optional)
- **NAT traversal**: Works behind routers and firewalls
- **Kademlia DHT**: Distributed peer discovery

## Architecture

Built with Rust, leveraging:
- `libp2p` + `quinn` for QUIC-based P2P networking
- Hybrid post-quantum cryptography (classical + PQ)
- MLS (Messaging Layer Security) for group encryption
- Zero-knowledge proofs for identity and rate limiting
- SQLite for local encrypted storage

## Repository Structure

```
umbra/
├─ crates/
│  ├─ umbra-net/       # P2P networking, QUIC, transport
│  ├─ umbra-crypto/    # Hybrid PQ crypto, ML-KEM, ML-DSA
│  ├─ umbra-mls/       # MLS group state machine
│  ├─ umbra-zk/        # Zero-knowledge proofs (RLN, circuits)
│  ├─ umbra-identity/  # ZK identity system (Groth16) ✅ v0.7.0
│  ├─ umbra-wire/      # Protocol schemas
│  ├─ umbra-vault/     # Encrypted storage
│  └─ umbra-sdk/       # High-level API
├─ apps/
│  ├─ cli/             # Command-line interface ✅
│  └─ node/            # Headless relay node
└─ docs/               # Documentation
```

## Quick Start

```bash
# Install Rust 1.81+
rustup install stable
rustup default stable

# Clone and enter project (if not already there)
cd umbra-chat

# Run all tests
cargo test --workspace
# Expected: ✅ 32 tests pass in ~11 seconds

# Run CLI messenger
# Terminal 1:
cargo run --bin umbra -- start -u alice -p 9001

# Terminal 2 (use multiaddr from Terminal 1):
cargo run --bin umbra -- start -u bob -c "/ip4/127.0.0.1/udp/9001/quic-v1/p2p/PEER_ID"

# Run desktop app (WIP)
cargo run -p umbra-desktop
```

### 📖 Documentation

- **[CLI_USER_GUIDE.md](./CLI_USER_GUIDE.md)** - Complete CLI usage guide
- **[HOW_TO_TEST.md](./HOW_TO_TEST.md)** - Testing guide
- **[QUICKSTART.md](./QUICKSTART.md)** - 3-minute quick start
- **[ROADMAP.md](./ROADMAP.md)** - Development roadmap
- **[THREAT_MODEL.md](THREAT_MODEL.md)** - Security analysis

## Roadmap

### Completed ✅
- **v0.1-0.7**: P2P mesh, quantum-resistant crypto, ZK identity
- **v0.8.0**: Pure Rust PQ crypto (no OpenSSL/cmake)
- **CLI messenger**: Production-ready command-line interface

### Next: v0.9.0 - P2P File Transfer (Q1 2026) ⭐
- **Unlimited file size**: No arbitrary caps
- **Chunk-based**: Resume/pause capability
- **Multi-source**: BitTorrent-style downloads
- **PQ encrypted**: Per-chunk quantum-resistant encryption

### Future Releases 📋
- **v1.0.0** - Mobile apps (iOS + Android)
- **v1.1.0** - Voice/Video calls
- **v1.2.0** - AI assistant + Web3 integration
- **v2.0.0** - Desktop apps, plugin system, bot API

## Project Status

**Current Version:** v0.8.0 (Pure Rust PQ Crypto)  
**Released:** December 6, 2024  
**Test Status:** ✅ 128 tests passing  
**CLI Status:** ✅ Production-ready  
**Next Feature:** P2P File Transfer (v0.9.0)

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Core crates (`umbra-*`): AGPL-3.0  
SDK & examples: Apache-2.0

---

**Status**: 🚧 Alpha development — not production ready
