pub trait BinaryTrait {
    type Header;
    type ProgramHeader;

    fn get_header(&self) -> &Self::Header;
    fn get_program_headers(&self) -> &[Self::ProgramHeader];
}
