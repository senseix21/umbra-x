use crate::error::{CryptoError, Result};
use x25519_dalek::{PublicKey, StaticSecret};
use sha2::{Sha256, Digest};

#[cfg(feature = "pq")]
use oqs::kem::{Kem, Algorithm, Ciphertext, SecretKey as PqSecretKey};

/// Hybrid shared secret combining X25519 and ML-KEM outputs
pub struct HybridSharedSecret {
    data: Vec<u8>,
}

impl HybridSharedSecret {
    pub fn as_bytes(&self) -> &[u8] {
        &self.data
    }
}

impl Drop for HybridSharedSecret {
    fn drop(&mut self) {
        use zeroize::Zeroize;
        self.data.zeroize();
    }
}

/// Hybrid KEM combining X25519 and ML-KEM (Kyber)
pub struct HybridKem {
    classical_secret: StaticSecret,
    classical_public: PublicKey,
    #[cfg(feature = "pq")]
    pq_kem: Kem,
    #[cfg(feature = "pq")]
    pq_secret: Option<PqSecretKey>,
    #[cfg(feature = "pq")]
    pq_public: Option<Vec<u8>>,
}

impl HybridKem {
    pub fn generate() -> Result<Self> {
        let classical_secret = StaticSecret::random_from_rng(rand::thread_rng());
        let classical_public = PublicKey::from(&classical_secret);
        
        #[cfg(feature = "pq")]
        {
            let pq_kem = Kem::new(Algorithm::Kyber768)
                .map_err(|e| CryptoError::PostQuantum(format!("Failed to init Kyber: {}", e)))?;
            
            let (pq_public, pq_secret) = pq_kem.keypair()
                .map_err(|e| CryptoError::PostQuantum(format!("Keypair gen failed: {}", e)))?;
            
            Ok(Self {
                classical_secret,
                classical_public,
                pq_kem,
                pq_secret: Some(pq_secret),
                pq_public: Some(pq_public.into_vec()),
            })
        }
        
        #[cfg(not(feature = "pq"))]
        Ok(Self {
            classical_secret,
            classical_public,
        })
    }
    
    pub fn classical_public_key(&self) -> &PublicKey {
        &self.classical_public
    }
    
    #[cfg(feature = "pq")]
    pub fn pq_public_key(&self) -> Result<Vec<u8>> {
        self.pq_public.clone().ok_or_else(|| 
            CryptoError::PostQuantum("PQ public key not initialized".to_string())
        )
    }
    
    #[cfg(feature = "pq")]
    pub fn encapsulate(&self, peer_classical_pk: &PublicKey, peer_pq_pk: &[u8]) -> Result<(Vec<u8>, HybridSharedSecret)> {
        let classical_shared = self.classical_secret.diffie_hellman(peer_classical_pk);
        
        let pq_pk = oqs::kem::PublicKey::from_vec(peer_pq_pk.to_vec())
            .map_err(|e| CryptoError::PostQuantum(format!("Invalid PQ public key: {}", e)))?;
        
        let (ciphertext, pq_shared) = self.pq_kem.encapsulate(&pq_pk)
            .map_err(|e| CryptoError::PostQuantum(format!("Encapsulation failed: {}", e)))?;
        
        let mut hasher = Sha256::new();
        hasher.update(b"UMBRA-HYBRID-KEM");
        hasher.update(classical_shared.as_bytes());
        hasher.update(pq_shared.as_ref());
        let combined = hasher.finalize().to_vec();
        
        Ok((ciphertext.into_vec(), HybridSharedSecret { data: combined }))
    }
    
    #[cfg(not(feature = "pq"))]
    pub fn encapsulate(&self, peer_classical_pk: &PublicKey) -> Result<HybridSharedSecret> {
        let classical_shared = self.classical_secret.diffie_hellman(peer_classical_pk);
        
        let mut hasher = Sha256::new();
        hasher.update(b"UMBRA-CLASSICAL-ONLY");
        hasher.update(classical_shared.as_bytes());
        let combined = hasher.finalize().to_vec();
        
        Ok(HybridSharedSecret { data: combined })
    }
    
    #[cfg(feature = "pq")]
    pub fn decapsulate(&self, peer_classical_pk: &PublicKey, pq_ciphertext: &[u8]) -> Result<HybridSharedSecret> {
        let classical_shared = self.classical_secret.diffie_hellman(peer_classical_pk);
        
        let ct = Ciphertext::from_vec(pq_ciphertext.to_vec())
            .map_err(|e| CryptoError::PostQuantum(format!("Invalid ciphertext: {}", e)))?;
        
        let pq_secret = self.pq_secret.as_ref()
            .ok_or_else(|| CryptoError::PostQuantum("PQ secret key not initialized".to_string()))?;
        
        let pq_shared = self.pq_kem.decapsulate(pq_secret, &ct)
            .map_err(|e| CryptoError::PostQuantum(format!("Decapsulation failed: {}", e)))?;
        
        let mut hasher = Sha256::new();
        hasher.update(b"UMBRA-HYBRID-KEM");
        hasher.update(classical_shared.as_bytes());
        hasher.update(pq_shared.as_ref());
        let combined = hasher.finalize().to_vec();
        
        Ok(HybridSharedSecret { data: combined })
    }
    
    #[cfg(not(feature = "pq"))]
    pub fn decapsulate(&self, peer_classical_pk: &PublicKey) -> Result<HybridSharedSecret> {
        self.encapsulate(peer_classical_pk)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_hybrid_kem_generation() {
        let kem = HybridKem::generate().unwrap();
        assert!(kem.classical_public_key().as_bytes().len() == 32);
    }
    
    #[test]
    #[cfg(feature = "pq")]
    fn test_pq_public_key() {
        let kem = HybridKem::generate().unwrap();
        let pq_pk = kem.pq_public_key().unwrap();
        assert!(pq_pk.len() > 0);
    }
    
    #[test]
    fn test_classical_only_encapsulation() {
        let alice = HybridKem::generate().unwrap();
        let bob = HybridKem::generate().unwrap();
        
        #[cfg(not(feature = "pq"))]
        {
            let alice_shared = alice.encapsulate(bob.classical_public_key()).unwrap();
            let bob_shared = bob.decapsulate(alice.classical_public_key()).unwrap();
            
            assert_eq!(alice_shared.as_bytes(), bob_shared.as_bytes());
        }
    }
    
    #[test]
    #[cfg(feature = "pq")]
    fn test_hybrid_kem_encap_decap() {
        let alice = HybridKem::generate().unwrap();
        let bob = HybridKem::generate().unwrap();
        
        let bob_pq_pk = bob.pq_public_key().unwrap();
        
        let (ciphertext, alice_shared) = alice.encapsulate(
            bob.classical_public_key(),
            &bob_pq_pk
        ).unwrap();
        
        let bob_shared = bob.decapsulate(
            alice.classical_public_key(),
            &ciphertext
        ).unwrap();
        
        assert_eq!(alice_shared.as_bytes(), bob_shared.as_bytes());
        assert_eq!(alice_shared.as_bytes().len(), 32);
    }
}
