use crate::traits::header_field::HeaderField;
use crate::utils::bytes_to_hex::bytes_to_hex;
use crate::utils::endian::Endian;

#[derive(Debug)]
pub struct ShAddralign {
    pub raw: [u8; 8],
    pub value: u64,
    pub as_hex: String
}

impl ShAddralign {
    pub fn new(raw: [u8; 8], endian: &Endian) -> Self {

        let as_hex = bytes_to_hex(&raw);
        let value = endian.read_u64(raw);

        Self { 
            raw, 
            value,
            as_hex
        }
    }
}

impl HeaderField for ShAddralign {
    fn describe(&self) -> String {
        self.value.to_string()
    }
}
