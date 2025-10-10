#[derive(Debug)]
pub struct EMachine {
    pub raw: [u8; 2],
}

impl EMachine {
    pub fn new(raw: [u8; 2]) -> Self {
        Self { 
            raw, 
        }
    }
}
