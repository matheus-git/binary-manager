use crate::traits::header_field::HeaderField;
use crate::utils::bytes_to_hex::bytes_to_hex;
use crate::utils::endian::Endian;

#[derive(Debug)]
pub struct ShOffset {
    pub raw: [u8; 8],
    pub value: String,
    pub as_hex: String
}

impl ShOffset {
    pub fn new(raw: [u8; 8], endian: &Endian) -> Self {

        let as_hex = bytes_to_hex(&raw);
        let value = format!("0x{:X}", endian.read_u64(raw));

        Self { 
            raw, 
            value,
            as_hex
        }
    }
}

impl HeaderField for ShOffset {
    fn describe(&self) -> String {
        self.value.clone()
    }
}
