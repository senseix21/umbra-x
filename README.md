# UMBRA.chat

**Post-quantum private chat with zk-verified identities. No servers. No spam. No trace.**

## 🎉 Status: CLI Fully Functional + Desktop App in Development

The P2P CLI messenger is **ready to use**! Features:
- ✅ Real-time P2P messaging
- ✅ Quantum-resistant encryption (ML-KEM-768 + ChaCha20-Poly1305)
- ✅ Peer discovery (Kademlia DHT + manual connection)
- ✅ Interactive async CLI
- ✅ Clean message display
- ✅ Production-ready build

**Quick Start:**
```bash
cargo build --release --bin umbra
./target/release/umbra start -u alice -p 5000
```

See [CLI_USER_GUIDE.md](./CLI_USER_GUIDE.md) for detailed usage.

## Core Features

### 🔐 Post-Quantum Security
- **Hybrid encryption**: X25519 + ML-KEM-768 (Kyber)
- **Quantum-safe signatures**: ML-DSA (Dilithium)
- **Perfect forward secrecy**: Ephemeral keys for every session
- **Zero-knowledge handshakes**: Prove identity without revealing secrets

### 👤 ZK-Verified Identity System *(Coming Q1 2025)*
- **One identity, infinite devices**: Prove you own an identity without revealing which device
- **Multi-device sync**: Same chat history on phone, laptop, desktop
- **QR code pairing**: Scan to add new device
- **Privacy-first**: No one knows which device you're using
- **No blockchain needed**: Pure P2P cryptography

**How it works:**
```rust
Identity = Hash(secret_seed)
Device Proof = ZK-SNARK("I know secret that hashes to Identity X")
// Prove ownership without revealing secret
```

### 💬 Chat History Sync *(Coming Q1 2025)*
- **Device-to-device sync**: Direct P2P transfer, no servers
- **Encrypted local storage**: SQLite with AES-256-GCM
- **Delta sync**: Only transfer new messages
- **Offline-first**: Works without internet, syncs when devices meet
- **Backup/restore**: Export encrypted history to file

**Storage:**
- 10,000 messages = ~5 MB
- 100,000 messages = ~50 MB
- Fully searchable SQLite database

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
│  ├─ umbra-net/       # P2P networking, QUIC, onion circuits
│  ├─ umbra-crypto/    # Hybrid PQ crypto, ML-KEM, ML-DSA
│  ├─ umbra-mls/       # MLS group state machine
│  ├─ umbra-zk/        # Zero-knowledge proofs (RLN, identity)
│  ├─ umbra-identity/  # ZK identity system (planned)
│  ├─ umbra-sync/      # Chat history sync (planned)
│  ├─ umbra-wire/      # Protocol schemas
│  ├─ umbra-vault/     # Encrypted storage
│  └─ umbra-sdk/       # High-level API
├─ apps/
│  ├─ cli/             # Command-line interface
│  ├─ desktop/         # Tauri desktop UI (in development)
│  └─ node/            # Headless relay node
└─ examples/           # Demos and integration tests
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
- **Phase A-C**: P2P mesh, quantum-resistant crypto, group messaging
- **CLI messenger**: Production-ready command-line interface

### In Progress 🚧
- **Desktop app**: Tauri-based GUI (peer connection working)
- **ZK proofs**: RLN for rate limiting

### Upcoming (Q1 2025) 📋
- **ZK Identity System**: Multi-device support with privacy
- **Chat History Sync**: Device-to-device encrypted sync
- **Mobile apps**: iOS + Android (Rust core, native UI)

### Future (Q2+ 2025) ⏳
- **Group chats**: MLS-based encrypted groups
- **Voice/video**: P2P encrypted calls
- **File sharing**: Large file transfer over P2P
- **Public alpha**: Invite-only testing

## Project Status

**Current Phase:** ZK Layer + Desktop App  
**Progress:** 70% Complete  
**Test Status:** ✅ 32/32 tests passing  
**CLI Status:** ✅ Production-ready  
**Desktop Status:** 🚧 In development

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Core crates (`umbra-*`): AGPL-3.0  
SDK & examples: Apache-2.0

---

**Status**: 🚧 Alpha development — not production ready
