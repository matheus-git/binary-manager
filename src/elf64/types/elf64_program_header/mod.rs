mod p_type;
mod p_paddr;
mod p_flags;
mod p_align;
mod p_filesz;
mod p_memsz;
mod p_offset;
mod p_vaddr;

use p_align::PAlign;
use p_type::PType;
use p_paddr::PPaddr;
use p_flags::PFlags;
use p_filesz::PFilesz;
use p_memsz::PMemsz;
use p_offset::POffset;
use p_vaddr::PVaddr;
use super::super::LoadELF64ProgramHeader;
use crate::utils::endian::Endian;

#[derive(Debug)]
pub struct Elf64ProgramHeader {
    pub p_type: PType,
    pub p_flags: PFlags,
    pub p_offset: POffset,
    pub p_vaddr: PVaddr,
    pub p_paddr: PPaddr,
    pub p_filesz: PFilesz,
    pub p_memsz: PMemsz,
    pub p_align: PAlign
}

impl Elf64ProgramHeader {
    pub fn new(load: &LoadELF64ProgramHeader, endian: &Endian) -> Self {
        Self { 
            p_type: PType::new(load.p_type, endian),
            p_offset: POffset::new(load.p_offset, endian),
            p_vaddr: PVaddr::new(load.p_vaddr, endian),
            p_paddr: PPaddr::new(load.p_paddr, endian),
            p_filesz: PFilesz::new(load.p_filesz, endian),
            p_memsz: PMemsz::new(load.p_memsz, endian),
            p_flags: PFlags::new(load.p_flags, endian),
            p_align: PAlign::new(load.p_align, endian),
        }
    }
}

impl From<&Elf64ProgramHeader> for Vec<u8> {
    fn from(h: &Elf64ProgramHeader) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend_from_slice(&h.p_type.raw);
        bytes.extend_from_slice(&h.p_flags.raw);
        bytes.extend_from_slice(&h.p_offset.raw);
        bytes.extend_from_slice(&h.p_vaddr.raw);
        bytes.extend_from_slice(&h.p_paddr.raw);
        bytes.extend_from_slice(&h.p_filesz.raw);
        bytes.extend_from_slice(&h.p_memsz.raw);
        bytes.extend_from_slice(&h.p_align.raw);
        bytes
    }
}
