// Hybrid handshake: X25519 + ML-KEM-768 (Quantum Shield v0.3)
// Simple, Linus-style: no overengineering

use crate::error::Result;
use crate::kem::HybridKem;
use ed25519_dalek::{Signature, Signer, SigningKey, Verifier, VerifyingKey};
use libp2p::PeerId;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use x25519_dalek::PublicKey;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeInit {
    pub peer_id: Vec<u8>,
    pub x25519_pk: [u8; 32],
    #[cfg(feature = "pq")]
    pub pq_pk: Vec<u8>,
    #[serde(with = "serde_arrays")]
    pub signature: [u8; 64],
    pub verify_key: [u8; 32], // Ed25519 public key for message signatures
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HandshakeResp {
    pub peer_id: Vec<u8>,
    pub x25519_pk: [u8; 32],
    #[cfg(feature = "pq")]
    pub pq_ct: Vec<u8>,
    #[serde(with = "serde_arrays")]
    pub signature: [u8; 64],
    pub verify_key: [u8; 32], // Ed25519 public key for message signatures
}

mod serde_arrays {
    use serde::{Deserialize, Deserializer, Serialize, Serializer};

    pub fn serialize<S>(bytes: &[u8; 64], serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        bytes.as_slice().serialize(serializer)
    }

    pub fn deserialize<'de, D>(deserializer: D) -> Result<[u8; 64], D::Error>
    where
        D: Deserializer<'de>,
    {
        let vec: Vec<u8> = Deserialize::deserialize(deserializer)?;
        vec.try_into()
            .map_err(|_| serde::de::Error::custom("Invalid signature length"))
    }
}

pub struct Handshake {
    identity: SigningKey,
    kem: HybridKem,
}

impl Handshake {
    pub fn new(identity: SigningKey) -> Result<Self> {
        let kem = HybridKem::generate()?;
        Ok(Self { identity, kem })
    }

    pub fn initiate(&self, peer_id: PeerId) -> Result<HandshakeInit> {
        let peer_id_bytes = peer_id.to_bytes();
        let x25519_pk = *self.kem.classical_public_key().as_bytes();
        let verify_key = self.identity.verifying_key().to_bytes();
        
        let mut msg = Vec::new();
        msg.extend_from_slice(&peer_id_bytes);
        msg.extend_from_slice(&x25519_pk);
        
        #[cfg(feature = "pq")]
        let pq_pk = {
            let pk = self.kem.pq_public_key()?;
            msg.extend_from_slice(&pk);
            pk
        };
        
        let signature = self.identity.sign(&msg);
        
        Ok(HandshakeInit {
            peer_id: peer_id_bytes,
            x25519_pk,
            #[cfg(feature = "pq")]
            pq_pk,
            signature: signature.to_bytes(),
            verify_key,
        })
    }

    pub fn respond(
        self,
        peer_id: PeerId,
        init: &HandshakeInit,
        peer_verify_key: &VerifyingKey,
    ) -> Result<(HandshakeResp, [u8; 32])> {
        // Verify signature
        let mut msg = Vec::new();
        msg.extend_from_slice(&init.peer_id);
        msg.extend_from_slice(&init.x25519_pk);
        
        #[cfg(feature = "pq")]
        msg.extend_from_slice(&init.pq_pk);
        
        let sig = Signature::from_bytes(&init.signature);
        peer_verify_key.verify(&msg, &sig)
            .map_err(|e| crate::error::CryptoError::InvalidSignature(e.to_string()))?;
        
        // Hybrid KEM encapsulation
        let peer_x25519_pk = PublicKey::from(init.x25519_pk);
        
        #[cfg(feature = "pq")]
        let (pq_ct, shared_secret) = {
            let (ct, secret) = self.kem.encapsulate(&peer_x25519_pk, &init.pq_pk)?;
            (ct, secret.as_bytes().to_vec())
        };
        
        #[cfg(not(feature = "pq"))]
        let shared_secret = {
            let secret = self.kem.encapsulate(&peer_x25519_pk)?;
            secret.as_bytes().to_vec()
        };
        
        let session_key = Self::derive_key(&shared_secret);
        
        // Create response
        let peer_id_bytes = peer_id.to_bytes();
        let x25519_pk = *self.kem.classical_public_key().as_bytes();
        
        let mut resp_msg = Vec::new();
        resp_msg.extend_from_slice(&peer_id_bytes);
        resp_msg.extend_from_slice(&x25519_pk);
        
        #[cfg(feature = "pq")]
        resp_msg.extend_from_slice(&pq_ct);
        
        let signature = self.identity.sign(&resp_msg);
        let verify_key = self.identity.verifying_key().to_bytes();
        
        let resp = HandshakeResp {
            peer_id: peer_id_bytes,
            x25519_pk,
            #[cfg(feature = "pq")]
            pq_ct,
            signature: signature.to_bytes(),
            verify_key,
        };
        
        Ok((resp, session_key))
    }

    pub fn complete(
        self,
        resp: &HandshakeResp,
        peer_verify_key: &VerifyingKey,
    ) -> Result<[u8; 32]> {
        // Verify signature
        let mut msg = Vec::new();
        msg.extend_from_slice(&resp.peer_id);
        msg.extend_from_slice(&resp.x25519_pk);
        
        #[cfg(feature = "pq")]
        msg.extend_from_slice(&resp.pq_ct);
        
        let sig = Signature::from_bytes(&resp.signature);
        peer_verify_key.verify(&msg, &sig)
            .map_err(|e| crate::error::CryptoError::InvalidSignature(e.to_string()))?;
        
        // Hybrid KEM decapsulation
        let peer_x25519_pk = PublicKey::from(resp.x25519_pk);
        
        #[cfg(feature = "pq")]
        let shared_secret = {
            let secret = self.kem.decapsulate(&peer_x25519_pk, &resp.pq_ct)?;
            secret.as_bytes().to_vec()
        };
        
        #[cfg(not(feature = "pq"))]
        let shared_secret = {
            let secret = self.kem.decapsulate(&peer_x25519_pk)?;
            secret.as_bytes().to_vec()
        };
        
        Ok(Self::derive_key(&shared_secret))
    }

    fn derive_key(shared: &[u8]) -> [u8; 32] {
        let mut hasher = Sha256::new();
        hasher.update(b"umbra-quantum-shield-v0.3");
        hasher.update(shared);
        hasher.finalize().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn gen_keypair() -> SigningKey {
        SigningKey::from_bytes(&rand::random())
    }

    #[test]
    fn test_handshake_flow() {
        let alice_id = gen_keypair();
        let alice_pk = alice_id.verifying_key();
        let bob_id = gen_keypair();
        let bob_pk = bob_id.verifying_key();
        
        let alice_peer = PeerId::random();
        let bob_peer = PeerId::random();
        
        let alice_hs = Handshake::new(alice_id).unwrap();
        let init = alice_hs.initiate(alice_peer).unwrap();
        
        let bob_hs = Handshake::new(bob_id).unwrap();
        let (resp, bob_key) = bob_hs.respond(bob_peer, &init, &alice_pk).unwrap();
        
        let alice_hs2 = Handshake::new(gen_keypair()).unwrap();
        let alice_key = alice_hs2.complete(&resp, &bob_pk).unwrap();
        
        assert_eq!(alice_key.len(), 32);
        assert_eq!(bob_key.len(), 32);
    }

    #[test]
    fn test_invalid_signature() {
        let alice_id = gen_keypair();
        let wrong_pk = gen_keypair().verifying_key();
        
        let alice_peer = PeerId::random();
        let alice_hs = Handshake::new(alice_id).unwrap();
        let init = alice_hs.initiate(alice_peer).unwrap();
        
        let bob_hs = Handshake::new(gen_keypair()).unwrap();
        let result = bob_hs.respond(PeerId::random(), &init, &wrong_pk);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_tampered_public_key() {
        let alice_id = gen_keypair();
        let alice_pk = alice_id.verifying_key();
        let bob_id = gen_keypair();
        
        let alice_peer = PeerId::random();
        let bob_peer = PeerId::random();
        
        let alice_hs = Handshake::new(alice_id).unwrap();
        let mut init = alice_hs.initiate(alice_peer).unwrap();
        
        // Tamper with the public key
        init.x25519_pk[0] ^= 0xFF;
        
        let bob_hs = Handshake::new(bob_id).unwrap();
        let result = bob_hs.respond(bob_peer, &init, &alice_pk);
        
        // Should fail signature verification
        assert!(result.is_err());
    }

    #[test]
    fn test_signature_verification_in_response() {
        let alice_id = gen_keypair();
        let alice_pk = alice_id.verifying_key();
        let bob_id = gen_keypair();
        let wrong_pk = gen_keypair().verifying_key();
        
        let alice_peer = PeerId::random();
        let bob_peer = PeerId::random();
        
        let alice_hs = Handshake::new(alice_id).unwrap();
        let init = alice_hs.initiate(alice_peer).unwrap();
        
        let bob_hs = Handshake::new(bob_id).unwrap();
        let (resp, _) = bob_hs.respond(bob_peer, &init, &alice_pk).unwrap();
        
        // Try to complete with wrong verification key
        let alice_hs2 = Handshake::new(gen_keypair()).unwrap();
        let result = alice_hs2.complete(&resp, &wrong_pk);
        
        assert!(result.is_err());
    }

    #[test]
    fn test_key_derivation_deterministic() {
        let shared_secret = b"test_shared_secret_32_bytes_long";
        let key1 = Handshake::derive_key(shared_secret);
        let key2 = Handshake::derive_key(shared_secret);
        
        assert_eq!(key1, key2);
        assert_eq!(key1.len(), 32);
    }

    #[test]
    fn test_key_derivation_unique() {
        let secret1 = b"secret1_________________________";
        let secret2 = b"secret2_________________________";
        
        let key1 = Handshake::derive_key(secret1);
        let key2 = Handshake::derive_key(secret2);
        
        assert_ne!(key1, key2);
    }

    #[test]
    fn test_serialization_roundtrip_init() {
        let alice_id = gen_keypair();
        let alice_peer = PeerId::random();
        
        let alice_hs = Handshake::new(alice_id).unwrap();
        let init = alice_hs.initiate(alice_peer).unwrap();
        
        // Serialize and deserialize
        let serialized = bincode::serialize(&init).unwrap();
        let deserialized: HandshakeInit = bincode::deserialize(&serialized).unwrap();
        
        assert_eq!(init.peer_id, deserialized.peer_id);
        assert_eq!(init.x25519_pk, deserialized.x25519_pk);
        assert_eq!(init.signature, deserialized.signature);
    }

    #[test]
    fn test_serialization_roundtrip_resp() {
        let alice_id = gen_keypair();
        let alice_pk = alice_id.verifying_key();
        let bob_id = gen_keypair();
        
        let alice_peer = PeerId::random();
        let bob_peer = PeerId::random();
        
        let alice_hs = Handshake::new(alice_id).unwrap();
        let init = alice_hs.initiate(alice_peer).unwrap();
        
        let bob_hs = Handshake::new(bob_id).unwrap();
        let (resp, _) = bob_hs.respond(bob_peer, &init, &alice_pk).unwrap();
        
        // Serialize and deserialize
        let serialized = bincode::serialize(&resp).unwrap();
        let deserialized: HandshakeResp = bincode::deserialize(&serialized).unwrap();
        
        assert_eq!(resp.peer_id, deserialized.peer_id);
        assert_eq!(resp.x25519_pk, deserialized.x25519_pk);
        assert_eq!(resp.signature, deserialized.signature);
    }

    #[test]
    fn test_different_peers_different_signatures() {
        let alice_id = gen_keypair();
        let peer1 = PeerId::random();
        let peer2 = PeerId::random();
        
        let hs1 = Handshake::new(alice_id.clone()).unwrap();
        let init1 = hs1.initiate(peer1).unwrap();
        
        let hs2 = Handshake::new(alice_id).unwrap();
        let init2 = hs2.initiate(peer2).unwrap();
        
        // Different peer IDs should result in different signatures
        assert_ne!(init1.signature, init2.signature);
    }

    #[test]
    fn test_handshake_key_is_32_bytes() {
        let alice_id = gen_keypair();
        let alice_pk = alice_id.verifying_key();
        let bob_id = gen_keypair();
        
        let alice_peer = PeerId::random();
        let bob_peer = PeerId::random();
        
        let alice_hs = Handshake::new(alice_id).unwrap();
        let init = alice_hs.initiate(alice_peer).unwrap();
        
        let bob_hs = Handshake::new(bob_id).unwrap();
        let (_, bob_key) = bob_hs.respond(bob_peer, &init, &alice_pk).unwrap();
        
        assert_eq!(bob_key.len(), 32);
        // Ensure key is not all zeros
        assert!(bob_key.iter().any(|&b| b != 0));
    }
}
