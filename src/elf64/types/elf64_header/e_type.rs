#[derive(Debug)]
pub struct EType {
    pub raw: [u8; 2],
}

impl EType {
    pub fn new(raw: [u8; 2]) -> Self {
        Self { 
            raw, 
        }
    }
}
