#[derive(Debug)]
pub struct EShnum {
    pub raw: [u8; 2],
}

impl EShnum{
    pub fn new(raw: [u8; 2]) -> Self {
        Self { 
            raw, 
        }
    }
}
