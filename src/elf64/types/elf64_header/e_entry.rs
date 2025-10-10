#[derive(Debug)]
pub struct EEntry {
    pub raw: [u8; 8],
    pub describe: String
}

impl EEntry {
    pub fn new(raw: [u8; 8]) -> Self {
        Self { 
            raw, 
            describe: "".to_string() 
        }
    }
}
