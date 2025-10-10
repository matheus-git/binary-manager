#[derive(Debug)]
pub struct EShentsize {
    pub raw: [u8; 2],
}

impl EShentsize {
    pub fn new(raw: [u8; 2]) -> Self {
        Self { 
            raw, 
        }
    }
}
