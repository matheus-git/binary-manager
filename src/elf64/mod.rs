mod loaders;
mod types;
pub mod printers;

use loaders::load_elf64_header::LoadELF64Header;
use loaders::load_elf64_program_header::LoadELF64ProgramHeader;
use loaders::load_elf64_section_header::LoadELF64SectionHeader;

use types::elf64_header::Elf64Header;
use types::elf64_program_header::Elf64ProgramHeader;
use types::elf64_section_header::Elf64SectionHeader;
use crate::traits::binary_trait::Binary;
use crate::utils::endian::Endian;
use crate::utils::string_until_null::string_until_null;

fn parse_program_headers(buf: &[u8], elf_header: &Elf64Header, endian: &Endian) -> Vec<Elf64ProgramHeader> {
    let mut headers = Vec::with_capacity(elf_header.e_phnum.value as usize);

    for i in 0..elf_header.e_phnum.value as usize {
        let start = elf_header.e_phoff.value as usize + i * elf_header.e_phentsize.value as usize;
        let end = start + elf_header.e_phentsize.value as usize;

        let raw_header = LoadELF64ProgramHeader::from_bytes(&buf[start..end]);
        headers.push(Elf64ProgramHeader::new(raw_header, endian));
    }

    headers
}

fn parse_section_headers(buf: &[u8], elf_header: &Elf64Header, endian: &Endian) -> Vec<Elf64SectionHeader> {
    let mut headers = Vec::with_capacity(elf_header.e_shnum.value as usize);

    for i in 0..elf_header.e_shnum.value as usize {
        let start = elf_header.e_shoff.value as usize + i * elf_header.e_shentsize.value as usize;
        let end = start + elf_header.e_shentsize.value as usize;

        let raw_header = LoadELF64SectionHeader::from_bytes(&buf[start..end]);
        headers.push(Elf64SectionHeader::new(raw_header, endian));
    }

    headers
}

fn resolve_section_name(section_headers: &mut Vec<Elf64SectionHeader>, buf: &[u8], elf_header: &Elf64Header, endian: &Endian){
    let strtab_section = &section_headers[elf_header.e_shstrndx.value as usize ];
    let strtab_section_offset = endian.read_u64(strtab_section.sh_offset.raw);

    for section in section_headers {
        let name_index = section.sh_name.value;
        let name = string_until_null(&buf[(strtab_section_offset as usize + name_index as usize)..]);
        section.sh_name.update_name(name.to_string());
    }
}

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
        
        let program_headers = parse_program_headers(buf, &elf_header, &endian);
        let mut section_headers = parse_section_headers(buf, &elf_header, &endian);

        resolve_section_name(&mut section_headers, buf, &elf_header, &endian);

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
