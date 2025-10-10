#[derive(Debug)]
pub struct EShstrndx {
    pub raw: [u8; 2],
}

impl EShstrndx {
    pub fn new(raw: [u8; 2]) -> Self {
        Self { 
            raw, 
        }
    }
}
