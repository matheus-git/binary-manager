#[derive(Debug)]
pub struct EShstrndx {
    pub raw: [u8; 2],
    pub describe: String
}

impl EShstrndx {
    pub fn new(raw: [u8; 2]) -> Self {
        Self { 
            raw, 
            describe: "".to_string() 
        }
    }
}
