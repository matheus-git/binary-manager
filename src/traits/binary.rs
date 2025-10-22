//use crate::elf64::types::elf64_header::Elf64Header;
//use crate::elf64::types::elf64_program_header::Elf64ProgramHeader;
//use crate::elf64::types::elf64_section_header::Elf64SectionHeader;

//pub enum BinaryHeader {
//    Elf64(Elf64Header),
//}
//
//pub enum ProgramHeader {
//    Elf64(Elf64ProgramHeader),
//}
//
//pub enum SectionHeader {
//    Elf64(Elf64SectionHeader),
//}

pub trait Binary {
    type Header;
    type ProgramHeader;
    type SectionHeader;

    fn get_header(&self) -> &Self::Header;
    fn get_program_headers(&self) -> &[Self::ProgramHeader];
    fn get_section_headers(&self) -> &[Self::SectionHeader];
}
