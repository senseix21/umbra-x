use thiserror::Error;

#[derive(Debug, Error)]
pub enum IdentityError {
    #[error("Invalid password")]
    InvalidPassword,
    
    #[error("Invalid secret length")]
    InvalidSecretLength,
    
    #[error("Circuit error: {0}")]
    Circuit(String),
    
    #[error("Proof generation failed: {0}")]
    ProofGeneration(String),
    
    #[error("Proof verification failed")]
    ProofVerification,
    
    #[error("Serialization error: {0}")]
    Serialization(String),
    
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
}
