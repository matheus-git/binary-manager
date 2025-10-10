#[derive(Debug)]
pub struct EVersion {
    pub raw: [u8; 4],
}

impl EVersion {
    pub fn new(raw: [u8; 4]) -> Self {
        Self { 
            raw, 
        }
    }
}
