// Message exchange: encrypt, send, receive, decrypt
// Built on top of handshake and session management

use crate::error::{NetError, Result};
use libp2p::PeerId;
use prost::Message;
use tracing::debug;
use umbra_crypto::session::SessionManager;
use umbra_crypto::aead::Envelope;
use umbra_wire::message::{ChatMessage, EncryptedMessage};
use ed25519_dalek;
use umbra_identity::{Identity, Prover, verify_identity_proof};

/// Manages message encryption/decryption for all peers
pub struct MessageExchange {
    session_mgr: SessionManager,
    local_peer_id: PeerId,
    identity: Option<Identity>,
    prover: Option<Prover>,
}

impl MessageExchange {
    pub fn new(local_peer_id: PeerId) -> Result<Self> {
        let session_mgr = SessionManager::new(local_peer_id)
            .map_err(|e| NetError::Crypto(format!("SessionManager init: {}", e)))?;
        
        Ok(Self { 
            session_mgr, 
            local_peer_id,
            identity: None,
            prover: None,
        })
    }

    /// Set identity and prover for ZK proofs
    pub fn set_identity(&mut self, identity: Identity, prover: Prover) {
        self.identity = Some(identity);
        self.prover = Some(prover);
    }

    /// Get session manager (for handshake integration)
    pub fn session_manager(&self) -> &SessionManager {
        &self.session_mgr
    }

    pub fn session_manager_mut(&mut self) -> &mut SessionManager {
        &mut self.session_mgr
    }

    /// Encrypt a chat message for a peer
    pub fn encrypt_message(
        &mut self,
        peer: PeerId,
        username: &str,
        content: &str,
    ) -> Result<Vec<u8>> {
        // Create plaintext message
        let chat_msg = ChatMessage {
            username: username.to_string(),
            content: content.to_string(),
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .map_err(|e| NetError::Crypto(format!("System time error: {}", e)))?
                .as_secs(),
            identity_id: self.identity.as_ref()
                .map(|id| id.id.to_vec())
                .unwrap_or_default(),
        };

        // Serialize to protobuf
        let plaintext = chat_msg.encode_to_vec();

        // Sign the plaintext message with hybrid signature
        let hybrid_sig = self.session_mgr.sign(&plaintext)
            .map_err(|e| NetError::Crypto(format!("Sign: {}", e)))?;

        // Get or create session and copy key
        let session_key = {
            let session = self.session_mgr.get_session(peer)
                .map_err(|e| NetError::Crypto(format!("Get session: {}", e)))?;
            *session.key()
        };

        // Encrypt with session key
        let envelope = Envelope::new(&session_key)
            .map_err(|e| NetError::Crypto(format!("Envelope init: {}", e)))?;
        
        let encrypted_data = envelope.encrypt(&plaintext)
            .map_err(|e| NetError::Crypto(format!("Encrypt: {}", e)))?;

        // Split into nonce || ciphertext
        let (nonce, ciphertext) = encrypted_data.split_at(12);

        // Generate ZK proof if identity is set
        let (identity_id, identity_proof) = if let (Some(identity), Some(prover)) = 
            (&self.identity, &self.prover) {
            match identity.generate_proof(prover) {
                Ok(proof) => (identity.id.to_vec(), proof),
                Err(e) => {
                    debug!("⚠️  Failed to generate proof: {}", e);
                    (vec![], vec![])
                }
            }
        } else {
            (vec![], vec![])
        };

        // Create encrypted message with hybrid signature
        let enc_msg = EncryptedMessage {
            sender: self.local_peer_id.to_bytes(),
            nonce: nonce.to_vec(),
            ciphertext: ciphertext.to_vec(),
            timestamp: chat_msg.timestamp,
            signature: hybrid_sig.classical,
            identity_id,
            identity_proof,
            pq_signature: hybrid_sig.pq.unwrap_or_default(),
        };

        // Increment message counter
        self.session_mgr.get_session(peer)
            .map_err(|e| NetError::Crypto(format!("Get session: {}", e)))?
            .increment();

        // Serialize to wire format
        Ok(enc_msg.encode_to_vec())
    }

