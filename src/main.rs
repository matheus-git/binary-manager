mod elf64;
mod traits;
mod utils;

use elf64::Elf64Binary;
use traits::binary_trait::BinaryTrait;

use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let path = "/home/matheus/Downloads/code/chapter5/ctf";
    let bytes: Vec<u8> = fs::read(path)?;

    println!("{:?}", Elf64Binary::new(&bytes).get_section_headers());
    Ok(())
}
