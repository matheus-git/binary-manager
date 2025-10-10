#[derive(Debug)]
pub struct EMachine {
    pub raw: [u8; 2],
    pub describe: String
}

impl EMachine {
    pub fn new(raw: [u8; 2]) -> Self {
        Self { 
            raw, 
            describe: "".to_string() 
        }
    }
}
