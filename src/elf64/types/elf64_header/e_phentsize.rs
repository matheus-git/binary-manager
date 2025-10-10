#[derive(Debug)]
pub struct EPhentsize {
    pub raw: [u8; 2],
}

impl EPhentsize {
    pub fn new(raw: [u8; 2]) -> Self {
        Self { 
            raw, 
        }
    }
}
