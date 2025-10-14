use crate::traits::header_field::HeaderField;
use crate::utils::bytes_to_hex::bytes_to_hex;
use crate::utils::endian::Endian;

#[derive(Debug)]
pub struct ShInfo {
    pub raw: [u8; 4],
    pub value: u32,
    pub as_hex: String
}

impl ShInfo {
    pub fn new(raw: [u8; 4], endian: &Endian) -> Self {

        let as_hex = bytes_to_hex(&raw);
        let value = endian.read_u32(raw);

        Self { 
            raw, 
            value,
            as_hex
        }
    }
}

impl HeaderField for ShInfo {
    fn describe(&self) -> String {
        self.value.to_string()
    }
}
