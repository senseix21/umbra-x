# UMBRA Development Roadmap (CLI-Focused)

**Status**: CLI Production Hardening  
**Last Updated**: 2024-11-29  
**Focus**: Making CLI production-ready for v1.0 launch  
**Progress**: CLI MVP ✅ | Security Hardening 🚧 | Public Alpha 📋

## 🎯 Mission: Production-Ready CLI Chat

**Primary Goal:** Ship a secure, usable P2P CLI messenger (like `irssi` but P2P + encrypted)

**What Works Now (CLI v0.1 - MVP):**
- ✅ Real-time P2P messaging (libp2p + QUIC)
- ✅ Group chat (2+ users via GossipSub)
- ✅ End-to-end encryption (ChaCha20-Poly1305)
- ✅ Clean terminal interface (492 LOC)
- ✅ Commands: `/help`, `/peers`, `/quit`, `/clear`
- ✅ Zero servers, zero logs

**Current Focus (CLI v0.2 - Security):**
- 🚧 Per-peer encryption (replace topic-based keys)
- 🚧 Session key exchange (X25519 + ML-KEM hybrid)
- 🚧 Forward secrecy (DH ratcheting)
- 🚧 Onion routing activation (3-hop circuits)

**Next Milestone:** CLI v1.0 Production Release

---

## 🚀 CLI-Focused Development Timeline

**New Strategy:** All development focused on making CLI production-ready

| Version | Timeline | Focus | Status |
|---------|----------|-------|--------|
| **CLI v0.1** | ✅ Complete | MVP (chat working) | ✅ Shipped |
| **CLI v0.2** | 2-3 weeks | Security hardening | 🚧 Current |
| **CLI v0.3** | 2 weeks | UX polish + features | 📋 Next |
| **CLI v1.0** | 1-2 weeks | Public release | 📋 Planned |

### CLI Version Roadmap

#### CLI v0.1 (MVP) ✅ **COMPLETE**
**Shipped:** November 2024  
**Goal:** Prove the concept works

**Features:**
- ✅ P2P messaging over QUIC
- ✅ Group chat (GossipSub)
- ✅ E2E encryption (basic)
- ✅ Terminal UI with colors
- ✅ Peer discovery (DHT + manual)

**Known Limitations:**
- ⚠️ Topic-based shared keys (dev mode)
- ⚠️ No forward secrecy
- ⚠️ Onion routing not active
- ⚠️ Basic error handling

---

#### CLI v0.2 (Security) 🚧 **IN PROGRESS** 
**Target:** Mid-December 2024 (2-3 weeks)  
**Goal:** Production-grade security

**Priority 1 - Encryption Upgrade:**
- [ ] Per-peer session keys (X25519 + ML-KEM handshake)
- [ ] Forward secrecy (DH ratcheting)
- [ ] Peer authentication (Ed25519 signatures)
- [ ] Session rotation (every 1000 messages or 24h)

**Priority 2 - Privacy Activation:**
- [ ] Activate onion routing (3-hop circuits)
- [ ] Enable cover traffic (Poisson timing)
- [ ] Remove topic-based key derivation
- [ ] Metadata privacy audit

**Priority 3 - Reliability:**
- [ ] Connection retry logic
- [ ] Message queue for offline peers
- [ ] Better error messages
- [ ] Graceful degradation

**Deliverables:**
- [ ] CLI binary with `--secure-mode` flag
- [ ] Security audit document
- [ ] Migration guide (v0.1 → v0.2)
- [ ] Performance benchmarks

---

#### CLI v0.3 (UX Polish) 📋 **PLANNED**
**Target:** Early January 2025 (2 weeks)  
**Goal:** Delightful user experience

**Features:**
- [ ] History/scrollback (last 100 messages)
- [ ] Tab completion for commands
- [ ] Nickname tab completion
- [ ] File transfer (small files <10MB)
- [ ] Desktop notifications
- [ ] Status indicators (typing, online/offline)
- [ ] Multi-channel support (switch with `/join`)
- [ ] Configuration file (`~/.umbra/config.toml`)

**UX Improvements:**
- [ ] Better onboarding (first-run wizard)
- [ ] Inline help (contextual hints)
- [ ] Color themes (dark/light/custom)
- [ ] Message timestamps toggle
- [ ] Sound notifications (optional)

**Deliverables:**
- [ ] User manual (comprehensive)
- [ ] CLI configuration guide
- [ ] Demo videos/GIFs
- [ ] Keyboard shortcuts reference

---

#### CLI v1.0 (Public Release) 📋 **PLANNED**
**Target:** Late January 2025 (1-2 weeks)  
**Goal:** Ship to the world

