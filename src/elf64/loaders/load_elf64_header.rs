extern crate plain;
use plain::Plain;

#[repr(C)]
#[derive(Debug)]
pub struct LoadELF64Header {
    pub e_ident: [u8; 16],
    pub e_type: [u8; 2],
    pub e_machine: [u8; 2],
    pub e_version: [u8; 4],
    pub e_entry: [u8; 8],
    pub e_phoff: [u8; 8],
    pub e_shoff: [u8; 8],
    pub e_flags: [u8; 4],
    pub e_ehsize: [u8; 2],
    pub e_phentsize: [u8; 2],
    pub e_phnum: [u8; 2],
    pub e_shentsize: [u8; 2],
    pub e_shnum: [u8; 2],
    pub e_shstrndx: [u8; 2],
}

unsafe impl Plain for LoadELF64Header {}

impl LoadELF64Header {
	pub fn from_bytes(buf: &[u8]) -> &LoadELF64Header {
		plain::from_bytes(buf).expect("The buffer is either too short or not aligned!")
	}
}
