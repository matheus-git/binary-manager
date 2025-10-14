use crate::traits::header_field::HeaderField;
use crate::utils::bytes_to_hex::bytes_to_hex;
use crate::utils::endian::Endian;

#[derive(Debug, Clone)]
pub enum ShFlagsValue {
    A, 
    W,
    X,
    M,
}

impl ShFlagsValue {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::A => "Alloc",
            Self::W => "Write",
            Self::X => "Execute",
            Self::M => "Mask",
        }
    }

    pub fn from_raw(raw: [u8; 8], endian: &Endian) -> Vec<Self> {
        let mask = endian.read_u64(raw); 
        let mut flags = Vec::new();

        if mask & 0x1 != 0 { flags.push(Self::W); }
        if mask & 0x2 != 0 { flags.push(Self::A); }
        if mask & 0x4 != 0 { flags.push(Self::X); }
        if mask & !(0x1 | 0x2 | 0x4) != 0 { flags.push(Self::M); }

        flags
    }
}

#[derive(Debug)]
pub struct ShFlags {
    pub raw: [u8; 8],
    pub value: Vec<ShFlagsValue>,
    pub as_hex: String,
}

impl ShFlags {
    pub fn new(raw: [u8; 8], endian: &Endian) -> Self {
        let as_hex = bytes_to_hex(&raw);
        let value = ShFlagsValue::from_raw(raw, endian);

        Self { raw, value, as_hex }
    }
}

impl HeaderField for ShFlags {
    fn describe(&self) -> String {
        if self.value.is_empty() {
            "None".to_string()
        } else {
            self.value
                .iter()
                .map(|v| v.as_str())
                .collect::<Vec<_>>()
                .join(" | ")
        }
    }
}

