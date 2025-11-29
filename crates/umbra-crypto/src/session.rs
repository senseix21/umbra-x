// Simple session key management
// No bullshit, just keys mapped to peers

use crate::error::Result;
use crate::kem::HybridKem;
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
    kem: HybridKem,
    sessions: HashMap<PeerId, SessionKey>,
}

impl SessionManager {
    pub fn new() -> Result<Self> {
        Ok(Self {
            kem: HybridKem::generate()?,
            sessions: HashMap::new(),
        })
    }

    /// Get or create session for peer
    pub fn get_session(&mut self, peer: PeerId) -> Result<&mut SessionKey> {
        // Check if exists and still valid
        if let Some(session) = self.sessions.get(&peer) {
            if !session.should_rotate() {
                return Ok(self.sessions.get_mut(&peer).unwrap());
            }
            // Expired, remove it
            self.sessions.remove(&peer);
        }

        // Create new session (will implement handshake later)
        // For now, derive from peer ID (like topic-based, but per-peer)
        let key = self.derive_session_key(&peer);
        self.sessions.insert(peer, SessionKey::new(key));

        // Enforce memory limit
        if self.sessions.len() > MAX_SESSIONS {
            self.evict_oldest();
        }

        Ok(self.sessions.get_mut(&peer).unwrap())
    }

    /// Temporary: derive key from peer ID
    /// TODO: Replace with proper handshake
    fn derive_session_key(&self, peer: &PeerId) -> [u8; 32] {
        use sha2::{Digest, Sha256};
        let mut hasher = Sha256::new();
        hasher.update(b"umbra-session-v1");
        hasher.update(peer.to_bytes());
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
        let mut mgr = SessionManager::new().unwrap();
        let peer = PeerId::random();
        
        let session = mgr.get_session(peer).unwrap();
        assert_eq!(session.msg_count, 0);
    }

    #[test]
    fn test_session_reuse() {
        let mut mgr = SessionManager::new().unwrap();
        let peer = PeerId::random();
        
        let key1 = *mgr.get_session(peer).unwrap().key();
        let key2 = *mgr.get_session(peer).unwrap().key();
        
        assert_eq!(key1, key2); // Same session
    }

    #[test]
    fn test_different_peers_different_keys() {
        let mut mgr = SessionManager::new().unwrap();
        let peer1 = PeerId::random();
        let peer2 = PeerId::random();
        
        let key1 = *mgr.get_session(peer1).unwrap().key();
        let key2 = *mgr.get_session(peer2).unwrap().key();
        
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_rotation_on_count() {
        let mut mgr = SessionManager::new().unwrap();
        let peer = PeerId::random();
        
        let session = mgr.get_session(peer).unwrap();
        session.msg_count = 1000;
        
        assert!(session.should_rotate());
    }

    #[test]
    fn test_cleanup() {
        let mut mgr = SessionManager::new().unwrap();
        let peer = PeerId::random();
        
        let session = mgr.get_session(peer).unwrap();
        session.msg_count = 1000; // Force expiry
        
        mgr.cleanup();
        assert_eq!(mgr.session_count(), 0);
    }
}
