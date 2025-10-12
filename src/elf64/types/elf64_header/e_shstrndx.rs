use crate::{traits::header_field::HeaderField, utils::{bytes_to_hex::bytes_to_hex, endian::Endian}};

#[derive(Debug)]
pub struct EShstrndx {
    pub raw: [u8; 2],
    pub value: u16,
    pub as_hex: String
}

impl EShstrndx {
    pub fn new(raw: [u8; 2], endian: &Endian) -> Self {

        let value = endian.read_u16(raw);
        let as_hex = bytes_to_hex(&raw);

        Self { 
            raw, 
            value,
            as_hex
        }
    }
}

impl HeaderField for EShstrndx {
    fn describe(&self) -> String {
        self.value.to_string()
    }
}
