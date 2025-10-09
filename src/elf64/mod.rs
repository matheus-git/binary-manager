mod loaders;
mod types;

use loaders::load_elf64_header::LoadELF64Header;
use loaders::load_elf64_program_header::LoadELF64ProgramHeader;
use crate::traits::binary_trait::BinaryTrait;

struct Elf64Binary<'a> {
    header: &'a LoadELF64Header,
    programs_header: Vec<&'a LoadELF64ProgramHeader>,
}

impl Elf64Binary<'_> {
    fn new(buf: &[u8]) -> Self{
        let elf_header =  LoadELF64Header::from_bytes(buf);
        let elf_programs_header = LoadELF64ProgramHeader::from_bytes(&buf[(elf_header.e_phoff as usize + elf_header.e_phentsize as usize * 5) as usize..] as &[u8]);

        Self { 
            header: elf_header, 
            programs_header: vec![elf_programs_header]
        }
    }
}

impl BinaryTrait for Elf64Binary<'_> {
    type Header = LoadELF64Header;
    type ProgramHeader = LoadELF64ProgramHeader;

    fn get_header(&self) -> &Self::Header {
        &self.header
    }

    fn get_program_headers(&self) -> &[&Self::ProgramHeader] {
        &self.programs_header
    }
}


pub fn load(bytes: Vec<u8>){
        println!("{:?}", elf_header);

    println!("{:?}", );
}
