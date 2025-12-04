# UMBRA Development Roadmap

**Last Updated**: December 4, 2024  
**Current Version**: v0.6.1  
**Status**: CLI Ready → ZK Identity (Groth16) → v1.0

---

## 🎯 Current State (v0.6.1)

**What Works**:
- ✅ CLI messenger (production-ready)
- ✅ Quantum-safe crypto (ML-KEM768 + Dilithium3)
- ✅ P2P networking (libp2p + QUIC)
- ✅ Encrypted messaging (ChaCha20-Poly1305)
- ✅ 87 tests passing
- ✅ 5,741 LOC across 7 crates

**What's Missing**:
- ❌ Persistent identity (restarts = new identity)
- ❌ Multi-device support (deferred to v0.9)

---

## 🚀 v0.7 — ZK Identity (Groth16) 📋 NEXT

**Target**: 3-4 weeks (December 5 - January 2, 2025)  
**Goal**: Add zero-knowledge verified identities

### What We're Building

```
User creates identity:
  secret_seed (32 bytes) ← password
  identity_id = Hash(secret_seed)
  
On connect:
  Generate ZK proof: "I know secret that hashes to identity_id"
  Network verifies WITHOUT learning secret
```

**Why Groth16**:
- ✅ True zero-knowledge
- ✅ Small proofs (~200 bytes)
- ✅ Fast verification (~10ms)
- ✅ Production-grade

---

### Week 1: Foundation (Dec 5-11)

**Days 1-2: Setup**
- [ ] Add arkworks dependencies
- [ ] Study Poseidon hash
- [ ] Proof-of-concept circuit

**Days 3-4: Identity Core**
```rust
// crates/umbra-identity/src/identity.rs
- [ ] Identity struct
- [ ] Password → secret (Argon2)
- [ ] identity_id = Hash(secret)
- [ ] Encrypted vault save/load
- [ ] Unit tests (10+)
```

**Days 5-6: ZK Circuit**
```rust
// crates/umbra-identity/src/circuit.rs
- [ ] IdentityCircuit
- [ ] Poseidon hash gadget
- [ ] Constraint: Hash(secret) == identity_id
```

---

### Week 2: Groth16 (Dec 12-18)

**Days 1-2: Setup**
```rust
// crates/umbra-identity/src/setup.rs
- [ ] Trusted setup
- [ ] Generate proving/verification keys
```

**Days 3-4: Prover**
```rust
// crates/umbra-identity/src/prover.rs
- [ ] prove(secret, identity_id) → Proof
- [ ] Benchmark (<150ms target)
```

**Days 5-6: Verifier**
```rust
// crates/umbra-identity/src/verifier.rs
- [ ] verify(proof, identity_id) → bool
- [ ] Integration tests
```

---

### Week 3: Integration (Dec 19-25)

**Days 1-2: Wire Protocol**
- [ ] Add identity_id to ChatMessage
- [ ] IdentityAnnouncement message

**Days 3-4: Network**
- [ ] SessionManager + identity_id
- [ ] Include identity in messages
- [ ] Verify on receipt

**Days 5-6: CLI**
- [ ] `identity create` command
- [ ] `identity show` command
- [ ] Auto-load on `start`
- [ ] `--no-identity` flag

---

### Week 4: Testing (Dec 26-31)

**Days 1-2: Integration Tests**
- [ ] Two identities chat locally
- [ ] Identity persists across restarts
- [ ] Anonymous mode still works

**Days 3-4: Performance**
- [ ] Optimize proving (<100ms)
- [ ] Audit secret handling
- [ ] Test invalid proofs

**Days 5-6: Documentation**
- [ ] Update README
- [ ] Update CLI_USER_GUIDE
- [ ] Write ZK_IDENTITY.md
- [ ] Demo video

---

## 📦 Code Estimates

```
New Crate: umbra-identity/
├── identity.rs      150 LOC
├── circuit.rs       200 LOC
├── setup.rs         100 LOC
├── prover.rs        150 LOC
├── verifier.rs      100 LOC
├── error.rs          50 LOC
└── lib.rs            50 LOC
Total:              ~800 LOC

Modifications:
├── umbra-wire       +50 LOC
├── umbra-crypto    +100 LOC
├── umbra-net        +50 LOC
├── apps/cli        +200 LOC
Total:              +400 LOC

Grand Total:       ~1,200 LOC
Tests:               ~20 new
```

---

## ✅ Acceptance Criteria

**Must Have**:
- ✅ Create identity with password
- ✅ Identity saved to encrypted vault
- ✅ Loads on restart (same identity_id)
- ✅ ZK proof generated (<150ms)
- ✅ Network verifies proof
- ✅ Messages show identity_id
- ✅ Backwards compatible (--no-identity)

**Performance**:
- ✅ Proving: <150ms
- ✅ Verification: <20ms
- ✅ Proof size: <300 bytes

---

## 📅 Milestones

| Version | Feature | Target |
|---------|---------|--------|
| v0.6.1 | CLI Ready | ✅ Dec 4 |
| v0.7.0 | ZK Identity | 📋 Jan 2 |
| v0.8.0 | History Sync | 📋 Jan 23 |
| v0.9.0 | Multi-Device | 📋 Feb 20 |
| v1.0.0 | Production | 🎯 Mar 6 |

---

## 🚫 Out of Scope (v0.7)

**Not implementing**:
- ❌ Multi-device (v0.9)
- ❌ Device registry (v0.9)
- ❌ QR code pairing (v0.9)
- ❌ Chat history (v0.8)

---

## 🎯 This Week (Dec 5-11)

**Day 1**: Create umbra-identity crate, add arkworks deps  
**Day 2**: Implement password → secret derivation  
**Day 3**: Start ZK circuit, research Poseidon  

---

**Focus**: ZK Identity (Groth16). Ship by January 2, 2025.

*Last updated: December 4, 2024*
