#[derive(Debug)]
pub struct EPhoff {
    pub raw: [u8; 8],
    pub describe: String
}

impl EPhoff {
    pub fn new(raw: [u8; 8]) -> Self {
        Self { 
            raw, 
            describe: "".to_string() 
        }
    }
}
