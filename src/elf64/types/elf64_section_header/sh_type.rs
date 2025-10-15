use crate::traits::header_field::HeaderField;
use crate::utils::bytes_to_hex::bytes_to_hex;
use crate::utils::endian::Endian;

#[derive(Debug, Clone)]
pub enum ShTypeValue {
    Null,
    ProgBits,
    SymTab,
    StrTab,
    Rela,
    Hash,
    Dynamic,
    Note,
    NoBits,
    Rel,
    DynSym,
    Other(()), 
}

impl ShTypeValue {
    pub fn from_u32(raw: u32) -> Self {
        match raw {
            0 => ShTypeValue::Null,
            1 => ShTypeValue::ProgBits,
            2 => ShTypeValue::SymTab,
            3 => ShTypeValue::StrTab,
            4 => ShTypeValue::Rela,
            5 => ShTypeValue::Hash,
            6 => ShTypeValue::Dynamic,
            7 => ShTypeValue::Note,
            8 => ShTypeValue::NoBits,
            9 => ShTypeValue::Rel,
            11 => ShTypeValue::DynSym,
            _ => ShTypeValue::Other(()),
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ShTypeValue::Null => "NULL",
            ShTypeValue::ProgBits => "PROGBITS",
            ShTypeValue::SymTab => "SYMTAB",
            ShTypeValue::StrTab => "STRTAB",
            ShTypeValue::Rela => "RELA",
            ShTypeValue::Hash => "HASH",
            ShTypeValue::Dynamic => "DYNAMIC",
            ShTypeValue::Note => "NOTE",
            ShTypeValue::NoBits => "NOBITS",
            ShTypeValue::Rel => "REL",
            ShTypeValue::DynSym => "DYNSYM",
            ShTypeValue::Other(_) => "OTHER",
        }
    }
}

#[derive(Debug)]
pub struct ShType {
    pub raw: [u8; 4],
    pub value: ShTypeValue,
    pub as_hex: String,
}

impl ShType {
    pub fn new(raw: [u8; 4], endian: &Endian) -> Self {
        let as_hex = bytes_to_hex(&raw);
        let raw_value = endian.read_u32(raw);
        let value = ShTypeValue::from_u32(raw_value);

        Self { raw, value, as_hex }
    }
}

impl HeaderField for ShType {
    fn describe(&self) -> String {
        self.value.as_str().to_string()
    }
}