**Pre-Release Checklist:**
- [ ] All v0.2 + v0.3 features complete
- [ ] Zero known critical bugs
- [ ] Cross-platform tested (macOS, Linux, Windows)
- [ ] Documentation complete
- [ ] Security audit (external review)
- [ ] Reproducible builds
- [ ] Distribution packages (homebrew, apt, etc.)

**Launch Activities:**
- [ ] Press release / blog post
- [ ] Demo video (3-5 min)
- [ ] Reddit/HN announcement
- [ ] GitHub release with binaries
- [ ] Website launch (umbra.chat)

**Success Metrics:**
- 100+ users in first week
- Zero security incidents
- <5% crash rate
- Positive community feedback

---

## Previous Phase History (Foundation)

These phases built the infrastructure that powers the CLI:

| Phase | Status | Purpose | Key Achievement |
|-------|--------|---------|-----------------|
| **A** | ✅ | Foundations | Monorepo, CI/CD, P2P basics |
| **B** | ✅ | P2P + Crypto | QUIC, Hybrid KEM, circuits |
| **C** | ✅ | Groups + Vault | MLS skeleton, encrypted storage |
| **D** | ✅ | ZK Layer | RLN, zkSNARKs, anti-spam |

**All foundational work is COMPLETE.** Future development is CLI-focused.

---

## 📋 CLI v0.2 Detailed Tasks (Current Sprint)

**Sprint Duration:** 2-3 weeks  
**Sprint Goal:** Production-grade security  
**Estimated Effort:** 30-40 hours

### Week 1: Session Key Exchange

**Day 1-2: Handshake Protocol (6-8 hours)**
```
Tasks:
- [ ] Design handshake wire format (protobuf)
- [ ] Implement HandshakeInitiator/Responder
- [ ] Add signature authentication
- [ ] Write handshake tests (unit + integration)

Files to create:
- crates/umbra-net/src/handshake.rs (new)
- crates/umbra-wire/proto/handshake.proto (new)

Expected outcome:
✅ Two peers can exchange X25519 + ML-KEM keys
✅ Derive shared session secret
✅ Handshake completes in <100ms
```

**Day 3-4: Session Management (6-8 hours)**
```
Tasks:
- [ ] Create SessionManager (peer_id → session_key map)
- [ ] Add session expiration (24h timeout)
- [ ] Implement session rotation logic
- [ ] Add tests for session lifecycle

Files to create:
- crates/umbra-crypto/src/session_manager.rs (new)

Expected outcome:
✅ SessionManager tracks per-peer keys
✅ Old sessions auto-expire
✅ Sessions rotate after 1000 messages
```

**Day 5: CLI Integration (4-6 hours)**
```
Tasks:
- [ ] Replace topic-based keys with SessionManager
- [ ] Update ChatSession to use per-peer encryption
- [ ] Add handshake initiation on first message
- [ ] Test multi-peer scenario (3+ users)

Files to modify:
- apps/cli/src/chat.rs

Expected outcome:
✅ CLI uses per-peer session keys
✅ Backward compatible with v0.1 (feature flag)
✅ Manual testing with 3 peers successful
```

---

### Week 2: Privacy Features

**Day 6-7: Activate Onion Routing (4-6 hours)**
```
Tasks:
- [ ] Enable circuit routing in P2PNode
- [ ] Route messages through 3-hop circuits
- [ ] Add circuit failure recovery
- [ ] Test with circuit node crashes

Files to modify:
- crates/umbra-net/src/transport.rs
- crates/umbra-net/src/circuit.rs

Expected outcome:
✅ All messages routed through circuits
✅ <200ms latency overhead
✅ Automatic circuit recovery
```

**Day 8-9: Forward Secrecy (4-6 hours)**
```
Tasks:
- [ ] Implement DH ratchet mechanism
- [ ] Add epoch-based key derivation
- [ ] Zeroize old keys after rotation
- [ ] Test key compromise scenarios

Files to create:
- crates/umbra-crypto/src/ratchet.rs (new)

Expected outcome:
✅ Keys rotate automatically
✅ Old messages not decryptable after rotation
✅ Memory properly zeroized
```

**Day 10: Cover Traffic (2-3 hours)**
```
Tasks:
- [ ] Enable cover traffic in P2PNode
- [ ] Configure Poisson distribution (λ = 0.1)
- [ ] Add cover traffic toggle (CLI flag)
- [ ] Monitor bandwidth overhead

Files to modify:
- crates/umbra-net/src/cover.rs
- apps/cli/src/main.rs (add --cover-traffic flag)

Expected outcome:
✅ Cover traffic sending dummy packets
✅ <10% bandwidth overhead
✅ User can disable if needed
```

