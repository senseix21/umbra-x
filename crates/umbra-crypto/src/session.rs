// Simple session key management
// No bullshit, just keys mapped to peers

use crate::error::Result;
use crate::identity::IdentityKey;
use ed25519_dalek::VerifyingKey;
use libp2p::PeerId;
use std::collections::HashMap;
use std::time::{Duration, Instant};
use zeroize::Zeroize;

const SESSION_TIMEOUT: Duration = Duration::from_secs(24 * 3600); // 24 hours
const MAX_SESSIONS: usize = 1000; // Memory limit

/// Session key for a peer
pub struct SessionKey {
    key: [u8; 32],
    created: Instant,
    msg_count: u64,
}

impl SessionKey {
    fn new(key: [u8; 32]) -> Self {
        Self {
            key,
            created: Instant::now(),
            msg_count: 0,
        }
    }

    pub fn key(&self) -> &[u8; 32] {
        &self.key
    }

    pub fn age(&self) -> Duration {
        self.created.elapsed()
    }

    pub fn increment(&mut self) {
        self.msg_count += 1;
    }

    pub fn msg_count(&self) -> u64 {
        self.msg_count
    }

    pub fn should_rotate(&self) -> bool {
        self.msg_count >= 1000 || self.age() >= SESSION_TIMEOUT
    }
}

impl Drop for SessionKey {
    fn drop(&mut self) {
        self.key.zeroize();
    }
}

/// Manages session keys for all peers
pub struct SessionManager {
    identity: IdentityKey,
    // Note: KEM instance removed as it's created per-handshake, not reused globally
    sessions: HashMap<PeerId, SessionKey>,
    peer_keys: HashMap<PeerId, VerifyingKey>,
    local_peer_id: PeerId,
}

impl SessionManager {
    pub fn new(local_peer_id: PeerId) -> Result<Self> {
        let identity = IdentityKey::generate()?;
        Ok(Self {
            identity,
            sessions: HashMap::new(),
            peer_keys: HashMap::new(),
            local_peer_id,
        })
    }

    /// Get our public identity key (for sharing with peers)
    pub fn public_key(&self) -> &VerifyingKey {
        self.identity.verifying_key()
    }

    /// Register a peer's public key
    pub fn register_peer(&mut self, peer: PeerId, verify_key: VerifyingKey) {
        self.peer_keys.insert(peer, verify_key);
    }

    /// Get peer's public key for verification
    pub fn get_peer_key(&self, peer: &PeerId) -> Option<&VerifyingKey> {
        self.peer_keys.get(peer)
    }

    /// Sign data with our identity key (returns hybrid signature)
    pub fn sign(&self, data: &[u8]) -> Result<crate::identity::HybridSignature> {
        self.identity.sign(data)
    }

    /// Verify signature from a peer (classical Ed25519 only for now)
    pub fn verify(&self, peer: &PeerId, data: &[u8], signature: &ed25519_dalek::Signature) -> Result<()> {
        use ed25519_dalek::Verifier;
        let peer_key = self.peer_keys.get(peer)
            .ok_or(crate::error::CryptoError::KeyDerivation(
                "Peer key not registered".to_string()
            ))?;
        
        peer_key.verify(data, signature)
            .map_err(|e| crate::error::CryptoError::InvalidSignature(e.to_string()))?;
        
        Ok(())
    }

    /// Get or create session for peer
    pub fn get_session(&mut self, peer: PeerId) -> Result<&mut SessionKey> {
        // Check if exists and still valid
        if let Some(session) = self.sessions.get(&peer) {
            if !session.should_rotate() {
                // Safe: we just checked the session exists
                return Ok(self.sessions.get_mut(&peer)
                    .expect("session exists from previous get check"));
            }
            // Expired, remove it
            self.sessions.remove(&peer);
        }

        // For now, derive from peer ID (handshake will replace this)
        // TODO: Initiate handshake here instead
        let key = self.derive_session_key(&peer);
        self.sessions.insert(peer, SessionKey::new(key));

        // Enforce memory limit
        if self.sessions.len() > MAX_SESSIONS {
            self.evict_oldest();
        }

        // Safe: we just inserted the session above
        Ok(self.sessions.get_mut(&peer)
            .expect("session exists from insert above"))
    }

