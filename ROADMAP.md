# UMBRA Development Roadmap

**Status**: Phase D (ZK Layer) — W10-W13  
**Last Updated**: 2024-11-29  
**Progress**: 65% Complete (Phases A, B, C ✅ | Phase D 🚧)

## Timeline Overview

| Phase | Weeks | Goals | Status |
|-------|-------|-------|--------|
| A | W1-W2 | Foundations | ✅ Complete |
| B | W3-W6 | P2P Core + Hybrid Crypto | ✅ Complete |
| C | W7-W9 | MLS Groups + Vault | ✅ Complete |
| D | W10-W13 | ZK Layer (RLN + PoH) | 🚧 In Progress |
| E | W14-W16 | Privacy Hardening | ⏳ Planned |
| F | W17-W20 | Public Alpha | ⏳ Planned |
| G | W21-W24 | Beta (zk-Mod, Bots, Mobile) | ⏳ Planned |
| H | W25-W28 | v1 (Audits & Launch) | ⏳ Planned |

---

## Phase A — Foundations (W1-W2) ✅

### Goals
- ✅ Monorepo structure with all crates
- ✅ CI/CD pipeline (GitHub Actions)
- ✅ Supply chain security (cargo-deny)
- ✅ Hello-mesh: two peers discover via libp2p + QUIC
- ✅ Threat model v0.1

### Deliverables
- [x] Workspace `Cargo.toml` with all crates
- [x] Core crate scaffolds: `umbra-{net, crypto, mls, zk, wire, vault, sdk}`
- [x] Apps: `node` (headless), `desktop` (stub)
- [x] Examples: `hello_mesh`, `simple_chat`
- [x] CI: fmt, clippy, tests, security audit
- [x] Docs: README, CONTRIBUTING, SECURITY, CODE_OF_CONDUCT, THREAT_MODEL
- [x] Integration test: 2-node QUIC handshake

### Acceptance Criteria
- ✅ `cargo test` passes on stable + nightly
- ✅ Two-node demo exchanging messages over QUIC
- ✅ CI configured

---

## Phase B — P2P Core + Hybrid Crypto (W3-W6) ✅

### Goals
Reliable P2P transport with onion circuits + cover traffic; PQ-hybrid sessions.

### Deliverables
- ✅ **umbra-net**: QUIC transport, NAT traversal, Kademlia DHT, gossip-sub
- ✅ **Onion circuits**: 3-hop circuit build/teardown skeleton, per-hop keys
- ✅ **Cover traffic**: Poisson scheduler, chaff frames
- ✅ **umbra-crypto**: Hybrid KEM (X25519 + ML-KEM), with feature gates
- ✅ **umbra-wire**: Protobuf schemas, semantic versioning
- ✅ **Integration test**: 50-node swarm test (ignored by default)

### Tasks Completed
- ✅ Implement gossipsub pub/sub messaging
- ✅ Integrate Kademlia DHT for peer discovery
- ✅ Circuit builder with 3-hop routing (skeleton)
- ✅ Cover traffic daemon with Poisson distribution
- ✅ Hybrid KEM (X25519 + ML-KEM-768) with zeroization
- ✅ Feature flags: `pq` for post-quantum support
- ✅ 50-node swarm integration test

### Acceptance Criteria
- ✅ cargo test passes with all features
- ✅ Gossipsub message exchange working
- ✅ Hybrid KEM encap/decap tested
- ✅ Circuit builder creates 3-hop paths
- ✅ Cover traffic scheduler generates intervals

---

## Phase C — Secure Groups (MLS) + Vault (W7-W9) ✅

### Goals
End-to-end groups (DMs + channels), ephemeral by default, optional sealed storage.

### Deliverables
- ✅ **umbra-mls**: Group state machine, member management, epoch rekey
- ✅ **umbra-vault**: RAM-only mode, sealed vault (ChaCha20-Poly1305), export/import blobs
- ⏳ **Desktop app (alpha)**: Tauri UI (skeleton created)
- ✅ **Tests**: Group add/remove, epoch management

### Tasks Completed
- [x] Group creation and member management
- [x] Epoch-based rekeying system
- [x] RAM-only ephemeral vault
- [x] Sealed vault with encryption
- [x] State export/import with secure wrapping
- [x] Zeroize for secure memory cleanup

