mod e_ident;
mod e_type;
mod e_machine;
mod e_version;
mod e_entry;
mod e_phoff;
mod e_shoff;
mod e_flags;
mod e_ehsize;
mod e_phentsize;
mod e_phnum;
mod e_shentsize;
mod e_shnum;
mod e_shstrndx;

use e_ehsize::EEhsize;
use e_entry::EEntry;
use e_flags::EFlags;
use e_ident::EIdent;
use e_machine::EMachine;
use e_phentsize::EPhentsize;
use e_phnum::EPhnum;
use e_phoff::EPhoff;
use e_shentsize::EShentsize;
use e_shnum::EShnum;
use e_shoff::EShoff;
use e_shstrndx::EShstrndx;
use e_type::EType;
use e_version::EVersion;

use crate::elf64::loaders::load_elf64_header::LoadELF64Header;

#[derive(Debug)]
pub struct Elf64Header {
    pub e_ident: EIdent,
    pub e_type: EType,
    pub e_machine: EMachine,
    pub e_version: EVersion,
    pub e_entry: EEntry,
    pub e_phoff: EPhoff,
    pub e_shoff: EShoff,
    pub e_flags: EFlags,
    pub e_ehsize: EEhsize,
    pub e_phentsize: EPhentsize,
    pub e_phnum: EPhnum,
    pub e_shentsize: EShentsize,
    pub e_shnum: EShnum,
    pub e_shstrndx: EShstrndx
}

impl Elf64Header {
    pub fn new(load: &LoadELF64Header) -> Self {
        Self { 
            e_ident: EIdent::new(load.e_ident),
            e_type: EType::new(load.e_type),
            e_machine: EMachine::new(load.e_machine),
            e_version: EVersion::new(load.e_version),
            e_entry: EEntry::new(load.e_entry),
            e_phoff: EPhoff::new(load.e_phoff),
            e_shoff: EShoff::new(load.e_shoff),
            e_flags: EFlags::new(load.e_flags),
            e_ehsize: EEhsize::new(load.e_ehsize),
            e_phentsize: EPhentsize::new(load.e_phentsize),
            e_phnum: EPhnum::new(load.e_phnum),
            e_shentsize: EShentsize::new(load.e_shentsize),
            e_shnum: EShnum::new(load.e_shnum),
            e_shstrndx: EShstrndx::new(load.e_shstrndx)
        }
    }
}
