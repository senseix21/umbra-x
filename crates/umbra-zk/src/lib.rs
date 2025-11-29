pub mod error;
pub mod rln;
pub mod credential;
pub mod policy;
pub mod merkle;

#[cfg(feature = "arkworks")]
pub mod circuit;

#[cfg(feature = "arkworks")]
pub mod groth16;

pub use error::{ZkError, Result};
pub use rln::{RlnProof, RlnProver, RlnVerifier, RlnConfig};
pub use credential::{HumanCredential, MintRequest, CredentialMint, CommitteeMember};
pub use policy::{RoomPolicy, PolicyEngine};
pub use merkle::MembershipTree;

#[cfg(feature = "arkworks")]
pub use circuit::{RlnCircuit, RlnPublicInputs, RlnWitness};

#[cfg(feature = "arkworks")]
pub use groth16::{RlnSetup, RlnGroth16Prover, RlnGroth16Verifier};
