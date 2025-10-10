#[derive(Debug)]
pub struct EEhsize {
    pub raw: [u8; 2],
}

impl EEhsize {
    pub fn new(raw: [u8; 2]) -> Self {
        Self { 
            raw, 
        }
    }
}