    /// Set session key from handshake (replaces symmetric derivation)
    pub fn set_session_key(&mut self, peer: PeerId, key: [u8; 32]) {
        self.sessions.insert(peer, SessionKey::new(key));
        
        // Enforce memory limit
        if self.sessions.len() > MAX_SESSIONS {
            self.evict_oldest();
        }
    }

    /// Temporary: derive key from both peer IDs (symmetric)
    /// TODO: Replace with proper handshake key exchange
    fn derive_session_key(&self, peer: &PeerId) -> [u8; 32] {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(b"umbra-session-v1");
        
        // Create symmetric key by ordering peer IDs
        let peer1 = self.local_peer_id.to_bytes();
        let peer2 = peer.to_bytes();
        
        // Sort to ensure same order on both sides
        if peer1.as_slice() < peer2.as_slice() {
            hasher.update(&peer1);
            hasher.update(&peer2);
        } else {
            hasher.update(&peer2);
            hasher.update(&peer1);
        }
        
        hasher.finalize().into()
    }

    /// Remove oldest session when over limit
    fn evict_oldest(&mut self) {
        if let Some((oldest_peer, _)) = self
            .sessions
            .iter()
            .max_by_key(|(_, s)| s.created.elapsed())
        {
            let peer = *oldest_peer;
            self.sessions.remove(&peer);
        }
    }

    /// Clean up expired sessions
    pub fn cleanup(&mut self) {
        self.sessions.retain(|_, s| !s.should_rotate());
    }

