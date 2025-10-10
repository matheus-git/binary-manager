#[derive(Debug)]
pub struct EFlags {
    pub raw: [u8; 4],
}

impl EFlags {
    pub fn new(raw: [u8; 4]) -> Self {
        Self { 
            raw, 
        }
    }
}
