use crate::error::IdentityError;
use crate::field_utils::compute_identity_id;
use serde::{Serialize, Deserialize};
use zeroize::{Zeroize, ZeroizeOnDrop};

#[derive(Clone, Serialize, Deserialize, Zeroize, ZeroizeOnDrop)]
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

        let secret: [u8; 32] = *blake3::hash(password.as_bytes()).as_bytes();
        let id = compute_identity_id(&secret)?;
        
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

    #[test]
    fn empty_password_fails() {
        assert!(Identity::create("").is_err());
    }

    #[test]
    fn secret_not_leaked_in_serialization() {
        let identity = Identity::create("password123").unwrap();
        let json = serde_json::to_string(&identity).unwrap();
        
        // Secret field should be skipped in serialization
        assert!(!json.contains("secret"));
    }
}
