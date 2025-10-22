mod sh_addr;
mod sh_addralign;
mod sh_entsize;
mod sh_flags;
mod sh_info;
mod sh_link;
mod sh_name;
mod sh_offset;
mod sh_size;
mod sh_type;

use sh_addr::ShAddr;
use sh_addralign::ShAddralign;
use sh_entsize::ShEntsize;
use sh_flags::ShFlags;
use sh_info::ShInfo;
use sh_link::ShLink;
use sh_name::ShName;
use sh_offset::ShOffset;
use sh_size::ShSize;
use sh_type::ShType;

use crate::{elf64::loaders::load_elf64_section_header::LoadELF64SectionHeader, utils::endian::Endian};

#[derive(Debug)]
pub struct Elf64SectionHeader {
    pub sh_name: ShName,
    pub sh_type: ShType,
    pub sh_flags: ShFlags,
    pub sh_addr: ShAddr,
    pub sh_offset: ShOffset,
    pub sh_size: ShSize,
    pub sh_link: ShLink,
    pub sh_info: ShInfo,
    pub sh_addralign: ShAddralign,
    pub sh_entsize: ShEntsize
}

impl Elf64SectionHeader {
    pub fn new(load: &LoadELF64SectionHeader, endian: &Endian) -> Self {
        Self {
            sh_name: ShName::new(load.sh_name, endian),
            sh_type: ShType::new(load.sh_type, endian),
            sh_flags: ShFlags::new(load.sh_flags, endian),
            sh_addr: ShAddr::new(load.sh_addr, endian),
            sh_offset: ShOffset::new(load.sh_offset, endian),
            sh_size: ShSize::new(load.sh_size, endian),
            sh_link: ShLink::new(load.sh_link, endian),
            sh_info: ShInfo::new(load.sh_info, endian),
            sh_addralign: ShAddralign::new(load.sh_addralign, endian),
            sh_entsize: ShEntsize::new(load.sh_entsize, endian),
        }
    }
}

impl From<&Elf64SectionHeader> for Vec<u8> {
    fn from(h: &Elf64SectionHeader) -> Vec<u8> {
        let mut bytes: Vec<u8> = Vec::new();
        bytes.extend_from_slice(&h.sh_name.raw);
        bytes.extend_from_slice(&h.sh_type.raw);
        bytes.extend_from_slice(&h.sh_flags.raw);
        bytes.extend_from_slice(&h.sh_addr.raw);
        bytes.extend_from_slice(&h.sh_offset.raw);
        bytes.extend_from_slice(&h.sh_size.raw);
        bytes.extend_from_slice(&h.sh_link.raw);
        bytes.extend_from_slice(&h.sh_info.raw);
        bytes.extend_from_slice(&h.sh_addralign.raw);
        bytes.extend_from_slice(&h.sh_entsize.raw);
        bytes
    }
}
