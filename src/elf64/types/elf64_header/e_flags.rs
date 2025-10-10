#[derive(Debug)]
pub struct EFlags {
    pub raw: [u8; 4],
    pub describe: String
}

impl EFlags {
    pub fn new(raw: [u8; 4]) -> Self {
        Self { 
            raw, 
            describe: "".to_string() 
        }
    }
}
