#[derive(Debug)]
pub struct EShentsize {
    pub raw: [u8; 2],
    pub describe: String
}

impl EShentsize {
    pub fn new(raw: [u8; 2]) -> Self {
        Self { 
            raw, 
            describe: "".to_string() 
        }
    }
}
