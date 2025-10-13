extern crate plain;
use plain::Plain;

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct LoadELF64ProgramHeader {
    pub p_type: [u8; 4],
    pub p_flags: [u8; 4],
    pub p_offset: [u8; 8],
    pub p_vaddr: [u8; 8],
    pub p_paddr: [u8; 8],
    pub p_filesz: [u8; 8],
    pub p_memsz: [u8; 8],
    pub p_align: [u8; 8],
}

unsafe impl Plain for LoadELF64ProgramHeader {}

impl LoadELF64ProgramHeader {
    pub fn from_bytes(buf: &[u8]) -> &LoadELF64ProgramHeader {
        plain::from_bytes(buf).expect("Buffer too short or not aligned")
    }
}