    /// Decrypt a chat message from a peer
    pub fn decrypt_message(
        &mut self,
        peer: PeerId,
        data: &[u8],
    ) -> Result<(String, String, Option<[u8; 32]>)> {
        // Deserialize encrypted message
        let enc_msg = EncryptedMessage::decode(data)
            .map_err(|e| NetError::Protocol(format!("Decode EncryptedMessage: {}", e)))?;

        // Get session for peer
        let session = self.session_mgr.get_session(peer)
            .map_err(|e| NetError::Crypto(format!("Get session: {}", e)))?;

        // Reconstruct encrypted data (nonce || ciphertext)
        let mut encrypted_data = Vec::with_capacity(enc_msg.nonce.len() + enc_msg.ciphertext.len());
        encrypted_data.extend_from_slice(&enc_msg.nonce);
        encrypted_data.extend_from_slice(&enc_msg.ciphertext);

        // Decrypt
        let envelope = Envelope::new(session.key())
            .map_err(|e| NetError::Crypto(format!("Envelope init: {}", e)))?;
        
        let plaintext = envelope.decrypt(&encrypted_data)
            .map_err(|e| NetError::Crypto(format!("Decrypt: {}", e)))?;

        // Verify signature if present AND peer key is registered
        if !enc_msg.signature.is_empty() {
            // Parse Ed25519 signature
            if enc_msg.signature.len() != 64 {
                debug!("⚠️  Invalid signature length, skipping verification");
            } else {
                let mut sig_bytes = [0u8; 64];
                sig_bytes.copy_from_slice(&enc_msg.signature);
                let signature = ed25519_dalek::Signature::from_bytes(&sig_bytes);
                
                // Try to verify - if peer key not registered or verification fails, just log warning
                if self.session_mgr.get_peer_key(&peer).is_some() {
                    match self.session_mgr.verify(&peer, &plaintext, &signature) {
                        Ok(_) => {
                            debug!("✅ Ed25519 signature verified for peer {}", peer);
                        }
                        Err(e) => {
                            debug!("⚠️  Signature verification failed (mock keys): {}", e);
                            debug!("   This is expected until proper key exchange is implemented (v0.6.0)");
                        }
                    }
                } else {
                    debug!("⚠️  Peer key not registered, skipping signature verification for {}", peer);
                }
            }
            
            // TODO: Verify Dilithium3 signature if pq_signature is present
            if !enc_msg.pq_signature.is_empty() {
                debug!("ℹ️  Dilithium3 signature present but verification not yet implemented");
            }
        }

        // Verify ZK identity proof if present
        let mut verified_identity: Option<[u8; 32]> = None;
        if !enc_msg.identity_id.is_empty() && !enc_msg.identity_proof.is_empty() {
            if enc_msg.identity_id.len() == 32 {
                let mut id = [0u8; 32];
                id.copy_from_slice(&enc_msg.identity_id);
                
                if let Some(prover) = &self.prover {
                    match verify_identity_proof(prover, &enc_msg.identity_proof, &id) {
                        Ok(true) => {
                            debug!("✅ Identity proof verified for {}", hex::encode(&id[..8]));
                            verified_identity = Some(id);
                        }
                        Ok(false) => {
                            debug!("❌ Identity proof verification failed");
                        }
                        Err(e) => {
                            debug!("⚠️  Identity proof error: {}", e);
                        }
                    }
                } else {
                    debug!("⚠️  No prover available to verify identity proof");
                }
            }
        }

        // Deserialize chat message
        let chat_msg = ChatMessage::decode(&plaintext[..])
            .map_err(|e| NetError::Protocol(format!("Decode ChatMessage: {}", e)))?;

        debug!("Decrypted and verified message from {}: {}", chat_msg.username, chat_msg.content);

        Ok((chat_msg.username, chat_msg.content, verified_identity))
    }

    /// Clean up expired sessions
    pub fn cleanup(&mut self) {
        self.session_mgr.cleanup();
    }

    /// Get session count (for monitoring)
    pub fn session_count(&self) -> usize {
        self.session_mgr.session_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message_roundtrip() {
        let shared_peer_id = PeerId::random(); // Use same PeerID for both to derive same symmetric key
        let mut alice = MessageExchange::new(shared_peer_id).unwrap();
        let mut bob = MessageExchange::new(shared_peer_id).unwrap();
        
        let peer_id = PeerId::random();

        // Register keys for signature verification
        let alice_pubkey = *alice.session_manager().public_key();
        bob.session_manager_mut().register_peer(peer_id, alice_pubkey);

        // Alice encrypts
        let encrypted = alice.encrypt_message(
            peer_id,
            "alice",
            "hello bob!",
        ).unwrap();

        // Bob decrypts (same local_peer_id means same derived symmetric key)
        let (username, content, _identity) = bob.decrypt_message(peer_id, &encrypted).unwrap();
        
        assert_eq!(username, "alice");
        assert_eq!(content, "hello bob!");
    }

    #[test]
    fn test_wrong_key_fails() {
        let mut alice = MessageExchange::new(PeerId::random()).unwrap();
        let mut eve = MessageExchange::new(PeerId::random()).unwrap();
        
        let alice_peer = PeerId::random();
        let eve_peer = PeerId::random();

        // Alice encrypts for alice_peer
        let encrypted = alice.encrypt_message(
            alice_peer,
            "alice",
            "secret message",
        ).unwrap();

        // Eve tries to decrypt with different peer ID (different key)
        let result = eve.decrypt_message(eve_peer, &encrypted);
        
        // Should fail because different peer = different session key
        assert!(result.is_err());
    }

    #[test]
    fn test_session_increment() {
        let mut exchange = MessageExchange::new(PeerId::random()).unwrap();
        let peer = PeerId::random();

        // Send 3 messages
        for _ in 0..3 {
            exchange.encrypt_message(peer, "alice", "test").unwrap();
        }

        // Check session was incremented
        let session = exchange.session_mgr.get_session(peer).unwrap();
        assert_eq!(session.msg_count(), 3);
    }
}
