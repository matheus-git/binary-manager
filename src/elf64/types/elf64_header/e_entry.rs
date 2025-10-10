#[derive(Debug)]
pub struct EEntry {
    pub raw: [u8; 8],
}

impl EEntry {
    pub fn new(raw: [u8; 8]) -> Self {
        Self { 
            raw, 
        }
    }
}
