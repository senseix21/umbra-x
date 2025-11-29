use crate::aead::Envelope;
use crate::error::Result;
use chacha20poly1305::{aead::OsRng, AeadCore, ChaCha20Poly1305, KeyInit};

/// Simple symmetric encryption for chat messages
/// Uses ChaCha20-Poly1305 AEAD with a shared key
pub struct ChatCrypto {
    envelope: Envelope,
}

impl ChatCrypto {
    /// Create new chat crypto with random key
    pub fn new() -> Self {
        let key = ChaCha20Poly1305::generate_key(&mut OsRng);
        let envelope = Envelope::new(&key).expect("Failed to create envelope");
        
        Self { envelope }
    }

    /// Encrypt plaintext message
    pub fn encrypt(&self, plaintext: &[u8]) -> Vec<u8> {
        self.envelope.encrypt(plaintext).unwrap_or_else(|_| plaintext.to_vec())
    }

    /// Decrypt ciphertext message
    pub fn decrypt(&self, ciphertext: &[u8]) -> Result<Vec<u8>> {
        self.envelope.decrypt(ciphertext).map(|z| z.to_vec())
    }
}

impl Default for ChatCrypto {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encrypt_decrypt() {
        let crypto = ChatCrypto::new();
        let message = b"Hello, UMBRA!";
        
        let encrypted = crypto.encrypt(message);
        let decrypted = crypto.decrypt(&encrypted).unwrap();
        
        assert_eq!(decrypted, message);
    }
}
