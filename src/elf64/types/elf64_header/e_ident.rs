#[derive(Debug)]
pub struct EIdent {
    pub raw: [u8; 16],
}

impl EIdent {
    pub fn new(raw: [u8; 16]) -> Self {
        Self { 
            raw, 
        }
    }
}
