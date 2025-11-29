use crate::error::{CryptoError, Result};
use ed25519_dalek::{Signer, Verifier, Signature, SigningKey, VerifyingKey};

#[cfg(feature = "pq")]
use oqs::sig::{Sig, Algorithm};

/// Identity keypair with hybrid signatures
pub struct IdentityKey {
    classical_signing: SigningKey,
    classical_verifying: VerifyingKey,
    #[cfg(feature = "pq")]
    pq_sig: Sig,
}

impl IdentityKey {
    pub fn generate() -> Result<Self> {
        let classical_signing = SigningKey::from_bytes(&rand::random());
        let classical_verifying = classical_signing.verifying_key();
        
        #[cfg(feature = "pq")]
        let pq_sig = Sig::new(Algorithm::Dilithium3)
            .map_err(|e| CryptoError::PostQuantum(format!("Dilithium init failed: {}", e)))?;
        
        Ok(Self {
            classical_signing,
            classical_verifying,
            #[cfg(feature = "pq")]
            pq_sig,
        })
    }
    
    pub fn verifying_key(&self) -> &VerifyingKey {
        &self.classical_verifying
    }
    
    #[cfg(feature = "pq")]
    pub fn pq_verifying_key(&self) -> Result<Vec<u8>> {
        let (pk, _sk) = self.pq_sig.keypair()
            .map_err(|e| CryptoError::PostQuantum(format!("Keypair gen failed: {}", e)))?;
        Ok(pk.into_vec())
    }
    
    /// Sign a message with hybrid signature
    pub fn sign(&self, message: &[u8]) -> Result<HybridSignature> {
        let classical_sig = self.classical_signing.sign(message);
        
        #[cfg(feature = "pq")]
        {
            let (_pk, sk) = self.pq_sig.keypair()
                .map_err(|e| CryptoError::PostQuantum(format!("Keypair gen failed: {}", e)))?;
            let pq_sig = self.pq_sig.sign(message, &sk)
                .map_err(|e| CryptoError::PostQuantum(format!("Signing failed: {}", e)))?;
            
            Ok(HybridSignature {
                classical: classical_sig.to_bytes().to_vec(),
                pq: Some(pq_sig.into_vec()),
            })
        }
        
        #[cfg(not(feature = "pq"))]
        Ok(HybridSignature {
            classical: classical_sig.to_bytes().to_vec(),
            pq: None,
        })
    }
    
    /// Verify a hybrid signature
    pub fn verify(&self, message: &[u8], signature: &HybridSignature) -> Result<()> {
        // Verify classical signature
        let sig = Signature::from_slice(&signature.classical)
            .map_err(|_| CryptoError::SignatureVerification)?;
        self.classical_verifying.verify(message, &sig)
            .map_err(|_| CryptoError::SignatureVerification)?;
        
        #[cfg(feature = "pq")]
        if let Some(pq_sig) = &signature.pq {
            let (pk, _sk) = self.pq_sig.keypair()
                .map_err(|e| CryptoError::PostQuantum(format!("Keypair gen failed: {}", e)))?;
            self.pq_sig.verify(message, pq_sig, &pk)
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
