# Changelog

All notable changes to UMBRA.chat will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Phase D - ZK Layer (In Progress)
- Enhanced RLN with Merkle tree integration
- Groth16 zkSNARK circuit structure (arkworks)
- Feature-gated post-quantum cryptography

## [0.3.0] - 2024-11-29

### Added - CLI MVP Release
- **Functional P2P Chat Application**
  - Interactive command-line interface for secure messaging
  - Real-time encrypted message sending and reception
  - Professional visual design with clear status indicators
  
- **End-to-End Encryption**
  - Session key derivation from peer IDs
  - ChaCha20-Poly1305 AEAD encryption for all messages
  - Automatic encryption/decryption pipeline
  
- **Peer Discovery & Connection**
  - Bootstrap node support for initial discovery
  - Direct peer connection via multiaddr
  - Automatic connection status tracking
  
- **User Experience**
  - Clean terminal interface without emojis
  - Real-time message display with sender identification
  - Connection status and peer information
  - Interactive message input with visual feedback

### Fixed
- Decryption errors due to asymmetric key derivation
- Message reception display issues
- Visual CLI formatting and consistency

### Security Notes
- ⚠️ Current session keys are deterministic (development only)
- ⚠️ No forward secrecy implemented yet
- ⚠️ Suitable for testing, NOT production use

## [0.2.0] - 2024-11-22

### Added - Phase C: MLS Groups + Vault
- **umbra-mls Crate**
  - Group state machine with member management
  - Epoch-based rekeying system
  - Add/remove member operations
  - Group lifecycle management

- **umbra-vault Crate**
  - RAM-only ephemeral storage mode
  - Sealed vault with ChaCha20-Poly1305 encryption
  - Secure export/import of state blobs
  - Zeroize integration for memory safety

### Security
- Memory cleanup with zeroize for sensitive data
- Encrypted state persistence with ML-KEM wrapping

## [0.1.0] - 2024-11-15

### Added - Phase B: P2P Core + Hybrid Crypto
- **umbra-net Crate**
  - QUIC transport via quinn + libp2p
  - Kademlia DHT for peer discovery
  - Gossipsub for pub/sub messaging
  - Onion circuit builder (3-hop routing skeleton)
  - Cover traffic daemon with Poisson scheduler

- **umbra-crypto Crate**
  - Hybrid KEM: X25519 + ML-KEM-768 (Kyber)
  - HPKE wrapper with ChaCha20-Poly1305 AEAD
  - Feature-gated post-quantum support
  - Identity signatures (Ed25519 + ML-DSA fallback)
  - Comprehensive KATs (Known Answer Tests)

- **umbra-wire Crate**
  - Protobuf message schemas
  - Semantic versioning for wire protocol
  - Test vectors for interoperability

- **Testing**
  - 50-node swarm integration test
  - Circuit building tests
  - Cover traffic scheduling tests
  - KEM encapsulation/decapsulation tests

### Security
- Hybrid post-quantum + classical cryptography
- Zeroization of sensitive key material
- Feature flags for gradual PQ adoption

## [0.0.1] - 2024-11-08

### Added - Phase A: Foundations
- **Project Structure**
  - Cargo workspace with modular crate layout
  - CI/CD pipeline (GitHub Actions)
  - Supply chain security (cargo-deny)
  - Reproducible build configuration

- **Core Crates** (Scaffolds)
  - `umbra-net` - Networking layer
  - `umbra-crypto` - Cryptography primitives
  - `umbra-mls` - Messaging Layer Security
  - `umbra-zk` - Zero-knowledge proofs
  - `umbra-wire` - Protocol definitions
  - `umbra-vault` - Secure storage
  - `umbra-sdk` - High-level API

- **Apps**
  - `node` - Headless daemon (CLI)
  - `desktop` - Tauri UI (scaffold)

- **Documentation**
  - README with project overview
  - THREAT_MODEL.md v0.1
  - CONTRIBUTING.md guidelines
  - CODE_OF_CONDUCT.md
  - SECURITY.md disclosure policy

- **Examples**
  - `hello_mesh` - Basic 2-node QUIC demo
  - `simple_chat` - Basic messaging demo

### Infrastructure
- GitHub Actions CI: fmt, clippy, tests
- cargo-deny for dependency auditing
- Test coverage tracking
- Automated security checks

---

## Version Naming Convention

- **0.x.x** - Pre-release development versions
- **1.0.0** - First production release (post-security audit)
- **Major.Minor.Patch** - Semantic versioning after 1.0.0

## Security Disclosure

See [SECURITY.md](./SECURITY.md) for reporting vulnerabilities.

## Links

- [Roadmap](./ROADMAP.md) - Development phases and timeline
- [README](./README.md) - Project overview and quickstart
- [Threat Model](./THREAT_MODEL.md) - Security architecture
