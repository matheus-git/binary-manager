mod loaders;
mod types;
mod printers;

use loaders::load_elf64_header::LoadELF64Header;
use loaders::load_elf64_program_header::LoadELF64ProgramHeader;
use loaders::load_elf64_section_header::LoadELF64SectionHeader;

use types::elf64_header::Elf64Header;
use types::elf64_program_header::Elf64ProgramHeader;
use types::elf64_section_header::Elf64SectionHeader;
use crate::traits::binary_trait::Binary;
use crate::utils::endian::Endian;

#[derive(Debug)]
pub struct Elf64Binary {
    header: Elf64Header,
    program_headers: Vec<Elf64ProgramHeader>,
    section_headers: Vec<Elf64SectionHeader>
}

impl Elf64Binary {
    pub fn new(buf: &[u8]) -> Self{
        let load_elf_header =  LoadELF64Header::from_bytes(buf);
        let elf_header = Elf64Header::new(load_elf_header);
        let endian: Endian = elf_header.e_ident.endian();

        let mut program_headers: Vec<Elf64ProgramHeader> = Vec::with_capacity(elf_header.e_phnum.value as usize);
        for i in 0..elf_header.e_phnum.value as usize {
            let start: usize = elf_header.e_phoff.value as usize + (elf_header.e_phentsize.value as usize * i);
            let end: usize = start + elf_header.e_phentsize.value as usize;
            let load_elf_program_headers = LoadELF64ProgramHeader::from_bytes(&buf[start..end]);
            let elf64_program_header = Elf64ProgramHeader::new(load_elf_program_headers, &endian);
            program_headers.push(elf64_program_header);
        }

        let mut section_headers: Vec<Elf64SectionHeader> = Vec::with_capacity(elf_header.e_shnum.value as usize);
        for i in 0..elf_header.e_shnum.value as usize {
            let start: usize = elf_header.e_shoff.value as usize + (elf_header.e_shentsize.value as usize * i);
            let end: usize = start + elf_header.e_shentsize.value as usize;
            let load_elf_section_headers = LoadELF64SectionHeader::from_bytes(&buf[start..end]);
            let elf64_program_header = Elf64SectionHeader::new(load_elf_section_headers, &endian);
            section_headers.push(elf64_program_header);
        }

        Self { 
            header: elf_header, 
            program_headers,
            section_headers
        }
    }
}

impl Binary for Elf64Binary {
    type Header = Elf64Header;
    type ProgramHeader = Elf64ProgramHeader;
    type SectionHeader = Elf64SectionHeader;

    fn get_header(&self) -> &Self::Header {
        &self.header
    }

    fn get_program_headers(&self) -> &[Self::ProgramHeader] {
        &self.program_headers
    }

    fn get_section_headers(&self) -> &[Self::SectionHeader] {
        &self.section_headers
    }
}
