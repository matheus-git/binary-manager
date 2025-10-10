#[derive(Debug)]
pub struct EPhoff {
    pub raw: [u8; 8],
}

impl EPhoff {
    pub fn new(raw: [u8; 8]) -> Self {
        Self { 
            raw, 
        }
    }
}
