#[derive(Debug)]
pub struct EPhnum {
    pub raw: [u8; 2],
}

impl EPhnum {
    pub fn new(raw: [u8; 2]) -> Self {
        Self { 
            raw, 
        }
    }
}