### Acceptance Criteria
- ✅ Group lifecycle tests pass
- ✅ Vault encryption/decryption works
- ✅ Export/import state blobs functional

---

## Phase D — ZK Layer: RLN + PoH (W10-W13) 🚧

### Goals
Anonymous anti-spam + personhood without KYC.

### Current Status (Week 10)
- ✅ Merkle tree for membership proofs
- ✅ Enhanced RLN prover/verifier
- ✅ Groth16 circuit structure (arkworks)
- ✅ Feature-gated zkSNARK support
- ✅ 15/15 basic tests passing
- 🚧 Circuit constraint refinement
- ⏳ Full credential issuance
- ⏳ System integration

### Deliverables
- **umbra-zk**: RLN (Rate-Limit Nullifier), Merkle membership tree
- **Circuit**: Groth16 zkSNARK structure (needs Poseidon hash)
- **Credential mint**: Committee threshold (skeleton exists)
- **Policy engine**: Room rate limits (complete)
- **Tests**: Rate limiting, nullifier detection, Merkle proofs

### Completed This Week
- [x] Implemented `merkle.rs` - SHA256 Merkle tree with proofs
- [x] Enhanced `rln.rs` - Integrated tree, dual-mode proofs
- [x] Created `circuit.rs` - R1CS constraints for RLN
- [x] Created `groth16.rs` - Prover/verifier wrapper
- [x] Added arkworks dependencies with feature gates
- [x] All basic RLN tests passing (15/15)
- [x] Build passing with/without arkworks

### Remaining Tasks (W11-W13)
- [ ] Fix Groth16 circuit (integrate Poseidon hash)
- [ ] Credential issuance with committee threshold
- [ ] Proof caching layer
- [ ] Integration with umbra-net messaging
- [ ] Spam simulation tests (1k msg/min)
- [ ] Performance optimization (<1.5s proof gen)
- [ ] `post_with_proof()` SDK API

### Technical Highlights
**Merkle Tree:**
```rust
let mut tree = MembershipTree::new();
tree.add_member(commitment)?;
let proof = tree.generate_proof(&commitment)?;
```

**RLN with Rate Limiting:**
```rust
let mut prover = RlnProver::new(config, secret);
let proof = prover.prove(b"message")?; // Ok
// ... 10 messages later ...
prover.prove(b"spam")?; // Error: RateLimitExceeded
```

**zkSNARK (when circuit fixed):**
```rust
#[cfg(feature = "arkworks")]
let setup = RlnSetup::trusted_setup()?;
let prover = RlnProver::new(config, secret).with_groth16(&setup);
let proof = prover.prove(b"message")?; // Zero-knowledge proof!
```

---

## Phase E — Privacy Hardening (W14-W16)

### Goals
Stop metadata leaks; measure without deanonymizing.

### Deliverables
- Fixed-size frames (512B) with fragmentation/reassembly
- Delayed ACKs, dummy ACKs, indistinguishable keepalives
- Traffic analysis harness: KS-distance test @ α=0.05
- Leak audit checklist (no stable IDs in headers, circuit rotation)
- Privacy-preserving telemetry (local-only, optional LDP)

### Tasks
- Implement frame padding and fragmentation
- Build TA harness with global passive adversary simulation
- Audit all network code for metadata leaks
- Add differential privacy to optional telemetry

---

## Phase F — Public Alpha (W17-W20)

### Goals
Ship testable alpha to early adopters.

### Deliverables
- Alpha builds (macOS/Linux/Windows): signed, reproducible
- UX: "Create identity", "Vanish on close", "Mint anonymous credential"
- Docs/site: quickstart, threat model v1.0, architecture whitepaper
- Chaos testing: 1k nodes, 24h soak test
- Performance: P50 <500ms intra-continent, 12h+ crash-free

### Tasks
- Reproducible builds setup (Nix/cargo2nix)
- Write quickstart guide + architecture docs
- Run chaos engineering tests (k8s kind cluster)
- Bug bash + UX polish

---

## Phase G — Beta: zk-Moderation, Bots, Mobile (W21-W24)

### Goals
Scale community features, dev surface, mobile preview.

