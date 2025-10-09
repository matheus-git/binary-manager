mod elf64;
mod traits;

use elf64::Elf64Binary;

use std::fs;
use std::io;

fn main() -> io::Result<()> {
    let path = "/home/matheus/Downloads/code/chapter5/ctf";
    let bytes: Vec<u8> = fs::read(path)?;

    Elf64Binary::new(&bytes);
    Ok(())
}
