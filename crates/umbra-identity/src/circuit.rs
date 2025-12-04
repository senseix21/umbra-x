use ark_ff::PrimeField;
use ark_r1cs_std::fields::fp::FpVar;
use ark_r1cs_std::prelude::*;
use ark_relations::r1cs::{ConstraintSynthesizer, ConstraintSystemRef, SynthesisError};

pub struct IdentityCircuit<F: PrimeField> {
    pub secret: Option<F>,
    pub identity_id: Option<F>,
}

impl<F: PrimeField> ConstraintSynthesizer<F> for IdentityCircuit<F> {
    fn generate_constraints(self, cs: ConstraintSystemRef<F>) -> Result<(), SynthesisError> {
        let secret_var = FpVar::new_witness(cs.clone(), || {
            self.secret.ok_or(SynthesisError::AssignmentMissing)
        })?;

        let id_var = FpVar::new_input(cs.clone(), || {
            self.identity_id.ok_or(SynthesisError::AssignmentMissing)
        })?;

        // Simple hash: H(x) = x^5 (placeholder for Poseidon)
        // Poseidon uses x^5 as S-box, this is a simplified version
        let hashed = &secret_var * &secret_var; // x^2
        let hashed = &hashed * &hashed;         // x^4  
        let hashed = &hashed * &secret_var;     // x^5

        hashed.enforce_equal(&id_var)?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ark_bn254::Fr;
    use ark_relations::r1cs::ConstraintSystem;
    use ark_ff::Field;

    #[test]
    fn test_circuit_hash() {
        let cs = ConstraintSystem::<Fr>::new_ref();
        let secret = Fr::from(3u64);
        let expected = secret.pow([5u64]); // 3^5 = 243
        
        let circuit = IdentityCircuit {
            secret: Some(secret),
            identity_id: Some(expected),
        };

        circuit.generate_constraints(cs.clone()).unwrap();
        assert!(cs.is_satisfied().unwrap());
    }

    #[test]
    fn test_circuit_fails_wrong_hash() {
        let cs = ConstraintSystem::<Fr>::new_ref();
        let secret = Fr::from(3u64);
        let wrong = Fr::from(100u64);
        
        let circuit = IdentityCircuit {
            secret: Some(secret),
            identity_id: Some(wrong),
        };

        circuit.generate_constraints(cs.clone()).unwrap();
        assert!(!cs.is_satisfied().unwrap());
    }
}
