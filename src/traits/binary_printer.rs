pub trait BinaryPrinter {
    type Header;
    type ProgramHeader;
    type SectionHeader;

    fn print_header(&self, header: &Self::Header);
    fn print_program_headers(&self, phs: &[Self::ProgramHeader]);
    fn print_section_headers(&self, shs: &[Self::SectionHeader]);
}
