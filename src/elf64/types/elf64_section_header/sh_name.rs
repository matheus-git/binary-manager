use crate::traits::header_field::HeaderField;
use crate::utils::bytes_to_hex::bytes_to_hex;
use crate::utils::endian::Endian;

#[derive(Debug)]
pub struct ShName {
    pub raw: [u8; 4],
    pub value: u32,
    pub as_hex: String,
    pub name: String
}

impl ShName {
    pub fn new(raw: [u8; 4], endian: &Endian) -> Self {

        let as_hex = bytes_to_hex(&raw);
        let value = endian.read_u32(raw);

        Self { 
            raw, 
            value,
            as_hex,
            name: String::new()
        }
    }

    pub fn update_name(&mut self, name: String){
        self.name = name;
    }
}

impl HeaderField for ShName {
    fn describe(&self) -> String {
        self.name.clone()
    }
}   
