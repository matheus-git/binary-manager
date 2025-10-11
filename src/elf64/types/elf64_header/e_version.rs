use crate::traits::header_field::HeaderField;
use crate::utils::bytes_to_hex::bytes_to_hex;
use crate::utils::endian::Endian;
use std::fmt;

#[derive(Debug)]
pub enum EVersionValue {
    None,
    Current
}

impl EVersionValue {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "Invalid version",
            Self::Current => "Current version",
        }
    }
}

impl fmt::Display for EVersionValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}


#[derive(Debug)]
pub struct EVersion {
    pub raw: [u8; 4],
    pub value: EVersionValue,
    pub as_hex: String
}

impl EVersion {
    pub fn new(raw: [u8; 4], endian: &Endian) -> Self {

        let value = match endian.read_u32(raw) {
            1 => EVersionValue::Current,
            _ => EVersionValue::None
        };

        let as_hex = bytes_to_hex(&raw);
        
        Self { 
            raw, 
            value,
            as_hex
        }
    }
}

impl HeaderField for EVersion {
    fn describe(&self) -> String {
        self.value.as_str().to_string()
    }
}
