// Message exchange: encrypt, send, receive, decrypt
// Built on top of handshake and session management

use crate::error::{NetError, Result};
use libp2p::PeerId;
use prost::Message;
use std::collections::HashMap;
use tracing::{debug, warn};
use umbra_crypto::session::{SessionKey, SessionManager};
use umbra_crypto::aead::Envelope;
use umbra_wire::message::{ChatMessage, EncryptedMessage};
use ed25519_dalek;

/// Manages message encryption/decryption for all peers
pub struct MessageExchange {
    session_mgr: SessionManager,
}

impl MessageExchange {
    pub fn new() -> Result<Self> {
        let session_mgr = SessionManager::new()
            .map_err(|e| NetError::Crypto(format!("SessionManager init: {}", e)))?;
        
        Ok(Self { session_mgr })
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
                .unwrap()
                .as_secs(),
        };

        // Serialize to protobuf
        let plaintext = chat_msg.encode_to_vec();

        // Sign the plaintext message (username || content || timestamp)
        let signature = self.session_mgr.sign(&plaintext);

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

        // Create encrypted message
        let enc_msg = EncryptedMessage {
            sender: peer.to_bytes(),
            nonce: nonce.to_vec(),
            ciphertext: ciphertext.to_vec(),
            timestamp: chat_msg.timestamp,
            signature: signature.to_bytes().to_vec(),
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
    ) -> Result<(String, String)> {
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

        // Verify signature if present
        if !enc_msg.signature.is_empty() {
            // Parse signature
            if enc_msg.signature.len() != 64 {
                return Err(NetError::Crypto("Invalid signature length".to_string()));
            }
            
            let mut sig_bytes = [0u8; 64];
            sig_bytes.copy_from_slice(&enc_msg.signature);
            let signature = ed25519_dalek::Signature::from_bytes(&sig_bytes);
            
            // Verify against plaintext
            self.session_mgr.verify(&peer, &plaintext, &signature)
                .map_err(|e| NetError::Crypto(format!("Signature verification failed: {}", e)))?;
        }

        // Deserialize chat message
        let chat_msg = ChatMessage::decode(&plaintext[..])
            .map_err(|e| NetError::Protocol(format!("Decode ChatMessage: {}", e)))?;

        debug!("Decrypted and verified message from {}: {}", chat_msg.username, chat_msg.content);

        Ok((chat_msg.username, chat_msg.content))
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
        let mut alice = MessageExchange::new().unwrap();
        let mut bob = MessageExchange::new().unwrap();
        
        let peer_id = PeerId::random();

        // Register keys for signature verification
        let alice_pubkey = alice.session_manager().public_key();
        bob.session_manager_mut().register_peer(peer_id, alice_pubkey);

        // Alice encrypts
        let encrypted = alice.encrypt_message(
            peer_id,
            "alice",
            "hello bob!",
        ).unwrap();

        // Bob decrypts (same peer ID means same derived key)
        let (username, content) = bob.decrypt_message(peer_id, &encrypted).unwrap();
        
        assert_eq!(username, "alice");
        assert_eq!(content, "hello bob!");
    }

    #[test]
    fn test_wrong_key_fails() {
        let mut alice = MessageExchange::new().unwrap();
        let mut eve = MessageExchange::new().unwrap();
        
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
        let mut exchange = MessageExchange::new().unwrap();
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