---

### Week 3: Testing & Polish

**Day 11-12: Integration Testing (6-8 hours)**
```
Tasks:
- [ ] End-to-end test (5 peers, 100 messages each)
- [ ] Network partition test (peer disconnect/reconnect)
- [ ] Message delivery guarantee test
- [ ] Performance benchmark (latency, throughput)

Files to create:
- apps/cli/tests/integration_test.rs (new)

Expected outcome:
✅ 100% message delivery (5 peers)
✅ Reconnection within 5 seconds
✅ <500ms P50 latency
```

**Day 13-14: Security Audit (4-6 hours)**
```
Tasks:
- [ ] Manual security review (crypto usage)
- [ ] Check for metadata leaks
- [ ] Verify key zeroization
- [ ] Write security assessment doc

Files to create:
- docs/CLI_SECURITY_AUDIT.md (new)

Expected outcome:
✅ No critical vulnerabilities
✅ All keys properly zeroized
✅ Minimal metadata leakage
```

**Day 15: Documentation (2-3 hours)**
```
Tasks:
- [ ] Update CLI_USER_GUIDE.md
- [ ] Write v0.1 → v0.2 migration guide
- [ ] Document new CLI flags
- [ ] Record demo video (3 min)

Files to update:
- CLI_USER_GUIDE.md
- README.md

Expected outcome:
✅ Users can upgrade smoothly
✅ All features documented
✅ Demo shows security improvements
```

---

### CLI v0.2 Acceptance Criteria

**Security:**
- ✅ Per-peer session keys (no shared topic keys)
- ✅ Forward secrecy (DH ratchet)
- ✅ Peer authentication (signatures)
- ✅ Onion routing active (3-hop)

**Performance:**
- ✅ Message latency <500ms P50
- ✅ Handshake latency <100ms
- ✅ Cover traffic overhead <10%
- ✅ Memory usage <100MB per peer

**Reliability:**
- ✅ 99% message delivery (5-peer test)
- ✅ Reconnection within 5s
- ✅ Zero crashes in 1-hour test
- ✅ Graceful error handling

**User Experience:**
- ✅ Backward compatible (feature flag)
- ✅ Clear error messages
- ✅ Smooth upgrade path
- ✅ Documentation complete

---

## 🎯 CLI Feature Backlog (v0.3+)

**High Priority (v0.3):**
- [ ] Message history/scrollback
- [ ] Tab completion (commands, usernames)
- [ ] File transfer (small files)
- [ ] Desktop notifications
- [ ] Configuration file support

**Medium Priority (v0.4):**
- [ ] Multi-channel (IRC-style)
- [ ] Direct messages (1:1 mode)
- [ ] Ignore/block users
- [ ] Message search
- [ ] Export chat logs (encrypted)

**Low Priority (v0.5+):**
- [ ] Plugin system (Lua/WASM)
- [ ] Custom themes
- [ ] Voice messages (recorded audio)
- [ ] Screen sharing (terminal output)
- [ ] Bridge to IRC/Matrix

**Deferred (Future):**
- Desktop UI (Tauri) - separate project
- Mobile apps - separate project
- ZK anti-spam integration - v2.0
- MLS group encryption - when needed for large groups

---

## 📊 Success Metrics

### CLI v0.2 Goals

| Metric | Target | Current | Status |
|--------|--------|---------|--------|
| Security score | 8/10 | 6/10 | 🚧 |
| Latency (P50) | <500ms | ~200ms | ✅ |
| Message delivery | >99% | 100% | ✅ |
| Memory usage | <100MB | ~50MB | ✅ |
| Test coverage | >80% | ~75% | 🚧 |

### CLI v1.0 Launch Goals

| Metric | Target | Status |
|--------|--------|--------|
| GitHub stars | 100+ | 📋 |
| Weekly users | 50+ | 📋 |
| Bug reports | <10 critical | 📋 |
| Security incidents | 0 | 📋 |
| User satisfaction | >80% positive | 📋 |

---

## 🔧 Development Workflow

**Daily:**
1. Pull latest changes (`git pull`)
2. Run tests (`cargo test`)
3. Code + commit small changes
4. Push to branch (`git push`)

**Weekly:**
1. Review roadmap progress
2. Update task estimates
3. Merge to main (if stable)
4. Tag release (if v0.x ready)

**Monthly:**
1. Security review
2. Performance benchmarks
3. User feedback review
4. Roadmap adjustment

---