    pub fn session_count(&self) -> usize {
        self.sessions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_session_creation() {
        let mut mgr = SessionManager::new(PeerId::random()).unwrap();
        let peer = PeerId::random();
        
        let session = mgr.get_session(peer).unwrap();
        assert_eq!(session.msg_count, 0);
    }

    #[test]
    fn test_session_reuse() {
        let mut mgr = SessionManager::new(PeerId::random()).unwrap();
        let peer = PeerId::random();
        
        let key1 = *mgr.get_session(peer).unwrap().key();
        let key2 = *mgr.get_session(peer).unwrap().key();
        
        assert_eq!(key1, key2); // Same session
    }

    #[test]
    fn test_different_peers_different_keys() {
        let mut mgr = SessionManager::new(PeerId::random()).unwrap();
        let peer1 = PeerId::random();
        let peer2 = PeerId::random();
        
        let key1 = *mgr.get_session(peer1).unwrap().key();
        let key2 = *mgr.get_session(peer2).unwrap().key();
        
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_rotation_on_count() {
        let mut mgr = SessionManager::new(PeerId::random()).unwrap();
        let peer = PeerId::random();
        
        let session = mgr.get_session(peer).unwrap();
        session.msg_count = 1000;
        
        assert!(session.should_rotate());
    }

    #[test]
    fn test_cleanup() {
        let mut mgr = SessionManager::new(PeerId::random()).unwrap();
        let peer = PeerId::random();
        
        let session = mgr.get_session(peer).unwrap();
        session.msg_count = 1000; // Force expiry
        
        mgr.cleanup();
        assert_eq!(mgr.session_count(), 0);
    }

    #[test]
    fn test_identity_management() {
        let mgr = SessionManager::new(PeerId::random()).unwrap();
        let pk = mgr.public_key();
        
        // Should be able to get public key
        assert_eq!(pk.as_bytes().len(), 32);
    }

    #[test]
    fn test_peer_registration() {
        let mut mgr = SessionManager::new(PeerId::random()).unwrap();
        let peer = PeerId::random();
        let peer_key = ed25519_dalek::SigningKey::from_bytes(&rand::random()).verifying_key();
        
        mgr.register_peer(peer, peer_key);
        assert!(mgr.peer_keys.contains_key(&peer));
    }

    #[test]
    fn test_session_increment() {
        let mut mgr = SessionManager::new(PeerId::random()).unwrap();
        let peer = PeerId::random();
        
        let session = mgr.get_session(peer).unwrap();
        assert_eq!(session.msg_count, 0);
        
        session.increment();
        assert_eq!(session.msg_count, 1);
        
        session.increment();
        assert_eq!(session.msg_count, 2);
    }

    #[test]
    fn test_session_age() {
        let mut mgr = SessionManager::new(PeerId::random()).unwrap();
        let peer = PeerId::random();
        
        let session = mgr.get_session(peer).unwrap();
        let age = session.age();
        
        // Should be very recent
        assert!(age.as_secs() < 1);
    }

    #[test]
    fn test_max_sessions_eviction() {
        let mut mgr = SessionManager::new(PeerId::random()).unwrap();
        
        // Create MAX_SESSIONS + 1 sessions
        for _ in 0..=MAX_SESSIONS {
            let peer = PeerId::random();
            mgr.get_session(peer).unwrap();
        }
        
        // Should have evicted oldest
        assert_eq!(mgr.session_count(), MAX_SESSIONS);
    }

    #[test]
    fn test_session_expiry_and_cleanup() {
        let mut mgr = SessionManager::new(PeerId::random()).unwrap();
        let peer = PeerId::random();
        
        // Get initial session
        let session = mgr.get_session(peer).unwrap();
        session.msg_count = 1000; // Mark for expiry
        
        // Cleanup should remove it
        mgr.cleanup();
        assert_eq!(mgr.session_count(), 0);
        
        // New session created on next access
        let session2 = mgr.get_session(peer).unwrap();
        assert_eq!(session2.msg_count, 0); // Fresh session
    }

    // Handshake methods removed - now in HandshakeBehaviour
    /*
    #[test]
    fn test_handshake_initiate() {
        let mgr = SessionManager::new(PeerId::random()).unwrap();
        let peer = PeerId::random();
        
        let init = mgr.initiate_handshake(peer).unwrap();
        assert_eq!(init.peer_id, peer.to_bytes());
        assert_eq!(init.x25519_pk.len(), 32);
        assert_eq!(init.signature.len(), 64);
    }

    #[test]
    fn test_handshake_respond_without_peer_key() {
        let mut mgr = SessionManager::new(PeerId::random()).unwrap();
        let peer = PeerId::random();
        
        let init = HandshakeInit {
            peer_id: peer.to_bytes(),
            x25519_pk: [0u8; 32],
            signature: [0u8; 64],
            verify_key: [0u8; 32],
        };
        
        // Should fail - peer not registered
        let result = mgr.respond_handshake(peer, &init);
        assert!(result.is_err());
    }
    */

    #[test]
    fn test_multiple_peer_sessions() {
        let mut mgr = SessionManager::new(PeerId::random()).unwrap();
        let peers: Vec<_> = (0..10).map(|_| PeerId::random()).collect();
        
        // Create sessions for all peers
        for peer in &peers {
            mgr.get_session(*peer).unwrap();
        }
        
        assert_eq!(mgr.session_count(), 10);
        
        // Each should have unique key
        let keys: Vec<_> = peers.iter()
            .map(|p| *mgr.get_session(*p).unwrap().key())
            .collect();
        
        for i in 0..keys.len() {
            for j in i+1..keys.len() {
                assert_ne!(keys[i], keys[j]);
            }
        }
    }

    #[test]
    fn test_session_key_deterministic() {
        let mut mgr = SessionManager::new(PeerId::random()).unwrap();
        let peer = PeerId::random();
        
        // Same peer should get same key (until rotation)
        let key1 = *mgr.get_session(peer).unwrap().key();
        let key2 = *mgr.get_session(peer).unwrap().key();
        let key3 = *mgr.get_session(peer).unwrap().key();
        
        assert_eq!(key1, key2);
        assert_eq!(key2, key3);
    }
}
