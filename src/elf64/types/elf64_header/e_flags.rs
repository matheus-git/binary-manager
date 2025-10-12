use crate::{traits::header_field::HeaderField, utils::{bytes_to_hex::bytes_to_hex, endian::Endian}};

#[derive(Debug)]
pub struct EFlags {
    pub raw: [u8; 4],
    pub value: u32,
    pub as_hex: String
}

impl EFlags {
    pub fn new(raw: [u8; 4], endian: &Endian) -> Self {

        let value = endian.read_u32(raw);
        let as_hex = bytes_to_hex(&raw);

        Self { 
            raw, 
            value,
            as_hex
        }
    }
}

impl HeaderField for EFlags {
    fn describe(&self) -> String {
        self.value.to_string()
    }
}
