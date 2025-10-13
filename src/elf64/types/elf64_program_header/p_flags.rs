use crate::traits::header_field::HeaderField;
use crate::utils::bytes_to_hex::bytes_to_hex;
use crate::utils::endian::Endian;

#[derive(Debug)]
pub enum PFlagsValue {
    R,
    W,
    X,
}

impl PFlagsValue {
    pub fn as_str(&self) -> &'static str {
        match self {
            PFlagsValue::R => "Read",
            PFlagsValue::W => "Write",
            PFlagsValue::X => "Execute",
        }
    }

    pub fn from_raw(raw: [u8; 4], endian: &Endian) -> Vec<Self> {
        let mask = endian.read_u32(raw);
        let mut flags = Vec::new();
        if mask & 0x4 != 0 { flags.push(PFlagsValue::R); }
        if mask & 0x2 != 0 { flags.push(PFlagsValue::W); }
        if mask & 0x1 != 0 { flags.push(PFlagsValue::X); }
        flags
    }
}

#[derive(Debug)]
pub struct PFlags {
    pub raw: [u8; 4],
    pub value: Vec<PFlagsValue>,
    pub as_hex: String,
}

impl PFlags {
    pub fn new(raw: [u8; 4], endian: &Endian) -> Self {
        let as_hex = bytes_to_hex(&raw);
        let value = PFlagsValue::from_raw(raw, endian);

        Self { 
            raw,
            value,
            as_hex,
        }
    }

    pub fn value_as_string(&self) -> String {
        if self.value.is_empty() {
            "None".to_string()
        } else {
            self.value.iter()
                .map(|f| f.as_str())
                .collect::<Vec<&str>>()
                .join(" | ")
        }
    }
}

impl HeaderField for PFlags {
    fn describe(&self) -> String {
        self.value_as_string()
    }
}
