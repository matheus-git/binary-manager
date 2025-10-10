mod loaders;
mod types;

use loaders::load_elf64_header::LoadELF64Header;
use loaders::load_elf64_program_header::LoadELF64ProgramHeader;

use types::elf64_header::Elf64Header;
use crate::traits::binary_trait::BinaryTrait;

#[derive(Debug)]
pub struct Elf64Binary {
    header: Elf64Header,
    programs_header: Vec<Elf64Header>,
}

impl Elf64Binary {
    pub fn new(buf: &[u8]) -> Self{
        let load_elf_header =  LoadELF64Header::from_bytes(buf);
        let elf_header = Elf64Header::new(load_elf_header);

        let elf_programs_header = Elf64Header::new(load_elf_header);

        Self { 
            header: elf_header, 
            programs_header: vec![elf_programs_header]
        }
    }
}

impl BinaryTrait for Elf64Binary {
    type Header = Elf64Header;
    type ProgramHeader = Elf64Header;

    fn get_header(&self) -> &Self::Header {
        &self.header
    }

    fn get_program_headers(&self) -> &[Self::ProgramHeader] {
        &self.programs_header
    }
}
