# UMBRA.chat

**Post-quantum private chat with zk-verified humans. No servers. No spam. No trace.**

## Core Promise

- **Post-quantum E2EE**: Hybrid X25519 + ML-KEM (Kyber) with ML-DSA (Dilithium) signatures
- **ZK-verified personhood**: Prove you're a unique human without revealing identity
- **Fully P2P mesh**: Onion routing with cover traffic prevents social graph mapping
- **Stateless clients**: RAM-only mode plus encrypted portable state for "vanish on close" sessions

## Architecture

Built with Rust, leveraging:
- `libp2p` + `quinn` for QUIC-based P2P networking
- Hybrid post-quantum cryptography (classical + PQ)
- MLS (Messaging Layer Security) for group encryption
- Zero-knowledge proofs for rate limiting and personhood (RLN/Semaphore)
- Pluggable onion routing with cover traffic

## Repository Structure

```
umbra/
├─ crates/
│  ├─ umbra-net/       # P2P networking, QUIC, onion circuits, cover traffic
│  ├─ umbra-crypto/    # Hybrid PQ crypto, HPKE, signatures
│  ├─ umbra-mls/       # MLS group state machine with PQ hybrid secrets
│  ├─ umbra-zk/        # Zero-knowledge proofs (RLN, personhood)
│  ├─ umbra-wire/      # Protocol schemas, versioning, test vectors
│  ├─ umbra-vault/     # Sealed storage, export/import
│  └─ umbra-sdk/       # High-level API, bot/capsule runtime
├─ apps/
│  ├─ node/            # CLI daemon for headless relays
│  └─ desktop/         # Tauri desktop UI
└─ examples/           # Demos and integration tests
```

## Quick Start

```bash
# Install Rust 1.81+
rustup install stable
rustup default stable

# Clone and enter project (if not already there)
cd umbra-chat

# Run all tests (fastest way to verify everything works)
cargo test --workspace
# Expected: ✅ 32 tests pass in ~11 seconds

# Run P2P demo (requires 2 terminals)
# Terminal 1:
cargo run --example hello_mesh node1
# Terminal 2 (use address from Terminal 1):
cargo run --example hello_mesh node2 <address>
```

### 📖 Testing & Documentation

- **[HOW_TO_TEST.md](./HOW_TO_TEST.md)** - Complete testing guide (start here!)
- **[TESTING.md](./TESTING.md)** - Detailed test documentation
- **[QUICKSTART.md](./QUICKSTART.md)** - 3-minute quick start
- **[CURRENT_STATUS.md](./CURRENT_STATUS.md)** - Detailed project metrics
- **[ROADMAP.md](./ROADMAP.md)** - Development roadmap

## Project Status

**Current Phase:** D (ZK Layer) - Week 10  
**Progress:** 65% Complete  
**Test Status:** ✅ 32/32 tests passing  

### Completed ✅
- **Phase A** (Foundations): Monorepo, CI, P2P mesh
- **Phase B** (P2P + Crypto): Hybrid KEMs, onion circuits, cover traffic  
- **Phase C** (MLS + Vault): Group messaging, encrypted storage

### In Progress 🚧
- **Phase D** (ZK Layer): RLN proofs ✅, zkSNARK circuits 🚧, credentials 📋

### Upcoming ⏳
- **Phase E-H**: Privacy hardening, public alpha, beta, v1 launch

## Security & Threat Model

See [THREAT_MODEL.md](THREAT_MODEL.md) for adversary assumptions and mitigations.

## Contributing

See [CONTRIBUTING.md](CONTRIBUTING.md).

## License

Core crates (`umbra-*`): AGPL-3.0  
SDK & examples: Apache-2.0

---

**Status**: 🚧 Alpha development — not production ready
