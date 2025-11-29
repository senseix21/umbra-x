/// MLS group state (stub for W7-W9)
pub struct GroupState {
    pub epoch: u64,
}

impl GroupState {
    pub fn new() -> Self {
        Self { epoch: 0 }
    }
}