### Deliverables
- **zk-Moderation**: Rule templates (rate, membership, time windows), admin UI
- **Capsule bots**: WASI runtime, local-only default, zkVM optional
- **Mobile preview**: UniFFI bindings, Android test build
- **Tests**: 5 communities with distinct policies, bot compliance proofs

### Tasks
- Build admin UI for policy management
- WASI bot runtime integration
- UniFFI bindings for iOS/Android
- Publish bot examples (translate, summarize, etc.)

---

## Phase H — v1 Readiness: Audits & Launch (W25-W28)

### Goals
Security review, hardening, launch playbook.

### Deliverables
- Internal audit (crypto, key lifecycle, state erase, circuits)
- External review (2–3 weeks): MLS hybrid, RLN, HPKE wrapper
- Incident response playbook: CVE intake, signed updates, key rotation
- Launch: docs, brand assets, "why PQ + zk + P2P" post, live demo

### Tasks
- Fix all P1 audit findings; triage P2s
- Set up update signing key ceremony
- Write launch blog post + press kit
- Prepare live demo with packet capture

---

## Milestone Checklist

### M1 (W2) Foundations ✅
- [x] CI green
- [x] Hello-mesh demo
- [x] Threat model v0.1

### M2 (W6) P2P + Hybrid ✅
- [x] Onion circuits + cover traffic
- [x] Hybrid KEM handshake
- [x] Wire v0.1

### M3 (W9) MLS + Vault ✅
- [x] Groups (member management)
- [x] RAM-only default
- [x] Sealed vault + export/import

### M4 (W13) ZK Layer 🚧
- [ ] RLN proofs in prod path
- [ ] Credential mint α
- [ ] Policy engine (skeleton exists)

### M5 (W16) Privacy Hardening
- [ ] Fixed frames
- [ ] TA harness report
- [ ] Leak audit pass

### M6 (W20) Public Alpha
- [ ] Repro builds
- [ ] Docs/site
- [ ] 500+ peers soak test

### M7 (W24) Beta
- [ ] zk-Moderation UI
- [ ] WASI bots
- [ ] Mobile preview

### M8 (W28) v1
- [ ] External audit fixes
- [ ] Incident response playbook
- [ ] Launch assets

---

## Current Sprint (W10-W13 - Phase D: ZK Layer)

**Start**: 2024-11-29  
**End**: 2024-12-20 (4 weeks)  
**Progress**: 50% Complete

### Week 10 Achievements ✅
- [x] Implemented Merkle tree for membership (129 lines)
- [x] Enhanced RLN with tree integration (+180 lines)
- [x] Created Groth16 zkSNARK circuit structure (195 lines)
- [x] Built Groth16 prover/verifier wrapper (265 lines)
- [x] Feature-gated arkworks support
- [x] 15/15 basic RLN tests passing
- [x] 32/32 workspace tests passing

### Remaining Tasks (W11-W13)
- [ ] Fix Groth16 circuit constraints (integrate Poseidon hash)
- [ ] Credential issuance with committee threshold
- [ ] Proof caching layer for performance
- [ ] Integration with umbra-net messaging
- [ ] Spam simulation tests (1k msg/min botnet)
- [ ] Performance optimization (<1.5s proofs)
- [ ] Complete `post_with_proof()` SDK API

### Known Issues
1. **Groth16 Circuit** (3 tests fail with arkworks feature)
   - Status: Using simplified hash instead of Poseidon
   - Workaround: Use default SHA256 mode
   - Timeline: Fix in Week 11

2. **Proof Performance** (2-3s vs target <1.5s)
   - Status: Circuit needs optimization
   - Strategy: GPU support, circuit simplification
   - Timeline: Week 11-12

### Blockers
None currently. All critical paths have working fallbacks.

---

## How to Test

See [TESTING.md](./TESTING.md) for comprehensive testing guide.

**Quick Test:**
```bash
cargo test --workspace          # All tests (33 tests, ~11s)
cargo run --example hello_mesh  # P2P demo
cargo run --example simple_chat # Messaging demo
```

**Current Test Status:**
- ✅ 32/32 tests passing (1 ignored)
- ✅ CI green on all platforms
- ✅ All critical functionality working
- 🚧 3 arkworks tests pending circuit fix

---

**Questions or suggestions?** Open a discussion or check [CURRENT_STATUS.md](./CURRENT_STATUS.md) for detailed metrics.
