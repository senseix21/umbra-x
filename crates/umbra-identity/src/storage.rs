use std::path::Path;
use std::fs;
use ark_serialize::CanonicalSerialize;
use crate::{Prover, Identity, IdentityError};

const KEYS_FILE: &str = "umbra_keys.bin";
const IDENTITY_FILE: &str = "umbra_identity.bin";

pub struct Storage {
    data_dir: std::path::PathBuf,
}

impl Storage {
    pub fn new(data_dir: impl AsRef<Path>) -> Result<Self, IdentityError> {
        let data_dir = data_dir.as_ref().to_path_buf();
        fs::create_dir_all(&data_dir)?;
        Ok(Self { data_dir })
    }

    pub fn save_keys(&self, prover: &Prover) -> Result<(), IdentityError> {
        let path = self.data_dir.join(KEYS_FILE);
        let mut bytes = Vec::new();
        
        prover.pk().serialize_compressed(&mut bytes)
            .map_err(|e| IdentityError::Serialization(e.to_string()))?;
        prover.vk().serialize_compressed(&mut bytes)
            .map_err(|e| IdentityError::Serialization(e.to_string()))?;
        
        fs::write(path, bytes)?;
        Ok(())
    }

    pub fn load_keys(&self) -> Result<Prover, IdentityError> {
        let path = self.data_dir.join(KEYS_FILE);
        let bytes = fs::read(path)?;
        
        Prover::from_bytes(&bytes)
    }

    pub fn save_identity(&self, identity: &Identity) -> Result<(), IdentityError> {
        let path = self.data_dir.join(IDENTITY_FILE);
        let json = serde_json::to_string(identity)
            .map_err(|e| IdentityError::Serialization(e.to_string()))?;
        fs::write(path, json)?;
        Ok(())
    }

    pub fn load_identity(&self) -> Result<Identity, IdentityError> {
        let path = self.data_dir.join(IDENTITY_FILE);
        let json = fs::read_to_string(path)?;
        serde_json::from_str(&json)
            .map_err(|e| IdentityError::Serialization(e.to_string()))
    }

    pub fn has_identity(&self) -> bool {
        self.data_dir.join(IDENTITY_FILE).exists()
    }

    pub fn has_keys(&self) -> bool {
        self.data_dir.join(KEYS_FILE).exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::tempdir;

    #[test]
    fn test_save_load_keys() {
        let dir = tempdir().unwrap();
        let storage = Storage::new(dir.path()).unwrap();
        
        let prover = Prover::setup().unwrap();
        storage.save_keys(&prover).unwrap();
        
        let _loaded = storage.load_keys().unwrap();
        assert!(storage.has_keys());
    }

    #[test]
    fn test_save_load_identity() {
        let dir = tempdir().unwrap();
        let storage = Storage::new(dir.path()).unwrap();
        
        let identity = Identity::create("password123").unwrap();
        storage.save_identity(&identity).unwrap();
        
        let loaded = storage.load_identity().unwrap();
        assert_eq!(identity.id, loaded.id);
        assert!(storage.has_identity());
    }
}
