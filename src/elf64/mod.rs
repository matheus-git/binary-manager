mod loaders;
mod types;

use loaders::load_elf64_header::LoadELF64Header;
use loaders::load_elf64_program_header::LoadELF64ProgramHeader;

use types::elf64_header::Elf64Header;
use types::elf64_program_header::Elf64ProgramHeader;
use crate::traits::binary_trait::BinaryTrait;
use crate::utils::endian::Endian;

#[derive(Debug)]
pub struct Elf64Binary {
    header: Elf64Header,
    programs_header: Vec<Elf64ProgramHeader>,
}

impl Elf64Binary {
    pub fn new(buf: &[u8]) -> Self{
        let load_elf_header =  LoadELF64Header::from_bytes(buf);
        let elf_header = Elf64Header::new(load_elf_header);
        let endian: Endian = elf_header.e_ident.endian();

        let mut programs_header: Vec<Elf64ProgramHeader> = Vec::with_capacity(elf_header.e_phnum.value as usize);
        for i in 0..elf_header.e_phnum.value as usize {
            let start: usize = elf_header.e_phoff.value as usize + (elf_header.e_phentsize.value as usize * i);
            let end: usize = start + elf_header.e_phentsize.value as usize;
            let load_elf_programs_header = LoadELF64ProgramHeader::from_bytes(&buf[start..end]);
            let elf64_program_header = Elf64ProgramHeader::new(load_elf_programs_header, &endian);
            programs_header.push(elf64_program_header);
        }

        Self { 
            header: elf_header, 
            programs_header
        }
    }
}

impl BinaryTrait for Elf64Binary {
    type Header = Elf64Header;
    type ProgramHeader = Elf64ProgramHeader;

    fn get_header(&self) -> &Self::Header {
        &self.header
    }

    fn get_program_headers(&self) -> &[Self::ProgramHeader] {
        &self.programs_header
    }
}
