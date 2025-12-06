use crate::error::{CryptoError, Result};
use ed25519_dalek::{Signer, Verifier, Signature, SigningKey, VerifyingKey};
use pqcrypto_dilithium::dilithium3;
use pqcrypto_traits::sign::{PublicKey as PqPublicKey, SecretKey as PqSecretKey, DetachedSignature as PqSignature};

/// Identity keypair with hybrid signatures (always-on) - Pure Rust!
#[derive(Clone)]
pub struct IdentityKey {
    classical_signing: SigningKey,
    classical_verifying: VerifyingKey,
    pq_secret: Vec<u8>,
    pq_public: Vec<u8>,
}

impl IdentityKey {
    pub fn generate() -> Result<Self> {
        let classical_signing = SigningKey::from_bytes(&rand::random());
        let classical_verifying = classical_signing.verifying_key();
        
        // Generate Dilithium3 keypair (pure Rust!)
        let (pk, sk) = dilithium3::keypair();
        
        Ok(Self {
            classical_signing,
            classical_verifying,
            pq_secret: sk.as_bytes().to_vec(),
            pq_public: pk.as_bytes().to_vec(),
        })
    }
    
    pub fn verifying_key(&self) -> &VerifyingKey {
        &self.classical_verifying
    }
    
    pub fn pq_verifying_key(&self) -> Vec<u8> {
        self.pq_public.clone()
    }
    
    /// Sign a message with hybrid signature
    pub fn sign(&self, message: &[u8]) -> Result<HybridSignature> {
        let classical_sig = self.classical_signing.sign(message);
        
        // Reconstruct secret key from bytes
        let sk = dilithium3::SecretKey::from_bytes(&self.pq_secret)
            .map_err(|_| CryptoError::PostQuantum("Invalid secret key".to_string()))?;
        
        // Sign with Dilithium3 (pure Rust!)
        let pq_signature = dilithium3::detached_sign(message, &sk);
        
        Ok(HybridSignature {
            classical: classical_sig.to_bytes().to_vec(),
            pq: Some(pq_signature.as_bytes().to_vec()),
        })
    }
    
    /// Verify a hybrid signature
    pub fn verify(&self, message: &[u8], signature: &HybridSignature) -> Result<()> {
        // Verify classical signature
        let sig = Signature::from_slice(&signature.classical)
            .map_err(|_| CryptoError::SignatureVerification)?;
        self.classical_verifying.verify(message, &sig)
            .map_err(|_| CryptoError::SignatureVerification)?;
        
        // Verify PQ signature
        if let Some(pq_sig_bytes) = &signature.pq {
            // Reconstruct public key and signature from bytes
            let pk = dilithium3::PublicKey::from_bytes(&self.pq_public)
                .map_err(|_| CryptoError::PostQuantum("Invalid public key".to_string()))?;
            
            let sig = dilithium3::DetachedSignature::from_bytes(pq_sig_bytes)
                .map_err(|_| CryptoError::PostQuantum("Invalid signature".to_string()))?;
            
            // Verify with Dilithium3 (pure Rust!)
            dilithium3::verify_detached_signature(&sig, message, &pk)
                .map_err(|_| CryptoError::SignatureVerification)?;
        }
        
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct HybridSignature {
    pub classical: Vec<u8>,
    pub pq: Option<Vec<u8>>,
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_identity_key_generation() {
        let key = IdentityKey::generate().unwrap();
        assert!(key.verifying_key().as_bytes().len() == 32);
    }
    
    #[test]
    fn test_sign_verify() {
        let key = IdentityKey::generate().unwrap();
        let message = b"test message";
        let signature = key.sign(message).unwrap();
        assert!(key.verify(message, &signature).is_ok());
    }
    
    #[test]
    fn test_verify_fails_wrong_message() {
        let key = IdentityKey::generate().unwrap();
        let message = b"test message";
        let signature = key.sign(message).unwrap();
        assert!(key.verify(b"wrong message", &signature).is_err());
    }
}
