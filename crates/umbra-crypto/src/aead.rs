use crate::error::{CryptoError, Result};
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng},
    ChaCha20Poly1305, Nonce,
};
use zeroize::Zeroizing;

const NONCE_SIZE: usize = 12;

/// AEAD envelope for encrypting message payloads
pub struct Envelope {
    cipher: ChaCha20Poly1305,
}

impl Envelope {
    pub fn new(key: &[u8]) -> Result<Self> {
        if key.len() != 32 {
            return Err(CryptoError::InvalidKeyLength {
                expected: 32,
                got: key.len(),
            });
        }
        
        let cipher = ChaCha20Poly1305::new_from_slice(key)
            .map_err(|e| CryptoError::Encryption(format!("Key init failed: {}", e)))?;
        
        Ok(Self { cipher })
    }
    
    /// Encrypt plaintext and return (nonce || ciphertext)
    pub fn encrypt(&self, plaintext: &[u8]) -> Result<Vec<u8>> {
        let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
        
        let ciphertext = self.cipher
            .encrypt(&nonce, plaintext)
            .map_err(|e| CryptoError::Encryption(format!("AEAD encrypt failed: {}", e)))?;
        
        let mut result = Vec::with_capacity(NONCE_SIZE + ciphertext.len());
        result.extend_from_slice(&nonce);
        result.extend_from_slice(&ciphertext);
        
        Ok(result)
    }
    
    /// Decrypt (nonce || ciphertext) and return plaintext
    pub fn decrypt(&self, data: &[u8]) -> Result<Zeroizing<Vec<u8>>> {
        if data.len() < NONCE_SIZE {
            return Err(CryptoError::Decryption("Data too short".to_string()));
        }
        
        let (nonce_bytes, ciphertext) = data.split_at(NONCE_SIZE);
        let nonce = Nonce::from_slice(nonce_bytes);
        
        let plaintext = self.cipher
            .decrypt(nonce, ciphertext)
            .map_err(|e| CryptoError::Decryption(format!("AEAD decrypt failed: {}", e)))?;
        
        Ok(Zeroizing::new(plaintext))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_envelope_encrypt_decrypt() {
        let key = ChaCha20Poly1305::generate_key(&mut OsRng);
        let envelope = Envelope::new(&key).unwrap();
        
        let plaintext = b"secret message";
        let encrypted = envelope.encrypt(plaintext).unwrap();
        let decrypted = envelope.decrypt(&encrypted).unwrap();
        
        assert_eq!(&**decrypted, plaintext);
    }
    
    #[test]
    fn test_envelope_wrong_key() {
        let key1 = ChaCha20Poly1305::generate_key(&mut OsRng);
        let key2 = ChaCha20Poly1305::generate_key(&mut OsRng);
        
        let envelope1 = Envelope::new(&key1).unwrap();
        let envelope2 = Envelope::new(&key2).unwrap();
        
        let plaintext = b"secret message";
        let encrypted = envelope1.encrypt(plaintext).unwrap();
        
        assert!(envelope2.decrypt(&encrypted).is_err());
    }
}
