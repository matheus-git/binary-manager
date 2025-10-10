#[derive(Debug)]
pub struct EVersion {
    pub raw: [u8; 4],
    pub describe: String
}

impl EVersion {
    pub fn new(raw: [u8; 4]) -> Self {
        Self { 
            raw, 
            describe: "".to_string() 
        }
    }
}
