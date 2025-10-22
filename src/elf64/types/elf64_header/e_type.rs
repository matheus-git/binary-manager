use crate::traits::header_field::HeaderField;
use crate::utils::bytes_to_hex::bytes_to_hex;
use crate::utils::endian::Endian;
use std::fmt;

#[derive(Debug)]
pub enum ETypeValue {
    None,
    Rel,
    Exec,
    Dyn,
    Core
}

impl ETypeValue {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::None => "An unknown type",
            Self::Rel => "A relocatable file",
            Self::Exec => "An executable file",
            Self::Dyn => "A shared object",
            Self::Core => "A core file"
        }
    }
}

impl fmt::Display for ETypeValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

#[derive(Debug)]
pub struct EType {
    pub raw: [u8; 2],
    pub value: ETypeValue,
    pub as_hex: String
}

impl EType {
    pub fn new(raw: [u8; 2], endian: &Endian) -> Self {

        let value = match endian.read_u16(raw) {
            1 => ETypeValue::Rel,
            2 => ETypeValue::Exec,
            3 => ETypeValue::Dyn,
            4 => ETypeValue::Core,
            _ => ETypeValue::None
        };

        let as_hex = bytes_to_hex(&raw);

        Self { 
            raw,
            value,
            as_hex
        }
    }
}

impl HeaderField for EType {
    fn describe(&self) -> String {
        self.value.as_str().to_string()
    }
}

impl From<&EType> for Vec<u8> {
    fn from(h: &EType) -> Vec<u8> {
        h.raw.to_vec()
    }
}
