use crate::{traits::header_field::HeaderField, utils::{bytes_to_hex::bytes_to_hex, endian::Endian}};

#[derive(Debug)]
pub struct EPhoff {
    pub raw: [u8; 8],
    pub value: u64,
    pub as_hex: String
}

impl EPhoff {
    pub fn new(raw: [u8; 8], endian: &Endian) -> Self {

        let value: u64 = endian.read_u64(raw); 
        let as_hex = bytes_to_hex(&raw);

        Self { 
            raw, 
            value,
            as_hex
        }
    }
}

impl HeaderField for EPhoff {
    fn describe(&self) -> String {
        self.value.to_string()
    }
}
