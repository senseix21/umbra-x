pub mod error;
pub mod kem;
pub mod identity;
pub mod aead;
pub mod chat_crypto;
pub mod session;

pub use error::{CryptoError, Result};
pub use kem::{HybridKem, HybridSharedSecret};
pub use identity::{IdentityKey, HybridSignature};
pub use aead::Envelope;
pub use chat_crypto::ChatCrypto;
pub use session::{SessionManager, SessionKey};

/// Re-export commonly used types
pub mod prelude {
    pub use crate::error::{CryptoError, Result};
    pub use crate::kem::{HybridKem, HybridSharedSecret};
    pub use crate::identity::{IdentityKey, HybridSignature};
    pub use crate::aead::Envelope;
    pub use crate::chat_crypto::ChatCrypto;
    pub use crate::session::{SessionManager, SessionKey};
}
