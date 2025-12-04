use crate::error::IdentityError;
use ark_ff::{Field, PrimeField, BigInteger};
use ark_bn254::Fr;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Identity {
    pub id: [u8; 32],
    #[serde(skip)]
    secret: [u8; 32],
}

impl Identity {
    pub fn create(password: &str) -> Result<Self, IdentityError> {
        if password.is_empty() {
            return Err(IdentityError::InvalidPassword);
        }

        // password → secret
        let secret: [u8; 32] = *blake3::hash(password.as_bytes()).as_bytes();
        
        // secret → identity_id via x^5 in field
        let secret_u64 = u64::from_le_bytes(secret[..8].try_into().unwrap());
        let secret_fr = Fr::from(secret_u64);
        let id_fr = secret_fr.pow([5u64]);
        
        // Convert field element back to bytes
        let mut id = [0u8; 32];
        let id_bytes = id_fr.into_bigint().to_bytes_le();
        id[..id_bytes.len().min(32)].copy_from_slice(&id_bytes[..id_bytes.len().min(32)]);
        
        Ok(Self { id, secret })
    }

    pub fn secret(&self) -> &[u8; 32] {
        &self.secret
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn same_password_same_identity() {
        let id1 = Identity::create("password123").unwrap();
        let id2 = Identity::create("password123").unwrap();
        assert_eq!(id1.id, id2.id);
    }

    #[test]
    fn different_password_different_identity() {
        let id1 = Identity::create("password123").unwrap();
        let id2 = Identity::create("password456").unwrap();
        assert_ne!(id1.id, id2.id);
    }
}
