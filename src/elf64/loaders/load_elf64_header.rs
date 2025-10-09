extern crate plain;
use plain::Plain;

#[repr(C)]
#[derive(Default, Debug)]
pub struct LoadELF64Header {
    pub e_ident: [u8; 16],
    pub e_type: u16,
    pub e_machine: u16,
    pub e_version: u32,
    pub e_entry: u64,
    pub e_phoff: u64,
    pub e_shoff: u64,
    pub e_flags: u32,
    pub e_ehsize: u16,
    pub e_phentsize: u16,
    pub e_phnum: u16,
    pub e_shentsize: u16,
    pub e_shnum: u16,
    pub e_shstrndx: u16,
}

unsafe impl Plain for LoadELF64Header {}

impl LoadELF64Header {
	pub fn from_bytes(buf: &[u8]) -> &LoadELF64Header {
		plain::from_bytes(buf).expect("The buffer is either too short or not aligned!")
	}
}
