use crate::traits::binary_printer::BinaryPrinter;

use super::types::{elf64_header::Elf64Header, elf64_program_header::Elf64ProgramHeader, elf64_section_header::Elf64SectionHeader};

pub struct Elf64Printer;

impl BinaryPrinter for Elf64Printer {
    type Header = Elf64Header;
    type ProgramHeader = Elf64ProgramHeader;
    type SectionHeader = Elf64SectionHeader;

    fn print_header(&self, header: &Self::Header){
        println!("Header");
    }

    fn print_program_headers(&self, phs: &[Self::ProgramHeader]) {
        
    }

    fn print_section_headers(&self, shs: &[Self::SectionHeader]) {
        
    }
}
