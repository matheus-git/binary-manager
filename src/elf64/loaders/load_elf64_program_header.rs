extern crate plain;
use plain::Plain;

#[repr(C)]
#[derive(Default, Debug, Clone, Copy)]
pub struct LoadELF64ProgramHeader {
    pub p_type: u32,
    pub p_flags: u32,
    pub p_offset: u64,
    pub p_vaddr: u64,
    pub p_paddr: u64,
    pub p_filesz: u64,
    pub p_memsz: u64,
    pub p_align: u64,
}

unsafe impl Plain for LoadELF64ProgramHeader {}

impl LoadELF64ProgramHeader {
    pub fn from_bytes(buf: &[u8]) -> &LoadELF64ProgramHeader {
        plain::from_bytes(buf).expect("Buffer too short or not aligned")
    }
}
