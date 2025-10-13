use crate::traits::header_field::HeaderField;
use crate::utils::bytes_to_hex::bytes_to_hex;
use crate::utils::endian::Endian;

#[derive(Debug)]
pub enum PTypeValue {
    Null,
    Load,
    Dynamic,
    Interp,
    Note,
    Shlib,
    Phdr,
    Loproc,
    Hiproc,
    GnuStack,
    Unknown(u32),
}

impl PTypeValue {
    pub fn as_str(&self) -> &'static str {
        match self {
            PTypeValue::Null => "NULL",
            PTypeValue::Load => "LOAD",
            PTypeValue::Dynamic => "DYNAMIC",
            PTypeValue::Interp => "INTERP",
            PTypeValue::Note => "NOTE",
            PTypeValue::Shlib => "SHLIB",
            PTypeValue::Phdr => "PHDR",
            PTypeValue::Loproc => "LOPROC",
            PTypeValue::Hiproc => "HIPROC",
            PTypeValue::GnuStack => "GNU_STACK",
            PTypeValue::Unknown(_) => "UNKNOWN",
        }
    }

    pub fn from_raw(raw: [u8; 4], endian: &Endian) -> Self {
        const PT_LOPROC: u32 = 0x70000000;
        const PT_HIPROC: u32 = 0x7fffffff;
        const PT_GNU_STACK: u32 = 0x6474e550;

        let val = endian.read_u32(raw);

        match val {
            0 => PTypeValue::Null,
            1 => PTypeValue::Load,
            2 => PTypeValue::Dynamic,
            3 => PTypeValue::Interp,
            4 => PTypeValue::Note,
            5 => PTypeValue::Shlib,
            6 => PTypeValue::Phdr,
            PT_LOPROC..=PT_HIPROC => PTypeValue::Loproc,
            PT_GNU_STACK => PTypeValue::GnuStack,
            _ => PTypeValue::Unknown(val),
        }
    }
}

#[derive(Debug)]
pub struct PType {
    pub raw: [u8; 4],
    pub value: PTypeValue,
    pub as_hex: String,
}

impl PType {
    pub fn new(raw: [u8; 4], endian: &Endian) -> Self {
        let as_hex = bytes_to_hex(&raw);
        let value = PTypeValue::from_raw(raw, endian);

        Self {
            raw,
            value,
            as_hex,
        }
    }
}

impl HeaderField for PType {
    fn describe(&self) -> String {
        self.value.as_str().to_string()
    }
}
