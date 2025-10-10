#[derive(Debug)]
pub struct EShoff {
    pub raw: [u8; 8],
}

impl EShoff {
    pub fn new(raw: [u8; 8]) -> Self {
        Self { 
            raw, 
        }
    }
}