## 📚 Resources

**Development:**
- [libp2p Rust Docs](https://docs.rs/libp2p)
- [Noise Protocol](http://noiseprotocol.org/)
- [Signal Double Ratchet](https://signal.org/docs/specifications/doubleratchet/)
- [MLS RFC 9420](https://datatracker.ietf.org/doc/rfc9420/)

**Security:**
- [UMBRA Threat Model](./THREAT_MODEL.md)
- [Crypto Best Practices](https://github.com/veorq/cryptocoding)
- [libp2p Security](https://docs.libp2p.io/concepts/security/)

**User Docs:**
- [CLI User Guide](./CLI_USER_GUIDE.md)
- [Testing Guide](./HOW_TO_TEST.md)
- [Group Chat Guide](./GROUP_CHAT_GUIDE.md)

---

## 🎉 Recent Achievements

**November 2024:**
- ✅ CLI MVP shipped (v0.1)
- ✅ Group chat working (2+ users)
- ✅ Encryption fixed (topic-based keys)
- ✅ Visual UI polished (no emojis)
- ✅ Documentation complete (8+ guides)

**What the Team Built:**
- 492 lines of CLI code
- 35/35 tests passing
- 7 core crates (3,706 LOC)
- 57 documentation files
- Zero critical bugs

---

## 🚀 Next Steps

**This Week (Week 1 of Sprint):**
1. Start handshake protocol design
2. Implement SessionManager
3. Begin CLI integration
4. Write integration tests

**This Month (CLI v0.2 Sprint):**
1. Complete security hardening
2. Activate privacy features
3. Pass security audit
4. Release v0.2 beta

**This Quarter (CLI v1.0 Launch):**
1. Add UX polish (v0.3)
2. External security review
3. Public release
4. Community building

---

**Focus:** CLI is the product. Everything else supports the CLI.

**Philosophy:** Ship fast, iterate based on real usage, security first.

**Timeline:** v0.2 in 3 weeks, v1.0 in 8 weeks.

---

*Last updated: 2024-11-29 | Maintained by UMBRA team*

---

## 📜 Appendix: Foundation Phases (Historical)

These phases built the infrastructure that powers the CLI. **All complete ✅**

<details>
<summary><b>Phase A — Foundations (W1-W2)</b></summary>

**Goal:** Monorepo structure, CI/CD, P2P basics

**Delivered:**
- Workspace with 7 core crates
- GitHub Actions CI/CD
- libp2p + QUIC transport
- 2-node P2P demo
- Threat model v0.1

</details>

<details>
<summary><b>Phase B — P2P Core + Hybrid Crypto (W3-W6)</b></summary>

**Goal:** Reliable P2P transport with PQ crypto

**Delivered:**
- Kademlia DHT + GossipSub
- Onion circuit builder (3-hop)
- Cover traffic (Poisson)
- Hybrid KEM (X25519 + ML-KEM-768)
- 50-node swarm test

</details>

<details>
<summary><b>Phase C — Secure Groups (MLS) + Vault (W7-W9)</b></summary>

**Goal:** End-to-end groups + encrypted storage

**Delivered:**
- MLS group state machine
- Epoch-based rekeying
- RAM-only vault
- ChaCha20-Poly1305 sealed storage
- State export/import

</details>

<details>
<summary><b>Phase D — ZK Layer (W10-W13)</b></summary>

**Goal:** Anonymous anti-spam with zero-knowledge

**Delivered:**
- Merkle membership trees
- RLN (Rate-Limit Nullifier)
- Groth16 zkSNARK circuits
- Policy engine for rate limits
- 15/15 tests passing

</details>

---

## 🎯 Frequently Asked Questions

**Q: Why CLI-first instead of desktop/mobile?**  
A: CLI is the fastest path to a working product. Desktop/mobile can come later as separate UIs on top of the same core.

**Q: What about the MLS integration?**  
A: MLS crate is complete but not yet integrated with CLI. Will add when needed for larger groups (10+ users).

**Q: When will ZK anti-spam be active?**  
A: Not a priority for v1.0. Current group chat model is invite-only (no spam risk). ZK becomes important for public channels (v2.0).

**Q: What happened to desktop UI (Tauri)?**  
A: Deferred to separate project. CLI is the core product. Desktop UI will be a wrapper around CLI functionality.

**Q: Is the CLI production-ready now?**  
A: v0.1 is demo-ready. v0.2 will be beta-ready. v1.0 will be production-ready. See roadmap above for timeline.

**Q: How can I help?**  
A: See [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines. Priority areas: testing, documentation, security review.

---

**End of Roadmap**
